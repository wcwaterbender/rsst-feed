use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self,BufReader};

use std::path::Path;

#[derive(Debug)]
enum Commands {
    AddFeed,
    RemoveFeed,
    ListFeed,
    DisplayFeed,
    BadCmd
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn add_feed_source(src: String) -> Result<(), Box<dyn Error>>{

    // check that we havent already added the source before
    if let Ok(lines) = read_lines("./sources") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == src{
                    Err("error: source already in file")?
                }
            }
        }
    }
    //todo, create sources file if none exists
    else {
        Err("no sources file found")?
    }

    //add our new source
    let mut file = OpenOptions::new()
        .write(true)
        .open("sources")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", src.as_str()) { 
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn remove_feed_source(src: String) -> Result<(), Box<dyn Error>>{
    let file_read = File::open("./sources").expect("no such file");
    let buf = BufReader::new(file_read);
    let sources: Vec<String> = buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect();
    
    let mut file_write = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("sources")
        .unwrap();

    for source in sources {
        if source != src {
            if source != "\n".to_string() {
                if let Err(e) = writeln!(file_write, "{}", source.as_str()) { 
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

fn list_sources() -> Result<(), Box<dyn Error>>{

    if let Ok(lines) = read_lines("./sources") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip); 
            }
        }
    }
    else {
        Err("no sources file found")?
    }
    Ok(())
}

fn display() -> Result<(), Box<dyn Error>>{

    println!("Display the Feed");
    Ok(())
}

fn handle_bad_cmd() -> Result<(), Box<dyn Error>>{

    println!("Display the Feed");
    Err("err reading input please fix")?
}

fn take_input() -> Result<(), Box<dyn Error>>{

    // accept arguments 
    let mut args = std::env::args().skip(1);

    // support add (2), remove(2), list(1), display(1)

    let (cmd, opt) = match args.next() {
        Some(s) => 
            match s.as_str() {
                "add" => 
                    match args.next() {
                        Some(s) => (Commands::AddFeed, Some(s)),
                        None => (Commands::BadCmd, None)
                    }
                "remove" => 
                    match args.next() {
                        Some(s) => (Commands::RemoveFeed, Some(s)),
                        None => (Commands::BadCmd, None)
                    }
                "list" => (Commands::ListFeed , None),
                "display" => (Commands::DisplayFeed, None),
                _ =>  (Commands::BadCmd, None)
            },
        None => Err("err reading input please fix")?
    };

    match cmd {
        Commands::AddFeed if opt.is_some()=> add_feed_source(opt.unwrap()),
        Commands::RemoveFeed if opt.is_some()=> remove_feed_source(opt.unwrap()),
        Commands::ListFeed => list_sources(),
        Commands::DisplayFeed => display(),
        _ => handle_bad_cmd()
    }
    
}

fn main() {
    let result = take_input();
    if let Err(err) = result {
        let _ = writeln!(std::io::stderr(), "{}", err);
    }
}
