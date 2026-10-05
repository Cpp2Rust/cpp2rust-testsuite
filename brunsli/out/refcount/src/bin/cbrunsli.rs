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
        let __this: Value<brunsli_JPEGComponent> = Rc::new(RefCell::new(Self {
            id: 0,
            h_samp_factor: 1,
            v_samp_factor: 1,
            quant_idx: 0_u8,
            width_in_blocks: 0_u32,
            height_in_blocks: 0_u32,
            num_blocks: 0_u32,
            coeffs: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_JPEGComponent> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_JPEGData> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_JPEGData> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    let num_blocks: Value<u64> = Rc::new(RefCell::new(
        (((jpg.with(|__s| __s.width) as u64).wrapping_add(15_u64)) >> 3_u32)
            .wrapping_mul((((jpg.with(|__s| __s.height) as u64).wrapping_add(15_u64)) >> 3_u32)),
    ));
    return (((7_u64).wrapping_mul((*num_blocks.borrow())))
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
pub fn ValueMarker_28(tag: u8) -> u8 {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    return (((((*tag.borrow()) as i32) << 3)
        | (kBrunsliWiringTypeVarint_25.with(|rc| *rc.borrow()) as i32)) as u8);
}
pub fn SectionMarker_29(tag: u8) -> u8 {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    return (((((*tag.borrow()) as i32) << 3)
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
pub fn BrunsliUnalignedRead16_66(p: AnyPtr) -> u16 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let t: Value<u16> = Rc::new(RefCell::new(0_u16));
    {
        ((t.as_pointer()) as Ptr<u16>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u16>() as usize);
        ((t.as_pointer()) as Ptr<u16>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite16_67(p: AnyPtr, v: u16) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let v: Value<u16> = Rc::new(RefCell::new(v));
    {
        (*p.borrow()).memcpy(
            &((v.as_pointer()) as Ptr<u16>).to_any(),
            ::std::mem::size_of::<u16>() as usize,
        );
        (*p.borrow()).clone()
    };
}
pub fn BrunsliUnalignedRead32_68(p: AnyPtr) -> u32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let t: Value<u32> = Rc::new(RefCell::new(0_u32));
    {
        ((t.as_pointer()) as Ptr<u32>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u32>() as usize);
        ((t.as_pointer()) as Ptr<u32>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedRead64_69(p: AnyPtr) -> u64 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let t: Value<u64> = Rc::new(RefCell::new(0_u64));
    {
        ((t.as_pointer()) as Ptr<u64>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u64>() as usize);
        ((t.as_pointer()) as Ptr<u64>).to_any()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite64_70(p: AnyPtr, v: u64) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let v: Value<u64> = Rc::new(RefCell::new(v));
    {
        (*p.borrow()).memcpy(
            &((v.as_pointer()) as Ptr<u64>).to_any(),
            ::std::mem::size_of::<u64>() as usize,
        );
        (*p.borrow()).clone()
    };
}
pub fn Append_71(dst: Ptr<Vec<u8>>, begin: Ptr<u8>, end: Ptr<u8>) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new(begin));
    let end: Value<Ptr<u8>> = Rc::new(RefCell::new(end));
    {
        let start_idx = (Ptr::<Vec<u8>>::decay(&(*dst.borrow())) as Ptr<u8>)
            .to_end()
            .get_offset();
        let count = (*end.borrow()).get_offset() - (*begin.borrow()).get_offset();
        let temp_vec: Vec<u8> = PtrValueIter::new(&(*begin.borrow()), count).collect();
        ((*dst.borrow()).clone() as Ptr<Vec<u8>>).with_mut(|v: &mut Vec<u8>| {
            v.splice(start_idx..start_idx, temp_vec);
        });
        ((*dst.borrow()).clone() as Ptr<Vec<u8>>) + start_idx
    };
}
pub fn Append_72(dst: Ptr<Vec<u8>>, begin: Ptr<u8>, length: usize) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new(begin));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    ({
        let _begin: Ptr<u8> = (*begin.borrow()).clone();
        let _end: Ptr<u8> = (*begin.borrow()).offset((*length.borrow()) as isize);
        Append_71((*dst.borrow()).clone(), _begin, _end)
    });
}
pub fn Append_73(dst: Ptr<Vec<u8>>, src: Ptr<Vec<u8>>) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    ({
        let _begin: Ptr<u8> = (Ptr::<Vec<u8>>::decay(&(src)) as Ptr<u8>);
        let _length: usize = (*src.upgrade().deref()).len();
        Append_72((*dst.borrow()).clone(), _begin, _length)
    });
}
pub fn Log2FloorNonZero_74(n: u32) -> i32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return (31 ^ (*n.borrow()).leading_zeros() as i32);
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
pub fn FastDivide_78(numerator: u32, denominator: u8) -> u8 {
    let numerator: Value<u32> = Rc::new(RefCell::new(numerator));
    let denominator: Value<u8> = Rc::new(RefCell::new(denominator));
    let result: Value<u32> = Rc::new(RefCell::new(
        (((*numerator.borrow()).wrapping_mul(
            (({
                let __idx = (*denominator.borrow()) as usize;
                kDivLut17_77.with(|rc| rc.borrow()[__idx])
            }) as u32),
        )) >> 17),
    ));
    if !((*result.borrow()) < 256_u32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"context.cc"),
                55,
                Ptr::<i8>::from_string_literal(b"FastDivide"),
            )
        });
        'loop_: while true {}
    };
    return ((*result.borrow()) as u8);
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
        let __this: Value<brunsli_Prob> = Rc::new(RefCell::new(Self {
            prob8: kInitProb_80.with(|rc| *rc.borrow()),
            total: kInitProbCount_81.with(|rc| *rc.borrow()),
            count: (((kInitProb_80.with(|rc| *rc.borrow()) as i32)
                * (kInitProbCount_81.with(|rc| *rc.borrow()) as i32)) as u16),
        }));
        let this: Ptr<brunsli_Prob> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
pub fn ZeroDensityContext_96(nonzeros_left: usize, k: usize, bits: usize) -> u16 {
    let nonzeros_left: Value<usize> = Rc::new(RefCell::new(nonzeros_left));
    let k: Value<usize> = Rc::new(RefCell::new(k));
    let bits: Value<usize> = Rc::new(RefCell::new(bits));
    return (((({
        let __idx = (*bits.borrow()) as usize;
        kNumNonzeroContext_93.with(|rc| rc.borrow()[__idx].clone())
    })
    .borrow()[(*nonzeros_left.borrow()) as usize] as i32)
        + (({
            let __idx = (*bits.borrow()) as usize;
            kFreqContext_92.with(|rc| rc.borrow()[__idx].clone())
        })
        .borrow()[(*k.borrow()) as usize] as i32)) as u16);
}
pub fn WeightedAverageContextDC_97(vals: Ptr<i32>, x: i32) -> i32 {
    let vals: Value<Ptr<i32>> = Rc::new(RefCell::new(vals));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((((1 + (elem!((*vals.borrow()), ((*x.borrow()) - 2)).read()))
            + (elem!((*vals.borrow()), ((*x.borrow()) - 1)).read()))
            + (elem!((*vals.borrow()), (*x.borrow())).read()))
            + (elem!((*vals.borrow()), ((*x.borrow()) + 1)).read())),
    ));
    if (((*sum.borrow()) >> kMaxAverageContext_82.with(|rc| *rc.borrow())) != 0) {
        return (kMaxAverageContext_82.with(|rc| *rc.borrow()) as i32);
    }
    return ({ Log2FloorNonZero_74(((*sum.borrow()) as u32)) });
}
pub fn WeightedAverageContext_98(vals: Ptr<i32>, prev_row_delta: i32) -> i32 {
    let vals: Value<Ptr<i32>> = Rc::new(RefCell::new(vals));
    let prev_row_delta: Value<i32> = Rc::new(RefCell::new(prev_row_delta));
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((({
            ({ (4 + (elem!((*vals.borrow()), 0).read())) } + {
                (({ (elem!((*vals.borrow()), -kDCTBlockSize_3.with(|rc| *rc.borrow())).read()) }
                    + { (elem!((*vals.borrow()), (*prev_row_delta.borrow())).read()) })
                    * 2)
            })
        } + {
            (elem!(
                (*vals.borrow()),
                (-2_i32 * kDCTBlockSize_3.with(|rc| *rc.borrow()))
            )
            .read())
        }) + (elem!(
            (*vals.borrow()),
            ((*prev_row_delta.borrow()) - kDCTBlockSize_3.with(|rc| *rc.borrow()))
        )
        .read()))
            + (elem!(
                (*vals.borrow()),
                ((*prev_row_delta.borrow()) + kDCTBlockSize_3.with(|rc| *rc.borrow()))
            )
            .read())),
    ));
    if (((*sum.borrow())
        >> ((kMaxAverageContext_82.with(|rc| *rc.borrow())).wrapping_add(2_usize)))
        != 0)
    {
        return (kMaxAverageContext_82.with(|rc| *rc.borrow()) as i32);
    }
    return (({ Log2FloorNonZero_74(((*sum.borrow()) as u32)) }) - 2);
}
thread_local!(
    pub static kACPredictPrecisionBits_99: Value<i32> = Rc::new(RefCell::new(13));
);
thread_local!(
    pub static kACPredictPrecision_100: Value<i32> = Rc::new(RefCell::new(
        (1 << kACPredictPrecisionBits_99.with(|rc| *rc.borrow())),
    ));
);
pub fn ACPredictContext_101(p: i64, avg_ctx: Ptr<usize>, sgn: Ptr<usize>) {
    let p: Value<i64> = Rc::new(RefCell::new(p));
    let avg_ctx: Value<Ptr<usize>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<usize>> = Rc::new(RefCell::new(sgn));
    let multiplier: Value<i32> = Rc::new(RefCell::new(0_i32));
    if ((*p.borrow()) >= 0_i64) {
        (*multiplier.borrow_mut()) = 1;
    } else {
        (*multiplier.borrow_mut()) = -1_i32;
        (*p.borrow_mut()) = { -(*p.borrow()) };
    }
    let ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*p.borrow()) >= ((1_u32 << kMaxAverageContext_82.with(|rc| *rc.borrow())) as i64)) {
        (*ctx.borrow_mut()) = kMaxAverageContext_82.with(|rc| *rc.borrow());
    } else {
        (*ctx.borrow_mut()) = (({
            Log2FloorNonZero_74(((2_u32).wrapping_mul(((*p.borrow()) as u32))).wrapping_add(1_u32))
        }) as usize);
    }
    (*avg_ctx.borrow()).write({ (*ctx.borrow()) });
    (*sgn.borrow()).write({
        (kMaxAverageContext_82.with(|rc| *rc.borrow()))
            .wrapping_add(((*multiplier.borrow()) as usize).wrapping_mul((*ctx.borrow())))
    });
}
pub fn ACPredictContextCol_102(
    prev: Ptr<i16>,
    cur: Ptr<i16>,
    mult: Ptr<i32>,
    avg_ctx: Ptr<usize>,
    sgn: Ptr<usize>,
) {
    let prev: Value<Ptr<i16>> = Rc::new(RefCell::new(prev));
    let cur: Value<Ptr<i16>> = Rc::new(RefCell::new(cur));
    let mult: Value<Ptr<i32>> = Rc::new(RefCell::new(mult));
    let avg_ctx: Value<Ptr<usize>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<usize>> = Rc::new(RefCell::new(sgn));
    let terms: Value<Box<[i16]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_i16).collect::<Box<[i16]>>()));
    (*terms.borrow_mut())[(0) as usize] = 0_i16;
    (*terms.borrow_mut())[(1) as usize] = {
        (({ ((elem!((*cur.borrow()), 1).read()) as i32) } + {
            ((elem!((*prev.borrow()), 1).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(2) as usize] = {
        (({ ((elem!((*cur.borrow()), 2).read()) as i32) } - {
            ((elem!((*prev.borrow()), 2).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(3) as usize] = {
        (({ ((elem!((*cur.borrow()), 3).read()) as i32) } + {
            ((elem!((*prev.borrow()), 3).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(4) as usize] = {
        (({ ((elem!((*cur.borrow()), 4).read()) as i32) } - {
            ((elem!((*prev.borrow()), 4).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(5) as usize] = {
        (({ ((elem!((*cur.borrow()), 5).read()) as i32) } + {
            ((elem!((*prev.borrow()), 5).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(6) as usize] = {
        (({ ((elem!((*cur.borrow()), 6).read()) as i32) } - {
            ((elem!((*prev.borrow()), 6).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(7) as usize] = {
        (({ ((elem!((*cur.borrow()), 7).read()) as i32) } + {
            ((elem!((*prev.borrow()), 7).read()) as i32)
        }) as i16)
    };
    let delta: Value<i64> = Rc::new(RefCell::new(
        (((((((({ ((*terms.borrow())[(0) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 0).read()) as i64)
        }) + ({ ((*terms.borrow())[(1) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 1).read()) as i64)
        })) + ({ ((*terms.borrow())[(2) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 2).read()) as i64)
        })) + ({ ((*terms.borrow())[(3) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 3).read()) as i64)
        })) + ({ ((*terms.borrow())[(4) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 4).read()) as i64)
        })) + ({ ((*terms.borrow())[(5) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 5).read()) as i64)
        })) + ({ ((*terms.borrow())[(6) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 6).read()) as i64)
        })) + ({ ((*terms.borrow())[(7) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 7).read()) as i64)
        })),
    ));
    ({
        ACPredictContext_101(
            ({ ((elem!((*prev.borrow()), 0).read()) as i64) } - {
                ((*delta.borrow()) / (kACPredictPrecision_100.with(|rc| *rc.borrow()) as i64))
            }),
            (*avg_ctx.borrow()).clone(),
            (*sgn.borrow()).clone(),
        )
    });
}
pub fn ACPredictContextRow_103(
    prev: Ptr<i16>,
    cur: Ptr<i16>,
    mult: Ptr<i32>,
    avg_ctx: Ptr<usize>,
    sgn: Ptr<usize>,
) {
    let prev: Value<Ptr<i16>> = Rc::new(RefCell::new(prev));
    let cur: Value<Ptr<i16>> = Rc::new(RefCell::new(cur));
    let mult: Value<Ptr<i32>> = Rc::new(RefCell::new(mult));
    let avg_ctx: Value<Ptr<usize>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<usize>> = Rc::new(RefCell::new(sgn));
    let terms: Value<Box<[i16]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_i16).collect::<Box<[i16]>>()));
    (*terms.borrow_mut())[(0) as usize] = 0_i16;
    (*terms.borrow_mut())[(1) as usize] = {
        (({ ((elem!((*cur.borrow()), 8).read()) as i32) } + {
            ((elem!((*prev.borrow()), 8).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(2) as usize] = {
        (({ ((elem!((*cur.borrow()), 16).read()) as i32) } - {
            ((elem!((*prev.borrow()), 16).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(3) as usize] = {
        (({ ((elem!((*cur.borrow()), 24).read()) as i32) } + {
            ((elem!((*prev.borrow()), 24).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(4) as usize] = {
        (({ ((elem!((*cur.borrow()), 32).read()) as i32) } - {
            ((elem!((*prev.borrow()), 32).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(5) as usize] = {
        (({ ((elem!((*cur.borrow()), 40).read()) as i32) } + {
            ((elem!((*prev.borrow()), 40).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(6) as usize] = {
        (({ ((elem!((*cur.borrow()), 48).read()) as i32) } - {
            ((elem!((*prev.borrow()), 48).read()) as i32)
        }) as i16)
    };
    (*terms.borrow_mut())[(7) as usize] = {
        (({ ((elem!((*cur.borrow()), 56).read()) as i32) } + {
            ((elem!((*prev.borrow()), 56).read()) as i32)
        }) as i16)
    };
    let delta: Value<i64> = Rc::new(RefCell::new(
        (((((((({ ((*terms.borrow())[(0) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 0).read()) as i64)
        }) + ({ ((*terms.borrow())[(1) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 1).read()) as i64)
        })) + ({ ((*terms.borrow())[(2) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 2).read()) as i64)
        })) + ({ ((*terms.borrow())[(3) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 3).read()) as i64)
        })) + ({ ((*terms.borrow())[(4) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 4).read()) as i64)
        })) + ({ ((*terms.borrow())[(5) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 5).read()) as i64)
        })) + ({ ((*terms.borrow())[(6) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 6).read()) as i64)
        })) + ({ ((*terms.borrow())[(7) as usize] as i64) } * {
            ((elem!((*mult.borrow()), 7).read()) as i64)
        })),
    ));
    ({
        ACPredictContext_101(
            ({ ((elem!((*prev.borrow()), 0).read()) as i64) } - {
                ((*delta.borrow()) / (kACPredictPrecision_100.with(|rc| *rc.borrow()) as i64))
            }),
            (*avg_ctx.borrow()).clone(),
            (*sgn.borrow()).clone(),
        )
    });
}
pub fn NumNonzerosContext_104(prev: Ptr<u8>, x: i32, y: i32) -> u8 {
    let prev: Value<Ptr<u8>> = Rc::new(RefCell::new(prev));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    let prediction: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*y.borrow()) == 0) {
        if ((*x.borrow()) == 0) {
            (*prediction.borrow_mut()) = 0_usize;
        } else {
            (*prediction.borrow_mut()) =
                ((elem!((*prev.borrow()), ((*x.borrow()) - 1)).read()) as usize);
        }
    } else if ((*x.borrow()) == 0) {
        (*prediction.borrow_mut()) = ((elem!((*prev.borrow()), (*x.borrow())).read()) as usize);
    } else {
        (*prediction.borrow_mut()) = ((((((elem!((*prev.borrow()), ((*x.borrow()) - 1)).read())
            as i32)
            + ((elem!((*prev.borrow()), (*x.borrow())).read()) as i32))
            + 1)
            / 2) as usize);
    }
    if !((*prediction.borrow()) <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"context.cc"),
                305,
                Ptr::<i8>::from_string_literal(b"NumNonzerosContext"),
            )
        });
        'loop_: while true {}
    };
    return (((*prediction.borrow()).wrapping_div(kNumNonZeroQuant_86.with(|rc| *rc.borrow())))
        as u8);
}
thread_local!(
    pub static kNumIsEmptyBlockContexts_105: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn IsEmptyBlockContext_106(prev: Ptr<i32>, x: i32) -> i32 {
    let prev: Value<Ptr<i32>> = Rc::new(RefCell::new(prev));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((elem!((*prev.borrow()), ((*x.borrow()) - 1)).read())
        + (elem!((*prev.borrow()), (*x.borrow())).read()));
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
    pub fn SizeInBytes(w: i32) -> usize {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        return (((((4 + ((10 + (3 * (*w.borrow()))) * kDCTBlockSize_3.with(|rc| *rc.borrow())))
            + (2 * (*w.borrow()))) as usize)
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
pub fn ComputeACPredictMultipliers_109(quant: Ptr<i32>, mult_row: Ptr<i32>, mult_col: Ptr<i32>) {
    let quant: Value<Ptr<i32>> = Rc::new(RefCell::new(quant));
    let mult_row: Value<Ptr<i32>> = Rc::new(RefCell::new(mult_row));
    let mult_col: Value<Ptr<i32>> = Rc::new(RefCell::new(mult_col));
    let y: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*y.borrow()) < 8_usize) {
        let x: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*x.borrow()) < 8_usize) {
            elem!(
                (*mult_row.borrow()),
                (*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow())))
            )
            .write({
                ({
                    ({
                        (elem!(
                            (*quant.borrow()),
                            (*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow())))
                        )
                        .read())
                    } * { kSqrt2FixedPoint_108.with(|rc| *rc.borrow()) })
                } / { (elem!((*quant.borrow()), (*y.borrow()).wrapping_mul(8_usize)).read()) })
            });
            elem!(
                (*mult_col.borrow()),
                ((*x.borrow()).wrapping_mul(8_usize)).wrapping_add((*y.borrow()))
            )
            .write({
                ({
                    ({
                        (elem!(
                            (*quant.borrow()),
                            (*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow())))
                        )
                        .read())
                    } * { kSqrt2FixedPoint_108.with(|rc| *rc.borrow()) })
                } / { (elem!((*quant.borrow()), (*x.borrow())).read()) })
            });
            (*x.borrow_mut()).prefix_inc();
        }
        (*y.borrow_mut()).prefix_inc();
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
        let __this: Value<brunsli_PermutationCoder> = Rc::new(RefCell::new(Self {
            values_: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_PermutationCoder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for brunsli_PermutationCoder {
    fn default() -> Self {
        { brunsli_PermutationCoder::new() }
    }
}
pub fn ComputeLehmerCode_112(sigma: Ptr<u32>, len: usize, code: Ptr<u32>) {
    let sigma: Value<Ptr<u32>> = Rc::new(RefCell::new(sigma));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let code: Value<Ptr<u32>> = Rc::new(RefCell::new(code));
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(*len.borrow()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let __rhs = ((*i.borrow()) as u32);
        elem!((items.as_pointer() as Ptr<u32>), (*i.borrow())).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let it: Value<Ptr<u32>> = Rc::new(RefCell::new(
            (items.as_pointer() as Ptr<u32>).offset(
                (items.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize
                            && value_0.read() == (elem!((*sigma.borrow()), (*i.borrow())).read())
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
        elem!((*code.borrow()), (*i.borrow())).write(__rhs);
        {
            let idx = (*it.borrow()).get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>).decay()
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn DecodeLehmerCode_113(code: Ptr<u32>, len: usize, sigma: Ptr<u32>) -> bool {
    let code: Value<Ptr<u32>> = Rc::new(RefCell::new(code));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let sigma: Value<Ptr<u32>> = Rc::new(RefCell::new(sigma));
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(*len.borrow()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let __rhs = ((*i.borrow()) as u32);
        elem!((items.as_pointer() as Ptr<u32>), (*i.borrow())).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let index: Value<u32> = Rc::new(RefCell::new(
            (elem!((*code.borrow()), (*i.borrow())).read()),
        ));
        if (((*index.borrow()) as usize) >= (*items.borrow()).len()) {
            return false;
        }
        let value: Value<u32> = Rc::new(RefCell::new(
            (elem!(
                (items.as_pointer() as Ptr<u32>),
                ((*index.borrow()) as usize)
            )
            .read()),
        ));
        {
            let idx = (items.as_pointer() as Ptr<u32>)
                .offset(((*index.borrow()) as i64) as isize)
                .get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>).decay()
        };
        elem!((*sigma.borrow()), (*i.borrow())).write({ (*value.borrow()) });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn BrunsliDumpAndAbort_79(f: Ptr<i8>, l: i32, fn_: Ptr<i8>) {
    let f: Value<Ptr<i8>> = Rc::new(RefCell::new(f));
    let l: Value<i32> = Rc::new(RefCell::new(l));
    let fn_: Value<Ptr<i8>> = Rc::new(RefCell::new(fn_));
    eprintln!("{}:{} ({})", (*f.borrow()), (*l.borrow()), (*fn_.borrow()));
    0;
    std::process::abort();
}
pub fn AdaptiveMedian_114(w: i32, n: i32, nw: i32) -> i32 {
    let w: Value<i32> = Rc::new(RefCell::new(w));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let nw: Value<i32> = Rc::new(RefCell::new(nw));
    let mx: Value<i32> = Rc::new(RefCell::new(if ((*w.borrow()) > (*n.borrow())) {
        (*w.borrow())
    } else {
        (*n.borrow())
    }));
    let mn: Value<i32> = Rc::new(RefCell::new(
        (((*w.borrow()) + (*n.borrow())) - (*mx.borrow())),
    ));
    if ((*nw.borrow()) > (*mx.borrow())) {
        return (*mn.borrow());
    } else if ((*nw.borrow()) < (*mn.borrow())) {
        return (*mx.borrow());
    } else {
        return (((*n.borrow()) + (*w.borrow())) - (*nw.borrow()));
    }
    panic!("ub: non-void function does not return a value")
}
pub fn PredictWithAdaptiveMedian_115(coeffs: Ptr<i16>, x: i32, y: i32, stride: i32) -> i32 {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    let stride: Value<i32> = Rc::new(RefCell::new(stride));
    let offset1: Value<i32> = Rc::new(RefCell::new(-kDCTBlockSize_3.with(|rc| *rc.borrow())));
    let offset2: Value<i32> = Rc::new(RefCell::new(-(*stride.borrow())));
    let offset3: Value<i32> = Rc::new(RefCell::new(((*offset2.borrow()) + (*offset1.borrow()))));
    if ((*y.borrow()) != 0) {
        if ((*x.borrow()) != 0) {
            return ({
                let _w: i32 = ((elem!((*coeffs.borrow()), (*offset1.borrow())).read()) as i32);
                let _n: i32 = ((elem!((*coeffs.borrow()), (*offset2.borrow())).read()) as i32);
                let _nw: i32 = ((elem!((*coeffs.borrow()), (*offset3.borrow())).read()) as i32);
                AdaptiveMedian_114(_w, _n, _nw)
            });
        } else {
            return ((elem!((*coeffs.borrow()), (*offset2.borrow())).read()) as i32);
        }
    } else {
        return if ((*x.borrow()) != 0) {
            ((elem!((*coeffs.borrow()), (*offset1.borrow())).read()) as i32)
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
pub fn FillQuantMatrix_118(is_chroma: bool, q: u32, dst: Ptr<u8>) {
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let q: Value<u32> = Rc::new(RefCell::new(q));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    if !(((*q.borrow()) >= 0_u32)
        && (((*q.borrow()) as usize) < kQFactorLimit_117.with(|rc| *rc.borrow())))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"quant_matrix.cc"),
                18,
                Ptr::<i8>::from_string_literal(b"FillQuantMatrix"),
            )
        });
        'loop_: while true {}
    };
    let in_: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((kDefaultQuantMatrix_12.with(|v| v.as_pointer()) as Ptr<Value<Box<[u8]>>>)
            .offset((*is_chroma.borrow()))
            .read()
            .as_pointer()) as Ptr<u8>),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let v: Value<u32> = Rc::new(RefCell::new(
            (((((elem!((*in_.borrow()), (*i.borrow())).read()) as u32)
                .wrapping_mul((*q.borrow())))
            .wrapping_add(32_u32))
                >> 6),
        ));
        elem!((*dst.borrow()), (*i.borrow())).write({
            (if ((*v.borrow()) < 1_u32) {
                1_u32
            } else {
                if ((*v.borrow()) > 255_u32) {
                    255_u32
                } else {
                    (*v.borrow())
                }
            } as u8)
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn FindBestMatrix_119(src: Ptr<i32>, is_chroma: bool, dst: Ptr<u8>) -> u32 {
    let src: Value<Ptr<i32>> = Rc::new(RefCell::new(src));
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let best_q: Value<u32> = Rc::new(RefCell::new(0_u32));
    let kMaxDiffCost: Value<usize> = Rc::new(RefCell::new(33_usize));
    let kWorstLen: Value<usize> = Rc::new(RefCell::new(
        ((kDCTBlockSize_3.with(|rc| *rc.borrow()) + 1) as usize)
            .wrapping_mul((((*kMaxDiffCost.borrow()).wrapping_add(1_usize)) as usize)),
    ));
    let best_len: Value<usize> = Rc::new(RefCell::new((*kWorstLen.borrow())));
    let q: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*q.borrow()) as usize) < kQFactorLimit_117.with(|rc| *rc.borrow())) {
        ({
            FillQuantMatrix_118(
                (*is_chroma.borrow()),
                (*q.borrow()),
                (*dst.borrow()).clone(),
            )
        });
        let last_diff: Value<i32> = Rc::new(RefCell::new(0));
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let j: Value<i32> = Rc::new(RefCell::new(
                (({
                    let __idx = (*k.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as i32),
            ));
            let new_diff: Value<i32> = Rc::new(RefCell::new(
                ({ (elem!((*src.borrow()), (*j.borrow())).read()) } - {
                    ((elem!((*dst.borrow()), (*j.borrow())).read()) as i32)
                }),
            ));
            let diff: Value<i32> =
                Rc::new(RefCell::new(((*new_diff.borrow()) - (*last_diff.borrow()))));
            (*last_diff.borrow_mut()) = (*new_diff.borrow());
            if ((*diff.borrow()) != 0) {
                (*len.borrow_mut()) = { (*len.borrow()).wrapping_add(1_usize) };
                if ((*diff.borrow()) < 0) {
                    (*diff.borrow_mut()) = { -(*diff.borrow()) };
                }
                (*diff.borrow_mut()) -= 1;
                if ((*diff.borrow()) == 0) {
                    (*len.borrow_mut()).postfix_inc();
                } else if ((*diff.borrow()) > 65535) {
                    (*len.borrow_mut()) = (*kWorstLen.borrow());
                    break;
                } else {
                    let diff_len: Value<u32> = Rc::new(RefCell::new(
                        ((({ Log2FloorNonZero_74(((*diff.borrow()) as u32)) }) + 1) as u32),
                    ));
                    if ((*diff_len.borrow()) == 16_u32) {
                        (*diff_len.borrow_mut()).postfix_dec();
                    }
                    (*len.borrow_mut()) = {
                        (*len.borrow()).wrapping_add(
                            ((((2_u32).wrapping_mul((*diff_len.borrow()))).wrapping_add(1_u32))
                                as usize),
                        )
                    };
                }
            }
            (*k.borrow_mut()).prefix_inc();
        }
        if ((*len.borrow()) < (*best_len.borrow())) {
            (*best_len.borrow_mut()) = (*len.borrow());
            (*best_q.borrow_mut()) = (*q.borrow());
        }
        (*q.borrow_mut()).prefix_inc();
    }
    ({
        FillQuantMatrix_118(
            (*is_chroma.borrow()),
            (*best_q.borrow()),
            (*dst.borrow()).clone(),
        )
    });
    return (*best_q.borrow());
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
pub fn WriteBits_120(n_bits: usize, bits: u64, storage: Ptr<brunsli_Storage>) {
    let n_bits: Value<usize> = Rc::new(RefCell::new(n_bits));
    let bits: Value<u64> = Rc::new(RefCell::new(bits));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if true {
    } else {
        write!(
            libcc2rs::cerr(),
            "WriteBits {:2} {:16x} {:10}\n",
            (*n_bits.borrow()),
            (*bits.borrow()),
            (*storage.borrow()).with(|__s| __s.pos),
        );
    }
    if !(((*bits.borrow()) >> (*n_bits.borrow())) == 0_u64) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                58,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    if !((*n_bits.borrow()) <= 56_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                59,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    if !({
        ((((*storage.borrow()).with(|__s| __s.pos)).wrapping_add((*n_bits.borrow()))) >> 3)
            .wrapping_add(7_usize)
    } < { (*storage.borrow()).with(|__s| __s.length) })
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"ans_encode.cc"),
                61,
                Ptr::<i8>::from_string_literal(b"WriteBits"),
            )
        });
        'loop_: while true {}
    };
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*storage.borrow())
            .with(|__s| __s.data.clone())
            .offset(((*storage.borrow()).with(|__s| __s.pos) >> 3) as isize),
    ));
    let v: Value<u64> = Rc::new(RefCell::new((((*p.borrow()).read()) as u64)));
    (*v.borrow_mut()) |=
        ({ (*bits.borrow()) } << { ((*storage.borrow()).with(|__s| __s.pos) & 7_usize) });
    ({ BrunsliUnalignedWrite64_70((*p.borrow()).to_any(), (*v.borrow())) });
    field!((*storage.borrow()), pos)
        .write({ ((*storage.borrow()).with(|__s| __s.pos)).wrapping_add((*n_bits.borrow())) });
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
        let __this: Value<brunsli_ANSCoder> = Rc::new(RefCell::new(Self {
            state_: (19_u32 << 16),
        }));
        let this: Ptr<brunsli_ANSCoder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    counts: Ptr<i32>,
    alphabet_size: i32,
    info: Ptr<brunsli_ANSEncSymbolInfo>,
) {
    let counts: Value<Ptr<i32>> = Rc::new(RefCell::new(counts));
    let alphabet_size: Value<i32> = Rc::new(RefCell::new(alphabet_size));
    let info: Value<Ptr<brunsli_ANSEncSymbolInfo>> = Rc::new(RefCell::new(info));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*s.borrow()) < (*alphabet_size.borrow())) {
        let freq: Value<u32> = Rc::new(RefCell::new(
            ((elem!((*counts.borrow()), (*s.borrow())).read()) as u32),
        ));
        field!(elem!((*info.borrow()), (*s.borrow())), freq_)
            .write({ ((elem!((*counts.borrow()), (*s.borrow())).read()) as u16) });
        field!(elem!((*info.borrow()), (*s.borrow())), start_).write(((*total.borrow()) as u16));
        (*total.borrow_mut()) =
            { (((*total.borrow()) as u32).wrapping_add((*freq.borrow()))) as i32 };
        (*s.borrow_mut()).prefix_inc();
    }
}
pub fn BuildAndStoreANSEncodingData_123(
    histogram: Ptr<i32>,
    table: Ptr<brunsli_ANSTable>,
    storage: Ptr<brunsli_Storage>,
) {
    let histogram: Value<Ptr<i32>> = Rc::new(RefCell::new(histogram));
    let table: Value<Ptr<brunsli_ANSTable>> = Rc::new(RefCell::new(table));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let num_symbols: Value<i32> = Rc::new(RefCell::new(0_i32));
    let symbols: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
    let counts: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = (*histogram.borrow()).offset((18) as isize).get_offset()
            - (*histogram.borrow()).get_offset();
        PtrValueIter::new(&(*histogram.borrow()), __count).collect::<Vec<_>>()
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
            (array_field_ptr!((*table.borrow()), info_) as Ptr<brunsli_ANSEncSymbolInfo>),
        )
    });
    ({
        EncodeCounts_125(
            ((counts.as_pointer() as Ptr<i32>).offset(0_usize)),
            (*omit_pos.borrow()),
            (*num_symbols.borrow()),
            (symbols.as_pointer() as Ptr<i32>),
            (*storage.borrow()).clone(),
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
pub fn FastLog2_127(v: i32) -> f64 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    if ((*v.borrow())
        < (((::std::mem::size_of::<[f32; 256]>() as usize)
            .wrapping_div((::std::mem::size_of::<f32>() as usize))) as i32))
    {
        return (({
            let __idx = (*v.borrow()) as usize;
            kLog2Table_126.with(|rc| rc.borrow()[__idx])
        }) as f64);
    }
    return ((*v.borrow()) as f64).log2();
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
pub fn ClusterCostDiff_129(size_a: i32, size_b: i32) -> f64 {
    let size_a: Value<i32> = Rc::new(RefCell::new(size_a));
    let size_b: Value<i32> = Rc::new(RefCell::new(size_b));
    let size_c: Value<i32> = Rc::new(RefCell::new(((*size_a.borrow()) + (*size_b.borrow()))));
    return (((((*size_a.borrow()) as f64) * ({ FastLog2_127((*size_a.borrow())) }))
        + (((*size_b.borrow()) as f64) * ({ FastLog2_127((*size_b.borrow())) })))
        - (((*size_c.borrow()) as f64) * ({ FastLog2_127((*size_c.borrow())) })));
}
pub fn PopulationCost_130(h: Ptr<brunsli_internal_enc_Histogram>) -> f64 {
    return ({
        let _data: Ptr<i32> = ((array_field_ptr!(h, data_) as Ptr<i32>).offset((0) as isize));
        let _total_count: i32 = h.with(|__s| __s.total_count_);
        PopulationCost_131(_data, _total_count)
    });
}
pub fn CompareAndPushToQueue_132(
    out: Ptr<brunsli_internal_enc_Histogram>,
    cluster_size: Ptr<i32>,
    idx1: i32,
    idx2: i32,
    pairs: Ptr<Vec<brunsli_HistogramPair>>,
) {
    let out: Value<Ptr<brunsli_internal_enc_Histogram>> = Rc::new(RefCell::new(out));
    let cluster_size: Value<Ptr<i32>> = Rc::new(RefCell::new(cluster_size));
    let idx1: Value<i32> = Rc::new(RefCell::new(idx1));
    let idx2: Value<i32> = Rc::new(RefCell::new(idx2));
    let pairs: Value<Ptr<Vec<brunsli_HistogramPair>>> = Rc::new(RefCell::new(pairs));
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
    let store_pair: Value<bool> = Rc::new(RefCell::new(false));
    let p: Value<brunsli_HistogramPair> = Rc::new(RefCell::new(<brunsli_HistogramPair>::default()));
    (*p.borrow_mut()).idx1 = ((*idx1.borrow()) as usize);
    (*p.borrow_mut()).idx2 = ((*idx2.borrow()) as usize);
    (*p.borrow_mut()).cost_diff = (5.0E-1
        * ({
            let _size_a: i32 = (elem!((*cluster_size.borrow()), (*idx1.borrow())).read());
            let _size_b: i32 = (elem!((*cluster_size.borrow()), (*idx2.borrow())).read());
            ClusterCostDiff_129(_size_a, _size_b)
        }));
    (*p.borrow_mut()).cost_diff -=
        { (*elem!((*out.borrow()), (*idx1.borrow())).upgrade().deref()).bit_cost_ };
    (*p.borrow_mut()).cost_diff -=
        { (*elem!((*out.borrow()), (*idx2.borrow())).upgrade().deref()).bit_cost_ };
    if ({ (*elem!((*out.borrow()), (*idx1.borrow())).upgrade().deref()).total_count_ } == 0) {
        (*p.borrow_mut()).cost_combo =
            { (*elem!((*out.borrow()), (*idx2.borrow())).upgrade().deref()).bit_cost_ };
        (*store_pair.borrow_mut()) = true;
    } else if ({ (*elem!((*out.borrow()), (*idx2.borrow())).upgrade().deref()).total_count_ } == 0)
    {
        (*p.borrow_mut()).cost_combo =
            { (*elem!((*out.borrow()), (*idx1.borrow())).upgrade().deref()).bit_cost_ };
        (*store_pair.borrow_mut()) = true;
    } else {
        let threshold: Value<f64> = Rc::new(RefCell::new(
            if (*(*pairs.borrow()).upgrade().deref()).is_empty() {
                1.0E+99
            } else {
                {
                    let __tmp_0: Value<f64> = Rc::new(RefCell::new(0.0E+0));
                    (if __tmp_0.as_pointer().read()
                        >= field_ptr!(
                            ((Ptr::<Vec<brunsli_HistogramPair>>::decay(&(*pairs.borrow())))
                                as Ptr<brunsli_HistogramPair>)
                                .offset(0_usize),
                            cost_diff
                        )
                        .read()
                    {
                        __tmp_0.as_pointer()
                    } else {
                        field_ptr!(
                            ((Ptr::<Vec<brunsli_HistogramPair>>::decay(&(*pairs.borrow())))
                                as Ptr<brunsli_HistogramPair>)
                                .offset(0_usize),
                            cost_diff
                        )
                    }
                    .read())
                }
            },
        ));
        let combo: Value<brunsli_internal_enc_Histogram> = Rc::new(RefCell::new(
            (*elem!((*out.borrow()), (*idx1.borrow())).upgrade().deref()).clone(),
        ));
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*out.borrow()).offset((*idx2.borrow()) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(&combo.as_pointer(), _other)
        });
        let cost_combo: Value<f64> =
            Rc::new(RefCell::new(({ PopulationCost_130(combo.as_pointer()) })));
        if ((*cost_combo.borrow()) < ((*threshold.borrow()) - { (*p.borrow()).cost_diff })) {
            (*p.borrow_mut()).cost_combo = (*cost_combo.borrow());
            (*store_pair.borrow_mut()) = true;
        }
    }
    if (*store_pair.borrow()) {
        (*p.borrow_mut()).cost_diff += { { (*p.borrow()).cost_combo } };
        if (!((*(*pairs.borrow()).upgrade().deref()).is_empty()))
            && ({
                let _p1: Ptr<brunsli_HistogramPair> =
                    (Ptr::<Vec<brunsli_HistogramPair>>::decay(&(*pairs.borrow()))
                        as Ptr<brunsli_HistogramPair>);
                operator_lt_128(_p1, p.as_pointer())
            })
        {
            {
                let a0_clone = (*(Ptr::<Vec<brunsli_HistogramPair>>::decay(&(*pairs.borrow()))
                    as Ptr<brunsli_HistogramPair>)
                    .upgrade()
                    .deref())
                .clone();
                (*pairs.borrow())
                    .with_mut(|__v: &mut Vec<brunsli_HistogramPair>| __v.push(a0_clone))
            };
            (Ptr::<Vec<brunsli_HistogramPair>>::decay(&(*pairs.borrow()))
                as Ptr<brunsli_HistogramPair>)
                .write((*p.borrow()).clone());
        } else {
            {
                let a0_clone = (*p.borrow()).clone();
                (*pairs.borrow())
                    .with_mut(|__v: &mut Vec<brunsli_HistogramPair>| __v.push(a0_clone))
            };
        }
    }
}
pub fn HistogramCombine_133(
    out: Ptr<brunsli_internal_enc_Histogram>,
    cluster_size: Ptr<i32>,
    symbols: Ptr<u32>,
    symbols_size: usize,
    max_clusters: usize,
) -> usize {
    let out: Value<Ptr<brunsli_internal_enc_Histogram>> = Rc::new(RefCell::new(out));
    let cluster_size: Value<Ptr<i32>> = Rc::new(RefCell::new(cluster_size));
    let symbols: Value<Ptr<u32>> = Rc::new(RefCell::new(symbols));
    let symbols_size: Value<usize> = Rc::new(RefCell::new(symbols_size));
    let max_clusters: Value<usize> = Rc::new(RefCell::new(max_clusters));
    let cost_diff_threshold: Value<f64> = Rc::new(RefCell::new(0.0E+0));
    let min_cluster_size: Value<usize> = Rc::new(RefCell::new(1_usize));
    let clusters: Value<Vec<u64>> = Rc::new(RefCell::new({
        let __count = (*symbols.borrow())
            .offset((*symbols_size.borrow()) as isize)
            .get_offset()
            - (*symbols.borrow()).get_offset();
        PtrValueIter::new(&(*symbols.borrow()), __count)
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
    let idx1: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*idx1.borrow()) < (*clusters.borrow()).len()) {
        let idx2: Value<usize> = Rc::new(RefCell::new((*idx1.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*idx2.borrow()) < (*clusters.borrow()).len()) {
            ({
                let _cluster_size: Ptr<i32> = (*cluster_size.borrow()).clone();
                let _idx1: i32 =
                    ((elem!((clusters.as_pointer() as Ptr<u64>), (*idx1.borrow())).read()) as i32);
                let _idx2: i32 =
                    ((elem!((clusters.as_pointer() as Ptr<u64>), (*idx2.borrow())).read()) as i32);
                CompareAndPushToQueue_132(
                    (*out.borrow()).clone(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (pairs.as_pointer()),
                )
            });
            (*idx2.borrow_mut()).prefix_inc();
        }
        (*idx1.borrow_mut()).prefix_inc();
    }
    'loop_: while ((*clusters.borrow()).len() > (*min_cluster_size.borrow())) {
        if ({
            (*elem!((pairs.as_pointer() as Ptr<brunsli_HistogramPair>), 0_usize)
                .upgrade()
                .deref())
            .cost_diff
        } >= (*cost_diff_threshold.borrow()))
        {
            (*cost_diff_threshold.borrow_mut()) = 1.0E+99;
            (*min_cluster_size.borrow_mut()) = (*max_clusters.borrow());
            continue 'loop_;
        }
        let best_idx1: Value<usize> = Rc::new(RefCell::new({
            (*elem!((pairs.as_pointer() as Ptr<brunsli_HistogramPair>), 0_usize)
                .upgrade()
                .deref())
            .idx1
        }));
        let best_idx2: Value<usize> = Rc::new(RefCell::new({
            (*elem!((pairs.as_pointer() as Ptr<brunsli_HistogramPair>), 0_usize)
                .upgrade()
                .deref())
            .idx2
        }));
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*out.borrow()).offset((*best_idx2.borrow()) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(
                &(*out.borrow()).offset((*best_idx1.borrow()) as isize),
                _other,
            )
        });
        let __rhs = {
            (*elem!((pairs.as_pointer() as Ptr<brunsli_HistogramPair>), 0_usize)
                .upgrade()
                .deref())
            .cost_combo
        };
        field!(elem!((*out.borrow()), (*best_idx1.borrow())), bit_cost_).write(__rhs);
        {
            let _ptr = elem!((*cluster_size.borrow()), (*best_idx1.borrow()));
            _ptr.write(
                _ptr.read() + { (elem!((*cluster_size.borrow()), (*best_idx2.borrow())).read()) },
            )
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*symbols_size.borrow())) {
            if ({ ((elem!((*symbols.borrow()), (*i.borrow())).read()) as usize) } == {
                (*best_idx2.borrow())
            }) {
                elem!((*symbols.borrow()), (*i.borrow())).write({ ((*best_idx1.borrow()) as u32) });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let cluster: Value<Ptr<u64>> = Rc::new(RefCell::new((clusters.as_pointer() as Ptr<u64>)));
        'loop_: while (*cluster.borrow()) != (clusters.as_pointer() as Ptr<u64>).to_end() {
            if ((((*cluster.borrow()).read()) as usize) >= (*best_idx2.borrow())) {
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
            if ((({ p.with(|__s| __s.idx1) } == { (*best_idx1.borrow()) })
                || ({ p.with(|__s| __s.idx2) } == { (*best_idx1.borrow()) }))
                || ({ p.with(|__s| __s.idx1) } == { (*best_idx2.borrow()) }))
                || ({ p.with(|__s| __s.idx2) } == { (*best_idx2.borrow()) })
            {
                continue 'loop_;
            }
            if ({
                let _p1: Ptr<brunsli_HistogramPair> =
                    (pairs.as_pointer() as Ptr<brunsli_HistogramPair>);
                let _p2: Ptr<brunsli_HistogramPair> = (p).clone();
                operator_lt_128(_p1, _p2)
            }) {
                let front: Value<brunsli_HistogramPair> = Rc::new(RefCell::new(
                    (*(pairs.as_pointer() as Ptr<brunsli_HistogramPair>)
                        .upgrade()
                        .deref())
                    .clone(),
                ));
                let __rhs = (*p.upgrade().deref()).clone();
                (pairs.as_pointer() as Ptr<brunsli_HistogramPair>).write(__rhs);
                (*copy_to.borrow()).write((*front.borrow()).clone());
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
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*clusters.borrow()).len()) {
            ({
                let _cluster_size: Ptr<i32> = (*cluster_size.borrow()).clone();
                let _idx1: i32 = ((*best_idx1.borrow()) as i32);
                let _idx2: i32 =
                    ((elem!((clusters.as_pointer() as Ptr<u64>), (*i.borrow())).read()) as i32);
                CompareAndPushToQueue_132(
                    (*out.borrow()).clone(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (pairs.as_pointer()),
                )
            });
            (*i.borrow_mut()).prefix_inc();
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
    in_: Ptr<brunsli_internal_enc_Histogram>,
    in_size: usize,
    out: Ptr<brunsli_internal_enc_Histogram>,
    symbols: Ptr<u32>,
) {
    let in_: Value<Ptr<brunsli_internal_enc_Histogram>> = Rc::new(RefCell::new(in_));
    let in_size: Value<usize> = Rc::new(RefCell::new(in_size));
    let out: Value<Ptr<brunsli_internal_enc_Histogram>> = Rc::new(RefCell::new(out));
    let symbols: Value<Ptr<u32>> = Rc::new(RefCell::new(symbols));
    let all_symbols: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = (*symbols.borrow())
            .offset((*in_size.borrow()) as isize)
            .get_offset()
            - (*symbols.borrow()).get_offset();
        PtrValueIter::new(&(*symbols.borrow()), __count)
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
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*in_size.borrow())) {
        let best_out: Value<i32> = Rc::new(RefCell::new(
            (if ((*i.borrow()) == 0_usize) {
                (elem!((*symbols.borrow()), 0).read())
            } else {
                (elem!((*symbols.borrow()), (*i.borrow()).wrapping_sub(1_usize)).read())
            } as i32),
        ));
        let best_bits: Value<f64> = Rc::new(RefCell::new(
            ({
                let _histogram: Ptr<brunsli_internal_enc_Histogram> =
                    (*in_.borrow()).offset((*i.borrow()) as isize);
                let _candidate: Ptr<brunsli_internal_enc_Histogram> =
                    (*out.borrow()).offset((*best_out.borrow()) as isize);
                HistogramBitCostDistance_134(_histogram, _candidate)
            }),
        ));
        'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
            let k: Value<i32> = Rc::new(RefCell::new(k.read()));
            let cur_bits: Value<f64> = Rc::new(RefCell::new(
                ({
                    let _histogram: Ptr<brunsli_internal_enc_Histogram> =
                        (*in_.borrow()).offset((*i.borrow()) as isize);
                    let _candidate: Ptr<brunsli_internal_enc_Histogram> =
                        (*out.borrow()).offset((*k.borrow()) as isize);
                    HistogramBitCostDistance_134(_histogram, _candidate)
                }),
            ));
            if ((*cur_bits.borrow()) < (*best_bits.borrow())) {
                (*best_bits.borrow_mut()) = (*cur_bits.borrow());
                (*best_out.borrow_mut()) = (*k.borrow());
            }
        }
        elem!((*symbols.borrow()), (*i.borrow())).write({ ((*best_out.borrow()) as u32) });
        (*i.borrow_mut()).prefix_inc();
    }
    'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
        let k: Value<i32> = Rc::new(RefCell::new(k.read()));
        ({
            brunsli_internal_enc_HistogramImpl::Clear(
                &(*out.borrow()).offset((*k.borrow()) as isize),
            )
        });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*in_size.borrow())) {
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*in_.borrow()).offset((*i.borrow()) as isize);
            brunsli_internal_enc_HistogramImpl::AddHistogram(
                &(*out.borrow())
                    .offset((elem!((*symbols.borrow()), (*i.borrow())).read()) as isize),
                _other,
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn HistogramReindex_136(out: Ptr<Vec<brunsli_internal_enc_Histogram>>, symbols: Ptr<Vec<u32>>) {
    let out: Value<Ptr<Vec<brunsli_internal_enc_Histogram>>> = Rc::new(RefCell::new(out));
    let symbols: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(symbols));
    let tmp: Value<Vec<brunsli_internal_enc_Histogram>> =
        Rc::new(RefCell::new((*(*out.borrow()).upgrade().deref()).clone()));
    let new_index: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let next_index: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*(*symbols.borrow()).upgrade().deref()).len() }) {
        if RefcountMapIter::find_key(
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
            &((elem!(
                ((Ptr::<Vec<u32>>::decay(&(*symbols.borrow()))) as Ptr<u32>),
                (*i.borrow())
            )
            .read()) as i32),
        ) == RefcountMapIter::end((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>))
        {
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry(
                        ((elem!(
                            ((Ptr::<Vec<u32>>::decay(&(*symbols.borrow()))) as Ptr<u32>),
                            (*i.borrow())
                        )
                        .read()) as i32),
                    )
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
                })
                .write((*next_index.borrow()));
            let __rhs = (*elem!(
                (tmp.as_pointer() as Ptr<brunsli_internal_enc_Histogram>),
                ((elem!(
                    ((Ptr::<Vec<u32>>::decay(&(*symbols.borrow()))) as Ptr<u32>),
                    (*i.borrow())
                )
                .read()) as usize)
            )
            .upgrade()
            .deref())
            .clone();
            elem!(
                ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                    as Ptr<brunsli_internal_enc_Histogram>),
                ((*next_index.borrow()) as usize)
            )
            .write(__rhs);
            (*next_index.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    {
        let __a0 = ((*next_index.borrow()) as usize) as usize;
        (*out.borrow()).with_mut(|__v: &mut Vec<brunsli_internal_enc_Histogram>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        })
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*(*symbols.borrow()).upgrade().deref()).len() }) {
        let __rhs = (((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry(
                    ((elem!(
                        ((Ptr::<Vec<u32>>::decay(&(*symbols.borrow()))) as Ptr<u32>),
                        (*i.borrow())
                    )
                    .read()) as i32),
                )
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
            })
            .read()) as u32);
        elem!(
            ((Ptr::<Vec<u32>>::decay(&(*symbols.borrow()))) as Ptr<u32>),
            (*i.borrow())
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn ClusterHistograms_137(
    in_: Ptr<Vec<brunsli_internal_enc_Histogram>>,
    num_contexts: usize,
    num_blocks: usize,
    block_group_offsets: Vec<u64>,
    max_histograms: usize,
    out: Ptr<Vec<brunsli_internal_enc_Histogram>>,
    histogram_symbols: Ptr<Vec<u32>>,
) {
    let num_contexts: Value<usize> = Rc::new(RefCell::new(num_contexts));
    let num_blocks: Value<usize> = Rc::new(RefCell::new(num_blocks));
    let block_group_offsets: Value<Vec<u64>> = Rc::new(RefCell::new(block_group_offsets));
    let max_histograms: Value<usize> = Rc::new(RefCell::new(max_histograms));
    let out: Value<Ptr<Vec<brunsli_internal_enc_Histogram>>> = Rc::new(RefCell::new(out));
    let histogram_symbols: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(histogram_symbols));
    let in_size: Value<usize> = Rc::new(RefCell::new(
        (*num_contexts.borrow()).wrapping_mul((*num_blocks.borrow())),
    ));
    let cluster_size: Value<Vec<i32>> =
        Rc::new(RefCell::new(vec![1; (*in_size.borrow()) as usize]));
    {
        let __a0 = (*in_size.borrow()) as usize;
        (*out.borrow()).with_mut(|__v: &mut Vec<brunsli_internal_enc_Histogram>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        })
    };
    {
        let __a0 = (*in_size.borrow()) as usize;
        (*histogram_symbols.borrow())
            .with_mut(|__v: &mut Vec<u32>| __v.resize_with(__a0, || <u32>::default()))
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*in_size.borrow())) {
        let __rhs = (*elem!(
            (Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                as Ptr<brunsli_internal_enc_Histogram>),
            (*i.borrow())
        )
        .upgrade()
        .deref())
        .clone();
        elem!(
            ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                as Ptr<brunsli_internal_enc_Histogram>),
            (*i.borrow())
        )
        .write(__rhs);
        let __rhs = ({
            PopulationCost_130(
                (Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset((*i.borrow())),
            )
        });
        field!(
            elem!(
                ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                    as Ptr<brunsli_internal_enc_Histogram>),
                (*i.borrow())
            ),
            bit_cost_
        )
        .write(__rhs);
        let __rhs = ((*i.borrow()) as u32);
        elem!(
            ((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>),
            (*i.borrow())
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*num_contexts.borrow()) > 1_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_blocks.borrow())) {
            ({
                let _symbols: Ptr<u32> =
                    (((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>)
                        .offset((*i.borrow()).wrapping_mul((*num_contexts.borrow()))));
                let _symbols_size: usize = (*num_contexts.borrow());
                HistogramCombine_133(
                    (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                    _symbols,
                    _symbols_size,
                    (*max_histograms.borrow()),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    thread_local!(
        static kMinClustersForHistogramRemap_138: Value<usize> = Rc::new(RefCell::new(24_usize));
    );
    let num_clusters: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*block_group_offsets.borrow()).len() > 1_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*block_group_offsets.borrow()).len()) {
            let offset: Value<usize> = Rc::new(RefCell::new(
                ((elem!(
                    (block_group_offsets.as_pointer() as Ptr<u64>),
                    (*i.borrow())
                )
                .read())
                .wrapping_mul(((*num_contexts.borrow()) as u64)) as usize),
            ));
            let next_offset: Value<usize> = Rc::new(RefCell::new(
                (if ((*i.borrow()).wrapping_add(1_usize) < (*block_group_offsets.borrow()).len()) {
                    (elem!(
                        (block_group_offsets.as_pointer() as Ptr<u64>),
                        (*i.borrow()).wrapping_add(1_usize)
                    )
                    .read())
                    .wrapping_mul(((*num_contexts.borrow()) as u64))
                } else {
                    ((*in_size.borrow()) as u64)
                } as usize),
            ));
            let length: Value<usize> = Rc::new(RefCell::new(
                (*next_offset.borrow()).wrapping_sub((*offset.borrow())),
            ));
            let nclusters: Value<usize> = Rc::new(RefCell::new(
                ({
                    HistogramCombine_133(
                        (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(0_usize)),
                        ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                        (((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>)
                            .offset((*offset.borrow()))),
                        (*length.borrow()),
                        (*max_histograms.borrow()),
                    )
                }),
            ));
            if ((*nclusters.borrow()) >= 2_usize)
                && ((*nclusters.borrow())
                    < kMinClustersForHistogramRemap_138.with(|rc| *rc.borrow()))
            {
                ({
                    let _in_: Ptr<brunsli_internal_enc_Histogram> =
                        ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset((*offset.borrow())));
                    let _symbols: Ptr<u32> =
                        (((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>)
                            .offset((*offset.borrow())));
                    HistogramRemap_135(
                        _in_,
                        (*length.borrow()),
                        (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(0_usize)),
                        _symbols,
                    )
                });
            }
            (*num_clusters.borrow_mut()) =
                { (*num_clusters.borrow()).wrapping_add((*nclusters.borrow())) };
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if ((*block_group_offsets.borrow()).len() <= 1_usize)
        || ((*num_clusters.borrow()) > (*max_histograms.borrow()))
    {
        (*num_clusters.borrow_mut()) = ({
            HistogramCombine_133(
                (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(0_usize)),
                ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize)),
                (((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>)
                    .offset(0_usize)),
                (*in_size.borrow()),
                (*max_histograms.borrow()),
            )
        });
        if ((*num_clusters.borrow()) >= 2_usize)
            && ((*num_clusters.borrow())
                < kMinClustersForHistogramRemap_138.with(|rc| *rc.borrow()))
        {
            ({
                HistogramRemap_135(
                    ((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(in_))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    (*in_size.borrow()),
                    (((Ptr::<Vec<brunsli_internal_enc_Histogram>>::decay(&(*out.borrow())))
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize)),
                    (((Ptr::<Vec<u32>>::decay(&(*histogram_symbols.borrow()))) as Ptr<u32>)
                        .offset(0_usize)),
                )
            });
        }
    }
    ({
        HistogramReindex_136(
            (*out.borrow()).clone(),
            (*histogram_symbols.borrow()).clone(),
        )
    });
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
        num_bands: usize,
        offsets: Ptr<Vec<u64>>,
    ) -> Self {
        let num_bands: Value<usize> = Rc::new(RefCell::new(num_bands));
        let __this: Value<brunsli_internal_enc_EntropyCodes> = Rc::new(RefCell::new(Self {
            clustered_: Rc::new(RefCell::new(Vec::new())),
            context_map_: Rc::new(RefCell::new(Vec::new())),
            ans_tables_: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_internal_enc_EntropyCodes> = __this.as_pointer();
        ({
            let _in_: Ptr<Vec<brunsli_internal_enc_Histogram>> = (histograms).clone();
            let _num_contexts: usize = kNumAvrgContexts_83.with(|rc| *rc.borrow());
            let _num_blocks: usize = (*num_bands.borrow());
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
        let __this: Value<brunsli_internal_enc_EntropySource> = Rc::new(RefCell::new(Self {
            num_bands_: 0_usize,
            histograms_: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_internal_enc_EntropySource> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_enc_DataStream_CodeWord> = Rc::new(RefCell::new(Self {
            context: 0_u32,
            value: 0_u16,
            code: 0_u8,
            nbits: 0_u8,
        }));
        let this: Ptr<brunsli_internal_enc_DataStream_CodeWord> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_enc_DataStream> = Rc::new(RefCell::new(Self {
            pos_: 3,
            bw_pos_: 0,
            ac_pos0_: 1,
            ac_pos1_: 2,
            low_: 0_u32,
            high_: (!0 as u32),
            bw_val_: 0_u32,
            bw_bitpos_: 0,
            code_words_: Rc::new(RefCell::new(Vec::new())),
        }));
        let this: Ptr<brunsli_internal_enc_DataStream> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    let size: Value<usize> = Rc::new(RefCell::new(
        (((((*jpg.with(|__s| __s.marker_order.clone()).borrow()).len()).wrapping_add(
            (272_usize).wrapping_mul((*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len()),
        ))
        .wrapping_add(
            (7_usize).wrapping_mul((*jpg.with(|__s| __s.scan_info.clone()).borrow()).len()),
        ))
        .wrapping_add(16_usize)),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() })
    {
        {
            let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
                ((7_usize).wrapping_mul(
                    (*{
                        (*elem!(
                            (jpg.with(|__s| __s.scan_info.as_pointer())
                                as Ptr<brunsli_JPEGScanInfo>),
                            (*i.borrow())
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
            (*size.borrow_mut()) = rhs_0
        };
        {
            let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
                ((7_usize).wrapping_mul(
                    (*{
                        (*elem!(
                            (jpg.with(|__s| __s.scan_info.as_pointer())
                                as Ptr<brunsli_JPEGScanInfo>),
                            (*i.borrow())
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
            (*size.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let nsize: Value<usize> = Rc::new(RefCell::new(if jpg.with(|__s| __s.has_zero_padding_bit) {
        (*jpg.with(|__s| __s.padding_bits.clone()).borrow()).len()
    } else {
        0_usize
    }));
    (*size.borrow_mut()) =
        { (*size.borrow()).wrapping_add((((*nsize.borrow()).wrapping_add(43_usize)) >> 3)) };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len()
    }) {
        {
            let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
                ((5_usize).wrapping_add(
                    (*((jpg.with(|__s| __s.inter_marker_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                        .offset((*i.borrow()))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>)
                        .upgrade()
                        .deref())
                    .len(),
                ) as u64),
            )) as usize;
            (*size.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).prefix_inc();
    }
    return (*size.borrow());
}
pub fn GetMaximumBrunsliEncodedSize_145(jpg: Ptr<brunsli_JPEGData>) -> usize {
    let hdr_size: Value<usize> = Rc::new(RefCell::new(((1 << 20) as usize)));
    {
        let rhs_0 = (*hdr_size.borrow()).wrapping_add(({ EstimateAuxDataSize_144((jpg).clone()) }));
        (*hdr_size.borrow_mut()) = rhs_0
    };
    'loop_: for mut data in jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        {
            let rhs_0 = (((*hdr_size.borrow()) as u64)
                .wrapping_add(((*data.upgrade().deref()).len() as u64)))
                as usize;
            (*hdr_size.borrow_mut()) = rhs_0
        };
    }
    'loop_: for mut data in jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        {
            let rhs_0 = (((*hdr_size.borrow()) as u64)
                .wrapping_add(((*data.upgrade().deref()).len() as u64)))
                as usize;
            (*hdr_size.borrow_mut()) = rhs_0
        };
    }
    {
        let rhs_0 = (((*hdr_size.borrow()) as u64)
            .wrapping_add(((*jpg.with(|__s| __s.tail_data.clone()).borrow()).len() as u64)))
            as usize;
        (*hdr_size.borrow_mut()) = rhs_0
    };
    let num_pixels: Value<usize> = Rc::new(RefCell::new(
        (({ jpg.with(|__s| __s.width) } * { jpg.with(|__s| __s.height) }) as usize)
            .wrapping_mul((*jpg.with(|__s| __s.components.clone()).borrow()).len()),
    ));
    return ((((*num_pixels.borrow()) as f64) * 1.2E+0) as usize)
        .wrapping_add((*hdr_size.borrow()));
}
pub fn Base128Size_146(val: usize) -> usize {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let size: Value<usize> = Rc::new(RefCell::new(1_usize));
    'loop_: while ((*val.borrow()) >= 128_usize) {
        (*size.borrow_mut()).prefix_inc();
        (*val.borrow_mut()) >>= 7;
    }
    return (*size.borrow());
}
pub fn EncodeBase128_147(val: usize, data: Ptr<u8>) -> usize {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut __do_while = true;
    'loop_: while __do_while || ((*val.borrow()) > 0_usize) {
        __do_while = false;
        let __rhs = ((((*val.borrow()) & 127_usize)
            | ((if ((*val.borrow()) >= 128_usize) {
                128
            } else {
                0
            }) as usize)) as u8);
        elem!((*data.borrow()), (*len.borrow_mut()).postfix_inc()).write(__rhs);
        (*val.borrow_mut()) >>= 7;
    }
    return (*len.borrow());
}
pub fn EncodeBase128Fix_148(val: usize, len: usize, data: Ptr<u8>) {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let __rhs = ((((*val.borrow()) & 127_usize)
            | ((if ((*i.borrow()).wrapping_add(1_usize) < (*len.borrow())) {
                128
            } else {
                0
            }) as usize)) as u8);
        ((*data.borrow_mut()).postfix_inc()).write(__rhs);
        (*val.borrow_mut()) >>= 7;
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn TransformApp0Marker_149(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
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
        let x_dens_hi: Value<u8> = Rc::new(RefCell::new(
            (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 11_usize).read()),
        ));
        let x_dens_lo: Value<u8> = Rc::new(RefCell::new(
            (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 12_usize).read()),
        ));
        let x_dens: Value<i32> = Rc::new(RefCell::new(
            ((((*x_dens_hi.borrow()) as i32) << 8) + ((*x_dens_lo.borrow()) as i32)),
        ));
        let y_dens_hi: Value<u8> = Rc::new(RefCell::new(
            (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 13_usize).read()),
        ));
        let y_dens_lo: Value<u8> = Rc::new(RefCell::new(
            (elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 14_usize).read()),
        ));
        let y_dens: Value<i32> = Rc::new(RefCell::new(
            ((((*y_dens_hi.borrow()) as i32) << 8) + ((*y_dens_lo.borrow()) as i32)),
        ));
        let density_ix: Value<i32> = Rc::new(RefCell::new(-1_i32));
        let k: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*k.borrow()) < kMaxApp0Densities_45.with(|rc| *rc.borrow())) {
            if ((*x_dens.borrow())
                == (({
                    let __idx = (*k.borrow()) as usize;
                    kApp0Densities_46.with(|rc| rc.borrow()[__idx])
                }) as i32))
                && ((*y_dens.borrow()) == (*x_dens.borrow()))
            {
                (*density_ix.borrow_mut()) = ((*k.borrow()) as i32);
            }
            (*k.borrow_mut()).prefix_inc();
        }
        if ((*density_ix.borrow()) >= 0) {
            let app0_status: Value<u8> = Rc::new(RefCell::new(
                (({
                    ((((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 9_usize).read()) as i32)
                        - 1)
                        | (((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 10_usize).read())
                            as i32)
                            << 1))
                } | { ((*density_ix.borrow()) << 3) }) as u8),
            ));
            ((*out.borrow()).clone() as Ptr<Vec<u8>>).write(
                (0..(1_usize) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            (Ptr::<Vec<u8>>::decay(&(*out.borrow())) as Ptr<u8>)
                .offset(0_usize as isize)
                .write((*app0_status.borrow()));
            return true;
        }
    }
    return false;
}
pub fn TransformApp2Marker_150(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
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
        let code: Value<Vec<u8>> = Rc::new(RefCell::new(
            (0..(2_usize) as usize)
                .map(|_| <u8>::default())
                .collect::<Vec<_>>(),
        ));
        elem!((code.as_pointer() as Ptr<u8>), 0_usize).write(128_u8);
        elem!((code.as_pointer() as Ptr<u8>), 1_usize)
            .write((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 84_usize).read()));
        ((*out.borrow()).clone() as Ptr<Vec<u8>>).write((*code.borrow()).clone());
        return true;
    }
    return false;
}
pub fn TransformApp12Marker_151(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
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
        let code: Value<Vec<u8>> = Rc::new(RefCell::new(
            (0..(2_usize) as usize)
                .map(|_| <u8>::default())
                .collect::<Vec<_>>(),
        ));
        elem!((code.as_pointer() as Ptr<u8>), 0_usize).write(129_u8);
        elem!((code.as_pointer() as Ptr<u8>), 1_usize)
            .write((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 15_usize).read()));
        ((*out.borrow()).clone() as Ptr<Vec<u8>>).write((*code.borrow()).clone());
        return true;
    }
    return false;
}
pub fn TransformApp14Marker_152(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
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
        let code: Value<Vec<u8>> = Rc::new(RefCell::new(
            (0..(2_usize) as usize)
                .map(|_| <u8>::default())
                .collect::<Vec<_>>(),
        ));
        elem!((code.as_pointer() as Ptr<u8>), 0_usize).write(130_u8);
        elem!((code.as_pointer() as Ptr<u8>), 1_usize)
            .write((elem!((Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>), 10_usize).read()));
        ((*out.borrow()).clone() as Ptr<Vec<u8>>).write((*code.borrow()).clone());
        return true;
    }
    return false;
}
pub fn TransformAppMarker_153(s: Ptr<Vec<u8>>, transformed_marker_count: Ptr<usize>) -> Vec<u8> {
    let transformed_marker_count: Value<Ptr<usize>> =
        Rc::new(RefCell::new(transformed_marker_count));
    let out: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp0Marker_149(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp2Marker_150(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp12Marker_151(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp14Marker_152(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return std::mem::take(&mut (*out.borrow_mut()));
    }
    return (*s.upgrade().deref()).clone();
}
pub fn GetQuantTableId_154(q: Ptr<brunsli_JPEGQuantTable>, is_chroma: bool, dst: Ptr<u8>) -> i32 {
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let j: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*j.borrow()) < kNumStockQuantTables_47.with(|rc| *rc.borrow())) {
        let match_found: Value<bool> = Rc::new(RefCell::new(true));
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (*match_found.borrow())
            && ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow()))
        {
            if ({
                (elem!(
                    (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                    ((*k.borrow()) as usize)
                )
                .read())
            } != {
                (({
                    let __idx = (*is_chroma.borrow()) as usize;
                    kStockQuantizationTables_48.with(|rc| rc.borrow()[__idx].clone())
                })
                .borrow()[(*j.borrow()) as usize]
                    .borrow()[(*k.borrow()) as usize] as i32)
            }) {
                (*match_found.borrow_mut()) = false;
            }
            (*k.borrow_mut()).prefix_inc();
        }
        if (*match_found.borrow()) {
            return (*j.borrow());
        }
        (*j.borrow_mut()).prefix_inc();
    }
    return (((kNumStockQuantTables_47.with(|rc| *rc.borrow()) as u32).wrapping_add(
        ({
            FindBestMatrix_119(
                ((q.with(|__s| __s.values.as_pointer()) as Ptr<i32>).offset(0_usize)),
                (*is_chroma.borrow()),
                (*dst.borrow()).clone(),
            )
        }),
    )) as i32);
}
pub fn EncodeVarint_155(n: i32, max_bits: i32, storage: Ptr<brunsli_Storage>) {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let max_bits: Value<i32> = Rc::new(RefCell::new(max_bits));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let b: Value<i32> = Rc::new(RefCell::new(0_i32));
    if !((*n.borrow()) < (1 << (*max_bits.borrow()))) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                215,
                Ptr::<i8>::from_string_literal(b"EncodeVarint"),
            )
        });
        'loop_: while true {}
    };
    (*b.borrow_mut()) = 0;
    'loop_: while ((*n.borrow()) != 0) && ((*b.borrow()) < (*max_bits.borrow())) {
        if (((*b.borrow()) + 1) != (*max_bits.borrow())) {
            ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
        }
        ({
            WriteBits_120(
                1_usize,
                (((*n.borrow()) & 1) as u64),
                (*storage.borrow()).clone(),
            )
        });
        (*n.borrow_mut()) >>= 1;
        (*b.borrow_mut()).prefix_inc();
    }
    if ((*b.borrow()) < (*max_bits.borrow())) {
        ({ WriteBits_120(1_usize, 0_u64, (*storage.borrow()).clone()) });
    }
}
pub fn EncodeLimitedVarint_156(
    bits: usize,
    nbits: i32,
    max_symbols: i32,
    storage: Ptr<brunsli_Storage>,
) {
    let bits: Value<usize> = Rc::new(RefCell::new(bits));
    let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
    let max_symbols: Value<i32> = Rc::new(RefCell::new(max_symbols));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let mask: Value<usize> = Rc::new(RefCell::new(
        (1_usize << (*nbits.borrow())).wrapping_sub(1_usize),
    ));
    let b: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*b.borrow()) < (*max_symbols.borrow())) {
        ({
            WriteBits_120(
                1_usize,
                (((*bits.borrow()) != 0_usize) as u64),
                (*storage.borrow()).clone(),
            )
        });
        if ((*bits.borrow()) == 0_usize) {
            break;
        }
        ({
            WriteBits_120(
                ((*nbits.borrow()) as usize),
                (((*bits.borrow()) & (*mask.borrow())) as u64),
                (*storage.borrow()).clone(),
            )
        });
        (*bits.borrow_mut()) >>= (*nbits.borrow());
        (*b.borrow_mut()).prefix_inc();
    }
}
pub fn EncodeQuantTables_157(jpg: Ptr<brunsli_JPEGData>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*jpg.with(|__s| __s.quant.clone()).borrow()).is_empty())
        || ((*jpg.with(|__s| __s.quant.clone()).borrow()).len() > 4_usize)
    {
        return false;
    }
    ({
        WriteBits_120(
            2_usize,
            (((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize) as u64),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        let q: Ptr<brunsli_JPEGQuantTable> = (jpg.with(|__s| __s.quant.as_pointer())
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*i.borrow()));
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let j: Value<i32> = Rc::new(RefCell::new(
                (({
                    let __idx = (*k.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as i32),
            ));
            if ((elem!(
                (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                ((*j.borrow()) as usize)
            )
            .read())
                == 0)
            {
                return false;
            }
            (*k.borrow_mut()).prefix_inc();
        }
        let quant_approx: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..64).map(|_| 0_u8).collect::<Box<[u8]>>()));
        let code: Value<i32> = Rc::new(RefCell::new(
            ({
                let _q: Ptr<brunsli_JPEGQuantTable> = (q).clone();
                let _is_chroma: bool = ((*i.borrow()) > 0_usize);
                let _dst: Ptr<u8> = (quant_approx.as_pointer() as Ptr<u8>);
                GetQuantTableId_154(_q, _is_chroma, _dst)
            }),
        ));
        ({
            WriteBits_120(
                1_usize,
                (((*code.borrow()) >= kNumStockQuantTables_47.with(|rc| *rc.borrow())) as u64),
                (*storage.borrow()).clone(),
            )
        });
        if ((*code.borrow()) < kNumStockQuantTables_47.with(|rc| *rc.borrow())) {
            ({
                WriteBits_120(
                    3_usize,
                    ((*code.borrow()) as u64),
                    (*storage.borrow()).clone(),
                )
            });
        } else {
            let q_factor: Value<usize> = Rc::new(RefCell::new(
                (((*code.borrow()) - kNumStockQuantTables_47.with(|rc| *rc.borrow())) as usize),
            ));
            if !((*q_factor.borrow()) < kQFactorLimit_117.with(|rc| *rc.borrow())) {
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
                    ((*q_factor.borrow()) as u64),
                    (*storage.borrow()).clone(),
                )
            });
            let last_diff: Value<i32> = Rc::new(RefCell::new(0));
            let k: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                let j: Value<i32> = Rc::new(RefCell::new(
                    (({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    }) as i32),
                ));
                let new_diff: Value<i32> = Rc::new(RefCell::new(
                    ({
                        (elem!(
                            (q.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                            ((*j.borrow()) as usize)
                        )
                        .read())
                    } - { ((*quant_approx.borrow())[(*j.borrow()) as usize] as i32) }),
                ));
                let diff: Value<i32> =
                    Rc::new(RefCell::new(((*new_diff.borrow()) - (*last_diff.borrow()))));
                (*last_diff.borrow_mut()) = (*new_diff.borrow());
                ({
                    WriteBits_120(
                        1_usize,
                        (((*diff.borrow()) != 0) as u64),
                        (*storage.borrow()).clone(),
                    )
                });
                if ((*diff.borrow()) != 0) {
                    ({
                        WriteBits_120(
                            1_usize,
                            (((*diff.borrow()) < 0) as u64),
                            (*storage.borrow()).clone(),
                        )
                    });
                    if ((*diff.borrow()) < 0) {
                        (*diff.borrow_mut()) = { -(*diff.borrow()) };
                    }
                    (*diff.borrow_mut()) -= 1;
                    if ((*diff.borrow()) > 65535) {
                        return false;
                    }
                    ({ EncodeVarint_155((*diff.borrow()), 16, (*storage.borrow()).clone()) });
                }
                (*k.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() })
    {
        ({
            WriteBits_120(
                2_usize,
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .quant_idx
                } as u64),
                (*storage.borrow()).clone(),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeHuffmanCode_158(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    is_known_last: bool,
    storage: Ptr<brunsli_Storage>,
) -> bool {
    let is_known_last: Value<bool> = Rc::new(RefCell::new(is_known_last));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        WriteBits_120(
            2_usize,
            ((huff.with(|__s| __s.slot_id) & 15) as u64),
            (*storage.borrow()).clone(),
        )
    });
    ({
        WriteBits_120(
            1_usize,
            ((huff.with(|__s| __s.slot_id) >> 4) as u64),
            (*storage.borrow()).clone(),
        )
    });
    if !(*is_known_last.borrow()) {
        ({
            WriteBits_120(
                1_usize,
                (huff.with(|__s| __s.is_last) as u64),
                (*storage.borrow()).clone(),
            )
        });
    } else if !(huff.with(|__s| __s.is_last)) {
        return false;
    }
    let is_dc_table: Value<i32> = Rc::new(RefCell::new(
        (((huff.with(|__s| __s.slot_id) >> 4) == 0) as i32),
    ));
    let total_count: Value<i32> = Rc::new(RefCell::new(0));
    let space: Value<i32> = Rc::new(RefCell::new(
        (1 << kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())),
    ));
    let max_len: Value<i32> = Rc::new(RefCell::new(
        kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()),
    ));
    let max_count: Value<i32> = Rc::new(RefCell::new(if ((*is_dc_table.borrow()) != 0) {
        kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())
    } else {
        kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())
    }));
    let found_match: Value<i32> = Rc::new(RefCell::new(0));
    let stock_table_idx: Value<i32> = Rc::new(RefCell::new(0));
    if ((*is_dc_table.borrow()) != 0) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < kNumStockDCHuffmanCodes_53.with(|rc| *rc.borrow()))
            && (!((*found_match.borrow()) != 0))
        {
            if ((((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>).offset(1_usize))
                as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockDCHuffmanCodeCounts_54.with(|v| v.as_pointer())
                        as Ptr<Value<Box<[i32]>>>)
                        .offset((*i.borrow()))
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
                            .offset((*i.borrow()))
                            .read()
                            .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                            .to_any(),
                        ::std::mem::size_of::<[i32; 13]>(),
                    )
                    == 0)
            {
                (*found_match.borrow_mut()) = 1;
                (*stock_table_idx.borrow_mut()) = (*i.borrow());
            }
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < kNumStockACHuffmanCodes_56.with(|rc| *rc.borrow()))
            && (!((*found_match.borrow()) != 0))
        {
            if ((((huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>).offset(1_usize))
                as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockACHuffmanCodeCounts_57.with(|v| v.as_pointer())
                        as Ptr<Value<Box<[i32]>>>)
                        .offset((*i.borrow()))
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
                            .offset((*i.borrow()))
                            .read()
                            .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                            .to_any(),
                        ::std::mem::size_of::<[i32; 163]>(),
                    )
                    == 0)
            {
                (*found_match.borrow_mut()) = 1;
                (*stock_table_idx.borrow_mut()) = (*i.borrow());
            }
            (*i.borrow_mut()).prefix_inc();
        }
    }
    ({
        WriteBits_120(
            1_usize,
            ((*found_match.borrow()) as u64),
            (*storage.borrow()).clone(),
        )
    });
    if ((*found_match.borrow()) != 0) {
        ({
            WriteBits_120(
                1_usize,
                ((*stock_table_idx.borrow()) as u64),
                (*storage.borrow()).clone(),
            )
        });
        return true;
    }
    'loop_: while ((*max_len.borrow()) > 0)
        && ((elem!(
            (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
            ((*max_len.borrow()) as usize)
        )
        .read())
            == 0)
    {
        (*max_len.borrow_mut()).prefix_dec();
    }
    if ((elem!(
        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
        0_usize
    )
    .read())
        != 0)
        || ((*max_len.borrow()) == 0)
    {
        return false;
    }
    ({
        WriteBits_120(
            4_usize,
            (((*max_len.borrow()) - 1) as u64),
            (*storage.borrow()).clone(),
        )
    });
    (*space.borrow_mut()) -=
        (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - (*max_len.borrow())));
    let i: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*i.borrow()) <= (*max_len.borrow())) {
        let count: Value<i32> = Rc::new(RefCell::new(
            ({
                (elem!(
                    (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                    ((*i.borrow()) as usize)
                )
                .read())
            } - {
                (if ((*i.borrow()) == (*max_len.borrow())) {
                    1
                } else {
                    0
                })
            }),
        ));
        let count_limit: Value<i32> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(
                ((*max_count.borrow()) - (*total_count.borrow())),
            ));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(
                ((*space.borrow())
                    >> (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - (*i.borrow()))),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        if ((*count.borrow()) > (*count_limit.borrow())) {
            if true {
            } else {
                write!(
                    libcc2rs::cerr(),
                    "len = {:} count = {:} limit = {:} space = {:} total = {:}\n",
                    (*i.borrow()),
                    (*count.borrow()),
                    (*count_limit.borrow()),
                    (*space.borrow()),
                    (*total_count.borrow()),
                );
            }
            return false;
        }
        if ((*count_limit.borrow()) > 0) {
            let nbits: Value<i32> = Rc::new(RefCell::new(
                (({ Log2FloorNonZero_74(((*count_limit.borrow()) as u32)) }) + 1),
            ));
            ({
                WriteBits_120(
                    ((*nbits.borrow()) as usize),
                    ((*count.borrow()) as u64),
                    (*storage.borrow()).clone(),
                )
            });
            (*total_count.borrow_mut()) += (*count.borrow());
            (*space.borrow_mut()) -= ((*count.borrow())
                * (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - (*i.borrow()))));
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if ({
        (elem!(
            (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
            ((*total_count.borrow()) as usize)
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
            if ((*is_dc_table.borrow()) != 0) {
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
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*total_count.borrow())) {
        let val: Value<i32> = Rc::new(RefCell::new(
            (elem!(
                (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                ((*i.borrow()) as usize)
            )
            .read()),
        ));
        let code: Value<i32> = Rc::new(RefCell::new(0_i32));
        let nbits: Value<i32> = Rc::new(RefCell::new(0_i32));
        if !({
            brunsli_PermutationCoderImpl::RemoveValue(
                &p.as_pointer(),
                ((*val.borrow()) as u8),
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
                (*storage.borrow()).clone(),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeScanInfo_159(si: Ptr<brunsli_JPEGScanInfo>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        WriteBits_120(
            6_usize,
            (si.with(|__s| __s.Ss) as u64),
            (*storage.borrow()).clone(),
        )
    });
    ({
        WriteBits_120(
            6_usize,
            (si.with(|__s| __s.Se) as u64),
            (*storage.borrow()).clone(),
        )
    });
    ({
        WriteBits_120(
            4_usize,
            (si.with(|__s| __s.Ah) as u64),
            (*storage.borrow()).clone(),
        )
    });
    ({
        WriteBits_120(
            4_usize,
            (si.with(|__s| __s.Al) as u64),
            (*storage.borrow()).clone(),
        )
    });
    ({
        WriteBits_120(
            2_usize,
            ((si.with(|__s| __s.num_components)).wrapping_sub(1_usize) as u64),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { si.with(|__s| __s.num_components) }) {
        let csi: Ptr<brunsli_JPEGComponentScanInfo> = (si.with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset((*i.borrow()));
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.comp_idx) as u64),
                (*storage.borrow()).clone(),
            )
        });
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.dc_tbl_idx) as u64),
                (*storage.borrow()).clone(),
            )
        });
        ({
            WriteBits_120(
                2_usize,
                (csi.with(|__s| __s.ac_tbl_idx) as u64),
                (*storage.borrow()).clone(),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let last_block_idx: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: for mut block_idx in si.with(|__s| __s.reset_points.as_pointer()) as Ptr<i32> {
        ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
        if !({ (block_idx.read()) } >= { ((*last_block_idx.borrow()) + 1) }) {
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
            let _n: i32 = (({ (block_idx.read()) } - { (*last_block_idx.borrow()) }) - 1);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeVarint_155(_n, 28, _storage)
        });
        (*last_block_idx.borrow_mut()) = { (block_idx.read()) };
    }
    ({ WriteBits_120(1_usize, 0_u64, (*storage.borrow()).clone()) });
    (*last_block_idx.borrow_mut()) = 0;
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*si.with(|__s| __s.extra_zero_runs.clone()).borrow()).len()
    }) {
        let block_idx: Value<i32> = Rc::new(RefCell::new({
            (*elem!(
                (si.with(|__s| __s.extra_zero_runs.as_pointer())
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                (*i.borrow())
            )
            .upgrade()
            .deref())
            .block_idx
        }));
        let num: Value<i32> = Rc::new(RefCell::new({
            (*elem!(
                (si.with(|__s| __s.extra_zero_runs.as_pointer())
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>),
                (*i.borrow())
            )
            .upgrade()
            .deref())
            .num_extra_zero_runs
        }));
        if !((*block_idx.borrow()) >= (*last_block_idx.borrow())) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    401,
                    Ptr::<i8>::from_string_literal(b"EncodeScanInfo"),
                )
            });
            'loop_: while true {}
        };
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < (*num.borrow())) {
            ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
            ({
                EncodeVarint_155(
                    ((*block_idx.borrow()) - (*last_block_idx.borrow())),
                    28,
                    (*storage.borrow()).clone(),
                )
            });
            (*last_block_idx.borrow_mut()) = (*block_idx.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({ WriteBits_120(1_usize, 0_u64, (*storage.borrow()).clone()) });
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
pub fn JumpToByteBoundary_161(storage: Ptr<brunsli_Storage>) {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let nbits: Value<i32> = Rc::new(RefCell::new(
        (((*storage.borrow()).with(|__s| __s.pos) & 7_usize) as i32),
    ));
    if ((*nbits.borrow()) > 0) {
        ({
            WriteBits_120(
                ((8 - (*nbits.borrow())) as usize),
                0_u64,
                (*storage.borrow()).clone(),
            )
        });
    }
}
pub fn EncodeAuxData_162(jpg: Ptr<brunsli_JPEGData>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*jpg.with(|__s| __s.marker_order.clone()).borrow()).is_empty())
        || ((((jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>)
            .to_last()
            .read()) as i32)
            != 217)
    {
        return false;
    }
    let have_dri: Value<bool> = Rc::new(RefCell::new(false));
    let num_scans: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*jpg.with(|__s| __s.marker_order.clone()).borrow()).len()
    }) {
        let marker: Value<u8> = Rc::new(RefCell::new(
            (elem!(
                (jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
                (*i.borrow())
            )
            .read()),
        ));
        if (((*marker.borrow()) as i32) < 192) {
            return false;
        }
        ({
            WriteBits_120(
                6_usize,
                ((((*marker.borrow()) as i32) - 192) as u64),
                (*storage.borrow()).clone(),
            )
        });
        if (((*marker.borrow()) as i32) == 221) {
            (*have_dri.borrow_mut()) = true;
        }
        if (((*marker.borrow()) as i32) == 218) {
            (*num_scans.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if (*have_dri.borrow()) {
        ({
            WriteBits_120(
                16_usize,
                (jpg.with(|__s| __s.restart_interval) as u64),
                (*storage.borrow()).clone(),
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
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len()
    }) {
        let is_known_last: Value<bool> = Rc::new(RefCell::new(
            ({ ((*i.borrow()).wrapping_add(1_usize)) } == {
                (*jpg.with(|__s| __s.huffman_code.clone()).borrow()).len()
            }),
        ));
        ({
            WriteBits_120(
                1_usize,
                ((*is_known_last.borrow()) as u64),
                (*storage.borrow()).clone(),
            )
        });
        if !({
            EncodeHuffmanCode_158(
                (jpg.with(|__s| __s.huffman_code.as_pointer()) as Ptr<brunsli_JPEGHuffmanCode>)
                    .offset((*i.borrow())),
                (*is_known_last.borrow()),
                (*storage.borrow()).clone(),
            )
        }) {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if ({ (*num_scans.borrow()) } != { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() }) {
        return false;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.scan_info.clone()).borrow()).len() })
    {
        if !({
            EncodeScanInfo_159(
                (jpg.with(|__s| __s.scan_info.as_pointer()) as Ptr<brunsli_JPEGScanInfo>)
                    .offset((*i.borrow())),
                (*storage.borrow()).clone(),
            )
        }) {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        WriteBits_120(
            2_usize,
            (((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize) as u64),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        ({
            WriteBits_120(
                2_usize,
                ({
                    (*elem!(
                        (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .index
                } as u64),
                (*storage.borrow()).clone(),
            )
        });
        if ({ (*i.borrow()) } != {
            ((*jpg.with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize)
        }) {
            ({
                WriteBits_120(
                    1_usize,
                    ({
                        (*elem!(
                            (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                            (*i.borrow())
                        )
                        .upgrade()
                        .deref())
                        .is_last
                    } as u64),
                    (*storage.borrow()).clone(),
                )
            });
        } else if !({
            (*elem!(
                (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                (*i.borrow())
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
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .precision
                } as u64),
                (*storage.borrow()).clone(),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let comp_ids: Value<i32> = Rc::new(RefCell::new(
        ({ MatchComponentIds_160(jpg.with(|__s| __s.components.as_pointer())) }),
    ));
    ({
        WriteBits_120(
            2_usize,
            ((*comp_ids.borrow()) as u64),
            (*storage.borrow()).clone(),
        )
    });
    if ((*comp_ids.borrow()) == kComponentIdsCustom_52.with(|rc| *rc.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*i.borrow()) } < {
            (*jpg.with(|__s| __s.components.clone()).borrow()).len()
        }) {
            ({
                WriteBits_120(
                    8_usize,
                    ({
                        (*elem!(
                            (jpg.with(|__s| __s.components.as_pointer())
                                as Ptr<brunsli_JPEGComponent>),
                            (*i.borrow())
                        )
                        .upgrade()
                        .deref())
                        .id
                    } as u64),
                    (*storage.borrow()).clone(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    let nsize: Value<usize> = Rc::new(RefCell::new(if jpg.with(|__s| __s.has_zero_padding_bit) {
        (*jpg.with(|__s| __s.padding_bits.clone()).borrow()).len()
    } else {
        0_usize
    }));
    if ({ (*nsize.borrow()) } > { (({ PaddingBitsLimit_17((jpg).clone()) }) as usize) }) {
        return false;
    }
    ({ EncodeLimitedVarint_156((*nsize.borrow()), 8, 4, (*storage.borrow()).clone()) });
    if ((*nsize.borrow()) > 0_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*nsize.borrow())) {
            ({
                WriteBits_120(
                    1_usize,
                    ((elem!(
                        (jpg.with(|__s| __s.padding_bits.as_pointer()) as Ptr<i32>),
                        (*i.borrow())
                    )
                    .read()) as u64),
                    (*storage.borrow()).clone(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    ({ JumpToByteBoundary_161((*storage.borrow()).clone()) });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len()
    }) {
        let s: Ptr<Vec<u8>> = ((jpg.with(|__s| __s.inter_marker_data.as_pointer())
            as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()))
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>);
        let buffer: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_u8).collect::<Box<[u8]>>()));
        let len: Value<usize> = Rc::new(RefCell::new(
            ({
                EncodeBase128_147(
                    (*s.upgrade().deref()).len(),
                    (buffer.as_pointer() as Ptr<u8>),
                )
            }),
        ));
        ({
            brunsli_StorageImpl::AppendBytes(
                &(*storage.borrow()),
                (buffer.as_pointer() as Ptr<u8>),
                (*len.borrow()),
            )
        });
        ({
            let _src: Ptr<u8> = (Ptr::<Vec<u8>>::decay(&(s)) as Ptr<u8>);
            let _len: usize = (*s.upgrade().deref()).len();
            brunsli_StorageImpl::AppendBytes(&(*storage.borrow()), _src, _len)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
impl brunsli_internal_enc_Histogram {}
pub fn ComputeCoeffOrder_163(num_zeros: Ptr<Vec<i32>>, order: Ptr<u32>) {
    let order: Value<Ptr<u32>> = Rc::new(RefCell::new(order));
    let pos_and_val: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize) as usize)
            .map(|_| <(Value<i32>, Value<i32>)>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let __rhs = (*i.borrow());
        (*(*elem!(
            (pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>),
            ((*i.borrow()) as usize)
        )
        .upgrade()
        .deref())
        .0
        .borrow_mut()) = __rhs;
        let __rhs = (elem!(
            (Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>),
            (({
                let __idx = (*i.borrow()) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            }) as usize)
        )
        .read());
        (*(*elem!(
            (pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>),
            ((*i.borrow()) as usize)
        )
        .upgrade()
        .deref())
        .1
        .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
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
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        let __rhs = ({
            let __idx = (*(*elem!(
                (pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>),
                (*i.borrow())
            )
            .upgrade()
            .deref())
            .0
            .borrow()) as usize;
            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
        });
        elem!((*order.borrow()), (*i.borrow())).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
impl brunsli_internal_enc_EntropyCodes {}
impl brunsli_internal_enc_DataStream {}
pub fn EncodeNumNonzeros_166(
    val: usize,
    p: Ptr<brunsli_Prob>,
    data_stream: Ptr<brunsli_internal_enc_DataStream>,
) {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
    let data_stream: Value<Ptr<brunsli_internal_enc_DataStream>> =
        Rc::new(RefCell::new(data_stream));
    if !((*val.borrow()) < ((1_u32 << kNumNonZeroBits_84.with(|rc| *rc.borrow())) as usize)) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                719,
                Ptr::<i8>::from_string_literal(b"EncodeNumNonzeros"),
            )
        });
        'loop_: while true {}
    };
    let bst: Value<Ptr<brunsli_Prob>> =
        Rc::new(RefCell::new((*p.borrow()).offset(-((1) as isize))));
    let ctx: Value<usize> = Rc::new(RefCell::new(1_usize));
    let mask: Value<usize> = Rc::new(RefCell::new(
        ((1 << ((kNumNonZeroBits_84.with(|rc| *rc.borrow())).wrapping_sub(1_usize))) as usize),
    ));
    'loop_: while ((*mask.borrow()) != 0_usize) {
        let bit: Value<i32> = Rc::new(RefCell::new(
            ((((*val.borrow()) & (*mask.borrow())) != 0_usize) as i32),
        ));
        ({
            brunsli_internal_enc_DataStreamImpl::AddBit(
                &(*data_stream.borrow()),
                (*bst.borrow()).offset((*ctx.borrow()) as isize),
                (*bit.borrow()),
            )
        });
        (*ctx.borrow_mut()) =
            { ((2_usize).wrapping_mul((*ctx.borrow()))).wrapping_add(((*bit.borrow()) as usize)) };
        (*mask.borrow_mut()) >>= 1;
    }
}
pub fn CollectAllCoeffs_167(coeffs: Ptr<i16>) -> i16 {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let all_coeffs: Value<i16> = Rc::new(RefCell::new(0_i16));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while (((*all_coeffs.borrow()) as i32) == 0)
        && ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow()))
    {
        (*all_coeffs.borrow_mut()) = {
            (((*all_coeffs.borrow()) as i32)
                | ((elem!((*coeffs.borrow()), (*k.borrow())).read()) as i32)) as i16
        };
        (*k.borrow_mut()).prefix_inc();
    }
    return (*all_coeffs.borrow());
}
pub fn EncodeCoeffOrder_168(order: Ptr<u32>, data_stream: Ptr<brunsli_internal_enc_DataStream>) {
    let order: Value<Ptr<u32>> = Rc::new(RefCell::new(order));
    let data_stream: Value<Ptr<brunsli_internal_enc_DataStream>> =
        Rc::new(RefCell::new(data_stream));
    let order_zigzag: Value<Box<[u32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_u32).collect::<Box<[u32]>>()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        (*order_zigzag.borrow_mut())[(*i.borrow()) as usize] = {
            ({
                let __idx = (elem!((*order.borrow()), (*i.borrow())).read()) as usize;
                kJPEGZigZagOrder_14.with(|rc| rc.borrow()[__idx])
            })
        };
        (*i.borrow_mut()).prefix_inc();
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
    let tail: Value<i32> = Rc::new(RefCell::new((kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1)));
    'loop_: while ((*tail.borrow()) >= 1)
        && ((*lehmer.borrow())[(*tail.borrow()) as usize] == 0_u32)
    {
        (*tail.borrow_mut()).prefix_dec();
    }
    let i: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*i.borrow()) <= (*tail.borrow())) {
        (*lehmer.borrow_mut())[(*i.borrow()) as usize].prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    thread_local!(
        static kSpan_169: Value<i32> = Rc::new(RefCell::new(16));
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        let start: Value<i32> = Rc::new(RefCell::new(if ((*i.borrow()) > 0) {
            (*i.borrow())
        } else {
            1
        }));
        let end: Value<i32> = Rc::new(RefCell::new(
            ((*i.borrow()) + kSpan_169.with(|rc| *rc.borrow())),
        ));
        let has_non_zero: Value<i32> = Rc::new(RefCell::new(0));
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            (*has_non_zero.borrow_mut()) = {
                (((*has_non_zero.borrow()) as u32) | (*lehmer.borrow())[(*j.borrow()) as usize])
                    as i32
            };
            (*j.borrow_mut()).prefix_inc();
        }
        if !((*has_non_zero.borrow()) != 0) {
            ({ brunsli_internal_enc_DataStreamImpl::AddBits(&(*data_stream.borrow()), 1, 0) });
            (*i.borrow_mut()) += kSpan_169.with(|rc| *rc.borrow());
            continue 'loop_;
        } else {
            ({ brunsli_internal_enc_DataStreamImpl::AddBits(&(*data_stream.borrow()), 1, 1) });
        }
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            let v: Value<i32> = Rc::new(RefCell::new(0_i32));
            if !((*lehmer.borrow())[(*j.borrow()) as usize]
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
            (*v.borrow_mut()) = ((*lehmer.borrow())[(*j.borrow()) as usize] as i32);
            'loop_: while ((*v.borrow()) >= 7) {
                ({ brunsli_internal_enc_DataStreamImpl::AddBits(&(*data_stream.borrow()), 3, 7) });
                (*v.borrow_mut()) -= 7;
            }
            ({
                brunsli_internal_enc_DataStreamImpl::AddBits(
                    &(*data_stream.borrow()),
                    3,
                    (*v.borrow()),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()) += kSpan_169.with(|rc| *rc.borrow());
    }
}
pub fn FrameTypeCode_170(jpg: Ptr<brunsli_JPEGData>) -> u32 {
    let code: Value<u32> = Rc::new(RefCell::new(0_u32));
    let shift: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() })
        && ((*i.borrow()) < 4_usize)
    {
        let h_samp: Value<u32> = Rc::new(RefCell::new(
            (({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .h_samp_factor
            } - 1) as u32),
        ));
        let v_samp: Value<u32> = Rc::new(RefCell::new(
            (({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .v_samp_factor
            } - 1) as u32),
        ));
        (*code.borrow_mut()) |= (((*h_samp.borrow()) << ((*shift.borrow()) + 4))
            | ((*v_samp.borrow()) << (*shift.borrow())));
        (*shift.borrow_mut()) += 8;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*code.borrow());
}
pub fn EncodeSignature_171(len: usize, data: Ptr<u8>, pos: Ptr<usize>) -> bool {
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    if ((*len.borrow()) < kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
        || ({ ((*pos.borrow()).read()) } > {
            (*len.borrow()).wrapping_sub(kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
        })
    {
        return false;
    }
    {
        (((*data.borrow()).offset(((*pos.borrow()).read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((kBrunsliSignature_44.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                kBrunsliSignatureSize_43.with(|rc| *rc.borrow()) as usize,
            );
        (((*data.borrow()).offset(((*pos.borrow()).read()) as isize)) as Ptr<u8>).to_any()
    };
    (*pos.borrow()).write({
        ((*pos.borrow()).read()).wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
    });
    return true;
}
pub fn EncodeValue_172(tag: u8, value: usize, data: Ptr<u8>, pos: Ptr<usize>) {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    let value: Value<usize> = Rc::new(RefCell::new(value));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let __rhs = ({ ValueMarker_28((*tag.borrow())) });
    elem!(
        (*data.borrow()),
        (*pos.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    {
        let rhs_0 = ((*pos.borrow()).read()).wrapping_add(
            ({
                let _val: usize = (*value.borrow());
                let _data: Ptr<u8> = (*data.borrow()).offset(((*pos.borrow()).read()) as isize);
                EncodeBase128_147(_val, _data)
            }),
        );
        (*pos.borrow()).write(rhs_0)
    };
}
pub fn EncodeHeader_173(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*state.borrow_mut());
    let version: Value<usize> = Rc::new(RefCell::new((jpg.with(|__s| __s.version) as usize)));
    let is_fallback: Value<bool> = Rc::new(RefCell::new(
        (((*version.borrow()) & 1_usize) == (kFallbackVersion_2.with(|rc| *rc.borrow()) as usize)),
    ));
    if (*is_fallback.borrow())
        && ((*version.borrow()) != (kFallbackVersion_2.with(|rc| *rc.borrow()) as usize))
    {
        return false;
    }
    if (((!(*is_fallback.borrow()))
        && ((jpg.with(|__s| __s.width) == 0) || (jpg.with(|__s| __s.height) == 0)))
        || ((*jpg.with(|__s| __s.components.clone()).borrow()).is_empty()))
        || ({ (*jpg.with(|__s| __s.components.clone()).borrow()).len() } > {
            (kMaxComponents_4.with(|rc| *rc.borrow()) as usize)
        })
    {
        return false;
    }
    if (((*version.borrow()) & (!7_u32 as usize)) != 0) {
        return false;
    }
    let version_comp: Value<usize> = Rc::new(RefCell::new(
        (({
            ((((*jpg.with(|__s| __s.components.clone()).borrow()).len()).wrapping_sub(1_usize))
                as u64)
        } | { (((*version.borrow()) << 2) as u64) }) as usize),
    ));
    let subsampling: Value<usize> = Rc::new(RefCell::new(
        (({ FrameTypeCode_170((jpg).clone()) }) as usize),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        let _tag: u8 = kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        EncodeValue_172(
            _tag,
            (jpg.with(|__s| __s.width) as usize),
            _data,
            (pos.as_pointer()),
        )
    });
    ({
        let _tag: u8 = kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        EncodeValue_172(
            _tag,
            (jpg.with(|__s| __s.height) as usize),
            _data,
            (pos.as_pointer()),
        )
    });
    ({
        let _tag: u8 = kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        EncodeValue_172(_tag, (*version_comp.borrow()), _data, (pos.as_pointer()))
    });
    ({
        let _tag: u8 = kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        EncodeValue_172(_tag, (*subsampling.borrow()), _data, (pos.as_pointer()))
    });
    (*len.borrow()).write({ (*pos.borrow()) });
    return true;
}
pub fn EncodeMetaData_174(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*state.borrow_mut());
    let metadata: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    let transformed_marker_count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.app_data.clone()).borrow()).len() }) {
        let s: Ptr<Vec<u8>> = ((jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()))
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
        (*i.borrow_mut()).prefix_inc();
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
    let other_app_count: Value<usize> = Rc::new(RefCell::new(
        (((*jpg.with(|__s| __s.app_data.clone()).borrow()).len() as u64)
            .wrapping_sub(((*transformed_marker_count.borrow()) as u64)) as usize),
    ));
    if ((*other_app_count.borrow())
        > (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many app markers: {:}\n",
            (*other_app_count.borrow()),
        );
        return false;
    }
    let com_count: Value<usize> = Rc::new(RefCell::new(
        (*jpg.with(|__s| __s.com_data.clone()).borrow()).len(),
    ));
    if ((*com_count.borrow()) > (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many com markers: {:}\n",
            (*com_count.borrow()),
        );
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
        (*len.borrow()).write(0_usize);
        return true;
    } else if ((*metadata.borrow()).len() == 1_usize) {
        (*len.borrow()).write(1_usize);
        let __rhs = (elem!((metadata.as_pointer() as Ptr<u8>), 0_usize).read());
        elem!((*data.borrow()), 0).write(__rhs);
        return true;
    }
    let pos: Value<usize> = Rc::new(RefCell::new(
        ({ EncodeBase128_147((*metadata.borrow()).len(), (*data.borrow()).clone()) }),
    ));
    let compressed_size: Value<usize> = Rc::new(RefCell::new(
        ((*len.borrow()).read()).wrapping_sub((*pos.borrow())),
    ));
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
            ((*data.borrow()).offset((*pos.borrow()) as isize))
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
            (*pos.borrow()),
            ((*len.borrow()).read()),
        );
        return false;
    }
    (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add((*compressed_size.borrow())) };
    (*len.borrow()).write({ (*pos.borrow()) });
    return true;
}
pub fn EncodeJPEGInternals_175(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*state.borrow_mut());
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::new(
        { (*data.borrow()).clone() },
        { ((*len.borrow()).read()) },
    )));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeAuxData_162(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeQuantData_176(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*state.borrow_mut());
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::new(
        { (*data.borrow()).clone() },
        { ((*len.borrow()).read()) },
    )));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeQuantTables_157(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeHistogramData_177(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::new(
        { (*data.borrow()).clone() },
        { ((*len.borrow()).read()) },
    )));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.components.clone()).borrow()).len() })
    {
        ({
            WriteBits_120(
                3_usize,
                ({
                    (*elem!(
                        ((*state.borrow()).with(|__s| __s.meta.as_pointer())
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .context_bits
                } as u64),
                (storage.as_pointer()),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        brunsli_internal_enc_EntropyCodesImpl::EncodeContextMap(
            &(*state.borrow()).with(|__s| __s.entropy_codes.clone()),
            (storage.as_pointer()),
        )
    });
    ({
        brunsli_internal_enc_EntropyCodesImpl::BuildAndStoreEntropyCodes(
            &(*state.borrow()).with(|__s| __s.entropy_codes.clone()),
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeDCData_178(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*jpg.upgrade().deref());
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::new(
        { (*data.borrow()).clone() },
        { ((*len.borrow()).read()) },
    )));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> =
            (*state.borrow()).with(|__s| __s.entropy_codes.clone());
        brunsli_internal_enc_DataStreamImpl::EncodeCodeWords(
            &field_ptr!((*state.borrow()), data_stream_dc),
            _s,
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeACData_179(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*jpg.upgrade().deref());
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::new(
        { (*data.borrow()).clone() },
        { ((*len.borrow()).read()) },
    )));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> =
            (*state.borrow()).with(|__s| __s.entropy_codes.clone());
        brunsli_internal_enc_DataStreamImpl::EncodeCodeWords(
            &field_ptr!((*state.borrow()), data_stream_ac),
            _s,
            (storage.as_pointer()),
        )
    });
    let __rhs = ({ brunsli_StorageImpl::GetBytesUsed(&storage.as_pointer()) });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeSection_180(
    jpg: Ptr<brunsli_JPEGData>,
    s: Ptr<brunsli_internal_enc_State>,
    tag: u8,
    write_section: FnPtr<
        fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
    >,
    section_size_bytes: usize,
    len: usize,
    data: Ptr<u8>,
    pos: Ptr<usize>,
) -> bool {
    let s: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(s));
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    let write_section: Value<
        FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >,
    > = Rc::new(RefCell::new(write_section));
    let section_size_bytes: Value<usize> = Rc::new(RefCell::new(section_size_bytes));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let pos_start: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    let marker: Value<u8> = Rc::new(RefCell::new(({ SectionMarker_29((*tag.borrow())) })));
    let __rhs = (*marker.borrow());
    elem!(
        (*data.borrow()),
        (*pos.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    (*pos.borrow())
        .write({ ((*pos.borrow()).read()).wrapping_add((*section_size_bytes.borrow())) });
    let section_size: Value<usize> = Rc::new(RefCell::new(
        (*len.borrow()).wrapping_sub(((*pos.borrow()).read())),
    ));
    if !({
        let _arg0: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _arg1: Ptr<brunsli_internal_enc_State> = (*s.borrow()).clone();
        let _arg2: Ptr<u8> = ((*data.borrow()).offset(((*pos.borrow()).read()) as isize));
        let _arg3: Ptr<usize> = (section_size.as_pointer());
        (*write_section.borrow()).call(_arg0, _arg1, _arg2, _arg3)
    }) {
        return false;
    }
    (*pos.borrow()).write({ ((*pos.borrow()).read()).wrapping_add((*section_size.borrow())) });
    if (((*section_size.borrow()) >> ((7_usize).wrapping_mul((*section_size_bytes.borrow()))))
        > 0_usize)
    {
        write!(libcc2rs::cerr(), "Section 0x",);
        libcc2rs::cerr()
            .write_all(&([(&[(*marker.borrow()) as u8] as &[u8]), (b" size " as &[u8])].concat()));
        write!(
            libcc2rs::cerr(),
            "{:} too large for {:} bytes base128 number.\n",
            (*section_size.borrow()),
            (*section_size_bytes.borrow()),
        );
        return false;
    }
    ({
        EncodeBase128Fix_148(
            (*section_size.borrow()),
            (*section_size_bytes.borrow()),
            ((*data.borrow()).offset(((*pos_start.borrow()).wrapping_add(1_usize)) as isize)),
        )
    });
    return true;
}
pub fn SampleNumNonZeros_181(m: Ptr<brunsli_internal_enc_ComponentMeta>) -> usize {
    let m: Value<Ptr<brunsli_internal_enc_ComponentMeta>> = Rc::new(RefCell::new(m));
    let num_blocks: Value<usize> = Rc::new(RefCell::new(
        (({ (*m.borrow()).with(|__s| __s.width_in_blocks) } * {
            (*m.borrow()).with(|__s| __s.height_in_blocks)
        }) as usize),
    ));
    if ((*num_blocks.borrow()) < ((32 * 32) as usize)) {
        return (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)
            .wrapping_mul((*num_blocks.borrow()));
    }
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
        (*m.borrow()).with(|__s| __s.ac_coeffs.clone()),
    ));
    let stride: Value<usize> = Rc::new(RefCell::new(
        ((*m.borrow()).with(|__s| __s.ac_stride) as usize),
    ));
    let width_in_blocks: Value<usize> = Rc::new(RefCell::new(
        ((*m.borrow()).with(|__s| __s.width_in_blocks) as usize),
    ));
    let num_zeros: Ptr<Vec<i32>> = (*m.borrow()).with(|__s| __s.num_zeros.as_pointer());
    thread_local!(
        static kStride_182: Value<i32> = Rc::new(RefCell::new(5));
    );
    let total_nonzeros: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_blocks.borrow())) {
        let x: Value<usize> = Rc::new(RefCell::new(
            (*i.borrow()).wrapping_rem((*width_in_blocks.borrow())),
        ));
        let y: Value<usize> = Rc::new(RefCell::new(
            (*i.borrow()).wrapping_div((*width_in_blocks.borrow())),
        ));
        let block: Value<Ptr<i16>> = Rc::new(RefCell::new(
            (*coeffs.borrow())
                .offset(
                    ((*x.borrow()).wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                        as isize,
                )
                .offset(((*y.borrow()).wrapping_mul((*stride.borrow()))) as isize),
        ));
        let k: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*k.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
            if (((elem!((*block.borrow()), (*k.borrow())).read()) as i32) == 0) {
                elem!(
                    (Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>),
                    (*k.borrow())
                )
                .with_mut(|__v| __v.prefix_inc());
            }
            (*k.borrow_mut()).prefix_inc();
        }
        (*total_nonzeros.borrow_mut()) = {
            (*total_nonzeros.borrow())
                .wrapping_add((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
        };
        (*i.borrow_mut()) =
            { (*i.borrow()).wrapping_add((kStride_182.with(|rc| *rc.borrow()) as usize)) };
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        {
            let rhs_0 = (*total_nonzeros.borrow()).wrapping_sub(
                ((elem!(
                    (Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>),
                    (*i.borrow())
                )
                .read()) as usize),
            );
            (*total_nonzeros.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).prefix_inc();
    }
    elem!((Ptr::<Vec<i32>>::decay(&(num_zeros)) as Ptr<i32>), 0_usize).write(0);
    return (*total_nonzeros.borrow()).wrapping_mul((kStride_182.with(|rc| *rc.borrow()) as usize));
}
pub fn SelectContextBits_183(num_symbols: usize) -> i32 {
    let num_symbols: Value<usize> = Rc::new(RefCell::new(num_symbols));
    thread_local!(
        static kContextBits_184: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6,
            6, 6, 6, 6,
        ])));
    );
    let log2_size: Value<usize> = Rc::new(RefCell::new(
        (({ Log2FloorNonZero_74(((*num_symbols.borrow()) as u32)) }) as usize),
    ));
    let scheme: Value<i32> = Rc::new(RefCell::new(
        ({
            let __idx = (*log2_size.borrow()) as usize;
            kContextBits_184.with(|rc| rc.borrow()[__idx])
        }),
    ));
    if !((*scheme.borrow()) < kNumSchemes_91.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                1029,
                Ptr::<i8>::from_string_literal(b"SelectContextBits"),
            )
        });
        'loop_: while true {}
    };
    return (*scheme.borrow());
}
pub fn PredictDCCoeffs_185(state: Ptr<brunsli_internal_enc_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*meta.upgrade().deref()).len() }) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()));
        let width: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.width_in_blocks)));
        let height: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.height_in_blocks)));
        let ac_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.ac_stride)));
        let dc_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.dc_stride)));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*y.borrow()) < (*height.borrow())) {
            let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                m.with(|__s| __s.ac_coeffs.clone())
                    .offset(((*ac_stride.borrow()) * (*y.borrow())) as isize),
            ));
            let pred_errors: Value<Ptr<i16>> = Rc::new(RefCell::new(
                m.with(|__s| __s.dc_prediction_errors.clone())
                    .offset(((*dc_stride.borrow()) * (*y.borrow())) as isize),
            ));
            let x: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*x.borrow()) < (*width.borrow())) {
                let err: Value<i32> = Rc::new(RefCell::new(
                    ({ ((elem!((*coeffs.borrow()), 0).read()) as i32) } - {
                        ({
                            PredictWithAdaptiveMedian_115(
                                (*coeffs.borrow()).clone(),
                                (*x.borrow()),
                                (*y.borrow()),
                                (*ac_stride.borrow()),
                            )
                        })
                    }),
                ));
                if ((*err.borrow()).abs() > kBrunsliMaxDCAbsVal_19.with(|rc| *rc.borrow())) {
                    write!(
                        libcc2rs::cerr(),
                        "Invalid DC coefficient: {:} after prediction: {:}\n",
                        (elem!((*coeffs.borrow()), 0).read()),
                        (*err.borrow()),
                    );
                    return false;
                }
                (*coeffs.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                let __rhs = ((*err.borrow()) as i16);
                ((*pred_errors.borrow_mut()).postfix_inc()).write(__rhs);
                (*x.borrow_mut()).prefix_inc();
            }
            (*y.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn CalculateMeta_186(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let num_components: Value<usize> = Rc::new(RefCell::new(
        (*jpg.with(|__s| __s.components.clone()).borrow()).len(),
    ));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    {
        let __a0 = (*num_components.borrow()) as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_enc_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_ComponentMeta>::default())
        })
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()));
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()));
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
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeDC_187(state: Ptr<brunsli_internal_enc_State>) {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new(
        ({
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
        }),
    ));
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> =
        field_ptr!((*state.borrow()), entropy_source);
    let data_stream: Ptr<brunsli_internal_enc_DataStream> =
        field_ptr!((*state.borrow()), data_stream_dc);
    let comps: Value<Vec<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <brunsli_ComponentStateDC>::default())
            .collect::<Vec<_>>(),
    ));
    let total_num_blocks: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()));
        ({
            brunsli_ComponentStateDCImpl::SetWidth(
                &(comps.as_pointer() as Ptr<brunsli_ComponentStateDC>).offset((*i.borrow())),
                m.with(|__s| __s.width_in_blocks),
            )
        });
        (*total_num_blocks.borrow_mut()) = {
            (*total_num_blocks.borrow()).wrapping_add(
                (({ m.with(|__s| __s.width_in_blocks) } * { m.with(|__s| __s.height_in_blocks) })
                    as usize),
            )
        };
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _num_bands: usize = (*num_components.borrow());
        brunsli_internal_enc_EntropySourceImpl::Resize(&entropy_source, _num_bands)
    });
    ({
        let _max_num_code_words: usize =
            ((3_usize).wrapping_mul((*total_num_blocks.borrow()))).wrapping_add(128_usize);
        brunsli_internal_enc_DataStreamImpl::Resize(&data_stream, _max_num_code_words)
    });
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
                ((comps.as_pointer() as Ptr<brunsli_ComponentStateDC>).offset((*i.borrow()))),
            ));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()));
            let width: Value<i32> = Rc::new(RefCell::new((*c.borrow()).with(|__s| __s.width)));
            let ac_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.ac_stride)));
            let dc_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.dc_stride)));
            let b_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.b_stride)));
            let y: Value<i32> = Rc::new(RefCell::new(
                ({ (*mcu_y.borrow()) } * { m.with(|__s| __s.v_samp) }),
            ));
            let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*c.borrow()).with(|__s| __s.prev_sign.as_pointer()) as Ptr<i32>)
                    .offset(1_usize)),
            ));
            let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*c.borrow()).with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>)
                    .offset(2_usize)),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ({ (*iy.borrow()) } < { m.with(|__s| __s.v_samp) }) {
                let dc_coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.dc_prediction_errors.clone())
                        .offset(((*y.borrow()) * (*dc_stride.borrow())) as isize),
                ));
                let ac_coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.ac_coeffs.clone())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.block_state.clone())
                        .offset(((*y.borrow()) * (*b_stride.borrow())) as isize),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    ({ brunsli_internal_enc_DataStreamImpl::ResizeForBlock(&data_stream) });
                    let coeff: Value<i16> =
                        Rc::new(RefCell::new((elem!((*dc_coeffs_in.borrow()), 0).read())));
                    let sign: Value<i32> =
                        Rc::new(RefCell::new(if (((*coeff.borrow()) as i32) > 0) {
                            1
                        } else {
                            if (((*coeff.borrow()) as i32) < 0) {
                                2
                            } else {
                                0
                            }
                        }));
                    let absval: Value<i32> = Rc::new(RefCell::new(if ((*sign.borrow()) == 2) {
                        -((*coeff.borrow()) as i32)
                    } else {
                        ((*coeff.borrow()) as i32)
                    }));
                    let all_coeffs: Value<i16> = Rc::new(RefCell::new(
                        (({ ((*coeff.borrow()) as i32) } | {
                            (({ CollectAllCoeffs_167((*ac_coeffs_in.borrow()).clone()) }) as i32)
                        }) as i16),
                    ));
                    let is_empty_block: Value<bool> =
                        Rc::new(RefCell::new((((*all_coeffs.borrow()) as i32) == 0)));
                    let is_empty_ctx: Value<i32> = Rc::new(RefCell::new(
                        ({
                            IsEmptyBlockContext_106(
                                (((*c.borrow()).with(|__s| __s.prev_is_nonempty.as_pointer())
                                    as Ptr<i32>)
                                    .offset(1_usize)),
                                (*x.borrow()),
                            )
                        }),
                    ));
                    ({
                        let _p: Ptr<brunsli_Prob> = (((*c.borrow())
                            .with(|__s| __s.is_empty_block_prob.as_pointer())
                            as Ptr<brunsli_Prob>)
                            .offset(((*is_empty_ctx.borrow()) as usize)));
                        let _bit: i32 = (!(*is_empty_block.borrow()) as i32);
                        brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                    });
                    elem!(
                        ((*c.borrow()).with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>),
                        (((*x.borrow()) + 1) as usize)
                    )
                    .write((!(*is_empty_block.borrow()) as i32));
                    (*block_state.borrow()).write({ ((*is_empty_block.borrow()) as u8) });
                    if !(*is_empty_block.borrow()) {
                        let is_zero: Value<i32> =
                            Rc::new(RefCell::new(((((*coeff.borrow()) as i32) == 0) as i32)));
                        ({
                            let _p: Ptr<brunsli_Prob> = (field_ptr!((*c.borrow()), is_zero_prob));
                            let _bit: i32 = (*is_zero.borrow());
                            brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                        });
                        if !((*is_zero.borrow()) != 0) {
                            let avrg_ctx: Value<i32> = Rc::new(RefCell::new(
                                ({
                                    let _vals: Ptr<i32> = (*prev_abs.borrow()).clone();
                                    let _x: i32 = (*x.borrow());
                                    WeightedAverageContextDC_97(_vals, _x)
                                }),
                            ));
                            let sign_ctx: Value<i32> = Rc::new(RefCell::new(
                                (((elem!((*prev_sgn.borrow()), (*x.borrow())).read()) * 3)
                                    + (elem!((*prev_sgn.borrow()), ((*x.borrow()) - 1)).read())),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> = (((*c.borrow())
                                    .with(|__s| __s.sign_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset(((*sign_ctx.borrow()) as usize)));
                                let _bit: i32 = ((*sign.borrow()) - 1);
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                            let zdens_ctx: Value<usize> = Rc::new(RefCell::new((*i.borrow())));
                            if ((*absval.borrow()) <= kNumDirectCodes_141.with(|rc| *rc.borrow())) {
                                ({
                                    let _code: usize = (((*absval.borrow()) - 1) as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = (((*avrg_ctx.borrow()) as u32) as usize);
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
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    (({
                                        Log2FloorNonZero_74(
                                            ((((*absval.borrow())
                                                - kNumDirectCodes_141.with(|rc| *rc.borrow()))
                                                + 1)
                                                as u32),
                                        )
                                    }) - 1),
                                ));
                                ({
                                    let _code: usize = ((kNumDirectCodes_141
                                        .with(|rc| *rc.borrow())
                                        + (*nbits.borrow()))
                                        as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = ((*avrg_ctx.borrow()) as usize);
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
                                let extra_bits: Value<i32> = Rc::new(RefCell::new(
                                    ((*absval.borrow())
                                        - ((kNumDirectCodes_141.with(|rc| *rc.borrow()) - 1)
                                            + (2 << (*nbits.borrow())))),
                                ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    (((*extra_bits.borrow()) >> (*nbits.borrow())) & 1),
                                ));
                                ({
                                    let _p: Ptr<brunsli_Prob> = (((*c.borrow())
                                        .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                        as Ptr<brunsli_Prob>)
                                        .offset(((*nbits.borrow()) as usize)));
                                    let _bit: i32 = (*first_extra_bit.borrow());
                                    brunsli_internal_enc_DataStreamImpl::AddBit(
                                        &data_stream,
                                        _p,
                                        _bit,
                                    )
                                });
                                if ((*nbits.borrow()) > 0) {
                                    (*extra_bits.borrow_mut()) &= ((1 << (*nbits.borrow())) - 1);
                                    ({
                                        let _nbits: i32 = (*nbits.borrow());
                                        let _bits: i32 = (*extra_bits.borrow());
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
                    elem!((*prev_sgn.borrow()), (*x.borrow())).write({ (*sign.borrow()) });
                    elem!((*prev_abs.borrow()), (*x.borrow())).write({ (*absval.borrow()) });
                    (*block_state.borrow_mut()).prefix_inc();
                    (*dc_coeffs_in.borrow_mut()).prefix_inc();
                    (*ac_coeffs_in.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                {
                    (*iy.borrow_mut()).prefix_inc();
                    (*y.borrow_mut()).prefix_inc()
                };
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*mcu_y.borrow_mut()).prefix_inc();
    }
}
pub fn EncodeAC_188(state: Ptr<brunsli_internal_enc_State>) {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new(
        ({
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
        }),
    ));
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> =
        field_ptr!((*state.borrow()), entropy_source);
    let data_stream: Ptr<brunsli_internal_enc_DataStream> =
        field_ptr!((*state.borrow()), data_stream_ac);
    let context_modes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (kContextAlgorithm_95.with(|v| v.as_pointer()) as Ptr<u8>).offset(
            (if (*state.borrow()).with(|__s| __s.use_legacy_context_model) {
                64
            } else {
                0
            }) as isize,
        ),
    ));
    let num_code_words: Value<usize> = Rc::new(RefCell::new(0_usize));
    let comps: Value<Vec<brunsli_ComponentState>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <brunsli_ComponentState>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()));
        let num_blocks: Value<usize> = Rc::new(RefCell::new(
            (({ m.with(|__s| __s.width_in_blocks) } * { m.with(|__s| __s.height_in_blocks) })
                as usize),
        ));
        (*num_code_words.borrow_mut()) = {
            (*num_code_words.borrow()).wrapping_add(
                (((2_usize).wrapping_mul(m.with(|__s| __s.approx_total_nonzeros)))
                    .wrapping_add(1024_usize))
                .wrapping_add((3_usize).wrapping_mul((*num_blocks.borrow()))),
            )
        };
        ({
            ComputeCoeffOrder_163(
                m.with(|__s| __s.num_zeros.as_pointer()),
                ((array_field_ptr!(
                    (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow())),
                    order
                ) as Ptr<u32>)
                    .offset((0) as isize)),
            )
        });
        ({
            let _mult_row: Ptr<i32> = ((array_field_ptr!(
                (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow())),
                mult_row
            ) as Ptr<i32>)
                .offset((0) as isize));
            let _mult_col: Ptr<i32> = ((array_field_ptr!(
                (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow())),
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
                &(comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow())),
                m.with(|__s| __s.width_in_blocks),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _num_bands: usize = (*state.borrow()).with(|__s| __s.num_contexts);
        brunsli_internal_enc_EntropySourceImpl::Resize(&entropy_source, _num_bands)
    });
    ({
        let _max_num_code_words: usize = (*num_code_words.borrow());
        brunsli_internal_enc_DataStreamImpl::Resize(&data_stream, _max_num_code_words)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        ({
            EncodeCoeffOrder_168(
                ((array_field_ptr!(
                    (comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow())),
                    order
                ) as Ptr<u32>)
                    .offset((0) as isize)),
                (data_stream).clone(),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentState>> = Rc::new(RefCell::new(
                ((comps.as_pointer() as Ptr<brunsli_ComponentState>).offset((*i.borrow()))),
            ));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()));
            let cur_ctx_bits: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.context_bits)));
            let cur_order: Value<Ptr<u32>> = Rc::new(RefCell::new(
                (array_field_ptr!((*c.borrow()), order) as Ptr<u32>),
            ));
            let width: Value<i32> = Rc::new(RefCell::new((*c.borrow()).with(|__s| __s.width)));
            let y: Value<i32> = Rc::new(RefCell::new(
                ({ (*mcu_y.borrow()) } * { m.with(|__s| __s.v_samp) }),
            ));
            let ac_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.ac_stride)));
            let b_stride: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.b_stride)));
            let prev_row_delta: Value<i32> = Rc::new(RefCell::new(
                (((1 - (2 * ((*y.borrow()) & 1))) * ((*width.borrow()) + 3))
                    * kDCTBlockSize_3.with(|rc| *rc.borrow())),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ({ (*iy.borrow()) } < { m.with(|__s| __s.v_samp) }) {
                let coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.ac_coeffs.clone())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.block_state.clone())
                        .offset(((*y.borrow()) * (*b_stride.borrow())) as isize),
                ));
                let prev_row_coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*coeffs_in.borrow()).offset(-((*ac_stride.borrow()) as isize)),
                ));
                let prev_col_coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*coeffs_in.borrow())
                        .offset(-((kDCTBlockSize_3.with(|rc| *rc.borrow())) as isize)),
                ));
                let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                    (((*c.borrow()).with(|__s| __s.prev_sign.as_pointer()) as Ptr<i32>)
                        .offset((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))),
                ));
                let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                    (((*c.borrow()).with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>)
                        .offset(
                            ((((((*y.borrow()) & 1) * ((*width.borrow()) + 3)) + 2)
                                * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                as usize),
                        )),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    ({ brunsli_internal_enc_DataStreamImpl::ResizeForBlock(&data_stream) });
                    let coeffs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16,
                    ])));
                    let last_nz: Value<i32> = Rc::new(RefCell::new(0));
                    let is_empty_block: Value<bool> =
                        Rc::new(RefCell::new((((*block_state.borrow()).read()) != 0)));
                    if !(*is_empty_block.borrow()) {
                        let k: Value<i32> = Rc::new(RefCell::new(1));
                        'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                            let k_nat: Value<i32> = Rc::new(RefCell::new(
                                ((elem!((*cur_order.borrow()), (*k.borrow())).read()) as i32),
                            ));
                            (*coeffs.borrow_mut())[(*k.borrow()) as usize] =
                                { (elem!((*coeffs_in.borrow()), (*k_nat.borrow())).read()) };
                            if ((*coeffs.borrow())[(*k.borrow()) as usize] != 0) {
                                (*last_nz.borrow_mut()) = (*k.borrow());
                            }
                            (*k.borrow_mut()).prefix_inc();
                        }
                        let nzero_context: Value<u8> = Rc::new(RefCell::new(
                            ({
                                NumNonzerosContext_104(
                                    ((*c.borrow()).with(|__s| __s.prev_num_nonzeros.as_pointer())
                                        as Ptr<u8>),
                                    (*x.borrow()),
                                    (*y.borrow()),
                                )
                            }),
                        ));
                        ({
                            EncodeNumNonzeros_166(
                                ((*last_nz.borrow()) as usize),
                                (array_field_ptr!((*c.borrow()), num_nonzero_prob)
                                    as Ptr<brunsli_Prob>)
                                    .offset(
                                        ((kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                                            .wrapping_mul(((*nzero_context.borrow()) as usize)))
                                            as isize,
                                    ),
                                (data_stream).clone(),
                            )
                        });
                    }
                    let k: Value<i32> =
                        Rc::new(RefCell::new((kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1)));
                    'loop_: while ((*k.borrow()) > (*last_nz.borrow())) {
                        elem!((*prev_sgn.borrow()), (*k.borrow())).write(0);
                        elem!((*prev_abs.borrow()), (*k.borrow())).write(0);
                        (*k.borrow_mut()).prefix_dec();
                    }
                    let num_nzeros: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let encoded_coeffs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16,
                    ])));
                    let k: Value<i32> = Rc::new(RefCell::new((*last_nz.borrow())));
                    'loop_: while ((*k.borrow()) >= 1) {
                        let coeff: Value<i16> =
                            Rc::new(RefCell::new((*coeffs.borrow())[(*k.borrow()) as usize]));
                        let is_zero: Value<i32> =
                            Rc::new(RefCell::new(((((*coeff.borrow()) as i32) == 0) as i32)));
                        if ((*k.borrow()) < (*last_nz.borrow())) {
                            let bucket: Value<i32> = Rc::new(RefCell::new(
                                (({
                                    let __idx =
                                        ((*num_nzeros.borrow()).wrapping_sub(1_usize)) as usize;
                                    kNonzeroBuckets_89.with(|rc| rc.borrow()[__idx])
                                }) as i32),
                            ));
                            let is_zero_ctx: Value<i32> = Rc::new(RefCell::new(
                                (((*bucket.borrow()) * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                    + (*k.borrow())),
                            ));
                            let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*c.borrow()).with(|__s| __s.is_zero_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset(((*is_zero_ctx.borrow()) as usize))),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> = (*p.borrow()).clone();
                                let _bit: i32 = (*is_zero.borrow());
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                        }
                        if !((*is_zero.borrow()) != 0) {
                            let sign: Value<i32> = Rc::new(RefCell::new(
                                (if (((*coeff.borrow()) as i32) > 0) {
                                    0
                                } else {
                                    1
                                }),
                            ));
                            let absval: Value<i32> =
                                Rc::new(RefCell::new(if ((*sign.borrow()) != 0) {
                                    -((*coeff.borrow()) as i32)
                                } else {
                                    ((*coeff.borrow()) as i32)
                                }));
                            let k_nat: Value<i32> = Rc::new(RefCell::new(
                                ((elem!((*cur_order.borrow()), (*k.borrow())).read()) as i32),
                            ));
                            let context_type: Value<usize> = Rc::new(RefCell::new(
                                ((elem!((*context_modes.borrow()), (*k_nat.borrow())).read())
                                    as usize),
                            ));
                            let avg_ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
                            let sign_ctx: Value<usize> = Rc::new(RefCell::new(
                                kMaxAverageContext_82.with(|rc| *rc.borrow()),
                            ));
                            if (((*context_type.borrow()) & 1_usize) != 0) && ((*y.borrow()) > 0) {
                                if ((*y.borrow()) > 0) {
                                    let offset: Value<usize> =
                                        Rc::new(RefCell::new((((*k_nat.borrow()) & 7) as usize)));
                                    ({
                                        let _prev: Ptr<i16> = (*prev_row_coeffs.borrow())
                                            .offset((*offset.borrow()) as isize);
                                        let _cur: Ptr<i16> = (encoded_coeffs.as_pointer()
                                            as Ptr<i16>)
                                            .offset((*offset.borrow()) as isize);
                                        let _mult: Ptr<i32> =
                                            ((array_field_ptr!((*c.borrow()), mult_col)
                                                as Ptr<i32>)
                                                .offset(
                                                    ((*offset.borrow()).wrapping_mul(8_usize))
                                                        as isize,
                                                ));
                                        ACPredictContextRow_103(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (avg_ctx.as_pointer()),
                                            (sign_ctx.as_pointer()),
                                        )
                                    });
                                }
                            } else if (((*context_type.borrow()) & 2_usize) != 0)
                                && ((*x.borrow()) > 0)
                            {
                                if ((*x.borrow()) > 0) {
                                    let offset: Value<usize> =
                                        Rc::new(RefCell::new((((*k_nat.borrow()) & !7) as usize)));
                                    ({
                                        let _prev: Ptr<i16> = (*prev_col_coeffs.borrow())
                                            .offset((*offset.borrow()) as isize);
                                        let _cur: Ptr<i16> = (encoded_coeffs.as_pointer()
                                            as Ptr<i16>)
                                            .offset((*offset.borrow()) as isize);
                                        let _mult: Ptr<i32> =
                                            ((array_field_ptr!((*c.borrow()), mult_row)
                                                as Ptr<i32>)
                                                .offset((*offset.borrow()) as isize));
                                        ACPredictContextCol_102(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (avg_ctx.as_pointer()),
                                            (sign_ctx.as_pointer()),
                                        )
                                    });
                                }
                            } else if !((*context_type.borrow()) != 0) {
                                (*avg_ctx.borrow_mut()) = (({
                                    let _vals: Ptr<i32> =
                                        (*prev_abs.borrow()).offset((*k.borrow()) as isize);
                                    let _prev_row_delta: i32 = (*prev_row_delta.borrow());
                                    WeightedAverageContext_98(_vals, _prev_row_delta)
                                })
                                    as usize);
                                (*sign_ctx.borrow_mut()) =
                                    (({ ((elem!((*prev_sgn.borrow()), (*k.borrow())).read()) * 3) }
                                        + {
                                            (elem!(
                                                (*prev_sgn.borrow()),
                                                ((*k.borrow())
                                                    - kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                            )
                                            .read())
                                        }) as usize);
                            }
                            (*sign_ctx.borrow_mut()) = {
                                ((*sign_ctx.borrow()).wrapping_mul(
                                    (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize),
                                ))
                                .wrapping_add(((*k.borrow()) as usize))
                            };
                            let sign_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*c.borrow()).with(|__s| __s.sign_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset((*sign_ctx.borrow()))),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> = (*sign_p.borrow()).clone();
                                let _bit: i32 = (*sign.borrow());
                                brunsli_internal_enc_DataStreamImpl::AddBit(&data_stream, _p, _bit)
                            });
                            elem!((*prev_sgn.borrow()), (*k.borrow()))
                                .write({ ((*sign.borrow()) + 1) });
                            let zdens_ctx: Value<usize> = Rc::new(RefCell::new(
                                (m.with(|__s| __s.context_offset)).wrapping_add(
                                    (({
                                        ZeroDensityContext_96(
                                            (*num_nzeros.borrow()),
                                            ((*k.borrow()) as usize),
                                            ((*cur_ctx_bits.borrow()) as usize),
                                        )
                                    }) as usize),
                                ),
                            ));
                            if ((*absval.borrow()) <= kNumDirectCodes_141.with(|rc| *rc.borrow())) {
                                ({
                                    let _code: usize = (((*absval.borrow()) - 1) as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
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
                                let base_code: Value<i32> = Rc::new(RefCell::new(
                                    (((*absval.borrow())
                                        - kNumDirectCodes_141.with(|rc| *rc.borrow()))
                                        + 1),
                                ));
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    (({ Log2FloorNonZero_74(((*base_code.borrow()) as u32)) }) - 1),
                                ));
                                ({
                                    let _code: usize = ((kNumDirectCodes_141
                                        .with(|rc| *rc.borrow())
                                        + (*nbits.borrow()))
                                        as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
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
                                let extra_bits: Value<i32> = Rc::new(RefCell::new(
                                    ((*base_code.borrow()) - (2 << (*nbits.borrow()))),
                                ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    (((*extra_bits.borrow()) >> (*nbits.borrow())) & 1),
                                ));
                                let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                    (((*c.borrow())
                                        .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                        as Ptr<brunsli_Prob>)
                                        .offset(
                                            ((((*k.borrow()) * 10) + (*nbits.borrow())) as usize),
                                        )),
                                ));
                                ({
                                    let _p: Ptr<brunsli_Prob> = (*p.borrow()).clone();
                                    let _bit: i32 = (*first_extra_bit.borrow());
                                    brunsli_internal_enc_DataStreamImpl::AddBit(
                                        &data_stream,
                                        _p,
                                        _bit,
                                    )
                                });
                                if ((*nbits.borrow()) > 0) {
                                    let left_over_bits: Value<i32> = Rc::new(RefCell::new(
                                        ((*extra_bits.borrow()) & ((1 << (*nbits.borrow())) - 1)),
                                    ));
                                    ({
                                        let _nbits: i32 = (*nbits.borrow());
                                        let _bits: i32 = (*left_over_bits.borrow());
                                        brunsli_internal_enc_DataStreamImpl::AddBits(
                                            &data_stream,
                                            _nbits,
                                            _bits,
                                        )
                                    });
                                }
                            }
                            (*num_nzeros.borrow_mut()).prefix_inc();
                            (*encoded_coeffs.borrow_mut())[(*k_nat.borrow()) as usize] =
                                (*coeff.borrow());
                            elem!((*prev_abs.borrow()), (*k.borrow()))
                                .write({ (*absval.borrow()) });
                        } else {
                            elem!((*prev_sgn.borrow()), (*k.borrow())).write(0);
                            elem!((*prev_abs.borrow()), (*k.borrow())).write(0);
                        }
                        (*k.borrow_mut()).prefix_dec();
                    }
                    if !((*num_nzeros.borrow()) <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
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
                        ((*c.borrow()).with(|__s| __s.prev_num_nonzeros.as_pointer()) as Ptr<u8>),
                        ((*x.borrow()) as usize)
                    )
                    .write(((*num_nzeros.borrow()) as u8));
                    (*block_state.borrow_mut()).prefix_inc();
                    (*coeffs_in.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*prev_sgn.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*prev_abs.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*prev_row_coeffs.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*prev_col_coeffs.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                (*prev_row_delta.borrow_mut()) *= -1_i32;
                {
                    (*iy.borrow_mut()).prefix_inc();
                    (*y.borrow_mut()).prefix_inc()
                };
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*mcu_y.borrow_mut()).prefix_inc();
    }
}
pub fn PrepareEntropyCodes_189(
    state: Ptr<brunsli_internal_enc_State>,
) -> Option<Value<brunsli_internal_enc_EntropyCodes>> {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let group_context_offsets: Value<Vec<u64>> = Rc::new(RefCell::new(
        (0..((1_usize).wrapping_add((*num_components.borrow()))) as usize)
            .map(|_| <u64>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ({
            (*elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                (*i.borrow())
            )
            .upgrade()
            .deref())
            .context_offset
        } as u64);
        elem!(
            (group_context_offsets.as_pointer() as Ptr<u64>),
            (*i.borrow()).wrapping_add(1_usize)
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    return ({
        brunsli_internal_enc_EntropySourceImpl::Finish(
            &field_ptr!((*state.borrow()), entropy_source),
            group_context_offsets.as_pointer(),
        )
    })
    .take();
}
pub fn BrunsliSerialize_190(
    state: Ptr<brunsli_internal_enc_State>,
    jpg: Ptr<brunsli_JPEGData>,
    skip_sections: u32,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let skip_sections: Value<u32> = Rc::new(RefCell::new(skip_sections));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    let ok: Value<bool> = Rc::new(RefCell::new(true));
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
            let tag: Value<u8> = Rc::new(RefCell::new(tag));
            let fn_: Value<
                FnPtr<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                >,
            > = Rc::new(RefCell::new(fn_));
            let size: Value<usize> = Rc::new(RefCell::new(size));
            return ({
                let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                let _s: Ptr<brunsli_internal_enc_State> = (state.read()).clone();
                let _tag: u8 = (*tag.borrow());
                let _write_section: FnPtr<
                    fn(
                        Ptr<brunsli_JPEGData>,
                        Ptr<brunsli_internal_enc_State>,
                        Ptr<u8>,
                        Ptr<usize>,
                    ) -> bool,
                > = (*fn_.borrow()).clone();
                let _section_size_bytes: usize = (*size.borrow());
                let _len: usize = ((len.read()).read());
                let _data: Ptr<u8> = (data.read()).clone();
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
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliSignatureTag_30.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _len: usize = ((*len.borrow()).read());
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (pos.as_pointer());
            EncodeSignature_171(_len, _data, _pos)
        });
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliHeaderTag_31.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliMetaDataTag_32.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
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
        if !(*ok.borrow()) {
            return false;
        }
    }
    (*len.borrow()).write({ (*pos.borrow()) });
    return true;
}
pub fn BrunsliEncodeJpeg_191(jpg: Ptr<brunsli_JPEGData>, data: Ptr<u8>, len: Ptr<usize>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let state: Value<brunsli_internal_enc_State> =
        Rc::new(RefCell::new(<brunsli_internal_enc_State>::default()));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        { (*state.borrow()).meta.as_pointer() };
    let num_components: Value<usize> = Rc::new(RefCell::new(
        (*jpg.with(|__s| __s.components.clone()).borrow()).len(),
    ));
    (*state.borrow_mut()).use_legacy_context_model = !((jpg.with(|__s| __s.version) & 2) != 0);
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        CalculateMeta_186(_jpg, _state)
    }) {
        return false;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ({
            SampleNumNonZeros_181(
                (({ (*state.borrow()).meta.as_pointer() }
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()))),
            )
        });
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                (*i.borrow())
            ),
            approx_total_nonzeros
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ({
            SelectContextBits_183(
                ({
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        (*i.borrow())
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
                (*i.borrow())
            ),
            context_bits
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let num_contexts: Value<usize> = Rc::new(RefCell::new((*num_components.borrow())));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                (*i.borrow())
            ),
            context_offset
        )
        .write((*num_contexts.borrow()));
        {
            let rhs_0 = (*num_contexts.borrow()).wrapping_add(
                (({
                    let __idx = ({
                        (*elem!(
                            (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                                as Ptr<brunsli_internal_enc_ComponentMeta>),
                            (*i.borrow())
                        )
                        .upgrade()
                        .deref())
                        .context_bits
                    }) as usize;
                    kNumNonzeroContextSkip_94.with(|rc| rc.borrow()[__idx])
                }) as usize),
            );
            (*num_contexts.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).prefix_inc();
    }
    (*state.borrow_mut()).num_contexts = (*num_contexts.borrow());
    let dc_prediction_errors: Value<Vec<Value<Vec<i16>>>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <Value<Vec<i16>>>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        {
            let __a0 = (({
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        (*i.borrow())
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
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .height_in_blocks
                }
            }) as usize) as usize;
            elem!(
                (dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>),
                (*i.borrow())
            )
            .with_mut(|__v: &mut Value<Vec<i16>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <i16>::default())
            })
        };
        let __rhs = ((dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>)
            .offset((*i.borrow()))
            .upgrade()
            .deref()
            .as_pointer() as Ptr<i16>);
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                (*i.borrow())
            ),
            dc_prediction_errors
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    if !({ PredictDCCoeffs_185((state.as_pointer())) }) {
        return false;
    }
    let block_state: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <Value<Vec<u8>>>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        {
            let __a0 = (({
                {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_enc_ComponentMeta>),
                        (*i.borrow())
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
                        (*i.borrow())
                    )
                    .upgrade()
                    .deref())
                    .height_in_blocks
                }
            }) as usize) as usize;
            elem!(
                (block_state.as_pointer() as Ptr<Value<Vec<u8>>>),
                (*i.borrow())
            )
            .with_mut(|__v: &mut Value<Vec<u8>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
            })
        };
        let __rhs = ((block_state.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()))
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>);
        field!(
            elem!(
                (Ptr::<Vec<brunsli_internal_enc_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_enc_ComponentMeta>),
                (*i.borrow())
            ),
            block_state
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    ({ EncodeDC_187((state.as_pointer())) });
    ({ EncodeAC_188((state.as_pointer())) });
    let entropy_codes: Value<Option<Value<brunsli_internal_enc_EntropyCodes>>> = Rc::new(
        RefCell::new(({ PrepareEntropyCodes_189((state.as_pointer())) }).take()),
    );
    (*state.borrow_mut()).entropy_codes = (*entropy_codes.borrow()).as_pointer();
    return ({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _len: Ptr<usize> = (*len.borrow()).clone();
        BrunsliSerialize_190(_state, _jpg, 0_u32, _data, _len)
    });
}
thread_local!(
    pub static kMaxBypassHeaderSize_192: Value<usize> = Rc::new(RefCell::new(((5 * 6) as usize)));
);
pub fn GetBrunsliBypassSize_193(jpg_size: usize) -> usize {
    let jpg_size: Value<usize> = Rc::new(RefCell::new(jpg_size));
    return ((*jpg_size.borrow()).wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow())))
        .wrapping_add(kMaxBypassHeaderSize_192.with(|rc| *rc.borrow()));
}
pub fn EncodeOriginalJpg_194(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    &(*state.borrow_mut());
    if ((jpg.with(|__s| __s.original_jpg.clone())).is_null())
        || ({ jpg.with(|__s| __s.original_jpg_size) } > { ((*len.borrow()).read()) })
    {
        return false;
    }
    {
        (*data.borrow()).to_any().memcpy(
            &(jpg.with(|__s| __s.original_jpg.clone()) as Ptr<u8>).to_any(),
            jpg.with(|__s| __s.original_jpg_size) as usize,
        );
        (*data.borrow()).to_any()
    };
    (*len.borrow()).write({ jpg.with(|__s| __s.original_jpg_size) });
    return true;
}
pub fn BrunsliEncodeJpegBypass_195(
    jpg_data: Ptr<u8>,
    jpg_data_len: usize,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let jpg_data: Value<Ptr<u8>> = Rc::new(RefCell::new(jpg_data));
    let jpg_data_len: Value<usize> = Rc::new(RefCell::new(jpg_data_len));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if !({
        let _len: usize = ((*len.borrow()).read());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeSignature_171(_len, _data, _pos)
    }) {
        return false;
    }
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
    if !({
        ReadJpeg_196(
            (*jpg_data.borrow()).clone(),
            (*jpg_data_len.borrow()),
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
    (*jpg.borrow_mut()).original_jpg = (*jpg_data.borrow()).clone();
    (*jpg.borrow_mut()).original_jpg_size = (*jpg_data_len.borrow());
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
        let _len: usize = ((*len.borrow()).read());
        let _data: Ptr<u8> = (*data.borrow()).clone();
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
        let _section_size_bytes: usize = ({ Base128Size_146((*jpg_data_len.borrow())) });
        let _len: usize = ((*len.borrow()).read());
        let _data: Ptr<u8> = (*data.borrow()).clone();
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
    (*len.borrow()).write({ (*pos.borrow()) });
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
    pub fn new(count: u32, left: i16, right: i16) -> Self {
        let count: Value<u32> = Rc::new(RefCell::new(count));
        let left: Value<i16> = Rc::new(RefCell::new(left));
        let right: Value<i16> = Rc::new(RefCell::new(right));
        let __this: Value<brunsli_HuffmanTree> = Rc::new(RefCell::new(Self {
            total_count: (*count.borrow()),
            index_left: (*left.borrow()),
            index_right_or_value: (*right.borrow()),
        }));
        let this: Ptr<brunsli_HuffmanTree> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn StoreVarLenUint8_197(n: usize, storage: Ptr<brunsli_Storage>) {
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*n.borrow()) == 0_usize) {
        ({ WriteBits_120(1_usize, 0_u64, (*storage.borrow()).clone()) });
    } else {
        ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
        let nbits: Value<usize> = Rc::new(RefCell::new(
            (({ Log2FloorNonZero_74(((*n.borrow()) as u32)) }) as usize),
        ));
        ({
            WriteBits_120(
                3_usize,
                ((*nbits.borrow()) as u64),
                (*storage.borrow()).clone(),
            )
        });
        ({
            let _n_bits: usize = (*nbits.borrow());
            let _bits: u64 = ((*n.borrow()).wrapping_sub((1_usize << (*nbits.borrow()))) as u64);
            WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
        });
    }
}
pub fn IndexOf_198(v: Ptr<Vec<u32>>, value: u32) -> usize {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*v.upgrade().deref()).len() }) {
        if ({ (elem!((Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), (*i.borrow())).read()) } == {
            (*value.borrow())
        }) {
            return (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*i.borrow());
}
pub fn MoveToFront_199(v: Ptr<Vec<u32>>, index: usize) {
    let v: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(v));
    let index: Value<usize> = Rc::new(RefCell::new(index));
    let value: Value<u32> = Rc::new(RefCell::new(
        (elem!(
            ((Ptr::<Vec<u32>>::decay(&(*v.borrow()))) as Ptr<u32>),
            (*index.borrow())
        )
        .read()),
    ));
    let i: Value<usize> = Rc::new(RefCell::new((*index.borrow())));
    'loop_: while ((*i.borrow()) != 0_usize) {
        let __rhs = (elem!(
            ((Ptr::<Vec<u32>>::decay(&(*v.borrow()))) as Ptr<u32>),
            (*i.borrow()).wrapping_sub(1_usize)
        )
        .read());
        elem!(
            ((Ptr::<Vec<u32>>::decay(&(*v.borrow()))) as Ptr<u32>),
            (*i.borrow())
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_dec();
    }
    elem!(
        ((Ptr::<Vec<u32>>::decay(&(*v.borrow()))) as Ptr<u32>),
        0_usize
    )
    .write((*value.borrow()));
}
pub fn MoveToFrontTransform_200(v: Ptr<Vec<u32>>) -> Vec<u32> {
    if (*v.upgrade().deref()).is_empty() {
        return (*v.upgrade().deref()).clone();
    }
    let max_value: Value<u32> = Rc::new(RefCell::new(
        ({
            let __count = (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>)
                .to_end()
                .get_offset()
                - (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>).get_offset();
            let max_index = PtrValueIter::new(&(Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), __count)
                .enumerate()
                .max_by_key(|&(_, val)| val)
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            (Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>) + max_index
        }
        .read()),
    ));
    let mtf: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(((*max_value.borrow()).wrapping_add(1_u32)) as usize) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) <= (*max_value.borrow())) {
        let __rhs = (*i.borrow());
        elem!((mtf.as_pointer() as Ptr<u32>), ((*i.borrow()) as usize)).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let result: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..((*v.upgrade().deref()).len()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*v.upgrade().deref()).len() }) {
        let index: Value<usize> = Rc::new(RefCell::new(
            ({
                let _v: Ptr<Vec<u32>> = mtf.as_pointer();
                let _value: u32 =
                    (elem!((Ptr::<Vec<u32>>::decay(&(v)) as Ptr<u32>), (*i.borrow())).read());
                IndexOf_198(_v, _value)
            }),
        ));
        if !((*index.borrow()) < (*mtf.borrow()).len()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"context_map_encode.cc"),
                    60,
                    Ptr::<i8>::from_string_literal(b"MoveToFrontTransform"),
                )
            });
            'loop_: while true {}
        };
        elem!((result.as_pointer() as Ptr<u32>), (*i.borrow())).write(((*index.borrow()) as u32));
        ({ MoveToFront_199((mtf.as_pointer()), (*index.borrow())) });
        (*i.borrow_mut()).prefix_inc();
    }
    return std::mem::take(&mut (*result.borrow_mut()));
}
pub fn RunLengthCodeZeros_201(
    v_in: Ptr<Vec<u32>>,
    max_run_length_prefix: Ptr<u32>,
    v_out: Ptr<Vec<u32>>,
    extra_bits: Ptr<Vec<u32>>,
) {
    let max_run_length_prefix: Value<Ptr<u32>> = Rc::new(RefCell::new(max_run_length_prefix));
    let v_out: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(v_out));
    let extra_bits: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(extra_bits));
    let max_reps: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*v_in.upgrade().deref()).len() }) {
        'loop_: while ({ (*i.borrow()) } < { (*v_in.upgrade().deref()).len() })
            && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), (*i.borrow())).read())
                != 0_u32)
        {
            (*i.borrow_mut()).prefix_inc();
        }
        let i0: Value<usize> = Rc::new(RefCell::new((*i.borrow())));
        'loop_: while ({ (*i.borrow()) } < { (*v_in.upgrade().deref()).len() })
            && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), (*i.borrow())).read())
                == 0_u32)
        {
            (*i.borrow_mut()).prefix_inc();
        }
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                ((*i.borrow()).wrapping_sub((*i0.borrow())) as u64),
            ));
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
    let __rhs =
        (if max_prefix.as_pointer().read() <= (*max_run_length_prefix.borrow()).clone().read() {
            max_prefix.as_pointer()
        } else {
            (*max_run_length_prefix.borrow()).clone()
        }
        .read());
    (*max_prefix.borrow_mut()) = __rhs;
    (*max_run_length_prefix.borrow()).write({ (*max_prefix.borrow()) });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*v_in.upgrade().deref()).len() }) {
        if ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), (*i.borrow())).read()) != 0_u32) {
            {
                let __a1 = (elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), (*i.borrow()))
                    .read())
                .wrapping_add(((*max_run_length_prefix.borrow()).read()));
                (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
            };
            {
                let __a1 = 0_u32;
                (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
            };
            (*i.borrow_mut()).prefix_inc();
        } else {
            let reps: Value<u32> = Rc::new(RefCell::new(1_u32));
            let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
            'loop_: while ({ (*k.borrow()) } < { (*v_in.upgrade().deref()).len() })
                && ((elem!((Ptr::<Vec<u32>>::decay(&(v_in)) as Ptr<u32>), (*k.borrow())).read())
                    == 0_u32)
            {
                (*reps.borrow_mut()).prefix_inc();
                (*k.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()) = { (*i.borrow()).wrapping_add(((*reps.borrow()) as usize)) };
            'loop_: while ((*reps.borrow()) != 0_u32) {
                if ((*reps.borrow()) < (2_u32 << (*max_prefix.borrow()))) {
                    let run_length_prefix: Value<u32> = Rc::new(RefCell::new(
                        (({ Log2FloorNonZero_74((*reps.borrow())) }) as u32),
                    ));
                    {
                        let a0_clone = (*run_length_prefix.borrow()).clone();
                        (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    {
                        let __a1 =
                            (*reps.borrow()).wrapping_sub((1_u32 << (*run_length_prefix.borrow())));
                        (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
                    };
                    break;
                } else {
                    {
                        let a0_clone = (*max_prefix.borrow()).clone();
                        (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    {
                        let __a1 = (1_u32 << (*max_prefix.borrow())).wrapping_sub((1_u32 as u32));
                        (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(__a1))
                    };
                    (*reps.borrow_mut()) = {
                        (*reps.borrow()).wrapping_sub(
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
    num_clusters: usize,
    storage: Ptr<brunsli_Storage>,
) {
    let num_clusters: Value<usize> = Rc::new(RefCell::new(num_clusters));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        StoreVarLenUint8_197(
            (*num_clusters.borrow()).wrapping_sub(1_usize),
            (*storage.borrow()).clone(),
        )
    });
    if ((*num_clusters.borrow()) == 1_usize) {
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
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*rle_symbols.borrow()).len()) {
        (*symbol_histogram.borrow_mut())
            [(elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow())).read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let use_rle: Value<bool> = Rc::new(RefCell::new(((*max_run_length_prefix.borrow()) > 0_u32)));
    ({
        WriteBits_120(
            1_usize,
            ((*use_rle.borrow()) as u64),
            (*storage.borrow()).clone(),
        )
    });
    if (*use_rle.borrow()) {
        ({
            WriteBits_120(
                4_usize,
                (((*max_run_length_prefix.borrow()).wrapping_sub(1_u32)) as u64),
                (*storage.borrow()).clone(),
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
            (*num_clusters.borrow()).wrapping_add(((*max_run_length_prefix.borrow()) as usize)),
            (bit_depths.as_pointer() as Ptr<u8>),
            (bit_codes.as_pointer() as Ptr<u16>),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*rle_symbols.borrow()).len()) {
        ({
            let _n_bits: usize = ((*bit_depths.borrow())
                [(elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow())).read()) as usize]
                as usize);
            let _bits: u64 = ((*bit_codes.borrow())
                [(elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow())).read()) as usize]
                as u64);
            WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
        });
        if ((elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow())).read()) > 0_u32)
            && ((elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow())).read())
                <= (*max_run_length_prefix.borrow()))
        {
            ({
                let _n_bits: usize = ((elem!((rle_symbols.as_pointer() as Ptr<u32>), (*i.borrow()))
                    .read()) as usize);
                let _bits: u64 =
                    ((elem!((extra_bits.as_pointer() as Ptr<u32>), (*i.borrow())).read()) as u64);
                WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
            });
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
}
pub fn GetPopulationCountPrecision_203(logcount: u32) -> u32 {
    let logcount: Value<u32> = Rc::new(RefCell::new(logcount));
    return (((*logcount.borrow()).wrapping_add(1_u32)) >> 1);
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
pub fn SmallestIncrement_208(count: i32) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    if !((*count.borrow()) > 0) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                39,
                Ptr::<i8>::from_string_literal(b"SmallestIncrement"),
            )
        });
        'loop_: while true {}
    };
    let bits: Value<i32> = Rc::new(RefCell::new(
        ({ Log2FloorNonZero_74(((*count.borrow()) as u32)) }),
    ));
    let drop_bits: Value<i32> = Rc::new(RefCell::new(
        ((((*bits.borrow()) as u32)
            .wrapping_sub(({ GetPopulationCountPrecision_203(((*bits.borrow()) as u32)) })))
            as i32),
    ));
    return (1 << (*drop_bits.borrow()));
}
pub fn RebalanceHistogram_209(
    targets: Ptr<f32>,
    max_symbol: i32,
    table_size: i32,
    omit_pos: Ptr<i32>,
    counts: Ptr<i32>,
) -> bool {
    let targets: Value<Ptr<f32>> = Rc::new(RefCell::new(targets));
    let max_symbol: Value<i32> = Rc::new(RefCell::new(max_symbol));
    let table_size: Value<i32> = Rc::new(RefCell::new(table_size));
    let omit_pos: Value<Ptr<i32>> = Rc::new(RefCell::new(omit_pos));
    let counts: Value<Ptr<i32>> = Rc::new(RefCell::new(counts));
    if !((*table_size.borrow()) >= 2) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                48,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let sum_nonrounded: Value<f32> = Rc::new(RefCell::new((0.0E+0 as f32)));
    let remainder_pos: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let remainder_log: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        if ((elem!((*targets.borrow()), (*n.borrow())).read()) > 0_f32) {
            (*sum_nonrounded.borrow_mut()) +=
                { (elem!((*targets.borrow()), (*n.borrow())).read()) };
            elem!((*counts.borrow()), (*n.borrow())).write({
                (((((elem!((*targets.borrow()), (*n.borrow())).read()) as f64) + 5.0E-1) as u32)
                    as i32)
            });
            if ((elem!((*counts.borrow()), (*n.borrow())).read()) == 0) {
                elem!((*counts.borrow()), (*n.borrow())).write(1);
            }
            if ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } == { (*table_size.borrow()) })
            {
                elem!((*counts.borrow()), (*n.borrow())).write({ ((*table_size.borrow()) - 1) });
            }
            let inc: Value<i32> = Rc::new(RefCell::new(
                ({ SmallestIncrement_208((elem!((*counts.borrow()), (*n.borrow())).read())) }),
            ));
            {
                let _ptr = elem!((*counts.borrow()), (*n.borrow()));
                _ptr.write(
                    _ptr.read() - {
                        ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } & {
                            ((*inc.borrow()) - 1)
                        })
                    },
                )
            };
            let target: Value<f32> = Rc::new(RefCell::new(if false {
                ((*sum_nonrounded.borrow()) - ((*sum.borrow()) as f32))
            } else {
                (elem!((*targets.borrow()), (*n.borrow())).read())
            }));
            if ((elem!((*counts.borrow()), (*n.borrow())).read()) == 0)
                || (({ (*target.borrow()) } > {
                    (({ (elem!((*counts.borrow()), (*n.borrow())).read()) } + {
                        ((*inc.borrow()) / 2)
                    }) as f32)
                }) && ({
                    ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } + { (*inc.borrow()) })
                } < { (*table_size.borrow()) }))
            {
                {
                    let _ptr = elem!((*counts.borrow()), (*n.borrow()));
                    _ptr.write(_ptr.read() + { (*inc.borrow()) })
                };
            }
            (*sum.borrow_mut()) += { (elem!((*counts.borrow()), (*n.borrow())).read()) };
            let count_log: Value<i32> = Rc::new(RefCell::new(
                ({
                    Log2FloorNonZero_74(((elem!((*counts.borrow()), (*n.borrow())).read()) as u32))
                }),
            ));
            if ((*count_log.borrow()) > (*remainder_log.borrow())) {
                (*remainder_pos.borrow_mut()) = (*n.borrow());
                (*remainder_log.borrow_mut()) = (*count_log.borrow());
            }
        }
        (*n.borrow_mut()).prefix_inc();
    }
    if !((*remainder_pos.borrow()) != -1_i32) {
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
        let _ptr = elem!((*counts.borrow()), (*remainder_pos.borrow()));
        _ptr.write(_ptr.read() - { ((*sum.borrow()) - (*table_size.borrow())) })
    };
    (*omit_pos.borrow()).write({ (*remainder_pos.borrow()) });
    return ((elem!((*counts.borrow()), (*remainder_pos.borrow())).read()) > 0);
}
pub fn RebalanceHistogram_210(
    targets: Ptr<f32>,
    max_symbol: i32,
    table_size: i32,
    omit_pos: Ptr<i32>,
    counts: Ptr<i32>,
) -> bool {
    let targets: Value<Ptr<f32>> = Rc::new(RefCell::new(targets));
    let max_symbol: Value<i32> = Rc::new(RefCell::new(max_symbol));
    let table_size: Value<i32> = Rc::new(RefCell::new(table_size));
    let omit_pos: Value<Ptr<i32>> = Rc::new(RefCell::new(omit_pos));
    let counts: Value<Ptr<i32>> = Rc::new(RefCell::new(counts));
    if !((*table_size.borrow()) >= 2) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                48,
                Ptr::<i8>::from_string_literal(b"RebalanceHistogram"),
            )
        });
        'loop_: while true {}
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let sum_nonrounded: Value<f32> = Rc::new(RefCell::new((0.0E+0 as f32)));
    let remainder_pos: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let remainder_log: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        if ((elem!((*targets.borrow()), (*n.borrow())).read()) > 0_f32) {
            (*sum_nonrounded.borrow_mut()) +=
                { (elem!((*targets.borrow()), (*n.borrow())).read()) };
            elem!((*counts.borrow()), (*n.borrow())).write({
                (((((elem!((*targets.borrow()), (*n.borrow())).read()) as f64) + 5.0E-1) as u32)
                    as i32)
            });
            if ((elem!((*counts.borrow()), (*n.borrow())).read()) == 0) {
                elem!((*counts.borrow()), (*n.borrow())).write(1);
            }
            if ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } == { (*table_size.borrow()) })
            {
                elem!((*counts.borrow()), (*n.borrow())).write({ ((*table_size.borrow()) - 1) });
            }
            let inc: Value<i32> = Rc::new(RefCell::new(
                ({ SmallestIncrement_208((elem!((*counts.borrow()), (*n.borrow())).read())) }),
            ));
            {
                let _ptr = elem!((*counts.borrow()), (*n.borrow()));
                _ptr.write(
                    _ptr.read() - {
                        ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } & {
                            ((*inc.borrow()) - 1)
                        })
                    },
                )
            };
            let target: Value<f32> = Rc::new(RefCell::new(if true {
                ((*sum_nonrounded.borrow()) - ((*sum.borrow()) as f32))
            } else {
                (elem!((*targets.borrow()), (*n.borrow())).read())
            }));
            if ((elem!((*counts.borrow()), (*n.borrow())).read()) == 0)
                || (({ (*target.borrow()) } > {
                    (({ (elem!((*counts.borrow()), (*n.borrow())).read()) } + {
                        ((*inc.borrow()) / 2)
                    }) as f32)
                }) && ({
                    ({ (elem!((*counts.borrow()), (*n.borrow())).read()) } + { (*inc.borrow()) })
                } < { (*table_size.borrow()) }))
            {
                {
                    let _ptr = elem!((*counts.borrow()), (*n.borrow()));
                    _ptr.write(_ptr.read() + { (*inc.borrow()) })
                };
            }
            (*sum.borrow_mut()) += { (elem!((*counts.borrow()), (*n.borrow())).read()) };
            let count_log: Value<i32> = Rc::new(RefCell::new(
                ({
                    Log2FloorNonZero_74(((elem!((*counts.borrow()), (*n.borrow())).read()) as u32))
                }),
            ));
            if ((*count_log.borrow()) > (*remainder_log.borrow())) {
                (*remainder_pos.borrow_mut()) = (*n.borrow());
                (*remainder_log.borrow_mut()) = (*count_log.borrow());
            }
        }
        (*n.borrow_mut()).prefix_inc();
    }
    if !((*remainder_pos.borrow()) != -1_i32) {
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
        let _ptr = elem!((*counts.borrow()), (*remainder_pos.borrow()));
        _ptr.write(_ptr.read() - { ((*sum.borrow()) - (*table_size.borrow())) })
    };
    (*omit_pos.borrow()).write({ (*remainder_pos.borrow()) });
    return ((elem!((*counts.borrow()), (*remainder_pos.borrow())).read()) > 0);
}
pub fn NormalizeCounts_124(
    counts: Ptr<i32>,
    omit_pos: Ptr<i32>,
    length: i32,
    precision_bits: i32,
    num_symbols: Ptr<i32>,
    symbols: Ptr<i32>,
) {
    let counts: Value<Ptr<i32>> = Rc::new(RefCell::new(counts));
    let omit_pos: Value<Ptr<i32>> = Rc::new(RefCell::new(omit_pos));
    let length: Value<i32> = Rc::new(RefCell::new(length));
    let precision_bits: Value<i32> = Rc::new(RefCell::new(precision_bits));
    let num_symbols: Value<Ptr<i32>> = Rc::new(RefCell::new(num_symbols));
    let symbols: Value<Ptr<i32>> = Rc::new(RefCell::new(symbols));
    if !((*precision_bits.borrow()) > 0) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                89,
                Ptr::<i8>::from_string_literal(b"NormalizeCounts"),
            )
        });
        'loop_: while true {}
    };
    let table_size: Value<i32> = Rc::new(RefCell::new((1 << (*precision_bits.borrow()))));
    let total: Value<u64> = Rc::new(RefCell::new(0_u64));
    let max_symbol: Value<i32> = Rc::new(RefCell::new(0));
    let symbol_count: Value<i32> = Rc::new(RefCell::new(0));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*length.borrow())) {
        (*total.borrow_mut()) = {
            (*total.borrow())
                .wrapping_add(((elem!((*counts.borrow()), (*n.borrow())).read()) as u64))
        };
        if ((elem!((*counts.borrow()), (*n.borrow())).read()) > 0) {
            if ((*symbol_count.borrow()) < kMaxNumSymbolsForSmallCode_121.with(|rc| *rc.borrow())) {
                elem!((*symbols.borrow()), (*symbol_count.borrow())).write({ (*n.borrow()) });
            }
            (*symbol_count.borrow_mut()).prefix_inc();
            (*max_symbol.borrow_mut()) = ((*n.borrow()) + 1);
        }
        (*n.borrow_mut()).prefix_inc();
    }
    (*num_symbols.borrow()).write({ (*symbol_count.borrow()) });
    if ((*symbol_count.borrow()) == 0) {
        return;
    }
    if ((*symbol_count.borrow()) == 1) {
        elem!((*counts.borrow()), (elem!((*symbols.borrow()), 0).read()))
            .write({ (*table_size.borrow()) });
        return;
    }
    if !((*symbol_count.borrow()) <= (*table_size.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                112,
                Ptr::<i8>::from_string_literal(b"NormalizeCounts"),
            )
        });
        'loop_: while true {}
    };
    let norm: Value<f32> = Rc::new(RefCell::new(
        ((1.0E+0 * ((*table_size.borrow()) as f32)) / ((*total.borrow()) as f32)),
    ));
    let targets: Value<Box<[f32]>> =
        Rc::new(RefCell::new((0..18).map(|_| 0_f32).collect::<Box<[f32]>>()));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        (*targets.borrow_mut())[(*n.borrow()) as usize] = {
            ({ (*norm.borrow()) } * { ((elem!((*counts.borrow()), (*n.borrow())).read()) as f32) })
        };
        (*n.borrow_mut()).prefix_inc();
    }
    if !({
        let _max_symbol: i32 = (*max_symbol.borrow());
        let _table_size: i32 = (*table_size.borrow());
        let _omit_pos: Ptr<i32> = (*omit_pos.borrow()).clone();
        let _counts: Ptr<i32> = (*counts.borrow()).clone();
        RebalanceHistogram_209(
            (targets.as_pointer() as Ptr<f32>),
            _max_symbol,
            _table_size,
            _omit_pos,
            _counts,
        )
    }) {
        if !({
            let _max_symbol: i32 = (*max_symbol.borrow());
            let _table_size: i32 = (*table_size.borrow());
            let _omit_pos: Ptr<i32> = (*omit_pos.borrow()).clone();
            let _counts: Ptr<i32> = (*counts.borrow()).clone();
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
    counts: Ptr<i32>,
    omit_pos: i32,
    num_symbols: i32,
    symbols: Ptr<i32>,
    storage: Ptr<brunsli_Storage>,
) {
    let counts: Value<Ptr<i32>> = Rc::new(RefCell::new(counts));
    let omit_pos: Value<i32> = Rc::new(RefCell::new(omit_pos));
    let num_symbols: Value<i32> = Rc::new(RefCell::new(num_symbols));
    let symbols: Value<Ptr<i32>> = Rc::new(RefCell::new(symbols));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let max_bits: Value<i32> = Rc::new(RefCell::new(5));
    if ((*num_symbols.borrow()) <= 2) {
        ({ WriteBits_120(1_usize, 1_u64, (*storage.borrow()).clone()) });
        if ((*num_symbols.borrow()) == 0) {
            ({
                WriteBits_120(
                    (((*max_bits.borrow()) + 1) as usize),
                    0_u64,
                    (*storage.borrow()).clone(),
                )
            });
        } else {
            ({
                WriteBits_120(
                    1_usize,
                    (((*num_symbols.borrow()) - 1) as u64),
                    (*storage.borrow()).clone(),
                )
            });
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
                ({
                    WriteBits_120(
                        ((*max_bits.borrow()) as usize),
                        ((elem!((*symbols.borrow()), (*i.borrow())).read()) as u64),
                        (*storage.borrow()).clone(),
                    )
                });
                (*i.borrow_mut()).prefix_inc();
            }
        }
        if ((*num_symbols.borrow()) == 2) {
            ({
                WriteBits_120(
                    (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as usize),
                    ((elem!((*counts.borrow()), (elem!((*symbols.borrow()), 0).read())).read())
                        as u64),
                    (*storage.borrow()).clone(),
                )
            });
        }
    } else {
        ({ WriteBits_120(1_usize, 0_u64, (*storage.borrow()).clone()) });
        let length: Value<i32> = Rc::new(RefCell::new(0));
        let logcounts: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let omit_log: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            if !({ (elem!((*counts.borrow()), (*i.borrow())).read()) } <= {
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
            if !((elem!((*counts.borrow()), (*i.borrow())).read()) >= 0) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                        156,
                        Ptr::<i8>::from_string_literal(b"EncodeCounts"),
                    )
                });
                'loop_: while true {}
            };
            if ((*i.borrow()) == (*omit_pos.borrow())) {
                (*length.borrow_mut()) = ((*i.borrow()) + 1);
            } else if ((elem!((*counts.borrow()), (*i.borrow())).read()) > 0) {
                let __rhs = (({
                    Log2FloorNonZero_74(((elem!((*counts.borrow()), (*i.borrow())).read()) as u32))
                }) + 1);
                (*logcounts.borrow_mut())[(*i.borrow()) as usize] = __rhs;
                (*length.borrow_mut()) = ((*i.borrow()) + 1);
                if ((*i.borrow()) < (*omit_pos.borrow())) {
                    let __rhs = {
                        let __tmp_1: Value<i32> = Rc::new(RefCell::new(
                            ((*logcounts.borrow())[(*i.borrow()) as usize] + 1),
                        ));
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
                        >= (logcounts.as_pointer() as Ptr<i32>)
                            .offset((*i.borrow()))
                            .read()
                    {
                        omit_log.as_pointer()
                    } else {
                        (logcounts.as_pointer() as Ptr<i32>).offset((*i.borrow()))
                    }
                    .read());
                    (*omit_log.borrow_mut()) = __rhs;
                }
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*logcounts.borrow_mut())[(*omit_pos.borrow()) as usize] = (*omit_log.borrow());
        ({
            let _n_bits: usize = (({
                let __idx = ((*length.borrow()) - 3) as usize;
                kHistogramLengthBitLengths_204.with(|rc| rc.borrow()[__idx])
            }) as usize);
            let _bits: u64 = (({
                let __idx = ((*length.borrow()) - 3) as usize;
                kHistogramLengthSymbols_205.with(|rc| rc.borrow()[__idx])
            }) as u64);
            WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
        });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*length.borrow())) {
            ({
                let _n_bits: usize = (({
                    let __idx = ((*logcounts.borrow())[(*i.borrow()) as usize]) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as usize);
                let _bits: u64 = (({
                    let __idx = ((*logcounts.borrow())[(*i.borrow()) as usize]) as usize;
                    kLogCountSymbols_207.with(|rc| rc.borrow()[__idx])
                }) as u64);
                WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*length.borrow())) {
            if ((*logcounts.borrow())[(*i.borrow()) as usize] > 1)
                && ((*i.borrow()) != (*omit_pos.borrow()))
            {
                let bitcount: Value<i32> = Rc::new(RefCell::new(
                    (({
                        GetPopulationCountPrecision_203(
                            (((*logcounts.borrow())[(*i.borrow()) as usize] - 1) as u32),
                        )
                    }) as i32),
                ));
                let drop_bits: Value<i32> = Rc::new(RefCell::new(
                    (((*logcounts.borrow())[(*i.borrow()) as usize] - 1) - (*bitcount.borrow())),
                ));
                if !(({ (elem!((*counts.borrow()), (*i.borrow())).read()) } & {
                    ((1 << (*drop_bits.borrow())) - 1)
                }) == 0)
                {
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
                    let _n_bits: usize = ((*bitcount.borrow()) as usize);
                    let _bits: u64 = (({
                        ({ (elem!((*counts.borrow()), (*i.borrow())).read()) } >> {
                            (*drop_bits.borrow())
                        })
                    } - { (1 << (*bitcount.borrow())) })
                        as u64);
                    WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub fn PopulationCost_131(data: Ptr<i32>, total_count: i32) -> f64 {
    let data: Value<Ptr<i32>> = Rc::new(RefCell::new(data));
    let total_count: Value<i32> = Rc::new(RefCell::new(total_count));
    if ((*total_count.borrow()) == 0) {
        return 7_f64;
    }
    let entropy_bits: Value<f64> = Rc::new(RefCell::new(
        (((*total_count.borrow()) * BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) as f64),
    ));
    let histogram_bits: Value<i32> = Rc::new(RefCell::new(0));
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let length: Value<i32> = Rc::new(RefCell::new(0));
    if ((*total_count.borrow()) > BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow())) {
        let total: Value<u64> = Rc::new(RefCell::new(((*total_count.borrow()) as u64)));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            if ((elem!((*data.borrow()), (*i.borrow())).read()) > 0) {
                (*count.borrow_mut()).prefix_inc();
                (*length.borrow_mut()) = (*i.borrow());
            }
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*count.borrow()) == 1) {
            return 7_f64;
        }
        (*length.borrow_mut()).prefix_inc();
        let max0: Value<u64> = Rc::new(RefCell::new(
            (((*total.borrow()).wrapping_mul(((*length.borrow()) as u64)))
                >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64)),
        ));
        let max1: Value<u64> = Rc::new(RefCell::new(
            (((*max0.borrow()).wrapping_mul(((*length.borrow()) as u64)))
                >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64)),
        ));
        let min_base: Value<u64> = Rc::new(RefCell::new(
            ((((*total.borrow()).wrapping_add((*max0.borrow()))).wrapping_add((*max1.borrow())))
                >> (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as u64)),
        ));
        (*total.borrow_mut()) = {
            (*total.borrow())
                .wrapping_add((*min_base.borrow()).wrapping_mul(((*count.borrow()) as u64)))
        };
        let kFixBits: Value<i64> = Rc::new(RefCell::new(32_i64));
        let kFixOne: Value<i64> = Rc::new(RefCell::new((1_i64 << (*kFixBits.borrow()))));
        let kDescaleBits: Value<i64> = Rc::new(RefCell::new(
            ((*kFixBits.borrow()) - (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as i64)),
        ));
        let kDescaleOne: Value<i64> = Rc::new(RefCell::new((1_i64 << (*kDescaleBits.borrow()))));
        let kDescaleMask: Value<i64> = Rc::new(RefCell::new(((*kDescaleOne.borrow()) - 1_i64)));
        let mult: Value<u32> = Rc::new(RefCell::new(
            ((((*kFixOne.borrow()) as u64).wrapping_div((*total.borrow()))) as u32),
        ));
        let error: Value<u32> = Rc::new(RefCell::new(
            ((((*kFixOne.borrow()) as u64).wrapping_rem((*total.borrow()))) as u32),
        ));
        let cumul: Value<u32> = Rc::new(RefCell::new((*error.borrow())));
        if (((*error.borrow()) as i64) < (*kDescaleOne.borrow())) {
            (*cumul.borrow_mut()) = {
                (((*cumul.borrow()) as i64)
                    + (((*kDescaleOne.borrow()) - ((*error.borrow()) as i64)) >> 1))
                    as u32
            };
        }
        if ((elem!((*data.borrow()), 0).read()) > 0) {
            let c: Value<u64> = Rc::new(RefCell::new(
                ((((elem!((*data.borrow()), 0).read()) as u64).wrapping_add((*min_base.borrow())))
                    .wrapping_mul(((*mult.borrow()) as u64)))
                .wrapping_add(((*cumul.borrow()) as u64)),
            ));
            let c_descaled: Value<u64> =
                Rc::new(RefCell::new(((*c.borrow()) >> (*kDescaleBits.borrow()))));
            if !((*c_descaled.borrow()) < (1_u64 << 31)) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                        236,
                        Ptr::<i8>::from_string_literal(b"PopulationCost"),
                    )
                });
                'loop_: while true {}
            };
            let log2count: Value<f64> = Rc::new(RefCell::new(
                ({ FastLog2_127(((*c_descaled.borrow()) as i32)) }),
            ));
            (*entropy_bits.borrow_mut()) -=
                ({ ((elem!((*data.borrow()), 0).read()) as f64) } * { (*log2count.borrow()) });
            (*cumul.borrow_mut()) = (((*c.borrow()) & ((*kDescaleMask.borrow()) as u64)) as u32);
        }
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) < (*length.borrow())) {
            if ((elem!((*data.borrow()), (*i.borrow())).read()) > 0) {
                let c: Value<u64> = Rc::new(RefCell::new(
                    ((((elem!((*data.borrow()), (*i.borrow())).read()) as u64)
                        .wrapping_add((*min_base.borrow())))
                    .wrapping_mul(((*mult.borrow()) as u64)))
                    .wrapping_add(((*cumul.borrow()) as u64)),
                ));
                let c_descaled: Value<u64> =
                    Rc::new(RefCell::new(((*c.borrow()) >> (*kDescaleBits.borrow()))));
                if !((*c_descaled.borrow()) < (1_u64 << 31)) {
                    ({
                        BrunsliDumpAndAbort_79(
                            Ptr::<i8>::from_string_literal(b"histogram_encode.cc"),
                            245,
                            Ptr::<i8>::from_string_literal(b"PopulationCost"),
                        )
                    });
                    'loop_: while true {}
                };
                let log2count: Value<f64> = Rc::new(RefCell::new(
                    ({ FastLog2_127(((*c_descaled.borrow()) as i32)) }),
                ));
                let log2floor: Value<i32> = Rc::new(RefCell::new(((*log2count.borrow()) as i32)));
                (*entropy_bits.borrow_mut()) -= ({
                    ((elem!((*data.borrow()), (*i.borrow())).read()) as f64)
                } * { (*log2count.borrow()) });
                (*histogram_bits.borrow_mut()) += (*log2floor.borrow());
                (*histogram_bits.borrow_mut()) += (({
                    let __idx = ((*log2floor.borrow()) + 1) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
                (*cumul.borrow_mut()) =
                    (((*c.borrow()) & ((*kDescaleMask.borrow()) as u64)) as u32);
            } else {
                (*histogram_bits.borrow_mut()) += (({
                    let __idx = (0) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
            }
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let log2norm: Value<f64> = Rc::new(RefCell::new(
            ((BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) as f64)
                - ({ FastLog2_127((*total_count.borrow())) })),
        ));
        if ((elem!((*data.borrow()), 0).read()) > 0) {
            let log2count: Value<f64> = Rc::new(RefCell::new(
                ({ ({ FastLog2_127((elem!((*data.borrow()), 0).read())) }) } + {
                    (*log2norm.borrow())
                }),
            ));
            (*entropy_bits.borrow_mut()) -=
                ({ ((elem!((*data.borrow()), 0).read()) as f64) } * { (*log2count.borrow()) });
            (*length.borrow_mut()) = 0;
            (*count.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) < 18) {
            if ((elem!((*data.borrow()), (*i.borrow())).read()) > 0) {
                let log2count: Value<f64> = Rc::new(RefCell::new(
                    ({ ({ FastLog2_127((elem!((*data.borrow()), (*i.borrow())).read())) }) } + {
                        (*log2norm.borrow())
                    }),
                ));
                let log2floor: Value<i32> = Rc::new(RefCell::new(((*log2count.borrow()) as i32)));
                (*entropy_bits.borrow_mut()) -= ({
                    ((elem!((*data.borrow()), (*i.borrow())).read()) as f64)
                } * { (*log2count.borrow()) });
                if ((*log2floor.borrow()) >= BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) {
                    (*log2floor.borrow_mut()) =
                        (BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()) - 1);
                }
                {
                    let rhs_0 = (((*histogram_bits.borrow()) as u32).wrapping_add(
                        ({ GetPopulationCountPrecision_203(((*log2floor.borrow()) as u32)) }),
                    )) as i32;
                    (*histogram_bits.borrow_mut()) = rhs_0
                };
                (*histogram_bits.borrow_mut()) += (({
                    let __idx = ((*log2floor.borrow()) + 1) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
                (*length.borrow_mut()) = (*i.borrow());
                (*count.borrow_mut()).prefix_inc();
            } else {
                (*histogram_bits.borrow_mut()) += (({
                    let __idx = (0) as usize;
                    kLogCountBitLengths_206.with(|rc| rc.borrow()[__idx])
                }) as i32);
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*length.borrow_mut()).prefix_inc();
    }
    if ((*count.borrow()) == 1) {
        return 7_f64;
    }
    if ((*count.borrow()) == 2) {
        return ((((((*entropy_bits.borrow()) as i32) + 1) + 12)
            + BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())) as f64);
    }
    (*histogram_bits.borrow_mut()) += (({
        let __idx = ((*length.borrow()) - 3) as usize;
        kHistogramLengthBitLengths_204.with(|rc| rc.borrow()[__idx])
    }) as i32);
    return ((((*histogram_bits.borrow()) + ((*entropy_bits.borrow()) as i32)) + 1) as f64);
}
thread_local!(
    pub static kCodeLengthCodes_211: Value<i32> = Rc::new(RefCell::new(18));
);
pub fn StoreHuffmanTreeOfHuffmanTreeToBitMask_212(
    num_codes: i32,
    code_length_bitdepth: Ptr<u8>,
    storage: Ptr<brunsli_Storage>,
) {
    let num_codes: Value<i32> = Rc::new(RefCell::new(num_codes));
    let code_length_bitdepth: Value<Ptr<u8>> = Rc::new(RefCell::new(code_length_bitdepth));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
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
    let codes_to_store: Value<usize> = Rc::new(RefCell::new(
        (kCodeLengthCodes_211.with(|rc| *rc.borrow()) as usize),
    ));
    if ((*num_codes.borrow()) > 1) {
        'loop_: while ((*codes_to_store.borrow()) > 0_usize) {
            if (((elem!(
                (*code_length_bitdepth.borrow()),
                ({
                    let __idx = ((*codes_to_store.borrow()).wrapping_sub(1_usize)) as usize;
                    kStorageOrder_213.with(|rc| rc.borrow()[__idx])
                })
            )
            .read()) as i32)
                != 0)
            {
                break;
            }
            (*codes_to_store.borrow_mut()).prefix_dec();
        }
    }
    let skip_some: Value<usize> = Rc::new(RefCell::new(0_usize));
    if (((elem!(
        (*code_length_bitdepth.borrow()),
        ({
            let __idx = (0) as usize;
            kStorageOrder_213.with(|rc| rc.borrow()[__idx])
        })
    )
    .read()) as i32)
        == 0)
        && (((elem!(
            (*code_length_bitdepth.borrow()),
            ({
                let __idx = (1) as usize;
                kStorageOrder_213.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            == 0)
    {
        (*skip_some.borrow_mut()) = 2_usize;
        if (((elem!(
            (*code_length_bitdepth.borrow()),
            ({
                let __idx = (2) as usize;
                kStorageOrder_213.with(|rc| rc.borrow()[__idx])
            })
        )
        .read()) as i32)
            == 0)
        {
            (*skip_some.borrow_mut()) = 3_usize;
        }
    }
    ({
        WriteBits_120(
            2_usize,
            ((*skip_some.borrow()) as u64),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new((*skip_some.borrow())));
    'loop_: while ((*i.borrow()) < (*codes_to_store.borrow())) {
        let l: Value<usize> = Rc::new(RefCell::new(
            ((elem!(
                (*code_length_bitdepth.borrow()),
                ({
                    let __idx = (*i.borrow()) as usize;
                    kStorageOrder_213.with(|rc| rc.borrow()[__idx])
                })
            )
            .read()) as usize),
        ));
        ({
            let _n_bits: usize = (({
                let __idx = (*l.borrow()) as usize;
                kHuffmanBitLengthHuffmanCodeBitLengths_215.with(|rc| rc.borrow()[__idx])
            }) as usize);
            let _bits: u64 = (({
                let __idx = (*l.borrow()) as usize;
                kHuffmanBitLengthHuffmanCodeSymbols_214.with(|rc| rc.borrow()[__idx])
            }) as u64);
            WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn StoreHuffmanTreeToBitMask_216(
    huffman_tree_size: usize,
    huffman_tree: Ptr<u8>,
    huffman_tree_extra_bits: Ptr<u8>,
    code_length_bitdepth: Ptr<u8>,
    code_length_bitdepth_symbols: Ptr<u16>,
    storage: Ptr<brunsli_Storage>,
) {
    let huffman_tree_size: Value<usize> = Rc::new(RefCell::new(huffman_tree_size));
    let huffman_tree: Value<Ptr<u8>> = Rc::new(RefCell::new(huffman_tree));
    let huffman_tree_extra_bits: Value<Ptr<u8>> = Rc::new(RefCell::new(huffman_tree_extra_bits));
    let code_length_bitdepth: Value<Ptr<u8>> = Rc::new(RefCell::new(code_length_bitdepth));
    let code_length_bitdepth_symbols: Value<Ptr<u16>> =
        Rc::new(RefCell::new(code_length_bitdepth_symbols));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*huffman_tree_size.borrow())) {
        let ix: Value<usize> = Rc::new(RefCell::new(
            ((elem!((*huffman_tree.borrow()), (*i.borrow())).read()) as usize),
        ));
        ({
            let _n_bits: usize =
                ((elem!((*code_length_bitdepth.borrow()), (*ix.borrow())).read()) as usize);
            let _bits: u64 =
                ((elem!((*code_length_bitdepth_symbols.borrow()), (*ix.borrow())).read()) as u64);
            WriteBits_120(_n_bits, _bits, (*storage.borrow()).clone())
        });
        'switch: {
            match { (*ix.borrow()) } {
                __v if __v == 16_usize => {
                    ({
                        WriteBits_120(
                            2_usize,
                            ((elem!((*huffman_tree_extra_bits.borrow()), (*i.borrow())).read())
                                as u64),
                            (*storage.borrow()).clone(),
                        )
                    });
                    break 'switch;
                }
                __v if __v == 17_usize => {
                    ({
                        WriteBits_120(
                            3_usize,
                            ((elem!((*huffman_tree_extra_bits.borrow()), (*i.borrow())).read())
                                as u64),
                            (*storage.borrow()).clone(),
                        )
                    });
                    break 'switch;
                }
                _ => {}
            }
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn StoreSimpleHuffmanTree_217(
    depths: Ptr<u8>,
    symbols: Ptr<usize>,
    num_symbols: usize,
    max_bits: usize,
    storage: Ptr<brunsli_Storage>,
) {
    let depths: Value<Ptr<u8>> = Rc::new(RefCell::new(depths));
    let symbols: Value<Ptr<usize>> = Rc::new(RefCell::new(symbols));
    let num_symbols: Value<usize> = Rc::new(RefCell::new(num_symbols));
    let max_bits: Value<usize> = Rc::new(RefCell::new(max_bits));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({ WriteBits_120(2_usize, 1_u64, (*storage.borrow()).clone()) });
    ({
        WriteBits_120(
            2_usize,
            ((*num_symbols.borrow()).wrapping_sub(1_usize) as u64),
            (*storage.borrow()).clone(),
        )
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
        let j: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*j.borrow()) < (*num_symbols.borrow())) {
            if ({
                ((elem!(
                    (*depths.borrow()),
                    (elem!((*symbols.borrow()), (*j.borrow())).read())
                )
                .read()) as i32)
            } < {
                ((elem!(
                    (*depths.borrow()),
                    (elem!((*symbols.borrow()), (*i.borrow())).read())
                )
                .read()) as i32)
            }) {
                {
                    let tmp = (*symbols.borrow()).offset((*j.borrow()) as isize).read();
                    (*symbols.borrow())
                        .offset((*j.borrow()) as isize)
                        .write((*symbols.borrow()).offset((*i.borrow()) as isize).read());
                    (*symbols.borrow())
                        .offset((*i.borrow()) as isize)
                        .write(tmp);
                };
            }
            (*j.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).postfix_inc();
    }
    if ((*num_symbols.borrow()) == 2_usize) {
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else if ((*num_symbols.borrow()) == 3_usize) {
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 2).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else {
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 0).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 1).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 2).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((elem!((*symbols.borrow()), 3).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _bits: u64 =
                (if (((elem!((*depths.borrow()), (elem!((*symbols.borrow()), 0).read())).read())
                    as i32)
                    == 1)
                {
                    1
                } else {
                    0
                } as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
    }
}
pub fn StoreHuffmanTree_218(depths: Ptr<u8>, num: usize, storage: Ptr<brunsli_Storage>) {
    let depths: Value<Ptr<u8>> = Rc::new(RefCell::new(depths));
    let num: Value<usize> = Rc::new(RefCell::new(num));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let arena: Value<Option<Value<Box<[u8]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array(
            (0..(2_usize).wrapping_mul((*num.borrow())))
                .map(|_| 0_u8)
                .collect::<Box<[u8]>>(),
        )
        .to_owned_opt(),
    ));
    let huffman_tree: Value<Ptr<u8>> = Rc::new(RefCell::new((*arena.borrow()).as_pointer()));
    let huffman_tree_extra_bits: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*arena.borrow())
            .as_pointer()
            .offset((*num.borrow()) as isize),
    ));
    let huffman_tree_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        WriteHuffmanTree_219(
            (*depths.borrow()).clone(),
            (*num.borrow()),
            (huffman_tree_size.as_pointer()),
            (*huffman_tree.borrow()).clone(),
            (*huffman_tree_extra_bits.borrow()).clone(),
        )
    });
    let huffman_tree_histogram: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ])));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*huffman_tree_size.borrow())) {
        (*huffman_tree_histogram.borrow_mut())
            [(elem!((*huffman_tree.borrow()), (*i.borrow())).read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let num_codes: Value<i32> = Rc::new(RefCell::new(0));
    let code: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kCodeLengthCodes_211.with(|rc| *rc.borrow())) {
        if ((*huffman_tree_histogram.borrow())[(*i.borrow()) as usize] != 0) {
            if ((*num_codes.borrow()) == 0) {
                (*code.borrow_mut()) = (*i.borrow());
                (*num_codes.borrow_mut()) = 1;
            } else if ((*num_codes.borrow()) == 1) {
                (*num_codes.borrow_mut()) = 2;
                break;
            }
        }
        (*i.borrow_mut()).prefix_inc();
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
            (*num_codes.borrow()),
            (code_length_bitdepth.as_pointer() as Ptr<u8>),
            (*storage.borrow()).clone(),
        )
    });
    if ((*num_codes.borrow()) == 1) {
        (*code_length_bitdepth.borrow_mut())[(*code.borrow()) as usize] = 0_u8;
    }
    ({
        StoreHuffmanTreeToBitMask_216(
            (*huffman_tree_size.borrow()),
            (*huffman_tree.borrow()).clone(),
            (*huffman_tree_extra_bits.borrow()).clone(),
            ((code_length_bitdepth.as_pointer() as Ptr<u8>).offset(0)),
            (code_length_bitdepth_symbols.as_pointer() as Ptr<u16>),
            (*storage.borrow()).clone(),
        )
    });
}
pub fn BuildAndStoreHuffmanTree_202(
    histogram: Ptr<u32>,
    length: usize,
    depth: Ptr<u8>,
    bits: Ptr<u16>,
    storage: Ptr<brunsli_Storage>,
) {
    let histogram: Value<Ptr<u32>> = Rc::new(RefCell::new(histogram));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let bits: Value<Ptr<u16>> = Rc::new(RefCell::new(bits));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let s4: Value<Box<[usize]>> =
        Rc::new(RefCell::new(Box::new([0_usize, 0_usize, 0_usize, 0_usize])));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*length.borrow())) {
        if ((elem!((*histogram.borrow()), (*i.borrow())).read()) != 0) {
            if ((*count.borrow()) < 4_usize) {
                (*s4.borrow_mut())[(*count.borrow()) as usize] = (*i.borrow());
            } else if ((*count.borrow()) > 4_usize) {
                break;
            }
            (*count.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).postfix_inc();
    }
    let max_bits_counter: Value<usize> =
        Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_usize)));
    let max_bits: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*max_bits_counter.borrow()) != 0) {
        (*max_bits_counter.borrow_mut()) >>= 1;
        (*max_bits.borrow_mut()).prefix_inc();
    }
    if ((*count.borrow()) <= 1_usize) {
        ({ WriteBits_120(4_usize, 1_u64, (*storage.borrow()).clone()) });
        ({
            WriteBits_120(
                (*max_bits.borrow()),
                ((*s4.borrow())[(0) as usize] as u64),
                (*storage.borrow()).clone(),
            )
        });
        return;
    }
    ({
        CreateHuffmanTree_220(
            (*histogram.borrow()).clone(),
            (*length.borrow()),
            15,
            (*depth.borrow()).clone(),
        )
    });
    ({
        ConvertBitDepthsToSymbols_221(
            (*depth.borrow()).clone(),
            (*length.borrow()),
            (*bits.borrow()).clone(),
        )
    });
    if ((*count.borrow()) <= 4_usize) {
        ({
            StoreSimpleHuffmanTree_217(
                (*depth.borrow()).clone(),
                (s4.as_pointer() as Ptr<usize>),
                (*count.borrow()),
                (*max_bits.borrow()),
                (*storage.borrow()).clone(),
            )
        });
    } else {
        ({
            StoreHuffmanTree_218(
                (*depth.borrow()).clone(),
                (*length.borrow()),
                (*storage.borrow()).clone(),
            )
        });
    }
}
pub fn SetDepth_222(
    p: Ptr<brunsli_HuffmanTree>,
    pool: Ptr<brunsli_HuffmanTree>,
    depth: Ptr<u8>,
    level: u8,
) {
    let pool: Value<Ptr<brunsli_HuffmanTree>> = Rc::new(RefCell::new(pool));
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let level: Value<u8> = Rc::new(RefCell::new(level));
    if ((p.with(|__s| __s.index_left) as i32) >= 0) {
        (*level.borrow_mut()).prefix_inc();
        ({
            let _p: Ptr<brunsli_HuffmanTree> =
                (*pool.borrow()).offset((p.with(|__s| __s.index_left)) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (*pool.borrow()).clone();
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            let _level: u8 = (*level.borrow());
            SetDepth_222(_p, _pool, _depth, _level)
        });
        ({
            let _p: Ptr<brunsli_HuffmanTree> =
                (*pool.borrow()).offset((p.with(|__s| __s.index_right_or_value)) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (*pool.borrow()).clone();
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            let _level: u8 = (*level.borrow());
            SetDepth_222(_p, _pool, _depth, _level)
        });
    } else {
        elem!((*depth.borrow()), p.with(|__s| __s.index_right_or_value))
            .write({ (*level.borrow()) });
    }
}
pub fn Compare_223(v0: Ptr<brunsli_HuffmanTree>, v1: Ptr<brunsli_HuffmanTree>) -> bool {
    return ({ v0.with(|__s| __s.total_count) } < { v1.with(|__s| __s.total_count) });
}
pub fn CreateHuffmanTree_220(data: Ptr<u32>, length: usize, tree_limit: i32, depth: Ptr<u8>) {
    let data: Value<Ptr<u32>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let tree_limit: Value<i32> = Rc::new(RefCell::new(tree_limit));
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let count_limit: Value<u32> = Rc::new(RefCell::new(1_u32));
    'loop_: while true {
        let tree: Value<Vec<brunsli_HuffmanTree>> = Rc::new(RefCell::new(Vec::new()));
        if ((2_usize).wrapping_mul((*length.borrow()))).wrapping_add(1_usize) as usize
            > (*tree.borrow()).capacity() as usize
        {
            let len_0 = (*tree.borrow()).len();
            (*tree.borrow_mut()).reserve_exact(
                ((2_usize).wrapping_mul((*length.borrow()))).wrapping_add(1_usize) as usize
                    - len_0 as usize,
            );
        };
        let i: Value<usize> = Rc::new(RefCell::new((*length.borrow())));
        'loop_: while ((*i.borrow()) != 0_usize) {
            (*i.borrow_mut()).prefix_dec();
            if ((elem!((*data.borrow()), (*i.borrow())).read()) != 0) {
                let count: Value<u32> = Rc::new(RefCell::new({
                    let __tmp_1: Value<u32> =
                        Rc::new(RefCell::new((*count_limit.borrow()).wrapping_sub(1_u32)));
                    (if (*data.borrow()).offset((*i.borrow()) as isize).read()
                        >= __tmp_1.as_pointer().read()
                    {
                        (*data.borrow()).offset((*i.borrow()) as isize)
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                }));
                {
                    let __a1 =
                        brunsli_HuffmanTree::new({ (*count.borrow()) }, { (-1_i32 as i16) }, {
                            ((*i.borrow()) as i16)
                        });
                    (*tree.borrow_mut()).push(__a1)
                };
            };
        }
        let n: Value<usize> = Rc::new(RefCell::new((*tree.borrow()).len()));
        if ((*n.borrow()) == 1_usize) {
            elem!((*depth.borrow()), {
                (*elem!((tree.as_pointer() as Ptr<brunsli_HuffmanTree>), 0_usize)
                    .upgrade()
                    .deref())
                .index_right_or_value
            })
            .write(1_u8);
            break;
        }
        (tree.as_pointer() as Ptr<brunsli_HuffmanTree>).sort_with_cmp(
            (tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .to_end()
                .get_offset(),
            |x, y| Compare_223.call(x, y),
        );
        let sentinel: Value<brunsli_HuffmanTree> = Rc::new(RefCell::new(brunsli_HuffmanTree::new(
            { <u32>::MAX },
            { (-1_i32 as i16) },
            { (-1_i32 as i16) },
        )));
        {
            let a0_clone = (*sentinel.borrow()).clone();
            (*tree.borrow_mut()).push(a0_clone)
        };
        {
            let a0_clone = (*sentinel.borrow()).clone();
            (*tree.borrow_mut()).push(a0_clone)
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        let j: Value<usize> = Rc::new(RefCell::new((*n.borrow()).wrapping_add(1_usize)));
        let k: Value<usize> = Rc::new(RefCell::new((*n.borrow()).wrapping_sub(1_usize)));
        'loop_: while ((*k.borrow()) != 0_usize) {
            let left: Value<usize> = Rc::new(RefCell::new(0_usize));
            let right: Value<usize> = Rc::new(RefCell::new(0_usize));
            if ({
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            } <= {
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*j.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            }) {
                (*left.borrow_mut()) = (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            } else {
                (*left.borrow_mut()) = (*j.borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            if ({
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            } <= {
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*j.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            }) {
                (*right.borrow_mut()) = (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            } else {
                (*right.borrow_mut()) = (*j.borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            let j_end: Value<usize> =
                Rc::new(RefCell::new(((*tree.borrow()).len()).wrapping_sub(1_usize)));
            let __rhs = ({
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*left.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            })
            .wrapping_add({
                (*elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*right.borrow())
                )
                .upgrade()
                .deref())
                .total_count
            });
            field!(
                elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*j_end.borrow())
                ),
                total_count
            )
            .write(__rhs);
            field!(
                elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*j_end.borrow())
                ),
                index_left
            )
            .write(((*left.borrow()) as i16));
            field!(
                elem!(
                    (tree.as_pointer() as Ptr<brunsli_HuffmanTree>),
                    (*j_end.borrow())
                ),
                index_right_or_value
            )
            .write(((*right.borrow()) as i16));
            {
                let a0_clone = (*sentinel.borrow()).clone();
                (*tree.borrow_mut()).push(a0_clone)
            };
            (*k.borrow_mut()).prefix_dec();
        }
        if !((*tree.borrow()).len()
            == ((2_usize).wrapping_mul((*n.borrow()))).wrapping_add(1_usize))
        {
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
                .offset(((2_usize).wrapping_mul((*n.borrow()))).wrapping_sub(1_usize));
            let _pool: Ptr<brunsli_HuffmanTree> =
                ((tree.as_pointer() as Ptr<brunsli_HuffmanTree>).offset(0_usize));
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            SetDepth_222(_p, _pool, _depth, 0_u8)
        });
        if ({
            (({
                let count = ((*depth.borrow()).offset((*length.borrow()) as isize)).get_offset()
                    - ((*depth.borrow()).offset((0) as isize)).get_offset();
                let max_index = PtrValueIter::new(&((*depth.borrow()).offset((0) as isize)), count)
                    .enumerate()
                    .max_by(|(_, val_a), (_, val_b)| {
                        val_a
                            .partial_cmp(val_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);
                ((*depth.borrow()).offset((0) as isize)) + max_index
            }
            .read()) as i32)
        } <= { (*tree_limit.borrow()) })
        {
            break;
        }
        (*count_limit.borrow_mut()) = { (*count_limit.borrow()).wrapping_mul(2_u32) };
    }
}
pub fn Reverse_224(v: Ptr<u8>, start: usize, end: usize) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let start: Value<usize> = Rc::new(RefCell::new(start));
    let end: Value<usize> = Rc::new(RefCell::new(end));
    (*end.borrow_mut()).prefix_dec();
    'loop_: while ((*start.borrow()) < (*end.borrow())) {
        let tmp: Value<u8> = Rc::new(RefCell::new(
            (elem!((*v.borrow()), (*start.borrow())).read()),
        ));
        elem!((*v.borrow()), (*start.borrow()))
            .write({ (elem!((*v.borrow()), (*end.borrow())).read()) });
        elem!((*v.borrow()), (*end.borrow())).write({ (*tmp.borrow()) });
        (*start.borrow_mut()).prefix_inc();
        (*end.borrow_mut()).prefix_dec();
    }
}
pub fn WriteHuffmanTreeRepetitions_225(
    previous_value: u8,
    value: u8,
    repetitions: usize,
    tree_size: Ptr<usize>,
    tree: Ptr<u8>,
    extra_bits_data: Ptr<u8>,
) {
    let previous_value: Value<u8> = Rc::new(RefCell::new(previous_value));
    let value: Value<u8> = Rc::new(RefCell::new(value));
    let repetitions: Value<usize> = Rc::new(RefCell::new(repetitions));
    let tree_size: Value<Ptr<usize>> = Rc::new(RefCell::new(tree_size));
    let tree: Value<Ptr<u8>> = Rc::new(RefCell::new(tree));
    let extra_bits_data: Value<Ptr<u8>> = Rc::new(RefCell::new(extra_bits_data));
    if !((*repetitions.borrow()) > 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"huffman_tree.cc"),
                151,
                Ptr::<i8>::from_string_literal(b"WriteHuffmanTreeRepetitions"),
            )
        });
        'loop_: while true {}
    };
    if (((*previous_value.borrow()) as i32) != ((*value.borrow()) as i32)) {
        elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write({ (*value.borrow()) });
        elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) == 7_usize) {
        elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write({ (*value.borrow()) });
        elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) < 3_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*repetitions.borrow())) {
            elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write({ (*value.borrow()) });
            elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        (*repetitions.borrow_mut()) = { (*repetitions.borrow()).wrapping_sub(3_usize) };
        let start: Value<usize> = Rc::new(RefCell::new(((*tree_size.borrow()).read())));
        'loop_: while true {
            elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write(16_u8);
            elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read()))
                .write({ (((*repetitions.borrow()) & 3_usize) as u8) });
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*repetitions.borrow_mut()) >>= 2;
            if ((*repetitions.borrow()) == 0_usize) {
                break;
            }
            (*repetitions.borrow_mut()).prefix_dec();
        }
        ({
            let _v: Ptr<u8> = (*tree.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_224(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (*extra_bits_data.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_224(_v, _start, _end)
        });
    }
}
pub fn WriteHuffmanTreeRepetitionsZeros_226(
    repetitions: usize,
    tree_size: Ptr<usize>,
    tree: Ptr<u8>,
    extra_bits_data: Ptr<u8>,
) {
    let repetitions: Value<usize> = Rc::new(RefCell::new(repetitions));
    let tree_size: Value<Ptr<usize>> = Rc::new(RefCell::new(tree_size));
    let tree: Value<Ptr<u8>> = Rc::new(RefCell::new(tree));
    let extra_bits_data: Value<Ptr<u8>> = Rc::new(RefCell::new(extra_bits_data));
    if ((*repetitions.borrow()) == 11_usize) {
        elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
        elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) < 3_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*repetitions.borrow())) {
            elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
            elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read())).write(0_u8);
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        (*repetitions.borrow_mut()) = { (*repetitions.borrow()).wrapping_sub(3_usize) };
        let start: Value<usize> = Rc::new(RefCell::new(((*tree_size.borrow()).read())));
        'loop_: while true {
            elem!((*tree.borrow()), ((*tree_size.borrow()).read())).write(17_u8);
            elem!((*extra_bits_data.borrow()), ((*tree_size.borrow()).read()))
                .write({ (((*repetitions.borrow()) & 7_usize) as u8) });
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*repetitions.borrow_mut()) >>= 3;
            if ((*repetitions.borrow()) == 0_usize) {
                break;
            }
            (*repetitions.borrow_mut()).prefix_dec();
        }
        ({
            let _v: Ptr<u8> = (*tree.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_224(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (*extra_bits_data.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_224(_v, _start, _end)
        });
    }
}
pub fn DecideOverRleUse_227(
    depth: Ptr<u8>,
    length: usize,
    use_rle_for_non_zero: Ptr<bool>,
    use_rle_for_zero: Ptr<bool>,
) {
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let use_rle_for_non_zero: Value<Ptr<bool>> = Rc::new(RefCell::new(use_rle_for_non_zero));
    let use_rle_for_zero: Value<Ptr<bool>> = Rc::new(RefCell::new(use_rle_for_zero));
    let total_reps_zero: Value<usize> = Rc::new(RefCell::new(0_usize));
    let total_reps_non_zero: Value<usize> = Rc::new(RefCell::new(0_usize));
    let count_reps_zero: Value<usize> = Rc::new(RefCell::new(1_usize));
    let count_reps_non_zero: Value<usize> = Rc::new(RefCell::new(1_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*length.borrow())) {
        let value: Value<u8> = Rc::new(RefCell::new(
            (elem!((*depth.borrow()), (*i.borrow())).read()),
        ));
        let reps: Value<usize> = Rc::new(RefCell::new(1_usize));
        let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*k.borrow()) < (*length.borrow()))
            && ({ ((elem!((*depth.borrow()), (*k.borrow())).read()) as i32) } == {
                ((*value.borrow()) as i32)
            })
        {
            (*reps.borrow_mut()).prefix_inc();
            (*k.borrow_mut()).prefix_inc();
        }
        if ((*reps.borrow()) >= 3_usize) && (((*value.borrow()) as i32) == 0) {
            (*total_reps_zero.borrow_mut()) =
                { (*total_reps_zero.borrow()).wrapping_add((*reps.borrow())) };
            (*count_reps_zero.borrow_mut()).prefix_inc();
        }
        if ((*reps.borrow()) >= 4_usize) && (((*value.borrow()) as i32) != 0) {
            (*total_reps_non_zero.borrow_mut()) =
                { (*total_reps_non_zero.borrow()).wrapping_add((*reps.borrow())) };
            (*count_reps_non_zero.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()) = { (*i.borrow()).wrapping_add((*reps.borrow())) };
    }
    (*use_rle_for_non_zero.borrow()).write({
        ((*total_reps_non_zero.borrow()) > (*count_reps_non_zero.borrow()).wrapping_mul(2_usize))
    });
    (*use_rle_for_zero.borrow()).write({
        ((*total_reps_zero.borrow()) > (*count_reps_zero.borrow()).wrapping_mul(2_usize))
    });
}
pub fn WriteHuffmanTree_219(
    depth: Ptr<u8>,
    length: usize,
    tree_size: Ptr<usize>,
    tree: Ptr<u8>,
    extra_bits_data: Ptr<u8>,
) {
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let tree_size: Value<Ptr<usize>> = Rc::new(RefCell::new(tree_size));
    let tree: Value<Ptr<u8>> = Rc::new(RefCell::new(tree));
    let extra_bits_data: Value<Ptr<u8>> = Rc::new(RefCell::new(extra_bits_data));
    let previous_value: Value<u8> = Rc::new(RefCell::new(8_u8));
    let new_length: Value<usize> = Rc::new(RefCell::new((*length.borrow())));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*length.borrow())) {
        if (((elem!(
            (*depth.borrow()),
            ((*length.borrow()).wrapping_sub((*i.borrow()))).wrapping_sub(1_usize)
        )
        .read()) as i32)
            == 0)
        {
            (*new_length.borrow_mut()).prefix_dec();
        } else {
            break;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let use_rle_for_non_zero: Value<bool> = Rc::new(RefCell::new(false));
    let use_rle_for_zero: Value<bool> = Rc::new(RefCell::new(false));
    if ((*length.borrow()) > 50_usize) {
        ({
            DecideOverRleUse_227(
                (*depth.borrow()).clone(),
                (*new_length.borrow()),
                (use_rle_for_non_zero.as_pointer()),
                (use_rle_for_zero.as_pointer()),
            )
        });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*new_length.borrow())) {
        let value: Value<u8> = Rc::new(RefCell::new(
            (elem!((*depth.borrow()), (*i.borrow())).read()),
        ));
        let reps: Value<usize> = Rc::new(RefCell::new(1_usize));
        if ((((*value.borrow()) as i32) != 0) && (*use_rle_for_non_zero.borrow()))
            || ((((*value.borrow()) as i32) == 0) && (*use_rle_for_zero.borrow()))
        {
            let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
            'loop_: while ((*k.borrow()) < (*new_length.borrow()))
                && ({ ((elem!((*depth.borrow()), (*k.borrow())).read()) as i32) } == {
                    ((*value.borrow()) as i32)
                })
            {
                (*reps.borrow_mut()).prefix_inc();
                (*k.borrow_mut()).prefix_inc();
            }
        }
        if (((*value.borrow()) as i32) == 0) {
            ({
                let _repetitions: usize = (*reps.borrow());
                let _tree_size: Ptr<usize> = (*tree_size.borrow()).clone();
                WriteHuffmanTreeRepetitionsZeros_226(
                    _repetitions,
                    _tree_size,
                    (*tree.borrow()).clone(),
                    (*extra_bits_data.borrow()).clone(),
                )
            });
        } else {
            ({
                let _previous_value: u8 = (*previous_value.borrow());
                let _value: u8 = (*value.borrow());
                let _repetitions: usize = (*reps.borrow());
                let _tree_size: Ptr<usize> = (*tree_size.borrow()).clone();
                let _tree: Ptr<u8> = (*tree.borrow()).clone();
                let _extra_bits_data: Ptr<u8> = (*extra_bits_data.borrow()).clone();
                WriteHuffmanTreeRepetitions_225(
                    _previous_value,
                    _value,
                    _repetitions,
                    _tree_size,
                    _tree,
                    _extra_bits_data,
                )
            });
            (*previous_value.borrow_mut()) = (*value.borrow());
        }
        (*i.borrow_mut()) = { (*i.borrow()).wrapping_add((*reps.borrow())) };
    }
}
pub fn ReverseBits_228(num_bits: i32, bits: u16) -> u16 {
    let num_bits: Value<i32> = Rc::new(RefCell::new(num_bits));
    let bits: Value<u16> = Rc::new(RefCell::new(bits));
    thread_local!(
        static kLut_229: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
            0_usize, 8_usize, 4_usize, 12_usize, 2_usize, 10_usize, 6_usize, 14_usize, 1_usize,
            9_usize, 5_usize, 13_usize, 3_usize, 11_usize, 7_usize, 15_usize,
        ])));
    );
    let retval: Value<usize> = Rc::new(RefCell::new(
        ({
            let __idx = (((*bits.borrow()) as i32) & 15) as usize;
            kLut_229.with(|rc| rc.borrow()[__idx])
        }),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(4));
    'loop_: while ((*i.borrow()) < (*num_bits.borrow())) {
        (*retval.borrow_mut()) <<= 4;
        (*bits.borrow_mut()) = { ((((*bits.borrow()) as i32) >> 4) as u16) };
        (*retval.borrow_mut()) |= ({
            let __idx = (((*bits.borrow()) as i32) & 15) as usize;
            kLut_229.with(|rc| rc.borrow()[__idx])
        });
        (*i.borrow_mut()) += 4;
    }
    (*retval.borrow_mut()) >>= (-(*num_bits.borrow()) & 3);
    return ((*retval.borrow()) as u16);
}
pub fn ConvertBitDepthsToSymbols_221(depth: Ptr<u8>, len: usize, bits: Ptr<u16>) {
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let bits: Value<Ptr<u16>> = Rc::new(RefCell::new(bits));
    let kMaxBits: Value<i32> = Rc::new(RefCell::new(16));
    let bl_count: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ])));
    {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*len.borrow())) {
            (*bl_count.borrow_mut())[(elem!((*depth.borrow()), (*i.borrow())).read()) as usize]
                .prefix_inc();
            (*i.borrow_mut()).prefix_inc();
        }
        (*bl_count.borrow_mut())[(0) as usize] = 0_u16;
    }
    let next_code: Value<Box<[u16]>> =
        Rc::new(RefCell::new((0..16).map(|_| 0_u16).collect::<Box<[u16]>>()));
    (*next_code.borrow_mut())[(0) as usize] = 0_u16;
    {
        let code: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<usize> = Rc::new(RefCell::new(1_usize));
        'loop_: while ((*i.borrow()) < ((*kMaxBits.borrow()) as usize)) {
            (*code.borrow_mut()) = {
                (((*code.borrow())
                    + ((*bl_count.borrow())[((*i.borrow()).wrapping_sub(1_usize)) as usize]
                        as i32))
                    << 1)
            };
            (*next_code.borrow_mut())[(*i.borrow()) as usize] = ((*code.borrow()) as u16);
            (*i.borrow_mut()).prefix_inc();
        }
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        if ((elem!((*depth.borrow()), (*i.borrow())).read()) != 0) {
            let __rhs = ({
                let _num_bits: i32 = ((elem!((*depth.borrow()), (*i.borrow())).read()) as i32);
                let _bits: u16 = (*next_code.borrow_mut())
                    [(elem!((*depth.borrow()), (*i.borrow())).read()) as usize]
                    .postfix_inc();
                ReverseBits_228(_num_bits, _bits)
            });
            elem!((*bits.borrow()), (*i.borrow())).write(__rhs);
        }
        (*i.borrow_mut()).prefix_inc();
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
        let __this: Value<brunsli_HuffmanTableEntry> = Rc::new(RefCell::new(Self {
            bits: 0_u8,
            value: 65535_u16,
        }));
        let this: Ptr<brunsli_HuffmanTableEntry> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for brunsli_HuffmanTableEntry {
    fn default() -> Self {
        { brunsli_HuffmanTableEntry::new() }
    }
}
pub fn DivCeil_232(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((((*a.borrow()) + (*b.borrow())) - 1) / (*b.borrow()));
}
pub fn ReadUint8_233(data: Ptr<u8>, pos: Ptr<usize>) -> i32 {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    return ((elem!(
        (*data.borrow()),
        (*pos.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .read()) as i32);
}
pub fn ReadUint16_234(data: Ptr<u8>, pos: Ptr<usize>) -> i32 {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let v: Value<i32> = Rc::new(RefCell::new(
        ((((elem!((*data.borrow()), ((*pos.borrow()).read())).read()) as i32) << 8)
            + ((elem!(
                (*data.borrow()),
                ((*pos.borrow()).read()).wrapping_add(1_usize)
            )
            .read()) as i32)),
    ));
    (*pos.borrow()).write({ ((*pos.borrow()).read()).wrapping_add(2_usize) });
    return (*v.borrow());
}
pub fn ProcessSOF_235(
    data: Ptr<u8>,
    len: usize,
    mode: brunsli_JpegReadMode,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let mode: Value<brunsli_JpegReadMode> = Rc::new(RefCell::new(mode));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if ((*jpg.borrow()).with(|__s| __s.width) != 0) {
        write!(libcc2rs::cerr(), "Duplicate SOF marker.\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_DUPLICATE_SOF);
        return false;
    }
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if ({ ((*pos.borrow()).read()).wrapping_add(((8) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (8),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    let precision: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    let height: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    let width: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    let num_components: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    if ((*precision.borrow()) < 8) || ((*precision.borrow()) > 8) {
        write!(
            libcc2rs::cerr(),
            "Invalid precision: {:}\n",
            (*precision.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_PRECISION);
        return false;
    };
    if ((*height.borrow()) < 1) || ((*height.borrow()) > kMaxDimPixels_11.with(|rc| *rc.borrow())) {
        write!(
            libcc2rs::cerr(),
            "Invalid height: {:}\n",
            (*height.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HEIGHT);
        return false;
    };
    if ((*width.borrow()) < 1) || ((*width.borrow()) > kMaxDimPixels_11.with(|rc| *rc.borrow())) {
        write!(libcc2rs::cerr(), "Invalid width: {:}\n", (*width.borrow()),);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_WIDTH);
        return false;
    };
    if ((*num_components.borrow()) < 1)
        || ((*num_components.borrow()) > kMaxComponents_4.with(|rc| *rc.borrow()))
    {
        write!(
            libcc2rs::cerr(),
            "Invalid num_components: {:}\n",
            (*num_components.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_NUMCOMP);
        return false;
    };
    if ({ ((*pos.borrow()).read()).wrapping_add(((3 * (*num_components.borrow())) as usize)) } > {
        (*len.borrow())
    }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3 * (*num_components.borrow())),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    field!((*jpg.borrow()), height).write((*height.borrow()));
    field!((*jpg.borrow()), width).write((*width.borrow()));
    {
        let __a0 = ((*num_components.borrow()) as usize) as usize;
        (*(*jpg.borrow())
            .with(|__s| __s.components.clone())
            .borrow_mut())
        .resize_with(__a0, || <brunsli_JPEGComponent>::default())
    };
    let ids_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
    }) {
        let id: Value<i32> = Rc::new(RefCell::new(
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
        ));
        if ((*elem!(
            (ids_seen.as_pointer() as Ptr<bool>),
            ((*id.borrow()) as usize)
        )
        .upgrade()
        .deref()) as bool)
        {
            write!(
                libcc2rs::cerr(),
                "Duplicate ID {:} in SOF.\n",
                (*id.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_DUPLICATE_COMPONENT_ID);
            return false;
        }
        elem!(
            (ids_seen.as_pointer() as Ptr<bool>),
            ((*id.borrow()) as usize)
        )
        .write(true);
        field!(
            elem!(
                ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>),
                (*i.borrow())
            ),
            id
        )
        .write((*id.borrow()));
        let factor: Value<i32> = Rc::new(RefCell::new(
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
        ));
        let h_samp_factor: Value<i32> = Rc::new(RefCell::new(((*factor.borrow()) >> 4)));
        let v_samp_factor: Value<i32> = Rc::new(RefCell::new(((*factor.borrow()) & 15)));
        if ((*h_samp_factor.borrow()) < 1)
            || ((*h_samp_factor.borrow()) > kBrunsliMaxSampling_27.with(|rc| *rc.borrow()))
        {
            write!(
                libcc2rs::cerr(),
                "Invalid h_samp_factor: {:}\n",
                (*h_samp_factor.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SAMP_FACTOR);
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
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SAMP_FACTOR);
            return false;
        };
        field!(
            elem!(
                ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>),
                (*i.borrow())
            ),
            h_samp_factor
        )
        .write((*h_samp_factor.borrow()));
        field!(
            elem!(
                ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>),
                (*i.borrow())
            ),
            v_samp_factor
        )
        .write((*v_samp_factor.borrow()));
        let __rhs = (({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as u8);
        field!(
            elem!(
                ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>),
                (*i.borrow())
            ),
            quant_idx
        )
        .write(__rhs);
        let __rhs = (if field_ptr!((*jpg.borrow()), max_h_samp_factor).read()
            >= h_samp_factor.as_pointer().read()
        {
            field_ptr!((*jpg.borrow()), max_h_samp_factor)
        } else {
            h_samp_factor.as_pointer()
        }
        .read());
        field!((*jpg.borrow()), max_h_samp_factor).write(__rhs);
        let __rhs = (if field_ptr!((*jpg.borrow()), max_v_samp_factor).read()
            >= v_samp_factor.as_pointer().read()
        {
            field_ptr!((*jpg.borrow()), max_v_samp_factor)
        } else {
            v_samp_factor.as_pointer()
        }
        .read());
        field!((*jpg.borrow()), max_v_samp_factor).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ({
        let _a: i32 = (*jpg.borrow()).with(|__s| __s.height);
        let _b: i32 = ((*jpg.borrow()).with(|__s| __s.max_v_samp_factor) * 8);
        DivCeil_232(_a, _b)
    });
    field!((*jpg.borrow()), MCU_rows).write(__rhs);
    let __rhs = ({
        let _a: i32 = (*jpg.borrow()).with(|__s| __s.width);
        let _b: i32 = ((*jpg.borrow()).with(|__s| __s.max_h_samp_factor) * 8);
        DivCeil_232(_a, _b)
    });
    field!((*jpg.borrow()), MCU_cols).write(__rhs);
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
    }) {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()))),
        ));
        if (({ (*jpg.borrow()).with(|__s| __s.max_h_samp_factor) } % {
            (*c.borrow()).with(|__s| __s.h_samp_factor)
        }) != 0)
            || (({ (*jpg.borrow()).with(|__s| __s.max_v_samp_factor) } % {
                (*c.borrow()).with(|__s| __s.v_samp_factor)
            }) != 0)
        {
            write!(libcc2rs::cerr(), "Non-integral subsampling ratios.\n",);
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SAMPLING_FACTORS);
            return false;
        }
        field!((*c.borrow()), width_in_blocks).write({
            (({ (*jpg.borrow()).with(|__s| __s.MCU_cols) } * {
                (*c.borrow()).with(|__s| __s.h_samp_factor)
            }) as u32)
        });
        field!((*c.borrow()), height_in_blocks).write({
            (({ (*jpg.borrow()).with(|__s| __s.MCU_rows) } * {
                (*c.borrow()).with(|__s| __s.v_samp_factor)
            }) as u32)
        });
        let num_blocks: Value<u64> = Rc::new(RefCell::new(
            ((*c.borrow()).with(|__s| __s.width_in_blocks) as u64)
                .wrapping_mul(((*c.borrow()).with(|__s| __s.height_in_blocks) as u64)),
        ));
        if (((*num_blocks.borrow()) as usize) > kBrunsliMaxNumBlocks_18.with(|rc| *rc.borrow())) {
            write!(libcc2rs::cerr(), "Image too large.\n",);
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_IMAGE_TOO_LARGE);
            return false;
        }
        field!((*c.borrow()), num_blocks).write((((*num_blocks.borrow()) as i32) as u32));
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
            {
                let __a0 = ((((*c.borrow()).with(|__s| __s.num_blocks))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                    as usize) as usize;
                (*(*c.borrow()).with(|__s| __s.coeffs.clone()).borrow_mut())
                    .resize_with(__a0, || <i16>::default())
            };
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if ({ (*start_pos.borrow()).wrapping_add((*marker_len.borrow())) } != {
        ((*pos.borrow()).read())
    }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessSOS_236(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if ({ ((*pos.borrow()).read()).wrapping_add(((3) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    let comps_in_scan: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    if (((*comps_in_scan.borrow()) as usize) < 1_usize)
        || ({ ((*comps_in_scan.borrow()) as usize) } > {
            (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
        })
    {
        write!(
            libcc2rs::cerr(),
            "Invalid static_cast<size_t>(comps_in_scan): {:}\n",
            ((*comps_in_scan.borrow()) as usize),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_COMPS_IN_SCAN);
        return false;
    };
    let scan_info: Value<brunsli_JPEGScanInfo> =
        Rc::new(RefCell::new(<brunsli_JPEGScanInfo>::default()));
    (*scan_info.borrow_mut()).num_components = ((*comps_in_scan.borrow()) as usize);
    if ({ ((*pos.borrow()).read()).wrapping_add(((2 * (*comps_in_scan.borrow())) as usize)) } > {
        (*len.borrow())
    }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2 * (*comps_in_scan.borrow())),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let ids_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*comps_in_scan.borrow())) {
        let id: Value<i32> = Rc::new(RefCell::new(
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
        ));
        if ((*elem!(
            (ids_seen.as_pointer() as Ptr<bool>),
            ((*id.borrow()) as usize)
        )
        .upgrade()
        .deref()) as bool)
        {
            write!(
                libcc2rs::cerr(),
                "Duplicate ID {:} in SOS.\n",
                (*id.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_DUPLICATE_COMPONENT_ID);
            return false;
        }
        elem!(
            (ids_seen.as_pointer() as Ptr<bool>),
            ((*id.borrow()) as usize)
        )
        .write(true);
        let found_index: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*j.borrow()) } < {
            (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
        }) {
            if ({
                {
                    (*elem!(
                        ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                            as Ptr<brunsli_JPEGComponent>),
                        (*j.borrow())
                    )
                    .upgrade()
                    .deref())
                    .id
                }
            } == { (*id.borrow()) })
            {
                field!(
                    elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        ((*i.borrow()) as usize)
                    ),
                    comp_idx
                )
                .write(((*j.borrow()) as u8));
                (*found_index.borrow_mut()) = true;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        if !(*found_index.borrow()) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find component with id {:}\n",
                (*id.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_COMPONENT_NOT_FOUND);
            return false;
        }
        let c: Value<i32> = Rc::new(RefCell::new(
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
        ));
        let dc_tbl_idx: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) >> 4)));
        let ac_tbl_idx: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) & 15)));
        if ((*dc_tbl_idx.borrow()) < 0) || ((*dc_tbl_idx.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid dc_tbl_idx: {:}\n",
                (*dc_tbl_idx.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
            return false;
        };
        if ((*ac_tbl_idx.borrow()) < 0) || ((*ac_tbl_idx.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid ac_tbl_idx: {:}\n",
                (*ac_tbl_idx.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
            return false;
        };
        field!(
            elem!(
                ({ (*scan_info.borrow()).components.as_pointer() }
                    as Ptr<brunsli_JPEGComponentScanInfo>),
                ((*i.borrow()) as usize)
            ),
            dc_tbl_idx
        )
        .write((*dc_tbl_idx.borrow()));
        field!(
            elem!(
                ({ (*scan_info.borrow()).components.as_pointer() }
                    as Ptr<brunsli_JPEGComponentScanInfo>),
                ((*i.borrow()) as usize)
            ),
            ac_tbl_idx
        )
        .write((*ac_tbl_idx.borrow()));
        (*i.borrow_mut()).prefix_inc();
    }
    if ({ ((*pos.borrow()).read()).wrapping_add(((3) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    (*scan_info.borrow_mut()).Ss =
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) });
    (*scan_info.borrow_mut()).Se =
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) });
    if ({ (*scan_info.borrow()).Ss } < 0) || ({ (*scan_info.borrow()).Ss } > 63) {
        write!(libcc2rs::cerr(), "Invalid scan_info.Ss: {:}\n", {
            (*scan_info.borrow()).Ss
        },);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_START_OF_SCAN);
        return false;
    };
    if ({ (*scan_info.borrow()).Se } < { (*scan_info.borrow()).Ss })
        || ({ (*scan_info.borrow()).Se } > 63)
    {
        write!(libcc2rs::cerr(), "Invalid scan_info.Se: {:}\n", {
            (*scan_info.borrow()).Se
        },);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_END_OF_SCAN);
        return false;
    };
    let c: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    (*scan_info.borrow_mut()).Ah = ((*c.borrow()) >> 4);
    (*scan_info.borrow_mut()).Al = ((*c.borrow()) & 15);
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
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*comps_in_scan.borrow())) {
        let found_dc_table: Value<bool> = Rc::new(RefCell::new(false));
        let found_ac_table: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*j.borrow()) } < {
            (*(*jpg.borrow())
                .with(|__s| __s.huffman_code.clone())
                .borrow())
            .len()
        }) {
            let slot_id: Value<i32> = Rc::new(RefCell::new({
                (*elem!(
                    ((*jpg.borrow()).with(|__s| __s.huffman_code.as_pointer())
                        as Ptr<brunsli_JPEGHuffmanCode>),
                    (*j.borrow())
                )
                .upgrade()
                .deref())
                .slot_id
            }));
            if ((*slot_id.borrow()) == {
                (*elem!(
                    ({ (*scan_info.borrow()).components.as_pointer() }
                        as Ptr<brunsli_JPEGComponentScanInfo>),
                    ((*i.borrow()) as usize)
                )
                .upgrade()
                .deref())
                .dc_tbl_idx
            }) {
                (*found_dc_table.borrow_mut()) = true;
            } else if ((*slot_id.borrow())
                == ({
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        ((*i.borrow()) as usize)
                    )
                    .upgrade()
                    .deref())
                    .ac_tbl_idx
                } + 16))
            {
                (*found_ac_table.borrow_mut()) = true;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        if ({ (*scan_info.borrow()).Ss } == 0) && (!(*found_dc_table.borrow())) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find DC Huffman table with index {:}\n",
                {
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        ((*i.borrow()) as usize)
                    )
                    .upgrade()
                    .deref())
                    .dc_tbl_idx
                },
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_NOT_FOUND);
            return false;
        }
        if ({ (*scan_info.borrow()).Se } > 0) && (!(*found_ac_table.borrow())) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find AC Huffman table with index {:}\n",
                {
                    (*elem!(
                        ({ (*scan_info.borrow()).components.as_pointer() }
                            as Ptr<brunsli_JPEGComponentScanInfo>),
                        ((*i.borrow()) as usize)
                    )
                    .upgrade()
                    .deref())
                    .ac_tbl_idx
                },
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_NOT_FOUND);
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    {
        let a0_clone = (*scan_info.borrow()).clone();
        (*(*jpg.borrow())
            .with(|__s| __s.scan_info.clone())
            .borrow_mut())
        .push(a0_clone)
    };
    if ({ (*start_pos.borrow()).wrapping_add((*marker_len.borrow())) } != {
        ((*pos.borrow()).read())
    }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDHT_237(
    data: Ptr<u8>,
    len: usize,
    mode: brunsli_JpegReadMode,
    dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let mode: Value<brunsli_JpegReadMode> = Rc::new(RefCell::new(mode));
    let dc_huff_lut: Value<Ptr<Vec<brunsli_HuffmanTableEntry>>> =
        Rc::new(RefCell::new(dc_huff_lut));
    let ac_huff_lut: Value<Ptr<Vec<brunsli_HuffmanTableEntry>>> =
        Rc::new(RefCell::new(ac_huff_lut));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if ({ ((*pos.borrow()).read()).wrapping_add(((2) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    if ((*marker_len.borrow()) == 2_usize) {
        write!(libcc2rs::cerr(), "DHT marker: no Huffman table found\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_EMPTY_DHT);
        return false;
    }
    'loop_: while ({ ((*pos.borrow()).read()) } < {
        (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
    }) {
        if ({
            ((*pos.borrow()).read())
                .wrapping_add(((1 + kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) as usize))
        } > { (*len.borrow()) })
        {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (1 + kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())),
                (*len.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let huff: Value<brunsli_JPEGHuffmanCode> =
            Rc::new(RefCell::new(<brunsli_JPEGHuffmanCode>::default()));
        (*huff.borrow_mut()).slot_id =
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) });
        let huffman_index: Value<i32> = Rc::new(RefCell::new({ (*huff.borrow()).slot_id }));
        let is_ac_table: Value<i32> = Rc::new(RefCell::new(
            ((({ (*huff.borrow()).slot_id } & 16) != 0) as i32),
        ));
        let huff_lut: Value<Ptr<brunsli_HuffmanTableEntry>> =
            Rc::new(RefCell::new(Ptr::<brunsli_HuffmanTableEntry>::null()));
        if ((*is_ac_table.borrow()) != 0) {
            (*huffman_index.borrow_mut()) -= 16;
            if ((*huffman_index.borrow()) < 0) || ((*huffman_index.borrow()) > 3) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid huffman_index: {:}\n",
                    (*huffman_index.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
                return false;
            };
            (*huff_lut.borrow_mut()) =
                (((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(*ac_huff_lut.borrow())))
                    as Ptr<brunsli_HuffmanTableEntry>)
                    .offset(
                        (((*huffman_index.borrow())
                            * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow()))
                            as usize),
                    ));
        } else {
            if ((*huffman_index.borrow()) < 0) || ((*huffman_index.borrow()) > 3) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid huffman_index: {:}\n",
                    (*huffman_index.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_INDEX);
                return false;
            };
            (*huff_lut.borrow_mut()) =
                (((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(*dc_huff_lut.borrow())))
                    as Ptr<brunsli_HuffmanTableEntry>)
                    .offset(
                        (((*huffman_index.borrow())
                            * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow()))
                            as usize),
                    ));
        }
        elem!(
            ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
            0_usize
        )
        .write(0);
        let total_count: Value<i32> = Rc::new(RefCell::new(0));
        let space: Value<i32> = Rc::new(RefCell::new(
            (1 << kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())),
        ));
        let max_depth: Value<i32> = Rc::new(RefCell::new(1));
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
            let count: Value<i32> = Rc::new(RefCell::new(
                ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
            ));
            if ((*count.borrow()) != 0) {
                (*max_depth.borrow_mut()) = (*i.borrow());
            }
            elem!(
                ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
                ((*i.borrow()) as usize)
            )
            .write((*count.borrow()));
            (*total_count.borrow_mut()) += (*count.borrow());
            (*space.borrow_mut()) -= ((*count.borrow())
                * (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - (*i.borrow()))));
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*is_ac_table.borrow()) != 0) {
            if ((*total_count.borrow()) < 0)
                || ((*total_count.borrow()) > kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()))
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid total_count: {:}\n",
                    (*total_count.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            };
        } else {
            if ((*total_count.borrow()) < 0)
                || ((*total_count.borrow()) > kJpegDCAlphabetSize_9.with(|rc| *rc.borrow()))
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid total_count: {:}\n",
                    (*total_count.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            };
        }
        if ({ ((*pos.borrow()).read()).wrapping_add(((*total_count.borrow()) as usize)) } > {
            (*len.borrow())
        }) {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (*total_count.borrow()),
                (*len.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let values_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(256_usize) as usize)
                .map(|_| false)
                .collect::<Vec<bool>>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*total_count.borrow())) {
            let value: Value<u8> = Rc::new(RefCell::new(
                (({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as u8),
            ));
            if !((*is_ac_table.borrow()) != 0) {
                if (((*value.borrow()) as i32) < 0)
                    || (((*value.borrow()) as i32)
                        > (kJpegDCAlphabetSize_9.with(|rc| *rc.borrow()) - 1))
                {
                    write!(libcc2rs::cerr(), "Invalid value: ",);
                    libcc2rs::cerr().write_all(
                        &([(&[(*value.borrow()) as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()),
                    );
                    field!((*jpg.borrow()), error)
                        .write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                    return false;
                };
            }
            if ((*elem!(
                (values_seen.as_pointer() as Ptr<bool>),
                ((*value.borrow()) as usize)
            )
            .upgrade()
            .deref()) as bool)
            {
                write!(libcc2rs::cerr(), "Duplicate Huffman code value ",);
                libcc2rs::cerr().write_all(
                    &([(&[(*value.borrow()) as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
                return false;
            }
            elem!(
                (values_seen.as_pointer() as Ptr<bool>),
                ((*value.borrow()) as usize)
            )
            .write(true);
            elem!(
                ({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>),
                ((*i.borrow()) as usize)
            )
            .write(((*value.borrow()) as i32));
            (*i.borrow_mut()).prefix_inc();
        }
        elem!(
            ({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>),
            ((*max_depth.borrow()) as usize)
        )
        .with_mut(|__v| __v.prefix_inc());
        elem!(
            ({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>),
            ((*total_count.borrow()) as usize)
        )
        .write(kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()));
        (*space.borrow_mut()) -=
            (1 << (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) - (*max_depth.borrow())));
        if ((*space.borrow()) < 0) {
            write!(libcc2rs::cerr(), "Invalid Huffman code lengths.\n",);
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_HUFFMAN_CODE);
            return false;
        } else if ((*space.borrow()) > 0)
            && (({ (*elem!((*huff_lut.borrow()), 0).upgrade().deref()).value } as i32) != 65535)
        {
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())) {
                field!(elem!((*huff_lut.borrow()), (*i.borrow())), bits).write(0_u8);
                field!(elem!((*huff_lut.borrow()), (*i.borrow())), value).write(65535_u16);
                (*i.borrow_mut()).prefix_inc();
            }
        }
        (*huff.borrow_mut()).is_last = ({ ((*pos.borrow()).read()) } == {
            (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
        });
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
            ({
                let _counts: Ptr<i32> =
                    (({ (*huff.borrow()).counts.as_pointer() } as Ptr<i32>).offset(0_usize));
                let _symbols: Ptr<i32> =
                    (({ (*huff.borrow()).values.as_pointer() } as Ptr<i32>).offset(0_usize));
                BuildJpegHuffmanTable_238(_counts, _symbols, (*huff_lut.borrow()).clone())
            });
        }
        {
            let a0_clone = (*huff.borrow()).clone();
            (*(*jpg.borrow())
                .with(|__s| __s.huffman_code.clone())
                .borrow_mut())
            .push(a0_clone)
        };
    }
    if ({ (*start_pos.borrow()).wrapping_add((*marker_len.borrow())) } != {
        ((*pos.borrow()).read())
    }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDQT_239(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if ({ ((*pos.borrow()).read()).wrapping_add(((2) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    if ((*marker_len.borrow()) == 2_usize) {
        write!(
            libcc2rs::cerr(),
            "DQT marker: no quantization table found\n",
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_EMPTY_DQT);
        return false;
    }
    'loop_: while ({ ((*pos.borrow()).read()) } < {
        (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
    }) && ({ (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len() } < {
        (kMaxQuantTables_5.with(|rc| *rc.borrow()) as usize)
    }) {
        if ({ ((*pos.borrow()).read()).wrapping_add(((1) as usize)) } > { (*len.borrow()) }) {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (1),
                (*len.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let quant_table_index: Value<i32> = Rc::new(RefCell::new(
            ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
        ));
        let quant_table_precision: Value<i32> =
            Rc::new(RefCell::new(((*quant_table_index.borrow()) >> 4)));
        if ((*quant_table_precision.borrow()) < 0) || ((*quant_table_precision.borrow()) > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_precision: {:}\n",
                (*quant_table_precision.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_QUANT_TBL_PRECISION);
            return false;
        };
        (*quant_table_index.borrow_mut()) &= 15;
        if ((*quant_table_index.borrow()) < 0) || ((*quant_table_index.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_index: {:}\n",
                (*quant_table_index.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_QUANT_TBL_INDEX);
            return false;
        };
        if ({
            ((*pos.borrow()).read()).wrapping_add(
                ((((*quant_table_precision.borrow()) + 1) * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                    as usize),
            )
        } > { (*len.borrow()) })
        {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (((*quant_table_precision.borrow()) + 1) * kDCTBlockSize_3.with(|rc| *rc.borrow())),
                (*len.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
            return false;
        };
        let table: Value<brunsli_JPEGQuantTable> =
            Rc::new(RefCell::new(<brunsli_JPEGQuantTable>::default()));
        (*table.borrow_mut()).index = (*quant_table_index.borrow());
        (*table.borrow_mut()).precision = (*quant_table_precision.borrow());
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
            let quant_val: Value<i32> =
                Rc::new(RefCell::new(if ((*quant_table_precision.borrow()) != 0) {
                    ({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) })
                } else {
                    ({ ReadUint8_233((*data.borrow()).clone(), (*pos.borrow()).clone()) })
                }));
            if ((*quant_val.borrow()) < 1) || ((*quant_val.borrow()) > 65535) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid quant_val: {:}\n",
                    (*quant_val.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_QUANT_VAL);
                return false;
            };
            elem!(
                ({ (*table.borrow()).values.as_pointer() } as Ptr<i32>),
                (({
                    let __idx = (*i.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as usize)
            )
            .write((*quant_val.borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        (*table.borrow_mut()).is_last = ({ ((*pos.borrow()).read()) } == {
            (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
        });
        {
            let a0_clone = (*table.borrow()).clone();
            (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow_mut()).push(a0_clone)
        };
    }
    if ({ (*start_pos.borrow()).wrapping_add((*marker_len.borrow())) } != {
        ((*pos.borrow()).read())
    }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessDRI_240(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    found_dri: Ptr<bool>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let found_dri: Value<Ptr<bool>> = Rc::new(RefCell::new(found_dri));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if ((*found_dri.borrow()).read()) {
        write!(libcc2rs::cerr(), "Duplicate DRI marker.\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_DUPLICATE_DRI);
        return false;
    }
    (*found_dri.borrow()).write(true);
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if ({ ((*pos.borrow()).read()).wrapping_add(((4) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (4),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        ({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }),
    ));
    field!((*jpg.borrow()), restart_interval).write((*restart_interval.borrow()));
    if ({ (*start_pos.borrow()).wrapping_add((*marker_len.borrow())) } != {
        ((*pos.borrow()).read())
    }) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_MARKER_SIZE);
        return false;
    };
    return true;
}
pub fn ProcessAPP_241(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if ({ ((*pos.borrow()).read()).wrapping_add(((2) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    if ((*marker_len.borrow()) < 2_usize) || ((*marker_len.borrow()) > 65535_usize) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker_len: {:}\n",
            (*marker_len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_MARKER_LEN);
        return false;
    };
    if ({ ((*pos.borrow()).read()).wrapping_add(((*marker_len.borrow()).wrapping_sub(2_usize))) }
        > { (*len.borrow()) })
    {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            ((*marker_len.borrow()).wrapping_sub(2_usize)),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let app_str_start: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*data.borrow())
            .offset(((*pos.borrow()).read()) as isize)
            .offset(-((3) as isize)),
    ));
    let app_str: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (*app_str_start.borrow())
            .offset((*marker_len.borrow()) as isize)
            .offset((1) as isize)
            .get_offset()
            - (*app_str_start.borrow()).get_offset();
        PtrValueIter::new(&(*app_str_start.borrow()), __count).collect::<Vec<_>>()
    }));
    (*pos.borrow()).write({
        ((*pos.borrow()).read()).wrapping_add((*marker_len.borrow()).wrapping_sub(2_usize))
    });
    ((*jpg.borrow()).with(|__s| __s.app_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new((*app_str.borrow()).clone())))
        },
    );
    return true;
}
pub fn ProcessCOM_242(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if ({ ((*pos.borrow()).read()).wrapping_add(((2) as usize)) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({ ReadUint16_234((*data.borrow()).clone(), (*pos.borrow()).clone()) }) as usize),
    ));
    if ((*marker_len.borrow()) < 2_usize) || ((*marker_len.borrow()) > 65535_usize) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker_len: {:}\n",
            (*marker_len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_MARKER_LEN);
        return false;
    };
    if ({ ((*pos.borrow()).read()).wrapping_add(((*marker_len.borrow()).wrapping_sub(2_usize))) }
        > { (*len.borrow()) })
    {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            ((*marker_len.borrow()).wrapping_sub(2_usize)),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    };
    let com_str_start: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*data.borrow())
            .offset(((*pos.borrow()).read()) as isize)
            .offset(-((3) as isize)),
    ));
    let com_str: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (*com_str_start.borrow())
            .offset((*marker_len.borrow()) as isize)
            .offset((1) as isize)
            .get_offset()
            - (*com_str_start.borrow()).get_offset();
        PtrValueIter::new(&(*com_str_start.borrow()), __count).collect::<Vec<_>>()
    }));
    (*pos.borrow()).write({
        ((*pos.borrow()).read()).wrapping_add((*marker_len.borrow()).wrapping_sub(2_usize))
    });
    ((*jpg.borrow()).with(|__s| __s.com_data.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new((*com_str.borrow()).clone())))
        },
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
    pub fn new(data: Ptr<u8>, len: usize, pos: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        let pos: Value<usize> = Rc::new(RefCell::new(pos));
        let __this: Value<brunsli_BitReaderState> = Rc::new(RefCell::new(Self {
            data_: (*data.borrow()).clone(),
            len_: (*len.borrow()),
            pos_: 0_usize,
            val_: 0_u64,
            bits_left_: 0_i32,
            next_marker_pos_: 0_usize,
        }));
        let this: Ptr<brunsli_BitReaderState> = __this.as_pointer();
        ({ brunsli_BitReaderStateImpl::Reset(&this, (*pos.borrow())) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn ReadSymbol_243(
    table: Ptr<brunsli_HuffmanTableEntry>,
    br: Ptr<brunsli_BitReaderState>,
) -> i32 {
    let table: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(table));
    let br: Value<Ptr<brunsli_BitReaderState>> = Rc::new(RefCell::new(br));
    let nbits: Value<i32> = Rc::new(RefCell::new(0_i32));
    ({ brunsli_BitReaderStateImpl::FillBitWindow(&(*br.borrow())) });
    let val: Value<i32> = Rc::new(RefCell::new(
        ((({ (*br.borrow()).with(|__s| __s.val_) } >> {
            ((*br.borrow()).with(|__s| __s.bits_left_) - 8)
        }) & 255_u64) as i32),
    ));
    (*table.borrow_mut()) += (*val.borrow());
    (*nbits.borrow_mut()) = (((*table.borrow()).with(|__s| __s.bits) as i32) - 8);
    if ((*nbits.borrow()) > 0) {
        {
            let _ptr = field!((*br.borrow()), bits_left_);
            _ptr.write(_ptr.read() - 8)
        };
        let __rhs = ((*table.borrow()).with(|__s| __s.value) as i32);
        (*table.borrow_mut()) += __rhs;
        (*val.borrow_mut()) = (({
            ({ (*br.borrow()).with(|__s| __s.val_) } >> {
                ({ (*br.borrow()).with(|__s| __s.bits_left_) } - { (*nbits.borrow()) })
            })
        } & { (((1 << (*nbits.borrow())) - 1) as u64) }) as i32);
        (*table.borrow_mut()) += (*val.borrow());
    }
    {
        let _ptr = field!((*br.borrow()), bits_left_);
        _ptr.write(_ptr.read() - ((*table.borrow()).with(|__s| __s.bits) as i32))
    };
    return ((*table.borrow()).with(|__s| __s.value) as i32);
}
pub fn HuffExtend_244(x: i32, s: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let s: Value<i32> = Rc::new(RefCell::new(s));
    if !((*s.borrow()) >= 1) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                575,
                Ptr::<i8>::from_string_literal(b"HuffExtend"),
            )
        });
        'loop_: while true {}
    };
    let half: Value<i32> = Rc::new(RefCell::new((1 << ((*s.borrow()) - 1))));
    if ((*x.borrow()) >= (*half.borrow())) {
        if !((*x.borrow()) < (1 << (*s.borrow()))) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                    578,
                    Ptr::<i8>::from_string_literal(b"HuffExtend"),
                )
            });
            'loop_: while true {}
        };
        return (*x.borrow());
    } else {
        return (((*x.borrow()) - (1 << (*s.borrow()))) + 1);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeDCTBlock_245(
    dc_huff: Ptr<brunsli_HuffmanTableEntry>,
    ac_huff: Ptr<brunsli_HuffmanTableEntry>,
    Ss: i32,
    Se: i32,
    Al: i32,
    eobrun: Ptr<i32>,
    reset_state: Ptr<bool>,
    num_zero_runs: Ptr<i32>,
    br: Ptr<brunsli_BitReaderState>,
    jpg: Ptr<brunsli_JPEGData>,
    last_dc_coeff: Ptr<i16>,
    coeffs: Ptr<i16>,
) -> bool {
    let dc_huff: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(dc_huff));
    let ac_huff: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(ac_huff));
    let Ss: Value<i32> = Rc::new(RefCell::new(Ss));
    let Se: Value<i32> = Rc::new(RefCell::new(Se));
    let Al: Value<i32> = Rc::new(RefCell::new(Al));
    let eobrun: Value<Ptr<i32>> = Rc::new(RefCell::new(eobrun));
    let reset_state: Value<Ptr<bool>> = Rc::new(RefCell::new(reset_state));
    let num_zero_runs: Value<Ptr<i32>> = Rc::new(RefCell::new(num_zero_runs));
    let br: Value<Ptr<brunsli_BitReaderState>> = Rc::new(RefCell::new(br));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let last_dc_coeff: Value<Ptr<i16>> = Rc::new(RefCell::new(last_dc_coeff));
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let Am: Value<i32> = Rc::new(RefCell::new((1 << (*Al.borrow()))));
    let eobrun_allowed: Value<bool> = Rc::new(RefCell::new(((*Ss.borrow()) > 0)));
    if ((*Ss.borrow()) == 0) {
        let s: Value<i32> = Rc::new(RefCell::new(
            ({ ReadSymbol_243((*dc_huff.borrow()).clone(), (*br.borrow()).clone()) }),
        ));
        if ((*s.borrow()) >= kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for DC coefficient.\n",
                (*s.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
            return false;
        }
        let diff: Value<i32> = Rc::new(RefCell::new(0));
        if ((*s.borrow()) > 0) {
            let bits: Value<i32> = Rc::new(RefCell::new(
                ({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), (*s.borrow())) }),
            ));
            (*diff.borrow_mut()) = ({ HuffExtend_244((*bits.borrow()), (*s.borrow())) });
        }
        let coeff: Value<i32> = Rc::new(RefCell::new(
            ({ (*diff.borrow()) } + { (((*last_dc_coeff.borrow()).read()) as i32) }),
        ));
        let dc_coeff: Value<i32> = Rc::new(RefCell::new(((*coeff.borrow()) * (*Am.borrow()))));
        elem!((*coeffs.borrow()), 0).write({ ((*dc_coeff.borrow()) as i16) });
        if ({ (*dc_coeff.borrow()) } != { ((elem!((*coeffs.borrow()), 0).read()) as i32) }) {
            write!(
                libcc2rs::cerr(),
                "Invalid DC coefficient {:}\n",
                (*dc_coeff.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_NON_REPRESENTABLE_DC_COEFF);
            return false;
        }
        (*last_dc_coeff.borrow()).write({ ((*coeff.borrow()) as i16) });
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    if (((*eobrun.borrow()).read()) > 0) {
        (*eobrun.borrow()).with_mut(|__v| __v.prefix_dec());
        return true;
    }
    (*num_zero_runs.borrow()).write(0);
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        let sr: Value<i32> = Rc::new(RefCell::new(
            ({ ReadSymbol_243((*ac_huff.borrow()).clone(), (*br.borrow()).clone()) }),
        ));
        if ((*sr.borrow()) >= kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                (*sr.borrow()),
                (*k.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
            return false;
        }
        let r: Value<i32> = Rc::new(RefCell::new(((*sr.borrow()) >> 4)));
        let s: Value<i32> = Rc::new(RefCell::new(((*sr.borrow()) & 15)));
        if ((*s.borrow()) > 0) {
            (*k.borrow_mut()) += (*r.borrow());
            if ((*k.borrow()) > (*Se.borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Out-of-band coefficient {:} band was {:}-{:}\n",
                    (*k.borrow()),
                    (*Ss.borrow()),
                    (*Se.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_OUT_OF_BAND_COEFF);
                return false;
            }
            if (((*s.borrow()) + (*Al.borrow())) >= kJpegDCAlphabetSize_9.with(|rc| *rc.borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Out of range AC coefficient value: s = {:} Al = {:} k = {:}\n",
                    (*s.borrow()),
                    (*Al.borrow()),
                    (*k.borrow()),
                );
                field!((*jpg.borrow()), error)
                    .write(brunsli_JPEGReadError_NON_REPRESENTABLE_AC_COEFF);
                return false;
            }
            let bits: Value<i32> = Rc::new(RefCell::new(
                ({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), (*s.borrow())) }),
            ));
            let coeff: Value<i32> = Rc::new(RefCell::new(
                ({ HuffExtend_244((*bits.borrow()), (*s.borrow())) }),
            ));
            elem!(
                (*coeffs.borrow()),
                ({
                    let __idx = (*k.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                })
            )
            .write({ (((*coeff.borrow()) * (*Am.borrow())) as i16) });
            (*num_zero_runs.borrow()).write(0);
        } else if ((*r.borrow()) == 15) {
            (*k.borrow_mut()) += 15;
            (*num_zero_runs.borrow()).with_mut(|__v| __v.prefix_inc());
        } else {
            if ((*eobrun_allowed.borrow()) && ((*k.borrow()) == (*Ss.borrow())))
                && (((*eobrun.borrow()).read()) == 0)
            {
                (*reset_state.borrow()).write(true);
            }
            (*eobrun.borrow()).write({ (1 << (*r.borrow())) });
            if ((*r.borrow()) > 0) {
                if !(*eobrun_allowed.borrow()) {
                    write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                    field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                    return false;
                }
                let __rhs =
                    ({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), (*r.borrow())) });
                {
                    let _ptr = (*eobrun.borrow()).clone();
                    _ptr.write(_ptr.read() + __rhs)
                };
            }
            break;
        }
        (*k.borrow_mut()).postfix_inc();
    }
    (*eobrun.borrow()).with_mut(|__v| __v.prefix_dec());
    return true;
}
pub fn RefineDCTBlock_246(
    ac_huff: Ptr<brunsli_HuffmanTableEntry>,
    Ss: i32,
    Se: i32,
    Al: i32,
    eobrun: Ptr<i32>,
    reset_state: Ptr<bool>,
    br: Ptr<brunsli_BitReaderState>,
    jpg: Ptr<brunsli_JPEGData>,
    coeffs: Ptr<i16>,
) -> bool {
    let ac_huff: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(ac_huff));
    let Ss: Value<i32> = Rc::new(RefCell::new(Ss));
    let Se: Value<i32> = Rc::new(RefCell::new(Se));
    let Al: Value<i32> = Rc::new(RefCell::new(Al));
    let eobrun: Value<Ptr<i32>> = Rc::new(RefCell::new(eobrun));
    let reset_state: Value<Ptr<bool>> = Rc::new(RefCell::new(reset_state));
    let br: Value<Ptr<brunsli_BitReaderState>> = Rc::new(RefCell::new(br));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let Am: Value<i32> = Rc::new(RefCell::new((1 << (*Al.borrow()))));
    let eobrun_allowed: Value<bool> = Rc::new(RefCell::new(((*Ss.borrow()) > 0)));
    if ((*Ss.borrow()) == 0) {
        let s: Value<i32> = Rc::new(RefCell::new(
            ({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), 1) }),
        ));
        let dc_coeff: Value<i16> = Rc::new(RefCell::new((elem!((*coeffs.borrow()), 0).read())));
        (*dc_coeff.borrow_mut()) =
            { (((*dc_coeff.borrow()) as i32) | ((*s.borrow()) * (*Am.borrow()))) as i16 };
        elem!((*coeffs.borrow()), 0).write({ (*dc_coeff.borrow()) });
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    let p1: Value<i32> = Rc::new(RefCell::new((*Am.borrow())));
    let m1: Value<i32> = Rc::new(RefCell::new(-(*Am.borrow())));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    let r: Value<i32> = Rc::new(RefCell::new(0_i32));
    let s: Value<i32> = Rc::new(RefCell::new(0_i32));
    let in_zero_run: Value<bool> = Rc::new(RefCell::new(false));
    if (((*eobrun.borrow()).read()) <= 0) {
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            (*s.borrow_mut()) =
                ({ ReadSymbol_243((*ac_huff.borrow()).clone(), (*br.borrow()).clone()) });
            if ((*s.borrow()) >= kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                    (*s.borrow()),
                    (*k.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
                return false;
            }
            (*r.borrow_mut()) = ((*s.borrow()) >> 4);
            (*s.borrow_mut()) &= 15;
            if ((*s.borrow()) != 0) {
                if ((*s.borrow()) != 1) {
                    write!(
                        libcc2rs::cerr(),
                        "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                        (*s.borrow()),
                        (*k.borrow()),
                    );
                    field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SYMBOL);
                    return false;
                }
                (*s.borrow_mut()) =
                    if (({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), 1) }) != 0) {
                        (*p1.borrow())
                    } else {
                        (*m1.borrow())
                    };
                (*in_zero_run.borrow_mut()) = false;
            } else {
                if ((*r.borrow()) != 15) {
                    if ((*eobrun_allowed.borrow()) && ((*k.borrow()) == (*Ss.borrow())))
                        && (((*eobrun.borrow()).read()) == 0)
                    {
                        (*reset_state.borrow()).write(true);
                    }
                    (*eobrun.borrow()).write({ (1 << (*r.borrow())) });
                    if ((*r.borrow()) > 0) {
                        if !(*eobrun_allowed.borrow()) {
                            write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                            field!((*jpg.borrow()), error)
                                .write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                            return false;
                        }
                        let __rhs = ({
                            brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), (*r.borrow()))
                        });
                        {
                            let _ptr = (*eobrun.borrow()).clone();
                            _ptr.write(_ptr.read() + __rhs)
                        };
                    }
                    break;
                }
                (*in_zero_run.borrow_mut()) = true;
            }
            let mut __do_while = true;
            'loop_: while __do_while || ((*k.borrow()) <= (*Se.borrow())) {
                __do_while = false;
                let thiscoef: Value<i16> = Rc::new(RefCell::new(
                    (elem!(
                        (*coeffs.borrow()),
                        ({
                            let __idx = (*k.borrow()) as usize;
                            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                        })
                    )
                    .read()),
                ));
                if (((*thiscoef.borrow()) as i32) != 0) {
                    if (({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), 1) }) != 0) {
                        if ((((*thiscoef.borrow()) as i32) & (*p1.borrow())) == 0) {
                            if (((*thiscoef.borrow()) as i32) >= 0) {
                                (*thiscoef.borrow_mut()) =
                                    { (((*thiscoef.borrow()) as i32) + (*p1.borrow())) as i16 };
                            } else {
                                (*thiscoef.borrow_mut()) =
                                    { (((*thiscoef.borrow()) as i32) + (*m1.borrow())) as i16 };
                            }
                        }
                    }
                    elem!(
                        (*coeffs.borrow()),
                        ({
                            let __idx = (*k.borrow()) as usize;
                            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                        })
                    )
                    .write({ (*thiscoef.borrow()) });
                } else {
                    if ((*r.borrow_mut()).prefix_dec() < 0) {
                        break;
                    }
                }
                (*k.borrow_mut()).postfix_inc();
            }
            if ((*s.borrow()) != 0) {
                if ((*k.borrow()) > (*Se.borrow())) {
                    write!(
                        libcc2rs::cerr(),
                        "Out-of-band coefficient {:} band was {:}-{:}\n",
                        (*k.borrow()),
                        (*Ss.borrow()),
                        (*Se.borrow()),
                    );
                    field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_OUT_OF_BAND_COEFF);
                    return false;
                }
                elem!(
                    (*coeffs.borrow()),
                    ({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .write({ ((*s.borrow()) as i16) });
            }
            (*k.borrow_mut()).postfix_inc();
        }
    }
    if (*in_zero_run.borrow()) {
        write!(libcc2rs::cerr(), "Extra zero run before end-of-block.\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_EXTRA_ZERO_RUN);
        return false;
    }
    if (((*eobrun.borrow()).read()) > 0) {
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            let thiscoef: Value<i16> = Rc::new(RefCell::new(
                (elem!(
                    (*coeffs.borrow()),
                    ({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read()),
            ));
            if (((*thiscoef.borrow()) as i32) != 0) {
                if (({ brunsli_BitReaderStateImpl::ReadBits(&(*br.borrow()), 1) }) != 0) {
                    if ((((*thiscoef.borrow()) as i32) & (*p1.borrow())) == 0) {
                        if (((*thiscoef.borrow()) as i32) >= 0) {
                            (*thiscoef.borrow_mut()) =
                                { (((*thiscoef.borrow()) as i32) + (*p1.borrow())) as i16 };
                        } else {
                            (*thiscoef.borrow_mut()) =
                                { (((*thiscoef.borrow()) as i32) + (*m1.borrow())) as i16 };
                        }
                    }
                }
                elem!(
                    (*coeffs.borrow()),
                    ({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .write({ (*thiscoef.borrow()) });
            }
            (*k.borrow_mut()).postfix_inc();
        }
    }
    (*eobrun.borrow()).with_mut(|__v| __v.prefix_dec());
    return true;
}
pub fn ProcessRestart_247(
    data: Ptr<u8>,
    len: usize,
    next_restart_marker: Ptr<i32>,
    br: Ptr<brunsli_BitReaderState>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let next_restart_marker: Value<Ptr<i32>> = Rc::new(RefCell::new(next_restart_marker));
    let br: Value<Ptr<brunsli_BitReaderState>> = Rc::new(RefCell::new(br));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if !({
        brunsli_BitReaderStateImpl::FinishStream(
            &(*br.borrow()),
            (*jpg.borrow()).clone(),
            (pos.as_pointer()),
        )
    }) {
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SCAN);
        return false;
    }
    let expected_marker: Value<i32> = Rc::new(RefCell::new(
        (208 + ((*next_restart_marker.borrow()).read())),
    ));
    if ((*pos.borrow()).wrapping_add(2_usize) > (*len.borrow()))
        || (((elem!((*data.borrow()), (*pos.borrow())).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < (*len.borrow())) {
                ((elem!((*data.borrow()), (*pos.borrow())).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
        return false;
    };
    let marker: Value<i32> = Rc::new(RefCell::new(
        ((elem!((*data.borrow()), (*pos.borrow()).wrapping_add(1_usize)).read()) as i32),
    ));
    if ((*marker.borrow()) != (*expected_marker.borrow())) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected restart marker {:} actual={:}\n",
            (*expected_marker.borrow()),
            (*marker.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_WRONG_RESTART_MARKER);
        return false;
    }
    ({ brunsli_BitReaderStateImpl::Reset(&(*br.borrow()), (*pos.borrow()).wrapping_add(2_usize)) });
    {
        let _ptr = (*next_restart_marker.borrow()).clone();
        _ptr.write(_ptr.read() + 1)
    };
    {
        let _ptr = (*next_restart_marker.borrow()).clone();
        _ptr.write(_ptr.read() & 7)
    };
    return true;
}
pub fn ProcessScan_248(
    data: Ptr<u8>,
    len: usize,
    dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>>,
    scan_progression: Ptr<Value<Box<[u16]>>>,
    is_progressive: bool,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let scan_progression: Value<Ptr<Value<Box<[u16]>>>> = Rc::new(RefCell::new(scan_progression));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(is_progressive));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if !({
        let _len: usize = (*len.borrow());
        let _pos: Ptr<usize> = (*pos.borrow()).clone();
        ProcessSOS_236(
            (*data.borrow()).clone(),
            _len,
            _pos,
            (*jpg.borrow()).clone(),
        )
    }) {
        return false;
    }
    let scan_info: Value<Ptr<brunsli_JPEGScanInfo>> = Rc::new(RefCell::new(
        (((*jpg.borrow()).with(|__s| __s.scan_info.as_pointer()) as Ptr<brunsli_JPEGScanInfo>)
            .to_last()),
    ));
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        ((*scan_info.borrow()).with(|__s| __s.num_components) > 1_usize),
    ));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(0_i32));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(0_i32));
    if (*is_interleaved.borrow()) {
        (*MCUs_per_row.borrow_mut()) = (*jpg.borrow()).with(|__s| __s.MCU_cols);
        (*MCU_rows.borrow_mut()) = (*jpg.borrow()).with(|__s| __s.MCU_rows);
    } else {
        let c: Ptr<brunsli_JPEGComponent> =
            ((*jpg.borrow()).with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>)
                .offset(
                    ({
                        (*elem!(
                            ((*scan_info.borrow()).with(|__s| __s.components.as_pointer())
                                as Ptr<brunsli_JPEGComponentScanInfo>),
                            0_usize
                        )
                        .upgrade()
                        .deref())
                        .comp_idx
                    } as usize),
                );
        (*MCUs_per_row.borrow_mut()) = ({
            let _a: i32 =
                ({ (*jpg.borrow()).with(|__s| __s.width) } * { c.with(|__s| __s.h_samp_factor) });
            let _b: i32 = (8 * (*jpg.borrow()).with(|__s| __s.max_h_samp_factor));
            DivCeil_232(_a, _b)
        });
        (*MCU_rows.borrow_mut()) = ({
            let _a: i32 =
                ({ (*jpg.borrow()).with(|__s| __s.height) } * { c.with(|__s| __s.v_samp_factor) });
            let _b: i32 = (8 * (*jpg.borrow()).with(|__s| __s.max_v_samp_factor));
            DivCeil_232(_a, _b)
        });
    }
    let last_dc_coeff: Value<Box<[i16]>> =
        Rc::new(RefCell::new(Box::new([0_i16, 0_i16, 0_i16, 0_i16])));
    let br: Value<brunsli_BitReaderState> = Rc::new(RefCell::new(brunsli_BitReaderState::new(
        { (*data.borrow()).clone() },
        { (*len.borrow()) },
        { ((*pos.borrow()).read()) },
    )));
    let restarts_to_go: Value<i32> = Rc::new(RefCell::new(
        (*jpg.borrow()).with(|__s| __s.restart_interval),
    ));
    let next_restart_marker: Value<i32> = Rc::new(RefCell::new(0));
    let eobrun: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let block_scan_index: Value<i32> = Rc::new(RefCell::new(0));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*scan_info.borrow()).with(|__s| __s.Al)
    } else {
        0
    }));
    let Ah: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*scan_info.borrow()).with(|__s| __s.Ah)
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*scan_info.borrow()).with(|__s| __s.Ss)
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*scan_info.borrow()).with(|__s| __s.Se)
    } else {
        63
    }));
    let scan_bitmask: Value<u16> = Rc::new(RefCell::new(
        (if ((*Ah.borrow()) == 0) {
            ((65535 << (*Al.borrow())) as u32)
        } else {
            (1_u32 << (*Al.borrow()))
        } as u16),
    ));
    let refinement_bitmask: Value<u16> =
        Rc::new(RefCell::new((((1 << (*Al.borrow())) - 1) as u16)));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < { (*scan_info.borrow()).with(|__s| __s.num_components) }) {
        let comp_idx: Value<i32> = Rc::new(RefCell::new(
            ({
                (*elem!(
                    ((*scan_info.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponentScanInfo>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .comp_idx
            } as i32),
        ));
        let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            if (({
                ((elem!((*scan_progression.borrow()), (*comp_idx.borrow())).read()).borrow()
                    [(*k.borrow()) as usize] as i32)
            } & { ((*scan_bitmask.borrow()) as i32) })
                != 0)
            {
                write!(
                    libcc2rs::cerr(),
                    "Overlapping scans: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    (*comp_idx.borrow()),
                    (*k.borrow()),
                    (elem!((*scan_progression.borrow()), (*i.borrow())).read()).borrow()
                        [(*k.borrow()) as usize],
                    (*scan_bitmask.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_OVERLAPPING_SCANS);
                return false;
            }
            if (({
                ((elem!((*scan_progression.borrow()), (*comp_idx.borrow())).read()).borrow()
                    [(*k.borrow()) as usize] as i32)
            } & { ((*refinement_bitmask.borrow()) as i32) })
                != 0)
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid scan order, a more refined scan was already done: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    (*comp_idx.borrow()),
                    (*k.borrow()),
                    (elem!((*scan_progression.borrow()), (*i.borrow())).read()).borrow()
                        [(*k.borrow()) as usize],
                    (*scan_bitmask.borrow()),
                );
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SCAN_ORDER);
                return false;
            }
            (elem!((*scan_progression.borrow()), (*comp_idx.borrow())).read()).borrow_mut()
                [(*k.borrow()) as usize] = {
                (((elem!((*scan_progression.borrow()), (*comp_idx.borrow())).read()).borrow()
                    [(*k.borrow()) as usize] as i32)
                    | ((*scan_bitmask.borrow()) as i32)) as u16
            };
            (*k.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*Al.borrow()) > 10) {
        write!(
            libcc2rs::cerr(),
            "Scan parameter Al = {:} is not supported in brunsli.\n",
            (*Al.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_NON_REPRESENTABLE_AC_COEFF);
        return false;
    }
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*MCU_rows.borrow())) {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*jpg.borrow()).with(|__s| __s.restart_interval) > 0) {
                if ((*restarts_to_go.borrow()) == 0) {
                    if ({
                        ProcessRestart_247(
                            (*data.borrow()).clone(),
                            (*len.borrow()),
                            (next_restart_marker.as_pointer()),
                            (br.as_pointer()),
                            (*jpg.borrow()).clone(),
                        )
                    }) {
                        (*restarts_to_go.borrow_mut()) =
                            (*jpg.borrow()).with(|__s| __s.restart_interval);
                        {
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                                .to_any()
                                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>).to_any()
                        };
                        if ((*eobrun.borrow()) > 0) {
                            write!(libcc2rs::cerr(), "End-of-block run too long.\n",);
                            field!((*jpg.borrow()), error)
                                .write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
                            return false;
                        }
                        (*eobrun.borrow_mut()) = -1_i32;
                    } else {
                        return false;
                    }
                }
                (*restarts_to_go.borrow_mut()).prefix_dec();
            }
            if ({ brunsli_BitReaderStateImpl::IsUnhealthy(&br.as_pointer()) }) {
                write!(libcc2rs::cerr(), "Unexpected end of scan.\n",);
                field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SCAN);
                return false;
            }
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ({ (*i.borrow()) } < {
                (*scan_info.borrow()).with(|__s| __s.num_components)
            }) {
                let si: Value<Ptr<brunsli_JPEGComponentScanInfo>> = Rc::new(RefCell::new(
                    (((*scan_info.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponentScanInfo>)
                        .offset((*i.borrow()))),
                ));
                let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
                    (((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>)
                        .offset(((*si.borrow()).with(|__s| __s.comp_idx) as usize))),
                ));
                let dc_lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
                    ((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(dc_huff_lut))
                        as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({ (*si.borrow()).with(|__s| __s.dc_tbl_idx) } * {
                                kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())
                            }) as usize),
                        )),
                ));
                let ac_lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
                    ((Ptr::<Vec<brunsli_HuffmanTableEntry>>::decay(&(ac_huff_lut))
                        as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({ (*si.borrow()).with(|__s| __s.ac_tbl_idx) } * {
                                kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())
                            }) as usize),
                        )),
                ));
                let nblocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*c.borrow()).with(|__s| __s.v_samp_factor)
                } else {
                    1
                }));
                let nblocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*c.borrow()).with(|__s| __s.h_samp_factor)
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*nblocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*nblocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_y.borrow()) * (*nblocks_y.borrow())) + (*iy.borrow())),
                        ));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*nblocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul((*c.borrow()).with(|__s| __s.width_in_blocks)))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        let reset_state: Value<bool> = Rc::new(RefCell::new(false));
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            (((*c.borrow()).with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>)
                                .offset(
                                    (((*block_idx.borrow())
                                        * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                        as usize),
                                )),
                        ));
                        if ((*Ah.borrow()) == 0) {
                            if !({
                                DecodeDCTBlock_245(
                                    (*dc_lut.borrow()).clone(),
                                    (*ac_lut.borrow()).clone(),
                                    (*Ss.borrow()),
                                    (*Se.borrow()),
                                    (*Al.borrow()),
                                    (eobrun.as_pointer()),
                                    (reset_state.as_pointer()),
                                    (num_zero_runs.as_pointer()),
                                    (br.as_pointer()),
                                    (*jpg.borrow()).clone(),
                                    ((last_dc_coeff.as_pointer() as Ptr<i16>)
                                        .offset((*si.borrow()).with(|__s| __s.comp_idx))),
                                    (*coeffs.borrow()).clone(),
                                )
                            }) {
                                return false;
                            }
                        } else {
                            if !({
                                RefineDCTBlock_246(
                                    (*ac_lut.borrow()).clone(),
                                    (*Ss.borrow()),
                                    (*Se.borrow()),
                                    (*Al.borrow()),
                                    (eobrun.as_pointer()),
                                    (reset_state.as_pointer()),
                                    (br.as_pointer()),
                                    (*jpg.borrow()).clone(),
                                    (*coeffs.borrow()).clone(),
                                )
                            }) {
                                return false;
                            }
                        }
                        if (*reset_state.borrow()) {
                            {
                                let __init = (*block_scan_index.borrow());
                                (*(*scan_info.borrow())
                                    .with(|__s| __s.reset_points.clone())
                                    .borrow_mut())
                                .push(__init)
                            };
                        }
                        if ((*num_zero_runs.borrow()) > 0) {
                            let info: Value<brunsli_JPEGScanInfo_ExtraZeroRunInfo> = Rc::new(
                                RefCell::new(<brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default()),
                            );
                            (*info.borrow_mut()).block_idx = (*block_scan_index.borrow());
                            (*info.borrow_mut()).num_extra_zero_runs = (*num_zero_runs.borrow());
                            {
                                let a0_clone = (*info.borrow()).clone();
                                (*(*scan_info.borrow())
                                    .with(|__s| __s.extra_zero_runs.clone())
                                    .borrow_mut())
                                .push(a0_clone)
                            };
                        }
                        (*block_scan_index.borrow_mut()).prefix_inc();
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        (*mcu_y.borrow_mut()).prefix_inc();
    }
    if ((*eobrun.borrow()) > 0) {
        write!(libcc2rs::cerr(), "End-of-block run too long.\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_EOB_RUN_TOO_LONG);
        return false;
    }
    if !({
        brunsli_BitReaderStateImpl::FinishStream(
            &br.as_pointer(),
            (*jpg.borrow()).clone(),
            (*pos.borrow()).clone(),
        )
    }) {
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_INVALID_SCAN);
        return false;
    }
    if ({ ((*pos.borrow()).read()) } > { (*len.borrow()) }) {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of file during scan. pos={:} len={:}\n",
            ((*pos.borrow()).read()),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNEXPECTED_EOF);
        return false;
    }
    return true;
}
pub fn FixupIndexes_249(jpg: Ptr<brunsli_JPEGData>) -> bool {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
    }) {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()))),
        ));
        let found_index: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*j.borrow()) } < {
            (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len()
        }) {
            if ({
                {
                    (*elem!(
                        ((*jpg.borrow()).with(|__s| __s.quant.as_pointer())
                            as Ptr<brunsli_JPEGQuantTable>),
                        (*j.borrow())
                    )
                    .upgrade()
                    .deref())
                    .index
                }
            } == { ((*c.borrow()).with(|__s| __s.quant_idx) as i32) })
            {
                field!((*c.borrow()), quant_idx).write(((*j.borrow()) as u8));
                (*found_index.borrow_mut()) = true;
                break;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        if !(*found_index.borrow()) {
            write!(libcc2rs::cerr(), "Quantization table with index ",);
            libcc2rs::cerr().write_all(
                &([
                    (&[(*c.borrow()).with(|__s| __s.quant_idx) as u8] as &[u8]),
                    (b" not found." as &[u8]),
                    (&[b'\n'] as &[u8]),
                ]
                .concat()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_QUANT_TABLE_NOT_FOUND);
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn FindNextMarker_250(data: Ptr<u8>, len: usize, pos: usize) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<usize> = Rc::new(RefCell::new(pos));
    thread_local!(
        static kIsValidMarker_251: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8,
        ])));
    );
    let num_skipped: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*pos.borrow()).wrapping_add(1_usize) < (*len.borrow()))
        && (((((elem!((*data.borrow()), (*pos.borrow())).read()) as i32) != 255)
            || (((elem!((*data.borrow()), (*pos.borrow()).wrapping_add(1_usize)).read()) as i32)
                < 192))
            || (!(({
                let __idx = (((elem!((*data.borrow()), (*pos.borrow()).wrapping_add(1_usize))
                    .read()) as i32)
                    - 192) as usize;
                kIsValidMarker_251.with(|rc| rc.borrow()[__idx])
            }) != 0)))
    {
        (*pos.borrow_mut()).prefix_inc();
        (*num_skipped.borrow_mut()).prefix_inc();
    }
    return (*num_skipped.borrow());
}
pub fn ReadJpeg_196(
    data: Ptr<u8>,
    len: usize,
    mode: brunsli_JpegReadMode,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let mode: Value<brunsli_JpegReadMode> = Rc::new(RefCell::new(mode));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*pos.borrow()).wrapping_add(2_usize) > (*len.borrow()))
        || (((elem!((*data.borrow()), (*pos.borrow())).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < (*len.borrow())) {
                ((elem!((*data.borrow()), (*pos.borrow())).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            (*len.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
        return false;
    };
    let marker: Value<i32> = Rc::new(RefCell::new(
        ((elem!((*data.borrow()), (*pos.borrow()).wrapping_add(1_usize)).read()) as i32),
    ));
    (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add(2_usize) };
    if ((*marker.borrow()) != 216) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected SOI marker, actual={:}\n",
            (*marker.borrow()),
        );
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_SOI_NOT_FOUND);
        return false;
    }
    let lut_size: Value<i32> = Rc::new(RefCell::new(
        (kMaxHuffmanTables_6.with(|rc| *rc.borrow())
            * kJpegHuffmanLutSize_231.with(|rc| *rc.borrow())),
    ));
    let dc_huff_lut: Value<Vec<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
        (0..((*lut_size.borrow()) as usize) as usize)
            .map(|_| <brunsli_HuffmanTableEntry>::default())
            .collect::<Vec<_>>(),
    ));
    let ac_huff_lut: Value<Vec<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
        (0..((*lut_size.borrow()) as usize) as usize)
            .map(|_| <brunsli_HuffmanTableEntry>::default())
            .collect::<Vec<_>>(),
    ));
    let found_sof: Value<bool> = Rc::new(RefCell::new(false));
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
        (*(*jpg.borrow())
            .with(|__s| __s.padding_bits.clone())
            .borrow_mut())
        .resize_with(__a0, || <i32>::default())
    };
    let is_progressive: Value<bool> = Rc::new(RefCell::new(false));
    let mut __do_while = true;
    'loop_: while __do_while || ((*marker.borrow()) != 217) {
        __do_while = false;
        let num_skipped: Value<usize> = Rc::new(RefCell::new(
            ({ FindNextMarker_250((*data.borrow()).clone(), (*len.borrow()), (*pos.borrow())) }),
        ));
        if ((*num_skipped.borrow()) > 0_usize) {
            {
                let __a1 = 255_u8;
                (*(*jpg.borrow())
                    .with(|__s| __s.marker_order.clone())
                    .borrow_mut())
                .push(__a1)
            };
            ((*jpg.borrow()).with(|__s| __s.inter_marker_data.as_pointer())
                as Ptr<Vec<Value<Vec<u8>>>>)
                .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                    __v.push(Rc::new(RefCell::new({
                        let __count = (*data.borrow())
                            .offset((*pos.borrow()) as isize)
                            .offset((*num_skipped.borrow()) as isize)
                            .get_offset()
                            - (*data.borrow())
                                .offset((*pos.borrow()) as isize)
                                .get_offset();
                        PtrValueIter::new(
                            &(*data.borrow()).offset((*pos.borrow()) as isize),
                            __count,
                        )
                        .collect::<Vec<_>>()
                    })))
                });
            (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add((*num_skipped.borrow())) };
        }
        if ((*pos.borrow()).wrapping_add(2_usize) > (*len.borrow()))
            || (((elem!((*data.borrow()), (*pos.borrow())).read()) as i32) != 255)
        {
            write!(
                libcc2rs::cerr(),
                "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
                (if ((*pos.borrow()) < (*len.borrow())) {
                    ((elem!((*data.borrow()), (*pos.borrow())).read()) as i32)
                } else {
                    0
                }),
                (*pos.borrow()),
                (*len.borrow()),
            );
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_MARKER_BYTE_NOT_FOUND);
            return false;
        };
        (*marker.borrow_mut()) =
            ((elem!((*data.borrow()), (*pos.borrow()).wrapping_add(1_usize)).read()) as i32);
        (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add(2_usize) };
        let ok: Value<bool> = Rc::new(RefCell::new(true));
        'switch: {
            match { (*marker.borrow()) } {
                __v if __v == 192 || __v == 193 || __v == 194 => {
                    (*is_progressive.borrow_mut()) = ((*marker.borrow()) == 194);
                    (*ok.borrow_mut()) = ({
                        ProcessSOF_235(
                            (*data.borrow()).clone(),
                            (*len.borrow()),
                            (*mode.borrow()),
                            (pos.as_pointer()),
                            (*jpg.borrow()).clone(),
                        )
                    });
                    (*found_sof.borrow_mut()) = true;
                    break 'switch;
                }
                __v if __v == 196 => {
                    (*ok.borrow_mut()) = ({
                        ProcessDHT_237(
                            (*data.borrow()).clone(),
                            (*len.borrow()),
                            (*mode.borrow()),
                            (dc_huff_lut.as_pointer()),
                            (ac_huff_lut.as_pointer()),
                            (pos.as_pointer()),
                            (*jpg.borrow()).clone(),
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
                    if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
                        (*ok.borrow_mut()) = ({
                            ProcessScan_248(
                                (*data.borrow()).clone(),
                                (*len.borrow()),
                                dc_huff_lut.as_pointer(),
                                ac_huff_lut.as_pointer(),
                                (scan_progression.as_pointer() as Ptr<Value<Box<[u16]>>>),
                                (*is_progressive.borrow()),
                                (pos.as_pointer()),
                                (*jpg.borrow()).clone(),
                            )
                        });
                    }
                    break 'switch;
                }
                __v if __v == 219 => {
                    (*ok.borrow_mut()) = ({
                        ProcessDQT_239(
                            (*data.borrow()).clone(),
                            (*len.borrow()),
                            (pos.as_pointer()),
                            (*jpg.borrow()).clone(),
                        )
                    });
                    break 'switch;
                }
                __v if __v == 221 => {
                    (*ok.borrow_mut()) = ({
                        ProcessDRI_240(
                            (*data.borrow()).clone(),
                            (*len.borrow()),
                            (pos.as_pointer()),
                            (found_dri.as_pointer()),
                            (*jpg.borrow()).clone(),
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
                    if (((*mode.borrow()) as i32) != (brunsli_JpegReadMode_JPEG_READ_TABLES as i32))
                    {
                        (*ok.borrow_mut()) = ({
                            ProcessAPP_241(
                                (*data.borrow()).clone(),
                                (*len.borrow()),
                                (pos.as_pointer()),
                                (*jpg.borrow()).clone(),
                            )
                        });
                    }
                    break 'switch;
                }
                __v if __v == 254 => {
                    if (((*mode.borrow()) as i32) != (brunsli_JpegReadMode_JPEG_READ_TABLES as i32))
                    {
                        (*ok.borrow_mut()) = ({
                            ProcessCOM_242(
                                (*data.borrow()).clone(),
                                (*len.borrow()),
                                (pos.as_pointer()),
                                (*jpg.borrow()).clone(),
                            )
                        });
                    }
                    break 'switch;
                }
                _ => {
                    write!(
                        libcc2rs::cerr(),
                        "Unsupported marker: {:} pos={:} len={:}\n",
                        (*marker.borrow()),
                        (*pos.borrow()),
                        (*len.borrow()),
                    );
                    field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_UNSUPPORTED_MARKER);
                    (*ok.borrow_mut()) = false;
                    break 'switch;
                }
            }
        };
        if !(*ok.borrow()) {
            return false;
        }
        {
            let __a1 = ((*marker.borrow()) as u8);
            (*(*jpg.borrow())
                .with(|__s| __s.marker_order.clone())
                .borrow_mut())
            .push(__a1)
        };
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode_JPEG_READ_HEADER as i32))
            && (*found_sof.borrow())
        {
            break;
        }
    }
    if !(*found_sof.borrow()) {
        write!(libcc2rs::cerr(), "Missing SOF marker.\n",);
        field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_SOF_NOT_FOUND);
        return false;
    }
    if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode_JPEG_READ_ALL as i32)) {
        if ((*pos.borrow()) < (*len.borrow())) {
            ((*jpg.borrow()).with(|__s| __s.tail_data.as_pointer()) as Ptr<Vec<u8>>).write({
                let __count = (*data.borrow())
                    .offset((*len.borrow()) as isize)
                    .get_offset()
                    - (*data.borrow())
                        .offset((*pos.borrow()) as isize)
                        .get_offset();
                PtrValueIter::new(&(*data.borrow()).offset((*pos.borrow()) as isize), __count)
                    .collect::<Vec<_>>()
            });
        }
        if !({ FixupIndexes_249((*jpg.borrow()).clone()) }) {
            return false;
        }
        if (*(*jpg.borrow())
            .with(|__s| __s.huffman_code.clone())
            .borrow())
        .is_empty()
        {
            write!(libcc2rs::cerr(), "Need at least one Huffman code table.\n",);
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_ERROR);
            return false;
        }
        if ({
            (*(*jpg.borrow())
                .with(|__s| __s.huffman_code.clone())
                .borrow())
            .len()
        } >= { (kMaxDHTMarkers_10.with(|rc| *rc.borrow()) as usize) })
        {
            write!(libcc2rs::cerr(), "Too many Huffman tables.\n",);
            field!((*jpg.borrow()), error).write(brunsli_JPEGReadError_HUFFMAN_TABLE_ERROR);
            return false;
        }
    }
    return true;
}
pub fn NextTableBitSize_252(count: Ptr<i32>, len: i32) -> i32 {
    let count: Value<Ptr<i32>> = Rc::new(RefCell::new(count));
    let len: Value<i32> = Rc::new(RefCell::new(len));
    let left: Value<i32> = Rc::new(RefCell::new(
        (1 << ((*len.borrow()) - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()))),
    ));
    'loop_: while ((*len.borrow()) < kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        (*left.borrow_mut()) -= { (elem!((*count.borrow()), (*len.borrow())).read()) };
        if ((*left.borrow()) <= 0) {
            break;
        }
        (*len.borrow_mut()).prefix_inc();
        (*left.borrow_mut()) <<= 1;
    }
    return ((*len.borrow()) - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()));
}
pub fn BuildJpegHuffmanTable_238(
    count: Ptr<i32>,
    symbols: Ptr<i32>,
    lut: Ptr<brunsli_HuffmanTableEntry>,
) {
    let count: Value<Ptr<i32>> = Rc::new(RefCell::new(count));
    let symbols: Value<Ptr<i32>> = Rc::new(RefCell::new(symbols));
    let lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(lut));
    let code: Value<brunsli_HuffmanTableEntry> =
        Rc::new(RefCell::new(brunsli_HuffmanTableEntry::new()));
    let table: Value<Ptr<brunsli_HuffmanTableEntry>> =
        Rc::new(RefCell::new(Ptr::<brunsli_HuffmanTableEntry>::null()));
    let len: Value<i32> = Rc::new(RefCell::new(0_i32));
    let idx: Value<i32> = Rc::new(RefCell::new(0_i32));
    let key: Value<i32> = Rc::new(RefCell::new(0_i32));
    let reps: Value<i32> = Rc::new(RefCell::new(0_i32));
    let low: Value<i32> = Rc::new(RefCell::new(0_i32));
    let table_bits: Value<i32> = Rc::new(RefCell::new(0_i32));
    let table_size: Value<i32> = Rc::new(RefCell::new(0_i32));
    let tmp_count: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    let total_count: Value<i32> = Rc::new(RefCell::new(0));
    (*len.borrow_mut()) = 1;
    'loop_: while ((*len.borrow()) <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        (*tmp_count.borrow_mut())[(*len.borrow()) as usize] =
            { (elem!((*count.borrow()), (*len.borrow())).read()) };
        (*total_count.borrow_mut()) += (*tmp_count.borrow())[(*len.borrow()) as usize];
        (*len.borrow_mut()).prefix_inc();
    }
    (*table.borrow_mut()) = (*lut.borrow()).clone();
    (*table_bits.borrow_mut()) = kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow());
    (*table_size.borrow_mut()) = (1 << (*table_bits.borrow()));
    if ((*total_count.borrow()) == 1) {
        (*code.borrow_mut()).bits = 0_u8;
        (*code.borrow_mut()).value = ((elem!((*symbols.borrow()), 0).read()) as u16);
        (*key.borrow_mut()) = 0;
        'loop_: while ((*key.borrow()) < (*table_size.borrow())) {
            elem!((*table.borrow()), (*key.borrow())).write({ (*code.borrow()).clone() });
            (*key.borrow_mut()).prefix_inc();
        }
        return;
    }
    (*key.borrow_mut()) = 0;
    (*idx.borrow_mut()) = 0;
    (*len.borrow_mut()) = 1;
    'loop_: while ((*len.borrow()) <= kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())) {
        'loop_: while ((*tmp_count.borrow())[(*len.borrow()) as usize] > 0) {
            (*code.borrow_mut()).bits = ((*len.borrow()) as u8);
            (*code.borrow_mut()).value =
                ((elem!((*symbols.borrow()), (*idx.borrow_mut()).postfix_inc()).read()) as u16);
            (*reps.borrow_mut()) =
                (1 << (kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()) - (*len.borrow())));
            'loop_: while ((*reps.borrow_mut()).postfix_dec() != 0) {
                let __rhs = (*code.borrow()).clone();
                elem!((*table.borrow()), (*key.borrow_mut()).postfix_inc()).write(__rhs);
            }
            (*tmp_count.borrow_mut())[(*len.borrow()) as usize].prefix_dec();
        }
        (*len.borrow_mut()).prefix_inc();
    }
    (*table.borrow_mut()) += (*table_size.borrow());
    (*table_size.borrow_mut()) = 0;
    (*low.borrow_mut()) = 0;
    (*len.borrow_mut()) = (kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()) + 1);
    'loop_: while ((*len.borrow()) <= kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow())) {
        'loop_: while ((*tmp_count.borrow())[(*len.borrow()) as usize] > 0) {
            if ((*low.borrow()) >= (*table_size.borrow())) {
                (*table.borrow_mut()) += (*table_size.borrow());
                (*table_bits.borrow_mut()) = ({
                    NextTableBitSize_252((tmp_count.as_pointer() as Ptr<i32>), (*len.borrow()))
                });
                (*table_size.borrow_mut()) = (1 << (*table_bits.borrow()));
                (*low.borrow_mut()) = 0;
                field!(elem!((*lut.borrow()), (*key.borrow())), bits).write(
                    (((*table_bits.borrow())
                        + kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow()))
                        as u8),
                );
                field!(elem!((*lut.borrow()), (*key.borrow())), value).write({
                    (({ (((*table.borrow()).clone() - (*lut.borrow()).clone()) as i64) } - {
                        ((*key.borrow()) as i64)
                    }) as u16)
                });
                (*key.borrow_mut()).prefix_inc();
            }
            (*code.borrow_mut()).bits =
                (((*len.borrow()) - kJpegHuffmanRootTableBits_230.with(|rc| *rc.borrow())) as u8);
            (*code.borrow_mut()).value =
                ((elem!((*symbols.borrow()), (*idx.borrow_mut()).postfix_inc()).read()) as u16);
            (*reps.borrow_mut()) =
                (1 << ((*table_bits.borrow()) - ({ (*code.borrow()).bits } as i32)));
            'loop_: while ((*reps.borrow_mut()).postfix_dec() != 0) {
                let __rhs = (*code.borrow()).clone();
                elem!((*table.borrow()), (*low.borrow_mut()).postfix_inc()).write(__rhs);
            }
            (*tmp_count.borrow_mut())[(*len.borrow()) as usize].prefix_dec();
        }
        (*len.borrow_mut()).prefix_inc();
    }
}
impl brunsli_Storage {
    pub fn new(data: Ptr<u8>, length: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let __this: Value<brunsli_Storage> = Rc::new(RefCell::new(Self {
            data: (*data.borrow()).clone(),
            length: (*length.borrow()),
            pos: 0_usize,
        }));
        let this: Ptr<brunsli_Storage> = __this.as_pointer();
        if !((*length.borrow()) > 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"write_bits.cc"),
                    14,
                    Ptr::<i8>::from_string_literal(b"Storage"),
                )
            });
            'loop_: while true {}
        };
        elem!((*data.borrow()), 0).write(0_u8);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl brunsli_Storage {}
pub fn ReadFileInternal_253(file: Ptr<CFile>, content: Ptr<Vec<i8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(file));
    let content: Value<Ptr<Vec<i8>>> = Rc::new(RefCell::new(content));
    if (match (*file.borrow()).with_mut(|__v: &mut CFile| __v.seek(0_i64, ::libc::SEEK_END)) {
        -1 => -1,
        _ => 0,
    } != 0)
    {
        eprintln!("Failed to seek end of input file.");
        return false;
    }
    let input_size: Value<i32> = Rc::new(RefCell::new(
        ((*file.borrow()).with(|__f| __f.tell()) as i32),
    ));
    if ((*input_size.borrow()) == 0) {
        eprintln!("Input file is empty.");
        return false;
    }
    if (match (*file.borrow()).with_mut(|__v: &mut CFile| __v.seek(0_i64, ::libc::SEEK_SET)) {
        -1 => -1,
        _ => 0,
    } != 0)
    {
        eprintln!("Failed to rewind input file to the beginning.");
        return false;
    }
    {
        (*content.borrow()).with_mut(|__v: &mut Vec<i8>| __v.pop());
        (*content.borrow()).with_mut(|__v: &mut Vec<i8>| {
            __v.resize(((*input_size.borrow()) as usize) as usize, 0)
        });
        (*content.borrow()).with_mut(|__v: &mut Vec<i8>| __v.push(0))
    };
    let read_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*read_pos.borrow()) } < {
        ((*(*content.borrow()).upgrade().deref()).len() - 1)
    }) {
        let bytes_read: Value<usize> = Rc::new(RefCell::new({
            let __a0 = ((if (*read_pos.borrow()) as usize
                >= (*((*content.borrow()).clone() as Ptr<Vec<i8>>)
                    .upgrade()
                    .deref())
                .len()
                .saturating_sub(1)
            {
                panic!("out of bounds access")
            } else {
                ((*content.borrow()).clone() as Ptr<Vec<i8>>)
                    .decay()
                    .offset((*read_pos.borrow()) as isize)
            }) as Ptr<i8>)
                .to_any();
            let __a1 = 1_usize;
            let __a2 = ((((*(*content.borrow()).upgrade().deref()).len() - 1) as u64)
                .wrapping_sub(((*read_pos.borrow()) as u64)) as usize);
            let __a3 = (*file.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        }));
        if ((*bytes_read.borrow()) == 0_usize) {
            eprintln!("Failed to read input file");
            return false;
        }
        (*read_pos.borrow_mut()) = { (*read_pos.borrow()).wrapping_add((*bytes_read.borrow())) };
    }
    return true;
}
pub fn ReadFile_254(file_name: Ptr<Vec<i8>>, content: Ptr<Vec<i8>>) -> bool {
    let content: Value<Ptr<Vec<i8>>> = Rc::new(RefCell::new(content));
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(Ptr::<Vec<i8>>::decay(&(file_name)) as Ptr<i8>).to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    if (*file.borrow()).is_null() {
        eprintln!("Failed to open input file.");
        return false;
    }
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({ ReadFileInternal_253((*file.borrow()).clone(), (*content.borrow()).clone()) }),
    ));
    if ({
        let __r = (*file.borrow()).with(|__f| __f.close());
        (*file.borrow()).delete();
        __r
    } != 0)
    {
        if (*ok.borrow()) {
            eprintln!("Failed to close input file.");
        }
        return false;
    }
    return (*ok.borrow());
}
pub fn WriteFileInternal_255(file: Ptr<CFile>, content: Ptr<Vec<i8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(file));
    let write_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*write_pos.borrow()) } < { ((*content.upgrade().deref()).len() - 1) }) {
        let bytes_written: Value<usize> = Rc::new(RefCell::new({
            let __a0 = (((Ptr::<Vec<i8>>::decay(&(content)) as Ptr<i8>)
                .offset((*write_pos.borrow()))) as Ptr<i8>)
                .to_any();
            let __a1 = 1_usize;
            let __a2 = ((((*content.upgrade().deref()).len() - 1) as u64)
                .wrapping_sub(((*write_pos.borrow()) as u64)) as usize);
            let __a3 = (*file.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        }));
        if ((*bytes_written.borrow()) == 0_usize) {
            eprintln!("Failed to write output.");
            return false;
        }
        (*write_pos.borrow_mut()) =
            { (*write_pos.borrow()).wrapping_add((*bytes_written.borrow())) };
    }
    return true;
}
pub fn WriteFile_256(file_name: Ptr<Vec<i8>>, content: Ptr<Vec<i8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(Ptr::<Vec<i8>>::decay(&(file_name)) as Ptr<i8>).to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    if (*file.borrow()).is_null() {
        eprintln!("Failed to open file for writing.");
        return false;
    }
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file: Ptr<CFile> = (*file.borrow()).clone();
            let _content: Ptr<Vec<i8>> = (content).clone();
            WriteFileInternal_255(_file, _content)
        }),
    ));
    if ({
        let __r = (*file.borrow()).with(|__f| __f.close());
        (*file.borrow()).delete();
        __r
    } != 0)
    {
        if (*ok.borrow()) {
            eprintln!("Failed to close output file.");
        }
        return false;
    }
    return (*ok.borrow());
}
pub fn ProcessFile_257(file_name: Ptr<Vec<i8>>, outfile_name: Ptr<Vec<i8>>) -> bool {
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file_name: Ptr<Vec<i8>> = (file_name).clone();
            let _content: Ptr<Vec<i8>> = (input.as_pointer());
            ReadFile_254(_file_name, _content)
        }),
    ));
    if !(*ok.borrow()) {
        return false;
    }
    let output: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    {
        let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
        let input_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
            (input.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>(),
        ));
        (*ok.borrow_mut()) = ({
            ReadJpeg_196(
                (*input_data.borrow()).clone(),
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
        if !(*ok.borrow()) {
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
        let output_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
            ((output.as_pointer() as Ptr<i8>).offset(0_usize)).reinterpret_cast::<u8>(),
        ));
        (*ok.borrow_mut()) = ({
            BrunsliEncodeJpeg_191(
                jpg.as_pointer(),
                (*output_data.borrow()).clone(),
                (output_size.as_pointer()),
            )
        });
        if !(*ok.borrow()) {
            eprintln!("Failed to transform JPEG to Brunsli");
            return false;
        }
        {
            (*output.borrow_mut()).pop();
            (*output.borrow_mut()).resize((*output_size.borrow()) as usize, 0);
            (*output.borrow_mut()).push(0)
        };
    }
    (*ok.borrow_mut()) = ({
        let _file_name: Ptr<Vec<i8>> = (outfile_name).clone();
        let _content: Ptr<Vec<i8>> = output.as_pointer();
        WriteFile_256(_file_name, _content)
    });
    return (*ok.borrow());
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
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<i8>>> = Rc::new(RefCell::new(argv));
    if ((*argc.borrow()) != 2) && ((*argc.borrow()) != 3) {
        eprintln!("Usage: cbrunsli FILE [OUTPUT_FILE, default=FILE.brn]");
        return 1;
    }
    let file_name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = (elem!((*argv.borrow()), 1).read()).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    if (*file_name.borrow()).len() <= 1 {
        eprintln!("Empty input file name.");
        return 1;
    }
    let outfile_name: Value<Vec<i8>> = Rc::new(RefCell::new(if ((*argc.borrow()) == 2) {
        {
            let mut r = (*file_name.borrow()).clone();
            r.pop();
            Ptr::<i8>::from_string_literal(b".brn").with_c_str(|__s| r.extend_from_slice(__s));
            r.push(0);
            r
        }
    } else {
        {
            let mut __bytes = (elem!((*argv.borrow()), 2).read()).to_c_bytes();
            __bytes.push(0);
            __bytes
        }
    }));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({ ProcessFile_257(file_name.as_pointer(), outfile_name.as_pointer()) }),
    ));
    return if (*ok.borrow()) { 0 } else { 1 };
}
pub trait brunsli_ANSCoderImpl {
    fn PutSymbol(&self, t: brunsli_ANSEncSymbolInfo, nbits: Ptr<u8>) -> u32;
    fn GetState(&self) -> u32;
}
impl brunsli_ANSCoderImpl for Ptr<brunsli_ANSCoder> {
    fn PutSymbol(&self, t: brunsli_ANSEncSymbolInfo, nbits: Ptr<u8>) -> u32 {
        let t: Value<brunsli_ANSEncSymbolInfo> = Rc::new(RefCell::new(t));
        let nbits: Value<Ptr<u8>> = Rc::new(RefCell::new(nbits));
        let bits: Value<u32> = Rc::new(RefCell::new(0_u32));
        (*nbits.borrow()).write(0_u8);
        if (((*self).with(|__s| __s.state_)
            >> (32 - BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())))
            >= ({ (*t.borrow()).freq_ } as u32))
        {
            (*bits.borrow_mut()) = ((*self).with(|__s| __s.state_) & 65535_u32);
            {
                let _ptr = field!((*self), state_);
                _ptr.write(_ptr.read() >> 16)
            };
            (*nbits.borrow()).write(16_u8);
        }
        field!((*self), state_).write({
            (((((*self).with(|__s| __s.state_)).wrapping_div(({ (*t.borrow()).freq_ } as u32)))
                << BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow()))
            .wrapping_add(
                (((*self).with(|__s| __s.state_)).wrapping_rem(({ (*t.borrow()).freq_ } as u32))),
            ))
            .wrapping_add(({ (*t.borrow()).start_ } as u32))
        });
        return (*bits.borrow());
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
    fn Reset(&self, pos: usize) {
        let pos: Value<usize> = Rc::new(RefCell::new(pos));
        field!((*self), pos_).write((*pos.borrow()));
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
        let c: Value<u8> = Rc::new(RefCell::new(
            (elem!(
                (*self).with(|__s| __s.data_.clone()),
                field!((*self), pos_).with_mut(|__v| __v.postfix_inc())
            )
            .read()),
        ));
        if (((*c.borrow()) as i32) == 255) {
            let escape: Value<u8> = Rc::new(RefCell::new(
                (elem!(
                    (*self).with(|__s| __s.data_.clone()),
                    (*self).with(|__s| __s.pos_)
                )
                .read()),
            ));
            if (((*escape.borrow()) as i32) == 0) {
                field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            } else {
                field!((*self), next_marker_pos_)
                    .write(((*self).with(|__s| __s.pos_)).wrapping_sub(1_usize));
            }
        }
        return (*c.borrow());
    }
    fn FillBitWindow(&self) {
        if ((*self).with(|__s| __s.bits_left_) <= 16) {
            'loop_: while ((*self).with(|__s| __s.bits_left_) <= 56) {
                {
                    let _ptr = field!((*self), val_);
                    _ptr.write(_ptr.read() << 8)
                };
                {
                    let _ptr = field!((*self), val_);
                    _ptr.write(
                        _ptr.read() | (({ brunsli_BitReaderStateImpl::GetNextByte(self) }) as u64),
                    )
                };
                {
                    let _ptr = field!((*self), bits_left_);
                    _ptr.write(_ptr.read() + 8)
                };
            }
        }
    }
    fn ReadBits(&self, nbits: i32) -> i32 {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        ({ brunsli_BitReaderStateImpl::FillBitWindow(self) });
        let val: Value<u64> = Rc::new(RefCell::new(
            (((*self).with(|__s| __s.val_)
                >> ((*self).with(|__s| __s.bits_left_) - (*nbits.borrow())))
                & ((1_u64 << (*nbits.borrow())).wrapping_sub(1_u64))),
        ));
        {
            let _ptr = field!((*self), bits_left_);
            _ptr.write(_ptr.read() - (*nbits.borrow()))
        };
        if !((*val.borrow()) < ((1_u32 << 31) as u64)) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"jpeg_data_reader.cc"),
                    471,
                    Ptr::<i8>::from_string_literal(b"ReadBits"),
                )
            });
            'loop_: while true {}
        };
        return ((*val.borrow()) as i32);
    }
    fn IsUnhealthy(&self) -> bool {
        return ((*self).with(|__s| __s.pos_)
            > (((*self).with(|__s| __s.next_marker_pos_)).wrapping_add(32_usize)));
    }
    fn FinishStream(&self, jpg: Ptr<brunsli_JPEGData>, pos: Ptr<usize>) -> bool {
        let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
        let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
        let npadbits: Value<i32> = Rc::new(RefCell::new(((*self).with(|__s| __s.bits_left_) & 7)));
        if ((*npadbits.borrow()) > 0) {
            let padmask: Value<u64> = Rc::new(RefCell::new(
                (1_u64 << (*npadbits.borrow())).wrapping_sub(1_u64),
            ));
            let padbits: Value<u64> = Rc::new(RefCell::new(
                (((*self).with(|__s| __s.val_)
                    >> ((*self).with(|__s| __s.bits_left_) - (*npadbits.borrow())))
                    & (*padmask.borrow())),
            ));
            if ((*padbits.borrow()) != (*padmask.borrow())) {
                field!((*jpg.borrow()), has_zero_padding_bit).write(true);
            }
            let i: Value<i32> = Rc::new(RefCell::new(((*npadbits.borrow()) - 1)));
            'loop_: while ((*i.borrow()) >= 0) {
                {
                    let __a1 = ((((*padbits.borrow()) >> (*i.borrow())) & 1_u64) as i32);
                    (*(*jpg.borrow())
                        .with(|__s| __s.padding_bits.clone())
                        .borrow_mut())
                    .push(__a1)
                };
                (*i.borrow_mut()).prefix_dec();
            }
        }
        let unused_bytes_left: Value<i32> =
            Rc::new(RefCell::new(((*self).with(|__s| __s.bits_left_) >> 3)));
        'loop_: while ((*unused_bytes_left.borrow_mut()).postfix_dec() > 0) {
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
        (*pos.borrow()).write({ (*self).with(|__s| __s.pos_) });
        return true;
    }
}
pub trait brunsli_ComponentStateImpl {
    fn SetWidth(&self, w: i32);
    fn InitAll(&self);
    fn destructor(&self);
}
impl brunsli_ComponentStateImpl for Ptr<brunsli_ComponentState> {
    fn SetWidth(&self, w: i32) {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        field!((*self), width).write((*w.borrow()));
        {
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_is_nonempty.clone())
                .borrow_mut())
            .resize(__a0, 1)
        };
        {
            let __a0 = ((*w.borrow()) as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_num_nonzeros.clone())
                .borrow_mut())
            .resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 = (((kDCTBlockSize_3.with(|rc| *rc.borrow()) * 2) * ((*w.borrow()) + 3))
                as usize) as usize;
            (*(*self).with(|__s| __s.prev_abs_coeff.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 =
                ((kDCTBlockSize_3.with(|rc| *rc.borrow()) * ((*w.borrow()) + 1)) as usize) as usize;
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
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (kNumNonzeroBuckets_90.with(|rc| *rc.borrow()) as i32)) {
            let k: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
                let v: Value<i32> = Rc::new(RefCell::new(
                    ((({
                        let __idx = (*k.borrow()) as usize;
                        kInitProb_110.with(|rc| rc.borrow()[__idx])
                    }) as i32)
                        + (9 * ((*i.borrow()) - 7))),
                ));
                if !((*v.borrow()) <= 255) {
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
                            .offset(
                                ((((*i.borrow()) * kDCTBlockSize_3.with(|rc| *rc.borrow()))
                                    + (*k.borrow())) as usize),
                            ),
                        ((*v.borrow()) as u8),
                    )
                });
                (*k.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*(*self).with(|__s| __s.sign_prob.clone()).borrow()).len())
        {
            if ((*i.borrow())
                < (kMaxAverageContext_82.with(|rc| *rc.borrow()))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
            {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset((*i.borrow())),
                        108_u8,
                    )
                });
            } else if ((*i.borrow())
                < (((kMaxAverageContext_82.with(|rc| *rc.borrow())).wrapping_add(1_usize))
                    as usize)
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
            {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset((*i.borrow())),
                        128_u8,
                    )
                });
            } else {
                ({
                    brunsli_ProbImpl::Init(
                        &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                            .offset((*i.borrow())),
                        148_u8,
                    )
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow())
            < (*(*self)
                .with(|__s| __s.first_extra_bit_prob.clone())
                .borrow())
            .len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.first_extra_bit_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset((*i.borrow())),
                    158_u8,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < kNumNonZeroContextCount_88.with(|rc| *rc.borrow())) {
            let non_zero_probs: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                (array_field_ptr!((*self), num_nonzero_prob) as Ptr<brunsli_Prob>).offset(
                    ((*i.borrow()).wrapping_mul(kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())))
                        as isize,
                ),
            ));
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ((*j.borrow()) < kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
                ({
                    let _probability: u8 = ({
                        let __idx = (*i.borrow()) as usize;
                        kInitProbNonzero_111.with(|rc| rc.borrow()[__idx].clone())
                    })
                    .borrow()[(*j.borrow()) as usize];
                    brunsli_ProbImpl::Init(
                        &(*non_zero_probs.borrow()).offset((*j.borrow()) as isize),
                        _probability,
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub trait brunsli_ComponentStateDCImpl {
    fn SetWidth(&self, w: i32);
    fn InitAll(&self);
    fn destructor(&self);
}
impl brunsli_ComponentStateDCImpl for Ptr<brunsli_ComponentStateDC> {
    fn SetWidth(&self, w: i32) {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        field!((*self), width).write((*w.borrow()));
        {
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
            (*(*self)
                .with(|__s| __s.prev_is_nonempty.clone())
                .borrow_mut())
            .resize(__a0, 1)
        };
        {
            let __a0 = (((*w.borrow()) + 3) as usize) as usize;
            (*(*self).with(|__s| __s.prev_abs_coeff.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
            (*(*self).with(|__s| __s.prev_sign.clone()).borrow_mut())
                .resize_with(__a0, || <i32>::default())
        };
    }
    fn destructor(&self) {
        field_ptr!(self, is_zero_prob).destructor();
    }
    fn InitAll(&self) {
        ({ brunsli_ProbImpl::Init(&field_ptr!((*self), is_zero_prob), 135_u8) });
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*(*self).with(|__s| __s.sign_prob.clone()).borrow()).len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.sign_prob.as_pointer()) as Ptr<brunsli_Prob>)
                        .offset((*i.borrow())),
                    128_u8,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow())
            < (*(*self).with(|__s| __s.is_empty_block_prob.clone()).borrow()).len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.is_empty_block_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset((*i.borrow())),
                    74_u8,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow())
            < (*(*self)
                .with(|__s| __s.first_extra_bit_prob.clone())
                .borrow())
            .len())
        {
            ({
                brunsli_ProbImpl::Init(
                    &((*self).with(|__s| __s.first_extra_bit_prob.as_pointer())
                        as Ptr<brunsli_Prob>)
                        .offset((*i.borrow())),
                    150_u8,
                )
            });
            (*i.borrow_mut()).prefix_inc();
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
        let num_values: Value<u32> = Rc::new(RefCell::new(
            ((*(*self).with(|__s| __s.values_.clone()).borrow()).len() as u32),
        ));
        if !((*num_values.borrow()) > 0_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"lehmer_code.cc"),
                    51,
                    Ptr::<i8>::from_string_literal(b"num_bits"),
                )
            });
            'loop_: while true {}
        };
        if ((*num_values.borrow()) <= 1_u32) {
            return 0;
        }
        return (({ Log2FloorNonZero_74((*num_values.borrow()).wrapping_sub(1_u32)) }) + 1);
    }
    fn Remove(&self, code: usize, value: Ptr<u8>) -> bool {
        let code: Value<usize> = Rc::new(RefCell::new(code));
        let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
        if ((*code.borrow()) >= (*(*self).with(|__s| __s.values_.clone()).borrow()).len()) {
            return false;
        }
        let __rhs = (elem!(
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>),
            (*code.borrow())
        )
        .read());
        (*value.borrow()).write(__rhs);
        {
            let idx = ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<u8>)
                .offset(((*code.borrow()) as i64) as isize)
                .get_offset();
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>)
                .with_mut(|__v: &mut Vec<u8>| __v.remove(idx));
            ((*self).with(|__s| __s.values_.as_pointer()) as Ptr<Vec<u8>>).decay()
        };
        return true;
    }
    fn RemoveValue(&self, value: u8, code: Ptr<i32>, nbits: Ptr<i32>) -> bool {
        let value: Value<u8> = Rc::new(RefCell::new(value));
        let code: Value<Ptr<i32>> = Rc::new(RefCell::new(code));
        let nbits: Value<Ptr<i32>> = Rc::new(RefCell::new(nbits));
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
        (*code.borrow()).write(__rhs);
        let __rhs = ({ brunsli_PermutationCoderImpl::num_bits(self) });
        (*nbits.borrow()).write(__rhs);
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
    fn Init(&self, probability: u8) {
        let probability: Value<u8> = Rc::new(RefCell::new(probability));
        field!((*self), prob8).write((*probability.borrow()));
        field!((*self), total).write(kInitProbCount_81.with(|rc| *rc.borrow()));
        field!((*self), count).write(
            (((kInitProbCount_81.with(|rc| *rc.borrow()) as i32) * ((*probability.borrow()) as i32))
                as u16),
        );
    }
    fn Add(&self, val: i32) {
        let val: Value<i32> = Rc::new(RefCell::new(val));
        field!((*self), total).with_mut(|__v| __v.prefix_inc());
        if ((*val.borrow()) == 0) {
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
    fn AppendBytes(&self, src: Ptr<u8>, len: usize) {
        unimplemented!()
    }
}
impl brunsli_StorageImpl for Ptr<brunsli_Storage> {
    fn GetBytesUsed(&self) -> usize {
        return ((((*self).with(|__s| __s.pos)).wrapping_add(7_usize)) >> 3);
    }
    fn AppendBytes(&self, src: Ptr<u8>, len: usize) {
        let src: Value<Ptr<u8>> = Rc::new(RefCell::new(src));
        let len: Value<usize> = Rc::new(RefCell::new(len));
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
        if !(({ brunsli_StorageImpl::GetBytesUsed(self) }).wrapping_add((*len.borrow()))
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
                .memcpy(&(*src.borrow()).to_any(), (*len.borrow()) as usize);
            ((*self)
                .with(|__s| __s.data.clone())
                .offset(((*self).with(|__s| __s.pos) >> 3) as isize) as Ptr<u8>)
                .to_any()
        };
        field!((*self), pos).write({
            ((*self).with(|__s| __s.pos)).wrapping_add((8_usize).wrapping_mul((*len.borrow())))
        });
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
    fn Resize(&self, max_num_code_words: usize) {
        let max_num_code_words: Value<usize> = Rc::new(RefCell::new(max_num_code_words));
        {
            let __a0 = (*max_num_code_words.borrow()) as usize;
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
            let new_size: Value<usize> = Rc::new(RefCell::new(
                ((kGrowMult_165.with(|rc| *rc.borrow())
                    * ((*(*self).with(|__s| __s.code_words_.clone()).borrow()).capacity() as f64))
                    as usize)
                    .wrapping_add(kSlackForOneBlock_140.with(|rc| *rc.borrow())),
            ));
            {
                let __a0 = (*new_size.borrow()) as usize;
                (*(*self).with(|__s| __s.code_words_.clone()).borrow_mut())
                    .resize_with(__a0, || {
                        <brunsli_internal_enc_DataStream_CodeWord>::default()
                    })
            };
        }
    }
    fn AddCode(
        &self,
        code: usize,
        band: usize,
        context: usize,
        s: Ptr<brunsli_internal_enc_EntropySource>,
    ) {
        let code: Value<usize> = Rc::new(RefCell::new(code));
        let band: Value<usize> = Rc::new(RefCell::new(band));
        let context: Value<usize> = Rc::new(RefCell::new(context));
        let s: Value<Ptr<brunsli_internal_enc_EntropySource>> = Rc::new(RefCell::new(s));
        let histo_ix: Value<usize> = Rc::new(RefCell::new(
            ((*band.borrow()).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
                .wrapping_add((*context.borrow())),
        ));
        let word: Value<brunsli_internal_enc_DataStream_CodeWord> =
            Rc::new(RefCell::new(brunsli_internal_enc_DataStream_CodeWord::new()));
        (*word.borrow_mut()).context = ((*histo_ix.borrow()) as u32);
        (*word.borrow_mut()).code = (((*code.borrow()) as u32) as u8);
        (*word.borrow_mut()).nbits = 0_u8;
        (*word.borrow_mut()).value = 0_u16;
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
        .write((*word.borrow()).clone());
        ({
            brunsli_internal_enc_EntropySourceImpl::AddCode(
                &(*s.borrow()),
                (*code.borrow()),
                (*histo_ix.borrow()),
            )
        });
    }
    fn AddBits(&self, nbits: i32, bits: i32) {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        let bits: Value<i32> = Rc::new(RefCell::new(bits));
        {
            let _ptr = field!((*self), bw_val_);
            _ptr.write(
                _ptr.read() | (((*bits.borrow()) << (*self).with(|__s| __s.bw_bitpos_)) as u32),
            )
        };
        {
            let _ptr = field!((*self), bw_bitpos_);
            _ptr.write(_ptr.read() + (*nbits.borrow()))
        };
        if ((*self).with(|__s| __s.bw_bitpos_) > 16) {
            let word: Value<brunsli_internal_enc_DataStream_CodeWord> =
                Rc::new(RefCell::new(brunsli_internal_enc_DataStream_CodeWord::new()));
            (*word.borrow_mut()).context = 0_u32;
            (*word.borrow_mut()).code = 0_u8;
            (*word.borrow_mut()).nbits = 16_u8;
            (*word.borrow_mut()).value = (((*self).with(|__s| __s.bw_val_) & 65535_u32) as u16);
            elem!(
                ((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>),
                ((*self).with(|__s| __s.bw_pos_) as usize)
            )
            .write((*word.borrow()).clone());
            field!((*self), bw_pos_).write((*self).with(|__s| __s.pos_));
            field!((*self), pos_).with_mut(|__v| __v.prefix_inc());
            {
                let _ptr = field!((*self), bw_val_);
                _ptr.write(_ptr.read() >> 16)
            };
            {
                let _ptr = field!((*self), bw_bitpos_);
                _ptr.write(_ptr.read() - 16)
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
    fn AddBit(&self, p: Ptr<brunsli_Prob>, bit: i32) {
        let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
        let bit: Value<i32> = Rc::new(RefCell::new(bit));
        let prob: Value<u8> = Rc::new(RefCell::new(
            ({ brunsli_ProbImpl::get_proba(&(*p.borrow())) }),
        ));
        ({ brunsli_ProbImpl::Add(&(*p.borrow()), (*bit.borrow())) });
        let diff: Value<u32> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.high_)).wrapping_sub((*self).with(|__s| __s.low_)),
        ));
        let split: Value<u32> = Rc::new(RefCell::new(
            ((((*self).with(|__s| __s.low_) as u64).wrapping_add(
                ((((*diff.borrow()) as u64).wrapping_mul(((*prob.borrow()) as u64))) >> 8),
            )) as u32),
        ));
        if ((*bit.borrow()) != 0) {
            field!((*self), low_).write((*split.borrow()).wrapping_add(1_u32));
        } else {
            field!((*self), high_).write((*split.borrow()));
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
                let _ptr = field!((*self), low_);
                _ptr.write(_ptr.read() << 16)
            };
            {
                let _ptr = field!((*self), high_);
                _ptr.write(_ptr.read() << 16)
            };
            {
                let _ptr = field!((*self), high_);
                _ptr.write(_ptr.read() | 65535_u32)
            };
        }
    }
    fn EncodeCodeWords(
        &self,
        s: Ptr<brunsli_internal_enc_EntropyCodes>,
        storage: Ptr<brunsli_Storage>,
    ) {
        let s: Value<Ptr<brunsli_internal_enc_EntropyCodes>> = Rc::new(RefCell::new(s));
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        ({ brunsli_internal_enc_DataStreamImpl::FlushBitWriter(self) });
        ({ brunsli_internal_enc_DataStreamImpl::FlushArithmeticCoder(self) });
        let ans: Value<brunsli_ANSCoder> = Rc::new(RefCell::new(brunsli_ANSCoder::new()));
        let i: Value<i32> = Rc::new(RefCell::new(((*self).with(|__s| __s.pos_) - 1)));
        'loop_: while ((*i.borrow()) >= 0) {
            let word: Value<Ptr<brunsli_internal_enc_DataStream_CodeWord>> = Rc::new(RefCell::new(
                (((*self).with(|__s| __s.code_words_.as_pointer())
                    as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                    .offset(((*i.borrow()) as usize))),
            ));
            if (((*word.borrow()).with(|__s| __s.nbits) as i32) == 0) {
                let info: Value<brunsli_ANSEncSymbolInfo> = Rc::new(RefCell::new(
                    (*elem!(
                        (array_field_ptr!(
                            ({
                                brunsli_internal_enc_EntropyCodesImpl::GetANSTable(
                                    &(*s.borrow()),
                                    ((*word.borrow()).with(|__s| __s.context) as i32),
                                )
                            }),
                            info_
                        ) as Ptr<brunsli_ANSEncSymbolInfo>),
                        (*word.borrow()).with(|__s| __s.code)
                    )
                    .upgrade()
                    .deref())
                    .clone(),
                ));
                let __rhs = (({
                    brunsli_ANSCoderImpl::PutSymbol(
                        &ans.as_pointer(),
                        (*info.borrow()).clone(),
                        (field_ptr!((*word.borrow()), nbits)),
                    )
                }) as u16);
                field!((*word.borrow()), value).write(__rhs);
            }
            (*i.borrow_mut()).prefix_dec();
        }
        let state: Value<u32> = Rc::new(RefCell::new(
            ({ brunsli_ANSCoderImpl::GetState(&ans.as_pointer()) }),
        ));
        let out: Value<Ptr<u16>> = Rc::new(RefCell::new(
            (*storage.borrow())
                .with(|__s| __s.data.clone())
                .reinterpret_cast::<u16>(),
        ));
        let out_start: Value<Ptr<u16>> = Rc::new(RefCell::new((*out.borrow()).clone()));
        ({
            let _p: AnyPtr = ((*out.borrow_mut()).postfix_inc() as Ptr<u16>).to_any();
            let _v: u16 = (((*state.borrow()) >> 16) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        ({
            let _p: AnyPtr = ((*out.borrow_mut()).postfix_inc() as Ptr<u16>).to_any();
            let _v: u16 = ((*state.borrow()) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*self).with(|__s| __s.pos_)) {
            let word: Ptr<brunsli_internal_enc_DataStream_CodeWord> = ((*self)
                .with(|__s| __s.code_words_.as_pointer())
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset(((*i.borrow()) as usize));
            if (word.with(|__s| __s.nbits) != 0) {
                ({
                    let _p: AnyPtr = ((*out.borrow_mut()).postfix_inc() as Ptr<u16>).to_any();
                    let _v: u16 = word.with(|__s| __s.value);
                    BrunsliUnalignedWrite16_67(_p, _v)
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        field!((*storage.borrow()), pos).write({
            ((*storage.borrow()).with(|__s| __s.pos)).wrapping_add(
                (((((*out.borrow()).clone() - (*out_start.borrow()).clone()) as i64) * 16_i64)
                    as usize),
            )
        });
    }
}
pub trait brunsli_internal_enc_EntropyCodesImpl {
    fn EncodeContextMap(&self, storage: Ptr<brunsli_Storage>);
    fn BuildAndStoreEntropyCodes(&self, storage: Ptr<brunsli_Storage>);
    fn GetANSTable(&self, context: i32) -> Ptr<brunsli_ANSTable>;
}
impl brunsli_internal_enc_EntropyCodesImpl for Ptr<brunsli_internal_enc_EntropyCodes> {
    fn EncodeContextMap(&self, storage: Ptr<brunsli_Storage>) {
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        ({
            let _context_map: Ptr<Vec<u32>> = (*self).with(|__s| __s.context_map_.as_pointer());
            let _num_clusters: usize = (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len();
            EncodeContextMap_164(_context_map, _num_clusters, (*storage.borrow()).clone())
        });
    }
    fn BuildAndStoreEntropyCodes(&self, storage: Ptr<brunsli_Storage>) {
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        {
            let __a0 = (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len() as usize;
            (*(*self).with(|__s| __s.ans_tables_.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_ANSTable>::default())
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*(*self).with(|__s| __s.clustered_.clone()).borrow()).len())
        {
            ({
                let _histogram: Ptr<i32> = ((array_field_ptr!(
                    ((*self).with(|__s| __s.clustered_.as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset((*i.borrow())),
                    data_
                ) as Ptr<i32>)
                    .offset((0) as isize));
                let _table: Ptr<brunsli_ANSTable> =
                    (((*self).with(|__s| __s.ans_tables_.as_pointer()) as Ptr<brunsli_ANSTable>)
                        .offset((*i.borrow())));
                BuildAndStoreANSEncodingData_123(_histogram, _table, (*storage.borrow()).clone())
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    fn GetANSTable(&self, context: i32) -> Ptr<brunsli_ANSTable> {
        let context: Value<i32> = Rc::new(RefCell::new(context));
        let entropy_ix: Value<i32> = Rc::new(RefCell::new(
            ((elem!(
                ((*self).with(|__s| __s.context_map_.as_pointer()) as Ptr<u32>),
                ((*context.borrow()) as usize)
            )
            .read()) as i32),
        ));
        return (((*self).with(|__s| __s.ans_tables_.as_pointer()) as Ptr<brunsli_ANSTable>)
            .offset(((*entropy_ix.borrow()) as usize)));
    }
}
pub trait brunsli_internal_enc_EntropySourceImpl {
    fn Resize(&self, num_bands: usize);
    fn AddCode(&self, code: usize, histo_ix: usize);
    fn Merge(&self, other: Ptr<brunsli_internal_enc_EntropySource>);
    fn Finish(&self, offsets: Ptr<Vec<u64>>) -> Option<Value<brunsli_internal_enc_EntropyCodes>>;
}
impl brunsli_internal_enc_EntropySourceImpl for Ptr<brunsli_internal_enc_EntropySource> {
    fn Resize(&self, num_bands: usize) {
        let num_bands: Value<usize> = Rc::new(RefCell::new(num_bands));
        field!((*self), num_bands_).write((*num_bands.borrow()));
        {
            let __a0 = (*num_bands.borrow())
                .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow()))
                as usize;
            (*(*self).with(|__s| __s.histograms_.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        };
    }
    fn AddCode(&self, code: usize, histo_ix: usize) {
        let code: Value<usize> = Rc::new(RefCell::new(code));
        let histo_ix: Value<usize> = Rc::new(RefCell::new(histo_ix));
        ({
            brunsli_internal_enc_HistogramImpl::Add(
                &((*self).with(|__s| __s.histograms_.as_pointer())
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset((*histo_ix.borrow())),
                (*code.borrow()),
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
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*i.borrow()) } < {
            (*other.with(|__s| __s.histograms_.clone()).borrow()).len()
        }) {
            ({
                let _other: Ptr<brunsli_internal_enc_Histogram> = (other
                    .with(|__s| __s.histograms_.as_pointer())
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset((*i.borrow()));
                brunsli_internal_enc_HistogramImpl::Merge(
                    &((*self).with(|__s| __s.histograms_.as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset((*i.borrow())),
                    _other,
                )
            });
            (*i.borrow_mut()).prefix_inc();
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
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            {
                let _ptr = elem!(
                    (array_field_ptr!((*self), data_) as Ptr::<i32>),
                    (*i.borrow())
                );
                _ptr.write(
                    _ptr.read() + {
                        (elem!(
                            (array_field_ptr!(other, data_) as Ptr::<i32>),
                            (*i.borrow())
                        )
                        .read())
                    },
                )
            };
            (*i.borrow_mut()).prefix_inc();
        }
        {
            let _ptr = field!((*self), total_count_);
            _ptr.write(_ptr.read() + { other.with(|__s| __s.total_count_) })
        };
    }
    fn Add(&self, val: usize) {
        let val: Value<usize> = Rc::new(RefCell::new(val));
        if !((*val.borrow()) < 18_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<i8>::from_string_literal(b"brunsli_encode.cc"),
                    522,
                    Ptr::<i8>::from_string_literal(b"Add"),
                )
            });
            'loop_: while true {}
        };
        elem!(
            (array_field_ptr!((*self), data_) as Ptr::<i32>),
            (*val.borrow())
        )
        .with_mut(|__v| __v.prefix_inc());
        field!((*self), total_count_).with_mut(|__v| __v.prefix_inc());
    }
    fn Merge(&self, other: Ptr<brunsli_internal_enc_Histogram>) {
        if (other.with(|__s| __s.total_count_) == 0) {
            return;
        }
        {
            let _ptr = field!((*self), total_count_);
            _ptr.write(_ptr.read() + { other.with(|__s| __s.total_count_) })
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < 18_usize) {
            {
                let _ptr = elem!(
                    (array_field_ptr!((*self), data_) as Ptr::<i32>),
                    (*i.borrow())
                );
                _ptr.write(
                    _ptr.read() + {
                        (elem!(
                            (array_field_ptr!(other, data_) as Ptr::<i32>),
                            (*i.borrow())
                        )
                        .read())
                    },
                )
            };
            (*i.borrow_mut()).prefix_inc();
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
