use crate::invoices::generate_invoices::{GenerateInvoices, Input};

#[test]
/// Should generate "notas fiscais"
///
fn generate_notas_fiscais() {
    let generate_invoices = GenerateInvoices::new();
    let input = Input {
        month: 1,
        year: 2022,
        input_type: "cash".to_string(),
    };
    let output = generate_invoices.execute(input);
    assert_eq!(output[0].date.to_string(), "2022-01-05 10:00:00")
}
