# gnosispay-cli

This command-line tool exports GnosisPay transactions to CSV for easy import into [iCompta](https://www.icompta-app.fr/) or other personal finance software.

It can also monitor on-chain Transfer() events and send alerts via [Pushover](https://pushover.net/).

Only EURe v2 is supported.

## Authors

- [@cizeon](https://github.com/Cizeon)

## Install

```bash
cargo install --path .
```

## Usage

Make sure to set the environment variables or pass them as command-line arguments.

```bash
export WALLET_ADDRESS=
export SESSION_TOKEN=
export GNOSISSCAN_API_KEY=
```

```bash
-=[ gnosispay-cli v0.1.0 ]=-

Usage: gnosispay-cli [OPTIONS] <COMMAND>

Commands:
  list
  export
  verify
  monitor
  help     Print this message or the help of the given subcommand(s)

Options:
      --verbose
  -h, --help     Print help
  -V, --version  Print version
```

### List transactions

```bash
-=[ gnosispay-cli v0.1.0 ]=-

Usage: gnosispay-cli list --gnosisscan-api-key <GNOSISSCAN_API_KEY> --wallet-address <WALLET_ADDRESS> --session-token <SESSION_TOKEN>

Options:
      --gnosisscan-api-key <GNOSISSCAN_API_KEY>  [env: GNOSISSCAN_API_KEY=]
      --wallet-address <WALLET_ADDRESS>          [env: WALLET_ADDRESS=]
      --session-token <SESSION_TOKEN>            [env: SESSION_TOKEN=]
  -h, --help                                     Print help
```

### Verify

Simply checks if your balance is correct.

```bash
$ gnosispay-cli verify
-=[ gnosispay-cli v0.1.0 ]=-

Total: 123.06
```

### Export transactions to CSV

```bash
$ gnosispay-cli export ~/Downloads/gnosis-transactions.csv
-=[ gnosispay-cli v0.1.0 ]=-

[+] CSV export to `/Users/user/Downloads/gnosis-transactions.csv`.
```

### Monitor

Monitor on-chain events and send notifications via [Pushover](https://pushover.net/).

Make sure to set the environment variables or pass them as command-line arguments.

```bash
export WALLET_ADDRESS=
export PUSHOVER_USER=
export PUSHOVER_TOKEN=
```

```bash
-=[ gnosispay-cli v0.1.0 ]=-

Usage: gnosispay-cli monitor [OPTIONS] --wallet-address <WALLET_ADDRESS> --pushover-user <PUSHOVER_USER> --pushover-token <PUSHOVER_TOKEN>

Options:
      --wallet-address <WALLET_ADDRESS>
          [env: WALLET_ADDRESS=]
      --rpc-url <RPC_URL>
          [env: ETH_RPC_URL=] [default: wss://rpc.gnosischain.com/wss]
      --pushover-user <PUSHOVER_USER>
          [env: PUSHOVER_USER=]
      --pushover-token <PUSHOVER_TOKEN>
          [env: PUSHOVER_TOKEN=]
  -h, --help
          Print help
```

### Kubernetes

gnosispay-cli monitor can be run from Docker and Kubernetes.
Make sure to have your .env set.

```bash
docker build .. -t gnosispay-monitor --platform linux/amd64
```

```bash
kubectl apply -f pod.yaml
```
