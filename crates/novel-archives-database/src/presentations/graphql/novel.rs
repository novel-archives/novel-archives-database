use super::*;

#[derive(new, Getters)]
pub struct NovelQuery {
    usecase: Arc<dyn usecases::NovelUsecase>,
}

pub struct NovelLoader {
    usecase: Arc<dyn usecases::NovelUsecase>,
}

#[derive(new, SimpleObject)]
pub struct Novel {
    id: String,
}

#[async_trait]
impl Loader<String> for NovelLoader {
    type Value = String;
}

#[Object]
impl NovelQuery {
    async fn users(&self, id: String) -> Result<Novel> {
        todo!()
    }
}
