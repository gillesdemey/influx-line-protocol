// weather,location=us-midwest temperature=82 1465839830100400200
//   |    -------------------- --------------  |
//   |             |             |             |
//   |             |             |             |
// +-----------+--------+-+---------+-+---------+
// |measurement|,tag_set| |field_set| |timestamp|
// +-----------+--------+-+---------+-+---------+

// DataPoint
struct Tag(String, String);
struct Field(String, String);

pub struct DataPoint {
    measurement: String,
    tag_set: Vec<Tag>,
    field_set: Vec<Field>,
    timestamp: u32
}

pub fn encode (dp: DataPoint) -> String {
    let serializedTags: String = dp.tag_set
        .iter()
        .map(|t| format!("{}={}", t.0, t.1))
        .collect()
        .join(',');

    let serializedFields: String = dp.field_set
        .iter()
        .map(|t| format!("{}={}", t.0, t.1))
        .collect()
        .join(',');

    format!("{}, {} {} {}",
        dp.measurement,
        serializedTags,
        serializedFields,
        dp.timestamp
    )
}

// pub fn decode (dp: String) -> DataPoint {
//
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
