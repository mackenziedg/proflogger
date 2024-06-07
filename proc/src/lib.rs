use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Log out the run time of the annotated function using a `AutoLogger`.
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

    let log_level = match args.into_iter().next() {
        Some(TokenTree::Literal(l)) => l.to_string(),
        _ => String::from("Trace"),
    };
    let log_level = log_level.trim_matches('"');

    let mut source = input.into_iter().peekable();

    let mut function_defs = vec![];
    let mut function_name = None;

    while let Some(TokenTree::Ident(ident)) = source.peek() {
        let s = ident.to_string();
        if ["fn", "pub", "const"].contains(&s.as_str()) {
            function_defs.push(TokenTree::Ident(ident.clone()));
        } else {
            function_name = Some(s);
        }
        source.next();
    }

    let function_defs = function_defs.into_iter().collect::<TokenStream>();

    let mut function_heading = vec![];

    while let Some(t) = source.peek() {
        if let TokenTree::Group(g) = t {
            if g.delimiter() == Delimiter::Brace {
                break;
            } else {
                function_heading.push(TokenTree::Group(g.clone()));
            }
        } else {
            function_heading.push(t.clone());
        }
        source.next();
    }
    let function_heading = function_heading.into_iter().collect::<TokenStream>();

    let function_body = match source.next() {
        Some(TokenTree::Group(g)) => {
            if g.delimiter() != Delimiter::Brace {
                panic!("Ill-formed function body.");
            }
            g
        }
        Some(body) => panic!("Ill-formed function body - {body}"),
        None => panic!("Nothing found."),
    };

    let function_name = function_name.as_ref().unwrap();
    let function_body = function_body.stream();
    let log_level = format!("log::Level::{log_level}");

    let result_text = format!(
        "{function_defs} {function_name}{function_heading} {{
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
