# Jsonify
[![Rust](https://github.com/rbas/jsonify/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rbas/jsonify/actions/workflows/rust.yml)

Jsonify is a fast and efficient command line tool for formatting and coloring JSON data. It takes JSON input and returns a clear, color-coded structure for easy reading. Ideal for developers and data analysts, jsonify is both blazingly fast and memory efficient.

### Formatting example
![jsonify formatting example](https://raw.githubusercontent.com/rbas/jsonify/main/assets/jsonify.svg)

### Handing error example
![jsonify error handling example](https://raw.githubusercontent.com/rbas/jsonify/main/assets/jsonify_error_handling.svg)


## Usage

By redirecting your output into `jsonify` you will get formatted and colored JSON structure

```
cat test.json | jsonify
```

You can combine `jsonify` with `curl` and read beautiful JSON structures in your command line

```
curl -s -X GET https://dummyjson.com/products/1 | jsonify
```

## Installation

To be able to run `jsofify` you will need to do following steps

1) Get `jsonify` binary file
2) Add the file or its link into your shell `$PATH`. _This step depeds on your shell_
3) Make sure that `jsonify` is executable (`chmod +x`)


### Get pre-built binary files

Download the pre-built binary file to your computer. You can find it in the section [releases](https://github.com/rbas/jsonify/releases) on GitHub.

Currently, binaries are pre-built for Mac architectures Mx silicons (`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`)


### Complie from source
If you want to compile `jsonify` from the source you will need [installed Rust](https://www.rust-lang.org/tools/install) and then clone this repository.
Once you will be done, go to the repository root and run following command 

```
cargo build --release
```
Rust will compile `jsonify` from the source code. Generate executable binary file will be at location `target/release/jsonify`.


## Contribute
You are welcomed to fork the project and create a branch for each new feature you wish to add. Ensure that you write necessary tests and run the current tests before making your pull request.

## License: MIT
© 2023 Martin Voldrich
This work is licensed under [MIT license](https://github.com/rbas/jsonify/blob/main/LICENSE).
