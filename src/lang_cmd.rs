use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static LANG_CMD: Lazy<HashMap<String, Command>> = Lazy::new(generate_lang_cmd_map);
#[derive(Debug)]
pub struct Command {
    pub compile: String,
    pub run: String,
    pub file_name: String,
}

pub fn generate_lang_cmd_map() -> HashMap<String, Command> {
    let mut map = HashMap::new();
    map.insert(
        "c17_gcc:10.2.0".to_string(), //C17
        Command {
            compile: "gcc-10 /judge/Main.c -O2 -lm -std=gnu17 -o /judge/Main.out 2> /judge/userStderr.txt"
                .to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.c".to_string(),
        },
    );
    map.insert(
        "cpp17_gcc:10.2.0".to_string(), //C++17
        Command {
            compile: "g++-10 /judge/Main.cpp -O2 -lm -std=gnu++17 -o /judge/Main.out 2> /judge/userStderr.txt"
                .to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.cpp".to_string(),
        },
    );
    map.insert(
        "cpp17-acl_gcc:10.2.0".to_string(), //C++17 + ACL
        Command {
            compile:
                "g++-10 /judge/Main.cpp -O2 -lm -std=gnu++17 -I/ -o /judge/Main.out 2> /judge/userStderr.txt"
                    .to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.cpp".to_string(),
        },
    );
    map.insert(
        "cpp20_gcc:10.2.0".to_string(), //C++20
        Command {
            compile: "g++-10 /judge/Main.cpp -O2 -lm -std=gnu++2a -o /judge/Main.out 2> /judge/userStderr.txt"
                .to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.cpp".to_string(),
        },
    );
    map.insert(
        "java:11.0.9".to_string(), //java11
        Command {
            compile: "javac -encoding UTF-8 /judge/Main.java 2> /judge/userStderr.txt".to_string(),
            run: "java /judge/Main < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                .to_string(),
            file_name: "Main.java".to_string(),
        },
    );
    map.insert(
        "python:3.9.0".to_string(), //python3
        Command {
            compile: "python3.9 -m py_compile /judge/Main.py 2> /judge/userStderr.txt".to_string(),
            run: "python3.9 /judge/Main.py < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.py".to_string(),
        },
    );
    map.insert(
        "pypy3:7.3.3".to_string(), //pypy3
        Command {
            compile: "pypy3 -m py_compile /judge/Main.py 2> /judge/userStderr.txt".to_string(),
            run: "pypy3 /judge/Main.py < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.py".to_string(),
        },
    );
    map.insert( "cs_mono:6.12.0.90".to_string(), //C#
        Command {
            compile: "source ~/.profile && mcs /judge/Main.cs -out:/judge/Main.exe 2> /judge/userStderr.txt".to_string(),
            run: "source ~/.profile && mono /judge/Main.exe < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.cs".to_string(), 
        },
    );
    map.insert( "cs_dotnet:5.0".to_string(), // C#
        Command {
            compile: "source ~/.profile && cd /judge/Main && dotnet new console && mv /judge/Main.cs /judge/Main/Program.cs && dotnet publish -c Release --nologo -v q -o . 2> /judge/userStderr.txt && cd /".to_string(),
            run: "source ~/.profile && dotnet /judge/Main/Main.dll < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.cs".to_string(),
        },
    );
    map.insert( "go:1.15.5".to_string(), //golang
        Command {
            compile: "source ~/.profile && mv /judge/Main.go /judge/Main && cd /judge/Main && go build -o main.out /judge/Main.go 2> /judge/userStderr.txt".to_string(),
            run: "./judge/Main/main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.go".to_string(),
        },
    );
    map.insert( "nim:1.4.0".to_string(),
        Command {
            compile: "source ~/.profile && nim cpp -d:release --opt:speed --multimethods:on -o:/judge/Main.out /judge/Main.nim 2> /judge/userStderr.txt".to_string(),
            run: "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.nim".to_string(),
        },
    );
    map.insert( "rust:1.48.0".to_string(),
        Command {
            compile: "source ~/.profile && cd /judge/rust_workspace && mv /judge/Main.rs /rust_workspace/src/main.rs && cargo build --release 2> /judge/userStderr.txt && cd /".to_string(),
            run: "./rust_workspace/target/release/Rust < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.rs".to_string(), 
        },
    );
    map.insert( "ruby:2.7.2".to_string(),
        Command {
            compile: "source ~/.profile && ruby -w -c /judge/Main.rb 2> /judge/userStderr.txt".to_string(),
            run: "source ~/.profile && ruby /judge/Main.rb < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.rb".to_string(),
        },
    );
    map.insert( "kotlin:1.4.10".to_string(),
        Command {
            compile: "source ~/.profile && kotlinc /judge/Main.kt -include-runtime -d Main.jar 2> /judge/userStderr.txt".to_string(),
            run: "source ~/.profile && kotlin /judge/Main.jar < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.kt".to_string(),
        },
    );
    map.insert(
        "fortran:10.2.0".to_string(),
        Command {
            compile: "gfortran-10 -O2 /judge/Main.f90 -o /judge/Main.out 2> /judge/userStderr.txt".to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.f90".to_string(),
        },
    );
    map.insert(
        "perl:5.30.0".to_string(),
        Command {
            compile: "perl -c /judge/Main.pl 2> /judge/userStderr.txt".to_string(),
            run: "perl /judge/Main.pl < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.pl".to_string(),
        },
    );
    map.insert( "raku:2020.10".to_string(),
        Command {
            compile: "source ~/.profile && perl6 -c /judge/Main.p6 2> /judge/userStderr.txt".to_string(),
            run: "source ~/.profile && perl6 /judge/Main.p6 < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.p6".to_string(),
        },
    );
    map.insert(
        "crystal:0.35.1".to_string(),
        Command {
            compile: "crystal build /judge/Main.cr -o /judge/Main.out 2> /judge/userStderr.txt".to_string(),
            run:
                "/judge/Main.out < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt"
                    .to_string(),
            file_name: "Main.cr".to_string(),
        },
    );
    map.insert(
        "text_cat:8.30".to_string(),
        Command {
            compile: ":".to_string(),
            run: "cat /judge/Main.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.txt".to_string(),
        },
    );
    map.insert(
        "bash:5.0.17".to_string(),
        Command {
            compile: "bash -n /judge/Main.sh 2> /judge/userStderr.txt".to_string(),
            run: "bash /judge/Main.sh < /judge/testcase.txt > /judge/userStdout.txt 2> /judge/userStderr.txt".to_string(),
            file_name: "Main.sh".to_string(),
        },
    );
    map
}
