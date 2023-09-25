use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Payment {
    pub date: NaiveDateTime,
    pub amount: Decimal,
}
