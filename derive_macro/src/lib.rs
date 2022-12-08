use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[proc_macro_derive(EnumDispatchIter)]
pub fn proc_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    impl_derive(input).into()
}

fn impl_derive(input: TokenStream) -> TokenStream {
    let Ok(enum_input) = syn::parse2::<syn::ItemEnum>(input) else {
        return quote!{compile_error!("Not an enum")};
    };
    let ident = enum_input.clone().ident;
    let variants = enum_input
        .variants
        .into_iter()
        .map(|v| {
            let i = v.ident;
            match v.fields {
                syn::Fields::Unnamed(_) => {
                    quote!(#i(#i::default()))
                }
                _ => quote!(compile_error!("Variant must be have a tuple field with the same name, custom names are not handled yet")),
            }
        })
        .collect::<Vec<_>>();

    let into_iter_fn = into_iter_func(&ident, &variants);

    quote! {
        impl EnumDispatchIter for #ident {
            #into_iter_fn
        }
    }
}

fn into_iter_func(ident: &Ident, variants: &[TokenStream]) -> TokenStream {
    quote! {
        fn into_iter() -> std::vec::IntoIter<Self> {
            vec![
                #(#ident::#variants),*
            ].into_iter()
        }
    }
}
