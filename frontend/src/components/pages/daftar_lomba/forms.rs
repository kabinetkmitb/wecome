use crate::types::form::{FormFieldProperty, FormFieldType};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref pendaftar_fields: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "nama lembaga".to_string(),
			placeholder: Some("Nama Lembaga".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "no telp".to_string(),
			placeholder: Some("No Telp".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "nim".to_string(),
			placeholder: Some("ex: 13320001".to_string()),
			input_type: FormFieldType::Text,
		},
	];
	pub static ref detail_fields: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "nama kompetisi".to_string(),
			placeholder: Some("Nama".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "kategori kompetisi".to_string(),
			placeholder: Some("Kategori Kompetisi".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "deskripsi kompetisi".to_string(),
			placeholder: Some("Deskripsi Kompetisi".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "tags kompetisi".to_string(),
			placeholder: Some("ex : matematika, biologi, fisika".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "tanggal pelaksanaan".to_string(),
			placeholder: None,
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "batas registrasi".to_string(),
			placeholder: None,
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "link registrasi lomba".to_string(),
			placeholder: Some("https://example.com".to_string()),
			input_type: FormFieldType::Text,
		},
	];
	pub static ref kontak_fields: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "website".to_string(),
			placeholder: Some("https://example.com".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "linkedin".to_string(),
			placeholder: Some("https://www.linkedin.com/in/example".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "instagram".to_string(),
			placeholder: None,
			input_type: FormFieldType::TextWithPrefix {
				prefix: "instagram.com/".to_string()
			},
		},
		FormFieldProperty {
			key: "twitter".to_string(),
			placeholder: None,
			input_type: FormFieldType::TextWithPrefix {
				prefix: "twitter.com/".to_string()
			},
		},
	];
}
