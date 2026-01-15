mod command;
mod matcher;
mod repls;
mod search_dir;
mod searcher;
mod types;

pub use types::MatchInfo;
pub use types::SearchOptions;

pub use repls::run_repl;
pub use search_dir::search_stream;
pub use searcher::searcher;
