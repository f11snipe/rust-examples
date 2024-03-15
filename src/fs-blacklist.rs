use std::fs;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // assert!(!fs::try_exists("does_not_exist.txt").expect("Can't check existence of file does_not_exist.txt"));
    // assert!(fs::try_exists("secret_file.txt").is_err());

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        if entry.path().display().to_string().contains("poopoo") {
            println!("POO: {}", entry.path().display());
            if entry.path().is_file() {
                let _ = fs::remove_file(&entry.path());
            }
        }
    }

    for res in rx {
        match res {
            Ok(event) => {
                // println!("{:?}", event.kind);
                if !event.kind.is_remove() {
                    // println!("changed: {:?}", event);
                    for pp in event.paths {
                        if pp.exists() {
                            match pp.to_str() {
                                Some(nn) => {
                                    if nn.contains("poopoo") {
                                        println!("NN={}", nn);
                                        if pp.is_file() {
                                            let _ = fs::remove_file(&pp);
                                        }
                                    }
                                },
                                None => println!("none..."),
                            }
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

