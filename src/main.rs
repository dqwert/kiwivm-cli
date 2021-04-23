use std::env;

fn main() {
    get_service_info();
}

#[tokio::main]
async fn get_service_info() -> Result<(), reqwest::Error> {
    let veid = match env::var("KIWIVM_VEID") {
        Ok(s) => s,
        Err(_) => String::from("invalid")
    };
    let api_key = match env::var("KIWIVM_API_KEY") {
        Ok(s) => s,
        Err(_) => String::from("invalid")
    };
    let url = format!("https://api.64clouds.com/v1/getServiceInfo?veid={}&api_key={}", veid, api_key);
    println!("url={}", url);

    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}

