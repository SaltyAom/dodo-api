use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

use crate::models::articles::Article;

use super::queries::articles;

pub struct Query {}

#[graphql_object]
impl Query {
    pub async fn getArticles() -> Vec<Article> {
        articles::get_articles().await
    }

    pub async fn searchArticle(title: String) -> Vec<Article> {
        articles::search_article(title).await
    }

    pub async fn getTrendings() -> Vec<Article> {
        articles::get_trendings().await
    }

    pub async fn getLatests() -> Vec<Article> {
        articles::get_trendings().await
    }

    pub async fn getRecommendations() -> Vec<Article> {
        articles::get_recommendations().await
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}