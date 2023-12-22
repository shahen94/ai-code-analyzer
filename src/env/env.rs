use dotenv::dotenv;

pub struct Env {}

impl Env {
  pub fn get_api_key() -> String {
    return Env::get_value("OPENAI_API_KEY");
  }

  pub fn get_value(key: &str) -> String {
    dotenv().ok();
    return std::env::var(key).unwrap();
  }
}