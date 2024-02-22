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

struct RandomScanDesc {
    pub rs_base: pg_sys::TableScanDescData,
    pub rng: ChaCha8Rng,
}

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
    //eprintln!("in scan_begin");

    Box::leak(Box::new(TableScanDescData {
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
    }))
}

// fn Datum_from_oid(typid: Oid, rng: &mut ChaCha8Rng) -> Option<Datum> {
//     let gen = create_closure(typid);
//     apply_builder(gen, rng)
// }

#[pg_guard]
pub extern "C" fn random_scan_getnextslot(
    scan: TableScanDesc,
    _direction: ScanDirection,
    slot: *mut TupleTableSlot,
) -> bool {
    unsafe { random_scan_getnextslot_impl(scan, slot) }
}

unsafe fn random_scan_getnextslot_impl(
    _scan: pg_sys::TableScanDesc,
    slot: *mut pg_sys::TupleTableSlot,
) -> bool {
    //log!("in scan_getnextslot {:#?}", slot);
    let mut rng = ChaCha8Rng::from_entropy();

    let tup_desc = (*slot).tts_tupleDescriptor;

    let tuple_desc = PgTupleDesc::from_pg_copy(tup_desc);

    if let Some(clear) = (*slot).tts_ops.as_ref().unwrap().clear {
        clear(slot);
    }

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

    // let mut values: Vec<Datum> = vec![];
    // let mut nulls: Vec<bool> = vec![];
    // for (col_index, attr) in tuple_desc.iter().enumerate() {
    //     nulls.push(false);
    //     match generate_random_data_for_oid(attr.atttypid, &mut rng) {
    //         Some(v) => values.push(v),
    //         None => {
    //             values.push(Datum::from(0));
    //             nulls[col_index] = true
    //         }
    //     }
    //     // *tts_isnull = true;
    // }
    //
    // (*slot).tts_values = values.as_mut_ptr();
    // (*slot).tts_isnull = nulls.as_mut_ptr();

    pg_sys::ExecStoreVirtualTuple(slot);

    true
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
