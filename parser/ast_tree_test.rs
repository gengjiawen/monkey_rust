#[cfg(test)]
mod tests {
    use crate::parse;
    use insta::*;

    pub fn test_ast_tree(name: &str, input: &str) {
        let let_ast = match parse(input) {
            Ok(node) => {
                serde_json::to_string_pretty(&node).unwrap()
            }
            Err(e) => format!("parse error: {}", e[0])
        };
        assert_snapshot!(name, let_ast);
    }

    #[test]
    fn test_let() {
        let input = "let a = 3";
        test_ast_tree("test_let", input)
    }

    #[test]
    fn test_string() {
        let input = r#""jw""#;
        test_ast_tree("test_string", input)
    }

    #[test]
    fn test_array() {
        let input = "[1, true]";
        test_ast_tree("test_array", input)
    }

    #[test]
    fn test_hash() {
        let input = r#"{"a": 1}"#;
        test_ast_tree("test_hash", input)
    }

    #[test]
    fn test_return() {
        let input = "return 3";
        test_ast_tree("test_return", input)
    }

    #[test]
    fn test_unary() {
        let input = "-3";
        test_ast_tree("test_unary", input)
    }

    #[test]
    fn test_binary() {
        let input = "1 + 2 * 3";
        test_ast_tree("test_binary", input)
    }

    #[test]
    fn test_if() {
        let input = "if (x < y) { x } else { y }";
        test_ast_tree("test_if", input)
    }

    #[test]
    fn test_func_declaration() {
        let input = "fn(x) { x };";
        test_ast_tree("test_func_declaration", input)
    }

    #[test]
    fn test_func_call() {
        let input = "add(1, 2)";
        test_ast_tree("test_func_call", input)
    }
}