use std::fs;
// use confy::ConfyError;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use walkdir::WalkDir;
use configparser::ini::Ini;
// use walkdir::{DirEntry, WalkDir};
// use configparser::ini::{Ini, WriteOptions};


// fn is_hidden(entry: &DirEntry) -> bool {
//     entry.file_name()
//          .to_str()
//          .map(|s| s.starts_with("."))
//          .unwrap_or(false)
// }


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
    let mut config: Ini = Ini::new();

    // You can easily load a file to get a clone of the map:
    let map = config.load("conf/general.ini");
    println!("{:?}", map);
    // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

    // If you want to access the value, then you can simply do:
    let blacklist_files_str: String = config.get("fs", "blacklist_files").unwrap();
    let blacklist_files: Vec<&str> = blacklist_files_str.split(',').into_iter().filter(|p| p.trim() != "").map(|p| p.trim()).collect::<Vec<&str>>();

    println!("BLACKLIST: {:?}", blacklist_files);

    // let testing: String = config.get("indent", "testing").unwrap();
    // let multi: String = config.get("extra", "multi").unwrap();

    // let mlist: Vec<&str> = multi.split(',').into_iter().filter(|p| p.trim() != "").collect::<Vec<&str>>();

    // println!("TESTING: {:?}", testing);
    // println!("MULTI: {:?}", multi);
    // dbg!(mlist);

    let (tx, rx) = std::sync::mpsc::channel();

    // assert!(!fs::try_exists("does_not_exist.txt").expect("Can't check existence of file does_not_exist.txt"));
    // assert!(fs::try_exists("secret_file.txt").is_err());

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: notify::INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        for val in blacklist_files.clone() {
            if entry.path().display().to_string().contains(&val) {
                println!("POO: {}", entry.path().display());
                if entry.path().is_file() {
                    let _ = fs::remove_file(&entry.path());
                }
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
                                    for val in blacklist_files.clone() {
                                        if nn.contains(&val) {
                                            println!("NN={}", nn);
                                            if pp.is_file() {
                                                let _ = fs::remove_file(&pp);
                                            }
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

