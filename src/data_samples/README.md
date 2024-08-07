# Aptos Transaction Verification

This project verifies transaction data from the Ethereum mainnet. It follows a specific flow to verify both Merkle and FRI transactions, and then registers a continuous page batch and verifies proof and register.

## Transaction Verification Flow

1. **Merkle Transactions Verification**
    - Transaction: `0x2d14f689b7a2244699e785403ee0fa293db20c74a8f978b56fd25665a376ea49`
    - Transaction: `0x4f3bb2102615a71e410090b23ec8769974878e9a328353ec1cd6617e2806c146`
    - Transaction: `0x528d9024a5d7e61c8fd51d87237ee01f0664850681817c8cec00cc0e2e47d874`

2. **FRI Transactions Verification**
    - Transaction: `0xc5d29deff5517746633b0743beef9b83745267cf1531a042161fd09fcd112385`
    - Transaction: `0x036780250a269d44c7c44b606b36c7bf584d5509dd1b1aaea9e889fa8135ab6b`
    - Transaction: `0x2e8cda13e5692510975a3e2c7bcdef59bd5c5d3b17d568b706d93a7c5fd134a5`
    - Transaction: `0x6437047e179caf0fc4c05828537c173050421b9b380e78042562967e2e234b9d`
    - Transaction: `0x9ceb2c3c096f3d97c827722296bce8f37a76f55841657838ad0c0af46972e380`
    - Transaction: `0xc10232525a32d2d7ae130076996cf3762b22edaa7f13d150d291f7c240d44439`
    - Transaction: `0x20e2922b60c6b770671e077cc187090671fba9f9a7762ff54ecc954775884577`

3. **Register Continuous Page Batch**
    - Transaction: `0x6f59bed6f3df4b87c03c49f11e627e842ae5708a3670f428ddfb83c5b98d3754`

4. **Verify Proof and Register**
    - Transaction: `0xd09a9f322a9241e949ed6d70056d40024a3a06dba6f1401d40d2c44083b491da`