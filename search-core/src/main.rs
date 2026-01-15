// src/main.rs
mod command;
mod matcher;
mod repls;
mod search_dir;
mod searcher;
mod types;

#[tokio::main]
async fn main() {
    repls::run_repl().await;
}
