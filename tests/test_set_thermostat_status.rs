#[macro_use]
extern crate serial_test;

mod common;

#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::common::app::create_app;

    #[actix_rt::test]
    #[serial]
    async fn test_get() {
        let mut app = create_app().await;

        let req = test::TestRequest::get()
            .uri("/graphql?query=mutation{setThermostatStatus(data:{status:true}){id,status,timestamp}}")
            .header("key", "123")
            .header("content-type", "application/json")
            .to_request();

        let resp: serde_json::Value = test::read_response_json(&mut app, req).await;
        let status = &resp["data"]["setThermostatStatus"];

        assert!(status.is_object());
        assert_eq!(status["id"], 2);
        assert_eq!(status["status"], true);
        assert!(status["timestamp"].is_number());
    }

    #[actix_rt::test]
    #[serial]
    async fn test_post() {
        let mut app = create_app().await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("key", "123")
            .header("content-type", "application/json")
            .set_payload(r#"{"query": "mutation{setThermostatStatus(data:{status:true}){id,status,timestamp}}"}"#)
            .to_request();

        let resp: serde_json::Value = test::read_response_json(&mut app, req).await;
        let status = &resp["data"]["setThermostatStatus"];

        assert!(status.is_object());
        assert_eq!(status["id"], 2);
        assert_eq!(status["status"], true);
        assert!(status["timestamp"].is_number());
    }
}
