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

#[pg_guard]
pub extern "C" fn random_index_fetch_begin(rel: Relation) -> *mut IndexFetchTableData {
    unsafe {
        let mut data = PgBox::<pg_sys::IndexFetchTableData>::alloc0();
        data.rel = rel;

        data.into_pg()
    }
}

#[pg_guard]
pub extern "C" fn random_index_fetch_reset(_data: *mut pg_sys::IndexFetchTableData) {}

#[pg_guard]
pub extern "C" fn random_index_fetch_end(_data: *mut pg_sys::IndexFetchTableData) {}

#[pg_guard]
pub extern "C" fn random_index_fetch_tuple(
    _scan: *mut pg_sys::IndexFetchTableData,
    _tid: pg_sys::ItemPointer,
    _snapshot: pg_sys::Snapshot,
    _slot: *mut pg_sys::TupleTableSlot,
    _call_again: *mut bool,
    _all_dead: *mut bool,
) -> bool {
    false
}

#[pg_guard]
#[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
pub extern "C" fn random_index_delete_tuples(
    _rel: pg_sys::Relation,
    _delstate: *mut pg_sys::TM_IndexDeleteOp,
) -> pg_sys::TransactionId {
    0
}

#[pg_guard]
pub extern "C" fn random_tuple_fetch_row_version(
    _rel: pg_sys::Relation,
    _tid: pg_sys::ItemPointer,
    _snapshot: pg_sys::Snapshot,
    _slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_tuple_tid_valid(
    _scan: pg_sys::TableScanDesc,
    _tid: pg_sys::ItemPointer,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_tuple_get_latest_tid(
    _scan: pg_sys::TableScanDesc,
    _tid: pg_sys::ItemPointer,
) {
}

#[pg_guard]
pub extern "C" fn random_tuple_satisfies_snapshot(
    _rel: pg_sys::Relation,
    _slot: *mut pg_sys::TupleTableSlot,
    _snapshot: pg_sys::Snapshot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_tuple_insert(
    _rel: pg_sys::Relation,
    _slot: *mut pg_sys::TupleTableSlot,
    _cid: pg_sys::CommandId,
    _options: ::std::os::raw::c_int,
    _bistate: *mut pg_sys::BulkInsertStateData,
) {
}

#[pg_guard]
pub extern "C" fn random_multi_insert(
    rel: pg_sys::Relation,
    slots: *mut *mut pg_sys::TupleTableSlot,
    nslots: ::std::os::raw::c_int,
    _cid: pg_sys::CommandId,
    _options: ::std::os::raw::c_int,
    _bistate: *mut pg_sys::BulkInsertStateData,
) {
}

#[pg_guard]
pub extern "C" fn random_finish_bulk_insert(
    rel: pg_sys::Relation,
    _options: ::std::os::raw::c_int,
) {
}

#[pg_guard]
pub extern "C" fn random_tuple_insert_speculative(
    _rel: pg_sys::Relation,
    _slot: *mut pg_sys::TupleTableSlot,
    _cid: pg_sys::CommandId,
    _options: ::std::os::raw::c_int,
    _bistate: *mut pg_sys::BulkInsertStateData,
    _specToken: pg_sys::uint32,
) {
}

#[pg_guard]
pub extern "C" fn random_tuple_complete_speculative(
    _rel: pg_sys::Relation,
    _slot: *mut pg_sys::TupleTableSlot,
    _specToken: pg_sys::uint32,
    _succeeded: bool,
) {
}

#[pg_guard]
pub extern "C" fn random_tuple_lock(
    _rel: pg_sys::Relation,
    _tid: pg_sys::ItemPointer,
    _snapshot: pg_sys::Snapshot,
    _slot: *mut pg_sys::TupleTableSlot,
    _cid: pg_sys::CommandId,
    _mode: pg_sys::LockTupleMode,
    _wait_policy: pg_sys::LockWaitPolicy,
    _flags: pg_sys::uint8,
    _tmfd: *mut pg_sys::TM_FailureData,
) -> pg_sys::TM_Result {
    0
}

#[pg_guard]
pub extern "C" fn random_tuple_delete(
    _rel: pg_sys::Relation,
    _tid: pg_sys::ItemPointer,
    _cid: pg_sys::CommandId,
    _snapshot: pg_sys::Snapshot,
    _crosscheck: pg_sys::Snapshot,
    _wait: bool,
    _tmfd: *mut pg_sys::TM_FailureData,
    _changingPart: bool,
) -> pg_sys::TM_Result {
    0
}

#[pg_guard]
pub extern "C" fn random_tuple_update(
    _rel: pg_sys::Relation,
    _otid: pg_sys::ItemPointer,
    _slot: *mut pg_sys::TupleTableSlot,
    _cid: pg_sys::CommandId,
    _snapshot: pg_sys::Snapshot,
    _crosscheck: pg_sys::Snapshot,
    _wait: bool,
    _tmfd: *mut pg_sys::TM_FailureData,
    _lockmode: *mut pg_sys::LockTupleMode,
    _update_indexes: *mut bool,
) -> pg_sys::TM_Result {
    0
}

pub extern "C" fn random_relation_nontransactional_truncate(_rel: pg_sys::Relation) {}

#[pg_guard]
pub extern "C" fn random_relation_copy_data(
    _rel: pg_sys::Relation,
    _newrnode: *const pg_sys::RelFileNode,
) {
}

#[pg_guard]
pub extern "C" fn random_relation_copy_for_cluster(
    _NewTable: pg_sys::Relation,
    _OldTable: pg_sys::Relation,
    _OldIndex: pg_sys::Relation,
    _use_sort: bool,
    _OldestXmin: pg_sys::TransactionId,
    _xid_cutoff: *mut pg_sys::TransactionId,
    _multi_cutoff: *mut pg_sys::MultiXactId,
    _num_tuples: *mut f64,
    _tups_vacuumed: *mut f64,
    _tups_recently_dead: *mut f64,
) {
}

#[pg_guard]
pub extern "C" fn random_relation_vacuum(
    _rel: pg_sys::Relation,
    _params: *mut pg_sys::VacuumParams,
    _bstrategy: pg_sys::BufferAccessStrategy,
) {
}

#[pg_guard]
pub extern "C" fn random_scan_analyze_next_block(
    _scan: pg_sys::TableScanDesc,
    _blockno: pg_sys::BlockNumber,
    _bstrategy: pg_sys::BufferAccessStrategy,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_scan_analyze_next_tuple(
    _scan: pg_sys::TableScanDesc,
    _OldestXmin: pg_sys::TransactionId,
    _liverows: *mut f64,
    _deadrows: *mut f64,
    _slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_scan_bitmap_next_block(
    _scan: pg_sys::TableScanDesc,
    _tbmres: *mut pg_sys::TBMIterateResult,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_scan_bitmap_next_tuple(
    _scan: pg_sys::TableScanDesc,
    _tbmres: *mut pg_sys::TBMIterateResult,
    _slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_index_build_range_scan(
    _table_rel: pg_sys::Relation,
    _index_rel: pg_sys::Relation,
    _index_info: *mut pg_sys::IndexInfo,
    _allow_sync: bool,
    _anyvisible: bool,
    _progress: bool,
    _start_blockno: pg_sys::BlockNumber,
    _numblocks: pg_sys::BlockNumber,
    _callback: pg_sys::IndexBuildCallback,
    _callback_state: *mut ::std::os::raw::c_void,
    _scan: pg_sys::TableScanDesc,
) -> f64 {
    0.0
}

#[pg_guard]
pub extern "C" fn random_index_validate_scan(
    _table_rel: pg_sys::Relation,
    _index_rel: pg_sys::Relation,
    _index_info: *mut pg_sys::IndexInfo,
    _snapshot: pg_sys::Snapshot,
    _state: *mut pg_sys::ValidateIndexState,
) {
}

#[pg_guard]
pub extern "C" fn random_relation_size(
    _rel: pg_sys::Relation,
    _forkNumber: pg_sys::ForkNumber,
) -> pg_sys::uint64 {
    0
}

#[pg_guard]
pub extern "C" fn random_relation_needs_toast_table(_rel: pg_sys::Relation) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_relation_estimate_size(
    _rel: pg_sys::Relation,
    _attr_widths: *mut pg_sys::int32,
    _pages: *mut pg_sys::BlockNumber,
    _tuples: *mut f64,
    _allvisfrac: *mut f64,
) {
}

#[pg_guard]
pub extern "C" fn random_scan_sample_next_block(
    _scan: pg_sys::TableScanDesc,
    _scanstate: *mut pg_sys::SampleScanState,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_scan_sample_next_tuple(
    _scan: pg_sys::TableScanDesc,
    _scanstate: *mut pg_sys::SampleScanState,
    _slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn deltalake_relation_set_new_filenode(
    rel: pg_sys::Relation,
    _newrnode: *const pg_sys::RelFileNode,
    persistence: ::std::os::raw::c_char,
    _freezeXid: *mut pg_sys::TransactionId,
    _minmulti: *mut pg_sys::MultiXactId,
) {
}

#[pg_guard]
#[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
pub extern "C" fn random_relation_toast_am(_rel: pg_sys::Relation) -> pg_sys::Oid {
    pg_sys::Oid::INVALID
}

#[pg_guard]
#[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
pub extern "C" fn random_relation_fetch_toast_slice(
    _toastrel: pg_sys::Relation,
    _valueid: pg_sys::Oid,
    _attrsize: pg_sys::int32,
    _sliceoffset: pg_sys::int32,
    _slicelength: pg_sys::int32,
    _result: *mut pg_sys::varlena,
) {
}

#[pg_guard]
#[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
pub extern "C" fn random_scan_set_tidrange(
    _scan: pg_sys::TableScanDesc,
    _mintid: pg_sys::ItemPointer,
    _maxtid: pg_sys::ItemPointer,
) {
}

#[pg_guard]
#[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
pub extern "C" fn random_scan_getnextslot_tidrange(
    _scan: pg_sys::TableScanDesc,
    _direction: pg_sys::ScanDirection,
    _slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    false
}

#[pg_guard]
pub extern "C" fn random_slot_callbacks(
    _rel: pg_sys::Relation,
) -> *const pg_sys::TupleTableSlotOps {
    unsafe { &pg_sys::TTSOpsVirtual }
}

pub static mut DELTALAKE_TABLE_AM_ROUTINE: pg_sys::TableAmRoutine = pg_sys::TableAmRoutine {
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
