use actix_web::{test, App};
use data_privacy_vault::auth;
use serde_json::json;
use data_privacy_vault::routes::{tokenize, detokenize};
use data_privacy_vault::storage::{store_tokenized_data, retrieve_original_data};

#[actix_web::test]
async fn test_tokenize_with_format_validation() {
    let app = test::init_service(App::new().wrap(auth::AuthMiddleware {}).service(tokenize)).await;

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
        assert_eq!(retrieve_original_data(token.as_str().unwrap()).await.unwrap(), original_value);
    }
}

#[actix_web::test]
async fn test_tokenize_with_invalid_format() {
    let app = test::init_service(App::new().wrap(auth::AuthMiddleware {}).service(tokenize)).await;

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

#[actix_web::test]
async fn test_detokenize_success() {
    let app = test::init_service(App::new().wrap(auth::AuthMiddleware {}).service(detokenize)).await;

    let token = "test-token";
    let original_value = "original-value";
    store_tokenized_data(token.to_string(), original_value.to_string()).await;

    let payload = json!({
        "id": "req-67890",
        "data": {
            "field1": token
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
    assert_eq!(detokenized_data["data"]["field1"]["found"].as_bool().unwrap(), true);
    assert_eq!(detokenized_data["data"]["field1"]["value"].as_str().unwrap(), original_value);
}

#[actix_web::test]
async fn test_detokenize_not_found() {
    let app = test::init_service(App::new().wrap(auth::AuthMiddleware {}).service(detokenize)).await;

    let payload = json!({
        "id": "req-67890",
        "data": {
            "field1": "nonexistent-token"
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
    assert_eq!(detokenized_data["data"]["field1"]["found"].as_bool().unwrap(), false);
    assert_eq!(detokenized_data["data"]["field1"]["value"].as_str().unwrap(), "");
}

#[actix_web::test]
async fn test_unauthorized_access() {
    let app = test::init_service(
        App::new()
            .wrap(auth::AuthMiddleware {})
            .service(tokenize)
            .service(detokenize),
    )
    .await;

    let tokenize_payload = json!({
        "id": "req-12345",
        "data": {
            "field1": "value1"
        }
    });

    let tokenize_req = test::TestRequest::post()
        .uri("/tokenize")
        .insert_header(("x-api-key", "invalid-key"))
        .set_json(&tokenize_payload)
        .to_request();

    let tokenize_resp = test::call_service(&app, tokenize_req).await;
    assert_eq!(tokenize_resp.status(), 401);

    let detokenize_payload = json!({
        "id": "req-67890",
        "data": {
            "field1": "test-token"
        }
    });

    let detokenize_req = test::TestRequest::post()
        .uri("/detokenize")
        .insert_header(("x-api-key", "invalid-key"))
        .set_json(&detokenize_payload)
        .to_request();

    let detokenize_resp = test::call_service(&app, detokenize_req).await;
    assert_eq!(detokenize_resp.status(), 401);
}
