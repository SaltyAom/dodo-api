use juniper::GraphQLObject;

use serde::Serialize;

#[derive(Serialize, GraphQLObject, Clone)]
pub struct Article {
    pub title: &'static str,
    pub date: i32,
    pub author: &'static str,
    pub image: &'static str,
    pub data: String,
}
