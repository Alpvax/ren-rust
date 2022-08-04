use serde::ser::SerializeSeq;
use serde_json::{json, Map, Value};

#[derive(Debug)]
pub struct TaggedMeta<'a>(&'static str, Option<&'a Meta>);
impl<'a> TaggedMeta<'a> {
    pub fn new(tag: &'static str) -> Self {
        Self(tag, None)
    }
    pub fn new_with_meta(tag: &'static str, meta: &'a Meta) -> Self {
        Self(tag, Some(meta))
    }
}
impl<'a> serde::Serialize for TaggedMeta<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = if let Some(meta) = self.1 {
            match json!(*meta) {
                Value::Object(m) => m,
                m => panic!("Expected meta to be an object, recieved: {:?}", m),
            }
        } else {
            Map::new()
        };
        map.insert("$".to_string(), self.0.into());
        map.serialize(serializer)
    }
}

pub(crate) fn serialise_tagged_seq<S>(
    serialiser: S,
    tag: &str,
    meta: Option<Map<String, Value>>,
    data_len: Option<usize>,
) -> Result<S::SerializeSeq, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serialiser.serialize_seq(data_len.map(|l| l + 1))?;
    seq.serialize_element(&if let Some(mut map) = meta {
        map.insert("$".to_owned(), tag.into());
        map.into()
    } else {
        json!({ "$": tag })
    })?;
    Ok(seq)
}

macro_rules! serialise_tagged {
    ($serialiser:expr, $tag:literal, [$($meta_key:expr => $meta_val:expr),* $(,)?], [$($data_item:expr),* $(,)?]) => {
        serialise_tagged!(@count $serialiser, $tag, [$($meta_key => $meta_val),*], []: 0 => [$($data_item),*])
    };
    ($serialiser:expr, $tag:literal, [$($meta_key:expr => $meta_val:expr),*], $items:expr) => {{
        use serde::ser::SerializeSeq;
        let mut seq = $serialiser.serialize_seq(Some($items.len()))?;
        seq.serialize_element(&serde_json::json!({
            "$": $tag,
            $($meta_key: $meta_val),*
        }))?;
        for item in $items {
            seq.serialize_element(item)?;
        }
        seq.end()
    }};
    (@count $serialiser:expr, $tag:literal, [$($meta_key:expr => $meta_val:expr),*], [$($data_item_counted:expr),*]: $i:expr => [$data_item:expr $(, $rest:expr)*]) => {
        serialise_tagged!(@count $serialiser, $tag, [$($meta_key => $meta_val),*], [$($data_item_counted,)* $data_item]: $i + 1 => [$($rest),*])
    };
    (@count $serialiser:expr, $tag:literal, [$($meta_key:expr => $meta_val:expr),*], [$($data_item:expr),*]: $i:expr => []) => {{
        use serde::ser::SerializeSeq;
        let mut seq = $serialiser.serialize_seq(Some($i))?;
        seq.serialize_element(&serde_json::json!({
            "$": $tag,
            $($meta_key: $meta_val),*
        }))?;
        $(
            seq.serialize_element($data_item)?;
        )*
        seq.end()
    }};
}

pub(crate) use serialise_tagged;

use crate::expr::Meta;
