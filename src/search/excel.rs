
pub mod excel {
    use calamine::{open_workbook, DataType, Reader, Xlsx};
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    /*
    this function is a function to test path refered contain a valid excel document with Sheet1 name
    */
    pub fn read_excel(path : String)-> String {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

        // Read whole worksheet data and provide some statistics
        if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
            let total_cells = range.get_size().0 * range.get_size().1;
            let non_empty_cells: usize = range.used_cells().count();
            println!(
                "Found {} cells in 'Sheet1', including {} non empty cells",
                total_cells, non_empty_cells
            );


            format!(
                "Found {} cells in 'Sheet1', including {} non empty cells",
                total_cells, non_empty_cells
            )
        }else {
            "".to_string()
        }
    }

    /*
    This function is to search excel file of keyword 
    probably able to use deserializer to rather than check each column one by one.
    */
    pub fn search_xlxs(path:String, search_query : String )-> Option<String>{
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
        let sheets = workbook.sheet_names();
        sheets.par_iter().map(|d|if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
            let total_cells = (range.get_size().0 , range.get_size().1);
            }else{
                "".to_string()
            });
        
        Some("Contain Data".to_string())
    }

    /* document */
}
