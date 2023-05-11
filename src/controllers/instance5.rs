#[allow(dead_code)]

use async_graphql::{
    InputObject,
    Object,
    SimpleObject,
};
use chrono::{DateTime, Utc};
use getset::{CopyGetters, Getters};

#[derive(Default, Clone)]
pub struct EventInstanceQueries5;

#[Object]
impl EventInstanceQueries5{
    async fn event_instances5(
        &self,
    ) -> Result<Vec<EventInstance5Object>, anyhow::Error> {
        let ret: Vec<EventInstance5Object> = vec![EventInstance5Object{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        }];
        Ok(ret)
    }
}

#[derive(Default, Clone)]
pub struct EventInstanceMutations5;

#[Object]
impl EventInstanceMutations5{
    async fn create5(
        &self,
        _input: EventInstanceCreateInput5,
    ) -> Result<EventInstance5Object, anyhow::Error> {
        let _model = EventInstance5{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        };
        let ret = EventInstance5Object{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        };
        Ok(ret)
    }
}

#[derive(InputObject)]
struct EventInstanceCreateInput5 {
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
pub struct EventInstance5 {
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


#[derive(SimpleObject)]
pub struct EventInstance5Object{
    title: String,
    description: String,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    start_transition_mins: i16,
    end_transition_mins: i16,
    guest_min_count: Option<i16>,
    guest_max_count: Option<i16>,
}
