-- Create elements table
CREATE TABLE IF NOT EXISTS elements (
    id UUID PRIMARY KEY,
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE CASCADE,
    guid VARCHAR(22) NOT NULL,
    element_type VARCHAR(100) NOT NULL,
    name VARCHAR(255),
    properties JSONB,
    bounds_min DOUBLE PRECISION[3],
    bounds_max DOUBLE PRECISION[3],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(model_id, guid)
);

CREATE INDEX idx_elements_model_id ON elements(model_id);
CREATE INDEX idx_elements_guid ON elements(guid);
CREATE INDEX idx_elements_type ON elements(element_type);
CREATE INDEX idx_elements_properties ON elements USING GIN(properties);

-- Spatial index (PostGIS)
-- CREATE INDEX idx_elements_bounds ON elements USING GIST(bounds_min, bounds_max);
