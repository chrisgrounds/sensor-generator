use csv::Writer;
use event_data::EventData;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

mod event_data;

fn write_event_to_csv(wtr: &mut Writer<File>, event_data: EventData) -> Result<(), Box<dyn Error>> {
    wtr.serialize(&event_data)?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    let total_events = 10000;
    let mut rng = rand::thread_rng();
    let mut wtr = Writer::from_path("random_data.csv").unwrap();

    let now = Instant::now();

    for _ in 0..total_events {
        let random_event: EventData = rng.gen();

        write_event_to_csv(&mut wtr, random_event).unwrap();
    }

    println!("Finished in {:?}!", now.elapsed());
}
