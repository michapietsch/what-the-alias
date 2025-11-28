use home::home_dir;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use rand::prelude::*;

fn main() {
    let shell = detect_shell();

    let entry_file = expand_home_dir(strip_home_dir_tilde(entry_file()));

    let entry_file = entry_file.to_string();

    let mut files = vec![entry_file.clone()];
    let mut aliases: Vec<String> = vec![];

    let [found_files, found_aliases] = scan_recursively(&entry_file);

    files.extend(found_files);
    aliases.extend(found_aliases);

    let mut rng = rand::rng();
    let picked_alias = aliases.choose(&mut rng);

    let re = Regex::new(r"^([a-z]*)=(.*)$").unwrap();
    if let Some(capts) = re.captures(&picked_alias.unwrap()) { 
        let actual_alias: &str = &capts[1];
        let command: String = strip_semicolon(strip_quotes(&capts[2]));

        println!("");
        println!("Did you know?");
        println!("You can use the alias: {:?} instead of {:?}", actual_alias, command);
        println!("");
    }
}

fn detect_shell() -> String {
    String::from("zsh")
}

fn entry_file() -> &'static str {
    "~/.zshrc"
}

fn scan_recursively(file: &str) -> [Vec<String>; 2] {
    match read_file(file) {
        Ok([lines, aliases]) => {
            let mut transformed_lines: Vec<String> = vec![];
            let mut found_aliases: Vec<String> = vec![];
            let mut children: Vec<String> = vec![];

            for line in &lines {
                let line_full_path = expand_home_dir(strip_home_dir_tilde(&line));
                transformed_lines.push(line_full_path.clone());
                let [found_files, these_found_aliases] = scan_recursively(&line_full_path);
                children.extend(found_files);
                found_aliases.extend(these_found_aliases);
            }

            found_aliases.extend(aliases);

            transformed_lines.extend(children);

            return [transformed_lines, found_aliases];
        }
        
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            println!("{}", file);
            return [vec![], vec![]];
        }
    }
}

fn read_file(file: &str) -> io::Result<[Vec<String>; 2]> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    let mut aliases: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.contains("source") {
            let re = Regex::new(r"\bsource\s+([^\s#;&|]+)").unwrap();

            if let Some(caps) = re.captures(&line) {
                let file_path: String = strip_quotes(&caps[1]).to_string();
                lines.push(file_path);
            }

        } 

        if line.contains("alias") {
            let reg = Regex::new(r"^alias (.*)$").unwrap();

            if let Some(capts) = reg.captures(&line) {
                let alias: String = strip_quotes(&capts[1]).to_string();
                aliases.push(alias);
            }
        }
    }

    Ok([lines, aliases])
}

fn strip_quotes(s: &str) -> String{
    s
    .replace('"', "")
    .replace('\'', "")
}

fn strip_semicolon(s: String) -> String {
    s.trim_end_matches(';').to_string()
}

fn strip_home_dir_tilde(s: &str) -> &str {
    s.strip_prefix("~/").unwrap_or(s)
}

fn expand_home_dir(path: &str) -> String {
    home_dir().expect("Failed to get home directory").join(path).to_str().unwrap().to_string()
}

