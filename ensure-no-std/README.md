# Testing no_std compilation

This crate serves as a test for no_std compatibility.

In order to test if the crate compiles, do

```bash
cargo rustc --target thumbv7em-none-eabihf
```

Here you can choose another target that does not include std or alloc.
You need to install the target on your system, for example:

```bash
rustup target add thumbv7em-none-eabihf
```

You can see your installed targets by using:

```bash
rustup show
```
