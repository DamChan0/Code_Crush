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
    pub match_len: usize,           // 매치된 텍스트 바이트 길이
    pub matched_text: Arc<str>,  // Arc<str>으로 변경
    pub line_range: (usize, usize), // (줄 시작 바이트, 줄 끝 바이트)
}

impl SearchOptions {
    pub fn new(case_insensitive: bool, include_hidden: bool) -> Self {
        Self {
            case_insensitive,
            include_hidden,
        }
    }

    pub fn with_case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = value;
        self
    }

    pub fn with_include_hidden(mut self, value: bool) -> Self {
        self.include_hidden = value;
        self
    }

    pub fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }

    pub fn include_hidden(&self) -> bool {
        self.include_hidden
    }
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

    pub fn get_line_text_raw(&self) -> anyhow::Result<String> {
        let file = File::open(&self.path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let line_bytes = &mmap[self.line_range.0..self.line_range.1];
        Ok(String::from_utf8_lossy(line_bytes).to_string())
    }

    pub fn get_line_text(&self) -> anyhow::Result<String> {
        Ok(self.get_line_text_raw()?.trim_end().to_string())
    }

    /// Returns the line text with the matched portion highlighted in purple and bold.
    ///
    /// If `match_start` or `match_end` is out of range, returns the line text without highlighting.
    /// This avoids panic and returns the line text as is.
    pub fn highlighted_line(&self) -> anyhow::Result<String> {
        let file = File::open(&self.path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let line_bytes = &mmap[self.line_range.0..self.line_range.1];
        let match_start = self.byte_offset.saturating_sub(self.line_range.0);
        let match_end = match_start.saturating_add(self.match_len);

        // If match_start or match_end is out of range, return the line text
        // This avoids panic and returns the line text as is
        if match_start > line_bytes.len() || match_end > line_bytes.len() {
            return self.get_line_text();
        }

        let matched = String::from_utf8_lossy(&line_bytes[match_start..match_end]);
        let highlighted = matched.purple().bold().to_string();
        let before = String::from_utf8_lossy(&line_bytes[..match_start]);
        let after = String::from_utf8_lossy(&line_bytes[match_end..]);
        Ok(format!("{}{}{}", before, highlighted, after)
            .trim_end()
            .to_string())
    }
}

impl fmt::Display for MatchInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}", self.make_pattern_link(), self.matched_text)
    }
}
