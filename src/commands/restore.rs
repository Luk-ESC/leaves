use std::fmt;
use std::fs::Metadata;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use inquire::Select;

struct FileToRestore {
    path: PathBuf,
    meta: Metadata,
}

impl FileToRestore {
    fn new(path: PathBuf, meta: Metadata) -> Self {
        Self { path, meta }
    }
}

impl fmt::Display for FileToRestore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

pub fn restore(config: crate::config::Config, automatic: bool, path: String) {
    let dest = std::path::absolute(path)
        .unwrap()
        .normalize_lexically()
        .unwrap();

    if dest.exists() {
        panic!("Can't restore file, output path already exists!");
    }

    // Make path relative from root
    let suffix: PathBuf = dest
        .components()
        .skip_while(|x| matches!(x, Component::Prefix(_) | Component::RootDir))
        .collect();

    let mut finds = vec![];
    for root in std::fs::read_dir(config.path).unwrap() {
        let mut file = root.unwrap().path();
        file.push(&suffix);

        if file.exists() {
            let meta = file.metadata().unwrap();
            finds.push(FileToRestore::new(file, meta));
        }
    }

    finds.sort_unstable_by_key(|x| std::cmp::Reverse(x.meta.created().unwrap()));

    let chosen = if automatic {
        finds.swap_remove(0)
    } else {
        println!("{} finds: ", finds.len());

        Select::new("Select file to restore: ", finds)
            .prompt()
            .unwrap()
    };

    copy_recursively(&chosen.path, &dest).unwrap();

    println!("{}", chosen.path.to_string_lossy().into_owned());
}

fn copy_recursively(src: &Path, dest: &Path) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("cp")
        .arg("-r")
        .arg(src)
        .arg(dest)
        .spawn()?;

    let status = cmd.wait()?;
    if !status.success() {
        panic!("Failed to run cp, exit code: {:?}", status.code());
    }

    Ok(())
}
