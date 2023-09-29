use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

use crate::invoices::contract_repository::ContractRepository;
use crate::invoices::presenter::Presenter;

#[derive(Clone, Serialize, Deserialize)]
pub struct Input {
    pub month: u32,
    pub year: i32,
    pub invoice_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub date: NaiveDateTime,
    pub amount: Decimal,
}

/// DIP: High level components should not depend on low level components
/// They should depend on abstractions.
pub struct GenerateInvoices<C, P: Presenter> {
    contract_repository: C,
    presenter: P,
}

impl<C, P> GenerateInvoices<C, P>
where
    C: ContractRepository,
    P: Presenter,
{
    pub fn new(contract_repository: C, presenter: P) -> Self {
        GenerateInvoices {
            contract_repository,
            presenter,
        }
    }

    pub fn execute(&mut self, input: Input) -> Result<P::Output, String> {
        let payments = self.contract_repository.list()?;
        let output = payments
            .iter()
            .map(|r| r.generate_invoices(input.month, input.year, input.invoice_type.clone()))
            .flatten()
            .map(|i| Output {
                date: i.date,
                amount: i.amount,
            })
            .collect::<Vec<Output>>();
        Ok(self.presenter.present(output))
    }
}
