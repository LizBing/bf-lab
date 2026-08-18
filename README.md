# bf-lab

一个使用 Rust 实现的 Brainfuck 工具链实验。目前项目可以把 BF 源码解析、优化并翻译为 freestanding C，也可以借助内置的 host runtime 直接运行 BF 程序或构建本机可执行文件。

项目的长期目标是探索以 BF 编写系统逻辑，并通过 BFNI（Brainfuck Native Interface）连接宿主机或裸机运行时。

## 功能

- 完整支持经典 BF 指令：`+ - < > [ ] , .`
- Lexer、Parser、IR 和 C codegen 分层实现
- 8 位 cell，加减按模 256 回绕
- 可选 tape 边界检查，默认开启
- 确定性的多函数 C translation unit 输出
- BFNI C ABI
- 使用标准输入输出的 host runtime
- `bf run`：临时构建并运行 BF 程序
- `bf build`：构建本机可执行文件
- `bf trans2c`：生成 freestanding C

## 环境要求

- 支持 Rust 2024 edition 的稳定版 Rust 工具链
- `bf run` 和 `bf build` 需要一个兼容 GCC/Clang 命令行参数的 C11 编译器

CLI 按以下顺序选择 C 编译器：

1. `--cc <COMMAND>`
2. 环境变量 `CC`
3. `cc`

`bf trans2c` 只生成 C，不需要本机安装 C 编译器。

## 安装

在仓库根目录运行：

```console
cargo install --path crates/bf-cli
```

安装后的命令名是 `bf`。默认安装位置为 `~/.cargo/bin/bf`，请确保 `~/.cargo/bin` 已加入 `PATH`。

更新本地安装：

```console
cargo install --path crates/bf-cli --force
```

卸载：

```console
cargo uninstall bf-cli
```

不安装也可以从 workspace 直接运行：

```console
cargo run -p bf-cli --bin bf -- run examples/hello.bf
```

## 快速开始

运行示例：

```console
bf run examples/hello.bf
```

输出：

```text
Hello, BrainFuck!
```

指定 tape 长度：

```console
bf run examples/hello.bf --tape-len 65536
```

### 构建本机可执行文件

```console
bf build examples/hello.bf
./hello
```

默认输出文件名取自 BF 源文件的 stem。也可以显式指定：

```console
bf build examples/hello.bf -o hello-bf
./hello-bf
```

构建后的 host 程序接受一个可选的 tape 长度参数：

```console
./hello-bf 65536
```

### 翻译为 freestanding C

```console
bf trans2c examples/hello.bf
```

默认在源文件旁生成 `examples/hello.c`。指定输出路径：

```console
bf trans2c examples/hello.bf -o generated.c
```

输出到 stdout：

```console
bf trans2c examples/hello.bf -o -
```

生成的 C 通过以下方式引用 BFNI：

```c
#include <bfni.h>
```

它是 freestanding BF 函数，而不是一个自带 `main` 的可执行程序。编译 translation unit 时需要提供 [`include/bfni.h`](include/bfni.h) 和一个实现 BFNI 的运行时。例如只检查生成代码：

```console
cc \
  -std=c11 \
  -ffreestanding \
  -Iinclude \
  -c generated.c \
  -o generated.o
```

## 编译选项

三个子命令都支持 BF IR 优化级别：

```console
bf run hello.bf -O0
bf run hello.bf -O1
```

默认使用 `-O1`。当前支持：

- `-O0`：不执行 IR 优化
- `-O1`：合并连续的 cell 加减和 tape 移动等局部优化

默认生成 tape 边界检查。可以显式进入 unsafe 模式：

```console
bf run hello.bf --unsafe
```

unsafe 模式要求 BF 程序自行保证数据指针始终位于 tape 内；越界行为未定义。

## BF 机器语义

- Cell 为无符号 8 位整数
- `+` 和 `-` 按模 256 回绕
- 数据指针初始位于 tape offset 0
- Tape 由运行时提供，长度必须大于 0
- Tape 初始内容由具体运行时决定
- Host runtime 使用 `calloc`，因此初始 tape 全为 0
- 输入 EOF 或 I/O 错误会填写 report，并让 BF 函数返回 `BF_FALSE`
- Checked 模式下，左右越界会填写 report，并让 BF 函数返回 `BF_FALSE`

完整 ABI 契约见 [`include/bfni.h`](include/bfni.h)。

## 架构

```text
BF source
    │
    ▼
bf-frontend    Lexer + Parser + AST
    │
    ▼
bf-ir          Lowering + IR optimization
    │
    ▼
bf-codegen     Freestanding C generation
    │
    ├── trans2c ──> generated C + external BFNI runtime
    │
    └── run/build ──> embedded host runtime ──> native executable
```

Workspace 结构：

```text
crates/
  bf-frontend/   BF lexer、parser 和 AST
  bf-ir/         IR、lowering 和 optimizer
  bf-codegen/    C codegen 和公开编译 API
  bf-cli/        `bf` 命令行工具
include/
  bfni.h         公共 BFNI ABI
runtime/
  host/          基于 stdio 和动态 tape 的 host runtime
examples/
  hello.bf       Hello 示例
```

`bf run` 和 `bf build` 会把当前 BFNI header 与 host runtime 内嵌进 `bf` 二进制，因此安装后不依赖仓库路径；修改 ABI 或 host runtime 后需要重新构建或安装 CLI。

## 开发

运行完整测试：

```console
cargo test --workspace
```

运行 CLI 的严格 Clippy 检查：

```console
cargo clippy -p bf-cli --bin bf --no-deps -- -D warnings
```

查看命令帮助：

```console
bf --help
bf run --help
bf build --help
bf trans2c --help
```

## 当前状态

这是一个实验性项目，CLI、生成的 C ABI 和内部 IR 仍可能变化。当前 host runtime 面向本机验证；裸机启动、设备驱动、中断和调度尚未实现。

## License

Apache License 2.0。详见 [`LICENSE`](LICENSE)。
