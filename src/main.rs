// add cli
// add web interface
// add
//import dependencies
use pdf::file::File;
use std::{
    env::{self, args},
    time::SystemTime,
};
use warp::Filter;

// main search filter is using Aho - Corasick Algorithm search.

pub struct FileResult {
    pub file_name: String,
    pub file_path: String,
}

/*
    Search Query Implementation for API from html
*/
pub struct SearchQuery {
    pub search_query: String, // Search Query is a string contained search query
    pub file_type: Vec<FileType>, // array contain multiple item from FileType or nothing on array,
    pub scope: Scope,         // Search directory should be containing web , local ,
}

pub struct Scope {
    pub search_directory: Vec<SearchDirectory>,
}

impl Default for Scope {
    fn default() -> Scope {
        Scope {
            search_directory: vec![],
        }
    }
}

/*
    Search directory should be containing the scope of search
*/
pub enum SearchDirectory {
    web,
    local,
    google_drive,
    gmail,
}

/*
    Filetype contain name of file type can be selected
*/
pub enum FileType {
    pdf,
    html,
    txt,
    xls,
    docx,
    csv,
}

/* this is for enum containing arguments for cli
        During planning this include ,
        1. Setup
        2. Run
        3. Run Indexing on local   .
*/

pub enum NamedArgs {
    Setup,
    Run,
    Index,
}

#[tokio::main]
async fn main() {
    // read command line argument
    let args: Vec<String> = env::args().collect();

    /*This is the server initialization  using warp by seamonster
     */
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));


    let args_contain = warp::path!("args").map(move || warp::reply::json(&args));

    let search = warp::path!("search" / String).map(read_pdf);

    // first class api router with first slash
    let first_class = warp::get().and(hello.or(args_contain).or(search));

    // serving server
    warp::serve(first_class).run(([127, 0, 0, 1], 3030)).await;
}
/*
    function to read pdf and search keyword from into inner content
    returning result success as vector of str and if err returning Pdferror
*/

fn read_pdf(keywords: String) -> String {
    let path = args().nth(1).expect("no file given");
    println!("read: {}", path);

    let now = SystemTime::now();

    let file = File::<Vec<u8>>::open(&path).unwrap();

    let mut usize_vec: Vec<usize> = vec![];

    let mut res: Vec<String> = vec![];

    for page in file.pages() {
        let page = page.unwrap();
        if let Some(ref c) = page.contents {
            if c.to_string().contains(&keywords) {
                let item_position = c.operations.iter().position(|x| {
                    x.operator == "Tj"
                        && x.operands
                            .iter()
                            .any(|y| y.as_string().is_ok() && format!("{}", y).contains(&keywords))
                });

                if let Some(position) = item_position {
                    usize_vec.push(position)
                }
                res.push(
                    usize_vec
                        .iter()
                        .map(|pp| {
                            c.operations[pp - 2..=pp + 40]
                                .iter()
                                .filter_map(|s| s.operands.iter().find(|y| y.as_string().is_ok()))
                                .map(|p| {
                                    p.clone()
                                        .as_string()
                                        .unwrap()
                                        .clone()
                                        .into_string()
                                        .unwrap_or_else(|_| "".to_string())
                                })
                                .collect::<Vec<String>>()
                                .join(" ")
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                )
            }
        }
    }
    let then = now.elapsed();
    return res
        .iter()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
        .join("\n")
        + &format!(" \n Elapsed : {}", &then.unwrap().as_millis());
}
