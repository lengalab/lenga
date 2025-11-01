use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DataEnum, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input,
};

/// Derive macro for VariantProvider trait
///
/// Generates a `get_variants_as_language_objects` method that returns all variants
/// of an enum as LanguageObject instances with default values.
///
/// This macro should be applied to enums that have a conversion to LanguageObject
/// (i.e., they implement `Into<LanguageObject>`).
///
/// Requirements:
/// - The enum must have a conversion to LanguageObject
/// - Each variant's inner type must implement Default
/// - This macro assumes it's used within the `language` crate
///
/// Usage:
/// ```ignore
/// #[derive(VariantProvider)]
/// pub enum StatementObject {
///     CompoundStatement(CompoundStatement),
///     IfStatement(IfStatement),
///     ReturnStatement(ReturnStatement),
///     Unknown(Unknown),
/// }
/// ```
///
/// Generates:
/// ```ignore
/// impl StatementObject {
///     pub fn get_variants_as_language_objects() -> Vec<LanguageObject> {
///         vec![
///             LanguageObject::CompoundStatement(CompoundStatement::default()),
///             LanguageObject::IfStatement(IfStatement::default()),
///             LanguageObject::ReturnStatement(ReturnStatement::default()),
///             LanguageObject::Unknown(Unknown::default()),
///         ]
///     }
/// }
/// ```
#[proc_macro_derive(VariantProvider)]
pub fn derive_variant_provider(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Only support enums
    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => {
            return syn::Error::new_spanned(name, "VariantProvider only supports enums")
                .to_compile_error()
                .into();
        }
    };

    // Generate code for each variant
    // Use crate:: since this will be used within the language crate
    let variant_instances: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            // Check if variant has fields
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Single-field tuple variant (e.g., IfStatement(IfStatement))
                    let field_type = &fields.unnamed.first().unwrap().ty;
                    quote! {
                        crate::language::c::language_object::LanguageObject::from(
                            #name::#variant_name(#field_type::default())
                        )
                    }
                }
                Fields::Unit => {
                    // Unit variant (no fields)
                    quote! {
                        crate::language::c::language_object::LanguageObject::from(
                            #name::#variant_name
                        )
                    }
                }
                _ => {
                    // Multi-field or named field variants not supported
                    quote! {
                        compile_error!("VariantProvider only supports enums with single-field tuple variants or unit variants")
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #name {
            pub fn get_variants_as_language_objects() -> Vec<crate::language::c::language_object::LanguageObject> {
                vec![
                    #(#variant_instances),*
                ]
            }
        }
    };

    expanded.into()
}

/// Derive macro for FieldInspect trait
///
/// Generates a `get_options` method that returns `Vec<LanguageObject>` with default
/// instances for fields that can be converted to LanguageObject.
///
/// For structs:
/// - String fields: returns vec![] (empty, user input)
/// - StatementObject: returns a vec with all StatementObject variants (CompoundStatement, IfStatement, ReturnStatement, Unknown)
/// - ExpressionObject: returns a vec with all ExpressionObject variants (AssignmentExpression, BinaryExpression, CallExpression, NumberLiteral, Reference, StringLiteral, Unknown)
/// - CompoundStatementObject: returns a vec with all CompoundStatementObject variants (Declaration, all expressions, all statements, Comment, Unknown)
/// - DeclarationObject: returns a vec with all DeclarationObject variants (Declaration, FunctionDeclaration, FunctionDefinition, PreprocInclude, Comment, Unknown)
/// - Box<T>, Option<T>, Vec<T> where T is one of the above: unwraps and handles the inner type
///
/// For enums:
/// - Delegates to the `get_options` method of the corresponding variant's inner type
/// - Each variant's inner type must implement FieldInspect
///
/// Usage (struct):
/// ```ignore
/// #[derive(FieldInspect)]
/// struct MyStruct {
///     name: String,
///     body: StatementObject,
///     value: Option<Box<ExpressionObject>>,
/// }
///
/// // Generates:
/// // impl MyStruct {
/// //     fn get_options(&self, field: &str) -> Vec<LanguageObject> {
/// //         match field {
/// //             "name" => vec![],
/// //             "body" => vec![
/// //                 LanguageObject::CompoundStatement(CompoundStatement::default()),
/// //                 LanguageObject::IfStatement(IfStatement::default()),
/// //                 LanguageObject::ReturnStatement(ReturnStatement::default()),
/// //                 LanguageObject::Unknown(Unknown::default()),
/// //             ],
/// //             "value" => vec![
/// //                 LanguageObject::AssignmentExpression(AssignmentExpression::default()),
/// //                 // ... all ExpressionObject variants
/// //             ],
/// //             _ => vec![],
/// //         }
/// //     }
/// // }
/// ```
///
/// Usage (enum):
/// ```ignore
/// #[derive(FieldInspect)]
/// enum StatementObject {
///     CompoundStatement(CompoundStatement),
///     IfStatement(IfStatement),
///     ReturnStatement(ReturnStatement),
///     Unknown(Unknown),
/// }
///
/// // Generates:
/// // impl StatementObject {
/// //     pub fn get_options(&self, field: &str) -> Vec<LanguageObject> {
/// //         match self {
/// //             StatementObject::CompoundStatement(inner) => inner.get_options(field),
/// //             StatementObject::IfStatement(inner) => inner.get_options(field),
/// //             StatementObject::ReturnStatement(inner) => inner.get_options(field),
/// //             StatementObject::Unknown(inner) => inner.get_options(field),
/// //         }
/// //     }
/// // }
/// ```
#[proc_macro_derive(FieldInspect)]
pub fn derive_field_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    match &input.data {
        Data::Struct(data) => {
            // Handle structs
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => {
                    return syn::Error::new_spanned(
                        name,
                        "FieldInspect only supports structs with named fields",
                    )
                    .to_compile_error()
                    .into();
                }
            };

            // Generate match arms for each field
            let mut match_arms = vec![];

            for field in fields {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                let field_type = &field.ty;

                // Unwrap Box, Option, Vec to get the inner type
                let inner_type = unwrap_type(field_type);

                if let Some(method_call) = get_language_object_variants(inner_type) {
                    // Generate a call to the enum's get_variants_as_language_objects() method
                    match_arms.push(quote! {
                        #field_name_str => #method_call
                    });
                }
            }

            // Generate the implementation for struct
            let expanded = quote! {
                impl #name {
                    pub fn get_options(&self, field: &str) -> Vec<crate::language::c::language_object::LanguageObject> {
                        match field {
                            #(#match_arms,)*
                            _ => vec![],
                        }
                    }
                }
            };

            expanded.into()
        }
        Data::Enum(DataEnum { variants, .. }) => {
            // Handle enums - delegate to each variant's inner type
            let match_arms: Vec<_> = variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                            // Single-field tuple variant (e.g., IfStatement(IfStatement))
                            quote! {
                                #name::#variant_name(inner) => inner.get_options(field)
                            }
                        }
                        Fields::Unit => {
                            // Unit variant (no fields) - return empty vec
                            quote! {
                                #name::#variant_name => vec![]
                            }
                        }
                        _ => {
                            // Multi-field or named field variants not supported
                            quote! {
                                compile_error!("FieldInspect only supports enums with single-field tuple variants or unit variants")
                            }
                        }
                    }
                })
                .collect();

            // Generate the implementation for enum
            let expanded = quote! {
                impl #name {
                    pub fn get_options(&self, field: &str) -> Vec<crate::language::c::language_object::LanguageObject> {
                        match self {
                            #(#match_arms,)*
                        }
                    }
                }
            };

            expanded.into()
        }
        _ => {
            syn::Error::new_spanned(name, "FieldInspect only supports structs and enums")
                .to_compile_error()
                .into()
        }
    }
}

/// Unwrap Box<T>, Option<T>, Vec<T> to get the inner type T
fn unwrap_type(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = &segment.ident;

            // Check for Box, Option, Vec
            if ident == "Box" || ident == "Option" || ident == "Vec" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        // Recursively unwrap in case of nested types like Option<Box<T>>
                        return unwrap_type(inner_ty);
                    }
                }
            }
        }
    }
    ty
}

/// Check if a type can be converted to LanguageObject and return all its variants
/// by calling the generated get_variants_as_language_objects() method
fn get_language_object_variants(ty: &Type) -> Option<proc_macro2::TokenStream> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident_str = segment.ident.to_string();

            return match ident_str.as_str() {
                "StatementObject" => Some(quote! {
                    crate::language::c::language_object::statement_object::StatementObject::get_variants_as_language_objects()
                }),
                "ExpressionObject" => Some(quote! {
                    crate::language::c::language_object::expression_object::ExpressionObject::get_variants_as_language_objects()
                }),
                "CompoundStatementObject" => Some(quote! {
                    crate::language::c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::get_variants_as_language_objects()
                }),
                "DeclarationObject" => Some(quote! {
                    crate::language::c::language_object::declaration_object::DeclarationObject::get_variants_as_language_objects()
                }),
                _ => None,
            };
        }
    }
    None
}
