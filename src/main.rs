use std::net::TcpListener;

use khang_first_project::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    run(listener)?.await
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;

//     #[actix_rt::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }
