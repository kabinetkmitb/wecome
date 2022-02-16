use crate::types::form::{FormFieldProperty, FormFieldType};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref detail_fields: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "nama kompetisi".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("Nama".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "kategori kompetisi".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "deskripsi kompetisi".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "tags kompetisi".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "tanggal pelaksanaan".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "batas registrasi".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "link registrasi lomba".to_string(),
			initial_value: "".to_string(),
			placeholder: Some("".to_string()),
			input_type: FormFieldType::Text,
		},
	];
}
