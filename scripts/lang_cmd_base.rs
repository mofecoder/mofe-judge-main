use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static LANG_CMD: Lazy<HashMap<String, Command>> = Lazy::new(generate_lang_cmd_map);
#[derive(Debug)]
pub struct Command {
    pub compile: String,
    pub run: String,
    pub file_name: String,
}

impl Command {
    fn new(compile: &str, run: &str, file_name: &str) -> Self {
        Self {
            compile: compile.to_string() + " 2> /judge/userStderr.txt",
            run: run.to_string() + " > /judge/userStdout.txt 2> /judge/userStderr.txt",
            file_name: file_name.to_string(),
        }
    }
}

pub fn generate_lang_cmd_map() -> HashMap<String, Command> {
    let mut map = HashMap::new();
    // $INSERT_HERE$
    map
}
