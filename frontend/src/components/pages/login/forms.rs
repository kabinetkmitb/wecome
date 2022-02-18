use crate::types::form::{FormFieldProperty, FormFieldType};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref LOGIN_FIELDS: Vec<FormFieldProperty> = vec![
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
	];
}
