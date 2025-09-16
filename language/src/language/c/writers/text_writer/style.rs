pub struct Style {
    pub indent_size: usize,
    pub function_declaration_parameter_count_linesplit_threshold: usize,
    pub fn_call_max_arg_per_line: usize,
    pub block_always_below: bool,
    pub function_type_always_above: bool,
}

impl Style {
    pub fn new(
        indent_size: usize,
        function_declaration_parameter_count_linesplit_threshold: usize,
        function_call_argument_count_linesplit_threshold: usize,
        block_always_below: bool,
        function_type_always_above: bool,
    ) -> Self {
        Style {
            indent_size,
            function_declaration_parameter_count_linesplit_threshold,
            fn_call_max_arg_per_line: function_call_argument_count_linesplit_threshold,
            block_always_below,
            function_type_always_above,
        }
    }

    pub fn gnu_style() -> Self {
        Style {
            indent_size: 4,
            function_declaration_parameter_count_linesplit_threshold: 4,
            fn_call_max_arg_per_line: 4,
            block_always_below: true,
            function_type_always_above: true,
        }
    }

    pub fn kernel_style() -> Self {
        Style {
            indent_size: 8,
            function_declaration_parameter_count_linesplit_threshold: 4,
            fn_call_max_arg_per_line: 2,
            block_always_below: true,
            function_type_always_above: false,
        }
    }

    pub fn rust_style() -> Self {
        Style {
            indent_size: 4,
            function_declaration_parameter_count_linesplit_threshold: 0,
            fn_call_max_arg_per_line: 0,
            block_always_below: false,
            function_type_always_above: false,
        }
    }
}
