# Withargs

`withargs` provides an attribute macro in order to make writing simple CLI programs easier. It lets you
write a main function that takes in parameters, and the macro takes care of all the boilerplate necessary
to read in the command line arguments and parse them to the specified type. As long as the type you expect
implements `FromStr`, you're good!

```rust
use withargs::withargs;

#[withargs]
fn main(a: i32, b: i32) {
    println!("Your total is {}", a + b);
}
```

# Goals for this project

- [x] Support for any number of arguments in the main function
- [x] Support for custom types
- [ ] Support for `Option<T>`
- [ ] Support for named arguments in any order with short (`-n`) or long (`--name`) flags
- [ ] Support for arguments with a default value
- [ ] Support for passing arguments via shell pipes
- [ ] Support for `bool` arguments using only a matching flag to use non-default value

# Why make this?

This is mainly a means of learning proc macros in Rust, but I wanted to build something that could potentially
become something useful. Where I see this fitting into the Rust ecosystem is for when you want to build very
simple CLI programs that something like [`clap`](https://github.com/clap-rs/clap) would be overkill for. This
project will not be everything that `clap` is, and nor should it.

# Credit where credit is due

This project borrows _many_ of the ideas from the [`fncli`](https://github.com/vidhanio/fncli) crate.
