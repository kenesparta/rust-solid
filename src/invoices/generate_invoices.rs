use crate::invoices::contract_repository::ContractRepository;
use chrono::{Datelike, Months, NaiveDateTime};
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
    pub payments: Vec<Payment>,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    pub date: NaiveDateTime,
    pub amount: Decimal,
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

/// DIP: High level components should not depend on low level components
/// They should depend on abstractions.
pub struct GenerateInvoices<T> {
    contract_repository: T,
}

impl<T> GenerateInvoices<T>
where
    T: ContractRepository,
{
    pub fn new(contract_repository: T) -> Self {
        GenerateInvoices {
            contract_repository,
        }
    }

    pub fn execute(&self, input: Input) -> Vec<Output> {
        let payments: Vec<Output> = self
            .contract_repository
            .list()
            .iter()
            .map(|r| {
                if input.input_type == "cash" {
                    return r
                        .payments
                        .iter()
                        .map(|p| Output {
                            date: p.date,
                            amount: p.amount,
                        })
                        .filter(|out| {
                            out.date.month() == input.month && out.date.year() == input.year
                        })
                        .collect::<Vec<Output>>();
                }

                if input.input_type == "accrual" {
                    return (0..=r.periods)
                        .map(|idx| {
                            let date_time: NaiveDateTime = r.date;
                            let amount: Decimal = r.amount;
                            Output {
                                date: date_time
                                    .checked_add_months(Months::new(idx as u32))
                                    .unwrap(),
                                amount: amount / Decimal::from(r.periods),
                            }
                        })
                        .filter(|out| {
                            out.date.month() == input.month && out.date.year() == input.year
                        })
                        .collect::<Vec<Output>>();
                }

                Vec::new()
            })
            .flatten()
            .collect();

        payments
    }
}
