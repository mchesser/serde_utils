pub mod bytes {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::borrow::{Borrow, Cow};

    pub fn deserialize<'de, D: Deserializer<'de>, T: From<Vec<u8>>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        let text: Cow<str> = Deserialize::deserialize(deserializer)?;
        let text_str: &str = text.borrow();

        base64::decode(text_str)
            .map_err(|err| de::Error::custom(err.to_string()))
            .map(|vec| From::from(vec))
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
    use std::borrow::{Borrow, Cow};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<f32>, D::Error> {
        let text: Cow<str> = Deserialize::deserialize(deserializer)?;
        let text_str: &str = text.borrow();

        let bytes = base64::decode(text_str).map_err(|err| de::Error::custom(err.to_string()))?;

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
