use crate::models::{BaseParser, IMangaChapterPage, IMangaInfo};
use async_trait::async_trait;

#[async_trait]
pub trait MangaParser: BaseParser {
    async fn fetch_manga_info(
        &self,
        manga_id: String,
    ) -> anyhow::Result<IMangaInfo>;

    async fn fetch_chapter_pages(
        &self,
        chapter_id: String,
    ) -> anyhow::Result<Vec<IMangaChapterPage>>;
}
