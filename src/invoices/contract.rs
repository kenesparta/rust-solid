use crate::invoices::invoice::Invoice;
use crate::invoices::invoice_generation_strategy::InvoiceGenerationFactory;
use crate::invoices::payment::Payment;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub enum InvoiceType {
    CASH,
    ACCRUAL,
}

impl std::str::FromStr for InvoiceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cash" => Ok(InvoiceType::CASH),
            "accrual" => Ok(InvoiceType::ACCRUAL),
            _ => Err(()),
        }
    }
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
    pub fn generate_invoices(&self, month: u32, year: i32, invoice_type: String) -> Vec<Invoice> {
        let inv_type_result = invoice_type.parse::<InvoiceType>().unwrap();
        let invoice_type_selection = InvoiceGenerationFactory::create(inv_type_result);
        invoice_type_selection.generate(self, month, year)
    }
}
