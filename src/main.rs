#[macro_use]
extern crate clap;
extern crate regex;
extern crate yaml_rust;

use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

mod solver;

fn is_usize(s: String) -> Result<(), String> {
    let n: Result<usize, std::num::ParseIntError> = s.parse();
    match n {
        Err(e) => Err(format!("Invalid amount: {}", e)),
        Ok(_) => Ok(()),
    }
}

fn is_valid_file(s: String) -> Result<(), String> {
    if std::path::PathBuf::from(s).exists() {
        Ok(())
    } else {
        Err(String::from("File does not exist"))
    }
}

fn is_valid_yaml_file(s: String) -> Result<(), String> {
    is_valid_file(s.clone())?;
    let mut file = std::fs::File::open(s).unwrap();
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        return Err(format!("Invalid string: {}", e));
    }
    match YamlLoader::load_from_str(&content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Invalid yaml: {}", e)),
    }
}

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generates completion scripts for your shell")
                .arg(
                    Arg::with_name("SHELL")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh"])
                        .help("The shell to generate the script for"),
                ),
        )
        .subcommand(
            SubCommand::with_name("solve")
                .about("Give words possible for a given input")
                .arg(
                    Arg::with_name("USED")
                        .required(true)
                        .help("All the letters already used"),
                )
                .arg(
                    Arg::with_name("PATTERN")
                        .required(true)
                        .help("Pattern for known letters (___bar for example)"),
                )
                .arg(
                    Arg::with_name("WORD_LIST")
                        .required(true)
                        .help("The word list to search in")
                        .validator(is_valid_file),
                ).arg(
                    Arg::with_name("NEXT_LETTER")
                        .short("-n")
                        .long("--next-letter")
                        .help("gives the letter that is present in most of the possible words")
                )
        )
        .subcommand(
            SubCommand::with_name("word")
                .about("Gives hard words to guess")
                .arg(
                    Arg::with_name("WORD_LIST")
                        .required(true)
                        .help("The word list to take from")
                        .validator(is_valid_file),
                ).arg(
                    Arg::with_name("AMOUNT")
                        .default_value("1")
                        .short("c")
                        .long("count")
                        .help("amount of words to generate")
                        .validator(is_usize)
                ).arg(
                    Arg::with_name("FREQ_FILE")
                        .short("f")
                        .long("freq")
                        .validator(is_valid_yaml_file)
                        .takes_value(true).
                        help("Takes the word with the most unfrequent letters, frequencies from a yaml file")
                ),
        )
}

fn load_wordlist(file: &std::path::Path) -> Result<String, std::io::Error> {
    let mut file = File::open(file).unwrap();
    let mut word_list = String::new();
    file.read_to_string(&mut word_list)?;
    Ok(word_list)
}

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        ("completions", Some(sub_matches)) => {
            let shell = sub_matches.value_of("SHELL").unwrap();
            build_cli().gen_completions_to(
                "another-rust-coin",
                shell.parse().unwrap(),
                &mut std::io::stdout(),
            );
        }
        ("solve", Some(sub_matches)) => {
            let file = std::path::PathBuf::from(sub_matches.value_of("WORD_LIST").unwrap());
            let used_letters = sub_matches.value_of("USED").unwrap();
            let pattern = sub_matches.value_of("PATTERN").unwrap();
            let word_list = load_wordlist(&file).expect("File containing invalid string");

            let match_regex = solver::match_regex(used_letters, pattern);

            let mut word_having: HashMap<char, u64> = HashMap::new();
            let alphabet = "abcdefghijklmnopqrstuvwxyz";

            for word_match in solver::get_matches(&word_list, &match_regex) {
                if sub_matches.is_present("NEXT_LETTER") {
                    for letter in alphabet.chars() {
                        if word_match.as_str().contains(letter) {
                            match word_having.get_mut(&letter) {
                                None => {
                                    word_having.insert(letter, 1);
                                }
                                Some(c) => {
                                    *c += 1;
                                }
                            }
                        }
                    }
                } else {
                    println!("{}", word_match.as_str());
                }
            }
            if sub_matches.is_present("NEXT_LETTER") {
                let mut max = 0;
                let mut max_letter = 'a';
                for (&letter, &amount) in word_having.iter() {
                    if amount > max && !used_letters.contains(letter) {
                        max_letter = letter;
                        max = amount;
                    }
                }
                println!("Next letter should be: {}", max_letter);
            }
        }
        (_, _) => (),
    }
}
