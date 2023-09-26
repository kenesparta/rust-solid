use crate::invoices::contract::{Contract, InvoiceType};
use crate::invoices::invoice::Invoice;
use chrono::{Datelike, Months, NaiveDateTime};
use rust_decimal::Decimal;

/// # Contract
/// We use OCP open-closed principle
/// We need to create extension points
///
/// # Strategy pattern
/// We are using the **strategy pattern**
/// Creates an interchangeable behavior.
///
/// # Dynamic Factory
/// Creates an instance based on a **string**
pub trait InvoiceGenerationStrategy {
    fn generate(&self, contract: &Contract, month: u32, year: i32) -> Vec<Invoice>;
}

pub struct CashBasisStrategy;

impl InvoiceGenerationStrategy for CashBasisStrategy {
    fn generate(&self, contract: &Contract, month: u32, year: i32) -> Vec<Invoice> {
        contract
            .payments
            .iter()
            .map(|p| Invoice {
                date: p.date,
                amount: p.amount,
            })
            .filter(|out| out.date.month() == month && out.date.year() == year)
            .collect::<Vec<Invoice>>()
    }
}

pub struct AccrualBasisStrategy;

impl InvoiceGenerationStrategy for AccrualBasisStrategy {
    fn generate(&self, contract: &Contract, month: u32, year: i32) -> Vec<Invoice> {
        (0..=contract.periods)
            .map(|idx| {
                let date_time: NaiveDateTime = contract.date;
                let amount: Decimal = contract.amount;
                Invoice {
                    date: date_time
                        .checked_add_months(Months::new(idx as u32))
                        .unwrap(),
                    amount: amount / Decimal::from(contract.periods),
                }
            })
            .filter(|out| out.date.month() == month && out.date.year() == year)
            .collect::<Vec<Invoice>>()
    }
}

pub struct InvoiceGenerationFactory;

impl InvoiceGenerationFactory {
    pub fn create(invoice_type: InvoiceType) -> Box<dyn InvoiceGenerationStrategy> {
        match invoice_type {
            InvoiceType::CASH => Box::new(CashBasisStrategy),
            InvoiceType::ACCRUAL => Box::new(AccrualBasisStrategy),
        }
    }
}
