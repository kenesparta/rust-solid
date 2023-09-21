use crate::invoices::generate_invoices::{GenerateInvoices, Input};
use rust_decimal::prelude::ToPrimitive;

#[test]
/// Should generate "notas fiscais" por regime
///
/// `type cash`
fn generate_notas_fiscais_cash() {
    let generate_invoices = GenerateInvoices::new();
    let input = Input {
        month: 1,
        year: 2022,
        input_type: "cash".to_string(),
    };
    let output = generate_invoices.execute(input);
    assert_eq!(output[0].date.to_string(), "2022-01-05 10:00:00");
    assert_eq!(output[0].amount.to_f64().unwrap(), 6000.0);
}

#[test]
/// Should generate "notas fiscais" por regime de competencia
///
/// `type accural`
fn generate_notas_fiscais_accrual_mont_1() {
    let generate_invoices = GenerateInvoices::new();
    let input = Input {
        month: 1,
        year: 2022,
        input_type: "accrual".to_string(),
    };
    let output = generate_invoices.execute(input);
    assert_eq!(output[0].date.to_string(), "2022-01-01 10:00:00");
    assert_eq!(output[0].amount.to_f64().unwrap(), 500.0);
}

#[test]
/// Should generate "notas fiscais" por regime de competencia
///
/// `type accural`
fn generate_notas_fiscais_accrual_month_2() {
    let generate_invoices = GenerateInvoices::new();
    let input = Input {
        month: 2,
        year: 2022,
        input_type: "accrual".to_string(),
    };
    let output = generate_invoices.execute(input);
    assert_eq!(output[0].date.to_string(), "2022-02-01 10:00:00");
    assert_eq!(output[0].amount.to_f64().unwrap(), 500.0);
}

#[test]
/// Should generate "notas fiscais" por regime de competencia
///
/// `type accural`
fn generate_notas_fiscais_accrual_month_11() {
    let generate_invoices = GenerateInvoices::new();
    let input = Input {
        month: 11,
        year: 2022,
        input_type: "accrual".to_string(),
    };
    let output = generate_invoices.execute(input);
    assert_eq!(output[0].date.to_string(), "2022-11-01 10:00:00");
    assert_eq!(output[0].amount.to_f64().unwrap(), 500.0);
}