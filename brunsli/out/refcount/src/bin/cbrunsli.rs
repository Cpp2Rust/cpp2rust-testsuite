extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static BRUNSLI_ANS_LOG_TAB_SIZE_0: Value<i32> = Rc::new(RefCell::new(10));
);
thread_local!(
    pub static BRUNSLI_ANS_TAB_SIZE_1: Value<i32> = Rc::new(RefCell::new(
        (1 << BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())),
    ));
);
thread_local!(
    pub static kFallbackVersion_2: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kDCTBlockSize_3: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kMaxComponents_4: Value<i32> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kMaxQuantTables_5: Value<i32> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kMaxHuffmanTables_6: Value<i32> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kJpegHuffmanMaxBitLength_7: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kJpegHuffmanAlphabetSize_8: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    pub static kJpegDCAlphabetSize_9: Value<i32> = Rc::new(RefCell::new(12));
);
thread_local!(
    pub static kMaxDHTMarkers_10: Value<i32> = Rc::new(RefCell::new(512));
);
thread_local!(
    pub static kMaxDimPixels_11: Value<i32> = Rc::new(RefCell::new(65535));
);
thread_local!(
    pub static kDefaultQuantMatrix_12: Value<Box<[Value<Box<[u8]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                16_u8, 11_u8, 10_u8, 16_u8, 24_u8, 40_u8, 51_u8, 61_u8, 12_u8, 12_u8, 14_u8, 19_u8,
                26_u8, 58_u8, 60_u8, 55_u8, 14_u8, 13_u8, 16_u8, 24_u8, 40_u8, 57_u8, 69_u8, 56_u8,
                14_u8, 17_u8, 22_u8, 29_u8, 51_u8, 87_u8, 80_u8, 62_u8, 18_u8, 22_u8, 37_u8, 56_u8,
                68_u8, 109_u8, 103_u8, 77_u8, 24_u8, 35_u8, 55_u8, 64_u8, 81_u8, 104_u8, 113_u8,
                92_u8, 49_u8, 64_u8, 78_u8, 87_u8, 103_u8, 121_u8, 120_u8, 101_u8, 72_u8, 92_u8,
                95_u8, 98_u8, 112_u8, 100_u8, 103_u8, 99_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                17_u8, 18_u8, 24_u8, 47_u8, 99_u8, 99_u8, 99_u8, 99_u8, 18_u8, 21_u8, 26_u8, 66_u8,
                99_u8, 99_u8, 99_u8, 99_u8, 24_u8, 26_u8, 56_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
                47_u8, 66_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
                99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
                99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
                99_u8, 99_u8, 99_u8, 99_u8,
            ]))),
        ])));
);
thread_local!(
    pub static kJPEGNaturalOrder_13: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 1_u32, 8_u32, 16_u32, 9_u32, 2_u32, 3_u32, 10_u32, 17_u32, 24_u32, 32_u32, 25_u32,
        18_u32, 11_u32, 4_u32, 5_u32, 12_u32, 19_u32, 26_u32, 33_u32, 40_u32, 48_u32, 41_u32,
        34_u32, 27_u32, 20_u32, 13_u32, 6_u32, 7_u32, 14_u32, 21_u32, 28_u32, 35_u32, 42_u32,
        49_u32, 56_u32, 57_u32, 50_u32, 43_u32, 36_u32, 29_u32, 22_u32, 15_u32, 23_u32, 30_u32,
        37_u32, 44_u32, 51_u32, 58_u32, 59_u32, 52_u32, 45_u32, 38_u32, 31_u32, 39_u32, 46_u32,
        53_u32, 60_u32, 61_u32, 54_u32, 47_u32, 55_u32, 62_u32, 63_u32, 63_u32, 63_u32, 63_u32,
        63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32,
        63_u32, 63_u32,
    ])));
);
thread_local!(
    pub static kJPEGZigZagOrder_14: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 1_u32, 5_u32, 6_u32, 14_u32, 15_u32, 27_u32, 28_u32, 2_u32, 4_u32, 7_u32, 13_u32,
        16_u32, 26_u32, 29_u32, 42_u32, 3_u32, 8_u32, 12_u32, 17_u32, 25_u32, 30_u32, 41_u32,
        43_u32, 9_u32, 11_u32, 18_u32, 24_u32, 31_u32, 40_u32, 44_u32, 53_u32, 10_u32, 19_u32,
        23_u32, 32_u32, 39_u32, 45_u32, 52_u32, 54_u32, 20_u32, 22_u32, 33_u32, 38_u32, 46_u32,
        51_u32, 55_u32, 60_u32, 21_u32, 34_u32, 37_u32, 47_u32, 50_u32, 56_u32, 59_u32, 61_u32,
        35_u32, 36_u32, 48_u32, 49_u32, 57_u32, 58_u32, 62_u32, 63_u32,
    ])));
);
pub type brunsli_JPEGReadError = i32;
pub const brunsli_JPEGReadError_OK: brunsli_JPEGReadError = 0;
pub const brunsli_JPEGReadError_SOI_NOT_FOUND: brunsli_JPEGReadError = 1;
pub const brunsli_JPEGReadError_SOF_NOT_FOUND: brunsli_JPEGReadError = 2;
pub const brunsli_JPEGReadError_UNEXPECTED_EOF: brunsli_JPEGReadError = 3;
pub const brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND: brunsli_JPEGReadError = 4;
pub const brunsli_JPEGReadError_UNSUPPORTED_MARKER: brunsli_JPEGReadError = 5;
pub const brunsli_JPEGReadError_WRONG_MARKER_SIZE: brunsli_JPEGReadError = 6;
pub const brunsli_JPEGReadError_INVALID_PRECISION: brunsli_JPEGReadError = 7;
pub const brunsli_JPEGReadError_INVALID_WIDTH: brunsli_JPEGReadError = 8;
pub const brunsli_JPEGReadError_INVALID_HEIGHT: brunsli_JPEGReadError = 9;
pub const brunsli_JPEGReadError_INVALID_NUMCOMP: brunsli_JPEGReadError = 10;
pub const brunsli_JPEGReadError_INVALID_SAMP_FACTOR: brunsli_JPEGReadError = 11;
pub const brunsli_JPEGReadError_INVALID_START_OF_SCAN: brunsli_JPEGReadError = 12;
pub const brunsli_JPEGReadError_INVALID_END_OF_SCAN: brunsli_JPEGReadError = 13;
pub const brunsli_JPEGReadError_INVALID_SCAN_BIT_POSITION: brunsli_JPEGReadError = 14;
pub const brunsli_JPEGReadError_INVALID_COMPS_IN_SCAN: brunsli_JPEGReadError = 15;
pub const brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX: brunsli_JPEGReadError = 16;
pub const brunsli_JPEGReadError_INVALID_QUANT_TBL_INDEX: brunsli_JPEGReadError = 17;
pub const brunsli_JPEGReadError_INVALID_QUANT_VAL: brunsli_JPEGReadError = 18;
pub const brunsli_JPEGReadError_INVALID_MARKER_LEN: brunsli_JPEGReadError = 19;
pub const brunsli_JPEGReadError_INVALID_SAMPLING_FACTORS: brunsli_JPEGReadError = 20;
pub const brunsli_JPEGReadError_INVALID_HUFFMAN_CODE: brunsli_JPEGReadError = 21;
pub const brunsli_JPEGReadError_INVALID_SYMBOL: brunsli_JPEGReadError = 22;
pub const brunsli_JPEGReadError_NON_REPRESENTABLE_DC_COEFF: brunsli_JPEGReadError = 23;
pub const brunsli_JPEGReadError_NON_REPRESENTABLE_AC_COEFF: brunsli_JPEGReadError = 24;
pub const brunsli_JPEGReadError_INVALID_SCAN: brunsli_JPEGReadError = 25;
pub const brunsli_JPEGReadError_OVERLAPPING_SCANS: brunsli_JPEGReadError = 26;
pub const brunsli_JPEGReadError_INVALID_SCAN_ORDER: brunsli_JPEGReadError = 27;
pub const brunsli_JPEGReadError_EXTRA_ZERO_RUN: brunsli_JPEGReadError = 28;
pub const brunsli_JPEGReadError_DUPLICATE_DRI: brunsli_JPEGReadError = 29;
pub const brunsli_JPEGReadError_DUPLICATE_SOF: brunsli_JPEGReadError = 30;
pub const brunsli_JPEGReadError_WRONG_RESTART_MARKER: brunsli_JPEGReadError = 31;
pub const brunsli_JPEGReadError_DUPLICATE_COMPONENT_ID: brunsli_JPEGReadError = 32;
pub const brunsli_JPEGReadError_COMPONENT_NOT_FOUND: brunsli_JPEGReadError = 33;
pub const brunsli_JPEGReadError_HUFFMAN_TABLE_NOT_FOUND: brunsli_JPEGReadError = 34;
pub const brunsli_JPEGReadError_HUFFMAN_TABLE_ERROR: brunsli_JPEGReadError = 35;
pub const brunsli_JPEGReadError_QUANT_TABLE_NOT_FOUND: brunsli_JPEGReadError = 36;
pub const brunsli_JPEGReadError_EMPTY_DHT: brunsli_JPEGReadError = 37;
pub const brunsli_JPEGReadError_EMPTY_DQT: brunsli_JPEGReadError = 38;
pub const brunsli_JPEGReadError_OUT_OF_BAND_COEFF: brunsli_JPEGReadError = 39;
pub const brunsli_JPEGReadError_EOB_RUN_TOO_LONG: brunsli_JPEGReadError = 40;
pub const brunsli_JPEGReadError_IMAGE_TOO_LARGE: brunsli_JPEGReadError = 41;
pub const brunsli_JPEGReadError_INVALID_QUANT_TBL_PRECISION: brunsli_JPEGReadError = 42;
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(268)]
pub struct brunsli_JPEGQuantTable {
    #[offset(0)]
    #[byte_size(256)]
    pub values: Value<Vec<i32>>,
    #[offset(256)]
    pub precision: i32,
    #[offset(260)]
    pub index: i32,
    #[offset(264)]
    pub is_last: bool,
}
impl Default for brunsli_JPEGQuantTable {
    fn default() -> Self {
        brunsli_JPEGQuantTable {
            values: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
            precision: 0,
            index: 0,
            is_last: true,
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1104)]
pub struct brunsli_JPEGHuffmanCode {
    #[offset(0)]
    #[byte_size(68)]
    pub counts: Value<Vec<i32>>,
    #[offset(68)]
    #[byte_size(1028)]
    pub values: Value<Vec<i32>>,
    #[offset(1096)]
    pub slot_id: i32,
    #[offset(1100)]
    pub is_last: bool,
}
impl Default for brunsli_JPEGHuffmanCode {
    fn default() -> Self {
        brunsli_JPEGHuffmanCode {
            counts: Rc::new(RefCell::new(
                std::array::from_fn::<_, 17, _>(|_| Default::default()).to_vec(),
            )),
            values: Rc::new(RefCell::new(
                std::array::from_fn::<_, 257, _>(|_| Default::default()).to_vec(),
            )),
            slot_id: 0,
            is_last: true,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct brunsli_JPEGComponentScanInfo {
    #[offset(0)]
    pub comp_idx: u8,
    #[offset(4)]
    pub dc_tbl_idx: i32,
    #[offset(8)]
    pub ac_tbl_idx: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct brunsli_JPEGScanInfo_ExtraZeroRunInfo {
    #[offset(0)]
    pub block_idx: i32,
    #[offset(4)]
    pub num_extra_zero_runs: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(120)]
pub struct brunsli_JPEGScanInfo {
    #[offset(0)]
    pub Ss: i32,
    #[offset(4)]
    pub Se: i32,
    #[offset(8)]
    pub Ah: i32,
    #[offset(12)]
    pub Al: i32,
    #[offset(16)]
    pub num_components: usize,
    #[offset(24)]
    #[byte_size(48)]
    pub components: Value<Vec<brunsli_JPEGComponentScanInfo>>,
    #[offset(72)]
    #[byte_size(24)]
    pub reset_points: Value<Vec<i32>>,
    #[offset(96)]
    #[byte_size(24)]
    pub extra_zero_runs: Value<Vec<brunsli_JPEGScanInfo_ExtraZeroRunInfo>>,
}
impl Default for brunsli_JPEGScanInfo {
    fn default() -> Self {
        brunsli_JPEGScanInfo {
            Ss: 0_i32,
            Se: 0_i32,
            Ah: 0_i32,
            Al: 0_i32,
            num_components: 0_usize,
            components: Rc::new(RefCell::new(
                std::array::from_fn::<_, 4, _>(|_| Default::default()).to_vec(),
            )),
            reset_points: Rc::new(RefCell::new(Default::default())),
            extra_zero_runs: Rc::new(RefCell::new(Default::default())),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(56)]
pub struct brunsli_JPEGComponent {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    pub h_samp_factor: i32,
    #[offset(8)]
    pub v_samp_factor: i32,
    #[offset(12)]
    pub quant_idx: u8,
    #[offset(16)]
    pub width_in_blocks: u32,
    #[offset(20)]
    pub height_in_blocks: u32,
    #[offset(24)]
    pub num_blocks: u32,
    #[offset(32)]
    #[byte_size(24)]
    pub coeffs: Value<Vec<i16>>,
}
impl brunsli_JPEGComponent {
    pub fn new() -> Self {
        Self {
            id: 0,
            h_samp_factor: 1,
            v_samp_factor: 1,
            quant_idx: 0_u8,
            width_in_blocks: 0_u32,
            height_in_blocks: 0_u32,
            num_blocks: 0_u32,
            coeffs: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl Default for brunsli_JPEGComponent {
    fn default() -> Self {
        { brunsli_JPEGComponent::new() }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(296)]
pub struct brunsli_JPEGData {
    #[offset(0)]
    pub width: i32,
    #[offset(4)]
    pub height: i32,
    #[offset(8)]
    pub version: i32,
    #[offset(12)]
    pub max_h_samp_factor: i32,
    #[offset(16)]
    pub max_v_samp_factor: i32,
    #[offset(20)]
    pub MCU_rows: i32,
    #[offset(24)]
    pub MCU_cols: i32,
    #[offset(28)]
    pub restart_interval: i32,
    #[offset(32)]
    #[byte_size(24)]
    pub app_data: Value<Vec<Value<Vec<u8>>>>,
    #[offset(56)]
    #[byte_size(24)]
    pub com_data: Value<Vec<Value<Vec<u8>>>>,
    #[offset(80)]
    #[byte_size(24)]
    pub quant: Value<Vec<brunsli_JPEGQuantTable>>,
    #[offset(104)]
    #[byte_size(24)]
    pub huffman_code: Value<Vec<brunsli_JPEGHuffmanCode>>,
    #[offset(128)]
    #[byte_size(24)]
    pub components: Value<Vec<brunsli_JPEGComponent>>,
    #[offset(152)]
    #[byte_size(24)]
    pub scan_info: Value<Vec<brunsli_JPEGScanInfo>>,
    #[offset(176)]
    #[byte_size(24)]
    pub marker_order: Value<Vec<u8>>,
    #[offset(200)]
    #[byte_size(24)]
    pub inter_marker_data: Value<Vec<Value<Vec<u8>>>>,
    #[offset(224)]
    #[byte_size(24)]
    pub tail_data: Value<Vec<u8>>,
    #[offset(248)]
    #[byte_size(8)]
    pub original_jpg: Ptr<u8>,
    #[offset(256)]
    pub original_jpg_size: usize,
    #[offset(264)]
    #[byte_size(4)]
    pub error: brunsli_JPEGReadError,
    #[offset(268)]
    pub has_zero_padding_bit: bool,
    #[offset(272)]
    #[byte_size(24)]
    pub padding_bits: Value<Vec<i32>>,
}
impl brunsli_JPEGData {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            version: 2,
            max_h_samp_factor: 1,
            max_v_samp_factor: 1,
            MCU_rows: 0,
            MCU_cols: 0,
            restart_interval: 0,
            app_data: Rc::new(RefCell::new(Vec::new())),
            com_data: Rc::new(RefCell::new(Vec::new())),
            quant: Rc::new(RefCell::new(Vec::new())),
            huffman_code: Rc::new(RefCell::new(Vec::new())),
            components: Rc::new(RefCell::new(Vec::new())),
            scan_info: Rc::new(RefCell::new(Vec::new())),
            marker_order: Rc::new(RefCell::new(Vec::new())),
            inter_marker_data: Rc::new(RefCell::new(Vec::new())),
            tail_data: Rc::new(RefCell::new(Vec::new())),
            original_jpg: Ptr::<u8>::null(),
            original_jpg_size: 0_usize,
            error: brunsli_JPEGReadError_OK,
            has_zero_padding_bit: false,
            padding_bits: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl Default for brunsli_JPEGData {
    fn default() -> Self {
        { brunsli_JPEGData::new() }
    }
}
pub fn JPEGDataIs420_15(jpg: Ptr<brunsli_JPEGData>) -> bool {
    return ((((((((((*jpg.with(|__s| __s.components.clone()).borrow()).len() == 3_usize)
        && (jpg.with(|__s| __s.max_h_samp_factor) == 2))
        && (jpg.with(|__s| __s.max_v_samp_factor) == 2))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 2))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 2))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                1_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                1_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                2_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                2_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 1));
}
pub fn JPEGDataIs444_16(jpg: Ptr<brunsli_JPEGData>) -> bool {
    return ((((((((((*jpg.with(|__s| __s.components.clone()).borrow()).len() == 3_usize)
        && (jpg.with(|__s| __s.max_h_samp_factor) == 1))
        && (jpg.with(|__s| __s.max_v_samp_factor) == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                1_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                1_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                2_usize
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } == 1))
        && ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                2_usize
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } == 1));
}
pub fn PaddingBitsLimit_17(jpg: Ptr<brunsli_JPEGData>) -> u64 {
    let mut num_blocks: u64 = (((jpg.with(|__s| __s.width) as u64).wrapping_add(15_u64)) >> 3_u32)
        .wrapping_mul((((jpg.with(|__s| __s.height) as u64).wrapping_add(15_u64)) >> 3_u32));
    return (((7_u64).wrapping_mul(num_blocks))
        .wrapping_mul(((*jpg.with(|__s| __s.components.clone()).borrow()).len() as u64)))
    .wrapping_add(256_u64);
}
thread_local!(
    pub static kBrunsliMaxNumBlocks_18: Value<usize> =
        Rc::new(RefCell::new(((1_u64 << 21) as usize)));
);
thread_local!(
    pub static kBrunsliMaxDCAbsVal_19: Value<i32> = Rc::new(RefCell::new(2054));
);
thread_local!(
    pub static kMaxContextMapAlphabetSize_20: Value<usize> = Rc::new(RefCell::new(272_usize));
);
thread_local!(
    pub static kHuffmanTableBits_21: Value<u32> = Rc::new(RefCell::new(8_u32));
);
thread_local!(
    pub static kMaxHuffmanBits_22: Value<usize> = Rc::new(RefCell::new(15_usize));
);
thread_local!(
    pub static kBrunsliShortMarkerLimit_23: Value<i32> = Rc::new(RefCell::new((64 + (3 * 256))));
);
thread_local!(
    pub static kBrunsliMultibyteMarkerLimit_24: Value<i32> = Rc::new(RefCell::new(1024));
);
thread_local!(
    pub static kBrunsliWiringTypeVarint_25: Value<u8> = Rc::new(RefCell::new(0_u8));
);
thread_local!(
    pub static kBrunsliWiringTypeLengthDelimited_26: Value<u8> = Rc::new(RefCell::new(2_u8));
);
thread_local!(
    pub static kBrunsliMaxSampling_27: Value<i32> = Rc::new(RefCell::new(15));
);
pub fn ValueMarker_28(mut tag: u8) -> u8 {
    return ((((tag as i32) << 3) | (kBrunsliWiringTypeVarint_25.with(|rc| *rc.borrow()) as i32))
        as u8);
}
pub fn SectionMarker_29(mut tag: u8) -> u8 {
    return ((((tag as i32) << 3)
        | (kBrunsliWiringTypeLengthDelimited_26.with(|rc| *rc.borrow()) as i32))
        as u8);
}
thread_local!(
    pub static kBrunsliSignatureTag_30: Value<u8> = Rc::new(RefCell::new(1_u8));
);
thread_local!(
    pub static kBrunsliHeaderTag_31: Value<u8> = Rc::new(RefCell::new(2_u8));
);
thread_local!(
    pub static kBrunsliMetaDataTag_32: Value<u8> = Rc::new(RefCell::new(3_u8));
);
thread_local!(
    pub static kBrunsliJPEGInternalsTag_33: Value<u8> = Rc::new(RefCell::new(4_u8));
);
thread_local!(
    pub static kBrunsliQuantDataTag_34: Value<u8> = Rc::new(RefCell::new(5_u8));
);
thread_local!(
    pub static kBrunsliHistogramDataTag_35: Value<u8> = Rc::new(RefCell::new(6_u8));
);
thread_local!(
    pub static kBrunsliDCDataTag_36: Value<u8> = Rc::new(RefCell::new(7_u8));
);
thread_local!(
    pub static kBrunsliACDataTag_37: Value<u8> = Rc::new(RefCell::new(8_u8));
);
thread_local!(
    pub static kBrunsliOriginalJpgTag_38: Value<u8> = Rc::new(RefCell::new(9_u8));
);
thread_local!(
    pub static kBrunsliHeaderWidthTag_39: Value<u8> = Rc::new(RefCell::new(1_u8));
);
thread_local!(
    pub static kBrunsliHeaderHeightTag_40: Value<u8> = Rc::new(RefCell::new(2_u8));
);
thread_local!(
    pub static kBrunsliHeaderVersionCompTag_41: Value<u8> = Rc::new(RefCell::new(3_u8));
);
thread_local!(
    pub static kBrunsliHeaderSubsamplingTag_42: Value<u8> = Rc::new(RefCell::new(4_u8));
);
thread_local!(
    pub static kBrunsliSignatureSize_43: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!(
    pub static kMaxApp0Densities_45: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
    pub static kApp0Densities_46: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        1_u16, 72_u16, 96_u16, 100_u16, 150_u16, 180_u16, 240_u16, 300_u16,
    ])));
);
thread_local!(
    pub static kNumStockQuantTables_47: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kStockQuantizationTables_48: Value<Box<[Value<Box<[Value<Box<[u8]>>]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                Rc::new(RefCell::new(Box::new([
                    3_u8, 2_u8, 2_u8, 3_u8, 5_u8, 8_u8, 10_u8, 12_u8, 2_u8, 2_u8, 3_u8, 4_u8, 5_u8,
                    12_u8, 12_u8, 11_u8, 3_u8, 3_u8, 3_u8, 5_u8, 8_u8, 11_u8, 14_u8, 11_u8, 3_u8,
                    3_u8, 4_u8, 6_u8, 10_u8, 17_u8, 16_u8, 12_u8, 4_u8, 4_u8, 7_u8, 11_u8, 14_u8,
                    22_u8, 21_u8, 15_u8, 5_u8, 7_u8, 11_u8, 13_u8, 16_u8, 21_u8, 23_u8, 18_u8,
                    10_u8, 13_u8, 16_u8, 17_u8, 21_u8, 24_u8, 24_u8, 20_u8, 14_u8, 18_u8, 19_u8,
                    20_u8, 22_u8, 20_u8, 21_u8, 20_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    8_u8, 6_u8, 5_u8, 8_u8, 12_u8, 20_u8, 26_u8, 31_u8, 6_u8, 6_u8, 7_u8, 10_u8,
                    13_u8, 29_u8, 30_u8, 28_u8, 7_u8, 7_u8, 8_u8, 12_u8, 20_u8, 29_u8, 35_u8,
                    28_u8, 7_u8, 9_u8, 11_u8, 15_u8, 26_u8, 44_u8, 40_u8, 31_u8, 9_u8, 11_u8,
                    19_u8, 28_u8, 34_u8, 55_u8, 52_u8, 39_u8, 12_u8, 18_u8, 28_u8, 32_u8, 41_u8,
                    52_u8, 57_u8, 46_u8, 25_u8, 32_u8, 39_u8, 44_u8, 52_u8, 61_u8, 60_u8, 51_u8,
                    36_u8, 46_u8, 48_u8, 49_u8, 56_u8, 50_u8, 52_u8, 50_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    6_u8, 4_u8, 4_u8, 6_u8, 10_u8, 16_u8, 20_u8, 24_u8, 5_u8, 5_u8, 6_u8, 8_u8,
                    10_u8, 23_u8, 24_u8, 22_u8, 6_u8, 5_u8, 6_u8, 10_u8, 16_u8, 23_u8, 28_u8,
                    22_u8, 6_u8, 7_u8, 9_u8, 12_u8, 20_u8, 35_u8, 32_u8, 25_u8, 7_u8, 9_u8, 15_u8,
                    22_u8, 27_u8, 44_u8, 41_u8, 31_u8, 10_u8, 14_u8, 22_u8, 26_u8, 32_u8, 42_u8,
                    45_u8, 37_u8, 20_u8, 26_u8, 31_u8, 35_u8, 41_u8, 48_u8, 48_u8, 40_u8, 29_u8,
                    37_u8, 38_u8, 39_u8, 45_u8, 40_u8, 41_u8, 40_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    5_u8, 3_u8, 3_u8, 5_u8, 7_u8, 12_u8, 15_u8, 18_u8, 4_u8, 4_u8, 4_u8, 6_u8,
                    8_u8, 17_u8, 18_u8, 17_u8, 4_u8, 4_u8, 5_u8, 7_u8, 12_u8, 17_u8, 21_u8, 17_u8,
                    4_u8, 5_u8, 7_u8, 9_u8, 15_u8, 26_u8, 24_u8, 19_u8, 5_u8, 7_u8, 11_u8, 17_u8,
                    20_u8, 33_u8, 31_u8, 23_u8, 7_u8, 11_u8, 17_u8, 19_u8, 24_u8, 31_u8, 34_u8,
                    28_u8, 15_u8, 19_u8, 23_u8, 26_u8, 31_u8, 36_u8, 36_u8, 30_u8, 22_u8, 28_u8,
                    29_u8, 29_u8, 34_u8, 30_u8, 31_u8, 30_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    2_u8, 1_u8, 1_u8, 2_u8, 2_u8, 4_u8, 5_u8, 6_u8, 1_u8, 1_u8, 1_u8, 2_u8, 3_u8,
                    6_u8, 6_u8, 6_u8, 1_u8, 1_u8, 2_u8, 2_u8, 4_u8, 6_u8, 7_u8, 6_u8, 1_u8, 2_u8,
                    2_u8, 3_u8, 5_u8, 9_u8, 8_u8, 6_u8, 2_u8, 2_u8, 4_u8, 6_u8, 7_u8, 11_u8, 10_u8,
                    8_u8, 2_u8, 4_u8, 6_u8, 6_u8, 8_u8, 10_u8, 11_u8, 9_u8, 5_u8, 6_u8, 8_u8, 9_u8,
                    10_u8, 12_u8, 12_u8, 10_u8, 7_u8, 9_u8, 10_u8, 10_u8, 11_u8, 10_u8, 10_u8,
                    10_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8,
                    3_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 3_u8, 1_u8, 1_u8, 1_u8, 2_u8,
                    2_u8, 3_u8, 3_u8, 3_u8, 1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 3_u8, 3_u8, 3_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    10_u8, 7_u8, 6_u8, 10_u8, 14_u8, 24_u8, 31_u8, 37_u8, 7_u8, 7_u8, 8_u8, 11_u8,
                    16_u8, 35_u8, 36_u8, 33_u8, 8_u8, 8_u8, 10_u8, 14_u8, 24_u8, 34_u8, 41_u8,
                    34_u8, 8_u8, 10_u8, 13_u8, 17_u8, 31_u8, 52_u8, 48_u8, 37_u8, 11_u8, 13_u8,
                    22_u8, 34_u8, 41_u8, 65_u8, 62_u8, 46_u8, 14_u8, 21_u8, 33_u8, 38_u8, 49_u8,
                    62_u8, 68_u8, 55_u8, 29_u8, 38_u8, 47_u8, 52_u8, 62_u8, 73_u8, 72_u8, 61_u8,
                    43_u8, 55_u8, 57_u8, 59_u8, 67_u8, 60_u8, 62_u8, 59_u8,
                ]))),
            ]))),
            Rc::new(RefCell::new(Box::new([
                Rc::new(RefCell::new(Box::new([
                    9_u8, 9_u8, 9_u8, 12_u8, 11_u8, 12_u8, 24_u8, 13_u8, 13_u8, 24_u8, 50_u8,
                    33_u8, 28_u8, 33_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    3_u8, 4_u8, 5_u8, 9_u8, 20_u8, 20_u8, 20_u8, 20_u8, 4_u8, 4_u8, 5_u8, 13_u8,
                    20_u8, 20_u8, 20_u8, 20_u8, 5_u8, 5_u8, 11_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                    20_u8, 9_u8, 13_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                    20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                    20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                    20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    9_u8, 9_u8, 12_u8, 24_u8, 50_u8, 50_u8, 50_u8, 50_u8, 9_u8, 11_u8, 13_u8,
                    33_u8, 50_u8, 50_u8, 50_u8, 50_u8, 12_u8, 13_u8, 28_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 24_u8, 33_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                    50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    5_u8, 5_u8, 7_u8, 14_u8, 30_u8, 30_u8, 30_u8, 30_u8, 5_u8, 6_u8, 8_u8, 20_u8,
                    30_u8, 30_u8, 30_u8, 30_u8, 7_u8, 8_u8, 17_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                    30_u8, 14_u8, 20_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                    30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                    30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                    30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    7_u8, 7_u8, 10_u8, 19_u8, 40_u8, 40_u8, 40_u8, 40_u8, 7_u8, 8_u8, 10_u8, 26_u8,
                    40_u8, 40_u8, 40_u8, 40_u8, 10_u8, 10_u8, 22_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                    40_u8, 19_u8, 26_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                    40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                    40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                    40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                    1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    2_u8, 2_u8, 2_u8, 5_u8, 10_u8, 10_u8, 10_u8, 10_u8, 2_u8, 2_u8, 3_u8, 7_u8,
                    10_u8, 10_u8, 10_u8, 10_u8, 2_u8, 3_u8, 6_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                    10_u8, 5_u8, 7_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                    10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                    10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                    10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                ]))),
                Rc::new(RefCell::new(Box::new([
                    10_u8, 11_u8, 14_u8, 28_u8, 59_u8, 59_u8, 59_u8, 59_u8, 11_u8, 13_u8, 16_u8,
                    40_u8, 59_u8, 59_u8, 59_u8, 59_u8, 14_u8, 16_u8, 34_u8, 59_u8, 59_u8, 59_u8,
                    59_u8, 59_u8, 28_u8, 40_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                    59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                    59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                    59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                ]))),
            ]))),
        ])));
);
thread_local!(
    pub static kComponentIds123_49: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kComponentIdsGray_50: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kComponentIdsRGB_51: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static kComponentIdsCustom_52: Value<i32> = Rc::new(RefCell::new(3));
);
thread_local!(
    pub static kNumStockDCHuffmanCodes_53: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static kStockDCHuffmanCodeCounts_54: Value<Box<[Value<Box<[i32]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0, 1, 5, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0,
            ]))),
        ])));
);
thread_local!(
    pub static kStockDCHuffmanCodeValues_55: Value<Box<[Value<Box<[i32]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 256,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 256,
            ]))),
        ])));
);
thread_local!(
    pub static kNumStockACHuffmanCodes_56: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static kStockACHuffmanCodeCounts_57: Value<Box<[Value<Box<[i32]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 126,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 120,
            ]))),
        ])));
);
thread_local!(
    pub static kStockACHuffmanCodeTotalCount_58: Value<i32> = Rc::new(RefCell::new(163));
);
thread_local!(
    pub static kStockACHuffmanCodeValues_59: Value<Box<[Value<Box<[i32]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50, 129, 145,
                161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130, 9, 10, 22, 23,
                24, 25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71,
                72, 73, 74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106,
                115, 116, 117, 118, 119, 120, 121, 122, 131, 132, 133, 134, 135, 136, 137, 138,
                146, 147, 148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168,
                169, 170, 178, 179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198,
                199, 200, 201, 202, 210, 211, 212, 213, 214, 215, 216, 217, 218, 225, 226, 227,
                228, 229, 230, 231, 232, 233, 234, 241, 242, 243, 244, 245, 246, 247, 248, 249,
                250, 256,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0, 1, 2, 3, 17, 4, 5, 33, 49, 6, 18, 65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20,
                66, 145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114, 209, 10, 22, 36, 52, 225,
                37, 241, 23, 24, 25, 26, 38, 39, 40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69,
                70, 71, 72, 73, 74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104,
                105, 106, 115, 116, 117, 118, 119, 120, 121, 122, 130, 131, 132, 133, 134, 135,
                136, 137, 138, 146, 147, 148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165,
                166, 167, 168, 169, 170, 178, 179, 180, 181, 182, 183, 184, 185, 186, 194, 195,
                196, 197, 198, 199, 200, 201, 202, 210, 211, 212, 213, 214, 215, 216, 217, 218,
                226, 227, 228, 229, 230, 231, 232, 233, 234, 242, 243, 244, 245, 246, 247, 248,
                249, 250, 256,
            ]))),
        ])));
);
thread_local!(
    pub static kDefaultDCValues_60: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
        14_u8, 15_u8,
    ])));
);
thread_local!(
    pub static kDefaultACValues_61: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        1_u8, 0_u8, 2_u8, 3_u8, 17_u8, 4_u8, 5_u8, 33_u8, 18_u8, 49_u8, 65_u8, 6_u8, 81_u8, 19_u8,
        97_u8, 7_u8, 34_u8, 113_u8, 50_u8, 129_u8, 20_u8, 145_u8, 161_u8, 8_u8, 35_u8, 66_u8,
        177_u8, 193_u8, 21_u8, 82_u8, 209_u8, 240_u8, 36_u8, 51_u8, 98_u8, 114_u8, 9_u8, 130_u8,
        10_u8, 22_u8, 52_u8, 225_u8, 23_u8, 37_u8, 241_u8, 24_u8, 25_u8, 26_u8, 38_u8, 39_u8,
        40_u8, 41_u8, 42_u8, 53_u8, 54_u8, 55_u8, 56_u8, 57_u8, 58_u8, 67_u8, 68_u8, 69_u8, 70_u8,
        71_u8, 72_u8, 73_u8, 74_u8, 83_u8, 84_u8, 85_u8, 86_u8, 87_u8, 88_u8, 89_u8, 90_u8, 99_u8,
        100_u8, 101_u8, 102_u8, 103_u8, 104_u8, 105_u8, 106_u8, 115_u8, 116_u8, 117_u8, 118_u8,
        119_u8, 120_u8, 121_u8, 122_u8, 131_u8, 132_u8, 133_u8, 134_u8, 135_u8, 136_u8, 137_u8,
        138_u8, 146_u8, 147_u8, 148_u8, 149_u8, 150_u8, 151_u8, 152_u8, 153_u8, 154_u8, 162_u8,
        163_u8, 164_u8, 165_u8, 166_u8, 167_u8, 168_u8, 169_u8, 170_u8, 178_u8, 179_u8, 180_u8,
        181_u8, 182_u8, 183_u8, 184_u8, 185_u8, 186_u8, 194_u8, 195_u8, 196_u8, 197_u8, 198_u8,
        199_u8, 200_u8, 201_u8, 202_u8, 210_u8, 211_u8, 212_u8, 213_u8, 214_u8, 215_u8, 216_u8,
        217_u8, 218_u8, 226_u8, 227_u8, 228_u8, 229_u8, 230_u8, 231_u8, 232_u8, 233_u8, 234_u8,
        242_u8, 243_u8, 244_u8, 245_u8, 246_u8, 247_u8, 248_u8, 249_u8, 250_u8, 16_u8, 32_u8,
        48_u8, 64_u8, 80_u8, 96_u8, 112_u8, 128_u8, 144_u8, 160_u8, 176_u8, 192_u8, 208_u8, 11_u8,
        12_u8, 13_u8, 14_u8, 15_u8, 27_u8, 28_u8, 29_u8, 30_u8, 31_u8, 43_u8, 44_u8, 45_u8, 46_u8,
        47_u8, 59_u8, 60_u8, 61_u8, 62_u8, 63_u8, 75_u8, 76_u8, 77_u8, 78_u8, 79_u8, 91_u8, 92_u8,
        93_u8, 94_u8, 95_u8, 107_u8, 108_u8, 109_u8, 110_u8, 111_u8, 123_u8, 124_u8, 125_u8,
        126_u8, 127_u8, 139_u8, 140_u8, 141_u8, 142_u8, 143_u8, 155_u8, 156_u8, 157_u8, 158_u8,
        159_u8, 171_u8, 172_u8, 173_u8, 174_u8, 175_u8, 187_u8, 188_u8, 189_u8, 190_u8, 191_u8,
        203_u8, 204_u8, 205_u8, 206_u8, 207_u8, 219_u8, 220_u8, 221_u8, 222_u8, 223_u8, 224_u8,
        235_u8, 236_u8, 237_u8, 238_u8, 239_u8, 251_u8, 252_u8, 253_u8, 254_u8, 255_u8,
    ])));
);
thread_local!(
    pub static kBrunsliSignature_44: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ({ SectionMarker_29(kBrunsliSignatureTag_30.with(|rc| *rc.borrow())) }),
        4_u8,
        (('B' as i8) as u8),
        210_u8,
        213_u8,
        (('N' as i8) as u8),
    ])));
);
thread_local!(
    pub static AppData_0xe0_62: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        224_u8,
        0_u8,
        16_u8,
        (('J' as i8) as u8),
        (('F' as i8) as u8),
        (('I' as i8) as u8),
        (('F' as i8) as u8),
        0_u8,
        1_u8,
        1_u8,
        0_u8,
        0_u8,
        1_u8,
        0_u8,
        1_u8,
        0_u8,
        0_u8,
    ])));
);
thread_local!(
    pub static AppData_0xec_64: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        236_u8,
        0_u8,
        17_u8,
        (('D' as i8) as u8),
        (('u' as i8) as u8),
        (('c' as i8) as u8),
        (('k' as i8) as u8),
        (('y' as i8) as u8),
        0_u8,
        1_u8,
        0_u8,
        4_u8,
        0_u8,
        0_u8,
        0_u8,
        100_u8,
        0_u8,
        0_u8,
    ])));
);
thread_local!(
    pub static AppData_0xee_65: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        238_u8,
        0_u8,
        14_u8,
        (('A' as i8) as u8),
        (('d' as i8) as u8),
        (('o' as i8) as u8),
        (('b' as i8) as u8),
        (('e' as i8) as u8),
        0_u8,
        100_u8,
        0_u8,
        0_u8,
        0_u8,
        0_u8,
        1_u8,
    ])));
);
thread_local!(
    pub static AppData_0xe2_63: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        226_u8, 12_u8, 88_u8, 73_u8, 67_u8, 67_u8, 95_u8, 80_u8, 82_u8, 79_u8, 70_u8, 73_u8, 76_u8,
        69_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8, 12_u8, 72_u8, 76_u8, 105_u8, 110_u8, 111_u8, 2_u8,
        16_u8, 0_u8, 0_u8, 109_u8, 110_u8, 116_u8, 114_u8, 82_u8, 71_u8, 66_u8, 32_u8, 88_u8,
        89_u8, 90_u8, 32_u8, 7_u8, 206_u8, 0_u8, 2_u8, 0_u8, 9_u8, 0_u8, 6_u8, 0_u8, 49_u8, 0_u8,
        0_u8, 97_u8, 99_u8, 115_u8, 112_u8, 77_u8, 83_u8, 70_u8, 84_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        73_u8, 69_u8, 67_u8, 32_u8, 115_u8, 82_u8, 71_u8, 66_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 246_u8, 214_u8, 0_u8, 1_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 211_u8, 45_u8, 72_u8, 80_u8, 32_u8, 32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 17_u8, 99_u8,
        112_u8, 114_u8, 116_u8, 0_u8, 0_u8, 1_u8, 80_u8, 0_u8, 0_u8, 0_u8, 51_u8, 100_u8, 101_u8,
        115_u8, 99_u8, 0_u8, 0_u8, 1_u8, 132_u8, 0_u8, 0_u8, 0_u8, 108_u8, 119_u8, 116_u8, 112_u8,
        116_u8, 0_u8, 0_u8, 1_u8, 240_u8, 0_u8, 0_u8, 0_u8, 20_u8, 98_u8, 107_u8, 112_u8, 116_u8,
        0_u8, 0_u8, 2_u8, 4_u8, 0_u8, 0_u8, 0_u8, 20_u8, 114_u8, 88_u8, 89_u8, 90_u8, 0_u8, 0_u8,
        2_u8, 24_u8, 0_u8, 0_u8, 0_u8, 20_u8, 103_u8, 88_u8, 89_u8, 90_u8, 0_u8, 0_u8, 2_u8, 44_u8,
        0_u8, 0_u8, 0_u8, 20_u8, 98_u8, 88_u8, 89_u8, 90_u8, 0_u8, 0_u8, 2_u8, 64_u8, 0_u8, 0_u8,
        0_u8, 20_u8, 100_u8, 109_u8, 110_u8, 100_u8, 0_u8, 0_u8, 2_u8, 84_u8, 0_u8, 0_u8, 0_u8,
        112_u8, 100_u8, 109_u8, 100_u8, 100_u8, 0_u8, 0_u8, 2_u8, 196_u8, 0_u8, 0_u8, 0_u8, 136_u8,
        118_u8, 117_u8, 101_u8, 100_u8, 0_u8, 0_u8, 3_u8, 76_u8, 0_u8, 0_u8, 0_u8, 134_u8, 118_u8,
        105_u8, 101_u8, 119_u8, 0_u8, 0_u8, 3_u8, 212_u8, 0_u8, 0_u8, 0_u8, 36_u8, 108_u8, 117_u8,
        109_u8, 105_u8, 0_u8, 0_u8, 3_u8, 248_u8, 0_u8, 0_u8, 0_u8, 20_u8, 109_u8, 101_u8, 97_u8,
        115_u8, 0_u8, 0_u8, 4_u8, 12_u8, 0_u8, 0_u8, 0_u8, 36_u8, 116_u8, 101_u8, 99_u8, 104_u8,
        0_u8, 0_u8, 4_u8, 48_u8, 0_u8, 0_u8, 0_u8, 12_u8, 114_u8, 84_u8, 82_u8, 67_u8, 0_u8, 0_u8,
        4_u8, 60_u8, 0_u8, 0_u8, 8_u8, 12_u8, 103_u8, 84_u8, 82_u8, 67_u8, 0_u8, 0_u8, 4_u8, 60_u8,
        0_u8, 0_u8, 8_u8, 12_u8, 98_u8, 84_u8, 82_u8, 67_u8, 0_u8, 0_u8, 4_u8, 60_u8, 0_u8, 0_u8,
        8_u8, 12_u8, 116_u8, 101_u8, 120_u8, 116_u8, 0_u8, 0_u8, 0_u8, 0_u8, 67_u8, 111_u8, 112_u8,
        121_u8, 114_u8, 105_u8, 103_u8, 104_u8, 116_u8, 32_u8, 40_u8, 99_u8, 41_u8, 32_u8, 49_u8,
        57_u8, 57_u8, 56_u8, 32_u8, 72_u8, 101_u8, 119_u8, 108_u8, 101_u8, 116_u8, 116_u8, 45_u8,
        80_u8, 97_u8, 99_u8, 107_u8, 97_u8, 114_u8, 100_u8, 32_u8, 67_u8, 111_u8, 109_u8, 112_u8,
        97_u8, 110_u8, 121_u8, 0_u8, 0_u8, 100_u8, 101_u8, 115_u8, 99_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 18_u8, 115_u8, 82_u8, 71_u8, 66_u8, 32_u8, 73_u8, 69_u8, 67_u8, 54_u8,
        49_u8, 57_u8, 54_u8, 54_u8, 45_u8, 50_u8, 46_u8, 49_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 18_u8, 115_u8, 82_u8, 71_u8, 66_u8, 32_u8, 73_u8, 69_u8,
        67_u8, 54_u8, 49_u8, 57_u8, 54_u8, 54_u8, 45_u8, 50_u8, 46_u8, 49_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 88_u8, 89_u8, 90_u8, 32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 243_u8, 81_u8,
        0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 22_u8, 204_u8, 88_u8, 89_u8, 90_u8, 32_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 88_u8,
        89_u8, 90_u8, 32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 111_u8, 162_u8, 0_u8, 0_u8, 56_u8,
        245_u8, 0_u8, 0_u8, 3_u8, 144_u8, 88_u8, 89_u8, 90_u8, 32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 98_u8, 153_u8, 0_u8, 0_u8, 183_u8, 133_u8, 0_u8, 0_u8, 24_u8, 218_u8, 88_u8, 89_u8,
        90_u8, 32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 36_u8, 160_u8, 0_u8, 0_u8, 15_u8, 132_u8,
        0_u8, 0_u8, 182_u8, 207_u8, 100_u8, 101_u8, 115_u8, 99_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 22_u8, 73_u8, 69_u8, 67_u8, 32_u8, 104_u8, 116_u8, 116_u8, 112_u8, 58_u8,
        47_u8, 47_u8, 119_u8, 119_u8, 119_u8, 46_u8, 105_u8, 101_u8, 99_u8, 46_u8, 99_u8, 104_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 22_u8, 73_u8, 69_u8,
        67_u8, 32_u8, 104_u8, 116_u8, 116_u8, 112_u8, 58_u8, 47_u8, 47_u8, 119_u8, 119_u8, 119_u8,
        46_u8, 105_u8, 101_u8, 99_u8, 46_u8, 99_u8, 104_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 100_u8, 101_u8, 115_u8, 99_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 46_u8, 73_u8, 69_u8, 67_u8, 32_u8, 54_u8, 49_u8,
        57_u8, 54_u8, 54_u8, 45_u8, 50_u8, 46_u8, 49_u8, 32_u8, 68_u8, 101_u8, 102_u8, 97_u8,
        117_u8, 108_u8, 116_u8, 32_u8, 82_u8, 71_u8, 66_u8, 32_u8, 99_u8, 111_u8, 108_u8, 111_u8,
        117_u8, 114_u8, 32_u8, 115_u8, 112_u8, 97_u8, 99_u8, 101_u8, 32_u8, 45_u8, 32_u8, 115_u8,
        82_u8, 71_u8, 66_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        46_u8, 73_u8, 69_u8, 67_u8, 32_u8, 54_u8, 49_u8, 57_u8, 54_u8, 54_u8, 45_u8, 50_u8, 46_u8,
        49_u8, 32_u8, 68_u8, 101_u8, 102_u8, 97_u8, 117_u8, 108_u8, 116_u8, 32_u8, 82_u8, 71_u8,
        66_u8, 32_u8, 99_u8, 111_u8, 108_u8, 111_u8, 117_u8, 114_u8, 32_u8, 115_u8, 112_u8, 97_u8,
        99_u8, 101_u8, 32_u8, 45_u8, 32_u8, 115_u8, 82_u8, 71_u8, 66_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 100_u8, 101_u8, 115_u8, 99_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        44_u8, 82_u8, 101_u8, 102_u8, 101_u8, 114_u8, 101_u8, 110_u8, 99_u8, 101_u8, 32_u8, 86_u8,
        105_u8, 101_u8, 119_u8, 105_u8, 110_u8, 103_u8, 32_u8, 67_u8, 111_u8, 110_u8, 100_u8,
        105_u8, 116_u8, 105_u8, 111_u8, 110_u8, 32_u8, 105_u8, 110_u8, 32_u8, 73_u8, 69_u8, 67_u8,
        54_u8, 49_u8, 57_u8, 54_u8, 54_u8, 45_u8, 50_u8, 46_u8, 49_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 44_u8, 82_u8, 101_u8, 102_u8, 101_u8, 114_u8,
        101_u8, 110_u8, 99_u8, 101_u8, 32_u8, 86_u8, 105_u8, 101_u8, 119_u8, 105_u8, 110_u8,
        103_u8, 32_u8, 67_u8, 111_u8, 110_u8, 100_u8, 105_u8, 116_u8, 105_u8, 111_u8, 110_u8,
        32_u8, 105_u8, 110_u8, 32_u8, 73_u8, 69_u8, 67_u8, 54_u8, 49_u8, 57_u8, 54_u8, 54_u8,
        45_u8, 50_u8, 46_u8, 49_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 118_u8, 105_u8, 101_u8, 119_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 19_u8, 164_u8, 254_u8,
        0_u8, 20_u8, 95_u8, 46_u8, 0_u8, 16_u8, 207_u8, 20_u8, 0_u8, 3_u8, 237_u8, 204_u8, 0_u8,
        4_u8, 19_u8, 11_u8, 0_u8, 3_u8, 92_u8, 158_u8, 0_u8, 0_u8, 0_u8, 1_u8, 88_u8, 89_u8, 90_u8,
        32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 76_u8, 9_u8, 86_u8, 0_u8, 80_u8, 0_u8, 0_u8, 0_u8,
        87_u8, 31_u8, 231_u8, 109_u8, 101_u8, 97_u8, 115_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 143_u8, 0_u8, 0_u8, 0_u8, 2_u8, 115_u8, 105_u8, 103_u8,
        32_u8, 0_u8, 0_u8, 0_u8, 0_u8, 67_u8, 82_u8, 84_u8, 32_u8, 99_u8, 117_u8, 114_u8, 118_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 4_u8, 0_u8, 0_u8, 0_u8, 0_u8, 5_u8, 0_u8, 10_u8, 0_u8,
        15_u8, 0_u8, 20_u8, 0_u8, 25_u8, 0_u8, 30_u8, 0_u8, 35_u8, 0_u8, 40_u8, 0_u8, 45_u8, 0_u8,
        50_u8, 0_u8, 55_u8, 0_u8, 59_u8, 0_u8, 64_u8, 0_u8, 69_u8, 0_u8, 74_u8, 0_u8, 79_u8, 0_u8,
        84_u8, 0_u8, 89_u8, 0_u8, 94_u8, 0_u8, 99_u8, 0_u8, 104_u8, 0_u8, 109_u8, 0_u8, 114_u8,
        0_u8, 119_u8, 0_u8, 124_u8, 0_u8, 129_u8, 0_u8, 134_u8, 0_u8, 139_u8, 0_u8, 144_u8, 0_u8,
        149_u8, 0_u8, 154_u8, 0_u8, 159_u8, 0_u8, 164_u8, 0_u8, 169_u8, 0_u8, 174_u8, 0_u8, 178_u8,
        0_u8, 183_u8, 0_u8, 188_u8, 0_u8, 193_u8, 0_u8, 198_u8, 0_u8, 203_u8, 0_u8, 208_u8, 0_u8,
        213_u8, 0_u8, 219_u8, 0_u8, 224_u8, 0_u8, 229_u8, 0_u8, 235_u8, 0_u8, 240_u8, 0_u8, 246_u8,
        0_u8, 251_u8, 1_u8, 1_u8, 1_u8, 7_u8, 1_u8, 13_u8, 1_u8, 19_u8, 1_u8, 25_u8, 1_u8, 31_u8,
        1_u8, 37_u8, 1_u8, 43_u8, 1_u8, 50_u8, 1_u8, 56_u8, 1_u8, 62_u8, 1_u8, 69_u8, 1_u8, 76_u8,
        1_u8, 82_u8, 1_u8, 89_u8, 1_u8, 96_u8, 1_u8, 103_u8, 1_u8, 110_u8, 1_u8, 117_u8, 1_u8,
        124_u8, 1_u8, 131_u8, 1_u8, 139_u8, 1_u8, 146_u8, 1_u8, 154_u8, 1_u8, 161_u8, 1_u8, 169_u8,
        1_u8, 177_u8, 1_u8, 185_u8, 1_u8, 193_u8, 1_u8, 201_u8, 1_u8, 209_u8, 1_u8, 217_u8, 1_u8,
        225_u8, 1_u8, 233_u8, 1_u8, 242_u8, 1_u8, 250_u8, 2_u8, 3_u8, 2_u8, 12_u8, 2_u8, 20_u8,
        2_u8, 29_u8, 2_u8, 38_u8, 2_u8, 47_u8, 2_u8, 56_u8, 2_u8, 65_u8, 2_u8, 75_u8, 2_u8, 84_u8,
        2_u8, 93_u8, 2_u8, 103_u8, 2_u8, 113_u8, 2_u8, 122_u8, 2_u8, 132_u8, 2_u8, 142_u8, 2_u8,
        152_u8, 2_u8, 162_u8, 2_u8, 172_u8, 2_u8, 182_u8, 2_u8, 193_u8, 2_u8, 203_u8, 2_u8, 213_u8,
        2_u8, 224_u8, 2_u8, 235_u8, 2_u8, 245_u8, 3_u8, 0_u8, 3_u8, 11_u8, 3_u8, 22_u8, 3_u8,
        33_u8, 3_u8, 45_u8, 3_u8, 56_u8, 3_u8, 67_u8, 3_u8, 79_u8, 3_u8, 90_u8, 3_u8, 102_u8, 3_u8,
        114_u8, 3_u8, 126_u8, 3_u8, 138_u8, 3_u8, 150_u8, 3_u8, 162_u8, 3_u8, 174_u8, 3_u8, 186_u8,
        3_u8, 199_u8, 3_u8, 211_u8, 3_u8, 224_u8, 3_u8, 236_u8, 3_u8, 249_u8, 4_u8, 6_u8, 4_u8,
        19_u8, 4_u8, 32_u8, 4_u8, 45_u8, 4_u8, 59_u8, 4_u8, 72_u8, 4_u8, 85_u8, 4_u8, 99_u8, 4_u8,
        113_u8, 4_u8, 126_u8, 4_u8, 140_u8, 4_u8, 154_u8, 4_u8, 168_u8, 4_u8, 182_u8, 4_u8, 196_u8,
        4_u8, 211_u8, 4_u8, 225_u8, 4_u8, 240_u8, 4_u8, 254_u8, 5_u8, 13_u8, 5_u8, 28_u8, 5_u8,
        43_u8, 5_u8, 58_u8, 5_u8, 73_u8, 5_u8, 88_u8, 5_u8, 103_u8, 5_u8, 119_u8, 5_u8, 134_u8,
        5_u8, 150_u8, 5_u8, 166_u8, 5_u8, 181_u8, 5_u8, 197_u8, 5_u8, 213_u8, 5_u8, 229_u8, 5_u8,
        246_u8, 6_u8, 6_u8, 6_u8, 22_u8, 6_u8, 39_u8, 6_u8, 55_u8, 6_u8, 72_u8, 6_u8, 89_u8, 6_u8,
        106_u8, 6_u8, 123_u8, 6_u8, 140_u8, 6_u8, 157_u8, 6_u8, 175_u8, 6_u8, 192_u8, 6_u8, 209_u8,
        6_u8, 227_u8, 6_u8, 245_u8, 7_u8, 7_u8, 7_u8, 25_u8, 7_u8, 43_u8, 7_u8, 61_u8, 7_u8, 79_u8,
        7_u8, 97_u8, 7_u8, 116_u8, 7_u8, 134_u8, 7_u8, 153_u8, 7_u8, 172_u8, 7_u8, 191_u8, 7_u8,
        210_u8, 7_u8, 229_u8, 7_u8, 248_u8, 8_u8, 11_u8, 8_u8, 31_u8, 8_u8, 50_u8, 8_u8, 70_u8,
        8_u8, 90_u8, 8_u8, 110_u8, 8_u8, 130_u8, 8_u8, 150_u8, 8_u8, 170_u8, 8_u8, 190_u8, 8_u8,
        210_u8, 8_u8, 231_u8, 8_u8, 251_u8, 9_u8, 16_u8, 9_u8, 37_u8, 9_u8, 58_u8, 9_u8, 79_u8,
        9_u8, 100_u8, 9_u8, 121_u8, 9_u8, 143_u8, 9_u8, 164_u8, 9_u8, 186_u8, 9_u8, 207_u8, 9_u8,
        229_u8, 9_u8, 251_u8, 10_u8, 17_u8, 10_u8, 39_u8, 10_u8, 61_u8, 10_u8, 84_u8, 10_u8,
        106_u8, 10_u8, 129_u8, 10_u8, 152_u8, 10_u8, 174_u8, 10_u8, 197_u8, 10_u8, 220_u8, 10_u8,
        243_u8, 11_u8, 11_u8, 11_u8, 34_u8, 11_u8, 57_u8, 11_u8, 81_u8, 11_u8, 105_u8, 11_u8,
        128_u8, 11_u8, 152_u8, 11_u8, 176_u8, 11_u8, 200_u8, 11_u8, 225_u8, 11_u8, 249_u8, 12_u8,
        18_u8, 12_u8, 42_u8, 12_u8, 67_u8, 12_u8, 92_u8, 12_u8, 117_u8, 12_u8, 142_u8, 12_u8,
        167_u8, 12_u8, 192_u8, 12_u8, 217_u8, 12_u8, 243_u8, 13_u8, 13_u8, 13_u8, 38_u8, 13_u8,
        64_u8, 13_u8, 90_u8, 13_u8, 116_u8, 13_u8, 142_u8, 13_u8, 169_u8, 13_u8, 195_u8, 13_u8,
        222_u8, 13_u8, 248_u8, 14_u8, 19_u8, 14_u8, 46_u8, 14_u8, 73_u8, 14_u8, 100_u8, 14_u8,
        127_u8, 14_u8, 155_u8, 14_u8, 182_u8, 14_u8, 210_u8, 14_u8, 238_u8, 15_u8, 9_u8, 15_u8,
        37_u8, 15_u8, 65_u8, 15_u8, 94_u8, 15_u8, 122_u8, 15_u8, 150_u8, 15_u8, 179_u8, 15_u8,
        207_u8, 15_u8, 236_u8, 16_u8, 9_u8, 16_u8, 38_u8, 16_u8, 67_u8, 16_u8, 97_u8, 16_u8,
        126_u8, 16_u8, 155_u8, 16_u8, 185_u8, 16_u8, 215_u8, 16_u8, 245_u8, 17_u8, 19_u8, 17_u8,
        49_u8, 17_u8, 79_u8, 17_u8, 109_u8, 17_u8, 140_u8, 17_u8, 170_u8, 17_u8, 201_u8, 17_u8,
        232_u8, 18_u8, 7_u8, 18_u8, 38_u8, 18_u8, 69_u8, 18_u8, 100_u8, 18_u8, 132_u8, 18_u8,
        163_u8, 18_u8, 195_u8, 18_u8, 227_u8, 19_u8, 3_u8, 19_u8, 35_u8, 19_u8, 67_u8, 19_u8,
        99_u8, 19_u8, 131_u8, 19_u8, 164_u8, 19_u8, 197_u8, 19_u8, 229_u8, 20_u8, 6_u8, 20_u8,
        39_u8, 20_u8, 73_u8, 20_u8, 106_u8, 20_u8, 139_u8, 20_u8, 173_u8, 20_u8, 206_u8, 20_u8,
        240_u8, 21_u8, 18_u8, 21_u8, 52_u8, 21_u8, 86_u8, 21_u8, 120_u8, 21_u8, 155_u8, 21_u8,
        189_u8, 21_u8, 224_u8, 22_u8, 3_u8, 22_u8, 38_u8, 22_u8, 73_u8, 22_u8, 108_u8, 22_u8,
        143_u8, 22_u8, 178_u8, 22_u8, 214_u8, 22_u8, 250_u8, 23_u8, 29_u8, 23_u8, 65_u8, 23_u8,
        101_u8, 23_u8, 137_u8, 23_u8, 174_u8, 23_u8, 210_u8, 23_u8, 247_u8, 24_u8, 27_u8, 24_u8,
        64_u8, 24_u8, 101_u8, 24_u8, 138_u8, 24_u8, 175_u8, 24_u8, 213_u8, 24_u8, 250_u8, 25_u8,
        32_u8, 25_u8, 69_u8, 25_u8, 107_u8, 25_u8, 145_u8, 25_u8, 183_u8, 25_u8, 221_u8, 26_u8,
        4_u8, 26_u8, 42_u8, 26_u8, 81_u8, 26_u8, 119_u8, 26_u8, 158_u8, 26_u8, 197_u8, 26_u8,
        236_u8, 27_u8, 20_u8, 27_u8, 59_u8, 27_u8, 99_u8, 27_u8, 138_u8, 27_u8, 178_u8, 27_u8,
        218_u8, 28_u8, 2_u8, 28_u8, 42_u8, 28_u8, 82_u8, 28_u8, 123_u8, 28_u8, 163_u8, 28_u8,
        204_u8, 28_u8, 245_u8, 29_u8, 30_u8, 29_u8, 71_u8, 29_u8, 112_u8, 29_u8, 153_u8, 29_u8,
        195_u8, 29_u8, 236_u8, 30_u8, 22_u8, 30_u8, 64_u8, 30_u8, 106_u8, 30_u8, 148_u8, 30_u8,
        190_u8, 30_u8, 233_u8, 31_u8, 19_u8, 31_u8, 62_u8, 31_u8, 105_u8, 31_u8, 148_u8, 31_u8,
        191_u8, 31_u8, 234_u8, 32_u8, 21_u8, 32_u8, 65_u8, 32_u8, 108_u8, 32_u8, 152_u8, 32_u8,
        196_u8, 32_u8, 240_u8, 33_u8, 28_u8, 33_u8, 72_u8, 33_u8, 117_u8, 33_u8, 161_u8, 33_u8,
        206_u8, 33_u8, 251_u8, 34_u8, 39_u8, 34_u8, 85_u8, 34_u8, 130_u8, 34_u8, 175_u8, 34_u8,
        221_u8, 35_u8, 10_u8, 35_u8, 56_u8, 35_u8, 102_u8, 35_u8, 148_u8, 35_u8, 194_u8, 35_u8,
        240_u8, 36_u8, 31_u8, 36_u8, 77_u8, 36_u8, 124_u8, 36_u8, 171_u8, 36_u8, 218_u8, 37_u8,
        9_u8, 37_u8, 56_u8, 37_u8, 104_u8, 37_u8, 151_u8, 37_u8, 199_u8, 37_u8, 247_u8, 38_u8,
        39_u8, 38_u8, 87_u8, 38_u8, 135_u8, 38_u8, 183_u8, 38_u8, 232_u8, 39_u8, 24_u8, 39_u8,
        73_u8, 39_u8, 122_u8, 39_u8, 171_u8, 39_u8, 220_u8, 40_u8, 13_u8, 40_u8, 63_u8, 40_u8,
        113_u8, 40_u8, 162_u8, 40_u8, 212_u8, 41_u8, 6_u8, 41_u8, 56_u8, 41_u8, 107_u8, 41_u8,
        157_u8, 41_u8, 208_u8, 42_u8, 2_u8, 42_u8, 53_u8, 42_u8, 104_u8, 42_u8, 155_u8, 42_u8,
        207_u8, 43_u8, 2_u8, 43_u8, 54_u8, 43_u8, 105_u8, 43_u8, 157_u8, 43_u8, 209_u8, 44_u8,
        5_u8, 44_u8, 57_u8, 44_u8, 110_u8, 44_u8, 162_u8, 44_u8, 215_u8, 45_u8, 12_u8, 45_u8,
        65_u8, 45_u8, 118_u8, 45_u8, 171_u8, 45_u8, 225_u8, 46_u8, 22_u8, 46_u8, 76_u8, 46_u8,
        130_u8, 46_u8, 183_u8, 46_u8, 238_u8, 47_u8, 36_u8, 47_u8, 90_u8, 47_u8, 145_u8, 47_u8,
        199_u8, 47_u8, 254_u8, 48_u8, 53_u8, 48_u8, 108_u8, 48_u8, 164_u8, 48_u8, 219_u8, 49_u8,
        18_u8, 49_u8, 74_u8, 49_u8, 130_u8, 49_u8, 186_u8, 49_u8, 242_u8, 50_u8, 42_u8, 50_u8,
        99_u8, 50_u8, 155_u8, 50_u8, 212_u8, 51_u8, 13_u8, 51_u8, 70_u8, 51_u8, 127_u8, 51_u8,
        184_u8, 51_u8, 241_u8, 52_u8, 43_u8, 52_u8, 101_u8, 52_u8, 158_u8, 52_u8, 216_u8, 53_u8,
        19_u8, 53_u8, 77_u8, 53_u8, 135_u8, 53_u8, 194_u8, 53_u8, 253_u8, 54_u8, 55_u8, 54_u8,
        114_u8, 54_u8, 174_u8, 54_u8, 233_u8, 55_u8, 36_u8, 55_u8, 96_u8, 55_u8, 156_u8, 55_u8,
        215_u8, 56_u8, 20_u8, 56_u8, 80_u8, 56_u8, 140_u8, 56_u8, 200_u8, 57_u8, 5_u8, 57_u8,
        66_u8, 57_u8, 127_u8, 57_u8, 188_u8, 57_u8, 249_u8, 58_u8, 54_u8, 58_u8, 116_u8, 58_u8,
        178_u8, 58_u8, 239_u8, 59_u8, 45_u8, 59_u8, 107_u8, 59_u8, 170_u8, 59_u8, 232_u8, 60_u8,
        39_u8, 60_u8, 101_u8, 60_u8, 164_u8, 60_u8, 227_u8, 61_u8, 34_u8, 61_u8, 97_u8, 61_u8,
        161_u8, 61_u8, 224_u8, 62_u8, 32_u8, 62_u8, 96_u8, 62_u8, 160_u8, 62_u8, 224_u8, 63_u8,
        33_u8, 63_u8, 97_u8, 63_u8, 162_u8, 63_u8, 226_u8, 64_u8, 35_u8, 64_u8, 100_u8, 64_u8,
        166_u8, 64_u8, 231_u8, 65_u8, 41_u8, 65_u8, 106_u8, 65_u8, 172_u8, 65_u8, 238_u8, 66_u8,
        48_u8, 66_u8, 114_u8, 66_u8, 181_u8, 66_u8, 247_u8, 67_u8, 58_u8, 67_u8, 125_u8, 67_u8,
        192_u8, 68_u8, 3_u8, 68_u8, 71_u8, 68_u8, 138_u8, 68_u8, 206_u8, 69_u8, 18_u8, 69_u8,
        85_u8, 69_u8, 154_u8, 69_u8, 222_u8, 70_u8, 34_u8, 70_u8, 103_u8, 70_u8, 171_u8, 70_u8,
        240_u8, 71_u8, 53_u8, 71_u8, 123_u8, 71_u8, 192_u8, 72_u8, 5_u8, 72_u8, 75_u8, 72_u8,
        145_u8, 72_u8, 215_u8, 73_u8, 29_u8, 73_u8, 99_u8, 73_u8, 169_u8, 73_u8, 240_u8, 74_u8,
        55_u8, 74_u8, 125_u8, 74_u8, 196_u8, 75_u8, 12_u8, 75_u8, 83_u8, 75_u8, 154_u8, 75_u8,
        226_u8, 76_u8, 42_u8, 76_u8, 114_u8, 76_u8, 186_u8, 77_u8, 2_u8, 77_u8, 74_u8, 77_u8,
        147_u8, 77_u8, 220_u8, 78_u8, 37_u8, 78_u8, 110_u8, 78_u8, 183_u8, 79_u8, 0_u8, 79_u8,
        73_u8, 79_u8, 147_u8, 79_u8, 221_u8, 80_u8, 39_u8, 80_u8, 113_u8, 80_u8, 187_u8, 81_u8,
        6_u8, 81_u8, 80_u8, 81_u8, 155_u8, 81_u8, 230_u8, 82_u8, 49_u8, 82_u8, 124_u8, 82_u8,
        199_u8, 83_u8, 19_u8, 83_u8, 95_u8, 83_u8, 170_u8, 83_u8, 246_u8, 84_u8, 66_u8, 84_u8,
        143_u8, 84_u8, 219_u8, 85_u8, 40_u8, 85_u8, 117_u8, 85_u8, 194_u8, 86_u8, 15_u8, 86_u8,
        92_u8, 86_u8, 169_u8, 86_u8, 247_u8, 87_u8, 68_u8, 87_u8, 146_u8, 87_u8, 224_u8, 88_u8,
        47_u8, 88_u8, 125_u8, 88_u8, 203_u8, 89_u8, 26_u8, 89_u8, 105_u8, 89_u8, 184_u8, 90_u8,
        7_u8, 90_u8, 86_u8, 90_u8, 166_u8, 90_u8, 245_u8, 91_u8, 69_u8, 91_u8, 149_u8, 91_u8,
        229_u8, 92_u8, 53_u8, 92_u8, 134_u8, 92_u8, 214_u8, 93_u8, 39_u8, 93_u8, 120_u8, 93_u8,
        201_u8, 94_u8, 26_u8, 94_u8, 108_u8, 94_u8, 189_u8, 95_u8, 15_u8, 95_u8, 97_u8, 95_u8,
        179_u8, 96_u8, 5_u8, 96_u8, 87_u8, 96_u8, 170_u8, 96_u8, 252_u8, 97_u8, 79_u8, 97_u8,
        162_u8, 97_u8, 245_u8, 98_u8, 73_u8, 98_u8, 156_u8, 98_u8, 240_u8, 99_u8, 67_u8, 99_u8,
        151_u8, 99_u8, 235_u8, 100_u8, 64_u8, 100_u8, 148_u8, 100_u8, 233_u8, 101_u8, 61_u8,
        101_u8, 146_u8, 101_u8, 231_u8, 102_u8, 61_u8, 102_u8, 146_u8, 102_u8, 232_u8, 103_u8,
        61_u8, 103_u8, 147_u8, 103_u8, 233_u8, 104_u8, 63_u8, 104_u8, 150_u8, 104_u8, 236_u8,
        105_u8, 67_u8, 105_u8, 154_u8, 105_u8, 241_u8, 106_u8, 72_u8, 106_u8, 159_u8, 106_u8,
        247_u8, 107_u8, 79_u8, 107_u8, 167_u8, 107_u8, 255_u8, 108_u8, 87_u8, 108_u8, 175_u8,
        109_u8, 8_u8, 109_u8, 96_u8, 109_u8, 185_u8, 110_u8, 18_u8, 110_u8, 107_u8, 110_u8, 196_u8,
        111_u8, 30_u8, 111_u8, 120_u8, 111_u8, 209_u8, 112_u8, 43_u8, 112_u8, 134_u8, 112_u8,
        224_u8, 113_u8, 58_u8, 113_u8, 149_u8, 113_u8, 240_u8, 114_u8, 75_u8, 114_u8, 166_u8,
        115_u8, 1_u8, 115_u8, 93_u8, 115_u8, 184_u8, 116_u8, 20_u8, 116_u8, 112_u8, 116_u8, 204_u8,
        117_u8, 40_u8, 117_u8, 133_u8, 117_u8, 225_u8, 118_u8, 62_u8, 118_u8, 155_u8, 118_u8,
        248_u8, 119_u8, 86_u8, 119_u8, 179_u8, 120_u8, 17_u8, 120_u8, 110_u8, 120_u8, 204_u8,
        121_u8, 42_u8, 121_u8, 137_u8, 121_u8, 231_u8, 122_u8, 70_u8, 122_u8, 165_u8, 123_u8, 4_u8,
        123_u8, 99_u8, 123_u8, 194_u8, 124_u8, 33_u8, 124_u8, 129_u8, 124_u8, 225_u8, 125_u8,
        65_u8, 125_u8, 161_u8, 126_u8, 1_u8, 126_u8, 98_u8, 126_u8, 194_u8, 127_u8, 35_u8, 127_u8,
        132_u8, 127_u8, 229_u8, 128_u8, 71_u8, 128_u8, 168_u8, 129_u8, 10_u8, 129_u8, 107_u8,
        129_u8, 205_u8, 130_u8, 48_u8, 130_u8, 146_u8, 130_u8, 244_u8, 131_u8, 87_u8, 131_u8,
        186_u8, 132_u8, 29_u8, 132_u8, 128_u8, 132_u8, 227_u8, 133_u8, 71_u8, 133_u8, 171_u8,
        134_u8, 14_u8, 134_u8, 114_u8, 134_u8, 215_u8, 135_u8, 59_u8, 135_u8, 159_u8, 136_u8, 4_u8,
        136_u8, 105_u8, 136_u8, 206_u8, 137_u8, 51_u8, 137_u8, 153_u8, 137_u8, 254_u8, 138_u8,
        100_u8, 138_u8, 202_u8, 139_u8, 48_u8, 139_u8, 150_u8, 139_u8, 252_u8, 140_u8, 99_u8,
        140_u8, 202_u8, 141_u8, 49_u8, 141_u8, 152_u8, 141_u8, 255_u8, 142_u8, 102_u8, 142_u8,
        206_u8, 143_u8, 54_u8, 143_u8, 158_u8, 144_u8, 6_u8, 144_u8, 110_u8, 144_u8, 214_u8,
        145_u8, 63_u8, 145_u8, 168_u8, 146_u8, 17_u8, 146_u8, 122_u8, 146_u8, 227_u8, 147_u8,
        77_u8, 147_u8, 182_u8, 148_u8, 32_u8, 148_u8, 138_u8, 148_u8, 244_u8, 149_u8, 95_u8,
        149_u8, 201_u8, 150_u8, 52_u8, 150_u8, 159_u8, 151_u8, 10_u8, 151_u8, 117_u8, 151_u8,
        224_u8, 152_u8, 76_u8, 152_u8, 184_u8, 153_u8, 36_u8, 153_u8, 144_u8, 153_u8, 252_u8,
        154_u8, 104_u8, 154_u8, 213_u8, 155_u8, 66_u8, 155_u8, 175_u8, 156_u8, 28_u8, 156_u8,
        137_u8, 156_u8, 247_u8, 157_u8, 100_u8, 157_u8, 210_u8, 158_u8, 64_u8, 158_u8, 174_u8,
        159_u8, 29_u8, 159_u8, 139_u8, 159_u8, 250_u8, 160_u8, 105_u8, 160_u8, 216_u8, 161_u8,
        71_u8, 161_u8, 182_u8, 162_u8, 38_u8, 162_u8, 150_u8, 163_u8, 6_u8, 163_u8, 118_u8, 163_u8,
        230_u8, 164_u8, 86_u8, 164_u8, 199_u8, 165_u8, 56_u8, 165_u8, 169_u8, 166_u8, 26_u8,
        166_u8, 139_u8, 166_u8, 253_u8, 167_u8, 110_u8, 167_u8, 224_u8, 168_u8, 82_u8, 168_u8,
        196_u8, 169_u8, 55_u8, 169_u8, 169_u8, 170_u8, 28_u8, 170_u8, 143_u8, 171_u8, 2_u8, 171_u8,
        117_u8, 171_u8, 233_u8, 172_u8, 92_u8, 172_u8, 208_u8, 173_u8, 68_u8, 173_u8, 184_u8,
        174_u8, 45_u8, 174_u8, 161_u8, 175_u8, 22_u8, 175_u8, 139_u8, 176_u8, 0_u8, 176_u8, 117_u8,
        176_u8, 234_u8, 177_u8, 96_u8, 177_u8, 214_u8, 178_u8, 75_u8, 178_u8, 194_u8, 179_u8,
        56_u8, 179_u8, 174_u8, 180_u8, 37_u8, 180_u8, 156_u8, 181_u8, 19_u8, 181_u8, 138_u8,
        182_u8, 1_u8, 182_u8, 121_u8, 182_u8, 240_u8, 183_u8, 104_u8, 183_u8, 224_u8, 184_u8,
        89_u8, 184_u8, 209_u8, 185_u8, 74_u8, 185_u8, 194_u8, 186_u8, 59_u8, 186_u8, 181_u8,
        187_u8, 46_u8, 187_u8, 167_u8, 188_u8, 33_u8, 188_u8, 155_u8, 189_u8, 21_u8, 189_u8,
        143_u8, 190_u8, 10_u8, 190_u8, 132_u8, 190_u8, 255_u8, 191_u8, 122_u8, 191_u8, 245_u8,
        192_u8, 112_u8, 192_u8, 236_u8, 193_u8, 103_u8, 193_u8, 227_u8, 194_u8, 95_u8, 194_u8,
        219_u8, 195_u8, 88_u8, 195_u8, 212_u8, 196_u8, 81_u8, 196_u8, 206_u8, 197_u8, 75_u8,
        197_u8, 200_u8, 198_u8, 70_u8, 198_u8, 195_u8, 199_u8, 65_u8, 199_u8, 191_u8, 200_u8,
        61_u8, 200_u8, 188_u8, 201_u8, 58_u8, 201_u8, 185_u8, 202_u8, 56_u8, 202_u8, 183_u8,
        203_u8, 54_u8, 203_u8, 182_u8, 204_u8, 53_u8, 204_u8, 181_u8, 205_u8, 53_u8, 205_u8,
        181_u8, 206_u8, 54_u8, 206_u8, 182_u8, 207_u8, 55_u8, 207_u8, 184_u8, 208_u8, 57_u8,
        208_u8, 186_u8, 209_u8, 60_u8, 209_u8, 190_u8, 210_u8, 63_u8, 210_u8, 193_u8, 211_u8,
        68_u8, 211_u8, 198_u8, 212_u8, 73_u8, 212_u8, 203_u8, 213_u8, 78_u8, 213_u8, 209_u8,
        214_u8, 85_u8, 214_u8, 216_u8, 215_u8, 92_u8, 215_u8, 224_u8, 216_u8, 100_u8, 216_u8,
        232_u8, 217_u8, 108_u8, 217_u8, 241_u8, 218_u8, 118_u8, 218_u8, 251_u8, 219_u8, 128_u8,
        220_u8, 5_u8, 220_u8, 138_u8, 221_u8, 16_u8, 221_u8, 150_u8, 222_u8, 28_u8, 222_u8, 162_u8,
        223_u8, 41_u8, 223_u8, 175_u8, 224_u8, 54_u8, 224_u8, 189_u8, 225_u8, 68_u8, 225_u8,
        204_u8, 226_u8, 83_u8, 226_u8, 219_u8, 227_u8, 99_u8, 227_u8, 235_u8, 228_u8, 115_u8,
        228_u8, 252_u8, 229_u8, 132_u8, 230_u8, 13_u8, 230_u8, 150_u8, 231_u8, 31_u8, 231_u8,
        169_u8, 232_u8, 50_u8, 232_u8, 188_u8, 233_u8, 70_u8, 233_u8, 208_u8, 234_u8, 91_u8,
        234_u8, 229_u8, 235_u8, 112_u8, 235_u8, 251_u8, 236_u8, 134_u8, 237_u8, 17_u8, 237_u8,
        156_u8, 238_u8, 40_u8, 238_u8, 180_u8, 239_u8, 64_u8, 239_u8, 204_u8, 240_u8, 88_u8,
        240_u8, 229_u8, 241_u8, 114_u8, 241_u8, 255_u8, 242_u8, 140_u8, 243_u8, 25_u8, 243_u8,
        167_u8, 244_u8, 52_u8, 244_u8, 194_u8, 245_u8, 80_u8, 245_u8, 222_u8, 246_u8, 109_u8,
        246_u8, 251_u8, 247_u8, 138_u8, 248_u8, 25_u8, 248_u8, 168_u8, 249_u8, 56_u8, 249_u8,
        199_u8, 250_u8, 87_u8, 250_u8, 231_u8, 251_u8, 119_u8, 252_u8, 7_u8, 252_u8, 152_u8,
        253_u8, 41_u8, 253_u8, 186_u8, 254_u8, 75_u8, 254_u8, 220_u8, 255_u8, 109_u8, 255_u8,
        255_u8,
    ])));
);
pub fn BrunsliUnalignedRead16_66(mut p: AnyPtr) -> u16 {
    let t: Value<u16> = Rc::new(RefCell::new(0_u16));
    {
        ((t.as_pointer()) as Ptr<u16>)
            .to_any()
            .memcpy(&p, ::std::mem::size_of::<u16>() as usize);
        ((t.as_pointer()) as Ptr<u16>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite16_67(mut p: AnyPtr, v: u16) {
    let v: Value<u16> = Rc::new(RefCell::new(v));
    {
        p.memcpy(
            &((v.as_pointer()) as Ptr<u16>).to_any(),
            ::std::mem::size_of::<u16>() as usize,
        );
        (p).clone()
    };
}
pub fn BrunsliUnalignedRead32_68(mut p: AnyPtr) -> u32 {
    let t: Value<u32> = Rc::new(RefCell::new(0_u32));
    {
        ((t.as_pointer()) as Ptr<u32>)
            .to_any()
            .memcpy(&p, ::std::mem::size_of::<u32>() as usize);
        ((t.as_pointer()) as Ptr<u32>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedRead64_69(mut p: AnyPtr) -> u64 {
    let t: Value<u64> = Rc::new(RefCell::new(0_u64));
    {
        ((t.as_pointer()) as Ptr<u64>)
            .to_any()
            .memcpy(&p, ::std::mem::size_of::<u64>() as usize);
        ((t.as_pointer()) as Ptr<u64>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite64_70(mut p: AnyPtr, v: u64) {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    {
        p.memcpy(
            &((v.as_pointer()) as Ptr<u64>).to_any(),
            ::std::mem::size_of::<u64>() as usize,
        );
        (p).clone()
    };
}
pub fn Append_71(mut dst: Ptr<Vec<u8>>, mut begin: Ptr<u8>, mut end: Ptr<u8>) {
    {
        let start_idx = (Ptr::<Vec<u8>>::decay(&(dst)) as Ptr<u8>)
            .to_end()
            .get_offset();
        let count = end.get_offset() - begin.get_offset();
        let temp_vec: Vec<u8> = PtrValueIter::new(&begin, count).collect();
        ((dst).clone() as Ptr<Vec<u8>>).with_mut(|v: &mut Vec<u8>| {
            v.splice(start_idx..start_idx, temp_vec);
        });
        ((dst).clone() as Ptr<Vec<u8>>) + start_idx
    };
}
pub fn Append_72(mut dst: Ptr<Vec<u8>>, mut begin: Ptr<u8>, mut length: usize) {
    ({
        let _begin: Ptr<u8> = (begin).clone();
        let _end: Ptr<u8> = begin.offset((length) as isize);
        Append_71((dst).clone(), _begin, _end)
    });
}
pub fn Append_73(mut dst: Ptr<Vec<u8>>, src: Ptr<Vec<u8>>) {
    ({
        let _begin: Ptr<u8> = (Ptr::<Vec<u8>>::decay(&(src)) as Ptr<u8>);
        let _length: usize = (*src.upgrade().deref()).len();
        Append_72((dst).clone(), _begin, _length)
    });
}
pub fn Log2FloorNonZero_74(mut n: u32) -> i32 {
    return (31 ^ n.leading_zeros() as i32);
}
pub fn BrunsliSuppressUnusedFunctions_75() {
    &((FnPtr::<fn(Ptr<Vec<u8>>, Ptr<Vec<u8>>)>::new(Append_73))
        .cast::<fn(Ptr<Vec<u8>>, Ptr<Vec<u8>>)>());
    &(FnPtr::<fn()>::new(BrunsliSuppressUnusedFunctions_75));
    &(FnPtr::<fn(AnyPtr) -> u16>::new(BrunsliUnalignedRead16_66));
    &(FnPtr::<fn(AnyPtr, u16)>::new(BrunsliUnalignedWrite16_67));
    &(FnPtr::<fn(AnyPtr) -> u32>::new(BrunsliUnalignedRead32_68));
    &(FnPtr::<fn(AnyPtr) -> u64>::new(BrunsliUnalignedRead64_69));
    &(FnPtr::<fn(AnyPtr, u64)>::new(BrunsliUnalignedWrite64_70));
    &(FnPtr::<fn(AnyPtr) -> u16>::new(BrunsliUnalignedRead16_66));
    &(FnPtr::<fn(AnyPtr, u16)>::new(BrunsliUnalignedWrite16_67));
    &(FnPtr::<fn(AnyPtr) -> u32>::new(BrunsliUnalignedRead32_68));
    &(FnPtr::<fn(AnyPtr) -> u64>::new(BrunsliUnalignedRead64_69));
    &(FnPtr::<fn(AnyPtr, u64)>::new(BrunsliUnalignedWrite64_70));
}
thread_local!(
    pub static kNormalizeThreshold_76: Value<u8> = Rc::new(RefCell::new(254_u8));
);
thread_local!(
    pub static kDivLut17_77: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 43690_u16, 32768_u16, 26214_u16, 21845_u16, 18724_u16, 16384_u16,
        14563_u16, 13107_u16, 11915_u16, 10922_u16, 10082_u16, 9362_u16, 8738_u16, 8192_u16,
        7710_u16, 7281_u16, 6898_u16, 6553_u16, 6241_u16, 5957_u16, 5698_u16, 5461_u16, 5242_u16,
        5041_u16, 4854_u16, 4681_u16, 4519_u16, 4369_u16, 4228_u16, 4096_u16, 3971_u16, 3855_u16,
        3744_u16, 3640_u16, 3542_u16, 3449_u16, 3360_u16, 3276_u16, 3196_u16, 3120_u16, 3048_u16,
        2978_u16, 2912_u16, 2849_u16, 2788_u16, 2730_u16, 2674_u16, 2621_u16, 2570_u16, 2520_u16,
        2473_u16, 2427_u16, 2383_u16, 2340_u16, 2299_u16, 2259_u16, 2221_u16, 2184_u16, 2148_u16,
        2114_u16, 2080_u16, 2048_u16, 2016_u16, 1985_u16, 1956_u16, 1927_u16, 1899_u16, 1872_u16,
        1846_u16, 1820_u16, 1795_u16, 1771_u16, 1747_u16, 1724_u16, 1702_u16, 1680_u16, 1659_u16,
        1638_u16, 1618_u16, 1598_u16, 1579_u16, 1560_u16, 1542_u16, 1524_u16, 1506_u16, 1489_u16,
        1472_u16, 1456_u16, 1440_u16, 1424_u16, 1409_u16, 1394_u16, 1379_u16, 1365_u16, 1351_u16,
        1337_u16, 1323_u16, 1310_u16, 1297_u16, 1285_u16, 1272_u16, 1260_u16, 1248_u16, 1236_u16,
        1224_u16, 1213_u16, 1202_u16, 1191_u16, 1180_u16, 1170_u16, 1159_u16, 1149_u16, 1139_u16,
        1129_u16, 1120_u16, 1110_u16, 1101_u16, 1092_u16, 1083_u16, 1074_u16, 1065_u16, 1057_u16,
        1048_u16, 1040_u16, 1032_u16, 1024_u16, 1016_u16, 1008_u16, 1000_u16, 992_u16, 985_u16,
        978_u16, 970_u16, 963_u16, 956_u16, 949_u16, 942_u16, 936_u16, 929_u16, 923_u16, 916_u16,
        910_u16, 903_u16, 897_u16, 891_u16, 885_u16, 879_u16, 873_u16, 868_u16, 862_u16, 856_u16,
        851_u16, 845_u16, 840_u16, 834_u16, 829_u16, 824_u16, 819_u16, 814_u16, 809_u16, 804_u16,
        799_u16, 794_u16, 789_u16, 784_u16, 780_u16, 775_u16, 771_u16, 766_u16, 762_u16, 757_u16,
        753_u16, 748_u16, 744_u16, 740_u16, 736_u16, 732_u16, 728_u16, 724_u16, 720_u16, 716_u16,
        712_u16, 708_u16, 704_u16, 700_u16, 697_u16, 693_u16, 689_u16, 686_u16, 682_u16, 679_u16,
        675_u16, 672_u16, 668_u16, 665_u16, 661_u16, 658_u16, 655_u16, 652_u16, 648_u16, 645_u16,
        642_u16, 639_u16, 636_u16, 633_u16, 630_u16, 627_u16, 624_u16, 621_u16, 618_u16, 615_u16,
        612_u16, 609_u16, 606_u16, 604_u16, 601_u16, 598_u16, 595_u16, 593_u16, 590_u16, 587_u16,
        585_u16, 582_u16, 579_u16, 577_u16, 574_u16, 572_u16, 569_u16, 567_u16, 564_u16, 562_u16,
        560_u16, 557_u16, 555_u16, 553_u16, 550_u16, 548_u16, 546_u16, 543_u16, 541_u16, 539_u16,
        537_u16, 534_u16, 532_u16, 530_u16, 528_u16, 526_u16, 524_u16, 522_u16, 520_u16, 518_u16,
        516_u16,
    ])));
);
pub fn FastDivide_78(mut numerator: u32, mut denominator: u8) -> u8 {
    let mut result: u32 = (((numerator).wrapping_mul(
        (({
            let __idx = (denominator) as usize;
            kDivLut17_77.with(|rc| rc.borrow()[__idx])
        }) as u32),
    )) >> 17);
    if !(result < 256_u32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"context.cc"),
                55,
                Ptr::<i8>::from_string_literal(b"FastDivide"),
            )
        });
        'loop_: while true {}
    };
    return (result as u8);
}
thread_local!(
    pub static kInitProb_80: Value<u8> = Rc::new(RefCell::new(134_u8));
);
thread_local!(
    pub static kInitProbCount_81: Value<u8> = Rc::new(RefCell::new(3_u8));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct brunsli_Prob {
    #[offset(0)]
    prob8: u8,
    #[offset(1)]
    total: u8,
    #[offset(2)]
    count: u16,
}
impl brunsli_Prob {
    pub fn new() -> Self {
        Self {
            prob8: kInitProb_80.with(|rc| *rc.borrow()),
            total: kInitProbCount_81.with(|rc| *rc.borrow()),
            count: (((kInitProb_80.with(|rc| *rc.borrow()) as i32)
                * (kInitProbCount_81.with(|rc| *rc.borrow()) as i32)) as u16),
        }
    }
}
impl Default for brunsli_Prob {
    fn default() -> Self {
        { brunsli_Prob::new() }
    }
}
thread_local!(
    pub static kMaxAverageContext_82: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
    pub static kNumAvrgContexts_83: Value<usize> = Rc::new(RefCell::new(
        (kMaxAverageContext_82.with(|rc| *rc.borrow())).wrapping_add(1_usize),
    ));
);
thread_local!(
    pub static kNumNonZeroBits_84: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!(
    pub static kNumNonZeroTreeSize_85: Value<usize> = Rc::new(RefCell::new(
        ((((1_u32 << kNumNonZeroBits_84.with(|rc| *rc.borrow())) as u32)
            .wrapping_sub((1_u32 as u32))) as usize),
    ));
);
thread_local!(
    pub static kNumNonZeroQuant_86: Value<usize> = Rc::new(RefCell::new(2_usize));
);
thread_local!(
    pub static kNumNonZeroContextMax_87: Value<usize> = Rc::new(RefCell::new(
        (kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
            .wrapping_div(kNumNonZeroQuant_86.with(|rc| *rc.borrow())),
    ));
);
thread_local!(
    pub static kNumNonZeroContextCount_88: Value<usize> = Rc::new(RefCell::new(
        (kNumNonZeroContextMax_87.with(|rc| *rc.borrow())).wrapping_add(1_usize),
    ));
);
thread_local!(
    pub static kNonzeroBuckets_89: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 4_u8, 5_u8, 5_u8, 5_u8, 6_u8, 6_u8, 6_u8, 6_u8, 7_u8, 7_u8,
        7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8,
        8_u8, 8_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8,
        10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
        10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
    ])));
);
thread_local!(
    pub static kNumNonzeroBuckets_90: Value<u8> = Rc::new(RefCell::new(11_u8));
);
thread_local!(
    pub static kNumSchemes_91: Value<i32> = Rc::new(RefCell::new(7));
);
thread_local!(
    pub static kFreqContext_92: Value<Box<[Value<Box<[u8]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 2_u8, 2_u8,
            2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 1_u8, 1_u8, 1_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 1_u8, 1_u8, 2_u8, 2_u8, 2_u8, 3_u8, 3_u8, 3_u8, 3_u8, 4_u8, 4_u8, 4_u8, 4_u8,
            4_u8, 4_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 6_u8, 6_u8, 6_u8, 6_u8,
            6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 7_u8, 7_u8,
            7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8,
            7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 2_u8, 2_u8, 2_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 4_u8, 5_u8, 5_u8, 6_u8, 6_u8, 7_u8, 7_u8, 8_u8, 8_u8,
            8_u8, 8_u8, 9_u8, 9_u8, 9_u8, 9_u8, 10_u8, 10_u8, 10_u8, 10_u8, 11_u8, 11_u8, 11_u8,
            11_u8, 12_u8, 12_u8, 12_u8, 12_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8,
            13_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 15_u8, 15_u8, 15_u8,
            15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8,
            15_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
            14_u8, 15_u8, 16_u8, 16_u8, 17_u8, 17_u8, 18_u8, 18_u8, 19_u8, 19_u8, 20_u8, 20_u8,
            21_u8, 21_u8, 22_u8, 22_u8, 23_u8, 23_u8, 24_u8, 24_u8, 24_u8, 24_u8, 25_u8, 25_u8,
            25_u8, 25_u8, 26_u8, 26_u8, 26_u8, 26_u8, 27_u8, 27_u8, 27_u8, 27_u8, 28_u8, 28_u8,
            28_u8, 28_u8, 29_u8, 29_u8, 29_u8, 29_u8, 30_u8, 30_u8, 30_u8, 30_u8, 31_u8, 31_u8,
            31_u8, 31_u8,
        ]))),
        Rc::new(RefCell::new(Box::new([
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
            14_u8, 15_u8, 16_u8, 17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8,
            26_u8, 27_u8, 28_u8, 29_u8, 30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8,
            38_u8, 39_u8, 40_u8, 41_u8, 42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8,
            50_u8, 51_u8, 52_u8, 53_u8, 54_u8, 55_u8, 56_u8, 57_u8, 58_u8, 59_u8, 60_u8, 61_u8,
            62_u8, 63_u8,
        ]))),
    ])));
);
thread_local!(
    pub static kNumNonzeroContext_93: Value<Box<[Value<Box<[u16]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                0_u16, 1_u16, 1_u16, 2_u16, 2_u16, 2_u16, 3_u16, 3_u16, 3_u16, 3_u16, 4_u16, 4_u16,
                4_u16, 4_u16, 4_u16, 4_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16,
                6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16,
                6_u16, 6_u16, 6_u16, 6_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16,
                7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16,
                7_u16, 7_u16, 7_u16, 7_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 2_u16, 2_u16, 4_u16, 4_u16, 4_u16, 6_u16, 6_u16, 6_u16, 6_u16, 8_u16, 8_u16,
                8_u16, 8_u16, 8_u16, 8_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16,
                10_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16,
                12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 14_u16, 14_u16, 14_u16,
                14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16,
                14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16,
                14_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 4_u16, 4_u16, 8_u16, 8_u16, 8_u16, 12_u16, 12_u16, 12_u16, 12_u16, 16_u16,
                16_u16, 16_u16, 16_u16, 16_u16, 16_u16, 20_u16, 20_u16, 20_u16, 20_u16, 20_u16,
                20_u16, 20_u16, 20_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16,
                24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 28_u16,
                28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16,
                28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16,
                28_u16, 28_u16, 28_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 8_u16, 8_u16, 16_u16, 16_u16, 16_u16, 24_u16, 24_u16, 24_u16, 24_u16,
                32_u16, 32_u16, 32_u16, 32_u16, 32_u16, 32_u16, 40_u16, 40_u16, 40_u16, 40_u16,
                40_u16, 40_u16, 40_u16, 40_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16,
                48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16,
                55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16,
                55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16,
                55_u16, 55_u16, 55_u16, 55_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 16_u16, 16_u16, 32_u16, 32_u16, 32_u16, 48_u16, 48_u16, 48_u16, 48_u16,
                64_u16, 64_u16, 64_u16, 64_u16, 64_u16, 64_u16, 80_u16, 80_u16, 80_u16, 80_u16,
                80_u16, 80_u16, 80_u16, 80_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16,
                95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16,
                109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16,
                109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16,
                109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 32_u16, 32_u16, 64_u16, 64_u16, 64_u16, 96_u16, 96_u16, 96_u16, 96_u16,
                127_u16, 127_u16, 127_u16, 127_u16, 127_u16, 127_u16, 157_u16, 157_u16, 157_u16,
                157_u16, 157_u16, 157_u16, 157_u16, 157_u16, 185_u16, 185_u16, 185_u16, 185_u16,
                185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16,
                185_u16, 185_u16, 185_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
                211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
                211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
            ]))),
            Rc::new(RefCell::new(Box::new([
                0_u16, 64_u16, 64_u16, 127_u16, 127_u16, 127_u16, 188_u16, 188_u16, 188_u16,
                188_u16, 246_u16, 246_u16, 246_u16, 246_u16, 246_u16, 246_u16, 300_u16, 300_u16,
                300_u16, 300_u16, 300_u16, 300_u16, 300_u16, 300_u16, 348_u16, 348_u16, 348_u16,
                348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16,
                348_u16, 348_u16, 348_u16, 348_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
                388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
                388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
                388_u16,
            ]))),
        ])));
);
thread_local!(
    pub static kNumNonzeroContextSkip_94: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        8_u16, 15_u16, 31_u16, 61_u16, 120_u16, 231_u16, 412_u16,
    ])));
);
thread_local!(
    pub static kContextAlgorithm_95: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 2_u8, 3_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8,
        0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
    ])));
);
pub fn ZeroDensityContext_96(mut nonzeros_left: usize, mut k: usize, mut bits: usize) -> u16 {
    return (((({
        let __idx = (bits) as usize;
        kNumNonzeroContext_93.with(|rc| rc.borrow()[__idx].clone())
    })
    .borrow()[(nonzeros_left) as usize] as i32)
        + (({
            let __idx = (bits) as usize;
            kFreqContext_92.with(|rc| rc.borrow()[__idx].clone())
        })
        .borrow()[(k) as usize] as i32)) as u16);
}
pub fn WeightedAverageContextDC_97(mut vals: Ptr<i32>, mut x: i32) -> i32 {
    let mut sum: i32 = ((((1 + (elem!(vals, (x - 2)).read())) + (elem!(vals, (x - 1)).read()))
        + (elem!(vals, x).read()))
        + (elem!(vals, (x + 1)).read()));
    if ((sum >> kMaxAverageContext_82.with(|rc| *rc.borrow())) != 0) {
        return (kMaxAverageContext_82.with(|rc| *rc.borrow()) as i32);
    }
    return ({ Log2FloorNonZero_74((sum as u32)) });
}
pub fn WeightedAverageContext_98(mut vals: Ptr<i32>, mut prev_row_delta: i32) -> i32 {
    let mut sum: i32 = ((({
        ({ (4 + (elem!(vals, 0).read())) } + {
            (({ (elem!(vals, -kDCTBlockSize_3.with(|rc| *rc.borrow())).read()) } + {
                (elem!(vals, prev_row_delta).read())
            }) * 2)
        })
    } + {
        (elem!(vals, (-2_i32 * kDCTBlockSize_3.with(|rc| *rc.borrow()))).read())
    }) + (elem!(
        vals,
        (prev_row_delta - kDCTBlockSize_3.with(|rc| *rc.borrow()))
    )
    .read()))
        + (elem!(
            vals,
            (prev_row_delta + kDCTBlockSize_3.with(|rc| *rc.borrow()))
        )
        .read()));
    if ((sum >> ((kMaxAverageContext_82.with(|rc| *rc.borrow())).wrapping_add(2_usize))) != 0) {
        return (kMaxAverageContext_82.with(|rc| *rc.borrow()) as i32);
    }
    return (({ Log2FloorNonZero_74((sum as u32)) }) - 2);
}
thread_local!(
    pub static kACPredictPrecisionBits_99: Value<i32> = Rc::new(RefCell::new(13));
);
thread_local!(
    pub static kACPredictPrecision_100: Value<i32> = Rc::new(RefCell::new(
        (1 << kACPredictPrecisionBits_99.with(|rc| *rc.borrow())),
    ));
);
pub fn ACPredictContext_101(mut p: i64, mut avg_ctx: Ptr<usize>, mut sgn: Ptr<usize>) {
    let mut multiplier: i32 = 0_i32;
    if (p >= 0_i64) {
        multiplier = 1;
    } else {
        multiplier = -1_i32;
        p = { -p };
    }
    let mut ctx: usize = 0_usize;
    if (p >= ((1_u32 << kMaxAverageContext_82.with(|rc| *rc.borrow())) as i64)) {
        ctx = kMaxAverageContext_82.with(|rc| *rc.borrow());
    } else {
        ctx = (({ Log2FloorNonZero_74(((2_u32).wrapping_mul((p as u32))).wrapping_add(1_u32)) })
            as usize);
    }
    avg_ctx.write({ ctx });
    sgn.write({
        (kMaxAverageContext_82.with(|rc| *rc.borrow()))
            .wrapping_add((multiplier as usize).wrapping_mul(ctx))
    });
}
pub fn ACPredictContextCol_102(
    mut prev: Ptr<i16>,
    mut cur: Ptr<i16>,
    mut mult: Ptr<i32>,
    mut avg_ctx: Ptr<usize>,
    mut sgn: Ptr<usize>,
) {
    let mut terms: [i16; 8] = [0_i16; 8];
    terms[(0) as usize] = 0_i16;
    terms[(1) as usize] =
        { (({ ((elem!(cur, 1).read()) as i32) } + { ((elem!(prev, 1).read()) as i32) }) as i16) };
    terms[(2) as usize] =
        { (({ ((elem!(cur, 2).read()) as i32) } - { ((elem!(prev, 2).read()) as i32) }) as i16) };
    terms[(3) as usize] =
        { (({ ((elem!(cur, 3).read()) as i32) } + { ((elem!(prev, 3).read()) as i32) }) as i16) };
    terms[(4) as usize] =
        { (({ ((elem!(cur, 4).read()) as i32) } - { ((elem!(prev, 4).read()) as i32) }) as i16) };
    terms[(5) as usize] =
        { (({ ((elem!(cur, 5).read()) as i32) } + { ((elem!(prev, 5).read()) as i32) }) as i16) };
    terms[(6) as usize] =
        { (({ ((elem!(cur, 6).read()) as i32) } - { ((elem!(prev, 6).read()) as i32) }) as i16) };
    terms[(7) as usize] =
        { (({ ((elem!(cur, 7).read()) as i32) } + { ((elem!(prev, 7).read()) as i32) }) as i16) };
    let mut delta: i64 =
        (((((((({ (terms[(0) as usize] as i64) } * { ((elem!(mult, 0).read()) as i64) })
            + ({ (terms[(1) as usize] as i64) } * { ((elem!(mult, 1).read()) as i64) }))
            + ({ (terms[(2) as usize] as i64) } * { ((elem!(mult, 2).read()) as i64) }))
            + ({ (terms[(3) as usize] as i64) } * { ((elem!(mult, 3).read()) as i64) }))
            + ({ (terms[(4) as usize] as i64) } * { ((elem!(mult, 4).read()) as i64) }))
            + ({ (terms[(5) as usize] as i64) } * { ((elem!(mult, 5).read()) as i64) }))
            + ({ (terms[(6) as usize] as i64) } * { ((elem!(mult, 6).read()) as i64) }))
            + ({ (terms[(7) as usize] as i64) } * { ((elem!(mult, 7).read()) as i64) }));
    ({
        ACPredictContext_101(
            ({ ((elem!(prev, 0).read()) as i64) } - {
                (delta / (kACPredictPrecision_100.with(|rc| *rc.borrow()) as i64))
            }),
            (avg_ctx).clone(),
            (sgn).clone(),
        )
    });
}
pub fn ACPredictContextRow_103(
    mut prev: Ptr<i16>,
    mut cur: Ptr<i16>,
    mut mult: Ptr<i32>,
    mut avg_ctx: Ptr<usize>,
    mut sgn: Ptr<usize>,
) {
    let mut terms: [i16; 8] = [0_i16; 8];
    terms[(0) as usize] = 0_i16;
    terms[(1) as usize] =
        { (({ ((elem!(cur, 8).read()) as i32) } + { ((elem!(prev, 8).read()) as i32) }) as i16) };
    terms[(2) as usize] =
        { (({ ((elem!(cur, 16).read()) as i32) } - { ((elem!(prev, 16).read()) as i32) }) as i16) };
    terms[(3) as usize] =
        { (({ ((elem!(cur, 24).read()) as i32) } + { ((elem!(prev, 24).read()) as i32) }) as i16) };
    terms[(4) as usize] =
        { (({ ((elem!(cur, 32).read()) as i32) } - { ((elem!(prev, 32).read()) as i32) }) as i16) };
    terms[(5) as usize] =
        { (({ ((elem!(cur, 40).read()) as i32) } + { ((elem!(prev, 40).read()) as i32) }) as i16) };
    terms[(6) as usize] =
        { (({ ((elem!(cur, 48).read()) as i32) } - { ((elem!(prev, 48).read()) as i32) }) as i16) };
    terms[(7) as usize] =
        { (({ ((elem!(cur, 56).read()) as i32) } + { ((elem!(prev, 56).read()) as i32) }) as i16) };
    let mut delta: i64 =
        (((((((({ (terms[(0) as usize] as i64) } * { ((elem!(mult, 0).read()) as i64) })
            + ({ (terms[(1) as usize] as i64) } * { ((elem!(mult, 1).read()) as i64) }))
            + ({ (terms[(2) as usize] as i64) } * { ((elem!(mult, 2).read()) as i64) }))
            + ({ (terms[(3) as usize] as i64) } * { ((elem!(mult, 3).read()) as i64) }))
            + ({ (terms[(4) as usize] as i64) } * { ((elem!(mult, 4).read()) as i64) }))
            + ({ (terms[(5) as usize] as i64) } * { ((elem!(mult, 5).read()) as i64) }))
            + ({ (terms[(6) as usize] as i64) } * { ((elem!(mult, 6).read()) as i64) }))
            + ({ (terms[(7) as usize] as i64) } * { ((elem!(mult, 7).read()) as i64) }));
    ({
        ACPredictContext_101(
            ({ ((elem!(prev, 0).read()) as i64) } - {
                (delta / (kACPredictPrecision_100.with(|rc| *rc.borrow()) as i64))
            }),
            (avg_ctx).clone(),
            (sgn).clone(),
        )
    });
}
pub fn NumNonzerosContext_104(mut prev: Ptr<u8>, mut x: i32, mut y: i32) -> u8 {
    let mut prediction: usize = 0_usize;
    if (y == 0) {
        if (x == 0) {
            prediction = 0_usize;
        } else {
            prediction = ((elem!(prev, (x - 1)).read()) as usize);
        }
    } else if (x == 0) {
        prediction = ((elem!(prev, x).read()) as usize);
    } else {
        prediction = ((((((elem!(prev, (x - 1)).read()) as i32)
            + ((elem!(prev, x).read()) as i32))
            + 1)
            / 2) as usize);
    }
    if !(prediction <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"context.cc"),
                305,
                Ptr::<i8>::from_string_literal(b"NumNonzerosContext"),
            )
        });
        'loop_: while true {}
    };
    return (((prediction).wrapping_div(kNumNonZeroQuant_86.with(|rc| *rc.borrow()))) as u8);
}
thread_local!(
    pub static kNumIsEmptyBlockContexts_105: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn IsEmptyBlockContext_106(mut prev: Ptr<i32>, mut x: i32) -> i32 {
    return ((elem!(prev, (x - 1)).read()) + (elem!(prev, x).read()));
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(152)]
pub struct brunsli_ComponentStateDC {
    #[offset(0)]
    pub width: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub is_zero_prob: brunsli_Prob,
    #[offset(8)]
    #[byte_size(24)]
    pub is_empty_block_prob: Value<Vec<brunsli_Prob>>,
    #[offset(32)]
    #[byte_size(24)]
    pub sign_prob: Value<Vec<brunsli_Prob>>,
    #[offset(56)]
    #[byte_size(24)]
    pub first_extra_bit_prob: Value<Vec<brunsli_Prob>>,
    #[offset(80)]
    #[byte_size(24)]
    pub prev_is_nonempty: Value<Vec<i32>>,
    #[offset(104)]
    #[byte_size(24)]
    pub prev_abs_coeff: Value<Vec<i32>>,
    #[offset(128)]
    #[byte_size(24)]
    pub prev_sign: Value<Vec<i32>>,
}
impl brunsli_ComponentStateDC {
    pub fn new() -> Self {
        let __this: Value<brunsli_ComponentStateDC> = Rc::new(RefCell::new(Self {
            width: 0,
            is_zero_prob: brunsli_Prob::new(),
            is_empty_block_prob: Rc::new(RefCell::new(
                (0..(kNumIsEmptyBlockContexts_105.with(|rc| *rc.borrow()) as usize) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            sign_prob: Rc::new(RefCell::new(
                (0..(9_usize) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (0..(10_usize) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new(Vec::new())),
            prev_abs_coeff: Rc::new(RefCell::new(Vec::new())),
            prev_sign: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_ComponentStateDC> = __this.as_pointer();
        ({ brunsli_ComponentStateDCImpl::InitAll(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for brunsli_ComponentStateDC {
    fn default() -> Self {
        { brunsli_ComponentStateDC::new() }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(9008)]
pub struct brunsli_ComponentState {
    #[offset(0)]
    pub width: i32,
    #[offset(4)]
    pub context_offset: i32,
    #[offset(8)]
    #[byte_size(256)]
    pub order: Value<Box<[u32]>>,
    #[offset(264)]
    #[byte_size(256)]
    pub mult_row: Value<Box<[i32]>>,
    #[offset(520)]
    #[byte_size(256)]
    pub mult_col: Value<Box<[i32]>>,
    #[offset(776)]
    #[byte_size(24)]
    pub is_zero_prob: Value<Vec<brunsli_Prob>>,
    #[offset(800)]
    #[byte_size(24)]
    pub sign_prob: Value<Vec<brunsli_Prob>>,
    #[offset(824)]
    #[byte_size(8064)]
    pub num_nonzero_prob: Value<Box<[brunsli_Prob]>>,
    #[offset(8888)]
    #[byte_size(24)]
    pub first_extra_bit_prob: Value<Vec<brunsli_Prob>>,
    #[offset(8912)]
    #[byte_size(24)]
    pub prev_is_nonempty: Value<Vec<i32>>,
    #[offset(8936)]
    #[byte_size(24)]
    pub prev_num_nonzeros: Value<Vec<u8>>,
    #[offset(8960)]
    #[byte_size(24)]
    pub prev_abs_coeff: Value<Vec<i32>>,
    #[offset(8984)]
    #[byte_size(24)]
    pub prev_sign: Value<Vec<i32>>,
}
impl brunsli_ComponentState {
    pub fn new() -> Self {
        let __this: Value<brunsli_ComponentState> = Rc::new(RefCell::new(Self {
            width: 0,
            context_offset: 0_i32,
            order: Rc::new(RefCell::new((0..64).map(|_| 0_u32).collect::<Box<[u32]>>())),
            mult_row: Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>())),
            mult_col: Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>())),
            is_zero_prob: Rc::new(RefCell::new(
                (0..(((kNumNonzeroBuckets_90.with(|rc| *rc.borrow()) as i32)
                    * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize)
                    as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            sign_prob: Rc::new(RefCell::new(
                (0..(((((2_usize).wrapping_mul(kMaxAverageContext_82.with(|rc| *rc.borrow()))
                    as usize)
                    .wrapping_add(1_usize)) as usize)
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                    as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            num_nonzero_prob: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2016, _>(
                |_| brunsli_Prob::new(),
            )))),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (0..((10 * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new(Vec::new())),
            prev_num_nonzeros: Rc::new(RefCell::new(Vec::new())),
            prev_abs_coeff: Rc::new(RefCell::new(Vec::new())),
            prev_sign: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_ComponentState> = __this.as_pointer();
        ({ brunsli_ComponentStateImpl::InitAll(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn SizeInBytes(mut w: i32) -> usize {
        return (((((4 + ((10 + (3 * w)) * kDCTBlockSize_3.with(|rc| *rc.borrow()))) + (2 * w))
            as usize)
            .wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            .wrapping_add(
                ((((((((kNumNonzeroBuckets_90.with(|rc| *rc.borrow()) as usize).wrapping_add(
                    ((2_usize).wrapping_mul(kMaxAverageContext_82.with(|rc| *rc.borrow()))
                        as usize),
                ) as usize)
                    .wrapping_add(11_usize)) as usize)
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
                    as usize)
                    .wrapping_add(
                        ((kNumNonZeroContextCount_88.with(|rc| *rc.borrow()))
                            .wrapping_mul(kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                            as usize),
                    )) as u64)
                    .wrapping_mul((4usize as u64)) as u64),
            ) as usize);
    }
}
impl Default for brunsli_ComponentState {
    fn default() -> Self {
        { brunsli_ComponentState::new() }
    }
}
thread_local!(
    pub static kSqrt2_107: Value<f64> = Rc::new(RefCell::new(1.414213562E+0));
);
thread_local!(
    pub static kSqrt2FixedPoint_108: Value<i32> = Rc::new(RefCell::new(
        ((kSqrt2_107.with(|rc| *rc.borrow())
            * (kACPredictPrecision_100.with(|rc| *rc.borrow()) as f64)) as i32),
    ));
);
pub fn ComputeACPredictMultipliers_109(
    mut quant: Ptr<i32>,
    mut mult_row: Ptr<i32>,
    mut mult_col: Ptr<i32>,
) {
    let mut y: usize = 0_usize;
    'loop_: while (y < 8_usize) {
        let mut x: usize = 0_usize;
        'loop_: while (x < 8_usize) {
            elem!(mult_row, (x).wrapping_add((8_usize).wrapping_mul(y))).write({
                ({
                    ({ (elem!(quant, (x).wrapping_add((8_usize).wrapping_mul(y))).read()) } * {
                        kSqrt2FixedPoint_108.with(|rc| *rc.borrow())
                    })
                } / { (elem!(quant, (y).wrapping_mul(8_usize)).read()) })
            });
            elem!(mult_col, ((x).wrapping_mul(8_usize)).wrapping_add(y)).write({
                ({
                    ({ (elem!(quant, (x).wrapping_add((8_usize).wrapping_mul(y))).read()) } * {
                        kSqrt2FixedPoint_108.with(|rc| *rc.borrow())
                    })
                } / { (elem!(quant, x).read()) })
            });
            x.prefix_inc();
        }
        y.prefix_inc();
    }
}
thread_local!(
    pub static kInitProb_110: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        228_u8, 216_u8, 216_u8, 195_u8, 192_u8, 189_u8, 182_u8, 184_u8, 179_u8, 176_u8, 171_u8,
        168_u8, 166_u8, 159_u8, 156_u8, 151_u8, 151_u8, 150_u8, 150_u8, 146_u8, 144_u8, 138_u8,
        138_u8, 137_u8, 135_u8, 131_u8, 127_u8, 126_u8, 124_u8, 123_u8, 124_u8, 123_u8, 122_u8,
        121_u8, 118_u8, 117_u8, 114_u8, 115_u8, 116_u8, 116_u8, 115_u8, 115_u8, 114_u8, 111_u8,
        111_u8, 111_u8, 112_u8, 111_u8, 110_u8, 110_u8, 110_u8, 111_u8, 111_u8, 114_u8, 110_u8,
        111_u8, 112_u8, 113_u8, 116_u8, 120_u8, 126_u8, 131_u8, 147_u8, 160_u8,
    ])));
);
thread_local!(
    pub static kInitProbNonzero_111: Value<Box<[Value<Box<[u8]>>]>> =
        Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::new([
                251_u8, 252_u8, 117_u8, 249_u8, 161_u8, 136_u8, 83_u8, 238_u8, 184_u8, 126_u8,
                137_u8, 129_u8, 140_u8, 119_u8, 70_u8, 213_u8, 160_u8, 175_u8, 174_u8, 130_u8,
                166_u8, 134_u8, 122_u8, 125_u8, 131_u8, 144_u8, 136_u8, 133_u8, 139_u8, 123_u8,
                79_u8, 216_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                254_u8, 252_u8, 174_u8, 232_u8, 189_u8, 155_u8, 122_u8, 177_u8, 204_u8, 173_u8,
                146_u8, 149_u8, 141_u8, 133_u8, 103_u8, 109_u8, 167_u8, 187_u8, 168_u8, 142_u8,
                154_u8, 147_u8, 125_u8, 139_u8, 144_u8, 138_u8, 138_u8, 153_u8, 141_u8, 133_u8,
                90_u8, 121_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                251_u8, 240_u8, 197_u8, 176_u8, 184_u8, 177_u8, 114_u8, 89_u8, 194_u8, 165_u8,
                153_u8, 161_u8, 158_u8, 136_u8, 92_u8, 95_u8, 123_u8, 171_u8, 160_u8, 140_u8,
                148_u8, 136_u8, 129_u8, 139_u8, 145_u8, 136_u8, 143_u8, 134_u8, 138_u8, 124_u8,
                92_u8, 154_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                247_u8, 220_u8, 201_u8, 110_u8, 194_u8, 176_u8, 147_u8, 59_u8, 175_u8, 171_u8,
                156_u8, 157_u8, 152_u8, 146_u8, 115_u8, 114_u8, 88_u8, 151_u8, 164_u8, 141_u8,
                153_u8, 135_u8, 141_u8, 131_u8, 146_u8, 139_u8, 140_u8, 145_u8, 138_u8, 137_u8,
                112_u8, 184_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                238_u8, 179_u8, 203_u8, 63_u8, 194_u8, 173_u8, 149_u8, 71_u8, 139_u8, 169_u8,
                154_u8, 159_u8, 150_u8, 146_u8, 117_u8, 143_u8, 78_u8, 122_u8, 152_u8, 137_u8,
                149_u8, 138_u8, 138_u8, 133_u8, 134_u8, 142_u8, 142_u8, 142_u8, 148_u8, 128_u8,
                118_u8, 199_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                227_u8, 127_u8, 200_u8, 44_u8, 192_u8, 170_u8, 148_u8, 100_u8, 102_u8, 161_u8,
                156_u8, 153_u8, 148_u8, 149_u8, 124_u8, 160_u8, 88_u8, 101_u8, 134_u8, 132_u8,
                149_u8, 145_u8, 134_u8, 134_u8, 136_u8, 141_u8, 138_u8, 142_u8, 144_u8, 137_u8,
                116_u8, 208_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                214_u8, 86_u8, 195_u8, 44_u8, 187_u8, 163_u8, 148_u8, 126_u8, 81_u8, 147_u8,
                156_u8, 152_u8, 150_u8, 144_u8, 121_u8, 172_u8, 96_u8, 95_u8, 117_u8, 122_u8,
                145_u8, 152_u8, 136_u8, 133_u8, 135_u8, 135_u8, 131_u8, 142_u8, 141_u8, 135_u8,
                114_u8, 217_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                198_u8, 56_u8, 191_u8, 54_u8, 171_u8, 162_u8, 147_u8, 144_u8, 74_u8, 128_u8,
                152_u8, 149_u8, 150_u8, 142_u8, 119_u8, 177_u8, 101_u8, 100_u8, 106_u8, 111_u8,
                135_u8, 154_u8, 136_u8, 137_u8, 136_u8, 132_u8, 133_u8, 142_u8, 144_u8, 130_u8,
                117_u8, 222_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                176_u8, 40_u8, 189_u8, 73_u8, 147_u8, 159_u8, 148_u8, 152_u8, 79_u8, 106_u8,
                147_u8, 149_u8, 151_u8, 139_u8, 123_u8, 188_u8, 108_u8, 110_u8, 106_u8, 97_u8,
                125_u8, 151_u8, 137_u8, 138_u8, 135_u8, 135_u8, 134_u8, 136_u8, 140_u8, 131_u8,
                116_u8, 221_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                148_u8, 33_u8, 185_u8, 88_u8, 117_u8, 158_u8, 145_u8, 163_u8, 95_u8, 91_u8, 137_u8,
                146_u8, 150_u8, 140_u8, 120_u8, 197_u8, 115_u8, 116_u8, 114_u8, 92_u8, 114_u8,
                144_u8, 130_u8, 133_u8, 132_u8, 133_u8, 129_u8, 140_u8, 138_u8, 130_u8, 111_u8,
                224_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                117_u8, 31_u8, 180_u8, 104_u8, 93_u8, 150_u8, 143_u8, 166_u8, 99_u8, 85_u8, 124_u8,
                139_u8, 148_u8, 142_u8, 118_u8, 201_u8, 105_u8, 120_u8, 120_u8, 90_u8, 107_u8,
                135_u8, 127_u8, 130_u8, 131_u8, 131_u8, 132_u8, 140_u8, 142_u8, 133_u8, 114_u8,
                229_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                87_u8, 35_u8, 170_u8, 110_u8, 78_u8, 141_u8, 144_u8, 176_u8, 106_u8, 90_u8, 112_u8,
                132_u8, 143_u8, 138_u8, 119_u8, 204_u8, 111_u8, 121_u8, 125_u8, 90_u8, 105_u8,
                131_u8, 124_u8, 122_u8, 129_u8, 128_u8, 129_u8, 137_u8, 138_u8, 133_u8, 114_u8,
                227_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                63_u8, 42_u8, 159_u8, 123_u8, 73_u8, 127_u8, 142_u8, 191_u8, 105_u8, 91_u8, 105_u8,
                123_u8, 139_u8, 137_u8, 120_u8, 209_u8, 117_u8, 110_u8, 122_u8, 98_u8, 110_u8,
                125_u8, 115_u8, 123_u8, 122_u8, 126_u8, 128_u8, 134_u8, 141_u8, 129_u8, 113_u8,
                229_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                45_u8, 53_u8, 146_u8, 135_u8, 71_u8, 114_u8, 138_u8, 193_u8, 100_u8, 98_u8, 98_u8,
                113_u8, 133_u8, 135_u8, 118_u8, 222_u8, 113_u8, 111_u8, 139_u8, 103_u8, 107_u8,
                126_u8, 111_u8, 119_u8, 121_u8, 122_u8, 127_u8, 135_u8, 141_u8, 128_u8, 114_u8,
                242_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                33_u8, 60_u8, 132_u8, 138_u8, 75_u8, 100_u8, 134_u8, 203_u8, 112_u8, 99_u8, 98_u8,
                105_u8, 126_u8, 131_u8, 115_u8, 229_u8, 107_u8, 93_u8, 121_u8, 106_u8, 108_u8,
                122_u8, 106_u8, 109_u8, 114_u8, 116_u8, 127_u8, 133_u8, 143_u8, 128_u8, 110_u8,
                242_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                24_u8, 70_u8, 118_u8, 134_u8, 76_u8, 87_u8, 130_u8, 201_u8, 110_u8, 96_u8, 99_u8,
                97_u8, 119_u8, 130_u8, 111_u8, 229_u8, 97_u8, 104_u8, 125_u8, 102_u8, 112_u8,
                125_u8, 101_u8, 109_u8, 113_u8, 114_u8, 125_u8, 129_u8, 142_u8, 127_u8, 112_u8,
                241_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                17_u8, 65_u8, 100_u8, 121_u8, 80_u8, 75_u8, 124_u8, 174_u8, 117_u8, 100_u8, 94_u8,
                93_u8, 114_u8, 128_u8, 110_u8, 216_u8, 103_u8, 94_u8, 113_u8, 122_u8, 118_u8,
                126_u8, 113_u8, 108_u8, 105_u8, 108_u8, 122_u8, 128_u8, 141_u8, 125_u8, 113_u8,
                238_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                12_u8, 70_u8, 82_u8, 132_u8, 78_u8, 65_u8, 118_u8, 155_u8, 136_u8, 103_u8, 97_u8,
                89_u8, 106_u8, 124_u8, 111_u8, 215_u8, 115_u8, 123_u8, 129_u8, 99_u8, 104_u8,
                127_u8, 110_u8, 108_u8, 101_u8, 109_u8, 118_u8, 126_u8, 136_u8, 123_u8, 110_u8,
                233_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                8_u8, 66_u8, 61_u8, 117_u8, 91_u8, 59_u8, 108_u8, 195_u8, 101_u8, 112_u8, 99_u8,
                99_u8, 99_u8, 116_u8, 106_u8, 230_u8, 127_u8, 99_u8, 144_u8, 101_u8, 118_u8,
                137_u8, 117_u8, 111_u8, 106_u8, 104_u8, 116_u8, 121_u8, 134_u8, 122_u8, 110_u8,
                223_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                6_u8, 78_u8, 42_u8, 146_u8, 101_u8, 54_u8, 94_u8, 201_u8, 116_u8, 102_u8, 110_u8,
                94_u8, 92_u8, 108_u8, 103_u8, 214_u8, 108_u8, 111_u8, 127_u8, 102_u8, 121_u8,
                132_u8, 120_u8, 121_u8, 95_u8, 98_u8, 110_u8, 121_u8, 129_u8, 117_u8, 107_u8,
                235_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                5_u8, 93_u8, 29_u8, 145_u8, 102_u8, 52_u8, 77_u8, 216_u8, 108_u8, 115_u8, 108_u8,
                102_u8, 89_u8, 97_u8, 94_u8, 229_u8, 89_u8, 103_u8, 139_u8, 120_u8, 103_u8, 151_u8,
                102_u8, 100_u8, 97_u8, 96_u8, 99_u8, 111_u8, 125_u8, 116_u8, 104_u8, 242_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                4_u8, 105_u8, 21_u8, 145_u8, 100_u8, 54_u8, 64_u8, 217_u8, 100_u8, 122_u8, 128_u8,
                87_u8, 88_u8, 91_u8, 87_u8, 230_u8, 112_u8, 80_u8, 148_u8, 95_u8, 146_u8, 123_u8,
                96_u8, 140_u8, 90_u8, 91_u8, 98_u8, 106_u8, 122_u8, 111_u8, 100_u8, 249_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                4_u8, 130_u8, 14_u8, 142_u8, 104_u8, 56_u8, 51_u8, 208_u8, 116_u8, 135_u8, 100_u8,
                89_u8, 82_u8, 84_u8, 75_u8, 239_u8, 85_u8, 85_u8, 122_u8, 125_u8, 94_u8, 144_u8,
                151_u8, 136_u8, 92_u8, 97_u8, 104_u8, 109_u8, 113_u8, 110_u8, 91_u8, 246_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                3_u8, 126_u8, 9_u8, 172_u8, 105_u8, 57_u8, 39_u8, 219_u8, 95_u8, 120_u8, 118_u8,
                96_u8, 93_u8, 75_u8, 66_u8, 241_u8, 102_u8, 134_u8, 96_u8, 156_u8, 146_u8, 162_u8,
                130_u8, 112_u8, 82_u8, 89_u8, 97_u8, 101_u8, 116_u8, 103_u8, 82_u8, 254_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                3_u8, 149_u8, 7_u8, 182_u8, 122_u8, 54_u8, 29_u8, 224_u8, 103_u8, 100_u8, 113_u8,
                96_u8, 90_u8, 74_u8, 55_u8, 250_u8, 127_u8, 94_u8, 118_u8, 93_u8, 135_u8, 160_u8,
                113_u8, 130_u8, 95_u8, 117_u8, 106_u8, 96_u8, 111_u8, 97_u8, 77_u8, 242_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                3_u8, 150_u8, 4_u8, 170_u8, 138_u8, 59_u8, 20_u8, 229_u8, 91_u8, 150_u8, 107_u8,
                98_u8, 92_u8, 68_u8, 48_u8, 245_u8, 113_u8, 64_u8, 114_u8, 111_u8, 134_u8, 127_u8,
                102_u8, 104_u8, 85_u8, 118_u8, 103_u8, 107_u8, 102_u8, 91_u8, 72_u8, 245_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                3_u8, 171_u8, 3_u8, 165_u8, 137_u8, 62_u8, 14_u8, 211_u8, 96_u8, 127_u8, 132_u8,
                121_u8, 95_u8, 62_u8, 37_u8, 248_u8, 102_u8, 57_u8, 144_u8, 85_u8, 127_u8, 191_u8,
                102_u8, 97_u8, 127_u8, 104_u8, 91_u8, 102_u8, 107_u8, 81_u8, 64_u8, 254_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                2_u8, 166_u8, 2_u8, 196_u8, 122_u8, 65_u8, 10_u8, 243_u8, 102_u8, 93_u8, 117_u8,
                92_u8, 96_u8, 63_u8, 29_u8, 251_u8, 169_u8, 159_u8, 149_u8, 96_u8, 91_u8, 139_u8,
                157_u8, 40_u8, 100_u8, 89_u8, 120_u8, 92_u8, 109_u8, 79_u8, 58_u8, 247_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                2_u8, 176_u8, 2_u8, 189_u8, 118_u8, 48_u8, 7_u8, 219_u8, 68_u8, 43_u8, 109_u8,
                96_u8, 129_u8, 75_u8, 19_u8, 254_u8, 2_u8, 3_u8, 185_u8, 6_u8, 102_u8, 127_u8,
                127_u8, 127_u8, 1_u8, 131_u8, 83_u8, 99_u8, 107_u8, 80_u8, 45_u8, 254_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                1_u8, 205_u8, 2_u8, 208_u8, 64_u8, 89_u8, 4_u8, 223_u8, 29_u8, 169_u8, 29_u8,
                123_u8, 118_u8, 76_u8, 11_u8, 240_u8, 202_u8, 243_u8, 65_u8, 6_u8, 12_u8, 243_u8,
                96_u8, 55_u8, 102_u8, 102_u8, 114_u8, 102_u8, 107_u8, 74_u8, 31_u8, 247_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                1_u8, 216_u8, 1_u8, 214_u8, 127_u8, 94_u8, 2_u8, 234_u8, 145_u8, 3_u8, 127_u8,
                106_u8, 155_u8, 80_u8, 4_u8, 247_u8, 4_u8, 65_u8, 86_u8, 127_u8, 127_u8, 127_u8,
                127_u8, 102_u8, 127_u8, 143_u8, 143_u8, 108_u8, 113_u8, 80_u8, 16_u8, 216_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8,
            ]))),
            Rc::new(RefCell::new(Box::new([
                2_u8, 199_u8, 1_u8, 222_u8, 93_u8, 94_u8, 1_u8, 232_u8, 2_u8, 65_u8, 74_u8, 139_u8,
                201_u8, 48_u8, 2_u8, 254_u8, 169_u8, 127_u8, 52_u8, 243_u8, 251_u8, 249_u8, 102_u8,
                86_u8, 202_u8, 153_u8, 65_u8, 65_u8, 146_u8, 69_u8, 8_u8, 238_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
                128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            ]))),
        ])));
);
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct brunsli_PermutationCoder {
    #[offset(0)]
    #[byte_size(24)]
    values_: Value<Vec<u8>>,
}
impl brunsli_PermutationCoder {
    pub fn new() -> Self {
        Self {
            values_: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl Default for brunsli_PermutationCoder {
    fn default() -> Self {
        { brunsli_PermutationCoder::new() }
    }
}
pub fn ComputeLehmerCode_112(mut sigma: Ptr<u32>, mut len: usize, mut code: Ptr<u32>) {
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(len) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let __rhs = (i as u32);
        (*items.borrow_mut())[i] = __rhs;
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let it: Value<Ptr<u32>> = Rc::new(RefCell::new({
            let count = ((items.as_pointer() as Ptr<u32>).to_end().get_offset()
                - (items.as_pointer() as Ptr<u32>).get_offset()) as usize;
            (items.as_pointer() as Ptr<u32>).offset(
                (items.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .take(count)
                    .position(|value_0| value_0.read() == (elem!(sigma, i).read()))
                    .unwrap_or(count) as isize,
            )
        }));
        if !((*it.borrow()) != (items.as_pointer() as Ptr<u32>).to_end()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"lehmer_code.cc"),
                    21,
                    Ptr::<i8>::from_string_literal(b"ComputeLehmerCode"),
                )
            });
            'loop_: while true {}
        };
        let __rhs = ((((*it.borrow()).get_offset() as isize)
            - ((items.as_pointer() as Ptr<u32>).get_offset() as isize))
            as u32);
        elem!(code, i).write(__rhs);
        {
            let idx = (*it.borrow()).get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>).decay()
        };
        i.prefix_inc();
    }
}
pub fn DecodeLehmerCode_113(mut code: Ptr<u32>, mut len: usize, mut sigma: Ptr<u32>) -> bool {
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(len) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let __rhs = (i as u32);
        (*items.borrow_mut())[i] = __rhs;
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let mut index: u32 = (elem!(code, i).read());
        if ((index as usize) >= (*items.borrow()).len()) {
            return false;
        }
        let mut value: u32 = { (*items.borrow())[(index as usize)] };
        {
            let idx = (items.as_pointer() as Ptr<u32>)
                .offset((index as i64) as isize)
                .get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>).decay()
        };
        elem!(sigma, i).write({ value });
        i.prefix_inc();
    }
    return true;
}
pub fn BrunsliDumpAndAbort_79(mut f: Ptr<i8>, mut l: i32, mut fn_: Ptr<i8>) {
    eprintln!("{}:{} ({})", f, l, fn_);
    0;
    std::process::abort();
}
pub fn AdaptiveMedian_114(w: i32, n: i32, mut nw: i32) -> i32 {
    let w: Value<i32> = Rc::new(RefCell::new(w));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let mut mx: i32 = if ((*w.borrow()) > (*n.borrow())) {
        (*w.borrow())
    } else {
        (*n.borrow())
    };
    let mut mn: i32 = (((*w.borrow()) + (*n.borrow())) - mx);
    if (nw > mx) {
        return mn;
    } else if (nw < mn) {
        return mx;
    } else {
        return (((*n.borrow()) + (*w.borrow())) - nw);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn PredictWithAdaptiveMedian_115(
    mut coeffs: Ptr<i16>,
    mut x: i32,
    mut y: i32,
    mut stride: i32,
) -> i32 {
    let mut offset1: i32 = -kDCTBlockSize_3.with(|rc| *rc.borrow());
    let mut offset2: i32 = -stride;
    let mut offset3: i32 = (offset2 + offset1);
    if (y != 0) {
        if (x != 0) {
            return ({
                let _w: i32 = ((elem!(coeffs, offset1).read()) as i32);
                let _n: i32 = ((elem!(coeffs, offset2).read()) as i32);
                let _nw: i32 = ((elem!(coeffs, offset3).read()) as i32);
                AdaptiveMedian_114(_w, _n, _nw)
            });
        } else {
            return ((elem!(coeffs, offset2).read()) as i32);
        }
    } else {
        return if (x != 0) {
            ((elem!(coeffs, offset1).read()) as i32)
        } else {
            0
        };
    }
    panic!("ub: non-void function does not return a value")
}
thread_local!(
    pub static kQFactorBits_116: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!(
    pub static kQFactorLimit_117: Value<usize> = Rc::new(RefCell::new(
        ((1_u32 << kQFactorBits_116.with(|rc| *rc.borrow())) as usize),
    ));
);
pub fn FillQuantMatrix_118(mut is_chroma: bool, mut q: u32, mut dst: Ptr<u8>) {
    if !((q >= 0_u32) && ((q as usize) < kQFactorLimit_117.with(|rc| *rc.borrow()))) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"quant_matrix.cc"),
                18,
                Ptr::<i8>::from_string_literal(b"FillQuantMatrix"),
            )
        });
        'loop_: while true {}
    };
    let mut in_: Ptr<u8> = (((kDefaultQuantMatrix_12.with(|v| v.as_pointer())
        as Ptr<Value<Box<[u8]>>>)
        .offset(is_chroma)
        .read()
        .as_pointer()) as Ptr<u8>);
    let mut i: i32 = 0;
    'loop_: while (i < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let mut v: u32 =
            (((((elem!(in_, i).read()) as u32).wrapping_mul(q)).wrapping_add(32_u32)) >> 6);
        elem!(dst, i).write({
            (if (v < 1_u32) {
                1_u32
            } else {
                if (v > 255_u32) { 255_u32 } else { v }
            } as u8)
        });
        i.prefix_inc();
    }
}
pub fn FindBestMatrix_119(mut src: Ptr<i32>, mut is_chroma: bool, mut dst: Ptr<u8>) -> u32 {
    let mut best_q: u32 = 0_u32;
    let mut kMaxDiffCost: usize = 33_usize;
    let mut kWorstLen: usize = ((kDCTBlockSize_3.with(|rc| *rc.borrow()) + 1) as usize)
        .wrapping_mul((((kMaxDiffCost).wrapping_add(1_usize)) as usize));
    let mut best_len: usize = kWorstLen;
    let mut q: u32 = 0_u32;
    'loop_: while ((q as usize) < kQFactorLimit_117.with(|rc| *rc.borrow())) {
        ({ FillQuantMatrix_118(is_chroma, q, (dst).clone()) });
        let mut last_diff: i32 = 0;
        let mut len: usize = 0_usize;
        let mut k: i32 = 0;
        'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let mut j: i32 = (({
                let __idx = (k) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            }) as i32);
            let mut new_diff: i32 =
                ({ (elem!(src, j).read()) } - { ((elem!(dst, j).read()) as i32) });
            let mut diff: i32 = (new_diff - last_diff);
            last_diff = new_diff;
            if (diff != 0) {
                len = { (len).wrapping_add(1_usize) };
                if (diff < 0) {
                    diff = { -diff };
                }
                diff -= 1;
                if (diff == 0) {
                    len.postfix_inc();
                } else if (diff > 65535) {
                    len = kWorstLen;
                    break;
                } else {
                    let mut diff_len: u32 = ((({ Log2FloorNonZero_74((diff as u32)) }) + 1) as u32);
                    if (diff_len == 16_u32) {
                        diff_len.postfix_dec();
                    }
                    len = {
                        (len).wrapping_add(
                            ((((2_u32).wrapping_mul(diff_len)).wrapping_add(1_u32)) as usize),
                        )
                    };
                }
            }
            k.prefix_inc();
        }
        if (len < best_len) {
            best_len = len;
            best_q = q;
        }
        q.prefix_inc();
    }
    ({ FillQuantMatrix_118(is_chroma, best_q, (dst).clone()) });
    return best_q;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct brunsli_Storage {
    #[offset(0)]
    #[byte_size(8)]
    pub data: Ptr<u8>,
    #[offset(8)]
    pub length: usize,
    #[offset(16)]
    pub pos: usize,
}
impl brunsli_Storage {}
pub fn WriteBits_120(mut n_bits: usize, mut bits: u64, mut storage: Ptr<brunsli_Storage>) {
    {}
    if !((bits >> n_bits) == 0_u64) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                58,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    if !(n_bits <= 56_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                59,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    if !({ (((storage.with(|__s| __s.pos)).wrapping_add(n_bits)) >> 3).wrapping_add(7_usize) } < {
        storage.with(|__s| __s.length)
    }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                61,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    let mut p: Ptr<u8> = storage
        .with(|__s| __s.data.clone())
        .offset((storage.with(|__s| __s.pos) >> 3) as isize);
    let mut v: u64 = ((p.read()) as u64);
    v |= ({ bits } << { (storage.with(|__s| __s.pos) & 7_usize) });
    ({ BrunsliUnalignedWrite64_70((p).to_any(), v) });
    field!(storage, pos).write({ (storage.with(|__s| __s.pos)).wrapping_add(n_bits) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct brunsli_ANSEncSymbolInfo {
    #[offset(0)]
    pub freq_: u16,
    #[offset(2)]
    pub start_: u16,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(72)]
pub struct brunsli_ANSTable {
    #[offset(0)]
    #[byte_size(72)]
    pub info_: Value<Box<[brunsli_ANSEncSymbolInfo]>>,
}
impl Default for brunsli_ANSTable {
    fn default() -> Self {
        brunsli_ANSTable {
            info_: Rc::new(RefCell::new(
                (0..18)
                    .map(|_| <brunsli_ANSEncSymbolInfo>::default())
                    .collect::<Box<[brunsli_ANSEncSymbolInfo]>>(),
            )),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct brunsli_ANSCoder {
    #[offset(0)]
    state_: u32,
}
impl brunsli_ANSCoder {
    pub fn new() -> Self {
        Self {
            state_: (19_u32 << 16),
        }
    }
}
impl Default for brunsli_ANSCoder {
    fn default() -> Self {
        { brunsli_ANSCoder::new() }
    }
}
thread_local!(
    pub static kMaxNumSymbolsForSmallCode_121: Value<i32> = Rc::new(RefCell::new(4));
);
pub fn ANSBuildInfoTable_122(
    mut counts: Ptr<i32>,
    mut alphabet_size: i32,
    mut info: Ptr<brunsli_ANSEncSymbolInfo>,
) {
    let mut total: i32 = 0;
    let mut s: i32 = 0;
    'loop_: while (s < alphabet_size) {
        let mut freq: u32 = ((elem!(counts, s).read()) as u32);
        field!(elem!(info, s), freq_).write({ ((elem!(counts, s).read()) as u16) });
        field!(elem!(info, s), start_).write((total as u16));
        total = { ((total as u32).wrapping_add(freq)) as i32 };
        s.prefix_inc();
    }
}
pub fn BuildAndStoreANSEncodingData_123(
    mut histogram: Ptr<i32>,
    mut table: Ptr<brunsli_ANSTable>,
    mut storage: Ptr<brunsli_Storage>,
) {
    let num_symbols: Value<i32> = Rc::new(RefCell::new(0_i32));
    let symbols: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
    let counts: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = histogram.offset((18) as isize).get_offset() - histogram.get_offset();
        PtrValueIter::new(&histogram, __count).collect::<Vec<_>>()
    }));
    let omit_pos: Value<i32> = Rc::new(RefCell::new(0));
    ({
        NormalizeCounts_124(
            ((counts.as_pointer() as Ptr<i32>).offset(0_usize)),
            (omit_pos.as_pointer()),
            18,
            BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()),
            (num_symbols.as_pointer()),
            (symbols.as_pointer() as Ptr<i32>),
        )
    });
    ({
        ANSBuildInfoTable_122(
            ((counts.as_pointer() as Ptr<i32>).offset(0_usize)),
            18,
            (array_field_ptr!(table, info_) as Ptr<brunsli_ANSEncSymbolInfo>),
        )
    });
    ({
        EncodeCounts_125(
            ((counts.as_pointer() as Ptr<i32>).offset(0_usize)),
            (*omit_pos.borrow()),
            (*num_symbols.borrow()),
            (symbols.as_pointer() as Ptr<i32>),
            (storage).clone(),
        )
    });
}
thread_local!(
    pub static kLog2Table_126: Value<Box<[f32]>> = Rc::new(RefCell::new(Box::new([
        0.0E+0,
        0.0E+0,
        1.0E+0,
        1.584962487E+0,
        2.0E+0,
        2.321928024E+0,
        2.584962606E+0,
        2.807354927E+0,
        3.0E+0,
        3.169924974E+0,
        3.321928024E+0,
        3.459431648E+0,
        3.584962606E+0,
        3.700439692E+0,
        3.807354927E+0,
        3.906890631E+0,
        4.0E+0,
        4.087462902E+0,
        4.169925213E+0,
        4.247927666E+0,
        4.321928024E+0,
        4.392317295E+0,
        4.459431648E+0,
        4.523561954E+0,
        4.584962368E+0,
        4.643856049E+0,
        4.70043993E+0,
        4.754887581E+0,
        4.807354927E+0,
        4.857981205E+0,
        4.906890392E+0,
        4.954196453E+0,
        5.0E+0,
        5.044394016E+0,
        5.087462902E+0,
        5.129282951E+0,
        5.169925213E+0,
        5.209453583E+0,
        5.247927666E+0,
        5.285402298E+0,
        5.321928024E+0,
        5.357552052E+0,
        5.392317295E+0,
        5.426264763E+0,
        5.459431648E+0,
        5.491853237E+0,
        5.523561954E+0,
        5.554588795E+0,
        5.584962368E+0,
        5.614709854E+0,
        5.643856049E+0,
        5.67242527E+0,
        5.70043993E+0,
        5.727920532E+0,
        5.754887581E+0,
        5.781359673E+0,
        5.807354927E+0,
        5.832890034E+0,
        5.857981205E+0,
        5.882643223E+0,
        5.906890392E+0,
        5.930737495E+0,
        5.954196453E+0,
        5.97728014E+0,
        6.0E+0,
        6.022367954E+0,
        6.044394016E+0,
        6.066089153E+0,
        6.087462902E+0,
        6.108524323E+0,
        6.129282951E+0,
        6.149746895E+0,
        6.169925213E+0,
        6.189824581E+0,
        6.209453583E+0,
        6.228818893E+0,
        6.247927666E+0,
        6.266786575E+0,
        6.285402298E+0,
        6.303780556E+0,
        6.321928024E+0,
        6.339849949E+0,
        6.357552052E+0,
        6.375039577E+0,
        6.392317295E+0,
        6.409390926E+0,
        6.426264763E+0,
        6.442943573E+0,
        6.459431648E+0,
        6.47573328E+0,
        6.491853237E+0,
        6.507794857E+0,
        6.523561954E+0,
        6.539158821E+0,
        6.554588795E+0,
        6.56985569E+0,
        6.584962368E+0,
        6.599912643E+0,
        6.614709854E+0,
        6.629356384E+0,
        6.643856049E+0,
        6.658211708E+0,
        6.67242527E+0,
        6.686500549E+0,
        6.70043993E+0,
        6.714245319E+0,
        6.727920532E+0,
        6.741466999E+0,
        6.754887581E+0,
        6.768184185E+0,
        6.781359673E+0,
        6.794415951E+0,
        6.807354927E+0,
        6.820178986E+0,
        6.832890034E+0,
        6.845489979E+0,
        6.857981205E+0,
        6.870364666E+0,
        6.882643223E+0,
        6.894817829E+0,
        6.906890392E+0,
        6.918863297E+0,
        6.930737495E+0,
        6.94251442E+0,
        6.954196453E+0,
        6.965784073E+0,
        6.97728014E+0,
        6.988684654E+0,
        7.0E+0,
        7.011227131E+0,
        7.022367954E+0,
        7.033422947E+0,
        7.044394016E+0,
        7.055282593E+0,
        7.066089153E+0,
        7.076815605E+0,
        7.087462902E+0,
        7.098031998E+0,
        7.108524323E+0,
        7.118941307E+0,
        7.129282951E+0,
        7.139551163E+0,
        7.149746895E+0,
        7.159871101E+0,
        7.169925213E+0,
        7.179909229E+0,
        7.189824581E+0,
        7.199672222E+0,
        7.209453583E+0,
        7.219168663E+0,
        7.228818893E+0,
        7.238404751E+0,
        7.247927666E+0,
        7.257387638E+0,
        7.266786575E+0,
        7.276124477E+0,
        7.285402298E+0,
        7.294620514E+0,
        7.303780556E+0,
        7.3128829E+0,
        7.321928024E+0,
        7.330916882E+0,
        7.339849949E+0,
        7.34872818E+0,
        7.357552052E+0,
        7.366322041E+0,
        7.375039577E+0,
        7.383704185E+0,
        7.392317295E+0,
        7.400879383E+0,
        7.409390926E+0,
        7.417852402E+0,
        7.426264763E+0,
        7.43462801E+0,
        7.442943573E+0,
        7.451210976E+0,
        7.459431648E+0,
        7.467605591E+0,
        7.47573328E+0,
        7.48381567E+0,
        7.491853237E+0,
        7.499845982E+0,
        7.507794857E+0,
        7.515699863E+0,
        7.523561954E+0,
        7.531381607E+0,
        7.539158821E+0,
        7.54689455E+0,
        7.554588795E+0,
        7.562242508E+0,
        7.56985569E+0,
        7.577428818E+0,
        7.584962368E+0,
        7.592456818E+0,
        7.599912643E+0,
        7.607330322E+0,
        7.614709854E+0,
        7.622051716E+0,
        7.629356384E+0,
        7.636624813E+0,
        7.643856049E+0,
        7.651051521E+0,
        7.658211708E+0,
        7.665336132E+0,
        7.67242527E+0,
        7.679480076E+0,
        7.686500549E+0,
        7.693487167E+0,
        7.70043993E+0,
        7.707359314E+0,
        7.714245319E+0,
        7.721099377E+0,
        7.727920532E+0,
        7.73470974E+0,
        7.741466999E+0,
        7.748192787E+0,
        7.754887581E+0,
        7.76155138E+0,
        7.768184185E+0,
        7.774786949E+0,
        7.781359673E+0,
        7.787902355E+0,
        7.794415951E+0,
        7.800899982E+0,
        7.807354927E+0,
        7.813781261E+0,
        7.820178986E+0,
        7.826548576E+0,
        7.832890034E+0,
        7.839203835E+0,
        7.845489979E+0,
        7.851748943E+0,
        7.857981205E+0,
        7.864186287E+0,
        7.870364666E+0,
        7.876516819E+0,
        7.882643223E+0,
        7.888743401E+0,
        7.894817829E+0,
        7.900866985E+0,
        7.906890392E+0,
        7.912889481E+0,
        7.918863297E+0,
        7.924812317E+0,
        7.930737495E+0,
        7.936637878E+0,
        7.94251442E+0,
        7.948367119E+0,
        7.954196453E+0,
        7.960001945E+0,
        7.965784073E+0,
        7.971543789E+0,
        7.97728014E+0,
        7.982993603E+0,
        7.988684654E+0,
        7.994353294E+0,
    ])));
);
pub fn FastLog2_127(mut v: i32) -> f64 {
    if (v
        < (((::std::mem::size_of::<[f32; 256]>() as usize)
            .wrapping_div((::std::mem::size_of::<f32>() as usize))) as i32))
    {
        return (({
            let __idx = (v) as usize;
            kLog2Table_126.with(|rc| rc.borrow()[__idx])
        }) as f64);
    }
    return (v as f64).log2();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct brunsli_HistogramPair {
    #[offset(0)]
    pub idx1: usize,
    #[offset(8)]
    pub idx2: usize,
    #[offset(16)]
    pub cost_combo: f64,
    #[offset(24)]
    pub cost_diff: f64,
}
impl std::cmp::Ord for brunsli_HistogramPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_128(
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: self.idx1.clone(),
                    idx2: self.idx2.clone(),
                    cost_combo: self.cost_combo.clone(),
                    cost_diff: self.cost_diff.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: other.idx1.clone(),
                    idx2: other.idx2.clone(),
                    cost_combo: other.cost_combo.clone(),
                    cost_diff: other.cost_diff.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_128(
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: other.idx1.clone(),
                    idx2: other.idx2.clone(),
                    cost_combo: other.cost_combo.clone(),
                    cost_diff: other.cost_diff.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: self.idx1.clone(),
                    idx2: self.idx2.clone(),
                    cost_combo: self.cost_combo.clone(),
                    cost_diff: self.cost_diff.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for brunsli_HistogramPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for brunsli_HistogramPair {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_128(
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: self.idx1.clone(),
                    idx2: self.idx2.clone(),
                    cost_combo: self.cost_combo.clone(),
                    cost_diff: self.cost_diff.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: other.idx1.clone(),
                    idx2: other.idx2.clone(),
                    cost_combo: other.cost_combo.clone(),
                    cost_diff: other.cost_diff.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_128(
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: other.idx1.clone(),
                    idx2: other.idx2.clone(),
                    cost_combo: other.cost_combo.clone(),
                    cost_diff: other.cost_diff.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(brunsli_HistogramPair {
                    idx1: self.idx1.clone(),
                    idx2: self.idx2.clone(),
                    cost_combo: self.cost_combo.clone(),
                    cost_diff: self.cost_diff.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for brunsli_HistogramPair {}
pub fn operator_lt_128(p1: Ptr<brunsli_HistogramPair>, p2: Ptr<brunsli_HistogramPair>) -> bool {
    if ({ p1.with(|__s| __s.cost_diff) } != { p2.with(|__s| __s.cost_diff) }) {
        return ({ p1.with(|__s| __s.cost_diff) } > { p2.with(|__s| __s.cost_diff) });
    }
    if !({ p1.with(|__s| __s.idx1) } < { p1.with(|__s| __s.idx2) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                35,
                Ptr::<i8>::from_string_literal(b"operator<"),
            )
        });
        'loop_: while true {}
    };
    if !({ p2.with(|__s| __s.idx1) } < { p2.with(|__s| __s.idx2) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                36,
                Ptr::<i8>::from_string_literal(b"operator<"),
            )
        });
        'loop_: while true {}
    };
    return ({ ((p1.with(|__s| __s.idx2)).wrapping_sub(p1.with(|__s| __s.idx1))) } > {
        ((p2.with(|__s| __s.idx2)).wrapping_sub(p2.with(|__s| __s.idx1)))
    });
}
pub fn ClusterCostDiff_129(mut size_a: i32, mut size_b: i32) -> f64 {
    let mut size_c: i32 = (size_a + size_b);
    return ((((size_a as f64) * ({ FastLog2_127(size_a) }))
        + ((size_b as f64) * ({ FastLog2_127(size_b) })))
        - ((size_c as f64) * ({ FastLog2_127(size_c) })));
}
pub fn PopulationCost_130(h: Ptr<brunsli_internal_enc_Histogram>) -> f64 {
    return ({
        let _data: Ptr<i32> = ((array_field_ptr!(h, data_) as Ptr<i32>).offset((0) as isize));
        let _total_count: i32 = h.with(|__s| __s.total_count_);
        PopulationCost_131(_data, _total_count)
    });
}
pub fn CompareAndPushToQueue_132(
    mut out: Ptr<brunsli_internal_enc_Histogram>,
    mut cluster_size: Ptr<i32>,
    idx1: i32,
    idx2: i32,
    mut pairs: Ptr<Vec<brunsli_HistogramPair>>,
) {
    let idx1: Value<i32> = Rc::new(RefCell::new(idx1));
    let idx2: Value<i32> = Rc::new(RefCell::new(idx2));
    if ((*idx1.borrow()) == (*idx2.borrow())) {
        return;
    };
    if ((*idx2.borrow()) < (*idx1.borrow())) {
        {
            let tmp = idx1.as_pointer().read();
            idx1.as_pointer().write(idx2.as_pointer().read());
            idx2.as_pointer().write(tmp);
        };
    }
    let mut store_pair: bool = false;
    let p: Value<brunsli_HistogramPair> = Rc::new(RefCell::new(<brunsli_HistogramPair>::default()));
    (*p.borrow_mut()).idx1 = ((*idx1.borrow()) as usize);
    (*p.borrow_mut()).idx2 = ((*idx2.borrow()) as usize);
    (*p.borrow_mut()).cost_diff = (5.0E-1
        * ({
            let _size_a: i32 = (elem!(cluster_size, (*idx1.borrow())).read());
            let _size_b: i32 = (elem!(cluster_size, (*idx2.borrow())).read());
            ClusterCostDiff_129(_size_a, _size_b)
        }));
    (*p.borrow_mut()).cost_diff -= { (*elem!(out, (*idx1.borrow())).upgrade().deref()).bit_cost_ };
    (*p.borrow_mut()).cost_diff -= { (*elem!(out, (*idx2.borrow())).upgrade().deref()).bit_cost_ };
    if ({ (*elem!(out, (*idx1.borrow())).upgrade().deref()).total_count_ } == 0) {
        (*p.borrow_mut()).cost_combo =
            { (*elem!(out, (*idx2.borrow())).upgrade().deref()).bit_cost_ };
        store_pair = true;
    } else if ({ (*elem!(out, (*idx2.borrow())).upgrade().deref()).total_count_ } == 0) {
        (*p.borrow_mut()).cost_combo =
            { (*elem!(out, (*idx1.borrow())).upgrade().deref()).bit_cost_ };
        store_pair = true;
    } else {
        let mut threshold: f64 = if (*pairs.upgrade().deref()).is_empty() {
            1.0E+99
        } else {
            {
                let __tmp_0: Value<f64> = Rc::new(RefCell::new(0.0E+0));
                (if __tmp_0.as_pointer().read()
                    >= field_ptr!(
                        ((Ptr::<Vec<brunsli_HistogramPair>>::decay(&(pairs)))
                            as Ptr<brunsli_HistogramPair>)
                            .offset(0_usize),
                        cost_diff
                    )
                    .read()
                {
                    __tmp_0.as_pointer()
                } else {
                    field_ptr!(
                        ((Ptr::<Vec<brunsli_HistogramPair>>::decay(&(pairs)))
                            as Ptr<brunsli_HistogramPair>)
                            .offset(0_usize),
                        cost_diff
                    )
                }
                .read())
            }
        };
        let combo: Value<brunsli_internal_enc_Histogram> = Rc::new(RefCell::new(
            (*elem!(out, (*idx1.borrow())).upgrade().deref()).clone(),
        ));
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> = out.offset((*idx2.borrow()) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(&combo.as_pointer(), _other)
        });
        let mut cost_combo: f64 = ({ PopulationCost_130(combo.as_pointer()) });
        if (cost_combo < (threshold - { (*p.borrow()).cost_diff })) {
            (*p.borrow_mut()).cost_combo = cost_combo;
            store_pair = true;
        }
    }
    if store_pair {
        (*p.borrow_mut()).cost_diff += { { (*p.borrow()).cost_combo } };
        if (!((*pairs.upgrade().deref()).is_empty()))
            && ({
                let _p1: Ptr<brunsli_HistogramPair> =
                    (Ptr::<Vec<brunsli_HistogramPair>>::decay(&(pairs))
                        as Ptr<brunsli_HistogramPair>);
                operator_lt_128(_p1, p.as_pointer())
            })
        {
            {
                let a0_clone = (*(Ptr::<Vec<brunsli_HistogramPair>>::decay(&(pairs))
                    as Ptr<brunsli_HistogramPair>)
                    .upgrade()
                    .deref())
                .clone();
                pairs.with_mut(|__v: &mut Vec<brunsli_HistogramPair>| __v.push(a0_clone))
            };
            (Ptr::<Vec<brunsli_HistogramPair>>::decay(&(pairs)) as Ptr<brunsli_HistogramPair>)
                .write((*p.borrow()).clone());
        } else {
            {
                let a0_clone = (*p.borrow()).clone();
                pairs.with_mut(|__v: &mut Vec<brunsli_HistogramPair>| __v.push(a0_clone))
            };
        }
    }
}
pub fn HistogramCombine_133(
    mut out: Ptr<brunsli_internal_enc_Histogram>,
    mut cluster_size: Ptr<i32>,
    mut symbols: Ptr<u32>,
    mut symbols_size: usize,
    mut max_clusters: usize,
) -> usize {
    let mut cost_diff_threshold: f64 = 0.0E+0;
    let mut min_cluster_size: usize = 1_usize;
    let clusters: Value<Vec<u64>> = Rc::new(RefCell::new({
        let __count = symbols.offset((symbols_size) as isize).get_offset() - symbols.get_offset();
        PtrValueIter::new(&symbols, __count)
            .map(|item| u64::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    (clusters.as_pointer() as Ptr<u64>)
        .sort((clusters.as_pointer() as Ptr<u64>).to_end().get_offset());
    {
        let __a0 = ((({
            let count = (clusters.as_pointer() as Ptr<u64>).to_end().get_offset()
                - (clusters.as_pointer() as Ptr<u64>).get_offset();
            if count <= 1 {
                (clusters.as_pointer() as Ptr<u64>).to_end()
            } else {
                let mut iter = PtrValueIter::new(&(clusters.as_pointer() as Ptr<u64>), count);
                let mut write_ptr = (clusters.as_pointer() as Ptr<u64>);
                let mut last_unique = iter.next().unwrap();

                // the first unique value is already in place
                write_ptr += 1;

                for current_val in iter {
                    if current_val != last_unique {
                        write_ptr.write(current_val.clone());
                        last_unique = current_val;
                        write_ptr += 1;
                    }
                }
                write_ptr
            }
        }
        .get_offset() as isize)
            - ((clusters.as_pointer() as Ptr<u64>).get_offset() as isize))
            as usize) as usize;
        (*clusters.borrow_mut()).resize_with(__a0, || <u64>::default())
    };
    let pairs: Value<Vec<brunsli_HistogramPair>> = Rc::new(RefCell::new(Vec::new()));
    if (((*clusters.borrow()).len())
        .wrapping_mul((((*clusters.borrow()).len()).wrapping_add(1_usize))))
    .wrapping_div(2_usize) as usize
        > (*pairs.borrow()).capacity() as usize
    {
        let len_0 = (*pairs.borrow()).len();
        (*pairs.borrow_mut()).reserve_exact(
            (((*clusters.borrow()).len())
                .wrapping_mul((((*clusters.borrow()).len()).wrapping_add(1_usize))))
            .wrapping_div(2_usize) as usize
                - len_0 as usize,
        );
    };
    let mut idx1: usize = 0_usize;
    'loop_: while (idx1 < (*clusters.borrow()).len()) {
        let mut idx2: usize = (idx1).wrapping_add(1_usize);
        'loop_: while (idx2 < (*clusters.borrow()).len()) {
            ({
                let _cluster_size: Ptr<i32> = (cluster_size).clone();
                let _idx1: i32 = ({ (*clusters.borrow())[idx1] } as i32);
                let _idx2: i32 = ({ (*clusters.borrow())[idx2] } as i32);
                CompareAndPushToQueue_132(
                    (out).clone(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (pairs.as_pointer()),
                )
            });
            idx2.prefix_inc();
        }
        idx1.prefix_inc();
    }
    'loop_: while ((*clusters.borrow()).len() > min_cluster_size) {
        if ({ (*pairs.borrow())[0_usize].cost_diff } >= cost_diff_threshold) {
            cost_diff_threshold = 1.0E+99;
            min_cluster_size = max_clusters;
            continue 'loop_;
        }
        let mut best_idx1: usize = { (*pairs.borrow())[0_usize].idx1 };
        let mut best_idx2: usize = { (*pairs.borrow())[0_usize].idx2 };
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> = out.offset((best_idx2) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(
                &out.offset((best_idx1) as isize),
                _other,
            )
        });
        let __rhs = { (*pairs.borrow())[0_usize].cost_combo };
        field!(elem!(out, best_idx1), bit_cost_).write(__rhs);
        {
            let __rhs = { (elem!(cluster_size, best_idx2).read()) };
            elem!(cluster_size, best_idx1).with_mut(|__v| *__v = *__v + __rhs)
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < symbols_size) {
            if ({ ((elem!(symbols, i).read()) as usize) } == { best_idx2 }) {
                elem!(symbols, i).write({ (best_idx1 as u32) });
            }
            i.prefix_inc();
        }
        let cluster: Value<Ptr<u64>> = Rc::new(RefCell::new((clusters.as_pointer() as Ptr<u64>)));
        'loop_: while (*cluster.borrow()) != (clusters.as_pointer() as Ptr<u64>).to_end() {
            if ((((*cluster.borrow()).read()) as usize) >= best_idx2) {
                {
                    let idx = (*cluster.borrow()).get_offset();
                    (clusters.as_pointer() as Ptr<Vec<u64>>)
                        .with_mut(|__v: &mut Vec<u64>| __v.remove(idx));
                    (clusters.as_pointer() as Ptr<Vec<u64>>).decay()
                };
                break;
            }
            (*cluster.borrow_mut()).prefix_inc();
        }
        let copy_to: Value<Ptr<brunsli_HistogramPair>> = Rc::new(RefCell::new(
            (pairs.as_pointer() as Ptr<brunsli_HistogramPair>),
        ));
        'loop_: for mut p in pairs.as_pointer() as Ptr<brunsli_HistogramPair> {
            if ((({ p.with(|__s| __s.idx1) } == { best_idx1 })
                || ({ p.with(|__s| __s.idx2) } == { best_idx1 }))
                || ({ p.with(|__s| __s.idx1) } == { best_idx2 }))
                || ({ p.with(|__s| __s.idx2) } == { best_idx2 })
            {
                continue 'loop_;
            }
            if ({
                let _p1: Ptr<brunsli_HistogramPair> =
                    (pairs.as_pointer() as Ptr<brunsli_HistogramPair>);
                let _p2: Ptr<brunsli_HistogramPair> = (p).clone();
                operator_lt_128(_p1, _p2)
            }) {
                let mut front: brunsli_HistogramPair = (*(pairs.as_pointer()
                    as Ptr<brunsli_HistogramPair>)
                    .upgrade()
                    .deref())
                .clone();
                let __rhs = (*p.upgrade().deref()).clone();
                (pairs.as_pointer() as Ptr<brunsli_HistogramPair>).write(__rhs);
                (*copy_to.borrow()).write((front).clone());
            } else {
                let __rhs = (*p.upgrade().deref()).clone();
                (*copy_to.borrow()).write(__rhs);
            }
            (*copy_to.borrow_mut()).prefix_inc();
        }
        {
            let __a0 = ((((*copy_to.borrow()).get_offset() as isize)
                - ((pairs.as_pointer() as Ptr<brunsli_HistogramPair>).get_offset() as isize))
                as usize) as usize;
            (*pairs.borrow_mut()).resize_with(__a0, || <brunsli_HistogramPair>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < (*clusters.borrow()).len()) {
            ({
                let _cluster_size: Ptr<i32> = (cluster_size).clone();
                let _idx1: i32 = (best_idx1 as i32);
                let _idx2: i32 = ({ (*clusters.borrow())[i] } as i32);
                CompareAndPushToQueue_132(
                    (out).clone(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (pairs.as_pointer()),
                )
            });
            i.prefix_inc();
        }
    }
    return (*clusters.borrow()).len();
}
pub fn HistogramBitCostDistance_134(
    histogram: Ptr<brunsli_internal_enc_Histogram>,
    candidate: Ptr<brunsli_internal_enc_Histogram>,
) -> f64 {
    if (histogram.with(|__s| __s.total_count_) == 0) {
        return 0.0E+0;
    }
    let tmp: Value<brunsli_internal_enc_Histogram> =
        Rc::new(RefCell::new((*histogram.upgrade().deref()).clone()));
    ({
        let _other: Ptr<brunsli_internal_enc_Histogram> = (candidate).clone();
        brunsli_internal_enc_HistogramImpl::AddHistogram(&tmp.as_pointer(), _other)
    });
    return ({ ({ PopulationCost_130(tmp.as_pointer()) }) } - {
        candidate.with(|__s| __s.bit_cost_)
    });
}
pub fn HistogramRemap_135(
    mut in_: Ptr<brunsli_internal_enc_Histogram>,
    mut in_size: usize,
    mut out: Ptr<brunsli_internal_enc_Histogram>,
    mut symbols: Ptr<u32>,
) {
    let all_symbols: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = symbols.offset((in_size) as isize).get_offset() - symbols.get_offset();
        PtrValueIter::new(&symbols, __count)
            .map(|item| i32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    (all_symbols.as_pointer() as Ptr<i32>)
        .sort((all_symbols.as_pointer() as Ptr<i32>).to_end().get_offset());
    {
        let __a0 = ((({
            let count = (all_symbols.as_pointer() as Ptr<i32>).to_end().get_offset()
                - (all_symbols.as_pointer() as Ptr<i32>).get_offset();
            if count <= 1 {
                (all_symbols.as_pointer() as Ptr<i32>).to_end()
            } else {
                let mut iter = PtrValueIter::new(&(all_symbols.as_pointer() as Ptr<i32>), count);
                let mut write_ptr = (all_symbols.as_pointer() as Ptr<i32>);
                let mut last_unique = iter.next().unwrap();

                // the first unique value is already in place
                write_ptr += 1;

                for current_val in iter {
                    if current_val != last_unique {
                        write_ptr.write(current_val.clone());
                        last_unique = current_val;
                        write_ptr += 1;
                    }
                }
                write_ptr
            }
        }
        .get_offset() as isize)
            - ((all_symbols.as_pointer() as Ptr<i32>).get_offset() as isize))
            as usize) as usize;
        (*all_symbols.borrow_mut()).resize_with(__a0, || <i32>::default())
    };
    let mut i: usize = 0_usize;
    'loop_: while (i < in_size) {
        let mut best_out: i32 = (if (i == 0_usize) {
            (elem!(symbols, 0).read())
        } else {
            (elem!(symbols, (i).wrapping_sub(1_usize)).read())
        } as i32);
        let mut best_bits: f64 = ({
            let _histogram: Ptr<brunsli_internal_enc_Histogram> = in_.offset((i) as isize);
            let _candidate: Ptr<brunsli_internal_enc_Histogram> = out.offset((best_out) as isize);
            HistogramBitCostDistance_134(_histogram, _candidate)
        });
        'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
            let mut k: i32 = k.read();
            let mut cur_bits: f64 = ({
                let _histogram: Ptr<brunsli_internal_enc_Histogram> = in_.offset((i) as isize);
                let _candidate: Ptr<brunsli_internal_enc_Histogram> = out.offset((k) as isize);
                HistogramBitCostDistance_134(_histogram, _candidate)
            });
            if (cur_bits < best_bits) {
                best_bits = cur_bits;
                best_out = k;
            }
        }
        elem!(symbols, i).write({ (best_out as u32) });
        i.prefix_inc();
    }
    'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
        let mut k: i32 = k.read();
        ({ brunsli_internal_enc_HistogramImpl::Clear(&out.offset((k) as isize)) });
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < in_size) {
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> = in_.offset((i) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(
                &out.offset((elem!(symbols, i).read()) as isize),
                _other,
            )
        });
        i.prefix_inc();
    }
}
pub fn HistogramReindex_136(
    mut out: Ptr<Vec<brunsli_internal_enc_Histogram>>,
    mut symbols: Ptr<Vec<u32>>,
) {
    let mut tmp: Vec<brunsli_internal_enc_Histogram> = (*out.upgrade().deref()).clone();
    let new_index: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let mut next_index: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*symbols.upgrade().deref()).len() }) {
        if RefcountMapIter::find_key(
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
            &((elem!(((Ptr::<Vec<u32>>::decay(&(symbols))) as Ptr<u32>), i).read()) as i32),
        ) == RefcountMapIter::end((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>))
        {
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry(
                        ((elem!(((Ptr::<Vec<u32>>::decay(&(symbols))) as Ptr<u32>), i).read())
                            as i32),
                    )
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
                })
                .write(next_index);
            let __rhs = (tmp
                [((elem!(((Ptr::<Vec<u32>>::decay(&(symbols))) as Ptr<u32>), i).read()) as usize)])
                .clone();
            elem!(
                ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                    as Ptr<brunsli_internal_enc_Histogram>),
                (next_index as usize)
            )
            .write(__rhs);
            next_index.prefix_inc();
        }
        i.prefix_inc();
    }
    {
        let __a0 = (next_index as usize) as usize;
        out.with_mut(|__v: &mut Vec<brunsli_internal_enc_Histogram>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        })
    };
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*symbols.upgrade().deref()).len() }) {
        let __rhs = (((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry(
                    ((elem!(((Ptr::<Vec<u32>>::decay(&(symbols))) as Ptr<u32>), i).read()) as i32),
                )
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
            })
            .read()) as u32);
        elem!(((Ptr::<Vec<u32>>::decay(&(symbols))) as Ptr<u32>), i).write(__rhs);
        i.prefix_inc();
    }
}
pub fn ClusterHistograms_137(
    in_: Ptr<Vec<brunsli_internal_enc_Histogram>>,
    mut num_contexts: usize,
    mut num_blocks: usize,
    mut block_group_offsets: Vec<u64>,
    mut max_histograms: usize,
    mut out: Ptr<Vec<brunsli_internal_enc_Histogram>>,
    mut histogram_symbols: Ptr<Vec<u32>>,
) {
    let mut in_size: usize = (num_contexts).wrapping_mul(num_blocks);
    let cluster_size: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; in_size as usize]));
    {
        let __a0 = in_size as usize;
        out.with_mut(|__v: &mut Vec<brunsli_internal_enc_Histogram>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        })
    };
    {
        let __a0 = in_size as usize;
        histogram_symbols.with_mut(|__v: &mut Vec<u32>| __v.resize_with(__a0, || <u32>::default()))
    };
    let mut i: usize = 0_usize;
    'loop_: while (i < in_size) {
        let __rhs = (*elem!(
            (Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                as Ptr<brunsli_internal_enc_Histogram>),
            i
        )
        .upgrade()
        .deref())
        .clone();
        elem!(
            ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                as Ptr<brunsli_internal_enc_Histogram>),
            i
        )
        .write(__rhs);
        let __rhs = ({
            PopulationCost_130(
                (Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(i),
            )
        });
        field!(
            elem!(
                ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                    as Ptr<brunsli_internal_enc_Histogram>),
                i
            ),
            bit_cost_
        )
        .write(__rhs);
        let __rhs = (i as u32);
        elem!(
            ((Ptr::<Vec<u32>>::decay(&(histogram_symbols))) as Ptr<u32>),
            i
        )
        .write(__rhs);
        i.prefix_inc();
    }
    if (num_contexts > 1_usize) {
        let mut i: usize = 0_usize;
        'loop_: while (i < num_blocks) {
            ({
                let _symbols: Ptr<u32> = (((Ptr::<Vec<u32>>::decay(&(histogram_symbols)))
                    as Ptr<u32>)
                    .offset((i).wrapping_mul(num_contexts)));
                let _symbols_size: usize = num_contexts;
                HistogramCombine_133(
                    (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                    _symbols,
                    _symbols_size,
                    max_histograms,
                )
            });
            i.prefix_inc();
        }
    }
    thread_local!(
        static kMinClustersForHistogramRemap_138: Value<usize> = Rc::new(RefCell::new(24_usize));
    );
    let mut num_clusters: usize = 0_usize;
    if (block_group_offsets.len() > 1_usize) {
        let mut i: usize = 0_usize;
        'loop_: while (i < block_group_offsets.len()) {
            let mut offset: usize =
                ((block_group_offsets[i]).wrapping_mul((num_contexts as u64)) as usize);
            let mut next_offset: usize = (if ((i).wrapping_add(1_usize) < block_group_offsets.len())
            {
                (block_group_offsets[(i).wrapping_add(1_usize)]).wrapping_mul((num_contexts as u64))
            } else {
                (in_size as u64)
            } as usize);
            let mut length: usize = (next_offset).wrapping_sub(offset);
            let mut nclusters: usize = ({
                HistogramCombine_133(
                    (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                    (((Ptr::<Vec<u32>>::decay(&(histogram_symbols))) as Ptr<u32>).offset(offset)),
                    length,
                    max_histograms,
                )
            });
            if (nclusters >= 2_usize)
                && (nclusters < kMinClustersForHistogramRemap_138.with(|rc| *rc.borrow()))
            {
                ({
                    let _in_: Ptr<brunsli_internal_enc_Histogram> =
                        ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(offset));
                    let _symbols: Ptr<u32> = (((Ptr::<Vec<u32>>::decay(&(histogram_symbols)))
                        as Ptr<u32>)
                        .offset(offset));
                    HistogramRemap_135(
                        _in_,
                        length,
                        (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(0_usize)),
                        _symbols,
                    )
                });
            }
            num_clusters = { (num_clusters).wrapping_add(nclusters) };
            i.prefix_inc();
        }
    }
    if (block_group_offsets.len() <= 1_usize) || (num_clusters > max_histograms) {
        num_clusters = ({
            HistogramCombine_133(
                (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(0_usize)),
                ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                (((Ptr::<Vec<u32>>::decay(&(histogram_symbols))) as Ptr<u32>).offset(0_usize)),
                in_size,
                max_histograms,
            )
        });
        if (num_clusters >= 2_usize)
            && (num_clusters < kMinClustersForHistogramRemap_138.with(|rc| *rc.borrow()))
        {
            ({
                HistogramRemap_135(
                    ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    in_size,
                    (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(out)))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    (((Ptr::<Vec<u32>>::decay(&(histogram_symbols))) as Ptr<u32>).offset(0_usize)),
                )
            });
        }
    }
    ({ HistogramReindex_136((out).clone(), (histogram_symbols).clone()) });
}
pub type brunsli_JpegReadMode = u32;
pub const brunsli_JpegReadMode_JPEG_READ_HEADER: brunsli_JpegReadMode = 0;
pub const brunsli_JpegReadMode_JPEG_READ_TABLES: brunsli_JpegReadMode = 1;
pub const brunsli_JpegReadMode_JPEG_READ_ALL: brunsli_JpegReadMode = 2;
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(584)]
pub struct brunsli_internal_enc_ComponentMeta {
    #[offset(0)]
    pub context_offset: usize,
    #[offset(8)]
    pub approx_total_nonzeros: usize,
    #[offset(16)]
    pub h_samp: i32,
    #[offset(20)]
    pub v_samp: i32,
    #[offset(24)]
    pub context_bits: i32,
    #[offset(28)]
    pub ac_stride: i32,
    #[offset(32)]
    pub dc_stride: i32,
    #[offset(36)]
    pub b_stride: i32,
    #[offset(40)]
    pub width_in_blocks: i32,
    #[offset(44)]
    pub height_in_blocks: i32,
    #[offset(48)]
    #[byte_size(8)]
    pub ac_coeffs: Ptr<i16>,
    #[offset(56)]
    #[byte_size(8)]
    pub dc_prediction_errors: Ptr<i16>,
    #[offset(64)]
    #[byte_size(8)]
    pub block_state: Ptr<u8>,
    #[offset(72)]
    #[byte_size(256)]
    pub num_zeros: Value<Vec<i32>>,
    #[offset(328)]
    #[byte_size(256)]
    pub quant: Value<Vec<i32>>,
}
impl Default for brunsli_internal_enc_ComponentMeta {
    fn default() -> Self {
        brunsli_internal_enc_ComponentMeta {
            context_offset: 0_usize,
            approx_total_nonzeros: 0_usize,
            h_samp: 0_i32,
            v_samp: 0_i32,
            context_bits: 0_i32,
            ac_stride: 0_i32,
            dc_stride: 0_i32,
            b_stride: 0_i32,
            width_in_blocks: 0_i32,
            height_in_blocks: 0_i32,
            ac_coeffs: Ptr::<i16>::null(),
            dc_prediction_errors: Ptr::<i16>::null(),
            block_state: Ptr::<u8>::null(),
            num_zeros: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
            quant: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(88)]
pub struct brunsli_internal_enc_Histogram {
    #[offset(0)]
    #[byte_size(72)]
    pub data_: Value<Box<[i32]>>,
    #[offset(72)]
    pub total_count_: i32,
    #[offset(80)]
    pub bit_cost_: f64,
}
impl brunsli_internal_enc_Histogram {
    pub fn new() -> Self {
        let __this: Value<brunsli_internal_enc_Histogram> = Rc::new(RefCell::new(Self {
            data_: Rc::new(RefCell::new((0..18).map(|_| 0_i32).collect::<Box<[i32]>>())),
            total_count_: 0_i32,
            bit_cost_: 0_f64,
        }));
        let this: Ptr<brunsli_internal_enc_Histogram> = __this.as_pointer();
        ({ brunsli_internal_enc_HistogramImpl::Clear(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for brunsli_internal_enc_Histogram {
    fn default() -> Self {
        { brunsli_internal_enc_Histogram::new() }
    }
}
thread_local!(
    static kMaxNumberOfHistograms_139: Value<usize> = Rc::new(RefCell::new(256_usize));
);
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(72)]
pub struct brunsli_internal_enc_EntropyCodes {
    #[offset(0)]
    #[byte_size(24)]
    clustered_: Value<Vec<brunsli_internal_enc_Histogram>>,
    #[offset(24)]
    #[byte_size(24)]
    context_map_: Value<Vec<u32>>,
    #[offset(48)]
    #[byte_size(24)]
    ans_tables_: Value<Vec<brunsli_ANSTable>>,
}
impl brunsli_internal_enc_EntropyCodes {
    pub fn new(
        histograms: Ptr<Vec<brunsli_internal_enc_Histogram>>,
        mut num_bands: usize,
        offsets: Ptr<Vec<u64>>,
    ) -> Self {
        let __this: Value<brunsli_internal_enc_EntropyCodes> = Rc::new(RefCell::new(Self {
            clustered_: Rc::new(RefCell::new(Vec::new())),
            context_map_: Rc::new(RefCell::new(Vec::new())),
            ans_tables_: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_internal_enc_EntropyCodes> = __this.as_pointer();
        ({
            let _in_: Ptr<Vec<brunsli_internal_enc_Histogram>> = (histograms).clone();
            let _num_contexts: usize = kNumAvrgContexts_83.with(|rc| *rc.borrow());
            let _num_blocks: usize = num_bands;
            let _block_group_offsets: Vec<u64> = (*offsets.upgrade().deref()).clone();
            let _max_histograms: usize = kMaxNumberOfHistograms_139.with(|rc| *rc.borrow());
            let _out: Ptr<Vec<brunsli_internal_enc_Histogram>> =
                (this.with(|__s| __s.clustered_.as_pointer()));
            let _histogram_symbols: Ptr<Vec<u32>> =
                (this.with(|__s| __s.context_map_.as_pointer()));
            ClusterHistograms_137(
                _in_,
                _num_contexts,
                _num_blocks,
                _block_group_offsets,
                _max_histograms,
                _out,
                _histogram_symbols,
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct brunsli_internal_enc_EntropySource {
    #[offset(0)]
    num_bands_: usize,
    #[offset(8)]
    #[byte_size(24)]
    histograms_: Value<Vec<brunsli_internal_enc_Histogram>>,
}
impl brunsli_internal_enc_EntropySource {
    pub fn new() -> Self {
        Self {
            num_bands_: 0_usize,
            histograms_: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl Default for brunsli_internal_enc_EntropySource {
    fn default() -> Self {
        { brunsli_internal_enc_EntropySource::new() }
    }
}
thread_local!(
    static kSlackForOneBlock_140: Value<usize> = Rc::new(RefCell::new(1024_usize));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
struct brunsli_internal_enc_DataStream_CodeWord {
    #[offset(0)]
    pub context: u32,
    #[offset(4)]
    pub value: u16,
    #[offset(6)]
    pub code: u8,
    #[offset(7)]
    pub nbits: u8,
}
impl brunsli_internal_enc_DataStream_CodeWord {
    pub fn new() -> Self {
        Self {
            context: 0_u32,
            value: 0_u16,
            code: 0_u8,
            nbits: 0_u8,
        }
    }
}
impl Default for brunsli_internal_enc_DataStream_CodeWord {
    fn default() -> Self {
        { brunsli_internal_enc_DataStream_CodeWord::new() }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(56)]
pub struct brunsli_internal_enc_DataStream {
    #[offset(0)]
    pos_: i32,
    #[offset(4)]
    bw_pos_: i32,
    #[offset(8)]
    ac_pos0_: i32,
    #[offset(12)]
    ac_pos1_: i32,
    #[offset(16)]
    low_: u32,
    #[offset(20)]
    high_: u32,
    #[offset(24)]
    bw_val_: u32,
    #[offset(28)]
    bw_bitpos_: i32,
    #[offset(32)]
    #[byte_size(24)]
    code_words_: Value<Vec<brunsli_internal_enc_DataStream_CodeWord>>,
}
impl brunsli_internal_enc_DataStream {
    pub fn new() -> Self {
        Self {
            pos_: 3,
            bw_pos_: 0,
            ac_pos0_: 1,
            ac_pos1_: 2,
            low_: 0_u32,
            high_: (!0 as u32),
            bw_val_: 0_u32,
            bw_bitpos_: 0,
            code_words_: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl Default for brunsli_internal_enc_DataStream {
    fn default() -> Self {
        { brunsli_internal_enc_DataStream::new() }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(192)]
pub struct brunsli_internal_enc_State {
    #[offset(0)]
    #[byte_size(32)]
    pub entropy_source: brunsli_internal_enc_EntropySource,
    #[offset(32)]
    #[byte_size(8)]
    pub entropy_codes: Ptr<brunsli_internal_enc_EntropyCodes>,
    #[offset(40)]
    #[byte_size(56)]
    pub data_stream_dc: brunsli_internal_enc_DataStream,
    #[offset(96)]
    #[byte_size(56)]
    pub data_stream_ac: brunsli_internal_enc_DataStream,
    #[offset(152)]
    #[byte_size(24)]
    pub meta: Value<Vec<brunsli_internal_enc_ComponentMeta>>,
    #[offset(176)]
    pub num_contexts: usize,
    #[offset(184)]
    pub use_legacy_context_model: bool,
}
impl Default for brunsli_internal_enc_State {
    fn default() -> Self {
        brunsli_internal_enc_State {
            entropy_source: <brunsli_internal_enc_EntropySource>::default(),
            entropy_codes: Ptr::<brunsli_internal_enc_EntropyCodes>::null(),
            data_stream_dc: <brunsli_internal_enc_DataStream>::default(),
            data_stream_ac: <brunsli_internal_enc_DataStream>::default(),
            meta: Rc::new(RefCell::new(Default::default())),
            num_contexts: 0_usize,
            use_legacy_context_model: false,
        }
    }
}
thread_local!(
    pub static kNumDirectCodes_141: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kBrotliQuality_142: Value<i32> = Rc::new(RefCell::new(6));
);
thread_local!(
    pub static kBrotliWindowBits_143: Value<i32> = Rc::new(RefCell::new(18));
);
pub fn EstimateAuxDataSize_144(jpg: Ptr<brunsli_JPEGData>) -> usize {
    let mut size: usize = (((((*jpg.with(|__s| __s.marker_order.clone()).borrow()).len())
        .wrapping_add(
            (272_usize).wrapping_mul((*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len()),
        ))
    .wrapping_add(
        (7_usize).wrapping_mul((*jpg.with(|__s| __s.scan_info.clone()).borrow()).len()),
    ))
    .wrapping_add(16_usize));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() }) {
        {
            let rhs_0 = ((size as u64).wrapping_add(
                ((7_usize).wrapping_mul(
                    (*{
                        (*elem!(
                            (jpg.with(|__s| __s.scan_info.as_pointer())
                                as Ptr<brunsli_JPEGScanInfo>),
                            i
                        )
                        .upgrade()
                        .deref())
                        .reset_points
                        .clone()
                    }
                    .borrow())
                    .len(),
                ) as u64),
            )) as usize;
            size = rhs_0
        };
        {
            let rhs_0 = ((size as u64).wrapping_add(
                ((7_usize).wrapping_mul(
                    (*{
                        (*elem!(
                            (jpg.with(|__s| __s.scan_info.as_pointer())
                                as Ptr<brunsli_JPEGScanInfo>),
                            i
                        )
                        .upgrade()
                        .deref())
                        .extra_zero_runs
                        .clone()
                    }
                    .borrow())
                    .len(),
                ) as u64),
            )) as usize;
            size = rhs_0
        };
        i.prefix_inc();
    }
    let mut nsize: usize = if jpg.with(|__s| __s.has_zero_padding_bit) {
        (*jpg.with(|__s| __s.padding_bits.clone()).borrow()).len()
    } else {
        0_usize
    };
    size = { (size).wrapping_add((((nsize).wrapping_add(43_usize)) >> 3)) };
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len() }) {
        {
            let rhs_0 = ((size as u64).wrapping_add(
                ((5_usize).wrapping_add(
                    (*((jpg.with(|__s| __s.inter_marker_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                        .offset(i)
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>)
                        .upgrade()
                        .deref())
                    .len(),
                ) as u64),
            )) as usize;
            size = rhs_0
        };
        i.prefix_inc();
    }
    return size;
}
pub fn GetMaximumBrunsliEncodedSize_145(jpg: Ptr<brunsli_JPEGData>) -> usize {
    let mut hdr_size: usize = ((1 << 20) as usize);
    {
        let rhs_0 = (hdr_size).wrapping_add(({ EstimateAuxDataSize_144((jpg).clone()) }));
        hdr_size = rhs_0
    };
    'loop_: for mut data in jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        {
            let rhs_0 =
                ((hdr_size as u64).wrapping_add(((*data.upgrade().deref()).len() as u64))) as usize;
            hdr_size = rhs_0
        };
    }
    'loop_: for mut data in jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        {
            let rhs_0 =
                ((hdr_size as u64).wrapping_add(((*data.upgrade().deref()).len() as u64))) as usize;
            hdr_size = rhs_0
        };
    }
    {
        let rhs_0 = ((hdr_size as u64)
            .wrapping_add(((*jpg.with(|__s| __s.tail_data.clone()).borrow()).len() as u64)))
            as usize;
        hdr_size = rhs_0
    };
    let mut num_pixels: usize = (({ jpg.with(|__s| __s.width) } * { jpg.with(|__s| __s.height) })
        as usize)
        .wrapping_mul((*jpg.with(|__s| __s.components.clone()).borrow()).len());
    return (((num_pixels as f64) * 1.2E+0) as usize).wrapping_add(hdr_size);
}
pub fn Base128Size_146(mut val: usize) -> usize {
    let mut size: usize = 1_usize;
    'loop_: while (val >= 128_usize) {
        size.prefix_inc();
        val >>= 7;
    }
    return size;
}
pub fn EncodeBase128_147(mut val: usize, mut data: Ptr<u8>) -> usize {
    let mut len: usize = 0_usize;
    let mut __do_while = true;
    'loop_: while __do_while || (val > 0_usize) {
        __do_while = false;
        let __rhs =
            (((val & 127_usize) | ((if (val >= 128_usize) { 128 } else { 0 }) as usize)) as u8);
        elem!(data, len.postfix_inc()).write(__rhs);
        val >>= 7;
    }
    return len;
}
pub fn EncodeBase128Fix_148(mut val: usize, mut len: usize, mut data: Ptr<u8>) {
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let __rhs = (((val & 127_usize)
            | ((if ((i).wrapping_add(1_usize) < len) {
                128
            } else {
                0
            }) as usize)) as u8);
        (data.postfix_inc()).write(__rhs);
        val >>= 7;
        i.prefix_inc();
    }
}
pub fn TransformApp0Marker_149(s: Ptr<Vec<u8>>, mut out: Ptr<Vec<u8>>) -> bool {
    if ((*s.upgrade().deref()).len() != 17_usize) {
        return false;
    }
    if (((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>) as Ptr<u8>)
        .to_any()
        .memcmp(
            &((AppData_0xe0_62.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
            9_usize,
        )
        != 0)
    {
        return false;
    }
    if ((((((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 9_usize).read()) as i32) == 1)
        || (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 9_usize).read()) as i32) == 2))
        && (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 10_usize).read()) as i32) < 4))
        && (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 15_usize).read()) as i32) == 0))
        && (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 16_usize).read()) as i32) == 0)
    {
        let mut x_dens_hi: u8 = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 11_usize).read());
        let mut x_dens_lo: u8 = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 12_usize).read());
        let mut x_dens: i32 = (((x_dens_hi as i32) << 8) + (x_dens_lo as i32));
        let mut y_dens_hi: u8 = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 13_usize).read());
        let mut y_dens_lo: u8 = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 14_usize).read());
        let mut y_dens: i32 = (((y_dens_hi as i32) << 8) + (y_dens_lo as i32));
        let mut density_ix: i32 = -1_i32;
        let mut k: usize = 0_usize;
        'loop_: while (k < kMaxApp0Densities_45.with(|rc| *rc.borrow())) {
            if (x_dens
                == (({
                    let __idx = (k) as usize;
                    kApp0Densities_46.with(|rc| rc.borrow()[__idx])
                }) as i32))
                && (y_dens == x_dens)
            {
                density_ix = (k as i32);
            }
            k.prefix_inc();
        }
        if (density_ix >= 0) {
            let mut app0_status: u8 = (({
                ((((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 9_usize).read()) as i32) - 1)
                    | (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 10_usize).read()) as i32)
                        << 1))
            } | { (density_ix << 3) }) as u8);
            ((out).clone() as Ptr<Vec<u8>>).write(
                (0..(1_usize) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            (Ptr::<Vec<u8>>::decay(&(out)) as Ptr<u8>)
                .offset(0_usize as isize)
                .write(app0_status);
            return true;
        }
    }
    return false;
}
pub fn TransformApp2Marker_150(s: Ptr<Vec<u8>>, mut out: Ptr<Vec<u8>>) -> bool {
    if (((*s.upgrade().deref()).len() == 3161_usize)
        && (!(((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xe2_63.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                84_usize,
            )
            != 0)))
        && (!(((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>).offset((85) as isize) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xe2_63.with(|v| v.as_pointer()) as Ptr<u8>).offset((85) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ((3161 - 85) as usize),
            )
            != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[0_usize] = 128_u8;
        code[1_usize] = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 84_usize).read());
        ((out).clone() as Ptr<Vec<u8>>).write((code).clone());
        return true;
    }
    return false;
}
pub fn TransformApp12Marker_151(s: Ptr<Vec<u8>>, mut out: Ptr<Vec<u8>>) -> bool {
    if (((*s.upgrade().deref()).len() == 18_usize)
        && (!(((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xec_64.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                15_usize,
            )
            != 0)))
        && (!(((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>).offset((16) as isize) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xec_64.with(|v| v.as_pointer()) as Ptr<u8>).offset((16) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ((18 - 16) as usize),
            )
            != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[0_usize] = 129_u8;
        code[1_usize] = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 15_usize).read());
        ((out).clone() as Ptr<Vec<u8>>).write((code).clone());
        return true;
    }
    return false;
}
pub fn TransformApp14Marker_152(s: Ptr<Vec<u8>>, mut out: Ptr<Vec<u8>>) -> bool {
    if (((*s.upgrade().deref()).len() == 15_usize)
        && (!((((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>).offset(0_usize)) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xee_65.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                10_usize,
            )
            != 0)))
        && (!((((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>).offset(11_usize)) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((AppData_0xee_65.with(|v| v.as_pointer()) as Ptr<u8>).offset((11) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ((15 - 11) as usize),
            )
            != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[0_usize] = 130_u8;
        code[1_usize] = (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 10_usize).read());
        ((out).clone() as Ptr<Vec<u8>>).write((code).clone());
        return true;
    }
    return false;
}
pub fn TransformAppMarker_153(
    s: Ptr<Vec<u8>>,
    mut transformed_marker_count: Ptr<usize>,
) -> Vec<u8> {
    let out: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp0Marker_149(_s, _out)
    }) {
        transformed_marker_count.with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp2Marker_150(_s, _out)
    }) {
        transformed_marker_count.with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp12Marker_151(_s, _out)
    }) {
        transformed_marker_count.with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp14Marker_152(_s, _out)
    }) {
        transformed_marker_count.with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    return (*s.upgrade().deref()).clone();
}
pub fn GetQuantTableId_154(
    q: Ptr<brunsli_JPEGQuantTable>,
    mut is_chroma: bool,
    mut dst: Ptr<u8>,
) -> i32 {
    let mut j: i32 = 0;
    'loop_: while (j < kNumStockQuantTables_47.with(|rc| *rc.borrow())) {
        let mut match_found: bool = true;
        let mut k: i32 = 0;
        'loop_: while (match_found) && (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            if ({
                (elem!(
                    (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                    (k as usize)
                )
                .read())
            } != {
                (({
                    let __idx = (is_chroma) as usize;
                    kStockQuantizationTables_48.with(|rc| rc.borrow()[__idx].clone())
                })
                .borrow()[(j) as usize]
                    .borrow()[(k) as usize] as i32)
            }) {
                match_found = false;
            }
            k.prefix_inc();
        }
        if match_found {
            return j;
        }
        j.prefix_inc();
    }
    return (((kNumStockQuantTables_47.with(|rc| *rc.borrow()) as u32).wrapping_add(
        ({
            FindBestMatrix_119(
                ((q.with(|__s| __s.values.as_pointer()) as Ptr<i32>).offset(0_usize)),
                is_chroma,
                dst,
            )
        }),
    )) as i32);
}
pub fn EncodeVarint_155(mut n: i32, mut max_bits: i32, mut storage: Ptr<brunsli_Storage>) {
    let mut b: i32 = 0_i32;
    if !(n < (1 << max_bits)) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                215,
                Ptr::<i8>::from_string_literal(b"EncodeVarint"),
            )
        });
        'loop_: while true {}
    };
    b = 0;
    'loop_: while (n != 0) && (b < max_bits) {
        if ((b + 1) != max_bits) {
            ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
        }
        ({ WriteBits_120(1_usize, ((n & 1) as u64), (storage).clone()) });
        n >>= 1;
        b.prefix_inc();
    }
    if (b < max_bits) {
        ({ WriteBits_120(1_usize, 0_u64, (storage).clone()) });
    }
}
pub fn EncodeLimitedVarint_156(
    mut bits: usize,
    mut nbits: i32,
    mut max_symbols: i32,
    mut storage: Ptr<brunsli_Storage>,
) {
    let mut mask: usize = (1_usize << nbits).wrapping_sub(1_usize);
    let mut b: i32 = 0;
    'loop_: while (b < max_symbols) {
        ({ WriteBits_120(1_usize, ((bits != 0_usize) as u64), (storage).clone()) });
        if (bits == 0_usize) {
            break;
        }
        ({ WriteBits_120((nbits as usize), ((bits & mask) as u64), (storage).clone()) });
        bits >>= nbits;
        b.prefix_inc();
    }
}
pub fn EncodeQuantTables_157(
    jpg: Ptr<brunsli_JPEGData>,
    mut storage: Ptr<brunsli_Storage>,
) -> bool {
    if ((*jpg.with(|__s| __s.quant.clone()).borrow()).is_empty())
        || ((*jpg.with(|__s| __s.quant.clone()).borrow()).len() > 4_usize)
    {
        return false;
    }
    ({
        WriteBits_120(
            2_usize,
            (((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize) as u64),
            (storage).clone(),
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        let q: Ptr<brunsli_JPEGQuantTable> =
            (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>).offset(i);
        let mut k: i32 = 0;
        'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let mut j: i32 = (({
                let __idx = (k) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            }) as i32);
            if ((elem!(
                (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                (j as usize)
            )
            .read())
                == 0)
            {
                return false;
            }
            k.prefix_inc();
        }
        let quant_approx: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..64).map(|_| 0_u8).collect::<Box<[u8]>>()));
        let mut code: i32 = ({
            let _q: Ptr<brunsli_JPEGQuantTable> = (q).clone();
            let _is_chroma: bool = (i > 0_usize);
            let _dst: Ptr<u8> = (quant_approx.as_pointer() as Ptr<u8>);
            GetQuantTableId_154(_q, _is_chroma, _dst)
        });
        ({
            WriteBits_120(
                1_usize,
                ((code >= kNumStockQuantTables_47.with(|rc| *rc.borrow())) as u64),
                (storage).clone(),
            )
        });
        if (code < kNumStockQuantTables_47.with(|rc| *rc.borrow())) {
            ({ WriteBits_120(3_usize, (code as u64), (storage).clone()) });
        } else {
            let mut q_factor: usize =
                ((code - kNumStockQuantTables_47.with(|rc| *rc.borrow())) as usize);
            if !(q_factor < kQFactorLimit_117.with(|rc| *rc.borrow())) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                        264,
                        Ptr::<i8>::from_string_literal(b"EncodeQuantTables"),
                    )
                });
                'loop_: while true {}
            };
            ({
                WriteBits_120(
                    kQFactorBits_116.with(|rc| *rc.borrow()),
                    (q_factor as u64),
                    (storage).clone(),
                )
            });
            let mut last_diff: i32 = 0;
            let mut k: i32 = 0;
            'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                let mut j: i32 = (({
                    let __idx = (k) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as i32);
                let mut new_diff: i32 = ({
                    (elem!(
                        (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                        (j as usize)
                    )
                    .read())
                } - { ((*quant_approx.borrow())[(j) as usize] as i32) });
                let mut diff: i32 = (new_diff - last_diff);
                last_diff = new_diff;
                ({ WriteBits_120(1_usize, ((diff != 0) as u64), (storage).clone()) });
                if (diff != 0) {
                    ({ WriteBits_120(1_usize, ((diff < 0) as u64), (storage).clone()) });
                    if (diff < 0) {
                        diff = { -diff };
                    }
                    diff -= 1;
                    if (diff > 65535) {
                        return false;
                    }
                    ({ EncodeVarint_155(diff, 16, (storage).clone()) });
                }
                k.prefix_inc();
            }
        }
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        ({
            WriteBits_120(
                2_usize,
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .quant_idx
                } as u64),
                (storage).clone(),
            )
        });
        i.prefix_inc();
    }
    return true;
}
pub fn EncodeHuffmanCode_158(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    mut is_known_last: bool,
    mut storage: Ptr<brunsli_Storage>,
) -> bool {
    ({
        WriteBits_120(
            2_usize,
            ((huff.with(|__s| __s.slot_id) & 15) as u64),
            (storage).clone(),
        )
    });
    ({
        WriteBits_120(
            1_usize,
            ((huff.with(|__s| __s.slot_id) >> 4) as u64),
            (storage).clone(),
        )
    });
    if !(is_known_last) {
        ({
            WriteBits_120(
                1_usize,
                (huff.with(|__s| __s.is_last) as u64),
                (storage).clone(),
            )
        });
    } else if !(huff.with(|__s| __s.is_last)) {
        return false;
    }
    let mut is_dc_table: i32 = (((huff.with(|__s| __s.slot_id) >> 4) == 0) as i32);
    let mut total_count: i32 = 0;
    let mut space: i32 = (1 << kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()));
    let mut max_len: i32 = kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow());
    let mut max_count: i32 = if (is_dc_table != 0) {
        kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())
    } else {
        kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())
    };
    let mut found_match: i32 = 0;
    let mut stock_table_idx: i32 = 0;
    if (is_dc_table != 0) {
        let mut i: i32 = 0;
        'loop_: while (i < kNumStockDCHuffmanCodes_53.with(|rc| *rc.borrow()))
            && (!(found_match != 0))
        {
            if ((((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>).offset(1_usize))
                as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockDCHuffmanCodeCounts_54.with(|v| v.as_pointer())
                        as Ptr<Value<Box<[i32]>>>)
                        .offset(i)
                        .read()
                        .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                        .to_any(),
                    ::std::mem::size_of::<[i32; 16]>(),
                )
                == 0)
                && ((((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>).offset(0_usize))
                    as Ptr<i32>)
                    .to_any()
                    .memcmp(
                        &((((kStockDCHuffmanCodeValues_55.with(|v| v.as_pointer())
                            as Ptr<Value<Box<[i32]>>>)
                            .offset(i)
                            .read()
                            .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                            .to_any(),
                        ::std::mem::size_of::<[i32; 13]>(),
                    )
                    == 0)
            {
                found_match = 1;
                stock_table_idx = i;
            }
            i.prefix_inc();
        }
    } else {
        let mut i: i32 = 0;
        'loop_: while (i < kNumStockACHuffmanCodes_56.with(|rc| *rc.borrow()))
            && (!(found_match != 0))
        {
            if ((((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>).offset(1_usize))
                as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockACHuffmanCodeCounts_57.with(|v| v.as_pointer())
                        as Ptr<Value<Box<[i32]>>>)
                        .offset(i)
                        .read()
                        .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                        .to_any(),
                    ::std::mem::size_of::<[i32; 16]>(),
                )
                == 0)
                && ((((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>).offset(0_usize))
                    as Ptr<i32>)
                    .to_any()
                    .memcmp(
                        &((((kStockACHuffmanCodeValues_59.with(|v| v.as_pointer())
                            as Ptr<Value<Box<[i32]>>>)
                            .offset(i)
                            .read()
                            .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                            .to_any(),
                        ::std::mem::size_of::<[i32; 163]>(),
                    )
                    == 0)
            {
                found_match = 1;
                stock_table_idx = i;
            }
            i.prefix_inc();
        }
    }
    ({ WriteBits_120(1_usize, (found_match as u64), (storage).clone()) });
    if (found_match != 0) {
        ({ WriteBits_120(1_usize, (stock_table_idx as u64), (storage).clone()) });
        return true;
    }
    'loop_: while (max_len > 0)
        && ((elem!(
            (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
            (max_len as usize)
        )
        .read())
            == 0)
    {
        max_len.prefix_dec();
    }
    if ((elem!(
        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
        0_usize
    )
    .read())
        != 0)
        || (max_len == 0)
    {
        return false;
    }
    ({ WriteBits_120(4_usize, ((max_len - 1) as u64), (storage).clone()) });
    space -= (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - max_len));
    let mut i: i32 = 1;
    'loop_: while (i <= max_len) {
        let mut count: i32 = ({
            (elem!(
                (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                (i as usize)
            )
            .read())
        } - { (if (i == max_len) { 1 } else { 0 }) });
        let mut count_limit: i32 = {
            let __tmp_0: Value<i32> = Rc::new(RefCell::new((max_count - total_count)));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(
                (space >> (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - i)),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        };
        if (count > count_limit) {
            {}
            return false;
        }
        if (count_limit > 0) {
            let mut nbits: i32 = (({ Log2FloorNonZero_74((count_limit as u32)) }) + 1);
            ({ WriteBits_120((nbits as usize), (count as u64), (storage).clone()) });
            total_count += count;
            space -= (count * (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - i)));
        }
        i.prefix_inc();
    }
    if ({
        (elem!(
            (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
            (total_count as usize)
        )
        .read())
    } != { kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()) })
    {
        return false;
    }
    let p: Value<brunsli_PermutationCoder> = Rc::new(RefCell::new(brunsli_PermutationCoder::new()));
    ({
        brunsli_PermutationCoderImpl::Init(
            &p.as_pointer(),
            if (is_dc_table != 0) {
                {
                    let __count = (kDefaultDCValues_60.with(|v| v.as_pointer()) as Ptr<u8>)
                        .to_end()
                        .get_offset()
                        - (kDefaultDCValues_60.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
                    PtrValueIter::new(
                        &(kDefaultDCValues_60.with(|v| v.as_pointer()) as Ptr<u8>),
                        __count,
                    )
                    .collect::<Vec<_>>()
                }
            } else {
                {
                    let __count = (kDefaultACValues_61.with(|v| v.as_pointer()) as Ptr<u8>)
                        .to_end()
                        .get_offset()
                        - (kDefaultACValues_61.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
                    PtrValueIter::new(
                        &(kDefaultACValues_61.with(|v| v.as_pointer()) as Ptr<u8>),
                        __count,
                    )
                    .collect::<Vec<_>>()
                }
            },
        )
    });
    let mut i: i32 = 0;
    'loop_: while (i < total_count) {
        let mut val: i32 = (elem!(
            (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
            (i as usize)
        )
        .read());
        let code: Value<i32> = Rc::new(RefCell::new(0_i32));
        let nbits: Value<i32> = Rc::new(RefCell::new(0_i32));
        if !({
            brunsli_PermutationCoderImpl::RemoveValue(
                &p.as_pointer(),
                (val as u8),
                (code.as_pointer()),
                (nbits.as_pointer()),
            )
        }) {
            return false;
        }
        ({
            EncodeLimitedVarint_156(
                ((*code.borrow()) as usize),
                2,
                (((*nbits.borrow()) + 1) >> 1),
                (storage).clone(),
            )
        });
        i.prefix_inc();
    }
    return true;
}
pub fn EncodeScanInfo_159(
    si: Ptr<brunsli_JPEGScanInfo>,
    mut storage: Ptr<brunsli_Storage>,
) -> bool {
    ({ WriteBits_120(6_usize, (si.with(|__s| __s.Ss) as u64), (storage).clone()) });
    ({ WriteBits_120(6_usize, (si.with(|__s| __s.Se) as u64), (storage).clone()) });
    ({ WriteBits_120(4_usize, (si.with(|__s| __s.Ah) as u64), (storage).clone()) });
    ({ WriteBits_120(4_usize, (si.with(|__s| __s.Al) as u64), (storage).clone()) });
    ({
        WriteBits_120(
            2_usize,
            ((si.with(|__s| __s.num_components)).wrapping_sub(1_usize) as u64),
            (storage).clone(),
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { si.with(|__s| __s.num_components) }) {
        let csi: Ptr<brunsli_JPEGComponentScanInfo> = (si.with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset(i);
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.comp_idx) as u64),
                (storage).clone(),
            )
        });
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.dc_tbl_idx) as u64),
                (storage).clone(),
            )
        });
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.ac_tbl_idx) as u64),
                (storage).clone(),
            )
        });
        i.prefix_inc();
    }
    let mut last_block_idx: i32 = -1_i32;
    'loop_: for mut block_idx in si.with(|__s| __s.reset_points.as_pointer()) as Ptr<i32> {
        ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
        if !({ (block_idx.read()) } >= { (last_block_idx + 1) }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    391,
                    Ptr::<i8>::from_string_literal(b"EncodeScanInfo"),
                )
            });
            'loop_: while true {}
        };
        ({
            let _n: i32 = (({ (block_idx.read()) } - { last_block_idx }) - 1);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            EncodeVarint_155(_n, 28, _storage)
        });
        last_block_idx = { (block_idx.read()) };
    }
    ({ WriteBits_120(1_usize, 0_u64, (storage).clone()) });
    last_block_idx = 0;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*si.with(|__s| __s.extra_zero_runs.clone()).borrow()).len() }) {
        let mut block_idx: i32 = {
            (*elem!(
                (si.with(|__s| __s.extra_zero_runs.as_pointer())
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                i
            )
            .upgrade()
            .deref())
            .block_idx
        };
        let mut num: i32 = {
            (*elem!(
                (si.with(|__s| __s.extra_zero_runs.as_pointer())
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                i
            )
            .upgrade()
            .deref())
            .num_extra_zero_runs
        };
        if !(block_idx >= last_block_idx) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    401,
                    Ptr::<i8>::from_string_literal(b"EncodeScanInfo"),
                )
            });
            'loop_: while true {}
        };
        let mut j: i32 = 0;
        'loop_: while (j < num) {
            ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
            ({ EncodeVarint_155((block_idx - last_block_idx), 28, (storage).clone()) });
            last_block_idx = block_idx;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    ({ WriteBits_120(1_usize, 0_u64, (storage).clone()) });
    return true;
}
pub fn MatchComponentIds_160(comps: Ptr<Vec<brunsli_JPEGComponent>>) -> i32 {
    if ((*comps.upgrade().deref()).len() == 1_usize)
        && ({
            (*elem!(
                (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps)) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .id
        } == 1)
    {
        return kComponentIdsGray_50.with(|rc| *rc.borrow());
    }
    if ((*comps.upgrade().deref()).len() == 3_usize) {
        if (({
            (*elem!(
                (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps)) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .id
        } == 1)
            && ({
                (*elem!(
                    (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps))
                        as Ptr<brunsli_JPEGComponent>),
                    1_usize
                )
                .upgrade()
                .deref())
                .id
            } == 2))
            && ({
                (*elem!(
                    (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps))
                        as Ptr<brunsli_JPEGComponent>),
                    2_usize
                )
                .upgrade()
                .deref())
                .id
            } == 3)
        {
            return kComponentIds123_49.with(|rc| *rc.borrow());
        } else if (({
            (*elem!(
                (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps)) as Ptr<brunsli_JPEGComponent>),
                0_usize
            )
            .upgrade()
            .deref())
            .id
        } == (('R' as i8) as i32))
            && ({
                (*elem!(
                    (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps))
                        as Ptr<brunsli_JPEGComponent>),
                    1_usize
                )
                .upgrade()
                .deref())
                .id
            } == (('G' as i8) as i32)))
            && ({
                (*elem!(
                    (Ptr::<Vec<brunsli_JPEGComponent>>::decay(&(comps))
                        as Ptr<brunsli_JPEGComponent>),
                    2_usize
                )
                .upgrade()
                .deref())
                .id
            } == (('B' as i8) as i32))
        {
            return kComponentIdsRGB_51.with(|rc| *rc.borrow());
        }
    }
    return kComponentIdsCustom_52.with(|rc| *rc.borrow());
}
pub fn JumpToByteBoundary_161(mut storage: Ptr<brunsli_Storage>) {
    let mut nbits: i32 = ((storage.with(|__s| __s.pos) & 7_usize) as i32);
    if (nbits > 0) {
        ({ WriteBits_120(((8 - nbits) as usize), 0_u64, (storage).clone()) });
    }
}
pub fn EncodeAuxData_162(jpg: Ptr<brunsli_JPEGData>, mut storage: Ptr<brunsli_Storage>) -> bool {
    if ((*jpg.with(|__s| __s.marker_order.clone()).borrow()).is_empty())
        || ((((jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>)
            .to_last()
            .read()) as i32)
            != 217)
    {
        return false;
    }
    let mut have_dri: bool = false;
    let mut num_scans: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.marker_order.clone()).borrow()).len() }) {
        let mut marker: u8 = (elem!(
            (jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
            i
        )
        .read());
        if ((marker as i32) < 192) {
            return false;
        }
        ({ WriteBits_120(6_usize, (((marker as i32) - 192) as u64), (storage).clone()) });
        if ((marker as i32) == 221) {
            have_dri = true;
        }
        if ((marker as i32) == 218) {
            num_scans.prefix_inc();
        }
        i.prefix_inc();
    }
    if have_dri {
        ({
            WriteBits_120(
                16_usize,
                (jpg.with(|__s| __s.restart_interval) as u64),
                (storage).clone(),
            )
        });
    }
    if !({ (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len() } < {
        (kMaxDHTMarkers_10.with(|rc| *rc.borrow()) as usize)
    }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                453,
                Ptr::<i8>::from_string_literal(b"EncodeAuxData"),
            )
        });
        'loop_: while true {}
    };
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len() }) {
        let mut is_known_last: bool = ({ ((i).wrapping_add(1_usize)) } == {
            (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len()
        });
        ({ WriteBits_120(1_usize, (is_known_last as u64), (storage).clone()) });
        if !({
            EncodeHuffmanCode_158(
                (jpg.with(|__s| __s.huffman_code.as_pointer()) as Ptr<brunsli_JPEGHuffmanCode>)
                    .offset(i),
                is_known_last,
                (storage).clone(),
            )
        }) {
            return false;
        }
        i.prefix_inc();
    }
    if ({ num_scans } != { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() }) {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() }) {
        if !({
            EncodeScanInfo_159(
                (jpg.with(|__s| __s.scan_info.as_pointer()) as Ptr<brunsli_JPEGScanInfo>).offset(i),
                (storage).clone(),
            )
        }) {
            return false;
        }
        i.prefix_inc();
    }
    ({
        WriteBits_120(
            2_usize,
            (((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize) as u64),
            (storage).clone(),
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        ({
            WriteBits_120(
                2_usize,
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .index
                } as u64),
                (storage).clone(),
            )
        });
        if ({ i } != {
            ((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize)
        }) {
            ({
                WriteBits_120(
                    1_usize,
                    ({
                        (*elem!(
                            (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                            i
                        )
                        .upgrade()
                        .deref())
                        .is_last
                    } as u64),
                    (storage).clone(),
                )
            });
        } else if !({
            (*elem!(
                (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                i
            )
            .upgrade()
            .deref())
            .is_last
        }) {
            return false;
        }
        ({
            WriteBits_120(
                4_usize,
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .precision
                } as u64),
                (storage).clone(),
            )
        });
        i.prefix_inc();
    }
    let mut comp_ids: i32 =
        ({ MatchComponentIds_160(jpg.with(|__s| __s.components.as_pointer())) });
    ({ WriteBits_120(2_usize, (comp_ids as u64), (storage).clone()) });
    if (comp_ids == kComponentIdsCustom_52.with(|rc| *rc.borrow())) {
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
            ({
                WriteBits_120(
                    8_usize,
                    ({
                        (*elem!(
                            (jpg.with(|__s| __s.components.as_pointer())
                                as Ptr<brunsli_JPEGComponent>),
                            i
                        )
                        .upgrade()
                        .deref())
                        .id
                    } as u64),
                    (storage).clone(),
                )
            });
            i.prefix_inc();
        }
    }
    let mut nsize: usize = if jpg.with(|__s| __s.has_zero_padding_bit) {
        (*jpg.with(|__s| __s.padding_bits.clone()).borrow()).len()
    } else {
        0_usize
    };
    if ({ nsize } > { (({ PaddingBitsLimit_17((jpg).clone()) }) as usize) }) {
        return false;
    }
    ({ EncodeLimitedVarint_156(nsize, 8, 4, (storage).clone()) });
    if (nsize > 0_usize) {
        let mut i: usize = 0_usize;
        'loop_: while (i < nsize) {
            ({
                WriteBits_120(
                    1_usize,
                    ((elem!(
                        (jpg.with(|__s| __s.padding_bits.as_pointer()) as Ptr<i32>),
                        i
                    )
                    .read()) as u64),
                    (storage).clone(),
                )
            });
            i.prefix_inc();
        }
    }
    ({ JumpToByteBoundary_161((storage).clone()) });
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len() }) {
        let s: Ptr<Vec<u8>> = ((jpg.with(|__s| __s.inter_marker_data.as_pointer())
            as Ptr<Value<Vec<u8>>>)
            .offset(i)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>);
        let buffer: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_u8).collect::<Box<[u8]>>()));
        let mut len: usize = ({
            EncodeBase128_147(
                (*s.upgrade().deref()).len(),
                (buffer.as_pointer() as Ptr<u8>),
            )
        });
        ({ brunsli_StorageImpl::AppendBytes(&storage, (buffer.as_pointer() as Ptr<u8>), len) });
        ({
            let _src: Ptr<u8> = (Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>);
            let _len: usize = (*s.upgrade().deref()).len();
            brunsli_StorageImpl::AppendBytes(&storage, _src, _len)
        });
        i.prefix_inc();
    }
    return true;
}
impl brunsli_internal_enc_Histogram {}
pub fn ComputeCoeffOrder_163(num_zeros: Ptr<Vec<i32>>, mut order: Ptr<u32>) {
    let pos_and_val: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize) as usize)
            .map(|_| <(Value<i32>, Value<i32>)>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let __rhs = i;
        (*(*pos_and_val.borrow())[(i as usize)].0.borrow_mut()) = __rhs;
        let __rhs = (elem!(
            (Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>),
            (({
                let __idx = (i) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            }) as usize)
        )
        .read());
        (*(*pos_and_val.borrow())[(i as usize)].1.borrow_mut()) = __rhs;
        i.prefix_inc();
    }
    (pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>).sort_with_cmp(
        (pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>)
            .to_end()
            .get_offset(),
        |x, y| {
            FnPtr::<fn(Ptr<(Value<i32>, Value<i32>)>, Ptr<(Value<i32>, Value<i32>)>) -> bool>::new(
                |a: Ptr<(Value<i32>, Value<i32>)>, b: Ptr<(Value<i32>, Value<i32>)>| -> bool {
                    {
                        return ({ (*(*a.upgrade().deref()).1.borrow()) } < {
                            (*(*b.upgrade().deref()).1.borrow())
                        });
                    }
                },
            )
            .call(x, y)
        },
    );
    let mut i: usize = 0_usize;
    'loop_: while (i < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        let __rhs = ({
            let __idx = (*(*pos_and_val.borrow())[i].0.borrow()) as usize;
            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
        });
        elem!(order, i).write(__rhs);
        i.prefix_inc();
    }
}
impl brunsli_internal_enc_EntropyCodes {}
impl brunsli_internal_enc_DataStream {}
pub fn EncodeNumNonzeros_166(
    mut val: usize,
    mut p: Ptr<brunsli_Prob>,
    mut data_stream: Ptr<brunsli_internal_enc_DataStream>,
) {
    if !(val < ((1_u32 << kNumNonZeroBits_84.with(|rc| *rc.borrow())) as usize)) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                719,
                Ptr::<i8>::from_string_literal(b"EncodeNumNonzeros"),
            )
        });
        'loop_: while true {}
    };
    let mut bst: Ptr<brunsli_Prob> = p.offset(-((1) as isize));
    let mut ctx: usize = 1_usize;
    let mut mask: usize =
        ((1 << ((kNumNonZeroBits_84.with(|rc| *rc.borrow())).wrapping_sub(1_usize))) as usize);
    'loop_: while (mask != 0_usize) {
        let mut bit: i32 = (((val & mask) != 0_usize) as i32);
        ({
            brunsli_internal_enc_DataStreamImpl::AddBit(
                &data_stream,
                bst.offset((ctx) as isize),
                bit,
            )
        });
        ctx = { ((2_usize).wrapping_mul(ctx)).wrapping_add((bit as usize)) };
        mask >>= 1;
    }
}
pub fn CollectAllCoeffs_167(mut coeffs: Ptr<i16>) -> i16 {
    let mut all_coeffs: i16 = 0_i16;
    let mut k: i32 = 1;
    'loop_: while ((all_coeffs as i32) == 0) && (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        all_coeffs = { ((all_coeffs as i32) | ((elem!(coeffs, k).read()) as i32)) as i16 };
        k.prefix_inc();
    }
    return all_coeffs;
}
pub fn EncodeCoeffOrder_168(
    mut order: Ptr<u32>,
    mut data_stream: Ptr<brunsli_internal_enc_DataStream>,
) {
    let order_zigzag: Value<Box<[u32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_u32).collect::<Box<[u32]>>()));
    let mut i: usize = 0_usize;
    'loop_: while (i < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        (*order_zigzag.borrow_mut())[(i) as usize] = {
            ({
                let __idx = (elem!(order, i).read()) as usize;
                kJPEGZigZagOrder_14.with(|rc| rc.borrow()[__idx])
            })
        };
        i.prefix_inc();
    }
    let lehmer: Value<Box<[u32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_u32).collect::<Box<[u32]>>()));
    ({
        ComputeLehmerCode_112(
            (order_zigzag.as_pointer() as Ptr<u32>),
            (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize),
            (lehmer.as_pointer() as Ptr<u32>),
        )
    });
    let mut tail: i32 = (kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1);
    'loop_: while (tail >= 1) && ((*lehmer.borrow())[(tail) as usize] == 0_u32) {
        tail.prefix_dec();
    }
    let mut i: i32 = 1;
    'loop_: while (i <= tail) {
        (*lehmer.borrow_mut())[(i) as usize].prefix_inc();
        i.prefix_inc();
    }
    thread_local!(
        static kSpan_169: Value<i32> = Rc::new(RefCell::new(16));
    );
    let mut i: i32 = 0;
    'loop_: while (i < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let mut start: i32 = if (i > 0) { i } else { 1 };
        let mut end: i32 = (i + kSpan_169.with(|rc| *rc.borrow()));
        let mut has_non_zero: i32 = 0;
        let mut j: i32 = start;
        'loop_: while (j < end) {
            has_non_zero = { ((has_non_zero as u32) | (*lehmer.borrow())[(j) as usize]) as i32 };
            j.prefix_inc();
        }
        if !(has_non_zero != 0) {
            ({ brunsli_internal_enc_DataStreamImpl::AddBits(&data_stream, 1, 0) });
            i += kSpan_169.with(|rc| *rc.borrow());
            continue 'loop_;
        } else {
            ({ brunsli_internal_enc_DataStreamImpl::AddBits(&data_stream, 1, 1) });
        }
        let mut j: i32 = start;
        'loop_: while (j < end) {
            let mut v: i32 = 0_i32;
            if !((*lehmer.borrow())[(j) as usize]
                <= (kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32))
            {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                        769,
                        Ptr::<i8>::from_string_literal(b"EncodeCoeffOrder"),
                    )
                });
                'loop_: while true {}
            };
            v = ((*lehmer.borrow())[(j) as usize] as i32);
            'loop_: while (v >= 7) {
                ({ brunsli_internal_enc_DataStreamImpl::AddBits(&data_stream, 3, 7) });
                v -= 7;
            }
            ({ brunsli_internal_enc_DataStreamImpl::AddBits(&data_stream, 3, v) });
            j.prefix_inc();
        }
        i += kSpan_169.with(|rc| *rc.borrow());
    }
}
pub fn FrameTypeCode_170(jpg: Ptr<brunsli_JPEGData>) -> u32 {
    let mut code: u32 = 0_u32;
    let mut shift: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() })
        && (i < 4_usize)
    {
        let mut h_samp: u32 = (({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            )
            .upgrade()
            .deref())
            .h_samp_factor
        } - 1) as u32);
        let mut v_samp: u32 = (({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            )
            .upgrade()
            .deref())
            .v_samp_factor
        } - 1) as u32);
        code |= ((h_samp << (shift + 4)) | (v_samp << shift));
        shift += 8;
        i.prefix_inc();
    }
    return code;
}
pub fn EncodeSignature_171(mut len: usize, mut data: Ptr<u8>, mut pos: Ptr<usize>) -> bool {
    if (len < kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
        || ({ (pos.read()) } > {
            (len).wrapping_sub(kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
        })
    {
        return false;
    }
    {
        ((data.offset((pos.read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((kBrunsliSignature_44.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                kBrunsliSignatureSize_43.with(|rc| *rc.borrow()) as usize,
            );
        ((data.offset((pos.read()) as isize)) as Ptr<u8>).to_any()
    };
    pos.write({ (pos.read()).wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow())) });
    return true;
}
pub fn EncodeValue_172(mut tag: u8, mut value: usize, mut data: Ptr<u8>, mut pos: Ptr<usize>) {
    let __rhs = ({ ValueMarker_28(tag) });
    elem!(data, pos.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    {
        let rhs_0 = (pos.read()).wrapping_add(
            ({
                let _val: usize = value;
                let _data: Ptr<u8> = data.offset((pos.read()) as isize);
                EncodeBase128_147(_val, _data)
            }),
        );
        pos.write(rhs_0)
    };
}
pub fn EncodeHeader_173(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(state);
    let mut version: usize = (jpg.with(|__s| __s.version) as usize);
    let mut is_fallback: bool =
        ((version & 1_usize) == (kFallbackVersion_2.with(|rc| *rc.borrow()) as usize));
    if (is_fallback) && (version != (kFallbackVersion_2.with(|rc| *rc.borrow()) as usize)) {
        return false;
    }
    if (((!(is_fallback))
        && ((jpg.with(|__s| __s.width) == 0) || (jpg.with(|__s| __s.height) == 0)))
        || ((*jpg.with(|__s| __s.components.clone()).borrow()).is_empty()))
        || ({ (*jpg.with(|__s| __s.components.clone()).borrow()).len() } > {
            (kMaxComponents_4.with(|rc| *rc.borrow()) as usize)
        })
    {
        return false;
    }
    if ((version & (!7_u32 as usize)) != 0) {
        return false;
    }
    let mut version_comp: usize = (({
        ((((*jpg.with(|__s| __s.components.clone()).borrow()).len()).wrapping_sub(1_usize)) as u64)
    } | { ((version << 2) as u64) }) as usize);
    let mut subsampling: usize = (({ FrameTypeCode_170((jpg).clone()) }) as usize);
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        let _tag: u8 = kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (data).clone();
        EncodeValue_172(
            _tag,
            (jpg.with(|__s| __s.width) as usize),
            _data,
            (pos.as_pointer()),
        )
    });
    ({
        let _tag: u8 = kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (data).clone();
        EncodeValue_172(
            _tag,
            (jpg.with(|__s| __s.height) as usize),
            _data,
            (pos.as_pointer()),
        )
    });
    ({
        let _tag: u8 = kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (data).clone();
        EncodeValue_172(_tag, version_comp, _data, (pos.as_pointer()))
    });
    ({
        let _tag: u8 = kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (data).clone();
        EncodeValue_172(_tag, subsampling, _data, (pos.as_pointer()))
    });
    len.write({ (*pos.borrow()) });
    return true;
}
pub fn EncodeMetaData_174(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(state);
    let metadata: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    let transformed_marker_count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.app_data.clone()).borrow()).len() }) {
        let s: Ptr<Vec<u8>> = ((jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
            .offset(i)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>);
        ({
            let _dst: Ptr<Vec<u8>> = (metadata.as_pointer());
            let _src: Value<Vec<u8>> = Rc::new(RefCell::new(
                ({
                    let _s: Ptr<Vec<u8>> = (s).clone();
                    let _transformed_marker_count: Ptr<usize> =
                        (transformed_marker_count.as_pointer());
                    TransformAppMarker_153(_s, _transformed_marker_count)
                }),
            ));
            Append_73(_dst, _src.as_pointer())
        });
        i.prefix_inc();
    }
    if ((*transformed_marker_count.borrow())
        > (kBrunsliShortMarkerLimit_23.with(|rc| *rc.borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many short markers: {:}\n",
            (*transformed_marker_count.borrow()),
        );
        return false;
    }
    let mut other_app_count: usize =
        (((*jpg.with(|__s| __s.app_data.clone()).borrow()).len() as u64)
            .wrapping_sub(((*transformed_marker_count.borrow()) as u64)) as usize);
    if (other_app_count > (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as usize)) {
        write!(
            libcc2rs::cerr(),
            "Too many app markers: {:}\n",
            other_app_count,
        );
        return false;
    }
    let mut com_count: usize = (*jpg.with(|__s| __s.com_data.clone()).borrow()).len();
    if (com_count > (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as usize)) {
        write!(libcc2rs::cerr(), "Too many com markers: {:}\n", com_count,);
        return false;
    }
    'loop_: for mut s in jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Value<Vec<u8>>> {
        let s: Ptr<Vec<u8>> = s.upgrade().deref().as_pointer();
        ({
            let _dst: Ptr<Vec<u8>> = (metadata.as_pointer());
            let _src: Ptr<Vec<u8>> = (s).clone();
            Append_73(_dst, _src)
        });
    }
    if !((*jpg.with(|__s| __s.tail_data.clone()).borrow()).is_empty()) {
        let marker: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([217_u8])));
        ({
            Append_72(
                (metadata.as_pointer()),
                (marker.as_pointer() as Ptr<u8>),
                1_usize,
            )
        });
        ({
            Append_73(
                (metadata.as_pointer()),
                jpg.with(|__s| __s.tail_data.as_pointer()),
            )
        });
    }
    if (*metadata.borrow()).is_empty() {
        len.write(0_usize);
        return true;
    } else if ((*metadata.borrow()).len() == 1_usize) {
        len.write(1_usize);
        let __rhs = { (*metadata.borrow())[0_usize] };
        elem!(data, 0).write(__rhs);
        return true;
    }
    let mut pos: usize = ({ EncodeBase128_147((*metadata.borrow()).len(), (data).clone()) });
    let compressed_size: Value<usize> = Rc::new(RefCell::new((len.read()).wrapping_sub(pos)));
    if !({
        // Compress into a buffer bounded by the input size, as the output buffer
        // may be much larger and is expensive to borrow if reinterpreted.
        let mut __out_len = (compressed_size.as_pointer()).read();
        let __max =
            unsafe { ::brotli_sys::BrotliEncoderMaxCompressedSize((*metadata.borrow()).len()) };
        let mut __out = vec![
            0u8;
            if __max == 0 {
                __out_len
            } else {
                __out_len.min(__max)
            }
        ];
        __out_len = __out.len();
        let __ok = (metadata.as_pointer() as Ptr<u8>).with_slice(
            (*metadata.borrow()).len(),
            |__in| unsafe {
                ::brotli_sys::BrotliEncoderCompress(
                    kBrotliQuality_142.with(|rc| *rc.borrow()),
                    kBrotliWindowBits_143.with(|rc| *rc.borrow()),
                    ::brotli_sys::BROTLI_MODE_GENERIC,
                    (*metadata.borrow()).len(),
                    __in.as_ptr(),
                    &mut __out_len,
                    __out.as_mut_ptr(),
                )
            },
        );
        if __ok != 0 {
            (data.offset((pos) as isize))
                .with_slice_mut(__out_len, |__s| __s.copy_from_slice(&__out[..__out_len]));
            (compressed_size.as_pointer()).write(__out_len);
        }
        __ok
    } != 0)
    {
        write!(
            libcc2rs::cerr(),
            "Brotli compression failed: input size = {:} pos = {:} len = {:}\n",
            (*metadata.borrow()).len(),
            pos,
            (len.read()),
        );
        return false;
    }
    pos = { (pos).wrapping_add((*compressed_size.borrow())) };
    len.write({ pos });
    return true;
}
pub fn EncodeJPEGInternals_175(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(state);
    let storage: Value<brunsli_Storage> =
        Rc::new(RefCell::new(brunsli_Storage::new({ (data).clone() }, {
            (len.read())
        })));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeAuxData_162(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    len.write(__rhs);
    return true;
}
pub fn EncodeQuantData_176(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(state);
    let storage: Value<brunsli_Storage> =
        Rc::new(RefCell::new(brunsli_Storage::new({ (data).clone() }, {
            (len.read())
        })));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeQuantTables_157(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    len.write(__rhs);
    return true;
}
pub fn EncodeHistogramData_177(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    let storage: Value<brunsli_Storage> =
        Rc::new(RefCell::new(brunsli_Storage::new({ (data).clone() }, {
            (len.read())
        })));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        ({
            WriteBits_120(
                3_usize,
                ({
                    (*elem!(
                        (state.with(|__s| __s.meta.as_pointer())
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .context_bits
                } as u64),
                (storage.as_pointer()),
            )
        });
        i.prefix_inc();
    }
    ({
        brunsli_internal_enc_EntropyCodesImpl::EncodeContextMap(
            &state.with(|__s| __s.entropy_codes.clone()),
            (storage.as_pointer()),
        )
    });
    ({
        brunsli_internal_enc_EntropyCodesImpl::BuildAndStoreEntropyCodes(
            &state.with(|__s| __s.entropy_codes.clone()),
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    len.write(__rhs);
    return true;
}
pub fn EncodeDCData_178(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(*jpg.upgrade().deref());
    let storage: Value<brunsli_Storage> =
        Rc::new(RefCell::new(brunsli_Storage::new({ (data).clone() }, {
            (len.read())
        })));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> =
            state.with(|__s| __s.entropy_codes.clone());
        brunsli_internal_enc_DataStreamImpl::EncodeCodeWords(
            &field_ptr!(state, data_stream_dc),
            _s,
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    len.write(__rhs);
    return true;
}
pub fn EncodeACData_179(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(*jpg.upgrade().deref());
    let storage: Value<brunsli_Storage> =
        Rc::new(RefCell::new(brunsli_Storage::new({ (data).clone() }, {
            (len.read())
        })));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> =
            state.with(|__s| __s.entropy_codes.clone());
        brunsli_internal_enc_DataStreamImpl::EncodeCodeWords(
            &field_ptr!(state, data_stream_ac),
            _s,
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    len.write(__rhs);
    return true;
}
pub fn EncodeSection_180(
    jpg: Ptr<brunsli_JPEGData>,
    mut s: Ptr<brunsli_internal_enc_State>,
    mut tag: u8,
    mut write_section: FnPtr<
        fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
    >,
    mut section_size_bytes: usize,
    mut len: usize,
    mut data: Ptr<u8>,
    mut pos: Ptr<usize>,
) -> bool {
    let mut pos_start: usize = (pos.read());
    let mut marker: u8 = ({ SectionMarker_29(tag) });
    let __rhs = marker;
    elem!(data, pos.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    pos.write({ (pos.read()).wrapping_add(section_size_bytes) });
    let section_size: Value<usize> = Rc::new(RefCell::new((len).wrapping_sub((pos.read()))));
    if !({
        let _arg0: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _arg1: Ptr<brunsli_internal_enc_State> = (s).clone();
        let _arg2: Ptr<u8> = (data.offset((pos.read()) as isize));
        let _arg3: Ptr<usize> = (section_size.as_pointer());
        write_section.call(_arg0, _arg1, _arg2, _arg3)
    }) {
        return false;
    }
    pos.write({ (pos.read()).wrapping_add((*section_size.borrow())) });
    if (((*section_size.borrow()) >> ((7_usize).wrapping_mul(section_size_bytes))) > 0_usize) {
        write!(libcc2rs::cerr(), "Section 0x",);
        libcc2rs::cerr().write_all(&([(&[marker as u8] as &[u8]), (b" size " as &[u8])].concat()));
        write!(
            libcc2rs::cerr(),
            "{:} too large for {:} bytes base128 number.\n",
            (*section_size.borrow()),
            section_size_bytes,
        );
        return false;
    }
    ({
        EncodeBase128Fix_148(
            (*section_size.borrow()),
            section_size_bytes,
            (data.offset(((pos_start).wrapping_add(1_usize)) as isize)),
        )
    });
    return true;
}
pub fn SampleNumNonZeros_181(mut m: Ptr<brunsli_internal_enc_ComponentMeta>) -> usize {
    let mut num_blocks: usize =
        (({ m.with(|__s| __s.width_in_blocks) } * { m.with(|__s| __s.height_in_blocks) }) as usize);
    if (num_blocks < ((32 * 32) as usize)) {
        return (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize).wrapping_mul(num_blocks);
    }
    let mut coeffs: Ptr<i16> = m.with(|__s| __s.ac_coeffs.clone());
    let mut stride: usize = (m.with(|__s| __s.ac_stride) as usize);
    let mut width_in_blocks: usize = (m.with(|__s| __s.width_in_blocks) as usize);
    let num_zeros: Ptr<Vec<i32>> = m.with(|__s| __s.num_zeros.as_pointer());
    thread_local!(
        static kStride_182: Value<i32> = Rc::new(RefCell::new(5));
    );
    let mut total_nonzeros: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < num_blocks) {
        let mut x: usize = (i).wrapping_rem(width_in_blocks);
        let mut y: usize = (i).wrapping_div(width_in_blocks);
        let mut block: Ptr<i16> = coeffs
            .offset(((x).wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))) as isize)
            .offset(((y).wrapping_mul(stride)) as isize);
        let mut k: usize = 0_usize;
        'loop_: while (k < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
            if (((elem!(block, k).read()) as i32) == 0) {
                elem!((Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>), k)
                    .with_mut(|__v| __v.prefix_inc());
            }
            k.prefix_inc();
        }
        total_nonzeros =
            { (total_nonzeros).wrapping_add((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) };
        i = { (i).wrapping_add((kStride_182.with(|rc| *rc.borrow()) as usize)) };
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        {
            let rhs_0 = (total_nonzeros).wrapping_sub(
                ((elem!((Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>), i).read()) as usize),
            );
            total_nonzeros = rhs_0
        };
        i.prefix_inc();
    }
    elem!((Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>), 0_usize).write(0);
    return (total_nonzeros).wrapping_mul((kStride_182.with(|rc| *rc.borrow()) as usize));
}
pub fn SelectContextBits_183(mut num_symbols: usize) -> i32 {
    thread_local!(
        static kContextBits_184: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6,
            6, 6, 6, 6,
        ])));
    );
    let mut log2_size: usize = (({ Log2FloorNonZero_74((num_symbols as u32)) }) as usize);
    let mut scheme: i32 = ({
        let __idx = (log2_size) as usize;
        kContextBits_184.with(|rc| rc.borrow()[__idx])
    });
    if !(scheme < kNumSchemes_91.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                1029,
                Ptr::<i8>::from_string_literal(b"SelectContextBits"),
            )
        });
        'loop_: while true {}
    };
    return scheme;
}
pub fn PredictDCCoeffs_185(mut state: Ptr<brunsli_internal_enc_State>) -> bool {
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*meta.upgrade().deref()).len() }) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset(i);
        let mut width: i32 = m.with(|__s| __s.width_in_blocks);
        let mut height: i32 = m.with(|__s| __s.height_in_blocks);
        let mut ac_stride: i32 = m.with(|__s| __s.ac_stride);
        let mut dc_stride: i32 = m.with(|__s| __s.dc_stride);
        let mut y: i32 = 0;
        'loop_: while (y < height) {
            let mut coeffs: Ptr<i16> = m
                .with(|__s| __s.ac_coeffs.clone())
                .offset((ac_stride * y) as isize);
            let mut pred_errors: Ptr<i16> = m
                .with(|__s| __s.dc_prediction_errors.clone())
                .offset((dc_stride * y) as isize);
            let mut x: i32 = 0;
            'loop_: while (x < width) {
                let mut err: i32 = ({ ((elem!(coeffs, 0).read()) as i32) } - {
                    ({ PredictWithAdaptiveMedian_115((coeffs).clone(), x, y, ac_stride) })
                });
                if (err.abs() > kBrunsliMaxDCAbsVal_19.with(|rc| *rc.borrow())) {
                    write!(
                        libcc2rs::cerr(),
                        "Invalid DC coefficient: {:} after prediction: {:}\n",
                        (elem!(coeffs, 0).read()),
                        err,
                    );
                    return false;
                }
                coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                let __rhs = (err as i16);
                (pred_errors.postfix_inc()).write(__rhs);
                x.prefix_inc();
            }
            y.prefix_inc();
        }
        i.prefix_inc();
    }
    return true;
}
pub fn CalculateMeta_186(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
) -> bool {
    let mut num_components: usize = (*jpg.with(|__s| __s.components.clone()).borrow()).len();
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    {
        let __a0 = num_components as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_enc_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_ComponentMeta>::default())
        })
    };
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let c: Ptr<brunsli_JPEGComponent> =
            (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i);
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset(i);
        if ({ (c.with(|__s| __s.quant_idx) as usize) } >= {
            (*jpg.with(|__s| __s.quant.clone()).borrow()).len()
        }) {
            return false;
        }
        let q: Ptr<brunsli_JPEGQuantTable> = (jpg.with(|__s| __s.quant.as_pointer())
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((c.with(|__s| __s.quant_idx) as usize));
        field!(m, h_samp).write(c.with(|__s| __s.h_samp_factor));
        field!(m, v_samp).write(c.with(|__s| __s.v_samp_factor));
        field!(m, width_in_blocks)
            .write({ ({ jpg.with(|__s| __s.MCU_cols) } * { m.with(|__s| __s.h_samp) }) });
        field!(m, height_in_blocks)
            .write({ ({ jpg.with(|__s| __s.MCU_rows) } * { m.with(|__s| __s.v_samp) }) });
        field!(m, ac_coeffs)
            .write(((c.with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>).offset(0_usize)));
        field!(m, ac_stride).write({
            ({ m.with(|__s| __s.width_in_blocks) } * { kDCTBlockSize_3.with(|rc| *rc.borrow()) })
        });
        field!(m, dc_stride).write({ m.with(|__s| __s.width_in_blocks) });
        field!(m, b_stride).write({ m.with(|__s| __s.width_in_blocks) });
        {
            ((m.with(|__s| __s.quant.as_pointer()) as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &(((q.with(|__s| __s.values.as_pointer()) as Ptr<i32>).offset(0_usize))
                        as Ptr<i32>)
                        .to_any(),
                    (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)
                        .wrapping_mul((::std::mem::size_of::<i32>() as usize))
                        as usize,
                );
            ((m.with(|__s| __s.quant.as_pointer()) as Ptr<i32>) as Ptr<i32>).to_any()
        };
        i.prefix_inc();
    }
    return true;
}
pub fn EncodeDC_187(mut state: Ptr<brunsli_internal_enc_State>) {
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    let mut mcu_rows: i32 = ({
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .height_in_blocks
        }
    } / {
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp
        }
    });
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> = field_ptr!(state, entropy_source);
    let data_stream: Ptr<brunsli_internal_enc_DataStream> = field_ptr!(state, data_stream_dc);
    let comps: Value<Vec<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
        (0..(num_components) as usize)
            .map(|_| <brunsli_ComponentStateDC>::default())
            .collect::<Vec<_>>(),
    ));
    let mut total_num_blocks: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset(i);
        ({
            brunsli_ComponentStateDCImpl::SetWidth(
                &(comps.as_pointer() as Ptr<brunsli_ComponentStateDC>).offset(i),
                m.with(|__s| __s.width_in_blocks),
            )
        });
        total_num_blocks = {
            (total_num_blocks).wrapping_add(
                (({ m.with(|__s| __s.width_in_blocks) } * { m.with(|__s| __s.height_in_blocks) })
                    as usize),
            )
        };
        i.prefix_inc();
    }
    ({
        let _num_bands: usize = num_components;
        brunsli_internal_enc_EntropySourceImpl::Resize(&entropy_source, _num_bands)
    });
    ({
        let _max_num_code_words: usize =
            ((3_usize).wrapping_mul(total_num_blocks)).wrapping_add(128_usize);
        brunsli_internal_enc_DataStreamImpl::Resize(&data_stream, _max_num_code_words)
    });
    let mut mcu_y: i32 = 0;
    'loop_: while (mcu_y < mcu_rows) {
        let mut i: usize = 0_usize;
        'loop_: while (i < num_components) {
            let mut c: Ptr<brunsli_ComponentStateDC> =
                ((comps.as_pointer() as Ptr<brunsli_ComponentStateDC>).offset(i));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset(i);
            let mut width: i32 = c.with(|__s| __s.width);
            let mut ac_stride: i32 = m.with(|__s| __s.ac_stride);
            let mut dc_stride: i32 = m.with(|__s| __s.dc_stride);
            let mut b_stride: i32 = m.with(|__s| __s.b_stride);
            let mut y: i32 = ({ mcu_y } * { m.with(|__s| __s.v_samp) });
            let mut prev_sgn: Ptr<i32> =
                ((c.with(|__s| __s.prev_sign.as_pointer()) as Ptr<i32>).offset(1_usize));
            let mut prev_abs: Ptr<i32> =
                ((c.with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>).offset(2_usize));
            let mut iy: i32 = 0;
            'loop_: while ({ iy } < { m.with(|__s| __s.v_samp) }) {
                let mut dc_coeffs_in: Ptr<i16> = m
                    .with(|__s| __s.dc_prediction_errors.clone())
                    .offset((y * dc_stride) as isize);
                let mut ac_coeffs_in: Ptr<i16> = m
                    .with(|__s| __s.ac_coeffs.clone())
                    .offset((y * ac_stride) as isize);
                let mut block_state: Ptr<u8> = m
                    .with(|__s| __s.block_state.clone())
                    .offset((y * b_stride) as isize);
                let mut x: i32 = 0;
                'loop_: while (x < width) {
                    ({ brunsli_internal_enc_DataStreamImpl::ResizeForBlock(&data_stream) });
                    let mut coeff: i16 = (elem!(dc_coeffs_in, 0).read());
                    let mut sign: i32 = if ((coeff as i32) > 0) {
                        1
                    } else {
                        if ((coeff as i32) < 0) { 2 } else { 0 }
                    };
                    let mut absval: i32 = if (sign == 2) {
                        -(coeff as i32)
                    } else {
                        (coeff as i32)
                    };
                    let mut all_coeffs: i16 = (({ (coeff as i32) } | {
                        (({ CollectAllCoeffs_167((ac_coeffs_in).clone()) }) as i32)
                    }) as i16);
                    let mut is_empty_block: bool = ((all_coeffs as i32) == 0);
                    let mut is_empty_ctx: i32 = ({
                        IsEmptyBlockContext_106(
                            ((c.with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>)
                                .offset(1_usize)),
                            x,
                        )
                    });
                    ({
                        let _p: Ptr<brunsli_Prob> = ((c
                            .with(|__s| __s.is_empty_block_prob.as_pointer())
                            as Ptr<brunsli_Prob>)
                            .offset((is_empty_ctx as usize)));
                        let _bit: i32 = (!(is_empty_block) as i32);
                        brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                    });
                    elem!(
                        (c.with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>),
                        ((x + 1) as usize)
                    )
                    .write((!(is_empty_block) as i32));
                    block_state.write({ (is_empty_block as u8) });
                    if !(is_empty_block) {
                        let mut is_zero: i32 = (((coeff as i32) == 0) as i32);
                        ({
                            let _p: Ptr<brunsli_Prob> = (field_ptr!(c, is_zero_prob));
                            let _bit: i32 = is_zero;
                            brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                        });
                        if !(is_zero != 0) {
                            let mut avrg_ctx: i32 = ({
                                let _vals: Ptr<i32> = (prev_abs).clone();
                                let _x: i32 = x;
                                WeightedAverageContextDC_97(_vals, _x)
                            });
                            let mut sign_ctx: i32 = (((elem!(prev_sgn, x).read()) * 3)
                                + (elem!(prev_sgn, (x - 1)).read()));
                            ({
                                let _p: Ptr<brunsli_Prob> = ((c
                                    .with(|__s| __s.sign_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset((sign_ctx as usize)));
                                let _bit: i32 = (sign - 1);
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                            let mut zdens_ctx: usize = i;
                            if (absval <= kNumDirectCodes_141.with(|rc| *rc.borrow())) {
                                ({
                                    let _code: usize = ((absval - 1) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = ((avrg_ctx as u32) as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    brunsli_internal_enc_DataStreamImpl::AddCode(
                                        &data_stream,
                                        _code,
                                        _band,
                                        _context,
                                        _s,
                                    )
                                });
                            } else {
                                let mut nbits: i32 = (({
                                    Log2FloorNonZero_74(
                                        (((absval - kNumDirectCodes_141.with(|rc| *rc.borrow()))
                                            + 1) as u32),
                                    )
                                }) - 1);
                                ({
                                    let _code: usize =
                                        ((kNumDirectCodes_141.with(|rc| *rc.borrow()) + nbits)
                                            as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = (avrg_ctx as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    brunsli_internal_enc_DataStreamImpl::AddCode(
                                        &data_stream,
                                        _code,
                                        _band,
                                        _context,
                                        _s,
                                    )
                                });
                                let mut extra_bits: i32 = (absval
                                    - ((kNumDirectCodes_141.with(|rc| *rc.borrow()) - 1)
                                        + (2 << nbits)));
                                let mut first_extra_bit: i32 = ((extra_bits >> nbits) & 1);
                                ({
                                    let _p: Ptr<brunsli_Prob> = ((c
                                        .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                        as Ptr<brunsli_Prob>)
                                        .offset((nbits as usize)));
                                    let _bit: i32 = first_extra_bit;
                                    brunsli_internal_enc_DataStreamImpl::AddBit(
                                        &data_stream,
                                        _p,
                                        _bit,
                                    )
                                });
                                if (nbits > 0) {
                                    extra_bits &= ((1 << nbits) - 1);
                                    ({
                                        let _nbits: i32 = nbits;
                                        let _bits: i32 = extra_bits;
                                        brunsli_internal_enc_DataStreamImpl::AddBits(
                                            &data_stream,
                                            _nbits,
                                            _bits,
                                        )
                                    });
                                }
                            }
                        }
                    }
                    elem!(prev_sgn, x).write({ sign });
                    elem!(prev_abs, x).write({ absval });
                    block_state.prefix_inc();
                    dc_coeffs_in.prefix_inc();
                    ac_coeffs_in += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    x.prefix_inc();
                }
                {
                    iy.prefix_inc();
                    y.prefix_inc()
                };
            }
            i.prefix_inc();
        }
        mcu_y.prefix_inc();
    }
}
pub fn EncodeAC_188(mut state: Ptr<brunsli_internal_enc_State>) {
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    let mut mcu_rows: i32 = ({
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .height_in_blocks
        }
    } / {
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp
        }
    });
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> = field_ptr!(state, entropy_source);
    let data_stream: Ptr<brunsli_internal_enc_DataStream> = field_ptr!(state, data_stream_ac);
    let mut context_modes: Ptr<u8> = (kContextAlgorithm_95.with(|v| v.as_pointer()) as Ptr<u8>)
        .offset(
            (if state.with(|__s| __s.use_legacy_context_model) {
                64
            } else {
                0
            }) as isize,
        );
    let mut num_code_words: usize = 0_usize;
    let comps: Value<Vec<brunsli_ComponentState>> = Rc::new(RefCell::new(
        (0..(num_components) as usize)
            .map(|_| <brunsli_ComponentState>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset(i);
        let mut num_blocks: usize =
            (({ m.with(|__s| __s.width_in_blocks) } * { m.with(|__s| __s.height_in_blocks) })
                as usize);
        num_code_words = {
            (num_code_words).wrapping_add(
                (((2_usize).wrapping_mul(m.with(|__s| __s.approx_total_nonzeros)))
                    .wrapping_add(1024_usize))
                .wrapping_add((3_usize).wrapping_mul(num_blocks)),
            )
        };
        ({
            ComputeCoeffOrder_163(
                m.with(|__s| __s.num_zeros.as_pointer()),
                ((array_field_ptr!(
                    (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i),
                    order
                ) as Ptr<u32>)
                    .offset((0) as isize)),
            )
        });
        ({
            let _mult_row: Ptr<i32> = ((array_field_ptr!(
                (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i),
                mult_row
            ) as Ptr<i32>)
                .offset((0) as isize));
            let _mult_col: Ptr<i32> = ((array_field_ptr!(
                (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i),
                mult_col
            ) as Ptr<i32>)
                .offset((0) as isize));
            ComputeACPredictMultipliers_109(
                (m.with(|__s| __s.quant.as_pointer()) as Ptr<i32>),
                _mult_row,
                _mult_col,
            )
        });
        ({
            brunsli_ComponentStateImpl::SetWidth(
                &(comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i),
                m.with(|__s| __s.width_in_blocks),
            )
        });
        i.prefix_inc();
    }
    ({
        let _num_bands: usize = state.with(|__s| __s.num_contexts);
        brunsli_internal_enc_EntropySourceImpl::Resize(&entropy_source, _num_bands)
    });
    ({
        let _max_num_code_words: usize = num_code_words;
        brunsli_internal_enc_DataStreamImpl::Resize(&data_stream, _max_num_code_words)
    });
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        ({
            EncodeCoeffOrder_168(
                ((array_field_ptr!(
                    (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i),
                    order
                ) as Ptr<u32>)
                    .offset((0) as isize)),
                (data_stream).clone(),
            )
        });
        i.prefix_inc();
    }
    let mut mcu_y: i32 = 0;
    'loop_: while (mcu_y < mcu_rows) {
        let mut i: usize = 0_usize;
        'loop_: while (i < num_components) {
            let mut c: Ptr<brunsli_ComponentState> =
                ((comps.as_pointer() as Ptr<brunsli_ComponentState>).offset(i));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset(i);
            let mut cur_ctx_bits: i32 = m.with(|__s| __s.context_bits);
            let mut cur_order: Ptr<u32> = (array_field_ptr!(c, order) as Ptr<u32>);
            let mut width: i32 = c.with(|__s| __s.width);
            let mut y: i32 = ({ mcu_y } * { m.with(|__s| __s.v_samp) });
            let mut ac_stride: i32 = m.with(|__s| __s.ac_stride);
            let mut b_stride: i32 = m.with(|__s| __s.b_stride);
            let mut prev_row_delta: i32 =
                (((1 - (2 * (y & 1))) * (width + 3)) * kDCTBlockSize_3.with(|rc| *rc.borrow()));
            let mut iy: i32 = 0;
            'loop_: while ({ iy } < { m.with(|__s| __s.v_samp) }) {
                let mut coeffs_in: Ptr<i16> = m
                    .with(|__s| __s.ac_coeffs.clone())
                    .offset((y * ac_stride) as isize);
                let mut block_state: Ptr<u8> = m
                    .with(|__s| __s.block_state.clone())
                    .offset((y * b_stride) as isize);
                let mut prev_row_coeffs: Ptr<i16> = coeffs_in.offset(-((ac_stride) as isize));
                let mut prev_col_coeffs: Ptr<i16> =
                    coeffs_in.offset(-((kDCTBlockSize_3.with(|rc| *rc.borrow())) as isize));
                let mut prev_sgn: Ptr<i32> = ((c.with(|__s| __s.prev_sign.as_pointer())
                    as Ptr<i32>)
                    .offset((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)));
                let mut prev_abs: Ptr<i32> =
                    ((c.with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>).offset(
                        (((((y & 1) * (width + 3)) + 2) * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                            as usize),
                    ));
                let mut x: i32 = 0;
                'loop_: while (x < width) {
                    ({ brunsli_internal_enc_DataStreamImpl::ResizeForBlock(&data_stream) });
                    let mut coeffs: [i16; 64] = [
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16,
                    ];
                    let mut last_nz: i32 = 0;
                    let mut is_empty_block: bool = ((block_state.read()) != 0);
                    if !(is_empty_block) {
                        let mut k: i32 = 1;
                        'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                            let mut k_nat: i32 = ((elem!(cur_order, k).read()) as i32);
                            coeffs[(k) as usize] = { (elem!(coeffs_in, k_nat).read()) };
                            if (coeffs[(k) as usize] != 0) {
                                last_nz = k;
                            }
                            k.prefix_inc();
                        }
                        let mut nzero_context: u8 = ({
                            NumNonzerosContext_104(
                                (c.with(|__s| __s.prev_num_nonzeros.as_pointer()) as Ptr<u8>),
                                x,
                                y,
                            )
                        });
                        ({
                            EncodeNumNonzeros_166(
                                (last_nz as usize),
                                (array_field_ptr!(c, num_nonzero_prob) as Ptr<brunsli_Prob>)
                                    .offset(
                                        ((kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                                            .wrapping_mul((nzero_context as usize)))
                                            as isize,
                                    ),
                                (data_stream).clone(),
                            )
                        });
                    }
                    let mut k: i32 = (kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1);
                    'loop_: while (k > last_nz) {
                        elem!(prev_sgn, k).write(0);
                        elem!(prev_abs, k).write(0);
                        k.prefix_dec();
                    }
                    let mut num_nzeros: usize = 0_usize;
                    let encoded_coeffs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16,
                    ])));
                    let mut k: i32 = last_nz;
                    'loop_: while (k >= 1) {
                        let mut coeff: i16 = coeffs[(k) as usize];
                        let mut is_zero: i32 = (((coeff as i32) == 0) as i32);
                        if (k < last_nz) {
                            let mut bucket: i32 = (({
                                let __idx = ((num_nzeros).wrapping_sub(1_usize)) as usize;
                                kNonzeroBuckets_89.with(|rc| rc.borrow()[__idx])
                            }) as i32);
                            let mut is_zero_ctx: i32 =
                                ((bucket * kDCTBlockSize_3.with(|rc| *rc.borrow())) + k);
                            let mut p: Ptr<brunsli_Prob> = ((c
                                .with(|__s| __s.is_zero_prob.as_pointer())
                                as Ptr<brunsli_Prob>)
                                .offset((is_zero_ctx as usize)));
                            ({
                                let _p: Ptr<brunsli_Prob> = (p).clone();
                                let _bit: i32 = is_zero;
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                        }
                        if !(is_zero != 0) {
                            let mut sign: i32 = (if ((coeff as i32) > 0) { 0 } else { 1 });
                            let mut absval: i32 = if (sign != 0) {
                                -(coeff as i32)
                            } else {
                                (coeff as i32)
                            };
                            let mut k_nat: i32 = ((elem!(cur_order, k).read()) as i32);
                            let mut context_type: usize =
                                ((elem!(context_modes, k_nat).read()) as usize);
                            let avg_ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
                            let sign_ctx: Value<usize> = Rc::new(RefCell::new(
                                kMaxAverageContext_82.with(|rc| *rc.borrow()),
                            ));
                            if ((context_type & 1_usize) != 0) && (y > 0) {
                                if (y > 0) {
                                    let mut offset: usize = ((k_nat & 7) as usize);
                                    ({
                                        let _prev: Ptr<i16> =
                                            prev_row_coeffs.offset((offset) as isize);
                                        let _cur: Ptr<i16> = (encoded_coeffs.as_pointer()
                                            as Ptr<i16>)
                                            .offset((offset) as isize);
                                        let _mult: Ptr<i32> = ((array_field_ptr!(c, mult_col)
                                            as Ptr<i32>)
                                            .offset(((offset).wrapping_mul(8_usize)) as isize));
                                        ACPredictContextRow_103(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (avg_ctx.as_pointer()),
                                            (sign_ctx.as_pointer()),
                                        )
                                    });
                                }
                            } else if ((context_type & 2_usize) != 0) && (x > 0) {
                                if (x > 0) {
                                    let mut offset: usize = ((k_nat & !7) as usize);
                                    ({
                                        let _prev: Ptr<i16> =
                                            prev_col_coeffs.offset((offset) as isize);
                                        let _cur: Ptr<i16> = (encoded_coeffs.as_pointer()
                                            as Ptr<i16>)
                                            .offset((offset) as isize);
                                        let _mult: Ptr<i32> = ((array_field_ptr!(c, mult_row)
                                            as Ptr<i32>)
                                            .offset((offset) as isize));
                                        ACPredictContextCol_102(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (avg_ctx.as_pointer()),
                                            (sign_ctx.as_pointer()),
                                        )
                                    });
                                }
                            } else if !(context_type != 0) {
                                (*avg_ctx.borrow_mut()) = (({
                                    let _vals: Ptr<i32> = prev_abs.offset((k) as isize);
                                    let _prev_row_delta: i32 = prev_row_delta;
                                    WeightedAverageContext_98(_vals, _prev_row_delta)
                                })
                                    as usize);
                                (*sign_ctx.borrow_mut()) =
                                    (({ ((elem!(prev_sgn, k).read()) * 3) } + {
                                        (elem!(
                                            prev_sgn,
                                            (k - kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                        )
                                        .read())
                                    }) as usize);
                            }
                            (*sign_ctx.borrow_mut()) = {
                                ((*sign_ctx.borrow()).wrapping_mul(
                                    (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize),
                                ))
                                .wrapping_add((k as usize))
                            };
                            let mut sign_p: Ptr<brunsli_Prob> =
                                ((c.with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                                    .offset((*sign_ctx.borrow())));
                            ({
                                let _p: Ptr<brunsli_Prob> = (sign_p).clone();
                                let _bit: i32 = sign;
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                            elem!(prev_sgn, k).write({ (sign + 1) });
                            let mut zdens_ctx: usize = (m.with(|__s| __s.context_offset))
                                .wrapping_add(
                                    (({
                                        ZeroDensityContext_96(
                                            num_nzeros,
                                            (k as usize),
                                            (cur_ctx_bits as usize),
                                        )
                                    }) as usize),
                                );
                            if (absval <= kNumDirectCodes_141.with(|rc| *rc.borrow())) {
                                ({
                                    let _code: usize = ((absval - 1) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = (*avg_ctx.borrow());
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    brunsli_internal_enc_DataStreamImpl::AddCode(
                                        &data_stream,
                                        _code,
                                        _band,
                                        _context,
                                        _s,
                                    )
                                });
                            } else {
                                let mut base_code: i32 =
                                    ((absval - kNumDirectCodes_141.with(|rc| *rc.borrow())) + 1);
                                let mut nbits: i32 =
                                    (({ Log2FloorNonZero_74((base_code as u32)) }) - 1);
                                ({
                                    let _code: usize =
                                        ((kNumDirectCodes_141.with(|rc| *rc.borrow()) + nbits)
                                            as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = (((*avg_ctx.borrow()) as u32) as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    brunsli_internal_enc_DataStreamImpl::AddCode(
                                        &data_stream,
                                        _code,
                                        _band,
                                        _context,
                                        _s,
                                    )
                                });
                                let mut extra_bits: i32 = (base_code - (2 << nbits));
                                let mut first_extra_bit: i32 = ((extra_bits >> nbits) & 1);
                                let mut p: Ptr<brunsli_Prob> = ((c
                                    .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset((((k * 10) + nbits) as usize)));
                                ({
                                    let _p: Ptr<brunsli_Prob> = (p).clone();
                                    let _bit: i32 = first_extra_bit;
                                    brunsli_internal_enc_DataStreamImpl::AddBit(
                                        &data_stream,
                                        _p,
                                        _bit,
                                    )
                                });
                                if (nbits > 0) {
                                    let mut left_over_bits: i32 = (extra_bits & ((1 << nbits) - 1));
                                    ({
                                        let _nbits: i32 = nbits;
                                        let _bits: i32 = left_over_bits;
                                        brunsli_internal_enc_DataStreamImpl::AddBits(
                                            &data_stream,
                                            _nbits,
                                            _bits,
                                        )
                                    });
                                }
                            }
                            num_nzeros.prefix_inc();
                            (*encoded_coeffs.borrow_mut())[(k_nat) as usize] = coeff;
                            elem!(prev_abs, k).write({ absval });
                        } else {
                            elem!(prev_sgn, k).write(0);
                            elem!(prev_abs, k).write(0);
                        }
                        k.prefix_dec();
                    }
                    if !(num_nzeros <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                                1329,
                                Ptr::<i8>::from_string_literal(b"EncodeAC"),
                            )
                        });
                        'loop_: while true {}
                    };
                    elem!(
                        (c.with(|__s| __s.prev_num_nonzeros.as_pointer()) as Ptr<u8>),
                        (x as usize)
                    )
                    .write((num_nzeros as u8));
                    block_state.prefix_inc();
                    coeffs_in += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    prev_sgn += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    prev_abs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    prev_row_coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    prev_col_coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    x.prefix_inc();
                }
                prev_row_delta *= -1_i32;
                {
                    iy.prefix_inc();
                    y.prefix_inc()
                };
            }
            i.prefix_inc();
        }
        mcu_y.prefix_inc();
    }
}
pub fn PrepareEntropyCodes_189(
    mut state: Ptr<brunsli_internal_enc_State>,
) -> Option<Value<brunsli_internal_enc_EntropyCodes>> {
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    let group_context_offsets: Value<Vec<u64>> = Rc::new(RefCell::new(
        (0..((1_usize).wrapping_add(num_components)) as usize)
            .map(|_| <u64>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let __rhs = ({
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            )
            .upgrade()
            .deref())
            .context_offset
        } as u64);
        (*group_context_offsets.borrow_mut())[(i).wrapping_add(1_usize)] = __rhs;
        i.prefix_inc();
    }
    return ({
        brunsli_internal_enc_EntropySourceImpl::Finish(
            &field_ptr!(state, entropy_source),
            group_context_offsets.as_pointer(),
        )
    })
    .take();
}
pub fn BrunsliSerialize_190(
    state: Ptr<brunsli_internal_enc_State>,
    jpg: Ptr<brunsli_JPEGData>,
    mut skip_sections: u32,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut ok: bool = true;
    let encode_section: Value<
        FnPtr<
            fn(
                u8,
                FnPtr<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                >,
                usize,
            ) -> bool,
        >,
    > = Rc::new(RefCell::new(lambda!(
        {
            let jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let state: Ptr<Ptr<brunsli_internal_enc_State>> = state.as_pointer();
            let len: Ptr<Ptr<usize>> = len.as_pointer();
            let data: Ptr<Ptr<u8>> = data.as_pointer();
            let pos: Ptr<usize> = pos.as_pointer();
        },
        |tag: u8,
         fn_: FnPtr<
            fn(
                Ptr<brunsli_JPEGData>,
                Ptr<brunsli_internal_enc_State>,
                Ptr::<u8>,
                Ptr::<usize>,
            ) -> bool,
        >,
         size: usize|
         -> bool {
            return ({
                let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                let _s: Ptr<brunsli_internal_enc_State> = (state.read());
                let _tag: u8 = tag;
                let _write_section: FnPtr<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                > = fn_;
                let _section_size_bytes: usize = size;
                let _len: usize = ((len.read()).read());
                let _data: Ptr<u8> = (data.read());
                let _pos: Ptr<usize> = (pos).clone();
                EncodeSection_180(
                    _jpg,
                    _s,
                    _tag,
                    _write_section,
                    _section_size_bytes,
                    _len,
                    _data,
                    _pos,
                )
            });
        }
    )));
    if !((skip_sections & (1_u32 << (kBrunsliSignatureTag_30.with(|rc| *rc.borrow()) as i32))) != 0)
    {
        ok = ({
            let _len: usize = ((*len.borrow()).read());
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (pos.as_pointer());
            EncodeSignature_171(_len, _data, _pos)
        });
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliHeaderTag_31.with(|rc| *rc.borrow()) as i32))) != 0) {
        ok = ({
            (*encode_section.borrow()).call(
                kBrunsliHeaderTag_31.with(|rc| *rc.borrow()),
                FnPtr::<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                >::new(EncodeHeader_173),
                1_usize,
            )
        });
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        ok = ({
            (*encode_section.borrow()).call(
                kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()),
                FnPtr::<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                >::new(EncodeJPEGInternals_175),
                ({ Base128Size_146(({ EstimateAuxDataSize_144((jpg).clone()) })) }),
            )
        });
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliMetaDataTag_32.with(|rc| *rc.borrow()) as i32))) != 0)
    {
        ok = ({
            let _tag: u8 = kBrunsliMetaDataTag_32.with(|rc| *rc.borrow());
            let _fn_: FnPtr<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            > = FnPtr::<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            >::new(EncodeMetaData_174);
            let _size: usize =
                ({ Base128Size_146(((*len.borrow()).read()).wrapping_sub((*pos.borrow()))) });
            (*encode_section.borrow()).call(_tag, _fn_, _size)
        })
        .clone();
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as i32))) != 0)
    {
        ok = ({
            (*encode_section.borrow()).call(
                kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()),
                FnPtr::<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                >::new(EncodeQuantData_176),
                2_usize,
            )
        });
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        ok = ({
            let _tag: u8 = kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow());
            let _fn_: FnPtr<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            > = FnPtr::<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            >::new(EncodeHistogramData_177);
            let _size: usize =
                ({ Base128Size_146(((*len.borrow()).read()).wrapping_sub((*pos.borrow()))) });
            (*encode_section.borrow()).call(_tag, _fn_, _size)
        })
        .clone();
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as i32))) != 0) {
        ok = ({
            let _tag: u8 = kBrunsliDCDataTag_36.with(|rc| *rc.borrow());
            let _fn_: FnPtr<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            > = FnPtr::<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            >::new(EncodeDCData_178);
            let _size: usize =
                ({ Base128Size_146(((*len.borrow()).read()).wrapping_sub((*pos.borrow()))) });
            (*encode_section.borrow()).call(_tag, _fn_, _size)
        })
        .clone();
        if !(ok) {
            return false;
        }
    }
    if !((skip_sections & (1_u32 << (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as i32))) != 0) {
        ok = ({
            let _tag: u8 = kBrunsliACDataTag_37.with(|rc| *rc.borrow());
            let _fn_: FnPtr<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            > = FnPtr::<
                fn(
                    Ptr<brunsli_JPEGData>,
                    Ptr<brunsli_internal_enc_State>,
                    Ptr<u8>,
                    Ptr<usize>,
                ) -> bool,
            >::new(EncodeACData_179);
            let _size: usize =
                ({ Base128Size_146(((*len.borrow()).read()).wrapping_sub((*pos.borrow()))) });
            (*encode_section.borrow()).call(_tag, _fn_, _size)
        })
        .clone();
        if !(ok) {
            return false;
        }
    }
    (*len.borrow()).write({ (*pos.borrow()) });
    return true;
}
pub fn BrunsliEncodeJpeg_191(
    jpg: Ptr<brunsli_JPEGData>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    let state: Value<brunsli_internal_enc_State> =
        Rc::new(RefCell::new(<brunsli_internal_enc_State>::default()));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        { (*state.borrow()).meta.as_pointer() };
    let mut num_components: usize = (*jpg.with(|__s| __s.components.clone()).borrow()).len();
    (*state.borrow_mut()).use_legacy_context_model = !((jpg.with(|__s| __s.version) & 2) != 0);
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        CalculateMeta_186(_jpg, _state)
    }) {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let __rhs = ({
            SampleNumNonZeros_181(
                (({ (*state.borrow()).meta.as_pointer() }
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset(i)),
            )
        });
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            ),
            approx_total_nonzeros
        )
        .write(__rhs);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let __rhs = ({
            SelectContextBits_183(
                ({
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .approx_total_nonzeros
                })
                .wrapping_add(1_usize),
            )
        });
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            ),
            context_bits
        )
        .write(__rhs);
        i.prefix_inc();
    }
    let mut num_contexts: usize = num_components;
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            ),
            context_offset
        )
        .write(num_contexts);
        {
            let rhs_0 = (num_contexts).wrapping_add(
                (({
                    let __idx = ({
                        (*elem!(
                            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                                as Ptr<brunsli_internal_enc_ComponentMeta>),
                            i
                        )
                        .upgrade()
                        .deref())
                        .context_bits
                    }) as usize;
                    kNumNonzeroContextSkip_94.with(|rc| rc.borrow()[__idx])
                }) as usize),
            );
            num_contexts = rhs_0
        };
        i.prefix_inc();
    }
    (*state.borrow_mut()).num_contexts = num_contexts;
    let dc_prediction_errors: Value<Vec<Value<Vec<i16>>>> = Rc::new(RefCell::new(
        (0..(num_components) as usize)
            .map(|_| <Value<Vec<i16>>>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        {
            let __a0 = (({
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                }
            } * {
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .height_in_blocks
                }
            }) as usize) as usize;
            elem!(
                (dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>),
                i
            )
            .with_mut(|__v: &mut Value<Vec<i16>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <i16>::default())
            })
        };
        let __rhs = ((dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>)
            .offset(i)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<i16>);
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            ),
            dc_prediction_errors
        )
        .write(__rhs);
        i.prefix_inc();
    }
    if !({ PredictDCCoeffs_185((state.as_pointer())) }) {
        return false;
    }
    let block_state: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(
        (0..(num_components) as usize)
            .map(|_| <Value<Vec<u8>>>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        {
            let __a0 = (({
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                }
            } * {
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .height_in_blocks
                }
            }) as usize) as usize;
            elem!((block_state.as_pointer() as Ptr<Value<Vec<u8>>>), i).with_mut(
                |__v: &mut Value<Vec<u8>>| {
                    (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                },
            )
        };
        let __rhs = ((block_state.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(i)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>);
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                i
            ),
            block_state
        )
        .write(__rhs);
        i.prefix_inc();
    }
    ({ EncodeDC_187((state.as_pointer())) });
    ({ EncodeAC_188((state.as_pointer())) });
    let mut entropy_codes: Option<Value<brunsli_internal_enc_EntropyCodes>> =
        ({ PrepareEntropyCodes_189((state.as_pointer())) }).take();
    (*state.borrow_mut()).entropy_codes = entropy_codes.as_pointer();
    return ({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _data: Ptr<u8> = data;
        let _len: Ptr<usize> = len;
        BrunsliSerialize_190(_state, _jpg, 0_u32, _data, _len)
    });
}
thread_local!(
    pub static kMaxBypassHeaderSize_192: Value<usize> = Rc::new(RefCell::new(((5 * 6) as usize)));
);
pub fn GetBrunsliBypassSize_193(mut jpg_size: usize) -> usize {
    return ((jpg_size).wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow())))
        .wrapping_add(kMaxBypassHeaderSize_192.with(|rc| *rc.borrow()));
}
pub fn EncodeOriginalJpg_194(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_enc_State>,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    &(state);
    if ((jpg.with(|__s| __s.original_jpg.clone())).is_null())
        || ({ jpg.with(|__s| __s.original_jpg_size) } > { (len.read()) })
    {
        return false;
    }
    {
        (data).to_any().memcpy(
            &(jpg.with(|__s| __s.original_jpg.clone()) as Ptr<u8>).to_any(),
            jpg.with(|__s| __s.original_jpg_size) as usize,
        );
        (data).to_any()
    };
    len.write({ jpg.with(|__s| __s.original_jpg_size) });
    return true;
}
pub fn BrunsliEncodeJpegBypass_195(
    mut jpg_data: Ptr<u8>,
    mut jpg_data_len: usize,
    mut data: Ptr<u8>,
    mut len: Ptr<usize>,
) -> bool {
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if !({
        let _len: usize = (len.read());
        let _data: Ptr<u8> = (data).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeSignature_171(_len, _data, _pos)
    }) {
        return false;
    }
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
    if !({
        ReadJpeg_196(
            (jpg_data).clone(),
            jpg_data_len,
            brunsli_JpegReadMode_JPEG_READ_HEADER,
            (jpg.as_pointer()),
        )
    }) {
        (*jpg.borrow_mut()).width = 0;
        (*jpg.borrow_mut()).height = 0;
        {
            let __a0 = 1_usize as usize;
            (*{ (*jpg.borrow()).components.clone() }.borrow_mut())
                .resize_with(__a0, || <brunsli_JPEGComponent>::default())
        };
        field!(
            elem!(
                ({ (*jpg.borrow()).components.as_pointer() } as Ptr<brunsli_JPEGComponent>),
                0_usize
            ),
            h_samp_factor
        )
        .write(1);
        field!(
            elem!(
                ({ (*jpg.borrow()).components.as_pointer() } as Ptr<brunsli_JPEGComponent>),
                0_usize
            ),
            v_samp_factor
        )
        .write(1);
    }
    (*jpg.borrow_mut()).version = kFallbackVersion_2.with(|rc| *rc.borrow());
    (*jpg.borrow_mut()).original_jpg = (jpg_data).clone();
    (*jpg.borrow_mut()).original_jpg_size = jpg_data_len;
    let state: Value<brunsli_internal_enc_State> =
        Rc::new(RefCell::new(<brunsli_internal_enc_State>::default()));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _s: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _tag: u8 = kBrunsliHeaderTag_31.with(|rc| *rc.borrow());
        let _write_section: FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        > = FnPtr::<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >::new(EncodeHeader_173);
        let _len: usize = (len.read());
        let _data: Ptr<u8> = (data).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeSection_180(_jpg, _s, _tag, _write_section, 1_usize, _len, _data, _pos)
    }) {
        return false;
    }
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _s: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _tag: u8 = kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow());
        let _write_section: FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        > = FnPtr::<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >::new(EncodeOriginalJpg_194);
        let _section_size_bytes: usize = ({ Base128Size_146(jpg_data_len) });
        let _len: usize = (len.read());
        let _data: Ptr<u8> = (data).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeSection_180(
            _jpg,
            _s,
            _tag,
            _write_section,
            _section_size_bytes,
            _len,
            _data,
            _pos,
        )
    }) {
        return false;
    }
    len.write({ (*pos.borrow()) });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct brunsli_HuffmanTree {
    #[offset(0)]
    pub total_count: u32,
    #[offset(4)]
    pub index_left: i16,
    #[offset(6)]
    pub index_right_or_value: i16,
}
impl brunsli_HuffmanTree {
    pub fn new(mut count: u32, mut left: i16, mut right: i16) -> Self {
        Self {
            total_count: count,
            index_left: left,
            index_right_or_value: right,
        }
    }
}
pub fn StoreVarLenUint8_197(mut n: usize, mut storage: Ptr<brunsli_Storage>) {
    if (n == 0_usize) {
        ({ WriteBits_120(1_usize, 0_u64, (storage).clone()) });
    } else {
        ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
        let mut nbits: usize = (({ Log2FloorNonZero_74((n as u32)) }) as usize);
        ({ WriteBits_120(3_usize, (nbits as u64), (storage).clone()) });
        ({
            let _n_bits: usize = nbits;
            let _bits: u64 = ((n).wrapping_sub((1_usize << nbits)) as u64);
            WriteBits_120(_n_bits, _bits, (storage).clone())
        });
    }
}
pub fn IndexOf_198(v: Ptr<Vec<u32>>, mut value: u32) -> usize {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v.upgrade().deref()).len() }) {
        if ({ (elem!((Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), i).read()) } == { value }) {
            return i;
        }
        i.prefix_inc();
    }
    return i;
}
pub fn MoveToFront_199(mut v: Ptr<Vec<u32>>, mut index: usize) {
    let mut value: u32 = (elem!(((Ptr::<Vec<u32>>::decay(&(v))) as Ptr<u32>), index).read());
    let mut i: usize = index;
    'loop_: while (i != 0_usize) {
        let __rhs = (elem!(
            ((Ptr::<Vec<u32>>::decay(&(v))) as Ptr<u32>),
            (i).wrapping_sub(1_usize)
        )
        .read());
        elem!(((Ptr::<Vec<u32>>::decay(&(v))) as Ptr<u32>), i).write(__rhs);
        i.prefix_dec();
    }
    elem!(((Ptr::<Vec<u32>>::decay(&(v))) as Ptr<u32>), 0_usize).write(value);
}
pub fn MoveToFrontTransform_200(v: Ptr<Vec<u32>>) -> Vec<u32> {
    if (*v.upgrade().deref()).is_empty() {
        return (*v.upgrade().deref()).clone();
    }
    let mut max_value: u32 = ({
        let __count = (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>)
            .to_end()
            .get_offset()
            - (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>).get_offset();
        let max_index = PtrValueIter::new(&(Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), __count)
            .enumerate()
            .max_by(|&(idx_a, val_a), &(idx_b, val_b)| {
                val_a.cmp(&val_b).then_with(|| idx_b.cmp(&idx_a))
            })
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>) + max_index
    }
    .read());
    let mtf: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(((max_value).wrapping_add(1_u32)) as usize) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let mut i: u32 = 0_u32;
    'loop_: while (i <= max_value) {
        let __rhs = i;
        (*mtf.borrow_mut())[(i as usize)] = __rhs;
        i.prefix_inc();
    }
    let mut result: Vec<u32> = (0..((*v.upgrade().deref()).len()) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v.upgrade().deref()).len() }) {
        let mut index: usize = ({
            let _v: Ptr<Vec<u32>> = mtf.as_pointer();
            let _value: u32 = (elem!((Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), i).read());
            IndexOf_198(_v, _value)
        });
        if !(index < (*mtf.borrow()).len()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"context_map_encode.cc"),
                    60,
                    Ptr::<i8>::from_string_literal(b"MoveToFrontTransform"),
                )
            });
            'loop_: while true {}
        };
        result[i] = (index as u32);
        ({ MoveToFront_199((mtf.as_pointer()), index) });
        i.prefix_inc();
    }
    return std::mem::take(&mut result);
}
pub fn RunLengthCodeZeros_201(
    v_in: Ptr<Vec<u32>>,
    mut max_run_length_prefix: Ptr<u32>,
    mut v_out: Ptr<Vec<u32>>,
    mut extra_bits: Ptr<Vec<u32>>,
) {
    let max_reps: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v_in.upgrade().deref()).len() }) {
        'loop_: while ({ i } < { (*v_in.upgrade().deref()).len() })
            && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), i).read()) != 0_u32)
        {
            i.prefix_inc();
        }
        let mut i0: usize = i;
        'loop_: while ({ i } < { (*v_in.upgrade().deref()).len() })
            && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), i).read()) == 0_u32)
        {
            i.prefix_inc();
        }
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((i).wrapping_sub(i0) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*max_reps.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        (*max_reps.borrow_mut()) = __rhs;
    }
    let max_prefix: Value<u32> = Rc::new(RefCell::new(
        (if ((*max_reps.borrow()) > 0_usize) {
            ({ Log2FloorNonZero_74(((*max_reps.borrow()) as u32)) })
        } else {
            0
        } as u32),
    ));
    let __rhs = (if max_prefix.as_pointer().read() <= (max_run_length_prefix).clone().read() {
        max_prefix.as_pointer()
    } else {
        (max_run_length_prefix).clone()
    }
    .read());
    (*max_prefix.borrow_mut()) = __rhs;
    max_run_length_prefix.write({ (*max_prefix.borrow()) });
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v_in.upgrade().deref()).len() }) {
        if ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), i).read()) != 0_u32) {
            {
                let __a1 = (elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), i).read())
                    .wrapping_add((max_run_length_prefix.read()));
                v_out.with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
            };
            {
                let __a1 = 0_u32;
                extra_bits.with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
            };
            i.prefix_inc();
        } else {
            let mut reps: u32 = 1_u32;
            let mut k: usize = (i).wrapping_add(1_usize);
            'loop_: while ({ k } < { (*v_in.upgrade().deref()).len() })
                && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), k).read()) == 0_u32)
            {
                reps.prefix_inc();
                k.prefix_inc();
            }
            i = { (i).wrapping_add((reps as usize)) };
            'loop_: while (reps != 0_u32) {
                if (reps < (2_u32 << (*max_prefix.borrow()))) {
                    let mut run_length_prefix: u32 = (({ Log2FloorNonZero_74(reps) }) as u32);
                    {
                        let a0_clone = run_length_prefix.clone();
                        v_out.with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    {
                        let __a1 = (reps).wrapping_sub((1_u32 << run_length_prefix));
                        extra_bits.with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
                    };
                    break;
                } else {
                    {
                        let a0_clone = (*max_prefix.borrow()).clone();
                        v_out.with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    {
                        let __a1 = (1_u32 << (*max_prefix.borrow())).wrapping_sub((1_u32 as u32));
                        extra_bits.with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
                    };
                    reps = {
                        (reps).wrapping_sub(
                            (2_u32 << (*max_prefix.borrow())).wrapping_sub((1_u32 as u32)),
                        )
                    };
                }
            }
        };
    }
}
pub fn EncodeContextMap_164(
    context_map: Ptr<Vec<u32>>,
    mut num_clusters: usize,
    mut storage: Ptr<brunsli_Storage>,
) {
    ({ StoreVarLenUint8_197((num_clusters).wrapping_sub(1_usize), (storage).clone()) });
    if (num_clusters == 1_usize) {
        return;
    }
    let transformed_symbols: Value<Vec<u32>> = Rc::new(RefCell::new(
        ({ MoveToFrontTransform_200((context_map).clone()) }),
    ));
    let rle_symbols: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let extra_bits: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let max_run_length_prefix: Value<u32> = Rc::new(RefCell::new(6_u32));
    ({
        RunLengthCodeZeros_201(
            transformed_symbols.as_pointer(),
            (max_run_length_prefix.as_pointer()),
            (rle_symbols.as_pointer()),
            (extra_bits.as_pointer()),
        )
    });
    let symbol_histogram: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..272).map(|_| 0_u32).collect::<Box<[u32]>>(),
    ));
    {
        ((symbol_histogram.as_pointer() as Ptr<u32>) as Ptr<u32>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u32; 272]>() as usize);
        ((symbol_histogram.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any()
    };
    let mut i: usize = 0_usize;
    'loop_: while (i < (*rle_symbols.borrow()).len()) {
        (*symbol_histogram.borrow_mut())[({ (*rle_symbols.borrow())[i] }) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut use_rle: bool = ((*max_run_length_prefix.borrow()) > 0_u32);
    ({ WriteBits_120(1_usize, (use_rle as u64), (storage).clone()) });
    if use_rle {
        ({
            WriteBits_120(
                4_usize,
                (((*max_run_length_prefix.borrow()).wrapping_sub(1_u32)) as u64),
                (storage).clone(),
            )
        });
    }
    let bit_depths: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..272).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let bit_codes: Value<Box<[u16]>> = Rc::new(RefCell::new(
        (0..272).map(|_| 0_u16).collect::<Box<[u16]>>(),
    ));
    {
        ((bit_depths.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 272]>() as usize);
        ((bit_depths.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    {
        ((bit_codes.as_pointer() as Ptr<u16>) as Ptr<u16>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u16; 272]>() as usize);
        ((bit_codes.as_pointer() as Ptr<u16>) as Ptr<u16>).to_any()
    };
    ({
        BuildAndStoreHuffmanTree_202(
            (symbol_histogram.as_pointer() as Ptr<u32>),
            (num_clusters).wrapping_add(((*max_run_length_prefix.borrow()) as usize)),
            (bit_depths.as_pointer() as Ptr<u8>),
            (bit_codes.as_pointer() as Ptr<u16>),
            (storage).clone(),
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while (i < (*rle_symbols.borrow()).len()) {
        ({
            let _n_bits: usize =
                ((*bit_depths.borrow())[({ (*rle_symbols.borrow())[i] }) as usize] as usize);
            let _bits: u64 =
                ((*bit_codes.borrow())[({ (*rle_symbols.borrow())[i] }) as usize] as u64);
            WriteBits_120(_n_bits, _bits, (storage).clone())
        });
        if ({ (*rle_symbols.borrow())[i] } > 0_u32)
            && ({ (*rle_symbols.borrow())[i] } <= (*max_run_length_prefix.borrow()))
        {
            ({
                let _n_bits: usize = ({ (*rle_symbols.borrow())[i] } as usize);
                let _bits: u64 = ({ (*extra_bits.borrow())[i] } as u64);
                WriteBits_120(_n_bits, _bits, (storage).clone())
            });
        }
        i.prefix_inc();
    }
    ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
}
pub fn GetPopulationCountPrecision_203(mut logcount: u32) -> u32 {
    return (((logcount).wrapping_add(1_u32)) >> 1);
}
thread_local!(
    pub static kHistogramLengthBitLengths_204: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([
            8_u8, 8_u8, 6_u8, 6_u8, 6_u8, 5_u8, 4_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 4_u8,
            5_u8, 7_u8,
        ])));
);
thread_local!(
    pub static kHistogramLengthSymbols_205: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        127_u16, 255_u16, 15_u16, 47_u16, 31_u16, 7_u16, 3_u16, 0_u16, 4_u16, 2_u16, 6_u16, 1_u16,
        5_u16, 11_u16, 23_u16, 63_u16,
    ])));
);
thread_local!(
    pub static kLogCountBitLengths_206: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        5_u8, 4_u8, 4_u8, 4_u8, 3_u8, 3_u8, 2_u8, 3_u8, 3_u8, 6_u8, 6_u8,
    ])));
);
thread_local!(
    pub static kLogCountSymbols_207: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        15_u16, 3_u16, 11_u16, 7_u16, 2_u16, 6_u16, 0_u16, 1_u16, 5_u16, 31_u16, 63_u16,
    ])));
);
pub fn SmallestIncrement_208(mut count: i32) -> i32 {
    if !(count > 0) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                39,
                Ptr::<i8>::from_string_literal(b"SmallestIncrement"),
            )
        });
        'loop_: while true {}
    };
    let mut bits: i32 = ({ Log2FloorNonZero_74((count as u32)) });
    let mut drop_bits: i32 =
        (((bits as u32).wrapping_sub(({ GetPopulationCountPrecision_203((bits as u32)) }))) as i32);
    return (1 << drop_bits);
}
pub fn RebalanceHistogram_209(
    mut targets: Ptr<f32>,
    mut max_symbol: i32,
    mut table_size: i32,
    mut omit_pos: Ptr<i32>,
    mut counts: Ptr<i32>,
) -> bool {
    if !(table_size >= 2) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                48,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    let mut sum: i32 = 0;
    let mut sum_nonrounded: f32 = (0.0E+0 as f32);
    let mut remainder_pos: i32 = -1_i32;
    let mut remainder_log: i32 = -1_i32;
    let mut n: i32 = 0;
    'loop_: while (n < max_symbol) {
        if ((elem!(targets, n).read()) > 0_f32) {
            sum_nonrounded += { (elem!(targets, n).read()) };
            elem!(counts, n)
                .write({ (((((elem!(targets, n).read()) as f64) + 5.0E-1) as u32) as i32) });
            if ((elem!(counts, n).read()) == 0) {
                elem!(counts, n).write(1);
            }
            if ({ (elem!(counts, n).read()) } == { table_size }) {
                elem!(counts, n).write({ (table_size - 1) });
            }
            let mut inc: i32 = ({ SmallestIncrement_208((elem!(counts, n).read())) });
            {
                let __rhs = { ({ (elem!(counts, n).read()) } & { (inc - 1) }) };
                elem!(counts, n).with_mut(|__v| *__v = *__v - __rhs)
            };
            let mut target: f32 = if false {
                (sum_nonrounded - (sum as f32))
            } else {
                (elem!(targets, n).read())
            };
            if ((elem!(counts, n).read()) == 0)
                || (({ target } > { (({ (elem!(counts, n).read()) } + { (inc / 2) }) as f32) })
                    && ({ ({ (elem!(counts, n).read()) } + { inc }) } < { table_size }))
            {
                {
                    let __rhs = { inc };
                    elem!(counts, n).with_mut(|__v| *__v = *__v + __rhs)
                };
            }
            sum += { (elem!(counts, n).read()) };
            let mut count_log: i32 = ({ Log2FloorNonZero_74(((elem!(counts, n).read()) as u32)) });
            if (count_log > remainder_log) {
                remainder_pos = n;
                remainder_log = count_log;
            }
        }
        n.prefix_inc();
    }
    if !(remainder_pos != -1_i32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                81,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    {
        let __rhs = { (sum - table_size) };
        elem!(counts, remainder_pos).with_mut(|__v| *__v = *__v - __rhs)
    };
    omit_pos.write({ remainder_pos });
    return ((elem!(counts, remainder_pos).read()) > 0);
}
pub fn RebalanceHistogram_210(
    mut targets: Ptr<f32>,
    mut max_symbol: i32,
    mut table_size: i32,
    mut omit_pos: Ptr<i32>,
    mut counts: Ptr<i32>,
) -> bool {
    if !(table_size >= 2) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                48,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    let mut sum: i32 = 0;
    let mut sum_nonrounded: f32 = (0.0E+0 as f32);
    let mut remainder_pos: i32 = -1_i32;
    let mut remainder_log: i32 = -1_i32;
    let mut n: i32 = 0;
    'loop_: while (n < max_symbol) {
        if ((elem!(targets, n).read()) > 0_f32) {
            sum_nonrounded += { (elem!(targets, n).read()) };
            elem!(counts, n)
                .write({ (((((elem!(targets, n).read()) as f64) + 5.0E-1) as u32) as i32) });
            if ((elem!(counts, n).read()) == 0) {
                elem!(counts, n).write(1);
            }
            if ({ (elem!(counts, n).read()) } == { table_size }) {
                elem!(counts, n).write({ (table_size - 1) });
            }
            let mut inc: i32 = ({ SmallestIncrement_208((elem!(counts, n).read())) });
            {
                let __rhs = { ({ (elem!(counts, n).read()) } & { (inc - 1) }) };
                elem!(counts, n).with_mut(|__v| *__v = *__v - __rhs)
            };
            let mut target: f32 = if true {
                (sum_nonrounded - (sum as f32))
            } else {
                (elem!(targets, n).read())
            };
            if ((elem!(counts, n).read()) == 0)
                || (({ target } > { (({ (elem!(counts, n).read()) } + { (inc / 2) }) as f32) })
                    && ({ ({ (elem!(counts, n).read()) } + { inc }) } < { table_size }))
            {
                {
                    let __rhs = { inc };
                    elem!(counts, n).with_mut(|__v| *__v = *__v + __rhs)
                };
            }
            sum += { (elem!(counts, n).read()) };
            let mut count_log: i32 = ({ Log2FloorNonZero_74(((elem!(counts, n).read()) as u32)) });
            if (count_log > remainder_log) {
                remainder_pos = n;
                remainder_log = count_log;
            }
        }
        n.prefix_inc();
    }
    if !(remainder_pos != -1_i32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                81,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    {
        let __rhs = { (sum - table_size) };
        elem!(counts, remainder_pos).with_mut(|__v| *__v = *__v - __rhs)
    };
    omit_pos.write({ remainder_pos });
    return ((elem!(counts, remainder_pos).read()) > 0);
}
pub fn NormalizeCounts_124(
    mut counts: Ptr<i32>,
    mut omit_pos: Ptr<i32>,
    mut length: i32,
    mut precision_bits: i32,
    mut num_symbols: Ptr<i32>,
    mut symbols: Ptr<i32>,
) {
    if !(precision_bits > 0) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                89,
                Ptr::<i8>::from_string_literal(b"NormalizeCounts"),
            )
        });
        'loop_: while true {}
    };
    let mut table_size: i32 = (1 << precision_bits);
    let mut total: u64 = 0_u64;
    let mut max_symbol: i32 = 0;
    let mut symbol_count: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while (n < length) {
        total = { (total).wrapping_add(((elem!(counts, n).read()) as u64)) };
        if ((elem!(counts, n).read()) > 0) {
            if (symbol_count < kMaxNumSymbolsForSmallCode_121.with(|rc| *rc.borrow())) {
                elem!(symbols, symbol_count).write({ n });
            }
            symbol_count.prefix_inc();
            max_symbol = (n + 1);
        }
        n.prefix_inc();
    }
    num_symbols.write({ symbol_count });
    if (symbol_count == 0) {
        return;
    }
    if (symbol_count == 1) {
        elem!(counts, (elem!(symbols, 0).read())).write({ table_size });
        return;
    }
    if !(symbol_count <= table_size) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                112,
                Ptr::<i8>::from_string_literal(b"NormalizeCounts"),
            )
        });
        'loop_: while true {}
    };
    let mut norm: f32 = ((1.0E+0 * (table_size as f32)) / (total as f32));
    let targets: Value<Box<[f32]>> =
        Rc::new(RefCell::new((0..18).map(|_| 0_f32).collect::<Box<[f32]>>()));
    let mut n: i32 = 0;
    'loop_: while (n < max_symbol) {
        (*targets.borrow_mut())[(n) as usize] =
            { ({ norm } * { ((elem!(counts, n).read()) as f32) }) };
        n.prefix_inc();
    }
    if !({
        let _max_symbol: i32 = max_symbol;
        let _table_size: i32 = table_size;
        let _omit_pos: Ptr<i32> = (omit_pos).clone();
        let _counts: Ptr<i32> = (counts).clone();
        RebalanceHistogram_209(
            (targets.as_pointer() as Ptr<f32>),
            _max_symbol,
            _table_size,
            _omit_pos,
            _counts,
        )
    }) {
        if !({
            let _max_symbol: i32 = max_symbol;
            let _table_size: i32 = table_size;
            let _omit_pos: Ptr<i32> = (omit_pos).clone();
            let _counts: Ptr<i32> = (counts).clone();
            RebalanceHistogram_210(
                (targets.as_pointer() as Ptr<f32>),
                _max_symbol,
                _table_size,
                _omit_pos,
                _counts,
            )
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                    126,
                    Ptr::<i8>::from_string_literal(b"NormalizeCounts"),
                )
            });
            'loop_: while true {}
        };
    }
}
pub fn EncodeCounts_125(
    mut counts: Ptr<i32>,
    mut omit_pos: i32,
    mut num_symbols: i32,
    mut symbols: Ptr<i32>,
    mut storage: Ptr<brunsli_Storage>,
) {
    let mut max_bits: i32 = 5;
    if (num_symbols <= 2) {
        ({ WriteBits_120(1_usize, 1_u64, (storage).clone()) });
        if (num_symbols == 0) {
            ({ WriteBits_120(((max_bits + 1) as usize), 0_u64, (storage).clone()) });
        } else {
            ({ WriteBits_120(1_usize, ((num_symbols - 1) as u64), (storage).clone()) });
            let mut i: i32 = 0;
            'loop_: while (i < num_symbols) {
                ({
                    WriteBits_120(
                        (max_bits as usize),
                        ((elem!(symbols, i).read()) as u64),
                        (storage).clone(),
                    )
                });
                i.prefix_inc();
            }
        }
        if (num_symbols == 2) {
            ({
                WriteBits_120(
                    (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as usize),
                    ((elem!(counts, (elem!(symbols, 0).read())).read()) as u64),
                    (storage).clone(),
                )
            });
        }
    } else {
        ({ WriteBits_120(1_usize, 0_u64, (storage).clone()) });
        let mut length: i32 = 0;
        let logcounts: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let omit_log: Value<i32> = Rc::new(RefCell::new(0));
        let mut i: i32 = 0;
        'loop_: while (i < 18) {
            if !({ (elem!(counts, i).read()) } <= {
                BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow())
            }) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                        155,
                        Ptr::<i8>::from_string_literal(b"EncodeCounts"),
                    )
                });
                'loop_: while true {}
            };
            if !((elem!(counts, i).read()) >= 0) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                        156,
                        Ptr::<i8>::from_string_literal(b"EncodeCounts"),
                    )
                });
                'loop_: while true {}
            };
            if (i == omit_pos) {
                length = (i + 1);
            } else if ((elem!(counts, i).read()) > 0) {
                let __rhs = (({ Log2FloorNonZero_74(((elem!(counts, i).read()) as u32)) }) + 1);
                (*logcounts.borrow_mut())[(i) as usize] = __rhs;
                length = (i + 1);
                if (i < omit_pos) {
                    let __rhs = {
                        let __tmp_1: Value<i32> =
                            Rc::new(RefCell::new(((*logcounts.borrow())[(i) as usize] + 1)));
                        (if omit_log.as_pointer().read() >= __tmp_1.as_pointer().read() {
                            omit_log.as_pointer()
                        } else {
                            __tmp_1.as_pointer()
                        }
                        .read())
                    };
                    (*omit_log.borrow_mut()) = __rhs;
                } else {
                    let __rhs = (if omit_log.as_pointer().read()
                        >= (logcounts.as_pointer() as Ptr<i32>).offset(i).read()
                    {
                        omit_log.as_pointer()
                    } else {
                        (logcounts.as_pointer() as Ptr<i32>).offset(i)
                    }
                    .read());
                    (*omit_log.borrow_mut()) = __rhs;
                }
            }
            i.prefix_inc();
        }
        (*logcounts.borrow_mut())[(omit_pos) as usize] = (*omit_log.borrow());
        ({
            let _n_bits: usize = (({
                let __idx = (length - 3) as usize;
                kHistogramLengthBitLengths_204.with(|rc| rc.borrow()[__idx])
            }) as usize);
            let _bits: u64 = (({
                let __idx = (length - 3) as usize;
                kHistogramLengthSymbols_205.with(|rc| rc.borrow()[__idx])
            }) as u64);
            WriteBits_120(_n_bits, _bits, (storage).clone())
        });
        let mut i: i32 = 0;
        'loop_: while (i < length) {
            ({
                let _n_bits: usize = (({
                    let __idx = ((*logcounts.borrow())[(i) as usize]) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as usize);
                let _bits: u64 = (({
                    let __idx = ((*logcounts.borrow())[(i) as usize]) as usize;
                    kLogCountSymbols_207.with(|rc| rc.borrow()[__idx])
                }) as u64);
                WriteBits_120(_n_bits, _bits, (storage).clone())
            });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < length) {
            if ((*logcounts.borrow())[(i) as usize] > 1) && (i != omit_pos) {
                let mut bitcount: i32 = (({
                    GetPopulationCountPrecision_203(
                        (((*logcounts.borrow())[(i) as usize] - 1) as u32),
                    )
                }) as i32);
                let mut drop_bits: i32 = (((*logcounts.borrow())[(i) as usize] - 1) - bitcount);
                if !(({ (elem!(counts, i).read()) } & { ((1 << drop_bits) - 1) }) == 0) {
                    ({
                        BrunsliDumpAndAbort_79(
                            Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                            184,
                            Ptr::<i8>::from_string_literal(b"EncodeCounts"),
                        )
                    });
                    'loop_: while true {}
                };
                ({
                    let _n_bits: usize = (bitcount as usize);
                    let _bits: u64 = (({ ({ (elem!(counts, i).read()) } >> { drop_bits }) } - {
                        (1 << bitcount)
                    }) as u64);
                    WriteBits_120(_n_bits, _bits, (storage).clone())
                });
            }
            i.prefix_inc();
        }
    }
}
pub fn PopulationCost_131(mut data: Ptr<i32>, mut total_count: i32) -> f64 {
    if (total_count == 0) {
        return 7_f64;
    }
    let mut entropy_bits: f64 =
        ((total_count * BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) as f64);
    let mut histogram_bits: i32 = 0;
    let mut count: i32 = 0;
    let mut length: i32 = 0;
    if (total_count > BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow())) {
        let mut total: u64 = (total_count as u64);
        let mut i: i32 = 0;
        'loop_: while (i < 18) {
            if ((elem!(data, i).read()) > 0) {
                count.prefix_inc();
                length = i;
            }
            i.prefix_inc();
        }
        if (count == 1) {
            return 7_f64;
        }
        length.prefix_inc();
        let mut max0: u64 = (((total).wrapping_mul((length as u64)))
            >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64));
        let mut max1: u64 = (((max0).wrapping_mul((length as u64)))
            >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64));
        let mut min_base: u64 = ((((total).wrapping_add(max0)).wrapping_add(max1))
            >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64));
        total = { (total).wrapping_add((min_base).wrapping_mul((count as u64))) };
        let mut kFixBits: i64 = 32_i64;
        let mut kFixOne: i64 = (1_i64 << kFixBits);
        let mut kDescaleBits: i64 =
            (kFixBits - (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as i64));
        let mut kDescaleOne: i64 = (1_i64 << kDescaleBits);
        let mut kDescaleMask: i64 = (kDescaleOne - 1_i64);
        let mut mult: u32 = (((kFixOne as u64).wrapping_div(total)) as u32);
        let mut error: u32 = (((kFixOne as u64).wrapping_rem(total)) as u32);
        let mut cumul: u32 = error;
        if ((error as i64) < kDescaleOne) {
            cumul = { ((cumul as i64) + ((kDescaleOne - (error as i64)) >> 1)) as u32 };
        }
        if ((elem!(data, 0).read()) > 0) {
            let mut c: u64 = ((((elem!(data, 0).read()) as u64).wrapping_add(min_base))
                .wrapping_mul((mult as u64)))
            .wrapping_add((cumul as u64));
            let mut c_descaled: u64 = (c >> kDescaleBits);
            if !(c_descaled < (1_u64 << 31)) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                        236,
                        Ptr::<i8>::from_string_literal(b"PopulationCost"),
                    )
                });
                'loop_: while true {}
            };
            let mut log2count: f64 = ({ FastLog2_127((c_descaled as i32)) });
            entropy_bits -= ({ ((elem!(data, 0).read()) as f64) } * { log2count });
            cumul = ((c & (kDescaleMask as u64)) as u32);
        }
        let mut i: i32 = 1;
        'loop_: while (i < length) {
            if ((elem!(data, i).read()) > 0) {
                let mut c: u64 = ((((elem!(data, i).read()) as u64).wrapping_add(min_base))
                    .wrapping_mul((mult as u64)))
                .wrapping_add((cumul as u64));
                let mut c_descaled: u64 = (c >> kDescaleBits);
                if !(c_descaled < (1_u64 << 31)) {
                    ({
                        BrunsliDumpAndAbort_79(
                            Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                            245,
                            Ptr::<i8>::from_string_literal(b"PopulationCost"),
                        )
                    });
                    'loop_: while true {}
                };
                let mut log2count: f64 = ({ FastLog2_127((c_descaled as i32)) });
                let mut log2floor: i32 = (log2count as i32);
                entropy_bits -= ({ ((elem!(data, i).read()) as f64) } * { log2count });
                histogram_bits += log2floor;
                histogram_bits += (({
                    let __idx = (log2floor + 1) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
                cumul = ((c & (kDescaleMask as u64)) as u32);
            } else {
                histogram_bits += (({
                    let __idx = (0) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
            }
            i.prefix_inc();
        }
    } else {
        let mut log2norm: f64 = ((BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as f64)
            - ({ FastLog2_127(total_count) }));
        if ((elem!(data, 0).read()) > 0) {
            let mut log2count: f64 =
                ({ ({ FastLog2_127((elem!(data, 0).read())) }) } + { log2norm });
            entropy_bits -= ({ ((elem!(data, 0).read()) as f64) } * { log2count });
            length = 0;
            count.prefix_inc();
        }
        let mut i: i32 = 1;
        'loop_: while (i < 18) {
            if ((elem!(data, i).read()) > 0) {
                let mut log2count: f64 =
                    ({ ({ FastLog2_127((elem!(data, i).read())) }) } + { log2norm });
                let mut log2floor: i32 = (log2count as i32);
                entropy_bits -= ({ ((elem!(data, i).read()) as f64) } * { log2count });
                if (log2floor >= BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) {
                    log2floor = (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) - 1);
                }
                {
                    let rhs_0 = ((histogram_bits as u32)
                        .wrapping_add(({ GetPopulationCountPrecision_203((log2floor as u32)) })))
                        as i32;
                    histogram_bits = rhs_0
                };
                histogram_bits += (({
                    let __idx = (log2floor + 1) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
                length = i;
                count.prefix_inc();
            } else {
                histogram_bits += (({
                    let __idx = (0) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
            }
            i.prefix_inc();
        }
        length.prefix_inc();
    }
    if (count == 1) {
        return 7_f64;
    }
    if (count == 2) {
        return (((((entropy_bits as i32) + 1) + 12)
            + BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) as f64);
    }
    histogram_bits += (({
        let __idx = (length - 3) as usize;
        kHistogramLengthBitLengths_204.with(|rc| rc.borrow()[__idx])
    }) as i32);
    return (((histogram_bits + (entropy_bits as i32)) + 1) as f64);
}
thread_local!(
    pub static kCodeLengthCodes_211: Value<i32> = Rc::new(RefCell::new(18));
);
pub fn StoreHuffmanTreeOfHuffmanTreeToBitMask_212(
    mut num_codes: i32,
    mut code_length_bitdepth: Ptr<u8>,
    mut storage: Ptr<brunsli_Storage>,
) {
    thread_local!(
        static kStorageOrder_213: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            1_u8, 2_u8, 3_u8, 4_u8, 0_u8, 5_u8, 17_u8, 6_u8, 16_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
            12_u8, 13_u8, 14_u8, 15_u8,
        ])));
    );
    thread_local!(
        static kHuffmanBitLengthHuffmanCodeSymbols_214: Value<Box<[u8]>> =
            Rc::new(RefCell::new(Box::new([
                0_u8, 7_u8, 3_u8, 2_u8, 1_u8, 15_u8,
            ])));
    );
    thread_local!(
        static kHuffmanBitLengthHuffmanCodeBitLengths_215: Value<Box<[u8]>> =
            Rc::new(RefCell::new(Box::new([2_u8, 4_u8, 3_u8, 2_u8, 2_u8, 4_u8])));
    );
    let mut codes_to_store: usize = (kCodeLengthCodes_211.with(|rc| *rc.borrow()) as usize);
    if (num_codes > 1) {
        'loop_: while (codes_to_store > 0_usize) {
            if (((elem!(
                code_length_bitdepth,
                ({
                    let __idx = ((codes_to_store).wrapping_sub(1_usize)) as usize;
                    kStorageOrder_213.with(|rc| rc.borrow()[__idx])
                })
            )
            .read()) as i32)
                != 0)
            {
                break;
            }
            codes_to_store.prefix_dec();
        }
    }
    let mut skip_some: usize = 0_usize;
    if (((elem!(
        code_length_bitdepth,
        ({
            let __idx = (0) as usize;
            kStorageOrder_213.with(|rc| rc.borrow()[__idx])
        })
    )
    .read()) as i32)
        == 0)
        && (((elem!(
            code_length_bitdepth,
            ({
                let __idx = (1) as usize;
                kStorageOrder_213.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            == 0)
    {
        skip_some = 2_usize;
        if (((elem!(
            code_length_bitdepth,
            ({
                let __idx = (2) as usize;
                kStorageOrder_213.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            == 0)
        {
            skip_some = 3_usize;
        }
    }
    ({ WriteBits_120(2_usize, (skip_some as u64), (storage).clone()) });
    let mut i: usize = skip_some;
    'loop_: while (i < codes_to_store) {
        let mut l: usize = ((elem!(
            code_length_bitdepth,
            ({
                let __idx = (i) as usize;
                kStorageOrder_213.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as usize);
        ({
            let _n_bits: usize = (({
                let __idx = (l) as usize;
                kHuffmanBitLengthHuffmanCodeBitLengths_215.with(|rc| rc.borrow()[__idx])
            }) as usize);
            let _bits: u64 = (({
                let __idx = (l) as usize;
                kHuffmanBitLengthHuffmanCodeSymbols_214.with(|rc| rc.borrow()[__idx])
            }) as u64);
            WriteBits_120(_n_bits, _bits, (storage).clone())
        });
        i.prefix_inc();
    }
}
pub fn StoreHuffmanTreeToBitMask_216(
    mut huffman_tree_size: usize,
    mut huffman_tree: Ptr<u8>,
    mut huffman_tree_extra_bits: Ptr<u8>,
    mut code_length_bitdepth: Ptr<u8>,
    mut code_length_bitdepth_symbols: Ptr<u16>,
    mut storage: Ptr<brunsli_Storage>,
) {
    let mut i: usize = 0_usize;
    'loop_: while (i < huffman_tree_size) {
        let mut ix: usize = ((elem!(huffman_tree, i).read()) as usize);
        ({
            let _n_bits: usize = ((elem!(code_length_bitdepth, ix).read()) as usize);
            let _bits: u64 = ((elem!(code_length_bitdepth_symbols, ix).read()) as u64);
            WriteBits_120(_n_bits, _bits, (storage).clone())
        });
        'switch: {
            match { ix } {
                __v if __v == 16_usize => {
                    ({
                        WriteBits_120(
                            2_usize,
                            ((elem!(huffman_tree_extra_bits, i).read()) as u64),
                            (storage).clone(),
                        )
                    });
                    break 'switch;
                }
                __v if __v == 17_usize => {
                    ({
                        WriteBits_120(
                            3_usize,
                            ((elem!(huffman_tree_extra_bits, i).read()) as u64),
                            (storage).clone(),
                        )
                    });
                    break 'switch;
                }
                _ => {}
            }
        };
        i.prefix_inc();
    }
}
pub fn StoreSimpleHuffmanTree_217(
    mut depths: Ptr<u8>,
    mut symbols: Ptr<usize>,
    mut num_symbols: usize,
    mut max_bits: usize,
    mut storage: Ptr<brunsli_Storage>,
) {
    ({ WriteBits_120(2_usize, 1_u64, (storage).clone()) });
    ({
        WriteBits_120(
            2_usize,
            ((num_symbols).wrapping_sub(1_usize) as u64),
            (storage).clone(),
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while (i < num_symbols) {
        let mut j: usize = (i).wrapping_add(1_usize);
        'loop_: while (j < num_symbols) {
            if ({ ((elem!(depths, (elem!(symbols, j).read())).read()) as i32) } < {
                ((elem!(depths, (elem!(symbols, i).read())).read()) as i32)
            }) {
                {
                    let tmp = symbols.offset((j) as isize).read();
                    symbols
                        .offset((j) as isize)
                        .write(symbols.offset((i) as isize).read());
                    symbols.offset((i) as isize).write(tmp);
                };
            }
            j.postfix_inc();
        }
        i.postfix_inc();
    }
    if (num_symbols == 2_usize) {
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else if (num_symbols == 3_usize) {
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 2).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else {
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 2).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((elem!(symbols, 3).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _bits: u64 = (if (((elem!(depths, (elem!(symbols, 0).read())).read()) as i32) == 1)
            {
                1
            } else {
                0
            } as u64);
            let _storage: Ptr<brunsli_Storage> = (storage).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
    }
}
pub fn StoreHuffmanTree_218(
    mut depths: Ptr<u8>,
    mut num: usize,
    mut storage: Ptr<brunsli_Storage>,
) {
    let mut arena: Option<Value<Box<[u8]>>> = Ptr::alloc_array(
        (0..(2_usize).wrapping_mul(num))
            .map(|_| 0_u8)
            .collect::<Box<[u8]>>(),
    )
    .to_owned_opt();
    let mut huffman_tree: Ptr<u8> = arena.as_pointer();
    let mut huffman_tree_extra_bits: Ptr<u8> = arena.as_pointer().offset((num) as isize);
    let huffman_tree_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        WriteHuffmanTree_219(
            (depths).clone(),
            num,
            (huffman_tree_size.as_pointer()),
            (huffman_tree).clone(),
            (huffman_tree_extra_bits).clone(),
        )
    });
    let huffman_tree_histogram: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ])));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*huffman_tree_size.borrow())) {
        (*huffman_tree_histogram.borrow_mut())[(elem!(huffman_tree, i).read()) as usize]
            .prefix_inc();
        i.prefix_inc();
    }
    let mut num_codes: i32 = 0;
    let mut code: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < kCodeLengthCodes_211.with(|rc| *rc.borrow())) {
        if ((*huffman_tree_histogram.borrow())[(i) as usize] != 0) {
            if (num_codes == 0) {
                code = i;
                num_codes = 1;
            } else if (num_codes == 1) {
                num_codes = 2;
                break;
            }
        }
        i.prefix_inc();
    }
    let code_length_bitdepth: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8,
    ])));
    let code_length_bitdepth_symbols: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
    ])));
    ({
        CreateHuffmanTree_220(
            ((huffman_tree_histogram.as_pointer() as Ptr<u32>).offset(0)),
            (kCodeLengthCodes_211.with(|rc| *rc.borrow()) as usize),
            5,
            ((code_length_bitdepth.as_pointer() as Ptr<u8>).offset(0)),
        )
    });
    ({
        ConvertBitDepthsToSymbols_221(
            (code_length_bitdepth.as_pointer() as Ptr<u8>),
            (kCodeLengthCodes_211.with(|rc| *rc.borrow()) as usize),
            ((code_length_bitdepth_symbols.as_pointer() as Ptr<u16>).offset(0)),
        )
    });
    ({
        StoreHuffmanTreeOfHuffmanTreeToBitMask_212(
            num_codes,
            (code_length_bitdepth.as_pointer() as Ptr<u8>),
            (storage).clone(),
        )
    });
    if (num_codes == 1) {
        (*code_length_bitdepth.borrow_mut())[(code) as usize] = 0_u8;
    }
    ({
        StoreHuffmanTreeToBitMask_216(
            (*huffman_tree_size.borrow()),
            (huffman_tree).clone(),
            (huffman_tree_extra_bits).clone(),
            ((code_length_bitdepth.as_pointer() as Ptr<u8>).offset(0)),
            (code_length_bitdepth_symbols.as_pointer() as Ptr<u16>),
            (storage).clone(),
        )
    });
}
pub fn BuildAndStoreHuffmanTree_202(
    mut histogram: Ptr<u32>,
    mut length: usize,
    mut depth: Ptr<u8>,
    mut bits: Ptr<u16>,
    mut storage: Ptr<brunsli_Storage>,
) {
    let mut count: usize = 0_usize;
    let s4: Value<Box<[usize]>> =
        Rc::new(RefCell::new(Box::new([0_usize, 0_usize, 0_usize, 0_usize])));
    let mut i: usize = 0_usize;
    'loop_: while (i < length) {
        if ((elem!(histogram, i).read()) != 0) {
            if (count < 4_usize) {
                (*s4.borrow_mut())[(count) as usize] = i;
            } else if (count > 4_usize) {
                break;
            }
            count.postfix_inc();
        }
        i.postfix_inc();
    }
    let mut max_bits_counter: usize = (length).wrapping_sub(1_usize);
    let mut max_bits: usize = 0_usize;
    'loop_: while (max_bits_counter != 0) {
        max_bits_counter >>= 1;
        max_bits.prefix_inc();
    }
    if (count <= 1_usize) {
        ({ WriteBits_120(4_usize, 1_u64, (storage).clone()) });
        ({
            WriteBits_120(
                max_bits,
                ((*s4.borrow())[(0) as usize] as u64),
                (storage).clone(),
            )
        });
        return;
    }
    ({ CreateHuffmanTree_220((histogram).clone(), length, 15, (depth).clone()) });
    ({ ConvertBitDepthsToSymbols_221((depth).clone(), length, (bits).clone()) });
    if (count <= 4_usize) {
        ({
            StoreSimpleHuffmanTree_217(
                (depth).clone(),
                (s4.as_pointer() as Ptr<usize>),
                count,
                max_bits,
                (storage).clone(),
            )
        });
    } else {
        ({ StoreHuffmanTree_218((depth).clone(), length, (storage).clone()) });
    }
}
pub fn SetDepth_222(
    p: Ptr<brunsli_HuffmanTree>,
    mut pool: Ptr<brunsli_HuffmanTree>,
    mut depth: Ptr<u8>,
    mut level: u8,
) {
    if ((p.with(|__s| __s.index_left) as i32) >= 0) {
        level.prefix_inc();
        ({
            let _p: Ptr<brunsli_HuffmanTree> = pool.offset((p.with(|__s| __s.index_left)) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (pool).clone();
            let _depth: Ptr<u8> = (depth).clone();
            let _level: u8 = level;
            SetDepth_222(_p, _pool, _depth, _level)
        });
        ({
            let _p: Ptr<brunsli_HuffmanTree> =
                pool.offset((p.with(|__s| __s.index_right_or_value)) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (pool).clone();
            let _depth: Ptr<u8> = (depth).clone();
            let _level: u8 = level;
            SetDepth_222(_p, _pool, _depth, _level)
        });
    } else {
        elem!(depth, p.with(|__s| __s.index_right_or_value)).write({ level });
    }
}
pub fn Compare_223(v0: Ptr<brunsli_HuffmanTree>, v1: Ptr<brunsli_HuffmanTree>) -> bool {
    return ({ v0.with(|__s| __s.total_count) } < { v1.with(|__s| __s.total_count) });
}
pub fn CreateHuffmanTree_220(
    mut data: Ptr<u32>,
    mut length: usize,
    mut tree_limit: i32,
    mut depth: Ptr<u8>,
) {
    let mut count_limit: u32 = 1_u32;
    'loop_: while true {
        let tree: Value<Vec<brunsli_HuffmanTree>> = Rc::new(RefCell::new(Vec::new()));
        if ((2_usize).wrapping_mul(length)).wrapping_add(1_usize) as usize
            > (*tree.borrow()).capacity() as usize
        {
            let len_0 = (*tree.borrow()).len();
            (*tree.borrow_mut()).reserve_exact(
                ((2_usize).wrapping_mul(length)).wrapping_add(1_usize) as usize - len_0 as usize,
            );
        };
        let mut i: usize = length;
        'loop_: while (i != 0_usize) {
            i.prefix_dec();
            if ((elem!(data, i).read()) != 0) {
                let mut count: u32 = {
                    let __tmp_1: Value<u32> =
                        Rc::new(RefCell::new((count_limit).wrapping_sub(1_u32)));
                    (if data.offset((i) as isize).read() >= __tmp_1.as_pointer().read() {
                        data.offset((i) as isize)
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                };
                {
                    let __a1 =
                        brunsli_HuffmanTree::new({ count }, { (-1_i32 as i16) }, { (i as i16) });
                    (*tree.borrow_mut()).push(__a1)
                };
            };
        }
        let mut n: usize = (*tree.borrow()).len();
        if (n == 1_usize) {
            elem!(depth, { (*tree.borrow())[0_usize].index_right_or_value }).write(1_u8);
            break;
        }
        (tree.as_pointer() as Ptr<brunsli_HuffmanTree>).sort_with_cmp(
            (tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .to_end()
                .get_offset(),
            |x, y| Compare_223.call(x, y),
        );
        let mut sentinel: brunsli_HuffmanTree =
            brunsli_HuffmanTree::new({ <u32>::MAX }, { (-1_i32 as i16) }, { (-1_i32 as i16) });
        {
            let a0_clone = sentinel.clone();
            (*tree.borrow_mut()).push(a0_clone)
        };
        {
            let a0_clone = sentinel.clone();
            (*tree.borrow_mut()).push(a0_clone)
        };
        let mut i: usize = 0_usize;
        let mut j: usize = (n).wrapping_add(1_usize);
        let mut k: usize = (n).wrapping_sub(1_usize);
        'loop_: while (k != 0_usize) {
            let mut left: usize = 0_usize;
            let mut right: usize = 0_usize;
            if ({ (*tree.borrow())[i].total_count } <= { (*tree.borrow())[j].total_count }) {
                left = i;
                i.prefix_inc();
            } else {
                left = j;
                j.prefix_inc();
            }
            if ({ (*tree.borrow())[i].total_count } <= { (*tree.borrow())[j].total_count }) {
                right = i;
                i.prefix_inc();
            } else {
                right = j;
                j.prefix_inc();
            }
            let mut j_end: usize = ((*tree.borrow()).len()).wrapping_sub(1_usize);
            let __rhs = ({ (*tree.borrow())[left].total_count })
                .wrapping_add({ (*tree.borrow())[right].total_count });
            (*tree.borrow_mut())[j_end].total_count = __rhs;
            (*tree.borrow_mut())[j_end].index_left = (left as i16);
            (*tree.borrow_mut())[j_end].index_right_or_value = (right as i16);
            {
                let a0_clone = sentinel.clone();
                (*tree.borrow_mut()).push(a0_clone)
            };
            k.prefix_dec();
        }
        if !((*tree.borrow()).len() == ((2_usize).wrapping_mul(n)).wrapping_add(1_usize)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"huffman_tree.cc"),
                    121,
                    Ptr::<i8>::from_string_literal(b"CreateHuffmanTree"),
                )
            });
            'loop_: while true {}
        };
        ({
            let _p: Ptr<brunsli_HuffmanTree> = (tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset(((2_usize).wrapping_mul(n)).wrapping_sub(1_usize));
            let _pool: Ptr<brunsli_HuffmanTree> =
                ((tree.as_pointer() as Ptr<brunsli_HuffmanTree>).offset(0_usize));
            let _depth: Ptr<u8> = (depth).clone();
            SetDepth_222(_p, _pool, _depth, 0_u8)
        });
        if ({
            (({
                let count = (depth.offset((length) as isize)).get_offset()
                    - (depth.offset((0) as isize)).get_offset();
                let max_index = PtrValueIter::new(&(depth.offset((0) as isize)), count)
                    .enumerate()
                    .max_by(|(idx_a, val_a), (idx_b, val_b)| {
                        val_a
                            .partial_cmp(val_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| idx_b.cmp(idx_a))
                    })
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);
                (depth.offset((0) as isize)) + max_index
            }
            .read()) as i32)
        } <= { tree_limit })
        {
            break;
        }
        count_limit = { (count_limit).wrapping_mul(2_u32) };
    }
}
pub fn Reverse_224(mut v: Ptr<u8>, mut start: usize, mut end: usize) {
    end.prefix_dec();
    'loop_: while (start < end) {
        let mut tmp: u8 = (elem!(v, start).read());
        elem!(v, start).write({ (elem!(v, end).read()) });
        elem!(v, end).write({ tmp });
        start.prefix_inc();
        end.prefix_dec();
    }
}
pub fn WriteHuffmanTreeRepetitions_225(
    mut previous_value: u8,
    mut value: u8,
    mut repetitions: usize,
    mut tree_size: Ptr<usize>,
    mut tree: Ptr<u8>,
    mut extra_bits_data: Ptr<u8>,
) {
    if !(repetitions > 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"huffman_tree.cc"),
                151,
                Ptr::<i8>::from_string_literal(b"WriteHuffmanTreeRepetitions"),
            )
        });
        'loop_: while true {}
    };
    if ((previous_value as i32) != (value as i32)) {
        elem!(tree, (tree_size.read())).write({ value });
        elem!(extra_bits_data, (tree_size.read())).write(0_u8);
        tree_size.with_mut(|__v| __v.prefix_inc());
        repetitions.prefix_dec();
    }
    if (repetitions == 7_usize) {
        elem!(tree, (tree_size.read())).write({ value });
        elem!(extra_bits_data, (tree_size.read())).write(0_u8);
        tree_size.with_mut(|__v| __v.prefix_inc());
        repetitions.prefix_dec();
    }
    if (repetitions < 3_usize) {
        let mut i: usize = 0_usize;
        'loop_: while (i < repetitions) {
            elem!(tree, (tree_size.read())).write({ value });
            elem!(extra_bits_data, (tree_size.read())).write(0_u8);
            tree_size.with_mut(|__v| __v.prefix_inc());
            i.prefix_inc();
        }
    } else {
        repetitions = { (repetitions).wrapping_sub(3_usize) };
        let mut start: usize = (tree_size.read());
        'loop_: while true {
            elem!(tree, (tree_size.read())).write(16_u8);
            elem!(extra_bits_data, (tree_size.read())).write({ ((repetitions & 3_usize) as u8) });
            tree_size.with_mut(|__v| __v.prefix_inc());
            repetitions >>= 2;
            if (repetitions == 0_usize) {
                break;
            }
            repetitions.prefix_dec();
        }
        ({
            let _v: Ptr<u8> = (tree).clone();
            let _start: usize = start;
            let _end: usize = (tree_size.read());
            Reverse_224(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (extra_bits_data).clone();
            let _start: usize = start;
            let _end: usize = (tree_size.read());
            Reverse_224(_v, _start, _end)
        });
    }
}
pub fn WriteHuffmanTreeRepetitionsZeros_226(
    mut repetitions: usize,
    mut tree_size: Ptr<usize>,
    mut tree: Ptr<u8>,
    mut extra_bits_data: Ptr<u8>,
) {
    if (repetitions == 11_usize) {
        elem!(tree, (tree_size.read())).write(0_u8);
        elem!(extra_bits_data, (tree_size.read())).write(0_u8);
        tree_size.with_mut(|__v| __v.prefix_inc());
        repetitions.prefix_dec();
    }
    if (repetitions < 3_usize) {
        let mut i: usize = 0_usize;
        'loop_: while (i < repetitions) {
            elem!(tree, (tree_size.read())).write(0_u8);
            elem!(extra_bits_data, (tree_size.read())).write(0_u8);
            tree_size.with_mut(|__v| __v.prefix_inc());
            i.prefix_inc();
        }
    } else {
        repetitions = { (repetitions).wrapping_sub(3_usize) };
        let mut start: usize = (tree_size.read());
        'loop_: while true {
            elem!(tree, (tree_size.read())).write(17_u8);
            elem!(extra_bits_data, (tree_size.read())).write({ ((repetitions & 7_usize) as u8) });
            tree_size.with_mut(|__v| __v.prefix_inc());
            repetitions >>= 3;
            if (repetitions == 0_usize) {
                break;
            }
            repetitions.prefix_dec();
        }
        ({
            let _v: Ptr<u8> = (tree).clone();
            let _start: usize = start;
            let _end: usize = (tree_size.read());
            Reverse_224(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (extra_bits_data).clone();
            let _start: usize = start;
            let _end: usize = (tree_size.read());
            Reverse_224(_v, _start, _end)
        });
    }
}
pub fn DecideOverRleUse_227(
    mut depth: Ptr<u8>,
    mut length: usize,
    mut use_rle_for_non_zero: Ptr<bool>,
    mut use_rle_for_zero: Ptr<bool>,
) {
    let mut total_reps_zero: usize = 0_usize;
    let mut total_reps_non_zero: usize = 0_usize;
    let mut count_reps_zero: usize = 1_usize;
    let mut count_reps_non_zero: usize = 1_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < length) {
        let mut value: u8 = (elem!(depth, i).read());
        let mut reps: usize = 1_usize;
        let mut k: usize = (i).wrapping_add(1_usize);
        'loop_: while (k < length) && ({ ((elem!(depth, k).read()) as i32) } == { (value as i32) })
        {
            reps.prefix_inc();
            k.prefix_inc();
        }
        if (reps >= 3_usize) && ((value as i32) == 0) {
            total_reps_zero = { (total_reps_zero).wrapping_add(reps) };
            count_reps_zero.prefix_inc();
        }
        if (reps >= 4_usize) && ((value as i32) != 0) {
            total_reps_non_zero = { (total_reps_non_zero).wrapping_add(reps) };
            count_reps_non_zero.prefix_inc();
        }
        i = { (i).wrapping_add(reps) };
    }
    use_rle_for_non_zero
        .write({ (total_reps_non_zero > (count_reps_non_zero).wrapping_mul(2_usize)) });
    use_rle_for_zero.write({ (total_reps_zero > (count_reps_zero).wrapping_mul(2_usize)) });
}
pub fn WriteHuffmanTree_219(
    mut depth: Ptr<u8>,
    mut length: usize,
    mut tree_size: Ptr<usize>,
    mut tree: Ptr<u8>,
    mut extra_bits_data: Ptr<u8>,
) {
    let mut previous_value: u8 = 8_u8;
    let mut new_length: usize = length;
    let mut i: usize = 0_usize;
    'loop_: while (i < length) {
        if (((elem!(depth, ((length).wrapping_sub(i)).wrapping_sub(1_usize)).read()) as i32) == 0) {
            new_length.prefix_dec();
        } else {
            break;
        }
        i.prefix_inc();
    }
    let use_rle_for_non_zero: Value<bool> = Rc::new(RefCell::new(false));
    let use_rle_for_zero: Value<bool> = Rc::new(RefCell::new(false));
    if (length > 50_usize) {
        ({
            DecideOverRleUse_227(
                (depth).clone(),
                new_length,
                (use_rle_for_non_zero.as_pointer()),
                (use_rle_for_zero.as_pointer()),
            )
        });
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < new_length) {
        let mut value: u8 = (elem!(depth, i).read());
        let mut reps: usize = 1_usize;
        if (((value as i32) != 0) && (*use_rle_for_non_zero.borrow()))
            || (((value as i32) == 0) && (*use_rle_for_zero.borrow()))
        {
            let mut k: usize = (i).wrapping_add(1_usize);
            'loop_: while (k < new_length)
                && ({ ((elem!(depth, k).read()) as i32) } == { (value as i32) })
            {
                reps.prefix_inc();
                k.prefix_inc();
            }
        }
        if ((value as i32) == 0) {
            ({
                let _repetitions: usize = reps;
                let _tree_size: Ptr<usize> = (tree_size).clone();
                WriteHuffmanTreeRepetitionsZeros_226(
                    _repetitions,
                    _tree_size,
                    (tree).clone(),
                    (extra_bits_data).clone(),
                )
            });
        } else {
            ({
                let _previous_value: u8 = previous_value;
                let _value: u8 = value;
                let _repetitions: usize = reps;
                let _tree_size: Ptr<usize> = (tree_size).clone();
                let _tree: Ptr<u8> = (tree).clone();
                let _extra_bits_data: Ptr<u8> = (extra_bits_data).clone();
                WriteHuffmanTreeRepetitions_225(
                    _previous_value,
                    _value,
                    _repetitions,
                    _tree_size,
                    _tree,
                    _extra_bits_data,
                )
            });
            previous_value = value;
        }
        i = { (i).wrapping_add(reps) };
    }
}
pub fn ReverseBits_228(mut num_bits: i32, mut bits: u16) -> u16 {
    thread_local!(
        static kLut_229: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
            0_usize, 8_usize, 4_usize, 12_usize, 2_usize, 10_usize, 6_usize, 14_usize, 1_usize,
            9_usize, 5_usize, 13_usize, 3_usize, 11_usize, 7_usize, 15_usize,
        ])));
    );
    let mut retval: usize = ({
        let __idx = ((bits as i32) & 15) as usize;
        kLut_229.with(|rc| rc.borrow()[__idx])
    });
    let mut i: i32 = 4;
    'loop_: while (i < num_bits) {
        retval <<= 4;
        bits = { (((bits as i32) >> 4) as u16) };
        retval |= ({
            let __idx = ((bits as i32) & 15) as usize;
            kLut_229.with(|rc| rc.borrow()[__idx])
        });
        i += 4;
    }
    retval >>= (-num_bits & 3);
    return (retval as u16);
}
pub fn ConvertBitDepthsToSymbols_221(mut depth: Ptr<u8>, mut len: usize, mut bits: Ptr<u16>) {
    let kMaxBits: Value<i32> = Rc::new(RefCell::new(16));
    let mut bl_count: [u16; 16] = [
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ];
    {
        let mut i: usize = 0_usize;
        'loop_: while (i < len) {
            bl_count[(elem!(depth, i).read()) as usize].prefix_inc();
            i.prefix_inc();
        }
        bl_count[(0) as usize] = 0_u16;
    }
    let mut next_code: [u16; 16] = [0_u16; 16];
    next_code[(0) as usize] = 0_u16;
    {
        let mut code: i32 = 0;
        let mut i: usize = 1_usize;
        'loop_: while (i < ((*kMaxBits.borrow()) as usize)) {
            code = { ((code + (bl_count[((i).wrapping_sub(1_usize)) as usize] as i32)) << 1) };
            next_code[(i) as usize] = (code as u16);
            i.prefix_inc();
        }
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        if ((elem!(depth, i).read()) != 0) {
            let __rhs = ({
                let _num_bits: i32 = ((elem!(depth, i).read()) as i32);
                let _bits: u16 = next_code[(elem!(depth, i).read()) as usize].postfix_inc();
                ReverseBits_228(_num_bits, _bits)
            });
            elem!(bits, i).write(__rhs);
        }
        i.prefix_inc();
    }
}
thread_local!(
    pub static kJpegHuffmanRootTableBits_230: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kJpegHuffmanLutSize_231: Value<i32> = Rc::new(RefCell::new(1024));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct brunsli_HuffmanTableEntry {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
impl brunsli_HuffmanTableEntry {
    pub fn new() -> Self {
        Self {
            bits: 0_u8,
            value: 65535_u16,
        }
    }
}
impl Default for brunsli_HuffmanTableEntry {
    fn default() -> Self {
        { brunsli_HuffmanTableEntry::new() }
    }
}
pub fn DivCeil_232(mut a: i32, mut b: i32) -> i32 {
    return (((a + b) - 1) / b);
}
pub fn ReadUint8_233(mut data: Ptr<u8>, mut pos: Ptr<usize>) -> i32 {
    return ((elem!(data, pos.with_mut(|__v| __v.postfix_inc())).read()) as i32);
}
pub fn ReadUint16_234(mut data: Ptr<u8>, mut pos: Ptr<usize>) -> i32 {
    let mut v: i32 = ((((elem!(data, (pos.read())).read()) as i32) << 8)
        + ((elem!(data, (pos.read()).wrapping_add(1_usize)).read()) as i32));
    pos.write({ (pos.read()).wrapping_add(2_usize) });
    return v;
}
pub fn ProcessSOF_235(
    mut data: Ptr<u8>,
    mut len: usize,
    mut mode: brunsli_JpegReadMode,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    if (jpg.with(|__s| __s.width) != 0) {
        write!(libcc2rs::cerr(), "Duplicate SOF marker.\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_DUPLICATE_SOF);
        return false;
    }
    let mut start_pos: usize = (pos.read());
    if ({ (pos.read()).wrapping_add(((8) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (8),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    let mut precision: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    let mut height: i32 = ({ ReadUint16_234((data).clone(), (pos).clone()) });
    let mut width: i32 = ({ ReadUint16_234((data).clone(), (pos).clone()) });
    let mut num_components: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    if (precision < 8) || (precision > 8) {
        write!(libcc2rs::cerr(), "Invalid precision: {:}\n", precision,);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_PRECISION);
        return false;
    };
    if (height < 1) || (height > kMaxDimPixels_11.with(|rc| *rc.borrow())) {
        write!(libcc2rs::cerr(), "Invalid height: {:}\n", height,);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HEIGHT);
        return false;
    };
    if (width < 1) || (width > kMaxDimPixels_11.with(|rc| *rc.borrow())) {
        write!(libcc2rs::cerr(), "Invalid width: {:}\n", width,);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_WIDTH);
        return false;
    };
    if (num_components < 1) || (num_components > kMaxComponents_4.with(|rc| *rc.borrow())) {
        write!(
            libcc2rs::cerr(),
            "Invalid num_components: {:}\n",
            num_components,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_NUMCOMP);
        return false;
    };
    if ({ (pos.read()).wrapping_add(((3 * num_components) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (3 * num_components),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    field!(jpg, height).write(height);
    field!(jpg, width).write(width);
    {
        let __a0 = (num_components as usize) as usize;
        (*jpg.with(|__s| __s.components.clone()).borrow_mut())
            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
    };
    let ids_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        let mut id: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        if ((*ids_seen.borrow())[(id as usize)] as bool) {
            write!(libcc2rs::cerr(), "Duplicate ID {:} in SOF.\n", id,);
            field!(jpg, error).write(brunsli_JPEGReadError_DUPLICATE_COMPONENT_ID);
            return false;
        }
        (*ids_seen.borrow_mut())[(id as usize)] = true;
        field!(
            elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            ),
            id
        )
        .write(id);
        let mut factor: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        let h_samp_factor: Value<i32> = Rc::new(RefCell::new((factor >> 4)));
        let v_samp_factor: Value<i32> = Rc::new(RefCell::new((factor & 15)));
        if ((*h_samp_factor.borrow()) < 1)
            || ((*h_samp_factor.borrow()) > kBrunsliMaxSampling_27.with(|rc| *rc.borrow()))
        {
            write!(
                libcc2rs::cerr(),
                "Invalid h_samp_factor: {:}\n",
                (*h_samp_factor.borrow()),
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SAMP_FACTOR);
            return false;
        };
        if ((*v_samp_factor.borrow()) < 1)
            || ((*v_samp_factor.borrow()) > kBrunsliMaxSampling_27.with(|rc| *rc.borrow()))
        {
            write!(
                libcc2rs::cerr(),
                "Invalid v_samp_factor: {:}\n",
                (*v_samp_factor.borrow()),
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SAMP_FACTOR);
            return false;
        };
        field!(
            elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            ),
            h_samp_factor
        )
        .write((*h_samp_factor.borrow()));
        field!(
            elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            ),
            v_samp_factor
        )
        .write((*v_samp_factor.borrow()));
        let __rhs = (({ ReadUint8_233((data).clone(), (pos).clone()) }) as u8);
        field!(
            elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            ),
            quant_idx
        )
        .write(__rhs);
        let __rhs =
            (if field_ptr!(jpg, max_h_samp_factor).read() >= h_samp_factor.as_pointer().read() {
                field_ptr!(jpg, max_h_samp_factor)
            } else {
                h_samp_factor.as_pointer()
            }
            .read());
        field!(jpg, max_h_samp_factor).write(__rhs);
        let __rhs =
            (if field_ptr!(jpg, max_v_samp_factor).read() >= v_samp_factor.as_pointer().read() {
                field_ptr!(jpg, max_v_samp_factor)
            } else {
                v_samp_factor.as_pointer()
            }
            .read());
        field!(jpg, max_v_samp_factor).write(__rhs);
        i.prefix_inc();
    }
    let __rhs = ({
        let _a: i32 = jpg.with(|__s| __s.height);
        let _b: i32 = (jpg.with(|__s| __s.max_v_samp_factor) * 8);
        DivCeil_232(_a, _b)
    });
    field!(jpg, MCU_rows).write(__rhs);
    let __rhs = ({
        let _a: i32 = jpg.with(|__s| __s.width);
        let _b: i32 = (jpg.with(|__s| __s.max_h_samp_factor) * 8);
        DivCeil_232(_a, _b)
    });
    field!(jpg, MCU_cols).write(__rhs);
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        let mut c: Ptr<brunsli_JPEGComponent> =
            ((jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i));
        if (({ jpg.with(|__s| __s.max_h_samp_factor) } % { c.with(|__s| __s.h_samp_factor) }) != 0)
            || (({ jpg.with(|__s| __s.max_v_samp_factor) } % { c.with(|__s| __s.v_samp_factor) })
                != 0)
        {
            write!(libcc2rs::cerr(), "Non-integral subsampling ratios.\n",);
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SAMPLING_FACTORS);
            return false;
        }
        field!(c, width_in_blocks).write({
            (({ jpg.with(|__s| __s.MCU_cols) } * { c.with(|__s| __s.h_samp_factor) }) as u32)
        });
        field!(c, height_in_blocks).write({
            (({ jpg.with(|__s| __s.MCU_rows) } * { c.with(|__s| __s.v_samp_factor) }) as u32)
        });
        let mut num_blocks: u64 = (c.with(|__s| __s.width_in_blocks) as u64)
            .wrapping_mul((c.with(|__s| __s.height_in_blocks) as u64));
        if ((num_blocks as usize) > kBrunsliMaxNumBlocks_18.with(|rc| *rc.borrow())) {
            write!(libcc2rs::cerr(), "Image too large.\n",);
            field!(jpg, error).write(brunsli_JPEGReadError_IMAGE_TOO_LARGE);
            return false;
        }
        field!(c, num_blocks).write(((num_blocks as i32) as u32));
        if ((mode as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
            {
                let __a0 = (((c.with(|__s| __s.num_blocks))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                    as usize) as usize;
                (*c.with(|__s| __s.coeffs.clone()).borrow_mut())
                    .resize_with(__a0, || <i16>::default())
            };
        }
        i.prefix_inc();
    }
    if ({ (start_pos).wrapping_add(marker_len) } != { (pos.read()) }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((pos.read()).wrapping_sub(start_pos)),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessSOS_236(
    mut data: Ptr<u8>,
    mut len: usize,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let mut start_pos: usize = (pos.read());
    if ({ (pos.read()).wrapping_add(((3) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (3),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    let mut comps_in_scan: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    if ((comps_in_scan as usize) < 1_usize)
        || ({ (comps_in_scan as usize) } > {
            (*jpg.with(|__s| __s.components.clone()).borrow()).len()
        })
    {
        write!(
            libcc2rs::cerr(),
            "Invalid static_cast<size_t>(comps_in_scan): {:}\n",
            (comps_in_scan as usize),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_COMPS_IN_SCAN);
        return false;
    };
    let scan_info: Value<brunsli_JPEGScanInfo> =
        Rc::new(RefCell::new(<brunsli_JPEGScanInfo>::default()));
    (*scan_info.borrow_mut()).num_components = (comps_in_scan as usize);
    if ({ (pos.read()).wrapping_add(((2 * comps_in_scan) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (2 * comps_in_scan),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let ids_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < comps_in_scan) {
        let mut id: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        if ((*ids_seen.borrow())[(id as usize)] as bool) {
            write!(libcc2rs::cerr(), "Duplicate ID {:} in SOS.\n", id,);
            field!(jpg, error).write(brunsli_JPEGReadError_DUPLICATE_COMPONENT_ID);
            return false;
        }
        (*ids_seen.borrow_mut())[(id as usize)] = true;
        let mut found_index: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ({ j } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
            if ({
                {
                    (*elem!(
                        (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                        j
                    )
                    .upgrade()
                    .deref())
                    .id
                }
            } == { id })
            {
                field!(
                    elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        (i as usize)
                    ),
                    comp_idx
                )
                .write((j as u8));
                found_index = true;
            }
            j.prefix_inc();
        }
        if !(found_index) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find component with id {:}\n",
                id,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_COMPONENT_NOT_FOUND);
            return false;
        }
        let mut c: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        let mut dc_tbl_idx: i32 = (c >> 4);
        let mut ac_tbl_idx: i32 = (c & 15);
        if (dc_tbl_idx < 0) || (dc_tbl_idx > 3) {
            write!(libcc2rs::cerr(), "Invalid dc_tbl_idx: {:}\n", dc_tbl_idx,);
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
            return false;
        };
        if (ac_tbl_idx < 0) || (ac_tbl_idx > 3) {
            write!(libcc2rs::cerr(), "Invalid ac_tbl_idx: {:}\n", ac_tbl_idx,);
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
            return false;
        };
        field!(
            elem!(
                ({ (*scan_info.borrow()).components.as_pointer() }
                    as Ptr<brunsli_JPEGComponentScanInfo>),
                (i as usize)
            ),
            dc_tbl_idx
        )
        .write(dc_tbl_idx);
        field!(
            elem!(
                ({ (*scan_info.borrow()).components.as_pointer() }
                    as Ptr<brunsli_JPEGComponentScanInfo>),
                (i as usize)
            ),
            ac_tbl_idx
        )
        .write(ac_tbl_idx);
        i.prefix_inc();
    }
    if ({ (pos.read()).wrapping_add(((3) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (3),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    (*scan_info.borrow_mut()).Ss = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    (*scan_info.borrow_mut()).Se = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    if ({ (*scan_info.borrow()).Ss } < 0) || ({ (*scan_info.borrow()).Ss } > 63) {
        write!(libcc2rs::cerr(), "Invalid scan_info.Ss: {:}\n", {
            (*scan_info.borrow()).Ss
        },);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_START_OF_SCAN);
        return false;
    };
    if ({ (*scan_info.borrow()).Se } < { (*scan_info.borrow()).Ss })
        || ({ (*scan_info.borrow()).Se } > 63)
    {
        write!(libcc2rs::cerr(), "Invalid scan_info.Se: {:}\n", {
            (*scan_info.borrow()).Se
        },);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_END_OF_SCAN);
        return false;
    };
    let mut c: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
    (*scan_info.borrow_mut()).Ah = (c >> 4);
    (*scan_info.borrow_mut()).Al = (c & 15);
    if ({ (*scan_info.borrow()).Ah } != 0)
        && ({ (*scan_info.borrow()).Al } != ({ (*scan_info.borrow()).Ah } - 1))
    {
        write!(
            libcc2rs::cerr(),
            "Invalid progressive parameters:  Al = {:} Ah = {:}\n",
            { (*scan_info.borrow()).Al },
            { (*scan_info.borrow()).Ah },
        );
    }
    let mut i: i32 = 0;
    'loop_: while (i < comps_in_scan) {
        let mut found_dc_table: bool = false;
        let mut found_ac_table: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ({ j } < { (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len() }) {
            let mut slot_id: i32 = {
                (*elem!(
                    (jpg.with(|__s| __s.huffman_code.as_pointer()) as Ptr<brunsli_JPEGHuffmanCode>),
                    j
                )
                .upgrade()
                .deref())
                .slot_id
            };
            if (slot_id == {
                (*elem!(
                    ({ (*scan_info.borrow()).components.as_pointer() }
                        as Ptr<brunsli_JPEGComponentScanInfo>),
                    (i as usize)
                )
                .upgrade()
                .deref())
                .dc_tbl_idx
            }) {
                found_dc_table = true;
            } else if (slot_id
                == ({
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        (i as usize)
                    )
                    .upgrade()
                    .deref())
                    .ac_tbl_idx
                } + 16))
            {
                found_ac_table = true;
            }
            j.prefix_inc();
        }
        if ({ (*scan_info.borrow()).Ss } == 0) && (!(found_dc_table)) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find DC Huffman table with index {:}\n",
                {
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        (i as usize)
                    )
                    .upgrade()
                    .deref())
                    .dc_tbl_idx
                },
            );
            field!(jpg, error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_NOT_FOUND);
            return false;
        }
        if ({ (*scan_info.borrow()).Se } > 0) && (!(found_ac_table)) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find AC Huffman table with index {:}\n",
                {
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        (i as usize)
                    )
                    .upgrade()
                    .deref())
                    .ac_tbl_idx
                },
            );
            field!(jpg, error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_NOT_FOUND);
            return false;
        }
        i.prefix_inc();
    }
    {
        let a0_clone = (*scan_info.borrow()).clone();
        (*jpg.with(|__s| __s.scan_info.clone()).borrow_mut()).push(a0_clone)
    };
    if ({ (start_pos).wrapping_add(marker_len) } != { (pos.read()) }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((pos.read()).wrapping_sub(start_pos)),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDHT_237(
    mut data: Ptr<u8>,
    mut len: usize,
    mut mode: brunsli_JpegReadMode,
    mut dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    mut ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let mut start_pos: usize = (pos.read());
    if ({ (pos.read()).wrapping_add(((2) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (2),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    if (marker_len == 2_usize) {
        write!(libcc2rs::cerr(), "DHT marker: no Huffman table found\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_EMPTY_DHT);
        return false;
    }
    'loop_: while ({ (pos.read()) } < { (start_pos).wrapping_add(marker_len) }) {
        if ({
            (pos.read())
                .wrapping_add(((1 + kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) as usize))
        } > { len })
        {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (pos.read()),
                (1 + kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())),
                len,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let huff: Value<brunsli_JPEGHuffmanCode> =
            Rc::new(RefCell::new(<brunsli_JPEGHuffmanCode>::default()));
        (*huff.borrow_mut()).slot_id = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        let mut huffman_index: i32 = { (*huff.borrow()).slot_id };
        let mut is_ac_table: i32 = ((({ (*huff.borrow()).slot_id } & 16) != 0) as i32);
        let mut huff_lut: Ptr<brunsli_HuffmanTableEntry> = Ptr::<brunsli_HuffmanTableEntry>::null();
        if (is_ac_table != 0) {
            huffman_index -= 16;
            if (huffman_index < 0) || (huffman_index > 3) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid huffman_index: {:}\n",
                    huffman_index,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
                return false;
            };
            huff_lut = (((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(ac_huff_lut)))
                as Ptr<brunsli_HuffmanTableEntry>)
                .offset(
                    ((huffman_index * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())) as usize),
                ));
        } else {
            if (huffman_index < 0) || (huffman_index > 3) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid huffman_index: {:}\n",
                    huffman_index,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
                return false;
            };
            huff_lut = (((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(dc_huff_lut)))
                as Ptr<brunsli_HuffmanTableEntry>)
                .offset(
                    ((huffman_index * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())) as usize),
                ));
        }
        elem!(
            ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
            0_usize
        )
        .write(0);
        let mut total_count: i32 = 0;
        let mut space: i32 = (1 << kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()));
        let mut max_depth: i32 = 1;
        let mut i: i32 = 1;
        'loop_: while (i <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
            let mut count: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
            if (count != 0) {
                max_depth = i;
            }
            elem!(
                ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
                (i as usize)
            )
            .write(count);
            total_count += count;
            space -= (count * (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - i)));
            i.prefix_inc();
        }
        if (is_ac_table != 0) {
            if (total_count < 0)
                || (total_count > kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()))
            {
                write!(libcc2rs::cerr(), "Invalid total_count: {:}\n", total_count,);
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            };
        } else {
            if (total_count < 0) || (total_count > kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())) {
                write!(libcc2rs::cerr(), "Invalid total_count: {:}\n", total_count,);
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            };
        }
        if ({ (pos.read()).wrapping_add(((total_count) as usize)) } > { len }) {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (pos.read()),
                (total_count),
                len,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let values_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(256_usize) as usize)
                .map(|_| false)
                .collect::<Vec<bool>>(),
        ));
        let mut i: i32 = 0;
        'loop_: while (i < total_count) {
            let mut value: u8 = (({ ReadUint8_233((data).clone(), (pos).clone()) }) as u8);
            if !(is_ac_table != 0) {
                if ((value as i32) < 0)
                    || ((value as i32) > (kJpegDCAlphabetSize_9.with(|rc| *rc.borrow()) - 1))
                {
                    write!(libcc2rs::cerr(), "Invalid value: ",);
                    libcc2rs::cerr()
                        .write_all(&([(&[value as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()));
                    field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                    return false;
                };
            }
            if ((*values_seen.borrow())[(value as usize)] as bool) {
                write!(libcc2rs::cerr(), "Duplicate Huffman code value ",);
                libcc2rs::cerr()
                    .write_all(&([(&[value as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()));
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            }
            (*values_seen.borrow_mut())[(value as usize)] = true;
            elem!(
                ({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>),
                (i as usize)
            )
            .write((value as i32));
            i.prefix_inc();
        }
        elem!(
            ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
            (max_depth as usize)
        )
        .with_mut(|__v| __v.prefix_inc());
        elem!(
            ({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>),
            (total_count as usize)
        )
        .write(kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()));
        space -= (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - max_depth));
        if (space < 0) {
            write!(libcc2rs::cerr(), "Invalid Huffman code lengths.\n",);
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
            return false;
        } else if (space > 0)
            && (({ (*elem!(huff_lut, 0).upgrade().deref()).value } as i32) != 65535)
        {
            let mut i: i32 = 0;
            'loop_: while (i < kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())) {
                field!(elem!(huff_lut, i), bits).write(0_u8);
                field!(elem!(huff_lut, i), value).write(65535_u16);
                i.prefix_inc();
            }
        }
        (*huff.borrow_mut()).is_last =
            ({ (pos.read()) } == { (start_pos).wrapping_add(marker_len) });
        if ((mode as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
            ({
                let _counts: Ptr<i32> =
                    (({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>).offset(0_usize));
                let _symbols: Ptr<i32> =
                    (({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>).offset(0_usize));
                BuildJpegHuffmanTable_238(_counts, _symbols, (huff_lut).clone())
            });
        }
        {
            let a0_clone = (*huff.borrow()).clone();
            (*jpg.with(|__s| __s.huffman_code.clone()).borrow_mut()).push(a0_clone)
        };
    }
    if ({ (start_pos).wrapping_add(marker_len) } != { (pos.read()) }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((pos.read()).wrapping_sub(start_pos)),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDQT_239(
    mut data: Ptr<u8>,
    mut len: usize,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let mut start_pos: usize = (pos.read());
    if ({ (pos.read()).wrapping_add(((2) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (2),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    if (marker_len == 2_usize) {
        write!(
            libcc2rs::cerr(),
            "DQT marker: no quantization table found\n",
        );
        field!(jpg, error).write(brunsli_JPEGReadError_EMPTY_DQT);
        return false;
    }
    'loop_: while ({ (pos.read()) } < { (start_pos).wrapping_add(marker_len) })
        && ({ (*jpg.with(|__s| __s.quant.clone()).borrow()).len() } < {
            (kMaxQuantTables_5.with(|rc| *rc.borrow()) as usize)
        })
    {
        if ({ (pos.read()).wrapping_add(((1) as usize)) } > { len }) {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (pos.read()),
                (1),
                len,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let mut quant_table_index: i32 = ({ ReadUint8_233((data).clone(), (pos).clone()) });
        let mut quant_table_precision: i32 = (quant_table_index >> 4);
        if (quant_table_precision < 0) || (quant_table_precision > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_precision: {:}\n",
                quant_table_precision,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_QUANT_TBL_PRECISION);
            return false;
        };
        quant_table_index &= 15;
        if (quant_table_index < 0) || (quant_table_index > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_index: {:}\n",
                quant_table_index,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_QUANT_TBL_INDEX);
            return false;
        };
        if ({
            (pos.read()).wrapping_add(
                (((quant_table_precision + 1) * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize),
            )
        } > { len })
        {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (pos.read()),
                ((quant_table_precision + 1) * kDCTBlockSize_3.with(|rc| *rc.borrow())),
                len,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let table: Value<brunsli_JPEGQuantTable> =
            Rc::new(RefCell::new(<brunsli_JPEGQuantTable>::default()));
        (*table.borrow_mut()).index = quant_table_index;
        (*table.borrow_mut()).precision = quant_table_precision;
        let mut i: i32 = 0;
        'loop_: while (i < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let mut quant_val: i32 = if (quant_table_precision != 0) {
                ({ ReadUint16_234((data).clone(), (pos).clone()) })
            } else {
                ({ ReadUint8_233((data).clone(), (pos).clone()) })
            };
            if (quant_val < 1) || (quant_val > 65535) {
                write!(libcc2rs::cerr(), "Invalid quant_val: {:}\n", quant_val,);
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_QUANT_VAL);
                return false;
            };
            elem!(
                ({ (*table.borrow()).values.as_pointer() } as Ptr<i32>),
                (({
                    let __idx = (i) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as usize)
            )
            .write(quant_val);
            i.prefix_inc();
        }
        (*table.borrow_mut()).is_last =
            ({ (pos.read()) } == { (start_pos).wrapping_add(marker_len) });
        {
            let a0_clone = (*table.borrow()).clone();
            (*jpg.with(|__s| __s.quant.clone()).borrow_mut()).push(a0_clone)
        };
    }
    if ({ (start_pos).wrapping_add(marker_len) } != { (pos.read()) }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((pos.read()).wrapping_sub(start_pos)),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDRI_240(
    mut data: Ptr<u8>,
    mut len: usize,
    mut pos: Ptr<usize>,
    mut found_dri: Ptr<bool>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    if (found_dri.read()) {
        write!(libcc2rs::cerr(), "Duplicate DRI marker.\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_DUPLICATE_DRI);
        return false;
    }
    found_dri.write(true);
    let mut start_pos: usize = (pos.read());
    if ({ (pos.read()).wrapping_add(((4) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (4),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    let mut restart_interval: i32 = ({ ReadUint16_234((data).clone(), (pos).clone()) });
    field!(jpg, restart_interval).write(restart_interval);
    if ({ (start_pos).wrapping_add(marker_len) } != { (pos.read()) }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((pos.read()).wrapping_sub(start_pos)),
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessAPP_241(
    mut data: Ptr<u8>,
    mut len: usize,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    if ({ (pos.read()).wrapping_add(((2) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (2),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    if (marker_len < 2_usize) || (marker_len > 65535_usize) {
        write!(libcc2rs::cerr(), "Invalid marker_len: {:}\n", marker_len,);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_MARKER_LEN);
        return false;
    };
    if ({ (pos.read()).wrapping_add(((marker_len).wrapping_sub(2_usize))) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            ((marker_len).wrapping_sub(2_usize)),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut app_str_start: Ptr<u8> = data.offset((pos.read()) as isize).offset(-((3) as isize));
    let mut app_str: Vec<u8> = {
        let __count = app_str_start
            .offset((marker_len) as isize)
            .offset((1) as isize)
            .get_offset()
            - app_str_start.get_offset();
        PtrValueIter::new(&app_str_start, __count).collect::<Vec<_>>()
    };
    pos.write({ (pos.read()).wrapping_add((marker_len).wrapping_sub(2_usize)) });
    (jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new((app_str).clone()))),
    );
    return true;
}
pub fn ProcessCOM_242(
    mut data: Ptr<u8>,
    mut len: usize,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    if ({ (pos.read()).wrapping_add(((2) as usize)) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            (2),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut marker_len: usize = (({ ReadUint16_234((data).clone(), (pos).clone()) }) as usize);
    if (marker_len < 2_usize) || (marker_len > 65535_usize) {
        write!(libcc2rs::cerr(), "Invalid marker_len: {:}\n", marker_len,);
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_MARKER_LEN);
        return false;
    };
    if ({ (pos.read()).wrapping_add(((marker_len).wrapping_sub(2_usize))) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (pos.read()),
            ((marker_len).wrapping_sub(2_usize)),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let mut com_str_start: Ptr<u8> = data.offset((pos.read()) as isize).offset(-((3) as isize));
    let mut com_str: Vec<u8> = {
        let __count = com_str_start
            .offset((marker_len) as isize)
            .offset((1) as isize)
            .get_offset()
            - com_str_start.get_offset();
        PtrValueIter::new(&com_str_start, __count).collect::<Vec<_>>()
    };
    pos.write({ (pos.read()).wrapping_add((marker_len).wrapping_sub(2_usize)) });
    (jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new((com_str).clone()))),
    );
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct brunsli_BitReaderState {
    #[offset(0)]
    #[byte_size(8)]
    pub data_: Ptr<u8>,
    #[offset(8)]
    pub len_: usize,
    #[offset(16)]
    pub pos_: usize,
    #[offset(24)]
    pub val_: u64,
    #[offset(32)]
    pub bits_left_: i32,
    #[offset(40)]
    pub next_marker_pos_: usize,
}
impl brunsli_BitReaderState {
    pub fn new(mut data: Ptr<u8>, mut len: usize, mut pos: usize) -> Self {
        let __this: Value<brunsli_BitReaderState> = Rc::new(RefCell::new(Self {
            data_: (data).clone(),
            len_: len,
            pos_: 0_usize,
            val_: 0_u64,
            bits_left_: 0_i32,
            next_marker_pos_: 0_usize,
        }));
        let this: Ptr<brunsli_BitReaderState> = __this.as_pointer();
        ({ brunsli_BitReaderStateImpl::Reset(&this, pos) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn ReadSymbol_243(
    mut table: Ptr<brunsli_HuffmanTableEntry>,
    mut br: Ptr<brunsli_BitReaderState>,
) -> i32 {
    let mut nbits: i32 = 0_i32;
    ({ brunsli_BitReaderStateImpl::FillBitWindow(&br) });
    let mut val: i32 = ((({ br.with(|__s| __s.val_) } >> { (br.with(|__s| __s.bits_left_) - 8) })
        & 255_u64) as i32);
    table += val;
    nbits = ((table.with(|__s| __s.bits) as i32) - 8);
    if (nbits > 0) {
        {
            field!(br, bits_left_).with_mut(|__v| *__v = *__v - 8)
        };
        let __rhs = (table.with(|__s| __s.value) as i32);
        table += __rhs;
        val = (({
            ({ br.with(|__s| __s.val_) } >> { ({ br.with(|__s| __s.bits_left_) } - { nbits }) })
        } & { (((1 << nbits) - 1) as u64) }) as i32);
        table += val;
    }
    {
        let __rhs = (table.with(|__s| __s.bits) as i32);
        field!(br, bits_left_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (table.with(|__s| __s.value) as i32);
}
pub fn HuffExtend_244(mut x: i32, mut s: i32) -> i32 {
    if !(s >= 1) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                575,
                Ptr::<i8>::from_string_literal(b"HuffExtend"),
            )
        });
        'loop_: while true {}
    };
    let mut half: i32 = (1 << (s - 1));
    if (x >= half) {
        if !(x < (1 << s)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                    578,
                    Ptr::<i8>::from_string_literal(b"HuffExtend"),
                )
            });
            'loop_: while true {}
        };
        return x;
    } else {
        return ((x - (1 << s)) + 1);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeDCTBlock_245(
    mut dc_huff: Ptr<brunsli_HuffmanTableEntry>,
    mut ac_huff: Ptr<brunsli_HuffmanTableEntry>,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut eobrun: Ptr<i32>,
    mut reset_state: Ptr<bool>,
    mut num_zero_runs: Ptr<i32>,
    mut br: Ptr<brunsli_BitReaderState>,
    mut jpg: Ptr<brunsli_JPEGData>,
    mut last_dc_coeff: Ptr<i16>,
    mut coeffs: Ptr<i16>,
) -> bool {
    let mut Am: i32 = (1 << Al);
    let mut eobrun_allowed: bool = (Ss > 0);
    if (Ss == 0) {
        let mut s: i32 = ({ ReadSymbol_243((dc_huff).clone(), (br).clone()) });
        if (s >= kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for DC coefficient.\n",
                s,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
            return false;
        }
        let mut diff: i32 = 0;
        if (s > 0) {
            let mut bits: i32 = ({ brunsli_BitReaderStateImpl::ReadBits(&br, s) });
            diff = ({ HuffExtend_244(bits, s) });
        }
        let mut coeff: i32 = ({ diff } + { ((last_dc_coeff.read()) as i32) });
        let mut dc_coeff: i32 = (coeff * Am);
        elem!(coeffs, 0).write({ (dc_coeff as i16) });
        if ({ dc_coeff } != { ((elem!(coeffs, 0).read()) as i32) }) {
            write!(libcc2rs::cerr(), "Invalid DC coefficient {:}\n", dc_coeff,);
            field!(jpg, error).write(brunsli_JPEGReadError_NON_REPRESENTABLE_DC_COEFF);
            return false;
        }
        last_dc_coeff.write({ (coeff as i16) });
        Ss.prefix_inc();
    }
    if (Ss > Se) {
        return true;
    }
    if ((eobrun.read()) > 0) {
        eobrun.with_mut(|__v| __v.prefix_dec());
        return true;
    }
    num_zero_runs.write(0);
    let mut k: i32 = Ss;
    'loop_: while (k <= Se) {
        let mut sr: i32 = ({ ReadSymbol_243((ac_huff).clone(), (br).clone()) });
        if (sr >= kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                sr,
                k,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
            return false;
        }
        let mut r: i32 = (sr >> 4);
        let mut s: i32 = (sr & 15);
        if (s > 0) {
            k += r;
            if (k > Se) {
                write!(
                    libcc2rs::cerr(),
                    "Out-of-band coefficient {:} band was {:}-{:}\n",
                    k,
                    Ss,
                    Se,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_OUT_OF_BAND_COEFF);
                return false;
            }
            if ((s + Al) >= kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Out of range AC coefficient value: s = {:} Al = {:} k = {:}\n",
                    s,
                    Al,
                    k,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_NON_REPRESENTABLE_AC_COEFF);
                return false;
            }
            let mut bits: i32 = ({ brunsli_BitReaderStateImpl::ReadBits(&br, s) });
            let mut coeff: i32 = ({ HuffExtend_244(bits, s) });
            elem!(
                coeffs,
                ({
                    let __idx = (k) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                })
            )
            .write({ ((coeff * Am) as i16) });
            num_zero_runs.write(0);
        } else if (r == 15) {
            k += 15;
            num_zero_runs.with_mut(|__v| __v.prefix_inc());
        } else {
            if ((eobrun_allowed) && (k == Ss)) && ((eobrun.read()) == 0) {
                reset_state.write(true);
            }
            eobrun.write({ (1 << r) });
            if (r > 0) {
                if !(eobrun_allowed) {
                    write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                    field!(jpg, error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                    return false;
                }
                let __rhs = ({ brunsli_BitReaderStateImpl::ReadBits(&br, r) });
                {
                    eobrun.with_mut(|__v| *__v = *__v + __rhs)
                };
            }
            break;
        }
        k.postfix_inc();
    }
    eobrun.with_mut(|__v| __v.prefix_dec());
    return true;
}
pub fn RefineDCTBlock_246(
    mut ac_huff: Ptr<brunsli_HuffmanTableEntry>,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut eobrun: Ptr<i32>,
    mut reset_state: Ptr<bool>,
    mut br: Ptr<brunsli_BitReaderState>,
    mut jpg: Ptr<brunsli_JPEGData>,
    mut coeffs: Ptr<i16>,
) -> bool {
    let mut Am: i32 = (1 << Al);
    let mut eobrun_allowed: bool = (Ss > 0);
    if (Ss == 0) {
        let mut s: i32 = ({ brunsli_BitReaderStateImpl::ReadBits(&br, 1) });
        let mut dc_coeff: i16 = (elem!(coeffs, 0).read());
        dc_coeff = { ((dc_coeff as i32) | (s * Am)) as i16 };
        elem!(coeffs, 0).write({ dc_coeff });
        Ss.prefix_inc();
    }
    if (Ss > Se) {
        return true;
    }
    let p1: Value<i32> = Rc::new(RefCell::new(Am));
    let m1: Value<i32> = Rc::new(RefCell::new(-Am));
    let mut k: i32 = Ss;
    let mut r: i32 = 0_i32;
    let mut s: i32 = 0_i32;
    let mut in_zero_run: bool = false;
    if ((eobrun.read()) <= 0) {
        'loop_: while (k <= Se) {
            s = ({ ReadSymbol_243((ac_huff).clone(), (br).clone()) });
            if (s >= kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                    s,
                    k,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
                return false;
            }
            r = (s >> 4);
            s &= 15;
            if (s != 0) {
                if (s != 1) {
                    write!(
                        libcc2rs::cerr(),
                        "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                        s,
                        k,
                    );
                    field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
                    return false;
                }
                s = if (({ brunsli_BitReaderStateImpl::ReadBits(&br, 1) }) != 0) {
                    (*p1.borrow())
                } else {
                    (*m1.borrow())
                };
                in_zero_run = false;
            } else {
                if (r != 15) {
                    if ((eobrun_allowed) && (k == Ss)) && ((eobrun.read()) == 0) {
                        reset_state.write(true);
                    }
                    eobrun.write({ (1 << r) });
                    if (r > 0) {
                        if !(eobrun_allowed) {
                            write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                            field!(jpg, error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                            return false;
                        }
                        let __rhs = ({ brunsli_BitReaderStateImpl::ReadBits(&br, r) });
                        {
                            eobrun.with_mut(|__v| *__v = *__v + __rhs)
                        };
                    }
                    break;
                }
                in_zero_run = true;
            }
            let mut __do_while = true;
            'loop_: while __do_while || (k <= Se) {
                __do_while = false;
                let mut thiscoef: i16 = (elem!(
                    coeffs,
                    ({
                        let __idx = (k) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read());
                if ((thiscoef as i32) != 0) {
                    if (({ brunsli_BitReaderStateImpl::ReadBits(&br, 1) }) != 0) {
                        if (((thiscoef as i32) & (*p1.borrow())) == 0) {
                            if ((thiscoef as i32) >= 0) {
                                thiscoef = { ((thiscoef as i32) + (*p1.borrow())) as i16 };
                            } else {
                                thiscoef = { ((thiscoef as i32) + (*m1.borrow())) as i16 };
                            }
                        }
                    }
                    elem!(
                        coeffs,
                        ({
                            let __idx = (k) as usize;
                            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                        })
                    )
                    .write({ thiscoef });
                } else {
                    if (r.prefix_dec() < 0) {
                        break;
                    }
                }
                k.postfix_inc();
            }
            if (s != 0) {
                if (k > Se) {
                    write!(
                        libcc2rs::cerr(),
                        "Out-of-band coefficient {:} band was {:}-{:}\n",
                        k,
                        Ss,
                        Se,
                    );
                    field!(jpg, error).write(brunsli_JPEGReadError_OUT_OF_BAND_COEFF);
                    return false;
                }
                elem!(
                    coeffs,
                    ({
                        let __idx = (k) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .write({ (s as i16) });
            }
            k.postfix_inc();
        }
    }
    if in_zero_run {
        write!(libcc2rs::cerr(), "Extra zero run before end-of-block.\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_EXTRA_ZERO_RUN);
        return false;
    }
    if ((eobrun.read()) > 0) {
        'loop_: while (k <= Se) {
            let mut thiscoef: i16 = (elem!(
                coeffs,
                ({
                    let __idx = (k) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                })
            )
            .read());
            if ((thiscoef as i32) != 0) {
                if (({ brunsli_BitReaderStateImpl::ReadBits(&br, 1) }) != 0) {
                    if (((thiscoef as i32) & (*p1.borrow())) == 0) {
                        if ((thiscoef as i32) >= 0) {
                            thiscoef = { ((thiscoef as i32) + (*p1.borrow())) as i16 };
                        } else {
                            thiscoef = { ((thiscoef as i32) + (*m1.borrow())) as i16 };
                        }
                    }
                }
                elem!(
                    coeffs,
                    ({
                        let __idx = (k) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .write({ thiscoef });
            }
            k.postfix_inc();
        }
    }
    eobrun.with_mut(|__v| __v.prefix_dec());
    return true;
}
pub fn ProcessRestart_247(
    mut data: Ptr<u8>,
    mut len: usize,
    mut next_restart_marker: Ptr<i32>,
    mut br: Ptr<brunsli_BitReaderState>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if !({ brunsli_BitReaderStateImpl::FinishStream(&br, (jpg).clone(), (pos.as_pointer())) }) {
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SCAN);
        return false;
    }
    let mut expected_marker: i32 = (208 + (next_restart_marker.read()));
    if ((*pos.borrow()).wrapping_add(2_usize) > len)
        || (((elem!(data, (*pos.borrow())).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < len) {
                ((elem!(data, (*pos.borrow())).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
        return false;
    };
    let mut marker: i32 = ((elem!(data, (*pos.borrow()).wrapping_add(1_usize)).read()) as i32);
    if (marker != expected_marker) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected restart marker {:} actual={:}\n",
            expected_marker,
            marker,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_WRONG_RESTART_MARKER);
        return false;
    }
    ({ brunsli_BitReaderStateImpl::Reset(&br, (*pos.borrow()).wrapping_add(2_usize)) });
    {
        next_restart_marker.with_mut(|__v| *__v = *__v + 1)
    };
    {
        next_restart_marker.with_mut(|__v| *__v = *__v & 7)
    };
    return true;
}
pub fn ProcessScan_248(
    mut data: Ptr<u8>,
    mut len: usize,
    dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    mut scan_progression: Ptr<Value<Box<[u16]>>>,
    mut is_progressive: bool,
    mut pos: Ptr<usize>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    if !({
        let _len: usize = len;
        let _pos: Ptr<usize> = (pos).clone();
        ProcessSOS_236((data).clone(), _len, _pos, (jpg).clone())
    }) {
        return false;
    }
    let mut scan_info: Ptr<brunsli_JPEGScanInfo> =
        ((jpg.with(|__s| __s.scan_info.as_pointer()) as Ptr<brunsli_JPEGScanInfo>).to_last());
    let mut is_interleaved: bool = (scan_info.with(|__s| __s.num_components) > 1_usize);
    let mut MCUs_per_row: i32 = 0_i32;
    let mut MCU_rows: i32 = 0_i32;
    if is_interleaved {
        MCUs_per_row = jpg.with(|__s| __s.MCU_cols);
        MCU_rows = jpg.with(|__s| __s.MCU_rows);
    } else {
        let c: Ptr<brunsli_JPEGComponent> =
            (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(
                ({
                    (*elem!(
                        (scan_info.with(|__s| __s.components.as_pointer())
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        0_usize
                    )
                    .upgrade()
                    .deref())
                    .comp_idx
                } as usize),
            );
        MCUs_per_row = ({
            let _a: i32 = ({ jpg.with(|__s| __s.width) } * { c.with(|__s| __s.h_samp_factor) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
            DivCeil_232(_a, _b)
        });
        MCU_rows = ({
            let _a: i32 = ({ jpg.with(|__s| __s.height) } * { c.with(|__s| __s.v_samp_factor) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
            DivCeil_232(_a, _b)
        });
    }
    let last_dc_coeff: Value<Box<[i16]>> =
        Rc::new(RefCell::new(Box::new([0_i16, 0_i16, 0_i16, 0_i16])));
    let br: Value<brunsli_BitReaderState> = Rc::new(RefCell::new(brunsli_BitReaderState::new(
        { (data).clone() },
        { len },
        { (pos.read()) },
    )));
    let mut restarts_to_go: i32 = jpg.with(|__s| __s.restart_interval);
    let next_restart_marker: Value<i32> = Rc::new(RefCell::new(0));
    let eobrun: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let block_scan_index: Value<i32> = Rc::new(RefCell::new(0));
    let mut Al: i32 = if is_progressive {
        scan_info.with(|__s| __s.Al)
    } else {
        0
    };
    let mut Ah: i32 = if is_progressive {
        scan_info.with(|__s| __s.Ah)
    } else {
        0
    };
    let mut Ss: i32 = if is_progressive {
        scan_info.with(|__s| __s.Ss)
    } else {
        0
    };
    let mut Se: i32 = if is_progressive {
        scan_info.with(|__s| __s.Se)
    } else {
        63
    };
    let mut scan_bitmask: u16 = (if (Ah == 0) {
        ((65535 << Al) as u32)
    } else {
        (1_u32 << Al)
    } as u16);
    let mut refinement_bitmask: u16 = (((1 << Al) - 1) as u16);
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { scan_info.with(|__s| __s.num_components) }) {
        let mut comp_idx: i32 = ({
            (*elem!(
                (scan_info.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>),
                i
            )
            .upgrade()
            .deref())
            .comp_idx
        } as i32);
        let mut k: i32 = Ss;
        'loop_: while (k <= Se) {
            if (({ ((elem!(scan_progression, comp_idx).read()).borrow()[(k) as usize] as i32) }
                & { (scan_bitmask as i32) })
                != 0)
            {
                write!(
                    libcc2rs::cerr(),
                    "Overlapping scans: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    comp_idx,
                    k,
                    (elem!(scan_progression, i).read()).borrow()[(k) as usize],
                    scan_bitmask,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_OVERLAPPING_SCANS);
                return false;
            }
            if (({ ((elem!(scan_progression, comp_idx).read()).borrow()[(k) as usize] as i32) }
                & { (refinement_bitmask as i32) })
                != 0)
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid scan order, a more refined scan was already done: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    comp_idx,
                    k,
                    (elem!(scan_progression, i).read()).borrow()[(k) as usize],
                    scan_bitmask,
                );
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SCAN_ORDER);
                return false;
            }
            (elem!(scan_progression, comp_idx).read()).borrow_mut()[(k) as usize] = {
                (((elem!(scan_progression, comp_idx).read()).borrow()[(k) as usize] as i32)
                    | (scan_bitmask as i32)) as u16
            };
            k.prefix_inc();
        }
        i.prefix_inc();
    }
    if (Al > 10) {
        write!(
            libcc2rs::cerr(),
            "Scan parameter Al = {:} is not supported in brunsli.\n",
            Al,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_NON_REPRESENTABLE_AC_COEFF);
        return false;
    }
    let mut mcu_y: i32 = 0;
    'loop_: while (mcu_y < MCU_rows) {
        let mut mcu_x: i32 = 0;
        'loop_: while (mcu_x < MCUs_per_row) {
            if (jpg.with(|__s| __s.restart_interval) > 0) {
                if (restarts_to_go == 0) {
                    if ({
                        ProcessRestart_247(
                            (data).clone(),
                            len,
                            (next_restart_marker.as_pointer()),
                            (br.as_pointer()),
                            (jpg).clone(),
                        )
                    }) {
                        restarts_to_go = jpg.with(|__s| __s.restart_interval);
                        {
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                                .to_any()
                                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>).to_any()
                        };
                        if ((*eobrun.borrow()) > 0) {
                            write!(libcc2rs::cerr(), "End-of-block run too long.\n",);
                            field!(jpg, error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                            return false;
                        }
                        (*eobrun.borrow_mut()) = -1_i32;
                    } else {
                        return false;
                    }
                }
                restarts_to_go.prefix_dec();
            }
            if ({ brunsli_BitReaderStateImpl::IsUnhealthy(&br.as_pointer()) }) {
                write!(libcc2rs::cerr(), "Unexpected end of scan.\n",);
                field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SCAN);
                return false;
            }
            let mut i: usize = 0_usize;
            'loop_: while ({ i } < { scan_info.with(|__s| __s.num_components) }) {
                let mut si: Ptr<brunsli_JPEGComponentScanInfo> = ((scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(i));
                let mut c: Ptr<brunsli_JPEGComponent> =
                    ((jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>)
                        .offset((si.with(|__s| __s.comp_idx) as usize)));
                let mut dc_lut: Ptr<brunsli_HuffmanTableEntry> =
                    ((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(dc_huff_lut))
                        as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({ si.with(|__s| __s.dc_tbl_idx) } * {
                                kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())
                            }) as usize),
                        ));
                let mut ac_lut: Ptr<brunsli_HuffmanTableEntry> =
                    ((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(ac_huff_lut))
                        as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({ si.with(|__s| __s.ac_tbl_idx) } * {
                                kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())
                            }) as usize),
                        ));
                let mut nblocks_y: i32 = if is_interleaved {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                };
                let mut nblocks_x: i32 = if is_interleaved {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while (iy < nblocks_y) {
                    let mut ix: i32 = 0;
                    'loop_: while (ix < nblocks_x) {
                        let mut block_y: i32 = ((mcu_y * nblocks_y) + iy);
                        let mut block_x: i32 = ((mcu_x * nblocks_x) + ix);
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        let reset_state: Value<bool> = Rc::new(RefCell::new(false));
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        let mut coeffs: Ptr<i16> =
                            ((c.with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>).offset(
                                ((block_idx * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize),
                            ));
                        if (Ah == 0) {
                            if !({
                                DecodeDCTBlock_245(
                                    (dc_lut).clone(),
                                    (ac_lut).clone(),
                                    Ss,
                                    Se,
                                    Al,
                                    (eobrun.as_pointer()),
                                    (reset_state.as_pointer()),
                                    (num_zero_runs.as_pointer()),
                                    (br.as_pointer()),
                                    (jpg).clone(),
                                    ((last_dc_coeff.as_pointer() as Ptr<i16>)
                                        .offset(si.with(|__s| __s.comp_idx))),
                                    (coeffs).clone(),
                                )
                            }) {
                                return false;
                            }
                        } else {
                            if !({
                                RefineDCTBlock_246(
                                    (ac_lut).clone(),
                                    Ss,
                                    Se,
                                    Al,
                                    (eobrun.as_pointer()),
                                    (reset_state.as_pointer()),
                                    (br.as_pointer()),
                                    (jpg).clone(),
                                    (coeffs).clone(),
                                )
                            }) {
                                return false;
                            }
                        }
                        if (*reset_state.borrow()) {
                            {
                                let __init = (*block_scan_index.borrow());
                                (*scan_info.with(|__s| __s.reset_points.clone()).borrow_mut())
                                    .push(__init)
                            };
                        }
                        if ((*num_zero_runs.borrow()) > 0) {
                            let mut info: brunsli_JPEGScanInfo_ExtraZeroRunInfo =
                                <brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default();
                            info.block_idx = (*block_scan_index.borrow());
                            info.num_extra_zero_runs = (*num_zero_runs.borrow());
                            {
                                let a0_clone = info.clone();
                                (*scan_info
                                    .with(|__s| __s.extra_zero_runs.clone())
                                    .borrow_mut())
                                .push(a0_clone)
                            };
                        }
                        (*block_scan_index.borrow_mut()).prefix_inc();
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            mcu_x.prefix_inc();
        }
        mcu_y.prefix_inc();
    }
    if ((*eobrun.borrow()) > 0) {
        write!(libcc2rs::cerr(), "End-of-block run too long.\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
        return false;
    }
    if !({
        brunsli_BitReaderStateImpl::FinishStream(&br.as_pointer(), (jpg).clone(), (pos).clone())
    }) {
        field!(jpg, error).write(brunsli_JPEGReadError_INVALID_SCAN);
        return false;
    }
    if ({ (pos.read()) } > { len }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of file during scan. pos={:} len={:}\n",
            (pos.read()),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    }
    return true;
}
pub fn FixupIndexes_249(mut jpg: Ptr<brunsli_JPEGData>) -> bool {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        let mut c: Ptr<brunsli_JPEGComponent> =
            ((jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i));
        let mut found_index: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ({ j } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            if ({
                {
                    (*elem!(
                        (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                        j
                    )
                    .upgrade()
                    .deref())
                    .index
                }
            } == { (c.with(|__s| __s.quant_idx) as i32) })
            {
                field!(c, quant_idx).write((j as u8));
                found_index = true;
                break;
            }
            j.prefix_inc();
        }
        if !(found_index) {
            write!(libcc2rs::cerr(), "Quantization table with index ",);
            libcc2rs::cerr().write_all(
                &([
                    (&[c.with(|__s| __s.quant_idx) as u8] as &[u8]),
                    (b" not found." as &[u8]),
                    (&[b'\n'] as &[u8]),
                ]
                .concat()),
            );
            field!(jpg, error).write(brunsli_JPEGReadError_QUANT_TABLE_NOT_FOUND);
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
pub fn FindNextMarker_250(mut data: Ptr<u8>, mut len: usize, mut pos: usize) -> usize {
    thread_local!(
        static kIsValidMarker_251: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8,
        ])));
    );
    let mut num_skipped: usize = 0_usize;
    'loop_: while ((pos).wrapping_add(1_usize) < len)
        && (((((elem!(data, pos).read()) as i32) != 255)
            || (((elem!(data, (pos).wrapping_add(1_usize)).read()) as i32) < 192))
            || (!(({
                let __idx =
                    (((elem!(data, (pos).wrapping_add(1_usize)).read()) as i32) - 192) as usize;
                kIsValidMarker_251.with(|rc| rc.borrow()[__idx])
            }) != 0)))
    {
        pos.prefix_inc();
        num_skipped.prefix_inc();
    }
    return num_skipped;
}
pub fn ReadJpeg_196(
    mut data: Ptr<u8>,
    mut len: usize,
    mut mode: brunsli_JpegReadMode,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*pos.borrow()).wrapping_add(2_usize) > len)
        || (((elem!(data, (*pos.borrow())).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < len) {
                ((elem!(data, (*pos.borrow())).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            len,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
        return false;
    };
    let mut marker: i32 = ((elem!(data, (*pos.borrow()).wrapping_add(1_usize)).read()) as i32);
    (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add(2_usize) };
    if (marker != 216) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected SOI marker, actual={:}\n",
            marker,
        );
        field!(jpg, error).write(brunsli_JPEGReadError_SOI_NOT_FOUND);
        return false;
    }
    let mut lut_size: i32 = (kMaxHuffmanTables_6.with(|rc| *rc.borrow())
        * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow()));
    let dc_huff_lut: Value<Vec<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
        (0..(lut_size as usize) as usize)
            .map(|_| <brunsli_HuffmanTableEntry>::default())
            .collect::<Vec<_>>(),
    ));
    let ac_huff_lut: Value<Vec<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
        (0..(lut_size as usize) as usize)
            .map(|_| <brunsli_HuffmanTableEntry>::default())
            .collect::<Vec<_>>(),
    ));
    let mut found_sof: bool = false;
    let found_dri: Value<bool> = Rc::new(RefCell::new(false));
    let scan_progression: Value<Box<[Value<Box<[u16]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16,
        ]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
    ])));
    {
        let __a0 = 0_usize as usize;
        (*jpg.with(|__s| __s.padding_bits.clone()).borrow_mut())
            .resize_with(__a0, || <i32>::default())
    };
    let mut is_progressive: bool = false;
    let mut __do_while = true;
    'loop_: while __do_while || (marker != 217) {
        __do_while = false;
        let mut num_skipped: usize = ({ FindNextMarker_250((data).clone(), len, (*pos.borrow())) });
        if (num_skipped > 0_usize) {
            {
                let __a1 = 255_u8;
                (*jpg.with(|__s| __s.marker_order.clone()).borrow_mut()).push(__a1)
            };
            (jpg.with(|__s| __s.inter_marker_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>)
                .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                    __v.push(Rc::new(RefCell::new({
                        let __count = data
                            .offset((*pos.borrow()) as isize)
                            .offset((num_skipped) as isize)
                            .get_offset()
                            - data.offset((*pos.borrow()) as isize).get_offset();
                        PtrValueIter::new(&data.offset((*pos.borrow()) as isize), __count)
                            .collect::<Vec<_>>()
                    })))
                });
            (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add(num_skipped) };
        }
        if ((*pos.borrow()).wrapping_add(2_usize) > len)
            || (((elem!(data, (*pos.borrow())).read()) as i32) != 255)
        {
            write!(
                libcc2rs::cerr(),
                "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
                (if ((*pos.borrow()) < len) {
                    ((elem!(data, (*pos.borrow())).read()) as i32)
                } else {
                    0
                }),
                (*pos.borrow()),
                len,
            );
            field!(jpg, error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
            return false;
        };
        marker = ((elem!(data, (*pos.borrow()).wrapping_add(1_usize)).read()) as i32);
        (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add(2_usize) };
        let mut ok: bool = true;
        'switch: {
            match { marker } {
                __v if __v == 192 || __v == 193 || __v == 194 => {
                    is_progressive = (marker == 194);
                    ok = ({
                        ProcessSOF_235((data).clone(), len, mode, (pos.as_pointer()), (jpg).clone())
                    });
                    found_sof = true;
                    break 'switch;
                }
                __v if __v == 196 => {
                    ok = ({
                        ProcessDHT_237(
                            (data).clone(),
                            len,
                            mode,
                            (dc_huff_lut.as_pointer()),
                            (ac_huff_lut.as_pointer()),
                            (pos.as_pointer()),
                            (jpg).clone(),
                        )
                    });
                    break 'switch;
                }
                __v if __v == 208
                    || __v == 209
                    || __v == 210
                    || __v == 211
                    || __v == 212
                    || __v == 213
                    || __v == 214
                    || __v == 215 =>
                {
                    break 'switch;
                }
                __v if __v == 217 => {
                    break 'switch;
                }
                __v if __v == 218 => {
                    if ((mode as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
                        ok = ({
                            ProcessScan_248(
                                (data).clone(),
                                len,
                                dc_huff_lut.as_pointer(),
                                ac_huff_lut.as_pointer(),
                                (scan_progression.as_pointer() as Ptr<Value<Box<[u16]>>>),
                                is_progressive,
                                (pos.as_pointer()),
                                (jpg).clone(),
                            )
                        });
                    }
                    break 'switch;
                }
                __v if __v == 219 => {
                    ok = ({
                        ProcessDQT_239((data).clone(), len, (pos.as_pointer()), (jpg).clone())
                    });
                    break 'switch;
                }
                __v if __v == 221 => {
                    ok = ({
                        ProcessDRI_240(
                            (data).clone(),
                            len,
                            (pos.as_pointer()),
                            (found_dri.as_pointer()),
                            (jpg).clone(),
                        )
                    });
                    break 'switch;
                }
                __v if __v == 224
                    || __v == 225
                    || __v == 226
                    || __v == 227
                    || __v == 228
                    || __v == 229
                    || __v == 230
                    || __v == 231
                    || __v == 232
                    || __v == 233
                    || __v == 234
                    || __v == 235
                    || __v == 236
                    || __v == 237
                    || __v == 238
                    || __v == 239 =>
                {
                    if ((mode as i32) != (brunsli_JpegReadMode_JPEG_READ_TABLES as i32)) {
                        ok = ({
                            ProcessAPP_241((data).clone(), len, (pos.as_pointer()), (jpg).clone())
                        });
                    }
                    break 'switch;
                }
                __v if __v == 254 => {
                    if ((mode as i32) != (brunsli_JpegReadMode_JPEG_READ_TABLES as i32)) {
                        ok = ({
                            ProcessCOM_242((data).clone(), len, (pos.as_pointer()), (jpg).clone())
                        });
                    }
                    break 'switch;
                }
                _ => {
                    write!(
                        libcc2rs::cerr(),
                        "Unsupported marker: {:} pos={:} len={:}\n",
                        marker,
                        (*pos.borrow()),
                        len,
                    );
                    field!(jpg, error).write(brunsli_JPEGReadError_UNSUPPORTED_MARKER);
                    ok = false;
                    break 'switch;
                }
            }
        };
        if !(ok) {
            return false;
        }
        {
            let __a1 = (marker as u8);
            (*jpg.with(|__s| __s.marker_order.clone()).borrow_mut()).push(__a1)
        };
        if ((mode as i32) == (brunsli_JpegReadMode_JPEG_READ_HEADER as i32)) && (found_sof) {
            break;
        }
    }
    if !(found_sof) {
        write!(libcc2rs::cerr(), "Missing SOF marker.\n",);
        field!(jpg, error).write(brunsli_JPEGReadError_SOF_NOT_FOUND);
        return false;
    }
    if ((mode as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
        if ((*pos.borrow()) < len) {
            (jpg.with(|__s| __s.tail_data.as_pointer()) as Ptr<Vec<u8>>).write({
                let __count = data.offset((len) as isize).get_offset()
                    - data.offset((*pos.borrow()) as isize).get_offset();
                PtrValueIter::new(&data.offset((*pos.borrow()) as isize), __count)
                    .collect::<Vec<_>>()
            });
        }
        if !({ FixupIndexes_249((jpg).clone()) }) {
            return false;
        }
        if (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).is_empty() {
            write!(libcc2rs::cerr(), "Need at least one Huffman code table.\n",);
            field!(jpg, error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_ERROR);
            return false;
        }
        if ({ (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len() } >= {
            (kMaxDHTMarkers_10.with(|rc| *rc.borrow()) as usize)
        }) {
            write!(libcc2rs::cerr(), "Too many Huffman tables.\n",);
            field!(jpg, error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_ERROR);
            return false;
        }
    }
    return true;
}
pub fn NextTableBitSize_252(mut count: Ptr<i32>, mut len: i32) -> i32 {
    let mut left: i32 = (1 << (len - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())));
    'loop_: while (len < kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        left -= { (elem!(count, len).read()) };
        if (left <= 0) {
            break;
        }
        len.prefix_inc();
        left <<= 1;
    }
    return (len - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()));
}
pub fn BuildJpegHuffmanTable_238(
    mut count: Ptr<i32>,
    mut symbols: Ptr<i32>,
    mut lut: Ptr<brunsli_HuffmanTableEntry>,
) {
    let mut code: brunsli_HuffmanTableEntry = brunsli_HuffmanTableEntry::new();
    let mut table: Ptr<brunsli_HuffmanTableEntry> = Ptr::<brunsli_HuffmanTableEntry>::null();
    let mut len: i32 = 0_i32;
    let mut idx: i32 = 0_i32;
    let mut key: i32 = 0_i32;
    let mut reps: i32 = 0_i32;
    let mut low: i32 = 0_i32;
    let mut table_bits: i32 = 0_i32;
    let mut table_size: i32 = 0_i32;
    let tmp_count: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    let mut total_count: i32 = 0;
    len = 1;
    'loop_: while (len <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        (*tmp_count.borrow_mut())[(len) as usize] = { (elem!(count, len).read()) };
        total_count += (*tmp_count.borrow())[(len) as usize];
        len.prefix_inc();
    }
    table = (lut).clone();
    table_bits = kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow());
    table_size = (1 << table_bits);
    if (total_count == 1) {
        code.bits = 0_u8;
        code.value = ((elem!(symbols, 0).read()) as u16);
        key = 0;
        'loop_: while (key < table_size) {
            elem!(table, key).write({ (code).clone() });
            key.prefix_inc();
        }
        return;
    }
    key = 0;
    idx = 0;
    len = 1;
    'loop_: while (len <= kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())) {
        'loop_: while ((*tmp_count.borrow())[(len) as usize] > 0) {
            code.bits = (len as u8);
            code.value = ((elem!(symbols, idx.postfix_inc()).read()) as u16);
            reps = (1 << (kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()) - len));
            'loop_: while (reps.postfix_dec() != 0) {
                let __rhs = (code).clone();
                elem!(table, key.postfix_inc()).write(__rhs);
            }
            (*tmp_count.borrow_mut())[(len) as usize].prefix_dec();
        }
        len.prefix_inc();
    }
    table += table_size;
    table_size = 0;
    low = 0;
    len = (kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()) + 1);
    'loop_: while (len <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        'loop_: while ((*tmp_count.borrow())[(len) as usize] > 0) {
            if (low >= table_size) {
                table += table_size;
                table_bits = ({ NextTableBitSize_252((tmp_count.as_pointer() as Ptr<i32>), len) });
                table_size = (1 << table_bits);
                low = 0;
                field!(elem!(lut, key), bits).write(
                    ((table_bits + kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())) as u8),
                );
                field!(elem!(lut, key), value).write({
                    (({ (((table).clone() - (lut).clone()) as i64) } - { (key as i64) }) as u16)
                });
                key.prefix_inc();
            }
            code.bits = ((len - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())) as u8);
            code.value = ((elem!(symbols, idx.postfix_inc()).read()) as u16);
            reps = (1 << (table_bits - (code.bits as i32)));
            'loop_: while (reps.postfix_dec() != 0) {
                let __rhs = (code).clone();
                elem!(table, low.postfix_inc()).write(__rhs);
            }
            (*tmp_count.borrow_mut())[(len) as usize].prefix_dec();
        }
        len.prefix_inc();
    }
}
impl brunsli_Storage {
    pub fn new(mut data: Ptr<u8>, mut length: usize) -> Self {
        let __this: brunsli_Storage = Self {
            data: (data).clone(),
            length: length,
            pos: 0_usize,
        };
        if !(length > 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"write_bits.cc"),
                    14,
                    Ptr::<i8>::from_string_literal(b"Storage"),
                )
            });
            'loop_: while true {}
        };
        elem!(data, 0).write(0_u8);
        __this
    }
}
impl brunsli_Storage {}
pub fn ReadFileInternal_253(mut file: Ptr<CFile>, mut content: Ptr<Vec<i8>>) -> bool {
    if (match file.with_mut(|__v: &mut CFile| __v.seek(0_i64, ::libc::SEEK_END)) {
        -1 => -1,
        _ => 0,
    } != 0)
    {
        eprintln!("Failed to seek end of input file.");
        return false;
    }
    let mut input_size: i32 = (file.with(|__f| __f.tell()) as i32);
    if (input_size == 0) {
        eprintln!("Input file is empty.");
        return false;
    }
    if (match file.with_mut(|__v: &mut CFile| __v.seek(0_i64, ::libc::SEEK_SET)) {
        -1 => -1,
        _ => 0,
    } != 0)
    {
        eprintln!("Failed to rewind input file to the beginning.");
        return false;
    }
    {
        content.with_mut(|__v: &mut Vec<i8>| __v.pop());
        content.with_mut(|__v: &mut Vec<i8>| __v.resize((input_size as usize) as usize, 0));
        content.with_mut(|__v: &mut Vec<i8>| __v.push(0))
    };
    let mut read_pos: usize = 0_usize;
    'loop_: while ({ read_pos } < { ((*content.upgrade().deref()).len() - 1) }) {
        let mut bytes_read: usize = {
            let __a0 = ((if read_pos as usize
                >= (*((content).clone() as Ptr<Vec<i8>>).upgrade().deref())
                    .len()
                    .saturating_sub(1)
            {
                panic!("out of bounds access")
            } else {
                ((content).clone() as Ptr<Vec<i8>>)
                    .decay()
                    .offset(read_pos as isize)
            }) as Ptr<i8>)
                .to_any();
            let __a1 = 1_usize;
            let __a2 = ((((*content.upgrade().deref()).len() - 1) as u64)
                .wrapping_sub((read_pos as u64)) as usize);
            let __a3 = (file).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        };
        if (bytes_read == 0_usize) {
            eprintln!("Failed to read input file");
            return false;
        }
        read_pos = { (read_pos).wrapping_add(bytes_read) };
    }
    return true;
}
pub fn ReadFile_254(file_name: Ptr<Vec<i8>>, mut content: Ptr<Vec<i8>>) -> bool {
    let mut file: Ptr<CFile> = match CFile::open(
        &(Ptr::<Vec<i8>>::decay(&(file_name)) as Ptr<i8>).to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    if (file).is_null() {
        eprintln!("Failed to open input file.");
        return false;
    }
    let mut ok: bool = ({ ReadFileInternal_253((file).clone(), (content).clone()) });
    if ({
        let __r = file.with(|__f| __f.close());
        file.delete();
        __r
    } != 0)
    {
        if ok {
            eprintln!("Failed to close input file.");
        }
        return false;
    }
    return ok;
}
pub fn WriteFileInternal_255(mut file: Ptr<CFile>, content: Ptr<Vec<i8>>) -> bool {
    let mut write_pos: usize = 0_usize;
    'loop_: while ({ write_pos } < { ((*content.upgrade().deref()).len() - 1) }) {
        let mut bytes_written: usize = {
            let __a0 = (((Ptr::<Vec<i8>>::decay(&(content)) as Ptr<i8>).offset(write_pos))
                as Ptr<i8>)
                .to_any();
            let __a1 = 1_usize;
            let __a2 = ((((*content.upgrade().deref()).len() - 1) as u64)
                .wrapping_sub((write_pos as u64)) as usize);
            let __a3 = (file).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        };
        if (bytes_written == 0_usize) {
            eprintln!("Failed to write output.");
            return false;
        }
        write_pos = { (write_pos).wrapping_add(bytes_written) };
    }
    return true;
}
pub fn WriteFile_256(file_name: Ptr<Vec<i8>>, content: Ptr<Vec<i8>>) -> bool {
    let mut file: Ptr<CFile> = match CFile::open(
        &(Ptr::<Vec<i8>>::decay(&(file_name)) as Ptr<i8>).to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    if (file).is_null() {
        eprintln!("Failed to open file for writing.");
        return false;
    }
    let mut ok: bool = ({
        let _file: Ptr<CFile> = (file).clone();
        let _content: Ptr<Vec<i8>> = (content).clone();
        WriteFileInternal_255(_file, _content)
    });
    if ({
        let __r = file.with(|__f| __f.close());
        file.delete();
        __r
    } != 0)
    {
        if ok {
            eprintln!("Failed to close output file.");
        }
        return false;
    }
    return ok;
}
pub fn ProcessFile_257(file_name: Ptr<Vec<i8>>, outfile_name: Ptr<Vec<i8>>) -> bool {
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    let mut ok: bool = ({
        let _file_name: Ptr<Vec<i8>> = (file_name).clone();
        let _content: Ptr<Vec<i8>> = (input.as_pointer());
        ReadFile_254(_file_name, _content)
    });
    if !(ok) {
        return false;
    }
    let output: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    {
        let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
        let mut input_data: Ptr<u8> = (input.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>();
        ok = ({
            ReadJpeg_196(
                (input_data).clone(),
                ((*input.borrow()).len() - 1),
                brunsli_JpegReadMode_JPEG_READ_ALL,
                (jpg.as_pointer()),
            )
        });
        {
            (*input.borrow_mut()).clear();
            (*input.borrow_mut()).push(0)
        };
        (*input.borrow_mut()).shrink_to_fit();
        if !(ok) {
            eprintln!("Failed to parse JPEG input.");
            return false;
        }
        let output_size: Value<usize> = Rc::new(RefCell::new(
            ({ GetMaximumBrunsliEncodedSize_145(jpg.as_pointer()) }),
        ));
        {
            (*output.borrow_mut()).pop();
            (*output.borrow_mut()).resize((*output_size.borrow()) as usize, 0);
            (*output.borrow_mut()).push(0)
        };
        let mut output_data: Ptr<u8> =
            ((output.as_pointer() as Ptr<i8>).offset(0_usize)).reinterpret_cast::<u8>();
        ok = ({
            BrunsliEncodeJpeg_191(
                jpg.as_pointer(),
                (output_data).clone(),
                (output_size.as_pointer()),
            )
        });
        if !(ok) {
            eprintln!("Failed to transform JPEG to Brunsli");
            return false;
        }
        {
            (*output.borrow_mut()).pop();
            (*output.borrow_mut()).resize((*output_size.borrow()) as usize, 0);
            (*output.borrow_mut()).push(0)
        };
    }
    ok = ({
        let _file_name: Ptr<Vec<i8>> = (outfile_name).clone();
        let _content: Ptr<Vec<i8>> = output.as_pointer();
        WriteFile_256(_file_name, _content)
    });
    return ok;
}
pub fn main() {
    let argv: Vec<Value<Vec<i8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.bytes().map(|c| c as i8).collect())))
        .collect();
    let mut argv: Value<Vec<Ptr<i8>>> = Rc::new(RefCell::new(
        argv.iter()
            .map(|x| {
                x.borrow_mut().push(0);
                x.as_pointer()
            })
            .collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<i8>>) -> i32 {
    if (argc != 2) && (argc != 3) {
        eprintln!("Usage: cbrunsli FILE [OUTPUT_FILE, default=FILE.brn]");
        return 1;
    }
    let file_name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = (elem!(argv, 1).read()).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    if (*file_name.borrow()).len() <= 1 {
        eprintln!("Empty input file name.");
        return 1;
    }
    let outfile_name: Value<Vec<i8>> = Rc::new(RefCell::new(if (argc == 2) {
        {
            let mut r = (*file_name.borrow()).clone();
            r.pop();
            Ptr::<i8>::from_string_literal(b".brn").with_c_str(|__s| r.extend_from_slice(__s));
            r.push(0);
            r
        }
    } else {
        {
            let mut __bytes = (elem!(argv, 2).read()).to_c_bytes();
            __bytes.push(0);
            __bytes
        }
    }));
    let mut ok: bool = ({ ProcessFile_257(file_name.as_pointer(), outfile_name.as_pointer()) });
    return if ok { 0 } else { 1 };
}
pub trait brunsli_ANSCoderImpl {
    fn PutSymbol(&self, t: brunsli_ANSEncSymbolInfo, nbits: Ptr<u8>) -> u32;
    fn GetState(&self) -> u32;
}
impl brunsli_ANSCoderImpl for Ptr<brunsli_ANSCoder> {
    fn PutSymbol(&self, mut t: brunsli_ANSEncSymbolInfo, mut nbits: Ptr<u8>) -> u32 {
        let mut bits: u32 = 0_u32;
        nbits.write(0_u8);
        if (((*self).with(|__s| __s.state_)
            >> (32 - BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())))
            >= (t.freq_ as u32))
        {
            bits = ((*self).with(|__s| __s.state_) & 65535_u32);
            {
                field!((*self), state_).with_mut(|__v| *__v = *__v >> 16)
            };
            nbits.write(16_u8);
        }
        field!((*self), state_).write({
            (((((*self).with(|__s| __s.state_)).wrapping_div((t.freq_ as u32)))
                << BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()))
            .wrapping_add((((*self).with(|__s| __s.state_)).wrapping_rem((t.freq_ as u32)))))
            .wrapping_add((t.start_ as u32))
        });
        return bits;
    }
    fn GetState(&self) -> u32 {
        return (*self).with(|__s| __s.state_);
    }
}
pub trait brunsli_BitReaderStateImpl {
    fn Reset(&self, pos: usize);
    fn GetNextByte(&self) -> u8;
    fn FillBitWindow(&self);
    fn ReadBits(&self, nbits: i32) -> i32;
    fn IsUnhealthy(&self) -> bool;
    fn FinishStream(&self, jpg: Ptr<brunsli_JPEGData>, pos: Ptr<usize>) -> bool;
}
impl brunsli_BitReaderStateImpl for Ptr<brunsli_BitReaderState> {
    fn Reset(&self, mut pos: usize) {
        field!((*self), pos_).write(pos);
        field!((*self), val_).write(0_u64);
        field!((*self), bits_left_).write(0);
        field!((*self), next_marker_pos_)
            .write(((*self).with(|__s| __s.len_)).wrapping_sub(2_usize));
        ({ brunsli_BitReaderStateImpl::FillBitWindow(self) });
    }
    fn GetNextByte(&self) -> u8 {
        if ((*self).with(|__s| __s.pos_) >= (*self).with(|__s| __s.next_marker_pos_)) {
            field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            return 0_u8;
        }
        let mut c: u8 = (elem!(
            (*self).with(|__s| __s.data_.clone()),
            field!((*self), pos_).with_mut(|__v| __v.postfix_inc())
        )
        .read());
        if ((c as i32) == 255) {
            let mut escape: u8 = (elem!(
                (*self).with(|__s| __s.data_.clone()),
                (*self).with(|__s| __s.pos_)
            )
            .read());
            if ((escape as i32) == 0) {
                field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            } else {
                field!((*self), next_marker_pos_)
                    .write(((*self).with(|__s| __s.pos_)).wrapping_sub(1_usize));
            }
        }
        return c;
    }
    fn FillBitWindow(&self) {
        if ((*self).with(|__s| __s.bits_left_) <= 16) {
            'loop_: while ((*self).with(|__s| __s.bits_left_) <= 56) {
                {
                    field!((*self), val_).with_mut(|__v| *__v = *__v << 8)
                };
                {
                    let __rhs = (({ brunsli_BitReaderStateImpl::GetNextByte(self) }) as u64);
                    field!((*self), val_).with_mut(|__v| *__v = *__v | __rhs)
                };
                {
                    field!((*self), bits_left_).with_mut(|__v| *__v = *__v + 8)
                };
            }
        }
    }
    fn ReadBits(&self, mut nbits: i32) -> i32 {
        ({ brunsli_BitReaderStateImpl::FillBitWindow(self) });
        let mut val: u64 = (((*self).with(|__s| __s.val_)
            >> ((*self).with(|__s| __s.bits_left_) - nbits))
            & ((1_u64 << nbits).wrapping_sub(1_u64)));
        {
            let __rhs = nbits;
            field!((*self), bits_left_).with_mut(|__v| *__v = *__v - __rhs)
        };
        if !(val < ((1_u32 << 31) as u64)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                    471,
                    Ptr::<i8>::from_string_literal(b"ReadBits"),
                )
            });
            'loop_: while true {}
        };
        return (val as i32);
    }
    fn IsUnhealthy(&self) -> bool {
        return ((*self).with(|__s| __s.pos_)
            > (((*self).with(|__s| __s.next_marker_pos_)).wrapping_add(32_usize)));
    }
    fn FinishStream(&self, mut jpg: Ptr<brunsli_JPEGData>, mut pos: Ptr<usize>) -> bool {
        let mut npadbits: i32 = ((*self).with(|__s| __s.bits_left_) & 7);
        if (npadbits > 0) {
            let mut padmask: u64 = (1_u64 << npadbits).wrapping_sub(1_u64);
            let mut padbits: u64 = (((*self).with(|__s| __s.val_)
                >> ((*self).with(|__s| __s.bits_left_) - npadbits))
                & padmask);
            if (padbits != padmask) {
                field!(jpg, has_zero_padding_bit).write(true);
            }
            let mut i: i32 = (npadbits - 1);
            'loop_: while (i >= 0) {
                {
                    let __a1 = (((padbits >> i) & 1_u64) as i32);
                    (*jpg.with(|__s| __s.padding_bits.clone()).borrow_mut()).push(__a1)
                };
                i.prefix_dec();
            }
        }
        let mut unused_bytes_left: i32 = ((*self).with(|__s| __s.bits_left_) >> 3);
        'loop_: while (unused_bytes_left.postfix_dec() > 0) {
            field!((*self), pos_).with_mut(|__v| __v.prefix_dec());
            if (((*self).with(|__s| __s.pos_) < (*self).with(|__s| __s.next_marker_pos_))
                && (((elem!(
                    (*self).with(|__s| __s.data_.clone()),
                    (*self).with(|__s| __s.pos_)
                )
                .read()) as i32)
                    == 0))
                && (((elem!(
                    (*self).with(|__s| __s.data_.clone()),
                    ((*self).with(|__s| __s.pos_)).wrapping_sub(1_usize)
                )
                .read()) as i32)
                    == 255)
            {
                field!((*self), pos_).with_mut(|__v| __v.prefix_dec());
            }
        }
        if ((*self).with(|__s| __s.pos_) > (*self).with(|__s| __s.next_marker_pos_)) {
            write!(libcc2rs::cerr(), "Unexpected end of scan.\n",);
            return false;
        }
        pos.write({ (*self).with(|__s| __s.pos_) });
        return true;
    }
}
pub trait brunsli_ComponentStateImpl {
    fn SetWidth(&self, w: i32);
    fn InitAll(&self);
    fn destructor(&self);
}
impl brunsli_ComponentStateImpl for Ptr<brunsli_ComponentState> {
    fn SetWidth(&self, mut w: i32) {
        field!((*self), width).write(w);
        {
            let __a0 = ((w + 1) as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_is_nonempty.clone())
                .borrow_mut())
            .resize(__a0, 1)
        };
        {
            let __a0 = (w as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_num_nonzeros.clone())
                .borrow_mut())
            .resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 =
                (((kDCTBlockSize_3.with(|rc| *rc.borrow()) * 2) * (w + 3)) as usize) as usize;
            (*(*self).with(|__s| __s.prev_abs_coeff.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = ((kDCTBlockSize_3.with(|rc| *rc.borrow()) * (w + 1)) as usize) as usize;
            (*(*self).with(|__s| __s.prev_sign.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
    }
    fn destructor(&self) {
        {
            let __p = (*self.upgrade().deref()).num_nonzero_prob.as_pointer();
            for __i in 0..__p.len() {
                brunsli_ProbImpl::destructor(&__p.offset(__i as isize));
            }
        }
    }
    fn InitAll(&self) {
        let mut i: i32 = 0;
        'loop_: while (i < (kNumNonzeroBuckets_90.with(|rc| *rc.borrow()) as i32)) {
            let mut k: i32 = 0;
            'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                let mut v: i32 = ((({
                    let __idx = (k) as usize;
                    kInitProb_110.with(|rc| rc.borrow()[__idx])
                }) as i32)
                    + (9 * (i - 7)));
                if !(v <= 255) {
                    ({
                        BrunsliDumpAndAbort_79(
                            Ptr::<i8>::from_string_literal(b"context.cc"),
                            227,
                            Ptr::<i8>::from_string_literal(b"InitAll"),
                        )
                    });
                    'loop_: while true {}
                };
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.is_zero_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset((((i * kDCTBlockSize_3.with(|rc| *rc.borrow())) + k) as usize)),
                        (v as u8),
                    )
                });
                k.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < (*(*self).with(|__s| __s.sign_prob.clone()).borrow()).len()) {
            if (i
                < (kMaxAverageContext_82.with(|rc| *rc.borrow()))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
            {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset(i),
                        108_u8,
                    )
                });
            } else if (i
                < (((kMaxAverageContext_82.with(|rc| *rc.borrow())).wrapping_add(1_usize))
                    as usize)
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
            {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset(i),
                        128_u8,
                    )
                });
            } else {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset(i),
                        148_u8,
                    )
                });
            }
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i
            < (*(*self)
                .with(|__s| __s.first_extra_bit_prob.clone())
                .borrow())
            .len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.first_extra_bit_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset(i),
                    158_u8,
                )
            });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < kNumNonZeroContextCount_88.with(|rc| *rc.borrow())) {
            let mut non_zero_probs: Ptr<brunsli_Prob> =
                (array_field_ptr!((*self), num_nonzero_prob) as Ptr<brunsli_Prob>).offset(
                    ((i).wrapping_mul(kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))) as isize,
                );
            let mut j: usize = 0_usize;
            'loop_: while (j < kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
                ({
                    let _probability: u8 = ({
                        let __idx = (i) as usize;
                        kInitProbNonzero_111.with(|rc| rc.borrow()[__idx].clone())
                    })
                    .borrow()[(j) as usize];
                    brunsli_ProbImpl::Init(&non_zero_probs.offset((j) as isize), _probability)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
}
pub trait brunsli_ComponentStateDCImpl {
    fn SetWidth(&self, w: i32);
    fn InitAll(&self);
    fn destructor(&self);
}
impl brunsli_ComponentStateDCImpl for Ptr<brunsli_ComponentStateDC> {
    fn SetWidth(&self, mut w: i32) {
        field!((*self), width).write(w);
        {
            let __a0 = ((w + 1) as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_is_nonempty.clone())
                .borrow_mut())
            .resize(__a0, 1)
        };
        {
            let __a0 = ((w + 3) as usize) as usize;
            (*(*self).with(|__s| __s.prev_abs_coeff.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = ((w + 1) as usize) as usize;
            (*(*self).with(|__s| __s.prev_sign.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
    }
    fn destructor(&self) {
        field_ptr!(self, is_zero_prob).destructor();
    }
    fn InitAll(&self) {
        ({ brunsli_ProbImpl::Init(&field_ptr!((*self), is_zero_prob), 135_u8) });
        let mut i: usize = 0_usize;
        'loop_: while (i < (*(*self).with(|__s| __s.sign_prob.clone()).borrow()).len()) {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                        .offset(i),
                    128_u8,
                )
            });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < (*(*self).with(|__s| __s.is_empty_block_prob.clone()).borrow()).len()) {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.is_empty_block_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset(i),
                    74_u8,
                )
            });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i
            < (*(*self)
                .with(|__s| __s.first_extra_bit_prob.clone())
                .borrow())
            .len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.first_extra_bit_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset(i),
                    150_u8,
                )
            });
            i.prefix_inc();
        }
    }
}
pub trait brunsli_PermutationCoderImpl {
    fn Init(&self, values: Vec<u8>);
    fn Clear(&self);
    fn num_bits(&self) -> i32;
    fn Remove(&self, code: usize, value: Ptr<u8>) -> bool;
    fn RemoveValue(&self, value: u8, code: Ptr<i32>, nbits: Ptr<i32>) -> bool;
}
impl brunsli_PermutationCoderImpl for Ptr<brunsli_PermutationCoder> {
    fn Init(&self, values: Vec<u8>) {
        let values: Value<Vec<u8>> = Rc::new(RefCell::new(values));
        ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>)
            .write(std::mem::take(&mut (*values.borrow_mut())));
    }
    fn Clear(&self) {
        std::mem::swap(
            &mut Vec::new(),
            &mut (*(*self).with(|__s| __s.values_.clone()).borrow_mut()),
        );
    }
    fn num_bits(&self) -> i32 {
        let mut num_values: u32 =
            ((*(*self).with(|__s| __s.values_.clone()).borrow()).len() as u32);
        if !(num_values > 0_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"lehmer_code.cc"),
                    51,
                    Ptr::<i8>::from_string_literal(b"num_bits"),
                )
            });
            'loop_: while true {}
        };
        if (num_values <= 1_u32) {
            return 0;
        }
        return (({ Log2FloorNonZero_74((num_values).wrapping_sub(1_u32)) }) + 1);
    }
    fn Remove(&self, mut code: usize, mut value: Ptr<u8>) -> bool {
        if (code >= (*(*self).with(|__s| __s.values_.clone()).borrow()).len()) {
            return false;
        }
        let __rhs = (elem!(
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>),
            code
        )
        .read());
        value.write(__rhs);
        {
            let idx = ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                .offset((code as i64) as isize)
                .get_offset();
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>)
                .with_mut(|__v: &mut Vec<u8>| __v.remove(idx));
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>).decay()
        };
        return true;
    }
    fn RemoveValue(&self, mut value: u8, mut code: Ptr<i32>, mut nbits: Ptr<i32>) -> bool {
        let it: Value<Ptr<u8>> = Rc::new(RefCell::new({
            let count = (((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                .to_end()
                .get_offset()
                - ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>).get_offset())
                as usize;
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>).offset(
                ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                    .clone()
                    .into_iter()
                    .take(count)
                    .position(|value_0| value_0.read() == value)
                    .unwrap_or(count) as isize,
            )
        }));
        if (*it.borrow()) == ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>).to_end() {
            return false;
        }
        let __rhs = ((((*it.borrow()).get_offset() as isize)
            - (((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>).get_offset() as isize))
            as i32);
        code.write(__rhs);
        let __rhs = ({ brunsli_PermutationCoderImpl::num_bits(self) });
        nbits.write(__rhs);
        {
            let idx = (*it.borrow()).get_offset();
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>)
                .with_mut(|__v: &mut Vec<u8>| __v.remove(idx));
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>).decay()
        };
        return true;
    }
}
pub trait brunsli_ProbImpl {
    fn destructor(&self);
    fn Init(&self, probability: u8);
    fn Add(&self, val: i32);
    fn get_proba(&self) -> u8;
}
impl brunsli_ProbImpl for Ptr<brunsli_Prob> {
    fn destructor(&self) {}
    fn Init(&self, mut probability: u8) {
        field!((*self), prob8).write(probability);
        field!((*self), total).write(kInitProbCount_81.with(|rc| *rc.borrow()));
        field!((*self), count).write(
            (((kInitProbCount_81.with(|rc| *rc.borrow()) as i32) * (probability as i32)) as u16),
        );
    }
    fn Add(&self, mut val: i32) {
        field!((*self), total).with_mut(|__v| __v.prefix_inc());
        if (val == 0) {
            field!((*self), count).write({ (((*self).with(|__s| __s.count) as i32) + 256) as u16 });
        } else {
            field!((*self), count).with_mut(|__v| __v.prefix_inc());
        }
        field!((*self), prob8).write(
            ({
                let _numerator: u32 = ((*self).with(|__s| __s.count) as u32);
                let _denominator: u8 = (*self).with(|__s| __s.total);
                FastDivide_78(_numerator, _denominator)
            }),
        );
        if (((*self).with(|__s| __s.total) as i32)
            == (kNormalizeThreshold_76.with(|rc| *rc.borrow()) as i32))
        {
            field!((*self), count).write({ (((*self).with(|__s| __s.count) as i32) >> 1) as u16 });
            field!((*self), total)
                .write((((kNormalizeThreshold_76.with(|rc| *rc.borrow()) as i32) >> 1) as u8));
        }
    }
    fn get_proba(&self) -> u8 {
        return (*self).with(|__s| __s.prob8);
    }
}
pub trait brunsli_StorageImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn GetBytesUsed(&self) -> usize;
    fn AppendBytes(&self, mut src: Ptr<u8>, mut len: usize) {
        unimplemented!()
    }
}
impl brunsli_StorageImpl for Ptr<brunsli_Storage> {
    fn GetBytesUsed(&self) -> usize {
        return ((((*self).with(|__s| __s.pos)).wrapping_add(7_usize)) >> 3);
    }
    fn AppendBytes(&self, mut src: Ptr<u8>, mut len: usize) {
        if !(((*self).with(|__s| __s.pos) & 7_usize) == 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"write_bits.cc"),
                    19,
                    Ptr::<i8>::from_string_literal(b"AppendBytes"),
                )
            });
            'loop_: while true {}
        };
        if !(({ brunsli_StorageImpl::GetBytesUsed(self) }).wrapping_add(len)
            <= (*self).with(|__s| __s.length))
        {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"write_bits.cc"),
                    20,
                    Ptr::<i8>::from_string_literal(b"AppendBytes"),
                )
            });
            'loop_: while true {}
        };
        {
            ((*self)
                .with(|__s| __s.data.clone())
                .offset(((*self).with(|__s| __s.pos) >> 3) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(&(src).to_any(), len as usize);
            ((*self)
                .with(|__s| __s.data.clone())
                .offset(((*self).with(|__s| __s.pos) >> 3) as isize) as Ptr<u8>)
                .to_any()
        };
        field!((*self), pos)
            .write({ ((*self).with(|__s| __s.pos)).wrapping_add((8_usize).wrapping_mul(len)) });
    }
    fn destructor(&self) {
        if !(({ brunsli_StorageImpl::GetBytesUsed(self) }) <= (*self).with(|__s| __s.length)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"write_bits.cc"),
                    26,
                    Ptr::<i8>::from_string_literal(b"~Storage"),
                )
            });
            'loop_: while true {}
        };
    }
}
pub trait brunsli_internal_enc_DataStreamImpl {
    fn Resize(&self, max_num_code_words: usize);
    fn ResizeForBlock(&self);
    fn AddCode(
        &self,
        code: usize,
        band: usize,
        context: usize,
        s: Ptr<brunsli_internal_enc_EntropySource>,
    );
    fn AddBits(&self, nbits: i32, bits: i32);
    fn FlushArithmeticCoder(&self);
    fn FlushBitWriter(&self);
    fn AddBit(&self, p: Ptr<brunsli_Prob>, bit: i32);
    fn EncodeCodeWords(
        &self,
        s: Ptr<brunsli_internal_enc_EntropyCodes>,
        storage: Ptr<brunsli_Storage>,
    );
}
impl brunsli_internal_enc_DataStreamImpl for Ptr<brunsli_internal_enc_DataStream> {
    fn Resize(&self, mut max_num_code_words: usize) {
        {
            let __a0 = max_num_code_words as usize;
            (*(*self).with(|__s| __s.code_words_.clone()).borrow_mut()).resize_with(__a0, || {
                <brunsli_internal_enc_DataStream_CodeWord>::default()
            })
        };
    }
    fn ResizeForBlock(&self) {
        if (((*self).with(|__s| __s.pos_) as usize)
            .wrapping_add(kSlackForOneBlock_140.with(|rc| *rc.borrow()))
            > (*(*self).with(|__s| __s.code_words_.clone()).borrow()).len())
        {
            thread_local!(
                static kGrowMult_165: Value<f64> = Rc::new(RefCell::new(1.2E+0));
            );
            let mut new_size: usize = ((kGrowMult_165.with(|rc| *rc.borrow())
                * ((*(*self).with(|__s| __s.code_words_.clone()).borrow()).capacity() as f64))
                as usize)
                .wrapping_add(kSlackForOneBlock_140.with(|rc| *rc.borrow()));
            {
                let __a0 = new_size as usize;
                (*(*self).with(|__s| __s.code_words_.clone()).borrow_mut())
                    .resize_with(__a0, || {
                        <brunsli_internal_enc_DataStream_CodeWord>::default()
                    })
            };
        }
    }
    fn AddCode(
        &self,
        mut code: usize,
        mut band: usize,
        mut context: usize,
        mut s: Ptr<brunsli_internal_enc_EntropySource>,
    ) {
        let mut histo_ix: usize = ((band)
            .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
        .wrapping_add(context);
        let mut word: brunsli_internal_enc_DataStream_CodeWord =
            brunsli_internal_enc_DataStream_CodeWord::new();
        word.context = (histo_ix as u32);
        word.code = ((code as u32) as u8);
        word.nbits = 0_u8;
        word.value = 0_u16;
        if !(((*self).with(|__s| __s.pos_) as usize)
            < (*(*self).with(|__s| __s.code_words_.clone()).borrow()).len())
        {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    631,
                    Ptr::<i8>::from_string_literal(b"AddCode"),
                )
            });
            'loop_: while true {}
        };
        elem!(
            ((*self).with(|__s| __s.code_words_.as_pointer())
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
            (field!((*self), pos_).with_mut(|__v| __v.postfix_inc()) as usize)
        )
        .write((word).clone());
        ({ brunsli_internal_enc_EntropySourceImpl::AddCode(&s, code, histo_ix) });
    }
    fn AddBits(&self, mut nbits: i32, mut bits: i32) {
        {
            let __rhs = ((bits << (*self).with(|__s| __s.bw_bitpos_)) as u32);
            field!((*self), bw_val_).with_mut(|__v| *__v = *__v | __rhs)
        };
        {
            let __rhs = nbits;
            field!((*self), bw_bitpos_).with_mut(|__v| *__v = *__v + __rhs)
        };
        if ((*self).with(|__s| __s.bw_bitpos_) > 16) {
            let mut word: brunsli_internal_enc_DataStream_CodeWord =
                brunsli_internal_enc_DataStream_CodeWord::new();
            word.context = 0_u32;
            word.code = 0_u8;
            word.nbits = 16_u8;
            word.value = (((*self).with(|__s| __s.bw_val_) & 65535_u32) as u16);
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.bw_pos_) as usize)
            )
            .write((word).clone());
            field!((*self), bw_pos_).write((*self).with(|__s| __s.pos_));
            field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            {
                field!((*self), bw_val_).with_mut(|__v| *__v = *__v >> 16)
            };
            {
                field!((*self), bw_bitpos_).with_mut(|__v| *__v = *__v - 16)
            };
        }
    }
    fn FlushArithmeticCoder(&self) {
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.ac_pos0_) as usize)
            ),
            value
        )
        .write((((*self).with(|__s| __s.high_) >> 16) as u16));
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.ac_pos1_) as usize)
            ),
            value
        )
        .write((((*self).with(|__s| __s.high_) & 65535_u32) as u16));
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.ac_pos0_) as usize)
            ),
            nbits
        )
        .write(16_u8);
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.ac_pos1_) as usize)
            ),
            nbits
        )
        .write(16_u8);
        field!((*self), low_).write(0_u32);
        field!((*self), high_).write((!0 as u32));
    }
    fn FlushBitWriter(&self) {
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.bw_pos_) as usize)
            ),
            nbits
        )
        .write(16_u8);
        field!(
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.bw_pos_) as usize)
            ),
            value
        )
        .write((((*self).with(|__s| __s.bw_val_) & 65535_u32) as u16));
    }
    fn AddBit(&self, mut p: Ptr<brunsli_Prob>, mut bit: i32) {
        let mut prob: u8 = ({ brunsli_ProbImpl::get_proba(&p) });
        ({ brunsli_ProbImpl::Add(&p, bit) });
        let mut diff: u32 =
            ((*self).with(|__s| __s.high_)).wrapping_sub((*self).with(|__s| __s.low_));
        let mut split: u32 = ((((*self).with(|__s| __s.low_) as u64)
            .wrapping_add((((diff as u64).wrapping_mul((prob as u64))) >> 8)))
            as u32);
        if (bit != 0) {
            field!((*self), low_).write((split).wrapping_add(1_u32));
        } else {
            field!((*self), high_).write(split);
        }
        if ((((*self).with(|__s| __s.low_) ^ (*self).with(|__s| __s.high_)) >> 16) == 0_u32) {
            field!(
                elem!(
                    ((*self).with(|__s| __s.code_words_.as_pointer())
                        as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                    ((*self).with(|__s| __s.ac_pos0_) as usize)
                ),
                value
            )
            .write((((*self).with(|__s| __s.high_) >> 16) as u16));
            field!(
                elem!(
                    ((*self).with(|__s| __s.code_words_.as_pointer())
                        as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                    ((*self).with(|__s| __s.ac_pos0_) as usize)
                ),
                nbits
            )
            .write(16_u8);
            field!((*self), ac_pos0_).write((*self).with(|__s| __s.ac_pos1_));
            field!((*self), ac_pos1_).write((*self).with(|__s| __s.pos_));
            field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            {
                field!((*self), low_).with_mut(|__v| *__v = *__v << 16)
            };
            {
                field!((*self), high_).with_mut(|__v| *__v = *__v << 16)
            };
            {
                field!((*self), high_).with_mut(|__v| *__v = *__v | 65535_u32)
            };
        }
    }
    fn EncodeCodeWords(
        &self,
        mut s: Ptr<brunsli_internal_enc_EntropyCodes>,
        mut storage: Ptr<brunsli_Storage>,
    ) {
        ({ brunsli_internal_enc_DataStreamImpl::FlushBitWriter(self) });
        ({ brunsli_internal_enc_DataStreamImpl::FlushArithmeticCoder(self) });
        let ans: Value<brunsli_ANSCoder> = Rc::new(RefCell::new(brunsli_ANSCoder::new()));
        let mut i: i32 = ((*self).with(|__s| __s.pos_) - 1);
        'loop_: while (i >= 0) {
            let mut word: Ptr<brunsli_internal_enc_DataStream_CodeWord> = (((*self)
                .with(|__s| __s.code_words_.as_pointer())
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset((i as usize)));
            if ((word.with(|__s| __s.nbits) as i32) == 0) {
                let mut info: brunsli_ANSEncSymbolInfo = (*elem!(
                    (array_field_ptr!(
                        ({
                            brunsli_internal_enc_EntropyCodesImpl::GetANSTable(
                                &s,
                                (word.with(|__s| __s.context) as i32),
                            )
                        }),
                        info_
                    ) as Ptr<brunsli_ANSEncSymbolInfo>),
                    word.with(|__s| __s.code)
                )
                .upgrade()
                .deref())
                .clone();
                let __rhs = (({
                    brunsli_ANSCoderImpl::PutSymbol(
                        &ans.as_pointer(),
                        (info).clone(),
                        (field_ptr!(word, nbits)),
                    )
                }) as u16);
                field!(word, value).write(__rhs);
            }
            i.prefix_dec();
        }
        let mut state: u32 = ({ brunsli_ANSCoderImpl::GetState(&ans.as_pointer()) });
        let mut out: Ptr<u16> = storage
            .with(|__s| __s.data.clone())
            .reinterpret_cast::<u16>();
        let mut out_start: Ptr<u16> = (out).clone();
        ({
            let _p: AnyPtr = (out.postfix_inc() as Ptr<u16>).to_any();
            let _v: u16 = ((state >> 16) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        ({
            let _p: AnyPtr = (out.postfix_inc() as Ptr<u16>).to_any();
            let _v: u16 = (state as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        let mut i: i32 = 0;
        'loop_: while (i < (*self).with(|__s| __s.pos_)) {
            let word: Ptr<brunsli_internal_enc_DataStream_CodeWord> = ((*self)
                .with(|__s| __s.code_words_.as_pointer())
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset((i as usize));
            if (word.with(|__s| __s.nbits) != 0) {
                ({
                    let _p: AnyPtr = (out.postfix_inc() as Ptr<u16>).to_any();
                    let _v: u16 = word.with(|__s| __s.value);
                    BrunsliUnalignedWrite16_67(_p, _v)
                });
            }
            i.prefix_inc();
        }
        field!(storage, pos).write({
            (storage.with(|__s| __s.pos))
                .wrapping_add((((((out).clone() - (out_start).clone()) as i64) * 16_i64) as usize))
        });
    }
}
pub trait brunsli_internal_enc_EntropyCodesImpl {
    fn EncodeContextMap(&self, storage: Ptr<brunsli_Storage>);
    fn BuildAndStoreEntropyCodes(&self, storage: Ptr<brunsli_Storage>);
    fn GetANSTable(&self, context: i32) -> Ptr<brunsli_ANSTable>;
}
impl brunsli_internal_enc_EntropyCodesImpl for Ptr<brunsli_internal_enc_EntropyCodes> {
    fn EncodeContextMap(&self, mut storage: Ptr<brunsli_Storage>) {
        ({
            let _context_map: Ptr<Vec<u32>> = (*self).with(|__s| __s.context_map_.as_pointer());
            let _num_clusters: usize = (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len();
            EncodeContextMap_164(_context_map, _num_clusters, (storage).clone())
        });
    }
    fn BuildAndStoreEntropyCodes(&self, mut storage: Ptr<brunsli_Storage>) {
        {
            let __a0 = (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len() as usize;
            (*(*self).with(|__s| __s.ans_tables_.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_ANSTable>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len()) {
            ({
                let _histogram: Ptr<i32> = ((array_field_ptr!(
                    ((*self).with(|__s| __s.clustered_.as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(i),
                    data_
                ) as Ptr<i32>)
                    .offset((0) as isize));
                let _table: Ptr<brunsli_ANSTable> =
                    (((*self).with(|__s| __s.ans_tables_.as_pointer()) as Ptr<brunsli_ANSTable>)
                        .offset(i));
                BuildAndStoreANSEncodingData_123(_histogram, _table, (storage).clone())
            });
            i.prefix_inc();
        }
    }
    fn GetANSTable(&self, mut context: i32) -> Ptr<brunsli_ANSTable> {
        let mut entropy_ix: i32 = ((elem!(
            ((*self).with(|__s| __s.context_map_.as_pointer()) as Ptr<u32>),
            (context as usize)
        )
        .read()) as i32);
        return (((*self).with(|__s| __s.ans_tables_.as_pointer()) as Ptr<brunsli_ANSTable>)
            .offset((entropy_ix as usize)));
    }
}
pub trait brunsli_internal_enc_EntropySourceImpl {
    fn Resize(&self, num_bands: usize);
    fn AddCode(&self, code: usize, histo_ix: usize);
    fn Merge(&self, other: Ptr<brunsli_internal_enc_EntropySource>);
    fn Finish(&self, offsets: Ptr<Vec<u64>>) -> Option<Value<brunsli_internal_enc_EntropyCodes>>;
}
impl brunsli_internal_enc_EntropySourceImpl for Ptr<brunsli_internal_enc_EntropySource> {
    fn Resize(&self, mut num_bands: usize) {
        field!((*self), num_bands_).write(num_bands);
        {
            let __a0 =
                (num_bands).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())) as usize;
            (*(*self).with(|__s| __s.histograms_.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        };
    }
    fn AddCode(&self, mut code: usize, mut histo_ix: usize) {
        ({
            brunsli_internal_enc_HistogramImpl::Add(
                &((*self).with(|__s| __s.histograms_.as_pointer())
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(histo_ix),
                code,
            )
        });
    }
    fn Finish(&self, offsets: Ptr<Vec<u64>>) -> Option<Value<brunsli_internal_enc_EntropyCodes>> {
        let histograms: Value<Vec<brunsli_internal_enc_Histogram>> =
            Rc::new(RefCell::new(Vec::new()));
        std::mem::swap(
            &mut (*histograms.borrow_mut()),
            &mut (*(*self).with(|__s| __s.histograms_.clone()).borrow_mut()),
        );
        return Ptr::alloc(brunsli_internal_enc_EntropyCodes::new(
            { histograms.as_pointer() },
            { (*self).with(|__s| __s.num_bands_) },
            { (offsets).clone() },
        ))
        .to_owned_opt()
        .take();
    }
    fn Merge(&self, other: Ptr<brunsli_internal_enc_EntropySource>) {
        if !({ (*(*self).with(|__s| __s.histograms_.clone()).borrow()).len() } >= {
            (*other.with(|__s| __s.histograms_.clone()).borrow()).len()
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    568,
                    Ptr::<i8>::from_string_literal(b"Merge"),
                )
            });
            'loop_: while true {}
        };
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*other.with(|__s| __s.histograms_.clone()).borrow()).len() }) {
            ({
                let _other: Ptr<brunsli_internal_enc_Histogram> = (other
                    .with(|__s| __s.histograms_.as_pointer())
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(i);
                brunsli_internal_enc_HistogramImpl::Merge(
                    &((*self).with(|__s| __s.histograms_.as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(i),
                    _other,
                )
            });
            i.prefix_inc();
        }
    }
}
pub trait brunsli_internal_enc_HistogramImpl {
    fn Clear(&self);
    fn AddHistogram(&self, other: Ptr<brunsli_internal_enc_Histogram>);
    fn Add(&self, val: usize);
    fn Merge(&self, other: Ptr<brunsli_internal_enc_Histogram>);
}
impl brunsli_internal_enc_HistogramImpl for Ptr<brunsli_internal_enc_Histogram> {
    fn Clear(&self) {
        {
            ((array_field_ptr!((*self), data_) as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i32; 18]>() as usize);
            ((array_field_ptr!((*self), data_) as Ptr<i32>) as Ptr<i32>).to_any()
        };
        field!((*self), total_count_).write(0);
    }
    fn AddHistogram(&self, other: Ptr<brunsli_internal_enc_Histogram>) {
        let mut i: i32 = 0;
        'loop_: while (i < 18) {
            {
                let __rhs = { (elem!((array_field_ptr!(other, data_) as Ptr::<i32>), i).read()) };
                elem!((array_field_ptr!((*self), data_) as Ptr::<i32>), i)
                    .with_mut(|__v| *__v = *__v + __rhs)
            };
            i.prefix_inc();
        }
        {
            let __rhs = { other.with(|__s| __s.total_count_) };
            field!((*self), total_count_).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn Add(&self, mut val: usize) {
        if !(val < 18_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    522,
                    Ptr::<i8>::from_string_literal(b"Add"),
                )
            });
            'loop_: while true {}
        };
        elem!((array_field_ptr!((*self), data_) as Ptr::<i32>), val)
            .with_mut(|__v| __v.prefix_inc());
        field!((*self), total_count_).with_mut(|__v| __v.prefix_inc());
    }
    fn Merge(&self, other: Ptr<brunsli_internal_enc_Histogram>) {
        if (other.with(|__s| __s.total_count_) == 0) {
            return;
        }
        {
            let __rhs = { other.with(|__s| __s.total_count_) };
            field!((*self), total_count_).with_mut(|__v| *__v = *__v + __rhs)
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < 18_usize) {
            {
                let __rhs = { (elem!((array_field_ptr!(other, data_) as Ptr::<i32>), i).read()) };
                elem!((array_field_ptr!((*self), data_) as Ptr::<i32>), i)
                    .with_mut(|__v| *__v = *__v + __rhs)
            };
            i.prefix_inc();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|_| ());
    let _ = BRUNSLI_ANS_TAB_SIZE_1.with(|_| ());
    let _ = kFallbackVersion_2.with(|_| ());
    let _ = kDCTBlockSize_3.with(|_| ());
    let _ = kMaxComponents_4.with(|_| ());
    let _ = kMaxQuantTables_5.with(|_| ());
    let _ = kMaxHuffmanTables_6.with(|_| ());
    let _ = kJpegHuffmanMaxBitLength_7.with(|_| ());
    let _ = kJpegHuffmanAlphabetSize_8.with(|_| ());
    let _ = kJpegDCAlphabetSize_9.with(|_| ());
    let _ = kMaxDHTMarkers_10.with(|_| ());
    let _ = kMaxDimPixels_11.with(|_| ());
    let _ = kDefaultQuantMatrix_12.with(|_| ());
    let _ = kJPEGNaturalOrder_13.with(|_| ());
    let _ = kJPEGZigZagOrder_14.with(|_| ());
    let _ = kBrunsliMaxNumBlocks_18.with(|_| ());
    let _ = kBrunsliMaxDCAbsVal_19.with(|_| ());
    let _ = kMaxContextMapAlphabetSize_20.with(|_| ());
    let _ = kHuffmanTableBits_21.with(|_| ());
    let _ = kMaxHuffmanBits_22.with(|_| ());
    let _ = kBrunsliShortMarkerLimit_23.with(|_| ());
    let _ = kBrunsliMultibyteMarkerLimit_24.with(|_| ());
    let _ = kBrunsliWiringTypeVarint_25.with(|_| ());
    let _ = kBrunsliWiringTypeLengthDelimited_26.with(|_| ());
    let _ = kBrunsliMaxSampling_27.with(|_| ());
    let _ = kBrunsliSignatureTag_30.with(|_| ());
    let _ = kBrunsliHeaderTag_31.with(|_| ());
    let _ = kBrunsliMetaDataTag_32.with(|_| ());
    let _ = kBrunsliJPEGInternalsTag_33.with(|_| ());
    let _ = kBrunsliQuantDataTag_34.with(|_| ());
    let _ = kBrunsliHistogramDataTag_35.with(|_| ());
    let _ = kBrunsliDCDataTag_36.with(|_| ());
    let _ = kBrunsliACDataTag_37.with(|_| ());
    let _ = kBrunsliOriginalJpgTag_38.with(|_| ());
    let _ = kBrunsliHeaderWidthTag_39.with(|_| ());
    let _ = kBrunsliHeaderHeightTag_40.with(|_| ());
    let _ = kBrunsliHeaderVersionCompTag_41.with(|_| ());
    let _ = kBrunsliHeaderSubsamplingTag_42.with(|_| ());
    let _ = kBrunsliSignatureSize_43.with(|_| ());
    let _ = kMaxApp0Densities_45.with(|_| ());
    let _ = kApp0Densities_46.with(|_| ());
    let _ = kNumStockQuantTables_47.with(|_| ());
    let _ = kStockQuantizationTables_48.with(|_| ());
    let _ = kComponentIds123_49.with(|_| ());
    let _ = kComponentIdsGray_50.with(|_| ());
    let _ = kComponentIdsRGB_51.with(|_| ());
    let _ = kComponentIdsCustom_52.with(|_| ());
    let _ = kNumStockDCHuffmanCodes_53.with(|_| ());
    let _ = kStockDCHuffmanCodeCounts_54.with(|_| ());
    let _ = kStockDCHuffmanCodeValues_55.with(|_| ());
    let _ = kNumStockACHuffmanCodes_56.with(|_| ());
    let _ = kStockACHuffmanCodeCounts_57.with(|_| ());
    let _ = kStockACHuffmanCodeTotalCount_58.with(|_| ());
    let _ = kStockACHuffmanCodeValues_59.with(|_| ());
    let _ = kDefaultDCValues_60.with(|_| ());
    let _ = kDefaultACValues_61.with(|_| ());
    let _ = kBrunsliSignature_44.with(|_| ());
    let _ = AppData_0xe0_62.with(|_| ());
    let _ = AppData_0xec_64.with(|_| ());
    let _ = AppData_0xee_65.with(|_| ());
    let _ = AppData_0xe2_63.with(|_| ());
    let _ = kNormalizeThreshold_76.with(|_| ());
    let _ = kDivLut17_77.with(|_| ());
    let _ = kInitProb_80.with(|_| ());
    let _ = kInitProbCount_81.with(|_| ());
    let _ = kMaxAverageContext_82.with(|_| ());
    let _ = kNumAvrgContexts_83.with(|_| ());
    let _ = kNumNonZeroBits_84.with(|_| ());
    let _ = kNumNonZeroTreeSize_85.with(|_| ());
    let _ = kNumNonZeroQuant_86.with(|_| ());
    let _ = kNumNonZeroContextMax_87.with(|_| ());
    let _ = kNumNonZeroContextCount_88.with(|_| ());
    let _ = kNonzeroBuckets_89.with(|_| ());
    let _ = kNumNonzeroBuckets_90.with(|_| ());
    let _ = kNumSchemes_91.with(|_| ());
    let _ = kFreqContext_92.with(|_| ());
    let _ = kNumNonzeroContext_93.with(|_| ());
    let _ = kNumNonzeroContextSkip_94.with(|_| ());
    let _ = kContextAlgorithm_95.with(|_| ());
    let _ = kACPredictPrecisionBits_99.with(|_| ());
    let _ = kACPredictPrecision_100.with(|_| ());
    let _ = kNumIsEmptyBlockContexts_105.with(|_| ());
    let _ = kSqrt2_107.with(|_| ());
    let _ = kSqrt2FixedPoint_108.with(|_| ());
    let _ = kInitProb_110.with(|_| ());
    let _ = kInitProbNonzero_111.with(|_| ());
    let _ = kQFactorBits_116.with(|_| ());
    let _ = kQFactorLimit_117.with(|_| ());
    let _ = kMaxNumSymbolsForSmallCode_121.with(|_| ());
    let _ = kLog2Table_126.with(|_| ());
    let _ = kMaxNumberOfHistograms_139.with(|_| ());
    let _ = kSlackForOneBlock_140.with(|_| ());
    let _ = kNumDirectCodes_141.with(|_| ());
    let _ = kBrotliQuality_142.with(|_| ());
    let _ = kBrotliWindowBits_143.with(|_| ());
    let _ = kMaxBypassHeaderSize_192.with(|_| ());
    let _ = kHistogramLengthBitLengths_204.with(|_| ());
    let _ = kHistogramLengthSymbols_205.with(|_| ());
    let _ = kLogCountBitLengths_206.with(|_| ());
    let _ = kLogCountSymbols_207.with(|_| ());
    let _ = kCodeLengthCodes_211.with(|_| ());
    let _ = kJpegHuffmanRootTableBits_230.with(|_| ());
    let _ = kJpegHuffmanLutSize_231.with(|_| ());
}
