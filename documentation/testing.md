## Test coverage report
![Coverage report](https://user-images.githubusercontent.com/90393399/229392978-7b2b52d9-a658-4efb-a675-a8a6d1549839.png)


## What was tested and how
The testing is done using Rust's built-in unit testing.

The program is thoroughly unit tested. Functions for the cli and file handling are not tested. Also the generation of random primes is not tested, because its output is random and therefore hard to test. However the algorithm for testing primality is tested.

The tests are written in the same file as the code they test. This is a convention in Rust.

## Inputs
The tests are run with both small and large inputs. The small inputs are used to test the correctness of the program. The large inputs are equivalent to the inputs that the program is expected to handle in practice.
## Running
Run ```cargo test``` in the terminal in the root directory of the project source.

**Note!** 

You need to have rustc and cargo installed.
