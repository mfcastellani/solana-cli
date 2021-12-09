# Solana Cli

Toy project with integration Rust/Solana, created using [this template](https://github.com/hashblock/solana-cli-program-template)

The idea of this project is to be an example of using the methods available in the Solana SDK for Rust in common actions such as balance inquiry, for example.

## Basics

Build and run test environment with our custom program (change command to fit your paths):

```shell
$ cd program

$ cargo build-bpf

$ cd ..

$ ls target/deploy/solana_cli_template_program_bpf.so

$ solana-test-validator --bpf-program SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv \
> ~/Development/autonomynetwork/solana-cli/target/deploy/solana_cli_template_program_bpf.so \
> --ledger ~/Development/autonomynetwork/solana-cli/.ledger --reset

$ solana config set --url localhost

$ solana-keygen new -o test.json 

$ solana airdrop --keypair test.json 100
```

## Accounts

The repo includes sample keys for `owners`. There are two owner accounts predefined **User1** and **User2**, each with two keypairs defined:

1. An owners 'wallet' account. This is funded by the Solana configurations default account and is used to fund program accounts.
2. An owners program 'account' which is used by the sample program for mint, transfer and burn operations

For more details check the `keys` folder.

## Commands

All commands accept the `--verbose` flag

### Help

```shell
$ cargo run -- help
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/solana_cli help`
solana_cli 0.1.0


USAGE:
    solana_cli [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show additional information

OPTIONS:
    -C, --config <PATH>        Configuration file to use [default: .../config.yml]
    -u, --url <URL>            JSON RPC URL for the cluster [default: value from configuration file]
        --keypair <KEYPAIR>    Filepath or URL to a keypair [default: client keypair]

SUBCOMMANDS:
    balance     Get balance
    burn        Burn (delete) a key/value pair from an account
    help        Prints this message or the help of the given subcommand(s)
    mint        Mint a new key/value pair to an account
    ping        Send a ping transaction
    transfer    Transfer a key/value pair from one account to another
```

### Ping

Creates a transaction sending 0 SOL from the signer's account to the signer's account. Returns the signature of the transaction.

```shell
$ cargo run -- ping --verbose --keypair test.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/solana_cli ping --verbose --keypair test.json`
JSON RPC URL: http://localhost:8899
Signature: 5JFyLrLBrQrPyX2csoYxhBs5Ux1WHko7LzPRxiG17JHgpSDHjjGvyCDPb9bEyw39Hj7QWj6szLqH2y4RjE3ZqzP9
```

### Balance

Returns an account's balance.

```shell
$ cargo run -- balance --verbose --keypair test.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/solana_cli balance --verbose --keypair test.json`
JSON RPC URL: http://localhost:8899
FVFK8Ttx1zAKfNWnKh5vpoPpfsdxBSiRsGSqNnUqM8Tm has a balance of ◎99.999995000
```

### Mint

Mint a key/value pair to an owning account.

```shell
$ cargo run -- mint --verbose -t User1 -k TheKey --value TheValue          
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/solana_cli mint --verbose -t User1 -k TheKey --value TheValue`
JSON RPC URL: http://localhost:8899
User1 to account key/value store {"TheKey": "TheValue"}
```

### Transfer

Transfer a key, and it's value, from one owning account to another.

```shell
$ cargo run -- transfer --verbose -f User1 -t User2 -k TheKey
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/solana_cli transfer --verbose -f User1 -t User2 -k TheKey`
JSON RPC URL: http://localhost:8899
User1 from account key/value store {}
User2 to account key/value store {"TheKey": "TheValue"}
```

### Burn 

Burn (delete) a key, and it's value, from an owning account.

```shell
$ cargo run -- burn -f User2 -k TheKey                       
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/solana_cli burn -f User2 -k TheKey`
User2 from account key/value store {}
```

## Tests

```shell
$ cargo test
```
