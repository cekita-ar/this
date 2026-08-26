use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct Placeholder {
    pub raw: String,
    pub name: String,
    pub required: bool,
    pub options: Option<Vec<String>>,
}

pub fn extract_placeholders(template: &str) -> Vec<Placeholder> {
    let pattern = r"\{\{\s*(?P<name>[a-zA-Z0-9_-]+)(?P<opt>\?)?(?::(?P<choices>[^}]+))?\s*\}\}";
    let re = Regex::new(pattern).unwrap();

    let mut placeholders = Vec::new();

    for caps in re.captures_iter(template) {
        let raw = caps[0].to_string();
        let name = caps.name("name").unwrap().as_str().to_string();
        let required = caps.name("opt").is_none(); // Si no hay '?', es obligatorio

        let options = caps.name("choices").map(|c| {
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
            options,
        });
    }

    placeholders
}