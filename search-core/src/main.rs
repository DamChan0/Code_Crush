use std::path::Path;

use search_core::{MatchInfo, search_dir, search_in_file};

fn main() {
    println!("=== Search Core 테스트 ===\n");

    // 1. 단일 파일 검색 테스트
    println!("📄 [1] 단일 파일 검색 테스트");
    println!("{}", "-".repeat(50));

    let test_file = Path::new("test_data/sample1.txt");
    let pattern = "hello";

    match search_in_file(test_file, pattern) {
        Ok(Some(matches)) => {
            print_matches(&matches, pattern);
        }
        Ok(None) => println!("매치 결과 없음"),
        Err(e) => println!("❌ 에러: {}", e),
    }

    // 2. 디렉토리 검색 테스트
    println!("\n📁 [2] 디렉토리 전체 검색 테스트");
    println!("{}", "-".repeat(50));

    let test_dir = Path::new("test_data");
    let pattern2 = "rust";

    match search_dir(test_dir, pattern2) {
        Ok(matches) => {
            print_matches(&matches, pattern2);
        }
        Err(e) => println!("❌ 에러: {}", e),
    }

    // 3. 한글 검색 테스트
    println!("\n🔤 [3] 한글 검색 테스트");
    println!("{}", "-".repeat(50));

    let pattern3 = "안녕";

    match search_dir(test_dir, pattern3) {
        Ok(matches) => {
            print_matches(&matches, pattern3);
        }
        Err(e) => println!("❌ 에러: {}", e),
    }

    // 4. 다중 매치 테스트
    println!("\n🔍 [4] 한 줄에 다중 매치 테스트");
    println!("{}", "-".repeat(50));

    let pattern4 = "test";

    match search_dir(test_dir, pattern4) {
        Ok(matches) => {
            print_matches(&matches, pattern4);
        }
        Err(e) => println!("❌ 에러: {}", e),
    }

    println!("\n=== 테스트 완료 ===");
}

fn print_matches(matches: &[MatchInfo], pattern: &str) {
    if matches.is_empty() {
        println!("  패턴 '{}' 에 대한 매치 결과 없음", pattern);
        return;
    }

    println!(
        "  패턴 '{}' 검색 결과: {} 개 발견\n",
        pattern,
        matches.len()
    );

    for (i, m) in matches.iter().enumerate() {
        println!("  [{}] {}", i + 1, m.make_pattern_link());
        println!("      내용: {}", m.highlighted_line());
        println!();
    }
}
