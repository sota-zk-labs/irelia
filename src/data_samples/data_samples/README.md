# Aptos Transaction Verification

This project verifies transaction data from the Ethereum mainnet. It follows a specific flow to verify both Merkle and
FRI transactions, and then registers a continuous page batch and verifies proof and register.

## Transaction Verification Flow

1. **Merkle Transactions Verification**
    - Transaction: `0xc4395419f01833b0ea55b668ce3fbfa019a79ed9054527ded731a5b0fb98fdf0`
    - Transaction: `0xf60174710760e819d6a1db3eaccafc20791baeda1acf52101cdbe4c07c9fde0a`
    - Transaction: `0x76754931cd660d8a71acda44724b0f496f1e31529c05b10a780ceac8f2bedabb`

2. **FRI Transactions Verification**
    - Transaction: `0xbbae532038258a0c780620d9946959adf9013884812a8267838be42a311fd01b`
    - Transaction: `0xf7b7fd62c448b14d643ef498013ec6a110c9d0fe21a3b585b5786f261d0a5bfe`
    - Transaction: `0xff3729f5f506d1f3c8a5e8225d1ba9ff404b605628014730a6f87a84cef7fc05`
    - Transaction: `0xf4f0d3ffca3b135b7fdc2ef932e705e69d55e346e46039a6b7c8eefca7a8c24b`
    - Transaction: `0xbe3207e7d7d9d4b04aa9992e5243a8754fc48e0cf5ea6849260f1623cd6163c0`
    - Transaction: `0x678c6cc0734710337a43a0d43083723fedd477bb9e3fedde7185aa898ccef9d1`
    - Transaction: `0xdf9ce584d4591d38f526c72cf1467c83a5c3fd26387efd6732f8da4bda5705bc`

3. **Register Continuous Page Batch**
    - Transaction: `0x55833a4cd9a46f1a387a06fba81a659fdc05f19667842cabfb36da8419b16067`
    - Transaction: `0xdb3454c6514283f049344ac26690744bb581b029454ef98e08ee2f520c13daec`
    - Transaction: `0x3208ee4d195f48f59995e254a68cb861230eea2f5be9af7e4647629a10cd933a`
    - Transaction: `0x15e2c6ab1e39491acb6a0ef9d9ebacfb9f3f019a18e23ea2296a50a0cce3dde6`

4. **Verify Proof and Register**
    - Transaction: `0x386990f7b196a03eabb981de805e7c15baebd82642974517ac521a68ddb880d0`