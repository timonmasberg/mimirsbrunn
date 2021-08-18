pub fn build_reverse_query(distance: &str, lat: f64, lon: f64) -> serde_json::Value {
    let json_lit = format!(
        r#"{{
            "query": {{
                "bool": {{
                    "must": {{
                        "match_all": {{}}
                    }},
                    "filter": {{
                        "geo_distance": {{
                            "distance": "{distance}",
                            "coord": {{
                                "lat": {lat},
                                "lon": {lon}
                            }}
                        }}
                    }}
                }}
            }},
            "sort": [
                {{
                    "_geo_distance": {{
                        "coord": {{
                            "lat": {lat},
                            "lon": {lon}
                        }},
                        "order": "asc",
                        "unit": "m",
                        "mode": "min",
                        "distance_type": "arc",
                        "ignore_unmapped": true
                    }}
                }}
            ]
        }}"#,
        distance = distance,
        lat = lat,
        lon = lon
    );

    serde_json::from_str(&json_lit).unwrap()
}
