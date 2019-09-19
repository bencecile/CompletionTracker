mod search;
pub use self::search::{search};
pub mod universe_tag;

use serde_derive::{Serialize};

#[derive(Serialize)]
pub struct APIResult<T> {
    pub success: bool,
    pub data: T,
}
