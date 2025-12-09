//! Exportação de mapas

use super::{DbResult, DbError};
use super::database::CartographicDatabase;
use super::thematic::ThematicMap;

/// Formato de exportação
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// SVG (vetorial)
    Svg,

    /// JSON (dados estruturados)
    Json,

    /// GeoJSON (padrão geográfico)
    GeoJson,

    /// CSV (tabular)
    Csv,

    /// HTML (mapa interativo)
    Html,
}

/// Exportador de mapas
pub struct MapExporter;

impl MapExporter {
    /// Exporta mapa temático para SVG
    pub fn to_svg(
        _db: &CartographicDatabase,
        _map: &ThematicMap,
    ) -> DbResult<String> {
        // TODO: Implementar renderização SVG
        Ok(String::from("<?xml version=\"1.0\"?>\n<svg><!-- Mapa aqui --></svg>"))
    }

    /// Exporta mapa para JSON
    pub fn to_json(
        db: &CartographicDatabase,
        map: &ThematicMap,
    ) -> DbResult<String> {
        use serde_json::json;

        let mut features = Vec::new();

        for layer in &map.layers {
            for &id in &layer.entities {
                if let Some(entity) = db.get(id) {
                    let geo_entity = entity.as_geo_entity();
                    let loc = geo_entity.location();

                    features.push(json!({
                        "id": id,
                        "name": geo_entity.name(),
                        "type": geo_entity.entity_type(),
                        "location": {
                            "lat": loc.lat,
                            "lon": loc.lon,
                        },
                        "layer": layer.name,
                    }));
                }
            }
        }

        let output = json!({
            "type": "ThematicMap",
            "title": map.title,
            "description": map.description,
            "features": features,
            "layers": map.layers.len(),
        });

        serde_json::to_string_pretty(&output)
            .map_err(|e| DbError::SerializationError(e.to_string()))
    }

    /// Exporta para GeoJSON
    pub fn to_geojson(
        db: &CartographicDatabase,
        map: &ThematicMap,
    ) -> DbResult<String> {
        use serde_json::json;

        let mut features = Vec::new();

        for layer in &map.layers {
            for &id in &layer.entities {
                if let Some(entity) = db.get(id) {
                    let geo_entity = entity.as_geo_entity();
                    let loc = geo_entity.location();

                    features.push(json!({
                        "type": "Feature",
                        "geometry": {
                            "type": "Point",
                            "coordinates": [loc.lon, loc.lat]
                        },
                        "properties": {
                            "id": id,
                            "name": geo_entity.name(),
                            "entity_type": geo_entity.entity_type(),
                            "layer": layer.name,
                        }
                    }));
                }
            }
        }

        let output = json!({
            "type": "FeatureCollection",
            "name": map.title,
            "crs": {
                "type": "name",
                "properties": {
                    "name": "EPSG:4326"
                }
            },
            "features": features,
        });

        serde_json::to_string_pretty(&output)
            .map_err(|e| DbError::SerializationError(e.to_string()))
    }

    /// Exporta para CSV
    pub fn to_csv(
        db: &CartographicDatabase,
        map: &ThematicMap,
    ) -> DbResult<String> {
        let mut csv = String::from("id,name,type,latitude,longitude,layer\n");

        for layer in &map.layers {
            for &id in &layer.entities {
                if let Some(entity) = db.get(id) {
                    let geo_entity = entity.as_geo_entity();
                    let loc = geo_entity.location();

                    csv.push_str(&format!(
                        "{},{},{},{},{},{}\n",
                        id,
                        geo_entity.name(),
                        geo_entity.entity_type(),
                        loc.lat,
                        loc.lon,
                        layer.name,
                    ));
                }
            }
        }

        Ok(csv)
    }

    /// Exporta para HTML interativo (usando Leaflet.js)
    pub fn to_html(
        db: &CartographicDatabase,
        map: &ThematicMap,
    ) -> DbResult<String> {
        let geojson = Self::to_geojson(db, map)?;

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <style>
        body {{ margin: 0; padding: 0; }}
        #map {{ height: 100vh; width: 100%; }}
        .legend {{
            background: white;
            padding: 10px;
            border-radius: 5px;
            box-shadow: 0 0 15px rgba(0,0,0,0.2);
        }}
    </style>
</head>
<body>
    <div id="map"></div>
    <script>
        var map = L.map('map').setView([0, 0], 2);

        L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            attribution: '&copy; OpenStreetMap contributors'
        }}).addTo(map);

        var geojsonData = {};

        var geojsonLayer = L.geoJSON(geojsonData, {{
            pointToLayer: function(feature, latlng) {{
                return L.circleMarker(latlng, {{
                    radius: 8,
                    fillColor: "#ff7800",
                    color: "#000",
                    weight: 1,
                    opacity: 1,
                    fillOpacity: 0.8
                }});
            }},
            onEachFeature: function(feature, layer) {{
                if (feature.properties) {{
                    var popupContent = '<h3>' + feature.properties.name + '</h3>';
                    popupContent += '<p>Tipo: ' + feature.properties.entity_type + '</p>';
                    popupContent += '<p>Camada: ' + feature.properties.layer + '</p>';
                    layer.bindPopup(popupContent);
                }}
            }}
        }}).addTo(map);

        map.fitBounds(geojsonLayer.getBounds());
    </script>
</body>
</html>"#,
            map.title,
            geojson,
        );

        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aviladb_cartographic::{
        database::{CartographicDatabase, DatabaseConfig},
        entities::Company,
        thematic::ThematicMapBuilder,
    };
    use crate::cartography::LatLon;

    #[test]
    fn test_json_export() {
        let mut db = CartographicDatabase::new(DatabaseConfig {
            auto_save: false,
            ..Default::default()
        });

        let company = Company::new(
            0,
            "Test Corp".to_string(),
            LatLon::new(-23.55, -46.63),
            "Rua Teste".to_string(),
            "Tech".to_string(),
        );

        db.add_company(company).unwrap();

        let map = ThematicMapBuilder::new(&db, "Mapa Teste".to_string())
            .add_companies_layer("Empresas".to_string())
            .build();

        let json = MapExporter::to_json(&db, &map).unwrap();
        assert!(json.contains("Test Corp"));
    }
}
