use std::io;
use std::io::Write;

pub fn parse_command(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut start = 0;
    let mut in_token = false;
    let mut quotes = ('\0', false);

    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        let c = chars[i];

        if quotes.1 {
            if c == quotes.0 {
                quotes.1 = false;
                tokens.push(input[start..i].to_string());
                in_token = false;
            }
            i += 1;
            continue;
        }

        if c == '"' || c == '\'' {
            quotes.0 = c;
            quotes.1 = true;
            start = i + 1;
            in_token = true;
            i += 1;
            continue;
        }

        if c.is_whitespace() {
            if in_token {
                tokens.push(input[start..i].to_string());
                in_token = false;
            }
            i += 1;
            continue;
        }

        if !in_token {
            start = i;
            in_token = true;
        }

        i += 1;
    }

    if in_token && !quotes.1 {
        tokens.push(input[start..].to_string());
    }

    if quotes.1 {
        let mut vec_in_loop = vec![];

        loop {
            print!("> ");
            match io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{e}");
                    return vec![];
                }
            };

            let mut input_user = String::new();
            match io::stdin().read_line(&mut input_user) {
                Ok(0) => {
                    eprintln!("Syntax error: Unterminated quoted string"); // EOF (Ctrl+D)
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    continue;
                }
            }
            if ckeck_quote(quotes.0, input_user.clone()) {
                tokens.push(input[start..].to_string());
                let correct_word: String = input_user.replace('\'', "");
                vec_in_loop.push(correct_word);

                if tokens[0] == "echo" {
                    let mut first = false;
                    for t in tokens.clone() {
                        if !first {
                            first = true
                        } else {
                            println!("{}", t)
                        }
                    }
                    for v in vec_in_loop {
                        print!("{}", v)
                    }
                    return vec![];
                }

                break;
            } else {
                vec_in_loop.push(input_user);
            }
        }

        let joined = format!("\n{}", vec_in_loop.join(""));
        let escaped = joined.replace("\"", "");
        if tokens.len()>1{
            tokens[1].push_str(&escaped);
        }
        return tokens;
    }

    tokens
}

fn ckeck_quote(quote: char, input: String) -> bool {
    let mut quotes = 0;
    for i in input.chars() {
        if i == quote {
            quotes += 1
        }
    }
    if quotes == 1 {
        return true;
    }
    false
}
