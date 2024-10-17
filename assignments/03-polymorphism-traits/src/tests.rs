#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};

    #[test]
    fn test_stdin_input() {
        let input = b"Hello\nworld\nHello world\nhello world";
        let mut command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        {
            let stdin = command.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(input).expect("Failed to write to stdin");
        }
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout, "Hello\nHello world\n");
    }

    #[test]
    fn test_file_input() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .arg("text.txt")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout, "Hello\nHello world\nAnother line with Hello\n");
    }

    #[test]
    fn test_regex() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("H.llo")
            .arg("text.txt")
            .arg("--regex")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout,
            "Hello\nHello world\nH3llo w0rld\nRegex test: H.llo\nAnother line with Hello\n"
        );
    }

    #[test]
    fn test_non_regex() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("H.llo")
            .arg("text.txt")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout, "Regex test: H.llo\n");
    }

    #[test]
    fn test_ignore_case() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("HeLlO")
            .arg("text.txt")
            .arg("--ignore-case")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout,
            "Hello\nHello world\nhello WORLD\nAnother line with Hello\n"
        );
    }

    #[test]
    fn test_invert_match() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .arg("text.txt")
            .arg("--invert-match")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout,
            "world\nhello WORLD\nRust is great\nrust is awesome\nH3llo w0rld\nSpecial characters: !@#$%^&*()\nRegex test: H.llo\nNo match here\n"
        );
    }

    #[test]
    fn test_regex_ignore_case_invert_match() {
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("H.llo")
            .arg("text.txt")
            .arg("--regex")
            .arg("--ignore-case")
            .arg("--invert-match")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
                stdout,
                "world\nRust is great\nrust is awesome\nSpecial characters: !@#$%^&*()\nNo match here\n"
            );
    }

    #[test]
    fn test_red() {
        // Set the environment variable to force colored output
        // (this took me so long to find :crying_emoji:)
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .arg("text.txt")
            .arg("--color=Red")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hello = "\x1b[31mHello\x1b[0m"; // "Hello" with ANSI codes for red
        assert_eq!(
            stdout,
            format!("{}\n{} world\nAnother line with {}\n", hello, hello, hello)
        );
    }

    #[test]
    fn test_white() {
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .arg("text.txt")
            .arg("--color=White")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hello = "\x1b[37mHello\x1b[0m"; // "Hello" with ANSI codes for white
        assert_eq!(
            stdout,
            format!("{}\n{} world\nAnother line with {}\n", hello, hello, hello)
        );
    }

    #[test]
    fn test_color_with_regex() {
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("H.llo")
            .arg("text.txt")
            .arg("--color=Red")
            .arg("--regex")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hello = "\x1b[31mHello\x1b[0m"; // "Hello" with ANSI codes for red
        let h3llo = "\x1b[31mH3llo\x1b[0m"; // "H3llo" with ANSI codes for red
        let h_dot_llo = "\x1b[31mH.llo\x1b[0m"; // "H.llo" with ANSI codes for red

        assert_eq!(
            stdout,
            format!(
                "{}\n{} world\n{} w0rld\nRegex test: {}\nAnother line with {}\n",
                hello, hello, h3llo, h_dot_llo, hello
            )
        );
    }

    #[test]
    fn test_color_with_ignore_case() {
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("HeLlO")
            .arg("text.txt")
            .arg("--color=Red")
            .arg("--ignore-case")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let big_hello = "\x1b[31mHello\x1b[0m"; // "Hello" with ANSI codes for red
        let little_hello = "\x1b[31mhello\x1b[0m"; // "hello" with ANSI codes for red

        assert_eq!(
            stdout,
            format!(
                "{}\n{} world\n{} WORLD\nAnother line with {}\n",
                big_hello, big_hello, little_hello, big_hello
            )
        );
    }

    #[test]
    fn test_color_with_invert_match() {
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("Hello")
            .arg("text.txt")
            .arg("--color=Red")
            .arg("--invert-match")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert_eq!(
            stdout,
            "world\nhello WORLD\nRust is great\nrust is awesome\nH3llo w0rld\nSpecial characters: !@#$%^&*()\nRegex test: H.llo\nNo match here\n"
        );
    }

    #[test]
    fn test_color_with_regex_ignore_case() {
        env::set_var("CLICOLOR_FORCE", "1");
        let command = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("H.llo")
            .arg("text.txt")
            .arg("--color=Red")
            .arg("--regex")
            .arg("--ignore-case")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cargo run");
        let output = command.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let big_hello = "\x1b[31mHello\x1b[0m"; // "Hello" with ANSI codes for red
        let h3llo = "\x1b[31mH3llo\x1b[0m"; // "H3llo"
        let h_dot_llo = "\x1b[31mH.llo\x1b[0m"; // "H.llo"
        let little_hello = "\x1b[31mhello\x1b[0m"; // "hello"

        assert_eq!(
            stdout,
            format!(
                "{}\n{} world\n{} WORLD\n{} w0rld\nRegex test: {}\nAnother line with {}\n",
                big_hello, big_hello, little_hello, h3llo, h_dot_llo, big_hello
            )
        );
    }
}
