use pgrx::pg_sys::panic::ErrorReport;
use pgrx::PgSqlErrorCode;
use std::collections::HashMap;
use rand::prelude::ThreadRng;
use supabase_wrappers::prelude::*;

use rand::Rng;

pgrx::pg_module_magic!();

// A simple FDW that helps to generate random test data
#[wrappers_fdw(
author = "Jasper Li",
website = "https://github.com/lij55/random_fdw.git",
error_type = "RandomFdwError"
)]
pub(crate) struct RandomFdw {
    // row counter
    row_cnt: i64,

    // total rows
    total_rows: i64,

    // target column list
    tgt_cols: Vec<Column>,

    // random generater
    rng: ThreadRng

}

enum RandomFdwError {}

impl From<RandomFdwError> for ErrorReport {
    fn from(_value: RandomFdwError) -> Self {
        ErrorReport::new(PgSqlErrorCode::ERRCODE_FDW_ERROR, "", "")
    }
}

type RandomFdwResult<T> = Result<T, RandomFdwError>;

impl ForeignDataWrapper<RandomFdwError> for RandomFdw {
    fn new(_options: &HashMap<String, String>) -> RandomFdwResult<Self> {
        Ok(Self {
            row_cnt: 0,
            total_rows: 0,
            rng :  ThreadRng::default(),
            tgt_cols: Vec::new(),
        })
    }

    fn begin_scan(
        &mut self,
        _quals: &[Qual],
        columns: &[Column],
        _sorts: &[Sort],
        _limit: &Option<Limit>,
        options: &HashMap<String, String>,
    ) -> RandomFdwResult<()> {
        // default is 1024
        self.total_rows = match options.get(&"total".to_string()) {
            Some(v)=> v.parse::<i64>().unwrap_or(1024),
            None => 1024
        };
        rand::thread_rng();
        self.row_cnt = 0;

        // save a copy of target columns
        self.tgt_cols = columns.to_vec();

        Ok(())
    }

    fn iter_scan(&mut self, row: &mut Row) -> RandomFdwResult<Option<()>> {
        // this is called on each row and we only return one row here
        if self.row_cnt < self.total_rows {
            // add values to row if they are in target column list
            for tgt_col in &self.tgt_cols {
                row.push(tgt_col.name.as_str(), Some(Cell::I32(self.rng.gen())));
            }

            self.row_cnt += 1;

            // return Some(()) to Postgres and continue data scan
            return Ok(Some(()));
        }

        // return 'None' to stop data scan
        Ok(None)
    }

    fn end_scan(&mut self) -> RandomFdwResult<()> {
        // we do nothing here, but you can do things like resource cleanup and etc.
        Ok(())
    }
}
