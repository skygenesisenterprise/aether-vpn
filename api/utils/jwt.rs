use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(user_id: i32, secret: &str) -> Result<String, Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<i32, Error> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;
    Ok(token_data.claims.sub.parse().unwrap())
}