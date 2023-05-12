pub mod instance0;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn event_instances0() -> instance0::EventInstance0Object {
        instance0::event_instances0()
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create0(
        input: instance0::EventInstanceCreateInput0,
    ) -> instance0::EventInstance0Object {
        instance0::create0(input).await.unwrap()
    }
}
