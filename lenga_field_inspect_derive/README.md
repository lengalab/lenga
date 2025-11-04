# Lenga-Field-Inspect-Derive

Macros for generating LanguageObject attribute objects for the Lenga package

## Usage

```toml
[dependencies]
lenga_field_inspect_derive = "1.0.0"
```

``` rust
#[derive(Debug, Clone, PartialEq, lenga_field_inspect_derive::FieldInspect)]
pub struct Declaration {
    pub id: Uuid,
    pub primitive_type: CType,
    pub identifier: String,
    pub value: Option<Box<ExpressionObject>>,
}
```

## Disclaimer

This macro package is not intended to be used outside of the source code of Lenga package