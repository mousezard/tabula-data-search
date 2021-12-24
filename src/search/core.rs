pub mod Search {
    use std::{fs, time::SystemTime};
    use pdf::file::File;
    use pdf::primitive::PdfString;
    use serde::Deserialize;
    use crate::search::core::Search::SearchDirectory::Local;
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
        search_query: String, // Search Query is a string contained search query
        file_type: Vec<FileType>, // array contain multiple item from FileType or nothing on array,
        scope: Scope,         // Search directory should be containing web , local ,
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
                _ => Self::pdf
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
    
        pub fn read_local(self,path: String,types : String) ->Vec<(String,String)> {
            let file_names = SearchQuery::read_local_type(path,types);
            file_names.iter().map( |x|(x.clone(),self.clone().read_pdf(x.to_owned()).concat())).collect()
        }
    
    
        /*
        Function to read file name in folder and return file name related to filter
        */
    
        pub fn read_local_type(path: String,types : String) -> Vec<String> {
            //returning as HashMap since Hashmap is move effective for handling data 15>
            let paths = fs::read_dir(path).unwrap();
    
            let types = format!(".{}", &types);
            let files_name: Vec<String> = paths
                .into_iter()
                .map(|f|    format!("{}", f.unwrap().path().display()))
                .collect();
            files_name.into_iter().filter(|v| v.contains(&types)== true).collect()
        }
    
    
        /*
            function to read pdf and search keyword from into inner content
            returning result success as vector of str and if err returning Pdferror
        */
    
        pub fn read_pdf(self,file_path : String) -> Vec<String> {
    
            println!("read: {}", file_path);
    
            let now = SystemTime::now();
    
            let file = File::<Vec<u8>>::open(&file_path).unwrap();
    
            let mut usize_vec: Vec<usize> = vec![];
    
            let mut res: Vec<String> = vec![];
            let elses = PdfString{ data: vec!() };
    
            for page in file.pages() {
                let page = page.unwrap();
                if let Some(ref c) = page.contents {
                    if c.to_string().contains(&self.search_query) {
                        let item_position = c.operations.iter().position(|x| {
                            x.operator == "Tj"
                                && x.operands.iter().any(|y| {
                                    y.as_string().is_ok()
                                        && format!("{}", y).contains(&self.search_query)
                                })
                        });
    
                        if let Some(position) = item_position {
                            usize_vec.push(position)
                        }
                        res.push(
                            usize_vec
                                .iter()
                                .map(|pp| {
                                    c.operations[pp - 2..pp + 25]
                                        .iter()
                                        .filter_map(|s| {
                                            s.operands.iter().find(|y| y.as_string().is_ok() &&  s.operator == "Tj")
                                        })
                                        .map(|p| {
                                            p.clone()
                                                .as_string()
                                                .unwrap_or_else(|_| &elses)
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
            return res;
        }
        /*
        used to read excel using  calamine
        */
        //pub fn read_excel(self, file_path : String) -> Vec<String> {
    
   //     }
    }
    
    impl Default for SearchQuery {
        fn default() -> SearchQuery {
            SearchQuery {
                search_query: "".to_string(),
                file_type: vec![],
                scope: Scope {
                    search_directory: vec![Local {
                        path: "".to_owned(),
                    }]
                },
            }
        }
    }
    
    #[derive(Deserialize)]
    pub struct Search{  
        pub search: String
    }
    
    
    /* this is unit test */
     #[cfg(test)]
    
     mod tests{
        /*
        Things that should have tested . 
        
        */
     }
    }