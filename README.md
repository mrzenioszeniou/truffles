__truffles__ is a command line tool built with Rust that scrapes data from real estate websites,
catalogues it, and serves it in CSVs under `~/.truffles/`.

## Building & Running

After you clone this repository, you'll need a Rust toolchain to build the code. If you don't have
`cargo` on your machine, you can grab it (and `rustup`) [here](https://rustup.rs/). Once you have it
installed just do:

```
cargo build
```

To start scraping data, just run:

```
cargo run
```

If you want an overview of options to customise the data scope just run:

```
cargo run -- -h
```
