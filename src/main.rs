use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[0]);
    let config_file = &args[1];
    let data_path = &args[2];
    let config_info = fs::read_to_string(config_file)?;
    let mut save_path = PathBuf::from("./save");
    DirBuilder::new().recursive(true).create(save_path)?;
    //save_path=PathBuf::from("./save/gfwlist.txt");
    for line in config_info.lines() {
        let l: Vec<&str> = line.split(':').collect();
        if l.len() < 2 {
            continue;
        }
        let ll: Vec<&str> = l[0].split('|').collect();
        let lll: Vec<&str> = l[1].split(',').collect();

        save_path = PathBuf::from("./save");
        save_path.push(ll[0]);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(save_path)?;
        let mut writer = BufWriter::new(file);
        if lll.len() < 2 && lll[0].is_empty() {
            if ll.len() > 1 {
                writer.write_all(ll[1].as_bytes())?;
                writer.write_all(ll[0].as_bytes())?;
                writer.write_all(ll[2].as_bytes())?;
            } else {
                //println!("writing {}", d);
                writer.write_all(ll[0].as_bytes())?;
            }
            writer.write_all(b"\n")?;
            continue;
        }
        for a in lll {
            if a.is_empty() {
                continue;
            }
            println!("opening {}", a);
            let mut path = PathBuf::from(data_path);
            path.push(a);
            //let  white = get_white_list(ll[0]);
            match get_white_list(ll[0]) {
                Ok(whitelist) => {
                    for line in read_lines(path)? {
                        //let c = b?;
                        //println!("{}", c);
                        if let Some(d) = get_domain(&line?) {
                            //println!("get:{}", d);

                            if whitelist.contains(&d) {
                                println!("in white: {}", &d);
                                continue;
                            }

                            //println!("waiting write:{}", d);
                            if ll.len() > 1 {
                                writer.write_all(ll[1].as_bytes())?;
                                writer.write_all(d.as_bytes())?;
                                writer.write_all(ll[2].as_bytes())?;
                            } else {
                                //println!("writing {}", d);
                                writer.write_all(d.as_bytes())?;
                            }

                            writer.write_all(b"\n")?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get white list for file '{}': {}", ll[0], e);
                }
            }
        }
    }

    println!("finshed");
    Ok(())
}
fn get_white_list(filename: &str) -> io::Result<HashSet<String>> {
    let mut path = PathBuf::from("./white");
    path.push(filename);

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines = reader.lines().filter_map(|line| line.ok()).collect();
            Ok(lines)
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(HashSet::new()),
        Err(e) => Err(e),
    }
}
fn get_domain(line: &String) -> Option<String> {
    let mut l = line.trim();
    if l.is_empty() {
        return None;
    } else if l.starts_with('#') {
        return None;
    } else if l.contains('.') {
        if l.starts_with("regexp:") || l.starts_with("include:") {
            return None;
        }
        if l.starts_with("full:") {
            l = &l[5..];
        }
        if l.contains('@') {
            //println!("{}", l);
            let c: Vec<&str> = l.split('@').collect();
            for i in 1..c.len() {
                if c[i].trim() == "cn" {
                    return None;
                }
            }
            l = c[0].trim();
            //println!("{}", l);
        }
        if l.contains('#') {
            let c: Vec<&str> = l.split('#').collect();
            l = c[0].trim();
            //println!("{}", l);
        }
        //let ll = String::from(l);
        return Some(l.to_string());
    } else {
        return None;
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
