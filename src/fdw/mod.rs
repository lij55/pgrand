use crate::utils::generate_random_data_for_oid;
use pgrx::pg_sys::*;
use pgrx::{extension_sql, pg_guard, pg_sys, PgBox, PgList, PgTupleDesc};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::ptr::addr_of_mut;

#[pg_guard]
pub extern "C" fn random_get_foreign_rel_size(
    _root: *mut PlannerInfo,
    _baserel: *mut RelOptInfo,
    _foreigntableid: Oid,
) {
    // get estimate row count and mean row width
    // let (rows, width) = state.get_rel_size().report_unwrap();
    // (*baserel).rows = rows as f64;
    // (*(*baserel).reltarget).width = width;
}

#[pg_guard]
pub extern "C" fn random_get_foreign_paths(
    root: *mut pgrx::prelude::pg_sys::PlannerInfo,
    baserel: *mut pgrx::prelude::pg_sys::RelOptInfo,
    _foreigntableid: pgrx::prelude::pg_sys::Oid,
) {
    debug2!("---> get_foreign_paths");
    unsafe {
        // create a ForeignPath node and add it as the only possible path
        let path = pgrx::prelude::pg_sys::create_foreignscan_path(
            root,
            baserel,
            ptr::null_mut(), // default pathtarget
            (*baserel).rows,
            0.0,
            0.0,
            ptr::null_mut(), // no pathkeys
            ptr::null_mut(), // no outer rel either
            ptr::null_mut(), // no extra plan
            ptr::null_mut(), // no fdw_private data
        );
        pgrx::prelude::pg_sys::add_path(baserel, &mut ((*path).path));
    }
}

#[pg_guard]
pub extern "C" fn get_foreign_plan(
    _root: *mut pgrx::prelude::pg_sys::PlannerInfo,
    baserel: *mut pgrx::prelude::pg_sys::RelOptInfo,
    _foreigntableid: pgrx::prelude::pg_sys::Oid,
    _best_path: *mut pgrx::prelude::pg_sys::ForeignPath,
    tlist: *mut pgrx::prelude::pg_sys::List,
    scan_clauses: *mut pgrx::prelude::pg_sys::List,
    outer_plan: *mut pgrx::prelude::pg_sys::Plan,
) -> *mut pgrx::prelude::pg_sys::ForeignScan {
    debug2!("---> get_foreign_plan");
    unsafe {
        pgrx::prelude::pg_sys::make_foreignscan(
            tlist,
            scan_clauses,
            (*baserel).relid,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            outer_plan,
        )
    }
}

type Options = HashMap<std::string::String, std::string::String>;

struct RandomFdwStat {
    pub test: u8,
    pub rng: ChaCha8Rng,
    pub opts: Options,
}

#[pg_guard]
pub extern "C" fn random_begin_foreign_scan(
    node: *mut pgrx::prelude::pg_sys::ForeignScanState,
    _eflags: c_int,
) {
    debug2!("---> begin_foreign_scan");
    unsafe {
        let mut my_fdw_state = PgBox::<RandomFdwStat>::alloc0();
        my_fdw_state.test = 100;
        my_fdw_state.rng = ChaCha8Rng::seed_from_u64(1);

        let foreign_table_id = (*((*node).ss.ss_currentRelation)).rd_id;

        let ftable = pg_sys::GetForeignTable(foreign_table_id);

        my_fdw_state.opts = options_to_hashmap((*ftable).options).unwrap();

        (*node).fdw_state = my_fdw_state.into_pg() as *mut ::std::os::raw::c_void;
    }
}

#[pg_guard]
pub extern "C" fn random_re_scan_foreign_scan(_node: *mut pgrx::prelude::pg_sys::ForeignScanState) {
    debug2!("---> re_scan_foreign_scan");
}

#[pg_guard]
pub extern "C" fn random_end_foreign_scan(node: *mut pgrx::prelude::pg_sys::ForeignScanState) {
    debug2!("---> end_foreign_scan");
    unsafe {
        let mut my_fdw_stat =
            PgBox::<RandomFdwStat>::from_pg((*node).fdw_state as *mut RandomFdwStat);
        debug2!("{}", my_fdw_stat.test);
        debug2!("{:?}", my_fdw_stat.opts);
    }
}

#[pg_guard]
pub extern "C" fn random_iterate_foreign_scan(
    node: *mut pgrx::prelude::pg_sys::ForeignScanState,
) -> *mut pgrx::prelude::pg_sys::TupleTableSlot {
    // `debug!` macros are quite expensive at the moment, so avoid logging in the inner loop
    // debug2!("---> iterate_foreign_scan");
    unsafe {
        let mut my_fdw_stat =
            PgBox::<RandomFdwStat>::from_pg((*node).fdw_state as *mut RandomFdwStat);
        //let mut rng = ChaCha8Rng::from_entropy();

        let mut rng = (*my_fdw_stat).rng.clone();

        // clear slot
        let slot = (*node).ss.ss_ScanTupleSlot;
        if let Some(clear) = (*(*slot).tts_ops).clear {
            clear(slot);
        }

        let tup_desc = (*slot).tts_tupleDescriptor;

        let tuple_desc = PgTupleDesc::from_pg_copy(tup_desc);

        for (col_index, attr) in tuple_desc.iter().enumerate() {
            //eprintln!("{col_index}: {}", attr.atttypid);
            let tts_isnull = (*slot).tts_isnull.add(col_index);
            let tts_value = (*slot).tts_values.add(col_index);

            match generate_random_data_for_oid(attr.atttypid, &mut rng) {
                Some(v) => *tts_value = v,
                None => *tts_isnull = true,
            }
            // *tts_isnull = true;
        }
        pgrx::prelude::pg_sys::ExecStoreVirtualTuple(slot);

        slot
    }
}

pub static mut RANDOM_FDW_ROUTINE: pg_sys::FdwRoutine = pg_sys::FdwRoutine {
    type_: pg_sys::NodeTag::T_FdwRoutine,
    BeginForeignScan: Some(random_begin_foreign_scan),
    IterateForeignScan: Some(random_iterate_foreign_scan),
    ReScanForeignScan: Some(random_re_scan_foreign_scan),
    EndForeignScan: Some(random_end_foreign_scan),
    GetForeignJoinPaths: None,
    GetForeignUpperPaths: None,
    AddForeignUpdateTargets: None,
    PlanForeignModify: None,
    BeginForeignModify: None,
    ExecForeignInsert: None,
    ExecForeignBatchInsert: None,
    GetForeignModifyBatchSize: None,
    ExecForeignUpdate: None,
    ExecForeignDelete: None,
    EndForeignModify: None,
    BeginForeignInsert: None,
    EndForeignInsert: None,
    IsForeignRelUpdatable: None,
    PlanDirectModify: None,
    BeginDirectModify: None,
    IterateDirectModify: None,
    EndDirectModify: None,
    GetForeignRowMarkType: None,
    RefetchForeignRow: None,
    GetForeignRelSize: Some(random_get_foreign_rel_size),
    GetForeignPaths: Some(random_get_foreign_paths),
    GetForeignPlan: Some(get_foreign_plan),
    ExplainForeignScan: None,
    ExplainForeignModify: None,
    ExplainDirectModify: None,
    AnalyzeForeignTable: None,
    ImportForeignSchema: None,
    ExecForeignTruncate: None,
    IsForeignScanParallelSafe: None,
    EstimateDSMForeignScan: None,
    InitializeDSMForeignScan: None,
    ReInitializeDSMForeignScan: None,
    InitializeWorkerForeignScan: None,
    ShutdownForeignScan: None,
    ReparameterizeForeignPathByChild: None,
    IsForeignPathAsyncCapable: None,
    ForeignAsyncRequest: None,
    ForeignAsyncConfigureWait: None,
    RecheckForeignScan: None,
    ForeignAsyncNotify: None,
};

#[pg_guard]
#[no_mangle]
extern "C" fn pg_finfo_random_fdw_handler() -> &'static pg_sys::Pg_finfo_record {
    const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
    &V1_API
}

extension_sql!(
    r#"
CREATE FUNCTION random_fdw_handler()
RETURNS fdw_handler AS 'MODULE_PATHNAME', 'random_fdw_handler' LANGUAGE C STRICT;
"#,
    name = "random_fdw_handler"
);

#[no_mangle]
#[pg_guard]
extern "C" fn random_fdw_handler(_fcinfo: pg_sys::FunctionCallInfo) -> *mut pg_sys::FdwRoutine {
    unsafe { addr_of_mut!(RANDOM_FDW_ROUTINE) }
}

// convert options definition to hashmap
pub unsafe fn options_to_hashmap(
    options: *mut pg_sys::List,
) -> Option<HashMap<std::string::String, std::string::String>> {
    let mut ret = HashMap::new();
    let options: PgList<pg_sys::DefElem> = PgList::from_pg(options);
    for option in options.iter_ptr() {
        let name = CStr::from_ptr((*option).defname);
        let value = CStr::from_ptr(pg_sys::defGetString(option));
        let name = name.to_str().unwrap();
        let value = value.to_str().unwrap();
        ret.insert(name.to_string(), value.to_string());
    }
    Some(ret)
}
