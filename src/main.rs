pub mod template;

// add cli
// add web interface
// add
//import dependencies
use pdf::file::File;
use std::{
    env::{self, args},
    fs,
    time::SystemTime,
};
use warp::Filter;
use crate::SearchDirectory::Local;

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
    Web {
        search_engine: String,
    },
    Local {
        path: String,
    },
    GoogleDrive {
        api_string: String,
        username: String,
        folder_name: String,
        shared: bool,
    },
    Gmail {
        api_string: String,
        username: String,
    },
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

impl SearchQuery {
    /*
        Method to create new search Query   
    */

    pub fn new(search : String) -> SearchQuery{
        SearchQuery { search_query: search, file_type: vec!(), scope: Scope { search_directory: vec!(Local{path : "".to_owned()}) } }
    }

    /*
    Function to read file name in folder and return file name related to filter
    */

    pub fn read_folder(path: String) -> Vec<String> {
        let env_path = args().nth(1).expect("no path given");
        let paths = fs::read_dir(env_path).unwrap();

        let files_name: Vec<String> = paths
            .into_iter()
            .map(|f| format!("{}", f.unwrap().path().display()))
            .collect();
        files_name
    }

    /*
        function to read pdf and search keyword from into inner content
        returning result success as vector of str and if err returning Pdferror
    */

    pub fn read_pdf(self) -> String {
        let path = args().nth(1).expect("no file given");
        
        println!("read: {}", path);

        let now = SystemTime::now();

        let file = File::<Vec<u8>>::open(&path).unwrap();

        let mut usize_vec: Vec<usize> = vec![];

        let mut res: Vec<String> = vec![];

        for page in file.pages() {
            let page = page.unwrap();
            if let Some(ref c) = page.contents {
                if c.to_string().contains(&self.search_query) {
                    let item_position = c.operations.iter().position(|x| {
                        x.operator == "Tj"
                            && x.operands.iter().any(|y| {
                                y.as_string().is_ok() && format!("{}", y).contains(&self.search_query)
                            })
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
                                    .filter_map(|s| {
                                        s.operands.iter().find(|y| y.as_string().is_ok())
                                    })
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
}

impl Default for SearchQuery {
    fn default() -> SearchQuery{
        SearchQuery { search_query: "".to_string(), file_type: vec!(), scope: Scope { search_directory: vec!(Local{path : "".to_owned()}) } }
    }
}


#[tokio::main]
async fn main() {
    // read command line argument
    let args: Vec<String> = env::args().collect();

    /*This is the server initialization  using warp by seamonster
     */
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let args_contain = warp::path!("args").map(move || warp::reply::json(&args));

    let search = warp::path!("search" / String).map(|query|SearchQuery::new(query).read_pdf());

    let serve_search = warp::path!("search_serve" / String).map(|query|SearchQuery::new(query).read_pdf());

    // first class api router with first slash
    let first_class = warp::get().and(hello.or(args_contain).or(search));

    // serving server
    warp::serve(first_class).run(([127, 0, 0, 1], 3030)).await;
}
