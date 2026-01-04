use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(format!("결과 {}", i)).unwrap();
            thread::sleep(Duration::from_millis(500)); // 검색 시뮬레이션
        }
        // tx가 drop되면 rx.recv()는 Err 반환
    });

    // 메시지가 올 때마다 처리
    while let Ok(msg) = rx.recv() {
        println!("수신: {}", msg);
    }
    println!("스트리밍 완료");
}
