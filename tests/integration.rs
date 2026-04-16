use actix_web::{test, App};
use serde_json::json;
use data_privacy_vault::routes::{tokenize, detokenize};

#[actix_web::test]
async fn test_tokenize_endpoint() {
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
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_detokenize_endpoint() {
    let app = test::init_service(App::new().service(detokenize)).await;

    let payload = json!({
        "id": "req-33445",
        "data": {
            "field1": "t6yh4f6",
            "field2": "gh67ned",
            "field3": "invalid_token"
        }
    });

    let req = test::TestRequest::post()
        .uri("/detokenize")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
