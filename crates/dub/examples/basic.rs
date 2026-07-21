use dub::{CreateLinkRequest, Dub, ListLinksParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API token from environment
    let api_token =
        std::env::var("DUB_API_TOKEN").expect("DUB_API_TOKEN environment variable not set");

    // Create the client
    let dub = Dub::new(api_token)?;

    // List existing links
    println!("Listing links...");
    let params = ListLinksParams {
        limit: Some(5),
        ..Default::default()
    };
    let links = dub.links().list(Some(params)).await?;
    println!("Found {} links:", links.len());
    for link in &links {
        println!("  - {}/{} -> {}", link.domain, link.key, link.url);
        println!("    Clicks: {}", link.clicks);
    }

    // Create a new link
    println!("\nCreating a new link...");
    let request = CreateLinkRequest {
        url: "https://github.com/rust-lang/rust".to_string(),
        domain: Some("dub.sh".to_string()),
        key: Some(format!("rust-{}", chrono::Utc::now().timestamp())),
        title: Some("Rust Programming Language".to_string()),
        description: Some(
            "A language empowering everyone to build reliable and efficient software.".to_string(),
        ),
        tags: Some(vec!["rust".to_string(), "programming".to_string()]),
        ..Default::default()
    };

    let new_link = dub.links().create(request).await?;
    println!("Created link:");
    println!("  URL: {}/{}", new_link.domain, new_link.key);
    println!("  Points to: {}", new_link.url);
    println!("  ID: {}", new_link.id);

    // Get link details
    println!("\nFetching link details...");
    let link = dub.links().get(&new_link.id).await?;
    println!("Link details:");
    println!("  Title: {:?}", link.title);
    println!("  Created: {}", link.created_at);
    println!("  Tags: {:?}", link.tags);

    // Get analytics
    println!("\nFetching analytics...");
    let analytics = dub.analytics().get_link(&new_link.id).await?;
    println!("Analytics:");
    println!("  Total clicks: {}", analytics.clicks);
    if !analytics.data.is_empty() {
        println!("  Data points: {}", analytics.data.len());
    }

    // List workspaces
    println!("\nListing workspaces...");
    let workspaces = dub.workspaces().list().await?;
    println!("Found {} workspaces:", workspaces.len());
    for workspace in workspaces {
        println!("  - {} ({})", workspace.name, workspace.slug);
    }

    // Clean up - delete the link
    println!("\nCleaning up (deleting link)...");
    dub.links().delete(&new_link.id).await?;
    println!("Link deleted successfully!");

    Ok(())
}
