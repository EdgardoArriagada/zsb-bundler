#[derive(Debug, PartialEq)]
enum Context {
    Normal,
    EmptyLine,
    DoubleQuoteString,
    SingleQuoteString,
    ParamExpansion,
}

trait VecExt<Char> {
    fn prev(&self) -> char;
    fn pre_prev(&self) -> char;
    fn ppush(&mut self, c: char);
    fn ppush2(&mut self, c1: char, c2: char);
    fn ppop(&mut self) -> Option<char>;
}

impl VecExt<char> for [char; 3] {
    fn prev(&self) -> char {
        self[0]
    }
    fn pre_prev(&self) -> char {
        self[1]
    }
    fn ppush(&mut self, c: char) {
        self[2] = self[1];
        self[1] = self[0];
        self[0] = c;
    }
    fn ppush2(&mut self, c1: char, c2: char) {
        self[2] = self[0];
        self[1] = c1;
        self[0] = c2;
    }
    fn ppop(&mut self) -> Option<char> {
        let result = self[0];
        self[0] = self[1];
        self[1] = self[2];
        self[2] = ' ';
        Some(result)
    }
}

pub fn bundle_lines(lines: String) -> String {
    let mut context = Context::Normal;
    let mut result = String::new();

    let mut ch = [' '; 3];

    let mut param_expansion_count = 0;
    let mut array_count = 0;
    let mut line_count = -1;

    for line in lines.lines() {
        line_count += 1;

        if param_expansion_count > 0 {
            panic!("Unmatched '{{' in line: {}", line);
        }

        match context {
            Context::EmptyLine => context = Context::Normal,
            Context::DoubleQuoteString | Context::SingleQuoteString => {
                result.push('\n');
                ch.ppush('\n');
            }
            _ => match ch.prev() {
                ' ' => {
                    if array_count == 0 && line_count != 0 && ch.pre_prev() != ';' {
                        result.pop();
                        ch.ppop();

                        result.push(';');
                        ch.ppush(';');
                    }
                }
                '\\' => {
                    if ch.pre_prev() == ' ' {
                        result.pop();
                        ch.ppop();
                    }
                }
                ';' => {
                    if ch.pre_prev() == ';' {
                        result.push(' ');
                        ch.ppush(' ');
                    }
                }
                '{' | '(' => {
                    result.push(' ');
                    ch.ppush(' ');
                }
                _ => {
                    if array_count == 0 {
                        result.push(';');
                        ch.ppush(';');
                    }
                    result.push(' ');
                    ch.ppush(' ');
                }
            },
        }

        if context == Context::Normal && line.is_empty() {
            context = Context::EmptyLine;
            continue;
        }

        let mut reached_char = false;

        for c in line.chars() {
            match context {
                Context::Normal => match c {
                    '#' => {
                        if ch.prev() == '$' || ch.prev() == '\\' {
                            result.push(c);
                            ch.ppush(c);
                            continue;
                        }

                        if !reached_char {
                            context = Context::EmptyLine;
                            break;
                        }

                        if ch.prev() == ' ' {
                            result.pop();
                            ch.ppop();
                        }

                        if array_count == 0 {
                            result.push(';');
                            ch.ppush(';');
                        }

                        result.push(' ');
                        ch.ppush(' ');
                        break;
                    }
                    ' ' => {
                        if reached_char && ch.prev() != ' ' {
                            result.push(c);
                            ch.ppush(c);
                        }
                    }
                    '"' => {
                        reached_char = true;
                        context = Context::DoubleQuoteString;
                        result.push(c);
                        ch.ppush(c);
                    }
                    '\'' => {
                        reached_char = true;
                        context = Context::SingleQuoteString;
                        result.push(c);
                        ch.ppush(c);
                    }
                    '{' => {
                        reached_char = true;
                        if ch.prev() == '$' {
                            param_expansion_count += 1;
                            context = Context::ParamExpansion;
                        }
                        result.push(c);
                        ch.ppush(c);
                    }
                    '(' => {
                        reached_char = true;
                        if ch.prev() == '=' {
                            array_count += 1;
                        }
                        result.push(c);
                        ch.ppush(c);
                    }
                    ')' => {
                        reached_char = true;
                        if array_count > 0 {
                            array_count -= 1;
                        }
                        result.push(c);
                        ch.ppush(c);
                    }
                    _ => {
                        reached_char = true;
                        result.push(c);
                        ch.ppush(c);
                    }
                },
                Context::DoubleQuoteString => match c {
                    '"' => {
                        if ch.prev() != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                        ch.ppush(c);
                    }
                    _ => {
                        result.push(c);
                        ch.ppush(c);
                    }
                },
                Context::SingleQuoteString => match c {
                    '\'' => {
                        if ch.prev() != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                        ch.ppush(c);
                    }
                    _ => {
                        result.push(c);
                        ch.ppush(c);
                    }
                },
                Context::ParamExpansion => match c {
                    '{' => {
                        param_expansion_count += 1;
                        result.push(c);
                        ch.ppush(c);
                    }
                    '}' => {
                        param_expansion_count -= 1;
                        result.push(c);
                        ch.ppush(c);
                        if param_expansion_count == 0 {
                            context = Context::Normal;
                        }
                    }
                    _ => {
                        result.push(c);
                        ch.ppush(c);
                    }
                },
                _ => {}
            }
        }
    }

    // add a "; " only if it does not has it
    if !result.ends_with("; ") {
        result.push(';');
        result.push(' ');
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

    #[test]
    fn test_border_case_2() {
        let bundled = get_bundled("border_case_2");

        let expected = get_expected("border_case_2");

        assert_eq!(bundled, expected);
    }
}
