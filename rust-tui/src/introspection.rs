use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryCacheEntry {
    pub db: Option<String>,
    pub sql: String,
}

pub fn full_query_info(cache: &BTreeMap<u64, QueryCacheEntry>, id: u64) -> Result<String, String> {
    cache
        .get(&id)
        .map(|v| v.sql.clone())
        .ok_or_else(|| "*** Invalid id. ***".to_string())
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
}
