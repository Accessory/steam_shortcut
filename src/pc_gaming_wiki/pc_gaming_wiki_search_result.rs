// use serde::{Deserialize, Serialize};

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PcGameWikiSearchResult {
//     pub warnings: Option<Warnings>,
//     pub batchcomplete: String,
//     pub query: Query,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Warnings {
//     pub main: Main,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Main {
//     #[serde(rename = "*")]
//     pub field: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Query {
//     pub searchinfo: Searchinfo,
//     pub search: Vec<Search>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Searchinfo {
//     pub totalhits: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Search {
//     pub ns: i64,
//     pub title: String,
//     pub pageid: i64,
//     pub size: i64,
//     pub wordcount: i64,
//     pub snippet: String,
//     pub timestamp: String,
// }
