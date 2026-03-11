use proc_macro::TokenStream;
use quote::{IdentFragment, ToTokens, TokenStreamExt, format_ident, quote};
use syn::{Field, Generics, Ident, Index, Item, ItemEnum, ItemStruct, Variant, parse_macro_input};

#[derive(Debug, PartialEq, Clone)]
pub enum Testable<'a> {
    Struct(TestableStruct<'a>),
    Enum(TestableEnum<'a>),
}

impl<'a> Testable<'a> {
    pub fn new(item: &'a Item) -> Self {
        match item {
            Item::Struct(item) => Testable::structure(item),
            Item::Enum(item) => Testable::enumeration(item),
            item => panic!("Unsupported item: {:#?}", item),
        }
    }

    pub fn structure(item: &'a ItemStruct) -> Self {
        let item = TestableStruct::new(item);
        Testable::Struct(item)
    }

    pub fn enumeration(item: &'a ItemEnum) -> Self {
        let item = TestableEnum::new(item);
        Testable::Enum(item)
    }
}

impl<'a> From<&'a Item> for Testable<'a> {
    fn from(item: &'a Item) -> Self {
        Self::new(item)
    }
}

impl<'a> ToTokens for Testable<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Struct(item) => item.to_tokens(tokens),
            Self::Enum(item) => item.to_tokens(tokens),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestableStruct<'a> {
    id: &'a Ident,
    generics: &'a Generics,
    fields: Vec<TestableField<'a>>,
}

impl<'a> TestableStruct<'a> {
    pub fn new(item: &'a ItemStruct) -> Self {
        let fields = item
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| TestableField::new(idx, field))
            .collect();

        Self {
            id: &item.ident,
            generics: &item.generics,
            fields,
        }
    }
}

impl<'a> ToTokens for TestableStruct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = self.id;
        let generics = self.generics;
        let fields = &self.fields;

        tokens.append_all(quote! {
            #[cfg(test)]
            impl #generics #id #generics {
                #(#fields)*
            }
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestableEnum<'a> {
    id: &'a Ident,
    variants: Vec<TestableVariant<'a>>,
}

impl<'a> TestableEnum<'a> {
    pub fn new(item: &'a ItemEnum) -> Self {
        let variants = item.variants.iter().map(TestableVariant::new).collect();

        Self {
            id: &item.ident,
            variants,
        }
    }
}

impl<'a> ToTokens for TestableEnum<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestableVariant<'a> {
    id: &'a Ident,
    fields: Vec<TestableField<'a>>,
}

impl<'a> TestableVariant<'a> {
    pub fn new(variant: &'a Variant) -> Self {
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| TestableField::new(idx, field))
            .collect();

        Self {
            id: &variant.ident,
            fields,
        }
    }
}

impl<'a> ToTokens for TestableVariant<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldRef<'a> {
    Ident(&'a Ident),
    Idx(Index),
}

impl<'a> FieldRef<'a> {
    pub fn new(idx: usize, id: &'a Option<Ident>) -> Self {
        match id {
            Some(id) => Self::ident(id),
            None => Self::idx(idx),
        }
    }

    pub fn ident(id: &'a Ident) -> Self {
        Self::Ident(id)
    }

    pub fn idx(idx: usize) -> Self {
        Self::Idx(idx.into())
    }
}

impl<'a> ToTokens for FieldRef<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = match self {
            Self::Ident(id) => quote! { #id },
            Self::Idx(idx) => quote! { #idx },
        };
        tokens.append_all(quote! {
            #id
        });
    }
}

impl<'a> IdentFragment for FieldRef<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Ident(id) => id.fmt(f),
            Self::Idx(idx) => idx.fmt(f),
        }
    }
}

impl<'a> std::fmt::Display for FieldRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(id) => write!(f, "{}", id),
            Self::Idx(idx) => write!(f, "{}", idx.index),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestableField<'a> {
    id: FieldRef<'a>,
}

impl<'a> TestableField<'a> {
    pub fn new(idx: usize, field: &'a Field) -> Self {
        Self {
            id: FieldRef::new(idx, &field.ident),
        }
    }

    fn assertion_ident(&self) -> Ident {
        format_ident!("assert_{}", self.id)
    }
}

impl<'a> ToTokens for TestableField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = &self.id;
        let assertion_id = self.assertion_ident();
        let msg = format!("Expected field {} to be {{:#?}}, found {{:#?}}", id);

        tokens.append_all(quote! {
            pub fn #assertion_id(&self, expected: impl Into<()>) {
                let expected = expected.into();
                let actual = self.#id;

                assert_eq!(expected, actual, #msg, expected, actual)
            }
        })
    }
}
