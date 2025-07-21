use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(hello))
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
    use super::*;
    use actix_web::rt; 
    use reqwest::Client;
    use std::{fs::read_dir, net::TcpListener, path::PathBuf};

    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let server = run(listener);
        rt::spawn(server);

        address
    }

    #[actix_web::test]
    async fn root_returns_a_200() {
        // Arrange
        let address = spawn_app();
        let client = Client::new();

        
        // Act
        let response = client
            .get(&address)
            .send()
            .await
            .expect("Failed to execute request.");

        
        // Assert
        assert_eq!(200, response.status().as_u16());
    }
    #[actix_web::test]
    async fn cours_routes_return_200() {
        // Arrange
        let address = spawn_app();
        let client = Client::new();
        let cours_dir = "static/html/cours";

        // Act 
        for entry in read_dir(cours_dir).expect("Could not read cours directory") {
            let entry = entry.expect("Could not read directory entry");
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                
            
                let route_name = file_name.split('.').next().unwrap_or(file_name);
                
                let url = format!("{}/cours/{}", &address, route_name);

                let response = client.get(&url).send().await.expect("Request failed");
                
                // Assert 
                assert_eq!(200, response.status().as_u16(), "Failed for URL: {}", url);
            }
        }
    }

    
    #[actix_web::test]
    async fn exercice_routes_return_200() {
        // Arrange
        let address = spawn_app();
        let client = Client::new();
        let exercice_dir = "static/html/exercice";

        // Act 
        for entry in read_dir(exercice_dir).expect("Could not read exercice directory") {
            let entry = entry.expect("Could not read directory entry");
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                
            
                let route_name = file_name.split('.').next().unwrap_or(file_name);
                
                let url = format!("{}/exercice/{}", &address, route_name);

                let response = client.get(&url).send().await.expect("Request failed");
                
                // Assert 
                assert_eq!(200, response.status().as_u16(), "Failed for URL: {}", url);
            }
        }
    }
   
}