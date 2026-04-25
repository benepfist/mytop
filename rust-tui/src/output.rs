use std::collections::BTreeMap;

pub fn format_show_variables(rows: &BTreeMap<String, String>) -> String {
    rows.iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn innodb_status_text(status: &str) -> String {
    status.to_string()
}

pub fn find_pager(available: &[&str]) -> String {
    if available.contains(&"less") {
        "less".to_string()
    } else {
        "more".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pager_prefers_less_with_more_fallback() {
        assert_eq!(find_pager(&["cat", "less"]), "less");
        assert_eq!(find_pager(&["cat"]), "more");
    }

    #[test]
    fn variables_are_formatted_as_key_value() {
        let mut v = BTreeMap::new();
        v.insert("autocommit".into(), "ON".into());
        assert_eq!(format_show_variables(&v), "autocommit: ON");
    }
}
