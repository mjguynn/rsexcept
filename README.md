# rsexcept

Rust's standard library has a function called [`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). The documentation comment states:

> It is **not** recommended to use this function for a general try/catch mechanism. 

So, naturally, I tried using it as a general try/catch mechanism. 

This crate provides a macro, `rsexcept!`, which provides a familiar, type-aware `try`/`catch` syntax. It doesn't work with `panic = "abort"`.

An example which demonstrates how to differentiate between exceptions based on their type:

```rust
use rsexcept::rsexcept;
fn main() {
    static ARR: [&'static str; 5] = ["hey", "this", "is", "a", "array"];
    let res = rsexcept! {
        try {
            std::panic::panic_any(&ARR[1..]);
        }
        catch {
            i32, _ => panic!("Nope!"),
            &[&str], ["uh", s, ..] => s.to_string(),
            &[&str], ["this", h, t @ ..] => {
                format!("{}_{}", h, t[1])
            }
        }
    };
    assert_eq!("is_array", res);
}
```

## Please don't actually use this

I wrote this because I thought it would be funny. Also, if someone is trash talking Rust and says "it doesn't even support exceptions!", you can smugly point them to this repo and brag about Rust's macro system.