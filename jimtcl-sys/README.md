# jimtcl-sys

A -sys crate that wraps the compilation of jimtcl C code.

Currently, we require that you compile with clang. And *libclang* C libraries
are a hard dependency. Use your system package manager for these.

```sh
export CC=clang
```

