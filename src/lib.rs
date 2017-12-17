// weather,location=us-midwest temperature=82 1465839830100400200
//   |    -------------------- --------------  |
//   |             |             |             |
//   |             |             |             |
// +-----------+--------+-+---------+-+---------+
// |measurement|,tag_set| |field_set| |timestamp|
// +-----------+--------+-+---------+-+---------+

pub struct DataPoint {
    measurement: String,
    tag_set: Vec<(String, String)>,
    field_set: Vec<(String, String)>,
    timestamp: u64
}

pub fn encode (dp: DataPoint) -> String {
    let tags: Vec<String> = dp.tag_set
        .iter()
        .map(|t| format!("{}={}", t.0, t.1))
        .collect();

    let fields: Vec<String> = dp.field_set
        .iter()
        .map(|t| format!("{}={}", t.0, t.1))
        .collect();

    format!("{},{} {} {}",
        dp.measurement,
        tags.join(","),
        fields.join(","),
        dp.timestamp
    )
}

// pub fn decode (dp: String) -> DataPoint {
//
// }


#[cfg(test)]
mod tests {
    use encode;
    use DataPoint;

    #[test]
    fn correct_encoding() {
        let dp = DataPoint {
            measurement: "weather".to_string(),
            tag_set: vec![
                ("location".to_string(), "us-midwest".to_string()),
                ("season".to_string(), "summer".to_string()),
            ],
            field_set: vec![("temperature".to_string(), "82".to_string())],
            timestamp: 1513541154228
        };

        assert_eq!(
            encode(dp),
            "weather,location=us-midwest,season=summer temperature=82 1513541154228".to_string()
        );
    }
}
