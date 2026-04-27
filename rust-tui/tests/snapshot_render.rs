use mytop_tui::output::{render_cmd_view, render_status_view, render_top_table};
use mytop_tui::summaries::CmdSummary;
use mytop_tui::top_view::ThreadRow;

#[test]
fn snapshot_top_table() {
    let actual = render_top_table(
        &[ThreadRow {
            id: 1,
            user: "alice".into(),
            db: Some("app".into()),
            host: "db01:3306".into(),
            command: "Query".into(),
            time: 2,
        }],
        false,
    );
    let expected = include_str!("golden/top_table.txt").trim_end();
    assert_eq!(actual, expected);
}

#[test]
fn snapshot_cmd_view() {
    let actual = render_cmd_view(&[CmdSummary {
        name: "select".into(),
        total: 20,
        pct: 0.5,
        delta: 4,
        delta_pct: 0.2,
    }]);
    let expected = include_str!("golden/cmd_view.txt").trim_end();
    assert_eq!(actual, expected);
}

#[test]
fn snapshot_status_view() {
    let actual = render_status_view(&[("Threads_running".into(), 10, 1)]);
    let expected = include_str!("golden/status_view.txt").trim_end();
    assert_eq!(actual, expected);
}
