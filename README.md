# sigop-rs

A CLI tool to optimize your Solidity function signatures. I wanted to create this after seeing
[transmissions11](https://github.com/transmissions11)'s comment about this optimization.

[Inspired by the great work of emn178](https://github.com/emn178/solidity-optimize-name).

## 🧪 How does it work ?

The optimizer takes a function signature such as `myFunction(address)` and tries to combine it with
a suffix generated from a dictionary.

For each combination, the 4-bytes function selector is computed and verified : if it contains a
specified number of zeros at the beginning, the optimization has been found.

## 🚀 Getting started !

### ⚙️ Installation

Building from source:
```shell
cargo build --release --all-features
```

### 🏁 Quickstart

```shell
env RUST_LOG=info ./target/release/sigop-cli -s "myAwesomeFunction(address)"
```

Which should print:
```shell
[2022-07-02T13:54:59Z INFO  sigop_cli] Found this optimization: myAwesomeFunction_Gh5(address)
```

Using `cast`, we can see the optimized function selector:
```shell
$ cast sig "myAwesomeFunction_Gh5(address)"
0x0000983d
```

### ✏️ Custom parameters

You can specify custom parameters used by the optimizer:
1. `level`: The maximum size of the suffix following the original function name.
2. `target`: The number of zero-bytes you want to have at the beginning of the optimized function
selector.

## 🤖 Author

Made with ❤️ by 🤖 [Luca Georges François](https://github.com/0xpanoramix) 🤖
