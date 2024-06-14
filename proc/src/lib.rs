use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Meta};

/// Log out the run time of the annotated function using a `proflogger::AutoLogger`.
///
/// # Panics
/// Will panic if the input function is not well formed or not supported.
#[proc_macro_attribute]
pub fn profile(args: TokenStream, input: TokenStream) -> TokenStream {
    // Input should look like
    //
    // #[profile]
    // fn function_name(args...) {}
    //
    // or
    //
    // #[profile(Error)]
    // fn function_name(args...) {}
    let log_level = if args.is_empty() {
        quote! {log::Level::Trace}
    } else if let syn::Meta::Path(path) = parse_macro_input!(args as Meta) {
        if let Some(ident) = path.get_ident() {
            match ident.to_string().as_str() {
                "Trace" => quote! {log::Level::Trace},
                "Debug" => quote! {log::Level::Debug},
                "Info" => quote! {log::Level::Info},
                "Warn" => quote! {log::Level::Warn},
                "Error" => quote! {log::Level::Error},
                l => panic!("Unknown log level {l}"),
            }
        } else {
            panic!("Unknown log level.")
        }
    } else {
        panic!("Unknown log level.")
    };

    let fin = parse_macro_input!(input as syn::Item);
    let syn::Item::Fn(fn_info) = fin else {
        panic!("#[profile] can only be used on a function.")
    };

    let attrs = fn_info.attrs;
    let vis = fn_info.vis;
    let sig = fn_info.sig;
    let profiler_name = format_ident!("_{}_profiler", sig.ident);
    let profiler_literal = format!("{}", sig.ident);
    let body = fn_info.block.stmts;

    let result_text = quote! {
        #(#attrs)*
        #vis #sig {
            #[cfg(debug_assertions)]
            let #profiler_name = if log::log_enabled!(#log_level) {
                Some(proflogger::AutoLogger::new(#profiler_literal, #log_level))
            } else {
                None
            };
            #log_level;

            #(#body)*
        }
    };

    result_text.into()
}
