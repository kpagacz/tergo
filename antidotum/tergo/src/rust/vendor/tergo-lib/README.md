# `tergo-lib`

## Description

This is the main entry point for `tergo` as a formatting tool.
It is a library, written in Rust, which exposes basic functions
to format source code of the R programming language given
a user-supplied or default formatting configuration.

## Configuration

You can see an example of a configuration file
in the [examples directory](./examples/tergo.toml).

* indent (`i32`): the number of characters constituting a single
    indent. Default: 2.

* line_length (`i32`): the maximum length of the line allowed
    in the formatted output. Default: 120.

* embracing_op_no_nl (`bool`): whether to remove line breaks inside
    the embracing operator (`{{ }}`).
    See: <https://style.tidyverse.org/syntax.html#embracing>
    Default: true.

* allow_nl_after_assignment (`bool`): whether to allow new lines after
    any kind of assignment operator (`=`, `<-`, `:=`) for a very long
    binary expressions. Default: false.

* space_before_complex_rhs_in_formula (`bool`): whether to add a space
    before complex expression in formulas. Default: true.
    See <https://style.tidyverse.org/syntax.html#infix-operators>.

* strip_suffix_whitespace_in_function_defs (`bool`): whether to strip
    any remaining whitespace (including new lines) from ends of the
    function definitions. Example:

    ```R
    function() {
      TRUE

    }
    ```

    If `strip_suffix_whitespace_in_function_defs` is set to `true`, then
    the above is formatted to:

    ```R
    function() {
      TRUE
    }
    ```

    Default: true.

* function_line_breaks (`string`): possible values include:
    `"single"`, `"double"`, `"hanging"`.
    See <https://style.tidyverse.org/functions.html#multi-line-function-definitions>
    `"double"` works the same as `"single"`, except the amount of indent for
    function arguments is doubled. Default: "hanging".

* insert_newline_in_quote_call (`bool`): whether to insert a new line
    in calls to `quote` where the inside expression is very long
    and contains mandatory line breaks (like a closure with `{}`).
    Default: true.
