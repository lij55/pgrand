use pgrx::pg_sys::*;
use pgrx::*;
use std::ptr::addr_of_mut;

/*
 * Start a scan of `rel`.  The callback has to return a TableScanDesc,
 * which will typically be embedded in a larger, AM specific, struct.
 *
 * If nkeys != 0, the results need to be filtered by those scan keys.
 *
 * pscan, if not NULL, will have already been initialized with
 * parallelscan_initialize(), and has to be for the same relation. Will
 * only be set coming from table_beginscan_parallel().
 *
 * `flags` is a bitmask indicating the type of scan (ScanOptions's
 * SO_TYPE_*, currently only one may be specified), options controlling
 * the scan's behaviour (ScanOptions's SO_ALLOW_*, several may be
 * specified, an AM may ignore unsupported ones) and whether the snapshot
 * needs to be deallocated at scan_end (ScanOptions's SO_TEMP_SNAPSHOT).
 */
#[pg_guard]
pub extern "C" fn random_scan_begin(
    rel: Relation,
    snapshot: Snapshot,
    nkeys: ::std::os::raw::c_int,
    key: *mut ScanKeyData,
    pscan: ParallelTableScanDesc,
    flags: uint32,
) -> TableScanDesc {
    let desc = Box::leak(Box::new(TableScanDescData {
        rs_rd: rel,
        rs_snapshot: snapshot,
        rs_nkeys: nkeys,
        rs_key: key,
        rs_mintid: ItemPointerData {
            ip_blkid: BlockIdData { bi_hi: 0, bi_lo: 0 },
            ip_posid: 0,
        },
        rs_maxtid: ItemPointerData {
            ip_blkid: BlockIdData { bi_hi: 0, bi_lo: 0 },
            ip_posid: 0,
        },
        rs_flags: flags,
        rs_parallel: pscan,
    }));
    desc
}

#[pg_guard]
pub extern "C" fn random_scan_end(_scan: TableScanDesc) {
    /*
     * Release resources and deallocate scan. If TableScanDesc.temp_snap,
     * TableScanDesc.rs_snapshot needs to be unregistered.
     */
}

#[pg_guard]
pub extern "C" fn random_rescan(
    scan: TableScanDesc,
    key: *mut ScanKeyData,
    set_params: bool,
    allow_strat: bool,
    allow_sync: bool,
    allow_pagemode: bool,
) {
    /*
     * Restart relation scan.  If set_params is set to true, allow_{strat,
     * sync, pagemode} (see scan_begin) changes should be taken into account.
     */
}

#[pg_guard]
pub extern "C" fn random_scan_getnextslot(
    scan: TableScanDesc,
    _direction: ScanDirection,
    slot: *mut TupleTableSlot,
) -> bool {
    unsafe { deltalake_scan_getnextslot_impl(scan, slot) }
}

unsafe fn deltalake_scan_getnextslot_impl(
    scan: pg_sys::TableScanDesc,
    slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    if let Some(clear) = (*slot).tts_ops.as_ref().unwrap().clear {
        clear(slot);
    }
    false
}

pub extern "C" fn random_parallelscan_estimate(rel: Relation) -> Size {
    unsafe { pg_sys::table_block_parallelscan_estimate(rel) }
}

#[pg_guard]
pub extern "C" fn random_parallelscan_initialize(
    rel: pg_sys::Relation,
    pscan: pg_sys::ParallelTableScanDesc,
) -> pg_sys::Size {
    unsafe { pg_sys::table_block_parallelscan_initialize(rel, pscan) }
}

#[pg_guard]
pub extern "C" fn random_parallelscan_reinitialize(
    rel: pg_sys::Relation,
    pscan: pg_sys::ParallelTableScanDesc,
) {
    unsafe { pg_sys::table_block_parallelscan_reinitialize(rel, pscan) }
}

pub static mut DELTALAKE_TABLE_AM_ROUTINE: pg_sys::TableAmRoutine = pg_sys::TableAmRoutine {
    type_: pg_sys::NodeTag::T_TableAmRoutine,
    slot_callbacks: None,
    scan_begin: Some(random_scan_begin),
    scan_end: Some(random_scan_end),
    scan_rescan: Some(random_rescan),
    scan_getnextslot: Some(random_scan_getnextslot),
    parallelscan_estimate: Some(random_parallelscan_estimate),
    parallelscan_initialize: Some(random_parallelscan_initialize),
    parallelscan_reinitialize: Some(random_parallelscan_reinitialize),
    index_fetch_begin: None,
    index_fetch_reset: None,
    index_fetch_end: None,
    index_fetch_tuple: None,
    tuple_fetch_row_version: None,
    tuple_tid_valid: None,
    tuple_get_latest_tid: None,
    tuple_satisfies_snapshot: None,
    tuple_insert: None,
    tuple_insert_speculative: None,
    tuple_complete_speculative: None,
    multi_insert: None,
    tuple_delete: None,
    tuple_update: None,
    tuple_lock: None,
    finish_bulk_insert: None,
    relation_nontransactional_truncate: None,
    relation_copy_data: None,
    relation_copy_for_cluster: None,
    relation_vacuum: None,
    scan_analyze_next_block: None,
    scan_analyze_next_tuple: None,
    index_build_range_scan: None,
    index_validate_scan: None,
    relation_size: None,
    relation_needs_toast_table: None,
    relation_estimate_size: None,
    scan_bitmap_next_block: None,
    scan_bitmap_next_tuple: None,
    scan_sample_next_block: None,
    scan_sample_next_tuple: None,
    #[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
    relation_set_new_filenode: None,
    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    relation_toast_am: None,
    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    relation_fetch_toast_slice: None,
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    scan_set_tidrange: None,
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    scan_getnextslot_tidrange: None,
    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    index_delete_tuples: None,
};

#[pg_guard]
#[no_mangle]
extern "C" fn pg_finfo_deltalake_tableam_handler() -> &'static pg_sys::Pg_finfo_record {
    const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
    &V1_API
}

extension_sql!(
    r#"
    CREATE FUNCTION deltalake_tableam_handler(internal)
    RETURNS table_am_handler AS 'MODULE_PATHNAME', 'deltalake_tableam_handler' LANGUAGE C STRICT;
    CREATE ACCESS METHOD parquet TYPE TABLE HANDLER deltalake_tableam_handler;
    COMMENT ON ACCESS METHOD parquet IS 'ParadeDB parquet table access method';
    "#,
    name = "deltalake_tableam_handler"
);
#[no_mangle]
#[pg_guard]
extern "C" fn deltalake_tableam_handler(
    _fcinfo: pg_sys::FunctionCallInfo,
) -> *mut pg_sys::TableAmRoutine {
    unsafe { addr_of_mut!(DELTALAKE_TABLE_AM_ROUTINE) }
}
