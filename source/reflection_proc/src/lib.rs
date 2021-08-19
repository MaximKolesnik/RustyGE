extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn reflect(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemStruct);

    let ref name = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let output = quote!{
        #item
        impl #impl_generics reflection::Reflected for #name #ty_generics #where_clause {}
    };
    TokenStream::from(output)
}
