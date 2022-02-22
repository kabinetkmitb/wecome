use crate::types::kompetisi::{
	AcceptKompetisiResponse, DeclineKompetisiResponse, KompetisiResponse, ProposeKompetisiPayload,
	ProposeKompetisiResponse,
};
use crate::utils::api::{request_get, request_post, request_put_without_body};

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

pub async fn accept_kompetisi(kompetisi_id: String) -> Result<AcceptKompetisiResponse, Error> {
	request_put_without_body::<AcceptKompetisiResponse>(format!(
		"/kompetisi/accept-kompetisi/{}",
		kompetisi_id
	))
	.await
}

pub async fn decline_kompetisi(kompetisi_id: String) -> Result<DeclineKompetisiResponse, Error> {
	request_put_without_body::<DeclineKompetisiResponse>(format!(
		"/kompetisi/decline-kompetisi/{}",
		kompetisi_id
	))
	.await
}
