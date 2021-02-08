use actix_web::{ get, web, App, HttpServer, Responder };

#[get("/")]
async fn index() -> impl Responder {
    return "indx"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .service(index)
    ).bind("127.0.0.1:80")?
    .run()
    .await
}
