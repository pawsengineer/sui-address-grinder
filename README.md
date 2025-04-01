# Sui Keytool Grinder

A Rust CLI tool for grinding Sui blockchain addresses and seed phrases.

## Overview

This project allows users to generate specific Sui blockchain addresses and their corresponding seed phrases using a grinding algorithm.

## Installation

Ensure you have Rust installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone <repository-url>
cd <repository-folder>
cargo build --release
```

## Usage

Run the CLI tool with the required arguments:

```sh
cargo run --release
```

Example usage:

```sh
cargo run --release -- --starts-with cat --ends-with paws --ignore-case
```

It's important to note that the `--starts-with` option disregards the 0x prefix. Therefore, to match addresses that begin with 0xpaws, you should provide paws as the argument.

### Output Format

The tool prints the generated Sui address and its corresponding seed phrase:

```
====================================================
Address:        <generated-address>
Seedphrase:     <generated-seedphrase>
====================================================
```

## Dependencies

- `clap` for parsing command-line arguments
- `sui_keys` for Sui key generation
- `sui_types` for Sui signature scheme

## License

This project is licensed under the MIT License.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Author

[Paws Engineer](https://github.com/pawsengineer)