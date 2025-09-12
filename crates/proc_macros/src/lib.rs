use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn main_pretty_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    let orig_name = func.sig.ident.clone();
    let inner_name = Ident::new(&format!("{}_inner", orig_name), Span::call_site());
    func.sig.ident = inner_name.clone();

    let expanded = quote! {
        #func

        fn main() {
            if let Err(e) = #inner_name() {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            }
        }
    };

    expanded.into()
}
