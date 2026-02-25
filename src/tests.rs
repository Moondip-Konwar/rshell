use crate::parse_input;

fn assert_parse(input: &str, expected_cmd: &str, expected_args: &[&str]) {
    let (cmd, args) = parse_input(input);

    eprintln!("INPUT: {:?}", input);
    eprintln!("EXPECTED CMD: {:?}", expected_cmd);
    eprintln!("GOT CMD:      {:?}", cmd);
    eprintln!("EXPECTED ARGS: {:?}", expected_args);
    eprintln!("GOT ARGS:      {:?}", args);

    assert_eq!(cmd, expected_cmd, "Command mismatch");
    assert_eq!(
        args,
        expected_args
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        "Args mismatch"
    );
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn test_simple_command() {
        assert_parse("ls", "ls", &[]);
    }

    #[test]
    fn test_command_with_args() {
        assert_parse("echo hello world", "echo", &["hello", "world"]);
    }

    #[test]
    fn test_quoted_arg() {
        assert_parse(r#"echo "Hello world""#, "echo", &["Hello world"]);
    }

    #[test]
    fn test_mixed_quotes() {
        assert_parse(
            r#"echo "Hello world" test 'single quotes'"#,
            "echo",
            &["Hello world", "test", "single quotes"],
        );
    }

    #[test]
    fn test_empty_input() {
        assert_parse("", "", &[]);
    }

    #[test]
    fn test_spaces_only() {
        assert_parse("   ", "", &[]);
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_parse("   echo  hello   world   ", "echo", &["hello", "world"]);
    }
}
