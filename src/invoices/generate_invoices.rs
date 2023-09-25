use crate::invoices::contract::InvoiceType;
use crate::invoices::contract_repository::ContractRepository;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub month: u32,
    pub year: i32,
    pub invoice_type: InvoiceType,
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

    pub fn execute(&mut self, input: Input) -> Result<Vec<Output>, String> {
        let payments = self.contract_repository.list()?;
        Ok(payments
            .iter()
            .map(|r| r.generate_invoices(input.month, input.year, input.invoice_type))
            .flatten()
            .map(|i| Output {
                date: i.date,
                amount: i.amount,
            })
            .collect())
    }
}
