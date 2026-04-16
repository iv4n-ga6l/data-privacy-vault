use actix_web::{test, App};
use serde_json::json;
use data_privacy_vault::routes::{tokenize, detokenize};
use data_privacy_vault::storage::{store_tokenized_data, retrieve_original_data};

#[actix_web::test]
async fn test_tokenize_endpoint_with_auth() {
    let app = test::init_service(App::new().service(tokenize)).await;

    let payload = json!({
        "id": "req-12345",
        "data": {
            "field1": "value1",
            "field2": "value2"
        }
    });

    let req = test::TestRequest::post()
        .uri("/tokenize")
        .insert_header(("x-api-key", "writer-key-123"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify data is stored in Redis
    let tokenized_data: serde_json::Value = test::read_body_json(resp).await;
    for (key, token) in tokenized_data["data"].as_object().unwrap() {
        let original_value = payload["data"][key].as_str().unwrap();
        assert_eq!(retrieve_original_data(token).unwrap(), original_value);
    }
}

#[actix_web::test]
async fn test_tokenize_endpoint_without_auth() {
    let app = test::init_service(App::new().service(tokenize)).await;

    let payload = json!({
        "id": "req-12345",
        "data": {
            "field1": "value1",
            "field2": "value2"
        }
    });

    let req = test::TestRequest::post()
        .uri("/tokenize")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_detokenize_endpoint_with_auth() {
    let app = test::init_service(App::new().service(detokenize)).await;

    // Pre-store some data in Redis
    store_tokenized_data("token1".to_string(), "value1".to_string());
    store_tokenized_data("token2".to_string(), "value2".to_string());

    let payload = json!({
        "id": "req-33445",
        "data": {
            "field1": "token1",
            "field2": "token2",
            "field3": "invalid_token"
        }
    });

    let req = test::TestRequest::post()
        .uri("/detokenize")
        .insert_header(("x-api-key", "reader-key-456"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let detokenized_data: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(detokenized_data["data"]["field1"]["value"], "value1");
    assert_eq!(detokenized_data["data"]["field2"]["value"], "value2");
    assert_eq!(detokenized_data["data"]["field3"]["found"], false);
}

#[actix_web::test]
async fn test_detokenize_endpoint_without_auth() {
    let app = test::init_service(App::new().service(detokenize)).await;

    let payload = json!({
        "id": "req-33445",
        "data": {
            "field1": "token1",
            "field2": "token2"
        }
    });

    let req = test::TestRequest::post()
        .uri("/detokenize")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}
