use actix_web::{post, web, HttpResponse, Responder};
use crate::storage::{store_tokenized_data, retrieve_original_data};
use crate::utils::{generate_token, validate_data_format};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct TokenizeRequest {
    id: String,
    data: HashMap<String, String>,
    format: Option<HashMap<String, String>>, // Optional format specification
}

#[derive(Serialize)]
struct TokenizeResponse {
    id: String,
    data: HashMap<String, String>,
}

#[post("/tokenize")]
async fn tokenize(req: web::Json<TokenizeRequest>) -> impl Responder {
    if let Some(format) = &req.format {
        if let Err(err) = validate_data_format(&req.data, format) {
            return HttpResponse::BadRequest().body(format!("Invalid data format: {}", err));
        }
    }

    let mut tokenized_data = HashMap::new();

    for (key, value) in &req.data {
        let token = generate_token(value);
        tokenized_data.insert(key.clone(), token.clone());
        store_tokenized_data(token, value.clone());
    }

    let response = TokenizeResponse {
        id: req.id.clone(),
        data: tokenized_data,
    };

    HttpResponse::Created().json(response)
}

#[derive(Deserialize)]
struct DetokenizeRequest {
    id: String,
    data: HashMap<String, String>,
}

#[derive(Serialize)]
struct DetokenizeResponse {
    id: String,
    data: HashMap<String, DetokenizeField>,
}

#[derive(Serialize)]
struct DetokenizeField {
    found: bool,
    value: String,
}

#[post("/detokenize")]
async fn detokenize(req: web::Json<DetokenizeRequest>) -> impl Responder {
    let mut detokenized_data = HashMap::new();

    for (key, token) in &req.data {
        if let Some(original_value) = retrieve_original_data(token) {
            detokenized_data.insert(
                key.clone(),
                DetokenizeField {
                    found: true,
                    value: original_value,
                },
            );
        } else {
            detokenized_data.insert(
                key.clone(),
                DetokenizeField {
                    found: false,
                    value: String::new(),
                },
            );
        }
    }

    let response = DetokenizeResponse {
        id: req.id.clone(),
        data: detokenized_data,
    };

    HttpResponse::Ok().json(response)
}
