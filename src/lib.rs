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
        println!("Syntax error: Unterminated quoted string");
        return vec![]
    }

    tokens
}
