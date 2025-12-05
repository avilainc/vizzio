// Unit tests for ML feature extraction
#[cfg(test)]
mod ml_tests {
    use crate::ml::*;

    #[test]
    fn test_feature_extractor() {
        let extractor = FeatureExtractor::new();
        let data = b"test binary data";

        let features = extractor.extract(data);
        assert!(features.is_ok());
        assert!(features.unwrap().len() > 0);
    }

    #[test]
    fn test_byte_histogram() {
        let extractor = FeatureExtractor::new();
        let data = vec![0u8; 100];

        let features = extractor.extract(&data).unwrap();
        assert_eq!(features.len(), 258); // 256 histogram + entropy + size
    }
}
