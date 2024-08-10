use std::env;
use std::fs::{self, DirBuilder};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::path::PathBuf;
fn main( ) {
    let args: Vec<String> = env::args().collect();
    let config_file = &args[1];
    let data_path = &args[2];
    let config_info = fs::read_to_string(config_file).unwrap();
    let mut save_path =PathBuf::from(data_path);
    save_path.push("save");
    DirBuilder::new().recursive(true).create(save_path);
    save_path=PathBuf::from(data_path);
    save_path.push("save/gfwlist.txt");
    let mut file = File::create(save_path).unwrap();
    for a in config_info.split(',')
    {
        let mut path = PathBuf::from(data_path);
        path.push(a);
        for b in read_lines(path).unwrap()
        {
            let c = b.unwrap();
            //println!("{}", c);       
            if let Some(d) = get_domain(&c){
                println!("{}", d);    
                file.write_all(d.as_bytes());
                file.write_all(b"\n");
            }
            
              
        }
        
    }

    println!("Hello, world!");
}
fn get_domain(line:&String) -> Option<String> 
{
    let mut l =line.trim();
    if l.is_empty()
    {
        return None;
    }
    else if l.starts_with('#') {
        return None;
    }
    else if l.contains('.') {
        if l.starts_with("regexp:") || l.starts_with("include:")   {
            return None;
        }
        if l.starts_with("full:") {
            l = &l[5..];           
        }
        if l.contains("@") {
            //println!("{}", l);    
            let c :Vec<&str> = l.split('@').collect();
            if c[1]=="cn" {
                return None;
            }
            l = c[0].trim();
            //println!("{}", l);   
        }
        let ll =String::from(l);
        return  Some(ll);
    }
    else {
        return None;
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
