

pub mod search;
pub mod template;
//pub mod cache;
// add cli
// add web interface
// add
//import dependencies
use search::core::Search::SearchQuery;
use serde_json::json;
use std::{
    env::{self},
};

use warp::Filter;

// main search filter is using Aho - Corasick Algorithm search.

#[tokio::main]
async fn main() {
    // read command line argument
    let args: Vec<String> = env::args().collect();

    /*
    This is the server initialization  using warp by seamonster
     */
    let main = warp::get().and(warp::path::end()).map(|| {
        template::Template::handlebar(
            "template_search",
            json!({}),
            template::Template::SEARCH_TEMPLATE.to_string(),
        )
    });

    let args_contain = warp::path!("args").map(move || warp::reply::json(&args));
    /*
    Search should have been using optimum ammount of cpu ,
    by collecting available cpu in system
    */
    let search = warp::path!("search")
        .and(warp::query().map(|s: search::core::Search::Search| s.search))
        .map(|query: String| {
            (
                query.clone(),
                SearchQuery::new(query).read_local("./".to_string(), "pdf".to_string()),
            )
        })
        .and_then(|(query, content)| async move {
            if true {
                Ok(template::Template::handlebar(
                    "template",
                    json!({"res" : json!(content),"query":query}),
                    template::Template::TEMPLATE.to_string(),
                ))
            } else {
                Err(warp::reject::not_found())
            }
        });
    let pub_file = warp::path!("pub").and(warp::fs::dir("./pub/"));

    //  let path = warp::path!("path").and(warp::path::end()).map(|| warp::reply::json(SearchQuery::read_local_type("./".to_string(),"./".to_string())));
    // first class api router with first slash
    let first_class = warp::get().and(main.or(args_contain).or(search));

    // serving server
    warp::serve(first_class).run(([127, 0, 0, 1], 8080)).await;
}

/* this is unit test */
#[cfg(test)]

mod tests {
    /*
    Things that should have tested .

    */
}
