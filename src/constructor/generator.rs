use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::constructor::plan::{ConstructorPlan, VariantShape};

pub(crate) fn generate(
    plan: &ConstructorPlan,
    visibility: &TokenStream,
    constant: &TokenStream,
    constructor_name: &Ident,
    self_ident: &TokenStream,
) -> TokenStream {
    let parameters = &plan.parameters;
    let pass_values = &plan.pass_values;

    match &plan.shape {
        VariantShape::Unit => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name() -> Self {
                    #self_ident
                }
            }
        }
        VariantShape::Tuple => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name(#(#parameters),*) -> Self {
                    #self_ident(#(#pass_values),*)
                }
            }
        }
        VariantShape::Struct => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name(#(#parameters),*) -> Self {
                    #self_ident {
                        #(#pass_values),*
                    }
                }
            }
        }
    }
}
