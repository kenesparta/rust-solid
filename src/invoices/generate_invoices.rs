use chrono::{Datelike, NaiveDateTime};
use postgres::{Client, NoTls};
use rust_decimal::Decimal;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Contract {
    pub id: Uuid,
    pub description: String,
    pub amount: Decimal,
    pub periods: i32,
    pub date: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub month: u32,
    pub year: i32,
    pub input_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub date: NaiveDateTime,
    pub amount: Decimal,
}

pub struct GenerateInvoices {}

impl GenerateInvoices {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, input: Input) -> Vec<Output> {
        let mut client = Client::connect("postgresql://user:user@localhost/user", NoTls)
            .expect("Failed to connect to database.");

        let rows = client
            .query(
                "SELECT id, description, amount, periods, date FROM ken.contract",
                &[],
            )
            .expect("Failed to fetch rows.");

        let contracts: Vec<Contract> = rows
            .iter()
            .map(|r| Contract {
                id: r.get("id"),
                description: r.get("description"),
                amount: r.get("amount"),
                periods: r.get("periods"),
                date: r.get("date"),
            })
            .collect();

        let payments: Vec<Output> = contracts
            .iter()
            .map(|r| {
                let payment = client
                    .query(
                        "SELECT amount, date FROM ken.payment WHERE id_contract = $1",
                        &[&r.id],
                    )
                    .expect("Failed to fetch rows.");
                payment
                    .iter()
                    .map(|p| Output {
                        date: p.get("date"),
                        amount: p.get("amount"),
                    })
                    .filter(|out| out.date.month() == input.month && out.date.year() == input.year)
                    .collect::<Vec<Output>>()
            })
            .flatten()
            .collect();

        payments
    }
}
