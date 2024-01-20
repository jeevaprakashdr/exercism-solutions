use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_billion_seconds = 1000000000;
    start
        .checked_add(Duration::seconds(one_billion_seconds))
        .unwrap()
}
