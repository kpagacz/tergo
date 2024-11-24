# tergo

`tergo` (lat. *to clean*) is a command line program that formats R code.

## Installation

### `cargo`

If you have [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
installed, then you can simply run:

```bash
cargo install tergo
```

, and the newest version of `tergo` is going to be installed on your machine.

### From source

If you want to install from source, you will need to have
[`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
installed on your machine.

1. Clone the repository:

    ```bash
    git clone https://github.com/kpagacz/tergo
    ```

2. Install the binary via cargo:

    ```bash
    cargo install --path ./tergo
    ```

`tergo` will be available after these steps.

## Usage

Run:

```bash
tergo --help
```

For `tergo`'s manual.

## Configuration

You can configure `tergo` via a `tergo.toml` file.
See [`tergo-lib` README](../balnea/README.md) or
[`tergo-lib` documentation](https://docs.rs/tergo-lib/latest/tergo_lib/struct.Config.html)
for more details about possible configuration keys and values.
