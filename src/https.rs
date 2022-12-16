use reqwest;




pub fn get(url : &str, project_ket: &str) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
        .header("X-API-Key", project_ket)
        .header("Content-Type", "application/json")
        .send()
        .unwrap();
    res
}

pub fn put(url : &str, project_key: &str, body: serde_json::Value) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let res = client.put(url)
        .header("X-API-Key", project_key)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .unwrap();
    res
}

pub fn post(url : &str, project_key: &str, body: serde_json::Value) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header("X-API-Key", project_key)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .unwrap();
    res
}

pub fn delete(url : &str, project_key: &str) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let res = client.delete(url)
        .header("X-API-Key", project_key)
        .header("Content-Type", "application/json")
        .send()
        .unwrap();
    res
}

pub fn patch(url : &str, project_key: &str, body: serde_json::Value) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let res = client.patch(url)
        .header("X-API-Key", project_key)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .unwrap();
    res
}