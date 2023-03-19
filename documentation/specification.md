# Specification Document

The goal of this project is to implement the RSA encryption algorihm as defined by R.L. Rivest, A. Shamir, and L. Adleman in their 1977 paper. 

Asymmetric cryptography is vital for secure data transmission in unsecure environments such as the Internet. RSA is widely used for that purpose. I chose this topic because I would like to better understand cryptography.

**Programming language:** Rust

**Language of documentation:** English

**Additional languages that I can peer review:** Python, C, Scala, JS/TS

**Degree programme:** Tietojenkäsittelytieteen kandidaatti (TKT)

## Algorithms and Data Structures

The program will implement the following algorithms:
- RSA encryption algorithm: This algorithm is used to encrypt and decrypt messages.
- Miller-Rabin algorithm for primality testing: This algorithm is used check for primality while generating the keys needed for RSA encryption.
- Extended Eucledian algorithm: Needed for generating the private key by calculating the greatest common divider


## Inputs

The program is a CLI tool that can
- Create a random pair of keys with lengths of 1024 bits.
- Encrypt a message with a given key.
- Decrypt a message with a given key.
 
For encryption and decryption the inputs are a message and a key.

## Time and space complexity

- For the Miller-Rabin test the time complexity is O(k log³ n) where k is the number of iterations used in the test and n is the number being tested.  
- For the Extended Eucledian algorithm the time complexity is O(log(min(a,b))) where a and b are the numbers for which we calculate the greatest common divider.
- Space complexity should be O(1) for all algorithms.

## Sources

- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://people.csail.mit.edu/rivest/Rsapaper.pdf
- https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
- https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

