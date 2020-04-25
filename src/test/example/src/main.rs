use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
use count_tys::count_tys;

macro_rules! declare_variadic_struct {
    ($struct_name:ident, <$($ty:ty),*>) => {
         struct $struct_name {
             // fields
         }
 
         impl $struct_name {
             pub const fn count() -> usize {
                 // count_tys!() can be used in an expression and even in a
                 // const expression context
                 // unlike macros without proc_macro_hack
                 // note: see issue #54727
                 // <https://github.com/rust-lang/rust/issues/54727>
                 // for more information.
                 count_tys!($($ty),*)
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
 
fn main() {
    assert_eq!(VariadicStruct::count(), 3);
}
