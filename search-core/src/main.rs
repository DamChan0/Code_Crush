// src/main.rs
mod command;
mod matcher;
mod repls;
mod search_dir;
mod searcher;
mod types;
mod tui;

#[tokio::main]
async fn main() {
    // TUI 모드로 실행
    if let Err(e) = tui::run_tui().await {
        eprintln!("에러: {}", e);
        std::process::exit(1);
    }
}
