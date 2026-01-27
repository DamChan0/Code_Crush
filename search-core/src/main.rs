// src/main.rs
mod command;
mod matcher;
mod repls;
mod search_dir;
mod searcher;
mod types;

use crate::repls::run_repl;

#[tokio::main]
async fn main() {
    // TUI 모드로 실행
    run_repl().await;
}
