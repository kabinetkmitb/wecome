use crate::types::kompetisi::{ProposeKompetisiPayload, ProposeKompetisiResponse};
use crate::utils::api::request_post;

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
