use std::io;
use std::io::Write;
use std::str::Chars;

pub fn parse_command(input: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = vec![];
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
                tokens.push(&input[start..i]);
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
                tokens.push(&input[start..i]);
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
        tokens.push(&input[start..]);
    }

    if quotes.1 {

        let mut vec_in_loop = vec![];

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input_user = String::new();
            match io::stdin().read_line(&mut input_user) {
                Ok(0) => {
                    println!("Syntax error: Unterminated quoted string");   // EOF (Ctrl+D)
                    break
                },
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    continue;
                }
            }
            if ckeck_quote(quotes.0, input_user.clone()) {
                tokens.push(&input[start..]);
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
