extern crate proc_macro;
// NOTE: As a challenge, I avoided using proc_macro2, syn, and quote! in this proc_macro set.
// It would probably be better using those things, but this was a good challenge

use proc_macro::{Delimiter, Group, /* Ident, Literal, Punct, Spacing, */ TokenStream, TokenTree};

#[proc_macro_attribute]
/// Makes a function accessible through the wasm interface using the C-ABI.  \
/// Usage: `#[wasm_export]` or `#[wasm_export("export_name")]`  \
/// Expands to `#[link_name = "..."] pub unsafe extern "C"`
pub fn wasm_export(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut body = if attr.is_empty() {
        "#[no_mangle] pub extern \"C\"".parse::<TokenStream>().unwrap()
    } else {
        let mut full = wrap_in_attribute(prefix_with("export_name=", attr));
        full.extend("pub extern \"C\"".parse::<TokenStream>());
        full
    };
    body.extend(input);
    return body;
}

#[proc_macro_attribute]
/// Imports a function from the wasm interface.  \
/// Conditionally compiles (non-wasm targets will not define the symbol). Uses the C-ABI.  \
/// Usage: `#[wasm_import("import_name", "module_name")]`, where both are optional (skip "import_name" with a `,`).
/// Expands to something akin to `#[cfg(target_family = "wasm")] #[link(wasm_import_module = "...")] extern "C" #[link_name = "...")]`
pub fn wasm_import(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Arg parse
    let mut import = None;
    let mut module = None;
    let mut iter = attr.into_iter();
    'a: {
        // parameter 'import_name' (string)
        let Some(x) = iter.next() else {break 'a};
        match x {
            TokenTree::Literal(x) => {
                import = Some(x);
                // consume comma
                let Some(x) = iter.next() else {break 'a};
                match x{
                    TokenTree::Punct(x) if x.as_char() == ',' => {},
                    _ => panic!("Expected `,` or end of macro")
                };
            }
            TokenTree::Punct(x) if x.as_char() == ',' => {}, // skip param
            _ => panic!("'import_name' is not a string")
        }
        // parameter 'module_name' (string)
        let Some(x) = iter.next() else {break 'a};
        let TokenTree::Literal(x) = x else { panic!("'module_name' is not a string") };
        module = Some(x);
        // Expect nothing after
        let None = iter.next() else { panic!("Expected end of macro") };
    }
    // Form the output
    let mut body = "#[cfg(target_family = \"wasm\")]".parse::<TokenStream>().unwrap();
    if let Some(x) = module {
        let tag = prefix_with("wasm_import_module=", std::iter::once(TokenTree::Literal(x)));
        let mut strm = TokenStream::new();
        strm.extend("link".parse::<TokenStream>());
        strm.extend_1(TokenTree::Group(Group::new(Delimiter::Parenthesis, tag)));
        body.extend(wrap_in_attribute(strm));
    }
    body.extend("extern \"C\"".parse::<TokenStream>());

    // The inside of the extern block
    let mut inner = TokenStream::new();
    if let Some(x) = import{ // Link name if specified
        inner.extend(wrap_in_attribute(prefix_with("link_name=", std::iter::once(TokenTree::Literal(x)))));
    }
    inner.extend(input);

    let inner = TokenTree::Group(Group::new(Delimiter::Brace, inner));
    body.extend_1(inner);
    return body;
}

// TokenStream containing `ident = tokens`
fn prefix_with<I>(prefix: &str, extend: I) -> TokenStream
 where I: IntoIterator<Item = TokenTree> {
    let mut strm = TokenStream::new();
    strm.extend(prefix.parse::<TokenStream>());
    strm.extend(extend);
    strm
}

// Converts `tokens` to `#[tokens]`
fn wrap_in_attribute(inner: TokenStream) -> TokenStream {
    let mut attribute = TokenStream::new();
    attribute.extend("#".parse::<TokenStream>());
    attribute.extend(std::iter::once(TokenTree::Group(Group::new(Delimiter::Bracket, inner))));
    attribute
}

// Adaptor for unsafe functionality
trait ExtendOne{
    fn extend_1(&mut self, val: TokenTree);
}
impl ExtendOne for TokenStream{
    fn extend_1(&mut self, val: TokenTree) {
        self.extend(std::iter::once(val));
    }
}