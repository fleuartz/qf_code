use crate::qf_encode::encode;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;

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

pub fn to_qf_code(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(directory_path).map_err(|_| QFError::QFFileError)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if let Some(file_path_str) = file_path.to_str() {
            if file_path_str.ends_with(".html") {
                process_html_file(file_path_str)?;
            }
        }
    }

    Ok(())
}

fn process_html_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open(file_path).map_err(|_| QFError::QFFileError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r#"<script[^>]*>(?P<script>[\s\S]*?"quoridorstrats_notation":[\s\S]*?)</script>"#)?;

    let script_tag = match re.captures(&contents) {
        Some(captures) => captures.name("script").map_or("", |m| m.as_str()),
        None => {
            eprintln!("Error: No script tag found in {}", file_path);
            return Ok(());
        }
    };

    let notations: Vec<String> = Regex::new(r#""quoridorstrats_notation":\s*"([^"]+)""#)?
        .captures_iter(script_tag)
        .map(|cap| cap[1].to_string())
        .collect();

    let notations_str = notations.join(",");

    match encode(&notations_str) {
        Ok(qf_code) => println!("{} <- ({})", qf_code, file_path),
        Err(err) => eprintln!("Error encoding notations: {:?}", err),
    }

    Ok(())
}
