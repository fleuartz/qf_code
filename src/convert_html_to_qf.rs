use crate::qf_encode::encode;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use colored::*;
use crypto::md5::Md5;
use crypto::digest::Digest;

#[derive(Debug, PartialEq)]
pub enum QFError {
    QFFileError,
}

impl fmt::Display for QFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QFError {:?}", self)
    }
}

impl Error for QFError {}

pub fn convert(directory_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{} \\{}", "Convert:".yellow().bold(), directory_path);
    let entries = fs::read_dir(directory_path).map_err(|_| QFError::QFFileError)?;

    let log_file_path = format!("{}/qf_code.log", directory_path);
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|_| QFError::QFFileError)?;

    let mut existing_hashes = HashSet::new();

    let log_contents = fs::read_to_string(&log_file_path).unwrap_or_default();
    for line in log_contents.lines() {
        if let Some(md5_hash) = extract_md5_hash(line) {
            existing_hashes.insert(md5_hash);
        }
    }

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if let Some(file_path_str) = file_path.to_str() {
            if file_path_str.ends_with(".html") {
                process_html_file(file_path_str, &mut log_file, &mut existing_hashes)?;
            }
        }
    }

    Ok(())
}

fn extract_md5_hash(line: &str) -> Option<String> {
    let re = Regex::new(r#"\[MD5: ([a-f0-9]+)\]"#).ok()?;
    if let Some(captures) = re.captures(line) {
        captures.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    }
}

fn process_html_file(
    file_path: &str,
    log_file: &mut fs::File,
    existing_hashes: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open(file_path).map_err(|_| QFError::QFFileError)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut hasher = Md5::new();
    hasher.input_str(&contents);
    let md5_hash = hasher.result_str();

    file.seek(SeekFrom::Start(0))?;

    let re = Regex::new(r#"<script[^>]*>(?P<script>[\s\S]*?"quoridorstrats_notation":[\s\S]*?)</script>"#)?;

    let script_tag = match re.captures(&contents) {
        Some(captures) => captures.name("script").map_or("", |m| m.as_str()),
        None => {
            eprintln!("{} No script tag found in {}", "Error:".red().bold(), file_path);
            return Ok(());
        }
    };

    let notations: Vec<String> = Regex::new(r#""quoridorstrats_notation":\s*"([^"]+)""#)?
        .captures_iter(script_tag)
        .map(|cap| cap[1].to_string())
        .collect();

    let notations_str = notations.join(",");

    match encode(&notations_str) {
        Ok(qf_code) => {
            println!("{} <- ({})", qf_code, file_path);

            if existing_hashes.contains(&md5_hash) {
                return Ok(());
            }

            writeln!(log_file, "{} <- ({}) [MD5: {}]", qf_code, file_path, md5_hash)?;
            existing_hashes.insert(md5_hash);
        }
        Err(err) => {
            eprintln!("{} encoding notations: {:?}", "Error:".red().bold(), err);
            writeln!(log_file, "{} encoding notations: {:?}", "Error:".red().bold(), err)?;
        }
    }

    Ok(())
}