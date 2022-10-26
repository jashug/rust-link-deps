# Local Link Dependencies

This crate adds the relative path `link-deps/<target-triple>` to the linker library search path.

Add it as a dev-dependency if you have examples that need to be linked, or add it as a dependency if you are building a binary. If included as a dependency, you probably don't want to publish to `crates.io`, since every dependant will inherit the linker library search path.

## Storing `link-deps` in version control

The contents of `link-deps` are mostly binary files, sometimes large (GNU shared objects include code, and static libraries do too).

It might make sense to commit `link-deps` for binary crates and not for library crates, similar to `Cargo.lock`.
