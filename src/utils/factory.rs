trait QueryBuilder {
    fn insert(&self, table: &str, fields: &[&str], values: &[&str]) -> String;
    fn select_all(&self, table: &str) -> String;
    fn select_by_id(&self, table: &str, id: u64) -> String;
    fn delete(&self, table: &str, id: u64) -> String;
    fn update(&self, table: &str, id: u64, fields: &[(&str, &str)]) -> String;
}

struct MySQLQueryBuilder;

fn new_mysql_query_builder() -> Box<dyn QueryBuilder> {
    Box::new(MySQLQueryBuilder)
}

impl QueryBuilder for MySQLQueryBuilder {
    fn insert(&self, table: &str, fields: &[&str], values: &[&str]) -> String {
        let fields_str = fields.join(", ");
        let values_str = values
            .iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<String>>()
            .join(", ");
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
        let set_str = fields
            .iter()
            .map(|(field, value)| format!("{} = '{}'", field, value))
            .collect::<Vec<String>>()
            .join(", ");
        format!("UPDATE {} SET {} WHERE id = {};", table, set_str, id)
    }
}

// Contoh penggunaan query builder
fn main() {
    let query_builder = new_mysql_query_builder();

    let insert_query = query_builder.insert("users", &["username", "email"], &["john_doe", "john@example.com"]);
    println!("Insert Query: {}", insert_query);

    let select_all_query = query_builder.select_all("users");
    println!("Select All Query: {}", select_all_query);

    let select_by_id_query = query_builder.select_by_id("users", 1);
    println!("Select By ID Query: {}", select_by_id_query);

    let delete_query = query_builder.delete("users", 1);
    println!("Delete Query: {}", delete_query);

    let update_query = query_builder.update("users", 1, &[("username", "john_doe_updated"), ("email", "john_updated@example.com")]);
    println!("Update Query: {}", update_query);
}
