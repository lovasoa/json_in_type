#![recursion_limit = "128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::TokenStream;
use syn::{
    Data,
    DeriveInput,
};

#[proc_macro_derive(JSONValue)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_jsonvalue_macro(&ast)
}

fn impl_jsonvalue_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Data::Struct(s) => impl_jsonvalue_macro_struct(&name, s),
        _ => unimplemented!()
    }
}

fn field_to_string(field: &syn::Field) -> Option<String> {
    field.clone().ident.map(|ident| format!("{}", ident))
}

fn field_to_ident(field: &syn::Field) -> Option<syn::Ident> {
    field.clone().ident
}

fn impl_jsonvalue_macro_struct(
    name: &syn::Ident,
    struct_data: &syn::DataStruct,
) -> TokenStream {
    let fs = struct_data.fields.clone();
    let first_name = fs.iter().take(1).flat_map(field_to_string);
    let names = fs.iter().skip(1).flat_map(field_to_string);
    let first_field = fs.iter().take(1).flat_map(field_to_ident);
    let fields = fs.iter().skip(1).flat_map(field_to_ident);
    let gen = quote! {
        use std::io;
        impl JSONValue for #name {
            fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
                w.write_all(b"{")?;
                #(
                    write!(w, "\"{}\":", #first_name)?;
                    self.#first_field.write_json(w)?;
                )*
                #(
                    write!(w, ",\"{}\":", #names)?;
                    self.#fields.write_json(w)?;
                )*
                w.write_all(b"}")
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
