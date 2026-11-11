use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct Placeholder {
    pub raw: String,
    pub name: String,
    pub required: bool,
    pub choices: Option<Vec<String>>,
}

pub fn extract_placeholders(template: &str) -> Vec<Placeholder> {
    let pattern = r"\{\{(?P<name>[\w\s]+)(?P<required>!)?\s*(?::(?P<choices>[^}]+))?\}\}";
    let re = Regex::new(pattern).unwrap();

    let mut placeholders = Vec::new();

    for caps in re.captures_iter(template) {
        let raw = caps[0].to_string();
        let name = caps.name("name").unwrap().as_str().to_string();
        let required = caps.name("required").is_some();

        let choices = caps.name("choices").map(|c| {
            c.as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        });

        placeholders.push(Placeholder {
            raw,
            name,
            required,
            choices,
        });
    }

    placeholders
}
