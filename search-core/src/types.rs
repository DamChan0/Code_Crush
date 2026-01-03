use std::fmt;
use std::path::PathBuf;

use colored::Colorize;

/// 검색 옵션을 담는 구조체
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// 대소문자 무시 여부
    pub case_insensitive: bool,
    /// 숨김 파일/디렉토리 포함 여부
    pub include_hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchInfo {
    pub path: PathBuf,
    pub line_number: usize, // 1-based
    pub column: usize,      // 1-based (char index)
    pub byte_offset: usize, // from file start
    pub matched_text: String,
    pub line_text: String,
}

impl MatchInfo {
    // format: "file_path:line_number:column"
    pub fn make_pattern_link(&self) -> String {
        format!(
            "{}:{}:{}",
            self.path.display(),
            self.line_number,
            self.column
        )
    }

    // format: "line_number:column"
    pub fn make_location(&self) -> String {
        format!("{}:{}", self.line_number, self.column)
    }

    // highlight the matched text in purple color
    pub fn highlighted_line(&self) -> String {
        self.line_text.replace(
            &self.matched_text,
            &self.matched_text.purple().bold().to_string(),
        )
    }
}

impl fmt::Display for MatchInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  '{}'", self.make_pattern_link(), self.matched_text)
    }
}
