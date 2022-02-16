use crate::schema::kompetisi;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone)]
#[table_name = "kompetisi"]
pub struct Kompetisi {
    pub id: String,
    pub nama_pendaftar: String,
    pub nim_pendaftar: String,
    pub email_pendaftar: String,
    pub nama_lembaga_pendaftar: String,
    pub nama_kompetisi: String,
    pub kategori_kompetisi: String,
    pub deskripsi_kompetisi: String,
    pub tags_kompetisi: String,
    pub tanggal_pelaksanaan: NaiveDateTime,
    pub batas_registrasi: NaiveDateTime,
    pub link_registrasi: String,
    pub link_webiste: String,
    pub link_linkedin: String,
    pub akun_instagram: String,
    pub id_line: String,
    pub akun_twitter: String,
    pub link_poster: String,
}
