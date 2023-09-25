use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Payment {
    pub date: NaiveDateTime,
    pub amount: Decimal,
}
