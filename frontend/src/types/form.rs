use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FormFieldType {
	Text,
	TextArea,
	DateTime,
	Select { options: Vec<String> },
	TextWithPrefix { prefix: String },
	TextHidden,
	TextWithBoxiconsLogo { boxicons_class: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormFieldProperty {
	pub key: String,
	pub placeholder: Option<String>,
	pub input_type: FormFieldType,
}
