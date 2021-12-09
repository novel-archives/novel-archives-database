use super::*;
use crate::domains;
use crate::domains::NovelRepository;

#[async_trait]
pub trait NovelUsecase: Send + Sync + 'static {
    async fn find_novels(&self, ids: &[String]) -> Vec<domains::Novel>;
}

pub struct NovelUsecaseImpl {
    _repository: Arc<dyn NovelRepository>,
}

#[async_trait]
impl NovelUsecase for NovelUsecaseImpl {
    async fn find_novels(&self, _ids: &[String]) -> Vec<domains::Novel> {
        todo!()
    }
}
