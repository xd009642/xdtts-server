use xd_tts::{XdTts, WAV_SPEC};
use anyhow::Context;
use hound::WavWriter;
use std::path::Path;
use std::io::Cursor;
use std::sync::Arc;
use tracing::{error, instrument};
use pavex::response::body::raw::Full;
use serde::{Serialize, Deserialize};
use pavex::response::Response;
use pavex::request::body::JsonBody;
use pavex::http::{HeaderName, HeaderValue};

#[derive(Clone)]
pub struct TtsContext {
    tts: Arc<XdTts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generate {
    text: String,
}

impl TtsContext {
    pub fn new() -> Self {
        Self {
            tts: XdTts::new(&Path::new("./models"), false).unwrap().into(),
        }
    }

    /// Generate audio from a TTS request
    pub async fn generate(&self, request: &JsonBody<Generate>) -> Response {
        let model = self.tts.clone();
        
        let trans = request.0.text.clone();
        let res: anyhow::Result<Vec<u8>> = tokio::task::spawn_blocking(move || {
            let mut audio = Vec::new();
            let mut audio_cursor = Cursor::new(&mut audio);

            let mut writer = WavWriter::new(&mut audio_cursor, WAV_SPEC).context("write wav header")?;

            model.generate_audio(&trans, &mut writer, None).context("TTS model failed")?;

            writer.finalize().context("Finish encoding wav data")?;

            Ok(audio)
        }).await.unwrap();

        match res {
            Ok(audio) => {
                Response::ok()
                    .set_raw_body(Full::new(audio.into()))
                    .append_header(HeaderName::from_static("content-type"), HeaderValue::from_static("audio/wav"))
            },
            Err(e) => {
                error!(error=%e);
                todo!()
            }
        }
    }

}
