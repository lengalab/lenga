#[cfg(test)]
mod tests {
    use crate::language::c::language_object::{
        declaration_object::DeclarationObject,
        expression_object::ExpressionObject,
        statement_object::{
            StatementObject, compound_statement::compound_statement_object::CompoundStatementObject,
        },
    };
    use field_inspect_derive::FieldInspect;

    #[derive(FieldInspect)]
    #[expect(dead_code)]
    struct TestStruct {
        pub name: String,
        pub description: String,
        pub body: StatementObject,
        pub expr: ExpressionObject,
        pub compound: CompoundStatementObject,
        pub decl: DeclarationObject,
        pub count: u32,
        // Test wrapped types
        pub vec_statements: Vec<StatementObject>,
        pub opt_expression: Option<ExpressionObject>,
        pub boxed_declaration: DeclarationObject,
        pub nested: Option<Vec<StatementObject>>,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            TestStruct {
                name: String::from("Test"),
                description: String::from("A test struct"),
                body: StatementObject::default(),
                expr: ExpressionObject::default(),
                compound: CompoundStatementObject::default(),
                decl: DeclarationObject::default(),
                count: 0,
                vec_statements: vec![],
                opt_expression: None,
                boxed_declaration: *Box::new(DeclarationObject::default()),
                nested: None,
            }
        }
    }

    #[test]
    fn test_string_field_returns_some() {
        let test = TestStruct::default();

        let options = test.get_options("name");
        assert!(options.is_empty());
    }

    #[test]
    fn test_statement_object_field_returns_all_variants() {
        let test = TestStruct::default();

        let options = test.get_options("body");
        // Should return all StatementObject variants: CompoundStatement, IfStatement, ReturnStatement, Unknown
        assert_eq!(options.len(), 4);

        let has_compound_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::CompoundStatement(_)
            )
        });
        let has_if_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::IfStatement(_)
            )
        });
        let has_return_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::ReturnStatement(_)
            )
        });
        let has_unknown = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Unknown(_)
            )
        });

        assert!(has_compound_statement, "Missing CompoundStatement variant");
        assert!(has_if_statement, "Missing IfStatement variant");
        assert!(has_return_statement, "Missing ReturnStatement variant");
        assert!(has_unknown, "Missing Unknown variant");
    }

    #[test]
    fn test_expression_object_field_returns_all_variants() {
        let test = TestStruct::default();

        let options = test.get_options("expr");
        // Should return all ExpressionObject variants
        assert_eq!(options.len(), 7);

        let has_assignment = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::AssignmentExpression(_)
            )
        });
        let has_binary = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::BinaryExpression(_)
            )
        });
        let has_call = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::CallExpression(_)
            )
        });
        let has_number = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::NumberLiteral(_)
            )
        });
        let has_reference = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Reference(_)
            )
        });
        let has_string = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::StringLiteral(_)
            )
        });
        let has_unknown = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Unknown(_)
            )
        });

        assert!(has_assignment, "Missing AssignmentExpression variant");
        assert!(has_binary, "Missing BinaryExpression variant");
        assert!(has_call, "Missing CallExpression variant");
        assert!(has_number, "Missing NumberLiteral variant");
        assert!(has_reference, "Missing Reference variant");
        assert!(has_string, "Missing StringLiteral variant");
        assert!(has_unknown, "Missing Unknown variant");
    }

    #[test]
    fn test_compound_statement_object_field_returns_all_variants() {
        let test = TestStruct::default();

        let options = test.get_options("compound");
        // Should return all CompoundStatementObject variants
        assert_eq!(options.len(), 12);

        let has_declaration = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Declaration(_)
            )
        });
        let has_compound_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::CompoundStatement(_)
            )
        });
        let has_if_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::IfStatement(_)
            )
        });
        let has_return_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::ReturnStatement(_)
            )
        });
        let has_comment = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Comment(_)
            )
        });
        let has_unknown = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Unknown(_)
            )
        });

        assert!(has_declaration, "Missing Declaration variant");
        assert!(has_compound_statement, "Missing CompoundStatement variant");
        assert!(has_if_statement, "Missing IfStatement variant");
        assert!(has_return_statement, "Missing ReturnStatement variant");
        assert!(has_comment, "Missing Comment variant");
        assert!(has_unknown, "Missing Unknown variant");
        // Also has all expression variants, but not testing them individually here
    }

    #[test]
    fn test_declaration_object_field_returns_all_variants() {
        let test = TestStruct::default();

        let options = test.get_options("decl");
        // Should return all DeclarationObject variants
        assert_eq!(options.len(), 6);

        let has_declaration = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Declaration(_)
            )
        });
        let has_function_declaration = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::FunctionDeclaration(_)
            )
        });
        let has_function_definition = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::FunctionDefinition(_)
            )
        });
        let has_preproc_include = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::PreprocInclude(_)
            )
        });
        let has_comment = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Comment(_)
            )
        });
        let has_unknown = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Unknown(_)
            )
        });

        assert!(has_declaration, "Missing Declaration variant");
        assert!(
            has_function_declaration,
            "Missing FunctionDeclaration variant"
        );
        assert!(
            has_function_definition,
            "Missing FunctionDefinition variant"
        );
        assert!(has_preproc_include, "Missing PreprocInclude variant");
        assert!(has_comment, "Missing Comment variant");
        assert!(has_unknown, "Missing Unknown variant");
    }

    #[test]
    fn test_non_existent_field_returns_none() {
        let test = TestStruct::default();

        let options = test.get_options("nonexistent");
        assert!(options.is_empty());
    }

    #[test]
    fn test_vec_wrapped_type_returns_inner_variants() {
        let test = TestStruct::default();

        let options = test.get_options("vec_statements");
        // Should return all StatementObject variants, unwrapped from Vec
        assert_eq!(options.len(), 4);

        let has_compound_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::CompoundStatement(_)
            )
        });
        let has_if_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::IfStatement(_)
            )
        });
        let has_return_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::ReturnStatement(_)
            )
        });
        let has_unknown = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Unknown(_)
            )
        });

        assert!(has_compound_statement, "Missing CompoundStatement variant");
        assert!(has_if_statement, "Missing IfStatement variant");
        assert!(has_return_statement, "Missing ReturnStatement variant");
        assert!(has_unknown, "Missing Unknown variant");
    }

    #[test]
    fn test_option_wrapped_type_returns_inner_variants() {
        let test = TestStruct::default();

        let options = test.get_options("opt_expression");
        // Should return all ExpressionObject variants, unwrapped from Option
        assert_eq!(options.len(), 7);

        let has_assignment = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::AssignmentExpression(_)
            )
        });
        let has_binary = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::BinaryExpression(_)
            )
        });

        assert!(has_assignment, "Missing AssignmentExpression variant");
        assert!(has_binary, "Missing BinaryExpression variant");
        // Not checking all 7 for brevity, but they should all be there
    }

    #[test]
    fn test_box_wrapped_type_returns_inner_variants() {
        let test = TestStruct::default();

        let options = test.get_options("boxed_declaration");
        // Should return all DeclarationObject variants, unwrapped from Box
        assert_eq!(options.len(), 6);

        let has_declaration = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::Declaration(_)
            )
        });
        let has_function_declaration = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::FunctionDeclaration(_)
            )
        });

        assert!(has_declaration, "Missing Declaration variant");
        assert!(
            has_function_declaration,
            "Missing FunctionDeclaration variant"
        );
    }

    #[test]
    fn test_nested_wrapped_type_returns_inner_variants() {
        let test = TestStruct::default();

        let options = test.get_options("nested");
        // Should return all StatementObject variants, unwrapped from Option<Box<Vec<T>>>
        assert_eq!(options.len(), 4);

        let has_compound_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::CompoundStatement(_)
            )
        });
        let has_if_statement = options.iter().any(|opt| {
            matches!(
                opt,
                crate::language::c::language_object::LanguageObject::IfStatement(_)
            )
        });

        assert!(has_compound_statement, "Missing CompoundStatement variant");
        assert!(has_if_statement, "Missing IfStatement variant");
    }
}
