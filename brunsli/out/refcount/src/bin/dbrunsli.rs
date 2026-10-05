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
        elem!((items.as_pointer() as Ptr<u32>), i).write(__rhs);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let it: Value<Ptr<u32>> = Rc::new(RefCell::new(
            (items.as_pointer() as Ptr<u32>).offset(
                (items.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize
                            && value_0.read() == (elem!(sigma, i).read())
                    })
                    .unwrap_or((items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize)
                    as isize,
            ),
        ));
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
        elem!((items.as_pointer() as Ptr<u32>), i).write(__rhs);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        let mut index: u32 = (elem!(code, i).read());
        if ((index as usize) >= (*items.borrow()).len()) {
            return false;
        }
        let mut value: u32 = (elem!((items.as_pointer() as Ptr<u32>), (index as usize)).read());
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
thread_local!(
    pub static kBitMask_120: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535,
    ])));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct brunsli_WordSource {
    #[offset(0)]
    #[byte_size(8)]
    pub data_: Ptr<u8>,
    #[offset(8)]
    pub len_: usize,
    #[offset(16)]
    pub pos_: usize,
    #[offset(24)]
    pub error_: bool,
    #[offset(25)]
    pub optimistic_: bool,
}
impl brunsli_WordSource {
    pub fn new(mut data: Ptr<u8>, mut len: usize, mut optimistic: bool) -> Self {
        Self {
            data_: (data).clone(),
            len_: (len & (!1 as usize)),
            pos_: 0_usize,
            error_: false,
            optimistic_: optimistic,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct brunsli_BitSource {
    #[offset(0)]
    pub val_: u32,
    #[offset(4)]
    pub bit_pos_: i32,
}
impl brunsli_BitSource {
    pub fn new() -> Self {
        Self {
            val_: 0_u32,
            bit_pos_: 0_i32,
        }
    }
}
impl Default for brunsli_BitSource {
    fn default() -> Self {
        { brunsli_BitSource::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(6)]
pub struct brunsli_ANSSymbolInfo {
    #[offset(0)]
    pub offset_: u16,
    #[offset(2)]
    pub freq_: u16,
    #[offset(4)]
    pub symbol_: u8,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6144)]
pub struct brunsli_ANSDecodingData {
    #[offset(0)]
    #[byte_size(6144)]
    pub map_: Value<Box<[brunsli_ANSSymbolInfo]>>,
}
impl brunsli_ANSDecodingData {
    pub fn new() -> Self {
        Self {
            map_: Rc::new(RefCell::new(
                (0..1024)
                    .map(|_| <brunsli_ANSSymbolInfo>::default())
                    .collect::<Box<[brunsli_ANSSymbolInfo]>>(),
            )),
        }
    }
}
impl Default for brunsli_ANSDecodingData {
    fn default() -> Self {
        { brunsli_ANSDecodingData::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct brunsli_ANSDecoder {
    #[offset(0)]
    state_: u32,
}
impl brunsli_ANSDecoder {
    pub fn new() -> Self {
        Self { state_: 0_u32 }
    }
}
impl Default for brunsli_ANSDecoder {
    fn default() -> Self {
        { brunsli_ANSDecoder::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct brunsli_BrunsliBitReader {
    #[offset(0)]
    #[byte_size(8)]
    pub next_: Ptr<u8>,
    #[offset(8)]
    #[byte_size(8)]
    pub end_: Ptr<u8>,
    #[offset(16)]
    pub num_bits_: u32,
    #[offset(20)]
    pub bits_: u32,
    #[offset(24)]
    pub num_debt_bytes_: u32,
    #[offset(28)]
    pub is_healthy_: bool,
    #[offset(29)]
    pub is_optimistic_: bool,
}
pub fn BrunsliBitReaderBitMask_121(mut n: u32) -> u32 {
    return !((4294967295_u32) << n);
}
pub fn BrunsliBitReaderOweByte_122(mut br: Ptr<brunsli_BrunsliBitReader>) {
    field!(br, num_bits_).write({ (br.with(|__s| __s.num_bits_)).wrapping_add(8_u32) });
    field!(br, num_debt_bytes_).with_mut(|__v| __v.postfix_inc());
}
pub fn BrunsliBitReaderMaybeFetchByte_123(mut br: Ptr<brunsli_BrunsliBitReader>, mut n_bits: u32) {
    if ({ br.with(|__s| __s.num_bits_) } < { n_bits }) {
        if ((({ br.with(|__s| __s.next_.clone()) } >= { br.with(|__s| __s.end_.clone()) }) as i64)
            != 0)
        {
            ({ BrunsliBitReaderOweByte_122((br).clone()) });
        } else {
            {
                let __rhs = {
                    ({ ((br.with(|__s| __s.next_.clone()).read()) as u32) } << {
                        br.with(|__s| __s.num_bits_)
                    })
                };
                field!(br, bits_).with_mut(|__v| *__v = *__v | __rhs)
            };
            field!(br, num_bits_).write({ (br.with(|__s| __s.num_bits_)).wrapping_add(8_u32) });
            field!(br, next_).with_mut(|__v| __v.postfix_inc());
        }
    }
}
pub fn BrunsliBitReaderGet_124(mut br: Ptr<brunsli_BrunsliBitReader>, mut n_bits: u32) -> u32 {
    if !(n_bits <= 24_u32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"bit_reader.cc"),
                110,
                Ptr::<i8>::from_string_literal(b"BrunsliBitReaderGet"),
            )
        });
        'loop_: while true {}
    };
    ({ BrunsliBitReaderMaybeFetchByte_123((br).clone(), n_bits) });
    if (n_bits > 8_u32) {
        ({ BrunsliBitReaderMaybeFetchByte_123((br).clone(), n_bits) });
        if (n_bits > 16_u32) {
            ({ BrunsliBitReaderMaybeFetchByte_123((br).clone(), n_bits) });
        }
    }
    return ({ br.with(|__s| __s.bits_) } & { ({ BrunsliBitReaderBitMask_121(n_bits) }) });
}
pub fn BrunsliBitReaderDrop_125(mut br: Ptr<brunsli_BrunsliBitReader>, mut n_bits: u32) {
    if !({ n_bits } <= { br.with(|__s| __s.num_bits_) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"bit_reader.cc"),
                121,
                Ptr::<i8>::from_string_literal(b"BrunsliBitReaderDrop"),
            )
        });
        'loop_: while true {}
    };
    {
        let __rhs = n_bits;
        field!(br, bits_).with_mut(|__v| *__v = *__v >> __rhs)
    };
    field!(br, num_bits_).write({ (br.with(|__s| __s.num_bits_)).wrapping_sub(n_bits) });
}
pub fn BrunsliBitReaderRead_126(mut br: Ptr<brunsli_BrunsliBitReader>, mut n_bits: u32) -> u32 {
    let mut result: u32 = ({ BrunsliBitReaderGet_124((br).clone(), n_bits) });
    ({ BrunsliBitReaderDrop_125((br).clone(), n_bits) });
    return result;
}
pub fn BrunsliBitReaderInit_127(mut br: Ptr<brunsli_BrunsliBitReader>) {
    field!(br, num_bits_).write(0_u32);
    field!(br, bits_).write(0_u32);
    field!(br, num_debt_bytes_).write(0_u32);
    field!(br, is_healthy_).write(true);
    field!(br, is_optimistic_).write(false);
}
pub fn BrunsliBitReaderResume_128(
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut buffer: Ptr<u8>,
    mut length: usize,
) {
    field!(br, next_).write((buffer).clone());
    field!(br, end_).write(buffer.offset((length) as isize));
    field!(br, is_optimistic_).write(false);
}
pub fn BrunsliBitReaderUnload_129(mut br: Ptr<brunsli_BrunsliBitReader>) {
    'loop_: while (br.with(|__s| __s.num_debt_bytes_) > 0_u32)
        && (br.with(|__s| __s.num_bits_) >= 8_u32)
    {
        field!(br, num_debt_bytes_).with_mut(|__v| __v.postfix_dec());
        field!(br, num_bits_).write({ (br.with(|__s| __s.num_bits_)).wrapping_sub(8_u32) });
    }
    'loop_: while (br.with(|__s| __s.num_bits_) >= 8_u32) {
        field!(br, next_).with_mut(|__v| __v.postfix_dec());
        field!(br, num_bits_).write({ (br.with(|__s| __s.num_bits_)).wrapping_sub(8_u32) });
    }
    let __rhs = ({ BrunsliBitReaderBitMask_121(br.with(|__s| __s.num_bits_)) });
    {
        field!(br, bits_).with_mut(|__v| *__v = *__v & __rhs)
    };
}
pub fn BrunsliBitReaderSuspend_130(mut br: Ptr<brunsli_BrunsliBitReader>) -> usize {
    ({ BrunsliBitReaderUnload_129((br).clone()) });
    let mut unused_bytes: usize =
        (((br.with(|__s| __s.end_.clone()) - br.with(|__s| __s.next_.clone())) as i64) as usize);
    field!(br, next_).write(Ptr::<u8>::null());
    field!(br, end_).write(Ptr::<u8>::null());
    return unused_bytes;
}
pub fn BrunsliBitReaderFinish_131(mut br: Ptr<brunsli_BrunsliBitReader>) {
    let mut n_bits: u32 = br.with(|__s| __s.num_bits_);
    if (n_bits >= 8_u32) {
        field!(br, is_healthy_).write(false);
        return;
    }
    if (n_bits > 0_u32) {
        let mut padding_bits: u32 = ({ BrunsliBitReaderRead_126((br).clone(), n_bits) });
        if (padding_bits != 0_u32) {
            field!(br, is_healthy_).write(false);
        }
    }
}
pub fn BrunsliBitReaderIsHealthy_132(mut br: Ptr<brunsli_BrunsliBitReader>) -> bool {
    ({ BrunsliBitReaderUnload_129((br).clone()) });
    return (br.with(|__s| __s.num_debt_bytes_) == 0_u32) && (br.with(|__s| __s.is_healthy_));
}
pub fn BrunsliBitReaderSetOptimistic_133(mut br: Ptr<brunsli_BrunsliBitReader>) {
    field!(br, is_optimistic_).write(true);
}
pub fn BrunsliBitReaderCanRead_134(
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut n_bits: usize,
) -> bool {
    if br.with(|__s| __s.is_optimistic_) {
        return true;
    }
    if (br.with(|__s| __s.num_debt_bytes_) != 0_u32) {
        return false;
    }
    if ({ (br.with(|__s| __s.num_bits_) as usize) } >= { n_bits }) {
        return true;
    }
    let mut num_extra_bytes: usize =
        ((((n_bits).wrapping_sub((br.with(|__s| __s.num_bits_) as usize))).wrapping_add(7_usize))
            >> 3);
    return ({
        br.with(|__s| __s.next_.clone())
            .offset((num_extra_bytes) as isize)
    } <= { br.with(|__s| __s.end_.clone()) });
}
pub type brunsli_BrunsliStatus = u32;
pub const brunsli_BrunsliStatus_BRUNSLI_OK: brunsli_BrunsliStatus = 0;
pub const brunsli_BrunsliStatus_BRUNSLI_NON_REPRESENTABLE: brunsli_BrunsliStatus = 1;
pub const brunsli_BrunsliStatus_BRUNSLI_MEMORY_ERROR: brunsli_BrunsliStatus = 2;
pub const brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM: brunsli_BrunsliStatus = 3;
pub const brunsli_BrunsliStatus_BRUNSLI_COMPRESSION_ERROR: brunsli_BrunsliStatus = 4;
pub const brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN: brunsli_BrunsliStatus = 5;
pub const brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR: brunsli_BrunsliStatus = 6;
pub const brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA: brunsli_BrunsliStatus = 7;
pub type brunsli_BrunsliDecoder_Status = u32;
pub const brunsli_BrunsliDecoder_Status_NEEDS_MORE_INPUT: brunsli_BrunsliDecoder_Status = 0;
pub const brunsli_BrunsliDecoder_Status_NEEDS_MORE_OUTPUT: brunsli_BrunsliDecoder_Status = 1;
pub const brunsli_BrunsliDecoder_Status_ERROR: brunsli_BrunsliDecoder_Status = 2;
pub const brunsli_BrunsliDecoder_Status_DONE: brunsli_BrunsliDecoder_Status = 3;
#[derive(Record, ByteRepr)]
#[byte_size(16)]
pub struct brunsli_BrunsliDecoder {
    #[offset(0)]
    #[byte_size(8)]
    jpg_: Option<Value<brunsli_JPEGData>>,
    #[offset(8)]
    #[byte_size(8)]
    state_: Option<Value<brunsli_internal_dec_State>>,
}
impl brunsli_BrunsliDecoder {
    pub fn new() -> Self {
        let __this: Value<brunsli_BrunsliDecoder> = Rc::new(RefCell::new(Self {
            jpg_: None,
            state_: None,
        }));
        let this: Ptr<brunsli_BrunsliDecoder> = __this.as_pointer();
        {
            let _p: Ptr<_> = Ptr::alloc(brunsli_JPEGData::new());
            (field_ptr!(this, jpg_) as Ptr<Option<Value<brunsli_JPEGData>>>)
                .write(_p.to_owned_opt())
        };
        {
            let _p: Ptr<_> = Ptr::alloc(brunsli_internal_dec_State::new());
            (field_ptr!(this, state_) as Ptr<Option<Value<brunsli_internal_dec_State>>>)
                .write(_p.to_owned_opt())
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for brunsli_BrunsliDecoder {
    fn default() -> Self {
        { brunsli_BrunsliDecoder::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct brunsli_BinaryArithmeticDecoder {
    #[offset(0)]
    low_: u32,
    #[offset(4)]
    high_: u32,
    #[offset(8)]
    value_: u32,
}
impl brunsli_BinaryArithmeticDecoder {
    pub fn new() -> Self {
        Self {
            low_: 0_u32,
            high_: 0_u32,
            value_: 0_u32,
        }
    }
}
impl Default for brunsli_BinaryArithmeticDecoder {
    fn default() -> Self {
        { brunsli_BinaryArithmeticDecoder::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct brunsli_HuffmanCode {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct brunsli_JPEGOutput {
    #[offset(0)]
    #[byte_size(8)]
    cb: FnPtr<fn(AnyPtr, Ptr<u8>, usize) -> usize>,
    #[offset(8)]
    #[byte_size(8)]
    data: AnyPtr,
}
impl brunsli_JPEGOutput {
    pub fn new(mut cb: FnPtr<fn(AnyPtr, Ptr<u8>, usize) -> usize>, mut data: AnyPtr) -> Self {
        Self {
            cb: (cb).clone(),
            data: (data).clone(),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(312)]
pub struct brunsli_internal_dec_ComponentMeta {
    #[offset(0)]
    pub context_offset: usize,
    #[offset(8)]
    pub h_samp: i32,
    #[offset(12)]
    pub v_samp: i32,
    #[offset(16)]
    pub context_bits: usize,
    #[offset(24)]
    pub ac_stride: i32,
    #[offset(28)]
    pub b_stride: i32,
    #[offset(32)]
    pub width_in_blocks: i32,
    #[offset(36)]
    pub height_in_blocks: i32,
    #[offset(40)]
    #[byte_size(8)]
    pub ac_coeffs: Ptr<i16>,
    #[offset(48)]
    #[byte_size(8)]
    pub block_state: Ptr<u8>,
    #[offset(56)]
    #[byte_size(256)]
    pub quant: Value<Vec<i32>>,
}
impl Default for brunsli_internal_dec_ComponentMeta {
    fn default() -> Self {
        brunsli_internal_dec_ComponentMeta {
            context_offset: 0_usize,
            h_samp: 0_i32,
            v_samp: 0_i32,
            context_bits: 0_usize,
            ac_stride: 0_i32,
            b_stride: 0_i32,
            width_in_blocks: 0_i32,
            height_in_blocks: 0_i32,
            ac_coeffs: Ptr::<i16>::null(),
            block_state: Ptr::<u8>::null(),
            quant: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
pub type brunsli_internal_dec_Stage = i32;
pub const brunsli_internal_dec_Stage_SIGNATURE: brunsli_internal_dec_Stage = 0;
pub const brunsli_internal_dec_Stage_HEADER: brunsli_internal_dec_Stage = 1;
pub const brunsli_internal_dec_Stage_FALLBACK: brunsli_internal_dec_Stage = 2;
pub const brunsli_internal_dec_Stage_SECTION: brunsli_internal_dec_Stage = 3;
pub const brunsli_internal_dec_Stage_SECTION_BODY: brunsli_internal_dec_Stage = 4;
pub const brunsli_internal_dec_Stage_DONE: brunsli_internal_dec_Stage = 5;
pub const brunsli_internal_dec_Stage_ERROR: brunsli_internal_dec_Stage = 6;
pub type brunsli_internal_dec_SerializationStatus = i32;
pub const brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT:
    brunsli_internal_dec_SerializationStatus = 0;
pub const brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT:
    brunsli_internal_dec_SerializationStatus = 1;
pub const brunsli_internal_dec_SerializationStatus_ERROR: brunsli_internal_dec_SerializationStatus =
    2;
pub const brunsli_internal_dec_SerializationStatus_DONE: brunsli_internal_dec_SerializationStatus =
    3;
#[derive(Record, ByteRepr)]
#[byte_size(96)]
pub struct brunsli_internal_dec_State {
    #[offset(0)]
    #[byte_size(4)]
    pub stage: brunsli_internal_dec_Stage,
    #[offset(4)]
    pub tags_met: u32,
    #[offset(8)]
    pub skip_tags: u32,
    #[offset(16)]
    #[byte_size(8)]
    pub data: Ptr<u8>,
    #[offset(24)]
    pub len: usize,
    #[offset(32)]
    pub pos: usize,
    #[offset(40)]
    #[byte_size(8)]
    pub context_map: Ptr<u8>,
    #[offset(48)]
    #[byte_size(8)]
    pub entropy_codes: Ptr<brunsli_ANSDecodingData>,
    #[offset(56)]
    pub use_legacy_context_model: bool,
    #[offset(57)]
    pub is_storage_allocated: bool,
    #[offset(64)]
    #[byte_size(24)]
    pub meta: Value<Vec<brunsli_internal_dec_ComponentMeta>>,
    #[offset(88)]
    #[byte_size(8)]
    pub internal: Option<Value<brunsli_internal_dec_InternalState>>,
}
impl brunsli_internal_dec_State {}
impl Default for brunsli_internal_dec_State {
    fn default() -> Self {
        brunsli_internal_dec_State {
            stage: brunsli_internal_dec_Stage_SIGNATURE,
            tags_met: 0_u32,
            skip_tags: 0_u32,
            data: Ptr::<u8>::null(),
            len: 0_usize,
            pos: 0_usize,
            context_map: Ptr::<u8>::null(),
            entropy_codes: Ptr::<brunsli_ANSDecodingData>::null(),
            use_legacy_context_model: false,
            is_storage_allocated: false,
            meta: Rc::new(RefCell::new(Default::default())),
            internal: None,
        }
    }
}
#[derive(Record, ByteRepr)]
#[byte_size(16)]
pub struct brunsli_Arena_brunsli_HuffmanCode_ {
    #[offset(0)]
    pub capacity: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub storage: Option<Value<Box<[brunsli_HuffmanCode]>>>,
}
impl brunsli_Arena_brunsli_HuffmanCode_ {
    pub fn move_from(_a0: Ptr<brunsli_Arena_brunsli_HuffmanCode_>) -> Self {
        Self {
            capacity: { (*_a0.upgrade().deref()).capacity },
            storage: field!(_a0, storage)
                .with_mut(|__v: &mut Option<Value<Box<[brunsli_HuffmanCode]>>>| __v.take()),
        }
    }
}
impl Default for brunsli_Arena_brunsli_HuffmanCode_ {
    fn default() -> Self {
        brunsli_Arena_brunsli_HuffmanCode_ {
            capacity: 0_usize,
            storage: None,
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct brunsli_HuffmanDecodingData {
    #[offset(0)]
    #[byte_size(24)]
    pub table_: Value<Vec<brunsli_HuffmanCode>>,
}
#[derive(Record, ByteRepr)]
#[byte_size(24)]
pub struct brunsli_internal_dec_OutputChunk {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<u8>,
    #[offset(8)]
    pub len: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub buffer: Option<Value<Vec<u8>>>,
}
impl brunsli_internal_dec_OutputChunk {
    pub fn new_1(mut data: Ptr<u8>, mut size: usize) -> Self {
        Self {
            next: (data).clone(),
            len: size,
            buffer: None,
        }
    }
    pub fn new_2(size: Option<usize>) -> Self {
        let mut size: usize = size.unwrap_or(0_usize);
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: Ptr::<u8>::null(),
            len: 0_usize,
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        {
            let _p: Ptr<_> = Ptr::alloc(
                (0..(size) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            (field_ptr!(this, buffer) as Ptr<Option<Value<Vec<u8>>>>).write(_p.to_owned_opt())
        };
        field!(this, next).write(
            (Ptr::<Vec<u8>>::decay(&(this.with(|__s| __s.buffer.clone()).as_pointer())) as Ptr<u8>),
        );
        field!(this, len).write(size);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(bytes: Vec<u8>) -> Self {
        let bytes: Value<Vec<u8>> = Rc::new(RefCell::new(bytes));
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: Ptr::<u8>::null(),
            len: 0_usize,
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        {
            let _p: Ptr<_> = Ptr::alloc((*bytes.borrow()).clone());
            (field_ptr!(this, buffer) as Ptr<Option<Value<Vec<u8>>>>).write(_p.to_owned_opt())
        };
        field!(this, next).write(
            (Ptr::<Vec<u8>>::decay(&(this.with(|__s| __s.buffer.clone()).as_pointer())) as Ptr<u8>),
        );
        field!(this, len).write((*bytes.borrow()).len());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_OutputChunk>) -> Self {
        Self {
            next: { (*_a0.upgrade().deref()).next.clone() },
            len: { (*_a0.upgrade().deref()).len },
            buffer: field!(_a0, buffer).with_mut(|__v: &mut Option<Value<Vec<u8>>>| __v.take()),
        }
    }
}
impl Default for brunsli_internal_dec_OutputChunk {
    fn default() -> Self {
        { brunsli_internal_dec_OutputChunk::new_2(None) }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2048)]
pub struct brunsli_HuffmanCodeTable {
    #[offset(0)]
    #[byte_size(1024)]
    pub depth: Value<Box<[i32]>>,
    #[offset(1024)]
    #[byte_size(1024)]
    pub code: Value<Box<[i32]>>,
}
impl Default for brunsli_HuffmanCodeTable {
    fn default() -> Self {
        brunsli_HuffmanCodeTable {
            depth: Rc::new(RefCell::new(
                (0..256).map(|_| 0_i32).collect::<Box<[i32]>>(),
            )),
            code: Rc::new(RefCell::new(
                (0..256).map(|_| 0_i32).collect::<Box<[i32]>>(),
            )),
        }
    }
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(72)]
pub struct brunsli_internal_dec_BitWriter {
    #[offset(0)]
    pub healthy: bool,
    #[offset(8)]
    #[byte_size(8)]
    pub output: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
    #[offset(16)]
    #[byte_size(24)]
    pub chunk: brunsli_internal_dec_OutputChunk,
    #[offset(40)]
    #[byte_size(8)]
    pub data: Ptr<u8>,
    #[offset(48)]
    pub pos: usize,
    #[offset(56)]
    pub put_buffer: u64,
    #[offset(64)]
    pub put_bits: i32,
}
impl brunsli_internal_dec_BitWriter {
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_BitWriter>) -> Self {
        Self {
            healthy: { (*_a0.upgrade().deref()).healthy },
            output: { (*_a0.upgrade().deref()).output.clone() },
            chunk: brunsli_internal_dec_OutputChunk::move_from({ field_ptr!(_a0, chunk) }),
            data: { (*_a0.upgrade().deref()).data.clone() },
            pos: { (*_a0.upgrade().deref()).pos },
            put_buffer: { (*_a0.upgrade().deref()).put_buffer },
            put_bits: { (*_a0.upgrade().deref()).put_bits },
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct brunsli_internal_dec_DCTCodingState {
    #[offset(0)]
    pub eob_run_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub cur_ac_huff_: Ptr<brunsli_HuffmanCodeTable>,
    #[offset(16)]
    #[byte_size(24)]
    pub refinement_bits_: Value<Vec<u16>>,
    #[offset(40)]
    pub refinement_bits_count_: usize,
}
impl Default for brunsli_internal_dec_DCTCodingState {
    fn default() -> Self {
        brunsli_internal_dec_DCTCodingState {
            eob_run_: 0_i32,
            cur_ac_huff_: Ptr::<brunsli_HuffmanCodeTable>::null(),
            refinement_bits_: Rc::new(RefCell::new(Default::default())),
            refinement_bits_count_: 0_usize,
        }
    }
}
pub type brunsli_internal_dec_EncodeScanState_Stage = u32;
pub const brunsli_internal_dec_EncodeScanState_Stage_HEAD:
    brunsli_internal_dec_EncodeScanState_Stage = 0;
pub const brunsli_internal_dec_EncodeScanState_Stage_BODY:
    brunsli_internal_dec_EncodeScanState_Stage = 1;
#[derive(Record, ByteRepr)]
#[byte_size(184)]
pub struct brunsli_internal_dec_EncodeScanState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_EncodeScanState_Stage,
    #[offset(4)]
    pub mcu_y: i32,
    #[offset(8)]
    #[byte_size(72)]
    pub bw: brunsli_internal_dec_BitWriter,
    #[offset(80)]
    #[byte_size(8)]
    pub last_dc_coeff: Value<Box<[i16]>>,
    #[offset(88)]
    pub restarts_to_go: i32,
    #[offset(92)]
    pub next_restart_marker: i32,
    #[offset(96)]
    pub block_scan_index: i32,
    #[offset(104)]
    #[byte_size(48)]
    pub coding_state: brunsli_internal_dec_DCTCodingState,
    #[offset(152)]
    pub extra_zero_runs_pos: usize,
    #[offset(160)]
    pub next_extra_zero_run_index: i32,
    #[offset(168)]
    pub next_reset_point_pos: usize,
    #[offset(176)]
    pub next_reset_point: i32,
}
impl brunsli_internal_dec_EncodeScanState {
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_EncodeScanState>) -> Self {
        Self {
            stage: { (*_a0.upgrade().deref()).stage },
            mcu_y: { (*_a0.upgrade().deref()).mcu_y },
            bw: brunsli_internal_dec_BitWriter::move_from({ field_ptr!(_a0, bw) }),
            last_dc_coeff: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 4, _>(
                |__i: usize| {
                    (*{ (*_a0.upgrade().deref()).last_dc_coeff.clone() }.borrow())[(__i) as usize]
                },
            )))),
            restarts_to_go: { (*_a0.upgrade().deref()).restarts_to_go },
            next_restart_marker: { (*_a0.upgrade().deref()).next_restart_marker },
            block_scan_index: { (*_a0.upgrade().deref()).block_scan_index },
            coding_state: { (*_a0.upgrade().deref()).coding_state.clone() },
            extra_zero_runs_pos: { (*_a0.upgrade().deref()).extra_zero_runs_pos },
            next_extra_zero_run_index: { (*_a0.upgrade().deref()).next_extra_zero_run_index },
            next_reset_point_pos: { (*_a0.upgrade().deref()).next_reset_point_pos },
            next_reset_point: { (*_a0.upgrade().deref()).next_reset_point },
        }
    }
}
impl Default for brunsli_internal_dec_EncodeScanState {
    fn default() -> Self {
        brunsli_internal_dec_EncodeScanState {
            stage: brunsli_internal_dec_EncodeScanState_Stage_HEAD,
            mcu_y: 0_i32,
            bw: <brunsli_internal_dec_BitWriter>::default(),
            last_dc_coeff: Rc::new(RefCell::new(Box::new([0_i16, 0_i16, 0_i16, 0_i16]))),
            restarts_to_go: 0_i32,
            next_restart_marker: 0_i32,
            block_scan_index: 0_i32,
            coding_state: <brunsli_internal_dec_DCTCodingState>::default(),
            extra_zero_runs_pos: 0_usize,
            next_extra_zero_run_index: 0_i32,
            next_reset_point_pos: 0_usize,
            next_reset_point: 0_i32,
        }
    }
}
pub type brunsli_internal_dec_SerializationState_Stage = u32;
pub const brunsli_internal_dec_SerializationState_Stage_INIT:
    brunsli_internal_dec_SerializationState_Stage = 0;
pub const brunsli_internal_dec_SerializationState_Stage_SERIALIZE_SECTION:
    brunsli_internal_dec_SerializationState_Stage = 1;
pub const brunsli_internal_dec_SerializationState_Stage_DONE:
    brunsli_internal_dec_SerializationState_Stage = 2;
pub const brunsli_internal_dec_SerializationState_Stage_ERROR:
    brunsli_internal_dec_SerializationState_Stage = 3;
#[derive(Record, ByteRepr)]
#[byte_size(376)]
pub struct brunsli_internal_dec_SerializationState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_SerializationState_Stage,
    #[offset(8)]
    #[byte_size(80)]
    pub output_queue: Value<Vec<brunsli_internal_dec_OutputChunk>>,
    #[offset(88)]
    pub section_index: usize,
    #[offset(96)]
    pub dht_index: i32,
    #[offset(100)]
    pub dqt_index: i32,
    #[offset(104)]
    pub app_index: i32,
    #[offset(108)]
    pub com_index: i32,
    #[offset(112)]
    pub data_index: i32,
    #[offset(116)]
    pub scan_index: i32,
    #[offset(120)]
    #[byte_size(24)]
    pub dc_huff_table: Value<Vec<brunsli_HuffmanCodeTable>>,
    #[offset(144)]
    #[byte_size(24)]
    pub ac_huff_table: Value<Vec<brunsli_HuffmanCodeTable>>,
    #[offset(168)]
    #[byte_size(8)]
    pub pad_bits: Ptr<i32>,
    #[offset(176)]
    #[byte_size(8)]
    pub pad_bits_end: Ptr<i32>,
    #[offset(184)]
    pub seen_dri_marker: bool,
    #[offset(185)]
    pub is_progressive: bool,
    #[offset(192)]
    #[byte_size(184)]
    pub scan_state: brunsli_internal_dec_EncodeScanState,
}
impl brunsli_internal_dec_SerializationState {
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_SerializationState>) -> Self {
        Self {
            stage: { (*_a0.upgrade().deref()).stage },
            output_queue: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).output_queue.clone() }.borrow_mut()),
            ))),
            section_index: { (*_a0.upgrade().deref()).section_index },
            dht_index: { (*_a0.upgrade().deref()).dht_index },
            dqt_index: { (*_a0.upgrade().deref()).dqt_index },
            app_index: { (*_a0.upgrade().deref()).app_index },
            com_index: { (*_a0.upgrade().deref()).com_index },
            data_index: { (*_a0.upgrade().deref()).data_index },
            scan_index: { (*_a0.upgrade().deref()).scan_index },
            dc_huff_table: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).dc_huff_table.clone() }.borrow_mut()),
            ))),
            ac_huff_table: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).ac_huff_table.clone() }.borrow_mut()),
            ))),
            pad_bits: { (*_a0.upgrade().deref()).pad_bits.clone() },
            pad_bits_end: { (*_a0.upgrade().deref()).pad_bits_end.clone() },
            seen_dri_marker: { (*_a0.upgrade().deref()).seen_dri_marker },
            is_progressive: { (*_a0.upgrade().deref()).is_progressive },
            scan_state: brunsli_internal_dec_EncodeScanState::move_from({
                field_ptr!(_a0, scan_state)
            }),
        }
    }
}
impl Default for brunsli_internal_dec_SerializationState {
    fn default() -> Self {
        brunsli_internal_dec_SerializationState {
            stage: brunsli_internal_dec_SerializationState_Stage_INIT,
            output_queue: Rc::new(RefCell::new(Default::default())),
            section_index: 0_usize,
            dht_index: 0,
            dqt_index: 0,
            app_index: 0,
            com_index: 0,
            data_index: 0,
            scan_index: 0,
            dc_huff_table: Rc::new(RefCell::new(Default::default())),
            ac_huff_table: Rc::new(RefCell::new(Default::default())),
            pad_bits: Ptr::<i32>::null(),
            pad_bits_end: Ptr::<i32>::null(),
            seen_dri_marker: false,
            is_progressive: false,
            scan_state: <brunsli_internal_dec_EncodeScanState>::default(),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(80)]
pub struct brunsli_internal_dec_AcDcState {
    #[offset(0)]
    pub next_mcu_y: i32,
    #[offset(8)]
    pub next_component: usize,
    #[offset(16)]
    pub next_iy: i32,
    #[offset(20)]
    pub next_x: i32,
    #[offset(24)]
    pub ac_coeffs_order_decoded: bool,
    #[offset(32)]
    #[byte_size(24)]
    pub ac: Value<Vec<brunsli_ComponentState>>,
    #[offset(56)]
    #[byte_size(24)]
    pub dc: Value<Vec<brunsli_ComponentStateDC>>,
}
impl Default for brunsli_internal_dec_AcDcState {
    fn default() -> Self {
        brunsli_internal_dec_AcDcState {
            next_mcu_y: 0,
            next_component: 0_usize,
            next_iy: 0,
            next_x: 0,
            ac_coeffs_order_decoded: false,
            ac: Rc::new(RefCell::new(Default::default())),
            dc: Rc::new(RefCell::new(Default::default())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct brunsli_internal_dec_SectionState {
    #[offset(0)]
    pub tag: usize,
    #[offset(8)]
    pub is_active: bool,
    #[offset(9)]
    pub is_section: bool,
    #[offset(12)]
    pub tags_met: u32,
    #[offset(16)]
    pub remaining: usize,
    #[offset(24)]
    pub milestone: usize,
    #[offset(32)]
    pub projected_end: usize,
}
impl Default for brunsli_internal_dec_SectionState {
    fn default() -> Self {
        brunsli_internal_dec_SectionState {
            tag: 0_usize,
            is_active: false,
            is_section: false,
            tags_met: 0_u32,
            remaining: 0_usize,
            milestone: 0_usize,
            projected_end: 0_usize,
        }
    }
}
pub type brunsli_internal_dec_HeaderState_Stage = u32;
pub const brunsli_internal_dec_HeaderState_Stage_READ_TAG: brunsli_internal_dec_HeaderState_Stage =
    0;
pub const brunsli_internal_dec_HeaderState_Stage_ENTER_SECTION:
    brunsli_internal_dec_HeaderState_Stage = 1;
pub const brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG:
    brunsli_internal_dec_HeaderState_Stage = 2;
pub const brunsli_internal_dec_HeaderState_Stage_ITEM_ENTER_SECTION:
    brunsli_internal_dec_HeaderState_Stage = 3;
pub const brunsli_internal_dec_HeaderState_Stage_ITEM_SKIP_CONTENTS:
    brunsli_internal_dec_HeaderState_Stage = 4;
pub const brunsli_internal_dec_HeaderState_Stage_ITEM_READ_VALUE:
    brunsli_internal_dec_HeaderState_Stage = 5;
pub const brunsli_internal_dec_HeaderState_Stage_FINALE: brunsli_internal_dec_HeaderState_Stage = 6;
pub const brunsli_internal_dec_HeaderState_Stage_DONE: brunsli_internal_dec_HeaderState_Stage = 7;
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(184)]
pub struct brunsli_internal_dec_HeaderState {
    #[offset(0)]
    pub stage: usize,
    #[offset(8)]
    #[byte_size(40)]
    pub section: brunsli_internal_dec_SectionState,
    #[offset(48)]
    pub remaining_skip_length: usize,
    #[offset(56)]
    #[byte_size(128)]
    pub varint_values: Value<Vec<u64>>,
}
impl Default for brunsli_internal_dec_HeaderState {
    fn default() -> Self {
        brunsli_internal_dec_HeaderState {
            stage: (brunsli_internal_dec_HeaderState_Stage_READ_TAG as usize),
            section: <brunsli_internal_dec_SectionState>::default(),
            remaining_skip_length: 0_usize,
            varint_values: Rc::new(RefCell::new(
                std::array::from_fn::<_, 16, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
pub type brunsli_internal_dec_FallbackState_Stage = u32;
pub const brunsli_internal_dec_FallbackState_Stage_READ_TAG:
    brunsli_internal_dec_FallbackState_Stage = 0;
pub const brunsli_internal_dec_FallbackState_Stage_ENTER_SECTION:
    brunsli_internal_dec_FallbackState_Stage = 1;
pub const brunsli_internal_dec_FallbackState_Stage_READ_CONTENTS:
    brunsli_internal_dec_FallbackState_Stage = 2;
pub const brunsli_internal_dec_FallbackState_Stage_DONE: brunsli_internal_dec_FallbackState_Stage =
    3;
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct brunsli_internal_dec_FallbackState {
    #[offset(0)]
    pub stage: usize,
    #[offset(8)]
    #[byte_size(24)]
    pub storage: Value<Vec<u8>>,
}
impl Default for brunsli_internal_dec_FallbackState {
    fn default() -> Self {
        brunsli_internal_dec_FallbackState {
            stage: (brunsli_internal_dec_FallbackState_Stage_READ_TAG as usize),
            storage: Rc::new(RefCell::new(Default::default())),
        }
    }
}
pub type brunsli_internal_dec_SectionHeaderState_Stage = u32;
pub const brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG:
    brunsli_internal_dec_SectionHeaderState_Stage = 0;
pub const brunsli_internal_dec_SectionHeaderState_Stage_READ_VALUE:
    brunsli_internal_dec_SectionHeaderState_Stage = 1;
pub const brunsli_internal_dec_SectionHeaderState_Stage_ENTER_SECTION:
    brunsli_internal_dec_SectionHeaderState_Stage = 2;
pub const brunsli_internal_dec_SectionHeaderState_Stage_DONE:
    brunsli_internal_dec_SectionHeaderState_Stage = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct brunsli_internal_dec_SectionHeaderState {
    #[offset(0)]
    pub stage: usize,
}
impl Default for brunsli_internal_dec_SectionHeaderState {
    fn default() -> Self {
        brunsli_internal_dec_SectionHeaderState {
            stage: (brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG as usize),
        }
    }
}
pub type brunsli_internal_dec_MetadataDecompressionStage = i32;
pub const brunsli_internal_dec_MetadataDecompressionStage_INITIAL:
    brunsli_internal_dec_MetadataDecompressionStage = 0;
pub const brunsli_internal_dec_MetadataDecompressionStage_READ_LENGTH:
    brunsli_internal_dec_MetadataDecompressionStage = 1;
pub const brunsli_internal_dec_MetadataDecompressionStage_DECOMPRESSING:
    brunsli_internal_dec_MetadataDecompressionStage = 2;
pub const brunsli_internal_dec_MetadataDecompressionStage_DONE:
    brunsli_internal_dec_MetadataDecompressionStage = 3;
pub type brunsli_internal_dec_MetadataState_Stage = u32;
pub const brunsli_internal_dec_MetadataState_Stage_READ_MARKER:
    brunsli_internal_dec_MetadataState_Stage = 0;
pub const brunsli_internal_dec_MetadataState_Stage_READ_TAIL:
    brunsli_internal_dec_MetadataState_Stage = 1;
pub const brunsli_internal_dec_MetadataState_Stage_READ_CODE:
    brunsli_internal_dec_MetadataState_Stage = 2;
pub const brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_HI:
    brunsli_internal_dec_MetadataState_Stage = 3;
pub const brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_LO:
    brunsli_internal_dec_MetadataState_Stage = 4;
pub const brunsli_internal_dec_MetadataState_Stage_READ_MULTIBYTE:
    brunsli_internal_dec_MetadataState_Stage = 5;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(72)]
pub struct brunsli_internal_dec_MetadataState {
    #[offset(0)]
    pub short_marker_count: usize,
    #[offset(8)]
    pub marker: u8,
    #[offset(9)]
    pub length_hi: u8,
    #[offset(16)]
    pub remaining_multibyte_length: usize,
    #[offset(24)]
    #[byte_size(8)]
    pub multibyte_sink: Ptr<Vec<u8>>,
    #[offset(32)]
    pub stage: usize,
    #[offset(40)]
    #[byte_size(8)]
    pub brotli: *mut ::brotli_sys::BrotliDecoderState,
    #[offset(48)]
    pub metadata_size: usize,
    #[offset(56)]
    pub decompressed_size: usize,
    #[offset(64)]
    pub result: brunsli_BrunsliStatus,
    #[offset(68)]
    #[byte_size(4)]
    pub decompression_stage: brunsli_internal_dec_MetadataDecompressionStage,
}
impl Default for brunsli_internal_dec_MetadataState {
    fn default() -> Self {
        brunsli_internal_dec_MetadataState {
            short_marker_count: 0_usize,
            marker: 0_u8,
            length_hi: 0_u8,
            remaining_multibyte_length: 0_usize,
            multibyte_sink: Ptr::<Vec<u8>>::null(),
            stage: (brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize),
            brotli: std::ptr::null_mut(),
            metadata_size: 0_usize,
            decompressed_size: 0_usize,
            result: brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
            decompression_stage: brunsli_internal_dec_MetadataDecompressionStage_INITIAL,
        }
    }
}
pub type brunsli_internal_dec_VarintState_Stage = u32;
pub const brunsli_internal_dec_VarintState_Stage_INIT: brunsli_internal_dec_VarintState_Stage = 0;
pub const brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION:
    brunsli_internal_dec_VarintState_Stage = 1;
pub const brunsli_internal_dec_VarintState_Stage_READ_DATA: brunsli_internal_dec_VarintState_Stage =
    2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct brunsli_internal_dec_VarintState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_VarintState_Stage,
    #[offset(8)]
    pub value: usize,
    #[offset(16)]
    pub i: usize,
}
impl Default for brunsli_internal_dec_VarintState {
    fn default() -> Self {
        brunsli_internal_dec_VarintState {
            stage: brunsli_internal_dec_VarintState_Stage_INIT,
            value: 0_usize,
            i: 0_usize,
        }
    }
}
pub type brunsli_internal_dec_JpegInternalsState_Stage = u32;
pub const brunsli_internal_dec_JpegInternalsState_Stage_INIT:
    brunsli_internal_dec_JpegInternalsState_Stage = 0;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_MARKERS:
    brunsli_internal_dec_JpegInternalsState_Stage = 1;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_DRI:
    brunsli_internal_dec_JpegInternalsState_Stage = 2;
pub const brunsli_internal_dec_JpegInternalsState_Stage_DECODE_HUFFMAN_MASK:
    brunsli_internal_dec_JpegInternalsState_Stage = 16;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST:
    brunsli_internal_dec_JpegInternalsState_Stage = 17;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_SIMPLE:
    brunsli_internal_dec_JpegInternalsState_Stage = 18;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_MAX_LEN:
    brunsli_internal_dec_JpegInternalsState_Stage = 19;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_COUNT:
    brunsli_internal_dec_JpegInternalsState_Stage = 20;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_PERMUTATION:
    brunsli_internal_dec_JpegInternalsState_Stage = 21;
pub const brunsli_internal_dec_JpegInternalsState_Stage_HUFFMAN_UPDATE:
    brunsli_internal_dec_JpegInternalsState_Stage = 22;
pub const brunsli_internal_dec_JpegInternalsState_Stage_PREPARE_READ_SCANS:
    brunsli_internal_dec_JpegInternalsState_Stage = 32;
pub const brunsli_internal_dec_JpegInternalsState_Stage_DECODE_SCAN_MASK:
    brunsli_internal_dec_JpegInternalsState_Stage = 64;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON:
    brunsli_internal_dec_JpegInternalsState_Stage = 65;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMPONENT:
    brunsli_internal_dec_JpegInternalsState_Stage = 66;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION:
    brunsli_internal_dec_JpegInternalsState_Stage = 67;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_DATA:
    brunsli_internal_dec_JpegInternalsState_Stage = 68;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION:
    brunsli_internal_dec_JpegInternalsState_Stage = 69;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_DATA:
    brunsli_internal_dec_JpegInternalsState_Stage = 70;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_QUANT:
    brunsli_internal_dec_JpegInternalsState_Stage = 128;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_QUANT:
    brunsli_internal_dec_JpegInternalsState_Stage = 129;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID_SCHEME:
    brunsli_internal_dec_JpegInternalsState_Stage = 130;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID:
    brunsli_internal_dec_JpegInternalsState_Stage = 131;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_PADDING_BITS:
    brunsli_internal_dec_JpegInternalsState_Stage = 132;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_PADDING_BITS:
    brunsli_internal_dec_JpegInternalsState_Stage = 133;
pub const brunsli_internal_dec_JpegInternalsState_Stage_ITERATE_MARKERS:
    brunsli_internal_dec_JpegInternalsState_Stage = 134;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_LENGTH:
    brunsli_internal_dec_JpegInternalsState_Stage = 135;
pub const brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_DATA:
    brunsli_internal_dec_JpegInternalsState_Stage = 136;
pub const brunsli_internal_dec_JpegInternalsState_Stage_DONE:
    brunsli_internal_dec_JpegInternalsState_Stage = 137;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(200)]
pub struct brunsli_internal_dec_JpegInternalsState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_JpegInternalsState_Stage,
    #[offset(4)]
    pub have_dri: bool,
    #[offset(8)]
    pub num_scans: usize,
    #[offset(16)]
    pub dht_count: usize,
    #[offset(24)]
    #[byte_size(32)]
    pub br: brunsli_BrunsliBitReader,
    #[offset(56)]
    pub is_known_last_huffman_code: usize,
    #[offset(64)]
    pub terminal_huffman_code_count: usize,
    #[offset(72)]
    pub is_dc_table: bool,
    #[offset(80)]
    pub total_count: usize,
    #[offset(88)]
    pub space: usize,
    #[offset(96)]
    pub max_len: usize,
    #[offset(104)]
    pub max_count: usize,
    #[offset(112)]
    pub i: usize,
    #[offset(120)]
    #[byte_size(24)]
    pub p: brunsli_PermutationCoder,
    #[offset(144)]
    #[byte_size(24)]
    pub varint: brunsli_internal_dec_VarintState,
    #[offset(168)]
    pub j: usize,
    #[offset(176)]
    pub last_block_idx: i32,
    #[offset(180)]
    pub last_num: i32,
    #[offset(184)]
    pub num_padding_bits: usize,
    #[offset(192)]
    pub intermarker_length: usize,
}
impl Default for brunsli_internal_dec_JpegInternalsState {
    fn default() -> Self {
        brunsli_internal_dec_JpegInternalsState {
            stage: brunsli_internal_dec_JpegInternalsState_Stage_INIT,
            have_dri: false,
            num_scans: 0_usize,
            dht_count: 0_usize,
            br: <brunsli_BrunsliBitReader>::default(),
            is_known_last_huffman_code: 0_usize,
            terminal_huffman_code_count: 0_usize,
            is_dc_table: false,
            total_count: 0_usize,
            space: 0_usize,
            max_len: 0_usize,
            max_count: 0_usize,
            i: 0_usize,
            p: <brunsli_PermutationCoder>::default(),
            varint: <brunsli_internal_dec_VarintState>::default(),
            j: 0_usize,
            last_block_idx: 0_i32,
            last_num: 0_i32,
            num_padding_bits: 0_usize,
            intermarker_length: 0_usize,
        }
    }
}
pub type brunsli_internal_dec_QuantDataState_Stage = u32;
pub const brunsli_internal_dec_QuantDataState_Stage_INIT:
    brunsli_internal_dec_QuantDataState_Stage = 0;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_NUM_QUANT:
    brunsli_internal_dec_QuantDataState_Stage = 1;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_STOCK:
    brunsli_internal_dec_QuantDataState_Stage = 2;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_Q_FACTOR:
    brunsli_internal_dec_QuantDataState_Stage = 3;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_IS_ZERO:
    brunsli_internal_dec_QuantDataState_Stage = 4;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_SIGN:
    brunsli_internal_dec_QuantDataState_Stage = 5;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_DIFF:
    brunsli_internal_dec_QuantDataState_Stage = 6;
pub const brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF:
    brunsli_internal_dec_QuantDataState_Stage = 7;
pub const brunsli_internal_dec_QuantDataState_Stage_UPDATE:
    brunsli_internal_dec_QuantDataState_Stage = 8;
pub const brunsli_internal_dec_QuantDataState_Stage_READ_QUANT_IDX:
    brunsli_internal_dec_QuantDataState_Stage = 9;
pub const brunsli_internal_dec_QuantDataState_Stage_FINISH:
    brunsli_internal_dec_QuantDataState_Stage = 10;
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(120)]
pub struct brunsli_internal_dec_QuantDataState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_QuantDataState_Stage,
    #[offset(8)]
    #[byte_size(32)]
    pub br: brunsli_BrunsliBitReader,
    #[offset(40)]
    pub i: usize,
    #[offset(48)]
    pub j: usize,
    #[offset(56)]
    pub data_precision: u8,
    #[offset(64)]
    #[byte_size(24)]
    pub vs: brunsli_internal_dec_VarintState,
    #[offset(88)]
    pub delta: i32,
    #[offset(92)]
    pub sign: i32,
    #[offset(96)]
    #[byte_size(24)]
    pub predictor: Value<Vec<u8>>,
}
impl Default for brunsli_internal_dec_QuantDataState {
    fn default() -> Self {
        brunsli_internal_dec_QuantDataState {
            stage: brunsli_internal_dec_QuantDataState_Stage_INIT,
            br: <brunsli_BrunsliBitReader>::default(),
            i: 0_usize,
            j: 0_usize,
            data_precision: 0_u8,
            vs: <brunsli_internal_dec_VarintState>::default(),
            delta: 0_i32,
            sign: 0_i32,
            predictor: Rc::new(RefCell::new(Default::default())),
        }
    }
}
pub type brunsli_internal_dec_HistogramDataState_Stage = u32;
pub const brunsli_internal_dec_HistogramDataState_Stage_INIT:
    brunsli_internal_dec_HistogramDataState_Stage = 0;
pub const brunsli_internal_dec_HistogramDataState_Stage_READ_SCHEME:
    brunsli_internal_dec_HistogramDataState_Stage = 1;
pub const brunsli_internal_dec_HistogramDataState_Stage_READ_NUM_HISTOGRAMS:
    brunsli_internal_dec_HistogramDataState_Stage = 2;
pub const brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP_CODE:
    brunsli_internal_dec_HistogramDataState_Stage = 3;
pub const brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP:
    brunsli_internal_dec_HistogramDataState_Stage = 4;
pub const brunsli_internal_dec_HistogramDataState_Stage_READ_HISTOGRAMS:
    brunsli_internal_dec_HistogramDataState_Stage = 5;
pub const brunsli_internal_dec_HistogramDataState_Stage_SKIP_CONTENT:
    brunsli_internal_dec_HistogramDataState_Stage = 6;
pub const brunsli_internal_dec_HistogramDataState_Stage_DONE:
    brunsli_internal_dec_HistogramDataState_Stage = 7;
#[derive(Record, ByteRepr)]
#[byte_size(104)]
pub struct brunsli_internal_dec_HistogramDataState {
    #[offset(0)]
    pub stage: brunsli_internal_dec_HistogramDataState_Stage,
    #[offset(8)]
    #[byte_size(32)]
    pub br: brunsli_BrunsliBitReader,
    #[offset(40)]
    pub max_run_length_prefix: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub entropy: Option<Value<brunsli_HuffmanDecodingData>>,
    #[offset(56)]
    pub i: usize,
    #[offset(64)]
    #[byte_size(24)]
    pub counts: Value<Vec<u32>>,
    #[offset(88)]
    #[byte_size(16)]
    pub arena: brunsli_Arena_brunsli_HuffmanCode_,
}
impl brunsli_internal_dec_HistogramDataState {
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_HistogramDataState>) -> Self {
        Self {
            stage: { (*_a0.upgrade().deref()).stage },
            br: { (*_a0.upgrade().deref()).br.clone() },
            max_run_length_prefix: { (*_a0.upgrade().deref()).max_run_length_prefix },
            entropy: field!(_a0, entropy)
                .with_mut(|__v: &mut Option<Value<brunsli_HuffmanDecodingData>>| __v.take()),
            i: { (*_a0.upgrade().deref()).i },
            counts: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).counts.clone() }.borrow_mut()),
            ))),
            arena: brunsli_Arena_brunsli_HuffmanCode_::move_from({ field_ptr!(_a0, arena) }),
        }
    }
}
impl Default for brunsli_internal_dec_HistogramDataState {
    fn default() -> Self {
        brunsli_internal_dec_HistogramDataState {
            stage: brunsli_internal_dec_HistogramDataState_Stage_INIT,
            br: <brunsli_BrunsliBitReader>::default(),
            max_run_length_prefix: 0_usize,
            entropy: None,
            i: 0_usize,
            counts: Rc::new(RefCell::new(Default::default())),
            arena: <brunsli_Arena_brunsli_HuffmanCode_>::default(),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(64)]
pub struct brunsli_internal_dec_Buffer {
    #[offset(0)]
    pub data_len: usize,
    #[offset(8)]
    pub borrowed_len: usize,
    #[offset(16)]
    #[byte_size(24)]
    pub data: Value<Vec<u8>>,
    #[offset(40)]
    #[byte_size(8)]
    pub external_data: Ptr<u8>,
    #[offset(48)]
    pub external_pos: usize,
    #[offset(56)]
    pub external_len: usize,
}
impl Default for brunsli_internal_dec_Buffer {
    fn default() -> Self {
        brunsli_internal_dec_Buffer {
            data_len: 0_usize,
            borrowed_len: 0_usize,
            data: Rc::new(RefCell::new(Default::default())),
            external_data: Ptr::<u8>::null(),
            external_pos: 0_usize,
            external_len: 0_usize,
        }
    }
}
#[derive(Record, ByteRepr)]
#[byte_size(1416)]
pub struct brunsli_internal_dec_InternalState {
    #[offset(0)]
    #[byte_size(80)]
    pub ac_dc: brunsli_internal_dec_AcDcState,
    #[offset(80)]
    #[byte_size(40)]
    pub section: brunsli_internal_dec_SectionState,
    #[offset(120)]
    #[byte_size(184)]
    pub header: brunsli_internal_dec_HeaderState,
    #[offset(304)]
    #[byte_size(32)]
    pub fallback: brunsli_internal_dec_FallbackState,
    #[offset(336)]
    #[byte_size(8)]
    pub section_header: brunsli_internal_dec_SectionHeaderState,
    #[offset(344)]
    #[byte_size(72)]
    pub metadata: brunsli_internal_dec_MetadataState,
    #[offset(416)]
    #[byte_size(200)]
    pub internals: brunsli_internal_dec_JpegInternalsState,
    #[offset(616)]
    #[byte_size(120)]
    pub quant: brunsli_internal_dec_QuantDataState,
    #[offset(736)]
    #[byte_size(104)]
    pub histogram: brunsli_internal_dec_HistogramDataState,
    #[offset(840)]
    #[byte_size(24)]
    pub context_map_: Value<Vec<u8>>,
    #[offset(864)]
    #[byte_size(24)]
    pub entropy_codes_: Value<Vec<brunsli_ANSDecodingData>>,
    #[offset(888)]
    #[byte_size(24)]
    pub block_state_: Value<Vec<Value<Vec<u8>>>>,
    #[offset(912)]
    pub is_meta_warm: bool,
    #[offset(913)]
    pub shallow_histograms: bool,
    #[offset(920)]
    pub num_contexts: usize,
    #[offset(928)]
    pub num_histograms: usize,
    #[offset(936)]
    pub subdecoders_initialized: bool,
    #[offset(940)]
    #[byte_size(4)]
    pub ans_decoder: brunsli_ANSDecoder,
    #[offset(944)]
    #[byte_size(8)]
    pub bit_reader: brunsli_BitSource,
    #[offset(952)]
    #[byte_size(12)]
    pub arith_decoder: brunsli_BinaryArithmeticDecoder,
    #[offset(964)]
    pub result: brunsli_BrunsliStatus,
    #[offset(968)]
    #[byte_size(4)]
    pub last_stage: brunsli_internal_dec_Stage,
    #[offset(976)]
    #[byte_size(64)]
    pub buffer: brunsli_internal_dec_Buffer,
    #[offset(1040)]
    #[byte_size(376)]
    pub serialization: brunsli_internal_dec_SerializationState,
}
impl brunsli_internal_dec_InternalState {
    pub fn move_from(_a0: Ptr<brunsli_internal_dec_InternalState>) -> Self {
        Self {
            ac_dc: { (*_a0.upgrade().deref()).ac_dc.clone() },
            section: { (*_a0.upgrade().deref()).section.clone() },
            header: { (*_a0.upgrade().deref()).header.clone() },
            fallback: { (*_a0.upgrade().deref()).fallback.clone() },
            section_header: { (*_a0.upgrade().deref()).section_header.clone() },
            metadata: { (*_a0.upgrade().deref()).metadata.clone() },
            internals: { (*_a0.upgrade().deref()).internals.clone() },
            quant: { (*_a0.upgrade().deref()).quant.clone() },
            histogram: brunsli_internal_dec_HistogramDataState::move_from({
                field_ptr!(_a0, histogram)
            }),
            context_map_: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).context_map_.clone() }.borrow_mut()),
            ))),
            entropy_codes_: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).entropy_codes_.clone() }.borrow_mut()),
            ))),
            block_state_: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).block_state_.clone() }.borrow_mut()),
            ))),
            is_meta_warm: { (*_a0.upgrade().deref()).is_meta_warm },
            shallow_histograms: { (*_a0.upgrade().deref()).shallow_histograms },
            num_contexts: { (*_a0.upgrade().deref()).num_contexts },
            num_histograms: { (*_a0.upgrade().deref()).num_histograms },
            subdecoders_initialized: { (*_a0.upgrade().deref()).subdecoders_initialized },
            ans_decoder: { (*_a0.upgrade().deref()).ans_decoder.clone() },
            bit_reader: { (*_a0.upgrade().deref()).bit_reader.clone() },
            arith_decoder: { (*_a0.upgrade().deref()).arith_decoder.clone() },
            result: { (*_a0.upgrade().deref()).result },
            last_stage: { (*_a0.upgrade().deref()).last_stage },
            buffer: { (*_a0.upgrade().deref()).buffer.clone() },
            serialization: brunsli_internal_dec_SerializationState::move_from({
                field_ptr!(_a0, serialization)
            }),
        }
    }
}
impl Default for brunsli_internal_dec_InternalState {
    fn default() -> Self {
        brunsli_internal_dec_InternalState {
            ac_dc: <brunsli_internal_dec_AcDcState>::default(),
            section: <brunsli_internal_dec_SectionState>::default(),
            header: <brunsli_internal_dec_HeaderState>::default(),
            fallback: <brunsli_internal_dec_FallbackState>::default(),
            section_header: <brunsli_internal_dec_SectionHeaderState>::default(),
            metadata: <brunsli_internal_dec_MetadataState>::default(),
            internals: <brunsli_internal_dec_JpegInternalsState>::default(),
            quant: <brunsli_internal_dec_QuantDataState>::default(),
            histogram: <brunsli_internal_dec_HistogramDataState>::default(),
            context_map_: Rc::new(RefCell::new(Default::default())),
            entropy_codes_: Rc::new(RefCell::new(Default::default())),
            block_state_: Rc::new(RefCell::new(Vec::new())),
            is_meta_warm: false,
            shallow_histograms: false,
            num_contexts: 0_usize,
            num_histograms: 0_usize,
            subdecoders_initialized: false,
            ans_decoder: <brunsli_ANSDecoder>::default(),
            bit_reader: <brunsli_BitSource>::default(),
            arith_decoder: <brunsli_BinaryArithmeticDecoder>::default(),
            result: brunsli_BrunsliStatus_BRUNSLI_OK,
            last_stage: brunsli_internal_dec_Stage_ERROR,
            buffer: <brunsli_internal_dec_Buffer>::default(),
            serialization: <brunsli_internal_dec_SerializationState>::default(),
        }
    }
}
thread_local!(
    pub static kNumDirectCodes_135: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kCoeffAlphabetSize_136: Value<i32> = Rc::new(RefCell::new(
        (kNumDirectCodes_135.with(|rc| *rc.borrow()) + 10),
    ));
);
thread_local!(
    pub static kKnownSectionTags_137: Value<u32> = Rc::new(RefCell::new(
        (((((((((1_u32 << (kBrunsliSignatureTag_30.with(|rc| *rc.borrow()) as i32))
            | (1_u32 << (kBrunsliHeaderTag_31.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliMetaDataTag_32.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow()) as i32))),
    ));
);
thread_local!(
    pub static kKnownHeaderVarintTags_138: Value<u32> = Rc::new(RefCell::new(
        ((((1_u32 << (kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow()) as i32))
            | (1_u32 << (kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow()) as i32)))
            | (1_u32 << (kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow()) as i32))),
    ));
);
pub fn IsBrunsli_139(mut data: Ptr<u8>, mut len: usize) -> bool {
    thread_local!(
        static kSignature_140: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            10_u8, 4_u8, 66_u8, 210_u8, 213_u8, 78_u8,
        ])));
    );
    thread_local!(
        static kSignatureLen_141: Value<usize> =
            Rc::new(RefCell::new(::std::mem::size_of::<[u8; 6]>()));
    );
    if (len < kSignatureLen_141.with(|rc| *rc.borrow())) {
        return false;
    }
    return (((kSignature_140.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>)
        .to_any()
        .memcmp(&(data).to_any(), kSignatureLen_141.with(|rc| *rc.borrow()))
        == 0);
}
pub fn DivCeil_142(mut a: i32, mut b: i32) -> i32 {
    return (((a + b) - 1) / b);
}
pub fn DecodeVarLenUint8_143(mut br: Ptr<brunsli_BrunsliBitReader>) -> u32 {
    if (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0) {
        let mut nbits: u32 = ({ BrunsliBitReaderRead_126((br).clone(), 3_u32) });
        if (nbits == 0_u32) {
            return 1_u32;
        } else {
            return ({ BrunsliBitReaderRead_126(br, nbits) }).wrapping_add((1_u32 << nbits));
        }
    }
    return 0_u32;
}
pub fn DecodeVarint_144(
    mut s: Ptr<brunsli_internal_dec_VarintState>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut max_bits: usize,
) -> bool {
    if ((s.with(|__s| __s.stage) as i32) == (brunsli_internal_dec_VarintState_Stage_INIT as i32)) {
        field!(s, value).write(0_usize);
        field!(s, i).write(0_usize);
        field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { (s.with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ s.with(|__s| __s.i) } >= { max_bits }) {
                        field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                        return true;
                    }
                    if ({ (s.with(|__s| __s.i)).wrapping_add(1_usize) } != { max_bits }) {
                        if !({ BrunsliBitReaderCanRead_134((br).clone(), 1_usize) }) {
                            return false;
                        }
                        if !(({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0) {
                            field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                            return true;
                        }
                    }
                    field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((br).clone(), 1_usize) }) {
                        return false;
                    }
                    let mut next_bit: usize =
                        (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as usize);
                    {
                        let __rhs = { ({ next_bit } << { s.with(|__s| __s.i) }) };
                        field!(s, value).with_mut(|__v| *__v = *__v | __rhs)
                    };
                    field!(s, i).with_mut(|__v| __v.prefix_inc());
                    field!(s, stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                132,
                                Ptr::<i8>::from_string_literal(b"DecodeVarint"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return false;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeLimitedVarint_145(
    mut s: Ptr<brunsli_internal_dec_VarintState>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut max_symbols: usize,
) -> bool {
    if ((s.with(|__s| __s.stage) as i32) == (brunsli_internal_dec_VarintState_Stage_INIT as i32)) {
        field!(s, value).write(0_usize);
        field!(s, i).write(0_usize);
        field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { (s.with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ s.with(|__s| __s.i) } < { max_symbols }) {
                        if !({ BrunsliBitReaderCanRead_134((br).clone(), 1_usize) }) {
                            return false;
                        }
                        if (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0) {
                            field!(s, stage)
                                .write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                            continue 'loop_;
                        }
                    }
                    field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                    return true;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((br).clone(), (2_u64 as usize)) }) {
                        return false;
                    }
                    let mut next_bits: usize =
                        (({ BrunsliBitReaderRead_126((br).clone(), (2_u64 as u32)) }) as usize);
                    {
                        let __rhs = {
                            ({ next_bits } << {
                                ((s.with(|__s| __s.i) as u64).wrapping_mul((2_u64 as u64)))
                            })
                        };
                        field!(s, value).with_mut(|__v| *__v = *__v | __rhs)
                    };
                    field!(s, i).with_mut(|__v| __v.prefix_inc());
                    field!(s, stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                169,
                                Ptr::<i8>::from_string_literal(b"DecodeLimitedVarint"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return false;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeLimitedVarint_146(
    mut s: Ptr<brunsli_internal_dec_VarintState>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut max_symbols: usize,
) -> bool {
    if ((s.with(|__s| __s.stage) as i32) == (brunsli_internal_dec_VarintState_Stage_INIT as i32)) {
        field!(s, value).write(0_usize);
        field!(s, i).write(0_usize);
        field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { (s.with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ s.with(|__s| __s.i) } < { max_symbols }) {
                        if !({ BrunsliBitReaderCanRead_134((br).clone(), 1_usize) }) {
                            return false;
                        }
                        if (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0) {
                            field!(s, stage)
                                .write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                            continue 'loop_;
                        }
                    }
                    field!(s, stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                    return true;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((br).clone(), (8_u64 as usize)) }) {
                        return false;
                    }
                    let mut next_bits: usize =
                        (({ BrunsliBitReaderRead_126((br).clone(), (8_u64 as u32)) }) as usize);
                    {
                        let __rhs = {
                            ({ next_bits } << {
                                ((s.with(|__s| __s.i) as u64).wrapping_mul((8_u64 as u64)))
                            })
                        };
                        field!(s, value).with_mut(|__v| *__v = *__v | __rhs)
                    };
                    field!(s, i).with_mut(|__v| __v.prefix_inc());
                    field!(s, stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                169,
                                Ptr::<i8>::from_string_literal(b"DecodeLimitedVarint"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return false;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn GenerateApp0Marker_147(mut app0_status: u8) -> Vec<u8> {
    let app0_marker: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (AppData_0xe0_62.with(|v| v.as_pointer()) as Ptr<u8>)
            .offset((17) as isize)
            .get_offset()
            - (AppData_0xe0_62.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
        PtrValueIter::new(
            &(AppData_0xe0_62.with(|v| v.as_pointer()) as Ptr<u8>),
            __count,
        )
        .collect::<Vec<_>>()
    }));
    elem!((app0_marker.as_pointer() as Ptr<u8>), 9_usize).write(
        (if (((app0_status as u32) & 1_u32) != 0) {
            2
        } else {
            1
        } as u8),
    );
    app0_status = { ((app0_status as i32) >> 1_u32) as u8 };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 10_usize)
        .write((((app0_status as u32) & 3_u32) as u8));
    app0_status = { ((app0_status as i32) >> 2_u32) as u8 };
    let mut x_dens: u16 = ({
        let __idx = (app0_status) as usize;
        kApp0Densities_46.with(|rc| rc.borrow()[__idx])
    });
    let __rhs = {
        elem!((app0_marker.as_pointer() as Ptr<u8>), 13_usize)
            .write((((x_dens as i32) >> 8_u32) as u8));
        (elem!((app0_marker.as_pointer() as Ptr<u8>), 13_usize).read())
    };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 11_usize).write(__rhs);
    let __rhs = {
        elem!((app0_marker.as_pointer() as Ptr<u8>), 14_usize)
            .write((((x_dens as u32) & 255_u32) as u8));
        (elem!((app0_marker.as_pointer() as Ptr<u8>), 14_usize).read())
    };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 12_usize).write(__rhs);
    return std::mem::take(&mut (*app0_marker.borrow_mut()));
}
pub fn GenerateAppMarker_148(mut marker: u8, mut code: u8) -> Vec<u8> {
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if ((marker as i32) == 128) {
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xe2_63.with(|v| v.as_pointer()) as Ptr<u8>)
                .offset((3161) as isize)
                .get_offset()
                - (AppData_0xe2_63.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xe2_63.with(|v| v.as_pointer()) as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        elem!((s.as_pointer() as Ptr<u8>), 84_usize).write(code);
    } else if ((marker as i32) == 129) {
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xec_64.with(|v| v.as_pointer()) as Ptr<u8>)
                .offset((18) as isize)
                .get_offset()
                - (AppData_0xec_64.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xec_64.with(|v| v.as_pointer()) as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        elem!((s.as_pointer() as Ptr<u8>), 15_usize).write(code);
    } else {
        if !((marker as i32) == 130) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    197,
                    Ptr::<i8>::from_string_literal(b"GenerateAppMarker"),
                )
            });
            'loop_: while true {}
        };
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xee_65.with(|v| v.as_pointer()) as Ptr<u8>)
                .offset((15) as isize)
                .get_offset()
                - (AppData_0xee_65.with(|v| v.as_pointer()) as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xee_65.with(|v| v.as_pointer()) as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        elem!((s.as_pointer() as Ptr<u8>), 10_usize).write(code);
    }
    return std::mem::take(&mut (*s.borrow_mut()));
}
pub fn ProcessMetaData_149(
    mut data: Ptr<u8>,
    mut len: usize,
    mut state: Ptr<brunsli_internal_dec_MetadataState>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let mut pos: usize = 0_usize;
    'loop_: while (pos < len) {
        'switch: {
            match { state.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize) => {
                    let __rhs = (elem!(data, pos.postfix_inc()).read());
                    field!(state, marker).write(__rhs);
                    if ((state.with(|__s| __s.marker) as i32) == 217) {
                        (jpg.with(|__s| __s.tail_data.as_pointer()) as Ptr<Vec<u8>>)
                            .write(Vec::new());
                        field!(state, stage)
                            .write((brunsli_internal_dec_MetadataState_Stage_READ_TAIL as usize));
                        continue 'loop_;
                    } else if ((state.with(|__s| __s.marker) as i32) < 64) {
                        field!(state, short_marker_count).with_mut(|__v| __v.postfix_inc());
                        if ({ state.with(|__s| __s.short_marker_count) } > {
                            (kBrunsliShortMarkerLimit_23.with(|rc| *rc.borrow()) as usize)
                        }) {
                            return false;
                        }
                        (jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>)
                            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                                __v.push(Rc::new(RefCell::new(
                                    ({ GenerateApp0Marker_147(state.with(|__s| __s.marker)) }),
                                )))
                            });
                        continue 'loop_;
                    } else if ((state.with(|__s| __s.marker) as i32) >= 128)
                        && ((state.with(|__s| __s.marker) as i32) <= 130)
                    {
                        field!(state, short_marker_count).with_mut(|__v| __v.postfix_inc());
                        if ({ state.with(|__s| __s.short_marker_count) } > {
                            (kBrunsliShortMarkerLimit_23.with(|rc| *rc.borrow()) as usize)
                        }) {
                            return false;
                        }
                        field!(state, stage)
                            .write((brunsli_internal_dec_MetadataState_Stage_READ_CODE as usize));
                        continue 'loop_;
                    }
                    if ((state.with(|__s| __s.marker) as i32) != 254)
                        && (((state.with(|__s| __s.marker) as i32) >> 4_u32) != 14)
                    {
                        return false;
                    }
                    field!(state, stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_HI as usize));
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_TAIL as usize) => {
                    ({
                        let _begin: Ptr<u8> = data.offset((pos) as isize);
                        let _end: Ptr<u8> = data.offset((len) as isize);
                        Append_71((jpg.with(|__s| __s.tail_data.as_pointer())), _begin, _end)
                    });
                    pos = len;
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_CODE as usize) => {
                    let mut code: u8 = (elem!(data, pos.postfix_inc()).read());
                    (jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>)
                        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                            __v.push(Rc::new(RefCell::new(
                                ({ GenerateAppMarker_148(state.with(|__s| __s.marker), code) }),
                            )))
                        });
                    field!(state, stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_HI as usize) =>
                {
                    let __rhs = (elem!(data, pos.postfix_inc()).read());
                    field!(state, length_hi).write(__rhs);
                    field!(state, stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_LO as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_LO as usize) =>
                {
                    let mut lo: u8 = (elem!(data, pos.postfix_inc()).read());
                    let mut marker_len: usize = (({
                        ((state.with(|__s| __s.length_hi) as i32) << 8_u32)
                    } + { (lo as i32) }) as usize);
                    if (marker_len < 2_usize) {
                        return false;
                    }
                    field!(state, remaining_multibyte_length)
                        .write((marker_len).wrapping_sub(2_usize));
                    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
                        state.with(|__s| __s.marker),
                        state.with(|__s| __s.length_hi),
                        lo,
                    ])));
                    let mut dest: Ptr<Vec<Value<Vec<u8>>>> =
                        if ((state.with(|__s| __s.marker) as i32) == 254) {
                            (jpg.with(|__s| __s.com_data.as_pointer()))
                        } else {
                            (jpg.with(|__s| __s.app_data.as_pointer()))
                        };
                    let mut delta: usize = if ((state.with(|__s| __s.marker) as i32) == 254) {
                        0_usize
                    } else {
                        state.with(|__s| __s.short_marker_count)
                    };
                    if ({ ((*dest.upgrade().deref()).len() as u64).wrapping_sub((delta as u64)) }
                        >= { (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as u64) })
                    {
                        return false;
                    }
                    {
                        let __init = {
                            let __count = (head.as_pointer() as Ptr<u8>)
                                .offset((3) as isize)
                                .get_offset()
                                - (head.as_pointer() as Ptr<u8>).get_offset();
                            PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                                .map(|item| u8::try_from(item).ok().unwrap())
                                .collect::<Vec<_>>()
                        };
                        ((dest).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
                            |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(__init))),
                        )
                    };
                    let __rhs = ((*dest.upgrade().deref())[(*dest.upgrade().deref()).len() - 1]
                        .as_pointer());
                    field!(state, multibyte_sink).write(__rhs);
                    field!(state, stage).write({
                        (if (state.with(|__s| __s.remaining_multibyte_length) > 0_usize) {
                            brunsli_internal_dec_MetadataState_Stage_READ_MULTIBYTE
                        } else {
                            brunsli_internal_dec_MetadataState_Stage_READ_MARKER
                        } as usize)
                    });
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_MetadataState_Stage_READ_MULTIBYTE as usize) =>
                {
                    let mut chunk_size: usize = ({
                        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                            (state.with(|__s| __s.remaining_multibyte_length) as u64),
                        ));
                        let __tmp_1: Value<u64> =
                            Rc::new(RefCell::new(((len).wrapping_sub(pos) as u64)));
                        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                            __tmp_0.as_pointer()
                        } else {
                            __tmp_1.as_pointer()
                        }
                        .read())
                    } as usize);
                    ({
                        Append_72(
                            state.with(|__s| __s.multibyte_sink.clone()),
                            data.offset((pos) as isize),
                            chunk_size,
                        )
                    });
                    field!(state, remaining_multibyte_length).write({
                        (state.with(|__s| __s.remaining_multibyte_length)).wrapping_sub(chunk_size)
                    });
                    pos = { (pos).wrapping_add(chunk_size) };
                    if (state.with(|__s| __s.remaining_multibyte_length) == 0_usize) {
                        field!(state, stage)
                            .write((brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize));
                    };
                    continue 'loop_;
                }
                _ => {
                    return false;
                }
            }
        };
    }
    return true;
}
pub fn DecodeHuffmanCode_150(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> = field_ptr!(s, internals);
    let mut br: Ptr<brunsli_BrunsliBitReader> = (field_ptr!(js, br));
    'loop_: while true {
        'switch: {
            match { (js.with(|__s| __s.stage) as i32) } {
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST as i32) =>
                {
                    if !({ BrunsliBitReaderCanRead_134((br).clone(), 1_usize) }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    field!(js, is_known_last_huffman_code)
                        .write((({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as usize));
                    {
                        let __init = <brunsli_JPEGHuffmanCode>::default();
                        (*jpg.with(|__s| __s.huffman_code.clone()).borrow_mut()).push(__init)
                    };
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_SIMPLE);
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_SIMPLE
                        as i32) =>
                {
                    if !({
                        BrunsliBitReaderCanRead_134(
                            (br).clone(),
                            ((5 + (!(js.with(|__s| __s.is_known_last_huffman_code) != 0) as i32))
                                as usize),
                        )
                    }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let mut huff: Ptr<brunsli_JPEGHuffmanCode> = ((jpg
                        .with(|__s| __s.huffman_code.as_pointer())
                        as Ptr<brunsli_JPEGHuffmanCode>)
                        .to_last());
                    let __rhs = (({ BrunsliBitReaderRead_126((br).clone(), 2_u32) }) as i32);
                    field!(huff, slot_id).write(__rhs);
                    field!(js, is_dc_table)
                        .write((({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) == 0_u32));
                    {
                        let __rhs = if js.with(|__s| __s.is_dc_table) {
                            0
                        } else {
                            16
                        };
                        field!(huff, slot_id).with_mut(|__v| *__v = *__v + __rhs)
                    };
                    let __rhs = (js.with(|__s| __s.is_known_last_huffman_code) != 0)
                        || (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0);
                    field!(huff, is_last).write(__rhs);
                    elem!(
                        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                        0_usize
                    )
                    .write(0);
                    let mut found_match: i32 =
                        (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as i32);
                    if (found_match != 0) {
                        if js.with(|__s| __s.is_dc_table) {
                            let mut huff_table_idx: i32 =
                                (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as i32);
                            {
                                (((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeCounts_54.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset(huff_table_idx)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as usize,
                                    );
                                (((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                            {
                                (((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeValues_55.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset(huff_table_idx)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 13]>() as usize,
                                    );
                                (((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                        } else {
                            let mut huff_table_idx: i32 =
                                (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as i32);
                            {
                                (((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeCounts_57.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset(huff_table_idx)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as usize,
                                    );
                                (((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                            {
                                (((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeValues_59.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset(huff_table_idx)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 163]>() as usize,
                                    );
                                (((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                        }
                        field!(js, stage)
                            .write(brunsli_internal_dec_JpegInternalsState_Stage_HUFFMAN_UPDATE);
                    } else {
                        ({
                            let _values: Vec<u8> = if js.with(|__s| __s.is_dc_table) {
                                {
                                    let __count = (kDefaultDCValues_60.with(|v| v.as_pointer())
                                        as Ptr<u8>)
                                        .to_end()
                                        .get_offset()
                                        - (kDefaultDCValues_60.with(|v| v.as_pointer()) as Ptr<u8>)
                                            .get_offset();
                                    PtrValueIter::new(
                                        &(kDefaultDCValues_60.with(|v| v.as_pointer()) as Ptr<u8>),
                                        __count,
                                    )
                                    .collect::<Vec<_>>()
                                }
                            } else {
                                {
                                    let __count = (kDefaultACValues_61.with(|v| v.as_pointer())
                                        as Ptr<u8>)
                                        .to_end()
                                        .get_offset()
                                        - (kDefaultACValues_61.with(|v| v.as_pointer()) as Ptr<u8>)
                                            .get_offset();
                                    PtrValueIter::new(
                                        &(kDefaultACValues_61.with(|v| v.as_pointer()) as Ptr<u8>),
                                        __count,
                                    )
                                    .collect::<Vec<_>>()
                                }
                            };
                            brunsli_PermutationCoderImpl::Init(&field_ptr!(js, p), _values)
                        });
                        field!(js, stage).write(
                            brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_MAX_LEN,
                        );
                    };
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_MAX_LEN
                        as i32) =>
                {
                    if !({ BrunsliBitReaderCanRead_134((br).clone(), 4_usize) }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    field!(js, max_len).write(
                        ((({ BrunsliBitReaderRead_126((br).clone(), 4_u32) }).wrapping_add(1_u32))
                            as usize),
                    );
                    field!(js, total_count).write(0_usize);
                    field!(js, max_count).write({
                        (if js.with(|__s| __s.is_dc_table) {
                            kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())
                        } else {
                            kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())
                        } as usize)
                    });
                    field!(js, space).write({
                        ((((1_u32 << kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) as u32)
                            .wrapping_sub(
                                (1_u32
                                    << ((kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())
                                        as usize)
                                        .wrapping_sub(js.with(|__s| __s.max_len)))),
                            )) as usize)
                    });
                    field!(js, i).write(1_usize);
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_COUNT);
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_COUNT
                        as i32) =>
                {
                    let mut huff: Ptr<brunsli_JPEGHuffmanCode> = ((jpg
                        .with(|__s| __s.huffman_code.as_pointer())
                        as Ptr<brunsli_JPEGHuffmanCode>)
                        .to_last());
                    if ({ js.with(|__s| __s.i) } <= { js.with(|__s| __s.max_len) }) {
                        let mut shift: usize = (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())
                            as usize)
                            .wrapping_sub(js.with(|__s| __s.i));
                        let mut count_limit: usize = ({
                            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                                ((js.with(|__s| __s.max_count))
                                    .wrapping_sub(js.with(|__s| __s.total_count))
                                    as u64),
                            ));
                            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                                (({ js.with(|__s| __s.space) } >> { shift }) as u64),
                            ));
                            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                                __tmp_0.as_pointer()
                            } else {
                                __tmp_1.as_pointer()
                            }
                            .read())
                        } as usize);
                        if (count_limit > 0_usize) {
                            let mut nbits: i32 =
                                (({ Log2FloorNonZero_74((count_limit as u32)) }) + 1);
                            if !({ BrunsliBitReaderCanRead_134((br).clone(), (nbits as usize)) }) {
                                return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                            }
                            let mut count: usize =
                                (({ BrunsliBitReaderRead_126((br).clone(), (nbits as u32)) })
                                    as usize);
                            if (count > count_limit) {
                                return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                            }
                            elem!(
                                (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                                js.with(|__s| __s.i)
                            )
                            .write((count as i32));
                            field!(js, total_count)
                                .write({ (js.with(|__s| __s.total_count)).wrapping_add(count) });
                            field!(js, space).write({
                                (js.with(|__s| __s.space))
                                    .wrapping_sub((count).wrapping_mul((1_usize << shift)))
                            });
                        }
                        field!(js, i).with_mut(|__v| __v.prefix_inc());
                        continue 'loop_;
                    }
                    elem!(
                        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                        js.with(|__s| __s.max_len)
                    )
                    .with_mut(|__v| __v.prefix_inc());
                    field!(js, i).write(0_usize);
                    field!(js, stage).write(
                        brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_PERMUTATION,
                    );
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_PERMUTATION
                        as i32) =>
                {
                    let mut huff: Ptr<brunsli_JPEGHuffmanCode> = ((jpg
                        .with(|__s| __s.huffman_code.as_pointer())
                        as Ptr<brunsli_JPEGHuffmanCode>)
                        .to_last());
                    if ({ js.with(|__s| __s.i) } < { js.with(|__s| __s.total_count) }) {
                        let mut nbits: i32 =
                            ({ brunsli_PermutationCoderImpl::num_bits(&field_ptr!(js, p)) });
                        if !({
                            DecodeLimitedVarint_145(
                                (field_ptr!(js, varint)),
                                (br).clone(),
                                (((nbits + 1) >> 1_u32) as usize),
                            )
                        }) {
                            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let value: Value<u8> = Rc::new(RefCell::new(0_u8));
                        if !({
                            let _code: usize = js.with(|__s| __s.varint.value);
                            brunsli_PermutationCoderImpl::Remove(
                                &field_ptr!(js, p),
                                _code,
                                (value.as_pointer()),
                            )
                        }) {
                            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                        }
                        elem!(
                            (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                            js.with(|__s| __s.i)
                        )
                        .write(((*value.borrow()) as i32));
                        field!(js, i).with_mut(|__v| __v.prefix_inc());
                        continue 'loop_;
                    }
                    elem!(
                        (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                        js.with(|__s| __s.total_count)
                    )
                    .write(kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()));
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_HUFFMAN_UPDATE);
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_HUFFMAN_UPDATE as i32) =>
                {
                    if (jpg.with(|__s| __s.huffman_code.as_pointer())
                        as Ptr<brunsli_JPEGHuffmanCode>)
                        .to_last()
                        .with(|__s| __s.is_last)
                    {
                        field!(js, terminal_huffman_code_count).with_mut(|__v| __v.postfix_inc());
                    }
                    if (js.with(|__s| __s.is_known_last_huffman_code) != 0) {
                        ({ brunsli_PermutationCoderImpl::Clear(&field_ptr!(js, p)) });
                        return brunsli_BrunsliStatus_BRUNSLI_OK;
                    }
                    if ({ (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len() } >= {
                        (kMaxDHTMarkers_10.with(|rc| *rc.borrow()) as usize)
                    }) {
                        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                    }
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST);
                    continue 'loop_;
                }
                _ => {
                    return brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR;
                }
            }
        };
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeScanInfo_151(
    mut state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> = field_ptr!(s, internals);
    let mut br: Ptr<brunsli_BrunsliBitReader> = (field_ptr!(js, br));
    let maybe_add_zero_run: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let js: Ptr<brunsli_internal_dec_JpegInternalsState> = (js).clone();
            let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new((*jpg.borrow()).clone()));
        },
        || {
            if (js.with(|__s| __s.last_num) > 0) {
                let info: Value<brunsli_JPEGScanInfo_ExtraZeroRunInfo> = Rc::new(RefCell::new(
                    <brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default(),
                ));
                (*info.borrow_mut()).block_idx = js.with(|__s| __s.last_block_idx);
                (*info.borrow_mut()).num_extra_zero_runs = js.with(|__s| __s.last_num);
                {
                    let a0_clone = (*info.borrow()).clone();
                    (*{
                        (*elem!(
                            ((*jpg.borrow()).with(|__s| __s.scan_info.as_pointer())
                                as Ptr<brunsli_JPEGScanInfo>),
                            js.with(|__s| __s.i)
                        )
                        .upgrade()
                        .deref())
                        .extra_zero_runs
                        .clone()
                    }
                    .borrow_mut())
                    .push(a0_clone)
                };
                field!(js, last_num).write(0);
            }
        }
    )));
    'loop_: while true {
        'switch: {
            match { ( ( js.with(|__s| __s . stage ) as i32 ) )  } { __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON as i32 ) )  =>  { { let mut si : Ptr<brunsli_JPEGScanInfo>  = ( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  )  ;
  ;
 ;
 if ! ( ( { BrunsliBitReaderCanRead_134 ( (br ).clone() , 22_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 6_u32  , ) } )  as i32 ) )  ;
 field!(si , Ss) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 6_u32  , ) } )  as i32 ) )  ;
 field!(si , Se) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 4_u32  , ) } )  as i32 ) )  ;
 field!(si , Ah) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 4_u32  , ) } )  as i32 ) )  ;
 field!(si , Al) .write( __rhs ) ;
 let __rhs = ( ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 2_u32  , ) } )  ) . wrapping_add ( 1_u32 ) ) as usize )  ;
 field!(si , num_components) .write( __rhs ) ;
 field!(js, j) .write( 0_usize  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMPONENT  ) ;
 ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMPONENT as i32 ) )  =>  { { let mut si : Ptr<brunsli_JPEGScanInfo>  = ( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  )  ;
  ;
 ;
 if ({ js.with(|__s| __s . j )  } < { si .with(|__s| __s . num_components )  }) { if ! ( ( { BrunsliBitReaderCanRead_134 ( (br ).clone() , 6_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 2_u32  , ) } )  as u8 ) )  ;
 field!(elem!((si .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), comp_idx) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 2_u32  , ) } )  as i32 ) )  ;
 field!(elem!((si .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), dc_tbl_idx) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 2_u32  , ) } )  as i32 ) )  ;
 field!(elem!((si .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), ac_tbl_idx) .write( __rhs ) ;
 field!(js, j) .with_mut(|__v| __v. postfix_inc ()) ;
 } else { field!(js, last_block_idx) .write( - 1_i32  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION  ) ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION as i32 ) )  =>  { { if ! ( ( { BrunsliBitReaderCanRead_134 ( (br ).clone() , 1_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } if ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 1_u32  , ) } )  != 0 ) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_DATA  ) ;
 } else { field!(js, last_block_idx) .write( 0  ) ;
 field!(js, last_num) .write( 0  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION  ) ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_DATA as i32 ) )  =>  { { let mut si : Ptr<brunsli_JPEGScanInfo>  = ( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  )  ;
  ;
 ;
 if ! ( ( { DecodeVarint_144 ( ( field_ptr!(js , varint)  )  , (br ).clone() , 28_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let block_idx : Value<i32 > = Rc::new(RefCell::new(( ({ js.with(|__s| __s . last_block_idx )  } + { ( ( js.with(|__s| __s . varint  . value ) as i32 ) )  }) + 1 ) )) ;
  ;
 ;
 {let __init = (*block_idx.borrow()) ;
    (*si .with(|__s| __s . reset_points .clone()).borrow_mut()) .push(__init)}  ;
 field!(js, last_block_idx) .write( (*block_idx.borrow())  ) ;
 if ( js.with(|__s| __s . last_block_idx ) > ( ( 1 << 30 ) ) ) { return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN   ;
 } field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION  ) ;
 ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION as i32 ) )  =>  { { if ! ( ( { BrunsliBitReaderCanRead_134 ( (br ).clone() , 1_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } if ( ( { BrunsliBitReaderRead_126 ( (br ).clone() , 1_u32  , ) } )  != 0 ) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_DATA  ) ;
 } else { ( { (*maybe_add_zero_run.borrow())  .call ( ) } ) ;
 field!(js, i) .with_mut(|__v| __v. prefix_inc ()) ;
 if ({ js.with(|__s| __s . i )  } < { js.with(|__s| __s . num_scans )  }) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON  ) ;
 ;
 continue 'loop_ ;
 } return brunsli_BrunsliStatus_BRUNSLI_OK   ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_DATA as i32 ) )  =>  { { if ! ( ( { DecodeVarint_144 ( ( field_ptr!(js , varint)  )  , (br ).clone() , 28_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let mut block_idx : i32  = ({ js.with(|__s| __s . last_block_idx )  } + { ( ( js.with(|__s| __s . varint  . value ) as i32 ) )  })  ;
  ;
 ;
 if ({ block_idx  } > { js.with(|__s| __s . last_block_idx )  }) { ( { (*maybe_add_zero_run.borrow())  .call ( ) } ) ;
 } field!(js, last_num) .with_mut(|__v| __v. prefix_inc ()) ;
 field!(js, last_block_idx) .write( block_idx  ) ;
 if ( js.with(|__s| __s . last_block_idx ) > ( ( 1 << 30 ) ) ) { return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN   ;
 } field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION  ) ;
 ;
 continue 'loop_ ;
 } }, _ =>  { return brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR   ;
 }, }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeCoeffOrder_152(
    mut order: Ptr<u32>,
    mut br: Ptr<brunsli_BitSource>,
    mut in_: Ptr<brunsli_WordSource>,
) -> bool {
    let lehmer: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ])));
    thread_local!(
        static kSpan_153: Value<i32> = Rc::new(RefCell::new(16));
    );
    let mut i: i32 = 0;
    'loop_: while (i < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        if !(({ brunsli_BitSourceImpl::ReadBits(&br, 1, (in_).clone()) }) != 0) {
            i += kSpan_153.with(|rc| *rc.borrow());
            continue 'loop_;
        }
        let mut start: i32 = if (i > 0) { i } else { 1 };
        let mut end: i32 = (i + kSpan_153.with(|rc| *rc.borrow()));
        let mut j: i32 = start;
        'loop_: while (j < end) {
            let mut v: u32 = 0_u32;
            'loop_: while (v <= (kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)) {
                let mut bits: u32 = ({ brunsli_BitSourceImpl::ReadBits(&br, 3, (in_).clone()) });
                v = { (v).wrapping_add(bits) };
                if (bits < 7_u32) {
                    break;
                }
            }
            if (v > (kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)) {
                return false;
            }
            (*lehmer.borrow_mut())[(j) as usize] = v;
            j.prefix_inc();
        }
        i += kSpan_153.with(|rc| *rc.borrow());
    }
    let mut end: i32 = (kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1);
    'loop_: while (end >= 1) && ((*lehmer.borrow())[(end) as usize] == 0_u32) {
        end.prefix_dec();
    }
    if ((*lehmer.borrow())[(end) as usize] == 1_u32) {
        return false;
    }
    let mut i: i32 = 1;
    'loop_: while (i <= end) {
        if ((*lehmer.borrow())[(i) as usize] == 0_u32) {
            return false;
        }
        (*lehmer.borrow_mut())[(i) as usize].prefix_dec();
        i.prefix_inc();
    }
    if !({
        DecodeLehmerCode_113(
            (lehmer.as_pointer() as Ptr<u32>),
            (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize),
            (order).clone(),
        )
    }) {
        return false;
    }
    let mut k: i32 = 0;
    'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        elem!(order, k).write({
            ({
                let __idx = (elem!(order, k).read()) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            })
        });
        k.prefix_inc();
    }
    return true;
}
pub fn DecodeNumNonzeros_154(
    mut p: Ptr<brunsli_Prob>,
    mut ac: Ptr<brunsli_BinaryArithmeticDecoder>,
    mut in_: Ptr<brunsli_WordSource>,
) -> usize {
    let mut bst: Ptr<brunsli_Prob> = p.offset(-((1) as isize));
    let mut ctx: usize = 1_usize;
    let mut b: usize = 0_usize;
    'loop_: while (b < kNumNonZeroBits_84.with(|rc| *rc.borrow())) {
        let mut bit: i32 = ({
            brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                &ac,
                (({ brunsli_ProbImpl::get_proba(&bst.offset((ctx) as isize)) }) as i32),
                (in_).clone(),
            )
        });
        ({
            let _val: i32 = bit;
            brunsli_ProbImpl::Add(&bst.offset((ctx) as isize), _val)
        });
        ctx = { ((2_usize).wrapping_mul(ctx)).wrapping_add((bit as usize)) };
        b.prefix_inc();
    }
    let mut val: usize =
        (ctx).wrapping_sub(((1_u32 << kNumNonZeroBits_84.with(|rc| *rc.borrow())) as usize));
    if !(val <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                593,
                Ptr::<i8>::from_string_literal(b"DecodeNumNonzeros"),
            )
        });
        'loop_: while true {}
    };
    return val;
}
pub fn EnsureSubdecodersInitialized_155(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut in_: Ptr<brunsli_WordSource>,
) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    if !(s.with(|__s| __s.subdecoders_initialized)) {
        ({ brunsli_ANSDecoderImpl::Init(&field_ptr!(s, ans_decoder), (in_).clone()) });
        ({ brunsli_BitSourceImpl::Init(&field_ptr!(s, bit_reader), (in_).clone()) });
        ({
            brunsli_BinaryArithmeticDecoderImpl::Init(&field_ptr!(s, arith_decoder), (in_).clone())
        });
        field!(s, subdecoders_initialized).write(true);
    }
}
pub fn FinalizeSubdecoders_156(mut state: Ptr<brunsli_internal_dec_State>) -> bool {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    if !({ brunsli_ANSDecoderImpl::CheckCRC(&field_ptr!(s, ans_decoder)) }) {
        return false;
    }
    if !({ brunsli_BitSourceImpl::Finish(&field_ptr!(s, bit_reader)) }) {
        return false;
    }
    field!(s, subdecoders_initialized).write(false);
    return true;
}
pub fn DecodeDC_157(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    let mut mcu_rows: i32 = ({
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .height_in_blocks
        }
    } / {
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp
        }
    });
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> = field_ptr!(s, ac_dc);
    let comps: Ptr<Vec<brunsli_ComponentStateDC>> = ac_dc_state.with(|__s| __s.dc.as_pointer());
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = num_components as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| {
                __v.resize_with(__a0, || <brunsli_ComponentStateDC>::default())
            })
        };
        let mut c: usize = 0_usize;
        'loop_: while (c < num_components) {
            ({
                let _w: i32 = {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        c
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                };
                brunsli_ComponentStateDCImpl::SetWidth(
                    &(Ptr::<Vec<brunsli_ComponentStateDC>>::decay(&(comps))
                        as Ptr<brunsli_ComponentStateDC>)
                        .offset(c),
                    _w,
                )
            });
            c.prefix_inc();
        }
    }
    if !({ brunsli_WordSourceImpl::CanRead(&in_, 5_usize) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({ EnsureSubdecodersInitialized_155((state).clone(), (in_).clone()) });
    let ans: Value<brunsli_ANSDecoder> =
        Rc::new(RefCell::new(s.with(|__s| __s.ans_decoder.clone())));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new(s.with(|__s| __s.bit_reader.clone())));
    let ac: Value<brunsli_BinaryArithmeticDecoder> =
        Rc::new(RefCell::new(s.with(|__s| __s.arith_decoder.clone())));
    let mut mcu_y: i32 = ac_dc_state.with(|__s| __s.next_mcu_y);
    'loop_: while (mcu_y < mcu_rows) {
        let mut i: usize = ac_dc_state.with(|__s| __s.next_component);
        'loop_: while (i < num_components) {
            let mut c: Ptr<brunsli_ComponentStateDC> =
                ((Ptr::<Vec<brunsli_ComponentStateDC>>::decay(&(comps))
                    as Ptr<brunsli_ComponentStateDC>)
                    .offset(i));
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset(i);
            let mut context_map: Ptr<u8> = state
                .with(|__s| __s.context_map.clone())
                .offset(((i).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow()))) as isize);
            let mut ac_stride: i32 = (m.with(|__s| __s.ac_stride) as i32);
            let mut b_stride: usize = (m.with(|__s| __s.b_stride) as usize);
            let mut width: i32 = m.with(|__s| __s.width_in_blocks);
            let mut y: i32 = ({ ({ mcu_y } * { m.with(|__s| __s.v_samp) }) } + {
                ac_dc_state.with(|__s| __s.next_iy)
            });
            let mut prev_sgn: Ptr<i32> =
                ((c.with(|__s| __s.prev_sign.as_pointer()) as Ptr<i32>).offset(1_usize));
            let mut prev_abs: Ptr<i32> =
                ((c.with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>).offset(2_usize));
            let mut iy: i32 = ac_dc_state.with(|__s| __s.next_iy);
            'loop_: while ({ iy } < { m.with(|__s| __s.v_samp) }) {
                let mut coeffs: Ptr<i16> = m
                    .with(|__s| __s.ac_coeffs.clone())
                    .offset((y * ac_stride) as isize)
                    .offset(
                        ({ ac_dc_state.with(|__s| __s.next_x) } * {
                            kDCTBlockSize_3.with(|rc| *rc.borrow())
                        }) as isize,
                    );
                let mut block_state: Ptr<u8> = m
                    .with(|__s| __s.block_state.clone())
                    .offset(((y as usize).wrapping_mul(b_stride)) as isize)
                    .offset((ac_dc_state.with(|__s| __s.next_x)) as isize);
                let mut x: i32 = ac_dc_state.with(|__s| __s.next_x);
                'loop_: while (x < width) {
                    if ((!({ brunsli_WordSourceImpl::CanRead(&in_, 6_usize) }) as i64) != 0) {
                        field!(ac_dc_state, next_mcu_y).write(mcu_y);
                        field!(ac_dc_state, next_component).write(i);
                        field!(ac_dc_state, next_iy).write(iy);
                        field!(ac_dc_state, next_x).write(x);
                        field!(s, ans_decoder).write((*ans.borrow()).clone());
                        field!(s, bit_reader).write((*br.borrow()).clone());
                        field!(s, arith_decoder).write((*ac.borrow()).clone());
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let mut is_empty_ctx: i32 = ({
                        IsEmptyBlockContext_106(
                            ((c.with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>)
                                .offset(1_usize)),
                            x,
                        )
                    });
                    let mut is_empty_p: Ptr<brunsli_Prob> =
                        ((c.with(|__s| __s.is_empty_block_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset((is_empty_ctx as usize)));
                    let mut is_empty_block: bool = !(({
                        brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                            &ac.as_pointer(),
                            (({ brunsli_ProbImpl::get_proba(&is_empty_p) }) as i32),
                            (in_).clone(),
                        )
                    }) != 0);
                    ({ brunsli_ProbImpl::Add(&is_empty_p, (!(is_empty_block) as i32)) });
                    elem!(
                        (c.with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>),
                        ((x + 1) as usize)
                    )
                    .write((!(is_empty_block) as i32));
                    block_state.write({ (is_empty_block as u8) });
                    let mut abs_val: i32 = 0;
                    let mut sign: i32 = 0;
                    if !(is_empty_block) {
                        let mut p_is_zero: Ptr<brunsli_Prob> = (field_ptr!(c, is_zero_prob));
                        let mut is_zero: i32 = ({
                            brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                &ac.as_pointer(),
                                (({ brunsli_ProbImpl::get_proba(&p_is_zero) }) as i32),
                                (in_).clone(),
                            )
                        });
                        ({ brunsli_ProbImpl::Add(&p_is_zero, is_zero) });
                        if !(is_zero != 0) {
                            let mut avg_ctx: i32 = ({
                                let _vals: Ptr<i32> = (prev_abs).clone();
                                let _x: i32 = x;
                                WeightedAverageContextDC_97(_vals, _x)
                            });
                            let mut sign_ctx: i32 = (((elem!(prev_sgn, x).read()) * 3)
                                + (elem!(prev_sgn, (x - 1)).read()));
                            let mut sign_p: Ptr<brunsli_Prob> =
                                ((c.with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                                    .offset((sign_ctx as usize)));
                            sign = ({
                                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                    &ac.as_pointer(),
                                    (({ brunsli_ProbImpl::get_proba(&sign_p) }) as i32),
                                    (in_).clone(),
                                )
                            });
                            ({ brunsli_ProbImpl::Add(&sign_p, sign) });
                            let mut entropy_ix: i32 = ((elem!(context_map, avg_ctx).read()) as i32);
                            let mut code: i32 = ({
                                let _code: Ptr<brunsli_ANSDecodingData> = state
                                    .with(|__s| __s.entropy_codes.clone())
                                    .offset((entropy_ix) as isize);
                                let _in_: Ptr<brunsli_WordSource> = (in_).clone();
                                brunsli_ANSDecoderImpl::ReadSymbol(&ans.as_pointer(), _code, _in_)
                            });
                            if (code < kNumDirectCodes_135.with(|rc| *rc.borrow())) {
                                abs_val = (code + 1);
                            } else {
                                let mut nbits: i32 =
                                    (code - kNumDirectCodes_135.with(|rc| *rc.borrow()));
                                let mut p_first_extra_bit: Ptr<brunsli_Prob> = ((c
                                    .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset((nbits as usize)));
                                let mut first_extra_bit: i32 = ({
                                    brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                        &ac.as_pointer(),
                                        (({ brunsli_ProbImpl::get_proba(&p_first_extra_bit) })
                                            as i32),
                                        (in_).clone(),
                                    )
                                });
                                ({ brunsli_ProbImpl::Add(&p_first_extra_bit, first_extra_bit) });
                                let mut extra_bits_val: i32 = (first_extra_bit << nbits);
                                if (nbits > 0) {
                                    extra_bits_val |= (({
                                        brunsli_BitSourceImpl::ReadBits(
                                            &br.as_pointer(),
                                            nbits,
                                            (in_).clone(),
                                        )
                                    })
                                        as i32);
                                }
                                abs_val = (((kNumDirectCodes_135.with(|rc| *rc.borrow()) - 1)
                                    + (2 << nbits))
                                    + extra_bits_val);
                            }
                        }
                    }
                    elem!(prev_abs, x).write({ abs_val });
                    elem!(prev_sgn, x).write({ if (abs_val != 0) { (sign + 1) } else { 0 } });
                    let __rhs = (({ ((1 - (2 * sign)) * abs_val) } + {
                        ({ PredictWithAdaptiveMedian_115((coeffs).clone(), x, y, ac_stride) })
                    }) as i16);
                    elem!(coeffs, 0).write(__rhs);
                    block_state.postfix_inc();
                    coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    x.prefix_inc();
                }
                field!(ac_dc_state, next_x).write(0);
                {
                    iy.prefix_inc();
                    y.prefix_inc()
                };
            }
            field!(ac_dc_state, next_iy).write(0);
            i.prefix_inc();
        }
        field!(ac_dc_state, next_component).write(0_usize);
        mcu_y.prefix_inc();
    }
    field!(ac_dc_state, next_mcu_y).write(0);
    field!(ac_dc_state, next_component).write(0_usize);
    field!(ac_dc_state, next_iy).write(0);
    field!(ac_dc_state, next_x).write(0);
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| __v.clear());
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| __v.shrink_to_fit());
    field!(s, ans_decoder).write((*ans.borrow()).clone());
    field!(s, bit_reader).write((*br.borrow()).clone());
    field!(s, arith_decoder).write((*ac.borrow()).clone());
    if !({ FinalizeSubdecoders_156((state).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeEmptyAcBlock_158(mut prev_sgn: Ptr<i32>, mut prev_abs: Ptr<i32>) {
    let mut k: i32 = 1;
    'loop_: while (k < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        elem!(prev_sgn, k).write(0);
        elem!(prev_abs, k).write(0);
        k.prefix_inc();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(184)]
pub struct brunsli_AcBlockCookie {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub prev_num_nonzeros: Ptr<u8>,
    #[offset(16)]
    #[byte_size(8)]
    pub prev_sgn: Ptr<i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub prev_abs: Ptr<i32>,
    #[offset(32)]
    #[byte_size(8)]
    pub num_nonzero_prob: Ptr<brunsli_Prob>,
    #[offset(40)]
    #[byte_size(8)]
    pub ac: Ptr<brunsli_BinaryArithmeticDecoder>,
    #[offset(48)]
    #[byte_size(8)]
    pub in_: Ptr<brunsli_WordSource>,
    #[offset(56)]
    #[byte_size(8)]
    pub ans: Ptr<brunsli_ANSDecoder>,
    #[offset(64)]
    #[byte_size(8)]
    pub br: Ptr<brunsli_BitSource>,
    #[offset(72)]
    #[byte_size(8)]
    pub coeffs: Ptr<i16>,
    #[offset(80)]
    #[byte_size(8)]
    pub prev_row_coeffs: Ptr<i16>,
    #[offset(88)]
    #[byte_size(8)]
    pub prev_col_coeffs: Ptr<i16>,
    #[offset(96)]
    #[byte_size(8)]
    pub is_zero_prob: Ptr<brunsli_Prob>,
    #[offset(104)]
    #[byte_size(8)]
    pub order: Ptr<u32>,
    #[offset(112)]
    #[byte_size(8)]
    pub context_modes: Ptr<u8>,
    #[offset(120)]
    #[byte_size(8)]
    pub mult_col: Ptr<i32>,
    #[offset(128)]
    #[byte_size(8)]
    pub mult_row: Ptr<i32>,
    #[offset(136)]
    pub prev_row_delta: i32,
    #[offset(144)]
    #[byte_size(8)]
    pub sign_prob: Ptr<brunsli_Prob>,
    #[offset(152)]
    pub context_bits: usize,
    #[offset(160)]
    #[byte_size(8)]
    pub context_map: Ptr<u8>,
    #[offset(168)]
    #[byte_size(8)]
    pub entropy_codes: Ptr<brunsli_ANSDecodingData>,
    #[offset(176)]
    #[byte_size(8)]
    pub first_extra_bit_prob: Ptr<brunsli_Prob>,
}
pub fn DecodeAcBlock_159(cookie: Ptr<brunsli_AcBlockCookie>) -> usize {
    let mut c: brunsli_AcBlockCookie = (*cookie.upgrade().deref()).clone();
    let ac: Value<brunsli_BinaryArithmeticDecoder> =
        Rc::new(RefCell::new((*c.ac.upgrade().deref()).clone()));
    let mut in_: Ptr<brunsli_WordSource> = (c.in_).clone();
    let ans: Value<brunsli_ANSDecoder> = Rc::new(RefCell::new((*c.ans.upgrade().deref()).clone()));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new((*c.br.upgrade().deref()).clone()));
    let mut num_nonzeros: usize = 0_usize;
    let mut nonzero_ctx: u8 = ({
        let _prev: Ptr<u8> = (c.prev_num_nonzeros).clone();
        let _x: i32 = c.x;
        let _y: i32 = c.y;
        NumNonzerosContext_104(_prev, _x, _y)
    });
    let mut last_nz: usize = ({
        DecodeNumNonzeros_154(
            c.num_nonzero_prob.offset(
                ((kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                    .wrapping_mul((nonzero_ctx as usize))) as isize,
            ),
            (ac.as_pointer()),
            (in_).clone(),
        )
    });
    let mut k: usize = (last_nz).wrapping_add(1_usize);
    'loop_: while (k < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        elem!(c.prev_sgn, k).write(0);
        elem!(c.prev_abs, k).write(0);
        k.prefix_inc();
    }
    let mut k: usize = last_nz;
    'loop_: while (k > 0_usize) {
        let mut is_zero: i32 = 0;
        if (k < last_nz) {
            let mut bucket: usize = (({
                let __idx = ((num_nonzeros).wrapping_sub(1_usize)) as usize;
                kNonzeroBuckets_89.with(|rc| rc.borrow()[__idx])
            }) as usize);
            let mut is_zero_ctx: usize = ((bucket)
                .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
            .wrapping_add(k);
            let p: Ptr<brunsli_Prob> = c.is_zero_prob.offset((is_zero_ctx) as isize);
            is_zero = ({
                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                    &ac.as_pointer(),
                    (({ brunsli_ProbImpl::get_proba(&p) }) as i32),
                    (in_).clone(),
                )
            });
            ({
                let _val: i32 = is_zero;
                brunsli_ProbImpl::Add(&p, _val)
            });
        }
        let mut abs_val: i32 = 0;
        let mut sign: i32 = 1;
        let mut k_nat: i32 = ((elem!(c.order, k).read()) as i32);
        if !(is_zero != 0) {
            let mut context_type: usize = ((elem!(c.context_modes, k_nat).read()) as usize);
            let avg_ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
            let sign_ctx: Value<usize> =
                Rc::new(RefCell::new(kMaxAverageContext_82.with(|rc| *rc.borrow())));
            if ((context_type & 1_usize) != 0) && (c.y > 0) {
                let mut offset: usize = ((k_nat & 7) as usize);
                ({
                    let _prev: Ptr<i16> = c.prev_row_coeffs.offset((offset) as isize);
                    let _cur: Ptr<i16> = c.coeffs.offset((offset) as isize);
                    let _mult: Ptr<i32> =
                        c.mult_col.offset(((offset).wrapping_mul(8_usize)) as isize);
                    ACPredictContextRow_103(
                        _prev,
                        _cur,
                        _mult,
                        (avg_ctx.as_pointer()),
                        (sign_ctx.as_pointer()),
                    )
                });
            } else if ((context_type & 2_usize) != 0) && (c.x > 0) {
                let mut offset: usize = ((k_nat & !7) as usize);
                ({
                    let _prev: Ptr<i16> = c.prev_col_coeffs.offset((offset) as isize);
                    let _cur: Ptr<i16> = c.coeffs.offset((offset) as isize);
                    let _mult: Ptr<i32> = c.mult_row.offset((offset) as isize);
                    ACPredictContextCol_102(
                        _prev,
                        _cur,
                        _mult,
                        (avg_ctx.as_pointer()),
                        (sign_ctx.as_pointer()),
                    )
                });
            } else if !(context_type != 0) {
                (*avg_ctx.borrow_mut()) = (({
                    let _vals: Ptr<i32> = c.prev_abs.offset((k) as isize);
                    let _prev_row_delta: i32 = c.prev_row_delta;
                    WeightedAverageContext_98(_vals, _prev_row_delta)
                }) as usize);
                (*sign_ctx.borrow_mut()) = (({ ((elem!(c.prev_sgn, k).read()) * 3) } + {
                    (elem!(
                        c.prev_sgn,
                        ((k as i32) - kDCTBlockSize_3.with(|rc| *rc.borrow()))
                    )
                    .read())
                }) as usize);
            }
            (*sign_ctx.borrow_mut()) = {
                ((*sign_ctx.borrow())
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                .wrapping_add(k)
            };
            let sign_p: Ptr<brunsli_Prob> = c.sign_prob.offset((*sign_ctx.borrow()) as isize);
            sign = ({
                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                    &ac.as_pointer(),
                    (({ brunsli_ProbImpl::get_proba(&sign_p) }) as i32),
                    (in_).clone(),
                )
            });
            ({
                let _val: i32 = sign;
                brunsli_ProbImpl::Add(&sign_p, _val)
            });
            elem!(c.prev_sgn, k).write({ (sign + 1) });
            sign = { (1 - (2 * sign)) };
            let mut z_dens_ctx: usize =
                (({ ZeroDensityContext_96(num_nonzeros, k, c.context_bits) }) as usize);
            let mut histo_ix: usize = ((z_dens_ctx)
                .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
            .wrapping_add((*avg_ctx.borrow()));
            let mut entropy_ix: usize = ((elem!(c.context_map, histo_ix).read()) as usize);
            let mut code: i32 = ({
                let _code: Ptr<brunsli_ANSDecodingData> =
                    c.entropy_codes.offset((entropy_ix) as isize);
                let _in_: Ptr<brunsli_WordSource> = (in_).clone();
                brunsli_ANSDecoderImpl::ReadSymbol(&ans.as_pointer(), _code, _in_)
            });
            if (code < kNumDirectCodes_135.with(|rc| *rc.borrow())) {
                abs_val = (code + 1);
            } else {
                let mut nbits: i32 = (code - kNumDirectCodes_135.with(|rc| *rc.borrow()));
                let p: Ptr<brunsli_Prob> = c
                    .first_extra_bit_prob
                    .offset((((k).wrapping_mul(10_usize)).wrapping_add((nbits as usize))) as isize);
                let mut first_extra_bit: i32 = ({
                    brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                        &ac.as_pointer(),
                        (({ brunsli_ProbImpl::get_proba(&p) }) as i32),
                        (in_).clone(),
                    )
                });
                ({
                    let _val: i32 = first_extra_bit;
                    brunsli_ProbImpl::Add(&p, _val)
                });
                let mut extra_bits_val: i32 = (first_extra_bit << nbits);
                if (nbits > 0) {
                    {
                        let rhs_0 = ((extra_bits_val as u32)
                            | ({
                                brunsli_BitSourceImpl::ReadBits(
                                    &br.as_pointer(),
                                    nbits,
                                    (in_).clone(),
                                )
                            })) as i32;
                        extra_bits_val = rhs_0
                    };
                }
                abs_val = (((((kNumDirectCodes_135.with(|rc| *rc.borrow()) - 1) as u32)
                    .wrapping_add((2_u32 << nbits)))
                .wrapping_add((extra_bits_val as u32))) as i32);
            }
            num_nonzeros.prefix_inc();
        } else {
            elem!(c.prev_sgn, k).write(0);
        }
        let mut coeff: i32 = (sign * abs_val);
        elem!(c.coeffs, k_nat).write({ (coeff as i16) });
        elem!(c.prev_abs, k).write({ abs_val });
        k.prefix_dec();
    }
    c.ans.write({ (*ans.borrow()).clone() });
    c.br.write({ (*br.borrow()).clone() });
    c.ac.write({ (*ac.borrow()).clone() });
    return num_nonzeros;
}
pub fn DecodeAC_160(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    let mut mcu_rows: i32 = ({
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .height_in_blocks
        }
    } / {
        {
            (*elem!(
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>),
                0_usize
            )
            .upgrade()
            .deref())
            .v_samp
        }
    });
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> = field_ptr!(s, ac_dc);
    let comps: Ptr<Vec<brunsli_ComponentState>> = ac_dc_state.with(|__s| __s.ac.as_pointer());
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = num_components as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| {
                __v.resize_with(__a0, || <brunsli_ComponentState>::default())
            })
        };
        let mut c: usize = 0_usize;
        'loop_: while (c < num_components) {
            ({
                let _w: i32 = {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        c
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                };
                brunsli_ComponentStateImpl::SetWidth(
                    &(Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                        as Ptr<brunsli_ComponentState>)
                        .offset(c),
                    _w,
                )
            });
            ({
                let _quant: Ptr<i32> = (({
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        c
                    )
                    .upgrade()
                    .deref())
                    .quant
                    .as_pointer()
                } as Ptr<i32>)
                    .offset(0_usize));
                let _mult_row: Ptr<i32> = (array_field_ptr!(
                    (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                        as Ptr<brunsli_ComponentState>)
                        .offset(c),
                    mult_row
                ) as Ptr<i32>);
                let _mult_col: Ptr<i32> = (array_field_ptr!(
                    (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                        as Ptr<brunsli_ComponentState>)
                        .offset(c),
                    mult_col
                ) as Ptr<i32>);
                ComputeACPredictMultipliers_109(_quant, _mult_row, _mult_col)
            });
            c.prefix_inc();
        }
    }
    if !({ brunsli_WordSourceImpl::CanRead(&in_, 5_usize) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({ EnsureSubdecodersInitialized_155((state).clone(), (in_).clone()) });
    if !(ac_dc_state.with(|__s| __s.ac_coeffs_order_decoded)) {
        'loop_: while ({ ac_dc_state.with(|__s| __s.next_component) } < { num_components }) {
            if !({ brunsli_WordSourceImpl::CanRead(&in_, 121_usize) }) {
                return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
            }
            if !({
                DecodeCoeffOrder_152(
                    (array_field_ptr!(
                        (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                            as Ptr<brunsli_ComponentState>)
                            .offset(ac_dc_state.with(|__s| __s.next_component)),
                        order
                    ) as Ptr<u32>),
                    (field_ptr!(s, bit_reader)),
                    (in_).clone(),
                )
            }) {
                return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
            }
            field!(ac_dc_state, next_component).with_mut(|__v| __v.postfix_inc());
        }
        field!(ac_dc_state, next_component).write(0_usize);
        field!(ac_dc_state, ac_coeffs_order_decoded).write(true);
    }
    let c: Value<brunsli_AcBlockCookie> = Rc::new(RefCell::new(<brunsli_AcBlockCookie>::default()));
    (*c.borrow_mut()).ac = (field_ptr!(s, arith_decoder));
    (*c.borrow_mut()).in_ = (in_).clone();
    (*c.borrow_mut()).ans = (field_ptr!(s, ans_decoder));
    (*c.borrow_mut()).br = (field_ptr!(s, bit_reader));
    (*c.borrow_mut()).entropy_codes = state.with(|__s| __s.entropy_codes.clone());
    (*c.borrow_mut()).context_modes = (kContextAlgorithm_95.with(|v| v.as_pointer()) as Ptr<u8>)
        .offset(
            (if state.with(|__s| __s.use_legacy_context_model) {
                64
            } else {
                0
            }) as isize,
        );
    let mut mcu_y: i32 = ac_dc_state.with(|__s| __s.next_mcu_y);
    'loop_: while (mcu_y < mcu_rows) {
        let mut i: usize = ac_dc_state.with(|__s| __s.next_component);
        'loop_: while (i < num_components) {
            let cst: Ptr<brunsli_ComponentState> =
                (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                    as Ptr<brunsli_ComponentState>)
                    .offset(i);
            (*c.borrow_mut()).prev_num_nonzeros =
                (cst.with(|__s| __s.prev_num_nonzeros.as_pointer()) as Ptr<u8>);
            (*c.borrow_mut()).num_nonzero_prob =
                (array_field_ptr!(cst, num_nonzero_prob) as Ptr<brunsli_Prob>);
            (*c.borrow_mut()).is_zero_prob =
                (cst.with(|__s| __s.is_zero_prob.as_pointer()) as Ptr<brunsli_Prob>);
            (*c.borrow_mut()).order = (array_field_ptr!(cst, order) as Ptr<u32>);
            (*c.borrow_mut()).mult_col = (array_field_ptr!(cst, mult_col) as Ptr<i32>);
            (*c.borrow_mut()).mult_row = (array_field_ptr!(cst, mult_row) as Ptr<i32>);
            (*c.borrow_mut()).sign_prob =
                (cst.with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>);
            (*c.borrow_mut()).first_extra_bit_prob =
                (cst.with(|__s| __s.first_extra_bit_prob.as_pointer()) as Ptr<brunsli_Prob>);
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset(i);
            (*c.borrow_mut()).context_map = state.with(|__s| __s.context_map.clone()).offset(
                ((m.with(|__s| __s.context_offset))
                    .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
                    as isize,
            );
            (*c.borrow_mut()).context_bits = m.with(|__s| __s.context_bits);
            let mut width: i32 = m.with(|__s| __s.width_in_blocks);
            let mut ac_stride: usize = (m.with(|__s| __s.ac_stride) as usize);
            let mut b_stride: usize = (m.with(|__s| __s.b_stride) as usize);
            let mut next_iy: i32 = ac_dc_state.with(|__s| __s.next_iy);
            (*c.borrow_mut()).y = ({ ({ mcu_y } * { m.with(|__s| __s.v_samp) }) } + { next_iy });
            (*c.borrow_mut()).prev_row_delta = {
                (((((1_u32)
                    .wrapping_sub((2_u32).wrapping_mul((({ (*c.borrow()).y } as u32) & 1_u32))))
                .wrapping_mul(((width + 3) as u32)))
                .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                    as i32)
            };
            let mut iy: i32 = next_iy;
            'loop_: while ({ iy } < { m.with(|__s| __s.v_samp) }) {
                let mut next_x: i32 = ac_dc_state.with(|__s| __s.next_x);
                let mut block_offset: usize =
                    ((next_x * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize);
                (*c.borrow_mut()).coeffs = {
                    m.with(|__s| __s.ac_coeffs.clone())
                        .offset((({ (*c.borrow()).y } as usize).wrapping_mul(ac_stride)) as isize)
                        .offset((block_offset) as isize)
                };
                (*c.borrow_mut()).prev_row_coeffs =
                    { { (*c.borrow()).coeffs.clone() }.offset(-((ac_stride) as isize)) };
                (*c.borrow_mut()).prev_col_coeffs = {
                    { (*c.borrow()).coeffs.clone() }
                        .offset(-((kDCTBlockSize_3.with(|rc| *rc.borrow())) as isize))
                };
                let mut block_state: Ptr<u8> = m
                    .with(|__s| __s.block_state.clone())
                    .offset((({ (*c.borrow()).y } as usize).wrapping_mul(b_stride)) as isize)
                    .offset((next_x) as isize);
                (*c.borrow_mut()).prev_sgn = ((cst.with(|__s| __s.prev_sign.as_pointer())
                    as Ptr<i32>)
                    .offset((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                .offset((block_offset) as isize);
                let __rhs = ((cst.with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>).offset(
                    (((((({ (*c.borrow()).y } as u32) & 1_u32).wrapping_mul(((width + 3) as u32)))
                        .wrapping_add(2_u32))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                        as usize),
                ))
                .offset((block_offset) as isize);
                (*c.borrow_mut()).prev_abs = __rhs;
                (*c.borrow_mut()).x = next_x;
                'loop_: while ({ (*c.borrow()).x } < width) {
                    let mut is_empty: bool = (((block_state.postfix_inc()).read()) != 0);
                    if !(is_empty) {
                        if ((!({ brunsli_WordSourceImpl::CanRead(&in_, 297_usize) }) as i64) != 0) {
                            field!(ac_dc_state, next_mcu_y).write(mcu_y);
                            field!(ac_dc_state, next_component).write(i);
                            field!(ac_dc_state, next_iy).write(iy);
                            field!(ac_dc_state, next_x).write({ (*c.borrow()).x });
                            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let mut num_nonzeros: usize = ({ DecodeAcBlock_159(c.as_pointer()) });
                        if !(num_nonzeros <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                    949,
                                    Ptr::<i8>::from_string_literal(b"DecodeAC"),
                                )
                            });
                            'loop_: while true {}
                        };
                        elem!({ (*c.borrow()).prev_num_nonzeros.clone() }, {
                            (*c.borrow()).x
                        })
                        .write({ (num_nonzeros as u8) });
                    } else {
                        ({
                            let _prev_sgn: Ptr<i32> = { (*c.borrow()).prev_sgn.clone() };
                            let _prev_abs: Ptr<i32> = { (*c.borrow()).prev_abs.clone() };
                            DecodeEmptyAcBlock_158(_prev_sgn, _prev_abs)
                        });
                        elem!({ (*c.borrow()).prev_num_nonzeros.clone() }, {
                            (*c.borrow()).x
                        })
                        .write(0_u8);
                    }
                    (*c.borrow_mut()).coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*c.borrow_mut()).prev_sgn += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*c.borrow_mut()).prev_abs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*c.borrow_mut()).prev_row_coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*c.borrow_mut()).prev_col_coeffs += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*c.borrow_mut()).x.prefix_inc();
                }
                (*c.borrow_mut()).prev_row_delta *= -1_i32;
                field!(ac_dc_state, next_x).write(0);
                {
                    iy.prefix_inc();
                    (*c.borrow_mut()).y.prefix_inc()
                };
            }
            field!(ac_dc_state, next_iy).write(0);
            i.prefix_inc();
        }
        field!(ac_dc_state, next_component).write(0_usize);
        mcu_y.prefix_inc();
    }
    field!(ac_dc_state, next_mcu_y).write(0);
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.clear());
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.shrink_to_fit());
    if !({ FinalizeSubdecoders_156((state).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn CheckCanRead_161(mut state: Ptr<brunsli_internal_dec_State>, mut required: usize) -> bool {
    let mut available: usize = (state.with(|__s| __s.len)).wrapping_sub(state.with(|__s| __s.pos));
    return (required <= available);
}
pub fn CheckCanReadByte_162(mut state: Ptr<brunsli_internal_dec_State>) -> bool {
    return ({ state.with(|__s| __s.pos) } != { state.with(|__s| __s.len) });
}
pub fn ReadByte_163(mut state: Ptr<brunsli_internal_dec_State>) -> u8 {
    return (elem!(
        state.with(|__s| __s.data.clone()),
        field!(state, pos).with_mut(|__v| __v.postfix_inc())
    )
    .read());
}
pub fn PeekByte_164(mut state: Ptr<brunsli_internal_dec_State>, mut offset: usize) -> u8 {
    return (elem!(
        state.with(|__s| __s.data.clone()),
        (state.with(|__s| __s.pos)).wrapping_add(offset)
    )
    .read());
}
pub fn SkipBytes_165(mut state: Ptr<brunsli_internal_dec_State>, mut len: usize) {
    field!(state, pos).write({ (state.with(|__s| __s.pos)).wrapping_add(len) });
}
pub fn GetBytesAvailable_166(mut state: Ptr<brunsli_internal_dec_State>) -> usize {
    return (state.with(|__s| __s.len)).wrapping_sub(state.with(|__s| __s.pos));
}
pub fn SkipAvailableBytes_167(mut state: Ptr<brunsli_internal_dec_State>, len: usize) -> usize {
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let available: Value<usize> =
        Rc::new(RefCell::new(({ GetBytesAvailable_166((state).clone()) })));
    let mut skip_bytes: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*len.borrow()) as u64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    field!(state, pos).write({ (state.with(|__s| __s.pos)).wrapping_add(skip_bytes) });
    return skip_bytes;
}
pub fn DecodeBase128_168(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut val: Ptr<usize>,
) -> brunsli_BrunsliStatus {
    val.write(0_usize);
    let mut b: u64 = 128_u64;
    let mut i: usize = 0_usize;
    'loop_: while (i < 9_usize) && ((b & 128_u64) != 0) {
        if !({ CheckCanRead_161((state).clone(), (i).wrapping_add(1_usize)) }) {
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
        b = (({ PeekByte_164((state).clone(), i) }) as u64);
        val.write({
            (((val.read()) as u64) | ((b & 127_u64) << ((i).wrapping_mul(7_usize)))) as usize
        });
        i.prefix_inc();
    }
    ({ SkipBytes_165((state).clone(), i) });
    return if ((b & 128_u64) == 0_u64) {
        brunsli_BrunsliStatus_BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
    };
}
pub fn Fail_169(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    field!(s, result).write(result);
    field!(s, last_stage).write(state.with(|__s| __s.stage));
    return brunsli_internal_dec_Stage_ERROR;
}
pub fn ReadTag_170(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    if !({ CheckCanReadByte_162((state).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    let mut marker: u8 = ({ ReadByte_163((state).clone()) });
    let mut tag: usize = (((marker as i32) >> 3_u32) as usize);
    if (tag == 0_usize) || (tag > 15_usize) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    field!(section, tag).write(tag);
    let mut wiring_type: usize = (((marker as u32) & 7_u32) as usize);
    if (wiring_type != (kBrunsliWiringTypeVarint_25.with(|rc| *rc.borrow()) as usize))
        && (wiring_type != (kBrunsliWiringTypeLengthDelimited_26.with(|rc| *rc.borrow()) as usize))
    {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    field!(section, is_section).write(
        (wiring_type == (kBrunsliWiringTypeLengthDelimited_26.with(|rc| *rc.borrow()) as usize)),
    );
    let mut tag_bit: u32 = (1_u32 << tag);
    if (({ section.with(|__s| __s.tags_met) } & { tag_bit }) != 0) {
        write!(libcc2rs::cerr(), "Duplicate marker {:x}\n", (marker as i32),);
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    {
        let __rhs = tag_bit;
        field!(section, tags_met).with_mut(|__v| *__v = *__v | __rhs)
    };
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn EnterSection_171(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    let section_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut status: brunsli_BrunsliStatus =
        ({ DecodeBase128_168((state).clone(), (section_size.as_pointer())) });
    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
        return status;
    }
    field!(section, is_active).write(true);
    field!(section, remaining).write((*section_size.borrow()));
    field!(section, milestone).write(state.with(|__s| __s.pos));
    field!(section, projected_end)
        .write({ (state.with(|__s| __s.pos)).wrapping_add(section.with(|__s| __s.remaining)) });
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn LeaveSection_172(mut section: Ptr<brunsli_internal_dec_SectionState>) {
    field!(section, is_active).write(false);
}
pub fn IsOutOfSectionBounds_173(mut state: Ptr<brunsli_internal_dec_State>) -> bool {
    return ({ state.with(|__s| __s.pos) } > {
        {
            (*state
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .projected_end
        }
    });
}
pub fn RemainingSectionLength_174(mut state: Ptr<brunsli_internal_dec_State>) -> usize {
    if ({ IsOutOfSectionBounds_173((state).clone()) }) {
        return 0_usize;
    }
    return ({
        (*state
            .with(|__s| __s.internal.clone())
            .as_ref()
            .unwrap()
            .borrow())
        .section
        .projected_end
    })
    .wrapping_sub(state.with(|__s| __s.pos));
}
pub fn IsAtSectionBoundary_175(mut state: Ptr<brunsli_internal_dec_State>) -> bool {
    return ({ state.with(|__s| __s.pos) } == {
        {
            (*state
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .projected_end
        }
    });
}
pub fn VerifySignature_176(
    mut state: Ptr<brunsli_internal_dec_State>,
) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    if !({
        CheckCanRead_161(
            (state).clone(),
            kBrunsliSignatureSize_43.with(|rc| *rc.borrow()),
        )
    }) {
        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA) });
    }
    let mut is_signature_ok: bool = ((state
        .with(|__s| __s.data.clone())
        .offset((state.with(|__s| __s.pos)) as isize)
        as Ptr<u8>)
        .to_any()
        .memcmp(
            &((kBrunsliSignature_44.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
            kBrunsliSignatureSize_43.with(|rc| *rc.borrow()),
        )
        != 0);
    field!(state, pos).write({
        (state.with(|__s| __s.pos)).wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
    });
    {
        field!(field!(s, section), tags_met).with_mut(|__v| {
            *__v = *__v | (1_u32 << (kBrunsliSignatureTag_30.with(|rc| *rc.borrow()) as i32))
        })
    };
    if is_signature_ok {
        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
    }
    return brunsli_internal_dec_Stage_HEADER;
}
pub fn DecodeHeader_177(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let hs: Ptr<brunsli_internal_dec_HeaderState> = field_ptr!(s, header);
    'loop_: while (hs.with(|__s| __s.stage)
        != (brunsli_internal_dec_HeaderState_Stage_DONE as usize))
    {
        'switch: {
            match { hs.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_READ_TAG as usize) => {
                    let mut status: brunsli_BrunsliStatus =
                        ({ ReadTag_170((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    if ({ s.with(|__s| __s.section.tag) } != {
                        (kBrunsliHeaderTag_31.with(|rc| *rc.borrow()) as usize)
                    }) || (!(s.with(|__s| __s.section.is_section)))
                    {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ENTER_SECTION as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ENTER_SECTION as usize) => {
                    let mut status: brunsli_BrunsliStatus =
                        ({ EnterSection_171((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize) => {
                    if ({ IsAtSectionBoundary_175((state).clone()) }) {
                        field!(hs, stage)
                            .write((brunsli_internal_dec_HeaderState_Stage_FINALE as usize));
                        break 'switch;
                    }
                    let mut status: brunsli_BrunsliStatus =
                        ({ ReadTag_170((state).clone(), (field_ptr!(hs, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    let mut tag_bit: u32 = (1_u32 << hs.with(|__s| __s.section.tag));
                    if hs.with(|__s| __s.section.is_section) {
                        if ((kKnownHeaderVarintTags_138.with(|rc| *rc.borrow()) & tag_bit) != 0) {
                            ({
                                Fail_169((state).clone(), brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                            });
                        }
                        field!(hs, stage).write(
                            (brunsli_internal_dec_HeaderState_Stage_ITEM_ENTER_SECTION as usize),
                        );
                        break 'switch;
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_VALUE as usize));
                    break 'switch;
                }
                __v if __v
                    == (brunsli_internal_dec_HeaderState_Stage_ITEM_ENTER_SECTION as usize) =>
                {
                    let mut status: brunsli_BrunsliStatus = ({
                        DecodeBase128_168((state).clone(), (field_ptr!(hs, remaining_skip_length)))
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    field!(hs, stage).write(
                        (brunsli_internal_dec_HeaderState_Stage_ITEM_SKIP_CONTENTS as usize),
                    );
                    break 'switch;
                }
                __v if __v
                    == (brunsli_internal_dec_HeaderState_Stage_ITEM_SKIP_CONTENTS as usize) =>
                {
                    let mut bytes_skipped: usize = ({
                        SkipAvailableBytes_167(
                            (state).clone(),
                            hs.with(|__s| __s.remaining_skip_length),
                        )
                    });
                    field!(hs, remaining_skip_length).write({
                        (hs.with(|__s| __s.remaining_skip_length)).wrapping_sub(bytes_skipped)
                    });
                    if (hs.with(|__s| __s.remaining_skip_length) > 0_usize) {
                        return ({
                            Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ITEM_READ_VALUE as usize) => {
                    let value: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let mut status: brunsli_BrunsliStatus =
                        ({ DecodeBase128_168((state).clone(), (value.as_pointer())) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    elem!(
                        (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                        hs.with(|__s| __s.section.tag)
                    )
                    .write(((*value.borrow()) as u64));
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_FINALE as usize) => {
                    let mut has_version: bool = (({ hs.with(|__s| __s.section.tags_met) } & {
                        (1_u32 << (kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow()) as i32))
                    }) != 0);
                    if !(has_version) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    let mut version_and_comp_count: usize = ((elem!(
                        (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                        (kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow()) as usize)
                    )
                    .read()) as usize);
                    let mut version: usize = (version_and_comp_count >> 2_u32);
                    field!(jpg, version).write((version as i32));
                    if (version == 1_usize) {
                        field!(jpg, width).write(0);
                        field!(jpg, height).write(0);
                        field!(hs, stage)
                            .write((brunsli_internal_dec_HeaderState_Stage_DONE as usize));
                        break 'switch;
                    }
                    if ((version & 1_usize) != 0_usize) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    if ((version & (!7_u32 as usize)) != 0_usize) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    field!(state, use_legacy_context_model).write(!((version & 2_usize) != 0));
                    {
                        field!(field!(s, section), tags_met).with_mut(|__v| {
                            *__v = *__v
                                | (1_u32
                                    << (kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow()) as i32))
                        })
                    };
                    let mut has_width: bool = (({ hs.with(|__s| __s.section.tags_met) } & {
                        (1_u32 << (kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow()) as i32))
                    }) != 0);
                    if !(has_width) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    let mut width: usize = ((elem!(
                        (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                        (kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow()) as usize)
                    )
                    .read()) as usize);
                    let mut has_height: bool = (({ hs.with(|__s| __s.section.tags_met) } & {
                        (1_u32 << (kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow()) as i32))
                    }) != 0);
                    if !(has_height) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    let mut height: usize = ((elem!(
                        (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                        (kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow()) as usize)
                    )
                    .read()) as usize);
                    if (width == 0_usize) || (height == 0_usize) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    if (width > (kMaxDimPixels_11.with(|rc| *rc.borrow()) as usize))
                        || (height > (kMaxDimPixels_11.with(|rc| *rc.borrow()) as usize))
                    {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    field!(jpg, width).write((width as i32));
                    field!(jpg, height).write((height as i32));
                    let mut num_components: usize =
                        (version_and_comp_count & 3_usize).wrapping_add(1_usize);
                    {
                        let __a0 = num_components as usize;
                        (*jpg.with(|__s| __s.components.clone()).borrow_mut())
                            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
                    };
                    let mut has_subsampling: bool = (({ hs.with(|__s| __s.section.tags_met) } & {
                        (1_u32 << (kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow()) as i32))
                    }) != 0);
                    if !(has_subsampling) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    let mut subsampling_code: usize = ((elem!(
                        (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                        (kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow()) as usize)
                    )
                    .read()) as usize);
                    let mut i: usize = 0_usize;
                    'loop_: while ({ i } < {
                        (*jpg.with(|__s| __s.components.clone()).borrow()).len()
                    }) {
                        let mut c: Ptr<brunsli_JPEGComponent> = ((jpg
                            .with(|__s| __s.components.as_pointer())
                            as Ptr<brunsli_JPEGComponent>)
                            .offset(i));
                        field!(c, v_samp_factor)
                            .write((((subsampling_code & 15_usize).wrapping_add(1_usize)) as i32));
                        subsampling_code >>= 4_u32;
                        field!(c, h_samp_factor)
                            .write((((subsampling_code & 15_usize).wrapping_add(1_usize)) as i32));
                        subsampling_code >>= 4_u32;
                        if ({ c.with(|__s| __s.v_samp_factor) } > {
                            kBrunsliMaxSampling_27.with(|rc| *rc.borrow())
                        }) {
                            return ({
                                Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                            });
                        }
                        if ({ c.with(|__s| __s.h_samp_factor) } > {
                            kBrunsliMaxSampling_27.with(|rc| *rc.borrow())
                        }) {
                            return ({
                                Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                            });
                        }
                        i.prefix_inc();
                    }
                    if !({ UpdateSubsamplingDerivatives_178((jpg).clone()) }) {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    ({ PrepareMeta_179((jpg).clone(), (state).clone()) });
                    field!(hs, stage).write((brunsli_internal_dec_HeaderState_Stage_DONE as usize));
                    break 'switch;
                }
                _ => {
                    return ({
                        Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR)
                    });
                }
            }
        };
    }
    ({ LeaveSection_172((field_ptr!(s, section))) });
    return if ({ jpg.with(|__s| __s.version) } == { kFallbackVersion_2.with(|rc| *rc.borrow()) }) {
        brunsli_internal_dec_Stage_FALLBACK
    } else {
        brunsli_internal_dec_Stage_SECTION
    };
}
pub fn DecodeMetaDataSection_180(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let ms: Ptr<brunsli_internal_dec_MetadataState> = field_ptr!(s, metadata);
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_DONE)
    {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_INITIAL)
    {
        if ({ IsAtSectionBoundary_175((state).clone()) }) {
            field!(ms, decompression_stage)
                .write(brunsli_internal_dec_MetadataDecompressionStage_DONE);
            return brunsli_BrunsliStatus_BRUNSLI_OK;
        }
        if (({ RemainingSectionLength_174((state).clone()) }) == 1_usize) {
            if !({ CheckCanReadByte_162((state).clone()) }) {
                return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
            }
            let data: Value<Box<[u8]>> =
                Rc::new(RefCell::new((0..1).map(|_| 0_u8).collect::<Box<[u8]>>()));
            (*data.borrow_mut())[(0) as usize] = ({ ReadByte_163((state).clone()) });
            let mut ok: bool = ({
                ProcessMetaData_149(
                    (data.as_pointer() as Ptr<u8>),
                    1_usize,
                    (ms).clone(),
                    (jpg).clone(),
                )
            }) && ({ brunsli_internal_dec_MetadataStateImpl::CanFinish(&ms) });
            field!(ms, decompression_stage)
                .write(brunsli_internal_dec_MetadataDecompressionStage_DONE);
            return if ok {
                brunsli_BrunsliStatus_BRUNSLI_OK
            } else {
                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
            };
        }
        field!(ms, decompression_stage)
            .write(brunsli_internal_dec_MetadataDecompressionStage_READ_LENGTH);
    }
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_READ_LENGTH)
    {
        let mut status: brunsli_BrunsliStatus =
            ({ DecodeBase128_168((state).clone(), (field_ptr!(ms, metadata_size))) });
        if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return status;
        }
        if ({ IsOutOfSectionBounds_173((state).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        if (({ RemainingSectionLength_174((state).clone()) }) == 0_usize) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        field!(ms, brotli).write(unsafe {
            ::brotli_sys::BrotliDecoderCreateInstance(None, None, std::ptr::null_mut())
        });
        if (ms.with(|__s| __s.brotli.clone())).is_null() {
            return brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR;
        }
        field!(ms, decompression_stage)
            .write(brunsli_internal_dec_MetadataDecompressionStage_DECOMPRESSING);
    }
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_DECOMPRESSING)
    {
        let finish_decompression: Value<FnPtr<fn(brunsli_BrunsliStatus) -> brunsli_BrunsliStatus>> =
            Rc::new(RefCell::new(lambda!(
                {
                    let ms: Ptr<brunsli_internal_dec_MetadataState> = (ms).clone();
                },
                |result: brunsli_BrunsliStatus| -> brunsli_BrunsliStatus {
                    if !(!((ms.with(|__s| __s.brotli.clone())).is_null())) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                1312,
                                Ptr::<i8>::from_string_literal(b"operator()"),
                            )
                        });
                        'loop_: while true {}
                    };
                    unsafe {
                        ::brotli_sys::BrotliDecoderDestroyInstance(
                            ms.with(|__s| __s.brotli.clone()),
                        )
                    };
                    field!(ms, brotli).write(std::ptr::null_mut());
                    field!(ms, decompression_stage)
                        .write(brunsli_internal_dec_MetadataDecompressionStage_DONE);
                    return result;
                }
            )));
        'loop_: while true {
            let mut available_bytes: usize = ({
                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                    (({ GetBytesAvailable_166((state).clone()) }) as u64),
                ));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                    (({ RemainingSectionLength_174((state).clone()) }) as u64),
                ));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            } as usize);
            let available_in: Value<usize> = Rc::new(RefCell::new(available_bytes));
            let next_in: Value<Ptr<u8>> = Rc::new(RefCell::new(
                state
                    .with(|__s| __s.data.clone())
                    .offset((state.with(|__s| __s.pos)) as isize),
            ));
            let available_out: Value<usize> = Rc::new(RefCell::new(0_usize));
            let mut result: ::brotli_sys::BrotliDecoderResult = {
                let __in = (next_in.as_pointer()).read();
                let __in_len = (available_in.as_pointer()).read();
                let __r = __in.with_slice(__in_len, |__s| {
                    (available_in.as_pointer()).with_mut(|_v1| {
                        (available_out.as_pointer()).with_mut(|_v3| unsafe {
                            ::brotli_sys::BrotliDecoderDecompressStream(
                                ms.with(|__s| __s.brotli.clone()),
                                _v1,
                                &mut __s.as_ptr(),
                                _v3,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                            )
                        })
                    })
                });
                (next_in.as_pointer())
                    .write(__in.offset(__in_len - (available_in.as_pointer()).read()));
                __r
            };
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_ERROR as i32)) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let chunk_size: Value<usize> = Rc::new(RefCell::new(0_usize));
            let mut chunk_data: Ptr<u8> = unsafe {
                (chunk_size.as_pointer()).with_mut(|_v1| {
                    let output: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(
                        ms.with(|__s| __s.brotli.clone()),
                        _v1 as *mut usize,
                    );
                    let slice = std::slice::from_raw_parts(output, *_v1);
                    let result: Ptr<Vec<u8>> = Ptr::alloc(slice.to_vec());
                    result.decay()
                })
            };
            field!(ms, decompressed_size).write({
                (ms.with(|__s| __s.decompressed_size)).wrapping_add((*chunk_size.borrow()))
            });
            if ({ ms.with(|__s| __s.decompressed_size) } > { ms.with(|__s| __s.metadata_size) }) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let mut consumed_bytes: usize =
                (available_bytes).wrapping_sub((*available_in.borrow()));
            ({ SkipBytes_165((state).clone(), consumed_bytes) });
            let mut chunk_ok: bool = ({
                ProcessMetaData_149(
                    (chunk_data).clone(),
                    (*chunk_size.borrow()),
                    (ms).clone(),
                    (jpg).clone(),
                )
            });
            if !(chunk_ok) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32)) {
                if (({ RemainingSectionLength_174((state).clone()) }) != 0_usize) {
                    return ({
                        (*finish_decompression.borrow())
                            .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                    });
                }
                if ({ ms.with(|__s| __s.decompressed_size) } != {
                    ms.with(|__s| __s.metadata_size)
                }) {
                    return ({
                        (*finish_decompression.borrow())
                            .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                    });
                }
                if !({ brunsli_internal_dec_MetadataStateImpl::CanFinish(&ms) }) {
                    return ({
                        (*finish_decompression.borrow())
                            .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                    });
                }
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK)
                });
            }
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as i32)) {
                continue 'loop_;
            }
            if !((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as i32)) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                        1352,
                        Ptr::<i8>::from_string_literal(b"DecodeMetaDataSection"),
                    )
                });
                'loop_: while true {}
            };
            if (({ RemainingSectionLength_174((state).clone()) }) == 0_usize) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
    }
    if !(false) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1361,
                Ptr::<i8>::from_string_literal(b"DecodeMetaDataSection"),
            )
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR;
}
pub fn CheckBoundary_181(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    if ((result as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        let mut last: bool = ({ ({ RemainingSectionLength_174((state).clone()) }) } <= {
            ({ GetBytesAvailable_166((state).clone()) })
        });
        return if last {
            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
        } else {
            brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA
        };
    } else {
        return result;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn PrepareBitReader_182(
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut state: Ptr<brunsli_internal_dec_State>,
) {
    let mut chunk_len: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
            (({ GetBytesAvailable_166((state).clone()) }) as u64),
        ));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
            (({ RemainingSectionLength_174((state).clone()) }) as u64),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    ({
        BrunsliBitReaderResume_128(
            (br).clone(),
            state
                .with(|__s| __s.data.clone())
                .offset((state.with(|__s| __s.pos)) as isize),
            chunk_len,
        )
    });
    if !({ BrunsliBitReaderIsHealthy_132((br).clone()) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1384,
                Ptr::<i8>::from_string_literal(b"PrepareBitReader"),
            )
        });
        'loop_: while true {}
    };
}
pub fn SuspendBitReader_183(
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut state: Ptr<brunsli_internal_dec_State>,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    let mut chunk_len: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
            (({ GetBytesAvailable_166((state).clone()) }) as u64),
        ));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
            (({ RemainingSectionLength_174((state).clone()) }) as u64),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    let mut unused_bytes: usize = ({ BrunsliBitReaderSuspend_130((br).clone()) });
    let mut consumed_bytes: usize = (chunk_len).wrapping_sub(unused_bytes);
    ({ SkipBytes_165((state).clone(), consumed_bytes) });
    let __rhs = ({ CheckBoundary_181((state).clone(), result) });
    result = __rhs;
    if !(({ BrunsliBitReaderIsHealthy_132((br).clone()) })
        || (((result as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32))
            && ((result as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1401,
                Ptr::<i8>::from_string_literal(b"SuspendBitReader"),
            )
        });
        'loop_: while true {}
    };
    return result;
}
pub fn DecodeJPEGInternalsSection_184(
    state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> = field_ptr!(s, internals);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(js, br))));
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_INIT as i32))
    {
        ({ BrunsliBitReaderInit_127((*br.borrow()).clone()) });
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_MARKERS);
    }
    ({ PrepareBitReader_182((*br.borrow()).clone(), (*state.borrow()).clone()) });
    let suspend_bit_reader: Value<FnPtr<fn(brunsli_BrunsliStatus) -> brunsli_BrunsliStatus>> =
        Rc::new(RefCell::new(lambda!(
            {
                let br: Ptr<Ptr<brunsli_BrunsliBitReader>> = br.as_pointer();
                let state: Ptr<Ptr<brunsli_internal_dec_State>> = state.as_pointer();
            },
            |result: brunsli_BrunsliStatus| -> brunsli_BrunsliStatus {
                return ({ SuspendBitReader_183((br.read()), (state.read()), result) });
            }
        )));
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_MARKERS as i32))
    {
        'loop_: while true {
            if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 6_usize) }) {
                return ({
                    (*suspend_bit_reader.borrow())
                        .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                });
            }
            let marker: Value<u8> = Rc::new(RefCell::new(
                (((192_u32)
                    .wrapping_add(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 6_u32) })))
                    as u8),
            ));
            {
                let a0_clone = (*marker.borrow()).clone();
                (*jpg.with(|__s| __s.marker_order.clone()).borrow_mut()).push(a0_clone)
            };
            if (((*marker.borrow()) as i32) == 196) {
                field!(js, dht_count).with_mut(|__v| __v.prefix_inc());
            }
            if (((*marker.borrow()) as i32) == 221) {
                field!(js, have_dri).write(true);
            }
            if (((*marker.borrow()) as i32) == 218) {
                field!(js, num_scans).with_mut(|__v| __v.prefix_inc());
            }
            if (((*marker.borrow()) as i32) == 217) {
                break;
            }
        }
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_DRI);
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_DRI as i32))
    {
        if js.with(|__s| __s.have_dri) {
            if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 16_usize) }) {
                return ({
                    (*suspend_bit_reader.borrow())
                        .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                });
            }
            let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 16_u32) }) as i32);
            field!(jpg, restart_interval).write(__rhs);
        }
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST);
    }
    if (((js.with(|__s| __s.stage) as i32)
        & (brunsli_internal_dec_JpegInternalsState_Stage_DECODE_HUFFMAN_MASK as i32))
        != 0)
    {
        let mut status: brunsli_BrunsliStatus =
            ({ DecodeHuffmanCode_150((*state.borrow()).clone(), (jpg).clone()) });
        if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call(status) });
        }
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_PREPARE_READ_SCANS);
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_PREPARE_READ_SCANS as i32))
    {
        if ({ js.with(|__s| __s.dht_count) } != { js.with(|__s| __s.terminal_huffman_code_count) })
        {
            write!(libcc2rs::cerr(), "Invalid number of DHT markers\n",);
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        if (js.with(|__s| __s.num_scans) > 0_usize) {
            {
                let __a0 = js.with(|__s| __s.num_scans) as usize;
                (*jpg.with(|__s| __s.scan_info.clone()).borrow_mut())
                    .resize_with(__a0, || <brunsli_JPEGScanInfo>::default())
            };
            field!(js, i).write(0_usize);
            field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON);
        } else {
            field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_QUANT);
        }
    }
    if (((js.with(|__s| __s.stage) as i32)
        & (brunsli_internal_dec_JpegInternalsState_Stage_DECODE_SCAN_MASK as i32))
        != 0)
    {
        let mut status: brunsli_BrunsliStatus =
            ({ DecodeScanInfo_151((*state.borrow()).clone(), (jpg).clone()) });
        if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call(status) });
        }
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_QUANT);
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_QUANT as i32))
    {
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 2_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let mut num_quant_tables: i32 =
            ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }).wrapping_add(1_u32))
                as i32);
        {
            let __a0 = (num_quant_tables as usize) as usize;
            (*jpg.with(|__s| __s.quant.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_JPEGQuantTable>::default())
        };
        field!(js, i).write(0_usize);
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_QUANT);
    }
    'loop_: while ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_QUANT as i32))
    {
        if ({ js.with(|__s| __s.i) } >= { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            field!(js, stage)
                .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID_SCHEME);
            break;
        }
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 7_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let mut q: Ptr<brunsli_JPEGQuantTable> = ((jpg.with(|__s| __s.quant.as_pointer())
            as Ptr<brunsli_JPEGQuantTable>)
            .offset(js.with(|__s| __s.i)));
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as i32);
        field!(q, index).write(__rhs);
        let __rhs = ({ js.with(|__s| __s.i) } == {
            ((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize)
        }) || (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0);
        field!(q, is_last).write(__rhs);
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 4_u32) }) as i32);
        field!(q, precision).write(__rhs);
        if (q.with(|__s| __s.precision) > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quantization table precision: {:}\n",
                q.with(|__s| __s.precision),
            );
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(js, i).with_mut(|__v| __v.prefix_inc());
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID_SCHEME as i32))
    {
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 2_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let mut comp_ids: i32 =
            (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as i32);
        thread_local!(
            static kMinRequiredComponents_185: Value<Box<[usize]>> =
                Rc::new(RefCell::new(Box::new([3_usize, 1_usize, 3_usize, 0_usize])));
        );
        if ({ (*jpg.with(|__s| __s.components.clone()).borrow()).len() } < {
            ({
                let __idx = (comp_ids) as usize;
                kMinRequiredComponents_185.with(|rc| rc.borrow()[__idx])
            })
        }) {
            write!(
                libcc2rs::cerr(),
                "Insufficient number of components for ColorId #{:}\n",
                comp_ids,
            );
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(js, stage)
            .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_PADDING_BITS);
        if (comp_ids == kComponentIds123_49.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write(1);
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    1_usize
                ),
                id
            )
            .write(2);
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    2_usize
                ),
                id
            )
            .write(3);
        } else if (comp_ids == kComponentIdsGray_50.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write(1);
        } else if (comp_ids == kComponentIdsRGB_51.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write((('R' as i8) as i32));
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    1_usize
                ),
                id
            )
            .write((('G' as i8) as i32));
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    2_usize
                ),
                id
            )
            .write((('B' as i8) as i32));
        } else {
            if !(comp_ids == kComponentIdsCustom_52.with(|rc| *rc.borrow())) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                        1529,
                        Ptr::<i8>::from_string_literal(b"DecodeJPEGInternalsSection"),
                    )
                });
                'loop_: while true {}
            };
            field!(js, i).write(0_usize);
            field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID);
        }
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID as i32))
    {
        'loop_: while ({ js.with(|__s| __s.i) } < {
            (*jpg.with(|__s| __s.components.clone()).borrow()).len()
        }) {
            if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 8_usize) }) {
                return ({
                    (*suspend_bit_reader.borrow())
                        .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                });
            }
            let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 8_u32) }) as i32);
            field!(
                elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    js.with(|__s| __s.i)
                ),
                id
            )
            .write(__rhs);
            field!(js, i).with_mut(|__v| __v.prefix_inc());
        }
        field!(js, stage)
            .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_PADDING_BITS);
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_PADDING_BITS as i32))
    {
        if !({ DecodeLimitedVarint_146((field_ptr!(js, varint)), (*br.borrow()).clone(), 4_usize) })
        {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        field!(js, num_padding_bits).write({ js.with(|__s| __s.varint.value) });
        field!(jpg, has_zero_padding_bit).write((js.with(|__s| __s.num_padding_bits) > 0_usize));
        if ({ js.with(|__s| __s.num_padding_bits) } > {
            (({ PaddingBitsLimit_17((jpg).clone()) }) as usize)
        }) {
            write!(
                libcc2rs::cerr(),
                "Suspicious number of padding bits {:}\n",
                js.with(|__s| __s.num_padding_bits),
            );
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(js, i).write(0_usize);
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_PADDING_BITS);
    }
    if ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_PADDING_BITS as i32))
    {
        'loop_: while ({ js.with(|__s| __s.i) } < { js.with(|__s| __s.num_padding_bits) }) {
            if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                return ({
                    (*suspend_bit_reader.borrow())
                        .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                });
            }
            {
                let __init = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as i32);
                (*jpg.with(|__s| __s.padding_bits.clone()).borrow_mut()).push(__init)
            };
            field!(js, i).with_mut(|__v| __v.prefix_inc());
        }
        ({ (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK) });
        ({ BrunsliBitReaderFinish_131((*br.borrow()).clone()) });
        if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        field!(js, i).write(0_usize);
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_ITERATE_MARKERS);
    } else {
        ({ (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK) });
    }
    'loop_: while true {
        switch!(match (js.with(|__s| __s.stage) as i32) {
            __v if __v
                == (brunsli_internal_dec_JpegInternalsState_Stage_ITERATE_MARKERS as i32) =>
            {
                {
                    if ({ js.with(|__s| __s.i) } >= {
                        (*jpg.with(|__s| __s.marker_order.clone()).borrow()).len()
                    }) {
                        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_DONE);
                    } else if (((elem!(
                        (jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
                        js.with(|__s| __s.i)
                    )
                    .read()) as i32)
                        == 255)
                    {
                        field!(js, stage).write(
                            brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_LENGTH,
                        );
                    } else {
                        field!(js, i).with_mut(|__v| __v.prefix_inc());
                    };
                    continue 'loop_;
                }
            }
            __v if __v
                == (brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_LENGTH
                    as i32) =>
            {
                {
                    let mut status: brunsli_BrunsliStatus = ({
                        DecodeBase128_168(
                            (*state.borrow()).clone(),
                            (field_ptr!(js, intermarker_length)),
                        )
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ CheckBoundary_181((*state.borrow()).clone(), status) });
                    }
                    if ({ js.with(|__s| __s.intermarker_length) } > {
                        ({ RemainingSectionLength_174((*state.borrow()).clone()) })
                    }) {
                        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                    }
                    {
                        let __init = Vec::new();
                        (jpg.with(|__s| __s.inter_marker_data.as_pointer())
                            as Ptr<Vec<Value<Vec<u8>>>>)
                            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                                __v.push(Rc::new(RefCell::new(__init)))
                            })
                    };
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_DATA);
                    continue 'loop_;
                }
            }
            __v if __v
                == (brunsli_internal_dec_JpegInternalsState_Stage_READ_INTERMARKER_DATA as i32) =>
            {
                {
                    let dest: Ptr<Vec<u8>> =
                        (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow())
                            [(*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len() - 1]
                            .as_pointer();
                    let piece_limit: Value<usize> = Rc::new(RefCell::new(
                        ((js.with(|__s| __s.intermarker_length) as u64)
                            .wrapping_sub(((*dest.upgrade().deref()).len() as u64))
                            as usize),
                    ));
                    let mut piece_size: usize = ({
                        let __tmp_0: Value<u64> =
                            Rc::new(RefCell::new(((*piece_limit.borrow()) as u64)));
                        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                            (({ GetBytesAvailable_166((*state.borrow()).clone()) }) as u64),
                        ));
                        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                            __tmp_0.as_pointer()
                        } else {
                            __tmp_1.as_pointer()
                        }
                        .read())
                    } as usize);
                    ({
                        Append_72(
                            (dest).clone(),
                            (*state.borrow())
                                .with(|__s| __s.data.clone())
                                .offset(((*state.borrow()).with(|__s| __s.pos)) as isize),
                            piece_size,
                        )
                    });
                    ({ SkipBytes_165((*state.borrow()).clone(), piece_size) });
                    if ({ (*dest.upgrade().deref()).len() } < {
                        js.with(|__s| __s.intermarker_length)
                    }) {
                        if !(({ GetBytesAvailable_166((*state.borrow()).clone()) }) == 0_usize) {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                    1613,
                                    Ptr::<i8>::from_string_literal(b"DecodeJPEGInternalsSection"),
                                )
                            });
                            'loop_: while true {}
                        };
                        if !(({ RemainingSectionLength_174((*state.borrow()).clone()) }) > 0_usize)
                        {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                    1614,
                                    Ptr::<i8>::from_string_literal(b"DecodeJPEGInternalsSection"),
                                )
                            });
                            'loop_: while true {}
                        };
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    field!(js, i).with_mut(|__v| __v.prefix_inc());
                    field!(js, stage)
                        .write(brunsli_internal_dec_JpegInternalsState_Stage_ITERATE_MARKERS);
                    continue 'loop_;
                }
            }
            _ => {
                {}
            }
        });
        break;
    }
    if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeQuantDataSection_186(
    state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let qs: Ptr<brunsli_internal_dec_QuantDataState> = field_ptr!(s, quant);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(qs, br))));
    if ((qs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_QuantDataState_Stage_INIT as i32))
    {
        ({ BrunsliBitReaderInit_127((*br.borrow()).clone()) });
        field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_READ_NUM_QUANT);
    }
    ({ PrepareBitReader_182((*br.borrow()).clone(), (*state.borrow()).clone()) });
    let suspend_bit_reader: Value<FnPtr<fn(brunsli_BrunsliStatus) -> brunsli_BrunsliStatus>> =
        Rc::new(RefCell::new(lambda!(
            {
                let br: Ptr<Ptr<brunsli_BrunsliBitReader>> = br.as_pointer();
                let state: Ptr<Ptr<brunsli_internal_dec_State>> = state.as_pointer();
            },
            |result: brunsli_BrunsliStatus| -> brunsli_BrunsliStatus {
                return ({ SuspendBitReader_183((br.read()), (state.read()), result) });
            }
        )));
    if ((qs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_QuantDataState_Stage_READ_NUM_QUANT as i32))
    {
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 2_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let mut num_quant_tables: usize =
            ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }).wrapping_add(1_u32))
                as usize);
        if ({ (*jpg.with(|__s| __s.quant.clone()).borrow()).len() } != { num_quant_tables }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        {
            let __a0 = (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize) as usize;
            (*qs.with(|__s| __s.predictor.clone()).borrow_mut())
                .resize_with(__a0, || <u8>::default())
        };
        field!(qs, i).write(0_usize);
        field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_READ_STOCK);
    }
    'loop_: while true {
        switch!(match (qs.with(|__s| __s.stage) as i32) {
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_READ_STOCK as i32) => {
                {
                    if ({ qs.with(|__s| __s.i) } >= {
                        (*jpg.with(|__s| __s.quant.clone()).borrow()).len()
                    }) {
                        std::mem::swap(
                            &mut Vec::new(),
                            &mut (*qs.with(|__s| __s.predictor.clone()).borrow_mut()),
                        );
                        field!(qs, i).write(0_usize);
                        field!(qs, stage)
                            .write(brunsli_internal_dec_QuantDataState_Stage_READ_QUANT_IDX);
                        continue 'loop_;
                    }
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 4_usize) }) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    field!(qs, data_precision).write(0_u8);
                    let mut is_short: bool =
                        !(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0);
                    if is_short {
                        let mut short_code: usize =
                            (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 3_u32) })
                                as usize);
                        let mut table: Ptr<i32> = ({
                            (*elem!(
                                (jpg.with(|__s| __s.quant.as_pointer())
                                    as Ptr<brunsli_JPEGQuantTable>),
                                qs.with(|__s| __s.i)
                            )
                            .upgrade()
                            .deref())
                            .values
                            .as_pointer()
                        } as Ptr<i32>);
                        let mut selector: usize = (if (qs.with(|__s| __s.i) > 0_usize) {
                            1
                        } else {
                            0
                        } as usize);
                        let mut k: usize = 0_usize;
                        'loop_: while (k < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
                            elem!(table, k).write({
                                (({
                                    let __idx = (selector) as usize;
                                    kStockQuantizationTables_48
                                        .with(|rc| rc.borrow()[__idx].clone())
                                })
                                .borrow()[(short_code) as usize]
                                    .borrow()[(k) as usize] as i32)
                            });
                            k.prefix_inc();
                        }
                        field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_UPDATE);
                    } else {
                        field!(qs, stage)
                            .write(brunsli_internal_dec_QuantDataState_Stage_READ_Q_FACTOR);
                    };
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_READ_Q_FACTOR as i32) => {
                {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 6_usize) }) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    let mut q_factor: u32 =
                        ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 6_u32) });
                    ({
                        let _is_chroma: bool = (qs.with(|__s| __s.i) > 0_usize);
                        let _dst: Ptr<u8> = (qs.with(|__s| __s.predictor.as_pointer()) as Ptr<u8>);
                        FillQuantMatrix_118(_is_chroma, q_factor, _dst)
                    });
                    field!(qs, j).write(0_usize);
                    field!(qs, delta).write(0);
                    field!(qs, stage)
                        .write(brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_IS_ZERO);
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_IS_ZERO as i32) => {
                {
                    if ({ qs.with(|__s| __s.j) } >= {
                        (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)
                    }) {
                        field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_UPDATE);
                        continue 'loop_;
                    }
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
                        field!(qs, stage)
                            .write(brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_SIGN);
                    } else {
                        field!(qs, stage)
                            .write(brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF);
                    };
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_SIGN as i32) => {
                {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    field!(qs, sign).write(
                        if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
                            -1_i32
                        } else {
                            1
                        },
                    );
                    field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_READ_DIFF);
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_READ_DIFF as i32) => {
                {
                    if !({
                        DecodeVarint_144((field_ptr!(qs, vs)), (*br.borrow()).clone(), 16_usize)
                    }) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    let mut diff: i32 = ((qs.with(|__s| __s.vs.value) as i32) + 1);
                    {
                        let __rhs = { ({ qs.with(|__s| __s.sign) } * { diff }) };
                        field!(qs, delta).with_mut(|__v| *__v = *__v + __rhs)
                    };
                    field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF);
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF as i32) => {
                {
                    let mut k: i32 = (({
                        let __idx = (qs.with(|__s| __s.j)) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    }) as i32);
                    let mut quant_value: i32 = ({
                        ((elem!(
                            (qs.with(|__s| __s.predictor.as_pointer()) as Ptr<u8>),
                            (k as usize)
                        )
                        .read()) as i32)
                    } + { qs.with(|__s| __s.delta) });
                    elem!(
                        ({
                            (*elem!(
                                (jpg.with(|__s| __s.quant.as_pointer())
                                    as Ptr<brunsli_JPEGQuantTable>),
                                qs.with(|__s| __s.i)
                            )
                            .upgrade()
                            .deref())
                            .values
                            .as_pointer()
                        } as Ptr<i32>),
                        (k as usize)
                    )
                    .write(quant_value);
                    if (quant_value <= 0) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                        });
                    }
                    if (quant_value >= 256) {
                        field!(qs, data_precision).write(1_u8);
                    }
                    if (quant_value >= 65536) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                        });
                    }
                    field!(qs, j).with_mut(|__v| __v.prefix_inc());
                    field!(qs, stage)
                        .write(brunsli_internal_dec_QuantDataState_Stage_READ_DIFF_IS_ZERO);
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_UPDATE as i32) => {
                {
                    if ({
                        {
                            (*elem!(
                                (jpg.with(|__s| __s.quant.as_pointer())
                                    as Ptr<brunsli_JPEGQuantTable>),
                                qs.with(|__s| __s.i)
                            )
                            .upgrade()
                            .deref())
                            .precision
                        }
                    } < { (qs.with(|__s| __s.data_precision) as i32) })
                    {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                        });
                    }
                    field!(qs, i).with_mut(|__v| __v.prefix_inc());
                    field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_READ_STOCK);
                    continue 'loop_;
                }
            }
            _ => {
                {}
            }
        });
        break;
    }
    'loop_: while ((qs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_QuantDataState_Stage_READ_QUANT_IDX as i32))
    {
        if ({ qs.with(|__s| __s.i) } >= {
            (*jpg.with(|__s| __s.components.clone()).borrow()).len()
        }) {
            field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_FINISH);
            continue 'loop_;
        }
        let mut c: Ptr<brunsli_JPEGComponent> = ((jpg.with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponent>)
            .offset(qs.with(|__s| __s.i)));
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 2_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as u8);
        field!(c, quant_idx).write(__rhs);
        if ({ (c.with(|__s| __s.quant_idx) as usize) } >= {
            (*jpg.with(|__s| __s.quant.clone()).borrow()).len()
        }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(qs, i).with_mut(|__v| __v.prefix_inc());
    }
    if !((qs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_QuantDataState_Stage_FINISH as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1787,
                Ptr::<i8>::from_string_literal(b"DecodeQuantDataSection"),
            )
        });
        'loop_: while true {}
    };
    ({ (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK) });
    ({ BrunsliBitReaderFinish_131((*br.borrow()).clone()) });
    if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeHistogramDataSection_187(
    state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let hs: Ptr<brunsli_internal_dec_HistogramDataState> = field_ptr!(s, histogram);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(hs, br))));
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_INIT as i32))
    {
        ({ BrunsliBitReaderInit_127((*br.borrow()).clone()) });
        if !(!((*jpg.with(|__s| __s.components.clone()).borrow()).is_empty())) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    1802,
                    Ptr::<i8>::from_string_literal(b"DecodeHistogramDataSection"),
                )
            });
            'loop_: while true {}
        };
        field!(s, num_contexts).write((*jpg.with(|__s| __s.components.clone()).borrow()).len());
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_READ_SCHEME);
        ({ brunsli_Arena_brunsli_HuffmanCode_Impl::reserve(&field_ptr!(hs, arena), 648_usize) });
    }
    ({ PrepareBitReader_182((*br.borrow()).clone(), (*state.borrow()).clone()) });
    if ({ ({ RemainingSectionLength_174((*state.borrow()).clone()) }) } <= {
        ({ GetBytesAvailable_166((*state.borrow()).clone()) })
    }) {
        ({ BrunsliBitReaderSetOptimistic_133((*br.borrow()).clone()) });
    }
    let suspend_bit_reader: Value<FnPtr<fn(brunsli_BrunsliStatus) -> brunsli_BrunsliStatus>> =
        Rc::new(RefCell::new(lambda!(
            {
                let br: Ptr<Ptr<brunsli_BrunsliBitReader>> = br.as_pointer();
                let state: Ptr<Ptr<brunsli_internal_dec_State>> = state.as_pointer();
            },
            |result: brunsli_BrunsliStatus| -> brunsli_BrunsliStatus {
                return ({ SuspendBitReader_183((br.read()), (state.read()), result) });
            }
        )));
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_SCHEME as i32))
    {
        let mut num_components: usize = (*jpg.with(|__s| __s.components.clone()).borrow()).len();
        if !(num_components <= 4_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    1822,
                    Ptr::<i8>::from_string_literal(b"DecodeHistogramDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({
            BrunsliBitReaderCanRead_134(
                (*br.borrow()).clone(),
                (3_usize).wrapping_mul(num_components),
            )
        }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < num_components) {
            let mut scheme: usize =
                (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 3_u32) }) as usize);
            if (scheme >= (kNumSchemes_91.with(|rc| *rc.borrow()) as usize)) {
                return ({
                    (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let m: Ptr<brunsli_internal_dec_ComponentMeta> = ((*state.borrow())
                .with(|__s| __s.meta.as_pointer())
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset(i);
            field!(m, context_bits).write(scheme);
            field!(m, context_offset).write(s.with(|__s| __s.num_contexts));
            field!(s, num_contexts).write({
                (s.with(|__s| __s.num_contexts)).wrapping_add(
                    (({
                        let __idx = (scheme) as usize;
                        kNumNonzeroContextSkip_94.with(|rc| rc.borrow()[__idx])
                    }) as usize),
                )
            });
            i.prefix_inc();
        }
        if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_READ_NUM_HISTOGRAMS);
    }
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_NUM_HISTOGRAMS as i32))
    {
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 11_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        field!(s, num_histograms).write(
            ((({ DecodeVarLenUint8_143((*br.borrow()).clone()) }).wrapping_add(1_u32)) as usize),
        );
        if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        if s.with(|__s| __s.shallow_histograms) {
            field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_SKIP_CONTENT);
        } else {
            {
                let __a0 = (s.with(|__s| __s.num_contexts))
                    .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow()))
                    as usize;
                (*s.with(|__s| __s.context_map_.clone()).borrow_mut())
                    .resize_with(__a0, || <u8>::default())
            };
            let __rhs = (s.with(|__s| __s.context_map_.as_pointer()) as Ptr<u8>);
            field!((*state.borrow()), context_map).write(__rhs);
            {
                let __a0 = s.with(|__s| __s.num_histograms) as usize;
                (*s.with(|__s| __s.entropy_codes_.clone()).borrow_mut())
                    .resize_with(__a0, || <brunsli_ANSDecodingData>::default())
            };
            let __rhs =
                (s.with(|__s| __s.entropy_codes_.as_pointer()) as Ptr<brunsli_ANSDecodingData>);
            field!((*state.borrow()), entropy_codes).write(__rhs);
            if (s.with(|__s| __s.num_histograms) > 1_usize) {
                field!(hs, stage)
                    .write(brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP_CODE);
            } else {
                field!(hs, i).write(0_usize);
                {
                    let __a0 = (kCoeffAlphabetSize_136.with(|rc| *rc.borrow()) as usize) as usize;
                    (*hs.with(|__s| __s.counts.clone()).borrow_mut())
                        .resize_with(__a0, || <u32>::default())
                };
                field!(hs, stage)
                    .write(brunsli_internal_dec_HistogramDataState_Stage_READ_HISTOGRAMS);
            }
        }
    }
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_SKIP_CONTENT as i32))
    {
        ({ (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK) });
        if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _len: usize = ({ RemainingSectionLength_174((*state.borrow()).clone()) });
            SkipAvailableBytes_167(_state, _len)
        });
        if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_DONE);
    }
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP_CODE as i32))
    {
        if !({
            BrunsliBitReaderCanRead_134(
                (*br.borrow()).clone(),
                (207_usize).wrapping_add((s.with(|__s| __s.num_histograms)).wrapping_mul(8_usize)),
            )
        }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        field!(hs, max_run_length_prefix).write(0_usize);
        let mut use_rle_for_zeros: bool =
            !(!(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0));
        if use_rle_for_zeros {
            field!(hs, max_run_length_prefix).write(
                ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 4_u32) }).wrapping_add(1_u32))
                    as usize),
            );
        }
        let mut alphabet_size: usize = (s.with(|__s| __s.num_histograms))
            .wrapping_add(hs.with(|__s| __s.max_run_length_prefix));
        {
            let _p: Ptr<_> = Ptr::alloc(<brunsli_HuffmanDecodingData>::default());
            (field_ptr!(hs, entropy) as Ptr<Option<Value<brunsli_HuffmanDecodingData>>>)
                .write(_p.to_owned_opt())
        };
        if !({
            let _arena: Ptr<brunsli_Arena_brunsli_HuffmanCode_> = (field_ptr!(hs, arena));
            brunsli_HuffmanDecodingDataImpl::ReadFromBitStream(
                &hs.with(|__s| __s.entropy.clone()).as_pointer(),
                alphabet_size,
                (*br.borrow()).clone(),
                Some(_arena),
            )
        }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(hs, i).write(0_usize);
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP);
    }
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_CONTEXT_MAP as i32))
    {
        let mut status: brunsli_BrunsliStatus = ({
            let _entropy: Ptr<brunsli_HuffmanDecodingData> =
                hs.with(|__s| __s.entropy.clone()).as_pointer();
            let _max_run_length_prefix: usize = hs.with(|__s| __s.max_run_length_prefix);
            let _index: Ptr<usize> = (field_ptr!(hs, i));
            DecodeContextMap_188(
                _entropy,
                _max_run_length_prefix,
                _index,
                (s.with(|__s| __s.context_map_.as_pointer())),
                (*br.borrow()).clone(),
            )
        });
        if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call(status) });
        }
        field!(hs, i).write(0_usize);
        {
            let __a0 = (kCoeffAlphabetSize_136.with(|rc| *rc.borrow()) as usize) as usize;
            (*hs.with(|__s| __s.counts.clone()).borrow_mut()).resize_with(__a0, || <u32>::default())
        };
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_READ_HISTOGRAMS);
    }
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_HISTOGRAMS as i32))
    {
        'loop_: while ({ hs.with(|__s| __s.i) } < { s.with(|__s| __s.num_histograms) }) {
            if !({
                BrunsliBitReaderCanRead_134(
                    (*br.borrow()).clone(),
                    ((9 + (kCoeffAlphabetSize_136.with(|rc| *rc.borrow()) * 11)) as usize),
                )
            }) {
                return ({
                    (*suspend_bit_reader.borrow())
                        .call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                });
            }
            if !({
                ReadHistogram_189(
                    (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u32),
                    (hs.with(|__s| __s.counts.as_pointer())),
                    (*br.borrow()).clone(),
                )
            }) {
                return ({
                    (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            if !({
                let _counts: Ptr<Vec<u32>> = hs.with(|__s| __s.counts.as_pointer());
                brunsli_ANSDecodingDataImpl::Init(
                    &(s.with(|__s| __s.entropy_codes_.as_pointer())
                        as Ptr<brunsli_ANSDecodingData>)
                        .offset(hs.with(|__s| __s.i)),
                    _counts,
                )
            }) {
                return ({
                    (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            field!(hs, i).with_mut(|__v| __v.prefix_inc());
        }
        {
            let _p: Ptr<_> = Default::default();
            (field_ptr!(hs, entropy) as Ptr<Option<Value<brunsli_HuffmanDecodingData>>>)
                .write(_p.to_owned_opt())
        };
        std::mem::swap(
            &mut Vec::new(),
            &mut (*hs.with(|__s| __s.counts.clone()).borrow_mut()),
        );
        ({ (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_OK) });
        ({ BrunsliBitReaderFinish_131((*br.borrow()).clone()) });
        if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        field!(hs, stage).write(brunsli_internal_dec_HistogramDataState_Stage_DONE);
    }
    ({ brunsli_Arena_brunsli_HuffmanCode_Impl::reset(&field_ptr!(hs, arena)) });
    if !((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_DONE as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1925,
                Ptr::<i8>::from_string_literal(b"DecodeHistogramDataSection"),
            )
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeDCDataSection_190(
    mut state: Ptr<brunsli_internal_dec_State>,
) -> brunsli_BrunsliStatus {
    let available: Value<usize> = Rc::new(RefCell::new(
        (({ GetBytesAvailable_166((state).clone()) }) & (!1 as usize)),
    ));
    let limit: Value<usize> = Rc::new(RefCell::new(
        ({ RemainingSectionLength_174((state).clone()) }),
    ));
    if !(((*limit.borrow()) & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1932,
                Ptr::<i8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    let mut chunk_len: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*limit.borrow()) as u64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    let mut is_last_chunk: bool = (chunk_len == (*limit.borrow()));
    let in_: Value<brunsli_WordSource> = Rc::new(RefCell::new(brunsli_WordSource::new(
        {
            state
                .with(|__s| __s.data.clone())
                .offset((state.with(|__s| __s.pos)) as isize)
        },
        { chunk_len },
        { is_last_chunk },
    )));
    let mut status: brunsli_BrunsliStatus = ({ DecodeDC_157((state).clone(), (in_.as_pointer())) });
    if !(({ (*in_.borrow()).pos_ } & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1941,
                Ptr::<i8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    if { (*in_.borrow()).error_ } {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if !({ (*in_.borrow()).pos_ } <= chunk_len) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1943,
                Ptr::<i8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    ({ SkipBytes_165((state).clone(), { (*in_.borrow()).pos_ }) });
    if is_last_chunk {
        if !((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    1946,
                    Ptr::<i8>::from_string_literal(b"DecodeDCDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({ IsAtSectionBoundary_175((state).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
    }
    return status;
}
pub fn DecodeACDataSection_191(
    mut state: Ptr<brunsli_internal_dec_State>,
) -> brunsli_BrunsliStatus {
    let available: Value<usize> = Rc::new(RefCell::new(
        (({ GetBytesAvailable_166((state).clone()) }) & (!1 as usize)),
    ));
    let limit: Value<usize> = Rc::new(RefCell::new(
        ({ RemainingSectionLength_174((state).clone()) }),
    ));
    if !(((*limit.borrow()) & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1955,
                Ptr::<i8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    let mut chunk_len: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*limit.borrow()) as u64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    let mut is_last_chunk: bool = (chunk_len == (*limit.borrow()));
    let in_: Value<brunsli_WordSource> = Rc::new(RefCell::new(brunsli_WordSource::new(
        {
            state
                .with(|__s| __s.data.clone())
                .offset((state.with(|__s| __s.pos)) as isize)
        },
        { chunk_len },
        { is_last_chunk },
    )));
    let mut status: brunsli_BrunsliStatus = ({ DecodeAC_160((state).clone(), (in_.as_pointer())) });
    if !(({ (*in_.borrow()).pos_ } & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1964,
                Ptr::<i8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    if { (*in_.borrow()).error_ } {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if !({ (*in_.borrow()).pos_ } <= chunk_len) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                1966,
                Ptr::<i8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    ({ SkipBytes_165((state).clone(), { (*in_.borrow()).pos_ }) });
    if is_last_chunk {
        if !((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    1969,
                    Ptr::<i8>::from_string_literal(b"DecodeACDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({ IsAtSectionBoundary_175((state).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
    }
    return status;
}
pub fn DecodeOriginalJpg_192(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let fs: Ptr<brunsli_internal_dec_FallbackState> = field_ptr!(s, fallback);
    'loop_: while (fs.with(|__s| __s.stage)
        != (brunsli_internal_dec_FallbackState_Stage_DONE as usize))
    {
        'switch: {
            match { fs.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_READ_TAG as usize) => {
                    let mut status: brunsli_BrunsliStatus =
                        ({ ReadTag_170((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    if ({ s.with(|__s| __s.section.tag) } != {
                        (kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow()) as usize)
                    }) || (!(s.with(|__s| __s.section.is_section)))
                    {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    field!(fs, stage)
                        .write((brunsli_internal_dec_FallbackState_Stage_ENTER_SECTION as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_ENTER_SECTION as usize) => {
                    let mut status: brunsli_BrunsliStatus =
                        ({ EnterSection_171((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    field!(jpg, original_jpg_size).write(s.with(|__s| __s.section.remaining));
                    if (jpg.with(|__s| __s.original_jpg_size) == 0_usize) {
                        field!(jpg, original_jpg).write(Ptr::<u8>::null());
                        field!(fs, stage)
                            .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                        break 'switch;
                    }
                    field!(fs, stage)
                        .write((brunsli_internal_dec_FallbackState_Stage_READ_CONTENTS as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_READ_CONTENTS as usize) => {
                    let chunk_size: Value<usize> =
                        Rc::new(RefCell::new(({ GetBytesAvailable_166((state).clone()) })));
                    if ((*chunk_size.borrow()) == 0_usize) {
                        return ({
                            Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
                        });
                    }
                    let mut src: Ptr<u8> = state
                        .with(|__s| __s.data.clone())
                        .offset((state.with(|__s| __s.pos)) as isize);
                    if (*fs.with(|__s| __s.storage.clone()).borrow()).is_empty() {
                        if ({ (*chunk_size.borrow()) } >= { jpg.with(|__s| __s.original_jpg_size) })
                        {
                            field!(jpg, original_jpg).write((src).clone());
                            ({
                                SkipBytes_165(
                                    (state).clone(),
                                    jpg.with(|__s| __s.original_jpg_size),
                                )
                            });
                            field!(fs, stage)
                                .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                            break 'switch;
                        }
                    }
                    let remaining: Value<usize> = Rc::new(RefCell::new(
                        ((jpg.with(|__s| __s.original_jpg_size) as u64).wrapping_sub(
                            ((*fs.with(|__s| __s.storage.clone()).borrow()).len() as u64),
                        ) as usize),
                    ));
                    let mut to_copy: usize = ({
                        let __tmp_0: Value<u64> =
                            Rc::new(RefCell::new(((*chunk_size.borrow()) as u64)));
                        let __tmp_1: Value<u64> =
                            Rc::new(RefCell::new(((*remaining.borrow()) as u64)));
                        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                            __tmp_0.as_pointer()
                        } else {
                            __tmp_1.as_pointer()
                        }
                        .read())
                    } as usize);
                    {
                        let start_idx = (fs.with(|__s| __s.storage.as_pointer()) as Ptr<u8>)
                            .to_end()
                            .get_offset();
                        let count = src.offset((to_copy) as isize).get_offset() - src.get_offset();
                        let temp_vec: Vec<u8> = PtrValueIter::new(&src, count).collect();
                        (fs.with(|__s| __s.storage.as_pointer()) as Ptr<Vec<u8>>).with_mut(
                            |v: &mut Vec<u8>| {
                                v.splice(start_idx..start_idx, temp_vec);
                            },
                        );
                        (fs.with(|__s| __s.storage.as_pointer()) as Ptr<Vec<u8>>) + start_idx
                    };
                    ({ SkipBytes_165((state).clone(), to_copy) });
                    if ({ (*fs.with(|__s| __s.storage.clone()).borrow()).len() } == {
                        jpg.with(|__s| __s.original_jpg_size)
                    }) {
                        let __rhs = (fs.with(|__s| __s.storage.as_pointer()) as Ptr<u8>);
                        field!(jpg, original_jpg).write(__rhs);
                        field!(fs, stage)
                            .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                        break 'switch;
                    }
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA) });
                }
                _ => {
                    return ({
                        Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR)
                    });
                }
            }
        };
    }
    ({ LeaveSection_172((field_ptr!(s, section))) });
    return brunsli_internal_dec_Stage_DONE;
}
pub fn ParseSection_193(mut state: Ptr<brunsli_internal_dec_State>) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let sh: Ptr<brunsli_internal_dec_SectionHeaderState> = field_ptr!(s, section_header);
    let mut result: brunsli_internal_dec_Stage = brunsli_internal_dec_Stage_ERROR;
    'loop_: while (sh.with(|__s| __s.stage)
        != (brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize))
    {
        'switch: {
            match { sh.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG as usize) => {
                    let mut status: brunsli_BrunsliStatus =
                        ({ ReadTag_170((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
                        if ({
                            HasSection_194(
                                (state).clone(),
                                (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                            )
                        }) {
                            return brunsli_internal_dec_Stage_DONE;
                        }
                    }
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    if s.with(|__s| __s.section.is_section) {
                        field!(sh, stage).write(
                            (brunsli_internal_dec_SectionHeaderState_Stage_ENTER_SECTION as usize),
                        );
                        continue 'loop_;
                    }
                    let mut tag_bit: u32 = (1_u32 << s.with(|__s| __s.section.tag));
                    let mut is_known_section_tag: bool =
                        ((kKnownSectionTags_137.with(|rc| *rc.borrow()) & tag_bit) != 0);
                    if is_known_section_tag {
                        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                    }
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_READ_VALUE as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_SectionHeaderState_Stage_READ_VALUE as usize) =>
                {
                    let sink: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let mut status: brunsli_BrunsliStatus =
                        ({ DecodeBase128_168((state).clone(), (sink.as_pointer())) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    result = brunsli_internal_dec_Stage_SECTION;
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_SectionHeaderState_Stage_ENTER_SECTION as usize) =>
                {
                    let mut status: brunsli_BrunsliStatus =
                        ({ EnterSection_171((state).clone(), (field_ptr!(s, section))) });
                    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169(state, status) });
                    }
                    result = brunsli_internal_dec_Stage_SECTION_BODY;
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize));
                    continue 'loop_;
                }
                _ => {
                    return ({
                        Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR)
                    });
                }
            }
        };
    }
    field!(sh, stage).write((brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG as usize));
    if !(result != brunsli_internal_dec_Stage_ERROR) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                2091,
                Ptr::<i8>::from_string_literal(b"ParseSection"),
            )
        });
        'loop_: while true {}
    };
    return result;
}
pub fn ProcessSection_195(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let mut tag_bit: i32 = ((1_u32 << s.with(|__s| __s.section.tag)) as i32);
    let mut is_known_section_tag: bool =
        ((kKnownSectionTags_137.with(|rc| *rc.borrow()) & (tag_bit as u32)) != 0);
    let mut skip_section: bool = (!(is_known_section_tag))
        || (({ state.with(|__s| __s.skip_tags) } & { (tag_bit as u32) }) != 0);
    if skip_section {
        let mut to_skip: usize = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                (({ GetBytesAvailable_166((state).clone()) }) as u64),
            ));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                (({ RemainingSectionLength_174((state).clone()) }) as u64),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        field!(state, pos).write({ (state.with(|__s| __s.pos)).wrapping_add(to_skip) });
        if (({ RemainingSectionLength_174((state).clone()) }) != 0_usize) {
            if !(({ GetBytesAvailable_166((state).clone()) }) == 0_usize) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                        2110,
                        Ptr::<i8>::from_string_literal(b"ProcessSection"),
                    )
                });
                'loop_: while true {}
            };
            return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA) });
        }
        return brunsli_internal_dec_Stage_SECTION;
    }
    'switch: {
        match { s.with(|__s| __s.section.tag) } {
            __v if __v == (kBrunsliMetaDataTag_32.with(|rc| *rc.borrow()) as usize) => {
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeMetaDataSection_180((state).clone(), (jpg).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as usize) => {
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeJPEGInternalsSection_184((state).clone(), (jpg).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (state).clone(),
                        (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeQuantDataSection_186((state).clone(), (jpg).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (state).clone(),
                        (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeHistogramDataSection_187((state).clone(), (jpg).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (state).clone(),
                        (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                if !({
                    HasSection_194(
                        (state).clone(),
                        (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                if ((({ RemainingSectionLength_174((state).clone()) }) & 1_usize) != 0_usize) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                ({ WarmupMeta_196((jpg).clone(), (state).clone()) });
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeDCDataSection_190((state).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (state).clone(),
                        (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                if ((({ RemainingSectionLength_174((state).clone()) }) & 1_usize) != 0_usize) {
                    return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
                }
                ({ WarmupMeta_196((jpg).clone(), (state).clone()) });
                let mut status: brunsli_BrunsliStatus =
                    ({ DecodeACDataSection_191((state).clone()) });
                if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169(state, status) });
                }
                break 'switch;
            }
            _ => {
                return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
            }
        }
    };
    if !({ IsAtSectionBoundary_175((state).clone()) }) {
        return ({ Fail_169(state, brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN) });
    }
    if ({ s.with(|__s| __s.section.tag) } == {
        (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as usize)
    }) {
        return brunsli_internal_dec_Stage_DONE;
    }
    return brunsli_internal_dec_Stage_SECTION;
}
pub fn UpdateSubsamplingDerivatives_178(mut jpg: Ptr<brunsli_JPEGData>) -> bool {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        let mut c: Ptr<brunsli_JPEGComponent> =
            ((jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i));
        let __rhs =
            (if field_ptr!(jpg, max_h_samp_factor).read() >= field_ptr!(c, h_samp_factor).read() {
                field_ptr!(jpg, max_h_samp_factor)
            } else {
                field_ptr!(c, h_samp_factor)
            }
            .read());
        field!(jpg, max_h_samp_factor).write(__rhs);
        let __rhs =
            (if field_ptr!(jpg, max_v_samp_factor).read() >= field_ptr!(c, v_samp_factor).read() {
                field_ptr!(jpg, max_v_samp_factor)
            } else {
                field_ptr!(c, v_samp_factor)
            }
            .read());
        field!(jpg, max_v_samp_factor).write(__rhs);
        i.prefix_inc();
    }
    let __rhs = ({
        let _a: i32 = jpg.with(|__s| __s.height);
        let _b: i32 = (jpg.with(|__s| __s.max_v_samp_factor) * 8);
        DivCeil_142(_a, _b)
    });
    field!(jpg, MCU_rows).write(__rhs);
    let __rhs = ({
        let _a: i32 = jpg.with(|__s| __s.width);
        let _b: i32 = (jpg.with(|__s| __s.max_h_samp_factor) * 8);
        DivCeil_142(_a, _b)
    });
    field!(jpg, MCU_cols).write(__rhs);
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() }) {
        let mut c: Ptr<brunsli_JPEGComponent> =
            ((jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i));
        field!(c, width_in_blocks).write({
            (({ jpg.with(|__s| __s.MCU_cols) } * { c.with(|__s| __s.h_samp_factor) }) as u32)
        });
        field!(c, height_in_blocks).write({
            (({ jpg.with(|__s| __s.MCU_rows) } * { c.with(|__s| __s.v_samp_factor) }) as u32)
        });
        if !(c.with(|__s| __s.width_in_blocks) <= 8205_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2211,
                    Ptr::<i8>::from_string_literal(b"UpdateSubsamplingDerivatives"),
                )
            });
            'loop_: while true {}
        };
        if !(c.with(|__s| __s.height_in_blocks) <= 8205_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2212,
                    Ptr::<i8>::from_string_literal(b"UpdateSubsamplingDerivatives"),
                )
            });
            'loop_: while true {}
        };
        let mut num_blocks: u32 =
            (c.with(|__s| __s.width_in_blocks)).wrapping_mul(c.with(|__s| __s.height_in_blocks));
        if ((num_blocks as usize) > kBrunsliMaxNumBlocks_18.with(|rc| *rc.borrow())) {
            return false;
        }
        field!(c, num_blocks).write(num_blocks);
        i.prefix_inc();
    }
    return true;
}
pub fn PrepareMeta_179(mut jpg: Ptr<brunsli_JPEGData>, mut state: Ptr<brunsli_internal_dec_State>) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let mut num_components: usize = (*jpg.with(|__s| __s.components.clone()).borrow()).len();
    {
        let _a0 = num_components as usize;
        (s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<u8>>>| __v.resize_with(_a0, <Value<Vec<u8>>>::default),
        )
    };
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    {
        let __a0 = num_components as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_dec_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_dec_ComponentMeta>::default())
        })
    };
    let mut i: usize = 0_usize;
    'loop_: while (i < num_components) {
        let c: Ptr<brunsli_JPEGComponent> =
            (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>).offset(i);
        let m: Ptr<brunsli_internal_dec_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset(i);
        field!(m, h_samp).write(c.with(|__s| __s.h_samp_factor));
        field!(m, v_samp).write(c.with(|__s| __s.v_samp_factor));
        field!(m, width_in_blocks)
            .write({ ({ jpg.with(|__s| __s.MCU_cols) } * { m.with(|__s| __s.h_samp) }) });
        field!(m, height_in_blocks)
            .write({ ({ jpg.with(|__s| __s.MCU_rows) } * { m.with(|__s| __s.v_samp) }) });
        i.prefix_inc();
    }
}
pub fn WarmupMeta_196(mut jpg: Ptr<brunsli_JPEGData>, mut state: Ptr<brunsli_internal_dec_State>) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        state.with(|__s| __s.meta.as_pointer());
    let mut num_components: usize = (*meta.upgrade().deref()).len();
    if !(state.with(|__s| __s.is_storage_allocated)) {
        field!(state, is_storage_allocated).write(true);
        let mut i: usize = 0_usize;
        'loop_: while (i < num_components) {
            let mut num_blocks: usize = (({
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                }
            } * {
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .height_in_blocks
                }
            }) as usize);
            {
                let __a0 = (num_blocks)
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
                    as usize;
                (*{
                    (*elem!(
                        (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                        i
                    )
                    .upgrade()
                    .deref())
                    .coeffs
                    .clone()
                }
                .borrow_mut())
                .resize_with(__a0, || <i16>::default())
            };
            {
                let __a0 = num_blocks as usize;
                elem!(
                    (s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Value<Vec<u8>>>),
                    i
                )
                .with_mut(|__v: &mut Value<Vec<u8>>| {
                    (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                })
            };
            let __rhs = ((s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset(i)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>);
            field!(
                elem!(
                    (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                        as Ptr<brunsli_internal_dec_ComponentMeta>),
                    i
                ),
                block_state
            )
            .write(__rhs);
            i.prefix_inc();
        }
    }
    if !(s.with(|__s| __s.is_meta_warm)) {
        field!(s, is_meta_warm).write(true);
        let mut c: usize = 0_usize;
        'loop_: while (c < num_components) {
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset(c);
            let q: Ptr<brunsli_JPEGQuantTable> =
                (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>).offset(
                    ({
                        (*elem!(
                            (jpg.with(|__s| __s.components.as_pointer())
                                as Ptr<brunsli_JPEGComponent>),
                            c
                        )
                        .upgrade()
                        .deref())
                        .quant_idx
                    } as usize),
                );
            field!(m, ac_coeffs).write(
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                        c
                    )
                    .upgrade()
                    .deref())
                    .coeffs
                    .as_pointer()
                } as Ptr<i16>),
            );
            field!(m, ac_stride).write({
                ({ m.with(|__s| __s.width_in_blocks) } * {
                    kDCTBlockSize_3.with(|rc| *rc.borrow())
                })
            });
            field!(m, b_stride).write({ m.with(|__s| __s.width_in_blocks) });
            {
                ((m.with(|__s| __s.quant.as_pointer()) as Ptr<i32>) as Ptr<i32>)
                    .to_any()
                    .memcpy(
                        &((q.with(|__s| __s.values.as_pointer()) as Ptr<i32>) as Ptr<i32>).to_any(),
                        (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)
                            .wrapping_mul((::std::mem::size_of::<i32>() as usize))
                            as usize,
                    );
                ((m.with(|__s| __s.quant.as_pointer()) as Ptr<i32>) as Ptr<i32>).to_any()
            };
            c.prefix_inc();
        }
    }
}
pub fn DoProcessJpeg_197(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    'loop_: while true {
        'switch: {
            match { state.with(|__s| __s.stage) } {
                __v if __v == brunsli_internal_dec_Stage_SIGNATURE => {
                    let __rhs = ({ VerifySignature_176((state).clone()) });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_HEADER => {
                    let __rhs = ({ DecodeHeader_177((state).clone(), (jpg).clone()) });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_FALLBACK => {
                    let __rhs = ({ DecodeOriginalJpg_192((state).clone(), (jpg).clone()) });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_SECTION => {
                    let __rhs = ({ ParseSection_193((state).clone()) });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_SECTION_BODY => {
                    let __rhs = ({ ProcessSection_195((state).clone(), (jpg).clone()) });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_DONE => {
                    if ({ state.with(|__s| __s.pos) } != { state.with(|__s| __s.len) }) {
                        let __rhs = ({
                            Fail_169((state).clone(), brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                        });
                        field!(state, stage).write(__rhs);
                        break 'switch;
                    }
                    return brunsli_BrunsliStatus_BRUNSLI_OK;
                }
                __v if __v == brunsli_internal_dec_Stage_ERROR => {
                    return {
                        (*state
                            .with(|__s| __s.internal.clone())
                            .as_ref()
                            .unwrap()
                            .borrow())
                        .result
                    };
                }
                _ => {
                    let __rhs = ({
                        Fail_169(
                            (state).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
                        )
                    });
                    field!(state, stage).write(__rhs);
                    break 'switch;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ChargeBuffer_198(mut state: Ptr<brunsli_internal_dec_State>) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    field!(b, borrowed_len).write(0_usize);
    field!(b, external_data).write(state.with(|__s| __s.data.clone()));
    field!(b, external_pos).write(state.with(|__s| __s.pos));
    field!(b, external_len).write(state.with(|__s| __s.len));
}
thread_local!(
    pub static kBufferMaxReadAhead_199: Value<usize> = Rc::new(RefCell::new(600_usize));
);
pub fn LoadInput_200(mut state: Ptr<brunsli_internal_dec_State>) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    if (b.with(|__s| __s.data_len) == 0_usize) {
        field!(state, data).write(b.with(|__s| __s.external_data.clone()));
        field!(state, pos).write(b.with(|__s| __s.external_pos));
        field!(state, len).write(b.with(|__s| __s.external_len));
        return;
    }
    if !({ b.with(|__s| __s.data_len) } <= { kBufferMaxReadAhead_199.with(|rc| *rc.borrow()) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                2337,
                Ptr::<i8>::from_string_literal(b"LoadInput"),
            )
        });
        'loop_: while true {}
    };
    let available: Value<usize> = Rc::new(RefCell::new(
        (b.with(|__s| __s.external_len)).wrapping_sub(b.with(|__s| __s.external_pos)),
    ));
    field!(b, borrowed_len).write(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                (kBufferMaxReadAhead_199.with(|rc| *rc.borrow()) as u64),
            ));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    );
    {
        ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>)
            .offset((b.with(|__s| __s.data_len)) as isize) as Ptr<u8>)
            .to_any()
            .memcpy(
                &(b.with(|__s| __s.external_data.clone())
                    .offset((b.with(|__s| __s.external_pos)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                b.with(|__s| __s.borrowed_len) as usize,
            );
        ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>)
            .offset((b.with(|__s| __s.data_len)) as isize) as Ptr<u8>)
            .to_any()
    };
    let __rhs = (b.with(|__s| __s.data.as_pointer()) as Ptr<u8>);
    field!(state, data).write(__rhs);
    field!(state, pos).write(0_usize);
    field!(state, len)
        .write((b.with(|__s| __s.data_len)).wrapping_add(b.with(|__s| __s.borrowed_len)));
}
pub fn UnloadInput_201(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut result: brunsli_BrunsliStatus,
) -> bool {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    if ({ state.with(|__s| __s.data.clone()) } == { b.with(|__s| __s.external_data.clone()) }) {
        field!(b, external_pos).write(state.with(|__s| __s.pos));
        if !({ b.with(|__s| __s.external_pos) } <= { b.with(|__s| __s.external_len) }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2364,
                    Ptr::<i8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        if ((result as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            return true;
        }
        if !(b.with(|__s| __s.data_len) == 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2366,
                    Ptr::<i8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        let mut available: usize =
            (b.with(|__s| __s.external_len)).wrapping_sub(b.with(|__s| __s.external_pos));
        if !(available < kBufferMaxReadAhead_199.with(|rc| *rc.borrow())) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2368,
                    Ptr::<i8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        if (*b.with(|__s| __s.data.clone()).borrow()).is_empty() {
            {
                let __a0 = (2_usize).wrapping_mul(kBufferMaxReadAhead_199.with(|rc| *rc.borrow()))
                    as usize;
                (*b.with(|__s| __s.data.clone()).borrow_mut()).resize_with(__a0, || <u8>::default())
            };
        }
        field!(b, data_len).write(available);
        {
            ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &(b.with(|__s| __s.external_data.clone())
                        .offset((b.with(|__s| __s.external_pos)) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    b.with(|__s| __s.data_len) as usize,
                );
            ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any()
        };
        field!(b, external_pos).write({ (b.with(|__s| __s.external_pos)).wrapping_add(available) });
        return false;
    }
    if ({ state.with(|__s| __s.pos) } >= { b.with(|__s| __s.data_len) }) {
        let mut used_borrowed_bytes: usize =
            (state.with(|__s| __s.pos)).wrapping_sub(b.with(|__s| __s.data_len));
        field!(b, data_len).write(0_usize);
        field!(b, external_pos)
            .write({ (b.with(|__s| __s.external_pos)).wrapping_add(used_borrowed_bytes) });
        return true;
    }
    field!(b, data_len)
        .write({ (b.with(|__s| __s.data_len)).wrapping_sub(state.with(|__s| __s.pos)) });
    if ((result as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if !({ (b.with(|__s| __s.external_pos)).wrapping_add(b.with(|__s| __s.borrowed_len)) } == {
            b.with(|__s| __s.external_len)
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2389,
                    Ptr::<i8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        if !({ (b.with(|__s| __s.data_len)).wrapping_add(b.with(|__s| __s.borrowed_len)) } < {
            kBufferMaxReadAhead_199.with(|rc| *rc.borrow())
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2391,
                    Ptr::<i8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        field!(b, data_len)
            .write({ (b.with(|__s| __s.data_len)).wrapping_add(b.with(|__s| __s.borrowed_len)) });
        field!(b, external_pos).write({
            (b.with(|__s| __s.external_pos)).wrapping_add(b.with(|__s| __s.borrowed_len))
        });
    }
    if !(!((*b.with(|__s| __s.data.clone()).borrow()).is_empty())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                2395,
                Ptr::<i8>::from_string_literal(b"UnloadInput"),
            )
        });
        'loop_: while true {}
    };
    if (state.with(|__s| __s.pos) > 0_usize) && (b.with(|__s| __s.data_len) > 0_usize) {
        {
            ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>)
                        .offset((state.with(|__s| __s.pos)) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    b.with(|__s| __s.data_len) as usize,
                );
            ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any()
        };
    }
    if !({ b.with(|__s| __s.data_len) } <= { kBufferMaxReadAhead_199.with(|rc| *rc.borrow()) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                2399,
                Ptr::<i8>::from_string_literal(b"UnloadInput"),
            )
        });
        'loop_: while true {}
    };
    return ((result as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32));
}
pub fn UnchargeBuffer_202(mut state: Ptr<brunsli_internal_dec_State>) {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    field!(state, data).write(b.with(|__s| __s.external_data.clone()));
    field!(state, pos).write(b.with(|__s| __s.external_pos));
    field!(state, len).write(b.with(|__s| __s.external_len));
}
pub fn ProcessJpeg_203(
    mut state: Ptr<brunsli_internal_dec_State>,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let s: Ptr<brunsli_internal_dec_InternalState> =
        state.with(|__s| __s.internal.clone()).as_pointer();
    if ({ state.with(|__s| __s.pos) } > { state.with(|__s| __s.len) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM;
    }
    ({ ChargeBuffer_198((state).clone()) });
    let mut result: brunsli_BrunsliStatus = brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    'loop_: while ((result as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if (state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_ERROR) {
            if ((s.with(|__s| __s.result) as i32)
                != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
            {
                return s.with(|__s| __s.result);
            }
            field!(s, result).write(brunsli_BrunsliStatus_BRUNSLI_OK);
            field!(state, stage).write(s.with(|__s| __s.last_stage));
            field!(s, last_stage).write(brunsli_internal_dec_Stage_ERROR);
        }
        ({ LoadInput_200((state).clone()) });
        if s.with(|__s| __s.section.is_active) {
            field!(field!(s, section), milestone).write(state.with(|__s| __s.pos));
            field!(field!(s, section), projected_end).write({
                (s.with(|__s| __s.section.milestone))
                    .wrapping_add(s.with(|__s| __s.section.remaining))
            });
        }
        {
            let __rhs = state.with(|__s| __s.tags_met);
            field!(field!(s, section), tags_met).with_mut(|__v| *__v = *__v | __rhs)
        };
        result = ({ DoProcessJpeg_197((state).clone(), (jpg).clone()) });
        if s.with(|__s| __s.section.is_active) {
            let mut processed_len: usize =
                (state.with(|__s| __s.pos)).wrapping_sub(s.with(|__s| __s.section.milestone));
            field!(field!(s, section), remaining)
                .write({ (s.with(|__s| __s.section.remaining)).wrapping_sub(processed_len) });
        }
        if !({ UnloadInput_201((state).clone(), result) }) {
            break;
        }
    }
    ({ UnchargeBuffer_202((state).clone()) });
    return result;
}
pub fn BrunsliDecodeJpeg_204(
    mut data: Ptr<u8>,
    mut len: usize,
    mut jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    if !(!(data).is_null()) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM;
    }
    let state: Value<brunsli_internal_dec_State> =
        Rc::new(RefCell::new(brunsli_internal_dec_State::new()));
    (*state.borrow_mut()).data = (data).clone();
    (*state.borrow_mut()).len = len;
    return ({ ProcessJpeg_203((state.as_pointer()), jpg) });
}
pub fn BrunsliEstimateDecoderPeakMemoryUsage_205(mut data: Ptr<u8>, mut len: usize) -> usize {
    if !(!(data).is_null()) {
        return (brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM as usize);
    }
    let state: Value<brunsli_internal_dec_State> =
        Rc::new(RefCell::new(brunsli_internal_dec_State::new()));
    (*state.borrow_mut()).data = (data).clone();
    (*state.borrow_mut()).len = len;
    (*state.borrow_mut()).skip_tags =
        !(1_u32 << (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as i32));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        { (*state.borrow()).internal.clone() }.as_pointer();
    field!(s, shallow_histograms).write(true);
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
    let mut status: brunsli_BrunsliStatus =
        ({ ProcessJpeg_203((state.as_pointer()), (jpg.as_pointer())) });
    if ((status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
        return 0_usize;
    }
    let mut out_size: usize = (2_usize).wrapping_mul(len);
    let mut total_num_blocks: usize = 0_usize;
    let mut component_state_size: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < (*{ (*jpg.borrow()).components.clone() }.borrow()).len()) {
        let c: Ptr<brunsli_JPEGComponent> =
            ({ (*jpg.borrow()).components.as_pointer() } as Ptr<brunsli_JPEGComponent>).offset(i);
        total_num_blocks =
            { (total_num_blocks).wrapping_add((c.with(|__s| __s.num_blocks) as usize)) };
        {
            let rhs_0 = (component_state_size).wrapping_add(
                ({
                    brunsli_ComponentState::SizeInBytes((c.with(|__s| __s.width_in_blocks) as i32))
                }),
            );
            component_state_size = rhs_0
        };
        i.prefix_inc();
    }
    let mut jpeg_data_size: usize =
        (((total_num_blocks).wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
            as u64)
            .wrapping_mul((::std::mem::size_of::<i16>() as u64)) as usize);
    let mut context_map_size: usize = (((s.with(|__s| __s.num_contexts))
        .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow()))
        as u64)
        .wrapping_mul((::std::mem::size_of::<i32>() as u64))
        as usize);
    let mut histogram_size: usize =
        ((s.with(|__s| __s.num_histograms) as u64).wrapping_mul((6144usize as u64)) as usize);
    let decode_peak: Value<usize> = Rc::new(RefCell::new(
        ((context_map_size).wrapping_add(histogram_size)).wrapping_add(component_state_size),
    ));
    let jpeg_writer_size: Value<usize> =
        Rc::new(RefCell::new(((1_u32 << 17_u32) as usize).wrapping_add(
            (((1_u32 << 16_u32) as usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))
                as usize),
        )));
    return ((((out_size).wrapping_add(jpeg_data_size) as u64).wrapping_add({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*decode_peak.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*jpeg_writer_size.borrow()) as u64)));
        (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    })) as usize);
}
impl brunsli_BrunsliDecoder {}
pub fn MoveToFront_207(mut v: Ptr<u8>, mut index: u8) {
    let mut value: u8 = (elem!(v, index).read());
    let mut i: u8 = index;
    'loop_: while (i != 0) {
        elem!(v, i).write({ (elem!(v, ((i as i32) - 1)).read()) });
        i.prefix_dec();
    }
    elem!(v, 0).write({ value });
}
pub fn InverseMoveToFrontTransform_208(mut v: Ptr<u8>, mut v_len: usize) {
    let mtf: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..256).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let mut i: usize = 0_usize;
    'loop_: while (i < 256_usize) {
        (*mtf.borrow_mut())[(i) as usize] = { (i as u8) };
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < v_len) {
        let mut index: u8 = (elem!(v, i).read());
        elem!(v, i).write({ (*mtf.borrow())[(index) as usize] });
        if (index != 0) {
            ({ MoveToFront_207((mtf.as_pointer() as Ptr<u8>), index) });
        }
        i.prefix_inc();
    }
}
pub fn DecodeContextMap_188(
    entropy: Ptr<brunsli_HuffmanDecodingData>,
    mut max_run_length_prefix: usize,
    mut index: Ptr<usize>,
    mut context_map: Ptr<Vec<u8>>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
) -> brunsli_BrunsliStatus {
    let i: Ptr<usize> = (index).clone();
    let mut map: Ptr<u8> = (Ptr::<Vec<u8>>::decay(&(context_map)) as Ptr<u8>);
    let mut length: usize = (*context_map.upgrade().deref()).len();
    'loop_: while ({ (i.read()) } < { length }) {
        if !({
            BrunsliBitReaderCanRead_134(
                (br).clone(),
                ((15_usize).wrapping_add(max_run_length_prefix)).wrapping_add(1_usize),
            )
        }) {
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
        let mut code: u32 = (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (br).clone();
            brunsli_HuffmanDecodingDataImpl::ReadSymbol(&entropy, _br)
        }) as u32);
        if (code == 0_u32) {
            elem!(map, (i.read())).write(0_u8);
            i.with_mut(|__v| __v.prefix_inc());
        } else if ((code as usize) <= max_run_length_prefix) {
            let mut reps: usize = ((((1_u32 as u32).wrapping_add((1_u32 << code)))
                .wrapping_add(((({ BrunsliBitReaderRead_126((br).clone(), code) }) as i32) as u32)))
                as usize);
            'loop_: while (reps.prefix_dec() != 0) {
                if ({ (i.read()) } >= { length }) {
                    return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                }
                elem!(map, (i.read())).write(0_u8);
                i.with_mut(|__v| __v.prefix_inc());
            }
        } else {
            elem!(map, (i.read()))
                .write({ (((code as usize).wrapping_sub(max_run_length_prefix)) as u8) });
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    if (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) != 0) {
        ({ InverseMoveToFrontTransform_208((map).clone(), length) });
    }
    return if ({ BrunsliBitReaderIsHealthy_132(br) }) {
        brunsli_BrunsliStatus_BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
    };
}
pub fn GetPopulationCountPrecision_209(mut logcount: u32) -> u32 {
    return (((logcount).wrapping_add(1_u32)) >> 1);
}
thread_local!(
    pub static kLengthTree_210: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        1_i8,
        2_i8,
        3_i8,
        4_i8,
        5_i8,
        6_i8,
        7_i8,
        (-10_i32 as i8),
        (-11_i32 as i8),
        (-12_i32 as i8),
        (-13_i32 as i8),
        (-14_i32 as i8),
        (-15_i32 as i8),
        2_i8,
        3_i8,
        (-9_i32 as i8),
        (-16_i32 as i8),
        2_i8,
        3_i8,
        (-8_i32 as i8),
        (-17_i32 as i8),
        2_i8,
        3_i8,
        (-5_i32 as i8),
        (-6_i32 as i8),
        (-7_i32 as i8),
        1_i8,
        (-18_i32 as i8),
        1_i8,
        (-3_i32 as i8),
        (-4_i32 as i8),
    ])));
);
thread_local!(
    pub static kLogCountTree_211: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        1_i8,
        2_i8,
        3_i8,
        (-6_i32 as i8),
        3_i8,
        4_i8,
        5_i8,
        (-4_i32 as i8),
        (-5_i32 as i8),
        (-7_i32 as i8),
        (-8_i32 as i8),
        2_i8,
        3_i8,
        (-1_i32 as i8),
        (-2_i32 as i8),
        (-3_i32 as i8),
        1_i8,
        0_i8,
        1_i8,
        (-9_i32 as i8),
        (-10_i32 as i8),
    ])));
);
pub fn ReadShortHuffmanCode_212(mut br: Ptr<brunsli_BrunsliBitReader>, mut tree: Ptr<i8>) -> usize {
    let mut pos: usize = 0_usize;
    let mut delta: i8 = 1_i8;
    'loop_: while ((delta as i32) > 0) {
        {
            let rhs_0 = (pos).wrapping_add(
                (((delta as u32).wrapping_add(({ BrunsliBitReaderRead_126((br).clone(), 1_u32) })))
                    as usize),
            );
            pos = rhs_0
        };
        delta = { (elem!(tree, pos).read()) };
    }
    return (-(delta as i32) as usize);
}
pub fn ReadHistogram_189(
    mut precision_bits: u32,
    mut counts: Ptr<Vec<u32>>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    if !(!((*counts.upgrade().deref()).is_empty())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_decode.cc"),
                41,
                Ptr::<i8>::from_string_literal(b"ReadHistogram"),
            )
        });
        'loop_: while true {}
    };
    let mut space: u32 = (1_u32 << precision_bits);
    let mut length: usize = (*counts.upgrade().deref()).len();
    {
        let mut __a0 = (Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>);
        while __a0 != (Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>).to_end() {
            let v = 0.clone();
            __a0.write(v);
            __a0 += 1;
        }
    };
    let mut histogram: Ptr<u32> = (Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>);
    let mut simple_code: i32 = (({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as i32);
    if (simple_code == 1) {
        let mut max_bits_counter: usize = (length).wrapping_sub(1_usize);
        let mut max_bits: u32 = 0_u32;
        let mut symbols: [i32; 2] = [0, 0_i32];
        let mut num_symbols: usize = ((({ BrunsliBitReaderRead_126((br).clone(), 1_u32) })
            .wrapping_add((1_u32 as u32))) as usize);
        'loop_: while (max_bits_counter != 0) {
            max_bits_counter >>= 1;
            max_bits.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < num_symbols) {
            let __rhs = (((({ BrunsliBitReaderRead_126((br).clone(), max_bits) }) as usize)
                .wrapping_rem(length)) as i32);
            symbols[(i) as usize] = __rhs;
            i.prefix_inc();
        }
        if (num_symbols == 1_usize) {
            elem!(histogram, symbols[(0) as usize]).write({ space });
        } else {
            if (symbols[(0) as usize] == symbols[(1) as usize]) {
                return false;
            }
            let mut value: u32 = ({ BrunsliBitReaderRead_126((br).clone(), precision_bits) });
            elem!(histogram, symbols[(0) as usize]).write({ value });
            elem!(histogram, symbols[(1) as usize]).write({ (space).wrapping_sub(value) });
        }
    } else {
        let mut real_length: usize = ({
            ReadShortHuffmanCode_212(
                (br).clone(),
                (kLengthTree_210.with(|v| v.as_pointer()) as Ptr<i8>),
            )
        });
        let mut total_count: u32 = 0_u32;
        let mut log_counts: [u32; 18] = [0_u32; 18];
        let mut omit_pos: usize = 0_usize;
        if !(real_length > 2_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"histogram_decode.cc"),
                    74,
                    Ptr::<i8>::from_string_literal(b"ReadHistogram"),
                )
            });
            'loop_: while true {}
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < real_length) {
            let __rhs = (({
                ReadShortHuffmanCode_212(
                    (br).clone(),
                    (kLogCountTree_211.with(|v| v.as_pointer()) as Ptr<i8>),
                )
            }) as u32);
            log_counts[(i) as usize] = __rhs;
            if (log_counts[(i) as usize] > log_counts[(omit_pos) as usize]) {
                omit_pos = i;
            }
            i.prefix_inc();
        }
        if !(omit_pos >= 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"histogram_decode.cc"),
                    80,
                    Ptr::<i8>::from_string_literal(b"ReadHistogram"),
                )
            });
            'loop_: while true {}
        };
        let mut i: usize = 0_usize;
        'loop_: while (i < real_length) {
            let mut code: u32 = log_counts[(i) as usize];
            if (i == omit_pos) {
                i.prefix_inc();
                continue 'loop_;
            } else if (code == 0_u32) {
                i.prefix_inc();
                continue 'loop_;
            } else if (code == 1_u32) {
                elem!(histogram, i).write(1_u32);
            } else {
                let mut bit_count: u32 =
                    ({ GetPopulationCountPrecision_209((code).wrapping_sub(1_u32)) });
                let __rhs = (1_u32 << ((code).wrapping_sub(1_u32))).wrapping_add(
                    ({ ({ BrunsliBitReaderRead_126((br).clone(), bit_count) }) } << {
                        (((code).wrapping_sub(1_u32)).wrapping_sub(bit_count))
                    }),
                );
                elem!(histogram, i).write(__rhs);
            }
            total_count = { (total_count).wrapping_add((elem!(histogram, i).read())) };
            i.prefix_inc();
        }
        if (total_count >= space) {
            return false;
        }
        elem!(histogram, omit_pos).write({ (space).wrapping_sub(total_count) });
    }
    return ({ BrunsliBitReaderIsHealthy_132(br) });
}
thread_local!(
    pub static kCodeLengthCodes_213: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kCodeLengthCodeOrder_214: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        1_u8, 2_u8, 3_u8, 4_u8, 0_u8, 5_u8, 17_u8, 6_u8, 16_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        12_u8, 13_u8, 14_u8, 15_u8,
    ])));
);
thread_local!(
    pub static kDefaultCodeLength_215: Value<u8> = Rc::new(RefCell::new(8_u8));
);
thread_local!(
    pub static kCodeLengthRepeatCode_216: Value<u8> = Rc::new(RefCell::new(16_u8));
);
pub fn ReadHuffmanCodeLengths_217(
    mut code_length_code_lengths: Ptr<u8>,
    mut num_symbols: usize,
    mut code_lengths: Ptr<u8>,
    mut br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    let mut symbol: usize = 0_usize;
    let mut prev_code_len: u8 = kDefaultCodeLength_215.with(|rc| *rc.borrow());
    let mut repeat: usize = 0_usize;
    let mut repeat_code_len: u8 = 0_u8;
    let mut kFullSpace: i32 = (1 << 15);
    let mut space: i32 = kFullSpace;
    let table: Value<Box<[brunsli_HuffmanCode]>> = Rc::new(RefCell::new(
        (0..32)
            .map(|_| <brunsli_HuffmanCode>::default())
            .collect::<Box<[brunsli_HuffmanCode]>>(),
    ));
    let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ])));
    let mut i: i32 = 0;
    'loop_: while (i < kCodeLengthCodes_213.with(|rc| *rc.borrow())) {
        (*counts.borrow_mut())[(elem!(code_length_code_lengths, i).read()) as usize].prefix_inc();
        i.prefix_inc();
    }
    if !(({
        BuildHuffmanTable_218(
            (table.as_pointer() as Ptr<brunsli_HuffmanCode>),
            5_usize,
            (code_length_code_lengths).clone(),
            (kCodeLengthCodes_213.with(|rc| *rc.borrow()) as usize),
            ((counts.as_pointer() as Ptr<u16>).offset(0)),
        )
    }) != 0)
    {
        return false;
    }
    'loop_: while (symbol < num_symbols) && (space > 0) {
        let mut p: Ptr<brunsli_HuffmanCode> = (table.as_pointer() as Ptr<brunsli_HuffmanCode>);
        let mut code_len: u8 = 0_u8;
        p += ({ BrunsliBitReaderGet_124((br).clone(), 5_u32) });
        ({ BrunsliBitReaderDrop_125((br).clone(), (p.with(|__s| __s.bits) as u32)) });
        code_len = (p.with(|__s| __s.value) as u8);
        if ((code_len as i32) < (kCodeLengthRepeatCode_216.with(|rc| *rc.borrow()) as i32)) {
            repeat = 0_usize;
            let __rhs = code_len;
            elem!(code_lengths, symbol.postfix_inc()).write(__rhs);
            if ((code_len as i32) != 0) {
                prev_code_len = code_len;
                space -= (kFullSpace >> (code_len as i32));
            }
        } else {
            let mut extra_bits: u32 = (((code_len as i32) - 14) as u32);
            let mut old_repeat: usize = 0_usize;
            let mut repeat_delta: usize = 0_usize;
            let mut new_len: u8 = 0_u8;
            if ((code_len as i32) == (kCodeLengthRepeatCode_216.with(|rc| *rc.borrow()) as i32)) {
                new_len = prev_code_len;
            }
            if ((repeat_code_len as i32) != (new_len as i32)) {
                repeat = 0_usize;
                repeat_code_len = new_len;
            }
            old_repeat = repeat;
            if (repeat > 0_usize) {
                repeat = { (repeat).wrapping_sub(2_usize) };
                repeat <<= extra_bits;
            }
            {
                let rhs_0 = (repeat).wrapping_add(
                    ((({ BrunsliBitReaderRead_126((br).clone(), extra_bits) })
                        .wrapping_add((3_u32 as u32))) as usize),
                );
                repeat = rhs_0
            };
            repeat_delta = (repeat).wrapping_sub(old_repeat);
            if ((symbol).wrapping_add(repeat_delta) > num_symbols) {
                return false;
            }
            {
                ((code_lengths.offset((symbol) as isize)) as Ptr<u8>)
                    .to_any()
                    .memset((repeat_code_len as i32) as u8, repeat_delta as usize);
                ((code_lengths.offset((symbol) as isize)) as Ptr<u8>).to_any()
            };
            symbol = { (symbol).wrapping_add(repeat_delta) };
            if ((repeat_code_len as i32) != 0) {
                space -= ((((repeat_delta).wrapping_mul((kFullSpace as usize))) as i32)
                    >> (repeat_code_len as i32));
            }
        }
    }
    if (space != 0) {
        return false;
    }
    {
        ((code_lengths.offset((symbol) as isize)) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ((num_symbols).wrapping_sub(symbol)) as usize);
        ((code_lengths.offset((symbol) as isize)) as Ptr<u8>).to_any()
    };
    return ({ BrunsliBitReaderIsHealthy_132(br) });
}
pub fn ReadSimpleCode_219(
    mut alphabet_size: u16,
    mut br: Ptr<brunsli_BrunsliBitReader>,
    mut table: Ptr<brunsli_HuffmanCode>,
) -> bool {
    let mut max_bits: u32 = (if ((alphabet_size as u32) > 1_u32) {
        (({ Log2FloorNonZero_74((alphabet_size as u32).wrapping_sub((1_u32 as u32))) }) + 1)
    } else {
        0
    } as u32);
    let mut num_symbols: usize =
        ((({ BrunsliBitReaderRead_126((br).clone(), 2_u32) }).wrapping_add(1_u32)) as usize);
    let symbols: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([0_u16, 0_u16, 0_u16, 0_u16])));
    let mut i: usize = 0_usize;
    'loop_: while (i < num_symbols) {
        let mut symbol: u16 = (({ BrunsliBitReaderRead_126((br).clone(), max_bits) }) as u16);
        if ((symbol as i32) >= (alphabet_size as i32)) {
            return false;
        }
        (*symbols.borrow_mut())[(i) as usize] = symbol;
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < (num_symbols).wrapping_sub(1_usize)) {
        let mut j: usize = (i).wrapping_add(1_usize);
        'loop_: while (j < num_symbols) {
            if (((*symbols.borrow())[(i) as usize] as i32)
                == ((*symbols.borrow())[(j) as usize] as i32))
            {
                return false;
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    if (num_symbols == 4_usize) {
        {
            let rhs_0 = (num_symbols)
                .wrapping_add((({ BrunsliBitReaderRead_126((br).clone(), 1_u32) }) as usize));
            num_symbols = rhs_0
        };
    }
    let swap_symbols: Value<FnPtr<fn(usize, usize)>> = Rc::new(RefCell::new(lambda!(
        {
            let symbols: Ptr<u16> = symbols.as_pointer();
        },
        |i: usize, j: usize| {
            let mut t: u16 = (elem!((symbols), j).read());
            elem!((symbols), j).write({ (elem!((symbols), i).read()) });
            elem!((symbols), i).write(t);
        }
    )));
    let mut table_size: usize = 1_usize;
    'switch: {
        match { num_symbols } {
            __v if __v == 1_usize => {
                elem!(table, 0).write({
                    brunsli_HuffmanCode {
                        bits: 0_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                break 'switch;
            }
            __v if __v == 2_usize => {
                if (((*symbols.borrow())[(0) as usize] as i32)
                    > ((*symbols.borrow())[(1) as usize] as i32))
                {
                    ({ (*swap_symbols.borrow()).call(0_usize, 1_usize) });
                }
                elem!(table, 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 1).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                table_size = 2_usize;
                break 'switch;
            }
            __v if __v == 3_usize => {
                if (((*symbols.borrow())[(1) as usize] as i32)
                    > ((*symbols.borrow())[(2) as usize] as i32))
                {
                    ({ (*swap_symbols.borrow()).call(1_usize, 2_usize) });
                }
                elem!(table, 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 2).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!(table, 3).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                table_size = 4_usize;
                break 'switch;
            }
            __v if __v == 4_usize => {
                let mut i: usize = 0_usize;
                'loop_: while (i < 3_usize) {
                    let mut j: usize = (i).wrapping_add(1_usize);
                    'loop_: while (j < 4_usize) {
                        if (((*symbols.borrow())[(i) as usize] as i32)
                            > ((*symbols.borrow())[(j) as usize] as i32))
                        {
                            ({ (*swap_symbols.borrow()).call(i, j) });
                        }
                        j.prefix_inc();
                    }
                    i.prefix_inc();
                }
                elem!(table, 0).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 2).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!(table, 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                elem!(table, 3).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(3) as usize],
                    }
                });
                table_size = 4_usize;
                break 'switch;
            }
            __v if __v == 5_usize => {
                if (((*symbols.borrow())[(2) as usize] as i32)
                    > ((*symbols.borrow())[(3) as usize] as i32))
                {
                    ({ (*swap_symbols.borrow()).call(2_usize, 3_usize) });
                }
                elem!(table, 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!(table, 2).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 3).write({
                    brunsli_HuffmanCode {
                        bits: 3_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                elem!(table, 4).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 5).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!(table, 6).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!(table, 7).write({
                    brunsli_HuffmanCode {
                        bits: 3_u8,
                        value: (*symbols.borrow())[(3) as usize],
                    }
                });
                table_size = 8_usize;
                break 'switch;
            }
            _ => {
                return false;
            }
        }
    };
    let mut goal_size: u32 = (1_u32 << kHuffmanTableBits_21.with(|rc| *rc.borrow()));
    'loop_: while (table_size != (goal_size as usize)) {
        {
            ((table.offset((table_size) as isize)) as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &((table.offset((0) as isize)) as Ptr<brunsli_HuffmanCode>).to_any(),
                    ((table_size as u64).wrapping_mul((4usize as u64)) as usize) as usize,
                );
            ((table.offset((table_size) as isize)) as Ptr<brunsli_HuffmanCode>).to_any()
        };
        table_size <<= 1;
    }
    return ({ BrunsliBitReaderIsHealthy_132(br) });
}
pub fn GetNextKey_221(mut key: i32, mut len: usize) -> i32 {
    let mut step: i32 = ((1_u32 << ((len).wrapping_sub(1_usize))) as i32);
    'loop_: while ((key & step) != 0) {
        step >>= 1;
    }
    return ((key & (step - 1)) + step);
}
pub fn ReplicateValue_222(
    mut table: Ptr<brunsli_HuffmanCode>,
    mut step: i32,
    mut end: i32,
    mut code: brunsli_HuffmanCode,
) {
    let mut __do_while = true;
    'loop_: while __do_while || (end > 0) {
        __do_while = false;
        end -= step;
        elem!(table, end).write({ (code).clone() });
    }
}
pub fn NextTableBitSize_223(mut count: Ptr<u16>, mut len: usize, mut root_bits: usize) -> usize {
    let mut left: usize = (1_usize << ((len).wrapping_sub(root_bits)));
    'loop_: while (len < kMaxHuffmanBits_22.with(|rc| *rc.borrow())) {
        if ({ left } <= { ((elem!(count, len).read()) as usize) }) {
            break;
        }
        left = { (left).wrapping_sub(((elem!(count, len).read()) as usize)) };
        len.prefix_inc();
        left <<= 1;
    }
    return (len).wrapping_sub(root_bits);
}
pub fn BuildHuffmanTable_218(
    mut root_table: Ptr<brunsli_HuffmanCode>,
    mut root_bits: usize,
    mut code_lengths: Ptr<u8>,
    mut code_lengths_size: usize,
    mut count: Ptr<u16>,
) -> u32 {
    let mut code: brunsli_HuffmanCode = <brunsli_HuffmanCode>::default();
    let mut table: Ptr<brunsli_HuffmanCode> = Ptr::<brunsli_HuffmanCode>::null();
    let mut len: usize = 0_usize;
    let mut symbol: usize = 0_usize;
    let mut key: i32 = 0_i32;
    let mut step: i32 = 0_i32;
    let mut low: i32 = 0_i32;
    let mut mask: i32 = 0_i32;
    let mut table_bits: usize = 0_usize;
    let mut table_size: i32 = 0_i32;
    let mut total_size: i32 = 0_i32;
    let mut offset: [u16; 16] = [0_u16; 16];
    let mut max_length: usize = 1_usize;
    if (code_lengths_size > ((1_u32 << kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize)) {
        return 0_u32;
    }
    let sorted_storage: Value<Vec<u16>> = Rc::new(RefCell::new(
        (0..(code_lengths_size) as usize)
            .map(|_| <u16>::default())
            .collect::<Vec<_>>(),
    ));
    let mut sorted: Ptr<u16> = (sorted_storage.as_pointer() as Ptr<u16>);
    {
        let mut sum: u16 = 0_u16;
        len = 1_usize;
        'loop_: while (len <= kMaxHuffmanBits_22.with(|rc| *rc.borrow())) {
            offset[(len) as usize] = sum;
            if ((elem!(count, len).read()) != 0) {
                sum = { (({ (sum as i32) } + { ((elem!(count, len).read()) as i32) }) as u16) };
                max_length = len;
            }
            len.postfix_inc();
        }
    }
    symbol = 0_usize;
    'loop_: while (symbol < code_lengths_size) {
        if (((elem!(code_lengths, symbol).read()) as i32) != 0) {
            let __rhs = (symbol as u16);
            elem!(
                sorted,
                offset[(elem!(code_lengths, symbol).read()) as usize].postfix_inc()
            )
            .write(__rhs);
        }
        symbol.postfix_inc();
    }
    table = (root_table).clone();
    table_bits = root_bits;
    table_size = ((1_u32 << table_bits) as i32);
    total_size = table_size;
    if ((offset[(kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize] as i32) == 1) {
        code.bits = 0_u8;
        code.value = { (elem!(sorted, 0).read()) };
        key = 0;
        'loop_: while (key < total_size) {
            elem!(table, key).write({ (code).clone() });
            key.prefix_inc();
        }
        return (total_size as u32);
    }
    if (table_bits > max_length) {
        table_bits = max_length;
        table_size = ((1_u32 << table_bits) as i32);
    }
    key = 0;
    symbol = 0_usize;
    code.bits = 1_u8;
    step = 2;
    let mut __do_while = true;
    'loop_: while __do_while || ((code.bits.prefix_inc() as usize) <= table_bits) {
        __do_while = false;
        'loop_: while (((elem!(count, code.bits).read()) as i32) != 0) {
            let __rhs = (elem!(sorted, symbol.postfix_inc()).read());
            code.value = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> = (table.offset((key) as isize));
                let _code: brunsli_HuffmanCode = (code).clone();
                ReplicateValue_222(_table, step, table_size, _code)
            });
            let __rhs = ({ GetNextKey_221(key, (code.bits as usize)) });
            key = __rhs;
            elem!(count, code.bits).with_mut(|__v| __v.prefix_dec());
        }
        step <<= 1;
    }
    'loop_: while (total_size != table_size) {
        {
            ((table.offset((table_size) as isize)) as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &((table.offset((0) as isize)) as Ptr<brunsli_HuffmanCode>).to_any(),
                    (table_size as usize).wrapping_mul((4usize as usize)) as usize,
                );
            ((table.offset((table_size) as isize)) as Ptr<brunsli_HuffmanCode>).to_any()
        };
        table_size <<= 1;
    }
    mask = (total_size - 1);
    low = -1_i32;
    {
        len = (root_bits).wrapping_add(1_usize);
        step = 2
    };
    'loop_: while (len <= max_length) {
        'loop_: while (((elem!(count, len).read()) as i32) != 0) {
            if ((key & mask) != low) {
                table += table_size;
                table_bits = ({ NextTableBitSize_223((count).clone(), len, root_bits) });
                table_size = ((1_u32 << table_bits) as i32);
                total_size += table_size;
                low = (key & mask);
                field!(elem!(root_table, low), bits)
                    .write((((table_bits).wrapping_add(root_bits)) as u8));
                field!(elem!(root_table, low), value).write({
                    (({ (((table).clone() - (root_table).clone()) as i64) } - { (low as i64) })
                        as u16)
                });
            }
            code.bits = (((len).wrapping_sub(root_bits)) as u8);
            let __rhs = (elem!(sorted, symbol.postfix_inc()).read());
            code.value = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> = (table.offset((key >> root_bits) as isize));
                let _code: brunsli_HuffmanCode = (code).clone();
                ReplicateValue_222(_table, step, table_size, _code)
            });
            let __rhs = ({ GetNextKey_221(key, len) });
            key = __rhs;
            elem!(count, len).with_mut(|__v| __v.prefix_dec());
        }
        {
            len.prefix_inc();
            step <<= 1
        };
    }
    return (total_size as u32);
}
impl brunsli_internal_dec_OutputChunk {
    pub fn new_4(bytes: Ptr<Vec<u8>>) -> Self {
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: Ptr::<u8>::null(),
            len: (*bytes.upgrade().deref()).len(),
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        let mut src: AnyPtr = ((Ptr::<Vec<u8>>::decay(&(bytes)) as Ptr<u8>) as Ptr<u8>).to_any();
        field!(this, next).write(src.reinterpret_cast::<u8>());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
thread_local!(
    pub static kJpegPrecision_224: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kBitWriterChunkSize_225: Value<usize> = Rc::new(RefCell::new(16384_usize));
);
pub fn DivCeil_226(mut a: i32, mut b: i32) -> i32 {
    return (((a + b) - 1) / b);
}
pub fn HasZeroByte_227(mut x: u64) -> u64 {
    return ((((x).wrapping_sub((72340172838076673_u64 as u64))) & !x) & 9259542123273814144_u64);
}
pub fn BitWriterInit_228(
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
    mut output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
) {
    field!(bw, output).write((output_queue).clone());
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> =
            Rc::new(RefCell::new(brunsli_internal_dec_OutputChunk::new_2({
                Some(kBitWriterChunkSize_225.with(|rc| *rc.borrow()))
            })));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!(bw, chunk),
            _arg0.as_pointer(),
        )
    });
    field!(bw, pos).write(0_usize);
    field!(bw, put_buffer).write(0_u64);
    field!(bw, put_bits).write(64);
    field!(bw, healthy).write(true);
    let __rhs =
        (Ptr::<Vec<u8>>::decay(&(bw.with(|__s| __s.chunk.buffer.clone()).as_pointer())) as Ptr<u8>);
    field!(bw, data).write(__rhs);
}
pub fn SwapBuffer_229(mut bw: Ptr<brunsli_internal_dec_BitWriter>) {
    field!(field!(bw, chunk), len).write({ bw.with(|__s| __s.pos) });
    {
        let __init = brunsli_internal_dec_OutputChunk::move_from({ field_ptr!(bw, chunk) });
        bw.with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.push(__init))
    };
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> =
            Rc::new(RefCell::new(brunsli_internal_dec_OutputChunk::new_2({
                Some(kBitWriterChunkSize_225.with(|rc| *rc.borrow()))
            })));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!(bw, chunk),
            _arg0.as_pointer(),
        )
    });
    let __rhs =
        (Ptr::<Vec<u8>>::decay(&(bw.with(|__s| __s.chunk.buffer.clone()).as_pointer())) as Ptr<u8>);
    field!(bw, data).write(__rhs);
    field!(bw, pos).write(0_usize);
}
pub fn Reserve_230(mut bw: Ptr<brunsli_internal_dec_BitWriter>, mut n_bytes: usize) {
    if ((({ ((bw.with(|__s| __s.pos)).wrapping_add(n_bytes)) } > {
        kBitWriterChunkSize_225.with(|rc| *rc.borrow())
    }) as i64)
        != 0)
    {
        ({ SwapBuffer_229((bw).clone()) });
    }
}
pub fn EmitByte_231(mut bw: Ptr<brunsli_internal_dec_BitWriter>, mut byte: i32) {
    let __rhs = (byte as u8);
    elem!(
        bw.with(|__s| __s.data.clone()),
        field!(bw, pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    if (byte == 255) {
        elem!(
            bw.with(|__s| __s.data.clone()),
            field!(bw, pos).with_mut(|__v| __v.postfix_inc())
        )
        .write(0_u8);
    }
}
pub fn DischargeBitBuffer_232(mut bw: Ptr<brunsli_internal_dec_BitWriter>) {
    ({ Reserve_230((bw).clone(), 12_usize) });
    if (({ HasZeroByte_227((!bw.with(|__s| __s.put_buffer) | 65535_u64)) }) != 0) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 56) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 48) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 40) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 32) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 24) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
            let _byte: i32 = (((bw.with(|__s| __s.put_buffer) >> 16) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
    } else {
        elem!(bw.with(|__s| __s.data.clone()), bw.with(|__s| __s.pos))
            .write({ (((bw.with(|__s| __s.put_buffer) >> 56) & 255_u64) as u8) });
        elem!(
            bw.with(|__s| __s.data.clone()),
            (bw.with(|__s| __s.pos)).wrapping_add(1_usize)
        )
        .write({ (((bw.with(|__s| __s.put_buffer) >> 48) & 255_u64) as u8) });
        elem!(
            bw.with(|__s| __s.data.clone()),
            (bw.with(|__s| __s.pos)).wrapping_add(2_usize)
        )
        .write({ (((bw.with(|__s| __s.put_buffer) >> 40) & 255_u64) as u8) });
        elem!(
            bw.with(|__s| __s.data.clone()),
            (bw.with(|__s| __s.pos)).wrapping_add(3_usize)
        )
        .write({ (((bw.with(|__s| __s.put_buffer) >> 32) & 255_u64) as u8) });
        elem!(
            bw.with(|__s| __s.data.clone()),
            (bw.with(|__s| __s.pos)).wrapping_add(4_usize)
        )
        .write({ (((bw.with(|__s| __s.put_buffer) >> 24) & 255_u64) as u8) });
        elem!(
            bw.with(|__s| __s.data.clone()),
            (bw.with(|__s| __s.pos)).wrapping_add(5_usize)
        )
        .write({ (((bw.with(|__s| __s.put_buffer) >> 16) & 255_u64) as u8) });
        field!(bw, pos).write({ (bw.with(|__s| __s.pos)).wrapping_add(6_usize) });
    }
    {
        field!(bw, put_buffer).with_mut(|__v| *__v = *__v << 48)
    };
    {
        field!(bw, put_bits).with_mut(|__v| *__v = *__v + 48)
    };
}
pub fn WriteBits_233(mut bw: Ptr<brunsli_internal_dec_BitWriter>, mut nbits: i32, mut bits: u64) {
    if (nbits == 0) {
        field!(bw, healthy).write(false);
        return;
    }
    {
        let __rhs = nbits;
        field!(bw, put_bits).with_mut(|__v| *__v = *__v - __rhs)
    };
    {
        let __rhs = { ({ bits } << { bw.with(|__s| __s.put_bits) }) };
        field!(bw, put_buffer).with_mut(|__v| *__v = *__v | __rhs)
    };
    if (bw.with(|__s| __s.put_bits) <= 16) {
        ({ DischargeBitBuffer_232((bw).clone()) });
    }
}
pub fn EmitMarker_234(mut bw: Ptr<brunsli_internal_dec_BitWriter>, mut marker: i32) {
    ({ Reserve_230((bw).clone(), 2_usize) });
    if !(marker != 255) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_writer.cc"),
                133,
                Ptr::<i8>::from_string_literal(b"EmitMarker"),
            )
        });
        'loop_: while true {}
    };
    elem!(
        bw.with(|__s| __s.data.clone()),
        field!(bw, pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(255_u8);
    let __rhs = (marker as u8);
    elem!(
        bw.with(|__s| __s.data.clone()),
        field!(bw, pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
}
pub fn JumpToByteBoundary_235(
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
    mut pad_bits: Ptr<Ptr<i32>>,
    mut pad_bits_end: Ptr<i32>,
) -> bool {
    let mut n_bits: usize = (((bw.with(|__s| __s.put_bits) as u32) & 7_u32) as usize);
    let mut pad_pattern: u8 = 0_u8;
    if (pad_bits.read()).is_null() {
        pad_pattern = (((1_u32 << n_bits).wrapping_sub(1_u32)) as u8);
    } else {
        pad_pattern = 0_u8;
        let mut src: Ptr<i32> = (pad_bits.read());
        'loop_: while (n_bits.postfix_dec() != 0) {
            pad_pattern = { ((pad_pattern as i32) << 1) as u8 };
            if ({ (src).clone() } >= { (pad_bits_end).clone() }) {
                return false;
            }
            {
                let rhs_0 =
                    ((pad_pattern as i32) | (!(!(((src.postfix_inc()).read()) != 0)) as i32)) as u8;
                pad_pattern = rhs_0
            };
        }
        pad_bits.write({ (src).clone() });
    }
    ({ Reserve_230((bw).clone(), 16_usize) });
    'loop_: while (bw.with(|__s| __s.put_bits) <= 56) {
        let mut c: i32 = (((bw.with(|__s| __s.put_buffer) >> 56) & 255_u64) as i32);
        ({ EmitByte_231((bw).clone(), c) });
        {
            field!(bw, put_buffer).with_mut(|__v| *__v = *__v << 8)
        };
        {
            field!(bw, put_bits).with_mut(|__v| *__v = *__v + 8)
        };
    }
    if (bw.with(|__s| __s.put_bits) < 64) {
        let mut pad_mask: i32 = ((255_u32 >> (64 - bw.with(|__s| __s.put_bits))) as i32);
        let mut c: i32 = (({ ({ (bw.with(|__s| __s.put_buffer) >> 56) } & { (!pad_mask as u64) }) }
            | { (pad_pattern as u64) }) as i32);
        ({ EmitByte_231((bw).clone(), c) });
    }
    field!(bw, put_buffer).write(0_u64);
    field!(bw, put_bits).write(64);
    return true;
}
pub fn BitWriterFinish_236(mut bw: Ptr<brunsli_internal_dec_BitWriter>) {
    if (bw.with(|__s| __s.pos) == 0_usize) {
        return;
    }
    field!(field!(bw, chunk), len).write({ bw.with(|__s| __s.pos) });
    {
        let __init = brunsli_internal_dec_OutputChunk::move_from({ field_ptr!(bw, chunk) });
        bw.with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.push(__init))
    };
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(
            brunsli_internal_dec_OutputChunk::new_1({ Ptr::<u8>::null() }, { 0_usize }),
        ));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!(bw, chunk),
            _arg0.as_pointer(),
        )
    });
    field!(bw, data).write(Ptr::<u8>::null());
    field!(bw, pos).write(0_usize);
}
pub fn DCTCodingStateInit_237(mut s: Ptr<brunsli_internal_dec_DCTCodingState>) {
    field!(s, eob_run_).write(0);
    field!(s, cur_ac_huff_).write(Ptr::<brunsli_HuffmanCodeTable>::null());
    (*s.with(|__s| __s.refinement_bits_.clone()).borrow_mut()).clear();
    if 64_usize as usize
        > (*s.with(|__s| __s.refinement_bits_.clone()).borrow()).capacity() as usize
    {
        let len_0 = (*s.with(|__s| __s.refinement_bits_.clone()).borrow()).len();
        (*s.with(|__s| __s.refinement_bits_.clone()).borrow_mut())
            .reserve_exact(64_usize as usize - len_0 as usize);
    };
    field!(s, refinement_bits_count_).write(0_usize);
}
pub fn Flush_238(
    mut s: Ptr<brunsli_internal_dec_DCTCodingState>,
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
) {
    if (s.with(|__s| __s.eob_run_) > 0) {
        let mut nbits: i32 = ({ Log2FloorNonZero_74((s.with(|__s| __s.eob_run_) as u32)) });
        let mut symbol: i32 = (nbits << 4_u32);
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!(s.with(|__s| __s.cur_ac_huff_.clone()), depth) as Ptr::<i32>),
                symbol
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!(s.with(|__s| __s.cur_ac_huff_.clone()), code) as Ptr::<i32>),
                symbol
            )
            .read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        if (nbits > 0) {
            ({
                let _nbits: i32 = nbits;
                let _bits: u64 = (({ s.with(|__s| __s.eob_run_) } & { ((1 << nbits) - 1) }) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
        }
        field!(s, eob_run_).write(0);
    }
    let mut num_words: usize = (s.with(|__s| __s.refinement_bits_count_) >> 4);
    let mut i: usize = 0_usize;
    'loop_: while (i < num_words) {
        ({
            WriteBits_233(
                (bw).clone(),
                16,
                ((elem!(
                    (s.with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>),
                    i
                )
                .read()) as u64),
            )
        });
        i.prefix_inc();
    }
    let mut tail: usize = (s.with(|__s| __s.refinement_bits_count_) & 15_usize);
    if (tail != 0) {
        ({
            WriteBits_233(
                (bw).clone(),
                (tail as i32),
                (((s.with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                    .to_last()
                    .read()) as u64),
            )
        });
    }
    (*s.with(|__s| __s.refinement_bits_.clone()).borrow_mut()).clear();
    field!(s, refinement_bits_count_).write(0_usize);
}
pub fn BufferEndOfBand_239(
    mut s: Ptr<brunsli_internal_dec_DCTCodingState>,
    mut ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    mut new_bits_array: Ptr<i32>,
    new_bits_count: usize,
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let new_bits_count: Value<usize> = Rc::new(RefCell::new(new_bits_count));
    if (s.with(|__s| __s.eob_run_) == 0) {
        field!(s, cur_ac_huff_).write((ac_huff).clone());
    }
    field!(s, eob_run_).with_mut(|__v| __v.prefix_inc());
    if ((*new_bits_count.borrow()) != 0) {
        let mut new_bits: u64 = 0_u64;
        let mut i: usize = 0_usize;
        'loop_: while (i < (*new_bits_count.borrow())) {
            new_bits = { ({ (new_bits << 1) } | { ((elem!(new_bits_array, i).read()) as u64) }) };
            i.prefix_inc();
        }
        let mut tail: usize = (s.with(|__s| __s.refinement_bits_count_) & 15_usize);
        if (tail != 0) {
            let mut stuff_bits_count: usize = ({
                let __tmp_0: Value<u64> =
                    Rc::new(RefCell::new(((16_usize).wrapping_sub(tail) as u64)));
                let __tmp_1: Value<u64> =
                    Rc::new(RefCell::new(((*new_bits_count.borrow()) as u64)));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            } as usize);
            let mut stuff_bits: u16 =
                ((new_bits >> ((*new_bits_count.borrow()).wrapping_sub(stuff_bits_count))) as u16);
            stuff_bits = {
                ((stuff_bits as u32) & ((1_u32 << stuff_bits_count).wrapping_sub(1_u32))) as u16
            };
            let __rhs = (({
                ({
                    (((s.with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                        .to_last()
                        .read()) as i32)
                } << { stuff_bits_count })
            } | { (stuff_bits as i32) }) as u16);
            (s.with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                .to_last()
                .write(__rhs);
            (*new_bits_count.borrow_mut()) =
                { (*new_bits_count.borrow()).wrapping_sub(stuff_bits_count) };
            field!(s, refinement_bits_count_).write({
                (s.with(|__s| __s.refinement_bits_count_)).wrapping_add(stuff_bits_count)
            });
        }
        'loop_: while ((*new_bits_count.borrow()) >= 16_usize) {
            {
                let __a1 =
                    ((new_bits >> ((*new_bits_count.borrow()).wrapping_sub(16_usize))) as u16);
                (*s.with(|__s| __s.refinement_bits_.clone()).borrow_mut()).push(__a1)
            };
            (*new_bits_count.borrow_mut()) = { (*new_bits_count.borrow()).wrapping_sub(16_usize) };
            field!(s, refinement_bits_count_)
                .write({ (s.with(|__s| __s.refinement_bits_count_)).wrapping_add(16_usize) });
        }
        if ((*new_bits_count.borrow()) != 0) {
            {
                let __a1 = ((new_bits
                    & (((1_u32 << (*new_bits_count.borrow())).wrapping_sub(1_u32)) as u64))
                    as u16);
                (*s.with(|__s| __s.refinement_bits_.clone()).borrow_mut()).push(__a1)
            };
            field!(s, refinement_bits_count_).write({
                (s.with(|__s| __s.refinement_bits_count_)).wrapping_add((*new_bits_count.borrow()))
            });
        }
    }
    if ({ s.with(|__s| __s.refinement_bits_count_) } > {
        ((32767 * (kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1)) as usize)
    }) {
        return false;
    }
    if (s.with(|__s| __s.eob_run_) == 32767) {
        ({ Flush_238((s).clone(), (bw).clone()) });
    }
    return true;
}
pub fn BuildHuffmanCodeTable_240(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    mut table: Ptr<brunsli_HuffmanCodeTable>,
) -> bool {
    let mut huff_code: [i32; 256] = [0_i32; 256];
    let mut huff_size: [u32; 257] = [0_u32; 257];
    let mut p: i32 = 0;
    let mut l: usize = 1_usize;
    'loop_: while (l <= (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize)) {
        let mut i: i32 = (elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), l).read());
        if ((p + i) > (kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()) + 1)) {
            return false;
        }
        'loop_: while (i.postfix_dec() != 0) {
            huff_size[(p.postfix_inc()) as usize] = (l as u32);
        }
        l.prefix_inc();
    }
    if (p == 0) {
        return true;
    }
    let mut last_p: i32 = (p - 1);
    huff_size[(last_p) as usize] = 0_u32;
    let mut code: i32 = 0;
    let mut si: u32 = huff_size[(0) as usize];
    p = 0;
    'loop_: while (huff_size[(p) as usize] != 0) {
        'loop_: while ((huff_size[(p) as usize]) == si) {
            huff_code[(p.postfix_inc()) as usize] = code;
            code.postfix_inc();
        }
        code <<= 1;
        si.postfix_inc();
    }
    p = 0;
    'loop_: while (p < last_p) {
        let mut i: i32 = (elem!(
            (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
            (p as usize)
        )
        .read());
        elem!((array_field_ptr!(table, depth) as Ptr::<i32>), i)
            .write((huff_size[(p) as usize] as i32));
        elem!((array_field_ptr!(table, code) as Ptr::<i32>), i).write(huff_code[(p) as usize]);
        p.postfix_inc();
    }
    return true;
}
pub fn EncodeSOI_241(mut state: Ptr<brunsli_internal_dec_SerializationState>) -> bool {
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(
        brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8, 216_u8] }),
    );
    return true;
}
pub fn EncodeEOI_242(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(
        brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8, 217_u8] }),
    );
    {
        let __init =
            brunsli_internal_dec_OutputChunk::new_4({ jpg.with(|__s| __s.tail_data.as_pointer()) });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    return true;
}
pub fn EncodeSOF_243(
    jpg: Ptr<brunsli_JPEGData>,
    mut marker: u8,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    if ((marker as i32) <= 194) {
        field!(state, is_progressive).write(((marker as i32) == 194));
    }
    let mut n_comps: usize = (*jpg.with(|__s| __s.components.clone()).borrow()).len();
    let mut marker_len: usize = (8_usize).wrapping_add((3_usize).wrapping_mul(n_comps));
    {
        let __init =
            brunsli_internal_dec_OutputChunk::new_2({ Some((marker_len).wrapping_add(2_usize)) });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    let mut data: Ptr<u8> = (Ptr::<Vec<u8>>::decay(
        &((state.with(|__s| __s.output_queue.as_pointer())
            as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .with(|__s| __s.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    let mut pos: usize = 0_usize;
    elem!(data, pos.postfix_inc()).write(255_u8);
    let __rhs = marker;
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = ((marker_len >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (marker_len as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (kJpegPrecision_224.with(|rc| *rc.borrow()) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = ((jpg.with(|__s| __s.height) >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (((jpg.with(|__s| __s.height) as u32) & 255_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = ((jpg.with(|__s| __s.width) >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (((jpg.with(|__s| __s.width) as u32) & 255_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (n_comps as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let mut i: usize = 0_usize;
    'loop_: while (i < n_comps) {
        let __rhs = ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            )
            .upgrade()
            .deref())
            .id
        } as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        let __rhs = (({
            ({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    i
                )
                .upgrade()
                .deref())
                .h_samp_factor
            } << 4_u32)
        } | {
            ({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    i
                )
                .upgrade()
                .deref())
                .v_samp_factor
            })
        }) as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        let mut quant_idx: usize = ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                i
            )
            .upgrade()
            .deref())
            .quant_idx
        } as usize);
        if ({ quant_idx } >= { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            return false;
        }
        let __rhs = ({
            (*elem!(
                (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                quant_idx
            )
            .upgrade()
            .deref())
            .index
        } as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        i.prefix_inc();
    }
    return true;
}
pub fn EncodeSOS_244(
    jpg: Ptr<brunsli_JPEGData>,
    scan_info: Ptr<brunsli_JPEGScanInfo>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let mut n_scans: usize = scan_info.with(|__s| __s.num_components);
    let mut marker_len: usize = (6_usize).wrapping_add((2_usize).wrapping_mul(n_scans));
    {
        let __init =
            brunsli_internal_dec_OutputChunk::new_2({ Some((marker_len).wrapping_add(2_usize)) });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    let mut data: Ptr<u8> = (Ptr::<Vec<u8>>::decay(
        &((state.with(|__s| __s.output_queue.as_pointer())
            as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .with(|__s| __s.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    let mut pos: usize = 0_usize;
    elem!(data, pos.postfix_inc()).write(255_u8);
    elem!(data, pos.postfix_inc()).write(218_u8);
    let __rhs = ((marker_len >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (marker_len as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (n_scans as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let mut i: usize = 0_usize;
    'loop_: while (i < n_scans) {
        let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
            .with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset(i);
        if ({ (si.with(|__s| __s.comp_idx) as usize) } >= {
            (*jpg.with(|__s| __s.components.clone()).borrow()).len()
        }) {
            return false;
        }
        let __rhs = ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                (si.with(|__s| __s.comp_idx) as usize)
            )
            .upgrade()
            .deref())
            .id
        } as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        let __rhs =
            (({ (si.with(|__s| __s.dc_tbl_idx) << 4_u32) } + { si.with(|__s| __s.ac_tbl_idx) })
                as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        i.prefix_inc();
    }
    let __rhs = (scan_info.with(|__s| __s.Ss) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (scan_info.with(|__s| __s.Se) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs =
        (({ (scan_info.with(|__s| __s.Ah) << 4_u32) } | { (scan_info.with(|__s| __s.Al)) }) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    return true;
}
pub fn EncodeDHT_245(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let huffman_code: Ptr<Vec<brunsli_JPEGHuffmanCode>> =
        jpg.with(|__s| __s.huffman_code.as_pointer());
    let mut marker_len: usize = 2_usize;
    let mut i: usize = (state.with(|__s| __s.dht_index) as usize);
    'loop_: while ({ i } < { (*huffman_code.upgrade().deref()).len() }) {
        let huff: Ptr<brunsli_JPEGHuffmanCode> =
            (Ptr::<Vec<brunsli_JPEGHuffmanCode>>::decay(&(huffman_code))
                as Ptr<brunsli_JPEGHuffmanCode>)
                .offset(i);
        marker_len = {
            (marker_len).wrapping_add((kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize))
        };
        let mut j: usize = 0_usize;
        'loop_: while ({ j } < { (*huff.with(|__s| __s.counts.clone()).borrow()).len() }) {
            {
                let rhs_0 = (marker_len).wrapping_add(
                    ((elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), j).read())
                        as usize),
                );
                marker_len = rhs_0
            };
            j.prefix_inc();
        }
        if huff.with(|__s| __s.is_last) {
            break;
        }
        i.prefix_inc();
    }
    {
        let __init =
            brunsli_internal_dec_OutputChunk::new_2({ Some((marker_len).wrapping_add(2_usize)) });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    let mut data: Ptr<u8> = (Ptr::<Vec<u8>>::decay(
        &((state.with(|__s| __s.output_queue.as_pointer())
            as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .with(|__s| __s.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    let mut pos: usize = 0_usize;
    elem!(data, pos.postfix_inc()).write(255_u8);
    elem!(data, pos.postfix_inc()).write(196_u8);
    let __rhs = ((marker_len >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (marker_len as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    'loop_: while true {
        let mut huffman_code_index: usize =
            (field!(state, dht_index).with_mut(|__v| __v.postfix_inc()) as usize);
        if ({ huffman_code_index } >= { (*huffman_code.upgrade().deref()).len() }) {
            return false;
        }
        let huff: Ptr<brunsli_JPEGHuffmanCode> =
            (Ptr::<Vec<brunsli_JPEGHuffmanCode>>::decay(&(huffman_code))
                as Ptr<brunsli_JPEGHuffmanCode>)
                .offset(huffman_code_index);
        let mut index: usize = (huff.with(|__s| __s.slot_id) as usize);
        let mut huff_table: Ptr<brunsli_HuffmanCodeTable> = Ptr::<brunsli_HuffmanCodeTable>::null();
        if ((index & 16_usize) != 0) {
            index = { (index).wrapping_sub(16_usize) };
            huff_table = ((state.with(|__s| __s.ac_huff_table.as_pointer())
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset(index));
        } else {
            huff_table = ((state.with(|__s| __s.dc_huff_table.as_pointer())
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset(index));
        }
        if !({
            let _huff: Ptr<brunsli_JPEGHuffmanCode> = (huff).clone();
            let _table: Ptr<brunsli_HuffmanCodeTable> = (huff_table).clone();
            BuildHuffmanCodeTable_240(_huff, _table)
        }) {
            return false;
        }
        let mut total_count: usize = 0_usize;
        let mut max_length: usize = 0_usize;
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*huff.with(|__s| __s.counts.clone()).borrow()).len() }) {
            if ((elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), i).read()) != 0) {
                max_length = i;
            }
            {
                let rhs_0 = (total_count).wrapping_add(
                    ((elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), i).read())
                        as usize),
                );
                total_count = rhs_0
            };
            i.prefix_inc();
        }
        total_count.prefix_dec();
        let __rhs = (huff.with(|__s| __s.slot_id) as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        let mut i: usize = 1_usize;
        'loop_: while (i <= (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize)) {
            let __rhs = ((if (i == max_length) {
                ((elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), i).read()) - 1)
            } else {
                (elem!((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>), i).read())
            }) as u8);
            elem!(data, pos.postfix_inc()).write(__rhs);
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while (i < total_count) {
            let __rhs =
                ((elem!((huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>), i).read()) as u8);
            elem!(data, pos.postfix_inc()).write(__rhs);
            i.prefix_inc();
        }
        if huff.with(|__s| __s.is_last) {
            break;
        }
    }
    return true;
}
pub fn EncodeDQT_246(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let mut marker_len: i32 = 2;
    let mut i: usize = (state.with(|__s| __s.dqt_index) as usize);
    'loop_: while ({ i } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        let table: Ptr<brunsli_JPEGQuantTable> =
            (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>).offset(i);
        marker_len += (1
            + ({
                (if (table.with(|__s| __s.precision) != 0) {
                    2
                } else {
                    1
                })
            } * { kDCTBlockSize_3.with(|rc| *rc.borrow()) }));
        if table.with(|__s| __s.is_last) {
            break;
        }
        i.prefix_inc();
    }
    {
        let __init = brunsli_internal_dec_OutputChunk::new_2({ Some(((marker_len + 2) as usize)) });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    let mut data: Ptr<u8> = (Ptr::<Vec<u8>>::decay(
        &((state.with(|__s| __s.output_queue.as_pointer())
            as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .with(|__s| __s.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    let mut pos: usize = 0_usize;
    elem!(data, pos.postfix_inc()).write(255_u8);
    elem!(data, pos.postfix_inc()).write(219_u8);
    let __rhs = ((marker_len >> 8_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    let __rhs = (((marker_len as u32) & 255_u32) as u8);
    elem!(data, pos.postfix_inc()).write(__rhs);
    'loop_: while true {
        let mut idx: usize = (field!(state, dqt_index).with_mut(|__v| __v.postfix_inc()) as usize);
        if ({ idx } >= { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            return false;
        }
        let table: Ptr<brunsli_JPEGQuantTable> =
            (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>).offset(idx);
        let __rhs =
            (({ (table.with(|__s| __s.precision) << 4_u32) } + { table.with(|__s| __s.index) })
                as u8);
        elem!(data, pos.postfix_inc()).write(__rhs);
        let mut i: usize = 0_usize;
        'loop_: while (i < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
            let mut val_idx: i32 = (({
                let __idx = (i) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            }) as i32);
            let mut val: i32 = (elem!(
                (table.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                (val_idx as usize)
            )
            .read());
            if (table.with(|__s| __s.precision) != 0) {
                let __rhs = ((val >> 8_u32) as u8);
                elem!(data, pos.postfix_inc()).write(__rhs);
            }
            let __rhs = (((val as u32) & 255_u32) as u8);
            elem!(data, pos.postfix_inc()).write(__rhs);
            i.prefix_inc();
        }
        if table.with(|__s| __s.is_last) {
            break;
        }
    }
    return true;
}
pub fn EncodeDRI_247(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    field!(state, seen_dri_marker).write(true);
    let dri_marker: Value<brunsli_internal_dec_OutputChunk> =
        Rc::new(RefCell::new(brunsli_internal_dec_OutputChunk::new_3({
            vec![
                255_u8,
                221_u8,
                0_u8,
                4_u8,
                ((jpg.with(|__s| __s.restart_interval) >> 8) as u8),
                ((jpg.with(|__s| __s.restart_interval) & 255) as u8),
            ]
        })));
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(
        brunsli_internal_dec_OutputChunk::move_from({ dri_marker.as_pointer() }),
    );
    return true;
}
pub fn EncodeRestart_248(
    mut marker: u8,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(
        brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8, marker] }),
    );
    return true;
}
pub fn EncodeAPP_249(
    jpg: Ptr<brunsli_JPEGData>,
    mut marker: u8,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    &(marker);
    let mut app_index: usize =
        (field!(state, app_index).with_mut(|__v| __v.postfix_inc()) as usize);
    if ({ app_index } >= { (*jpg.with(|__s| __s.app_data.clone()).borrow()).len() }) {
        return false;
    }
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut())
        .push(brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8] }));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset(app_index)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    return true;
}
pub fn EncodeCOM_250(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let mut com_index: usize =
        (field!(state, com_index).with_mut(|__v| __v.postfix_inc()) as usize);
    if ({ com_index } >= { (*jpg.with(|__s| __s.com_data.clone()).borrow()).len() }) {
        return false;
    }
    (*state.with(|__s| __s.output_queue.clone()).borrow_mut())
        .push(brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8] }));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset(com_index)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    return true;
}
pub fn EncodeInterMarkerData_251(
    jpg: Ptr<brunsli_JPEGData>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let mut index: usize = (field!(state, data_index).with_mut(|__v| __v.postfix_inc()) as usize);
    if ({ index } >= { (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len() }) {
        return false;
    }
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.inter_marker_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset(index)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*state.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
    };
    return true;
}
pub fn EncodeDCTBlockSequential_252(
    mut coeffs: Ptr<i16>,
    dc_huff: Ptr<brunsli_HuffmanCodeTable>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    mut num_zero_runs: i32,
    mut last_dc_coeff: Ptr<i16>,
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let mut temp2: i16 = 0_i16;
    let mut temp: i16 = 0_i16;
    temp2 = { (elem!(coeffs, 0).read()) };
    temp = { (({ (temp2 as i32) } - { ((last_dc_coeff.read()) as i32) }) as i16) };
    last_dc_coeff.write({ temp2 });
    temp2 = temp;
    if ((temp as i32) < 0) {
        temp = { (-(temp as i32) as i16) };
        temp2.postfix_dec();
    }
    let mut dc_nbits: i32 = if ((temp as i32) == 0) {
        0
    } else {
        (({ Log2FloorNonZero_74((temp as u32)) }) + 1)
    };
    ({
        let _nbits: i32 =
            (elem!((array_field_ptr!(dc_huff, depth) as Ptr::<i32>), dc_nbits).read());
        let _bits: u64 =
            ((elem!((array_field_ptr!(dc_huff, code) as Ptr::<i32>), dc_nbits).read()) as u64);
        WriteBits_233((bw).clone(), _nbits, _bits)
    });
    if (dc_nbits > 0) {
        ({
            let _nbits: i32 = dc_nbits;
            let _bits: u64 = (((temp2 as u32) & ((1_u32 << dc_nbits).wrapping_sub(1_u32))) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
    }
    let mut r: i32 = 0;
    let mut k: i32 = 1;
    'loop_: while (k < 64) {
        if ((({
            temp = {
                (elem!(
                    coeffs,
                    ({
                        let __idx = (k) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read())
            };
            temp
        }) as i32)
            == 0)
        {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        if ((temp as i32) < 0) {
            temp = { (-(temp as i32) as i16) };
            temp2 = (!(temp as i32) as i16);
        } else {
            temp2 = temp;
        }
        'loop_: while (r > 15) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
            r -= 16;
        }
        let mut ac_nbits: i32 = (({ Log2FloorNonZero_74((temp as u32)) }) + 1);
        let mut symbol: i32 = ((r << 4_u32) + ac_nbits);
        ({
            let _nbits: i32 =
                (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), symbol).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), symbol).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        ({
            let _nbits: i32 = ac_nbits;
            let _bits: u64 = (((temp2 as i32) & ((1 << ac_nbits) - 1)) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        r = 0;
        k.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < num_zero_runs) {
        ({
            let _nbits: i32 = (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        r -= 16;
        i.prefix_inc();
    }
    if (r > 0) {
        ({
            let _nbits: i32 = (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 0).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 0).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
    }
    return true;
}
pub fn EncodeDCTBlockProgressive_253(
    mut coeffs: Ptr<i16>,
    dc_huff: Ptr<brunsli_HuffmanCodeTable>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut num_zero_runs: i32,
    mut coding_state: Ptr<brunsli_internal_dec_DCTCodingState>,
    mut last_dc_coeff: Ptr<i16>,
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let mut eob_run_allowed: bool = (Ss > 0);
    let mut temp2: i16 = 0_i16;
    let mut temp: i16 = 0_i16;
    if (Ss == 0) {
        temp2 = { (({ ((elem!(coeffs, 0).read()) as i32) } >> { Al }) as i16) };
        temp = { (({ (temp2 as i32) } - { ((last_dc_coeff.read()) as i32) }) as i16) };
        last_dc_coeff.write({ temp2 });
        temp2 = temp;
        if ((temp as i32) < 0) {
            temp = { (-(temp as i32) as i16) };
            temp2.postfix_dec();
        }
        let mut nbits: i32 = if ((temp as i32) == 0) {
            0
        } else {
            (({ Log2FloorNonZero_74((temp as u32)) }) + 1)
        };
        ({
            let _nbits: i32 =
                (elem!((array_field_ptr!(dc_huff, depth) as Ptr::<i32>), nbits).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(dc_huff, code) as Ptr::<i32>), nbits).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        if (nbits > 0) {
            ({
                let _nbits: i32 = nbits;
                let _bits: u64 = (((temp2 as i32) & ((1 << nbits) - 1)) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
        }
        Ss.prefix_inc();
    }
    if (Ss > Se) {
        return true;
    }
    let mut r: i32 = 0;
    let mut k: i32 = Ss;
    'loop_: while (k <= Se) {
        if ((({
            temp = {
                (elem!(
                    coeffs,
                    ({
                        let __idx = (k) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read())
            };
            temp
        }) as i32)
            == 0)
        {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        if ((temp as i32) < 0) {
            temp = { (-(temp as i32) as i16) };
            temp = { ((temp as i32) >> Al) as i16 };
            temp2 = (!(temp as i32) as i16);
        } else {
            temp = { ((temp as i32) >> Al) as i16 };
            temp2 = temp;
        }
        if ((temp as i32) == 0) {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        ({ Flush_238((coding_state).clone(), (bw).clone()) });
        'loop_: while (r > 15) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
            r -= 16;
        }
        let mut nbits: i32 = (({ Log2FloorNonZero_74((temp as u32)) }) + 1);
        let mut symbol: i32 = ((r << 4_u32) + nbits);
        ({
            let _nbits: i32 =
                (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), symbol).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), symbol).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        ({
            let _nbits: i32 = nbits;
            let _bits: u64 = (((temp2 as i32) & ((1 << nbits) - 1)) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        r = 0;
        k.prefix_inc();
    }
    if (num_zero_runs > 0) {
        ({ Flush_238((coding_state).clone(), (bw).clone()) });
        let mut i: i32 = 0;
        'loop_: while (i < num_zero_runs) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
            r -= 16;
            i.prefix_inc();
        }
    }
    if (r > 0) {
        ({
            BufferEndOfBand_239(
                (coding_state).clone(),
                (ac_huff).clone(),
                Ptr::<i32>::null(),
                0_usize,
                (bw).clone(),
            )
        });
        if !(eob_run_allowed) {
            ({ Flush_238((coding_state).clone(), (bw).clone()) });
        }
    }
    return true;
}
pub fn EncodeRefinementBits_254(
    mut coeffs: Ptr<i16>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut coding_state: Ptr<brunsli_internal_dec_DCTCodingState>,
    mut bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let mut eob_run_allowed: bool = (Ss > 0);
    if (Ss == 0) {
        ({
            WriteBits_233(
                (bw).clone(),
                1,
                ((({ ((elem!(coeffs, 0).read()) as i32) } >> { Al }) & 1) as u64),
            )
        });
        Ss.prefix_inc();
    }
    if (Ss > Se) {
        return true;
    }
    let mut abs_values: [i32; 64] = [0_i32; 64];
    let mut eob: i32 = 0;
    let mut k: i32 = Ss;
    'loop_: while (k <= Se) {
        let mut abs_val: i16 = (((elem!(
            coeffs,
            ({
                let __idx = (k) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            .abs() as i16);
        abs_values[(k) as usize] = ((abs_val as i32) >> Al);
        if (abs_values[(k) as usize] == 1) {
            eob = k;
        }
        k.postfix_inc();
    }
    let mut r: i32 = 0;
    let refinement_bits: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut refinement_bits_count: usize = 0_usize;
    let mut k: i32 = Ss;
    'loop_: while (k <= Se) {
        if (abs_values[(k) as usize] == 0) {
            r.postfix_inc();
            k.postfix_inc();
            continue 'loop_;
        }
        'loop_: while (r > 15) && (k <= eob) {
            ({ Flush_238((coding_state).clone(), (bw).clone()) });
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((bw).clone(), _nbits, _bits)
            });
            r -= 16;
            let mut i: usize = 0_usize;
            'loop_: while (i < refinement_bits_count) {
                ({
                    WriteBits_233(
                        (bw).clone(),
                        1,
                        ((*refinement_bits.borrow())[(i) as usize] as u64),
                    )
                });
                i.prefix_inc();
            }
            refinement_bits_count = 0_usize;
        }
        if (abs_values[(k) as usize] > 1) {
            (*refinement_bits.borrow_mut())[(refinement_bits_count.postfix_inc()) as usize] =
                (((abs_values[(k) as usize] as u32) & 1_u32) as i32);
            k.postfix_inc();
            continue 'loop_;
        }
        ({ Flush_238((coding_state).clone(), (bw).clone()) });
        let mut symbol: i32 = ((r << 4_u32) + 1);
        let mut new_non_zero_bit: i32 = if (((elem!(
            coeffs,
            ({
                let __idx = (k) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            < 0)
        {
            0
        } else {
            1
        };
        ({
            let _nbits: i32 =
                (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), symbol).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), symbol).read()) as u64);
            WriteBits_233((bw).clone(), _nbits, _bits)
        });
        ({ WriteBits_233((bw).clone(), 1, (new_non_zero_bit as u64)) });
        let mut i: usize = 0_usize;
        'loop_: while (i < refinement_bits_count) {
            ({
                WriteBits_233(
                    (bw).clone(),
                    1,
                    ((*refinement_bits.borrow())[(i) as usize] as u64),
                )
            });
            i.prefix_inc();
        }
        refinement_bits_count = 0_usize;
        r = 0;
        k.postfix_inc();
    }
    if (r > 0) || (refinement_bits_count != 0) {
        if !({
            BufferEndOfBand_239(
                (coding_state).clone(),
                (ac_huff).clone(),
                (refinement_bits.as_pointer() as Ptr<i32>),
                refinement_bits_count,
                (bw).clone(),
            )
        }) {
            return false;
        }
        if !(eob_run_allowed) {
            ({ Flush_238((coding_state).clone(), (bw).clone()) });
        }
    }
    return true;
}
pub fn DoEncodeScan_255(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset((state.with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!(state, scan_state);
    let mut restart_interval: i32 = if state.with(|__s| __s.seen_dri_marker) {
        jpg.with(|__s| __s.restart_interval)
    } else {
        0
    };
    let get_next_extra_zero_run_index: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.extra_zero_runs_pos) } < {
                (*scan_info.with(|__s| __s.extra_zero_runs.clone()).borrow()).len()
            }) {
                return {
                    (*elem!(
                        (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                            as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                        ss.with(|__s| __s.extra_zero_runs_pos)
                    )
                    .upgrade()
                    .deref())
                    .block_idx
                };
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    let get_next_reset_point: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.next_reset_point_pos) } < {
                (*scan_info.with(|__s| __s.reset_points.clone()).borrow()).len()
            }) {
                return (elem!(
                    (scan_info.with(|__s| __s.reset_points.as_pointer()) as Ptr<i32>),
                    field!(ss, next_reset_point_pos).with_mut(|__v| __v.postfix_inc())
                )
                .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    if ((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_HEAD as i32))
    {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (state).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                (state.with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write(restart_interval);
        field!(ss, next_restart_marker).write(0);
        field!(ss, block_scan_index).write(0);
        field!(ss, extra_zero_runs_pos).write(0_usize);
        field!(ss, next_extra_zero_run_index)
            .write(({ (*get_next_extra_zero_run_index.borrow()).call() }).clone());
        field!(ss, next_reset_point_pos).write(0_usize);
        field!(ss, next_reset_point).write(({ (*get_next_reset_point.borrow()).call() }).clone());
        field!(ss, mcu_y).write(0);
        {
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
        };
        field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_BODY);
    }
    let mut bw: Ptr<brunsli_internal_dec_BitWriter> = (field_ptr!(ss, bw));
    let mut coding_state: Ptr<brunsli_internal_dec_DCTCodingState> = (field_ptr!(ss, coding_state));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<i8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let mut is_interleaved: bool = (scan_info.with(|__s| __s.num_components) > 1_usize);
    let base_component: Ptr<brunsli_JPEGComponent> =
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
    let mut h_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    };
    let mut v_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    };
    let mut MCUs_per_row: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.width) } * { h_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut MCU_rows: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.height) } * { v_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut is_progressive: bool = state.with(|__s| __s.is_progressive);
    let mut Al: i32 = if is_progressive {
        scan_info.with(|__s| __s.Al)
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
    let mut want_ac: bool = ((Ss != 0) || (Se != 0));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let mut has_ac: bool = (*complete_ac.borrow())
        || ({
            HasSection_194(
                (parsing_state).clone(),
                (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
            )
        });
    if (want_ac) && (!(has_ac)) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new(has_ac));
    let mut complete: bool = if want_ac {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    };
    let mut last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        ({
            {
                (*parsing_state
                    .with(|__s| __s.internal.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())
                .ac_dc
                .next_mcu_y
            }
        } * { v_group })
    };
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { last_mcu_y }) {
        let mut mcu_x: i32 = 0;
        'loop_: while (mcu_x < MCUs_per_row) {
            if (restart_interval > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((coding_state).clone(), (bw).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
                    let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({ EmitMarker_234((bw).clone(), (208 + ss.with(|__s| __s.next_restart_marker))) });
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v + 1)
                };
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v & 7)
                };
                field!(ss, restarts_to_go).write(restart_interval);
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let mut i: usize = 0_usize;
            'loop_: while ({ i } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(i);
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let mut n_blocks_y: i32 = if is_interleaved {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while (iy < n_blocks_y) {
                    let mut ix: i32 = 0;
                    'loop_: while (ix < n_blocks_x) {
                        let mut block_y: i32 =
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { n_blocks_y }) } + { iy });
                        let mut block_x: i32 = ((mcu_x * n_blocks_x) + ix);
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({ Flush_238((coding_state).clone(), (bw).clone()) });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let mut num_zero_runs: i32 = 0;
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            num_zero_runs = {
                                (*elem!(
                                    (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                                        as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                                    ss.with(|__s| __s.extra_zero_runs_pos)
                                )
                                .upgrade()
                                .deref())
                                .num_extra_zero_runs
                            };
                            field!(ss, extra_zero_runs_pos).with_mut(|__v| __v.prefix_inc());
                            field!(ss, next_extra_zero_run_index).write(
                                ({ (*get_next_extra_zero_run_index.borrow()).call() }).clone(),
                            );
                        }
                        let mut coeffs: Ptr<i16> = ((c.with(|__s| __s.coeffs.as_pointer())
                            as Ptr<i16>)
                            .offset(((block_idx << 6) as usize)));
                        let mut ok: bool = false;
                        if (0 == 0) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else if (0 == 1) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockProgressive_253(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _num_zero_runs,
                                    _coding_state,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            });
                        }
                        if !(ok) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            mcu_x.prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { MCU_rows }) {
        if !(bw.with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((coding_state).clone(), (bw).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
        let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((bw).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!(state, scan_index).with_mut(|__v| __v.postfix_inc());
    if !(bw.with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn DoEncodeScan_256(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset((state.with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!(state, scan_state);
    let mut restart_interval: i32 = if state.with(|__s| __s.seen_dri_marker) {
        jpg.with(|__s| __s.restart_interval)
    } else {
        0
    };
    let get_next_extra_zero_run_index: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.extra_zero_runs_pos) } < {
                (*scan_info.with(|__s| __s.extra_zero_runs.clone()).borrow()).len()
            }) {
                return {
                    (*elem!(
                        (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                            as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                        ss.with(|__s| __s.extra_zero_runs_pos)
                    )
                    .upgrade()
                    .deref())
                    .block_idx
                };
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    let get_next_reset_point: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.next_reset_point_pos) } < {
                (*scan_info.with(|__s| __s.reset_points.clone()).borrow()).len()
            }) {
                return (elem!(
                    (scan_info.with(|__s| __s.reset_points.as_pointer()) as Ptr<i32>),
                    field!(ss, next_reset_point_pos).with_mut(|__v| __v.postfix_inc())
                )
                .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    if ((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_HEAD as i32))
    {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (state).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                (state.with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write(restart_interval);
        field!(ss, next_restart_marker).write(0);
        field!(ss, block_scan_index).write(0);
        field!(ss, extra_zero_runs_pos).write(0_usize);
        field!(ss, next_extra_zero_run_index)
            .write(({ (*get_next_extra_zero_run_index.borrow()).call() }).clone());
        field!(ss, next_reset_point_pos).write(0_usize);
        field!(ss, next_reset_point).write(({ (*get_next_reset_point.borrow()).call() }).clone());
        field!(ss, mcu_y).write(0);
        {
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
        };
        field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_BODY);
    }
    let mut bw: Ptr<brunsli_internal_dec_BitWriter> = (field_ptr!(ss, bw));
    let mut coding_state: Ptr<brunsli_internal_dec_DCTCodingState> = (field_ptr!(ss, coding_state));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<i8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let mut is_interleaved: bool = (scan_info.with(|__s| __s.num_components) > 1_usize);
    let base_component: Ptr<brunsli_JPEGComponent> =
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
    let mut h_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    };
    let mut v_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    };
    let mut MCUs_per_row: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.width) } * { h_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut MCU_rows: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.height) } * { v_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut is_progressive: bool = state.with(|__s| __s.is_progressive);
    let mut Al: i32 = if is_progressive {
        scan_info.with(|__s| __s.Al)
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
    let mut want_ac: bool = ((Ss != 0) || (Se != 0));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let mut has_ac: bool = (*complete_ac.borrow())
        || ({
            HasSection_194(
                (parsing_state).clone(),
                (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
            )
        });
    if (want_ac) && (!(has_ac)) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new(has_ac));
    let mut complete: bool = if want_ac {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    };
    let mut last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        ({
            {
                (*parsing_state
                    .with(|__s| __s.internal.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())
                .ac_dc
                .next_mcu_y
            }
        } * { v_group })
    };
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { last_mcu_y }) {
        let mut mcu_x: i32 = 0;
        'loop_: while (mcu_x < MCUs_per_row) {
            if (restart_interval > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((coding_state).clone(), (bw).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
                    let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({ EmitMarker_234((bw).clone(), (208 + ss.with(|__s| __s.next_restart_marker))) });
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v + 1)
                };
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v & 7)
                };
                field!(ss, restarts_to_go).write(restart_interval);
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let mut i: usize = 0_usize;
            'loop_: while ({ i } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(i);
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let mut n_blocks_y: i32 = if is_interleaved {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while (iy < n_blocks_y) {
                    let mut ix: i32 = 0;
                    'loop_: while (ix < n_blocks_x) {
                        let mut block_y: i32 =
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { n_blocks_y }) } + { iy });
                        let mut block_x: i32 = ((mcu_x * n_blocks_x) + ix);
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({ Flush_238((coding_state).clone(), (bw).clone()) });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let mut num_zero_runs: i32 = 0;
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            num_zero_runs = {
                                (*elem!(
                                    (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                                        as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                                    ss.with(|__s| __s.extra_zero_runs_pos)
                                )
                                .upgrade()
                                .deref())
                                .num_extra_zero_runs
                            };
                            field!(ss, extra_zero_runs_pos).with_mut(|__v| __v.prefix_inc());
                            field!(ss, next_extra_zero_run_index).write(
                                ({ (*get_next_extra_zero_run_index.borrow()).call() }).clone(),
                            );
                        }
                        let mut coeffs: Ptr<i16> = ((c.with(|__s| __s.coeffs.as_pointer())
                            as Ptr<i16>)
                            .offset(((block_idx << 6) as usize)));
                        let mut ok: bool = false;
                        if (1 == 0) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else if (1 == 1) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockProgressive_253(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _num_zero_runs,
                                    _coding_state,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            });
                        }
                        if !(ok) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            mcu_x.prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { MCU_rows }) {
        if !(bw.with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((coding_state).clone(), (bw).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
        let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((bw).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!(state, scan_index).with_mut(|__v| __v.postfix_inc());
    if !(bw.with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn DoEncodeScan_257(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset((state.with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!(state, scan_state);
    let mut restart_interval: i32 = if state.with(|__s| __s.seen_dri_marker) {
        jpg.with(|__s| __s.restart_interval)
    } else {
        0
    };
    let get_next_extra_zero_run_index: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.extra_zero_runs_pos) } < {
                (*scan_info.with(|__s| __s.extra_zero_runs.clone()).borrow()).len()
            }) {
                return {
                    (*elem!(
                        (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                            as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                        ss.with(|__s| __s.extra_zero_runs_pos)
                    )
                    .upgrade()
                    .deref())
                    .block_idx
                };
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    let get_next_reset_point: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (ss).clone();
            let scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
        },
        || -> i32 {
            if ({ ss.with(|__s| __s.next_reset_point_pos) } < {
                (*scan_info.with(|__s| __s.reset_points.clone()).borrow()).len()
            }) {
                return (elem!(
                    (scan_info.with(|__s| __s.reset_points.as_pointer()) as Ptr<i32>),
                    field!(ss, next_reset_point_pos).with_mut(|__v| __v.postfix_inc())
                )
                .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }
    )));
    if ((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_HEAD as i32))
    {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (state).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                (state.with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write(restart_interval);
        field!(ss, next_restart_marker).write(0);
        field!(ss, block_scan_index).write(0);
        field!(ss, extra_zero_runs_pos).write(0_usize);
        field!(ss, next_extra_zero_run_index)
            .write(({ (*get_next_extra_zero_run_index.borrow()).call() }).clone());
        field!(ss, next_reset_point_pos).write(0_usize);
        field!(ss, next_reset_point).write(({ (*get_next_reset_point.borrow()).call() }).clone());
        field!(ss, mcu_y).write(0);
        {
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
            ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
        };
        field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_BODY);
    }
    let mut bw: Ptr<brunsli_internal_dec_BitWriter> = (field_ptr!(ss, bw));
    let mut coding_state: Ptr<brunsli_internal_dec_DCTCodingState> = (field_ptr!(ss, coding_state));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<i8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let mut is_interleaved: bool = (scan_info.with(|__s| __s.num_components) > 1_usize);
    let base_component: Ptr<brunsli_JPEGComponent> =
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
    let mut h_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    };
    let mut v_group: i32 = if is_interleaved {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    };
    let mut MCUs_per_row: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.width) } * { h_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut MCU_rows: i32 = ({
        let _a: i32 = ({ jpg.with(|__s| __s.height) } * { v_group });
        let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let mut is_progressive: bool = state.with(|__s| __s.is_progressive);
    let mut Al: i32 = if is_progressive {
        scan_info.with(|__s| __s.Al)
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
    let mut want_ac: bool = ((Ss != 0) || (Se != 0));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let mut has_ac: bool = (*complete_ac.borrow())
        || ({
            HasSection_194(
                (parsing_state).clone(),
                (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
            )
        });
    if (want_ac) && (!(has_ac)) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new(has_ac));
    let mut complete: bool = if want_ac {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    };
    let mut last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        ({
            {
                (*parsing_state
                    .with(|__s| __s.internal.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())
                .ac_dc
                .next_mcu_y
            }
        } * { v_group })
    };
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { last_mcu_y }) {
        let mut mcu_x: i32 = 0;
        'loop_: while (mcu_x < MCUs_per_row) {
            if (restart_interval > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((coding_state).clone(), (bw).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
                    let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({ EmitMarker_234((bw).clone(), (208 + ss.with(|__s| __s.next_restart_marker))) });
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v + 1)
                };
                {
                    field!(ss, next_restart_marker).with_mut(|__v| *__v = *__v & 7)
                };
                field!(ss, restarts_to_go).write(restart_interval);
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let mut i: usize = 0_usize;
            'loop_: while ({ i } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(i);
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = (state
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let mut n_blocks_y: i32 = if is_interleaved {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while (iy < n_blocks_y) {
                    let mut ix: i32 = 0;
                    'loop_: while (ix < n_blocks_x) {
                        let mut block_y: i32 =
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { n_blocks_y }) } + { iy });
                        let mut block_x: i32 = ((mcu_x * n_blocks_x) + ix);
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({ Flush_238((coding_state).clone(), (bw).clone()) });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let mut num_zero_runs: i32 = 0;
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            num_zero_runs = {
                                (*elem!(
                                    (scan_info.with(|__s| __s.extra_zero_runs.as_pointer())
                                        as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                                    ss.with(|__s| __s.extra_zero_runs_pos)
                                )
                                .upgrade()
                                .deref())
                                .num_extra_zero_runs
                            };
                            field!(ss, extra_zero_runs_pos).with_mut(|__v| __v.prefix_inc());
                            field!(ss, next_extra_zero_run_index).write(
                                ({ (*get_next_extra_zero_run_index.borrow()).call() }).clone(),
                            );
                        }
                        let mut coeffs: Ptr<i16> = ((c.with(|__s| __s.coeffs.as_pointer())
                            as Ptr<i16>)
                            .offset(((block_idx << 6) as usize)));
                        let mut ok: bool = false;
                        if (2 == 0) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else if (2 == 1) {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeDCTBlockProgressive_253(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _num_zero_runs,
                                    _coding_state,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            });
                        } else {
                            ok = ({
                                let _coeffs: Ptr<i16> = (coeffs).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (coding_state).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (bw).clone();
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            });
                        }
                        if !(ok) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            mcu_x.prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { MCU_rows }) {
        if !(bw.with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((coding_state).clone(), (bw).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!(state, pad_bits));
        let _pad_bits_end: Ptr<i32> = state.with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((bw).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((bw).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!(state, scan_index).with_mut(|__v| __v.postfix_inc());
    if !(bw.with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn EncodeScan_258(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset((state.with(|__s| __s.scan_index) as usize));
    let mut is_progressive: bool = state.with(|__s| __s.is_progressive);
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
    let mut need_sequential: bool =
        (!(is_progressive)) || ((((Ah == 0) && (Al == 0)) && (Ss == 0)) && (Se == 63));
    if need_sequential {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
            DoEncodeScan_255(_jpg, _parsing_state, _state)
        });
    } else if (Ah == 0) {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
            DoEncodeScan_256(_jpg, _parsing_state, _state)
        });
    } else {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
            DoEncodeScan_257(_jpg, _parsing_state, _state)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn SerializeSection_259(
    mut marker: u8,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    mut state: Ptr<brunsli_internal_dec_SerializationState>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_SerializationStatus {
    let to_status: Value<FnPtr<fn(bool) -> brunsli_internal_dec_SerializationStatus>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(bool) -> brunsli_internal_dec_SerializationStatus,
        >::new(
            |result: bool| -> brunsli_internal_dec_SerializationStatus {
                {
                    return if result {
                        brunsli_internal_dec_SerializationStatus_DONE
                    } else {
                        brunsli_internal_dec_SerializationStatus_ERROR
                    };
                }
            },
        )));
    'switch: {
        match { (marker as i32) } {
            __v if __v == 192 || __v == 193 || __v == 194 || __v == 201 || __v == 202 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _marker: u8 = marker;
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeSOF_243(_jpg, _marker, _state)
                        }),
                    )
                });
            }
            __v if __v == 196 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeDHT_245(_jpg, _state)
                        }),
                    )
                });
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
                return ({ (*to_status.borrow()).call(({ EncodeRestart_248(marker, state) })) });
            }
            __v if __v == 217 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeEOI_242(_jpg, _state)
                        }),
                    )
                });
            }
            __v if __v == 218 => {
                return ({
                    let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                    let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                    let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                    EncodeScan_258(_jpg, _parsing_state, _state)
                });
            }
            __v if __v == 219 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeDQT_246(_jpg, _state)
                        }),
                    )
                });
            }
            __v if __v == 221 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeDRI_247(_jpg, _state)
                        }),
                    )
                });
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
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _marker: u8 = marker;
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeAPP_249(_jpg, _marker, _state)
                        }),
                    )
                });
            }
            __v if __v == 254 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeCOM_250(_jpg, _state)
                        }),
                    )
                });
            }
            __v if __v == 255 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = state;
                            EncodeInterMarkerData_251(_jpg, _state)
                        }),
                    )
                });
            }
            _ => {
                return brunsli_internal_dec_SerializationStatus_ERROR;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn PushOutput_260(
    mut in_: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
    mut available_out: Ptr<usize>,
    mut next_out: Ptr<Ptr<u8>>,
) {
    'loop_: while ((available_out.read()) > 0_usize) {
        if (*in_.upgrade().deref()).is_empty() {
            return;
        }
        let chunk: Ptr<brunsli_internal_dec_OutputChunk> =
            (Ptr::<Vec<brunsli_internal_dec_OutputChunk>>::decay(&(in_))
                as Ptr<brunsli_internal_dec_OutputChunk>);
        let mut to_copy: usize = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((available_out.read()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new((chunk.with(|__s| __s.len) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        if (to_copy > 0_usize) {
            {
                ((next_out.read()) as Ptr<u8>).to_any().memcpy(
                    &(chunk.with(|__s| __s.next.clone()) as Ptr<u8>).to_any(),
                    to_copy as usize,
                );
                ((next_out.read()) as Ptr<u8>).to_any()
            };
            let __rhs = to_copy;
            {
                let _ptr = next_out.clone();
                _ptr.write(_ptr.read() + __rhs)
            };
            available_out.write({ (available_out.read()).wrapping_sub(to_copy) });
            {
                let _ptr = field!(chunk, next);
                _ptr.write(_ptr.read() + to_copy)
            };
            field!(chunk, len).write({ (chunk.with(|__s| __s.len)).wrapping_sub(to_copy) });
        }
        if (chunk.with(|__s| __s.len) == 0_usize) {
            in_.with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.remove(0));
        }
    }
}
pub fn WriteJpeg_261(jpg: Ptr<brunsli_JPEGData>, out: brunsli_JPEGOutput) -> bool {
    let out: Value<brunsli_JPEGOutput> = Rc::new(RefCell::new(out));
    let state: Value<brunsli_internal_dec_State> =
        Rc::new(RefCell::new(brunsli_internal_dec_State::new()));
    (*state.borrow_mut()).stage = brunsli_internal_dec_Stage_DONE;
    let buffer: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(16384_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    'loop_: while true {
        let next_out: Value<Ptr<u8>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<u8>)));
        let available_out: Value<usize> = Rc::new(RefCell::new((*buffer.borrow()).len()));
        let mut status: brunsli_internal_dec_SerializationStatus = ({
            let _state: Ptr<brunsli_internal_dec_State> = (state.as_pointer());
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _available_out: Ptr<usize> = (available_out.as_pointer());
            let _next_out: Ptr<Ptr<u8>> = (next_out.as_pointer());
            SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
        });
        if (status != brunsli_internal_dec_SerializationStatus_DONE)
            && (status != brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT)
        {
            return false;
        }
        let mut to_write: usize = (((*buffer.borrow()).len() as u64)
            .wrapping_sub(((*available_out.borrow()) as u64))
            as usize);
        if !({
            brunsli_JPEGOutputImpl::Write(
                &out.as_pointer(),
                (buffer.as_pointer() as Ptr<u8>),
                to_write,
            )
        }) {
            return false;
        }
        if (status == brunsli_internal_dec_SerializationStatus_DONE) {
            return true;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn SerializeJpeg_206(
    mut state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
    available_out: Ptr<usize>,
    next_out: Ptr<Ptr<u8>>,
) -> brunsli_internal_dec_SerializationStatus {
    let available_out: Value<Ptr<usize>> = Rc::new(RefCell::new(available_out));
    let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
    let ss: Ptr<brunsli_internal_dec_SerializationState> = field_ptr!(
        state.with(|__s| __s.internal.clone()).as_pointer(),
        serialization
    );
    let maybe_push_output: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let ss: Ptr<brunsli_internal_dec_SerializationState> = (ss).clone();
            let available_out: Ptr<Ptr<usize>> = available_out.as_pointer();
            let next_out: Ptr<Ptr<Ptr<u8>>> = next_out.as_pointer();
        },
        || {
            if ((ss.with(|__s| __s.stage) as i32)
                != (brunsli_internal_dec_SerializationState_Stage_ERROR as i32))
            {
                ({
                    PushOutput_260(
                        (ss.with(|__s| __s.output_queue.as_pointer())),
                        (available_out.read()),
                        (next_out.read()),
                    )
                });
            }
        }
    )));
    ({ (*maybe_push_output.borrow()).call() });
    'loop_: while true {
        switch!(match (ss.with(|__s| __s.stage) as i32) {
            __v if __v == (brunsli_internal_dec_SerializationState_Stage_INIT as i32) => {
                {
                    let mut can_start_serialization: bool =
                        (state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE);
                    if ({
                        HasSection_194(
                            (state).clone(),
                            (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as u32),
                        )
                    }) || ({
                        HasSection_194(
                            (state).clone(),
                            (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                        )
                    }) {
                        can_start_serialization = true;
                    }
                    if !(can_start_serialization) {
                        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
                    }
                    if ({ jpg.with(|__s| __s.version) } == {
                        kFallbackVersion_2.with(|rc| *rc.borrow())
                    }) {
                        if (jpg.with(|__s| __s.original_jpg.clone())).is_null() {
                            field!(ss, stage)
                                .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                            break;
                        }
                        {
                            let __init = brunsli_internal_dec_OutputChunk::new_1(
                                { jpg.with(|__s| __s.original_jpg.clone()) },
                                { jpg.with(|__s| __s.original_jpg_size) },
                            );
                            (*ss.with(|__s| __s.output_queue.clone()).borrow_mut()).push(__init)
                        };
                        field!(ss, stage).write(brunsli_internal_dec_SerializationState_Stage_DONE);
                        break;
                    }
                    if ({ (jpg.with(|__s| __s.version) & 1) } == {
                        kFallbackVersion_2.with(|rc| *rc.borrow())
                    }) {
                        field!(ss, stage)
                            .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                        break;
                    }
                    if (*jpg.with(|__s| __s.marker_order.clone()).borrow()).is_empty() {
                        field!(ss, stage)
                            .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                        break;
                    }
                    {
                        let __a0 = (kMaxHuffmanTables_6.with(|rc| *rc.borrow()) as usize) as usize;
                        (*ss.with(|__s| __s.dc_huff_table.clone()).borrow_mut())
                            .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                    };
                    {
                        let __a0 = (kMaxHuffmanTables_6.with(|rc| *rc.borrow()) as usize) as usize;
                        (*ss.with(|__s| __s.ac_huff_table.clone()).borrow_mut())
                            .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                    };
                    if jpg.with(|__s| __s.has_zero_padding_bit) {
                        field!(ss, pad_bits)
                            .write((jpg.with(|__s| __s.padding_bits.as_pointer()) as Ptr<i32>));
                        let __rhs = ss.with(|__s| __s.pad_bits.clone()).offset(
                            ((*jpg.with(|__s| __s.padding_bits.clone()).borrow()).len()) as isize,
                        );
                        field!(ss, pad_bits_end).write(__rhs);
                    }
                    ({ EncodeSOI_241((ss).clone()) });
                    ({ (*maybe_push_output.borrow()).call() });
                    field!(ss, stage)
                        .write(brunsli_internal_dec_SerializationState_Stage_SERIALIZE_SECTION);
                    break;
                }
            }
            __v if __v
                == (brunsli_internal_dec_SerializationState_Stage_SERIALIZE_SECTION as i32) =>
            {
                {
                    if ({ ss.with(|__s| __s.section_index) } >= {
                        (*jpg.with(|__s| __s.marker_order.clone()).borrow()).len()
                    }) {
                        field!(ss, stage).write(brunsli_internal_dec_SerializationState_Stage_DONE);
                        break;
                    }
                    let mut marker: u8 = (elem!(
                        (jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
                        ss.with(|__s| __s.section_index)
                    )
                    .read());
                    let mut status: brunsli_internal_dec_SerializationStatus = ({
                        let _marker: u8 = marker;
                        let _parsing_state: Ptr<brunsli_internal_dec_State> = (state).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> = (ss).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        SerializeSection_259(_marker, _parsing_state, _state, _jpg)
                    });
                    if (status == brunsli_internal_dec_SerializationStatus_ERROR) {
                        if true {
                        } else {
                            write!(libcc2rs::cerr(), "Failed to encode marker ",);
                            libcc2rs::cerr().write_all(&([(&[marker as u8] as &[u8])].concat()));
                            write!(libcc2rs::cerr(), "\n",);
                        }
                        field!(ss, stage)
                            .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                        break;
                    }
                    ({ (*maybe_push_output.borrow()).call() });
                    if (status == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT) {
                        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
                    } else if (status != brunsli_internal_dec_SerializationStatus_DONE) {
                        if !(false) {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<i8>::from_string_literal(b"jpeg_data_writer.cc"),
                                    1073,
                                    Ptr::<i8>::from_string_literal(b"SerializeJpeg"),
                                )
                            });
                            'loop_: while true {}
                        };
                        field!(ss, stage)
                            .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                        break;
                    }
                    field!(ss, section_index).with_mut(|__v| __v.prefix_inc());
                    break;
                }
            }
            __v if __v == (brunsli_internal_dec_SerializationState_Stage_DONE as i32) => {
                {
                    if !((*ss.with(|__s| __s.output_queue.clone()).borrow()).is_empty()) {
                        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT;
                    } else {
                        return brunsli_internal_dec_SerializationStatus_DONE;
                    }
                }
            }
            _ => {
                return brunsli_internal_dec_SerializationStatus_ERROR;
            }
        });
    }
    panic!("ub: non-void function does not return a value")
}
impl brunsli_internal_dec_State {
    pub fn new() -> Self {
        Self {
            stage: brunsli_internal_dec_Stage_SIGNATURE,
            tags_met: 0_u32,
            skip_tags: 0_u32,
            data: Ptr::<u8>::null(),
            len: 0_usize,
            pos: 0_usize,
            context_map: Ptr::<u8>::null(),
            entropy_codes: Ptr::<brunsli_ANSDecodingData>::null(),
            use_legacy_context_model: false,
            is_storage_allocated: false,
            meta: Rc::new(RefCell::new(Vec::new())),
            internal: Ptr::alloc(<brunsli_internal_dec_InternalState>::default()).to_owned_opt(),
        }
    }
}
impl brunsli_internal_dec_State {}
impl brunsli_internal_dec_State {}
pub fn HasSection_194(mut state: Ptr<brunsli_internal_dec_State>, mut tag: u32) -> bool {
    return (({
        {
            (*state
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .tags_met
        }
    } & { (1_u32 << tag) })
        != 0);
}
pub fn StringWriter_262(mut data: AnyPtr, mut buf: Ptr<u8>, mut count: usize) -> usize {
    let mut output: Ptr<Vec<i8>> = data.reinterpret_cast::<Vec<i8>>();
    {
        ((output).clone() as Ptr<Vec<i8>>).with_mut(|__v: &mut Vec<i8>| {
            __v.pop();
            buf.reinterpret_cast::<i8>()
                .with_slice(count as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0);
        });
        ((output).clone() as Ptr<Vec<i8>>)
    };
    return count;
}
pub fn ReadFileInternal_263(mut file: Ptr<CFile>, mut content: Ptr<Vec<i8>>) -> bool {
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
pub fn ReadFile_264(file_name: Ptr<Vec<i8>>, mut content: Ptr<Vec<i8>>) -> bool {
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
    let mut ok: bool = ({ ReadFileInternal_263((file).clone(), (content).clone()) });
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
pub fn WriteFileInternal_265(mut file: Ptr<CFile>, content: Ptr<Vec<i8>>) -> bool {
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
pub fn WriteFile_266(file_name: Ptr<Vec<i8>>, content: Ptr<Vec<i8>>) -> bool {
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
        WriteFileInternal_265(_file, _content)
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
pub fn ProcessFile_267(file_name: Ptr<Vec<i8>>, outfile_name: Ptr<Vec<i8>>) -> bool {
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    let mut ok: bool = ({
        let _file_name: Ptr<Vec<i8>> = (file_name).clone();
        let _content: Ptr<Vec<i8>> = (input.as_pointer());
        ReadFile_264(_file_name, _content)
    });
    if !(ok) {
        return false;
    }
    let output: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    {
        let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
        let mut input_data: Ptr<u8> = (input.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>();
        let mut status: brunsli_BrunsliStatus = ({
            BrunsliDecodeJpeg_204(
                (input_data).clone(),
                ((*input.borrow()).len() - 1),
                (jpg.as_pointer()),
            )
        });
        ok = ((status as i32) == (brunsli_BrunsliStatus_BRUNSLI_OK as i32));
        if ({ (*jpg.borrow()).version } != kFallbackVersion_2.with(|rc| *rc.borrow())) {
            {
                (*input.borrow_mut()).clear();
                (*input.borrow_mut()).push(0)
            };
            (*input.borrow_mut()).shrink_to_fit();
        }
        if !(ok) {
            eprintln!("Failed to parse Brunsli input.");
            return false;
        }
        let mut writer: brunsli_JPEGOutput = brunsli_JPEGOutput::new(
            { FnPtr::<fn(AnyPtr, Ptr<u8>, usize) -> usize>::new(StringWriter_262) },
            { ((output.as_pointer()) as Ptr<Vec<i8>>).to_any() },
        );
        ok = ({ WriteJpeg_261(jpg.as_pointer(), (writer).clone()) });
        if !(ok) {
            eprintln!("Failed to serialize JPEG data.");
            return false;
        }
    }
    ok = ({
        let _file_name: Ptr<Vec<i8>> = (outfile_name).clone();
        let _content: Ptr<Vec<i8>> = output.as_pointer();
        WriteFile_266(_file_name, _content)
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
        eprintln!("Usage: dbrunsli FILE [OUTPUT_FILE, default=FILE.jpg]");
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
            Ptr::<i8>::from_string_literal(b".jpg").with_c_str(|__s| r.extend_from_slice(__s));
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
    let mut ok: bool = ({ ProcessFile_267(file_name.as_pointer(), outfile_name.as_pointer()) });
    return if ok { 0 } else { 1 };
}
pub trait brunsli_ANSDecoderImpl {
    fn Init(&self, in_: Ptr<brunsli_WordSource>);
    fn ReadSymbol(&self, code: Ptr<brunsli_ANSDecodingData>, in_: Ptr<brunsli_WordSource>) -> i32;
    fn CheckCRC(&self) -> bool;
}
impl brunsli_ANSDecoderImpl for Ptr<brunsli_ANSDecoder> {
    fn Init(&self, mut in_: Ptr<brunsli_WordSource>) {
        field!((*self), state_).write((({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32));
        let __rhs = ({ ((*self).with(|__s| __s.state_) << 16_u32) } | {
            (({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32)
        });
        field!((*self), state_).write(__rhs);
    }
    fn ReadSymbol(
        &self,
        code: Ptr<brunsli_ANSDecodingData>,
        mut in_: Ptr<brunsli_WordSource>,
    ) -> i32 {
        let mut res: u32 = ((*self).with(|__s| __s.state_)
            & ((BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow()) - 1) as u32));
        let s: Ptr<brunsli_ANSSymbolInfo> =
            (array_field_ptr!(code, map_) as Ptr<brunsli_ANSSymbolInfo>).offset((res) as isize);
        field!((*self), state_).write({
            ((s.with(|__s| __s.freq_) as u32).wrapping_mul(
                ((*self).with(|__s| __s.state_)
                    >> BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())),
            ))
            .wrapping_add((s.with(|__s| __s.offset_) as u32))
        });
        if ((*self).with(|__s| __s.state_) < (1_u32 << 16_u32)) {
            let __rhs = ({ ((*self).with(|__s| __s.state_) << 16_u32) } | {
                (({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32)
            });
            field!((*self), state_).write(__rhs);
        }
        return (s.with(|__s| __s.symbol_) as i32);
    }
    fn CheckCRC(&self) -> bool {
        return ((*self).with(|__s| __s.state_) == (19_u32 << 16_u32));
    }
}
pub trait brunsli_ANSDecodingDataImpl {
    fn Init(&self, counts: Ptr<Vec<u32>>) -> bool;
}
impl brunsli_ANSDecodingDataImpl for Ptr<brunsli_ANSDecodingData> {
    fn Init(&self, counts: Ptr<Vec<u32>>) -> bool {
        let mut pos: usize = 0_usize;
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*counts.upgrade().deref()).len() }) {
            let mut j: usize = 0_usize;
            'loop_: while ({ j } < {
                ((elem!((Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>), i).read()) as usize)
            }) {
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        pos
                    ),
                    symbol_
                )
                .write((i as u8));
                let __rhs =
                    ((elem!((Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>), i).read()) as u16);
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        pos
                    ),
                    freq_
                )
                .write(__rhs);
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        pos
                    ),
                    offset_
                )
                .write((j as u16));
                {
                    j.prefix_inc();
                    pos.prefix_inc()
                };
            }
            i.prefix_inc();
        }
        return (pos == (BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow()) as usize));
    }
}
pub trait brunsli_Arena_brunsli_HuffmanCode_Impl {
    fn reserve(&self, limit: usize);
    fn data(&self) -> Ptr<brunsli_HuffmanCode> {
        unimplemented!()
    }
    fn reset(&self);
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_Arena_brunsli_HuffmanCode_>,
    ) -> Ptr<brunsli_Arena_brunsli_HuffmanCode_>;
}
impl brunsli_Arena_brunsli_HuffmanCode_Impl for Ptr<brunsli_Arena_brunsli_HuffmanCode_> {
    fn reserve(&self, mut limit: usize) {
        if ((*self).with(|__s| __s.capacity) < limit) {
            field!((*self), capacity).write(limit);
            (field_ptr!((*self), storage) as Ptr<Option<Value<Box<[brunsli_HuffmanCode]>>>>).write(
                Ptr::alloc_array(
                    (0..(*self).with(|__s| __s.capacity))
                        .map(|_| <brunsli_HuffmanCode>::default())
                        .collect::<Box<[brunsli_HuffmanCode]>>(),
                )
                .to_owned_opt(),
            );
        }
    }
    fn reset(&self) {
        field!((*self), capacity).write(0_usize);
        (field_ptr!((*self), storage) as Ptr<Option<Value<Box<[brunsli_HuffmanCode]>>>>)
            .write(None);
    }
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_Arena_brunsli_HuffmanCode_>,
    ) -> Ptr<brunsli_Arena_brunsli_HuffmanCode_> {
        field!((*self), capacity).write({ { (*_a0.upgrade().deref()).capacity } });
        (field_ptr!((*self), storage) as Ptr<Option<Value<Box<[brunsli_HuffmanCode]>>>>).write(
            field!(_a0, storage)
                .with_mut(|__v: &mut Option<Value<Box<[brunsli_HuffmanCode]>>>| __v.take()),
        );
        return (*self).clone();
    }
    fn data(&self) -> Ptr<brunsli_HuffmanCode> {
        return (*self).with(|__s| __s.storage.clone()).as_pointer();
    }
}
pub trait brunsli_BinaryArithmeticDecoderImpl {
    fn Init(&self, in_: Ptr<brunsli_WordSource>);
    fn ReadBit(&self, prob: i32, in_: Ptr<brunsli_WordSource>) -> i32;
}
impl brunsli_BinaryArithmeticDecoderImpl for Ptr<brunsli_BinaryArithmeticDecoder> {
    fn Init(&self, mut in_: Ptr<brunsli_WordSource>) {
        field!((*self), low_).write(0_u32);
        field!((*self), high_).write(!0_u32);
        field!((*self), value_).write((({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32));
        let __rhs = ({ ((*self).with(|__s| __s.value_) << 16_u32) } | {
            (({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32)
        });
        field!((*self), value_).write(__rhs);
    }
    fn ReadBit(&self, mut prob: i32, mut in_: Ptr<brunsli_WordSource>) -> i32 {
        let mut diff: u32 =
            ((*self).with(|__s| __s.high_)).wrapping_sub((*self).with(|__s| __s.low_));
        let mut split: u32 = ((((*self).with(|__s| __s.low_) as u64)
            .wrapping_add((((diff as u64).wrapping_mul((prob as u64))) >> 8_u32)))
            as u32);
        let mut bit: i32 = 0_i32;
        if ((*self).with(|__s| __s.value_) > split) {
            field!((*self), low_).write((split).wrapping_add(1_u32));
            bit = 1;
        } else {
            field!((*self), high_).write(split);
            bit = 0;
        }
        if ((((*self).with(|__s| __s.low_) ^ (*self).with(|__s| __s.high_)) >> 16_u32) == 0_u32) {
            let __rhs = ({ ((*self).with(|__s| __s.value_) << 16_u32) } | {
                (({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32)
            });
            field!((*self), value_).write(__rhs);
            {
                field!((*self), low_).with_mut(|__v| *__v = *__v << 16_u32)
            };
            {
                field!((*self), high_).with_mut(|__v| *__v = *__v << 16_u32)
            };
            {
                field!((*self), high_).with_mut(|__v| *__v = *__v | 65535_u32)
            };
        }
        return bit;
    }
}
pub trait brunsli_BitSourceImpl {
    fn Init(&self, in_: Ptr<brunsli_WordSource>);
    fn ReadBits(&self, nbits: i32, in_: Ptr<brunsli_WordSource>) -> u32;
    fn Finish(&self) -> bool;
}
impl brunsli_BitSourceImpl for Ptr<brunsli_BitSource> {
    fn Init(&self, mut in_: Ptr<brunsli_WordSource>) {
        field!((*self), val_).write((({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32));
        field!((*self), bit_pos_).write(0);
    }
    fn ReadBits(&self, mut nbits: i32, mut in_: Ptr<brunsli_WordSource>) -> u32 {
        if (((*self).with(|__s| __s.bit_pos_) + nbits) > 16) {
            let mut new_bits: u32 = (({ brunsli_WordSourceImpl::GetNextWord(&in_) }) as u32);
            {
                let __rhs = (new_bits << 16);
                field!((*self), val_).with_mut(|__v| *__v = *__v | __rhs)
            };
        }
        let mut result: u32 = (((*self).with(|__s| __s.val_) >> (*self).with(|__s| __s.bit_pos_))
            & (({
                let __idx = (nbits) as usize;
                kBitMask_120.with(|rc| rc.borrow()[__idx])
            }) as u32));
        {
            let __rhs = nbits;
            field!((*self), bit_pos_).with_mut(|__v| *__v = *__v + __rhs)
        };
        if ((*self).with(|__s| __s.bit_pos_) > 16) {
            {
                field!((*self), bit_pos_).with_mut(|__v| *__v = *__v - 16)
            };
            {
                field!((*self), val_).with_mut(|__v| *__v = *__v >> 16)
            };
        }
        return result;
    }
    fn Finish(&self) -> bool {
        let mut n_bits: usize = ((16 - (*self).with(|__s| __s.bit_pos_)) as usize);
        if (n_bits > 0_usize) {
            let mut padding_bits: i32 = ((((*self).with(|__s| __s.val_)
                >> (*self).with(|__s| __s.bit_pos_))
                & (({
                    let __idx = (n_bits) as usize;
                    kBitMask_120.with(|rc| rc.borrow()[__idx])
                }) as u32)) as i32);
            if (padding_bits != 0) {
                return false;
            }
        }
        return true;
    }
}
pub trait brunsli_BrunsliDecoderImpl {
    fn destructor(&self);
    fn Decode(
        &self,
        available_in: Ptr<usize>,
        next_in: Ptr<Ptr<u8>>,
        available_out: Ptr<usize>,
        next_out: Ptr<Ptr<u8>>,
    ) -> brunsli_BrunsliDecoder_Status;
}
impl brunsli_BrunsliDecoderImpl for Ptr<brunsli_BrunsliDecoder> {
    fn destructor(&self) {}
    fn Decode(
        &self,
        mut available_in: Ptr<usize>,
        mut next_in: Ptr<Ptr<u8>>,
        mut available_out: Ptr<usize>,
        mut next_out: Ptr<Ptr<u8>>,
    ) -> brunsli_BrunsliDecoder_Status {
        let mut jpg: Ptr<brunsli_JPEGData> = (*self).with(|__s| __s.jpg_.clone()).as_pointer();
        if !(!(jpg).is_null()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2511,
                    Ptr::<i8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        let mut state: Ptr<brunsli_internal_dec_State> =
            (*self).with(|__s| __s.state_.clone()).as_pointer();
        if !(!(state).is_null()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2513,
                    Ptr::<i8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        field!(state, data).write({ (next_in.read()) });
        field!(state, pos).write(0_usize);
        field!(state, len).write({ (available_in.read()) });
        let mut parse_status: brunsli_BrunsliStatus =
            ({ ProcessJpeg_203((state).clone(), (jpg).clone()) });
        let mut consumed_bytes: usize = state.with(|__s| __s.pos);
        available_in.write({ (available_in.read()).wrapping_sub(consumed_bytes) });
        let __rhs = consumed_bytes;
        {
            let _ptr = next_in.clone();
            _ptr.write(_ptr.read() + __rhs)
        };
        if ((parse_status as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32))
            && ((parse_status as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            return brunsli_BrunsliDecoder_Status_ERROR;
        }
        if !((available_in.read()) == 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                    2529,
                    Ptr::<i8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        let mut serialization_status: brunsli_internal_dec_SerializationStatus = ({
            let _state: Ptr<brunsli_internal_dec_State> = (state).clone();
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _available_out: Ptr<usize> = (available_out).clone();
            let _next_out: Ptr<Ptr<u8>> = (next_out).clone();
            SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
        });
        if (serialization_status == brunsli_internal_dec_SerializationStatus_ERROR) {
            return brunsli_BrunsliDecoder_Status_ERROR;
        }
        'switch: {
            match { serialization_status } {
                __v if __v == brunsli_internal_dec_SerializationStatus_DONE => {
                    if !((parse_status as i32) == (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                2540,
                                Ptr::<i8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_DONE;
                }
                __v if __v == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT => {
                    if !((parse_status as i32)
                        == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                2545,
                                Ptr::<i8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_NEEDS_MORE_INPUT;
                }
                __v if __v == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT => {
                    if !((available_out.read()) == 0_usize) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                2551,
                                Ptr::<i8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_NEEDS_MORE_OUTPUT;
                }
                __v if __v == brunsli_internal_dec_SerializationStatus_ERROR => {
                    return brunsli_BrunsliDecoder_Status_ERROR;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<i8>::from_string_literal(b"brunsli_decode.cc"),
                                2559,
                                Ptr::<i8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_ERROR;
                }
            }
        };
        panic!("ub: non-void function does not return a value")
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
pub trait brunsli_HuffmanDecodingDataImpl {
    fn ReadFromBitStream(
        &self,
        mut alphabet_size: usize,
        mut br: Ptr<brunsli_BrunsliBitReader>,
        arena: Option<Ptr<brunsli_Arena_brunsli_HuffmanCode_>>,
    ) -> bool {
        unimplemented!()
    }
    fn ReadSymbol(&self, mut br: Ptr<brunsli_BrunsliBitReader>) -> u16 {
        unimplemented!()
    }
}
impl brunsli_HuffmanDecodingDataImpl for Ptr<brunsli_HuffmanDecodingData> {
    fn ReadFromBitStream(
        &self,
        mut alphabet_size: usize,
        mut br: Ptr<brunsli_BrunsliBitReader>,
        arena: Option<Ptr<brunsli_Arena_brunsli_HuffmanCode_>>,
    ) -> bool {
        let mut arena: Ptr<brunsli_Arena_brunsli_HuffmanCode_> =
            arena.unwrap_or(Ptr::<brunsli_Arena_brunsli_HuffmanCode_>::null());
        let local_arena: Value<brunsli_Arena_brunsli_HuffmanCode_> =
            Rc::new(RefCell::new(<brunsli_Arena_brunsli_HuffmanCode_>::default()));
        if (arena).is_null() {
            arena = (local_arena.as_pointer());
        }
        if (alphabet_size > ((1 << kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize)) {
            return false;
        }
        let code_lengths: Value<Vec<u8>> =
            Rc::new(RefCell::new(vec![0_u8; alphabet_size as usize]));
        let mut simple_code_or_skip: u32 = ({ BrunsliBitReaderRead_126((br).clone(), 2_u32) });
        if (simple_code_or_skip == 1_u32) {
            {
                let __a0 =
                    ((1_u32 << kHuffmanTableBits_21.with(|rc| *rc.borrow())) as usize) as usize;
                (*(*self).with(|__s| __s.table_.clone()).borrow_mut())
                    .resize_with(__a0, || <brunsli_HuffmanCode>::default())
            };
            return ({
                ReadSimpleCode_219(
                    (alphabet_size as u16),
                    br,
                    ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<brunsli_HuffmanCode>),
                )
            });
        }
        let code_length_code_lengths: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8,
        ])));
        let mut space: i32 = 32;
        let mut num_codes: i32 = 0;
        thread_local!(
            static huff_220: Value<Box<[brunsli_HuffmanCode]>> = Rc::new(RefCell::new(Box::new([
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 0_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 4_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 3_u16,
                },
                brunsli_HuffmanCode {
                    bits: 3_u8,
                    value: 2_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 0_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 4_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 3_u16,
                },
                brunsli_HuffmanCode {
                    bits: 4_u8,
                    value: 1_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 0_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 4_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 3_u16,
                },
                brunsli_HuffmanCode {
                    bits: 3_u8,
                    value: 2_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 0_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 4_u16,
                },
                brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: 3_u16,
                },
                brunsli_HuffmanCode {
                    bits: 4_u8,
                    value: 5_u16,
                },
            ])));
        );
        let mut i: usize = (simple_code_or_skip as usize);
        'loop_: while (i < (kCodeLengthCodes_213.with(|rc| *rc.borrow()) as usize)) && (space > 0) {
            let mut code_len_idx: i32 = (({
                let __idx = (i) as usize;
                kCodeLengthCodeOrder_214.with(|rc| rc.borrow()[__idx])
            }) as i32);
            let mut p: Ptr<brunsli_HuffmanCode> =
                (huff_220.with(|v| v.as_pointer()) as Ptr<brunsli_HuffmanCode>);
            let mut v: u8 = 0_u8;
            p += ({ BrunsliBitReaderGet_124((br).clone(), 4_u32) });
            ({ BrunsliBitReaderDrop_125((br).clone(), (p.with(|__s| __s.bits) as u32)) });
            v = (p.with(|__s| __s.value) as u8);
            (*code_length_code_lengths.borrow_mut())[(code_len_idx) as usize] = v;
            if ((v as i32) != 0) {
                space = { ((space as u32).wrapping_sub((32_u32 >> (v as i32)))) as i32 };
                num_codes.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut ok: bool = ((num_codes == 1) || (space == 0))
            && ({
                ReadHuffmanCodeLengths_217(
                    (code_length_code_lengths.as_pointer() as Ptr<u8>),
                    alphabet_size,
                    ((code_lengths.as_pointer() as Ptr<u8>).offset(0_usize)),
                    (br).clone(),
                )
            });
        if (!(ok)) || (!({ BrunsliBitReaderIsHealthy_132((br).clone()) })) {
            return false;
        }
        let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16,
        ])));
        let mut i: usize = 0_usize;
        'loop_: while (i < alphabet_size) {
            (*counts.borrow_mut())
                [(elem!((code_lengths.as_pointer() as Ptr<u8>), i).read()) as usize]
                .prefix_inc();
            i.prefix_inc();
        }
        ({
            brunsli_Arena_brunsli_HuffmanCode_Impl::reserve(
                &arena,
                (alphabet_size).wrapping_add(376_usize),
            )
        });
        let mut table_size: u32 = ({
            BuildHuffmanTable_218(
                ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&arena) }),
                (kHuffmanTableBits_21.with(|rc| *rc.borrow()) as usize),
                ((code_lengths.as_pointer() as Ptr<u8>).offset(0_usize)),
                alphabet_size,
                ((counts.as_pointer() as Ptr<u16>).offset(0)),
            )
        });
        ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<Vec<brunsli_HuffmanCode>>).write({
            let __count = ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&arena) })
                .offset((table_size) as isize)
                .get_offset()
                - ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&arena) }).get_offset();
            PtrValueIter::new(
                &({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&arena) }),
                __count,
            )
            .map(|item| brunsli_HuffmanCode::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
        });
        return (table_size > 0_u32);
    }
    fn ReadSymbol(&self, mut br: Ptr<brunsli_BrunsliBitReader>) -> u16 {
        let mut n_bits: u32 = 0_u32;
        let mut table: Ptr<brunsli_HuffmanCode> =
            ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<brunsli_HuffmanCode>);
        table += ({
            BrunsliBitReaderGet_124((br).clone(), kHuffmanTableBits_21.with(|rc| *rc.borrow()))
        });
        n_bits = (table.with(|__s| __s.bits) as u32);
        if (n_bits > kHuffmanTableBits_21.with(|rc| *rc.borrow())) {
            ({
                BrunsliBitReaderDrop_125((br).clone(), kHuffmanTableBits_21.with(|rc| *rc.borrow()))
            });
            n_bits = { (n_bits).wrapping_sub(kHuffmanTableBits_21.with(|rc| *rc.borrow())) };
            let __rhs = (table.with(|__s| __s.value) as i32);
            table += __rhs;
            table += ({ BrunsliBitReaderGet_124((br).clone(), n_bits) });
        }
        ({ BrunsliBitReaderDrop_125((br).clone(), (table.with(|__s| __s.bits) as u32)) });
        return table.with(|__s| __s.value);
    }
}
pub trait brunsli_JPEGOutputImpl {
    fn Write(&self, buf: Ptr<u8>, len: usize) -> bool;
}
impl brunsli_JPEGOutputImpl for Ptr<brunsli_JPEGOutput> {
    fn Write(&self, mut buf: Ptr<u8>, mut len: usize) -> bool {
        if (len == 0_usize) {
            return true;
        }
        let mut bytes_written: usize = ({
            let _arg0: AnyPtr = (*self).with(|__s| __s.data.clone());
            (*self)
                .with(|__s| __s.cb.clone())
                .call(_arg0, (buf).clone(), len)
        });
        return (bytes_written == len);
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
    fn RemoveValue(&self, value: u8, mut code: Ptr<i32>, mut nbits: Ptr<i32>) -> bool {
        let value: Value<u8> = Rc::new(RefCell::new(value));
        let it: Value<Ptr<u8>> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>).offset(
                ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0
                            < ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                                .to_end()
                                .get_offset() as usize
                            && value_0.read() == (*value.borrow())
                    })
                    .unwrap_or(
                        ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                            .to_end()
                            .get_offset() as usize,
                    ) as isize,
            ),
        ));
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
pub trait brunsli_WordSourceImpl {
    fn GetNextWord(&self) -> u16;
    fn CanRead(&self, n: usize) -> bool;
}
impl brunsli_WordSourceImpl for Ptr<brunsli_WordSource> {
    fn GetNextWord(&self) -> u16 {
        let mut val: u16 = 0_u16;
        if ((*self).with(|__s| __s.pos_) < (*self).with(|__s| __s.len_)) {
            val = ({
                BrunsliUnalignedRead16_66(
                    ((*self)
                        .with(|__s| __s.data_.clone())
                        .offset(((*self).with(|__s| __s.pos_)) as isize)
                        as Ptr<u8>)
                        .to_any(),
                )
            });
        } else {
            field!((*self), error_).write(true);
        }
        field!((*self), pos_).write({ ((*self).with(|__s| __s.pos_)).wrapping_add(2_usize) });
        return val;
    }
    fn CanRead(&self, mut n: usize) -> bool {
        if (*self).with(|__s| __s.optimistic_) {
            return true;
        }
        let mut delta: usize = (2_usize).wrapping_mul(n);
        let mut projected_end: usize = ((*self).with(|__s| __s.pos_)).wrapping_add(delta);
        if (projected_end < (*self).with(|__s| __s.pos_)) {
            return false;
        }
        return (projected_end <= (*self).with(|__s| __s.len_));
    }
}
pub trait brunsli_internal_dec_BitWriterImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_BitWriter>,
    ) -> Ptr<brunsli_internal_dec_BitWriter>;
}
impl brunsli_internal_dec_BitWriterImpl for Ptr<brunsli_internal_dec_BitWriter> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_BitWriter>,
    ) -> Ptr<brunsli_internal_dec_BitWriter> {
        field!((*self), healthy).write({ { (*_a0.upgrade().deref()).healthy } });
        field!((*self), output).write({ { (*_a0.upgrade().deref()).output.clone() } });
        ({
            let _arg0: Ptr<brunsli_internal_dec_OutputChunk> = field_ptr!(_a0, chunk);
            brunsli_internal_dec_OutputChunkImpl::move_assign(&field_ptr!((*self), chunk), _arg0)
        });
        field!((*self), data).write({ { (*_a0.upgrade().deref()).data.clone() } });
        field!((*self), pos).write({ { (*_a0.upgrade().deref()).pos } });
        field!((*self), put_buffer).write({ { (*_a0.upgrade().deref()).put_buffer } });
        field!((*self), put_bits).write({ { (*_a0.upgrade().deref()).put_bits } });
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_EncodeScanStateImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_EncodeScanState>,
    ) -> Ptr<brunsli_internal_dec_EncodeScanState>;
}
impl brunsli_internal_dec_EncodeScanStateImpl for Ptr<brunsli_internal_dec_EncodeScanState> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_EncodeScanState>,
    ) -> Ptr<brunsli_internal_dec_EncodeScanState> {
        field!((*self), stage).write({ { (*_a0.upgrade().deref()).stage } });
        field!((*self), mcu_y).write({ { (*_a0.upgrade().deref()).mcu_y } });
        ({
            let _arg0: Ptr<brunsli_internal_dec_BitWriter> = field_ptr!(_a0, bw);
            brunsli_internal_dec_BitWriterImpl::move_assign(&field_ptr!((*self), bw), _arg0)
        });
        {
            ((array_field_ptr!((*self), last_dc_coeff)) as Ptr<i16>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, last_dc_coeff)) as Ptr<i16>).to_any(),
                    8_usize as usize,
                );
            ((array_field_ptr!((*self), last_dc_coeff)) as Ptr<i16>).to_any()
        };
        field!((*self), restarts_to_go).write({ { (*_a0.upgrade().deref()).restarts_to_go } });
        field!((*self), next_restart_marker)
            .write({ { (*_a0.upgrade().deref()).next_restart_marker } });
        field!((*self), block_scan_index).write({ { (*_a0.upgrade().deref()).block_scan_index } });
        field!((*self), coding_state).write({ { (*_a0.upgrade().deref()).coding_state.clone() } });
        field!((*self), extra_zero_runs_pos)
            .write({ { (*_a0.upgrade().deref()).extra_zero_runs_pos } });
        field!((*self), next_extra_zero_run_index)
            .write({ { (*_a0.upgrade().deref()).next_extra_zero_run_index } });
        field!((*self), next_reset_point_pos)
            .write({ { (*_a0.upgrade().deref()).next_reset_point_pos } });
        field!((*self), next_reset_point).write({ { (*_a0.upgrade().deref()).next_reset_point } });
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_HistogramDataStateImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_HistogramDataState>,
    ) -> Ptr<brunsli_internal_dec_HistogramDataState>;
}
impl brunsli_internal_dec_HistogramDataStateImpl for Ptr<brunsli_internal_dec_HistogramDataState> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_HistogramDataState>,
    ) -> Ptr<brunsli_internal_dec_HistogramDataState> {
        field!((*self), stage).write({ { (*_a0.upgrade().deref()).stage } });
        field!((*self), br).write({ { (*_a0.upgrade().deref()).br.clone() } });
        field!((*self), max_run_length_prefix)
            .write({ { (*_a0.upgrade().deref()).max_run_length_prefix } });
        (field_ptr!((*self), entropy) as Ptr<Option<Value<brunsli_HuffmanDecodingData>>>).write(
            field!(_a0, entropy)
                .with_mut(|__v: &mut Option<Value<brunsli_HuffmanDecodingData>>| __v.take()),
        );
        field!((*self), i).write({ { (*_a0.upgrade().deref()).i } });
        ((*self).with(|__s| __s.counts.as_pointer()) as Ptr<Vec<u32>>).write(std::mem::take(
            &mut (*{ (*_a0.upgrade().deref()).counts.clone() }.borrow_mut()),
        ));
        ({
            let _arg0: Ptr<brunsli_Arena_brunsli_HuffmanCode_> = field_ptr!(_a0, arena);
            brunsli_Arena_brunsli_HuffmanCode_Impl::move_assign(&field_ptr!((*self), arena), _arg0)
        });
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_InternalStateImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_InternalState>,
    ) -> Ptr<brunsli_internal_dec_InternalState>;
}
impl brunsli_internal_dec_InternalStateImpl for Ptr<brunsli_internal_dec_InternalState> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_InternalState>,
    ) -> Ptr<brunsli_internal_dec_InternalState> {
        field!((*self), ac_dc).write({ { (*_a0.upgrade().deref()).ac_dc.clone() } });
        field!((*self), section).write({ { (*_a0.upgrade().deref()).section.clone() } });
        field!((*self), header).write({ { (*_a0.upgrade().deref()).header.clone() } });
        field!((*self), fallback).write({ { (*_a0.upgrade().deref()).fallback.clone() } });
        field!((*self), section_header)
            .write({ { (*_a0.upgrade().deref()).section_header.clone() } });
        field!((*self), metadata).write({ { (*_a0.upgrade().deref()).metadata.clone() } });
        field!((*self), internals).write({ { (*_a0.upgrade().deref()).internals.clone() } });
        field!((*self), quant).write({ { (*_a0.upgrade().deref()).quant.clone() } });
        ({
            let _arg0: Ptr<brunsli_internal_dec_HistogramDataState> = field_ptr!(_a0, histogram);
            brunsli_internal_dec_HistogramDataStateImpl::move_assign(
                &field_ptr!((*self), histogram),
                _arg0,
            )
        });
        ((*self).with(|__s| __s.context_map_.as_pointer()) as Ptr<Vec<u8>>).write(std::mem::take(
            &mut (*{ (*_a0.upgrade().deref()).context_map_.clone() }.borrow_mut()),
        ));
        ((*self).with(|__s| __s.entropy_codes_.as_pointer()) as Ptr<Vec<brunsli_ANSDecodingData>>)
            .write(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).entropy_codes_.clone() }.borrow_mut()),
            ));
        ((*self).with(|__s| __s.block_state_.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).write(
            std::mem::take(&mut (*{ (*_a0.upgrade().deref()).block_state_.clone() }.borrow_mut())),
        );
        field!((*self), is_meta_warm).write({ { (*_a0.upgrade().deref()).is_meta_warm } });
        field!((*self), shallow_histograms)
            .write({ { (*_a0.upgrade().deref()).shallow_histograms } });
        field!((*self), num_contexts).write({ { (*_a0.upgrade().deref()).num_contexts } });
        field!((*self), num_histograms).write({ { (*_a0.upgrade().deref()).num_histograms } });
        field!((*self), subdecoders_initialized)
            .write({ { (*_a0.upgrade().deref()).subdecoders_initialized } });
        field!((*self), ans_decoder).write({ { (*_a0.upgrade().deref()).ans_decoder.clone() } });
        field!((*self), bit_reader).write({ { (*_a0.upgrade().deref()).bit_reader.clone() } });
        field!((*self), arith_decoder)
            .write({ { (*_a0.upgrade().deref()).arith_decoder.clone() } });
        field!((*self), result).write({ { (*_a0.upgrade().deref()).result } });
        field!((*self), last_stage).write({ { (*_a0.upgrade().deref()).last_stage } });
        field!((*self), buffer).write({ { (*_a0.upgrade().deref()).buffer.clone() } });
        ({
            let _arg0: Ptr<brunsli_internal_dec_SerializationState> =
                field_ptr!(_a0, serialization);
            brunsli_internal_dec_SerializationStateImpl::move_assign(
                &field_ptr!((*self), serialization),
                _arg0,
            )
        });
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_MetadataStateImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn CanFinish(&self) -> bool;
}
impl brunsli_internal_dec_MetadataStateImpl for Ptr<brunsli_internal_dec_MetadataState> {
    fn CanFinish(&self) -> bool {
        return ((*self).with(|__s| __s.stage)
            == (brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize))
            || ((*self).with(|__s| __s.stage)
                == (brunsli_internal_dec_MetadataState_Stage_READ_TAIL as usize));
    }
    fn destructor(&self) {
        if !(((*self).with(|__s| __s.brotli.clone())).is_null()) {
            unsafe {
                ::brotli_sys::BrotliDecoderDestroyInstance((*self).with(|__s| __s.brotli.clone()))
            };
            field!((*self), brotli).write(std::ptr::null_mut());
        }
    }
}
pub trait brunsli_internal_dec_OutputChunkImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_OutputChunk>,
    ) -> Ptr<brunsli_internal_dec_OutputChunk>;
}
impl brunsli_internal_dec_OutputChunkImpl for Ptr<brunsli_internal_dec_OutputChunk> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_OutputChunk>,
    ) -> Ptr<brunsli_internal_dec_OutputChunk> {
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next.clone() } });
        field!((*self), len).write({ { (*_a0.upgrade().deref()).len } });
        (field_ptr!((*self), buffer) as Ptr<Option<Value<Vec<u8>>>>)
            .write(field!(_a0, buffer).with_mut(|__v: &mut Option<Value<Vec<u8>>>| __v.take()));
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_SerializationStateImpl {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_SerializationState>,
    ) -> Ptr<brunsli_internal_dec_SerializationState>;
}
impl brunsli_internal_dec_SerializationStateImpl for Ptr<brunsli_internal_dec_SerializationState> {
    fn move_assign(
        &self,
        _a0: Ptr<brunsli_internal_dec_SerializationState>,
    ) -> Ptr<brunsli_internal_dec_SerializationState> {
        field!((*self), stage).write({ { (*_a0.upgrade().deref()).stage } });
        ((*self).with(|__s| __s.output_queue.as_pointer())
            as Ptr<Vec<brunsli_internal_dec_OutputChunk>>)
            .write(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).output_queue.clone() }.borrow_mut()),
            ));
        field!((*self), section_index).write({ { (*_a0.upgrade().deref()).section_index } });
        field!((*self), dht_index).write({ { (*_a0.upgrade().deref()).dht_index } });
        field!((*self), dqt_index).write({ { (*_a0.upgrade().deref()).dqt_index } });
        field!((*self), app_index).write({ { (*_a0.upgrade().deref()).app_index } });
        field!((*self), com_index).write({ { (*_a0.upgrade().deref()).com_index } });
        field!((*self), data_index).write({ { (*_a0.upgrade().deref()).data_index } });
        field!((*self), scan_index).write({ { (*_a0.upgrade().deref()).scan_index } });
        ((*self).with(|__s| __s.dc_huff_table.as_pointer()) as Ptr<Vec<brunsli_HuffmanCodeTable>>)
            .write(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).dc_huff_table.clone() }.borrow_mut()),
            ));
        ((*self).with(|__s| __s.ac_huff_table.as_pointer()) as Ptr<Vec<brunsli_HuffmanCodeTable>>)
            .write(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).ac_huff_table.clone() }.borrow_mut()),
            ));
        field!((*self), pad_bits).write({ { (*_a0.upgrade().deref()).pad_bits.clone() } });
        field!((*self), pad_bits_end).write({ { (*_a0.upgrade().deref()).pad_bits_end.clone() } });
        field!((*self), seen_dri_marker).write({ { (*_a0.upgrade().deref()).seen_dri_marker } });
        field!((*self), is_progressive).write({ { (*_a0.upgrade().deref()).is_progressive } });
        ({
            let _arg0: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!(_a0, scan_state);
            brunsli_internal_dec_EncodeScanStateImpl::move_assign(
                &field_ptr!((*self), scan_state),
                _arg0,
            )
        });
        return (*self).clone();
    }
}
pub trait brunsli_internal_dec_StateImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl brunsli_internal_dec_StateImpl for Ptr<brunsli_internal_dec_State> {
    fn destructor(&self) {}
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
    let _ = kBitMask_120.with(|_| ());
    let _ = kNumDirectCodes_135.with(|_| ());
    let _ = kCoeffAlphabetSize_136.with(|_| ());
    let _ = kKnownSectionTags_137.with(|_| ());
    let _ = kKnownHeaderVarintTags_138.with(|_| ());
    let _ = kBufferMaxReadAhead_199.with(|_| ());
    let _ = kLengthTree_210.with(|_| ());
    let _ = kLogCountTree_211.with(|_| ());
    let _ = kCodeLengthCodes_213.with(|_| ());
    let _ = kCodeLengthCodeOrder_214.with(|_| ());
    let _ = kDefaultCodeLength_215.with(|_| ());
    let _ = kCodeLengthRepeatCode_216.with(|_| ());
    let _ = kJpegPrecision_224.with(|_| ());
    let _ = kBitWriterChunkSize_225.with(|_| ());
}
