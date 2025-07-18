use actix_web::{App, HttpServer};
use actix_web::{HttpResponse, Responder, get};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .listen(listener)?
        .run()
        .await
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod test {
    use std::net::TcpListener;

    use super::run;

    #[tokio::test]
    async fn root_returns_a_200() {
        // Arrange
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        run(listener);

        // Act
    }
}
