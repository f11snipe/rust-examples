use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use rust_examples::{self, watch};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigGroup {
    watch: Option<Vec<String>>,
    blacklist: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigData {
    fs: ConfigGroup,
    ps: ConfigGroup,
}

fn main() -> Result<(), serde_yaml::Error> {
    let file = File::open("conf/config.yaml").expect("Missing config.yaml file");
    let buf_reader = BufReader::new(file);

    let testing: ConfigData = serde_yaml::from_reader(buf_reader)?;

    dbg!(&testing);

    let fs_watch = &testing.fs.watch.unwrap_or_else(|| Vec::new());
    let fs_blacklist = &testing.fs.blacklist.unwrap_or_else(|| Vec::new());

    let ps_watch = &testing.ps.watch.unwrap_or_else(|| Vec::new());
    let ps_blacklist = &testing.ps.blacklist.unwrap_or_else(|| Vec::new());

    for f in fs_watch {
        println!("FS Watch: {}", f);
    }
    for f in fs_blacklist {
        println!("FS Blacklist: {}", f);
    }

    for p in ps_watch {
        println!("PS Watch: {}", p);
    }
    for p in ps_blacklist {
        println!("PS Blacklist: {}", p);
    }

    for d in fs_watch {
        if let Err(e) = rust_examples::walk(d, fs_blacklist) {
            println!("error: {:?}", e);
        }
    }

    if let Err(e) = rust_examples::watch(".", fs_blacklist) {
        println!("error: {:?}", e);
    }

    if let Err(e) = watch(".", fs_blacklist) {
        println!("error: {:?}", e);
    }

    Ok(())
}
