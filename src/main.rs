extern crate clap;
use clap::{command, Arg, ArgAction};
use std::fs;
//use std::io;
//use std::io::BufRead;
// use std::collections::HashMap;

fn main() {
    // requires `cargo` feature, reading name, version, author, and description from `Cargo.toml`
    let cmd_args = command!()
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .help("Input. Can be a file or if not provided, stdin")
                .required(true)
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("If set, prints counts for each unique line")
        )
        .get_matches();

    // Open input
    let input = cmd_args.get_one::<String>("input").unwrap();
    let my_data = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    // Add each line to a hashmap, where the key is the line and the value is the count
    let mut my_map = std::collections::HashMap::new();
    for line in my_data.lines() {
        let count = my_map.entry(line).or_insert(0);
        *count += 1;
    }

    // Print out the keys of the hashmap
    let hash_keys = my_map.keys();
    for key in hash_keys {
        // If the count flag is set, print the count of each key
        if cmd_args.get_flag("count") {
            let count = my_map.get(key).unwrap();
            println!("{}\t{}", key, count);
        } else {
            println!("{}", key);
        }
    }
}
