## What I did:
- Changed argument parsing to use the `clap` library
- Implemented encryption and decryption commands.

## Progress:
The core functionality complete. The program can now generate keys, encrypt and decrypt messages.

## What I learned:
New library, new syntax.

## What was unclear/hard:
The encryption sometimes ended up with the most significant byte being 0, which were ignored by the BigInt datastructure I used. This caused the decryption to fail sometimes. I fixed this by adding 0 bytes to the beginning of the encrypted message. This is not a perfect solution, since it writes 0 bytes at the end of the file, but it works for now.

## What's next:
- Refactor the encryption and decryption functions to make them testable.
- Add tests for the encryption and decryption functions.
- Further increase test coverage.
- Make encryption and decryption input and output default to stdin and stdout to allow for piping.

## Time used: 
14 h
