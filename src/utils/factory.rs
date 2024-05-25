use std::sync::Arc;

pub trait QueryBuilder: Send + Sync {
    fn insert(&self, table: &str, fields: &[&str], values: &[&str]) -> String;
    fn select_all(&self, table: &str) -> String;
    fn select_by_id(&self, table: &str, id: u64) -> String;
    fn delete(&self, table: &str, id: u64) -> String;
    fn update(&self, table: &str, id: u64, fields: &[(&str, &str)]) -> String;
}

struct MySQLQueryBuilder;

pub fn new_mysql_query_builder() -> Arc<dyn QueryBuilder> {
    Arc::new(MySQLQueryBuilder)
}

impl QueryBuilder for MySQLQueryBuilder {
    fn insert(&self, table: &str, fields: &[&str], values: &[&str]) -> String {
        let fields_str = fields.join(", ");
        let values_str = values.iter().map(|v| format!("'{}'", v)).collect::<Vec<String>>().join(", ");
        format!("INSERT INTO {} ({}) VALUES ({});", table, fields_str, values_str)
    }

    fn select_all(&self, table: &str) -> String {
        format!("SELECT * FROM {};", table)
    }

    fn select_by_id(&self, table: &str, id: u64) -> String {
        format!("SELECT * FROM {} WHERE id = {};", table, id)
    }

    fn delete(&self, table: &str, id: u64) -> String {
        format!("DELETE FROM {} WHERE id = {};", table, id)
    }

    fn update(&self, table: &str, id: u64, fields: &[(&str, &str)]) -> String {
        let set_str = fields.iter().map(|(field, value)| format!("{} = '{}'", field, value)).collect::<Vec<String>>().join(", ");
        format!("UPDATE {} SET {} WHERE id = {};", table, set_str, id)
    }
}
