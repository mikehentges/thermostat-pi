use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{lambda_runtime::Error, service_fn, IntoResponse, Request};

extern crate temp_data;
use temp_data::TempData;

use log::{debug, error};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

// Implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    debug!("logger has been set up");

    lambda_http::run(service_fn(my_handler)).await?;

    Ok(())
}

async fn my_handler(request: Request) -> Result<impl IntoResponse, Error> {
    debug!("handling a request, Request is: {:?}", request);

    let request_json = match request.body() {
        lambda_http::Body::Text(json_string) => json_string,
        _ => "",
    };

    debug!("Request JSON is : {:?}", request_json);
    let request_struct: TempData = serde_json::from_str(request_json)?;

    // set up as a DynamoDB client
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // build the values that are stored in the DB
    let record_date_av = AttributeValue::S(request_struct.record_date.clone());
    let thermostat_on_av = AttributeValue::S(request_struct.thermostat_on.to_string());
    let temperature_av = AttributeValue::N(request_struct.temperature.to_string());
    let thermostat_value_av = AttributeValue::N(request_struct.thermostat_value.to_string());
    let record_day_av: AttributeValue =
        AttributeValue::S(request_struct.record_date[..10].to_string());

    // Store our data in the DB
    let _resp = client
        .put_item()
        .table_name("Shop_Thermostat")
        .item("Record_Day", record_day_av)
        .item("Record_Date", record_date_av)
        .item("Thermostat_On", thermostat_on_av)
        .item("Temperature", temperature_av)
        .item("Thermostat_Value", thermostat_value_av)
        .send()
        .await
        .map_err(|err| {
            error!("failed to put item in Shop_Thermostat, error: {}", err);
            FailureResponse {
                body: "The lambda encountered an error and your message was not saved".to_owned(),
            }
        })?;

    debug! {
        "Successfully stored item {:?}", &request_struct
    }
    debug! {"finishing up"}

    Ok("the lambda was successful".to_string())
}
