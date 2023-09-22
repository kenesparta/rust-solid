use postgres::types::ToSql;
use postgres::{Client, NoTls, Row};

use crate::invoices::connection::{DatabaseAdapter, QueryValues};

pub struct PgAdapter {
    connection: Client,
}

impl PgAdapter {
    pub fn new() -> Self {
        let connection = Client::connect("postgresql://user:user@localhost/user", NoTls)
            .expect("Failed to connect to database.");
        PgAdapter { connection }
    }
}

impl DatabaseAdapter for PgAdapter {
    fn query(
        &mut self,
        statement: String,
        params: Vec<QueryValues>,
    ) -> Result<Vec<Row>, Box<dyn std::error::Error>> {
        let dyn_params: Vec<&(dyn ToSql + Sync)> = params
            .iter()
            .map(|value| match value {
                QueryValues::Integer(i) => i as &(dyn ToSql + Sync),
                QueryValues::Text(s) => s as &(dyn ToSql + Sync),
                QueryValues::Float(f) => f as &(dyn ToSql + Sync),
                QueryValues::Uuid(u) => u as &(dyn ToSql + Sync),
            })
            .collect();
        let rows = self.connection.query(&statement, &dyn_params);
        match rows {
            Ok(r) => Ok(r),
            Err(err) => Err(err.into()),
        }
    }
}
