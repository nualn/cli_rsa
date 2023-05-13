# Usage

You can download the program from the from the release page as a precompiled binary for Linux or Windows or you can compile the program yourself by cloning the repository and running `cargo build --release` in the root directory of the project source.

The program can be run with the following commands:

## Generate a keypair
```
cargo run generate
```
This command generates a keypair and saves it to two files: `key.public` and `key.private`. 

## Encrypt a message or file
```
cargo run encrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command encrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the encrypted message to the file at `<OUT_PATH>`.

`<IN_PATH>` defaults to stdin and `<OUT_PATH>` defaults to stdout, so the following command is also valid:
```
cargo run --release -- encrypt -k key.private 
```

Any type of file can be encrypted.

## Decrypt a message or file
```
cargo run decrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command decrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the decrypted message to the file at `<OUT_PATH>`.

`<IN_PATH>` defaults to stdin and `<OUT_PATH>` defaults to stdout, so the following command is also valid:
```
cargo run --release -- decrypt -k key.public
```

## Help

For more help run the program with the `--help` flag:
```
cargo run -- --help
```
