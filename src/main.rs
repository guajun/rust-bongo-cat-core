use rdev::{listen, Event, EventType};
use std::io::{self, Write};
use serde::Serialize;
use std::sync::mpsc;
use std::thread;
use clap::{Parser, Subcommand};

// Linux 特定的导入
#[cfg(target_os = "linux")]
use evdev::{Device, InputEventKind};
#[cfg(target_os = "linux")]
use tokio_stream::StreamExt;

// 定义一个我们自己的事件结构体，这样更容易通过 JSON 传递
#[derive(Serialize)]
struct BongoEvent {
    event_type: String,
    // 'key' 字段可以是按键名，也可以是鼠标按钮名
    key: String,
}

// 命令行参数结构
#[derive(Parser)]
#[command(name = "bongo-cat-core")]
#[command(about = "A core program for bongo cat that captures keyboard and mouse events")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Use rdev for Win, Mac, Linux (x11) input detection
    Rdev,
    /// Use evdev for Linux (Wayland) direct device access
    Evdev {
        /// Keyboard device path (e.g., /dev/input/event3)
        #[arg(short, long)]
        keyboard: String,
        /// Mouse device path (e.g., /dev/input/event4)
        #[arg(short, long)]
        mouse: String,
    },
}

// 这是 rdev 事件发生时的回调函数
fn rdev_callback(event: Event, tx: &mpsc::Sender<BongoEvent>) {
    // 将 rdev 的事件转换成我们自己的 BongoEvent
    let bongo_event = match event.event_type {
        EventType::KeyPress(key) => Some(BongoEvent {
            event_type: "key_down".to_string(),
            key: format!("{:?}", key), // 将按键枚举转换成字符串，例如 "KeyA", "Return"
        }),
        EventType::ButtonPress(button) => Some(BongoEvent {
            event_type: "mouse_down".to_string(),
            key: format!("{:?}", button), // 例如 "Left", "Right"
        }),
        // 我们暂时不关心按键释放、鼠标移动等事件，所以忽略它们
        _ => None,
    };

    // 如果是我们关心的事件类型，就把它通过通道发送出去
    if let Some(event) = bongo_event {
        let _ = tx.send(event);
    }
}

// evdev 事件处理函数 (仅 Linux)
#[cfg(target_os = "linux")]
async fn handle_evdev_device(device_path: &str, tx: &mpsc::Sender<BongoEvent>, device_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::open(device_path)?;
    println!("Listening for {} events on '{}' with evdev. Press Ctrl+C to exit.", device_type, device.name().unwrap_or("Unknown Device"));
    
    let mut stream = device.into_event_stream()?;

    while let Some(Ok(event)) = stream.next().await {
        match device_type {
            "keyboard" => {
                if let InputEventKind::Key(_) = event.kind() {
                    match event.value() {
                        1 => { // Press
                            let bongo_event = BongoEvent {
                                event_type: "key_down".to_string(),
                                key: format!("{:?}", event.code()),
                            };
                            let _ = tx.send(bongo_event);
                        }
                        0 => { // Release
                            // 可以在这里处理按键释放事件，如果需要的话
                        }
                        2 => { // Repeat (ignoring)
                        }
                        _ => {}
                    }
                }
            }
            "mouse" => {
                if let InputEventKind::Key(_) = event.kind() {
                    match event.value() {
                        1 => { // Press
                            let bongo_event = BongoEvent {
                                event_type: "mouse_down".to_string(),
                                key: format!("{:?}", event.code()),
                            };
                            let _ = tx.send(bongo_event);
                        }
                        0 => { // Release
                            // 可以在这里处理鼠标释放事件，如果需要的话
                        }
                        2 => { // Repeat (ignoring)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    println!("--- TTY Bongo Cat Core ---");
    
    let (tx, rx) = mpsc::channel::<BongoEvent>();
    
    match cli.command {
        Commands::Rdev => {
            println!("Using rdev for input detection...");
            start_rdev_listener(tx);
        }
        Commands::Evdev { keyboard, mouse } => {
            #[cfg(target_os = "linux")]
            {
                println!("Using evdev for input detection...");
                
                // 启动键盘监听
                let tx_clone = tx.clone();
                let keyboard_path = keyboard.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_evdev_device(&keyboard_path, &tx_clone, "keyboard").await {
                        eprintln!("Error with keyboard evdev: {}", e);
                    }
                });
                
                // 启动鼠标监听
                let tx_clone = tx.clone();
                let mouse_path = mouse.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_evdev_device(&mouse_path, &tx_clone, "mouse").await {
                        eprintln!("Error with mouse evdev: {}", e);
                    }
                });
            }
            
            #[cfg(not(target_os = "linux"))]
            {
                eprintln!("Error: evdev is only supported on Linux. Use 'rdev' command instead.");
                return Ok(());
            }
        }
    }
    
    println!("Bongo Cat Core started. Listening for events...");
    io::stdout().flush().unwrap();
    
    // 主线程负责接收事件并发送 JSON 输出
    while let Ok(event) = rx.recv() {
        if let Ok(json) = serde_json::to_string(&event) {
            // 打印到标准输出，这样 Electron 程序就能读到
            println!("{}", json);
            // 强制刷新输出缓冲区，确保 Electron 能立刻收到消息，这很重要！
            io::stdout().flush().unwrap();
        }
    }

    Ok(())
}

// 启动 rdev 监听器的辅助函数
fn start_rdev_listener(tx: mpsc::Sender<BongoEvent>) {
    // 在新线程中运行 rdev 监听
    let tx_clone = tx.clone();
    thread::spawn(move || {
        if let Err(error) = listen(move |event| rdev_callback(event, &tx_clone)) {
            eprintln!("Error with rdev: {:?}", error);
        }
    });
}