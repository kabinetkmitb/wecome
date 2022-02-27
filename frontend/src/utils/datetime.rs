pub fn from_rfc3339_to_date(date: String) -> String {
	(&date[0..10]).to_string()
}
