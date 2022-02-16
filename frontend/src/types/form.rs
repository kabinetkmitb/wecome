use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FormFieldType {
	Text,
	TextField,
	DateTime,
	TextWithPrefix { prefix: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormFieldProperty {
	pub key: String,
	pub initial_value: String,
	pub placeholder: Option<String>,
	pub input_type: FormFieldType,
}
