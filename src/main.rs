enum Context {
    Normal,
    EmptyLine,
    InComment,
    InString,
}

fn bundle_lines(lines: String) -> String {
    let mut context = Context::Normal;

    let mut result = String::new();
    let mut prev_char = ' ';

    for line in lines.lines() {
        match context {
            Context::InComment | Context::EmptyLine => context = Context::Normal,
            Context::InString => {}
            _ => match prev_char {
                ' ' | ';' => {}
                '{' => result.push(' '),
                _ => result.push(';'),
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
                        context = Context::InComment;
                        break;
                    }
                    ' ' => {
                        if reached_char && prev_char != ' ' {
                            result.push(c);
                        }
                    }
                    '\'' | '"' => {
                        reached_char = true;
                        context = Context::InString;
                        result.push(c);
                    }
                    _ => {
                        reached_char = true;
                        result.push(c);
                    }
                },
                Context::InString => match c {
                    '\'' | '"' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                },
                _ => {}
            }

            prev_char = c;
        }
    }

    result
}

fn main() {
    let home = std::env::var("HOME").unwrap();

    let path = format!("{}/.zsh-spell-book/development.zsh", home);

    let lines = std::fs::read_to_string(path).unwrap();

    let result = bundle_lines(lines);

    println!("-------------------");

    println!("{}", result);
}
