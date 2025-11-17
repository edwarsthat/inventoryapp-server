use inventario_server::config::{database, env};
use inventario_server::models::errors::server_error::{ServerError, ServerErrorKind};
use inventario_server::server::app;
use std::error::Error;
use std::process;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = env::load_config()?;
    let db_pool = database::create_pool(&config.database_url).await?;

    let addr = format!("{}:{}", config.url, config.port);
    let app =  app::create_router(db_pool);

    
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("Servidor escuchando en {}", addr);
            listener
        }
        Err(err) => {
            return Err(Box::new(ServerError::new(
                400,
                &format!("Error al vincular el socket: {}", err),
                ServerErrorKind::BindError,
                "run",
            )));
        }
    };

    axum::serve(listener, app).await?;

    Ok(())
}


// async fn shutdown_signal() {
//     tokio::signal::ctrl_c()
//         .await
//         .expect("Error al esperar señal de cierre");
// }
