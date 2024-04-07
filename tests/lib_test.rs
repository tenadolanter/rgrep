#[cfg(test)]
mod tests {
    use rgrep::search;
    use rgrep::search_case_insensitive;

    #[test]
    fn on_result() {
        let query = "productive";
        let content = "\
Rust:
fast, safe, productive.
Pick three.";
        assert_eq!(vec!["fast, safe, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "productive";
        let content = "\
Rust:
fast, safe, productive.
Pick three.";
        assert_eq!(vec!["fast, safe, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        let content = "\
Rust:
fast, safe, productive.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
