fn push_till(chars: &[char], from: usize, till: char) -> (String, usize) {
    let mut i: usize = from;
    let mut substr = String::new();
    while i < chars.len() && chars[i] != till {
        substr += chars[i].to_string().as_str();
        i += 1;
    }

    (substr, i)
}

pub fn parse_input(input: &str) -> (String, Vec<String>) {
    let mut args: Vec<String> = Vec::new();
    let chars: Vec<char> = input.trim_start().chars().collect();

    // Parse command
    let (command, mut i) = push_till(&chars, 0, ' ');

    // Parse args
    while i < chars.len() {
        let arg: String;

        match chars[i] {
            // Skip whitespace
            ' ' => {
                i += 1;
                continue;
            }

            // Parse double quotes string
            '"' => {
                i += 1; // Skip the starting "
                (arg, i) = push_till(&chars, i, '"');

                i += 1; // Skip the ending "
                args.push(arg);
                continue; // Go back to loop start
            }

            // Parse double quotes string
            '\'' => {
                i += 1; // Skip the starting '
                (arg, i) = push_till(&chars, i, '\'');

                i += 1; // Skip the ending '
                args.push(arg);
                continue; // Go back to loop start
            }

            // Parse flag
            _ => {
                (arg, i) = push_till(&chars, i, ' ');
                args.push(arg);
                continue;
            }
        }
    }

    (command, args)
}
