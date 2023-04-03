## Test coverage report
![](https://codecov.io/gh/nualn/tiralabra/branch/main/graphs/sunburst.svg?token=QWGERCEMN8)

## What was tested and how
Currently there are unit tests for the mathematical functions in the algorithms.rs file. The other logic is still untested.
The testing is done using Rust's built-in unit testing.

## Inputs
The testinputs are smal integers for most tests. There are now some tests with a large prime number, and the plan is to test everything with large inputs.

## Running
Run ```cargo test``` in the terminal
**Note!** You need to have rustc and cargo installed.
