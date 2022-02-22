use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindManyKompetisiQuery {
    pub kategori_kompetisi: Option<String>,
    pub nama_kompetisi: Option<String>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}
