use regex::Regex;

use crate::cli::Args;

pub fn query(config: crate::config::Config, args: &Args, path: bool, search: &str) {
    let search = Regex::new(search).unwrap();

    for entry in crate::roots::walk_all(&config, args) {
        let is_match = if path {
            search.is_match(&entry.path().to_string_lossy())
        } else {
            search.is_match(&entry.file_name().to_string_lossy())
        };

        if is_match {
            println!("{}", entry.path().to_string_lossy());
        }
    }
}
