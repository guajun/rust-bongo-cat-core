use rdev::{listen, Event, EventType};
use evdev::{Device, InputEventKind};
use glob::glob;
use std::io::{self, Write};
use serde::Serialize;
use std::sync::mpsc;
use std::thread;
use tokio_stream::StreamExt;

// 定义一个我们自己的事件结构体，这样更容易通过 JSON 传递
#[derive(Serialize)]
struct BongoEvent {
    event_type: String,
    // 'key' 字段可以是按键名，也可以是鼠标按钮名
    key: String,
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

// evdev 事件处理函数
async fn handle_evdev_device(device_path: &str, tx: &mpsc::Sender<BongoEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::open(device_path)?;
    println!("Listening for keyboard events on '{}' with evdev. Press Ctrl+C to exit.", device.name().unwrap_or("Unknown Device"));
    
    let mut stream = device.into_event_stream()?;

    while let Some(Ok(event)) = stream.next().await {
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

    Ok(())
}

// 扫描并列出可用的输入设备
fn scan_input_devices() -> Result<Vec<(String, std::path::PathBuf)>, Box<dyn std::error::Error>> {
    let mut devices = Vec::new();
    for entry in glob("/dev/input/event*")? {
        let path = entry?;
        if let Ok(device) = Device::open(&path) {
            if let Some(name) = device.name() {
                println!("  -> Found: '{}' at {}", name, path.display());
                devices.push((name.to_string(), path));
            }
        }
    }
    Ok(devices)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- TTY Bongo Cat Core ---");
    println!("Choose input method:");
    println!("1. Use rdev (cross-platform, works on most systems)");
    println!("2. Use evdev (Linux only, more direct device access)");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();
    
    let (tx, rx) = mpsc::channel::<BongoEvent>();
    
    match choice {
        "1" => {
            println!("Using rdev for input detection...");
            
            // 在新线程中运行 rdev 监听
            let tx_clone = tx.clone();
            thread::spawn(move || {
                if let Err(error) = listen(move |event| rdev_callback(event, &tx_clone)) {
                    eprintln!("Error with rdev: {:?}", error);
                }
            });
        }
        "2" => {
            println!("Scanning for input devices...");
            let devices = scan_input_devices()?;
            
            if devices.is_empty() {
                eprintln!("No input devices found. Do you have the correct permissions?");
                eprintln!("Try running: sudo usermod -aG input $USER (and then log out and back in)");
                return Ok(());
            }

            println!("\nPlease enter the full path of the device you want to listen to (e.g., /dev/input/event3):");
            let mut input_path = String::new();
            io::stdin().read_line(&mut input_path)?;
            let device_path = input_path.trim().to_string();
            
            // 在新任务中运行 evdev 监听
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_evdev_device(&device_path, &tx_clone).await {
                    eprintln!("Error with evdev: {}", e);
                }
            });
        }
        _ => {
            println!("Invalid choice. Using rdev as default.");
            
            // 在新线程中运行 rdev 监听
            let tx_clone = tx.clone();
            thread::spawn(move || {
                if let Err(error) = listen(move |event| rdev_callback(event, &tx_clone)) {
                    eprintln!("Error with rdev: {:?}", error);
                }
            });
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