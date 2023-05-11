#[allow(dead_code)]

use async_graphql::{
    InputObject,
    Object,
    SimpleObject,
};
use chrono::{DateTime, Utc};
use getset::{CopyGetters, Getters};

#[derive(Default, Clone)]
pub struct EventInstanceQueries9;

#[Object]
impl EventInstanceQueries9{
    async fn event_instances9(
        &self,
    ) -> Result<Vec<EventInstance9Object>, anyhow::Error> {
        let ret: Vec<EventInstance9Object> = vec![EventInstance9Object{
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
pub struct EventInstanceMutations9;

#[Object]
impl EventInstanceMutations9{
    async fn create9(
        &self,
        _input: EventInstanceCreateInput9,
    ) -> Result<EventInstance9Object, anyhow::Error> {
        let _model = EventInstance9{
            title: "a".to_string(),
            description: "a".to_string(),
            from_date: Utc::now(),
            to_date: Utc::now(),
            start_transition_mins: 0,
            end_transition_mins: 1,
            guest_min_count: None,
            guest_max_count: None,
        };
        let ret = EventInstance9Object{
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
struct EventInstanceCreateInput9 {
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
pub struct EventInstance9 {
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
pub struct EventInstance9Object{
    title: String,
    description: String,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    start_transition_mins: i16,
    end_transition_mins: i16,
    guest_min_count: Option<i16>,
    guest_max_count: Option<i16>,
}
