#[macro_use]
extern crate log;

use dotenv::dotenv;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use tokio_postgres::NoTls;

use crate::config::Config;

mod config;
mod model;
mod recharge;
mod withdraw;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    let cfg = Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
