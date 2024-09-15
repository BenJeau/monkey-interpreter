use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Serialize)]
struct Entry<K: Serialize, V: Serialize> {
    key: K,
    value: V,
}

pub fn serialize<S, K, V>(map: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize + Ord,
    V: Serialize,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;
    for (key, value) in map {
        seq.serialize_element(&Entry { key, value })?;
    }
    seq.end()
}
