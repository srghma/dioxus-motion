use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Fields, Meta};

fn get_transition_from_attrs(attrs: &[Attribute]) -> Option<String> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("transition"))
        .and_then(|attr| {
            if let Ok(Meta::Path(path)) = attr.parse_args::<Meta>() {
                path.get_ident().map(|ident| ident.to_string())
            } else {
                None
            }
        })
}

fn get_layout_from_attrs(attrs: &[Attribute]) -> Option<syn::Path> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("layout"))
        .and_then(|attr| {
            if let Ok(Meta::Path(path)) = attr.parse_args::<Meta>() {
                Some(path.clone())
            } else {
                None
            }
        })
}

// Helper to extract layout nesting information from enum variants
fn get_layout_depth(variants: &[&syn::Variant]) -> Vec<(syn::Ident, usize)> {
    let mut layout_depth = Vec::new();
    let mut current_depth = 0;

    for variant in variants {
        // Check if this variant has a layout attribute
        if variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("layout"))
        {
            current_depth += 1;
        }

        // Check if this variant ends a layout
        if variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("end_layout"))
        {
            if current_depth > 0 {
                current_depth -= 1;
            }
        }

        // Associate current depth with this variant
        layout_depth.push((variant.ident.clone(), current_depth));
    }

    layout_depth
}

#[proc_macro_derive(MotionTransitions, attributes(transition, layout, end_layout))]
pub fn derive_route_transitions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("MotionTransitions can only be derived for enums"),
    };

    let transition_match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let transition = get_transition_from_attrs(&variant.attrs)
            .map(|t| format_ident!("{}", t))
            .unwrap_or(format_ident!("Fade"));

        match &variant.fields {
            Fields::Named(fields) => {
                let field_patterns = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! { #name: _ }
                });
                quote! {
                    Self::#variant_ident { #(#field_patterns,)* } => TransitionVariant::#transition
                }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_ident(..) => TransitionVariant::#transition }
            }
            Fields::Unit => {
                quote! { Self::#variant_ident {} => TransitionVariant::#transition }
            }
        }
    });

    let component_match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let component_name = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                quote! {
                    Self::#variant_ident { #(ref #field_names,)* } => {
                        rsx! { #component_name { #(#field_names: #field_names.clone(),)* } }
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_ident(..) => rsx! { #component_name {} } }
            }
            Fields::Unit => {
                quote! { Self::#variant_ident {} => rsx! { #component_name {} } }
            }
        }
    });

    let layout_match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let layout = get_layout_from_attrs(&variant.attrs);

        match &variant.fields {
            Fields::Named(fields) => {
                let field_patterns = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! { #name: _ }
                });
                if let Some(layout_path) = layout {
                    quote! {
                        Self::#variant_ident { #(#field_patterns,)* } => Some(rsx! { #layout_path {} })
                    }
                } else {
                    quote! {
                        Self::#variant_ident { #(#field_patterns,)* } => None
                    }
                }
            }
            Fields::Unnamed(_) => {
                if let Some(layout_path) = layout {
                    quote! { Self::#variant_ident(..) => Some(rsx! { #layout_path {} }) }
                } else {
                    quote! { Self::#variant_ident(..) => None }
                }
            }
            Fields::Unit => {
                if let Some(layout_path) = layout {
                    quote! { Self::#variant_ident => Some(rsx! { #layout_path {} }) }
                } else {
                    quote! { Self::#variant_ident => None }
                }
            }
        }
    });

    // Generate layout depth match arms
    let layout_depths = get_layout_depth(&variants.iter().collect::<Vec<_>>());
    let layout_depth_match_arms =
        layout_depths.iter().map(|(variant_ident, depth)| {
            match &variants
                .iter()
                .find(|v| &v.ident == variant_ident)
                .unwrap()
                .fields
            {
                Fields::Named(fields) => {
                    let field_patterns = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! { #name: _ }
                    });
                    quote! {
                        Self::#variant_ident { #(#field_patterns,)* } => #depth
                    }
                }
                Fields::Unnamed(_) => {
                    quote! { Self::#variant_ident(..) => #depth }
                }
                Fields::Unit => {
                    quote! { Self::#variant_ident {} => #depth }
                }
            }
        });

    let expanded = quote! {
        impl AnimatableRoute for  #name {
            fn get_transition(&self) -> TransitionVariant {
                match self {
                    #(#transition_match_arms,)*
                    _ => TransitionVariant::Fade,
                }
            }

            fn get_component(&self) -> Element {
                match self {
                    #(#component_match_arms,)*
                }
            }

            fn get_layout(&self) -> Option<Element> {
                match self {
                    #(#layout_match_arms,)*
                    _ => None,
                }
            }

            // New method to get layout depth
            fn get_layout_depth(&self) -> usize {
                match self {
                    #(#layout_depth_match_arms,)*
                    _ => 0,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
