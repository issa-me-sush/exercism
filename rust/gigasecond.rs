use time::{Duration, PrimitiveDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    start + gigasecond
}