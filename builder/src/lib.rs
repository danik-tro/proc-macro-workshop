use proc_macro::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: command_ident,
        attrs,
        generics,
        data,
        vis,
    } = parse_macro_input!(input as DeriveInput);

    let syn::Data::Struct(struct_data) = data else {
        panic!("Unable to implement for non-struct type.")
    };

    let syn::Fields::Named(fields) = struct_data.fields else {
        panic!("Expected struct with named fields.")
    };

    let field_name = fields
        .named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<&Option<syn::Ident>>>();
    let field_type = fields
        .named
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<&syn::Type>>();

    let builder_ident = &format_ident!("{}Builder", command_ident);

    TokenStream::from(quote! {
        #vis struct #builder_ident {
            #(
                pub #field_name: std::option::Option<#field_type>,
            )*
        }

        impl #builder_ident {
            #(pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = std::option::Option::Some(#field_name);
                self
            })*

            pub fn build(&mut self) -> std::result::Result<#command_ident, Box<dyn std::error::Error + 'static>> {
                std::result::Result::Ok(#command_ident {
                    #(
                        #field_name: self.#field_name.take().expect("Field was not specified."),
                    )*
                })
            }
        }

        impl #command_ident {
            fn builder() -> #builder_ident {
                #builder_ident {
                    #(
                        #field_name: std::option::Option::None,
                    )*
                }
            }
        }
    })
}
