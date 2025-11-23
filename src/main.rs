use home::home_dir;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let shell = detect_shell();
    println!("Detected shell: {}", shell);

    let entry_file = expand_home_dir(strip_home_dir_tilde(entry_file()));
    println!("Entry file: {}", entry_file);

    let entry_file = entry_file.to_string();

    let mut files = vec![entry_file.clone()];
    files.extend(find_files(&entry_file));

    println!("All files:");

    for file in files {
        println!("{}", file);
    }
}

fn detect_shell() -> String {
    String::from("zsh")
}

fn entry_file() -> &'static str {
    "~/.zshrc"
}

fn find_files(file: &str) -> Vec<String> {
    match read_file(file) {
        Ok(mut lines) => {
            let mut transformed_lines: Vec<String> = vec![];
            let mut children: Vec<String> = vec![];

            for line in &lines {
                let line_full_path = expand_home_dir(strip_home_dir_tilde(&line));
                transformed_lines.push(line_full_path.clone());
                children.extend(find_files(&line_full_path));
            }

            transformed_lines.extend(children);

            return transformed_lines;
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            println!("{}", file);
            return vec![];
        }
    }
}

fn read_file(file: &str) -> io::Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.contains("source") {
            let re = Regex::new(r"\bsource\s+([^\s#;&|]+)").unwrap();

            if let Some(caps) = re.captures(&line) {
                let file_path: String = strip_quotes(&caps[1]).to_string();
                lines.push(file_path);
            }

        }
    }

    Ok(lines)
}

fn strip_quotes(s: &str) -> &str {
    s.strip_prefix('"')
     .and_then(|s| s.strip_suffix('"'))
     .unwrap_or(s)
}

fn strip_home_dir_tilde(s: &str) -> &str {
    s.strip_prefix("~/").unwrap_or(s)
}

fn expand_home_dir(path: &str) -> String {
    home_dir().expect("Failed to get home directory").join(path).to_str().unwrap().to_string()
}

