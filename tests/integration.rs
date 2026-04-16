use actix_web::{test, App};
use serde_json::json;
use data_privacy_vault::routes::{tokenize, detokenize};
use data_privacy_vault::storage::{store_tokenized_data, retrieve_original_data};

#[actix_web::test]
async fn test_tokenize_with_format_validation() {
    let app = test::init_service(App::new().service(tokenize)).await;

    let payload = json!({
        "id": "req-12345",
        "data": {
            "field1": "value1",
            "field2": "value2"
        },
        "format": {
            "field1": "string",
            "field2": "string"
        }
    });

    let req = test::TestRequest::post()
        .uri("/tokenize")
        .insert_header(("x-api-key", "writer-key-123"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let tokenized_data: serde_json::Value = test::read_body_json(resp).await;
    for (key, token) in tokenized_data["data"].as_object().unwrap() {
        let original_value = payload["data"][key].as_str().unwrap();
        assert_eq!(retrieve_original_data(token).unwrap(), original_value);
    }
}

#[actix_web::test]
async fn test_tokenize_with_invalid_format() {
    let app = test::init_service(App::new().service(tokenize)).await;

    let payload = json!({
        "id": "req-12345",
        "data": {
            "field1": "value1",
            "field2": 12345
        },
        "format": {
            "field1": "string",
            "field2": "string"
        }
    });

    let req = test::TestRequest::post()
        .uri("/tokenize")
        .insert_header(("x-api-key", "writer-key-123"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
