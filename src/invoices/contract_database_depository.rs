use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::{Contract, Payment};
use postgres::{Client, NoTls};

pub struct PostgresAdapter {
    pub(crate) client: Client,
}

/// # Adapter
/// The Adapter Pattern is a structural design pattern that allows you to adapt the interface of
/// one class into another interface that clients expect
impl PostgresAdapter {
    pub fn new() -> Result<Self, postgres::Error> {
        let client = Client::connect("postgresql://user:user@localhost/user", NoTls)?;
        Ok(PostgresAdapter { client })
    }
}

impl ContractRepository for PostgresAdapter {
    fn list(&mut self) -> Result<Vec<Contract>, String> {
        let rows = self
            .client
            .query(
                "SELECT id, description, amount, periods, date FROM ken.contract",
                &[],
            )
            .expect("Failed to fetch rows.");

        Ok(rows
            .iter()
            .map(|r| {
                let id = r.get("id");
                let payment_rows = self
                    .client
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
