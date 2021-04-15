use cached::proc_macro::cached;

use crate::models::articles::Article;

use std::fs::read_to_string;

use super::data;

pub fn parse_article(article: Article) -> Article {
    Article {
        title: article.title,
        date: article.date,
        author: article.author,
        image: article.image,
        data: read_to_string(
            format!("markdown/{}.md", article.data)
        ).unwrap_or_else(|_| article.data),
    }
}

pub fn parse_articles(articles: Vec<Article>) -> Vec<Article> {
    articles
        .into_iter()
        .map(|article| parse_article(article))
        .collect()
}

#[cached]
pub async fn get_articles() -> Vec<Article> {
    parse_articles(
        data::ARTICLES
            .to_owned()
    )
}

#[cached]
pub async fn search_article(title: String) -> Vec<Article> {
    data::ARTICLES
        .to_owned()
        .into_iter()
        .filter(|article| article.title.contains(&title))
        .map(|article| parse_article(article))
        .collect()
}

#[cached]
pub async fn get_trendings() -> Vec<Article> {
    parse_articles(
        data::TRENDINGS
            .to_owned()
    )
}

#[cached]
pub async fn get_latests() -> Vec<Article> {
    parse_articles(
        data::LATESTS
            .to_owned()
    )
}

#[cached]
pub async fn get_recommendations() -> Vec<Article> {
    parse_articles(
        data::RECOMMENDATIONS
            .to_owned()
    )
}
