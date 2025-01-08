use actix_web:: {
    get,post,web,App, HttpResponse, HttpServer, Responder, 
    http::{
        header::{self,ContentType}}
};

#[actix_web::main]

#[get("/welcome")]
async fn welcome() -> Result<HttpResponse>{
    Println!("Welcome is called");
    Ok(
        HttpResponse::build(StatusCode::Ok)
        .content_type(ContentType::plaintext())
        .body(include_str!("welcome.html"))
    )

}

async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
            App::new()

            .route("/hello",web::get().to(hello_world))
            .route("/post_example",web::post().to(post_example))
    })
            .bind(("127.0.0.1",8000))?.run()
            .await
}
    


async fn hello_world() -> impl Responder{
    HttpResponse::Ok().body("Hello World")

}

async fn post_example() -> impl Responder{
    HttpResponse::Ok().body("Hello World")

}