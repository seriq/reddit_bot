use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::env;

pub async fn run() -> Result<(), reqwest::Error> {
    let params = set_login_parameters();
    let user_agent = "reddit_bot by ".to_owned() + &params.username;
    let login_response = login(&params, &user_agent).await?;
    println!("{:?}", login_response);

    let base_url: reqwest::Url = "https://oauth.reddit.com/".parse().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        (login_response.token_type + &login_response.access_token)
            .parse()
            .unwrap(),
    );
    headers.insert("User-Agent", user_agent.parse().unwrap());
    let resp = Client::new()
        .get(base_url.join("api/v1/me").unwrap())
        .headers(headers)
        .send()
        .await?
        .json::<MeResponse>()
        .await?;
    //let stuff = serde_json::from_str(&resp).unwrap();
    println!("{:#?}", resp);
    //    In [8]: headers = {"Authorization": "bearer fhTdafZI-0ClEzzYORfBSCR7x3M", "User-Agent": "ChangeMeClient/0.1 by YourUsername"}
    //    In [9]: response = requests.get("https://oauth.reddit.com/api/v1/me", headers=headers)
    //    In [10]: response.json()
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct MeResponse {
    features: Value,
}
async fn login(
    params: &LoginParameters,
    user_agent: &String,
) -> Result<LoginResponse, reqwest::Error> {
    let client_id_and_secret = set_client_id_and_secret();
    let resp = Client::new()
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(
            client_id_and_secret.username,
            Some(client_id_and_secret.password),
        )
        .header("User-Agent", user_agent)
        .form(params)
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;
    Ok(resp)
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginParameters {
    grant_type: String,
    username: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ClientIdAndSecret {
    username: String,
    password: String,
}

fn set_login_parameters() -> LoginParameters {
    let username = env::var("REDDIT_BOT_USERNAME")
        .expect("Must have environment variable REDDIT_BOT_USERNAME set.");
    let password = env::var("REDDIT_BOT_PASSWORD")
        .expect("Must have environment variable REDDIT_BOT_PASSWORD set.");
    LoginParameters {
        grant_type: "password".to_owned(),
        username,
        password,
    }
}

fn set_client_id_and_secret() -> ClientIdAndSecret {
    let username =
        env::var("REDDIT_CLIENT_ID").expect("Must have environment variable REDDIT_CLIENT_ID set.");
    let password = env::var("REDDIT_CLIENT_PASSWORD")
        .expect("Must have environment variable REDDIT_CLIENT_PASSWORD set.");
    ClientIdAndSecret { username, password }
}
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    body: Value,
}

async fn bla() -> Result<(), Box<dyn std::error::Error>> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", echo_json);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
