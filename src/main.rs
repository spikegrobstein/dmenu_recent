// usage: dmenu_remember [ -c <count> ] [ --no-output ] <filename>
//
// reads from STDIN a command to add
// reads in the filename (if it exists) and dedupes it, then prepends the item.
// outputs the item to STDOUT

extern crate clap;
use clap::{Arg,App};

use anyhow::{Result, anyhow};

use std::env;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const DEFAULT_FILENAME: &str = ".dmenu.recent";

fn main() {
    let default_file = default_file_path();

    let matches = App::new("dmenu_remember")
        .version("0.0.0")
        .author("Spike Grobstein <me@spike.cx>")
        .arg(Arg::with_name("count")
            .help("The number of items to remember")
            .short("c")
            .long("count")
            .takes_value(true)
            .validator(|count| {
                match count.parse::<u32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Count must be an integer.".to_string()),
                }
            })
        )
        .arg(Arg::with_name("no-output")
            .help("Do not output anything after adding to the recent file.")
            .long("no-output")
            .takes_value(false)
        )
        .arg(Arg::with_name("file")
            .help("The file to use for the database.")
            .index(1)
            .default_value(&default_file)
        )
        .get_matches();


    let input = read_input().unwrap();
    eprintln!("got value: '{}'", input);

    let recentfile = matches.value_of("file").unwrap();

    add_item_to_recentfile(&input, &recentfile, 6).unwrap();

    eprintln!("done.");
}

fn read_input() -> Result<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input)? {
        0 =>
            Err(anyhow!("Expected input, but got nothing.")),
        _ =>
            Ok(input.trim_end_matches('\n').to_string()), // we good
    }
}

fn add_item_to_recentfile(item: &str, recentfile: &str, max: usize) -> Result<()> {
    // read in the recentfile
    // dedupe along the way (remove anything that matches item)
    // prepend item
    // write out the file again.

    let mut entries = Vec::new();
    entries.push(item.to_string()); // push the item first

    load_recentfile(&mut entries, &recentfile, max);

    // now we have our entrylist.
    // write it out to a file
    let mut f = File::create(&recentfile)?;
    for line in &entries {
        f.write_fmt(format_args!("{}\n", line))?;
    }

    Ok(())
}

fn load_recentfile(entries: &mut Vec<String>, recentfile: &str, max: usize) -> usize {
    let f = match File::open(&recentfile) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("No recentfile. not loading anything.");
            return 0
        },
    };

    let mut reader = BufReader::new(f);

    loop {
        eprintln!("loop iteration...");

        let mut line = String::new();
        if let Err(_) = reader.read_line(&mut line) {
            break;
        }

        if line.len() == 0 {
            break;
        }

        // remove any whitespace on the edges
        let line = line.trim_end_matches('\n');

        eprintln!("got line: {}", line);

        let line = line.to_string();

        if ! entries.contains(&line) {
            entries.push(line);
        }

        if entries.len() >= max {
            break;
        }
    }

    eprintln!("Done loading recentfile");

    entries.len() - 1
}

fn default_file_path() -> String {
    let home_path = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => "./".to_string(),
    };

    let mut path = PathBuf::from(home_path);
    path.push(DEFAULT_FILENAME);

    // TODO maybe this should use some option chaining or something.
    path.to_str()
        .unwrap()
        .to_string()
}
