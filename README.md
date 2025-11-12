# Bongo Cat Core

Rust 实现的 Bongo Cat 跨平台键鼠操作捕获库

## 特性

- 🖥️ **跨平台支持**: Windows、macOS、Linux
- 🎯 **多种输入模式**: 支持 rdev（跨平台）和 evdev（Linux 专用）
- 🚀 **一键构建**: 支持为所有常见平台构建二进制文件
- 📱 **平台特定优化**: Linux 系统保留完整命令行选项，其他系统简化使用

## 快速开始

### 编译

```bash
# 编译当前平台
cargo build --release
```

### 使用方法

#### Linux 系统

Linux 系统支持两种模式，可通过命令行参数选择：

**使用 rdev 模式（推荐）**：
```bash
./target/release/bongo-cat-core rdev
```

**使用 evdev 模式**：
```bash
./target/release/bongo-cat-core evdev --keyboard /dev/input/event3 --mouse /dev/input/event4
```

#### 查找设备路径

使用以下命令查找键盘和鼠标的设备路径：

**方法一：使用 evtest（推荐）**
```bash
# 安装 evtest
sudo apt install evtest  # Ubuntu/Debian
sudo yum install evtest  # CentOS/RHEL

# 列出所有输入设备
sudo evtest

# 或者直接查看设备列表
sudo evtest --list
```

**方法二：使用 /proc/bus/input/devices**
```bash
cat /proc/bus/input/devices
```

**方法三：使用 lsinput**
```bash
# 安装 input-utils
sudo apt install input-utils  # Ubuntu/Debian

# 列出所有输入设备
lsinput
```

**示例输出和识别方法：**

```
# evtest 输出示例
/dev/input/event0:      Power Button
/dev/input/event1:      Power Button
/dev/input/event2:      Sleep Button
/dev/input/event3:      AT Translated Set 2 keyboard         # ← 这是键盘
/dev/input/event4:      SynPS/2 Synaptics TouchPad           # ← 这是触摸板
/dev/input/event5:      Logitech USB Optical Mouse           # ← 这是鼠标
```

根据输出，你可以这样使用：
```bash
./target/release/bongo-cat-core evdev --keyboard /dev/input/event3 --mouse /dev/input/event5
```

#### Windows 和 macOS 系统

非 Linux 系统直接运行，无需命令行参数：

```bash
# Windows
./target/release/bongo-cat-core.exe

# macOS
./target/release/bongo-cat-core
```

## 多平台构建

### 推荐方案：本地构建 + 手动发布

由于交叉编译的复杂性，推荐使用以下简单方案：

#### 1. 本地构建当前平台
```bash
# 构建当前平台（快速且可靠）
cargo build --release
```

#### 2. 在对应系统上构建其他平台
- **Windows**: 在 Windows 机器上运行 `cargo build --release`
- **macOS**: 在 macOS 机器上运行 `cargo build --release`
- **Linux ARM64**: 在 ARM64 Linux 机器上运行 `cargo build --release`

#### 3. 使用 Docker（可选）
```bash
# 使用官方 Rust 镜像
docker run --rm -v $(pwd):/app -w /app rust:latest cargo build --release
```

### GitHub Actions：代码质量检查

项目配置了 GitHub Actions 专注于代码质量检查：

- ✅ 代码格式检查（`cargo fmt`）
- ✅ 代码质量检查（`cargo clippy`）
- ✅ 构建测试
- ✅ 单元测试
- ✅ 跨平台兼容性验证

推送代码时会自动运行这些检查，确保代码质量。

### 验证构建产物

#### GitHub Actions 构建产物验证

1. **查看构建状态**：
   - 访问 GitHub 仓库的 "Actions" 页面
   - 查看 "Build and Release" 工作流的运行状态

2. **下载测试版本**：
   - 在 Actions 页面找到最新的构建
   - 下载对应平台的 artifact 进行测试

3. **发布版本验证**：
   - 创建标签后，在 "Releases" 页面下载二进制文件
   - 测试下载的二进制文件是否正常工作

#### 本地快速验证

```bash
# 构建并测试当前平台
cargo build --release

# 测试程序是否能正常启动（非 Linux 系统）
./target/release/bongo-cat-core &
# 按 Ctrl+C 退出测试

# Linux 系统测试两种模式
./target/release/bongo-cat-core rdev &
# 按 Ctrl+C 退出测试

# 测试 evdev 模式参数解析
./target/release/bongo-cat-core evdev --help
```

## 输入模式说明

### rdev 模式
- **适用平台**: Windows、macOS、Linux
- **特点**: 跨平台兼容，无需特殊权限
- **推荐场景**: 大多数使用场景，特别是非 Linux 系统

### evdev 模式
- **适用平台**: 仅 Linux
- **特点**: 直接访问设备，性能更好
- **推荐场景**: Linux 系统，特别是 Wayland 环境
- **权限要求**: 需要 root 权限或将用户添加到 `input` 组

## 权限设置

### Linux evdev 模式权限设置

```bash
# 将当前用户添加到 input 组
sudo usermod -aG input $USER

# 重新登录或重启后生效
```

或者使用 sudo 运行：
```bash
sudo ./target/release/bongo-cat-core evdev --keyboard /dev/input/event3 --mouse /dev/input/event4
```

## 交叉编译工具链安装

### 手动安装

如果自动安装失败，可以手动安装：

#### Ubuntu/Debian
```bash
# 安装交叉编译工具链
sudo apt update
sudo apt install gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu
sudo apt install gcc-mingw-w64-x86-64

# 安装 Rust 目标平台
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### CentOS/RHEL/Fedora
```bash
# 安装交叉编译工具链
sudo yum install gcc-x86_64-linux-gnu gcc-aarch64-linux-gnu
sudo yum install gcc-mingw64-x86_64

# 安装 Rust 目标平台
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### Arch Linux
```bash
# 安装交叉编译工具链
sudo pacman -S gcc-multilib
sudo pacman -S mingw-w64-gcc

# 安装 Rust 目标平台
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### macOS
```bash
# 安装 Rust 目标平台
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# 对于 Linux 目标，需要安装交叉编译工具
brew install x86_64-linux-gnu-gcc aarch64-linux-gnu-gcc
```

### 常见问题解决

#### 1. pkg-config 交叉编译错误
如果遇到 pkg-config 相关错误，可以设置环境变量：

```bash
export PKG_CONFIG_ALLOW_CROSS=1
```

#### 2. macOS 构建失败
macOS 构建需要 Xcode 命令行工具：

```bash
xcode-select --install
```

#### 3. Windows 构建失败
确保安装了 MinGW-w64 工具链。

#### 4. 权限问题
如果遇到权限问题，可能需要使用 sudo 运行构建脚本，或者确保用户在正确的组中。

### 替代方案：使用 Docker

如果本地交叉编译遇到问题，可以使用 Docker 进行构建：

```bash
# 使用官方 Rust 镜像构建 Linux 目标
docker run --rm -v $(pwd):/app -w /app rust:latest cargo build --release

# 对于其他平台，可以使用专门的交叉编译镜像
# 如：ghcr.io/cross-rs/cross:latest
```

更多详细信息请参考：[Rust 交叉编译指南](https://rust-lang.github.io/rustup/cross-compilation.html)

## 输出格式

程序以 JSON 格式输出键鼠事件：

```json
{"event_type":"key_down","key":"KeyA"}
{"event_type":"mouse_down","key":"Left"}
```

## 开发

### 项目结构
```
bongo-cat-core/
├── .github/workflows/
│   ├── build.yml      # 多平台构建和发布
│   └── test.yml       # 代码测试
├── src/
│   └── main.rs       # 主程序入口
├── .gitignore        # Git 忽略规则
├── Cargo.toml        # 项目配置和依赖
└── README.md         # 项目文档
```

### 依赖项
- `rdev`: 跨平台输入事件捕获
- `evdev`: Linux 专用设备访问
- `tokio`: 异步运行时
- `clap`: 命令行参数解析（仅 Linux）
- `serde`: JSON 序列化