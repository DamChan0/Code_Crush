use memmap2::Mmap;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
// 'colored' 크레이트의 Colorize 트레이트를 가져와야 문자열에 .purple(), .bold() 등을 쓸 수 있습니다.
use colored::Colorize;
use std::sync::Arc; // Arc를 가져옵니다.

/// 검색 옵션을 담는 구조체
///
/// #[derive(...)]: 컴파일러가 기본적인 기능(Trait)을 자동으로 구현해줍니다.
/// - Debug: println!("{:?}", options)로 개발자가 내용을 볼 수 있게 함
/// - Clone: 옵션을 여러 스레드(워커)에 복사해서 나눠주기 위해 필요
/// - Default: SearchOptions::default()로 기본값을 쉽게 생성하기 위해 필요
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// 대소문자 무시 여부 (true면 'a'와 'A'를 같게 취급)
    pub case_insensitive: bool,
    /// 숨김 파일(.git, .env 등) 포함 여부
    pub include_hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchInfo {
    pub path: PathBuf,              // 파일 경로
    pub line_number: usize,         // 줄 번호 (1-based)
    pub column: usize,              // 컬럼 번호 (1-based, 글자 기준)
    pub byte_offset: usize,         // 매치된 패턴의 시작 바이트 위치
    pub matched_text: Arc<String>,  // Arc<String>으로 변경
    pub line_range: (usize, usize), // (줄 시작 바이트, 줄 끝 바이트)
}

impl MatchInfo {
    pub fn make_pattern_link(&self) -> String {
        format!(
            "{}:{}:{}",
            self.path.display(),
            self.line_number,
            self.column
        )
    }

    pub fn make_location(&self) -> String {
        format!("{}:{}", self.line_number, self.column)
    }

    pub fn get_line_text(&self) -> anyhow::Result<String> {
        let file = File::open(&self.path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let line_bytes = &mmap[self.line_range.0..self.line_range.1];
        Ok(String::from_utf8_lossy(line_bytes).trim_end().to_string())
    }

    pub fn highlighted_line(&self) -> anyhow::Result<String> {
        let line_text = self.get_line_text()?;
        let highlighted = self.matched_text.purple().bold().to_string();
        Ok(line_text.replace(&self.matched_text.to_string().as_str(), &highlighted))
    }
}

impl fmt::Display for MatchInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  '{}'", self.make_pattern_link(), self.matched_text) // matched_text 사용
    }
}
