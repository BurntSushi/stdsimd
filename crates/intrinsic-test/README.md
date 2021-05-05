Generate and run programs using equivalent C and Rust intrinsics, checking that
each produces the same result from random inputs.

# Usage
```
USAGE:
    intrinsic-test [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --cppcompiler <CPPCOMPILER>    The C++ compiler to use for compiling the c++ code [default: clang++]
        --toolchain <TOOLCHAIN>        The rust toolchain to use for building the rust code [default: nightly]

ARGS:
    <INPUT>    The input file containing the intrinsics
```

The intrinsic.csv is the arm neon tracking google sheet (https://docs.google.com/spreadsheets/d/1MqW1g8c7tlhdRWQixgdWvR4uJHNZzCYAf4V0oHjZkwA/edit#gid=0)
that contains the intrinsic list. The done percentage column should be renamed to "enabled".

