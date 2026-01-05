use std::io::{self, Write};
use std::path::PathBuf;

use crate::command::Command;
use crate::{SearchOptions, search_dir, search_in_file};

pub fn run_repl() {
    println!("Search Core REPL v0.1.0");
    println!("사용법: <pattern> [path]");
    println!("명령어: help, quit\n");

    let options = SearchOptions::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("입력 읽기 실패");
            continue;
        }

        match Command::parse(&input) {
            Ok(Command::Search { pattern, path }) => {
                let rootpath = if path == "." {
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                } else {
                    PathBuf::from(&path)
                };
                let result = if rootpath.is_file() {
                    search_in_file(&rootpath, &pattern, &options).map(|opt| opt.unwrap_or_default())
                } else {
                    search_dir(&rootpath, &pattern, &options)
                };
                match result {
                    Ok(matches) => {
                        if matches.is_empty() {
                            println!("결과 없음\n");
                        } else {
                            for m in &matches {
                                println!("{}:{}", m.make_pattern_link(), m.highlighted_line());
                            }
                            println!("\n총 {} 개 발견\n", matches.len());
                        }
                    }
                    Err(e) => eprintln!("search error: {}\n", e),
                }
            }
            Ok(Command::Help) => {
                println!("사용법:");
                println!("  <pattern>        현재 디렉토리에서 검색");
                println!("  <pattern> <path> 지정 경로에서 검색");
                println!("  help, h          도움말");
                println!("  quit, q, exit    종료\n");
            }
            Ok(Command::Quit) => {
                println!("Bye!");
                break;
            }
            Ok(Command::Invalid(msg)) => eprintln!("{}\n", msg),
            Err(e) => eprintln!("parse error: {}\n", e),
        }
    }
}
