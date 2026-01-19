use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            path: PathBuf::from("/persistent/old_roots"),
        }
    }
}

// fn main() -> Result<(), confy::ConfyError> {
//     let cfg: MyConfig = confy::load("my-app-name", None)?;
//     Ok(())
// }
