use proc_macro::TokenStream;
use syn::ItemEnum;

mod simple_enum;

#[proc_macro_derive(SimplifiedEnum)]
pub fn simple_enum(input: TokenStream) -> TokenStream {
    let ast: ItemEnum = syn::parse(input).expect("SimplifiedEnum can be only be derived for enums");
    simple_enum::impl_wrapped_macro(&ast)
}


/*mod spanned_token;

#[proc_macro_derive(SpannedToken)]
pub fn spanned_token(input: TokenStream) -> TokenStream {
    let ast: ItemEnum = syn::parse(input).expect("SpannedToken can be only be derived for enums");
    spanned_token::impl_wrapped_macro(&ast)
}*/
