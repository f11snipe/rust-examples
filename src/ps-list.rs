use std::error::Error;
use regex::Regex;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("http://localhost:1234/tmp/ps")?.text()?;
    // println!("{:#?}", resp);
    let parts = resp.split('\n').into_iter().filter(|p| p.trim() != "");
    // let p = &mut parts;
    // for part in parts {
    //     println!("Part: {}", part)
    // }
    let coll1 = parts.clone().collect::<Vec<&str>>();
    dbg!(coll1);
    // let c = &mut coll1;
    // let mut coll2: Vec<&str> = parts.collect();
    // dbg!(coll2);

    let output = Command::new("ps")
                        .arg("aux")
                        .output()
                        .expect("failed to execute process");

    println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    let sst = String::from_utf8_lossy(&output.stdout);
    let pp2 = sst.split("\n").into_iter().filter(|p| p.trim() != "").map(|p| p.to_lowercase());
    // dbg!(pp2);

    // let outstr = format!("{:?}", output.stdout);
    // let lines = outstr.split("\n").into_iter().filter(|p| p.trim() != "").collect::<Vec<&str>>();

    // let re = Regex::new(r"^\s*([^\s]+)\s+([0-9]+)\s+").unwrap();
    let re2 = Regex::new(r"^\s*(?<user>[^\s]+)\s+(?<pid>[0-9]+)\s+").unwrap();

    for line in pp2 {
        // println!("Check: {}", line);
        let coll1 = parts.clone().map(|p: &str| p.to_lowercase()).collect::<Vec<String>>();
        // let keywords = &mut coll1;
        for kwd in coll1 {
            // println!("CONTAINS: {}", kwd);
            if line.contains(&kwd) {
                // println!("Found process {}", kwd);
                // println!("\t{}", line);

                let Some(caps) = re2.captures(&line) else {
                    println!("NO MATCH (cant find pid) {}", kwd);
                    println!("LINE: {}", line);
                    continue;
                };

                println!("FOUND: '{}' (pid={}, user={})", kwd, &caps["pid"], &caps["user"]);

                // let mut res = vec![];
                // for (_, [user, pid]) in re.captures_iter(line).map(|c| c.extract()) {
                //     println!("USER={} | PID={}", user, pid);
                //     res.push((user, pid));
                // }
                // dbg!(res);
            }
        }
    }

    Ok(())
}

