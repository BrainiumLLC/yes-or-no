use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn yes_or_no(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name = parse_macro_input!(input as syn::Ident);

    let expanded = quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum #name {
            Yes,
            No,
        }

        impl #name {
            pub fn from_bool(flag: bool) -> Self {
                flag.into()
            }

            pub fn yes(self) -> bool {
                matches!(self, Self::Yes)
            }

            pub fn no(self) -> bool {
                matches!(self, Self::No)
            }
        }

        impl Default for #name {
            fn default() -> Self {
                Self::No
            }
        }

        impl From<bool> for #name {
            fn from(flag: bool) -> Self {
                if flag {
                    Self::Yes
                } else {
                    Self::No
                }
            }
        }

        impl Into<bool> for #name {
            fn into(self) -> bool {
                self.yes()
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;

            fn not(self) -> Self::Output {
                if self.yes() {
                    Self::No
                } else {
                    Self::Yes
                }
            }
        }
    };
    //panic!("{}", expanded.to_string());
    expanded.into()
}
