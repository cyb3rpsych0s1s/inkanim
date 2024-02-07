use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Fields};
use quote::quote;

#[proc_macro_derive(RedsWidgetCompound)]
pub fn derive_reds_widget_compound(item: TokenStream) -> TokenStream {
    let syn::DeriveInput{ ident, data, .. } = parse_macro_input!(item as syn::DeriveInput);
    match &data {
        syn::Data::Struct(data) => derive_reds_widget_compound_for_struct(&ident, data),
        syn::Data::Enum(_) | syn::Data::Union(_) => syn::Error::new(ident.span(), "RedsWidgetCompound cannot be derived neither on union nor enum")
        .to_compile_error()
        .into(),
    }
}

#[proc_macro_derive(RedsWidgetLeaf)]
pub fn derive_reds_widget_leaf(item: TokenStream) -> TokenStream {
    let syn::DeriveInput{ ident, data, .. } = parse_macro_input!(item as syn::DeriveInput);
    match &data {
        syn::Data::Struct(data) => derive_reds_widget_leaf_for_struct(&ident, data),
        syn::Data::Enum(_) | syn::Data::Union(_) => syn::Error::new(ident.span(), "RedsWidgetLeaf cannot be derived neither on union nor enum")
        .to_compile_error()
        .into(),
    }
}

#[proc_macro_derive(RedsValue)]
pub fn derive_reds_value(item: TokenStream) -> TokenStream {
    let syn::DeriveInput{ ident, data, .. } = parse_macro_input!(item as syn::DeriveInput);
    match &data {
        syn::Data::Enum(data) => derive_reds_value_for_enum(&ident, data),
        syn::Data::Struct(data) => derive_reds_value_for_struct(&ident, data),
        syn::Data::Union(_) => syn::Error::new(ident.span(), "RedsValue cannot be derived on union")
        .to_compile_error()
        .into(),
    }
}

/// used with Redscript inkWidget class descendants
fn derive_reds_widget_leaf_for_struct(name: &syn::Ident, r#struct: &syn::DataStruct) -> TokenStream {
    let oneliners = r#struct.fields.iter().map(|x| x.ident.clone());
    quote! {
        impl crate::RedsWidgetLeaf for #name {
            fn reds_widget_leaf(&self, instance: &str, parent: Option<&str>) -> String {
                use ::red4ext_rs::conv::NativeRepr;
                use crate::RedsValue;
                let mut steps = vec![];
                steps.push(format!("let {} = new {}();", instance, Self::NAME));
                #(
                    if let Some(v) = self.#oneliners.reds_value() {
                        steps.push(::std::format!("{}.{} = {};", instance, ::std::stringify!(#oneliners), v));
                    }
                )*
                steps.join("\n")
            }
        }
    }.into()
}

/// used with Redscript native struct
fn derive_reds_value_for_struct(name: &syn::Ident, r#struct: &syn::DataStruct) -> TokenStream {
    let is_tuple = match r#struct.fields {
        Fields::Unnamed(_) => true,
        _ => false,
    };
    if is_tuple {
        let indexes = r#struct.fields.iter().enumerate().map(|(index, _)| syn::Index::from(index));
        return quote! {
            impl crate::RedsValue for #name {
                fn reds_value(&self) -> Option<String> {
                    use ::red4ext_rs::conv::NativeRepr;
                    if self == &Self::default() {
                        return None;
                    }
                    let mut args = Vec::<String>::new();
                    #(
                        args.push(self.#indexes.reds_value().unwrap_or(Default::default()));
                    )*
                    Some(format!("new {}({})", Self::NAME, args.join(", ")))
                }
            }
        }.into()
    }
    let fields = r#struct.fields.iter().map(|x| x.ident.clone());
    quote! {
        impl crate::RedsValue for #name {
            fn reds_value(&self) -> Option<String> {
                use ::red4ext_rs::conv::NativeRepr;
                if self == &Self::default() {
                    return None;
                }
                let mut args = Vec::<String>::new();
                #(
                    args.push(self.#fields.reds_value().unwrap_or(Default::default()));
                )*
                Some(format!("new {}({})", Self::NAME, args.join(", ")))
            }
        }
    }.into()
}

/// used with Redscript enums
fn derive_reds_value_for_enum(name: &syn::Ident, r#enum: &syn::DataEnum) -> TokenStream {
    let matches = r#enum.variants.iter().map(|x| {
        let variant = &x.ident;
        if x.fields.len() > 0 {
            return syn::Error::new(variant.span(), "RedsValue can only be derived on enum with unit variants")
            .to_compile_error()
            .into()
        }
        quote!{
            #name::#variant => Some(::std::format!("{}.{}", Self::NAME, ::std::stringify!(#variant)))
        }
    });
    quote! {
        impl crate::RedsValue for #name {
            fn reds_value(&self) -> Option<String> {
                use ::red4ext_rs::conv::NativeRepr;
                if self == &Self::default() {
                    return None;
                }
                match self {
                    #(#matches),*
                }
            }
        }
    }.into()
}

fn derive_reds_widget_compound_for_struct(name: &syn::Ident, r#struct: &syn::DataStruct) -> TokenStream {
    let oneliners = r#struct.fields
    .iter()
    .filter(|x| x.ident != Some(syn::Ident::new(::std::stringify!(children), x.span())))
    .map(|x| x.ident.clone());
    quote! {
        impl crate::RedsWidgetCompound for #name {
            fn reds_widget_compound(&self, instance: &str, parent: Option<&str>) -> String {
                use ::red4ext_rs::conv::NativeRepr;
                use crate::widget::layout::inkEChildOrder;
                use crate::RedsValue;
                let mut steps = vec![];
                steps.push(format!("let {} = new {}();", instance, Self::NAME));
                #(
                    if let Some(v) = self.#oneliners.reds_value() {
                        steps.push(::std::format!("{}.{} = {};", instance, ::std::stringify!(#oneliners), v));
                    }
                )*
                let mut name;
                if self.child_order == inkEChildOrder::Forward {
                    for child in self.children.iter() {
                        name = child.name().expect("no child to be a inkMultiChildren");
                        steps.push(child.reds_widget(name, self.name.reds_value().as_deref()));
                    }
                } else {
                    for child in self.children.iter().rev() {
                        name = child.name().expect("no child to be a inkMultiChildren");
                        steps.push(child.reds_widget(name, self.name.reds_value().as_deref()));
                    }
                }
                steps.join("\n")
            }
        }
    }.into()
}