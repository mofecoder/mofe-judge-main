use std::collections::HashMap;

pub fn generate_lang_cmd_map() -> HashMap<String, Command> {
    let mut map = HashMap::new();
    map.insert(
        "c17_gcc:10.2.0".to_string(),
        Command {
            compile: "gcc-10 Main.c -O2 -lm -std=gnu17 -o Main.out 2> userStderr.txt".to_string(),
            run: "./Main.out < testcase.txt > userStdout.txt 2> userStderr.txt".to_string(),
            file_name: "Main.c".to_string(),
        },
    );

    map
}

#[derive(Debug)]
pub struct Command {
    pub compile: String,
    pub run: String,
    pub file_name: String,
}
