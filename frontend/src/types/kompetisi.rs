use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ProposeKompetisiPayload {
	pub nama_lembaga_pendaftar: String,
	pub no_telp: String,
	pub nama_kompetisi: String,
	pub kategori_kompetisi: String,
	pub deskripsi_kompetisi: String,
	pub tags_kompetisi: String,
	pub tanggal_pelaksanaan: String,
	pub batas_awal_registrasi: String,
	pub batas_akhir_registrasi: String,
	pub link_registrasi: String,
	pub link_webiste: String,
	pub link_linkedin: String,
	pub akun_instagram: String,
	pub id_line: String,
	pub akun_twitter: String,
	pub link_poster: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProposeKompetisiResponse {
	pub message: String,
}
