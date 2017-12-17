// weather,location=us-midwest temperature=82 1465839830100400200
//   |    -------------------- --------------  |
//   |             |             |             |
//   |             |             |             |
// +-----------+--------+-+---------+-+---------+
// |measurement|,tag_set| |field_set| |timestamp|
// +-----------+--------+-+---------+-+---------+

use std::fmt;

pub struct DataPoint<T> {
    measurement: String,
    tag_set: Vec<(String, String)>,
    field_set: Vec<(String, T)>,
    timestamp: u64
}

impl<T: fmt::Display> DataPoint<T> {
    pub fn encode (&self) -> String {
        let tags: Vec<String> = self.tag_set
            .iter()
            .map(|t| format!("{}={}", t.0, t.1))
            .collect();

        let fields: Vec<String> = self.field_set
            .iter()
            .map(|t| format!("{}={}", t.0, t.1))
            .collect();

        format!("{},{} {} {}",
            self.measurement,
            tags.join(","),
            fields.join(","),
            self.timestamp
        )
    }
}


// pub fn decode (dp: String) -> DataPoint {
//
// }


#[cfg(test)]
mod tests {
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
            timestamp: 1465839830100400200
        };

        assert_eq!(
            dp.encode(),
            "weather,location=us-midwest,season=summer temperature=82 1465839830100400200".to_string()
        );
    }
}
