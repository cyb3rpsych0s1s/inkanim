extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Reds)]
pub fn derive_reds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = match &input.data {
        syn::Data::Struct(_) => {
            let is_class = match is_class(&input) {
                Ok(x) => x,
                Err(e) => return abort(input, format!("{e}")),
            };
            if is_class {
                derive_reds_class(&input)
            } else {
                derive_reds_struct(&input)
            }
        }
        syn::Data::Enum(_) => derive_reds_enum(&input),
        syn::Data::Union(_) => {
            return abort(input, "union is not supported");
        }
    };

    todo!()
}

fn is_class(input: &DeriveInput) -> Result<bool, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("reds") {
            let metalist = attr.meta.require_list()?;
            let mut is_class: Option<bool> = None;
            metalist.parse_nested_meta(|x| {
                if x.path.is_ident("class") {
                    is_class = Some(true);
                } else if x.path.is_ident("struct") {
                    is_class = Some(false);
                }
                Ok(())
            })?;
            if let Some(is_class) = is_class {
                return Ok(is_class);
            } else {
                return Err(fail(input, "expects #[reds(class)] or #[reds(struct)]"));
            }
        }
    }
    Ok(true)
}

fn derive_reds_enum(input: &DeriveInput) -> proc_macro2::TokenStream {
    todo!()
}
fn derive_reds_class(input: &DeriveInput) -> proc_macro2::TokenStream {
    todo!()
}
fn derive_reds_struct(input: &DeriveInput) -> proc_macro2::TokenStream {
    todo!()
}

fn abort(input: impl Spanned, msg: impl AsRef<str>) -> TokenStream {
    fail(input, msg).to_compile_error().into()
}
fn fail(input: impl Spanned, msg: impl AsRef<str>) -> Error {
    Error::new(input.span(), msg.as_ref())
}
