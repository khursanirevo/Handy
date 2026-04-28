use anyhow::Result;
use log::debug;
use std::path::Path;
use sherpa_rs::transducer::{TransducerRecognizer, TransducerConfig};

pub struct SherpaOnnxEngine {
    recognizer: TransducerRecognizer,
}

impl SherpaOnnxEngine {
    pub fn new() -> Self {
        // Placeholder for now, actual initialization will happen in load_model
        // This is to match the interface of other engines.
        unimplemented!("SherpaOnnxEngine::new should not be called directly. Use load_model instead.");
    }

    pub fn load_model(model_path: &Path) -> Result<Self> {
        debug!("Loading SherpaOnnx model from: {:?}", model_path);

        let encoder_path = model_path.join("encoder-epoch-19-avg-1.onnx");
        let decoder_path = model_path.join("decoder-epoch-19-avg-1.onnx");
        let joiner_path = model_path.join("joiner-epoch-19-avg-1.onnx");
        let tokens_path = model_path.join("tokens.txt"); // Assuming tokens.txt is directly in the model directory

        // Check if all required files exist
        if !encoder_path.exists() {
            return Err(anyhow::anyhow!("Encoder model not found at {:?}", encoder_path));
        }
        if !decoder_path.exists() {
            return Err(anyhow::anyhow!("Decoder model not found at {:?}", decoder_path));
        }
        if !joiner_path.exists() {
            return Err(anyhow::anyhow!("Joiner model not found at {:?}", joiner_path));
        }
        if !tokens_path.exists() {
            return Err(anyhow::anyhow!("Tokens file not found at {:?}", tokens_path));
        }

        let config = TransducerConfig {
            encoder: encoder_path.to_str().unwrap().to_string(),
            decoder: decoder_path.to_str().unwrap().to_string(),
            joiner: joiner_path.to_str().unwrap().to_string(),
            tokens: tokens_path.to_str().unwrap().to_string(),
            num_threads: 2, // TODO: Make configurable
            sample_rate: 16000,
            feature_dim: 80,
            decoding_method: "greedy_search".to_string(), // TODO: Make configurable
            hotwords_file: "".to_string(),
            hotwords_score: 1.5,
            modeling_unit: "".to_string(),
            bpe_vocab: "".to_string(),
            blank_penalty: 0.0,
            model_type: "transducer".to_string(),
            debug: false,
            provider: Some("cpu".to_string()), // TODO: Make configurable (e.g., cuda, directml)
        };

        let recognizer = TransducerRecognizer::new(config)
            .map_err(|e| anyhow::anyhow!("Failed to create SherpaOnnx recognizer: {}", e))?;
        debug!("SherpaOnnx model loaded successfully.");

        Ok(Self { recognizer })
    }

    pub fn transcribe_samples(&mut self, samples: Vec<f32>) -> Result<String> {
        debug!("Transcribing audio samples with SherpaOnnx.");
        let text = self.recognizer.transcribe(16000, samples.as_slice());
        debug!("SherpaOnnx transcription result: {}", text);
        Ok(text)
    }
}
