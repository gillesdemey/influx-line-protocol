// weather,location=us-midwest temperature=82 1465839830100400200
//   |    -------------------- --------------  |
//   |             |             |             |
//   |             |             |             |
// +-----------+--------+-+---------+-+---------+
// |measurement|,tag_set| |field_set| |timestamp|
// +-----------+--------+-+---------+-+---------+

use std::fmt;

pub struct DataPoint<T: fmt::Display> {
    measurement: String,
    tag_set: Vec<(String, String)>,
    field_set: Vec<(String, T)>,
    timestamp: u64
}

impl<T> DataPoint<T> where T: fmt::Display {
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

    fn from (s: &str) -> DataPoint<T> {
        DataPoint {
            measurement: String::from("weather"),
            tag_set: vec![
                (String::from("location"), String::from("us-midwest")),
                (String::from("season"), String::from("summer")),
            ],
            field_set: vec![
                (String::from("temperature"), 82)
            ],
            timestamp: 1465839830100400200
        }
    }
}

#[cfg(test)]
mod tests {
    use DataPoint;

    #[test]
    fn correct_encoding() {
        let dp = DataPoint {
            measurement: String::from("weather"),
            tag_set: vec![
                (String::from("location"), String::from("us-midwest")),
                (String::from("season"), String::from("summer")),
            ],
            field_set: vec![(String::from("temperature"), 82)],
            timestamp: 1465839830100400200
        };

        let line = String::from("weather,location=us-midwest,season=summer temperature=82 1465839830100400200");

        assert_eq!(dp.encode(), line);
    }

    fn correct_decoding() {
        let line = "weather,location=us-midwest,season=summer temperature=82 1465839830100400200";

        let dp = DataPoint {
            measurement: String::from("weather"),
            tag_set: vec![
                (String::from("location"), String::from("us-midwest")),
                (String::from("season"), String::from("summer")),
            ],
            field_set: vec![(String::from("temperature"), 82)],
            timestamp: 1465839830100400200
        };

        assert_eq!(DataPoint::from(line).measurement, String::from("weather"));
    }
}
