This crate serves as a test for no_std compatibility.

In order to test if the crate compiles, do

```bash
cargo rustc -- -C link-arg=-nostartfiles
```
