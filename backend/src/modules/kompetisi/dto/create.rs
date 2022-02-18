use crate::schema::kompetisi;
use crate::utils::serializer::{default_time, json_time};
use chrono::NaiveDateTime;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Clone, Debug, Derivative)]
#[derivative(Default)]
#[table_name = "kompetisi"]
pub struct CreateKompetisi {
    #[derivative(Default(value = "crate::utils::cuid::get_cuid()"))]
    #[serde(default = "crate::utils::cuid::get_cuid")]
    pub id: String,
    pub user_id: String,
    pub no_telp: String,
    pub nama_lembaga_pendaftar: String,
    pub nama_kompetisi: String,
    pub kategori_kompetisi: String,
    pub deskripsi_kompetisi: String,
    pub tags_kompetisi: String,
    #[serde(default = "default_time", with = "json_time")]
    #[derivative(Default(value = "default_time()"))]
    pub tanggal_pelaksanaan: NaiveDateTime,
    #[serde(default = "default_time", with = "json_time")]
    #[derivative(Default(value = "default_time()"))]
    pub batas_awal_registrasi: NaiveDateTime,
    #[serde(default = "default_time", with = "json_time")]
    #[derivative(Default(value = "default_time()"))]
    pub batas_akhir_registrasi: NaiveDateTime,
    #[serde(default = "default_status")]
    #[derivative(Default(value = "String::from(\"Pending\")"))]
    pub status: String,
    pub link_registrasi: String,
    pub link_website: String,
    pub link_linkedin: String,
    pub akun_instagram: String,
    pub id_line: String,
    pub akun_twitter: String,
    pub link_poster: String,
}

fn default_status() -> String {
    String::from("Pending")
}
