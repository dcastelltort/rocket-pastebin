
use crossbeam;
use std::thread;
use std::time::{Duration};
use std::path::{Path};
use std::fs;
use std::io;

pub const DEFAULT_RETENTION_SECONDS : u64 = 3600 * 24 * 40;
pub const DEFAULT_CLEANUP_INTERVAL_SECONDS : u64 = 60;

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
                    match self.cleanup_bins() {
                        Ok(nb_deleted) => {
                            println!("{} pastebins removed", nb_deleted);
                        }
                        Err(s) => {
                            println!("cleanup failed because: {}", s);
                        }
                    }
                }
            });
        });
    }

    fn cleanup_bins(&self) -> io::Result<u32> {
        let dir = Path::new(&self.path_to_clean);
        let mut nb_removed = 0;
        if dir.is_dir() {
            for e in fs::read_dir(dir)? {
                let entry = e?;
                let path = entry.path();
                if path.is_file() {
                    let metadata = entry.metadata()?;
                    let atime = metadata.accessed()?;
                    //println!("file {:?} atime {:?}", path, atime.elapsed().unwrap());
                    if  atime.elapsed().unwrap() > self.retention{
                        let filename = path.to_str().unwrap_or("");
                        if filename.is_empty() == false {
                            println!("deleted file : {:?}", filename);
                            fs::remove_file(filename)?;
                            nb_removed += 1;
                        }
                    }
                }
            }
        }
        Ok(nb_removed)
    }
    pub fn stop(&self) {

    }
}