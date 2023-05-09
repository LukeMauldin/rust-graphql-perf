use async_graphql::{
    Object,
};
#[derive(Clone)]
pub struct Query;

#[Object]
impl Query {
  async fn howdy(&self) -> &'static str {
    "partner"
  }
}