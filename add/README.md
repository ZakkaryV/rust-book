# Workspaces
`add/` is a Workspace: a Cargo construct for defining sets of binary / library moduels and their given relationships. 

A Workspace's members are self-contained modules with their own manifests. Running `cargo build` in the root of the Workspace will build all constituent crates into the single `workspace/target/` directory rather than building each one separately. This reduces compile time and redundancy. It is necessary to declare a module's dependencies on other adjacents under `[dependencies]`.

Use the `-p` flag with `cargo bulid` & `cargo run` to consume a specific sub-module, ie `cargo run -p adder` (in the Workspace root).

Crates have their own `Cargo.toml`, but share a `Cargo.lock` in the Workspace root. This ensures all crates depending on some specific version of a module will all use that single version. 

This particular workspace contains 2 libraries, where B depends on A, and 1 binary who depends on both libraries. 

# Testing
Run `cargo test` in the Workspace root.

```
running 1 test
test tests::it_adds_one ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-dcb1c859a41d7204

running 1 test
test tests::adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-a8fddb5bf5cf2fb5

```

