mod assert;
mod model;

#[proc_macro_attribute]
pub fn unit_test_assertions(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(item as syn::Item);
    let testable = Into::<assert::Testable>::into(&ast);

    quote::quote! {
        #[cfg_attr(test, derive(Debug, PartialEq, Clone))]
        #ast

        #testable
    }
    .into()
}

#[proc_macro]
pub fn test_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Data Model: Input token stream: {:#?}", _input);
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn smithy(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Smithy: Input token stream: {:#?}", _input);
    proc_macro::TokenStream::new()
}
