use crate::usecases::NovelUsecase;

pub trait NovelPresentation {
    type Usecase: NovelUsecase;
    fn usecase(&self) -> &Self::Usecase;
}
