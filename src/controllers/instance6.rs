use async_graphql::{
    InputObject,
    Object,
};
use chrono::{DateTime, Utc};
use getset::{CopyGetters, Getters};

#[derive(Default, Clone)]
pub struct EventInstanceQueries6;

#[Object]
impl EventInstanceQueries6{
    async fn event_instances6(
        &self,
    ) -> Result<Vec<EventInstance6Object>, anyhow::Error> {
        let ret = vec![EventInstance6{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        }];
        Ok(ret.into_gql())
    }
}

#[derive(Default, Clone)]
pub struct EventInstanceMutations6;

#[Object]
impl EventInstanceMutations6{
    async fn create6(
        &self,
        _input: EventInstanceCreateInput6,
    ) -> Result<EventInstance6Object, anyhow::Error> {
        let ret = EventInstance6{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        };
        Ok(ret.into_gql())
    }
}

#[derive(InputObject)]
struct EventInstanceCreateInput6 {
    title: String,
    description: String,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    start_transition_mins: i16,
    end_transition_mins: i16,
    guest_min_count: Option<i16>,
    guest_max_count: Option<i16>,
}

#[derive(Getters, CopyGetters, Clone)]
pub struct EventInstance6 {
    #[getset(get = "pub")]
    title: String,
    #[getset(get = "pub")]
    description: String,
    #[getset(get_copy = "pub")]
    from_date: DateTime<Utc>,
    #[getset(get_copy = "pub")]
    to_date: DateTime<Utc>,
    #[getset(get_copy = "pub")]
    start_transition_mins: i16,
    #[getset(get_copy = "pub")]
    end_transition_mins: i16,
    #[getset(get_copy = "pub")]
    guest_min_count: Option<i16>,
    #[getset(get_copy = "pub")]
    guest_max_count: Option<i16>,
}

pub trait IntoGQL {
    type Item;

    fn into_gql(self) -> Self::Item;
}

impl<M: IntoGQL, I: IntoIterator<Item = M>> IntoGQL for I {
    type Item = Vec<M::Item>;

    fn into_gql(self) -> Vec<M::Item> {
        self.into_iter().map(IntoGQL::into_gql).collect()
    }
}


macro_rules! derive_graphql_wrapper {
    ($vis:vis $ident:ident) => {
        ::paste::paste! {
            pub struct [<$ident Object>] ($vis $ident);

            impl IntoGQL for $ident {
                type Item = [<$ident Object>];

                fn into_gql(self) -> [<$ident Object>] {
                    [<$ident Object>](self)
                }
            }
        }
    };
}

derive_graphql_wrapper!(EventInstance6);

#[Object(name = "EventInstance6")]
impl EventInstance6Object {
    async fn title(&self) -> &String {
        self.0.title()
    }

    async fn description(&self) -> &String {
        self.0.description()
    }

    async fn from_date(&self) -> DateTime<Utc> {
        self.0.from_date()
    }

    async fn to_date(&self) -> DateTime<Utc>{
        self.0.to_date()
    }

    async fn start_transition_mins(&self) -> i16 {
        self.0.start_transition_mins()
    }

    async fn end_transition_mins(&self) -> i16 {
        self.0.end_transition_mins()
    }

    async fn guest_min_count(&self) -> Option<i16> {
        self.0.guest_min_count()
    }

    async fn guest_max_count(&self) -> Option<i16> {
        self.0.guest_max_count()
    }
}