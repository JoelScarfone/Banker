# A Quick Bank

### Usage

I used Rust 1.58.1, with basic libraries such as csv and serde to parse and output csv files. I also used clap to parse command line arguments, although there is only one right now with no other options, flags, or features. 

```
cd banker
cargo run -- ./examples/basic_transactions.csv
```

##### Testing

```
cargo test
```

There are only some basic unit tests. If we want integration tests, we should create libraries to use and have integration tests for those. The last testing phase would be to use the binary, i put two csv examples in `banker/examples` that show a variety of different cases.

### Assumptions
I made quite a few assumptions basing on some of the things read in the problem scope.
* You can only dispute a transaction that was a debit. It does not make sense to reduce someone's account balance because they disputed a withdrawal. To handle this, we need different types of disputes that the problem does not allude to or clarify.
* Based on the previous point, the account balance can not be negative. If we try to dispute a claim after we have already withdrew the funds, nothing will happen.
* locked accounts can still be used. It would be straight forward to make all transactions no-ops and have some reporting/error logged, but due to the lack of clarity on what we do in different error cases I chose to let the accounts still be used if they are locked.

### Implementation

The implementation is fairly straight forward. It starts with using [clap](https://github.com/clap-rs/clap) to parse the input file from the command line. This is overkill, but should the program expand this will be used heavily. We then stream the transactions and deserialize each one. During deserialization, we modify floats to be u64s (we can change this to signed integers should the program evolve and need negatives), so that we can maintain the floating point value to 4 digits precisely. This puts a limit on the account size (which is `2 ^ 64 / 10000`). The program does not handle overflows for simplicity, but they are acknowledged. Modifying back to a float is done during serialization later while outputting account balances.

For the logic of determining account states, we use an `Account` struct. `Accounts` are stored in a `Bank` and modified through `Transactions`. `Bank`s also store historical transactions that are debits to later dispute. `Banks` also store current disputed `Transactions` to later be resolved. There are many no-op cases where invalid states can occur. Ideally these would be tracked, reported, monitored, and logged but for simplicity they are just no-ops. 

