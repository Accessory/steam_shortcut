// use crate::pc_gaming_wiki::pc_gaming_wiki_search_result::PcGameWikiSearchResult;
// use crate::pc_gaming_wiki::pc_gaming_wiki_wiki_result::PcGameWikiWikiResult;

// const PC_GAMING_WIKI_SEARCH_URL_PREFIX: &str =
//     "https://www.pcgamingwiki.com/w/api.php?action=query&list=search&srsearch=";
// const PC_GAMING_WIKI_SEARCH_URL_SUFFIX: &str = "&format=json";
// const PC_GAMING_WIKI_WIKI_URL_PREFIX: &str =
//     "https://www.pcgamingwiki.com/w/api.php?action=parse&format=json&pageid=";
// const PC_GAMING_WIKI_WIKI_URL_SUFFIX: &str = "&redirects=true&prop=wikitext";

// pub(crate) fn get_steam_id(game_title: &str) -> Option<u32> {
//     let page_id = get_page_id(game_title)?;
//     let url = format!("{PC_GAMING_WIKI_WIKI_URL_PREFIX}{page_id}{PC_GAMING_WIKI_WIKI_URL_SUFFIX}");
//     let wiki_result: PcGameWikiWikiResult = ureq::get(&url).call().ok()?.into_json().ok()?;
//
//     let start = wiki_result.parse.wikitext.field.find("steam appid  = ")? + 15;
//     let end = wiki_result.parse.wikitext.field[start..].find('\n')?;
//     let rtn = wiki_result.parse.wikitext.field[start..start + end].to_string();
//
//     rtn.parse().ok()
// }

// pub(crate) fn get_page_id(game_title: &str) -> Option<i64> {
//     let url = format!(
//         "{PC_GAMING_WIKI_SEARCH_URL_PREFIX}{}{PC_GAMING_WIKI_SEARCH_URL_SUFFIX}",
//         urlencoding::encode(&game_title.replace("-", ""))
//     );
//     println!("Url: {url}");
//     let search_result: PcGameWikiSearchResult = ureq::get(&url).call().ok()?.into_json().ok()?;
//     Some(search_result.query.search.first()?.pageid)
// }
