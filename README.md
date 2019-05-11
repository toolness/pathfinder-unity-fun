This is an experiment in building a C API for the Canvas API portion of [Pathfinder][].

For more details on the context behind this, see 
[Pathfinder issue #147](https://github.com/pcwalton/pathfinder/issues/147).

## Quick start

Right now all we have is an extremely simple dynamic library that can be
called from other languages via FFI.

```
git submodule init
git submodule update
cargo build
cargo test
python3 test_dylib.py
```

This will just build the dynamic library, call it from Python 3, and
print a success message.

[Pathfinder]: https://github.com/pcwalton/pathfinder
