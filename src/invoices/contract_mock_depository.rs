use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::generate_invoices::{Contract, Payment};

pub struct ContractMockRepository {}

impl ContractMockRepository {
    pub fn new() -> Self {
        ContractMockRepository {}
    }
}

impl ContractRepository for ContractMockRepository {
    fn list(&self) -> Vec<Contract> {
        let mut contracts: Vec<Contract> = Vec::new();
        contracts.push(Contract {
            id: Default::default(),
            description: "".to_string(),
            amount: Decimal::try_from(6000.0).unwrap(),
            periods: 12,
            date: {
                let date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
                let time = NaiveTime::from_hms_opt(10, 00, 00).unwrap();
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

        contracts
    }
}
