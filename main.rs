//yahloo
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<RedditPost>,
}

#[derive(Debug, Deserialize)]
struct RedditPost {
    data: PostData,
}

#[derive(Debug, Deserialize)]
struct PostData {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subreddit = "news"; // You can change this to any subreddit
    let url = format!("https://www.reddit.com/r/{}/top.json?limit=10", subreddit);

    let res = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "RustRedditScraper/1.0")
        .send()
        .await?
        .json::<RedditResponse>()
        .await?;

    println!("\nTop posts from r/{}:\n", subreddit);

    for (i, post) in res.data.children.iter().enumerate() {
        println!("{}. {}", i + 1, post.data.title);
        println!("   🔗 {}", post.data.url);
    }

    Ok(())
}
//hope you like it buddy
