mod invoices;

use crate::invoices::contract_database_depository::PostgresAdapter;
use crate::invoices::invoice_generation::{GenerateInvoices, Input};
use crate::invoices::presenter::JsonPresenter;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, Result};


async fn invoice(input: web::Json<Input>) -> Result<HttpResponse> {
    let db_adapter = web::block(move || {
        let db_adapter = PostgresAdapter::new().unwrap();
        let json_presenter = JsonPresenter {};
        let mut generate_invoices = GenerateInvoices::new(db_adapter, json_presenter);
        generate_invoices.execute(input.into_inner())
    }).await;

    match db_adapter {
        Ok(row) => {
            Ok(HttpResponse::Ok().json(row.ok()))
        }
        Err(e) => Err(error::ErrorBadRequest(e.to_string())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/invoice", web::post().to(invoice)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/post", web::post().to(invoice))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().to_string(), "404 Not Found".to_string());
    }
}
