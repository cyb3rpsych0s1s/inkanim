use std::fmt;

use serde::de::{self, MapAccess};
use serde::Deserialize;

use crate::ink::{HDRColor, Range, Vector2};

pub fn deserialize_vector2_from_anything<'de, D>(deserializer: D) -> Result<Range, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct RangeVisitor;

    impl<'de> de::Visitor<'de> for RangeVisitor {
        type Value = Range;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("either a Vector2, or its simpler integer or float representation")
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Percent(v as f32))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Percent(v))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_f32(v as f32)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let (key, value) = &mut map.next_entry::<&str, &str>()?.unwrap();
            if key == &"$type" {
                if value == &"Vector2" {
                    return Ok(Range::Position(Vector2::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
                if value == &"HDRColor" {
                    return Ok(Range::Color(HDRColor::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
            }
            Err(de::Error::custom("unknown type"))
        }
    }

    deserializer.deserialize_any(RangeVisitor)
}
