use reqwest;
use std::io;
use serde::{Serialize, Deserialize};
use anyhow::Result;

const ZIP_URL: &str = "https://zipcloud.ibsnet.co.jp/api/search";

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    zipcode: String,
    prefcode: String,
    address1: String,
    address2: String,
    address3: String,
    kana1: String,
    kana2: String,
    kana3: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    status: i32,
    message: Option<String>,
    results: Vec<Detail>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("検索したい郵便番号を入力してください(ハイフンなし)");

    let mut input_zip_code = String::new();

    io::stdin().read_line(&mut input_zip_code).unwrap();

    let input_zip_code = input_zip_code.trim().to_string();

    let url = format!("{}?zipcode={}", ZIP_URL, input_zip_code);

    // 構造体のすべてのフィールドを使わない場合は、warningがでる
    let contents = fetch_zipcode_api(&url).await.unwrap();
    //let res = serde_json::to_string(&contents).unwrap();

    println!("{:?}", contents);

    Ok(())
}

async fn fetch_zipcode_api(url: &str) -> Result<Address, reqwest::Error> {
    let r = reqwest::get(url).await?;

    r.json::<Address>().await
}

#[cfg(test)]
mod url_test {
    use super::*;

    #[tokio::test]
    async fn test_zip_url() {
        let mut server = mockito::Server::new_async().await;

        let url = server.url();

        let detail = vec![Detail {
            zipcode: "a".to_string(),
            prefcode: "a".to_string(),
            address1: "a".to_string(),
            address2: "a".to_string(),
            address3: "a".to_string(),
            kana1: "a".to_string(),
            kana2: "a".to_string(),
            kana3: "a".to_string(),
        }];

        let address = Address {
            status: 200,
            message: None,
            results: detail,
        };

        let json_body = serde_json::to_string(&address).unwrap();

        let mock = server
            .mock("GET", "/?zipcode=1310031")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .create_async()
            .await;

        let r = fetch_zipcode_api(&format!("{url}/?zipcode=1310031")).await.unwrap();

        assert_eq!(r.status, 200);

        mock.assert_async().await;
    }
}