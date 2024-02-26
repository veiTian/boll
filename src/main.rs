use crossterm::event::{self, Event, KeyCode};
use std::{
    io::{self, Write},
    process,
    sync::mpsc,
    thread,
    time::Duration,
};

fn main() {
    let mut boll = Boll::new(1, 2);
    let mut map = vec![vec!["O"; 10]; 10];

    // 创建用于渲染视图的通道
    let (tx, rx) = mpsc::channel();

    // 创建用于监听按键事件的线程
    let event_thread = thread::spawn(move || {
        loop {
            // 监听终端事件
            if event::poll(Duration::from_millis(1000)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    // 将按键事件发送到渲染视图的线程
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    // 渲染视图的线程
    let render_thread = thread::spawn(move || {
        // 清屏
        print!("\x1B[2J\x1B[1;1H");
        // 立即刷新标准输出
        let _ = io::stdout().flush();

        loop {
            let Boll { x, y } = boll;

            map[y][x] = "●";

            for v in &map {
                println!("");
                for s in v {
                    print!("{} ", s);
                }
            }
            println!("");

            map[y][x] = "O";

            std::thread::sleep(Duration::from_millis(10));

            // 接收从事件监听线程发送过来的按键事件
            if let Ok(key_event) = rx.recv() {
                // 检测按下的键并处理
                match key_event.code {
                    KeyCode::Up if boll.y > 0 => boll.y -= 1,
                    KeyCode::Down if boll.y < 9 => boll.y += 1,
                    KeyCode::Left if boll.x > 0 => boll.x -= 1,
                    KeyCode::Right if boll.x < 9 => boll.x += 1,
                    KeyCode::Esc | KeyCode::Char('q') => process::exit(0), // 退出游戏
                    _ => (),
                }
            }

            // 清屏
            print!("\x1B[2J\x1B[1;1H");
        }
    });

    // 等待两个线程结束
    event_thread.join().unwrap();
    render_thread.join().unwrap();

    println!("\n");
}

#[derive(Debug)]
struct Boll {
    x: usize,
    y: usize,
}

impl Boll {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
