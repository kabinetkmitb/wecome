use crate::types::form::{FormFieldProperty, FormFieldType};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref PENDAFTAR_FIELDS: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "nama lembaga".to_string(),
			placeholder: Some("Nama Lembaga".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "kontak penanggung jawab pendaftar".to_string(),
			placeholder: Some(
				"Harap memasukkan nomor aktif agar dapat dihubungi PR We-Come".to_string()
			),
			input_type: FormFieldType::TextWithBoxiconsLogo {
				boxicons_class: "bx bxl-whatsapp".to_string()
			},
		}
	];
	pub static ref DETAIL_FIELDS: Vec<FormFieldProperty> = vec![
		FormFieldProperty {
			key: "nama kompetisi".to_string(),
			placeholder: Some("Nama".to_string()),
			input_type: FormFieldType::Text,
		},
		FormFieldProperty {
			key: "kategori kompetisi".to_string(),
			placeholder: Some("Kategori Kompetisi".to_string()),
			input_type: FormFieldType::Select {
				options: vec![
					"riset".to_string(),
					"seni".to_string(),
					"akademik".to_string(),
					"teknologi".to_string(),
					"bisnis".to_string(),
					"bahasa".to_string(),
					"olahraga".to_string(),
					"konferensi".to_string(),
				]
			},
		},
		FormFieldProperty {
			key: "deskripsi kompetisi".to_string(),
			placeholder: Some("Deskripsi Kompetisi".to_string()),
			input_type: FormFieldType::TextArea,
		},
		FormFieldProperty {
			key: "tags kompetisi".to_string(),
			placeholder: Some("ex : matematika, biologi, fisika".to_string()),
			input_type: FormFieldType::TextArea,
		},
		FormFieldProperty {
			key: "tanggal pelaksanaan".to_string(),
			placeholder: None,
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "batas awal registrasi".to_string(),
			placeholder: None,
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "batas akhir registrasi".to_string(),
			placeholder: None,
			input_type: FormFieldType::DateTime,
		},
		FormFieldProperty {
			key: "link registrasi lomba".to_string(),
			placeholder: Some("https://example.com".to_string()),
			input_type: FormFieldType::Text,
		},
	];
	pub static ref KONTAK_FIELDS: Vec<FormFieldProperty> = vec![
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
			key: "line".to_string(),
			placeholder: Some("Id akun line".to_string()),
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
