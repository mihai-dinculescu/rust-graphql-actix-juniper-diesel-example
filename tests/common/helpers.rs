use actix_http::error::Error;
use actix_http::Request;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::test;

#[allow(dead_code)] // not all integration tests use this
pub async fn set_thermostat_status<S>(app: &mut S, new_status: bool)
where
    S: Service<Request = Request, Response = ServiceResponse, Error = Error>,
{
    let payload = format!(
        r#"{{"query": "mutation{{setThermostatStatus(data:{{status:{new_status}}}){{id,status,timestamp}}}}"}}"#,
        new_status = new_status
    );

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("key", "123")
        .header("content-type", "application/json")
        .set_payload(payload)
        .to_request();

    test::read_response(app, req).await;
}
