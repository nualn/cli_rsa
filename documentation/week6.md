## What I did:
- Did the peer review.
- Improved testing.
- Improved documentation.

## Progress:
- Test coverage increased somewhat. 
- Tests are now run with both small and large inputs.

## What I learned:
- Not much. This week was mostly about improving testing and documentation.

## What was unclear/hard:
- Is the unit testing good enough? I'm not sure.
- Should the correctness of the program be tested in some additional way aside from testing the encryption and decryption functions.
- Performance testing does not seem to be very relevant for this program. The encryption and decryption functions grow linearly with the size of the inputs. Miller-rabin should have time complexity O(k log³ n) and grow with the key size. However the key size used in the program is constant and the speed of key generation is highly dependent on what the random number generator outputs.

## What's next:
- Improve documentation.
- Add end-to-end tests.

## Time used: 
7h