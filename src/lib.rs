use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
};

struct YesOrNo {
    vis: syn::Visibility,
    name: syn::Ident,
    serde: Option<syn::LitBool>,
}

impl Parse for YesOrNo {
    fn parse(input: ParseStream) -> Result<Self> {
        let vis = input.parse()?;
        let name = input.parse()?;
        let _comma: Option<syn::token::Comma> = input.parse()?;
        let serde = input.parse()?;
        Ok(Self { vis, name, serde })
    }
}

#[proc_macro]
pub fn yes_or_no(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let YesOrNo { vis, name, serde } = parse_macro_input!(input as YesOrNo);
    let serde_derive = {
        if let Some(true) = serde.map(|lit| lit.value()) {
            quote! {
                #[derive(serde::Deserialize, serde::Serialize)]
            }
        } else {
            quote! {}
        }
    };
    let expanded = quote! {
        #serde_derive
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #vis enum #name {
            No,
            Yes,
        }

        impl #name {
            pub const fn from_bool(flag: bool) -> Self {
                if flag {
                    Self::Yes
                } else {
                    Self::No
                }
            }

            pub const fn yes(self) -> bool {
                matches!(self, Self::Yes)
            }

            pub const fn no(self) -> bool {
                matches!(self, Self::No)
            }

            pub const fn and(self, other: Self) -> Self {
                Self::from_bool(self.yes() & other.yes())
            }

            pub const fn or(self, other: Self) -> Self {
                Self::from_bool(self.yes() | other.yes())
            }

            pub const fn xor(self, other: Self) -> Self {
                Self::from_bool(self.yes() ^ other.yes())
            }

            pub const fn not(self) -> Self {
                Self::from_bool(self.no())
            }
        }

        impl Default for #name {
            fn default() -> Self {
                Self::No
            }
        }

        impl From<bool> for #name {
            fn from(flag: bool) -> Self {
                Self::from_bool(flag)
            }
        }

        impl Into<bool> for #name {
            fn into(self) -> bool {
                self.yes()
            }
        }

        impl std::ops::BitAnd for #name {
            type Output = Self;

            fn bitand(self, other: Self) -> Self::Output {
                self.and(other)
            }
        }

        impl std::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, other: Self) {
                *self = self.and(other);
            }
        }

        impl std::ops::BitOr for #name {
            type Output = Self;

            fn bitor(self, other: Self) -> Self::Output {
                self.or(other)
            }
        }

        impl std::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, other: Self) {
                *self = self.or(other);
            }
        }

        impl std::ops::BitXor for #name {
            type Output = Self;

            fn bitxor(self, other: Self) -> Self::Output {
                self.xor(other)
            }
        }

        impl std::ops::BitXorAssign for #name {
            fn bitxor_assign(&mut self, other: Self) {
                *self = self.xor(other);
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;

            fn not(self) -> Self::Output {
                self.not()
            }
        }
    };
    // panic!("{}", expanded.to_string());
    expanded.into()
}
