use crate::types::form::{FormFieldProperty, FormFieldType};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref REGISTER_FIELDS: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "full name".to_string(),
			placeholder: Some("Name".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "nim".to_string(),
			placeholder: Some("Nim".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "email".to_string(),
			placeholder: Some("Email".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "kata sandi".to_string(),
			placeholder: Some("Kata Sandi".to_string()),
			input_type: FormFieldType::TextHidden,
		},
		FormFieldProperty {
			key: "konfirmasi kata sandi".to_string(),
			placeholder: Some("Konfirmasi Kata Sandi".to_string()),
			input_type: FormFieldType::TextHidden,
		},
	];
}
