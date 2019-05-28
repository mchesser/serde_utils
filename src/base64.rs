pub mod bytes {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        <&str>::deserialize(deserializer).and_then(|value| {
            base64::decode(&value).map_err(|err| de::Error::custom(err.to_string()))
        })
    }

    pub fn serialize<T: AsRef<[u8]>, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::encode(value.as_ref()))
    }
}

pub mod f32 {
    use byteorder::{ByteOrder, LittleEndian};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<f32>, D::Error> {
        let bytes = <&str>::deserialize(deserializer).and_then(|value| {
            base64::decode(&value).map_err(|err| de::Error::custom(err.to_string()))
        })?;

        let mut result = vec![0.0; bytes.len() / 4];
        LittleEndian::read_f32_into(&bytes, &mut result);
        Ok(result)
    }

    pub fn serialize<T: AsRef<[f32]>, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut buffer = vec![0; 4 * value.as_ref().len()];
        LittleEndian::write_f32_into(value.as_ref(), &mut buffer);
        serializer.serialize_str(&base64::encode(&buffer))
    }
}
