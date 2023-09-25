use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Invoice {
    pub date: NaiveDateTime,
    pub amount: Decimal,
}
