use rand::distributions::{Distribution, Standard};
use rand::Rng;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum EventType {
    SensorTriggered,
    SensorAcknowledged,
    SensorOff,
}

#[derive(Debug)]
pub struct EventData {
    pub event_type: EventType,
    pub timestamp: u64,
    pub correlation_id: u64,
}

impl Distribution<EventData> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EventData {
        let (timestamp, correlation_id) = rng.gen();
        let event_type = match rng.gen_range(0..=EventType::COUNT) {
            0 => EventType::SensorAcknowledged,
            1 => EventType::SensorTriggered,
            _ => EventType::SensorOff,
        };

        EventData {
            event_type,
            timestamp,
            correlation_id,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_event: EventData = rng.gen();
    println!("{:?}", random_event);
}
