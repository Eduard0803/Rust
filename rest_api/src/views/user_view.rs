use colored::*;
use serde_json;

pub fn log_request(method: &str, path: &str, details: &str) {
    print(&format!(
        "{} {} {}\n",
        method.color_method(),
        path,
        details
    ));
}

pub fn print(output: &str) {
    use std::io::{self, Write};
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush stdout");
}

pub trait ColorizeMethod {
    fn color_method(&self) -> String;
}

impl ColorizeMethod for &str {
    fn color_method(&self) -> String {
        match *self {
            "GET" => self.blue().to_string(),
            "POST" => self.green().to_string(),
            "DELETE" => self.red().to_string(),
            _ => self.to_string(),
        }
    }
}

pub fn log_json<T: serde::Serialize>(data: &T) {
    let json = serde_json::to_string_pretty(data).unwrap();
    print(&format!("{}\n", json));
}
