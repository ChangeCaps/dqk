use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Generics};

pub fn derive_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let span = span(input.data);

    add_generic(&mut input.generics);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics crate::Spanned for #name #ty_generics #where_clause {
            fn span(&self) -> crate::Span {
                #span
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_generic(generics: &mut Generics) {
    for generics in generics.type_params_mut() {
        generics.bounds.push_value(parse_quote!(crate::Spanned));
    }
}

fn span(data: Data) -> TokenStream {
    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named) => {
                let fields = named.named.iter().map(|field| {
                    let name = field.ident.as_ref().unwrap();

                    quote!(&self.#name)
                });

                impl_span(fields)
            }
            Fields::Unnamed(unnamed) => {
                let mut index = 0u32;

                let fields = unnamed.unnamed.iter().map(|_| {
                    let ident = Ident::new(&index.to_string(), Span::call_site());
                    index += 1;

                    quote!(&self.#ident)
                });

                impl_span(fields)
            }
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|variant| {
                let name = &variant.ident;

                match variant.fields {
                    Fields::Named(ref fields) => {
                        let idents = fields
                            .named
                            .iter()
                            .map(|field| {
                                let name = &field.ident.as_ref().unwrap();
                                quote!(#name)
                            })
                            .collect::<Vec<_>>();

                        let span = impl_span(idents.iter().cloned());

                        quote! {
                            Self::#name { #(#idents),* } => #span
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let mut index = 0;

                        let idents = fields
                            .unnamed
                            .iter()
                            .map(|_| {
                                let ident = Ident::new(&format!("i{}", index), Span::call_site());
                                index += 1;

                                quote!(#ident)
                            })
                            .collect::<Vec<_>>();

                        let span = impl_span(idents.iter().cloned());

                        quote! {
                            Self::#name(#(#idents),*) => #span
                        }
                    }
                    Fields::Unit => unimplemented!(),
                }
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        _ => unimplemented!(),
    }
}

fn impl_span(fields: impl Iterator<Item = TokenStream>) -> TokenStream {
    quote! {
        #(crate::Spanned::span(#fields)) | *
    }
}
