use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(attr.into(), item.into())
        .map_or_else(|err| err.to_compile_error(), std::convert::identity)
        .into()
}

mod entry {
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::{parse::Result, spanned::Spanned, Error, FnArg, ItemFn, Signature};

    pub(crate) fn main(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
        if !attr.is_empty() {
            return Err(Error::new(attr.span(), "Unexpected attribute argument(s)"));
        }

        let func = syn::parse2(item)?;

        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = &func;

        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            inputs,
            variadic,
            output,
            ..
        } = &sig;

        if variadic.is_some() {
            return Err(Error::new(variadic.span(), "Unexpected variadic argument"));
        }

        let var_bindings = inputs
            .iter()
            .map(parse_fn_arg)
            .collect::<Result<Vec<_>>>()?;

        let exit_fn = quote!(
            fn exit(msg: &str) -> ! {
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        );

        let too_many_args = format!(
            "Error: too many arguments provided (Expected {})",
            inputs.len()
        );

        Ok(quote_spanned!(func.span()=>
            #(#attrs)*
            #vis #constness #asyncness #unsafety #abi #fn_token #ident #generics() #output {
                let mut args = std::env::args().skip(1).into_iter();

                #(#var_bindings)*

                if args.next().is_some() {
                    exit(#too_many_args);
                }

                #block
            }

            #exit_fn
        ))
    }

    fn parse_fn_arg(arg: &FnArg) -> Result<TokenStream> {
        match arg {
            FnArg::Typed(pattern) => {
                let type_ = &pattern.ty;

                let binding = quote_spanned!(arg.span()=>
                    let #arg = args
                        .next()
                        .map(|arg| {
                            arg.parse().unwrap_or_else(|err| {
                                exit(&format!(
                                    "Error: unable to parse {} from {:?}\n\tOriginating from error: {}",
                                    stringify!(#type_),
                                    arg,
                                    err
                                ))
                            })
                        })
                        .unwrap_or_else(|| exit(&format!("Error: missing argument {}", stringify!(#arg))));
                );

                Ok(binding)
            }
            FnArg::Receiver(_) => Err(Error::new(arg.span(), "Unexpected `self` arg")),
        }
    }
}
