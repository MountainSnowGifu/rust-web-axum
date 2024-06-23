extern crate jsonwebtoken as jwt;

use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use base64::engine::{general_purpose, Engine};
use http::StatusCode;
use jwt::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

//https://atmarkit.itmedia.co.jp/ait/articles/1608/10/news021.html
//https://qiita.com/Marusoccer/items/1c597dd91f4446509b61
//https://qiita.com/knaot0/items/8427918564400968bd2b
//https://qiita.com/Marusoccer/items/c8632134f08a935ccfc9

pub async fn authenticate(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // extract the authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    tracing::debug!("authenticate:{:#?}", auth_header);

    Ok(next.run(req).await)
}

pub async fn login(req: Request<Body>) -> impl IntoResponse {
    // extract the authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let mut _authenticated = false;

    //BASIC認証ロジック
    match auth_header {
        Some(val) => {
            tracing::debug!("auth_header:{:#?}", val);
            _authenticated = is_basic_authenticated(val.to_string());
        }
        None => {
            _authenticated = false;
        }
    }

    if _authenticated {
        let jwt = generate_jwt().unwrap();
        tracing::debug!("jwt:{:#?}", jwt);
        let decoding_key = DecodingKey::from_secret(JWT_ENCODING_KEY);
        let validation = Validation::default();
        match decode::<Claims>(&jwt, &decoding_key, &validation) {
            Ok(TokenData { claims, .. }) => {
                println!("Decoded claims: {:?}", claims);
            }
            Err(err) => {
                println!("Failed to decode token: {:?}", err);
            }
        }
        (StatusCode::OK, "authenticated").into_response()
    } else {
        let mut response = Response::new(Body::from("Unauthorized"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;
        response.headers_mut().insert(
            "WWW-Authenticate",
            "Basic realm=\"Access to the protected area\""
                .parse()
                .unwrap(),
        );
        response.into_response()
    }
}

const BASIC_AUTH_KEY: &str = "b25lOmtpdA=="; //onekit
fn is_basic_authenticated(auth_header: String) -> bool {
    let second_word_result = auth_header.split_whitespace().nth(1);
    match second_word_result {
        Some(second_word) => {
            tracing::debug!("{second_word:<30} (second_word)");
            let b64 = general_purpose::STANDARD.encode(second_word);
            tracing::debug!("{b64:<30} (Base64 encoded)");
            let decoded = general_purpose::STANDARD.decode(b64).unwrap();
            let decoded = String::from_utf8(decoded).unwrap();
            tracing::debug!("{decoded:<30} (Base64 decoded)");
            tracing::debug!("{BASIC_AUTH_KEY:<30} (BASIC_AUTH_KEY)");
            BASIC_AUTH_KEY == decoded
        }
        None => {
            return false;
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    exp: i64,
}

const JWT_ENCODING_KEY: &[u8; 13] = b"onekit_secret";

fn generate_jwt() -> anyhow::Result<String> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp();
    let claims = Claims {
        user_id: "master".to_string(),
        exp: exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_ENCODING_KEY),
    )
    .unwrap();
    Ok(token)
}
