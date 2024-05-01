use ch102trait::{notify, notify2, Summary};

fn main() {
  let tweet = ch102trait::Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {} by {}", tweet.summarize(), tweet.summarize_author());
  notify(&tweet);
  notify2(&tweet);
}
