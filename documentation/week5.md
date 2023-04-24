## What I did:
- Did the peer review.
- Refactored the encryption and decryption functions to take generic input and output. 
- Added tests for the encryption and decryption functions.
- Made encryption and decryption input and output default to stdin and stdout to allow for piping.

## Progress:
- Test coverage increased somewhat. 
- Able to pipe input and output.

## What I learned:
A bit about the Traits feature of Rust. I used it to make the encryption and decryption functions generic over the type of input and output they take. This allowed me to test them with a `Vec<u8>` as input and output, and use stdin and stdout or a file in the actual program.

## What was unclear/hard:

## What's next:
- Increase code coverage. At this point most untested code is code used to write and read from files or parse command line arguments but there are still some functions that could use some tests.
- Write documentation.

## Time used: 
8h