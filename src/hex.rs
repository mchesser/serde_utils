pub mod u8x6 {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 6], D::Error> {
        let parse_hex = |value: &str| {
            if value.len() != 12 {
                return Err(de::Error::invalid_length(value.len(), &"12"));
            }

            let d = |range| {
                u8::from_str_radix(&value[range], 16)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(value), &"hex digit"))
            };
            Ok([d(0..2)?, d(2..4)?, d(4..6)?, d(6..8)?, d(8..10)?, d(10..12)?])
        };
        <&str>::deserialize(deserializer).and_then(|value| parse_hex(value))
    }

    pub fn serialize<S: Serializer>(value: &[u8; 6], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode(&value.as_ref()))
    }
}

pub mod bytes {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        <&str>::deserialize(deserializer)
            .and_then(|value| hex::decode(&value).map_err(|err| de::Error::custom(err.to_string())))
    }

    pub fn serialize<T: AsRef<[u8]>, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode(&value.as_ref()))
    }
}
