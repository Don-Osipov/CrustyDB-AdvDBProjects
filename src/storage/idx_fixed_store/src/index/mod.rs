use common::ids::{PageId, StateType};

pub const INDEX_TYPE: StateType = StateType::Tree;

// This must be a power of 2 for extendible hashing to work
pub const STARTING_PAGE_CAPACITY: PageId = 8;

pub mod fixed_index_file;
pub mod fixed_index_page;
pub mod fixed_index_trait;
pub mod fixed_index_tests;
