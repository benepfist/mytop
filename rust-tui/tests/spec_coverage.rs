use mytop_tui::commands::{parse_explain_command, parse_kill_command, set_delay_secs};
use mytop_tui::data::{PollState, ProcessRow, poll_once};
use mytop_tui::filters::{Filters, StringOrRegex};
use mytop_tui::help::{pod_sections, print_help};
use mytop_tui::introspection::{
    ProcesslistRowLite, QueryCacheEntry, explain_from_processlist, full_query_info_from_processlist,
};
use mytop_tui::interactive::{InteractiveState, handle_keypress, submit_prompt};
use mytop_tui::output::{format_show_variables, render_innodb_view};
use mytop_tui::startup::{Config, merge_config, parse_kv_config};
use mytop_tui::summaries::{command_summary, get_qps, show_status};
use mytop_tui::top_view::{TopViewOptions, prepare_top_rows};
use std::collections::BTreeMap;

struct MockClient {
    processlist: Vec<ProcessRow>,
    status: BTreeMap<String, String>,
    variables: BTreeMap<String, String>,
    innodb_status: String,
}

impl mytop_tui::data::DatabaseClient for MockClient {
    fn show_full_processlist(&mut self) -> Result<Vec<ProcessRow>, String> {
        Ok(self.processlist.clone())
    }
    fn show_status(&mut self) -> Result<BTreeMap<String, String>, String> {
        Ok(self.status.clone())
    }
    fn show_variables(&mut self) -> Result<BTreeMap<String, String>, String> {
        Ok(self.variables.clone())
    }
    fn show_engine_innodb_status(&mut self) -> Result<String, String> {
        Ok(self.innodb_status.clone())
    }
}

#[test]
fn spec_01_configuration_and_startup() {
    let defaults = Config::default();
    let file = parse_kv_config("host=db01\nport=3307");
    let cli = parse_kv_config("host=db02:4406\nuser=alice");
    let cfg = merge_config(defaults, file, cli);
    assert_eq!(cfg.host, "db02");
    assert_eq!(cfg.port, 4406);
    assert_eq!(cfg.user, "alice");
}

#[test]
fn spec_02_main_loop_and_modes() {
    let mut state = InteractiveState::default();
    handle_keypress(&mut state, 'm');
    assert_eq!(format!("{:?}", state.mode), "Qps");
    handle_keypress(&mut state, 'q');
    assert!(!state.running);
}

#[test]
fn spec_03_top_view_rendering() {
    let rows = vec![
        mytop_tui::top_view::ThreadRow {
            id: 1,
            user: "alice".into(),
            db: Some("app".into()),
            host: "db01:3306".into(),
            command: "Query".into(),
            time: 3,
        },
        mytop_tui::top_view::ThreadRow {
            id: 2,
            user: "alice".into(),
            db: Some("app".into()),
            host: "db01:3307".into(),
            command: "Sleep".into(),
            time: 10,
        },
    ];
    let filters = Filters {
        user: StringOrRegex::Exact("alice".into()),
        db: StringOrRegex::Exact("app".into()),
        host: StringOrRegex::Exact("db01".into()),
    };
    let out = prepare_top_rows(
        &rows,
        &filters,
        TopViewOptions {
            sort_desc: true,
            hide_idle: true,
        },
    );
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].command, "Query");
}

#[test]
fn spec_04_interactive_commands() {
    assert!(parse_kill_command("7").eq(&mytop_tui::commands::CommandFeedback::Ok("kill thread=7".into())));
    assert!(parse_explain_command("7").eq(&mytop_tui::commands::CommandFeedback::Ok("explain thread=7".into())));
    assert_eq!(set_delay_secs("0"), 1);
}

#[test]
fn spec_05_filters_and_patterns() {
    let mut f = Filters::default();
    f.user = StringOrRegex::parse("/ali/");
    assert!(f.user.matches("alice"));
    f.reset();
    assert_eq!(f, Filters::default());
}

#[test]
fn spec_06_query_introspection() {
    let mut cache: BTreeMap<u64, QueryCacheEntry> = BTreeMap::new();
    let rows = vec![ProcesslistRowLite {
        id: 11,
        db: Some("app".into()),
        info: Some("select * from jobs".into()),
    }];
    assert_eq!(
        full_query_info_from_processlist(&mut cache, &rows, 11).unwrap(),
        "select * from jobs"
    );
    let explain = explain_from_processlist(&mut cache, &rows, 11).unwrap();
    assert_eq!(explain[0], "USE app");
}

#[test]
fn spec_07_status_and_command_summaries() {
    assert_eq!(get_qps(10, 13), 3);
    let prev = BTreeMap::from([("Com_select".to_string(), 1)]);
    let cur = BTreeMap::from([
        ("Com_select".to_string(), 3),
        ("Threads_running".to_string(), 8),
    ]);
    let cmd = command_summary(&prev, &cur);
    assert_eq!(cmd[0].delta, 2);
    let status = show_status(&prev, &cur);
    assert_eq!(status[0].0, "Threads_running");
}

#[test]
fn spec_08_variable_and_innodb_output() {
    let vars = BTreeMap::from([("autocommit".to_string(), "ON".to_string())]);
    let vars_text = format_show_variables(&vars);
    assert!(vars_text.contains("Variable | Value"));
    let innodb = render_innodb_view("ok", &["less"]);
    assert!(innodb.contains("pager=less"));
}

#[test]
fn spec_09_db_access_and_utilities() {
    let mut client = MockClient {
        processlist: vec![ProcessRow {
            id: 1,
            user: "alice".into(),
            host: "db01:3306".into(),
            db: Some("app".into()),
            command: "Query".into(),
            time: 1,
            state: None,
            info: Some("select 1".into()),
        }],
        status: BTreeMap::from([("Questions".into(), "10".into())]),
        variables: BTreeMap::new(),
        innodb_status: "ok".into(),
    };
    let mut state = PollState::default();
    let res = poll_once(&mut client, &mut state).unwrap();
    assert_eq!(res.snapshot.processlist.len(), 1);
    assert!(state.qcache.contains_key(&1));
}

#[test]
fn spec_10_help_and_documentation() {
    let help = print_help(false);
    assert!(help.contains("[q]"));
    let sections = pod_sections();
    assert!(sections.contains(&"Synopsis"));
}

#[test]
fn spec_prompt_submission_feedback() {
    let mut state = InteractiveState::default();
    handle_keypress(&mut state, 'k');
    let feedback = submit_prompt(&mut state, "abc").unwrap();
    assert_eq!(
        feedback,
        mytop_tui::commands::CommandFeedback::Error("*** Invalid id. ***".to_string())
    );
}
