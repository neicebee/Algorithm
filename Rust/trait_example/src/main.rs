use trait_example::*;

fn main() {
    let tweet = Tweet {
        username: String::from("f1r3_r41n"),
        content: String::from("트레이트 구현 단계 ing.."),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());
}