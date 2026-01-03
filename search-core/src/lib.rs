mod matcher;
mod search;
mod search_dir;
mod types;

pub use types::MatchInfo;
pub use types::SearchOptions;

pub use search::search_in_file;
pub use search_dir::search_dir;
