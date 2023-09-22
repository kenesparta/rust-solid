use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::{Contract, Payment};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
use std::fmt::Error;

pub struct ContractMockRepository {}

impl ContractMockRepository {
    pub fn new() -> Self {
        ContractMockRepository {}
    }
}

impl ContractRepository for ContractMockRepository {
    fn list(&self) -> Result<Vec<Contract>, String> {
        let mut contracts: Vec<Contract> = Vec::new();
        contracts.push(Contract {
            id: Default::default(),
            description: "".to_string(),
            amount: Decimal::try_from(6000.0).unwrap(),
            periods: 12,
            date: {
                let date = NaiveDate::from_ymd_opt(2022, 1, 1).ok_or("s".to_string())?;
                let time = NaiveTime::from_hms_opt(10, 00, 00).ok_or("s".to_string())?;
                let datetime = NaiveDateTime::new(date, time);
                datetime
            },
            payments: {
                let mut payments: Vec<Payment> = Vec::new();
                payments.push(Payment {
                    date: Default::default(),
                    amount: Default::default(),
                });
                payments
            },
        });

        Ok(contracts)
    }
}
