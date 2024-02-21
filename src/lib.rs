mod fdw;
mod tableam;
mod utils;

use crate::utils::PARADE_GUC;
use pgrx::*;

pgrx::pg_module_magic!();

#[pg_guard]
pub extern "C" fn _PG_init() {
    PARADE_GUC.init();
}
