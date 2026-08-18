use std::{
    env,
    ffi::OsString,
    fs,
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use bf_codegen::{CodeFile, CompileOptions, compile_function};
use clap::{Args, Parser, Subcommand};

const ENTRY_NAME: &str = "bf_entry";
const BFNI_HEADER: &str = include_str!("../assets/bfni.h");
const HOST_RUNTIME: &str = include_str!("../assets/bfni_host.c");

/// Compile and run Brainfuck programs.
#[derive(Debug, Parser)]
#[command(name = "bf", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compile a BF source to a temporary native executable and run it.
    Run {
        #[command(flatten)]
        translation: TranslationArgs,

        /// Number of cells supplied by the host runtime.
        #[arg(long, default_value_t = 4096, value_name = "CELLS")]
        tape_len: u64,

        /// C compiler command. Defaults to $CC, then cc.
        #[arg(long, value_name = "COMMAND")]
        cc: Option<OsString>,
    },

    /// Build a BF source as a native executable using the host runtime.
    Build {
        #[command(flatten)]
        translation: TranslationArgs,

        /// Native executable to create. Defaults to the source file stem.
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// C compiler command. Defaults to $CC, then cc.
        #[arg(long, value_name = "COMMAND")]
        cc: Option<OsString>,
    },

    /// Translate a BF source into freestanding C.
    #[command(name = "trans2c")]
    Trans2c {
        #[command(flatten)]
        translation: TranslationArgs,

        /// C file to create. Use - for stdout; defaults to SOURCE with .c.
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Args)]
struct TranslationArgs {
    /// Brainfuck source file.
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// BF IR optimization level.
    #[arg(
        short = 'O',
        default_value_t = 1,
        value_parser = clap::value_parser!(u8).range(0..=1)
    )]
    opt_level: u8,

    /// Disable generated tape boundary checks.
    #[arg(long = "unsafe")]
    unsafe_mode: bool,
}

fn translate(args: &TranslationArgs) -> Result<Vec<u8>, String> {
    let source = fs::read_to_string(&args.source)
        .map_err(|error| format!("failed to read {}: {error}", args.source.display()))?;
    let options = CompileOptions {
        opt_level: i32::from(args.opt_level),
        boundary_check: !args.unsafe_mode,
    };
    let function = compile_function(&source, ENTRY_NAME, options).map_err(|error| {
        format!(
            "failed to compile {} as {ENTRY_NAME}: {error}",
            args.source.display()
        )
    })?;

    let mut code_file = CodeFile::new();
    code_file
        .add_bf_func(function)
        .map_err(|error| error.to_string())?;

    let mut output = Vec::new();
    code_file
        .write_to(&mut output)
        .map_err(|error| format!("failed to generate C: {error}"))?;
    Ok(output)
}

fn write_c_output(code: &[u8], output: &Path) -> Result<(), String> {
    if output == Path::new("-") {
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        writer
            .write_all(code)
            .and_then(|()| writer.flush())
            .map_err(|error| format!("failed to write standard output: {error}"))
    } else {
        let file = File::create(output)
            .map_err(|error| format!("failed to create {}: {error}", output.display()))?;
        let mut writer = BufWriter::new(file);
        writer
            .write_all(code)
            .and_then(|()| writer.flush())
            .map_err(|error| format!("failed to write {}: {error}", output.display()))
    }
}

fn native_output_path(source: &Path) -> Result<PathBuf, String> {
    let stem = source
        .file_stem()
        .ok_or_else(|| format!("source has no file stem: {}", source.display()))?;
    let mut name = OsString::from(stem);
    name.push(env::consts::EXE_SUFFIX);
    Ok(PathBuf::from(name))
}

fn compiler_command(requested: Option<OsString>) -> OsString {
    requested
        .or_else(|| env::var_os("CC"))
        .unwrap_or_else(|| "cc".into())
}

fn build_native(
    translation: &TranslationArgs,
    output: &Path,
    cc: Option<OsString>,
) -> Result<(), String> {
    let code = translate(translation)?;
    let temp = tempfile::tempdir()
        .map_err(|error| format!("failed to create temporary build directory: {error}"))?;
    let generated_path = temp.path().join("generated.c");
    let runtime_path = temp.path().join("bfni_host.c");
    let header_path = temp.path().join("bfni.h");

    fs::write(&generated_path, code)
        .map_err(|error| format!("failed to write {}: {error}", generated_path.display()))?;
    fs::write(&runtime_path, HOST_RUNTIME)
        .map_err(|error| format!("failed to write {}: {error}", runtime_path.display()))?;
    fs::write(&header_path, BFNI_HEADER)
        .map_err(|error| format!("failed to write {}: {error}", header_path.display()))?;

    let compiler = compiler_command(cc);
    let status = Command::new(&compiler)
        .arg("-std=c11")
        .arg("-O2")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Wpedantic")
        .arg("-Wno-unused-variable")
        .arg("-I")
        .arg(temp.path())
        .arg(&runtime_path)
        .arg(&generated_path)
        .arg("-o")
        .arg(output)
        .status()
        .map_err(|error| {
            format!(
                "failed to start C compiler {}: {error}",
                compiler.to_string_lossy()
            )
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("C compiler exited with {status}"))
    }
}

fn child_exit_code(status: std::process::ExitStatus) -> ExitCode {
    match status.code().and_then(|code| u8::try_from(code).ok()) {
        Some(code) => ExitCode::from(code),
        None => ExitCode::FAILURE,
    }
}

fn dispatch(cli: Cli) -> Result<ExitCode, String> {
    match cli.command {
        Commands::Trans2c {
            translation,
            output,
        } => {
            let code = translate(&translation)?;
            let output = output.unwrap_or_else(|| translation.source.with_extension("c"));
            write_c_output(&code, &output)?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Build {
            translation,
            output,
            cc,
        } => {
            let output = match output {
                Some(path) => path,
                None => native_output_path(&translation.source)?,
            };
            build_native(&translation, &output, cc)?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Run {
            translation,
            tape_len,
            cc,
        } => {
            if tape_len == 0 {
                return Err("tape length must be greater than zero".into());
            }

            let temp = tempfile::tempdir()
                .map_err(|error| format!("failed to create temporary run directory: {error}"))?;
            let executable = temp
                .path()
                .join(format!("bf-run{}", env::consts::EXE_SUFFIX));
            build_native(&translation, &executable, cc)?;

            let status = Command::new(&executable)
                .arg(tape_len.to_string())
                .status()
                .map_err(|error| format!("failed to run {}: {error}", executable.display()))?;
            Ok(child_exit_code(status))
        }
    }
}

fn main() -> ExitCode {
    match dispatch(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("bf: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_run_options() {
        let cli = Cli::try_parse_from([
            "bf",
            "run",
            "hello.bf",
            "-O0",
            "--unsafe",
            "--tape-len",
            "64",
        ])
        .unwrap();

        let Commands::Run {
            translation,
            tape_len,
            ..
        } = cli.command
        else {
            panic!("expected run command");
        };
        assert_eq!(translation.source, PathBuf::from("hello.bf"));
        assert_eq!(translation.opt_level, 0);
        assert!(translation.unsafe_mode);
        assert_eq!(tape_len, 64);
    }

    #[test]
    fn parses_build_output() {
        let cli = Cli::try_parse_from(["bf", "build", "hello.bf", "-o", "hello-bin"]).unwrap();

        let Commands::Build { output, .. } = cli.command else {
            panic!("expected build command");
        };
        assert_eq!(output, Some(PathBuf::from("hello-bin")));
    }

    #[test]
    fn parses_trans2c_stdout() {
        let cli = Cli::try_parse_from(["bf", "trans2c", "hello.bf", "-o", "-"]).unwrap();

        let Commands::Trans2c { output, .. } = cli.command else {
            panic!("expected trans2c command");
        };
        assert_eq!(output, Some(PathBuf::from("-")));
    }

    #[test]
    fn derives_native_output_from_source_stem() {
        let output = native_output_path(Path::new("examples/hello.bf")).unwrap();
        assert_eq!(
            output,
            PathBuf::from(format!("hello{}", env::consts::EXE_SUFFIX))
        );
    }

    #[test]
    fn embedded_runtime_assets_match_workspace_copies() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_header = manifest_dir.join("../../include/bfni.h");
        let workspace_runtime = manifest_dir.join("../../runtime/host/bfni_host.c");

        // The published package is self-contained and has no workspace parent.
        // When testing in this repository, keep both public copies in sync.
        if workspace_header.exists() && workspace_runtime.exists() {
            assert_eq!(fs::read_to_string(workspace_header).unwrap(), BFNI_HEADER);
            assert_eq!(fs::read_to_string(workspace_runtime).unwrap(), HOST_RUNTIME);
        }
    }
}
