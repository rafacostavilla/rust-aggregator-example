use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet{
        username: String::from("Rafa Villa"),
        content: String::from("this is my first tweet"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle{
        headline: String::from("Oscar winner complains about Academy"),
        author: String::from("Rafael Costa Villa"),
        location: String::from("AKL, New Zealand"),
        content: String::from("However he won the Oscar, he couldn't agree with \
        the award after all the recent controversy"),
    };

    // println!("1 new tweet: {}", tweet.summarize());
    aggregator::notify(&tweet);
    println!("--------------------------------------------------");
    // println!("New article available! {}", article.summarize());
    aggregator::notify(&article);
    println!("--------------------------------------------------");
    println!("{}", aggregator::return_summarizable().summarize());
}
