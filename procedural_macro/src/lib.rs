use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta};
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
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world(&self) {
                println!("Hello, world!");
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Foo, attributes(foo, bar))]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Extract attribute values
    let attrs: Vec<String> = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("foo") || attr.path().is_ident("bar") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(expr_lit) = &meta.value {
                        if let Lit::Str(lit) = &expr_lit.lit {
                            Some(lit.value())
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
    let count = input
        .attrs
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

#[proc_macro_derive(MyMagic)]
pub fn my_magic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn my_metadata() {
                let shape = #name::SHAPE;
                println!("Struct fields: {:?}", shape);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(MyMagicDescription)]
pub fn my_magic_description(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data
    {
        &fields.named
    } else {
        panic!("MyDupa only supports structs with named fields");
    };

    let describe_lines = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            println!("{}: {:?}", stringify!(#name), self.#name);
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn describe(&self) {
                #(#describe_lines)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(RetryCalculation, attributes(calculation))]
pub fn derive_calculation_helper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut calc_count = 1usize;
    let mut found_calc = false;

    for attr in &input.attrs {
        if attr.path().is_ident("calculation") {
            found_calc = true;
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Int(lit_int) = &expr_lit.lit {
                        if let Ok(val) = lit_int.base10_parse::<usize>() {
                            calc_count = val;
                        } else {
                            return syn::Error::new_spanned(
                                lit_int,
                                "calculation value must be a positive integer",
                            )
                            .to_compile_error()
                            .into();
                        }
                    } else {
                        return syn::Error::new_spanned(
                            expr_lit,
                            "calculation value must be a numeric literal",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
        }
    }

    if !found_calc {
        return syn::Error::new_spanned(struct_name, "missing #[calculation = N] attribute")
            .to_compile_error()
            .into();
    }

    let expanded = quote! {
        impl Calc for #struct_name {
            fn calc_count(&self) -> usize {
                #calc_count
            }
        }
    };

    TokenStream::from(expanded)
}
