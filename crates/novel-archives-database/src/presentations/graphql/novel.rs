use std::collections::HashMap;

use super::*;

#[derive(new, Getters)]
pub struct NovelQuery {
    _usecase: Arc<dyn usecases::NovelUsecase>,
}

pub struct NovelLoader {
    _usecase: Arc<dyn usecases::NovelUsecase>,
}

#[derive(new, SimpleObject, Clone)]
pub struct Novel {
    id: String,
}

#[async_trait]
impl Loader<String> for NovelLoader {
    type Value = Novel;
    type Error = Arc<anyhow::Error>;
    async fn load(&self, _keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        todo!()
    }
}

#[Object]
impl NovelQuery {
    async fn users(&self, _id: String) -> Result<Novel> {
        todo!()
    }
}
