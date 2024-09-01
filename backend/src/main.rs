use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use htmx::telemetry::{get_subscriber, init_subscriber};
use htmx::configuration::get_configuration;
use htmx::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("htmx".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        //configuration.application.host, configuration.application.port
        "0.0.0.0", "4000"
    );
    let listener = TcpListener::bind(address).expect("Failed to bind port 8000");

    run(listener, connection_pool)?.await
}

