use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryCacheEntry {
    pub db: Option<String>,
    pub sql: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcesslistRowLite {
    pub id: u64,
    pub db: Option<String>,
    pub info: Option<String>,
}

pub fn merge_cache_from_processlist(
    cache: &mut BTreeMap<u64, QueryCacheEntry>,
    processlist: &[ProcesslistRowLite],
) {
    for row in processlist {
        if let Some(sql) = &row.info {
            cache.insert(
                row.id,
                QueryCacheEntry {
                    db: row.db.clone(),
                    sql: sql.clone(),
                },
            );
        }
    }
}

pub fn full_query_info(cache: &BTreeMap<u64, QueryCacheEntry>, id: u64) -> Result<String, String> {
    cache
        .get(&id)
        .map(|v| v.sql.clone())
        .ok_or_else(|| "*** Invalid id. ***".to_string())
}

pub fn full_query_info_from_processlist(
    cache: &mut BTreeMap<u64, QueryCacheEntry>,
    processlist: &[ProcesslistRowLite],
    id: u64,
) -> Result<String, String> {
    merge_cache_from_processlist(cache, processlist);
    full_query_info(cache, id)
}

pub fn explain_sql(cache: &BTreeMap<u64, QueryCacheEntry>, id: u64) -> Result<Vec<String>, String> {
    let entry = cache
        .get(&id)
        .ok_or_else(|| "*** Invalid id. ***".to_string())?;
    let mut cmds = Vec::new();
    if let Some(db) = &entry.db {
        cmds.push(format!("USE {db}"));
    }
    cmds.push(format!("EXPLAIN {}", entry.sql));
    Ok(cmds)
}

pub fn explain_from_processlist(
    cache: &mut BTreeMap<u64, QueryCacheEntry>,
    processlist: &[ProcesslistRowLite],
    id: u64,
) -> Result<Vec<String>, String> {
    merge_cache_from_processlist(cache, processlist);
    explain_sql(cache, id)
}

pub fn print_table(rows: &[BTreeMap<String, String>]) -> String {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|(k, v)| format!("{k}: {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_id_returns_error() {
        let cache = BTreeMap::new();
        assert_eq!(
            full_query_info(&cache, 1),
            Err("*** Invalid id. ***".to_string())
        );
    }

    #[test]
    fn explain_uses_database_before_explain() {
        let mut cache = BTreeMap::new();
        cache.insert(
            1,
            QueryCacheEntry {
                db: Some("app".into()),
                sql: "select * from t".into(),
            },
        );

        let result = explain_sql(&cache, 1).unwrap();
        assert_eq!(result[0], "USE app");
        assert!(result[1].starts_with("EXPLAIN"));
    }

    #[test]
    fn full_query_info_returns_sql_for_valid_id() {
        let mut cache = BTreeMap::new();
        cache.insert(
            7,
            QueryCacheEntry {
                db: None,
                sql: "select 1".into(),
            },
        );

        assert_eq!(full_query_info(&cache, 7).unwrap(), "select 1");
    }

    #[test]
    fn explain_without_database_only_emits_explain() {
        let mut cache = BTreeMap::new();
        cache.insert(
            5,
            QueryCacheEntry {
                db: None,
                sql: "select * from metrics".into(),
            },
        );

        let result = explain_sql(&cache, 5).unwrap();
        assert_eq!(result, vec!["EXPLAIN select * from metrics".to_string()]);
    }

    #[test]
    fn full_query_and_explain_work_with_processlist_cache_merge() {
        let mut cache = BTreeMap::new();
        let rows = vec![ProcesslistRowLite {
            id: 9,
            db: Some("analytics".into()),
            info: Some("select * from jobs".into()),
        }];

        assert_eq!(
            full_query_info_from_processlist(&mut cache, &rows, 9).unwrap(),
            "select * from jobs"
        );

        let explain = explain_from_processlist(&mut cache, &rows, 9).unwrap();
        assert_eq!(explain[0], "USE analytics");
        assert_eq!(explain[1], "EXPLAIN select * from jobs");
    }

    #[test]
    fn print_table_joins_cells_and_rows() {
        let mut row1 = BTreeMap::new();
        row1.insert("id".to_string(), "42".to_string());
        row1.insert("name".to_string(), "jobs".to_string());
        let mut row2 = BTreeMap::new();
        row2.insert("state".to_string(), "running".to_string());
        let printed = print_table(&[row1, row2]);

        assert_eq!(printed, "id: 42, name: jobs\nstate: running");
    }
}
