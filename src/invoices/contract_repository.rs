use crate::invoices::generate_invoices::Contract;

/// The objective of **Repositories** is the aggregates persistence.
/// Aggregates are clusters of Domain objects such as `entities` and `Value Objects`
pub trait ContractRepository {
    fn list(&self) -> Vec<Contract>;
}
