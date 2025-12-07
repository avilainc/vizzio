# Machine Learning Models

This directory contains pre-trained ML models for malware detection:

## Models

### malware_classifier.onnx
- **Type:** Random Forest / Neural Network
- **Purpose:** Binary classification (malware vs benign)
- **Accuracy:** ~95%
- **Features:** 258 dimensions (byte histogram, entropy, etc.)

### ransomware_detector.onnx
- **Type:** CNN
- **Purpose:** Ransomware family detection
- **Accuracy:** ~92%
- **Features:** Behavioral patterns, file operations

## Training

Models were trained on datasets including:
- VirusTotal samples
- EMBER dataset
- Custom collected samples

## Usage

```rust
use crate::ml::*;

let predictor = MalwarePredictor::new(
    Box::new(NeuralNetwork::load("models/malware_classifier.onnx")?)
);

let result = predictor.predict(&binary_data)?;
println!("Malware: {}, Confidence: {:.2}%",
    result.is_malware, result.confidence * 100.0);
```

## Updating Models

To retrain models:
```bash
cargo run --bin train_models -- --dataset ./training_data
```
