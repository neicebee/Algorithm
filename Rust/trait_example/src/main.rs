use trait_example::*;

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("f1r3_r41n"),
        content: String::from("트레이트 구현 단계 ing.."),
        reply: false,
        retweet: false,
    }
}
fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("korea"),
        author: String::from("f1r3_r41n"),
        content: String::from("text..."),
    };
    let tweet = returns_summarizable();

    println!("새 트윗 1개: {}", tweet.summarize());
    println!("새로운 기사: {}", article.summarize());
}