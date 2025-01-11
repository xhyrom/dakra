use stt::config;

#[tokio::main]
async fn main() {
    let model_path = std::env::args()
        .nth(1)
        .expect("Please specify path to model");
    let wav_path = std::env::args()
        .nth(2)
        .expect("Please specify path to wav file");

    let provider = stt::create_provider(&config::SpeechToTextConfig {
        model_path: Some(model_path),
        worker_url: None,
        api_key: None,
    });

    let wav = std::fs::read(wav_path).expect("Failed to read wav file");
    let text = provider.transcribe(wav).await.unwrap();

    println!("{}", text);
}
