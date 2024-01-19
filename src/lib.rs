use pgrx::pg_sys::panic::ErrorReport;
use pgrx::{AnyNumeric, Date, JsonB, PgBuiltInOids, PgOid, PgSqlErrorCode, Timestamp};
use std::collections::HashMap;
use rand::prelude::ThreadRng;
use supabase_wrappers::prelude::*;
use std::iter::zip;
use pgrx::pg_sys::Oid;


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
    row_cnt: u64,

    // total rows
    total_rows: u64,

    // target column list
    tgt_cols: Vec<Column>,

    fn_cols: Vec<Box<CellBuilder>>,

    // random generater
    rng:  ThreadRng

}

//type  GenFun = fn (rng: &mut ThreadRng) -> Cell;
// CellBuilder is the closure to generate value for a cell
type CellBuilder = dyn Fn(&mut ThreadRng) -> Cell;

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
            Some(v)=> v.parse::<u64>().unwrap_or(1024),
            None => 1024
        };
        rand::thread_rng();
        self.row_cnt = 0;

        // save a copy of target columns
        self.tgt_cols = columns.to_vec();

        for c in columns {
            self.fn_cols.push(create_numeric_closure(c.type_oid,100, 1000000))
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

fn create_numeric_closure(oid: Oid, min: i64, max: i64) -> Box<CellBuilder> {
    // Box::new(move |rng: &mut ThreadRng| -> Cell {
    //     Cell::I64(rng.gen_range(min..max))
    // })
    match PgOid::from(oid) {
        PgOid::BuiltIn(PgBuiltInOids::INT2OID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::I16(rng.gen_range(min as i16..max as i16))
            })
        }
        PgOid::BuiltIn(PgBuiltInOids::INT4OID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::I32(rng.gen_range(min as i32..max as i32))
            })
        }
        PgOid::BuiltIn(PgBuiltInOids::INT8OID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::I64(rng.gen_range(min..max))
            })
        }
        PgOid::BuiltIn(PgBuiltInOids::FLOAT4OID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::F32(rng.gen_range(0 as f32..10 as f32))
            })
        }

        PgOid::BuiltIn(PgBuiltInOids::FLOAT8OID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::F64(rng.gen_range(0 as f64..10 as f64))
            })
        }
        PgOid::BuiltIn(PgBuiltInOids::NUMERICOID) => {
            Box::new(move |rng: &mut ThreadRng| -> Cell {
                Cell::Numeric(AnyNumeric::try_from(rng.gen_range(100 as f32 ..1000 as f32)).unwrap_or_default())
            })
        }

        _ => Box::new(move |rng: &mut ThreadRng| -> Cell {
            Cell::I8(rng.gen_range(1..10))
        }),
    }
}

fn apply_builder<F>(f: F, rng: &mut ThreadRng) -> Cell where
    F: Fn(&mut ThreadRng) -> Cell {
    f(rng)
}

// fn get_closure(oid: Oid) -> Box<CellBuilder> {
//     match oid {
//         PgOid::BuiltIn(PgBuiltInOids::BOOLOID) => {
//             Some(Cell::Bool(bool::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::CHAROID) => {
//             Some(Cell::I8(i8::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::INT2OID) => {
//             Box::new(move |rng: &mut ThreadRng| -> Cell {
//                 Cell::I64(rng.gen_range(min..max))
//             })
//         }
//         PgOid::BuiltIn(PgBuiltInOids::FLOAT4OID) => {
//             Some(Cell::F32(f32::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::INT4OID) => {
//             Some(Cell::I32(i32::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::FLOAT8OID) => {
//             Some(Cell::F64(f64::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::INT8OID) => {
//             Some(Cell::I64(i64::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::NUMERICOID) => {
//             Some(Cell::Numeric(AnyNumeric::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::TEXTOID) => {
//             Some(Cell::String(String::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::DATEOID) => {
//             Some(Cell::Date(Date::from_datum(datum, false).unwrap()))
//         }
//         PgOid::BuiltIn(PgBuiltInOids::TIMESTAMPOID) => Some(Cell::Timestamp(
//             Timestamp::from_datum(datum, false).unwrap(),
//         )),
//         PgOid::BuiltIn(PgBuiltInOids::JSONBOID) => {
//             Some(Cell::Json(JsonB::from_datum(datum, false).unwrap()))
//         }
//         _ => None,
//     }
// }