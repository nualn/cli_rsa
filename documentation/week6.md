## What I did:
- Did the peer review.
- Improved testing.
- Improved documentation.
- Added very basic padding to make encryption work with non-text files.

## Progress:
- Test coverage increased somewhat. 
- Tests are now run with both small and large inputs.
- Pad encryption input with a 1 byte to preserve leading zeros so that program works with non-text files.

## What I learned:
- Learned about more about padding.
- Not much else. This week was mostly about improving testing and documentation.

## What was unclear/hard:
- Is the unit testing good enough? I'm not sure.
- Should the correctness of the program be tested in some additional way aside from testing the encryption and decryption functions.
- Performance testing does not seem to be very relevant for this program. The encryption and decryption functions grow linearly with the size of the inputs. Miller-rabin should have time complexity O(k log³ n) and grow with the key size. However the key size used in the program is constant and the speed of key generation is highly dependent on what the random number generator outputs.

## What's next:
- Improve documentation.
- Add end-to-end tests.

## Time used: 
8h