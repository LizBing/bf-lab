use std::{
    fs,
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::ExitCode,
    str::FromStr,
};

use bf_codegen::{CodeFile, CompileOptions, compile_function};
use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionSpec {
    name: String,
    path: PathBuf,
}

impl FromStr for FunctionSpec {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (name, path) = raw
            .split_once('=')
            .ok_or_else(|| "expected NAME=FILE".to_owned())?;

        if name.is_empty() {
            return Err("function name cannot be empty".into());
        }

        if path.is_empty() {
            return Err("source path cannot be empty".into());
        }

        Ok(Self {
            name: name.into(),
            path: path.into(),
        })
    }
}

/// Compile Brainfuck sources into a C translation unit.
#[derive(Debug, Parser)]
#[command(name = "bfc", version, about)]
struct CLI {
    /// Output C file. Writes to standard output when omitted.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Add a BF source as NAME=FILE. May be specified more than once.
    #[arg(
        short = 'f',
        long = "function",
        value_name = "NAME=FILE",
        required = true
    )]
    functions: Vec<FunctionSpec>,

    /// Optimization level.
    #[arg(
        short = 'O',
        default_value_t = 1,
        value_parser = clap::value_parser!(u8).range(0..=1)
    )]
    opt_level: u8,

    /// Disable tape boundary checks.
    #[arg(long = "unsafe")]
    unsafe_mode: bool,
}

fn run(cli: CLI) -> Result<(), String> {
    let options = CompileOptions {
        opt_level: i32::from(cli.opt_level),
        boundary_check: !cli.unsafe_mode,
    };
    let mut code_file = CodeFile::new();

    for spec in cli.functions {
        let source = fs::read_to_string(&spec.path)
            .map_err(|error| format!("failed to read {}: {error}", spec.path.display()))?;
        let function = compile_function(&source, &spec.name, options).map_err(|error| {
            format!(
                "failed to compile {} as {}: {error}",
                spec.path.display(),
                spec.name
            )
        })?;

        code_file
            .add_bf_func(function)
            .map_err(|error| error.to_string())?;
    }

    match cli.output {
        Some(path) => {
            let file = File::create(&path)
                .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
            let mut writer = BufWriter::new(file);
            code_file
                .write_to(&mut writer)
                .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
            writer
                .flush()
                .map_err(|error| format!("failed to write {}: {error}", path.display()))
        }
        None => {
            let stdout = io::stdout();
            let mut writer = stdout.lock();
            code_file
                .write_to(&mut writer)
                .map_err(|error| format!("failed to write standard output: {error}"))?;
            writer
                .flush()
                .map_err(|error| format!("failed to write standard output: {error}"))
        }
    }
}

fn main() -> ExitCode {
    match run(CLI::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("bfc: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_functions_and_unsafe_mode() {
        let cli = CLI::try_parse_from([
            "bfc",
            "-f",
            "hello=hello.bf",
            "--function",
            "shell=shell.bf",
            "-O0",
            "--unsafe",
        ])
        .unwrap();

        assert_eq!(cli.functions.len(), 2);
        assert_eq!(cli.functions[0].name, "hello");
        assert_eq!(cli.opt_level, 0);
        assert!(cli.unsafe_mode);
    }

    #[test]
    fn rejects_malformed_function_specs() {
        assert!(CLI::try_parse_from(["bfc", "-f", "hello.bf"]).is_err());
    }
}
