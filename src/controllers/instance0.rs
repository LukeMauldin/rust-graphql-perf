pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn hello() -> String {
        "Hello, world!".to_owned()
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, juniper::EmptyMutation<()>, juniper::EmptySubscription<()>>;