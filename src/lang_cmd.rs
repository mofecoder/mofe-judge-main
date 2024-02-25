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
            run: run.to_string()
                + "< /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt",
            file_name: file_name.to_string(),
        }
    }
}

pub fn generate_lang_cmd_map() -> HashMap<String, Command> {
    let mut map = HashMap::new();
    // $INSERT_HERE$
    map.insert(
        "c17_gcc:12.2.0".to_string(),
        Command::new(
            "gcc-12 ./Main.c -O2 -lm -std=gnu17 -o Main.out -DONLINE_JUDGE -Wall -Wextra",
            "./Main.out",
            "Main.c",
        ),
    );
    map.insert(
        "cpp20_gcc:12.2.0".to_string(),
        Command::new(
            "g++-12 ./Main.cpp -O2 -lm -std=gnu++20 -I/opt/ac-library -I/opt/testlib -o Main.out -DONLINE_JUDGE -Wall -Wextra -fsplit-stack",
            "./Main.out",
            "Main.cpp"
        )
    );
    map.insert(
        "cpp23_gcc:12.2.0".to_string(),
        Command::new(
            "g++-12 ./Main.cpp -O2 -lm -std=gnu++2b -I/opt/ac-library -I/opt/testlib -o Main.out -DONLINE_JUDGE -Wall -Wextra -fsplit-stack",
            "./Main.out",
            "Main.cpp"
        )
    );
    map.insert(
        "java:17.0.7".to_string(),
        Command::new("javac ./Main.java", "java Main", "Main.java"),
    );
    map.insert(
        "python:3.11.4".to_string(),
        Command::new(
            "python3.11 -m py_compile ./Main.py",
            "python3.11 ./Main.py",
            "Main.py",
        ),
    );
    map.insert(
        "go:1.20.6".to_string(),
        Command::new("go build -o Main.out ./Main.go", "./Main.out", "Main.go"),
    );
    map.insert(
        "cs_dotnet:7.0".to_string(),
        Command::new(
            "dotnet publish -o . -c Release -v q --nologo 1>&2",
            "./Main",
            "Main.cs",
        ),
    );
    map.insert(
        "nim:1.6.14".to_string(),
        Command::new(
            "nim cpp -d:release --opt:speed --multimethods:on -o:Main.out Main.nim",
            "./Main.out",
            "Main.nim",
        ),
    );
    map.insert(
        "rust:1.71.0".to_string(),
        Command::new(
            "mv Main.rs src/main.rs && cargo build --release --quiet --offline",
            "./target/release/Rust",
            "Main.rs",
        ),
    );
    map.insert(
        "ruby:3.2.2".to_string(),
        Command::new("ruby -w -c ./Main.rb", "ruby ./Main.rb", "Main.rb"),
    );
    map.insert(
        "kotlin:1.9.0".to_string(),
        Command::new(
            "kotlinc ./Main.kt -include-runtime -d Main.jar",
            "kotlin Main.jar",
            "Main.kt",
        ),
    );
    map.insert(
        "fortran:12.2.0".to_string(),
        Command::new(
            "gfortran -O2 Main.f90 -o Main.out",
            "./Main.out",
            "Main.f90",
        ),
    );
    map.insert(
        "crystal:1.8.2".to_string(),
        Command::new(
            "crystal build --release --no-debug --no-color -o Main.out ./Main.cr",
            "./Main.out",
            "Main.cr",
        ),
    );
    map.insert(
        "text_cat:8.32".to_string(),
        Command::new(":", "cat Main.txt", "Main.txt"),
    );
    map.insert(
        "perl:5.36.0".to_string(),
        Command::new("perl -W -c ./Main.pl", "perl -X ./Main.pl", "Main.pl"),
    );
    map.insert(
        "raku:2022.12".to_string(),
        Command::new("perl6 -c ./Main.p6", "perl6 ./Main.p6", "Main.p6"),
    );
    map.insert(
        "bash:5.2.15".to_string(),
        Command::new("bash -n ./Main.sh", "bash ./Main.sh", "Main.sh"),
    );
    map.insert(
        "pypy310:7.3.12".to_string(),
        Command::new(
            "pypy3 -m py_compile ./Main.py",
            "pypy3 ./Main.py",
            "Main.py",
        ),
    );
    map.insert(
        "dc:1.4.1".to_string(),
        Command::new(":", "dc Main.dc", "Main.dc"),
    );
    map
}
