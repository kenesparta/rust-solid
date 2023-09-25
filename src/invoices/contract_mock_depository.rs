use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::contracts::Contract;
use crate::invoices::payment::Payment;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;

pub struct MockAdapter {}

/// # Adapter
/// The Adapter Pattern is a structural design pattern that allows you to adapt the interface of
/// one class into another interface that clients expect
impl MockAdapter {
    pub fn new() -> Result<Self, postgres::Error> {
        Ok(MockAdapter {})
    }
}

impl ContractRepository for MockAdapter {
    fn list(&mut self) -> Result<Vec<Contract>, String> {
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
