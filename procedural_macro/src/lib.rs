use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Lit, Expr};
// use facet::Facet;

#[proc_macro_derive(GenerateStruct)]
pub fn generate_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn new() -> Self {
                Self {
                    id: 0,
                    name: String::from("John Doe"),
                    active: false,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        fn hello_world() {
            println!("Hello, world!");
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Foo, attributes(foo))]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // Extract attributes
    let attrs: Vec<String> = input.attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("foo") {
                Some(attr.path().get_ident()?.to_string())
            } else {
                None
            }
        })
        .collect();

    let expanded = quote! {
        impl Foo for #name {
            fn get_attributes(&self) -> Vec<&'static str> {
                vec![#(#attrs),*]
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Repeat, attributes(count))]
pub fn derive_repeat(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // Extract count attribute
    let count = input.attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("count") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(expr_lit) = &meta.value {
                        if let Lit::Int(lit) = &expr_lit.lit {
                            lit.base10_parse::<usize>().ok()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(1);

    let expanded = quote! {
        impl Repeat for #name {
            fn repeat(&self) {
                for i in 0..#count {
                    println!("Repeating {} times: {}", #count, i + 1);
                }
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(CountFields)]
pub fn count_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn count_fields() -> usize {
                #name::SHAPE.fields().len()
            }
        }
    };

    TokenStream::from(expanded)
}