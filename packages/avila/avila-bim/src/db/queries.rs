//! Optimized database queries

/// Query para buscar elementos com propriedades específicas
pub const FIND_ELEMENTS_WITH_PROPERTY: &str = r#"
SELECT e.*
FROM elements e
WHERE e.model_id = $1
  AND e.properties @> $2::jsonb
ORDER BY e.element_type, e.name
"#;

/// Query para estatísticas de modelo
pub const MODEL_STATISTICS: &str = r#"
SELECT
    element_type,
    COUNT(*) as count,
    AVG((bounds_max[1] - bounds_min[1]) *
        (bounds_max[2] - bounds_min[2]) *
        (bounds_max[3] - bounds_min[3])) as avg_volume
FROM elements
WHERE model_id = $1
GROUP BY element_type
ORDER BY count DESC
"#;

/// Query para bounding box de modelo completo
pub const MODEL_BOUNDS: &str = r#"
SELECT
    ARRAY[MIN(bounds_min[1]), MIN(bounds_min[2]), MIN(bounds_min[3])] as min,
    ARRAY[MAX(bounds_max[1]), MAX(bounds_max[2]), MAX(bounds_max[3])] as max
FROM elements
WHERE model_id = $1
"#;
