mod spanned;

#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    spanned::derive_spanned(input)
}
