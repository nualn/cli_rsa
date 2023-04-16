# Implementation document

## The general structure of the program

The program is a CLI tool that can be used to generate a pair of keys, encrypt a message with a given key and decrypt a message with a given key.

The program can be run with the following commands:
```
cargo run generate
```
This command generates a keypair and saves it to two files: `key.public` and `key.private`. 

```
cargo run encrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command encrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the encrypted message to the file at `<OUT_PATH>`.

```
cargo run decrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command decrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the decrypted message to the file at `<OUT_PATH>`.

For more help (or rather the same information) run the program with the `--help` flag:
```
cargo run -- --help
```

The program code is structured as follows:
- `src/main.rs` contains the main function and the CLI interface.
- `src/algorithms` contains the implementations of the algorithms used in the program. These include the Miller-Rabin primality test, the Extended Euclidean algorithm and a modular exponentiation function.
- `src/keys.rs` contains the implementation of the `Key` struct, which is used to represent a key, and the `KeyPair` struct, which is used to represent a RSA keypair. Methods related to the keys, including key genereation, encryption and decryption functions are also implemented in this file.

## Achived time and space complexity

The time and space complexity of the algorithm will be discussed here in the future.

## Deficiencies and improvements

The program is not yet complete. The following improvements are planned:
- Make encryption and decryption input and output default to stdin and stdout to allow for piping.

## References
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
- https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm