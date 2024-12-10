pub fn split_string_respect_braces(input_string: Option<String>) -> Vec<String> {
    match input_string {
        None => Vec::new(),
        Some(string) => {
            let mut result = Vec::new();
            let mut current_string = String::new();
            let mut brace_count = 0;

            for character in string.chars() {
                match character {
                    '{' => {
                        brace_count += 1;
                        current_string.push(character);
                    }
                    '}' => {
                        brace_count -= 1;
                        current_string.push(character);
                    }
                    ',' if brace_count == 0 => {
                        if !current_string.is_empty() {
                            result.push(current_string.trim().to_string());
                            current_string = String::new();
                        }
                    }
                    _ => current_string.push(character),
                }
            }

            if !current_string.is_empty() {
                result.push(current_string.trim().to_string());
            }

            result
        }
    }
}
