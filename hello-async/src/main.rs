use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let url = &args[1];
    trpl::block_on(async {
        match page_title(url).await {
            Some(title) => println!("Page title: {title}"),
            None => println!("Could not find a title for the page at {url}"),
        }
    })
}
