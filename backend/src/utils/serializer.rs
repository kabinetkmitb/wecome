use chrono::{DateTime, NaiveDateTime, Utc};

pub fn time_to_json(t: NaiveDateTime) -> String {
    DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
}

pub mod json_time {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&format!("{} 00:00:00", &time), "%d/%m/%Y %H:%M:%S")
            .map_err(serde::de::Error::custom)
    }
}

pub mod option_json_time {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        time: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if time.is_none() {
            false.serialize(serializer)
        } else {
            time_to_json(time.unwrap().clone()).serialize(serializer)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        let final_deserialize = match NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", &time),
            "%d/%m/%Y %H:%M:%S",
        ) {
            Ok(data) => data,
            Err(e) => return Err(serde::de::Error::custom(e)),
        };

        Ok(Some(final_deserialize))
    }
}

pub fn default_time() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
