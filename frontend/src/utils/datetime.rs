use chrono::DateTime;

pub fn from_rfc3339_to_date(date_string: String) -> String {
	let date = DateTime::parse_from_rfc3339(&date_string).unwrap();

	date.format("%d-%m-%Y").to_string()
}
