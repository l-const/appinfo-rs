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
then in a terminal do:

```bash
app -h

or

app -v
```
All the info are retrieved at compile time.

### Dependency:
* clap-rs: (https://docs.rs/clap/latest/clap/)

Install clap:
```bash
cargo add clap
```


### Future Enhacements

- [ ] Validation logic that the macro is applied only in `main`.
- [ ] Override of vars through the macro args.
