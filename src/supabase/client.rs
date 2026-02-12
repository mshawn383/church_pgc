use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PasswordLoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SupabaseSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn supabase_login(
    url: &str,
    anon_key: &str,
    email: &str,
    password: &str,
) -> Result<SupabaseSession, String> {
    let client = Client::new();

    let endpoint = format!("{}/auth/v1/token?grant_type=password", url);

    let body = PasswordLoginRequest { email, password };

    let res = client
        .post(endpoint)
        .header("apikey", anon_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Login failed: {}", res.status()));
    }

    let session = res.json::<SupabaseSession>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(session)
}