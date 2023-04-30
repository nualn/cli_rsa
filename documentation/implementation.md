# Implementation document

## The general structure of the program

The program is a CLI tool that can be used to generate a pair of keys, encrypt a message with a given key and decrypt a message with a given key.

The program code is structured as follows:
- `src/main.rs` contains the main function that is the entry point of the program and the CLI interface. The CLI interface is implemented using the [clap](https://docs.rs/clap/2.33.3/clap/) crate.
- `src/algorithms` contains the implementations of the algorithms used in the program. These include the Miller-Rabin primality test, the Extended Euclidean algorithm and a modular exponentiation function.
- `src/keys.rs` contains the implementation of the `Key` struct, which is used to represent a key, and the `KeyPair` struct, which is used to represent a RSA keypair. Methods related to the keys, including key genereation, encryption and decryption functions are also implemented in this file.

## Achived time and space complexity

The time and space complexity of the algorithm will be discussed here in the future.

## Deficiencies and improvements

The program implements the RSA cryptosystem, but it is not secure. The program does not implement secure padding, which is a crucial part of the RSA cryptosystem. Without padding the program is vulnerable to attacks such as the [chosen ciphertext attack](https://en.wikipedia.org/wiki/Chosen-ciphertext_attack).

The program functions therefore only as a demonstration of the RSA cryptosystem and is not suitable for real world use.

To make the program secure, a padding scheme such as [OAEP](https://en.wikipedia.org/wiki/Optimal_asymmetric_encryption_padding) should be implemented, as well as larger key sizes.

## References
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
- https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm