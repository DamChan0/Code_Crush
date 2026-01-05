use std::path::PathBuf;
use std::process;

use clap::Parser;
use search_core::{SearchOptions, run_repl, search_dir, search_in_file};

/// 초고속 코드 검색 엔진
#[derive(Parser, Debug)]
#[command(name = "search-core")]
#[command(version = "0.1.0")]
#[command(about = "Fast code search engine with precise location info")]
struct Args {
    /// 검색할 패턴
    pattern: Option<String>,

    /// 검색할 경로 (파일 또는 디렉토리)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// 대소문자 무시
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    /// 숨김 파일/디렉토리 포함
    #[arg(long = "hidden")]
    include_hidden: bool,

    #[arg(long = "repl")]
    repl: bool,
}

fn main() {
    let args = Args::parse();

    if args.repl || args.pattern.is_none() {
        run_repl();
        return;
    }

    let options = SearchOptions {
        case_insensitive: args.ignore_case,
        include_hidden: args.include_hidden,
    };

    let result = if args.path.is_file() {
        search_in_file(&args.path, &args.pattern.unwrap(), &options)
            .map(|opt| opt.unwrap_or_default())
    } else if args.path.is_dir() {
        search_dir(&args.path, &args.pattern.unwrap(), &options)
    } else {
        eprintln!(
            "error: '{}' is not a valid file or directory",
            args.path.display()
        );
        process::exit(1);
    };

    match result {
        Ok(matches) => {
            for m in &matches {
                println!("{}:{}", m.make_pattern_link(), m.highlighted_line());
            }
            if matches.is_empty() {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
