use crate::invoices::invoice_generation::Output;

/// # Presenter
/// The presenter pattern is a design pattern often used in software development to implement user
/// interfaces, typically within the Model-View-Presenter (MVP) architecture. This pattern helps in
/// organizing the code related to user interface (UI) in a manner that separates concerns, enhances
/// testability, and improves code structure.
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
