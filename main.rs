//yahlo0
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    data: Listing,
}

#[derive(Deserialize)]
struct Listing {
    children: Vec<Post>,
}

#[derive(Deserialize)]
struct Post {
    data: PostDetails,
}

#[derive(Deserialize)]
struct PostDetails {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subreddit = "news";
    let api = format!("https://www.reddit.com/r/{}/top.json?limit=10", subreddit);

    let client = reqwest::Client::new();
    let resp = client
        .get(api)
        .header("User-Agent", "rust-scraper/0.1")
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    println!("\nTop posts from r/{}:\n", subreddit);

    for (i, post) in resp.data.children.iter().enumerate() {
        println!("{}. {}", i + 1, post.data.title);
        println!("   {}", post.data.url);
    }

    Ok(())
}
//hope you like it buddy
