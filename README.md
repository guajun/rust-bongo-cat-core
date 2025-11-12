# Bongo Cat Core
Rust 实现的 Bongo Cat 跨平台键鼠操作捕获库
# Get Started
## 编译
```bash
cargo build --release
```
## 运行
### 使用 rdev 模式（Win, Mac, Linux x11）
```bash
cargo run -- rdev
```

### 使用 evdev 模式（仅 Linux Wayland）
假设键盘设备为 `/dev/input/event3`，鼠标设备为 `/dev/input/event4`
```bash
sudo ./target/release/bongo-cat-core -- evdev --keyboard /dev/input/event3 --mouse /dev/input/event4
```