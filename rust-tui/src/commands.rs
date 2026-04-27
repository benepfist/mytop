use crate::filters::StringOrRegex;

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

pub fn parse_kill_command(input: &str) -> CommandFeedback {
    match parse_thread_id(input) {
        CommandFeedback::Ok(thread) => CommandFeedback::Ok(format!("kill {thread}")),
        CommandFeedback::Error(err) => CommandFeedback::Error(err),
    }
}

pub fn parse_explain_command(input: &str) -> CommandFeedback {
    match parse_thread_id(input) {
        CommandFeedback::Ok(thread) => CommandFeedback::Ok(format!("explain {thread}")),
        CommandFeedback::Error(err) => CommandFeedback::Error(err),
    }
}

pub fn plan_kill_thread(input: &str, confirmed: bool) -> CommandFeedback {
    match parse_thread_id(input) {
        CommandFeedback::Ok(thread) if confirmed => {
            let id = thread.trim_start_matches("thread=");
            CommandFeedback::Ok(format!("KILL {id}"))
        }
        CommandFeedback::Ok(_) => {
            CommandFeedback::Error("*** Confirmation required for KILL thread. ***".to_string())
        }
        CommandFeedback::Error(err) => CommandFeedback::Error(err),
    }
}

pub fn plan_kill_user(user: &str, confirmed: bool) -> CommandFeedback {
    let user = user.trim();
    if user.is_empty() {
        return CommandFeedback::Error("*** Invalid user. ***".to_string());
    }

    if !confirmed {
        return CommandFeedback::Error("*** Confirmation required for KILL user. ***".to_string());
    }

    if matches!(user, "root" | "mysql.sys" | "replication") {
        return CommandFeedback::Error("*** Refusing dangerous user kill. ***".to_string());
    }

    CommandFeedback::Ok(format!("KILL USER {user}"))
}

pub fn parse_filter_value(input: &str) -> StringOrRegex {
    StringOrRegex::parse(input)
}

pub fn parse_sort_order(input: &str) -> CommandFeedback {
    match input.trim().to_ascii_lowercase().as_str() {
        "asc" | "time_asc" => CommandFeedback::Ok("sort=time_asc".to_string()),
        "desc" | "time_desc" | "" => CommandFeedback::Ok("sort=time_desc".to_string()),
        _ => CommandFeedback::Error("*** Invalid sort. Use asc|desc. ***".to_string()),
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

    #[test]
    fn kill_and_explain_use_thread_validation() {
        assert_eq!(
            parse_kill_command("7"),
            CommandFeedback::Ok("kill thread=7".to_string())
        );
        assert_eq!(
            parse_explain_command("7"),
            CommandFeedback::Ok("explain thread=7".to_string())
        );
        assert_eq!(
            parse_kill_command("oops"),
            CommandFeedback::Error("*** Invalid id. ***".to_string())
        );
    }

    #[test]
    fn kill_plans_require_confirmation_and_safety_guard() {
        assert_eq!(
            plan_kill_thread("8", false),
            CommandFeedback::Error("*** Confirmation required for KILL thread. ***".to_string())
        );
        assert_eq!(
            plan_kill_thread("8", true),
            CommandFeedback::Ok("KILL 8".to_string())
        );

        assert_eq!(
            plan_kill_user("alice", false),
            CommandFeedback::Error("*** Confirmation required for KILL user. ***".to_string())
        );
        assert_eq!(
            plan_kill_user("root", true),
            CommandFeedback::Error("*** Refusing dangerous user kill. ***".to_string())
        );
        assert_eq!(
            plan_kill_user("alice", true),
            CommandFeedback::Ok("KILL USER alice".to_string())
        );
    }

    #[test]
    fn sort_input_is_validated() {
        assert_eq!(
            parse_sort_order("desc"),
            CommandFeedback::Ok("sort=time_desc".to_string())
        );
        assert_eq!(
            parse_sort_order("asc"),
            CommandFeedback::Ok("sort=time_asc".to_string())
        );
        assert_eq!(
            parse_sort_order("random"),
            CommandFeedback::Error("*** Invalid sort. Use asc|desc. ***".to_string())
        );
    }
}
