use rdev::{listen, Event, EventType};
use serde::Serialize;
use std::io::{self, Write};

// 定义一个我们自己的事件结构体，这样更容易通过 JSON 传递
#[derive(Serialize)]
struct BongoEvent {
    event_type: String,
    // 'key' 字段可以是按键名，也可以是鼠标按钮名
    key: String,
}

// 这是事件发生时的回调函数
// rdev 库会在捕获到任何事件时调用这个函数
fn callback(event: Event) {
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

    // 如果是我们关心的事件类型，就把它转换成 JSON 字符串并打印出来
    if let Some(event) = bongo_event {
        // Serde 会帮我们把 BongoEvent 结构体转换成 {"event_type":"key_down","key":"KeyA"} 这样的 JSON
        if let Ok(json) = serde_json::to_string(&event) {
            // 打印到标准输出，这样 Electron 程序就能读到
            println!("{}", json);
            // 强制刷新输出缓冲区，确保 Electron 能立刻收到消息，这很重要！
            io::stdout().flush().unwrap();
        }
    }
}

fn main() {
    println!("Bongo Cat Core started. Listening for events...");
    io::stdout().flush().unwrap();

    // listen 是一个阻塞函数，它会一直在这里运行，直到程序出错或被关闭
    // 它会把捕获到的所有事件都交给我们的 callback 函数处理
    if let Err(error) = listen(callback) {
        // 如果出错了，打印错误信息
        eprintln!("Error: {:?}", error)
    }
}