use crate::invoices::contract::Contract;
use crate::invoices::payment::Payment;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

#[test]
/// Should generate invoices fom contract
///
fn generate_invoices_from_contract() {
    let contract = Contract {
        id: Default::default(),
        description: "".to_string(),
        amount: Decimal::try_from(6_000.0).unwrap(),
        periods: 12,
        date: {
            let date = NaiveDate::from_ymd_opt(2022, 1, 1)
                .ok_or("date error".to_string())
                .unwrap();
            let time = NaiveTime::from_hms_opt(10, 00, 00)
                .ok_or("date error".to_string())
                .unwrap();
            let datetime = NaiveDateTime::new(date, time);
            datetime
        },
        payments: vec![],
    };
    let c = contract.generate_invoices(1, 2022, "accrual".to_string());
    assert_eq!(c[0].date.to_string(), "2022-01-01 10:00:00");
    assert_eq!(c[0].amount.to_f64().unwrap(), 500.0);
}

#[test]
/// Should generate the contract salary.
///
fn generate_contract_salary() {
    let contract = Contract {
        id: Default::default(),
        description: "".to_string(),
        amount: Decimal::try_from(6_000.0).unwrap(),
        periods: 12,
        date: {
            let date = NaiveDate::from_ymd_opt(2022, 1, 1)
                .ok_or("".to_string())
                .unwrap();
            let time = NaiveTime::from_hms_opt(10, 00, 00)
                .ok_or("".to_string())
                .unwrap();
            let datetime = NaiveDateTime::new(date, time);
            datetime
        },
        payments: {
            let mut payments: Vec<Payment> = Vec::new();
            payments.push(Payment {
                date: Default::default(),
                amount: Decimal::try_from(2_000.0).unwrap(),
            });
            payments
        },
    };
    assert_eq!(contract.get_balance(), 4_000.0);
}
