use crate::schema::kompetisi;
use crate::utils::serializer::option_json_time;
use chrono::NaiveDateTime;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Clone, Debug, Derivative, AsChangeset)]
#[derivative(Default)]
#[table_name = "kompetisi"]
pub struct UpdateKompetisi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub no_telp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub nama_lembaga_pendaftar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub nama_kompetisi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub kategori_kompetisi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub deskripsi_kompetisi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub tags_kompetisi: Option<String>,
    #[serde(with = "option_json_time")]
    #[derivative(Default(value = "None"))]
    pub tanggal_pelaksanaan: Option<NaiveDateTime>,
    #[serde(with = "option_json_time")]
    #[derivative(Default(value = "None"))]
    pub batas_awal_registrasi: Option<NaiveDateTime>,
    #[serde(with = "option_json_time")]
    #[derivative(Default(value = "None"))]
    pub batas_akhir_registrasi: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub link_registrasi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub link_website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub link_linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub akun_instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub id_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub akun_twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub link_poster: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub status_kompetisi: Option<String>,
    #[serde(with = "option_json_time")]
    #[derivative(Default(value = "None"))]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct AcceptKompetisiResponse {
    pub message: String,
}
