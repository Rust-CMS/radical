use async_trait::async_trait;

use super::Page;

#[async_trait]
pub trait Model {
    async fn create() -> Page;
    async fn read_one(id: i32) -> Page;
    async fn read_all() -> Vec<Page>;
    async fn update(id: u32) -> Page;
    async fn delete(id: u32) -> Page;
}