# CafeCoder-rs
## Test
### 1. Setup a mysql server
You can set up a mysql server using docker.
```console
$ cd test_db
$ docker-compose up -d
```

### 2. Execute
```console
$ cargo run
```

## Deploy
### 1. Release Build
```console
$ cargo build --release
```

### 2. Execute
```console
$ ./target/release/cafecoder-rs
```