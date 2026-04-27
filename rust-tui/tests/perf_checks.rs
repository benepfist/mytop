use mytop_tui::filters::Filters;
use mytop_tui::top_view::{ThreadRow, TopViewOptions, prepare_top_rows};
use std::time::Instant;

#[test]
#[ignore = "Performance smoke test: run explicitly in CI/release jobs"]
fn perf_prepare_top_rows_large_processlist() {
    let rows = (0..50_000)
        .map(|i| ThreadRow {
            id: i,
            user: if i % 2 == 0 { "app".into() } else { "etl".into() },
            db: Some("prod".into()),
            host: format!("db{}.internal:3306", i % 10),
            command: if i % 3 == 0 {
                "Query".into()
            } else {
                "Sleep".into()
            },
            time: (i % 300) as u64,
        })
        .collect::<Vec<_>>();

    let start = Instant::now();
    let rendered = prepare_top_rows(
        &rows,
        &Filters::default(),
        TopViewOptions {
            sort_desc: true,
            hide_idle: true,
        },
    );
    let elapsed = start.elapsed();

    assert!(!rendered.is_empty());
    assert!(elapsed.as_secs_f64() < 2.0, "elapsed={elapsed:?}");
}
