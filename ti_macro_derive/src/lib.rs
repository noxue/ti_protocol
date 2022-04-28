extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TiPack)]
pub fn ti_pack_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_ti_pack_macro(&ast)
}

fn impl_ti_pack_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TiPack for #name {
            fn pack(&self) -> Result<Vec<u8>, String> {
                pack(self)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(TiUnPack)]
pub fn ti_unpack_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_ti_unpack_macro(&ast)
}

fn impl_ti_unpack_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TiUnPack for #name {
            fn unpack<'a>(encoded: &'a [u8]) -> Result<Self, String> where Self: Deserialize<'a>{
                unpack(encoded)
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
