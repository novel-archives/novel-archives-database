use super::*;
use crate::domains;
use crate::domains::NovelRepository;

pub trait NovelUsecase: Send + Sync + 'static {
    fn find_novels(&self, ids: &[String]) -> Vec<domains::Novel>;
}

pub struct NovelUsecaseImpl {
    repository: Arc<dyn NovelRepository>,
}
