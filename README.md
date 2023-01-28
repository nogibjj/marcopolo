# marco polo command line
step 1: create new project
`cargo init --name marcopolo`

step 2: check out structure by using `tree`
```bash
.
├── Cargo.toml
├── LICENSE
├── Makefile
├── README.md
└── src
    └── main.rs
```

step 3: `make run`

step 4: run the target `./target/debug/marcopolo`

step 5: `touch src/lib.rs`

step 6: to reproduce
```bash
    9  make format
   10  make lint
   11  cargo -- --help
   12  cargo run -- --help
   13  cargo run -- speak -h
   14  cargo run -- speak --name marco
   15  cargo run -- speak --name bob
   16  ./target/debug/marcopolo
   17  ./target/debug/marcopolo speak --name marco
```
## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
