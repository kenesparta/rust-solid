use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::{Contract, Payment};
use postgres::{Client, NoTls};
use std::fmt::Error;

pub struct ContractDatabaseRepository {}

impl ContractDatabaseRepository {
    pub fn new() -> Self {
        ContractDatabaseRepository {}
    }
}

impl ContractRepository for ContractDatabaseRepository {
    fn list(&self) -> Result<Vec<Contract>, String> {
        let mut client = Client::connect("postgresql://user:user@localhost/user", NoTls)
            .expect("Failed to connect to database.");

        let rows = client
            .query(
                "SELECT id, description, amount, periods, date FROM ken.contract",
                &[],
            )
            .expect("Failed to fetch rows.");

        Ok(rows
            .iter()
            .map(|r| {
                let id = r.get("id");
                let payment_rows = client
                    .query(
                        "SELECT amount, date FROM ken.payment WHERE id_contract = $1",
                        &[&id],
                    )
                    .expect("Failed to fetch rows.");
                let payments = payment_rows
                    .iter()
                    .map(|p| Payment {
                        date: p.get("date"),
                        amount: p.get("amount"),
                    })
                    .collect::<Vec<Payment>>();

                Contract {
                    id,
                    payments,
                    description: r.get("description"),
                    amount: r.get("amount"),
                    periods: r.get("periods"),
                    date: r.get("date"),
                }
            })
            .collect::<Vec<Contract>>())
    }
}
