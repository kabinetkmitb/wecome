use crate::utils::serializer::{default_time, json_time};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// DONT CHANGE THE ORDER OF DESERIALIZED FIELDS
#[derive(Deserialize, Serialize, Queryable, Clone)]
pub struct Kompetisi {
    pub id: String,
    pub nama_lembaga_pendaftar: String,
    pub nim_pendaftar: String,
    pub no_telp: String,
    pub nama_kompetisi: String,
    pub kategori_kompetisi: String,
    pub deskripsi_kompetisi: String,
    pub tags_kompetisi: String,
    #[serde(default = "default_time", with = "json_time")]
    pub tanggal_pelaksanaan: NaiveDateTime,
    #[serde(default = "default_time", with = "json_time")]
    pub batas_awal_registrasi: NaiveDateTime,
    #[serde(default = "default_time", with = "json_time")]
    pub batas_akhir_registrasi: NaiveDateTime,
    pub link_registrasi: String,
    pub link_website: String,
    pub link_linkedin: String,
    pub akun_instagram: String,
    pub id_line: String,
    pub akun_twitter: String,
    pub link_poster: String,
    pub status_kompetisi: String,
    pub user_id: String,
}
