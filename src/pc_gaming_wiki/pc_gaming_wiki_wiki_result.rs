// use serde::{Deserialize, Serialize};
// use serde_json::Value;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PcGameWikiWikiResult {
//     pub parse: Parse,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Parse {
//     pub title: String,
//     #[serde(rename = "pageid")]
//     pub page_id: i64,
//     pub redirects: Vec<Value>,
//     pub wikitext: Wikitext,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Wikitext {
//     #[serde(rename = "*")]
//     pub field: String,
// }
