// src/streamer.rs
use anyhow::Result;
use ignore::WalkBuilder;
use num_cpus;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::searcher;
use crate::types::{MatchInfo, SearchOptions};

/// search directory and return a stream of `MatchInfo`
pub fn search_stream(
    root: &Path,
    pattern: &str,
    options: SearchOptions,
) -> ReceiverStream<Result<MatchInfo>> {
    let (tx, rx) = mpsc::channel(1000);

    let root = root.to_path_buf();
    // using Arc for shared memory with other threads
    let pattern = Arc::new(pattern.to_string());
    let options = Arc::new(options);

    tokio::task::spawn_blocking(move || {
        let walker = WalkBuilder::new(&root)
            .hidden(options.include_hidden)
            .threads(num_cpus::get())
            // TODO: ? why use build_parallel?
            .build_parallel();

        walker.run(|| {
            // will run in each thread
            let tx = tx.clone();
            let pattern = pattern.clone();
            let options = options.clone();

            Box::new(move |entry_result| {
                let entry = match entry_result {
                    Ok(e) => e,
                    Err(err) => {
                        let _ = tx.blocking_send(Err(anyhow::Error::from(err)));
                        return ignore::WalkState::Continue;
                    }
                };

                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();

                    // search file and return matches
                    // TODO: why use Arc::clone?
                    match searcher::searcher(path, Arc::clone(&pattern), &options) {
                        Ok(matches) => {
                            for info in matches {
                                if tx.blocking_send(Ok(info)).is_err() {
                                    return ignore::WalkState::Quit;
                                }
                            }
                        }
                        Err(err) => {
                            let _ = tx.blocking_send(Err(err));
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });
    });

    ReceiverStream::new(rx)
}
