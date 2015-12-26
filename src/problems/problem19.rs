use chrono::UTC;
use chrono::offset::TimeZone;
use chrono::Datelike;
use chrono::Weekday;

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	let mut current = UTC.ymd(1901, 1, 1);
	let end = UTC.ymd(2000, 12, 31);

	let mut count = 0;
	while current < end {
		if current.weekday() == Weekday::Sun {
			count += 1;
		}

		if current.month() == 12 {
			current = current.with_month(1).unwrap().with_year(current.year() + 1).unwrap()
		} else {
			current = current.with_month(current.month() + 1).unwrap();
		}
	}

    count
}
