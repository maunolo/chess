use std::collections::HashMap;

pub struct Style {
  pub properties: HashMap<String, String>
}

impl Style {
  pub fn new(value: &str) -> Style {
    let mut properties = HashMap::new();
    for item in value.split(";").map(|s| {
      if !s.is_empty() {
        let split: Vec<&str> = s.split(":").collect();
        Some((split[0].to_string(), split[1].to_string()))
      } else {
        None
      }
    }) {
      if item.is_some() {
        let (key, value) = item.unwrap();
        properties.insert(key, value);
      }
    }
    Style {
      properties
    }
  }

  pub fn set(&mut self, key: &str, value: &str) {
    self.properties.insert(String::from(key), String::from(value));
  }

  pub fn to_string(&self) -> String {
    let mut style = String::new();
    for (key, value) in &self.properties {
      style.push_str(format!("{}:{};", key, value).as_str());
    }
    style
  }
}

pub trait StyleExt {
  fn style_set(&self, key: &str, value: &str);
}
