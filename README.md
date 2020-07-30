# wttr cli
a cli app to interact with https://wttr.in/ via the CLI -- super bare-bones

## installing
```sh
# make sure you have the rust toolchain installed
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/fyko/wttr
```

## meta
```xl
CLI tool for listing the weather

USAGE:
    wttr [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     Wether or not to display a prettier view
    -s, --short      Wether or not to display a smaller view
    -V, --version    Prints version information
```  

## examples
```sh
wttr --pretty
wttr --short
wttr -V
```