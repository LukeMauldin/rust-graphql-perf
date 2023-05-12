use chrono::{DateTime, Utc};

#[derive(juniper::GraphQLObject)]
pub struct EventInstance4Object {
    pub title: String,
    pub description: String,
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub start_transition_mins: i32,
    pub end_transition_mins: i32,
    pub guest_min_count: Option<i32>,
    pub guest_max_count: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct EventInstanceCreateInput4 {
    title: String,
    description: String,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    start_transition_mins: i32,
    end_transition_mins: i32,
    guest_min_count: Option<i32>,
    guest_max_count: Option<i32>,
}

pub fn event_instances4() -> EventInstance4Object {
    EventInstance4Object {
        title: "a".to_string(),
        description: "a".to_string(),
        from_date: Utc::now(),
        to_date: Utc::now(),
        start_transition_mins: 4,
        end_transition_mins: 1,
        guest_min_count: None,
        guest_max_count: None,
    }
}

pub async fn create4(
    _input: EventInstanceCreateInput4,
) -> Result<EventInstance4Object, anyhow::Error> {
    let ret = EventInstance4Object {
        title: "a".to_string(),
        description: "a".to_string(),
        from_date: Utc::now(),
        to_date: Utc::now(),
        start_transition_mins: 4,
        end_transition_mins: 1,
        guest_min_count: None,
        guest_max_count: None,
    };
    Ok(ret)
}
