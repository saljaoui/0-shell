pub fn parse_command(input: &str) -> Vec<&str> {
    let mut vec: Vec<&str> = vec![];
    let mut tokens: Vec<String> = vec![];
    let mut command: String = String::new();
    let mut quotes = (String::new(), false);
    
    for i in input.chars() {
        if i.to_string() == quotes.0 && quotes.1 == true {
            quotes.1 = false;
        } else if (i == '"' || i == '\'') && quotes.1 == false {
            quotes.1 = true;
            quotes.0 = i.to_string();
        } else if quotes.1 {
            command.push(i);
        } else if !i.is_whitespace() {
            command.push(i);
        } else {
            if !command.is_empty() {
                tokens.push(command.clone());
                command.clear();
            }
        }
    }
    
    if !command.is_empty() {
        tokens.push(command);
    }
    
    // Convert to &str references
    let mut res: Vec<&str> = vec![];
    for token in &tokens {
        res.push(token.as_str());
    }
    println!("{:?}", res);
    
    res.clone()
}