use crate::invoices::invoice_generation::Output;

pub trait Presenter {
    type Output;
    fn present(&self, output: Vec<Output>) -> Self::Output;
}

pub struct JsonPresenter;

impl Presenter for JsonPresenter {
    type Output = Vec<Output>;

    fn present(&self, output: Vec<Output>) -> Vec<Output> {
        output
    }
}

pub struct CsvPresenter;

impl Presenter for CsvPresenter {
    type Output = String;

    fn present(&self, output: Vec<Output>) -> String {
        output
            .iter()
            .map(|out| {
                format!(
                    "{},{}",
                    out.date.format("%Y-%m-%d").to_string(),
                    out.amount.to_string()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
