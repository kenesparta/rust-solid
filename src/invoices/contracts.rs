use crate::invoices::invoice::Invoice;
use crate::invoices::payment::Payment;
use chrono::{Datelike, Months, NaiveDateTime};
use rust_decimal::Decimal;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum InvoiceType {
    CASH,
    ACCRUAL,
}

/// Aggregate Root: Contract
#[derive(Debug)]
pub struct Contract {
    pub id: Uuid,
    pub description: String,
    pub amount: Decimal,
    pub periods: i32,
    pub date: NaiveDateTime,
    pub payments: Vec<Payment>,
}

impl Contract {
    pub fn generate_invoices(
        &self,
        month: u32,
        year: i32,
        invoice_type: InvoiceType,
    ) -> Vec<Invoice> {
        match invoice_type {
            InvoiceType::CASH => self
                .payments
                .iter()
                .map(|p| Invoice {
                    date: p.date,
                    amount: p.amount,
                })
                .filter(|out| out.date.month() == month && out.date.year() == year)
                .collect::<Vec<Invoice>>(),
            InvoiceType::ACCRUAL => {
                return (0..=self.periods)
                    .map(|idx| {
                        let date_time: NaiveDateTime = self.date;
                        let amount: Decimal = self.amount;
                        Invoice {
                            date: date_time
                                .checked_add_months(Months::new(idx as u32))
                                .unwrap(),
                            amount: amount / Decimal::from(self.periods),
                        }
                    })
                    .filter(|out| out.date.month() == month && out.date.year() == year)
                    .collect::<Vec<Invoice>>();
            }
        }
    }
}
