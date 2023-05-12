pub struct QueryRoot;

//pub type Schema = juniper::RootNode<'static, QueryRoot, juniper::EmptyMutation<()>, juniper::EmptySubscription<()>>;

#[juniper::graphql_object]
impl QueryRoot {
    fn get_instance0() -> EventInstance0Object {
        EventInstance0Object{
            title: "title".to_string()
        }
    }
}

 #[derive(juniper::GraphQLObject)]
 struct EventInstance0Object {
    title: String,
 }

// #[Object(name = "EventInstance0")]
// impl EventInstance0Object {
//     async fn title(&self) -> &String {
//         self.0.title()
//     }

//     async fn description(&self) -> &String {
//         self.0.description()
//     }

//     async fn from_date(&self) -> DateTime<Utc> {
//         self.0.from_date()
//     }

//     async fn to_date(&self) -> DateTime<Utc>{
//         self.0.to_date()
//     }

//     async fn start_transition_mins(&self) -> i16 {
//         self.0.start_transition_mins()
//     }

//     async fn end_transition_mins(&self) -> i16 {
//         self.0.end_transition_mins()
//     }

//     async fn guest_min_count(&self) -> Option<i16> {
//         self.0.guest_min_count()
//     }

//     async fn guest_max_count(&self) -> Option<i16> {
//         self.0.guest_max_count()
//     }
// }