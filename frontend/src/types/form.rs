use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FormFieldType {
	Text,
	TextField,
	DateTime,
	Select { options: Vec<String> },
	TextWithPrefix { prefix: String },
	TextHidden,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormFieldProperty {
	pub key: String,
	pub placeholder: Option<String>,
	pub input_type: FormFieldType,
}
