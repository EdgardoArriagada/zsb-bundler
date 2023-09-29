#[derive(Debug)]
enum Context {
    Normal,
    EmptyLine,
    Comment,
    DoubleQuoteString,
    SingleQuoteString,
    ParamExpansion,
}

pub fn bundle_lines(lines: String) -> String {
    let mut context = Context::Normal;

    let mut result = String::new();
    let mut param_expansion_count = 0;
    let mut array_count = 0;
    let mut prev_char = ' ';
    let mut pre_prev_char = ' ';

    for line in lines.lines() {
        if param_expansion_count > 0 {
            panic!("Unmatched '{{' in line: {}", line);
        }

        match context {
            Context::Comment => context = Context::Normal,
            Context::EmptyLine => {
                if prev_char == ' ' && pre_prev_char != ' ' && pre_prev_char != ';' {
                    result.pop();
                    result.push(';');
                    result.push(' ');
                }
                prev_char = ' ';
                pre_prev_char = ' ';
                context = Context::Normal
            }
            Context::DoubleQuoteString | Context::SingleQuoteString => result.push('\n'),
            _ => match prev_char {
                ' ' => {}
                '\\' => {
                    if pre_prev_char == ' ' {
                        result.pop();
                    }
                }
                ';' => {
                    if pre_prev_char == ';' {
                        result.push(' ');
                    }
                }
                '{' | '(' => result.push(' '),
                _ => {
                    if array_count == 0 {
                        result.push(';');
                    }
                    result.push(' ');
                }
            },
        }

        if line.is_empty() {
            context = Context::EmptyLine;
            continue;
        }

        let mut reached_char = false;

        for c in line.chars() {
            match context {
                Context::Normal => match c {
                    '#' => {
                        if prev_char == '$' || prev_char == '\\' {
                            result.push(c);
                            continue;
                        }

                        context = Context::Comment;
                        if !reached_char {
                            break;
                        }

                        if prev_char == ' ' {
                            result.pop();
                        }

                        if array_count == 0 {
                            result.push(';');
                        }

                        result.push(' ');
                        break;
                    }
                    ' ' => {
                        if reached_char && prev_char != ' ' {
                            result.push(c);
                        }
                    }
                    '"' => {
                        reached_char = true;
                        context = Context::DoubleQuoteString;
                        result.push(c);
                    }
                    '\'' => {
                        reached_char = true;
                        context = Context::SingleQuoteString;
                        result.push(c);
                    }
                    '{' => {
                        reached_char = true;
                        if prev_char == '$' {
                            param_expansion_count += 1;
                            context = Context::ParamExpansion;
                        }
                        result.push(c);
                    }
                    '(' => {
                        reached_char = true;
                        if prev_char == '=' {
                            array_count += 1;
                        }
                        result.push(c);
                    }
                    ')' => {
                        reached_char = true;
                        if array_count > 0 {
                            array_count -= 1;
                        }
                        result.push(c);
                    }
                    _ => {
                        reached_char = true;
                        result.push(c);
                    }
                },
                Context::DoubleQuoteString => match c {
                    '"' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                },
                Context::SingleQuoteString => match c {
                    '\'' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                },
                Context::ParamExpansion => match c {
                    '{' => {
                        param_expansion_count += 1;
                        result.push(c);
                    }
                    '}' => {
                        param_expansion_count -= 1;
                        result.push(c);
                        if param_expansion_count == 0 {
                            context = Context::Normal;
                        }
                    }
                    _ => result.push(c),
                },
                _ => {}
            }

            pre_prev_char = prev_char;
            prev_char = c;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::OnceLock;

    static DIR_CACHE: OnceLock<String> = OnceLock::new();

    pub fn get_dir() -> &'static str {
        DIR_CACHE.get_or_init(|| std::env::current_dir().unwrap().display().to_string())
    }

    fn get_path(file_name: &str, extension: &str) -> String {
        format!("{}/src/test_utils/{}{}", get_dir(), file_name, extension)
    }

    fn get_lines(file_name: &str) -> String {
        std::fs::read_to_string(get_path(file_name, ".zsh")).unwrap()
    }

    fn get_expected(file_name: &str) -> String {
        let mut result = std::fs::read_to_string(get_path(file_name, "_expected.zsh")).unwrap();
        // Remove the last newline
        result.truncate(result.len() - 1);

        result
    }

    fn get_bundled(file_name: &str) -> String {
        bundle_lines(get_lines(file_name))
    }

    #[test]
    fn test_basic_function() {
        let bundled = get_bundled("basic_function");

        let expected = get_expected("basic_function");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_print_string() {
        let bundled = get_bundled("print_string");

        let expected = get_expected("print_string");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn fn_with_comments() {
        let bundled = get_bundled("fn_with_comments");

        let expected = get_expected("fn_with_comments");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_multi_line_string() {
        let bundled = get_bundled("multi_line_string");

        let expected = get_expected("multi_line_string");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_interpolated_string() {
        let bundled = get_bundled("interpolated_string");

        let expected = get_expected("interpolated_string");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_array_len() {
        let bundled = get_bundled("array_len");

        let expected = get_expected("array_len");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_complex_array() {
        let bundled = get_bundled("complex_array");

        let expected = get_expected("complex_array");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_case_keyword() {
        let bundled = get_bundled("case_keyword");

        let expected = get_expected("case_keyword");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_multi_line_instruction() {
        let bundled = get_bundled("multi_line_instruction");

        let expected = get_expected("multi_line_instruction");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_border_case() {
        let bundled = get_bundled("border_case");

        let expected = get_expected("border_case");

        assert_eq!(bundled, expected);
    }
}
