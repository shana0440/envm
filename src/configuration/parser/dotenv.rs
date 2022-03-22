use std::collections::HashMap;

// Note: the comment after the value will stay, however in this project we only care the key.
// not support mulit lines
pub fn parse(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .map(|it| it.trim())
        .filter(|it| !(it.starts_with("#") || it.is_empty() || !it.contains('=')))
        .map(|it| it.splitn(2, '=').collect())
        .map(|it: Vec<&str>| match it.len() {
            1 => (it[0].trim().to_string().clone(), String::new()),
            2 => (
                it[0].trim().to_string().clone(),
                it[1].trim().to_string().clone(),
            ),
            _ => panic!("not a key-value pair: {:?}", it),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_dotenv() {
        let map = parse(
            r#"
        # here is comment

        APP_URL=http://localhost:3000
        "#,
        );

        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get("APP_URL"),
            Some(&String::from("http://localhost:3000"))
        );
    }
}
