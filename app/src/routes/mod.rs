use pavex::blueprint::{router::POST, Blueprint};
use pavex::f;

pub mod tts;

pub fn register(bp: &mut Blueprint) {
    bp.route(POST, "/api/v1/generate", f!(crate::routes::tts::TtsContext::generate));
}
