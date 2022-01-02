pub mod Search {
    use crate::search::core::Search::SearchDirectory::Local;
    use crate::search::excel::excel::read_excel;
    use aho_corasick::AhoCorasickBuilder;
    use lopdf::{Document, Error};
    use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IntoParallelRefIterator};
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::{self, PathBuf};
    use std::{fs, str::Matches, time::SystemTime};
    // main search filter is using Aho - Corasick Algorithm search.

    pub struct FileResult {
        pub file_name: String,
        pub file_path: String,
    }

    /*
        Search Query Implementation for API from html
    */
    #[derive(Clone)]
    pub struct SearchQuery {
        search_query: String,     // Search Query is a string contained search query
        file_type: Vec<FileType>, // array contain multiple item from FileType or nothing on array,
        scope: Scope,             // Search directory should be containing web , local ,
    }

    impl Clone for FileType {
        fn clone(&self) -> Self {
            match self {
                Self::pdf => Self::pdf,
                Self::html => Self::html,
                Self::txt => Self::txt,
                Self::xls => Self::xls,
                Self::docx => Self::docx,
                Self::csv => Self::csv,
            }
        }

        fn clone_from(&mut self, source: &Self) {
            match self {
                Self::pdf => Self::pdf,
                _ => Self::pdf,
            };
        }
    }

    #[derive(Clone)]
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
    #[derive(Clone)]
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

        pub fn new(search: String) -> SearchQuery {
            SearchQuery {
                search_query: search,
                file_type: vec![],
                scope: Scope {
                    search_directory: vec![Local {
                        path: "".to_owned(),
                    }],
                },
            }
        }

        pub fn read_local(self, path: String, types: String) -> Vec<(String, String)> {
            let file_names = SearchQuery::read_local_type(path);
            let key = self.search_query.as_str();
            
            file_names
                .par_iter()
                .map(|x| match x {
                    p if p.contains(".pdf") => (x.clone() ,SearchQuery::search_pdf_v2(key,x.to_owned()).concat()) ,
                    e if e.contains(".xlsx") => (x.clone() ,read_excel(x.to_owned())),
                    _ => (x.clone(),String::from("not readable"))
                }).collect()
        }

        /*
        Function to read file name in folder and return file name related to filter
        */

        pub fn read_local_type(path: String) -> Vec<String> {
            //returning as HashMap since Hashmap is move effective for handling data 15>
            let paths = fs::read_dir(path).unwrap();
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

        fn search_pdf_v2(pattern: &str, file_path: String) -> Vec<String> {
            let pattern = &[pattern];

            let mut res: Vec<String> = vec![];

            let haystack = SearchQuery::read_pdf(&file_path);
        
            println!("{}",haystack);
            let ac = AhoCorasickBuilder::new()
                .ascii_case_insensitive(true)
                .build(pattern);

            let mut matches = vec![];
            for mat in ac.find_iter(&haystack) {
                matches.push((mat.pattern(), mat.start(), mat.end()));
            }
            for string in matches {
                let max_len = haystack.capacity();
                let max_len_usize = if max_len < string.2 + 100 {
                    max_len
                }  else{
                    string.2 + 100 
                };
                res.push(haystack[string.1 .. max_len_usize].to_string());
            }
            return res;
        }

        fn read_pdf(path: &str)->String{
            let docs = Document::load(path).unwrap();
            let pages = docs.get_pages();
            let pages_num = pages.iter().last().unwrap().0;
            (1..*pages_num).collect::<Vec<u32>>().par_iter().map(
                |i|docs.extract_text(&[*i]).unwrap()
            ).collect()
        }
        /*
        used to read excel using  calamine
        */
        pub fn read_excel(self, file_path : String) -> Vec<String> {
            vec![]
        }
    }

    impl Default for SearchQuery {
        fn default() -> SearchQuery {
            SearchQuery {
                search_query: "".to_string(),
                file_type: vec![],
                scope: Scope {
                    search_directory: vec![Local {
                        path: "".to_owned(),
                    }],
                },
            }
        }
    }

    #[derive(Deserialize)]
    pub struct Search {
        pub search: String,
    }

    /* this is unit test */
    #[cfg(test)]

    mod tests {
        /*
        Things that should have tested .

        */
    }
}
