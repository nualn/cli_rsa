# CLI RSA

The goal of this project is to implement the RSA key generation and encryption. 

Asymmetric cryptography is vital for secure data transmission in unsecure environments such as the Internet. RSA is widely used for that purpose. The purpose of this project is to learn about encryption and the RSA cryptosystem.

## Usage

The program can be run with the following commands:

### Generate a keypair
```
[executable] generate
```
This command generates a keypair and saves it to two files: `key.public` and `key.private`. 

### Encrypt a message or file
```
[executable] encrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command encrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the encrypted message to the file at `<OUT_PATH>`.

`<IN_PATH>` defaults to stdin and `<OUT_PATH>` defaults to stdout, so the following command is also valid:
```
[executable] encrypt -k key.private 
```

Any type of file can be encrypted.

### Decrypt a message or file
```
[executable] decrypt --in-path <IN_PATH> --out-path <OUT_PATH> --key-path <KEY_PATH>
```
This command decrypts the contents of the file at `<IN_PATH>` with the key at `<KEY_PATH>` and saves the decrypted message to the file at `<OUT_PATH>`.

`<IN_PATH>` defaults to stdin and `<OUT_PATH>` defaults to stdout, so the following command is also valid:
```
[executable] decrypt -k key.public
```

### Help

For more help run the program with the `--help` flag:
```
[executable] --help
```

## Deficiencies and improvements

The program implements the RSA cryptosystem, but it is not secure. The program does not implement secure padding, which is a crucial part of the RSA cryptosystem. Without padding the program is vulnerable to attacks such as the [chosen ciphertext attack](https://en.wikipedia.org/wiki/Chosen-ciphertext_attack).

The program functions therefore only as a demonstration of the RSA cryptosystem and is not suitable for real world use.

To make the program secure, a padding scheme such as [OAEP](https://en.wikipedia.org/wiki/Optimal_asymmetric_encryption_padding) should be implemented, as well as larger key sizes.