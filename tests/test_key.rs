#[macro_use]
extern crate serial_test;

mod common;

#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::common::app::create_app;
    use ::lib::models::errors::GraphQLErrors;

    #[actix_rt::test]
    #[serial]
    async fn test_no_key() {
        let mut app = create_app().await;

        let req = test::TestRequest::get()
            .uri("/graphql?query={thermostatStatus{id,status,timestamp}}")
            .to_request();

        let response: GraphQLErrors = test::read_response_json(&mut app, req).await;
        let message = &response.errors[0].message;

        assert_eq!(message, "Missing header: key");
    }

    #[actix_rt::test]
    #[serial]
    async fn test_wrong_key() {
        let mut app = create_app().await;

        let req = test::TestRequest::get()
            .uri("/graphql?query={thermostatStatus{id,status,timestamp}}")
            .header("key", "321")
            .to_request();

        let response: GraphQLErrors = test::read_response_json(&mut app, req).await;
        let message = &response.errors[0].message;

        assert_eq!(message, "Invalid Key");
    }

    #[actix_rt::test]
    #[serial]
    async fn test_correct_key() {
        let mut app = create_app().await;

        let req = test::TestRequest::get()
            .uri("/graphql?query={thermostatStatus{id,status,timestamp}}")
            .header("key", "123")
            .to_request();

        let response: serde_json::Value = test::read_response_json(&mut app, req).await;
        let status = &response["data"]["thermostatStatus"];

        assert!(status.is_object());
        assert_eq!(status["id"], 1);
        assert_eq!(status["status"], false);
        assert!(status["timestamp"].is_number());
    }
}
