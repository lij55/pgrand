#![allow(dead_code)]
mod functions;

use pgrx::pg_sys::*;
use pgrx::*;
use std::ptr::addr_of_mut;

use crate::utils::*;
use rand::SeedableRng;
use rand_chacha;
use rand_chacha::ChaCha8Rng;

use crate::tableam::functions::*;

struct RandomScanData {
    pub base: TableScanDescData,
    // private state
    // it will cause `sscan->rs_rd` become to 0
    //pub rng: ChaCha8Rng,
}

type RandomScanDesc = *mut RandomScanData;

#[pg_guard]
pub extern "C" fn random_scan_begin(
    rel: Relation,
    snapshot: Snapshot,
    nkeys: ::std::os::raw::c_int,
    key: *mut ScanKeyData,
    pscan: ParallelTableScanDesc,
    flags: uint32,
) -> TableScanDesc {
    //eprintln!("in scan_begin");

    unsafe {
        //let mut scan = PgBox::<TableScanDescData>::alloc0();
        let mut scan = PgBox::<RandomScanData>::alloc0();

        scan.base.rs_rd = rel;
        scan.base.rs_snapshot = snapshot;
        scan.base.rs_nkeys = nkeys;
        scan.base.rs_key = key;
        scan.base.rs_parallel = pscan;
        scan.base.rs_flags = flags;

        // scan.rng = ChaCha8Rng::from_entropy();
        // log!("{:?}", scan.rng);

        scan.into_pg() as TableScanDesc
    }
}

//static mut MEMCTX: PgMemoryContexts = PgMemoryContexts::new("pg_search_index_build");

#[pg_guard]
pub extern "C" fn random_scan_getnextslot(
    scan: TableScanDesc,
    _direction: ScanDirection,
    slot: *mut TupleTableSlot,
) -> bool {
    unsafe {
        let mut rng = ChaCha8Rng::from_entropy();
        let _random_scan = scan as RandomScanDesc;
        if let Some(clear) = (*slot).tts_ops.as_ref().unwrap().clear {
            clear(slot);
        }

        //let mut oldctx = PgMemoryContexts::set_as_current(&mut *(*random_scan).memctx_ptr);
        //PgMemoryContexts::CurrentMemoryContext.reset();

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
        //PgMemoryContexts::set_as_current(&mut oldctx);
        //(*slot).tts_flags |= TTS_FLAG_SHOULDFREE as u16;
        pg_sys::ExecStoreVirtualTuple(slot);
        //PgMemoryContexts::set_as_current(&mut oldctx);

        true
    }
}

pub static mut RANDOM_TABLE_AM_ROUTINE: pg_sys::TableAmRoutine = pg_sys::TableAmRoutine {
    type_: pg_sys::NodeTag::T_TableAmRoutine,
    slot_callbacks: Some(random_slot_callbacks),
    scan_begin: Some(random_scan_begin),
    scan_end: Some(random_scan_end),
    scan_rescan: Some(random_rescan),
    scan_getnextslot: Some(random_scan_getnextslot),
    parallelscan_estimate: Some(random_parallelscan_estimate),
    parallelscan_initialize: Some(random_parallelscan_initialize),
    parallelscan_reinitialize: Some(random_parallelscan_reinitialize),
    index_fetch_begin: Some(random_index_fetch_begin),
    index_fetch_reset: Some(random_index_fetch_reset),
    index_fetch_end: Some(random_index_fetch_end),
    index_fetch_tuple: Some(random_index_fetch_tuple),
    tuple_fetch_row_version: Some(random_tuple_fetch_row_version),
    tuple_tid_valid: Some(random_tuple_tid_valid),
    tuple_get_latest_tid: Some(random_tuple_get_latest_tid),
    tuple_satisfies_snapshot: Some(random_tuple_satisfies_snapshot),
    tuple_insert: Some(random_tuple_insert),
    tuple_insert_speculative: Some(random_tuple_insert_speculative),
    tuple_complete_speculative: Some(random_tuple_complete_speculative),
    multi_insert: Some(random_multi_insert),
    tuple_delete: Some(random_tuple_delete),
    tuple_update: Some(random_tuple_update),
    tuple_lock: Some(random_tuple_lock),
    finish_bulk_insert: Some(random_finish_bulk_insert),
    relation_nontransactional_truncate: Some(random_relation_nontransactional_truncate),
    relation_copy_data: Some(random_relation_copy_data),
    relation_copy_for_cluster: Some(random_relation_copy_for_cluster),
    relation_vacuum: Some(random_relation_vacuum),
    scan_analyze_next_block: Some(random_scan_analyze_next_block),
    scan_analyze_next_tuple: Some(random_scan_analyze_next_tuple),
    index_build_range_scan: Some(random_index_build_range_scan),
    index_validate_scan: Some(random_index_validate_scan),
    relation_size: Some(random_relation_size),
    relation_needs_toast_table: Some(random_relation_needs_toast_table),
    relation_estimate_size: Some(random_relation_estimate_size),
    scan_bitmap_next_block: Some(random_scan_bitmap_next_block),
    scan_bitmap_next_tuple: Some(random_scan_bitmap_next_tuple),
    scan_sample_next_block: Some(random_scan_sample_next_block),
    scan_sample_next_tuple: Some(random_scan_sample_next_tuple),
    #[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
    relation_set_new_filenode: Some(deltalake_relation_set_new_filenode),
    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    relation_toast_am: Some(random_relation_toast_am),
    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    relation_fetch_toast_slice: Some(random_relation_fetch_toast_slice),
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    scan_set_tidrange: Some(random_scan_set_tidrange),
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    scan_getnextslot_tidrange: Some(random_scan_getnextslot_tidrange),
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    index_delete_tuples: Some(random_index_delete_tuples),
    #[cfg(any(feature = "pg16"))]
    relation_set_new_filelocator: Some(random_relation_set_new_filelocator),
};

#[pg_guard]
#[no_mangle]
extern "C" fn pg_finfo_random_tableam_handler() -> &'static pg_sys::Pg_finfo_record {
    const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
    &V1_API
}

extension_sql!(
    r#"
CREATE FUNCTION random_tableam_handler(internal)
RETURNS table_am_handler AS 'MODULE_PATHNAME', 'random_tableam_handler' LANGUAGE C STRICT;
CREATE ACCESS METHOD random TYPE TABLE HANDLER random_tableam_handler;
COMMENT ON ACCESS METHOD random IS 'generate random data';
"#,
    name = "random_tableam_handler"
);
#[no_mangle]
#[pg_guard]
extern "C" fn random_tableam_handler(
    _fcinfo: pg_sys::FunctionCallInfo,
) -> *mut pg_sys::TableAmRoutine {
    unsafe { addr_of_mut!(RANDOM_TABLE_AM_ROUTINE) }
}
