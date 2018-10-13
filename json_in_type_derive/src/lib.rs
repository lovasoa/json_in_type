#![recursion_limit = "128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::TokenStream;
use syn::{
    Data,
    DeriveInput,
    spanned::Spanned
};

#[proc_macro_derive(JSONValue)]
pub fn jsonvalue_macro_derive(input: TokenStream) -> TokenStream {
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
        Data::Enum(e) => impl_jsonvalue_macro_enum(&name, e),
        Data::Union(_) => unimplemented!()
    }
}


fn ident_to_litbytes(ident: &syn::Ident, first: bool) -> syn::LitByteStr {
    let mut obj_key_str = format!("\"{}\":", ident);
    obj_key_str.insert(0, if first { '{' } else { ',' });
    syn::LitByteStr::new(obj_key_str.as_bytes(), ident.span())
}

fn field_to_litbytes(field: &syn::Field, first: bool) -> Option<syn::LitByteStr> {
    field.clone().ident.map(|ident| ident_to_litbytes(&ident, first))
}

fn field_to_ident(field: &syn::Field) -> Option<syn::Ident> {
    field.ident.clone()
}

fn write_named_fields(
    fields_named: &syn::FieldsNamed,
) -> impl quote::ToTokens {
    let fs = fields_named.named.clone();
    let names: Vec<syn::LitByteStr> = fs.iter()
        .enumerate()
        .flat_map(|(i, f)| field_to_litbytes(f, i == 0))
        .collect();
    let fields: Vec<syn::Ident> = fs.iter()
        .flat_map(field_to_ident)
        .collect();
    let end = syn::LitByteStr::new(if fields.is_empty() { b"{}" } else { b"}" }, fs.span());
    quote! {
        #(
            w.write_all(#names)?;
            self.#fields.write_json(w)?;
        )*
        w.write_all(#end)
    }
}

fn write_unnamed_fields(
    fields_named: &syn::FieldsUnnamed,
) -> impl quote::ToTokens {
    let fs = fields_named.unnamed.clone();
    let nums: Vec<u32> = fs.iter().enumerate().map(|(i, _)| i as u32).collect();
    let commas: Vec<syn::LitByteStr> = nums.iter()
        .map(|i| syn::LitByteStr::new(if *i == 0 { b"[" } else { b"," }, fs.span()))
        .collect();
    let members: Vec<syn::Member> = nums.iter()
        .map(|i| syn::Member::Unnamed(syn::Index { index: *i, span: fs.span() }))
        .collect();
    let end = syn::LitByteStr::new(if commas.is_empty() { b"[]" } else { b"]" }, fs.span());
    quote! {
        #(
            w.write_all(#commas)?;
            self.#members.write_json(w)?;
        )*
        w.write_all(#end)
    }
}

fn write_fields(
    fields: &syn::Fields,
) -> Box<quote::ToTokens> {
    match fields {
        syn::Fields::Named(fields_named) =>
            Box::new(write_named_fields(fields_named)),
        syn::Fields::Unnamed(fields_unnamed) =>
            Box::new(write_unnamed_fields(fields_unnamed)),
        syn::Fields::Unit =>
            Box::new(quote! {w.write_all(b"null")})
    }
}

fn impl_jsonvalue_macro_struct(
    name: &syn::Ident,
    struct_data: &syn::DataStruct,
) -> TokenStream {
    let write_fields_ts = write_fields(&struct_data.fields);
    (quote! {
        impl JSONValue for #name {
            fn write_json<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
                #write_fields_ts
            }
        }
    }).into()
}

fn impl_jsonvalue_macro_enum(
    name: &syn::Ident,
    struct_data: &syn::DataEnum,
) -> TokenStream {
    let vs = struct_data.variants.clone();
    let idents: Vec<syn::Ident> = vs.iter().map(|v| v.ident.clone()).collect();
    let variants_json: Vec<_> = idents.clone().iter()
        .map(|ident| ident_to_litbytes(ident, true))
        .collect();
    let names = std::iter::repeat(name);
    for v in vs.iter() {
        match v.fields {
            syn::Fields::Unnamed(_) => unimplemented!(),
            syn::Fields::Named(_) => unimplemented!(),
            syn::Fields::Unit => (),
        }
    }
    (quote! {
        impl JSONValue for #name {
            fn write_json<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
                match self {
                #(
                    #names::#idents => {
                        w.write_all(#variants_json)?;
                        w.write_all(b"true}")
                    }
                ),*
                }
            }
        }
    }).into()
}