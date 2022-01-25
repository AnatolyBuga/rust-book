//Trait Summary has to be brought into scope
use crate::traits2::{Summary, Tweet, NewsArticle, notify};

pub fn traits () {
    let tweet = Tweet {
        username: String::from("anatoly.bugakov"),
        content: String::from("Trump 2020"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("2 new tweet: {}", tweet.default());
    notify(&tweet);
    let k = "hello".as_bytes();
}
