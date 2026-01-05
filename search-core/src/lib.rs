mod command;
mod matcher;
mod repls;
mod search;
mod search_dir;
mod types;

pub use types::MatchInfo;
pub use types::SearchOptions;

pub use repls::run_repl;
pub use search::search_in_file;
pub use search_dir::search_dir;
