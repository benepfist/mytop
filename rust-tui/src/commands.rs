#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandFeedback {
    Ok(String),
    Error(String),
}

pub fn parse_thread_id(input: &str) -> CommandFeedback {
    match input.trim().parse::<u64>() {
        Ok(id) => CommandFeedback::Ok(format!("thread={id}")),
        Err(_) => CommandFeedback::Error("*** Invalid id. ***".to_string()),
    }
}

pub fn set_delay_secs(input: &str) -> u64 {
    input.trim().parse::<u64>().unwrap_or(1).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_thread_id_returns_user_feedback() {
        assert_eq!(
            parse_thread_id("abc"),
            CommandFeedback::Error("*** Invalid id. ***".to_string())
        );
        assert_eq!(
            parse_thread_id("42"),
            CommandFeedback::Ok("thread=42".to_string())
        );
    }

    #[test]
    fn delay_has_minimum_of_one() {
        assert_eq!(set_delay_secs("0"), 1);
        assert_eq!(set_delay_secs("3"), 3);
    }
}
