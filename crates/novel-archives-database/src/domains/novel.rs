use super::*;

#[derive(Entity, new, Getters)]
pub struct Novel {
    id: Id<Self>,
    title: NovelTitle,
    description: NovelDescription,
}

#[derive(PartialEq)]
pub struct NovelTitle(String);

#[derive(PartialEq)]
pub struct NovelDescription(String);
