use quote::quote;
use proc_macro_hack::proc_macro_hack;

use tys_counting_fsm_for_ty_tt_cluster_seq::TysCountingFSMForTyTtClusterSeq;

// TODO: Switch out from proc_macro_hack and edit documentation when
// "error: procedural macros cannot be expanded to expressions" is resloved.
// note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
#[proc_macro_hack]
/// Returns the count of [`:ty`]s in the given [`TokenStream`]
/// as a constant expression of type usize
///
/// # Arguments
/// 
/// * `input` - A [`TokenStream`] in which [`:ty`]s must be counted
///
/// # Example
///
/// ```
/// extern crate proc_macro_hack;
/// use proc_macro_hack::proc_macro_hack;
/// #[proc_macro_hack]
/// use count_tts::count_tys;
/// 
/// // It not necessarily must be a struct, it could be a generic
/// // Read more about macro_rules! here:
/// // <https://doc.rust-lang.org/rust-by-example/macros.html>
/// macro_rules! declare_variadic_struct {
///    ($struct_name:ident, <$($ty:ty),*>) => {
///         struct $struct_name {
///             // fields
///         }
/// 
///         impl $struct_name {
///             pub const fn count() -> usize {
///                 // count_tys!() can be used in an expression and even
///                 // const expression context
///                 // unlike macros without proc_macro_hack
///                 // note: see issue #54727
///                 // <https://github.com/rust-lang/rust/issues/54727>
///                 // for more information.
///                 count_tys!($($ty:ty)*)
///             }
///         }
///     };
/// }
/// 
/// // declare_variadic_struct!(VariadicStruct, <usize, usize, usize>);
/// // expands into the following:
/// //
/// // struct VariadicStruct {
/// //      // fields
/// // }
/// //
/// // impl VariadicStuct {
/// //      pub const fn count() -> usize {
/// //          3usize
/// //      }
/// // }
/// declare_variadic_struct!(VariadicStruct, <usize, usize, usize>);
/// assert_eq!(VariadicStruct::count(), 3);
/// ```
/// 
/// [`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
/// [`TokenTree`]: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html
/// [`:ty`]: https://doc.rust-lang.org/rust-by-example/macros/designators.html
pub fn count_tys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// Unlike the following macro by example when applied to $($tys:ty),* [`pattern`],
// 
// ```
// // https://danielkeep.github.io/tlborm/book/blk-counting.html
// macro_rules! count_tts {
//    () => {0usize};
//    ($_head:tt $($tail:tt)*) => {1usize + count_tts!($($tail)*)};
// }
// ```
// 
// the following implementation does not produce heavy [`TokenStream`] of "1usize + 1usize...".
// and does not treat composite [`designators`](such as :expr) as one ( [`TokenTree`] )
// 
// [`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
// [`TokenTree`]: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html
// [`designator`]: https://doc.rust-lang.org/rust-by-example/macros/designators.html
// [`pattern`]: https://doc.rust-lang.org/rust-by-example/macros/designators.html
// 
    let tys_counting_fsm_for_ty_tt_cluster_seq = TysCountingFSMForTyTtClusterSeq::<usize>::new();
    let tys_count :usize = unsafe {
        tys_counting_fsm_for_ty_tt_cluster_seq
            .unsafe_into_iter_transition(proc_macro2::TokenStream::from(input))
        }.get_count();
    let expanded_tt  = quote!{#tys_count};
    proc_macro::TokenStream::from(expanded_tt)
}