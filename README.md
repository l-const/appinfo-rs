# appinfo-rs
Macro to derive appinfo at compile time using cargo compile-time env vars and `clap-rs`.

* https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

## Usage

```rust
#[appinfo:main]
fn main(){
    // my wonderful application
    println!("hello world")!
    cli.do_stuff()
    server.listen(3000)
}
```

##### Add clap:
```bash
cargo add clap
```
##### Dependency:
* clap-rs: (https://docs.rs/clap/latest/clap/)

then in a terminal do:

```bash
Example:
❯ ./wonderful-cli -h
wonderful-cli (version: 0.1.0, commit revision: 0.1.0, repository: https://github.com/l-const/wonderful-cli) - A cli app that utilises the appinfo macro.
```
Info: All the info are retrieved at compile time.

### Future Enhacements

- [ ] Validation logic that the macro is applied only in `main`.
- [ ] Override of vars through the macro args.
- [ ] Overridable execution/usage info.
- [ ] Git commit revision using a compile/build time env var.
