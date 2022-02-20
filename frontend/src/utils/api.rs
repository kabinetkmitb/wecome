use dotenv_codegen::dotenv;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::types::error::Error;

const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_KEY: &str = "token";

lazy_static! {
	/// Jwt token read from local storage.
	pub static ref TOKEN: RwLock<Option<String>> = {
		if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
			RwLock::new(Some(token))
		} else {
			RwLock::new(None)
		}
	};
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
	if let Some(t) = token.clone() {
		LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
	} else {
		LocalStorage::delete(TOKEN_KEY);
	}
	let mut token_lock = TOKEN.write();
	*token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
	let token_lock = TOKEN.read();
	token_lock.clone()
}

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
	B: Serialize + std::fmt::Debug,
{
	let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
	let url = format!("{}{}", API_ROOT, url);
	let mut builder = reqwest::Client::new()
		.request(method, url)
		.header("Content-Type", "application/json");
	if let Some(token) = get_token() {
		builder = builder.bearer_auth(token);
	}

	if allow_body {
		builder = builder.json(&body);
	}

	let response = builder.send().await;

	if let Ok(data) = response {
		if data.status().is_success() {
			let data: Result<T, _> = data.json::<T>().await;
			if let Ok(data) = data {
				Ok(data)
			} else {
				Err(Error::DeserializeError)
			}
		} else {
			match data.status().as_u16() {
				401 => Err(Error::Unauthorized),
				403 => Err(Error::Forbidden),
				404 => Err(Error::NotFound),
				500 => Err(Error::InternalServerError),
				_ => Err(Error::RequestError),
			}
		}
	} else {
		Err(Error::RequestError)
	}
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
{
	request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
{
	request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
	B: Serialize + std::fmt::Debug,
{
	request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
	B: Serialize + std::fmt::Debug,
{
	request(reqwest::Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn search(search: String, category: String) -> String {
	let mut query = "?".to_string();
	if search != "" {
		query = format!("{}&nama_kompetisi={}", query, search);
	}

	if category != "" {
		query = format!("{}&kategori_kompetisi={}", query, category);
	}

	query
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UploadResponse {
	url: String,
}

pub async fn upload_file(buffer: web_sys::Blob, file_name: String) -> Result<String, Error> {
	let formdata = web_sys::FormData::new().unwrap();
	match formdata.append_with_str("upload_preset", "h4oea9l0") {
		Ok(_) => {}
		Err(_) => {
			return Err(Error::RequestError);
		}
	}
	match formdata.append_with_blob_and_filename("file", &buffer, file_name.as_str()) {
		Ok(_) => {}
		Err(_) => {
			return Err(Error::RequestError);
		}
	}

	let mut opts = RequestInit::new();
	opts.method("POST");
	opts.mode(RequestMode::Cors);
	opts.body(Some(&formdata));

	let url = "https://api.cloudinary.com/v1_1/dncbtxucm/image/upload".to_string();

	let request = match Request::new_with_str_and_init(&url, &opts) {
		Ok(request) => request,
		Err(e) => {
			log::error!("Error creating request: {:?}", e);
			return Err(Error::RequestError);
		}
	};

	match request.headers().set(
		"Accept",
		"application/json, application/xml, text/plain, text/html, *.*",
	) {
		Ok(_) => {}
		Err(e) => {
			log::error!("Error setting header: {:?}", e);
			return Err(Error::RequestError);
		}
	}

	let window = web_sys::window().unwrap();
	let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await {
		Ok(resp) => resp,
		Err(e) => {
			log::error!("Error fetching: {:?}", e);
			return Err(Error::RequestError);
		}
	};

	// `resp_value` is a `Response` object.
	let resp: Response = resp_value.dyn_into().unwrap();

	// Convert this other `Promise` into a rust `Future`.
	let json = match JsFuture::from(resp.json().unwrap()).await {
		Ok(json) => json,
		Err(e) => {
			log::error!("Error parsing response: {:?}", e);
			return Err(Error::RequestError);
		}
	};

	// Use serde to parse the JSON into a struct.
	let response: UploadResponse = json.into_serde().unwrap();

	Ok(response.url)
}
