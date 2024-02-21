use pgrx::pg_sys::panic::ErrorReport;
use pgrx::{AnyNumeric, PgBuiltInOids, PgOid, PgSqlErrorCode};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

use pgrx::pg_sys::Oid;
use std::iter::zip;
use supabase_wrappers::prelude::*;

use fake::{Fake};

use rand::{Rng, SeedableRng};
use rand_chacha;

// A simple FDW that helps to generate random test data
#[wrappers_fdw(
    author = "Jasper Li",
    website = "https://github.com/lij55/random_fdw.git",
    error_type = "RandomFdwError"
)]
pub(crate) struct RandomFdw {
    // row counter
    row_cnt: u64,

    // total rows
    total_rows: u64,

    // target column list
    tgt_cols: Vec<Column>,

    fn_cols: Vec<Box<CellBuilder>>,

    // random generater
    rng: ChaCha8Rng,
}

//type  GenFun = fn (rng: &mut ThreadRng) -> Cell;
// CellBuilder is the closure to generate value for a cell
type CellBuilder = dyn Fn(&mut ChaCha8Rng) -> Cell;

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
            rng: ChaCha8Rng::from_entropy(),
            tgt_cols: Vec::new(),
            fn_cols: Vec::new(),
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
            Some(v) => v.parse::<u64>().unwrap_or(1024),
            None => 1024,
        };

        self.row_cnt = 0;

        // save a copy of target columns
        self.tgt_cols = columns.to_vec();

        for c in columns {
            self.fn_cols.push(create_closure(c.type_oid, options))
        }
        let seed = match options.get(&"seed".to_string()) {
            Some(v) => v.parse::<u64>().unwrap_or(0),
            None => 0,
        };

        if seed > 0 {
            self.rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
            //Faker.fake_with_rng(StdRng::from_seed(seed));
        }
        Ok(())
    }

    fn iter_scan(&mut self, row: &mut Row) -> RandomFdwResult<Option<()>> {
        // this is called on each row and we only return one row here
        if self.row_cnt < self.total_rows {
            // add values to row if they are in target column list
            for (tgt_col, f) in zip(&self.tgt_cols, &mut self.fn_cols.iter()) {
                row.push(tgt_col.name.as_str(), Some(apply_builder(f, &mut self.rng)));
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

fn create_closure(oid: Oid, _options: &HashMap<String, String>) -> Box<CellBuilder> {
    let min = 10;
    let max = 1000;
    let _max_len = 29;
    // Box::new(move |rng: &mut ThreadRng| -> Cell {
    //     Cell::I64(rng.gen_range(min..max))
    // })
    match PgOid::from(oid) {
        PgOid::BuiltIn(PgBuiltInOids::INT2OID) => Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
            Cell::I16(rng.gen_range(min as i16..max as i16))
        }),
        PgOid::BuiltIn(PgBuiltInOids::INT4OID) => Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
            Cell::I32(rng.gen_range(min as i32..max as i32))
        }),
        PgOid::BuiltIn(PgBuiltInOids::INT8OID) => {
            Box::new(move |rng: &mut ChaCha8Rng| -> Cell { Cell::I64(rng.gen_range(min..max)) })
        }
        PgOid::BuiltIn(PgBuiltInOids::FLOAT4OID) => Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
            Cell::F32(rng.gen_range(0 as f32..10 as f32))
        }),

        PgOid::BuiltIn(PgBuiltInOids::FLOAT8OID) => Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
            Cell::F64(rng.gen_range(0 as f64..10 as f64))
        }),
        PgOid::BuiltIn(PgBuiltInOids::NUMERICOID) => {
            Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
                Cell::Numeric(
                    AnyNumeric::try_from(
                        (100 as f32..1000 as f32).fake_with_rng::<f32, ChaCha8Rng>(rng),
                    )
                    .unwrap_or_default(),
                )
            })
        }

        PgOid::BuiltIn(PgBuiltInOids::CHAROID) => {
            Box::new(move |rng: &mut ChaCha8Rng| -> Cell { Cell::I8(rng.gen()) })
        }

        PgOid::BuiltIn(PgBuiltInOids::TEXTOID) => Box::new(move |rng: &mut ChaCha8Rng| -> Cell {
            Cell::String((10..20).fake_with_rng::<String, ChaCha8Rng>(rng))
        }),
        _ => Box::new(move |_rng: &mut ChaCha8Rng| -> Cell { Cell::String(String::from("")) }),
    }
}

fn apply_builder<F>(f: F, rng: &mut ChaCha8Rng) -> Cell
where
    F: Fn(&mut ChaCha8Rng) -> Cell,
{
    f(rng)
}
