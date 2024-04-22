use crate::helpers::TestApi;
use pavex::http::StatusCode;
use reqwest::header::CONTENT_TYPE;

#[tokio::test]
async fn tts_works() {
    let api = TestApi::spawn().await;

    let response = api.tts_generate("hello!").await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "audio/wav");
}
