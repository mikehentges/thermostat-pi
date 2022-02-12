use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use thermostat_pi::shared_data::{AccessSharedData, SharedData};
use thermostat_pi::telemetry::{get_subscriber, init_subscriber};
use time::OffsetDateTime;
// use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let subscriber = get_subscriber("thermostat-pi".into(), "debug".into());
    init_subscriber(subscriber);

    let common_data = SharedData {
        continue_read_temp: true,
        current_temp: 0.0,
        thermostat_value: 55,
        thermostat_on: false,
        thermostat_change_datetime: OffsetDateTime::UNIX_EPOCH,
    };
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    // clone some values to pass to the thread that is going to read the thermometer in a thread

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = thermostat_pi::run(listener, &sd).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn post_set_thermostat_value() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let body = "thermostat_setting=70";
    let response = client
        .post(&format!("{}/set_thermostat", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn post_set_thermostat_value_empty() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let invalid_body = "";
    let error_message = "missing thermostat value";

    //Act
    let response = client
        .post(&format!("{}/set_thermostat", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(
        400,
        response.status().as_u16(),
        "the API did not fail with 400 Bad Request when payload was {}.",
        error_message
    );
}

#[tokio::test]
async fn get_thermostat_data() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/get_thermostat", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(
        "{\"thermostat_setting\":55}",
        response.text().await.unwrap()
    );
}
