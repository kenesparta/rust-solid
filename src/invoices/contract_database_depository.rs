use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::{Contract, Payment};
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
            .map(|r| {
                let id = r.get("id");
                let payment = client
                    .query(
                        "SELECT amount, date FROM ken.payment WHERE id_contract = $1",
                        &[&id],
                    )
                    .expect("Failed to fetch rows.");
                Contract {
                    id,
                    description: r.get("description"),
                    amount: r.get("amount"),
                    periods: r.get("periods"),
                    date: r.get("date"),
                    payments: payment
                        .iter()
                        .map(|p| Payment {
                            date: p.get("date"),
                            amount: p.get("amount"),
                        })
                        .collect::<Vec<Payment>>(),
                }
            })
            .collect::<Vec<Contract>>()
    }
}
