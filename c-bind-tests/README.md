# Testing the C-Bindings

Poseidon C-bindings allow hash functions from other languages.
It is used notably in geth, so we choose to test the bindings through golang.

You can launch the tests from the tests directory (c-bind-tests):
```bash
go test -v
```
or from the project's root directory:
```bash
(cd c-bind-tests && go test -v)
```
