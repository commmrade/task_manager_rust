
use jwt_simple::{
    claims::Claims,
    common::VerificationOptions,
    prelude::{Duration, HS256Key, MACLike},

};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
}

pub fn make_token(name: &str) -> String {
    let key = HS256Key::from_bytes(b"vladivostok1488");
    let claims = Claims::with_custom_claims(
        User {
            username: name.to_string(),
        },
        Duration::from_hours(10),
    );

    let token = key.authenticate(claims);
    token.unwrap()
}

pub fn check_token(token: &str) -> bool {
    let secret_key = HS256Key::from_bytes(b"vladivostok1488");

    let mut options = VerificationOptions::default();
    options.max_validity = Some(Duration::from_hours(10));

    secret_key.verify_token::<User>(&token, Some(options)).is_ok()
}
pub fn check_token_and_name(token: &str, name: &str) -> bool {
    let secret_key = HS256Key::from_bytes(b"vladivostok1488");

    let mut options = VerificationOptions::default();
    options.max_validity = Some(Duration::from_hours(10));

    secret_key.verify_token::<User>(&token, Some(options))
    .map(|clms| clms.custom.username == name)
    .unwrap_or_else(|why| { return false; })
}
