use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(StyledComponent)]
pub fn style_component_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote::quote! {
        impl StyledComponent for #name {
            fn get_style(&self) -> &Style {
                &self.style
            }
            fn get_style_mut(&mut self) -> &mut Style {
                &mut self.style
            }
        }
    };

    expanded.into()
}
