use crate::types::kompetisi::{
	KompetisiResponse, ProposeKompetisiPayload, ProposeKompetisiResponse,
};
use crate::utils::api::{request_get, request_post};

use crate::types::error::Error;

pub async fn propose_kompetisi(
	kompetisi_payload: ProposeKompetisiPayload,
) -> Result<ProposeKompetisiResponse, Error> {
	request_post::<ProposeKompetisiPayload, ProposeKompetisiResponse>(
		"/kompetisi/propose-kompetisi".to_string(),
		kompetisi_payload,
	)
	.await
}

pub async fn get_kompetisi(query: String) -> Result<Vec<KompetisiResponse>, Error> {
	request_get::<Vec<KompetisiResponse>>(format!("/kompetisi{}", query)).await
}
