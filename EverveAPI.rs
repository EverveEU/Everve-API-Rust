extern crate reqwest;

use std::collections::HashMap;

struct EverveAPI {
    api_key: String,
    base_url: String,
    format: String,
}

impl EverveAPI {
    fn make_request(&self, endpoint: &str, mut params: HashMap<&str, String>) -> Result<String, reqwest::Error> {
        params.insert("api_key", self.api_key.clone());
        params.insert("format", self.format.clone());
        let client = reqwest::blocking::Client::new();
        let resp = client.get(&format!("{}{}", self.base_url, endpoint))
            .query(&params)
            .send()?
            .text()?;
        Ok(resp)
    }

    fn get_user(&self) -> Result<String, reqwest::Error> {
        self.make_request("user", HashMap::new())
    }

    fn get_socials(&self) -> Result<String, reqwest::Error> {
        self.make_request("socials", HashMap::new())
    }

    fn get_categories(&self, id: Option<&str>) -> Result<String, reqwest::Error> {
        let endpoint = match id {
            Some(id) => format!("categories/{}", id),
            None => "categories".to_string(),
        };
        self.make_request(&endpoint, HashMap::new())
    }

    fn create_order(&self, params: HashMap<&str, String>) -> Result<String, reqwest::Error> {
        self.make_request("orders", params)
    }

    fn get_orders(&self, id: Option<&str>) -> Result<String, reqwest::Error> {
        let endpoint = match id {
            Some(id) => format!("orders/{}", id),
            None => "orders".to_string(),
        };
        self.make_request(&endpoint, HashMap::new())
    }

    fn update_order(&self, id: &str, params: HashMap<&str, String>) -> Result<String, reqwest::Error> {
        self.make_request(&format!("orders/{}", id), params)
    }

    fn delete_order(&self, id: &str) -> Result<String, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("_method", "DELETE".to_string());
        self.make_request(&format!("orders/{}", id), params)
    }
}

// EXAMPLE:
fn main() -> Result<(), reqwest::Error> {
    let api = EverveAPI {
        api_key: "your_api_key_here".to_string(),
        base_url: "https://api.everve.net/v3/".to_string(),
        format: "json".to_string(),
    };

    println!("User Info: {}", api.get_user()?);
    println!("Socials: {}", api.get_socials()?);
    println!("Categories: {}", api.get_categories(None)?);

    let mut new_order_params = HashMap::new();
    new_order_params.insert("param1", "value1".to_string());
    println!("New Order: {}", api.create_order(new_order_params)?);

    println!("Orders: {}", api.get_orders(None)?);

    let mut update_order_params = HashMap::new();
    update_order_params.insert("param1", "newValue1".to_string());
    println!("Updated Order: {}", api.update_order("1", update_order_params)?);

    println!("Deleted Order: {}", api.delete_order("1")?);

    Ok(())
}
