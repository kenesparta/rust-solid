use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::Contract;
use postgres::{Client, NoTls};

pub struct ContractDatabaseRepository {}

impl ContractRepository for ContractDatabaseRepository {
     fn list() -> Vec<Contract> {
        let mut client = Client::connect("postgresql://user:user@localhost/user", NoTls)
            .expect("Failed to connect to database.");

        let rows = client
            .query(
                "SELECT id, description, amount, periods, date FROM ken.contract",
                &[],
            )
            .expect("Failed to fetch rows.");

        rows.iter()
            .map(|r| Contract {
                id: r.get("id"),
                description: r.get("description"),
                amount: r.get("amount"),
                periods: r.get("periods"),
                date: r.get("date"),
            })
            .collect::<Vec<Contract>>()
    }
}
