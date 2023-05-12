use chrono::{DateTime, Utc};

#[derive(juniper::GraphQLObject)]
pub struct EventInstance8Object {
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
pub struct EventInstanceCreateInput8 {
    title: String,
    description: String,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    start_transition_mins: i32,
    end_transition_mins: i32,
    guest_min_count: Option<i32>,
    guest_max_count: Option<i32>,
}

pub fn event_instances8() -> EventInstance8Object {
    EventInstance8Object {
        title: "a".to_string(),
        description: "a".to_string(),
        from_date: Utc::now(),
        to_date: Utc::now(),
        start_transition_mins: 8,
        end_transition_mins: 1,
        guest_min_count: None,
        guest_max_count: None,
    }
}

pub async fn create8(
    _input: EventInstanceCreateInput8,
) -> Result<EventInstance8Object, anyhow::Error> {
    let ret = EventInstance8Object {
        title: "a".to_string(),
        description: "a".to_string(),
        from_date: Utc::now(),
        to_date: Utc::now(),
        start_transition_mins: 8,
        end_transition_mins: 1,
        guest_min_count: None,
        guest_max_count: None,
    };
    Ok(ret)
}
