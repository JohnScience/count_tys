# count_tys!() function-like macro

Returns the count of comma-delimited [`:ty`]s (types) in the given [`TokenStream`]
as a constant expression of type [`usize`]

## Arguments

* `input` - A [`TokenStream`] in which comma-delimited [`:ty`]s (types) must be counted

## Examples

### Basic usage

```rust
// count_tys!($($ty:ty),*)
```

### More complete example

#### Cargo.toml

1.  [package]
2.  name = "example"
3.  version = "0.1.0"
4.  authors = ["Dmitrii - Demenev <demenev.dmitriy1@mail.ru>"]
5.  edition = "2018"
6.
7.  [dependencies]
8. proc-macro-hack = "0.5"
9. count-tys = "0.1"

#### main.rs 

```rust
extern crate proc_macro_hack;
use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
use count_tts::count_tys;

// It not necessarily must be a struct, it could be a generic
// Read more about macro_rules! here:
// <https://doc.rust-lang.org/rust-by-example/macros.html>
macro_rules! declare_variadic_struct {
   ($struct_name:ident, <$($ty:ty),*>) => {
        struct $struct_name {
            // fields
        }

        impl $struct_name {
            pub const fn count() -> usize {
                // count_tys!() can be used in an expression and even
                // const expression context
                // unlike macros without proc_macro_hack
                // note: see issue #54727
                // <https://github.com/rust-lang/rust/issues/54727>
                // for more information.
                count_tys!($($ty:ty),*)
            }
        }
    };
}

// declare_variadic_struct!(VariadicStruct, <usize, usize, usize>);
// expands into the following:
//
// struct VariadicStruct {
//      // fields
// }
//
// impl VariadicStuct {
//      pub const fn count() -> usize {
//          3usize
//      }
// }
declare_variadic_struct!(VariadicStruct, <usize, usize, usize>);
assert_eq!(VariadicStruct::count(), 3);
```

[`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
[`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
[`TokenTree`]: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html
[`:ty`]: https://doc.rust-lang.org/rust-by-example/macros/designators.html
