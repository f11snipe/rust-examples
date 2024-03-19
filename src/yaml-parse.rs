use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigData {
    fs: HashMap<String, Vec<String>>,
    ps: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigGroup {
    watch: Option<Vec<String>>,
    blacklist: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigDataTest {
    fs: ConfigGroup,
    ps: ConfigGroup,
}

fn main() -> Result<(), serde_yaml::Error> {
    let file = File::open("conf/config.yaml").expect("Missing config.yaml file");
    let buf_reader = BufReader::new(file);

    let testing: ConfigDataTest = serde_yaml::from_reader(buf_reader)?;

    dbg!(&testing);

    match testing.fs.watch {
        Some(lst) => println!("FS Watch: {:?}", lst),
        None => println!("FS Watch: NOTHING"),
    }

    // match testing.fs.blacklist {
    //     Some(lst) => println!("FS Blacklist: {:?}", lst),
    //     None => println!("FS Blacklist: NOTHING"),
    // }

    match testing.ps.watch {
        Some(lst) => println!("PS Watch: {:?}", lst),
        None => println!("PS Watch: NOTHING"),
    }

    match testing.ps.blacklist {
        Some(lst) => println!("PS Watch: {:?}", lst),
        None => println!("PS Blacklist: NOTHING"),
    }

    let fsbl = &testing.fs.blacklist.unwrap_or_else(|| Vec::new());

    for f in fsbl {
        println!("FS BL: {}", f);
    }

    for f in fsbl {
        println!("FS BL: {}", f);
    }

// borrow of partially moved value: `testing.fs.watch`
// partial move occurs because value has type `Vec<std::string::String>`, which does not implement the `Copy` trait
    // if let Some(lst) = &testing.fs.watch {
    //     println!("FS watch enabled!");
    //     for f in lst {
    //         println!("FS WATCH: {}", f);
    //     }
    // }

    let point = Point { x: 1.0, y: 2.0 };

    let yaml = serde_yaml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    let deserialized_point: Point = serde_yaml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);
    Ok(())
}
