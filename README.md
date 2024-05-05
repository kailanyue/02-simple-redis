# Geektime Rust 语言训练营

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持


```bash
cargo install cargo-generate
cargo install cargo-deny --locked
cargo install typos-cli
cargo install git-cliff
cargo install cargo-nextest --locked
cargo install cargo-update
cargo install tokei
```
### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

新的项目会使用 `kailanyue/template` 模版生成基本的代码：

```bash
cargo generate kailanyue/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

```bash
# 首次运行
pre-commit install

# 修改添加到暂存区
git add filename
git add .

# 运行提交检查
git commit -a
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny

# 重新生成配置文件
cargo deny init

# 检查
cargo deny check -d

# Windows 中换行符解决
git config --global --get core.autocrlf
git config --local --get core.autocrlf
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff

# 初始化 git-cliff
git-cliff init

# 生成 changelog
git-cliff -o CHANGELOG.md
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

### 安装 tokei
Tokei is a program that displays statistics about your code.

```bash
cargo install tokei

tokei .
```


### 安装 cargo-update
要更新使用 `cargo install` 安装的包，您可以使用以下方法：

1. **使用 cargo-update 工具**：
    - 首先，您需要安装 `cargo-update` 工具。如果您尚未安装它，可以运行以下命令：
      ```
      cargo install cargo-update
      ```
    - 然后，使用以下命令来更新所有已安装的包：
      ```
      cargo install-update -a
      ```
    - 这将检查您使用 `cargo install` 安装的所有内容，并将其更新为最新版本。

2. **手动更新**：
    - 如果您知道特定包的名称，您可以直接运行以下命令来更新它：
      ```
      cargo install --force 包名
      ```
      其中 `包名` 是您要更新的包的名称。
    - 如果您不确定已安装的包的名称，您可以运行以下命令来列出所有已安装的包：
      ```
      cargo install --list
      ```
      然后根据列表中的包名选择要更新的包。



### cargo 中 locked 参数的含义
`--locked` 是 `cargo install` 命令的一个选项，用于强制使用已打包的 `Cargo.lock` 文件。具体含义如下：

- 默认情况下，`cargo install` 会忽略包含在软件包中的 `Cargo.lock` 文件。这意味着 Cargo 将重新计算要使用的依赖项的版本，可能会使用自发布软件包后发布的较新版本。
- 使用 `--locked` 标志可以强制 Cargo 使用软件包中的 `Cargo.lock` 文件（如果可用）。这对于确保可重现的构建非常有用，以使用发布软件包时可用的完全相同的依赖项集。
- 如果您希望使用发布软件包时的确切依赖项版本，或者需要确保构建与特定版本的依赖项一致，可以使用 `--locked`。

请注意，使用 `--locked` 的缺点是您将不会收到任何依赖项的修复或更新。如果软件包中的 `Cargo.lock` 文件不可用，Cargo 将继续重新计算依赖项版本。
