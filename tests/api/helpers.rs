use api::{
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

// Ensure that the `tracing` stack is only initialized once using `Lazy`.
// This is necessary because multiple test cases run in parallel, and
// initializing tracing multiple times causes a runtime error.
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = String::from("test");
    let default_filter_level = String::from("debug");
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let application = Application::build()
        .await
        .expect("Failed to build application.");

    let address = format!("http://127.0.0.1:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());
    let test_app = TestApp { address };

    test_app
}
