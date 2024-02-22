#![allow(unused_imports)]

use core::ffi::c_char;
use pgrx::pg_sys::*;
use pgrx::*;

#[pg_guard]
pub extern "C" fn random_scan_end(_scan: TableScanDesc) {
    /*
     * Release resources and deallocate scan. If TableScanDesc.temp_snap,
     * TableScanDesc.rs_snapshot needs to be unregistered.
     */
}

#[pg_guard]
pub extern "C" fn random_rescan(
    _scan: TableScanDesc,
    _key: *mut ScanKeyData,
    _set_params: bool,
    _allow_strat: bool,
    _allow_sync: bool,
    _allow_pagemode: bool,
) {
    /*
     * Restart relation scan.  If set_params is set to true, allow_{strat,
     * sync, pagemode} (see scan_begin) changes should be taken into account.
     */
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
    _rel: pg_sys::Relation,
    _slots: *mut *mut pg_sys::TupleTableSlot,
    _nslots: ::std::os::raw::c_int,
    _cid: pg_sys::CommandId,
    _options: ::std::os::raw::c_int,
    _bistate: *mut pg_sys::BulkInsertStateData,
) {
}

#[pg_guard]
pub extern "C" fn random_finish_bulk_insert(
    _rel: pg_sys::Relation,
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
    _spec_token: pg_sys::uint32,
) {
}

#[pg_guard]
pub extern "C" fn random_tuple_complete_speculative(
    _rel: pg_sys::Relation,
    _slot: *mut pg_sys::TupleTableSlot,
    _spec_token: pg_sys::uint32,
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
    _changing_part: bool,
) -> pg_sys::TM_Result {
    0
}

#[pg_guard]
#[cfg(any(feature = "pg15"))]
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

#[pg_guard]
#[cfg(feature = "pg16")]
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
    _update_indexes: *mut pg_sys::TU_UpdateIndexes,
) -> pg_sys::TM_Result {
    0
}

pub extern "C" fn random_relation_nontransactional_truncate(_rel: pg_sys::Relation) {}

#[pg_guard]
#[cfg(any(feature = "pg15"))]
pub extern "C" fn random_relation_copy_data(
    _rel: pg_sys::Relation,
    _newrnode: *const pg_sys::RelFileNode,
) {
}

#[pg_guard]
#[cfg(feature = "pg16")]
pub extern "C" fn random_relation_copy_data(
    _rel: pg_sys::Relation,
    _newrnode: *const pg_sys::RelFileLocator,
) {
}

#[pg_guard]
pub extern "C" fn random_relation_copy_for_cluster(
    _new_table: pg_sys::Relation,
    _old_table: pg_sys::Relation,
    _old_index: pg_sys::Relation,
    _use_sort: bool,
    _oldest_xmin: pg_sys::TransactionId,
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
    _oldest_xmin: pg_sys::TransactionId,
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
    _fork_number: pg_sys::ForkNumber,
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
#[cfg(any(feature = "pg15"))]
pub extern "C" fn deltalake_relation_set_new_filenode(
    _rel: pg_sys::Relation,
    _newrnode: *const pg_sys::RelFileNode,
    _persistence: ::std::os::raw::c_char,
    _freeze_xid: *mut pg_sys::TransactionId,
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

#[pg_guard]
#[cfg(feature = "pg16")]
pub extern "C" fn random_relation_set_new_filelocator(
    _rel: pg_sys::Relation,
    _newrlocator: *const pg_sys::RelFileLocator,
    _persistence: c_char,
    _freeze_xid: *mut pg_sys::TransactionId,
    _minmulti: *mut pg_sys::MultiXactId,
) {
}
