trait GetInfo {
  fn get_info(&self) -> String;

  fn hide_info(&self) {
    println!("This is a hidden info");
  }
}

struct Massage {
  author: String,
  text: String,
}

struct Post {
  author: String,
  content: String,
  likes: u32,
}

impl GetInfo for Massage {
  fn get_info(&self) -> String {
    format!("Author: {}, Text: {}", self.author, self.text)
  }
}

impl GetInfo for Post {
  fn get_info(&self) -> String {
    format!("Author: {}, Content: {}, Likes: {}", self.author, self.content, self.likes)
  }
}

fn main() {
  let massage = Massage { author: "Alice".to_string(), text: "Hello, world!".to_string() };
  let post = Post { author: "Bob".to_string(), content: "Rust is awesome!".to_string(), likes: 100 };
  println!("{}", massage.get_info());
  println!("{}", post.get_info());

  post.hide_info();
  massage.hide_info();
}