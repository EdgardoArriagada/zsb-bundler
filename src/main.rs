enum Context {
    Normal,
    EmptyLine,
    InComment,
    InString,
}

fn main() {
    let home = std::env::var("HOME").unwrap();
    let lines =
        std::fs::read_to_string(format!("{}/.zsh-spell-book/development.zsh", home)).unwrap();

    let mut context = Context::Normal;

    let mut result = String::new();
    let mut prev_char = ' ';

    for line in lines.lines() {
        match context {
            Context::InComment | Context::EmptyLine => context = Context::Normal,
            Context::InString => {}
            _ => match prev_char {
                ' ' | ';' => {}
                _ => result.push(';'),
            },
        }

        if line.is_empty() {
            context = Context::EmptyLine;
            continue;
        }

        for c in line.chars() {
            match context {
                Context::Normal => match c {
                    '#' => {
                        context = Context::InComment;
                    }
                    ' ' => {
                        if prev_char != ' ' {
                            result.push(' ');
                        }
                    }
                    '\'' | '"' => {
                        context = Context::InString;
                        result.push(c);
                    }
                    _ => {
                        result.push(c);
                    }
                },
                Context::InString => match c {
                    '\'' | '"' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                            result.push(c);
                        }
                    }
                    _ => result.push(c),
                },
                _ => {}
            }

            prev_char = c;
        }
    }

    println!("-------------------");

    println!("{}", result);
}
