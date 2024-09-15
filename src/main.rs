use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet{
        username: String::from("Rafa Villa"),
        content: String::from("this is my first tweet"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
