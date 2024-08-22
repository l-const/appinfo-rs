
<p align="center">
    <img width="400" alt="Alacritty Logo" src="https://github.com/l-const/appinfo-rs/blob/main/appinfo-logo.png">
    <h5 align="center">A procedural macro to derive application info at compile time, using cargo compile-time env vars and `clap-rs`.</h5>
</p>



### Usage

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


### Details: 
* All the info are retrieved at compile time.
* Default env vars: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
* For more, see here: https://github.com/l-const/appinfo-rs/blob/main/src/cli.rs#L5-L9

### Future Enhacements

- [ ] Validation logic that the macro is applied only in `main`.
- [ ] Override of vars through the macro args.
- [ ] Overridable execution/usage info.
- [ ] Git commit revision using a compile/build time env var.
