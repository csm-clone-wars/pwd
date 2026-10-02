# pwd
A reimplementation of the Linux command pwd (Print Working Directory) in Rust.

## Overview
`pwd` displays the full path of the directory you are currently in.

Example:
```bash
pwd
```

```bash
/home/csm/Documents
``` 

## Development Environment
- Operating System: Linux Mint
- Rust Toolchain

## Building the Project
- clone the repo
- cd into the cloned repo
- to build:
```
cargo build --release
```
- to build and run:
```
cargo run -- -help
```



## Commands
```
    pwd <no args>        -> Print working directory logical path
    pwd -version,  -v    -> Print pwd version
    pwd -help,     -h    -> Print pwd commands list
    pwd -logical,  -l    -> Print working directory logical path
    pwd -physical, -p    -> Print working directory physical path (resolving sym-links)
```

## License
MIT