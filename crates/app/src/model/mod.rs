//! Aggregated model for the application's entire data system.

pub mod contacts;
pub mod portfolio;
pub mod rpcs;
pub mod user;

use self::{
    portfolio::{AlloyAddress, AlloyU256},
    user::UserProfile,
};
use super::*;

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub portfolio: portfolio::RawDataModel<AlloyAddress, AlloyU256>,
    pub user: UserProfile,
}

impl Model {
    pub fn new(user: UserProfile) -> Self {
        Self {
            user,
            portfolio: portfolio::RawDataModel::new(),
        }
    }
}
