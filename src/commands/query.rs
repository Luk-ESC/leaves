use regex::Regex;
use walkdir::WalkDir;

pub fn query(config: crate::config::Config, path: bool, search: String) {
    let search = Regex::new(&search).unwrap();

    let walkdir = WalkDir::new(config.path).into_iter();

    for entry in walkdir {
        let Ok(entry) = entry else {
            continue;
        };

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
