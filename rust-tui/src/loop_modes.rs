#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Top,
    Qps,
    Cmd,
    Innodb,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Top,
    Qps,
    Cmd,
    Innodb,
    Status,
    Quit,
    Other,
}

pub fn parse_key(input: char) -> Key {
    match input {
        't' => Key::Top,
        'm' => Key::Qps,
        'c' => Key::Cmd,
        'I' => Key::Innodb,
        'S' => Key::Status,
        'q' => Key::Quit,
        _ => Key::Other,
    }
}

pub fn switch_mode(current: Mode, key: Key) -> Option<Mode> {
    match key {
        Key::Top => Some(Mode::Top),
        Key::Qps => Some(Mode::Qps),
        Key::Cmd => Some(Mode::Cmd),
        Key::Innodb => Some(Mode::Innodb),
        Key::Status => Some(Mode::Status),
        Key::Quit => None,
        Key::Other => Some(current),
    }
}

pub fn run_cycles(batchmode: bool) -> usize {
    if batchmode { 1 } else { 2 }
}

pub fn run_event_loop_with_keys(initial: Mode, batchmode: bool, keys: &[char]) -> Option<Mode> {
    let mut mode = initial;
    for (idx, ch) in keys.iter().enumerate() {
        if batchmode && idx >= 1 {
            break;
        }
        let key = parse_key(*ch);
        let next = switch_mode(mode, key)?;
        mode = next;
    }
    Some(mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batchmode_runs_one_cycle() {
        assert_eq!(run_cycles(true), 1);
        assert_eq!(run_cycles(false), 2);
    }

    #[test]
    fn mode_switch_keys_are_supported() {
        assert_eq!(switch_mode(Mode::Top, Key::Cmd), Some(Mode::Cmd));
        assert_eq!(switch_mode(Mode::Cmd, Key::Innodb), Some(Mode::Innodb));
        assert_eq!(switch_mode(Mode::Innodb, Key::Status), Some(Mode::Status));
    }

    #[test]
    fn quit_exits_loop() {
        assert_eq!(switch_mode(Mode::Top, Key::Quit), None);
        assert_eq!(run_event_loop_with_keys(Mode::Top, false, &['m', 'q']), None);
    }

    #[test]
    fn parse_key_maps_supported_shortcuts() {
        assert_eq!(parse_key('t'), Key::Top);
        assert_eq!(parse_key('m'), Key::Qps);
        assert_eq!(parse_key('x'), Key::Other);
    }
}
