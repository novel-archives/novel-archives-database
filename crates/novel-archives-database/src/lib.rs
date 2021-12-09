#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate domain_utils_macro;

#[macro_use]
extern crate derive_getters;

#[macro_use]
extern crate async_trait;

pub mod domains;
pub mod presentations;
pub mod usecases;

use async_std::sync::Arc;
