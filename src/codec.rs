use std::fmt;

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
    format!("{}, {} {} {}",
        dp.measurement,
        dp.tag_set.join(','),
        dp.field_set.join(','),
        dp.timestamp
    )
}

pub fn decode (dp: String) -> DataPoint {

}
