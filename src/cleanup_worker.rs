
use crossbeam;
use std::thread;
use std::time::{Duration, SystemTime};
use std::path::{Path};
use std::fs::{self, DirEntry};

pub const DEFAULT_RETENTION_SECONDS : u64 = 3600 * 24 * 40;

pub struct CleanupWorker {
    cleanup_interval_seconds : u64,
    path_to_clean : String,
    retention : Duration
}

impl CleanupWorker {
    pub fn new(cleanup_interval_seconds: u64, path_to_clean : String, retention : Duration) -> CleanupWorker {
        CleanupWorker {
            cleanup_interval_seconds: cleanup_interval_seconds, 
            path_to_clean: path_to_clean,
            retention : retention
        }
    }

    pub fn start(&self) {
        crossbeam::scope(|scoped| {
            scoped.spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(self.cleanup_interval_seconds));
                    println!("Running cleanup old pastebin loop");
                    let dir = Path::new(&self.path_to_clean);
                    if dir.is_dir() {
                        for e in fs::read_dir(dir).unwrap() {
                            let entry = e.unwrap();
                            let path = entry.path();
                            if path.is_file() {
                                let metadata = entry.metadata().unwrap();
                                let atime = metadata.accessed().unwrap();
                                if  atime.elapsed().unwrap() > self.retention{
                                    let filename = path.to_str().unwrap();
                                    println!("deleted file : {:?}", filename);
                                    fs::remove_file(filename).unwrap();
                                }
                            }
                        }
                    }
                }
            });
        });
    }

    pub fn stop(&self) {

    }
}