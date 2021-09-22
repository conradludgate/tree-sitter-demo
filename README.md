# tree-sitter-demo

Just a demo parsing a go file using the [tree-sitter](https://tree-sitter.github.io) library

## Example

```
cargo run
```

Output:
```
source_file(
    package_clause(
        "package",
        "main",
    ),
    "\n\n",
    import_declaration(
        "import",
        import_spec(
            "\"fmt\"",
        ),
    ),
    "\n\n",
    type_declaration(
        "type",
        type_spec(
            "Fib",
            struct_type(
                "struct",
                field_declaration_list(
                    "{",
                    field_declaration(
                        "a",
                        ",",
                        "b",
                        "int",
                    ),
                    "\n",
                    "}",
                ),
            ),
        ),
    ),
    "\n\n",
    method_declaration(
        "func",
        parameter_list(
            "(",
            parameter_declaration(
                "f",
                pointer_type(
                    "*",
                    "Fib",
                ),
            ),
            ")",
        ),
        "Next",
        "()",
        "int",
        block(
            "{",
            short_var_declaration(
                expression_list(
                    "a",
                    ",",
                    "b",
                ),
                ":=",
                expression_list(
                    selector_expression(
                        "f",
                        ".",
                        "a",
                    ),
                    ",",
                    selector_expression(
                        "f",
                        ".",
                        "b",
                    ),
                ),
            ),
            "\n",
            assignment_statement(
                expression_list(
                    selector_expression(
                        "f",
                        ".",
                        "a",
                    ),
                ),
                "=",
                expression_list(
                    "b",
                ),
            ),
            "\n",
            assignment_statement(
                expression_list(
                    selector_expression(
                        "f",
                        ".",
                        "b",
                    ),
                ),
                "=",
                expression_list(
                    binary_expression(
                        "b",
                        "+",
                        "a",
                    ),
                ),
            ),
            "\n",
            return_statement(
                "return",
                expression_list(
                    "a",
                ),
            ),
            "\n",
            "}",
        ),
    ),
    "\n\n",
    function_declaration(
        "func",
        "foo",
        parameter_list(
            "(",
            parameter_declaration(
                "bar",
                "string",
            ),
            ")",
        ),
        parameter_list(
            "(",
            parameter_declaration(
                "baz",
                "int",
            ),
            ")",
        ),
        block(
            "{",
            assignment_statement(
                expression_list(
                    "baz",
                ),
                "=",
                expression_list(
                    call_expression(
                        "len",
                        argument_list(
                            "(",
                            "bar",
                            ")",
                        ),
                    ),
                ),
            ),
            "\n",
            "return",
            "\n",
            "}",
        ),
    ),
    "\n\n",
    function_declaration(
        "func",
        "main",
        "()",
        block(
            "{",
            short_var_declaration(
                expression_list(
                    "fib",
                ),
                ":=",
                expression_list(
                    composite_literal(
                        "Fib",
                        literal_value(
                            "{",
                            element(
                                "0x0",
                            ),
                            ",",
                            element(
                                "1",
                            ),
                            "}",
                        ),
                    ),
                ),
            ),
            "\n",
            for_statement(
                "for",
                for_clause(
                    short_var_declaration(
                        expression_list(
                            "i",
                        ),
                        ":=",
                        expression_list(
                            "0",
                        ),
                    ),
                    ";",
                    binary_expression(
                        "i",
                        "<",
                        "10",
                    ),
                    ";",
                    inc_statement(
                        "i",
                        "++",
                    ),
                ),
                block(
                    "{",
                    call_expression(
                        selector_expression(
                            "fmt",
                            ".",
                            "Println",
                        ),
                        argument_list(
                            "(",
                            call_expression(
                                selector_expression(
                                    "fib",
                                    ".",
                                    "Next",
                                ),
                                "()",
                            ),
                            ")",
                        ),
                    ),
                    "\n",
                    "}",
                ),
            ),
            "\n",
            "}",
        ),
    ),
    "\n",
)
```