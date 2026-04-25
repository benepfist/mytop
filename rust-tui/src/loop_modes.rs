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

pub fn switch_mode(current: Mode, key: Key) -> Option<Mode> {
    match key {
        Key::Top => Some(Mode::Top),
        Key::Qps => Some(Mode::Qps),
        Key::Cmd => Some(Mode::Cmd),
        Key::Innodb => Some(Mode::Innodb),
        Key::Status => Some(Mode::Status),
        Key::Quit | Key::Other => Some(current),
    }
}

pub fn run_cycles(batchmode: bool) -> usize {
    if batchmode { 1 } else { 2 }
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
}
