// src/repl.rs

// [학습 1] 입출력 관련 라이브러리
// std::io: 사용자 입력을 받고 화면에 출력하기 위해 필요
// Write trait: 'flush()'를 쓰기 위해 필요 (프롬프트 '> '를 즉시 보여주기 위함)
use std::io::{self, Write};
use std::path::PathBuf;

// [학습 2] 비동기 스트림 처리
// StreamExt trait: 스트림의 .next() 메소드를 쓰기 위해 필수적입니다.
use futures::StreamExt;

use crate::command::Command;
// [학습 3] 모듈 가져오기
// streamer: 검색을 총괄하는 지휘자
// types: 데이터 구조체
use crate::{search_dir::search_stream, types::SearchOptions};

/// REPL을 실행합니다.
/// [학습 4] async fn
/// 이 함수 내부에서 .await를 사용해야 하므로 비동기 함수로 선언합니다.
/// main.rs에서 이 함수를 호출할 때도 .await를 붙여야 합니다.
pub async fn run_repl() {
    println!("Code_Crush v0.1.0");
    println!("사용법: <pattern> [path]");
    println!("명령어: help, quit\n");

    // 기본 검색 옵션 생성 (대소문자 구분 X, 숨김 파일 X 등)
    let options = SearchOptions::default();

    loop {
        // 1. 프롬프트 출력
        print!("> ");
        // [학습 5] flush()
        // Rust의 println!은 버퍼링됩니다. 줄바꿈(\n)이 없으면 화면에 바로 안 나올 수 있는데,
        // flush()를 호출하면 버퍼를 비워서 '> '가 즉시 화면에 뜨게 강제합니다.
        io::stdout().flush().unwrap();

        // 2. 사용자 입력 받기
        let mut input = String::new();
        // stdin().read_line(): 사용자가 엔터를 칠 때까지 기다립니다 (Blocking).
        // 비동기 환경에서는 tokio::io::stdin()을 쓰는 게 더 좋지만,
        // REPL 같은 간단한 입력 대기는 std::io를 써도 큰 문제는 없습니다.
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("입력 읽기 실패");
            continue;
        }

        // 3. 명령어 파싱 및 실행
        match Command::parse(&input) {
            Ok(Command::Search { pattern, path }) => {
                // 경로 결정: 입력이 없으면 현재 디렉토리(".")
                let rootpath = if path == "." {
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                } else {
                    PathBuf::from(&path)
                };

                println!("🔍 검색 시작: '{}' in {:?}", pattern, rootpath);

                // [학습 6] 스트림 생성
                // 이 함수는 호출 즉시 리턴되며, 백그라운드 스레드들이 돌기 시작합니다.
                let mut stream = search_stream(&rootpath, &pattern, options.clone());

                // [학습 7] 스트림 소비 (Real-time Output)
                // stream.next().await:
                // - 채널에 데이터가 올 때까지 기다립니다 (Non-blocking wait).
                // - 데이터가 오면 Some(Result), 채널이 닫히면(검색 끝) None을 반환합니다.
                let mut count = 0;
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(info) => {
                            count += 1;
                            // MatchInfo의 Display 트레이트 구현 덕분에 바로 출력 가능
                            println!("{}", info);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                println!("✅ 완료: 총 {}건 발견\n", count);
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
