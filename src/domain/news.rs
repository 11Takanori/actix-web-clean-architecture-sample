#[derive(Debug, Clone, Copy)]
pub struct NewsId(pub u32);

pub type NewsIds = Vec<NewsId>;

#[derive(Debug, Clone)]
pub struct News {
    pub id: NewsId,
    pub parent_id: NewsId,
    pub text: String,
}

pub type NewsList = Vec<News>;

impl News {
    pub fn new(id: NewsId, parent_id: NewsId, text: impl Into<String>) -> Self {
        News {
            id,
            parent_id,
            text: text.into(),
        }
    }
}
