use crate::commands::{
    CommandFeedback, parse_explain_command, parse_filter_value, parse_kill_command,
    parse_reset_command, parse_sort_order, set_delay_secs,
};
use crate::filters::Filters;
use crate::loop_modes::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptMode {
    KillThread,
    ExplainThread,
    FilterUser,
    FilterDb,
    FilterHost,
    SetDelay,
    SetSort,
    ResetFilters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractiveState {
    pub mode: Mode,
    pub prompt: Option<PromptMode>,
    pub running: bool,
    pub delay_secs: u64,
    pub sort_desc: bool,
    pub filters: Filters,
    pub last_feedback: Option<String>,
}

impl Default for InteractiveState {
    fn default() -> Self {
        Self {
            mode: Mode::Top,
            prompt: None,
            running: true,
            delay_secs: 5,
            sort_desc: true,
            filters: Filters::default(),
            last_feedback: None,
        }
    }
}

pub fn handle_keypress(state: &mut InteractiveState, key: char) {
    match key {
        't' => state.mode = Mode::Top,
        'm' => state.mode = Mode::Qps,
        'c' => state.mode = Mode::Cmd,
        'I' => state.mode = Mode::Innodb,
        'S' => state.mode = Mode::Status,
        'q' => state.running = false,
        'k' => state.prompt = Some(PromptMode::KillThread),
        'e' => state.prompt = Some(PromptMode::ExplainThread),
        'u' => state.prompt = Some(PromptMode::FilterUser),
        'd' => state.prompt = Some(PromptMode::FilterDb),
        'h' => state.prompt = Some(PromptMode::FilterHost),
        's' => state.prompt = Some(PromptMode::SetDelay),
        'o' => state.prompt = Some(PromptMode::SetSort),
        'r' => state.prompt = Some(PromptMode::ResetFilters),
        _ => {}
    }
}

pub fn submit_prompt(state: &mut InteractiveState, input: &str) -> Option<CommandFeedback> {
    let prompt = state.prompt?;

    let feedback = match prompt {
        PromptMode::KillThread => parse_kill_command(input),
        PromptMode::ExplainThread => parse_explain_command(input),
        PromptMode::FilterUser => {
            state.filters.user = parse_filter_value(input);
            CommandFeedback::Ok("filter=user updated".to_string())
        }
        PromptMode::FilterDb => {
            state.filters.db = parse_filter_value(input);
            CommandFeedback::Ok("filter=db updated".to_string())
        }
        PromptMode::FilterHost => {
            state.filters.host = parse_filter_value(input);
            CommandFeedback::Ok("filter=host updated".to_string())
        }
        PromptMode::SetDelay => {
            state.delay_secs = set_delay_secs(input);
            CommandFeedback::Ok(format!("delay={}", state.delay_secs))
        }
        PromptMode::SetSort => match parse_sort_order(input) {
            CommandFeedback::Ok(v) => {
                state.sort_desc = !v.contains("time_asc");
                CommandFeedback::Ok(v)
            }
            CommandFeedback::Error(e) => CommandFeedback::Error(e),
        },
        PromptMode::ResetFilters => match parse_reset_command(input) {
            CommandFeedback::Ok(msg) => {
                state.filters.reset();
                CommandFeedback::Ok(msg)
            }
            CommandFeedback::Error(err) => CommandFeedback::Error(err),
        },
    };

    state.last_feedback = Some(match &feedback {
        CommandFeedback::Ok(msg) => msg.clone(),
        CommandFeedback::Error(msg) => msg.clone(),
    });
    state.prompt = None;

    Some(feedback)
}

pub fn run_cycles(batchmode: bool) -> usize {
    if batchmode { 1 } else { usize::MAX }
}

pub fn run_key_sequence(batchmode: bool, keys: &[char]) -> InteractiveState {
    let mut state = InteractiveState::default();
    let max_cycles = run_cycles(batchmode);

    for (idx, key) in keys.iter().enumerate() {
        if idx >= max_cycles || !state.running {
            break;
        }
        handle_keypress(&mut state, *key);
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::StringOrRegex;

    #[test]
    fn batchmode_processes_only_one_cycle() {
        let state = run_key_sequence(true, &['m', 'c']);
        assert_eq!(state.mode, Mode::Qps);
    }

    #[test]
    fn keybinding_matrix_switches_modes_and_quits() {
        let mut state = InteractiveState::default();
        handle_keypress(&mut state, 'c');
        assert_eq!(state.mode, Mode::Cmd);
        handle_keypress(&mut state, 'I');
        assert_eq!(state.mode, Mode::Innodb);
        handle_keypress(&mut state, 'S');
        assert_eq!(state.mode, Mode::Status);
        handle_keypress(&mut state, 'q');
        assert!(!state.running);
    }

    #[test]
    fn prompt_modes_handle_commands_and_validation_feedback() {
        let mut state = InteractiveState::default();

        handle_keypress(&mut state, 'k');
        let msg = submit_prompt(&mut state, "not-a-number");
        assert_eq!(
            msg,
            Some(CommandFeedback::Error("*** Invalid id. ***".to_string()))
        );

        handle_keypress(&mut state, 'e');
        let msg = submit_prompt(&mut state, "42");
        assert_eq!(
            msg,
            Some(CommandFeedback::Ok("explain thread=42".to_string()))
        );
    }

    #[test]
    fn prompt_modes_update_filters_delay_and_sort() {
        let mut state = InteractiveState::default();

        handle_keypress(&mut state, 'u');
        let _ = submit_prompt(&mut state, "/report/");
        assert_eq!(state.filters.user, StringOrRegex::Pattern("report".into()));

        handle_keypress(&mut state, 'd');
        let _ = submit_prompt(&mut state, "analytics");
        assert_eq!(state.filters.db, StringOrRegex::Exact("analytics".into()));

        handle_keypress(&mut state, 's');
        let _ = submit_prompt(&mut state, "0");
        assert_eq!(state.delay_secs, 1);

        handle_keypress(&mut state, 'o');
        let _ = submit_prompt(&mut state, "asc");
        assert!(!state.sort_desc);

        handle_keypress(&mut state, 'r');
        let _ = submit_prompt(&mut state, "filters");
        assert_eq!(state.filters, Filters::default());
    }
}
