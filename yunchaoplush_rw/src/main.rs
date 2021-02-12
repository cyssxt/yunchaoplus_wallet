#[macro_use] extern crate log;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

mod model;
mod recharge;
mod withdraw;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(recharge::create_recharge)
            .service(recharge::get_recharge)
            .service(recharge::get_recharge_list)
            .service(withdraw::create_withdraw)
            .service(withdraw::get_withdraw)
            .service(withdraw::update_withdraw)
            .service(withdraw::get_withdraw_list)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
