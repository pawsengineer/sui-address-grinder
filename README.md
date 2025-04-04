# Sui Address Grinder

A Rust CLI tool for grinding Sui blockchain addresses and seed phrases.

## Overview

This project allows users to generate specific Sui blockchain addresses and their corresponding seed phrases using a grinding algorithm.

## Installation

Ensure you have Rust installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone git@github.com:pawsengineer/sui-address-grinder.git
cd sui-address-grinder
```

## Build and install

To build and install `sui-address-grinder`, run:

```sh
./install.sh
```

Verify Installation
After installation, verify that it is installed correctly by running:

```sh
sui-address-grinder --version
```

## Usage

Run the CLI tool with the arguments:

```sh
sui-address-grinder --starts-with ca7 --ends-with fff --cores 10 --ignore-case
```

It's important to note that the `--starts-with` option disregards the 0x prefix. Therefore, to match addresses that begin with `0xca7`, you should provide `ca7` as the argument.

### Output Format

The tool prints the generated Sui address and its corresponding seed phrase:

```
====================================================
Address:        <generated-address>
Seedphrase:     <generated-seedphrase>
====================================================
```

## License

This project is licensed under the MIT License.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Author

[Paws Engineer](https://github.com/pawsengineer)
