/* This ia an actix Microservice that has multiple routes:
A. type: "/" that returns a message : "Hello, random dish recommendation for you today!"
B. type: "/movie" that returns a random dish recommended
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use webdocker::random_food;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, random dish recommendation for you today!")
}

//create a function that returns a random best food
#[get("/food")]
async fn food() -> impl Responder {
    //print the random food
    println!("Random Food: {}", random_food());
    HttpResponse::Ok().body(random_food())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(food))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
