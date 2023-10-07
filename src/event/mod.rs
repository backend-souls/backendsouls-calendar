pub mod request;
pub mod response;

pub struct Event {}

impl Event {
    const MEDIA_TYPE: &'static str = "application/jscalendar+json;type=event";
}
