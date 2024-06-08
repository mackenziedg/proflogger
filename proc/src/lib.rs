use proc_macro::{Delimiter, TokenStream, TokenTree};

fn get_log_level(args: TokenStream) -> String {
    let log_level = match args.into_iter().next() {
        Some(TokenTree::Literal(l)) => l.to_string(),
        _ => String::from("Trace"),
    };
    format!("log::Level::{}", log_level.trim_matches('"'))
}

struct FuncInfo {
    qualifiers: TokenStream,
    name: String,
}

fn get_function_quals(
    source: &mut std::iter::Peekable<proc_macro::token_stream::IntoIter>,
) -> FuncInfo {
    let mut name = None;
    let mut qualifiers = vec![];
    while let Some(TokenTree::Ident(ident)) = source.peek() {
        let s = ident.to_string();
        // TODO: This doesn't properly deal with extern functions. It's also not the cleanest way
        // to write this anyway and should be fixed up.
        if ["fn", "pub", "const", "async", "unsafe"].contains(&s.as_str()) {
            qualifiers.push(TokenTree::Ident(ident.clone()));
        } else {
            name = Some(s);
        }
        source.next();
    }
    let name = name.unwrap();

    let qualifiers = qualifiers.into_iter().collect::<TokenStream>();
    FuncInfo { qualifiers, name }
}

fn get_function_signature(
    source: &mut std::iter::Peekable<proc_macro::token_stream::IntoIter>,
) -> TokenStream {
    let mut function_signature = vec![];
    while let Some(t) = source.peek() {
        if let TokenTree::Group(g) = t {
            if g.delimiter() == Delimiter::Brace {
                break;
            }
            function_signature.push(TokenTree::Group(g.clone()));
        } else {
            function_signature.push(t.clone());
        }
        source.next();
    }
    function_signature.into_iter().collect::<TokenStream>()
}

/// Log out the run time of the annotated function using a `AutoLogger`.
///
/// # Panics
/// Will panic if the input function is not well formed or not supported.
/// Currently, the only known unsupported functions are those qualified as `extern`.
#[proc_macro_attribute]
pub fn profile(args: TokenStream, input: TokenStream) -> TokenStream {
    // Input should look like
    //
    // #[profile]
    // fn function_name(args...) {}
    //
    // or
    //
    // #[profile("Error")]
    // fn function_name(args...) {}
    let log_level = get_log_level(args);

    let mut source = input.into_iter().peekable();

    let function_info = get_function_quals(&mut source);
    let function_signature = get_function_signature(&mut source);

    let function_body = match source.next() {
        Some(TokenTree::Group(g)) => {
            assert!(
                (g.delimiter() == Delimiter::Brace),
                "Ill-formed function body."
            );
            g.stream()
        }
        Some(body) => panic!("Ill-formed function body - {body}"),
        None => panic!("Nothing found."),
    };

    let function_quals = function_info.qualifiers;
    let function_name = function_info.name;

    let result_text = format!(
        "{function_quals} {function_name}{function_signature} {{
            #[cfg(debug_assertions)]
            let _profiler_{function_name} = if log::log_enabled!({log_level}) {{
                Some(AutoLogger::new(\"{function_name}\", {log_level}))
            }} else {{
                None
            }};
            {function_body}
        }}",
    );

    result_text
        .parse::<TokenStream>()
        .expect("Macro generated invalid tokens")
}
