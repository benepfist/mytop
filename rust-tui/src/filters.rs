#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringOrRegex {
    MatchAll,
    Exact(String),
    Pattern(String),
}

impl StringOrRegex {
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Self::MatchAll;
        }
        if trimmed.starts_with('/') && trimmed.ends_with('/') && trimmed.len() >= 2 {
            return Self::Pattern(trimmed[1..trimmed.len() - 1].to_string());
        }
        Self::Exact(trimmed.to_string())
    }

    pub fn matches(&self, value: &str) -> bool {
        match self {
            Self::MatchAll => true,
            Self::Exact(s) => value == s,
            Self::Pattern(p) => value.contains(p),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filters {
    pub user: StringOrRegex,
    pub db: StringOrRegex,
    pub host: StringOrRegex,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            user: StringOrRegex::MatchAll,
            db: StringOrRegex::MatchAll,
            host: StringOrRegex::MatchAll,
        }
    }
}

impl Filters {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rules_are_applied() {
        assert_eq!(StringOrRegex::parse(""), StringOrRegex::MatchAll);
        assert_eq!(
            StringOrRegex::parse("alice"),
            StringOrRegex::Exact("alice".into())
        );
        assert_eq!(
            StringOrRegex::parse("/report/"),
            StringOrRegex::Pattern("report".into())
        );
    }

    #[test]
    fn reset_sets_match_all() {
        let mut f = Filters {
            user: StringOrRegex::Exact("x".into()),
            db: StringOrRegex::Exact("y".into()),
            host: StringOrRegex::Exact("z".into()),
        };
        f.reset();
        assert_eq!(f, Filters::default());
    }
}
