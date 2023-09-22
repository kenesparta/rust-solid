use postgres::Row;
use uuid::Uuid;

pub enum QueryValues {
    Integer(i32),
    Float(f64),
    Text(String),
    Uuid(Uuid),
}

/// # Adapter
/// The Adapter Pattern is a structural design pattern that allows you to adapt the interface of
/// one class into another interface that clients expect
pub trait DatabaseAdapter {
    fn query(
        &mut self,
        query: String,
        params: Vec<QueryValues>,
    ) -> Result<Vec<Row>, Box<dyn std::error::Error>>;
}
