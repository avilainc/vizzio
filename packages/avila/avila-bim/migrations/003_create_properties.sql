-- Create properties table (normalized)
CREATE TABLE IF NOT EXISTS properties (
    id UUID PRIMARY KEY,
    element_id UUID NOT NULL REFERENCES elements(id) ON DELETE CASCADE,
    property_set VARCHAR(255) NOT NULL,
    property_name VARCHAR(255) NOT NULL,
    property_value TEXT NOT NULL,
    property_type VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_properties_element_id ON properties(element_id);
CREATE INDEX idx_properties_name ON properties(property_name);
CREATE INDEX idx_properties_pset ON properties(property_set);
