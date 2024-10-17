main resoruces
The Little Book of Rust Macros https://veykril.github.io/tlborm/

minor resources
The Why and How of Rust Declarative Macros https://blog.lanesawyer.dev/27848/the-why-and-how-of-rust-declarative-macros

`rustc --pretty expanded -Z unstable-options src/lib.rs --test`

Now that's a bit more complicated than I would prefer for a debugging command, but it makes sense once it's broken down:

rustc is the Rust compiler
In order to use the --pretty expanded flag to preserve spacing after compilation, -Z unstable-options is required
-Z unstable-options requires the nightly compiler (which can be turned on for a single workspace using rustup override set nightly)
src/lib.rs is the name of the file to compile, which is the one I'm writing my code in
--test means to compile the test code, which I needed since my macros are only used in the tests

```rust
macro_rules! test_example {
  (None, $arr:expr, $($func:ident),*) = > {
    $(
      assert_eq!(None, $func($arr)); //when answer is None
    )*
  };
  ($ans:expr, $arr:expr, $($func:ident),*) => {
    $(
      assert_eq!($ans, $func($arr).unwrap()); //when answer is Some
    )*
  }
}
```

something that i must test in another project. Run cargo expand and see what there
https://www.youtube.com/watch?v=CHRNj5oubwc

```rust
struct SUV;
struct Sedan;

trait LandCapable {
  fn drive(&self);
}

//static dispatch?
fn road_trip(vehicle: &impl LandCapable) {
  vehicle.drive();
}
//dynamic dispatch?
fn road_trip2(vehicle: &dyn LandCapable) {
  vehicle.drive();
}
```
