pub struct ClassList {
  pub class_name: String
}

impl ClassList {
  pub fn new(class_name: &str) -> ClassList {
    ClassList {
      class_name: String::from(class_name)
    }
  }

  pub fn add(& mut self, class: &str) {
    self.class_name.push_str(format!(" {}", class).as_str());
  }

  pub fn remove(& mut self, class: &str) {
    let mut class_list: Vec<&str> = self.class_name.split(" ").collect();
    class_list.retain(|c| *c != class);
    self.class_name = class_list.join(" ");
  }
}

pub trait ClassListExt {
  fn class_list_add(&self, class: &str);
  fn class_list_remove(&self, class: &str);
  fn class_list_include(&self, class: &str) -> bool;
}