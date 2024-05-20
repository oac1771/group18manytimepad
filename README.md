# group18manytimepad - Cryptography 

## Approach to solve many time pad
1. Finding the length of the key - We can find the length of the key by xoring the ciphertexts with each other. The key length will be the gcd of the lengths of the ciphertexts.
2. Finding the key - We can find the key by xoring the ciphertexts with the plaintexts. The key will be the xor of the ciphertext and the plaintext.
3. Decrypting the ciphertexts - We can decrypt the ciphertexts by xoring the ciphertexts with the key.
