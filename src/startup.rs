use crate::routes::{health, hello};
use axum::{routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

pub type Server = Serve<Router, Router>;

pub struct Application {
    pub port: u16,
    pub server: Server,
}

impl Application {
    pub async fn build() -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind("0.0.0.0:8000").await?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener)?;

        tracing::info!("Server running on port {}", port);

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener) -> Result<Serve<Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/", get(hello::world))
        .route("/health", get(health::check));

    let server = axum::serve(listener, app);

    Ok(server)
}
