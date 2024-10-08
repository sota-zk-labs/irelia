# Aptos Transaction Verification

This project verifies transaction data from the Ethereum mainnet. It follows a specific flow to verify both Merkle and
FRI transactions, and then registers a continuous page batch and verifies proof and register.

## Transaction Verification Flow

1. **Merkle Transactions Verification**
    - Transaction: `0x3c4a9bd3b7a82494fa5bde9b9c21c15b6a9b1fd71208167a2f6d8112b687423d`
    - Transaction: `0xf8d9b17ecb9f35cd90efcc147434fa35d80d1be684d497008bccf49f085632d2`
    - Transaction: `0x69f36f7556bce1a983a4595bca44214947e12a1f88d0df8d1ec0ca31c391ccbb`

2. **FRI Transactions Verification**
    - Transaction: `0xda89df69c280f0839dd3ea7ecd77eed5e0f6708735d2a2d62389550dbcd3bf92`
    - Transaction: `0xf3aeda8f4870e612f1d15249d1261d05a211b6e5da43eafa467cd7621734103f`
    - Transaction: `0x2393305cc963d4dff7779bcc381345a05ed79ea1d3e7a2cc9390672229342c31`
    - Transaction: `0x682c804e215cc2df323b53cb4591247b07109b3bc558658c48642acf4a4c6a8a`
    - Transaction: `0x2ce4991e628c32e43f02cebc626f6ab2d91e87b6db74081c5054406e72806b1a`
    - Transaction: `0x89165e0b4da556ce987049412d868dfd10e31cb4deb7692a925ec3e0f739c747`
    - Transaction: `0xc5fe89f1713d97dcda70c6ccc5cb6807d68f2b3755f90121c829456f5382510e`

3. **Register Continuous Page Batch**
    - Transaction: `0x71a3856fce577a72030e1af7081fa6fd501c60355a85b58907e758333f6087c2`
    - Transaction: `0x96616b0e2ba4deeadfcb7fd8b8ca1e6023c76207784a22c5e3ec6e2597523bd3`
    - Transaction: `0xc7609607c7c69eae3b160a16c487c1fe5f12796f394e4f5b378027d10aa57e9a`
    - Transaction: `0xf58052724c4027f245553dc3396bd3655368f350715c668c52c52b0a0bf3d70c`

4. **Verify Proof and Register**
    - Transaction: `0x915ad5674e68f8f47b3be85496443abd583fe37f7c3fbe2dce1c0e119f487c08`