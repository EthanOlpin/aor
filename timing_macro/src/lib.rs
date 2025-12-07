use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;

/// Times a block of code and prints the duration to stderr.
/// Variables declared inside are accessible after the macro.
///
/// ```
/// timing! {
///     let x = expensive_computation();
///     do_more_work();
/// }
/// // x is accessible here
/// ```
#[proc_macro]
pub fn timing(input: TokenStream) -> TokenStream {
    let stmts = match syn::Block::parse_within.parse(input) {
        Ok(stmts) => stmts,
        Err(e) => return e.to_compile_error().into(),
    };

    let code_str = stmts
        .iter()
        .map(|s| quote!(#s).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let expanded = quote! {
        let __timing_start = ::std::time::Instant::now();
        #(#stmts)*
        ::std::eprintln!(
            "[{}:{}:{}] {} took {:?}",
            ::std::file!(),
            ::std::line!(),
            ::std::column!(),
            #code_str,
            __timing_start.elapsed()
        );
    };

    TokenStream::from(expanded)
}
