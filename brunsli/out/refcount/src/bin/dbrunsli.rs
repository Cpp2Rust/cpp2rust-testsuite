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
        ('B' as u8),
        210_u8,
        213_u8,
        ('N' as u8),
    ])));
);
thread_local!(
    pub static AppData_0xe0_62: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        224_u8,
        0_u8,
        16_u8,
        ('J' as u8),
        ('F' as u8),
        ('I' as u8),
        ('F' as u8),
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
        ('D' as u8),
        ('u' as u8),
        ('c' as u8),
        ('k' as u8),
        ('y' as u8),
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
        ('A' as u8),
        ('d' as u8),
        ('o' as u8),
        ('b' as u8),
        ('e' as u8),
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
                Ptr::<u8>::from_string_literal(b"context.cc"),
                55,
                Ptr::<u8>::from_string_literal(b"FastDivide"),
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
                Ptr::<u8>::from_string_literal(b"context.cc"),
                305,
                Ptr::<u8>::from_string_literal(b"NumNonzerosContext"),
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
                    Ptr::<u8>::from_string_literal(b"lehmer_code.cc"),
                    21,
                    Ptr::<u8>::from_string_literal(b"ComputeLehmerCode"),
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
pub fn BrunsliDumpAndAbort_79(f: Ptr<u8>, l: i32, fn_: Ptr<u8>) {
    let f: Value<Ptr<u8>> = Rc::new(RefCell::new(f));
    let l: Value<i32> = Rc::new(RefCell::new(l));
    let fn_: Value<Ptr<u8>> = Rc::new(RefCell::new(fn_));
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
                Ptr::<u8>::from_string_literal(b"quant_matrix.cc"),
                18,
                Ptr::<u8>::from_string_literal(b"FillQuantMatrix"),
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
    pub fn new(data: Ptr<u8>, len: usize, optimistic: bool) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        let optimistic: Value<bool> = Rc::new(RefCell::new(optimistic));
        let __this: Value<brunsli_WordSource> = Rc::new(RefCell::new(Self {
            data_: (*data.borrow()).clone(),
            len_: ((*len.borrow()) & (!1 as usize)),
            pos_: 0_usize,
            error_: false,
            optimistic_: (*optimistic.borrow()),
        }));
        let this: Ptr<brunsli_WordSource> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_BitSource> = Rc::new(RefCell::new(Self {
            val_: 0_u32,
            bit_pos_: 0_i32,
        }));
        let this: Ptr<brunsli_BitSource> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_ANSDecodingData> = Rc::new(RefCell::new(Self {
            map_: Rc::new(RefCell::new(
                (0..1024)
                    .map(|_| <brunsli_ANSSymbolInfo>::default())
                    .collect::<Box<[brunsli_ANSSymbolInfo]>>(),
            )),
        }));
        let this: Ptr<brunsli_ANSDecodingData> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_ANSDecoder> = Rc::new(RefCell::new(Self { state_: 0_u32 }));
        let this: Ptr<brunsli_ANSDecoder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
pub fn BrunsliBitReaderBitMask_121(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return !((4294967295_u32) << (*n.borrow()));
}
pub fn BrunsliBitReaderOweByte_122(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    field!((*br.borrow()), num_bits_)
        .write({ ((*br.borrow()).with(|__s| __s.num_bits_)).wrapping_add(8_u32) });
    field!((*br.borrow()), num_debt_bytes_).with_mut(|__v| __v.postfix_inc());
}
pub fn BrunsliBitReaderMaybeFetchByte_123(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if ({ (*br.borrow()).with(|__s| __s.num_bits_) } < { (*n_bits.borrow()) }) {
        if ((({ (*br.borrow()).with(|__s| __s.next_.clone()) } >= {
            (*br.borrow()).with(|__s| __s.end_.clone())
        }) as i64)
            != 0)
        {
            ({ BrunsliBitReaderOweByte_122((*br.borrow()).clone()) });
        } else {
            {
                let _ptr = field!((*br.borrow()), bits_);
                _ptr.write(
                    _ptr.read() | {
                        ({ (((*br.borrow()).with(|__s| __s.next_.clone()).read()) as u32) } << {
                            (*br.borrow()).with(|__s| __s.num_bits_)
                        })
                    },
                )
            };
            field!((*br.borrow()), num_bits_)
                .write({ ((*br.borrow()).with(|__s| __s.num_bits_)).wrapping_add(8_u32) });
            field!((*br.borrow()), next_).with_mut(|__v| __v.postfix_inc());
        }
    }
}
pub fn BrunsliBitReaderGet_124(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) -> u32 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if !((*n_bits.borrow()) <= 24_u32) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"bit_reader.cc"),
                110,
                Ptr::<u8>::from_string_literal(b"BrunsliBitReaderGet"),
            )
        });
        'loop_: while true {}
    };
    ({ BrunsliBitReaderMaybeFetchByte_123((*br.borrow()).clone(), (*n_bits.borrow())) });
    if ((*n_bits.borrow()) > 8_u32) {
        ({ BrunsliBitReaderMaybeFetchByte_123((*br.borrow()).clone(), (*n_bits.borrow())) });
        if ((*n_bits.borrow()) > 16_u32) {
            ({ BrunsliBitReaderMaybeFetchByte_123((*br.borrow()).clone(), (*n_bits.borrow())) });
        }
    }
    return ({ (*br.borrow()).with(|__s| __s.bits_) } & {
        ({ BrunsliBitReaderBitMask_121((*n_bits.borrow())) })
    });
}
pub fn BrunsliBitReaderDrop_125(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if !({ (*n_bits.borrow()) } <= { (*br.borrow()).with(|__s| __s.num_bits_) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"bit_reader.cc"),
                121,
                Ptr::<u8>::from_string_literal(b"BrunsliBitReaderDrop"),
            )
        });
        'loop_: while true {}
    };
    {
        let _ptr = field!((*br.borrow()), bits_);
        _ptr.write(_ptr.read() >> (*n_bits.borrow()))
    };
    field!((*br.borrow()), num_bits_)
        .write({ ((*br.borrow()).with(|__s| __s.num_bits_)).wrapping_sub((*n_bits.borrow())) });
}
pub fn BrunsliBitReaderRead_126(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) -> u32 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ BrunsliBitReaderGet_124((*br.borrow()).clone(), (*n_bits.borrow())) }),
    ));
    ({ BrunsliBitReaderDrop_125((*br.borrow()).clone(), (*n_bits.borrow())) });
    return (*result.borrow());
}
pub fn BrunsliBitReaderInit_127(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    field!((*br.borrow()), num_bits_).write(0_u32);
    field!((*br.borrow()), bits_).write(0_u32);
    field!((*br.borrow()), num_debt_bytes_).write(0_u32);
    field!((*br.borrow()), is_healthy_).write(true);
    field!((*br.borrow()), is_optimistic_).write(false);
}
pub fn BrunsliBitReaderResume_128(
    br: Ptr<brunsli_BrunsliBitReader>,
    buffer: Ptr<u8>,
    length: usize,
) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let buffer: Value<Ptr<u8>> = Rc::new(RefCell::new(buffer));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    field!((*br.borrow()), next_).write((*buffer.borrow()).clone());
    field!((*br.borrow()), end_).write((*buffer.borrow()).offset((*length.borrow()) as isize));
    field!((*br.borrow()), is_optimistic_).write(false);
}
pub fn BrunsliBitReaderUnload_129(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    'loop_: while ((*br.borrow()).with(|__s| __s.num_debt_bytes_) > 0_u32)
        && ((*br.borrow()).with(|__s| __s.num_bits_) >= 8_u32)
    {
        field!((*br.borrow()), num_debt_bytes_).with_mut(|__v| __v.postfix_dec());
        field!((*br.borrow()), num_bits_)
            .write({ ((*br.borrow()).with(|__s| __s.num_bits_)).wrapping_sub(8_u32) });
    }
    'loop_: while ((*br.borrow()).with(|__s| __s.num_bits_) >= 8_u32) {
        field!((*br.borrow()), next_).with_mut(|__v| __v.postfix_dec());
        field!((*br.borrow()), num_bits_)
            .write({ ((*br.borrow()).with(|__s| __s.num_bits_)).wrapping_sub(8_u32) });
    }
    let __rhs = ({ BrunsliBitReaderBitMask_121((*br.borrow()).with(|__s| __s.num_bits_)) });
    {
        let _ptr = field!((*br.borrow()), bits_);
        _ptr.write(_ptr.read() & __rhs)
    };
}
pub fn BrunsliBitReaderSuspend_130(br: Ptr<brunsli_BrunsliBitReader>) -> usize {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    ({ BrunsliBitReaderUnload_129((*br.borrow()).clone()) });
    let unused_bytes: Value<usize> = Rc::new(RefCell::new(
        ((((*br.borrow()).with(|__s| __s.end_.clone())
            - (*br.borrow()).with(|__s| __s.next_.clone())) as i64) as usize),
    ));
    field!((*br.borrow()), next_).write(Ptr::<u8>::null());
    field!((*br.borrow()), end_).write(Ptr::<u8>::null());
    return (*unused_bytes.borrow());
}
pub fn BrunsliBitReaderFinish_131(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new((*br.borrow()).with(|__s| __s.num_bits_)));
    if ((*n_bits.borrow()) >= 8_u32) {
        field!((*br.borrow()), is_healthy_).write(false);
        return;
    }
    if ((*n_bits.borrow()) > 0_u32) {
        let padding_bits: Value<u32> = Rc::new(RefCell::new(
            ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*n_bits.borrow())) }),
        ));
        if ((*padding_bits.borrow()) != 0_u32) {
            field!((*br.borrow()), is_healthy_).write(false);
        }
    }
}
pub fn BrunsliBitReaderIsHealthy_132(br: Ptr<brunsli_BrunsliBitReader>) -> bool {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    ({ BrunsliBitReaderUnload_129((*br.borrow()).clone()) });
    return ((*br.borrow()).with(|__s| __s.num_debt_bytes_) == 0_u32)
        && ((*br.borrow()).with(|__s| __s.is_healthy_));
}
pub fn BrunsliBitReaderSetOptimistic_133(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    field!((*br.borrow()), is_optimistic_).write(true);
}
pub fn BrunsliBitReaderCanRead_134(br: Ptr<brunsli_BrunsliBitReader>, n_bits: usize) -> bool {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<usize> = Rc::new(RefCell::new(n_bits));
    if (*br.borrow()).with(|__s| __s.is_optimistic_) {
        return true;
    }
    if ((*br.borrow()).with(|__s| __s.num_debt_bytes_) != 0_u32) {
        return false;
    }
    if ({ ((*br.borrow()).with(|__s| __s.num_bits_) as usize) } >= { (*n_bits.borrow()) }) {
        return true;
    }
    let num_extra_bytes: Value<usize> = Rc::new(RefCell::new(
        ((((*n_bits.borrow()).wrapping_sub(((*br.borrow()).with(|__s| __s.num_bits_) as usize)))
            .wrapping_add(7_usize))
            >> 3),
    ));
    return ({
        (*br.borrow())
            .with(|__s| __s.next_.clone())
            .offset((*num_extra_bytes.borrow()) as isize)
    } <= { (*br.borrow()).with(|__s| __s.end_.clone()) });
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
        let __this: Value<brunsli_BinaryArithmeticDecoder> = Rc::new(RefCell::new(Self {
            low_: 0_u32,
            high_: 0_u32,
            value_: 0_u32,
        }));
        let this: Ptr<brunsli_BinaryArithmeticDecoder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    pub fn new(cb: FnPtr<fn(AnyPtr, Ptr<u8>, usize) -> usize>, data: AnyPtr) -> Self {
        let cb: Value<FnPtr<fn(AnyPtr, Ptr<u8>, usize) -> usize>> = Rc::new(RefCell::new(cb));
        let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
        let __this: Value<brunsli_JPEGOutput> = Rc::new(RefCell::new(Self {
            cb: (*cb.borrow()).clone(),
            data: (*data.borrow()).clone(),
        }));
        let this: Ptr<brunsli_JPEGOutput> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_Arena_brunsli_HuffmanCode_> = Rc::new(RefCell::new(Self {
            capacity: { (*_a0.upgrade().deref()).capacity },
            storage: field!(_a0, storage)
                .with_mut(|__v: &mut Option<Value<Box<[brunsli_HuffmanCode]>>>| __v.take()),
        }));
        let this: Ptr<brunsli_Arena_brunsli_HuffmanCode_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    pub fn new_1(data: Ptr<u8>, size: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: (*data.borrow()).clone(),
            len: (*size.borrow()),
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(size: Option<usize>) -> Self {
        let size: Value<usize> = Rc::new(RefCell::new(size.unwrap_or(0_usize)));
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: Ptr::<u8>::null(),
            len: 0_usize,
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        {
            let _p: Ptr<_> = Ptr::alloc(
                (0..(*size.borrow()) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            (field_ptr!(this, buffer) as Ptr<Option<Value<Vec<u8>>>>).write(_p.to_owned_opt())
        };
        field!(this, next).write(
            (Ptr::<Vec<u8>>::decay(&(this.with(|__s| __s.buffer.clone()).as_pointer())) as Ptr<u8>),
        );
        field!(this, len).write((*size.borrow()));
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
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: { (*_a0.upgrade().deref()).next.clone() },
            len: { (*_a0.upgrade().deref()).len },
            buffer: field!(_a0, buffer).with_mut(|__v: &mut Option<Value<Vec<u8>>>| __v.take()),
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_dec_BitWriter> = Rc::new(RefCell::new(Self {
            healthy: { (*_a0.upgrade().deref()).healthy },
            output: { (*_a0.upgrade().deref()).output.clone() },
            chunk: brunsli_internal_dec_OutputChunk::move_from({ field_ptr!(_a0, chunk) }),
            data: { (*_a0.upgrade().deref()).data.clone() },
            pos: { (*_a0.upgrade().deref()).pos },
            put_buffer: { (*_a0.upgrade().deref()).put_buffer },
            put_bits: { (*_a0.upgrade().deref()).put_bits },
        }));
        let this: Ptr<brunsli_internal_dec_BitWriter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_dec_EncodeScanState> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_internal_dec_EncodeScanState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_dec_SerializationState> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_internal_dec_SerializationState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_dec_HistogramDataState> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_internal_dec_HistogramDataState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<brunsli_internal_dec_InternalState> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_internal_dec_InternalState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
pub fn IsBrunsli_139(data: Ptr<u8>, len: usize) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    thread_local!(
        static kSignature_140: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            10_u8, 4_u8, 66_u8, 210_u8, 213_u8, 78_u8,
        ])));
    );
    thread_local!(
        static kSignatureLen_141: Value<usize> =
            Rc::new(RefCell::new(::std::mem::size_of::<[u8; 6]>()));
    );
    if ((*len.borrow()) < kSignatureLen_141.with(|rc| *rc.borrow())) {
        return false;
    }
    return (((kSignature_140.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>)
        .to_any()
        .memcmp(
            &(*data.borrow()).to_any(),
            kSignatureLen_141.with(|rc| *rc.borrow()),
        )
        == 0);
}
pub fn DivCeil_142(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((((*a.borrow()) + (*b.borrow())) - 1) / (*b.borrow()));
}
pub fn DecodeVarLenUint8_143(br: Ptr<brunsli_BrunsliBitReader>) -> u32 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
        let nbits: Value<u32> = Rc::new(RefCell::new(
            ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 3_u32) }),
        ));
        if ((*nbits.borrow()) == 0_u32) {
            return 1_u32;
        } else {
            return ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*nbits.borrow())) })
                .wrapping_add((1_u32 << (*nbits.borrow())));
        }
    }
    return 0_u32;
}
pub fn DecodeVarint_144(
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_bits: usize,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_bits: Value<usize> = Rc::new(RefCell::new(max_bits));
    if (((*s.borrow()).with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_VarintState_Stage_INIT as i32))
    {
        field!((*s.borrow()), value).write(0_usize);
        field!((*s.borrow()), i).write(0_usize);
        field!((*s.borrow()), stage)
            .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { ((*s.borrow()).with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ (*s.borrow()).with(|__s| __s.i) } >= { (*max_bits.borrow()) }) {
                        field!((*s.borrow()), stage)
                            .write(brunsli_internal_dec_VarintState_Stage_INIT);
                        return true;
                    }
                    if ({ ((*s.borrow()).with(|__s| __s.i)).wrapping_add(1_usize) } != {
                        (*max_bits.borrow())
                    }) {
                        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                            return false;
                        }
                        if !(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
                            field!((*s.borrow()), stage)
                                .write(brunsli_internal_dec_VarintState_Stage_INIT);
                            return true;
                        }
                    }
                    field!((*s.borrow()), stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                        return false;
                    }
                    let next_bit: Value<usize> = Rc::new(RefCell::new(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as usize),
                    ));
                    {
                        let _ptr = field!((*s.borrow()), value);
                        _ptr.write(
                            _ptr.read() | {
                                ({ (*next_bit.borrow()) } << { (*s.borrow()).with(|__s| __s.i) })
                            },
                        )
                    };
                    field!((*s.borrow()), i).with_mut(|__v| __v.prefix_inc());
                    field!((*s.borrow()), stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                132,
                                Ptr::<u8>::from_string_literal(b"DecodeVarint"),
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
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_symbols: usize,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_symbols: Value<usize> = Rc::new(RefCell::new(max_symbols));
    if (((*s.borrow()).with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_VarintState_Stage_INIT as i32))
    {
        field!((*s.borrow()), value).write(0_usize);
        field!((*s.borrow()), i).write(0_usize);
        field!((*s.borrow()), stage)
            .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { ((*s.borrow()).with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ (*s.borrow()).with(|__s| __s.i) } < { (*max_symbols.borrow()) }) {
                        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                            return false;
                        }
                        if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
                            field!((*s.borrow()), stage)
                                .write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                            continue 'loop_;
                        }
                    }
                    field!((*s.borrow()), stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                    return true;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), (2_u64 as usize)) })
                    {
                        return false;
                    }
                    let next_bits: Value<usize> = Rc::new(RefCell::new(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (2_u64 as u32)) })
                            as usize),
                    ));
                    {
                        let _ptr = field!((*s.borrow()), value);
                        _ptr.write(
                            _ptr.read() | {
                                ({ (*next_bits.borrow()) } << {
                                    (((*s.borrow()).with(|__s| __s.i) as u64)
                                        .wrapping_mul((2_u64 as u64)))
                                })
                            },
                        )
                    };
                    field!((*s.borrow()), i).with_mut(|__v| __v.prefix_inc());
                    field!((*s.borrow()), stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                169,
                                Ptr::<u8>::from_string_literal(b"DecodeLimitedVarint"),
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
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_symbols: usize,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_symbols: Value<usize> = Rc::new(RefCell::new(max_symbols));
    if (((*s.borrow()).with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_VarintState_Stage_INIT as i32))
    {
        field!((*s.borrow()), value).write(0_usize);
        field!((*s.borrow()), i).write(0_usize);
        field!((*s.borrow()), stage)
            .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
    }
    'loop_: while true {
        'switch: {
            match { ((*s.borrow()).with(|__s| __s.stage) as i32) } {
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION as i32) => {
                    if ({ (*s.borrow()).with(|__s| __s.i) } < { (*max_symbols.borrow()) }) {
                        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                            return false;
                        }
                        if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
                            field!((*s.borrow()), stage)
                                .write(brunsli_internal_dec_VarintState_Stage_READ_DATA);
                            continue 'loop_;
                        }
                    }
                    field!((*s.borrow()), stage).write(brunsli_internal_dec_VarintState_Stage_INIT);
                    return true;
                }
                __v if __v == (brunsli_internal_dec_VarintState_Stage_READ_DATA as i32) => {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), (8_u64 as usize)) })
                    {
                        return false;
                    }
                    let next_bits: Value<usize> = Rc::new(RefCell::new(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (8_u64 as u32)) })
                            as usize),
                    ));
                    {
                        let _ptr = field!((*s.borrow()), value);
                        _ptr.write(
                            _ptr.read() | {
                                ({ (*next_bits.borrow()) } << {
                                    (((*s.borrow()).with(|__s| __s.i) as u64)
                                        .wrapping_mul((8_u64 as u64)))
                                })
                            },
                        )
                    };
                    field!((*s.borrow()), i).with_mut(|__v| __v.prefix_inc());
                    field!((*s.borrow()), stage)
                        .write(brunsli_internal_dec_VarintState_Stage_READ_CONTINUATION);
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                169,
                                Ptr::<u8>::from_string_literal(b"DecodeLimitedVarint"),
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
pub fn GenerateApp0Marker_147(app0_status: u8) -> Vec<u8> {
    let app0_status: Value<u8> = Rc::new(RefCell::new(app0_status));
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
        (if ((((*app0_status.borrow()) as u32) & 1_u32) != 0) {
            2
        } else {
            1
        } as u8),
    );
    (*app0_status.borrow_mut()) = { (((*app0_status.borrow()) as i32) >> 1_u32) as u8 };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 10_usize)
        .write(((((*app0_status.borrow()) as u32) & 3_u32) as u8));
    (*app0_status.borrow_mut()) = { (((*app0_status.borrow()) as i32) >> 2_u32) as u8 };
    let x_dens: Value<u16> = Rc::new(RefCell::new(
        ({
            let __idx = (*app0_status.borrow()) as usize;
            kApp0Densities_46.with(|rc| rc.borrow()[__idx])
        }),
    ));
    let __rhs = {
        elem!((app0_marker.as_pointer() as Ptr<u8>), 13_usize)
            .write(((((*x_dens.borrow()) as i32) >> 8_u32) as u8));
        (elem!((app0_marker.as_pointer() as Ptr<u8>), 13_usize).read())
    };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 11_usize).write(__rhs);
    let __rhs = {
        elem!((app0_marker.as_pointer() as Ptr<u8>), 14_usize)
            .write(((((*x_dens.borrow()) as u32) & 255_u32) as u8));
        (elem!((app0_marker.as_pointer() as Ptr<u8>), 14_usize).read())
    };
    elem!((app0_marker.as_pointer() as Ptr<u8>), 12_usize).write(__rhs);
    return std::mem::take(&mut (*app0_marker.borrow_mut()));
}
pub fn GenerateAppMarker_148(marker: u8, code: u8) -> Vec<u8> {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let code: Value<u8> = Rc::new(RefCell::new(code));
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if (((*marker.borrow()) as i32) == 128) {
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
        elem!((s.as_pointer() as Ptr<u8>), 84_usize).write((*code.borrow()));
    } else if (((*marker.borrow()) as i32) == 129) {
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
        elem!((s.as_pointer() as Ptr<u8>), 15_usize).write((*code.borrow()));
    } else {
        if !(((*marker.borrow()) as i32) == 130) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    197,
                    Ptr::<u8>::from_string_literal(b"GenerateAppMarker"),
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
        elem!((s.as_pointer() as Ptr<u8>), 10_usize).write((*code.borrow()));
    }
    return std::mem::take(&mut (*s.borrow_mut()));
}
pub fn ProcessMetaData_149(
    data: Ptr<u8>,
    len: usize,
    state: Ptr<brunsli_internal_dec_MetadataState>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let state: Value<Ptr<brunsli_internal_dec_MetadataState>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*pos.borrow()) < (*len.borrow())) {
        'switch: {
            match { (*state.borrow()).with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize) => {
                    let __rhs = (elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).read());
                    field!((*state.borrow()), marker).write(__rhs);
                    if (((*state.borrow()).with(|__s| __s.marker) as i32) == 217) {
                        ((*jpg.borrow()).with(|__s| __s.tail_data.as_pointer()) as Ptr<Vec<u8>>)
                            .write(Vec::new());
                        field!((*state.borrow()), stage)
                            .write((brunsli_internal_dec_MetadataState_Stage_READ_TAIL as usize));
                        continue 'loop_;
                    } else if (((*state.borrow()).with(|__s| __s.marker) as i32) < 64) {
                        field!((*state.borrow()), short_marker_count)
                            .with_mut(|__v| __v.postfix_inc());
                        if ({ (*state.borrow()).with(|__s| __s.short_marker_count) } > {
                            (kBrunsliShortMarkerLimit_23.with(|rc| *rc.borrow()) as usize)
                        }) {
                            return false;
                        }
                        ((*jpg.borrow()).with(|__s| __s.app_data.as_pointer())
                            as Ptr<Vec<Value<Vec<u8>>>>)
                            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                                __v.push(Rc::new(RefCell::new(
                                    ({
                                        GenerateApp0Marker_147(
                                            (*state.borrow()).with(|__s| __s.marker),
                                        )
                                    }),
                                )))
                            });
                        continue 'loop_;
                    } else if (((*state.borrow()).with(|__s| __s.marker) as i32) >= 128)
                        && (((*state.borrow()).with(|__s| __s.marker) as i32) <= 130)
                    {
                        field!((*state.borrow()), short_marker_count)
                            .with_mut(|__v| __v.postfix_inc());
                        if ({ (*state.borrow()).with(|__s| __s.short_marker_count) } > {
                            (kBrunsliShortMarkerLimit_23.with(|rc| *rc.borrow()) as usize)
                        }) {
                            return false;
                        }
                        field!((*state.borrow()), stage)
                            .write((brunsli_internal_dec_MetadataState_Stage_READ_CODE as usize));
                        continue 'loop_;
                    }
                    if (((*state.borrow()).with(|__s| __s.marker) as i32) != 254)
                        && ((((*state.borrow()).with(|__s| __s.marker) as i32) >> 4_u32) != 14)
                    {
                        return false;
                    }
                    field!((*state.borrow()), stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_HI as usize));
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_TAIL as usize) => {
                    ({
                        let _begin: Ptr<u8> = (*data.borrow()).offset((*pos.borrow()) as isize);
                        let _end: Ptr<u8> = (*data.borrow()).offset((*len.borrow()) as isize);
                        Append_71(
                            ((*jpg.borrow()).with(|__s| __s.tail_data.as_pointer())),
                            _begin,
                            _end,
                        )
                    });
                    (*pos.borrow_mut()) = (*len.borrow());
                    continue 'loop_;
                }
                __v if __v == (brunsli_internal_dec_MetadataState_Stage_READ_CODE as usize) => {
                    let code: Value<u8> = Rc::new(RefCell::new(
                        (elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).read()),
                    ));
                    ((*jpg.borrow()).with(|__s| __s.app_data.as_pointer())
                        as Ptr<Vec<Value<Vec<u8>>>>)
                        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                            __v.push(Rc::new(RefCell::new(
                                ({
                                    GenerateAppMarker_148(
                                        (*state.borrow()).with(|__s| __s.marker),
                                        (*code.borrow()),
                                    )
                                }),
                            )))
                        });
                    field!((*state.borrow()), stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_MARKER as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_HI as usize) =>
                {
                    let __rhs = (elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).read());
                    field!((*state.borrow()), length_hi).write(__rhs);
                    field!((*state.borrow()), stage)
                        .write((brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_LO as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_MetadataState_Stage_READ_LENGTH_LO as usize) =>
                {
                    let lo: Value<u8> = Rc::new(RefCell::new(
                        (elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).read()),
                    ));
                    let marker_len: Value<usize> = Rc::new(RefCell::new(
                        (({ (((*state.borrow()).with(|__s| __s.length_hi) as i32) << 8_u32) } + {
                            ((*lo.borrow()) as i32)
                        }) as usize),
                    ));
                    if ((*marker_len.borrow()) < 2_usize) {
                        return false;
                    }
                    field!((*state.borrow()), remaining_multibyte_length)
                        .write((*marker_len.borrow()).wrapping_sub(2_usize));
                    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
                        (*state.borrow()).with(|__s| __s.marker),
                        (*state.borrow()).with(|__s| __s.length_hi),
                        (*lo.borrow()),
                    ])));
                    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> = Rc::new(RefCell::new(
                        if (((*state.borrow()).with(|__s| __s.marker) as i32) == 254) {
                            ((*jpg.borrow()).with(|__s| __s.com_data.as_pointer()))
                        } else {
                            ((*jpg.borrow()).with(|__s| __s.app_data.as_pointer()))
                        },
                    ));
                    let delta: Value<usize> = Rc::new(RefCell::new(
                        if (((*state.borrow()).with(|__s| __s.marker) as i32) == 254) {
                            0_usize
                        } else {
                            (*state.borrow()).with(|__s| __s.short_marker_count)
                        },
                    ));
                    if ({
                        ((*(*dest.borrow()).upgrade().deref()).len() as u64)
                            .wrapping_sub(((*delta.borrow()) as u64))
                    } >= { (kBrunsliMultibyteMarkerLimit_24.with(|rc| *rc.borrow()) as u64) })
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
                        ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
                            |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(__init))),
                        )
                    };
                    let __rhs = ((*(*dest.borrow()).upgrade().deref())
                        [(*(*dest.borrow()).upgrade().deref()).len() - 1]
                        .as_pointer());
                    field!((*state.borrow()), multibyte_sink).write(__rhs);
                    field!((*state.borrow()), stage).write({
                        (if ((*state.borrow()).with(|__s| __s.remaining_multibyte_length) > 0_usize)
                        {
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
                    let chunk_size: Value<usize> = Rc::new(RefCell::new(
                        ({
                            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                                ((*state.borrow()).with(|__s| __s.remaining_multibyte_length)
                                    as u64),
                            ));
                            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                                ((*len.borrow()).wrapping_sub((*pos.borrow())) as u64),
                            ));
                            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                                __tmp_0.as_pointer()
                            } else {
                                __tmp_1.as_pointer()
                            }
                            .read())
                        } as usize),
                    ));
                    ({
                        Append_72(
                            (*state.borrow()).with(|__s| __s.multibyte_sink.clone()),
                            (*data.borrow()).offset((*pos.borrow()) as isize),
                            (*chunk_size.borrow()),
                        )
                    });
                    field!((*state.borrow()), remaining_multibyte_length).write({
                        ((*state.borrow()).with(|__s| __s.remaining_multibyte_length))
                            .wrapping_sub((*chunk_size.borrow()))
                    });
                    (*pos.borrow_mut()) = { (*pos.borrow()).wrapping_add((*chunk_size.borrow())) };
                    if ((*state.borrow()).with(|__s| __s.remaining_multibyte_length) == 0_usize) {
                        field!((*state.borrow()), stage)
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
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> = field_ptr!(s, internals);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(js, br))));
    'loop_: while true {
        'switch: {
            match { (js.with(|__s| __s.stage) as i32) } {
                __v if __v
                    == (brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST as i32) =>
                {
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 1_usize) }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    field!(js, is_known_last_huffman_code).write(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as usize),
                    );
                    {
                        let __init = <brunsli_JPEGHuffmanCode>::default();
                        (*(*jpg.borrow())
                            .with(|__s| __s.huffman_code.clone())
                            .borrow_mut())
                        .push(__init)
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
                            (*br.borrow()).clone(),
                            ((5 + (!(js.with(|__s| __s.is_known_last_huffman_code) != 0) as i32))
                                as usize),
                        )
                    }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*jpg.borrow()).with(|__s| __s.huffman_code.as_pointer())
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    let __rhs =
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as i32);
                    field!((*huff.borrow()), slot_id).write(__rhs);
                    field!(js, is_dc_table).write(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) == 0_u32),
                    );
                    {
                        let _ptr = field!((*huff.borrow()), slot_id);
                        _ptr.write(
                            _ptr.read()
                                + if js.with(|__s| __s.is_dc_table) {
                                    0
                                } else {
                                    16
                                },
                        )
                    };
                    let __rhs = (js.with(|__s| __s.is_known_last_huffman_code) != 0)
                        || (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0);
                    field!((*huff.borrow()), is_last).write(__rhs);
                    elem!(
                        ((*huff.borrow()).with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                        0_usize
                    )
                    .write(0);
                    let found_match: Value<i32> = Rc::new(RefCell::new(
                        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as i32),
                    ));
                    if ((*found_match.borrow()) != 0) {
                        if js.with(|__s| __s.is_dc_table) {
                            let huff_table_idx: Value<i32> = Rc::new(RefCell::new(
                                (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) })
                                    as i32),
                            ));
                            {
                                ((((*huff.borrow()).with(|__s| __s.counts.as_pointer())
                                    as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeCounts_54.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()))
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as usize,
                                    );
                                ((((*huff.borrow()).with(|__s| __s.counts.as_pointer())
                                    as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                            {
                                ((((*huff.borrow()).with(|__s| __s.values.as_pointer())
                                    as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeValues_55.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()))
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 13]>() as usize,
                                    );
                                ((((*huff.borrow()).with(|__s| __s.values.as_pointer())
                                    as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                        } else {
                            let huff_table_idx: Value<i32> = Rc::new(RefCell::new(
                                (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) })
                                    as i32),
                            ));
                            {
                                ((((*huff.borrow()).with(|__s| __s.counts.as_pointer())
                                    as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeCounts_57.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()))
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as usize,
                                    );
                                ((((*huff.borrow()).with(|__s| __s.counts.as_pointer())
                                    as Ptr<i32>)
                                    .offset(1_usize)) as Ptr<i32>)
                                    .to_any()
                            };
                            {
                                ((((*huff.borrow()).with(|__s| __s.values.as_pointer())
                                    as Ptr<i32>)
                                    .offset(0_usize)) as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeValues_59.with(|v| v.as_pointer())
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()))
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 163]>() as usize,
                                    );
                                ((((*huff.borrow()).with(|__s| __s.values.as_pointer())
                                    as Ptr<i32>)
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
                    if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 4_usize) }) {
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    field!(js, max_len).write(
                        ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 4_u32) })
                            .wrapping_add(1_u32)) as usize),
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
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*jpg.borrow()).with(|__s| __s.huffman_code.as_pointer())
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    if ({ js.with(|__s| __s.i) } <= { js.with(|__s| __s.max_len) }) {
                        let shift: Value<usize> = Rc::new(RefCell::new(
                            (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize)
                                .wrapping_sub(js.with(|__s| __s.i)),
                        ));
                        let count_limit: Value<usize> = Rc::new(RefCell::new(
                            ({
                                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                                    ((js.with(|__s| __s.max_count))
                                        .wrapping_sub(js.with(|__s| __s.total_count))
                                        as u64),
                                ));
                                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                                    (({ js.with(|__s| __s.space) } >> { (*shift.borrow()) })
                                        as u64),
                                ));
                                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                                    __tmp_0.as_pointer()
                                } else {
                                    __tmp_1.as_pointer()
                                }
                                .read())
                            } as usize),
                        ));
                        if ((*count_limit.borrow()) > 0_usize) {
                            let nbits: Value<i32> = Rc::new(RefCell::new(
                                (({ Log2FloorNonZero_74(((*count_limit.borrow()) as u32)) }) + 1),
                            ));
                            if !({
                                BrunsliBitReaderCanRead_134(
                                    (*br.borrow()).clone(),
                                    ((*nbits.borrow()) as usize),
                                )
                            }) {
                                return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                            }
                            let count: Value<usize> = Rc::new(RefCell::new(
                                (({
                                    BrunsliBitReaderRead_126(
                                        (*br.borrow()).clone(),
                                        ((*nbits.borrow()) as u32),
                                    )
                                }) as usize),
                            ));
                            if ((*count.borrow()) > (*count_limit.borrow())) {
                                return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                            }
                            elem!(
                                ((*huff.borrow()).with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                                js.with(|__s| __s.i)
                            )
                            .write(((*count.borrow()) as i32));
                            field!(js, total_count).write({
                                (js.with(|__s| __s.total_count)).wrapping_add((*count.borrow()))
                            });
                            field!(js, space).write({
                                (js.with(|__s| __s.space)).wrapping_sub(
                                    (*count.borrow()).wrapping_mul((1_usize << (*shift.borrow()))),
                                )
                            });
                        }
                        field!(js, i).with_mut(|__v| __v.prefix_inc());
                        continue 'loop_;
                    }
                    elem!(
                        ((*huff.borrow()).with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
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
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*jpg.borrow()).with(|__s| __s.huffman_code.as_pointer())
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    if ({ js.with(|__s| __s.i) } < { js.with(|__s| __s.total_count) }) {
                        let nbits: Value<i32> = Rc::new(RefCell::new(
                            ({ brunsli_PermutationCoderImpl::num_bits(&field_ptr!(js, p)) }),
                        ));
                        if !({
                            DecodeLimitedVarint_145(
                                (field_ptr!(js, varint)),
                                (*br.borrow()).clone(),
                                ((((*nbits.borrow()) + 1) >> 1_u32) as usize),
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
                            ((*huff.borrow()).with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                            js.with(|__s| __s.i)
                        )
                        .write(((*value.borrow()) as i32));
                        field!(js, i).with_mut(|__v| __v.prefix_inc());
                        continue 'loop_;
                    }
                    elem!(
                        ((*huff.borrow()).with(|__s| __s.values.as_pointer()) as Ptr<i32>),
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
                    if ((*jpg.borrow()).with(|__s| __s.huffman_code.as_pointer())
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
                    if ({
                        (*(*jpg.borrow())
                            .with(|__s| __s.huffman_code.clone())
                            .borrow())
                        .len()
                    } >= { (kMaxDHTMarkers_10.with(|rc| *rc.borrow()) as usize) })
                    {
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
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> = field_ptr!(s, internals);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(js, br))));
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
            match { ( ( js.with(|__s| __s . stage ) as i32 ) )  } { __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON as i32 ) )  =>  { { let si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  ) )) ;
  ;
 ;
 if ! ( ( { BrunsliBitReaderCanRead_134 ( ((*br.borrow()) ).clone() , 22_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 6_u32  , ) } )  as i32 ) )  ;
 field!((*si.borrow()) , Ss) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 6_u32  , ) } )  as i32 ) )  ;
 field!((*si.borrow()) , Se) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 4_u32  , ) } )  as i32 ) )  ;
 field!((*si.borrow()) , Ah) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 4_u32  , ) } )  as i32 ) )  ;
 field!((*si.borrow()) , Al) .write( __rhs ) ;
 let __rhs = ( ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 2_u32  , ) } )  ) . wrapping_add ( 1_u32 ) ) as usize )  ;
 field!((*si.borrow()) , num_components) .write( __rhs ) ;
 field!(js, j) .write( 0_usize  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMPONENT  ) ;
 ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMPONENT as i32 ) )  =>  { { let si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  ) )) ;
  ;
 ;
 if ({ js.with(|__s| __s . j )  } < { (*si.borrow()) .with(|__s| __s . num_components )  }) { if ! ( ( { BrunsliBitReaderCanRead_134 ( ((*br.borrow()) ).clone() , 6_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 2_u32  , ) } )  as u8 ) )  ;
 field!(elem!(((*si.borrow()) .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), comp_idx) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 2_u32  , ) } )  as i32 ) )  ;
 field!(elem!(((*si.borrow()) .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), dc_tbl_idx) .write( __rhs ) ;
 let __rhs = ( ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 2_u32  , ) } )  as i32 ) )  ;
 field!(elem!(((*si.borrow()) .with(|__s| __s . components .as_pointer())  as Ptr<brunsli_JPEGComponentScanInfo>), js.with(|__s| __s . j ) ), ac_tbl_idx) .write( __rhs ) ;
 field!(js, j) .with_mut(|__v| __v. postfix_inc ()) ;
 } else { field!(js, last_block_idx) .write( - 1_i32  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION  ) ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION as i32 ) )  =>  { { if ! ( ( { BrunsliBitReaderCanRead_134 ( ((*br.borrow()) ).clone() , 1_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } if ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 1_u32  , ) } )  != 0 ) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_DATA  ) ;
 } else { field!(js, last_block_idx) .write( 0  ) ;
 field!(js, last_num) .write( 0  ) ;
 field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION  ) ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_DATA as i32 ) )  =>  { { let si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*jpg.borrow()) .with(|__s| __s . scan_info .as_pointer())  as Ptr<brunsli_JPEGScanInfo>).offset(js.with(|__s| __s . i ) )  ) )) ;
  ;
 ;
 if ! ( ( { DecodeVarint_144 ( ( field_ptr!(js , varint)  )  , ((*br.borrow()) ).clone() , 28_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let block_idx : Value<i32 > = Rc::new(RefCell::new(( ({ js.with(|__s| __s . last_block_idx )  } + { ( ( js.with(|__s| __s . varint  . value ) as i32 ) )  }) + 1 ) )) ;
  ;
 ;
 {let __init = (*block_idx.borrow()) ;
    (*(*si.borrow()) .with(|__s| __s . reset_points .clone()).borrow_mut()) .push(__init)}  ;
 field!(js, last_block_idx) .write( (*block_idx.borrow())  ) ;
 if ( js.with(|__s| __s . last_block_idx ) > ( ( 1 << 30 ) ) ) { return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN   ;
 } field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_RESET_POINT_CONTINUATION  ) ;
 ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_CONTINUATION as i32 ) )  =>  { { if ! ( ( { BrunsliBitReaderCanRead_134 ( ((*br.borrow()) ).clone() , 1_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } if ( ( { BrunsliBitReaderRead_126 ( ((*br.borrow()) ).clone() , 1_u32  , ) } )  != 0 ) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_DATA  ) ;
 } else { ( { (*maybe_add_zero_run.borrow())  .call ( ) } ) ;
 field!(js, i) .with_mut(|__v| __v. prefix_inc ()) ;
 if ({ js.with(|__s| __s . i )  } < { js.with(|__s| __s . num_scans )  }) { field!(js, stage) .write( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_COMMON  ) ;
 ;
 continue 'loop_ ;
 } return brunsli_BrunsliStatus_BRUNSLI_OK   ;
 } ;
 continue 'loop_ ;
 } }, __v if __v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage_READ_SCAN_ZERO_RUN_DATA as i32 ) )  =>  { { if ! ( ( { DecodeVarint_144 ( ( field_ptr!(js , varint)  )  , ((*br.borrow()) ).clone() , 28_usize  , ) } )  ) { return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA   ;
 } let block_idx : Value<i32 > = Rc::new(RefCell::new(({ js.with(|__s| __s . last_block_idx )  } + { ( ( js.with(|__s| __s . varint  . value ) as i32 ) )  }) )) ;
  ;
 ;
 if ({ (*block_idx.borrow())  } > { js.with(|__s| __s . last_block_idx )  }) { ( { (*maybe_add_zero_run.borrow())  .call ( ) } ) ;
 } field!(js, last_num) .with_mut(|__v| __v. prefix_inc ()) ;
 field!(js, last_block_idx) .write( (*block_idx.borrow())  ) ;
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
    order: Ptr<u32>,
    br: Ptr<brunsli_BitSource>,
    in_: Ptr<brunsli_WordSource>,
) -> bool {
    let order: Value<Ptr<u32>> = Rc::new(RefCell::new(order));
    let br: Value<Ptr<brunsli_BitSource>> = Rc::new(RefCell::new(br));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
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
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        if !(({ brunsli_BitSourceImpl::ReadBits(&(*br.borrow()), 1, (*in_.borrow()).clone()) })
            != 0)
        {
            (*i.borrow_mut()) += kSpan_153.with(|rc| *rc.borrow());
            continue 'loop_;
        }
        let start: Value<i32> = Rc::new(RefCell::new(if ((*i.borrow()) > 0) {
            (*i.borrow())
        } else {
            1
        }));
        let end: Value<i32> = Rc::new(RefCell::new(
            ((*i.borrow()) + kSpan_153.with(|rc| *rc.borrow())),
        ));
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            let v: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*v.borrow()) <= (kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)) {
                let bits: Value<u32> = Rc::new(RefCell::new(
                    ({
                        brunsli_BitSourceImpl::ReadBits(&(*br.borrow()), 3, (*in_.borrow()).clone())
                    }),
                ));
                (*v.borrow_mut()) = { (*v.borrow()).wrapping_add((*bits.borrow())) };
                if ((*bits.borrow()) < 7_u32) {
                    break;
                }
            }
            if ((*v.borrow()) > (kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)) {
                return false;
            }
            (*lehmer.borrow_mut())[(*j.borrow()) as usize] = (*v.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()) += kSpan_153.with(|rc| *rc.borrow());
    }
    let end: Value<i32> = Rc::new(RefCell::new((kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1)));
    'loop_: while ((*end.borrow()) >= 1) && ((*lehmer.borrow())[(*end.borrow()) as usize] == 0_u32)
    {
        (*end.borrow_mut()).prefix_dec();
    }
    if ((*lehmer.borrow())[(*end.borrow()) as usize] == 1_u32) {
        return false;
    }
    let i: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*i.borrow()) <= (*end.borrow())) {
        if ((*lehmer.borrow())[(*i.borrow()) as usize] == 0_u32) {
            return false;
        }
        (*lehmer.borrow_mut())[(*i.borrow()) as usize].prefix_dec();
        (*i.borrow_mut()).prefix_inc();
    }
    if !({
        DecodeLehmerCode_113(
            (lehmer.as_pointer() as Ptr<u32>),
            (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize),
            (*order.borrow()).clone(),
        )
    }) {
        return false;
    }
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        elem!((*order.borrow()), (*k.borrow())).write({
            ({
                let __idx = (elem!((*order.borrow()), (*k.borrow())).read()) as usize;
                kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
            })
        });
        (*k.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn DecodeNumNonzeros_154(
    p: Ptr<brunsli_Prob>,
    ac: Ptr<brunsli_BinaryArithmeticDecoder>,
    in_: Ptr<brunsli_WordSource>,
) -> usize {
    let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
    let ac: Value<Ptr<brunsli_BinaryArithmeticDecoder>> = Rc::new(RefCell::new(ac));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let bst: Value<Ptr<brunsli_Prob>> =
        Rc::new(RefCell::new((*p.borrow()).offset(-((1) as isize))));
    let ctx: Value<usize> = Rc::new(RefCell::new(1_usize));
    let b: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*b.borrow()) < kNumNonZeroBits_84.with(|rc| *rc.borrow())) {
        let bit: Value<i32> = Rc::new(RefCell::new(
            ({
                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                    &(*ac.borrow()),
                    (({
                        brunsli_ProbImpl::get_proba(
                            &(*bst.borrow()).offset((*ctx.borrow()) as isize),
                        )
                    }) as i32),
                    (*in_.borrow()).clone(),
                )
            }),
        ));
        ({
            let _val: i32 = (*bit.borrow());
            brunsli_ProbImpl::Add(&(*bst.borrow()).offset((*ctx.borrow()) as isize), _val)
        });
        (*ctx.borrow_mut()) =
            { ((2_usize).wrapping_mul((*ctx.borrow()))).wrapping_add(((*bit.borrow()) as usize)) };
        (*b.borrow_mut()).prefix_inc();
    }
    let val: Value<usize> = Rc::new(RefCell::new(
        (*ctx.borrow())
            .wrapping_sub(((1_u32 << kNumNonZeroBits_84.with(|rc| *rc.borrow())) as usize)),
    ));
    if !((*val.borrow()) <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                593,
                Ptr::<u8>::from_string_literal(b"DecodeNumNonzeros"),
            )
        });
        'loop_: while true {}
    };
    return (*val.borrow());
}
pub fn EnsureSubdecodersInitialized_155(
    state: Ptr<brunsli_internal_dec_State>,
    in_: Ptr<brunsli_WordSource>,
) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    if !(s.with(|__s| __s.subdecoders_initialized)) {
        ({ brunsli_ANSDecoderImpl::Init(&field_ptr!(s, ans_decoder), (*in_.borrow()).clone()) });
        ({ brunsli_BitSourceImpl::Init(&field_ptr!(s, bit_reader), (*in_.borrow()).clone()) });
        ({
            brunsli_BinaryArithmeticDecoderImpl::Init(
                &field_ptr!(s, arith_decoder),
                (*in_.borrow()).clone(),
            )
        });
        field!(s, subdecoders_initialized).write(true);
    }
}
pub fn FinalizeSubdecoders_156(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
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
    state: Ptr<brunsli_internal_dec_State>,
    in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new(
        ({
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
        }),
    ));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> = field_ptr!(s, ac_dc);
    let comps: Ptr<Vec<brunsli_ComponentStateDC>> = ac_dc_state.with(|__s| __s.dc.as_pointer());
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = (*num_components.borrow()) as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| {
                __v.resize_with(__a0, || <brunsli_ComponentStateDC>::default())
            })
        };
        let c: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            ({
                let _w: i32 = {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        (*c.borrow())
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                };
                brunsli_ComponentStateDCImpl::SetWidth(
                    &(Ptr::<Vec<brunsli_ComponentStateDC>>::decay(&(comps))
                        as Ptr<brunsli_ComponentStateDC>)
                        .offset((*c.borrow())),
                    _w,
                )
            });
            (*c.borrow_mut()).prefix_inc();
        }
    }
    if !({ brunsli_WordSourceImpl::CanRead(&(*in_.borrow()), 5_usize) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({ EnsureSubdecodersInitialized_155((*state.borrow()).clone(), (*in_.borrow()).clone()) });
    let ans: Value<brunsli_ANSDecoder> =
        Rc::new(RefCell::new(s.with(|__s| __s.ans_decoder.clone())));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new(s.with(|__s| __s.bit_reader.clone())));
    let ac: Value<brunsli_BinaryArithmeticDecoder> =
        Rc::new(RefCell::new(s.with(|__s| __s.arith_decoder.clone())));
    let mcu_y: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_mcu_y)));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_component)));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
                ((Ptr::<Vec<brunsli_ComponentStateDC>>::decay(&(comps))
                    as Ptr<brunsli_ComponentStateDC>)
                    .offset((*i.borrow()))),
            ));
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*i.borrow()));
            let context_map: Value<Ptr<u8>> = Rc::new(RefCell::new(
                (*state.borrow())
                    .with(|__s| __s.context_map.clone())
                    .offset(
                        ((*i.borrow()).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
                            as isize,
                    ),
            ));
            let ac_stride: Value<i32> = Rc::new(RefCell::new((m.with(|__s| __s.ac_stride) as i32)));
            let b_stride: Value<usize> =
                Rc::new(RefCell::new((m.with(|__s| __s.b_stride) as usize)));
            let width: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.width_in_blocks)));
            let y: Value<i32> = Rc::new(RefCell::new(
                ({ ({ (*mcu_y.borrow()) } * { m.with(|__s| __s.v_samp) }) } + {
                    ac_dc_state.with(|__s| __s.next_iy)
                }),
            ));
            let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*c.borrow()).with(|__s| __s.prev_sign.as_pointer()) as Ptr<i32>)
                    .offset(1_usize)),
            ));
            let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*c.borrow()).with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>)
                    .offset(2_usize)),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_iy)));
            'loop_: while ({ (*iy.borrow()) } < { m.with(|__s| __s.v_samp) }) {
                let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.ac_coeffs.clone())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize)
                        .offset(
                            ({ ac_dc_state.with(|__s| __s.next_x) } * {
                                kDCTBlockSize_3.with(|rc| *rc.borrow())
                            }) as isize,
                        ),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.block_state.clone())
                        .offset(
                            (((*y.borrow()) as usize).wrapping_mul((*b_stride.borrow()))) as isize,
                        )
                        .offset((ac_dc_state.with(|__s| __s.next_x)) as isize),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_x)));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    if ((!({ brunsli_WordSourceImpl::CanRead(&(*in_.borrow()), 6_usize) }) as i64)
                        != 0)
                    {
                        field!(ac_dc_state, next_mcu_y).write((*mcu_y.borrow()));
                        field!(ac_dc_state, next_component).write((*i.borrow()));
                        field!(ac_dc_state, next_iy).write((*iy.borrow()));
                        field!(ac_dc_state, next_x).write((*x.borrow()));
                        field!(s, ans_decoder).write((*ans.borrow()).clone());
                        field!(s, bit_reader).write((*br.borrow()).clone());
                        field!(s, arith_decoder).write((*ac.borrow()).clone());
                        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                    }
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
                    let is_empty_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                        (((*c.borrow()).with(|__s| __s.is_empty_block_prob.as_pointer())
                            as Ptr<brunsli_Prob>)
                            .offset(((*is_empty_ctx.borrow()) as usize))),
                    ));
                    let is_empty_block: Value<bool> = Rc::new(RefCell::new(
                        !(({
                            brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                &ac.as_pointer(),
                                (({ brunsli_ProbImpl::get_proba(&(*is_empty_p.borrow())) }) as i32),
                                (*in_.borrow()).clone(),
                            )
                        }) != 0),
                    ));
                    ({
                        brunsli_ProbImpl::Add(
                            &(*is_empty_p.borrow()),
                            (!(*is_empty_block.borrow()) as i32),
                        )
                    });
                    elem!(
                        ((*c.borrow()).with(|__s| __s.prev_is_nonempty.as_pointer()) as Ptr<i32>),
                        (((*x.borrow()) + 1) as usize)
                    )
                    .write((!(*is_empty_block.borrow()) as i32));
                    (*block_state.borrow()).write({ ((*is_empty_block.borrow()) as u8) });
                    let abs_val: Value<i32> = Rc::new(RefCell::new(0));
                    let sign: Value<i32> = Rc::new(RefCell::new(0));
                    if !(*is_empty_block.borrow()) {
                        let p_is_zero: Value<Ptr<brunsli_Prob>> =
                            Rc::new(RefCell::new((field_ptr!((*c.borrow()), is_zero_prob))));
                        let is_zero: Value<i32> = Rc::new(RefCell::new(
                            ({
                                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                    &ac.as_pointer(),
                                    (({ brunsli_ProbImpl::get_proba(&(*p_is_zero.borrow())) })
                                        as i32),
                                    (*in_.borrow()).clone(),
                                )
                            }),
                        ));
                        ({ brunsli_ProbImpl::Add(&(*p_is_zero.borrow()), (*is_zero.borrow())) });
                        if !((*is_zero.borrow()) != 0) {
                            let avg_ctx: Value<i32> = Rc::new(RefCell::new(
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
                            let sign_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*c.borrow()).with(|__s| __s.sign_prob.as_pointer())
                                    as Ptr<brunsli_Prob>)
                                    .offset(((*sign_ctx.borrow()) as usize))),
                            ));
                            (*sign.borrow_mut()) = ({
                                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                    &ac.as_pointer(),
                                    (({ brunsli_ProbImpl::get_proba(&(*sign_p.borrow())) }) as i32),
                                    (*in_.borrow()).clone(),
                                )
                            });
                            ({ brunsli_ProbImpl::Add(&(*sign_p.borrow()), (*sign.borrow())) });
                            let entropy_ix: Value<i32> = Rc::new(RefCell::new(
                                ((elem!((*context_map.borrow()), (*avg_ctx.borrow())).read())
                                    as i32),
                            ));
                            let code: Value<i32> = Rc::new(RefCell::new(
                                ({
                                    let _code: Ptr<brunsli_ANSDecodingData> = (*state.borrow())
                                        .with(|__s| __s.entropy_codes.clone())
                                        .offset((*entropy_ix.borrow()) as isize);
                                    let _in_: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                    brunsli_ANSDecoderImpl::ReadSymbol(
                                        &ans.as_pointer(),
                                        _code,
                                        _in_,
                                    )
                                }),
                            ));
                            if ((*code.borrow()) < kNumDirectCodes_135.with(|rc| *rc.borrow())) {
                                (*abs_val.borrow_mut()) = ((*code.borrow()) + 1);
                            } else {
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    ((*code.borrow())
                                        - kNumDirectCodes_135.with(|rc| *rc.borrow())),
                                ));
                                let p_first_extra_bit: Value<Ptr<brunsli_Prob>> =
                                    Rc::new(RefCell::new(
                                        (((*c.borrow())
                                            .with(|__s| __s.first_extra_bit_prob.as_pointer())
                                            as Ptr<brunsli_Prob>)
                                            .offset(((*nbits.borrow()) as usize))),
                                    ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    ({
                                        brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                                            &ac.as_pointer(),
                                            (({
                                                brunsli_ProbImpl::get_proba(
                                                    &(*p_first_extra_bit.borrow()),
                                                )
                                            }) as i32),
                                            (*in_.borrow()).clone(),
                                        )
                                    }),
                                ));
                                ({
                                    brunsli_ProbImpl::Add(
                                        &(*p_first_extra_bit.borrow()),
                                        (*first_extra_bit.borrow()),
                                    )
                                });
                                let extra_bits_val: Value<i32> = Rc::new(RefCell::new(
                                    ((*first_extra_bit.borrow()) << (*nbits.borrow())),
                                ));
                                if ((*nbits.borrow()) > 0) {
                                    (*extra_bits_val.borrow_mut()) |= (({
                                        brunsli_BitSourceImpl::ReadBits(
                                            &br.as_pointer(),
                                            (*nbits.borrow()),
                                            (*in_.borrow()).clone(),
                                        )
                                    })
                                        as i32);
                                }
                                (*abs_val.borrow_mut()) =
                                    (((kNumDirectCodes_135.with(|rc| *rc.borrow()) - 1)
                                        + (2 << (*nbits.borrow())))
                                        + (*extra_bits_val.borrow()));
                            }
                        }
                    }
                    elem!((*prev_abs.borrow()), (*x.borrow())).write({ (*abs_val.borrow()) });
                    elem!((*prev_sgn.borrow()), (*x.borrow())).write({
                        if ((*abs_val.borrow()) != 0) {
                            ((*sign.borrow()) + 1)
                        } else {
                            0
                        }
                    });
                    let __rhs = (({ ((1 - (2 * (*sign.borrow()))) * (*abs_val.borrow())) } + {
                        ({
                            PredictWithAdaptiveMedian_115(
                                (*coeffs.borrow()).clone(),
                                (*x.borrow()),
                                (*y.borrow()),
                                (*ac_stride.borrow()),
                            )
                        })
                    }) as i16);
                    elem!((*coeffs.borrow()), 0).write(__rhs);
                    (*block_state.borrow_mut()).postfix_inc();
                    (*coeffs.borrow_mut()) += kDCTBlockSize_3.with(|rc| *rc.borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                field!(ac_dc_state, next_x).write(0);
                {
                    (*iy.borrow_mut()).prefix_inc();
                    (*y.borrow_mut()).prefix_inc()
                };
            }
            field!(ac_dc_state, next_iy).write(0);
            (*i.borrow_mut()).prefix_inc();
        }
        field!(ac_dc_state, next_component).write(0_usize);
        (*mcu_y.borrow_mut()).prefix_inc();
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
    if !({ FinalizeSubdecoders_156((*state.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeEmptyAcBlock_158(prev_sgn: Ptr<i32>, prev_abs: Ptr<i32>) {
    let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(prev_sgn));
    let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(prev_abs));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*k.borrow()) < kDCTBlockSize_3.with(|rc| *rc.borrow())) {
        elem!((*prev_sgn.borrow()), (*k.borrow())).write(0);
        elem!((*prev_abs.borrow()), (*k.borrow())).write(0);
        (*k.borrow_mut()).prefix_inc();
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
    let c: Value<brunsli_AcBlockCookie> =
        Rc::new(RefCell::new((*cookie.upgrade().deref()).clone()));
    let ac: Value<brunsli_BinaryArithmeticDecoder> = Rc::new(RefCell::new(
        (*{ (*c.borrow()).ac.clone() }.upgrade().deref()).clone(),
    ));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new({ (*c.borrow()).in_.clone() }));
    let ans: Value<brunsli_ANSDecoder> = Rc::new(RefCell::new(
        (*{ (*c.borrow()).ans.clone() }.upgrade().deref()).clone(),
    ));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new(
        (*{ (*c.borrow()).br.clone() }.upgrade().deref()).clone(),
    ));
    let num_nonzeros: Value<usize> = Rc::new(RefCell::new(0_usize));
    let nonzero_ctx: Value<u8> = Rc::new(RefCell::new(
        ({
            let _prev: Ptr<u8> = { (*c.borrow()).prev_num_nonzeros.clone() };
            let _x: i32 = { (*c.borrow()).x };
            let _y: i32 = { (*c.borrow()).y };
            NumNonzerosContext_104(_prev, _x, _y)
        }),
    ));
    let last_nz: Value<usize> = Rc::new(RefCell::new(
        ({
            DecodeNumNonzeros_154(
                { (*c.borrow()).num_nonzero_prob.clone() }.offset(
                    ((kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                        .wrapping_mul(((*nonzero_ctx.borrow()) as usize)))
                        as isize,
                ),
                (ac.as_pointer()),
                (*in_.borrow()).clone(),
            )
        }),
    ));
    let k: Value<usize> = Rc::new(RefCell::new((*last_nz.borrow()).wrapping_add(1_usize)));
    'loop_: while ((*k.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
        elem!({ (*c.borrow()).prev_sgn.clone() }, (*k.borrow())).write(0);
        elem!({ (*c.borrow()).prev_abs.clone() }, (*k.borrow())).write(0);
        (*k.borrow_mut()).prefix_inc();
    }
    let k: Value<usize> = Rc::new(RefCell::new((*last_nz.borrow())));
    'loop_: while ((*k.borrow()) > 0_usize) {
        let is_zero: Value<i32> = Rc::new(RefCell::new(0));
        if ((*k.borrow()) < (*last_nz.borrow())) {
            let bucket: Value<usize> = Rc::new(RefCell::new(
                (({
                    let __idx = ((*num_nonzeros.borrow()).wrapping_sub(1_usize)) as usize;
                    kNonzeroBuckets_89.with(|rc| rc.borrow()[__idx])
                }) as usize),
            ));
            let is_zero_ctx: Value<usize> = Rc::new(RefCell::new(
                ((*bucket.borrow())
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                .wrapping_add((*k.borrow())),
            ));
            let p: Ptr<brunsli_Prob> =
                { (*c.borrow()).is_zero_prob.clone() }.offset((*is_zero_ctx.borrow()) as isize);
            (*is_zero.borrow_mut()) = ({
                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                    &ac.as_pointer(),
                    (({ brunsli_ProbImpl::get_proba(&p) }) as i32),
                    (*in_.borrow()).clone(),
                )
            });
            ({
                let _val: i32 = (*is_zero.borrow());
                brunsli_ProbImpl::Add(&p, _val)
            });
        }
        let abs_val: Value<i32> = Rc::new(RefCell::new(0));
        let sign: Value<i32> = Rc::new(RefCell::new(1));
        let k_nat: Value<i32> = Rc::new(RefCell::new(
            ((elem!({ (*c.borrow()).order.clone() }, (*k.borrow())).read()) as i32),
        ));
        if !((*is_zero.borrow()) != 0) {
            let context_type: Value<usize> = Rc::new(RefCell::new(
                ((elem!({ (*c.borrow()).context_modes.clone() }, (*k_nat.borrow())).read())
                    as usize),
            ));
            let avg_ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
            let sign_ctx: Value<usize> =
                Rc::new(RefCell::new(kMaxAverageContext_82.with(|rc| *rc.borrow())));
            if (((*context_type.borrow()) & 1_usize) != 0) && ({ (*c.borrow()).y } > 0) {
                let offset: Value<usize> =
                    Rc::new(RefCell::new((((*k_nat.borrow()) & 7) as usize)));
                ({
                    let _prev: Ptr<i16> = { (*c.borrow()).prev_row_coeffs.clone() }
                        .offset((*offset.borrow()) as isize);
                    let _cur: Ptr<i16> =
                        { (*c.borrow()).coeffs.clone() }.offset((*offset.borrow()) as isize);
                    let _mult: Ptr<i32> = { (*c.borrow()).mult_col.clone() }
                        .offset(((*offset.borrow()).wrapping_mul(8_usize)) as isize);
                    ACPredictContextRow_103(
                        _prev,
                        _cur,
                        _mult,
                        (avg_ctx.as_pointer()),
                        (sign_ctx.as_pointer()),
                    )
                });
            } else if (((*context_type.borrow()) & 2_usize) != 0) && ({ (*c.borrow()).x } > 0) {
                let offset: Value<usize> =
                    Rc::new(RefCell::new((((*k_nat.borrow()) & !7) as usize)));
                ({
                    let _prev: Ptr<i16> = { (*c.borrow()).prev_col_coeffs.clone() }
                        .offset((*offset.borrow()) as isize);
                    let _cur: Ptr<i16> =
                        { (*c.borrow()).coeffs.clone() }.offset((*offset.borrow()) as isize);
                    let _mult: Ptr<i32> =
                        { (*c.borrow()).mult_row.clone() }.offset((*offset.borrow()) as isize);
                    ACPredictContextCol_102(
                        _prev,
                        _cur,
                        _mult,
                        (avg_ctx.as_pointer()),
                        (sign_ctx.as_pointer()),
                    )
                });
            } else if !((*context_type.borrow()) != 0) {
                (*avg_ctx.borrow_mut()) = (({
                    let _vals: Ptr<i32> =
                        { (*c.borrow()).prev_abs.clone() }.offset((*k.borrow()) as isize);
                    let _prev_row_delta: i32 = { (*c.borrow()).prev_row_delta };
                    WeightedAverageContext_98(_vals, _prev_row_delta)
                }) as usize);
                (*sign_ctx.borrow_mut()) =
                    (({ ((elem!({ (*c.borrow()).prev_sgn.clone() }, (*k.borrow())).read()) * 3) }
                        + {
                            (elem!(
                                { (*c.borrow()).prev_sgn.clone() },
                                (((*k.borrow()) as i32) - kDCTBlockSize_3.with(|rc| *rc.borrow()))
                            )
                            .read())
                        }) as usize);
            }
            (*sign_ctx.borrow_mut()) = {
                ((*sign_ctx.borrow())
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                .wrapping_add((*k.borrow()))
            };
            let sign_p: Ptr<brunsli_Prob> =
                { (*c.borrow()).sign_prob.clone() }.offset((*sign_ctx.borrow()) as isize);
            (*sign.borrow_mut()) = ({
                brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                    &ac.as_pointer(),
                    (({ brunsli_ProbImpl::get_proba(&sign_p) }) as i32),
                    (*in_.borrow()).clone(),
                )
            });
            ({
                let _val: i32 = (*sign.borrow());
                brunsli_ProbImpl::Add(&sign_p, _val)
            });
            elem!({ (*c.borrow()).prev_sgn.clone() }, (*k.borrow()))
                .write({ ((*sign.borrow()) + 1) });
            (*sign.borrow_mut()) = { (1 - (2 * (*sign.borrow()))) };
            let z_dens_ctx: Value<usize> = Rc::new(RefCell::new(
                (({
                    ZeroDensityContext_96((*num_nonzeros.borrow()), (*k.borrow()), {
                        (*c.borrow()).context_bits
                    })
                }) as usize),
            ));
            let histo_ix: Value<usize> = Rc::new(RefCell::new(
                ((*z_dens_ctx.borrow()).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
                    .wrapping_add((*avg_ctx.borrow())),
            ));
            let entropy_ix: Value<usize> = Rc::new(RefCell::new(
                ((elem!({ (*c.borrow()).context_map.clone() }, (*histo_ix.borrow())).read())
                    as usize),
            ));
            let code: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _code: Ptr<brunsli_ANSDecodingData> =
                        { (*c.borrow()).entropy_codes.clone() }
                            .offset((*entropy_ix.borrow()) as isize);
                    let _in_: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                    brunsli_ANSDecoderImpl::ReadSymbol(&ans.as_pointer(), _code, _in_)
                }),
            ));
            if ((*code.borrow()) < kNumDirectCodes_135.with(|rc| *rc.borrow())) {
                (*abs_val.borrow_mut()) = ((*code.borrow()) + 1);
            } else {
                let nbits: Value<i32> = Rc::new(RefCell::new(
                    ((*code.borrow()) - kNumDirectCodes_135.with(|rc| *rc.borrow())),
                ));
                let p: Ptr<brunsli_Prob> = { (*c.borrow()).first_extra_bit_prob.clone() }.offset(
                    (((*k.borrow()).wrapping_mul(10_usize))
                        .wrapping_add(((*nbits.borrow()) as usize))) as isize,
                );
                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                    ({
                        brunsli_BinaryArithmeticDecoderImpl::ReadBit(
                            &ac.as_pointer(),
                            (({ brunsli_ProbImpl::get_proba(&p) }) as i32),
                            (*in_.borrow()).clone(),
                        )
                    }),
                ));
                ({
                    let _val: i32 = (*first_extra_bit.borrow());
                    brunsli_ProbImpl::Add(&p, _val)
                });
                let extra_bits_val: Value<i32> = Rc::new(RefCell::new(
                    ((*first_extra_bit.borrow()) << (*nbits.borrow())),
                ));
                if ((*nbits.borrow()) > 0) {
                    {
                        let rhs_0 = (((*extra_bits_val.borrow()) as u32)
                            | ({
                                brunsli_BitSourceImpl::ReadBits(
                                    &br.as_pointer(),
                                    (*nbits.borrow()),
                                    (*in_.borrow()).clone(),
                                )
                            })) as i32;
                        (*extra_bits_val.borrow_mut()) = rhs_0
                    };
                }
                (*abs_val.borrow_mut()) = (((((kNumDirectCodes_135.with(|rc| *rc.borrow()) - 1)
                    as u32)
                    .wrapping_add((2_u32 << (*nbits.borrow()))))
                .wrapping_add(((*extra_bits_val.borrow()) as u32)))
                    as i32);
            }
            (*num_nonzeros.borrow_mut()).prefix_inc();
        } else {
            elem!({ (*c.borrow()).prev_sgn.clone() }, (*k.borrow())).write(0);
        }
        let coeff: Value<i32> = Rc::new(RefCell::new(((*sign.borrow()) * (*abs_val.borrow()))));
        elem!({ (*c.borrow()).coeffs.clone() }, (*k_nat.borrow()))
            .write({ ((*coeff.borrow()) as i16) });
        elem!({ (*c.borrow()).prev_abs.clone() }, (*k.borrow())).write({ (*abs_val.borrow()) });
        (*k.borrow_mut()).prefix_dec();
    }
    { (*c.borrow()).ans.clone() }.write({ (*ans.borrow()).clone() });
    { (*c.borrow()).br.clone() }.write({ (*br.borrow()).clone() });
    { (*c.borrow()).ac.clone() }.write({ (*ac.borrow()).clone() });
    return (*num_nonzeros.borrow());
}
pub fn DecodeAC_160(
    state: Ptr<brunsli_internal_dec_State>,
    in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new(
        ({
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
        }),
    ));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> = field_ptr!(s, ac_dc);
    let comps: Ptr<Vec<brunsli_ComponentState>> = ac_dc_state.with(|__s| __s.ac.as_pointer());
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = (*num_components.borrow()) as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| {
                __v.resize_with(__a0, || <brunsli_ComponentState>::default())
            })
        };
        let c: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            ({
                let _w: i32 = {
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        (*c.borrow())
                    )
                    .upgrade()
                    .deref())
                    .width_in_blocks
                };
                brunsli_ComponentStateImpl::SetWidth(
                    &(Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                        as Ptr<brunsli_ComponentState>)
                        .offset((*c.borrow())),
                    _w,
                )
            });
            ({
                let _quant: Ptr<i32> = (({
                    (*elem!(
                        (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                            as Ptr<brunsli_internal_dec_ComponentMeta>),
                        (*c.borrow())
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
                        .offset((*c.borrow())),
                    mult_row
                ) as Ptr<i32>);
                let _mult_col: Ptr<i32> = (array_field_ptr!(
                    (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                        as Ptr<brunsli_ComponentState>)
                        .offset((*c.borrow())),
                    mult_col
                ) as Ptr<i32>);
                ComputeACPredictMultipliers_109(_quant, _mult_row, _mult_col)
            });
            (*c.borrow_mut()).prefix_inc();
        }
    }
    if !({ brunsli_WordSourceImpl::CanRead(&(*in_.borrow()), 5_usize) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({ EnsureSubdecodersInitialized_155((*state.borrow()).clone(), (*in_.borrow()).clone()) });
    if !(ac_dc_state.with(|__s| __s.ac_coeffs_order_decoded)) {
        'loop_: while ({ ac_dc_state.with(|__s| __s.next_component) } < {
            (*num_components.borrow())
        }) {
            if !({ brunsli_WordSourceImpl::CanRead(&(*in_.borrow()), 121_usize) }) {
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
                    (*in_.borrow()).clone(),
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
    (*c.borrow_mut()).in_ = (*in_.borrow()).clone();
    (*c.borrow_mut()).ans = (field_ptr!(s, ans_decoder));
    (*c.borrow_mut()).br = (field_ptr!(s, bit_reader));
    (*c.borrow_mut()).entropy_codes = (*state.borrow()).with(|__s| __s.entropy_codes.clone());
    (*c.borrow_mut()).context_modes = (kContextAlgorithm_95.with(|v| v.as_pointer()) as Ptr<u8>)
        .offset(
            (if (*state.borrow()).with(|__s| __s.use_legacy_context_model) {
                64
            } else {
                0
            }) as isize,
        );
    let mcu_y: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_mcu_y)));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_component)));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let cst: Ptr<brunsli_ComponentState> =
                (Ptr::<Vec<brunsli_ComponentState>>::decay(&(comps))
                    as Ptr<brunsli_ComponentState>)
                    .offset((*i.borrow()));
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
                    .offset((*i.borrow()));
            (*c.borrow_mut()).context_map = (*state.borrow())
                .with(|__s| __s.context_map.clone())
                .offset(
                    ((m.with(|__s| __s.context_offset))
                        .wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow())))
                        as isize,
                );
            (*c.borrow_mut()).context_bits = m.with(|__s| __s.context_bits);
            let width: Value<i32> = Rc::new(RefCell::new(m.with(|__s| __s.width_in_blocks)));
            let ac_stride: Value<usize> =
                Rc::new(RefCell::new((m.with(|__s| __s.ac_stride) as usize)));
            let b_stride: Value<usize> =
                Rc::new(RefCell::new((m.with(|__s| __s.b_stride) as usize)));
            let next_iy: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_iy)));
            (*c.borrow_mut()).y = ({ ({ (*mcu_y.borrow()) } * { m.with(|__s| __s.v_samp) }) } + {
                (*next_iy.borrow())
            });
            (*c.borrow_mut()).prev_row_delta = {
                (((((1_u32)
                    .wrapping_sub((2_u32).wrapping_mul((({ (*c.borrow()).y } as u32) & 1_u32))))
                .wrapping_mul((((*width.borrow()) + 3) as u32)))
                .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                    as i32)
            };
            let iy: Value<i32> = Rc::new(RefCell::new((*next_iy.borrow())));
            'loop_: while ({ (*iy.borrow()) } < { m.with(|__s| __s.v_samp) }) {
                let next_x: Value<i32> = Rc::new(RefCell::new(ac_dc_state.with(|__s| __s.next_x)));
                let block_offset: Value<usize> = Rc::new(RefCell::new(
                    (((*next_x.borrow()) * kDCTBlockSize_3.with(|rc| *rc.borrow())) as usize),
                ));
                (*c.borrow_mut()).coeffs = {
                    m.with(|__s| __s.ac_coeffs.clone())
                        .offset(
                            (({ (*c.borrow()).y } as usize).wrapping_mul((*ac_stride.borrow())))
                                as isize,
                        )
                        .offset((*block_offset.borrow()) as isize)
                };
                (*c.borrow_mut()).prev_row_coeffs =
                    { { (*c.borrow()).coeffs.clone() }.offset(-((*ac_stride.borrow()) as isize)) };
                (*c.borrow_mut()).prev_col_coeffs = {
                    { (*c.borrow()).coeffs.clone() }
                        .offset(-((kDCTBlockSize_3.with(|rc| *rc.borrow())) as isize))
                };
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    m.with(|__s| __s.block_state.clone())
                        .offset(
                            (({ (*c.borrow()).y } as usize).wrapping_mul((*b_stride.borrow())))
                                as isize,
                        )
                        .offset((*next_x.borrow()) as isize),
                ));
                (*c.borrow_mut()).prev_sgn = ((cst.with(|__s| __s.prev_sign.as_pointer())
                    as Ptr<i32>)
                    .offset((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)))
                .offset((*block_offset.borrow()) as isize);
                let __rhs = ((cst.with(|__s| __s.prev_abs_coeff.as_pointer()) as Ptr<i32>).offset(
                    (((((({ (*c.borrow()).y } as u32) & 1_u32)
                        .wrapping_mul((((*width.borrow()) + 3) as u32)))
                    .wrapping_add(2_u32))
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as u32)))
                        as usize),
                ))
                .offset((*block_offset.borrow()) as isize);
                (*c.borrow_mut()).prev_abs = __rhs;
                (*c.borrow_mut()).x = (*next_x.borrow());
                'loop_: while ({ (*c.borrow()).x } < (*width.borrow())) {
                    let is_empty: Value<bool> = Rc::new(RefCell::new(
                        ((((*block_state.borrow_mut()).postfix_inc()).read()) != 0),
                    ));
                    if !(*is_empty.borrow()) {
                        if ((!({ brunsli_WordSourceImpl::CanRead(&(*in_.borrow()), 297_usize) })
                            as i64)
                            != 0)
                        {
                            field!(ac_dc_state, next_mcu_y).write((*mcu_y.borrow()));
                            field!(ac_dc_state, next_component).write((*i.borrow()));
                            field!(ac_dc_state, next_iy).write((*iy.borrow()));
                            field!(ac_dc_state, next_x).write({ (*c.borrow()).x });
                            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let num_nonzeros: Value<usize> =
                            Rc::new(RefCell::new(({ DecodeAcBlock_159(c.as_pointer()) })));
                        if !((*num_nonzeros.borrow())
                            <= kNumNonZeroTreeSize_85.with(|rc| *rc.borrow()))
                        {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                    949,
                                    Ptr::<u8>::from_string_literal(b"DecodeAC"),
                                )
                            });
                            'loop_: while true {}
                        };
                        elem!({ (*c.borrow()).prev_num_nonzeros.clone() }, {
                            (*c.borrow()).x
                        })
                        .write({ ((*num_nonzeros.borrow()) as u8) });
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
                    (*iy.borrow_mut()).prefix_inc();
                    (*c.borrow_mut()).y.prefix_inc()
                };
            }
            field!(ac_dc_state, next_iy).write(0);
            (*i.borrow_mut()).prefix_inc();
        }
        field!(ac_dc_state, next_component).write(0_usize);
        (*mcu_y.borrow_mut()).prefix_inc();
    }
    field!(ac_dc_state, next_mcu_y).write(0);
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.clear());
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.shrink_to_fit());
    if !({ FinalizeSubdecoders_156((*state.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn CheckCanRead_161(state: Ptr<brunsli_internal_dec_State>, required: usize) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let required: Value<usize> = Rc::new(RefCell::new(required));
    let available: Value<usize> = Rc::new(RefCell::new(
        ((*state.borrow()).with(|__s| __s.len)).wrapping_sub((*state.borrow()).with(|__s| __s.pos)),
    ));
    return ((*required.borrow()) <= (*available.borrow()));
}
pub fn CheckCanReadByte_162(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return ({ (*state.borrow()).with(|__s| __s.pos) } != {
        (*state.borrow()).with(|__s| __s.len)
    });
}
pub fn ReadByte_163(state: Ptr<brunsli_internal_dec_State>) -> u8 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return (elem!(
        (*state.borrow()).with(|__s| __s.data.clone()),
        field!((*state.borrow()), pos).with_mut(|__v| __v.postfix_inc())
    )
    .read());
}
pub fn PeekByte_164(state: Ptr<brunsli_internal_dec_State>, offset: usize) -> u8 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    return (elem!(
        (*state.borrow()).with(|__s| __s.data.clone()),
        ((*state.borrow()).with(|__s| __s.pos)).wrapping_add((*offset.borrow()))
    )
    .read());
}
pub fn SkipBytes_165(state: Ptr<brunsli_internal_dec_State>, len: usize) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    field!((*state.borrow()), pos)
        .write({ ((*state.borrow()).with(|__s| __s.pos)).wrapping_add((*len.borrow())) });
}
pub fn GetBytesAvailable_166(state: Ptr<brunsli_internal_dec_State>) -> usize {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return ((*state.borrow()).with(|__s| __s.len))
        .wrapping_sub((*state.borrow()).with(|__s| __s.pos));
}
pub fn SkipAvailableBytes_167(state: Ptr<brunsli_internal_dec_State>, len: usize) -> usize {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let available: Value<usize> = Rc::new(RefCell::new(
        ({ GetBytesAvailable_166((*state.borrow()).clone()) }),
    ));
    let skip_bytes: Value<usize> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*len.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    ));
    field!((*state.borrow()), pos)
        .write({ ((*state.borrow()).with(|__s| __s.pos)).wrapping_add((*skip_bytes.borrow())) });
    return (*skip_bytes.borrow());
}
pub fn DecodeBase128_168(
    state: Ptr<brunsli_internal_dec_State>,
    val: Ptr<usize>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let val: Value<Ptr<usize>> = Rc::new(RefCell::new(val));
    (*val.borrow()).write(0_usize);
    let b: Value<u64> = Rc::new(RefCell::new(128_u64));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < 9_usize) && (((*b.borrow()) & 128_u64) != 0) {
        if !({
            CheckCanRead_161(
                (*state.borrow()).clone(),
                (*i.borrow()).wrapping_add(1_usize),
            )
        }) {
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
        (*b.borrow_mut()) = (({ PeekByte_164((*state.borrow()).clone(), (*i.borrow())) }) as u64);
        (*val.borrow()).write({
            ((((*val.borrow()).read()) as u64)
                | (((*b.borrow()) & 127_u64) << ((*i.borrow()).wrapping_mul(7_usize))))
                as usize
        });
        (*i.borrow_mut()).prefix_inc();
    }
    ({ SkipBytes_165((*state.borrow()).clone(), (*i.borrow())) });
    return if (((*b.borrow()) & 128_u64) == 0_u64) {
        brunsli_BrunsliStatus_BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
    };
}
pub fn Fail_169(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    field!(s, result).write((*result.borrow()));
    field!(s, last_stage).write((*state.borrow()).with(|__s| __s.stage));
    return brunsli_internal_dec_Stage_ERROR;
}
pub fn ReadTag_170(
    state: Ptr<brunsli_internal_dec_State>,
    section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    if !({ CheckCanReadByte_162((*state.borrow()).clone()) }) {
        return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
    }
    let marker: Value<u8> = Rc::new(RefCell::new(({ ReadByte_163((*state.borrow()).clone()) })));
    let tag: Value<usize> = Rc::new(RefCell::new(
        ((((*marker.borrow()) as i32) >> 3_u32) as usize),
    ));
    if ((*tag.borrow()) == 0_usize) || ((*tag.borrow()) > 15_usize) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    field!((*section.borrow()), tag).write((*tag.borrow()));
    let wiring_type: Value<usize> = Rc::new(RefCell::new(
        ((((*marker.borrow()) as u32) & 7_u32) as usize),
    ));
    if ((*wiring_type.borrow()) != (kBrunsliWiringTypeVarint_25.with(|rc| *rc.borrow()) as usize))
        && ((*wiring_type.borrow())
            != (kBrunsliWiringTypeLengthDelimited_26.with(|rc| *rc.borrow()) as usize))
    {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    field!((*section.borrow()), is_section).write(
        ((*wiring_type.borrow())
            == (kBrunsliWiringTypeLengthDelimited_26.with(|rc| *rc.borrow()) as usize)),
    );
    let tag_bit: Value<u32> = Rc::new(RefCell::new((1_u32 << (*tag.borrow()))));
    if (({ (*section.borrow()).with(|__s| __s.tags_met) } & { (*tag_bit.borrow()) }) != 0) {
        write!(
            libcc2rs::cerr(),
            "Duplicate marker {:x}\n",
            ((*marker.borrow()) as i32),
        );
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    {
        let _ptr = field!((*section.borrow()), tags_met);
        _ptr.write(_ptr.read() | (*tag_bit.borrow()))
    };
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn EnterSection_171(
    state: Ptr<brunsli_internal_dec_State>,
    section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    let section_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({ DecodeBase128_168((*state.borrow()).clone(), (section_size.as_pointer())) }),
    ));
    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
        return (*status.borrow());
    }
    field!((*section.borrow()), is_active).write(true);
    field!((*section.borrow()), remaining).write((*section_size.borrow()));
    field!((*section.borrow()), milestone).write((*state.borrow()).with(|__s| __s.pos));
    field!((*section.borrow()), projected_end).write({
        ((*state.borrow()).with(|__s| __s.pos))
            .wrapping_add((*section.borrow()).with(|__s| __s.remaining))
    });
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn LeaveSection_172(section: Ptr<brunsli_internal_dec_SectionState>) {
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    field!((*section.borrow()), is_active).write(false);
}
pub fn IsOutOfSectionBounds_173(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return ({ (*state.borrow()).with(|__s| __s.pos) } > {
        {
            (*(*state.borrow())
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .projected_end
        }
    });
}
pub fn RemainingSectionLength_174(state: Ptr<brunsli_internal_dec_State>) -> usize {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    if ({ IsOutOfSectionBounds_173((*state.borrow()).clone()) }) {
        return 0_usize;
    }
    return ({
        (*(*state.borrow())
            .with(|__s| __s.internal.clone())
            .as_ref()
            .unwrap()
            .borrow())
        .section
        .projected_end
    })
    .wrapping_sub((*state.borrow()).with(|__s| __s.pos));
}
pub fn IsAtSectionBoundary_175(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return ({ (*state.borrow()).with(|__s| __s.pos) } == {
        {
            (*(*state.borrow())
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .projected_end
        }
    });
}
pub fn VerifySignature_176(state: Ptr<brunsli_internal_dec_State>) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    if !({
        CheckCanRead_161(
            (*state.borrow()).clone(),
            kBrunsliSignatureSize_43.with(|rc| *rc.borrow()),
        )
    }) {
        return ({
            Fail_169(
                (*state.borrow()).clone(),
                brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA,
            )
        });
    }
    let is_signature_ok: Value<bool> = Rc::new(RefCell::new(
        (((*state.borrow())
            .with(|__s| __s.data.clone())
            .offset(((*state.borrow()).with(|__s| __s.pos)) as isize) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((kBrunsliSignature_44.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any(),
                kBrunsliSignatureSize_43.with(|rc| *rc.borrow()),
            )
            != 0),
    ));
    field!((*state.borrow()), pos).write({
        ((*state.borrow()).with(|__s| __s.pos))
            .wrapping_add(kBrunsliSignatureSize_43.with(|rc| *rc.borrow()))
    });
    {
        let _ptr = field!(field!(s, section), tags_met);
        _ptr.write(
            _ptr.read() | (1_u32 << (kBrunsliSignatureTag_30.with(|rc| *rc.borrow()) as i32)),
        )
    };
    if (*is_signature_ok.borrow()) {
        return ({
            Fail_169(
                (*state.borrow()).clone(),
                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
            )
        });
    }
    return brunsli_internal_dec_Stage_HEADER;
}
pub fn DecodeHeader_177(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let hs: Ptr<brunsli_internal_dec_HeaderState> = field_ptr!(s, header);
    'loop_: while (hs.with(|__s| __s.stage)
        != (brunsli_internal_dec_HeaderState_Stage_DONE as usize))
    {
        'switch: {
            match { hs.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_READ_TAG as usize) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ ReadTag_170((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    if ({ s.with(|__s| __s.section.tag) } != {
                        (kBrunsliHeaderTag_31.with(|rc| *rc.borrow()) as usize)
                    }) || (!(s.with(|__s| __s.section.is_section)))
                    {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ENTER_SECTION as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ENTER_SECTION as usize) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ EnterSection_171((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize) => {
                    if ({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
                        field!(hs, stage)
                            .write((brunsli_internal_dec_HeaderState_Stage_FINALE as usize));
                        break 'switch;
                    }
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ ReadTag_170((*state.borrow()).clone(), (field_ptr!(hs, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    let tag_bit: Value<u32> =
                        Rc::new(RefCell::new((1_u32 << hs.with(|__s| __s.section.tag))));
                    if hs.with(|__s| __s.section.is_section) {
                        if ((kKnownHeaderVarintTags_138.with(|rc| *rc.borrow())
                            & (*tag_bit.borrow()))
                            != 0)
                        {
                            ({
                                Fail_169(
                                    (*state.borrow()).clone(),
                                    brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                                )
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
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            DecodeBase128_168(
                                (*state.borrow()).clone(),
                                (field_ptr!(hs, remaining_skip_length)),
                            )
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    field!(hs, stage).write(
                        (brunsli_internal_dec_HeaderState_Stage_ITEM_SKIP_CONTENTS as usize),
                    );
                    break 'switch;
                }
                __v if __v
                    == (brunsli_internal_dec_HeaderState_Stage_ITEM_SKIP_CONTENTS as usize) =>
                {
                    let bytes_skipped: Value<usize> = Rc::new(RefCell::new(
                        ({
                            SkipAvailableBytes_167(
                                (*state.borrow()).clone(),
                                hs.with(|__s| __s.remaining_skip_length),
                            )
                        }),
                    ));
                    field!(hs, remaining_skip_length).write({
                        (hs.with(|__s| __s.remaining_skip_length))
                            .wrapping_sub((*bytes_skipped.borrow()))
                    });
                    if (hs.with(|__s| __s.remaining_skip_length) > 0_usize) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA,
                            )
                        });
                    }
                    field!(hs, stage)
                        .write((brunsli_internal_dec_HeaderState_Stage_ITEM_READ_TAG as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_HeaderState_Stage_ITEM_READ_VALUE as usize) => {
                    let value: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ DecodeBase128_168((*state.borrow()).clone(), (value.as_pointer())) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
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
                    let has_version: Value<bool> = Rc::new(RefCell::new(
                        (({ hs.with(|__s| __s.section.tags_met) } & {
                            (1_u32
                                << (kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow()) as i32))
                        }) != 0),
                    ));
                    if !(*has_version.borrow()) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    let version_and_comp_count: Value<usize> = Rc::new(RefCell::new(
                        ((elem!(
                            (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                            (kBrunsliHeaderVersionCompTag_41.with(|rc| *rc.borrow()) as usize)
                        )
                        .read()) as usize),
                    ));
                    let version: Value<usize> =
                        Rc::new(RefCell::new(((*version_and_comp_count.borrow()) >> 2_u32)));
                    field!((*jpg.borrow()), version).write(((*version.borrow()) as i32));
                    if ((*version.borrow()) == 1_usize) {
                        field!((*jpg.borrow()), width).write(0);
                        field!((*jpg.borrow()), height).write(0);
                        field!(hs, stage)
                            .write((brunsli_internal_dec_HeaderState_Stage_DONE as usize));
                        break 'switch;
                    }
                    if (((*version.borrow()) & 1_usize) != 0_usize) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    if (((*version.borrow()) & (!7_u32 as usize)) != 0_usize) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    field!((*state.borrow()), use_legacy_context_model)
                        .write(!(((*version.borrow()) & 2_usize) != 0));
                    {
                        let _ptr = field!(field!(s, section), tags_met);
                        _ptr.write(
                            _ptr.read()
                                | (1_u32
                                    << (kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow()) as i32)),
                        )
                    };
                    let has_width: Value<bool> = Rc::new(RefCell::new(
                        (({ hs.with(|__s| __s.section.tags_met) } & {
                            (1_u32 << (kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow()) as i32))
                        }) != 0),
                    ));
                    if !(*has_width.borrow()) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    let width: Value<usize> = Rc::new(RefCell::new(
                        ((elem!(
                            (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                            (kBrunsliHeaderWidthTag_39.with(|rc| *rc.borrow()) as usize)
                        )
                        .read()) as usize),
                    ));
                    let has_height: Value<bool> = Rc::new(RefCell::new(
                        (({ hs.with(|__s| __s.section.tags_met) } & {
                            (1_u32 << (kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow()) as i32))
                        }) != 0),
                    ));
                    if !(*has_height.borrow()) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    let height: Value<usize> = Rc::new(RefCell::new(
                        ((elem!(
                            (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                            (kBrunsliHeaderHeightTag_40.with(|rc| *rc.borrow()) as usize)
                        )
                        .read()) as usize),
                    ));
                    if ((*width.borrow()) == 0_usize) || ((*height.borrow()) == 0_usize) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    if ((*width.borrow()) > (kMaxDimPixels_11.with(|rc| *rc.borrow()) as usize))
                        || ((*height.borrow())
                            > (kMaxDimPixels_11.with(|rc| *rc.borrow()) as usize))
                    {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    field!((*jpg.borrow()), width).write(((*width.borrow()) as i32));
                    field!((*jpg.borrow()), height).write(((*height.borrow()) as i32));
                    let num_components: Value<usize> = Rc::new(RefCell::new(
                        ((*version_and_comp_count.borrow()) & 3_usize).wrapping_add(1_usize),
                    ));
                    {
                        let __a0 = (*num_components.borrow()) as usize;
                        (*(*jpg.borrow())
                            .with(|__s| __s.components.clone())
                            .borrow_mut())
                        .resize_with(__a0, || <brunsli_JPEGComponent>::default())
                    };
                    let has_subsampling: Value<bool> = Rc::new(RefCell::new(
                        (({ hs.with(|__s| __s.section.tags_met) } & {
                            (1_u32
                                << (kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow()) as i32))
                        }) != 0),
                    ));
                    if !(*has_subsampling.borrow()) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    let subsampling_code: Value<usize> = Rc::new(RefCell::new(
                        ((elem!(
                            (hs.with(|__s| __s.varint_values.as_pointer()) as Ptr<u64>),
                            (kBrunsliHeaderSubsamplingTag_42.with(|rc| *rc.borrow()) as usize)
                        )
                        .read()) as usize),
                    ));
                    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
                    'loop_: while ({ (*i.borrow()) } < {
                        (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
                    }) {
                        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
                            (((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                                as Ptr<brunsli_JPEGComponent>)
                                .offset((*i.borrow()))),
                        ));
                        field!((*c.borrow()), v_samp_factor).write(
                            ((((*subsampling_code.borrow()) & 15_usize).wrapping_add(1_usize))
                                as i32),
                        );
                        (*subsampling_code.borrow_mut()) >>= 4_u32;
                        field!((*c.borrow()), h_samp_factor).write(
                            ((((*subsampling_code.borrow()) & 15_usize).wrapping_add(1_usize))
                                as i32),
                        );
                        (*subsampling_code.borrow_mut()) >>= 4_u32;
                        if ({ (*c.borrow()).with(|__s| __s.v_samp_factor) } > {
                            kBrunsliMaxSampling_27.with(|rc| *rc.borrow())
                        }) {
                            return ({
                                Fail_169(
                                    (*state.borrow()).clone(),
                                    brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                                )
                            });
                        }
                        if ({ (*c.borrow()).with(|__s| __s.h_samp_factor) } > {
                            kBrunsliMaxSampling_27.with(|rc| *rc.borrow())
                        }) {
                            return ({
                                Fail_169(
                                    (*state.borrow()).clone(),
                                    brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                                )
                            });
                        }
                        (*i.borrow_mut()).prefix_inc();
                    }
                    if !({ UpdateSubsamplingDerivatives_178((*jpg.borrow()).clone()) }) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    ({ PrepareMeta_179((*jpg.borrow()).clone(), (*state.borrow()).clone()) });
                    field!(hs, stage).write((brunsli_internal_dec_HeaderState_Stage_DONE as usize));
                    break 'switch;
                }
                _ => {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
                        )
                    });
                }
            }
        };
    }
    ({ LeaveSection_172((field_ptr!(s, section))) });
    return if ({ (*jpg.borrow()).with(|__s| __s.version) } == {
        kFallbackVersion_2.with(|rc| *rc.borrow())
    }) {
        brunsli_internal_dec_Stage_FALLBACK
    } else {
        brunsli_internal_dec_Stage_SECTION
    };
}
pub fn DecodeMetaDataSection_180(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let ms: Ptr<brunsli_internal_dec_MetadataState> = field_ptr!(s, metadata);
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_DONE)
    {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if (ms.with(|__s| __s.decompression_stage)
        == brunsli_internal_dec_MetadataDecompressionStage_INITIAL)
    {
        if ({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
            field!(ms, decompression_stage)
                .write(brunsli_internal_dec_MetadataDecompressionStage_DONE);
            return brunsli_BrunsliStatus_BRUNSLI_OK;
        }
        if (({ RemainingSectionLength_174((*state.borrow()).clone()) }) == 1_usize) {
            if !({ CheckCanReadByte_162((*state.borrow()).clone()) }) {
                return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
            }
            let data: Value<Box<[u8]>> =
                Rc::new(RefCell::new((0..1).map(|_| 0_u8).collect::<Box<[u8]>>()));
            (*data.borrow_mut())[(0) as usize] = ({ ReadByte_163((*state.borrow()).clone()) });
            let ok: Value<bool> = Rc::new(RefCell::new(
                ({
                    ProcessMetaData_149(
                        (data.as_pointer() as Ptr<u8>),
                        1_usize,
                        (ms).clone(),
                        (*jpg.borrow()).clone(),
                    )
                }) && ({ brunsli_internal_dec_MetadataStateImpl::CanFinish(&ms) }),
            ));
            field!(ms, decompression_stage)
                .write(brunsli_internal_dec_MetadataDecompressionStage_DONE);
            return if (*ok.borrow()) {
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
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({ DecodeBase128_168((*state.borrow()).clone(), (field_ptr!(ms, metadata_size))) }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return (*status.borrow());
        }
        if ({ IsOutOfSectionBounds_173((*state.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
        if (({ RemainingSectionLength_174((*state.borrow()).clone()) }) == 0_usize) {
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
                    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
                    if !(!((ms.with(|__s| __s.brotli.clone())).is_null())) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                1312,
                                Ptr::<u8>::from_string_literal(b"operator()"),
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
                    return (*result.borrow());
                }
            )));
        'loop_: while true {
            let available_bytes: Value<usize> = Rc::new(RefCell::new(
                ({
                    let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                        (({ GetBytesAvailable_166((*state.borrow()).clone()) }) as u64),
                    ));
                    let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                        (({ RemainingSectionLength_174((*state.borrow()).clone()) }) as u64),
                    ));
                    (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                        __tmp_0.as_pointer()
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                } as usize),
            ));
            let available_in: Value<usize> = Rc::new(RefCell::new((*available_bytes.borrow())));
            let next_in: Value<Ptr<u8>> = Rc::new(RefCell::new(
                (*state.borrow())
                    .with(|__s| __s.data.clone())
                    .offset(((*state.borrow()).with(|__s| __s.pos)) as isize),
            ));
            let available_out: Value<usize> = Rc::new(RefCell::new(0_usize));
            let result: Value<::brotli_sys::BrotliDecoderResult> = Rc::new(RefCell::new(unsafe {
                let _a2: Ptr<*const u8> = Ptr::alloc(
                    (&*(*(next_in.as_pointer()).upgrade().deref())
                        .upgrade()
                        .deref()) as *const u8,
                );

                (available_in.as_pointer()).with_mut(|_v1| {
                    _a2.with_mut(|_v2| {
                        (available_out.as_pointer()).with_mut(|_v3| {
                            ::brotli_sys::BrotliDecoderDecompressStream(
                                ms.with(|__s| __s.brotli.clone()),
                                _v1 as *mut usize,
                                _v2 as *mut *const u8,
                                _v3 as *mut usize,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                            )
                        })
                    })
                })
            }));
            if (((*result.borrow()) as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_ERROR as i32)) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let chunk_size: Value<usize> = Rc::new(RefCell::new(0_usize));
            let chunk_data: Value<Ptr<u8>> = Rc::new(RefCell::new(unsafe {
                (chunk_size.as_pointer()).with_mut(|_v1| {
                    let output: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(
                        ms.with(|__s| __s.brotli.clone()),
                        _v1 as *mut usize,
                    );
                    let slice = std::slice::from_raw_parts(output, *_v1);
                    let result: Ptr<Vec<u8>> = Ptr::alloc(slice.to_vec());
                    result.decay()
                })
            }));
            field!(ms, decompressed_size).write({
                (ms.with(|__s| __s.decompressed_size)).wrapping_add((*chunk_size.borrow()))
            });
            if ({ ms.with(|__s| __s.decompressed_size) } > { ms.with(|__s| __s.metadata_size) }) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let consumed_bytes: Value<usize> = Rc::new(RefCell::new(
                (*available_bytes.borrow()).wrapping_sub((*available_in.borrow())),
            ));
            ({ SkipBytes_165((*state.borrow()).clone(), (*consumed_bytes.borrow())) });
            let chunk_ok: Value<bool> = Rc::new(RefCell::new(
                ({
                    ProcessMetaData_149(
                        (*chunk_data.borrow()).clone(),
                        (*chunk_size.borrow()),
                        (ms).clone(),
                        (*jpg.borrow()).clone(),
                    )
                }),
            ));
            if !(*chunk_ok.borrow()) {
                return ({
                    (*finish_decompression.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            if (((*result.borrow()) as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32))
            {
                if (({ RemainingSectionLength_174((*state.borrow()).clone()) }) != 0_usize) {
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
            if (((*result.borrow()) as i32)
                == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as i32))
            {
                continue 'loop_;
            }
            if !(((*result.borrow()) as i32)
                == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as i32))
            {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                        1352,
                        Ptr::<u8>::from_string_literal(b"DecodeMetaDataSection"),
                    )
                });
                'loop_: while true {}
            };
            if (({ RemainingSectionLength_174((*state.borrow()).clone()) }) == 0_usize) {
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
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1361,
                Ptr::<u8>::from_string_literal(b"DecodeMetaDataSection"),
            )
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR;
}
pub fn CheckBoundary_181(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    if (((*result.borrow()) as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        let last: Value<bool> = Rc::new(RefCell::new(
            ({ ({ RemainingSectionLength_174((*state.borrow()).clone()) }) } <= {
                ({ GetBytesAvailable_166((*state.borrow()).clone()) })
            }),
        ));
        return if (*last.borrow()) {
            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
        } else {
            brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA
        };
    } else {
        return (*result.borrow());
    }
    panic!("ub: non-void function does not return a value")
}
pub fn PrepareBitReader_182(
    br: Ptr<brunsli_BrunsliBitReader>,
    state: Ptr<brunsli_internal_dec_State>,
) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let chunk_len: Value<usize> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                (({ GetBytesAvailable_166((*state.borrow()).clone()) }) as u64),
            ));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                (({ RemainingSectionLength_174((*state.borrow()).clone()) }) as u64),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    ));
    ({
        BrunsliBitReaderResume_128(
            (*br.borrow()).clone(),
            (*state.borrow())
                .with(|__s| __s.data.clone())
                .offset(((*state.borrow()).with(|__s| __s.pos)) as isize),
            (*chunk_len.borrow()),
        )
    });
    if !({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1384,
                Ptr::<u8>::from_string_literal(b"PrepareBitReader"),
            )
        });
        'loop_: while true {}
    };
}
pub fn SuspendBitReader_183(
    br: Ptr<brunsli_BrunsliBitReader>,
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    let chunk_len: Value<usize> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                (({ GetBytesAvailable_166((*state.borrow()).clone()) }) as u64),
            ));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                (({ RemainingSectionLength_174((*state.borrow()).clone()) }) as u64),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    ));
    let unused_bytes: Value<usize> = Rc::new(RefCell::new(
        ({ BrunsliBitReaderSuspend_130((*br.borrow()).clone()) }),
    ));
    let consumed_bytes: Value<usize> = Rc::new(RefCell::new(
        (*chunk_len.borrow()).wrapping_sub((*unused_bytes.borrow())),
    ));
    ({ SkipBytes_165((*state.borrow()).clone(), (*consumed_bytes.borrow())) });
    let __rhs = ({ CheckBoundary_181((*state.borrow()).clone(), (*result.borrow())) });
    (*result.borrow_mut()) = __rhs;
    if !(({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) })
        || ((((*result.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32))
            && (((*result.borrow()) as i32)
                != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1401,
                Ptr::<u8>::from_string_literal(b"SuspendBitReader"),
            )
        });
        'loop_: while true {}
    };
    return (*result.borrow());
}
pub fn DecodeJPEGInternalsSection_184(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
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
                let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
                return ({
                    SuspendBitReader_183(
                        (br.read()).clone(),
                        (state.read()).clone(),
                        (*result.borrow()),
                    )
                });
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
                (*(*jpg.borrow())
                    .with(|__s| __s.marker_order.clone())
                    .borrow_mut())
                .push(a0_clone)
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
            field!((*jpg.borrow()), restart_interval).write(__rhs);
        }
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_HUFFMAN_LAST);
    }
    if (((js.with(|__s| __s.stage) as i32)
        & (brunsli_internal_dec_JpegInternalsState_Stage_DECODE_HUFFMAN_MASK as i32))
        != 0)
    {
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({ DecodeHuffmanCode_150((*state.borrow()).clone(), (*jpg.borrow()).clone()) }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call((*status.borrow())) });
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
                (*(*jpg.borrow())
                    .with(|__s| __s.scan_info.clone())
                    .borrow_mut())
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
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({ DecodeScanInfo_151((*state.borrow()).clone(), (*jpg.borrow()).clone()) }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call((*status.borrow())) });
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
        let num_quant_tables: Value<i32> = Rc::new(RefCell::new(
            ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }).wrapping_add(1_u32))
                as i32),
        ));
        {
            let __a0 = ((*num_quant_tables.borrow()) as usize) as usize;
            (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow_mut())
                .resize_with(__a0, || <brunsli_JPEGQuantTable>::default())
        };
        field!(js, i).write(0_usize);
        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_READ_QUANT);
    }
    'loop_: while ((js.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage_READ_QUANT as i32))
    {
        if ({ js.with(|__s| __s.i) } >= {
            (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len()
        }) {
            field!(js, stage)
                .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_COMP_ID_SCHEME);
            break;
        }
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 7_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let q: Value<Ptr<brunsli_JPEGQuantTable>> = Rc::new(RefCell::new(
            (((*jpg.borrow()).with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>)
                .offset(js.with(|__s| __s.i))),
        ));
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as i32);
        field!((*q.borrow()), index).write(__rhs);
        let __rhs = ({ js.with(|__s| __s.i) } == {
            ((*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len()).wrapping_sub(1_usize)
        }) || (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0);
        field!((*q.borrow()), is_last).write(__rhs);
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 4_u32) }) as i32);
        field!((*q.borrow()), precision).write(__rhs);
        if ((*q.borrow()).with(|__s| __s.precision) > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quantization table precision: {:}\n",
                (*q.borrow()).with(|__s| __s.precision),
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
        let comp_ids: Value<i32> = Rc::new(RefCell::new(
            (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as i32),
        ));
        thread_local!(
            static kMinRequiredComponents_185: Value<Box<[usize]>> =
                Rc::new(RefCell::new(Box::new([3_usize, 1_usize, 3_usize, 0_usize])));
        );
        if ({ (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len() } < {
            ({
                let __idx = (*comp_ids.borrow()) as usize;
                kMinRequiredComponents_185.with(|rc| rc.borrow()[__idx])
            })
        }) {
            write!(
                libcc2rs::cerr(),
                "Insufficient number of components for ColorId #{:}\n",
                (*comp_ids.borrow()),
            );
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
            });
        }
        field!(js, stage)
            .write(brunsli_internal_dec_JpegInternalsState_Stage_READ_NUM_PADDING_BITS);
        if ((*comp_ids.borrow()) == kComponentIds123_49.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write(1);
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    1_usize
                ),
                id
            )
            .write(2);
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    2_usize
                ),
                id
            )
            .write(3);
        } else if ((*comp_ids.borrow()) == kComponentIdsGray_50.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write(1);
        } else if ((*comp_ids.borrow()) == kComponentIdsRGB_51.with(|rc| *rc.borrow())) {
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    0_usize
                ),
                id
            )
            .write((('R' as u8) as i32));
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    1_usize
                ),
                id
            )
            .write((('G' as u8) as i32));
            field!(
                elem!(
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
                    2_usize
                ),
                id
            )
            .write((('B' as u8) as i32));
        } else {
            if !((*comp_ids.borrow()) == kComponentIdsCustom_52.with(|rc| *rc.borrow())) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                        1529,
                        Ptr::<u8>::from_string_literal(b"DecodeJPEGInternalsSection"),
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
            (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
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
                    ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                        as Ptr<brunsli_JPEGComponent>),
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
        field!((*jpg.borrow()), has_zero_padding_bit)
            .write((js.with(|__s| __s.num_padding_bits) > 0_usize));
        if ({ js.with(|__s| __s.num_padding_bits) } > {
            (({ PaddingBitsLimit_17((*jpg.borrow()).clone()) }) as usize)
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
                (*(*jpg.borrow())
                    .with(|__s| __s.padding_bits.clone())
                    .borrow_mut())
                .push(__init)
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
                        (*(*jpg.borrow())
                            .with(|__s| __s.marker_order.clone())
                            .borrow())
                        .len()
                    }) {
                        field!(js, stage).write(brunsli_internal_dec_JpegInternalsState_Stage_DONE);
                    } else if (((elem!(
                        ((*jpg.borrow()).with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
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
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            DecodeBase128_168(
                                (*state.borrow()).clone(),
                                (field_ptr!(js, intermarker_length)),
                            )
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({
                            CheckBoundary_181((*state.borrow()).clone(), (*status.borrow()))
                        });
                    }
                    if ({ js.with(|__s| __s.intermarker_length) } > {
                        ({ RemainingSectionLength_174((*state.borrow()).clone()) })
                    }) {
                        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                    }
                    {
                        let __init = Vec::new();
                        ((*jpg.borrow()).with(|__s| __s.inter_marker_data.as_pointer())
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
                    let dest: Ptr<Vec<u8>> = (*(*jpg.borrow())
                        .with(|__s| __s.inter_marker_data.clone())
                        .borrow())[(*(*jpg.borrow())
                        .with(|__s| __s.inter_marker_data.clone())
                        .borrow())
                    .len()
                        - 1]
                    .as_pointer();
                    let piece_limit: Value<usize> = Rc::new(RefCell::new(
                        ((js.with(|__s| __s.intermarker_length) as u64)
                            .wrapping_sub(((*dest.upgrade().deref()).len() as u64))
                            as usize),
                    ));
                    let piece_size: Value<usize> = Rc::new(RefCell::new(
                        ({
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
                        } as usize),
                    ));
                    ({
                        Append_72(
                            (dest).clone(),
                            (*state.borrow())
                                .with(|__s| __s.data.clone())
                                .offset(((*state.borrow()).with(|__s| __s.pos)) as isize),
                            (*piece_size.borrow()),
                        )
                    });
                    ({ SkipBytes_165((*state.borrow()).clone(), (*piece_size.borrow())) });
                    if ({ (*dest.upgrade().deref()).len() } < {
                        js.with(|__s| __s.intermarker_length)
                    }) {
                        if !(({ GetBytesAvailable_166((*state.borrow()).clone()) }) == 0_usize) {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                    1613,
                                    Ptr::<u8>::from_string_literal(b"DecodeJPEGInternalsSection"),
                                )
                            });
                            'loop_: while true {}
                        };
                        if !(({ RemainingSectionLength_174((*state.borrow()).clone()) }) > 0_usize)
                        {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                    1614,
                                    Ptr::<u8>::from_string_literal(b"DecodeJPEGInternalsSection"),
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
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
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
                let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
                return ({
                    SuspendBitReader_183(
                        (br.read()).clone(),
                        (state.read()).clone(),
                        (*result.borrow()),
                    )
                });
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
        let num_quant_tables: Value<usize> = Rc::new(RefCell::new(
            ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }).wrapping_add(1_u32))
                as usize),
        ));
        if ({ (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len() } != {
            (*num_quant_tables.borrow())
        }) {
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
                        (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len()
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
                    let is_short: Value<bool> = Rc::new(RefCell::new(
                        !(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0),
                    ));
                    if (*is_short.borrow()) {
                        let short_code: Value<usize> = Rc::new(RefCell::new(
                            (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 3_u32) })
                                as usize),
                        ));
                        let table: Value<Ptr<i32>> = Rc::new(RefCell::new(
                            ({
                                (*elem!(
                                    ((*jpg.borrow()).with(|__s| __s.quant.as_pointer())
                                        as Ptr<brunsli_JPEGQuantTable>),
                                    qs.with(|__s| __s.i)
                                )
                                .upgrade()
                                .deref())
                                .values
                                .as_pointer()
                            } as Ptr<i32>),
                        ));
                        let selector: Value<usize> = Rc::new(RefCell::new(
                            (if (qs.with(|__s| __s.i) > 0_usize) {
                                1
                            } else {
                                0
                            } as usize),
                        ));
                        let k: Value<usize> = Rc::new(RefCell::new(0_usize));
                        'loop_: while ((*k.borrow())
                            < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
                        {
                            elem!((*table.borrow()), (*k.borrow())).write({
                                (({
                                    let __idx = (*selector.borrow()) as usize;
                                    kStockQuantizationTables_48
                                        .with(|rc| rc.borrow()[__idx].clone())
                                })
                                .borrow()[(*short_code.borrow()) as usize]
                                    .borrow()[(*k.borrow()) as usize]
                                    as i32)
                            });
                            (*k.borrow_mut()).prefix_inc();
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
                    let q_factor: Value<u32> = Rc::new(RefCell::new(
                        ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 6_u32) }),
                    ));
                    ({
                        let _is_chroma: bool = (qs.with(|__s| __s.i) > 0_usize);
                        let _dst: Ptr<u8> = (qs.with(|__s| __s.predictor.as_pointer()) as Ptr<u8>);
                        FillQuantMatrix_118(_is_chroma, (*q_factor.borrow()), _dst)
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
                    let diff: Value<i32> =
                        Rc::new(RefCell::new(((qs.with(|__s| __s.vs.value) as i32) + 1)));
                    {
                        let _ptr = field!(qs, delta);
                        _ptr.write(
                            _ptr.read() + { ({ qs.with(|__s| __s.sign) } * { (*diff.borrow()) }) },
                        )
                    };
                    field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF);
                    continue 'loop_;
                }
            }
            __v if __v == (brunsli_internal_dec_QuantDataState_Stage_APPLY_DIFF as i32) => {
                {
                    let k: Value<i32> = Rc::new(RefCell::new(
                        (({
                            let __idx = (qs.with(|__s| __s.j)) as usize;
                            kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                        }) as i32),
                    ));
                    let quant_value: Value<i32> = Rc::new(RefCell::new(
                        ({
                            ((elem!(
                                (qs.with(|__s| __s.predictor.as_pointer()) as Ptr<u8>),
                                ((*k.borrow()) as usize)
                            )
                            .read()) as i32)
                        } + { qs.with(|__s| __s.delta) }),
                    ));
                    elem!(
                        ({
                            (*elem!(
                                ((*jpg.borrow()).with(|__s| __s.quant.as_pointer())
                                    as Ptr<brunsli_JPEGQuantTable>),
                                qs.with(|__s| __s.i)
                            )
                            .upgrade()
                            .deref())
                            .values
                            .as_pointer()
                        } as Ptr<i32>),
                        ((*k.borrow()) as usize)
                    )
                    .write((*quant_value.borrow()));
                    if ((*quant_value.borrow()) <= 0) {
                        return ({
                            (*suspend_bit_reader.borrow())
                                .call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                        });
                    }
                    if ((*quant_value.borrow()) >= 256) {
                        field!(qs, data_precision).write(1_u8);
                    }
                    if ((*quant_value.borrow()) >= 65536) {
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
                                ((*jpg.borrow()).with(|__s| __s.quant.as_pointer())
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
            (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len()
        }) {
            field!(qs, stage).write(brunsli_internal_dec_QuantDataState_Stage_FINISH);
            continue 'loop_;
        }
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                as Ptr<brunsli_JPEGComponent>)
                .offset(qs.with(|__s| __s.i))),
        ));
        if !({ BrunsliBitReaderCanRead_134((*br.borrow()).clone(), 2_usize) }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let __rhs = (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }) as u8);
        field!((*c.borrow()), quant_idx).write(__rhs);
        if ({ ((*c.borrow()).with(|__s| __s.quant_idx) as usize) } >= {
            (*(*jpg.borrow()).with(|__s| __s.quant.clone()).borrow()).len()
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
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1787,
                Ptr::<u8>::from_string_literal(b"DecodeQuantDataSection"),
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
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let hs: Ptr<brunsli_internal_dec_HistogramDataState> = field_ptr!(s, histogram);
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new((field_ptr!(hs, br))));
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_INIT as i32))
    {
        ({ BrunsliBitReaderInit_127((*br.borrow()).clone()) });
        if !(!((*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).is_empty())) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    1802,
                    Ptr::<u8>::from_string_literal(b"DecodeHistogramDataSection"),
                )
            });
            'loop_: while true {}
        };
        field!(s, num_contexts)
            .write((*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len());
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
                let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
                return ({
                    SuspendBitReader_183(
                        (br.read()).clone(),
                        (state.read()).clone(),
                        (*result.borrow()),
                    )
                });
            }
        )));
    if ((hs.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage_READ_SCHEME as i32))
    {
        let num_components: Value<usize> = Rc::new(RefCell::new(
            (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len(),
        ));
        if !((*num_components.borrow()) <= 4_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    1822,
                    Ptr::<u8>::from_string_literal(b"DecodeHistogramDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({
            BrunsliBitReaderCanRead_134(
                (*br.borrow()).clone(),
                (3_usize).wrapping_mul((*num_components.borrow())),
            )
        }) {
            return ({
                (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA)
            });
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let scheme: Value<usize> = Rc::new(RefCell::new(
                (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 3_u32) }) as usize),
            ));
            if ((*scheme.borrow()) >= (kNumSchemes_91.with(|rc| *rc.borrow()) as usize)) {
                return ({
                    (*suspend_bit_reader.borrow()).call(brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN)
                });
            }
            let m: Ptr<brunsli_internal_dec_ComponentMeta> = ((*state.borrow())
                .with(|__s| __s.meta.as_pointer())
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*i.borrow()));
            field!(m, context_bits).write((*scheme.borrow()));
            field!(m, context_offset).write(s.with(|__s| __s.num_contexts));
            field!(s, num_contexts).write({
                (s.with(|__s| __s.num_contexts)).wrapping_add(
                    (({
                        let __idx = (*scheme.borrow()) as usize;
                        kNumNonzeroContextSkip_94.with(|rc| rc.borrow()[__idx])
                    }) as usize),
                )
            });
            (*i.borrow_mut()).prefix_inc();
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
        let use_rle_for_zeros: Value<bool> = Rc::new(RefCell::new(
            !(!(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0)),
        ));
        if (*use_rle_for_zeros.borrow()) {
            field!(hs, max_run_length_prefix).write(
                ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 4_u32) }).wrapping_add(1_u32))
                    as usize),
            );
        }
        let alphabet_size: Value<usize> = Rc::new(RefCell::new(
            (s.with(|__s| __s.num_histograms))
                .wrapping_add(hs.with(|__s| __s.max_run_length_prefix)),
        ));
        {
            let _p: Ptr<_> = Ptr::alloc(<brunsli_HuffmanDecodingData>::default());
            (field_ptr!(hs, entropy) as Ptr<Option<Value<brunsli_HuffmanDecodingData>>>)
                .write(_p.to_owned_opt())
        };
        if !({
            let _arena: Ptr<brunsli_Arena_brunsli_HuffmanCode_> = (field_ptr!(hs, arena));
            brunsli_HuffmanDecodingDataImpl::ReadFromBitStream(
                &hs.with(|__s| __s.entropy.clone()).as_pointer(),
                (*alphabet_size.borrow()),
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
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
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
            }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
            return ({ (*suspend_bit_reader.borrow()).call((*status.borrow())) });
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
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1925,
                Ptr::<u8>::from_string_literal(b"DecodeHistogramDataSection"),
            )
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus_BRUNSLI_OK;
}
pub fn DecodeDCDataSection_190(state: Ptr<brunsli_internal_dec_State>) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available: Value<usize> = Rc::new(RefCell::new(
        (({ GetBytesAvailable_166((*state.borrow()).clone()) }) & (!1 as usize)),
    ));
    let limit: Value<usize> = Rc::new(RefCell::new(
        ({ RemainingSectionLength_174((*state.borrow()).clone()) }),
    ));
    if !(((*limit.borrow()) & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1932,
                Ptr::<u8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    let chunk_len: Value<usize> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*limit.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    ));
    let is_last_chunk: Value<bool> =
        Rc::new(RefCell::new(((*chunk_len.borrow()) == (*limit.borrow()))));
    let in_: Value<brunsli_WordSource> = Rc::new(RefCell::new(brunsli_WordSource::new(
        {
            (*state.borrow())
                .with(|__s| __s.data.clone())
                .offset(((*state.borrow()).with(|__s| __s.pos)) as isize)
        },
        { (*chunk_len.borrow()) },
        { (*is_last_chunk.borrow()) },
    )));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({ DecodeDC_157((*state.borrow()).clone(), (in_.as_pointer())) }),
    ));
    if !(({ (*in_.borrow()).pos_ } & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1941,
                Ptr::<u8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    if { (*in_.borrow()).error_ } {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if !({ (*in_.borrow()).pos_ } <= (*chunk_len.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1943,
                Ptr::<u8>::from_string_literal(b"DecodeDCDataSection"),
            )
        });
        'loop_: while true {}
    };
    ({ SkipBytes_165((*state.borrow()).clone(), { (*in_.borrow()).pos_ }) });
    if (*is_last_chunk.borrow()) {
        if !(((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    1946,
                    Ptr::<u8>::from_string_literal(b"DecodeDCDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
    }
    return (*status.borrow());
}
pub fn DecodeACDataSection_191(state: Ptr<brunsli_internal_dec_State>) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available: Value<usize> = Rc::new(RefCell::new(
        (({ GetBytesAvailable_166((*state.borrow()).clone()) }) & (!1 as usize)),
    ));
    let limit: Value<usize> = Rc::new(RefCell::new(
        ({ RemainingSectionLength_174((*state.borrow()).clone()) }),
    ));
    if !(((*limit.borrow()) & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1955,
                Ptr::<u8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    let chunk_len: Value<usize> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*available.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*limit.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
    ));
    let is_last_chunk: Value<bool> =
        Rc::new(RefCell::new(((*chunk_len.borrow()) == (*limit.borrow()))));
    let in_: Value<brunsli_WordSource> = Rc::new(RefCell::new(brunsli_WordSource::new(
        {
            (*state.borrow())
                .with(|__s| __s.data.clone())
                .offset(((*state.borrow()).with(|__s| __s.pos)) as isize)
        },
        { (*chunk_len.borrow()) },
        { (*is_last_chunk.borrow()) },
    )));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({ DecodeAC_160((*state.borrow()).clone(), (in_.as_pointer())) }),
    ));
    if !(({ (*in_.borrow()).pos_ } & 1_usize) == 0_usize) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1964,
                Ptr::<u8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    if { (*in_.borrow()).error_ } {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
    }
    if !({ (*in_.borrow()).pos_ } <= (*chunk_len.borrow())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                1966,
                Ptr::<u8>::from_string_literal(b"DecodeACDataSection"),
            )
        });
        'loop_: while true {}
    };
    ({ SkipBytes_165((*state.borrow()).clone(), { (*in_.borrow()).pos_ }) });
    if (*is_last_chunk.borrow()) {
        if !(((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    1969,
                    Ptr::<u8>::from_string_literal(b"DecodeACDataSection"),
                )
            });
            'loop_: while true {}
        };
        if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
            return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
        }
    }
    return (*status.borrow());
}
pub fn DecodeOriginalJpg_192(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let fs: Ptr<brunsli_internal_dec_FallbackState> = field_ptr!(s, fallback);
    'loop_: while (fs.with(|__s| __s.stage)
        != (brunsli_internal_dec_FallbackState_Stage_DONE as usize))
    {
        'switch: {
            match { fs.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_READ_TAG as usize) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ ReadTag_170((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    if ({ s.with(|__s| __s.section.tag) } != {
                        (kBrunsliOriginalJpgTag_38.with(|rc| *rc.borrow()) as usize)
                    }) || (!(s.with(|__s| __s.section.is_section)))
                    {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    field!(fs, stage)
                        .write((brunsli_internal_dec_FallbackState_Stage_ENTER_SECTION as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_ENTER_SECTION as usize) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ EnterSection_171((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    field!((*jpg.borrow()), original_jpg_size)
                        .write(s.with(|__s| __s.section.remaining));
                    if ((*jpg.borrow()).with(|__s| __s.original_jpg_size) == 0_usize) {
                        field!((*jpg.borrow()), original_jpg).write(Ptr::<u8>::null());
                        field!(fs, stage)
                            .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                        break 'switch;
                    }
                    field!(fs, stage)
                        .write((brunsli_internal_dec_FallbackState_Stage_READ_CONTENTS as usize));
                    break 'switch;
                }
                __v if __v == (brunsli_internal_dec_FallbackState_Stage_READ_CONTENTS as usize) => {
                    let chunk_size: Value<usize> = Rc::new(RefCell::new(
                        ({ GetBytesAvailable_166((*state.borrow()).clone()) }),
                    ));
                    if ((*chunk_size.borrow()) == 0_usize) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA,
                            )
                        });
                    }
                    let src: Value<Ptr<u8>> = Rc::new(RefCell::new(
                        (*state.borrow())
                            .with(|__s| __s.data.clone())
                            .offset(((*state.borrow()).with(|__s| __s.pos)) as isize),
                    ));
                    if (*fs.with(|__s| __s.storage.clone()).borrow()).is_empty() {
                        if ({ (*chunk_size.borrow()) } >= {
                            (*jpg.borrow()).with(|__s| __s.original_jpg_size)
                        }) {
                            field!((*jpg.borrow()), original_jpg).write((*src.borrow()).clone());
                            ({
                                SkipBytes_165(
                                    (*state.borrow()).clone(),
                                    (*jpg.borrow()).with(|__s| __s.original_jpg_size),
                                )
                            });
                            field!(fs, stage)
                                .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                            break 'switch;
                        }
                    }
                    let remaining: Value<usize> = Rc::new(RefCell::new(
                        (((*jpg.borrow()).with(|__s| __s.original_jpg_size) as u64).wrapping_sub(
                            ((*fs.with(|__s| __s.storage.clone()).borrow()).len() as u64),
                        ) as usize),
                    ));
                    let to_copy: Value<usize> = Rc::new(RefCell::new(
                        ({
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
                        } as usize),
                    ));
                    {
                        let start_idx = (fs.with(|__s| __s.storage.as_pointer()) as Ptr<u8>)
                            .to_end()
                            .get_offset();
                        let count = (*src.borrow())
                            .offset((*to_copy.borrow()) as isize)
                            .get_offset()
                            - (*src.borrow()).get_offset();
                        let temp_vec: Vec<u8> =
                            PtrValueIter::new(&(*src.borrow()), count).collect();
                        (fs.with(|__s| __s.storage.as_pointer()) as Ptr<Vec<u8>>).with_mut(
                            |v: &mut Vec<u8>| {
                                v.splice(start_idx..start_idx, temp_vec);
                            },
                        );
                        (fs.with(|__s| __s.storage.as_pointer()) as Ptr<Vec<u8>>) + start_idx
                    };
                    ({ SkipBytes_165((*state.borrow()).clone(), (*to_copy.borrow())) });
                    if ({ (*fs.with(|__s| __s.storage.clone()).borrow()).len() } == {
                        (*jpg.borrow()).with(|__s| __s.original_jpg_size)
                    }) {
                        let __rhs = (fs.with(|__s| __s.storage.as_pointer()) as Ptr<u8>);
                        field!((*jpg.borrow()), original_jpg).write(__rhs);
                        field!(fs, stage)
                            .write((brunsli_internal_dec_FallbackState_Stage_DONE as usize));
                        break 'switch;
                    }
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA,
                        )
                    });
                }
                _ => {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
                        )
                    });
                }
            }
        };
    }
    ({ LeaveSection_172((field_ptr!(s, section))) });
    return brunsli_internal_dec_Stage_DONE;
}
pub fn ParseSection_193(state: Ptr<brunsli_internal_dec_State>) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let sh: Ptr<brunsli_internal_dec_SectionHeaderState> = field_ptr!(s, section_header);
    let result: Value<brunsli_internal_dec_Stage> =
        Rc::new(RefCell::new(brunsli_internal_dec_Stage_ERROR));
    'loop_: while (sh.with(|__s| __s.stage)
        != (brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize))
    {
        'switch: {
            match { sh.with(|__s| __s.stage) } {
                __v if __v == (brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG as usize) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ ReadTag_170((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32)
                        == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        if ({
                            HasSection_194(
                                (*state.borrow()).clone(),
                                (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                            )
                        }) {
                            return brunsli_internal_dec_Stage_DONE;
                        }
                    }
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    if s.with(|__s| __s.section.is_section) {
                        field!(sh, stage).write(
                            (brunsli_internal_dec_SectionHeaderState_Stage_ENTER_SECTION as usize),
                        );
                        continue 'loop_;
                    }
                    let tag_bit: Value<u32> =
                        Rc::new(RefCell::new((1_u32 << s.with(|__s| __s.section.tag))));
                    let is_known_section_tag: Value<bool> = Rc::new(RefCell::new(
                        ((kKnownSectionTags_137.with(|rc| *rc.borrow()) & (*tag_bit.borrow()))
                            != 0),
                    ));
                    if (*is_known_section_tag.borrow()) {
                        return ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                    }
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_READ_VALUE as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_SectionHeaderState_Stage_READ_VALUE as usize) =>
                {
                    let sink: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ DecodeBase128_168((*state.borrow()).clone(), (sink.as_pointer())) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    (*result.borrow_mut()) = brunsli_internal_dec_Stage_SECTION;
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize));
                    continue 'loop_;
                }
                __v if __v
                    == (brunsli_internal_dec_SectionHeaderState_Stage_ENTER_SECTION as usize) =>
                {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({ EnterSection_171((*state.borrow()).clone(), (field_ptr!(s, section))) }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                        return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                    }
                    (*result.borrow_mut()) = brunsli_internal_dec_Stage_SECTION_BODY;
                    field!(sh, stage)
                        .write((brunsli_internal_dec_SectionHeaderState_Stage_DONE as usize));
                    continue 'loop_;
                }
                _ => {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
                        )
                    });
                }
            }
        };
    }
    field!(sh, stage).write((brunsli_internal_dec_SectionHeaderState_Stage_READ_TAG as usize));
    if !((*result.borrow()) != brunsli_internal_dec_Stage_ERROR) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                2091,
                Ptr::<u8>::from_string_literal(b"ParseSection"),
            )
        });
        'loop_: while true {}
    };
    return (*result.borrow());
}
pub fn ProcessSection_195(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let tag_bit: Value<i32> = Rc::new(RefCell::new(
        ((1_u32 << s.with(|__s| __s.section.tag)) as i32),
    ));
    let is_known_section_tag: Value<bool> = Rc::new(RefCell::new(
        ((kKnownSectionTags_137.with(|rc| *rc.borrow()) & ((*tag_bit.borrow()) as u32)) != 0),
    ));
    let skip_section: Value<bool> = Rc::new(RefCell::new(
        (!(*is_known_section_tag.borrow()))
            || (({ (*state.borrow()).with(|__s| __s.skip_tags) } & {
                ((*tag_bit.borrow()) as u32)
            }) != 0),
    ));
    if (*skip_section.borrow()) {
        let to_skip: Value<usize> = Rc::new(RefCell::new(
            ({
                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                    (({ GetBytesAvailable_166((*state.borrow()).clone()) }) as u64),
                ));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                    (({ RemainingSectionLength_174((*state.borrow()).clone()) }) as u64),
                ));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            } as usize),
        ));
        field!((*state.borrow()), pos)
            .write({ ((*state.borrow()).with(|__s| __s.pos)).wrapping_add((*to_skip.borrow())) });
        if (({ RemainingSectionLength_174((*state.borrow()).clone()) }) != 0_usize) {
            if !(({ GetBytesAvailable_166((*state.borrow()).clone()) }) == 0_usize) {
                ({
                    BrunsliDumpAndAbort_79(
                        Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                        2110,
                        Ptr::<u8>::from_string_literal(b"ProcessSection"),
                    )
                });
                'loop_: while true {}
            };
            return ({
                Fail_169(
                    (*state.borrow()).clone(),
                    brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA,
                )
            });
        }
        return brunsli_internal_dec_Stage_SECTION;
    }
    'switch: {
        match { s.with(|__s| __s.section.tag) } {
            __v if __v == (kBrunsliMetaDataTag_32.with(|rc| *rc.borrow()) as usize) => {
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        DecodeMetaDataSection_180(
                            (*state.borrow()).clone(),
                            (*jpg.borrow()).clone(),
                        )
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as usize) => {
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        DecodeJPEGInternalsSection_184(
                            (*state.borrow()).clone(),
                            (*jpg.borrow()).clone(),
                        )
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (*state.borrow()).clone(),
                        (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        DecodeQuantDataSection_186(
                            (*state.borrow()).clone(),
                            (*jpg.borrow()).clone(),
                        )
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (*state.borrow()).clone(),
                        (kBrunsliJPEGInternalsTag_33.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        DecodeHistogramDataSection_187(
                            (*state.borrow()).clone(),
                            (*jpg.borrow()).clone(),
                        )
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (*state.borrow()).clone(),
                        (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                if !({
                    HasSection_194(
                        (*state.borrow()).clone(),
                        (kBrunsliQuantDataTag_34.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                if ((({ RemainingSectionLength_174((*state.borrow()).clone()) }) & 1_usize)
                    != 0_usize)
                {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                ({ WarmupMeta_196((*jpg.borrow()).clone(), (*state.borrow()).clone()) });
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({ DecodeDCDataSection_190((*state.borrow()).clone()) }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            __v if __v == (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as usize) => {
                if !({
                    HasSection_194(
                        (*state.borrow()).clone(),
                        (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as u32),
                    )
                }) {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                if ((({ RemainingSectionLength_174((*state.borrow()).clone()) }) & 1_usize)
                    != 0_usize)
                {
                    return ({
                        Fail_169(
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                        )
                    });
                }
                ({ WarmupMeta_196((*jpg.borrow()).clone(), (*state.borrow()).clone()) });
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({ DecodeACDataSection_191((*state.borrow()).clone()) }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
                    return ({ Fail_169((*state.borrow()).clone(), (*status.borrow())) });
                }
                break 'switch;
            }
            _ => {
                return ({
                    Fail_169(
                        (*state.borrow()).clone(),
                        brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                    )
                });
            }
        }
    };
    if !({ IsAtSectionBoundary_175((*state.borrow()).clone()) }) {
        return ({
            Fail_169(
                (*state.borrow()).clone(),
                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
            )
        });
    }
    if ({ s.with(|__s| __s.section.tag) } == {
        (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as usize)
    }) {
        return brunsli_internal_dec_Stage_DONE;
    }
    return brunsli_internal_dec_Stage_SECTION;
}
pub fn UpdateSubsamplingDerivatives_178(jpg: Ptr<brunsli_JPEGData>) -> bool {
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
        let __rhs = (if field_ptr!((*jpg.borrow()), max_h_samp_factor).read()
            >= field_ptr!((*c.borrow()), h_samp_factor).read()
        {
            field_ptr!((*jpg.borrow()), max_h_samp_factor)
        } else {
            field_ptr!((*c.borrow()), h_samp_factor)
        }
        .read());
        field!((*jpg.borrow()), max_h_samp_factor).write(__rhs);
        let __rhs = (if field_ptr!((*jpg.borrow()), max_v_samp_factor).read()
            >= field_ptr!((*c.borrow()), v_samp_factor).read()
        {
            field_ptr!((*jpg.borrow()), max_v_samp_factor)
        } else {
            field_ptr!((*c.borrow()), v_samp_factor)
        }
        .read());
        field!((*jpg.borrow()), max_v_samp_factor).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ({
        let _a: i32 = (*jpg.borrow()).with(|__s| __s.height);
        let _b: i32 = ((*jpg.borrow()).with(|__s| __s.max_v_samp_factor) * 8);
        DivCeil_142(_a, _b)
    });
    field!((*jpg.borrow()), MCU_rows).write(__rhs);
    let __rhs = ({
        let _a: i32 = (*jpg.borrow()).with(|__s| __s.width);
        let _b: i32 = ((*jpg.borrow()).with(|__s| __s.max_h_samp_factor) * 8);
        DivCeil_142(_a, _b)
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
        if !((*c.borrow()).with(|__s| __s.width_in_blocks) <= 8205_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2211,
                    Ptr::<u8>::from_string_literal(b"UpdateSubsamplingDerivatives"),
                )
            });
            'loop_: while true {}
        };
        if !((*c.borrow()).with(|__s| __s.height_in_blocks) <= 8205_u32) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2212,
                    Ptr::<u8>::from_string_literal(b"UpdateSubsamplingDerivatives"),
                )
            });
            'loop_: while true {}
        };
        let num_blocks: Value<u32> = Rc::new(RefCell::new(
            ((*c.borrow()).with(|__s| __s.width_in_blocks))
                .wrapping_mul((*c.borrow()).with(|__s| __s.height_in_blocks)),
        ));
        if (((*num_blocks.borrow()) as usize) > kBrunsliMaxNumBlocks_18.with(|rc| *rc.borrow())) {
            return false;
        }
        field!((*c.borrow()), num_blocks).write((*num_blocks.borrow()));
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn PrepareMeta_179(jpg: Ptr<brunsli_JPEGData>, state: Ptr<brunsli_internal_dec_State>) {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let num_components: Value<usize> = Rc::new(RefCell::new(
        (*(*jpg.borrow()).with(|__s| __s.components.clone()).borrow()).len(),
    ));
    {
        let _a0 = (*num_components.borrow()) as usize;
        (s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<u8>>>| __v.resize_with(_a0, <Value<Vec<u8>>>::default),
        )
    };
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    {
        let __a0 = (*num_components.borrow()) as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_dec_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_dec_ComponentMeta>::default())
        })
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let c: Ptr<brunsli_JPEGComponent> =
            ((*jpg.borrow()).with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()));
        let m: Ptr<brunsli_internal_dec_ComponentMeta> =
            (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*i.borrow()));
        field!(m, h_samp).write(c.with(|__s| __s.h_samp_factor));
        field!(m, v_samp).write(c.with(|__s| __s.v_samp_factor));
        field!(m, width_in_blocks).write({
            ({ (*jpg.borrow()).with(|__s| __s.MCU_cols) } * { m.with(|__s| __s.h_samp) })
        });
        field!(m, height_in_blocks).write({
            ({ (*jpg.borrow()).with(|__s| __s.MCU_rows) } * { m.with(|__s| __s.v_samp) })
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn WarmupMeta_196(jpg: Ptr<brunsli_JPEGData>, state: Ptr<brunsli_internal_dec_State>) {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*state.borrow()).with(|__s| __s.meta.as_pointer());
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    if !((*state.borrow()).with(|__s| __s.is_storage_allocated)) {
        field!((*state.borrow()), is_storage_allocated).write(true);
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let num_blocks: Value<usize> = Rc::new(RefCell::new(
                (({
                    {
                        (*elem!(
                            (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                                as Ptr<brunsli_internal_dec_ComponentMeta>),
                            (*i.borrow())
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
                            (*i.borrow())
                        )
                        .upgrade()
                        .deref())
                        .height_in_blocks
                    }
                }) as usize),
            ));
            {
                let __a0 = (*num_blocks.borrow())
                    .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize))
                    as usize;
                (*{
                    (*elem!(
                        ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                            as Ptr<brunsli_JPEGComponent>),
                        (*i.borrow())
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
                let __a0 = (*num_blocks.borrow()) as usize;
                elem!(
                    (s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Value<Vec<u8>>>),
                    (*i.borrow())
                )
                .with_mut(|__v: &mut Value<Vec<u8>>| {
                    (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                })
            };
            let __rhs = ((s.with(|__s| __s.block_state_.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset((*i.borrow()))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>);
            field!(
                elem!(
                    (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                        as Ptr<brunsli_internal_dec_ComponentMeta>),
                    (*i.borrow())
                ),
                block_state
            )
            .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if !(s.with(|__s| __s.is_meta_warm)) {
        field!(s, is_meta_warm).write(true);
        let c: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                (Ptr::<Vec<brunsli_internal_dec_ComponentMeta>>::decay(&(meta))
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*c.borrow()));
            let q: Ptr<brunsli_JPEGQuantTable> =
                ((*jpg.borrow()).with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>)
                    .offset(
                        ({
                            (*elem!(
                                ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                                    as Ptr<brunsli_JPEGComponent>),
                                (*c.borrow())
                            )
                            .upgrade()
                            .deref())
                            .quant_idx
                        } as usize),
                    );
            field!(m, ac_coeffs).write(
                ({
                    (*elem!(
                        ((*jpg.borrow()).with(|__s| __s.components.as_pointer())
                            as Ptr<brunsli_JPEGComponent>),
                        (*c.borrow())
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
            (*c.borrow_mut()).prefix_inc();
        }
    }
}
pub fn DoProcessJpeg_197(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    'loop_: while true {
        'switch: {
            match { (*state.borrow()).with(|__s| __s.stage) } {
                __v if __v == brunsli_internal_dec_Stage_SIGNATURE => {
                    let __rhs = ({ VerifySignature_176((*state.borrow()).clone()) });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_HEADER => {
                    let __rhs =
                        ({ DecodeHeader_177((*state.borrow()).clone(), (*jpg.borrow()).clone()) });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_FALLBACK => {
                    let __rhs = ({
                        DecodeOriginalJpg_192((*state.borrow()).clone(), (*jpg.borrow()).clone())
                    });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_SECTION => {
                    let __rhs = ({ ParseSection_193((*state.borrow()).clone()) });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_SECTION_BODY => {
                    let __rhs = ({
                        ProcessSection_195((*state.borrow()).clone(), (*jpg.borrow()).clone())
                    });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
                __v if __v == brunsli_internal_dec_Stage_DONE => {
                    if ({ (*state.borrow()).with(|__s| __s.pos) } != {
                        (*state.borrow()).with(|__s| __s.len)
                    }) {
                        let __rhs = ({
                            Fail_169(
                                (*state.borrow()).clone(),
                                brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN,
                            )
                        });
                        field!((*state.borrow()), stage).write(__rhs);
                        break 'switch;
                    }
                    return brunsli_BrunsliStatus_BRUNSLI_OK;
                }
                __v if __v == brunsli_internal_dec_Stage_ERROR => {
                    return {
                        (*(*state.borrow())
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
                            (*state.borrow()).clone(),
                            brunsli_BrunsliStatus_BRUNSLI_DECOMPRESSION_ERROR,
                        )
                    });
                    field!((*state.borrow()), stage).write(__rhs);
                    break 'switch;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ChargeBuffer_198(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    field!(b, borrowed_len).write(0_usize);
    field!(b, external_data).write((*state.borrow()).with(|__s| __s.data.clone()));
    field!(b, external_pos).write((*state.borrow()).with(|__s| __s.pos));
    field!(b, external_len).write((*state.borrow()).with(|__s| __s.len));
}
thread_local!(
    pub static kBufferMaxReadAhead_199: Value<usize> = Rc::new(RefCell::new(600_usize));
);
pub fn LoadInput_200(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    if (b.with(|__s| __s.data_len) == 0_usize) {
        field!((*state.borrow()), data).write(b.with(|__s| __s.external_data.clone()));
        field!((*state.borrow()), pos).write(b.with(|__s| __s.external_pos));
        field!((*state.borrow()), len).write(b.with(|__s| __s.external_len));
        return;
    }
    if !({ b.with(|__s| __s.data_len) } <= { kBufferMaxReadAhead_199.with(|rc| *rc.borrow()) }) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                2337,
                Ptr::<u8>::from_string_literal(b"LoadInput"),
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
    field!((*state.borrow()), data).write(__rhs);
    field!((*state.borrow()), pos).write(0_usize);
    field!((*state.borrow()), len)
        .write((b.with(|__s| __s.data_len)).wrapping_add(b.with(|__s| __s.borrowed_len)));
}
pub fn UnloadInput_201(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    if ({ (*state.borrow()).with(|__s| __s.data.clone()) } == {
        b.with(|__s| __s.external_data.clone())
    }) {
        field!(b, external_pos).write((*state.borrow()).with(|__s| __s.pos));
        if !({ b.with(|__s| __s.external_pos) } <= { b.with(|__s| __s.external_len) }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2364,
                    Ptr::<u8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        if (((*result.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            return true;
        }
        if !(b.with(|__s| __s.data_len) == 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2366,
                    Ptr::<u8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        let available: Value<usize> = Rc::new(RefCell::new(
            (b.with(|__s| __s.external_len)).wrapping_sub(b.with(|__s| __s.external_pos)),
        ));
        if !((*available.borrow()) < kBufferMaxReadAhead_199.with(|rc| *rc.borrow())) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2368,
                    Ptr::<u8>::from_string_literal(b"UnloadInput"),
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
        field!(b, data_len).write((*available.borrow()));
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
        field!(b, external_pos)
            .write({ (b.with(|__s| __s.external_pos)).wrapping_add((*available.borrow())) });
        return false;
    }
    if ({ (*state.borrow()).with(|__s| __s.pos) } >= { b.with(|__s| __s.data_len) }) {
        let used_borrowed_bytes: Value<usize> = Rc::new(RefCell::new(
            ((*state.borrow()).with(|__s| __s.pos)).wrapping_sub(b.with(|__s| __s.data_len)),
        ));
        field!(b, data_len).write(0_usize);
        field!(b, external_pos).write({
            (b.with(|__s| __s.external_pos)).wrapping_add((*used_borrowed_bytes.borrow()))
        });
        return true;
    }
    field!(b, data_len).write({
        (b.with(|__s| __s.data_len)).wrapping_sub((*state.borrow()).with(|__s| __s.pos))
    });
    if (((*result.borrow()) as i32) == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if !({ (b.with(|__s| __s.external_pos)).wrapping_add(b.with(|__s| __s.borrowed_len)) } == {
            b.with(|__s| __s.external_len)
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2389,
                    Ptr::<u8>::from_string_literal(b"UnloadInput"),
                )
            });
            'loop_: while true {}
        };
        if !({ (b.with(|__s| __s.data_len)).wrapping_add(b.with(|__s| __s.borrowed_len)) } < {
            kBufferMaxReadAhead_199.with(|rc| *rc.borrow())
        }) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2391,
                    Ptr::<u8>::from_string_literal(b"UnloadInput"),
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
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                2395,
                Ptr::<u8>::from_string_literal(b"UnloadInput"),
            )
        });
        'loop_: while true {}
    };
    if ((*state.borrow()).with(|__s| __s.pos) > 0_usize) && (b.with(|__s| __s.data_len) > 0_usize) {
        {
            ((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &((b.with(|__s| __s.data.as_pointer()) as Ptr<u8>)
                        .offset(((*state.borrow()).with(|__s| __s.pos)) as isize)
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
                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                2399,
                Ptr::<u8>::from_string_literal(b"UnloadInput"),
            )
        });
        'loop_: while true {}
    };
    return (((*result.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32));
}
pub fn UnchargeBuffer_202(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = field_ptr!(s, buffer);
    field!((*state.borrow()), data).write(b.with(|__s| __s.external_data.clone()));
    field!((*state.borrow()), pos).write(b.with(|__s| __s.external_pos));
    field!((*state.borrow()), len).write(b.with(|__s| __s.external_len));
}
pub fn ProcessJpeg_203(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> = (*state.borrow())
        .with(|__s| __s.internal.clone())
        .as_pointer();
    if ({ (*state.borrow()).with(|__s| __s.pos) } > { (*state.borrow()).with(|__s| __s.len) }) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM;
    }
    ({ ChargeBuffer_198((*state.borrow()).clone()) });
    let result: Value<brunsli_BrunsliStatus> =
        Rc::new(RefCell::new(brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA));
    'loop_: while (((*result.borrow()) as i32)
        == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
    {
        if ((*state.borrow()).with(|__s| __s.stage) == brunsli_internal_dec_Stage_ERROR) {
            if ((s.with(|__s| __s.result) as i32)
                != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
            {
                return s.with(|__s| __s.result);
            }
            field!(s, result).write(brunsli_BrunsliStatus_BRUNSLI_OK);
            field!((*state.borrow()), stage).write(s.with(|__s| __s.last_stage));
            field!(s, last_stage).write(brunsli_internal_dec_Stage_ERROR);
        }
        ({ LoadInput_200((*state.borrow()).clone()) });
        if s.with(|__s| __s.section.is_active) {
            field!(field!(s, section), milestone).write((*state.borrow()).with(|__s| __s.pos));
            field!(field!(s, section), projected_end).write({
                (s.with(|__s| __s.section.milestone))
                    .wrapping_add(s.with(|__s| __s.section.remaining))
            });
        }
        {
            let _ptr = field!(field!(s, section), tags_met);
            _ptr.write(_ptr.read() | (*state.borrow()).with(|__s| __s.tags_met))
        };
        (*result.borrow_mut()) =
            ({ DoProcessJpeg_197((*state.borrow()).clone(), (*jpg.borrow()).clone()) });
        if s.with(|__s| __s.section.is_active) {
            let processed_len: Value<usize> = Rc::new(RefCell::new(
                ((*state.borrow()).with(|__s| __s.pos))
                    .wrapping_sub(s.with(|__s| __s.section.milestone)),
            ));
            field!(field!(s, section), remaining).write({
                (s.with(|__s| __s.section.remaining)).wrapping_sub((*processed_len.borrow()))
            });
        }
        if !({ UnloadInput_201((*state.borrow()).clone(), (*result.borrow())) }) {
            break;
        }
    }
    ({ UnchargeBuffer_202((*state.borrow()).clone()) });
    return (*result.borrow());
}
pub fn BrunsliDecodeJpeg_204(
    data: Ptr<u8>,
    len: usize,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if !(!(*data.borrow()).is_null()) {
        return brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM;
    }
    let state: Value<brunsli_internal_dec_State> =
        Rc::new(RefCell::new(brunsli_internal_dec_State::new()));
    (*state.borrow_mut()).data = (*data.borrow()).clone();
    (*state.borrow_mut()).len = (*len.borrow());
    return ({ ProcessJpeg_203((state.as_pointer()), (*jpg.borrow()).clone()) });
}
pub fn BrunsliEstimateDecoderPeakMemoryUsage_205(data: Ptr<u8>, len: usize) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    if !(!(*data.borrow()).is_null()) {
        return (brunsli_BrunsliStatus_BRUNSLI_INVALID_PARAM as usize);
    }
    let state: Value<brunsli_internal_dec_State> =
        Rc::new(RefCell::new(brunsli_internal_dec_State::new()));
    (*state.borrow_mut()).data = (*data.borrow()).clone();
    (*state.borrow_mut()).len = (*len.borrow());
    (*state.borrow_mut()).skip_tags =
        !(1_u32 << (kBrunsliHistogramDataTag_35.with(|rc| *rc.borrow()) as i32));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        { (*state.borrow()).internal.clone() }.as_pointer();
    field!(s, shallow_histograms).write(true);
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({ ProcessJpeg_203((state.as_pointer()), (jpg.as_pointer())) }),
    ));
    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32)) {
        return 0_usize;
    }
    let out_size: Value<usize> = Rc::new(RefCell::new((2_usize).wrapping_mul((*len.borrow()))));
    let total_num_blocks: Value<usize> = Rc::new(RefCell::new(0_usize));
    let component_state_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*{ (*jpg.borrow()).components.clone() }.borrow()).len()) {
        let c: Ptr<brunsli_JPEGComponent> = ({ (*jpg.borrow()).components.as_pointer() }
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()));
        (*total_num_blocks.borrow_mut()) =
            { (*total_num_blocks.borrow()).wrapping_add((c.with(|__s| __s.num_blocks) as usize)) };
        {
            let rhs_0 = (*component_state_size.borrow()).wrapping_add(
                ({
                    brunsli_ComponentState::SizeInBytes((c.with(|__s| __s.width_in_blocks) as i32))
                }),
            );
            (*component_state_size.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let jpeg_data_size: Value<usize> = Rc::new(RefCell::new(
        (((*total_num_blocks.borrow())
            .wrapping_mul((kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) as u64)
            .wrapping_mul((::std::mem::size_of::<i16>() as u64)) as usize),
    ));
    let context_map_size: Value<usize> = Rc::new(RefCell::new(
        (((s.with(|__s| __s.num_contexts)).wrapping_mul(kNumAvrgContexts_83.with(|rc| *rc.borrow()))
            as u64)
            .wrapping_mul((::std::mem::size_of::<i32>() as u64)) as usize),
    ));
    let histogram_size: Value<usize> = Rc::new(RefCell::new(
        ((s.with(|__s| __s.num_histograms) as u64).wrapping_mul((6144usize as u64)) as usize),
    ));
    let decode_peak: Value<usize> = Rc::new(RefCell::new(
        ((*context_map_size.borrow()).wrapping_add((*histogram_size.borrow())))
            .wrapping_add((*component_state_size.borrow())),
    ));
    let jpeg_writer_size: Value<usize> =
        Rc::new(RefCell::new(((1_u32 << 17_u32) as usize).wrapping_add(
            (((1_u32 << 16_u32) as usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))
                as usize),
        )));
    return ((((*out_size.borrow()).wrapping_add((*jpeg_data_size.borrow())) as u64).wrapping_add({
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
pub fn MoveToFront_207(v: Ptr<u8>, index: u8) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let index: Value<u8> = Rc::new(RefCell::new(index));
    let value: Value<u8> = Rc::new(RefCell::new(
        (elem!((*v.borrow()), (*index.borrow())).read()),
    ));
    let i: Value<u8> = Rc::new(RefCell::new((*index.borrow())));
    'loop_: while ((*i.borrow()) != 0) {
        elem!((*v.borrow()), (*i.borrow()))
            .write({ (elem!((*v.borrow()), (((*i.borrow()) as i32) - 1)).read()) });
        (*i.borrow_mut()).prefix_dec();
    }
    elem!((*v.borrow()), 0).write({ (*value.borrow()) });
}
pub fn InverseMoveToFrontTransform_208(v: Ptr<u8>, v_len: usize) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let v_len: Value<usize> = Rc::new(RefCell::new(v_len));
    let mtf: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..256).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < 256_usize) {
        (*mtf.borrow_mut())[(*i.borrow()) as usize] = { ((*i.borrow()) as u8) };
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*v_len.borrow())) {
        let index: Value<u8> = Rc::new(RefCell::new((elem!((*v.borrow()), (*i.borrow())).read())));
        elem!((*v.borrow()), (*i.borrow())).write({ (*mtf.borrow())[(*index.borrow()) as usize] });
        if ((*index.borrow()) != 0) {
            ({ MoveToFront_207((mtf.as_pointer() as Ptr<u8>), (*index.borrow())) });
        }
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn DecodeContextMap_188(
    entropy: Ptr<brunsli_HuffmanDecodingData>,
    max_run_length_prefix: usize,
    index: Ptr<usize>,
    context_map: Ptr<Vec<u8>>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> brunsli_BrunsliStatus {
    let max_run_length_prefix: Value<usize> = Rc::new(RefCell::new(max_run_length_prefix));
    let index: Value<Ptr<usize>> = Rc::new(RefCell::new(index));
    let context_map: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(context_map));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let i: Ptr<usize> = (*index.borrow()).clone();
    let map: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u8>>::decay(&(*context_map.borrow())) as Ptr<u8>),
    ));
    let length: Value<usize> = Rc::new(RefCell::new(
        (*(*context_map.borrow()).upgrade().deref()).len(),
    ));
    'loop_: while ({ (i.read()) } < { (*length.borrow()) }) {
        if !({
            BrunsliBitReaderCanRead_134(
                (*br.borrow()).clone(),
                ((15_usize).wrapping_add((*max_run_length_prefix.borrow()))).wrapping_add(1_usize),
            )
        }) {
            return brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA;
        }
        let code: Value<u32> = Rc::new(RefCell::new(
            (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                brunsli_HuffmanDecodingDataImpl::ReadSymbol(&entropy, _br)
            }) as u32),
        ));
        if ((*code.borrow()) == 0_u32) {
            elem!((*map.borrow()), (i.read())).write(0_u8);
            i.with_mut(|__v| __v.prefix_inc());
        } else if (((*code.borrow()) as usize) <= (*max_run_length_prefix.borrow())) {
            let reps: Value<usize> = Rc::new(RefCell::new(
                ((((1_u32 as u32).wrapping_add((1_u32 << (*code.borrow())))).wrapping_add(
                    ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*code.borrow())) })
                        as i32) as u32),
                )) as usize),
            ));
            'loop_: while ((*reps.borrow_mut()).prefix_dec() != 0) {
                if ({ (i.read()) } >= { (*length.borrow()) }) {
                    return brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN;
                }
                elem!((*map.borrow()), (i.read())).write(0_u8);
                i.with_mut(|__v| __v.prefix_inc());
            }
        } else {
            elem!((*map.borrow()), (i.read())).write({
                ((((*code.borrow()) as usize).wrapping_sub((*max_run_length_prefix.borrow())))
                    as u8)
            });
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    if (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) != 0) {
        ({ InverseMoveToFrontTransform_208((*map.borrow()).clone(), (*length.borrow())) });
    }
    return if ({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) }) {
        brunsli_BrunsliStatus_BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus_BRUNSLI_INVALID_BRN
    };
}
pub fn GetPopulationCountPrecision_209(logcount: u32) -> u32 {
    let logcount: Value<u32> = Rc::new(RefCell::new(logcount));
    return (((*logcount.borrow()).wrapping_add(1_u32)) >> 1);
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
pub fn ReadShortHuffmanCode_212(br: Ptr<brunsli_BrunsliBitReader>, tree: Ptr<i8>) -> usize {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let tree: Value<Ptr<i8>> = Rc::new(RefCell::new(tree));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    let delta: Value<i8> = Rc::new(RefCell::new(1_i8));
    'loop_: while (((*delta.borrow()) as i32) > 0) {
        {
            let rhs_0 = (*pos.borrow()).wrapping_add(
                ((((*delta.borrow()) as u32)
                    .wrapping_add(({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) })))
                    as usize),
            );
            (*pos.borrow_mut()) = rhs_0
        };
        (*delta.borrow_mut()) = { (elem!((*tree.borrow()), (*pos.borrow())).read()) };
    }
    return (-((*delta.borrow()) as i32) as usize);
}
pub fn ReadHistogram_189(
    precision_bits: u32,
    counts: Ptr<Vec<u32>>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    let precision_bits: Value<u32> = Rc::new(RefCell::new(precision_bits));
    let counts: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(counts));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    if !(!((*(*counts.borrow()).upgrade().deref()).is_empty())) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"histogram_decode.cc"),
                41,
                Ptr::<u8>::from_string_literal(b"ReadHistogram"),
            )
        });
        'loop_: while true {}
    };
    let space: Value<u32> = Rc::new(RefCell::new((1_u32 << (*precision_bits.borrow()))));
    let length: Value<usize> = Rc::new(RefCell::new((*(*counts.borrow()).upgrade().deref()).len()));
    {
        let mut __a0 = (Ptr::<Vec<u32>>::decay(&(*counts.borrow())) as Ptr<u32>);
        while __a0 != (Ptr::<Vec<u32>>::decay(&(*counts.borrow())) as Ptr<u32>).to_end() {
            let v = 0.clone();
            __a0.write(v);
            __a0 += 1;
        }
    };
    let histogram: Value<Ptr<u32>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u32>>::decay(&(*counts.borrow())) as Ptr<u32>),
    ));
    let simple_code: Value<i32> = Rc::new(RefCell::new(
        (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as i32),
    ));
    if ((*simple_code.borrow()) == 1) {
        let max_bits_counter: Value<usize> =
            Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_usize)));
        let max_bits: Value<u32> = Rc::new(RefCell::new(0_u32));
        let symbols: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32])));
        let num_symbols: Value<usize> = Rc::new(RefCell::new(
            ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) })
                .wrapping_add((1_u32 as u32))) as usize),
        ));
        'loop_: while ((*max_bits_counter.borrow()) != 0) {
            (*max_bits_counter.borrow_mut()) >>= 1;
            (*max_bits.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
            let __rhs =
                (((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*max_bits.borrow())) })
                    as usize)
                    .wrapping_rem((*length.borrow()))) as i32);
            (*symbols.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*num_symbols.borrow()) == 1_usize) {
            elem!((*histogram.borrow()), (*symbols.borrow())[(0) as usize])
                .write({ (*space.borrow()) });
        } else {
            if ((*symbols.borrow())[(0) as usize] == (*symbols.borrow())[(1) as usize]) {
                return false;
            }
            let value: Value<u32> = Rc::new(RefCell::new(
                ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*precision_bits.borrow())) }),
            ));
            elem!((*histogram.borrow()), (*symbols.borrow())[(0) as usize])
                .write({ (*value.borrow()) });
            elem!((*histogram.borrow()), (*symbols.borrow())[(1) as usize])
                .write({ (*space.borrow()).wrapping_sub((*value.borrow())) });
        }
    } else {
        let real_length: Value<usize> = Rc::new(RefCell::new(
            ({
                ReadShortHuffmanCode_212(
                    (*br.borrow()).clone(),
                    (kLengthTree_210.with(|v| v.as_pointer()) as Ptr<i8>),
                )
            }),
        ));
        let total_count: Value<u32> = Rc::new(RefCell::new(0_u32));
        let log_counts: Value<Box<[u32]>> =
            Rc::new(RefCell::new((0..18).map(|_| 0_u32).collect::<Box<[u32]>>()));
        let omit_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
        if !((*real_length.borrow()) > 2_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"histogram_decode.cc"),
                    74,
                    Ptr::<u8>::from_string_literal(b"ReadHistogram"),
                )
            });
            'loop_: while true {}
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*real_length.borrow())) {
            let __rhs = (({
                ReadShortHuffmanCode_212(
                    (*br.borrow()).clone(),
                    (kLogCountTree_211.with(|v| v.as_pointer()) as Ptr<i8>),
                )
            }) as u32);
            (*log_counts.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            if ((*log_counts.borrow())[(*i.borrow()) as usize]
                > (*log_counts.borrow())[(*omit_pos.borrow()) as usize])
            {
                (*omit_pos.borrow_mut()) = (*i.borrow());
            }
            (*i.borrow_mut()).prefix_inc();
        }
        if !((*omit_pos.borrow()) >= 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"histogram_decode.cc"),
                    80,
                    Ptr::<u8>::from_string_literal(b"ReadHistogram"),
                )
            });
            'loop_: while true {}
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*real_length.borrow())) {
            let code: Value<u32> =
                Rc::new(RefCell::new((*log_counts.borrow())[(*i.borrow()) as usize]));
            if ((*i.borrow()) == (*omit_pos.borrow())) {
                (*i.borrow_mut()).prefix_inc();
                continue 'loop_;
            } else if ((*code.borrow()) == 0_u32) {
                (*i.borrow_mut()).prefix_inc();
                continue 'loop_;
            } else if ((*code.borrow()) == 1_u32) {
                elem!((*histogram.borrow()), (*i.borrow())).write(1_u32);
            } else {
                let bit_count: Value<u32> = Rc::new(RefCell::new(
                    ({ GetPopulationCountPrecision_209((*code.borrow()).wrapping_sub(1_u32)) }),
                ));
                let __rhs = (1_u32 << ((*code.borrow()).wrapping_sub(1_u32))).wrapping_add(
                    ({
                        ({
                            BrunsliBitReaderRead_126((*br.borrow()).clone(), (*bit_count.borrow()))
                        })
                    } << {
                        (((*code.borrow()).wrapping_sub(1_u32)).wrapping_sub((*bit_count.borrow())))
                    }),
                );
                elem!((*histogram.borrow()), (*i.borrow())).write(__rhs);
            }
            (*total_count.borrow_mut()) = {
                (*total_count.borrow())
                    .wrapping_add((elem!((*histogram.borrow()), (*i.borrow())).read()))
            };
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*total_count.borrow()) >= (*space.borrow())) {
            return false;
        }
        elem!((*histogram.borrow()), (*omit_pos.borrow()))
            .write({ (*space.borrow()).wrapping_sub((*total_count.borrow())) });
    }
    return ({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) });
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
    code_length_code_lengths: Ptr<u8>,
    num_symbols: usize,
    code_lengths: Ptr<u8>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    let code_length_code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_length_code_lengths));
    let num_symbols: Value<usize> = Rc::new(RefCell::new(num_symbols));
    let code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_lengths));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let symbol: Value<usize> = Rc::new(RefCell::new(0_usize));
    let prev_code_len: Value<u8> =
        Rc::new(RefCell::new(kDefaultCodeLength_215.with(|rc| *rc.borrow())));
    let repeat: Value<usize> = Rc::new(RefCell::new(0_usize));
    let repeat_code_len: Value<u8> = Rc::new(RefCell::new(0_u8));
    let kFullSpace: Value<i32> = Rc::new(RefCell::new((1 << 15)));
    let space: Value<i32> = Rc::new(RefCell::new((*kFullSpace.borrow())));
    let table: Value<Box<[brunsli_HuffmanCode]>> = Rc::new(RefCell::new(
        (0..32)
            .map(|_| <brunsli_HuffmanCode>::default())
            .collect::<Box<[brunsli_HuffmanCode]>>(),
    ));
    let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ])));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < kCodeLengthCodes_213.with(|rc| *rc.borrow())) {
        (*counts.borrow_mut())
            [(elem!((*code_length_code_lengths.borrow()), (*i.borrow())).read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    if !(({
        BuildHuffmanTable_218(
            (table.as_pointer() as Ptr<brunsli_HuffmanCode>),
            5_usize,
            (*code_length_code_lengths.borrow()).clone(),
            (kCodeLengthCodes_213.with(|rc| *rc.borrow()) as usize),
            ((counts.as_pointer() as Ptr<u16>).offset(0)),
        )
    }) != 0)
    {
        return false;
    }
    'loop_: while ((*symbol.borrow()) < (*num_symbols.borrow())) && ((*space.borrow()) > 0) {
        let p: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
            (table.as_pointer() as Ptr<brunsli_HuffmanCode>),
        ));
        let code_len: Value<u8> = Rc::new(RefCell::new(0_u8));
        (*p.borrow_mut()) += ({ BrunsliBitReaderGet_124((*br.borrow()).clone(), 5_u32) });
        ({
            BrunsliBitReaderDrop_125(
                (*br.borrow()).clone(),
                ((*p.borrow()).with(|__s| __s.bits) as u32),
            )
        });
        (*code_len.borrow_mut()) = ((*p.borrow()).with(|__s| __s.value) as u8);
        if (((*code_len.borrow()) as i32)
            < (kCodeLengthRepeatCode_216.with(|rc| *rc.borrow()) as i32))
        {
            (*repeat.borrow_mut()) = 0_usize;
            let __rhs = (*code_len.borrow());
            elem!(
                (*code_lengths.borrow()),
                (*symbol.borrow_mut()).postfix_inc()
            )
            .write(__rhs);
            if (((*code_len.borrow()) as i32) != 0) {
                (*prev_code_len.borrow_mut()) = (*code_len.borrow());
                (*space.borrow_mut()) -= ((*kFullSpace.borrow()) >> ((*code_len.borrow()) as i32));
            }
        } else {
            let extra_bits: Value<u32> =
                Rc::new(RefCell::new(((((*code_len.borrow()) as i32) - 14) as u32)));
            let old_repeat: Value<usize> = Rc::new(RefCell::new(0_usize));
            let repeat_delta: Value<usize> = Rc::new(RefCell::new(0_usize));
            let new_len: Value<u8> = Rc::new(RefCell::new(0_u8));
            if (((*code_len.borrow()) as i32)
                == (kCodeLengthRepeatCode_216.with(|rc| *rc.borrow()) as i32))
            {
                (*new_len.borrow_mut()) = (*prev_code_len.borrow());
            }
            if (((*repeat_code_len.borrow()) as i32) != ((*new_len.borrow()) as i32)) {
                (*repeat.borrow_mut()) = 0_usize;
                (*repeat_code_len.borrow_mut()) = (*new_len.borrow());
            }
            (*old_repeat.borrow_mut()) = (*repeat.borrow());
            if ((*repeat.borrow()) > 0_usize) {
                (*repeat.borrow_mut()) = { (*repeat.borrow()).wrapping_sub(2_usize) };
                (*repeat.borrow_mut()) <<= (*extra_bits.borrow());
            }
            {
                let rhs_0 = (*repeat.borrow()).wrapping_add(
                    ((({
                        BrunsliBitReaderRead_126((*br.borrow()).clone(), (*extra_bits.borrow()))
                    })
                    .wrapping_add((3_u32 as u32))) as usize),
                );
                (*repeat.borrow_mut()) = rhs_0
            };
            (*repeat_delta.borrow_mut()) = (*repeat.borrow()).wrapping_sub((*old_repeat.borrow()));
            if ((*symbol.borrow()).wrapping_add((*repeat_delta.borrow())) > (*num_symbols.borrow()))
            {
                return false;
            }
            {
                (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>)
                    .to_any()
                    .memset(
                        ((*repeat_code_len.borrow()) as i32) as u8,
                        (*repeat_delta.borrow()) as usize,
                    );
                (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>).to_any()
            };
            (*symbol.borrow_mut()) = { (*symbol.borrow()).wrapping_add((*repeat_delta.borrow())) };
            if (((*repeat_code_len.borrow()) as i32) != 0) {
                (*space.borrow_mut()) -= ((((*repeat_delta.borrow())
                    .wrapping_mul(((*kFullSpace.borrow()) as usize)))
                    as i32)
                    >> ((*repeat_code_len.borrow()) as i32));
            }
        }
    }
    if ((*space.borrow()) != 0) {
        return false;
    }
    {
        (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>)
            .to_any()
            .memset(
                (0) as u8,
                ((*num_symbols.borrow()).wrapping_sub((*symbol.borrow()))) as usize,
            );
        (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>).to_any()
    };
    return ({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) });
}
pub fn ReadSimpleCode_219(
    alphabet_size: u16,
    br: Ptr<brunsli_BrunsliBitReader>,
    table: Ptr<brunsli_HuffmanCode>,
) -> bool {
    let alphabet_size: Value<u16> = Rc::new(RefCell::new(alphabet_size));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(table));
    let max_bits: Value<u32> = Rc::new(RefCell::new(
        (if (((*alphabet_size.borrow()) as u32) > 1_u32) {
            (({
                Log2FloorNonZero_74(((*alphabet_size.borrow()) as u32).wrapping_sub((1_u32 as u32)))
            }) + 1)
        } else {
            0
        } as u32),
    ));
    let num_symbols: Value<usize> = Rc::new(RefCell::new(
        ((({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }).wrapping_add(1_u32))
            as usize),
    ));
    let symbols: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([0_u16, 0_u16, 0_u16, 0_u16])));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
        let symbol: Value<u16> = Rc::new(RefCell::new(
            (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), (*max_bits.borrow())) }) as u16),
        ));
        if (((*symbol.borrow()) as i32) >= ((*alphabet_size.borrow()) as i32)) {
            return false;
        }
        (*symbols.borrow_mut())[(*i.borrow()) as usize] = (*symbol.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow()).wrapping_sub(1_usize)) {
        let j: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*j.borrow()) < (*num_symbols.borrow())) {
            if (((*symbols.borrow())[(*i.borrow()) as usize] as i32)
                == ((*symbols.borrow())[(*j.borrow()) as usize] as i32))
            {
                return false;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*num_symbols.borrow()) == 4_usize) {
        {
            let rhs_0 = (*num_symbols.borrow()).wrapping_add(
                (({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 1_u32) }) as usize),
            );
            (*num_symbols.borrow_mut()) = rhs_0
        };
    }
    let swap_symbols: Value<FnPtr<fn(usize, usize)>> = Rc::new(RefCell::new(lambda!(
        {
            let symbols: Ptr<u16> = symbols.as_pointer();
        },
        |i: usize, j: usize| {
            let i: Value<usize> = Rc::new(RefCell::new(i));
            let j: Value<usize> = Rc::new(RefCell::new(j));
            let t: Value<u16> = Rc::new(RefCell::new((elem!((symbols), (*j.borrow())).read())));
            elem!((symbols), (*j.borrow())).write({ (elem!((symbols), (*i.borrow())).read()) });
            elem!((symbols), (*i.borrow())).write((*t.borrow()));
        }
    )));
    let table_size: Value<usize> = Rc::new(RefCell::new(1_usize));
    'switch: {
        match { (*num_symbols.borrow()) } {
            __v if __v == 1_usize => {
                elem!((*table.borrow()), 0).write({
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
                elem!((*table.borrow()), 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 1).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                (*table_size.borrow_mut()) = 2_usize;
                break 'switch;
            }
            __v if __v == 3_usize => {
                if (((*symbols.borrow())[(1) as usize] as i32)
                    > ((*symbols.borrow())[(2) as usize] as i32))
                {
                    ({ (*swap_symbols.borrow()).call(1_usize, 2_usize) });
                }
                elem!((*table.borrow()), 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 2).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!((*table.borrow()), 3).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                (*table_size.borrow_mut()) = 4_usize;
                break 'switch;
            }
            __v if __v == 4_usize => {
                let i: Value<usize> = Rc::new(RefCell::new(0_usize));
                'loop_: while ((*i.borrow()) < 3_usize) {
                    let j: Value<usize> =
                        Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
                    'loop_: while ((*j.borrow()) < 4_usize) {
                        if (((*symbols.borrow())[(*i.borrow()) as usize] as i32)
                            > ((*symbols.borrow())[(*j.borrow()) as usize] as i32))
                        {
                            ({ (*swap_symbols.borrow()).call((*i.borrow()), (*j.borrow())) });
                        }
                        (*j.borrow_mut()).prefix_inc();
                    }
                    (*i.borrow_mut()).prefix_inc();
                }
                elem!((*table.borrow()), 0).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 2).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!((*table.borrow()), 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                elem!((*table.borrow()), 3).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(3) as usize],
                    }
                });
                (*table_size.borrow_mut()) = 4_usize;
                break 'switch;
            }
            __v if __v == 5_usize => {
                if (((*symbols.borrow())[(2) as usize] as i32)
                    > ((*symbols.borrow())[(3) as usize] as i32))
                {
                    ({ (*swap_symbols.borrow()).call(2_usize, 3_usize) });
                }
                elem!((*table.borrow()), 0).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 1).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!((*table.borrow()), 2).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 3).write({
                    brunsli_HuffmanCode {
                        bits: 3_u8,
                        value: (*symbols.borrow())[(2) as usize],
                    }
                });
                elem!((*table.borrow()), 4).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 5).write({
                    brunsli_HuffmanCode {
                        bits: 2_u8,
                        value: (*symbols.borrow())[(1) as usize],
                    }
                });
                elem!((*table.borrow()), 6).write({
                    brunsli_HuffmanCode {
                        bits: 1_u8,
                        value: (*symbols.borrow())[(0) as usize],
                    }
                });
                elem!((*table.borrow()), 7).write({
                    brunsli_HuffmanCode {
                        bits: 3_u8,
                        value: (*symbols.borrow())[(3) as usize],
                    }
                });
                (*table_size.borrow_mut()) = 8_usize;
                break 'switch;
            }
            _ => {
                return false;
            }
        }
    };
    let goal_size: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << kHuffmanTableBits_21.with(|rc| *rc.borrow())),
    ));
    'loop_: while ((*table_size.borrow()) != ((*goal_size.borrow()) as usize)) {
        {
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &(((*table.borrow()).offset((0) as isize)) as Ptr<brunsli_HuffmanCode>)
                        .to_any(),
                    (((*table_size.borrow()) as u64).wrapping_mul((4usize as u64)) as usize)
                        as usize,
                );
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
        };
        (*table_size.borrow_mut()) <<= 1;
    }
    return ({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) });
}
pub fn GetNextKey_221(key: i32, len: usize) -> i32 {
    let key: Value<i32> = Rc::new(RefCell::new(key));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let step: Value<i32> = Rc::new(RefCell::new(
        ((1_u32 << ((*len.borrow()).wrapping_sub(1_usize))) as i32),
    ));
    'loop_: while (((*key.borrow()) & (*step.borrow())) != 0) {
        (*step.borrow_mut()) >>= 1;
    }
    return (((*key.borrow()) & ((*step.borrow()) - 1)) + (*step.borrow()));
}
pub fn ReplicateValue_222(
    table: Ptr<brunsli_HuffmanCode>,
    step: i32,
    end: i32,
    code: brunsli_HuffmanCode,
) {
    let table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(table));
    let step: Value<i32> = Rc::new(RefCell::new(step));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    let code: Value<brunsli_HuffmanCode> = Rc::new(RefCell::new(code));
    let mut __do_while = true;
    'loop_: while __do_while || ((*end.borrow()) > 0) {
        __do_while = false;
        (*end.borrow_mut()) -= (*step.borrow());
        elem!((*table.borrow()), (*end.borrow())).write({ (*code.borrow()).clone() });
    }
}
pub fn NextTableBitSize_223(count: Ptr<u16>, len: usize, root_bits: usize) -> usize {
    let count: Value<Ptr<u16>> = Rc::new(RefCell::new(count));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let root_bits: Value<usize> = Rc::new(RefCell::new(root_bits));
    let left: Value<usize> = Rc::new(RefCell::new(
        (1_usize << ((*len.borrow()).wrapping_sub((*root_bits.borrow())))),
    ));
    'loop_: while ((*len.borrow()) < kMaxHuffmanBits_22.with(|rc| *rc.borrow())) {
        if ({ (*left.borrow()) } <= {
            ((elem!((*count.borrow()), (*len.borrow())).read()) as usize)
        }) {
            break;
        }
        (*left.borrow_mut()) = {
            (*left.borrow())
                .wrapping_sub(((elem!((*count.borrow()), (*len.borrow())).read()) as usize))
        };
        (*len.borrow_mut()).prefix_inc();
        (*left.borrow_mut()) <<= 1;
    }
    return (*len.borrow()).wrapping_sub((*root_bits.borrow()));
}
pub fn BuildHuffmanTable_218(
    root_table: Ptr<brunsli_HuffmanCode>,
    root_bits: usize,
    code_lengths: Ptr<u8>,
    code_lengths_size: usize,
    count: Ptr<u16>,
) -> u32 {
    let root_table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(root_table));
    let root_bits: Value<usize> = Rc::new(RefCell::new(root_bits));
    let code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_lengths));
    let code_lengths_size: Value<usize> = Rc::new(RefCell::new(code_lengths_size));
    let count: Value<Ptr<u16>> = Rc::new(RefCell::new(count));
    let code: Value<brunsli_HuffmanCode> = Rc::new(RefCell::new(<brunsli_HuffmanCode>::default()));
    let table: Value<Ptr<brunsli_HuffmanCode>> =
        Rc::new(RefCell::new(Ptr::<brunsli_HuffmanCode>::null()));
    let len: Value<usize> = Rc::new(RefCell::new(0_usize));
    let symbol: Value<usize> = Rc::new(RefCell::new(0_usize));
    let key: Value<i32> = Rc::new(RefCell::new(0_i32));
    let step: Value<i32> = Rc::new(RefCell::new(0_i32));
    let low: Value<i32> = Rc::new(RefCell::new(0_i32));
    let mask: Value<i32> = Rc::new(RefCell::new(0_i32));
    let table_bits: Value<usize> = Rc::new(RefCell::new(0_usize));
    let table_size: Value<i32> = Rc::new(RefCell::new(0_i32));
    let total_size: Value<i32> = Rc::new(RefCell::new(0_i32));
    let offset: Value<Box<[u16]>> =
        Rc::new(RefCell::new((0..16).map(|_| 0_u16).collect::<Box<[u16]>>()));
    let max_length: Value<usize> = Rc::new(RefCell::new(1_usize));
    if ((*code_lengths_size.borrow())
        > ((1_u32 << kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize))
    {
        return 0_u32;
    }
    let sorted_storage: Value<Vec<u16>> = Rc::new(RefCell::new(
        (0..(*code_lengths_size.borrow()) as usize)
            .map(|_| <u16>::default())
            .collect::<Vec<_>>(),
    ));
    let sorted: Value<Ptr<u16>> = Rc::new(RefCell::new((sorted_storage.as_pointer() as Ptr<u16>)));
    {
        let sum: Value<u16> = Rc::new(RefCell::new(0_u16));
        (*len.borrow_mut()) = 1_usize;
        'loop_: while ((*len.borrow()) <= kMaxHuffmanBits_22.with(|rc| *rc.borrow())) {
            (*offset.borrow_mut())[(*len.borrow()) as usize] = (*sum.borrow());
            if ((elem!((*count.borrow()), (*len.borrow())).read()) != 0) {
                (*sum.borrow_mut()) = {
                    (({ ((*sum.borrow()) as i32) } + {
                        ((elem!((*count.borrow()), (*len.borrow())).read()) as i32)
                    }) as u16)
                };
                (*max_length.borrow_mut()) = (*len.borrow());
            }
            (*len.borrow_mut()).postfix_inc();
        }
    }
    (*symbol.borrow_mut()) = 0_usize;
    'loop_: while ((*symbol.borrow()) < (*code_lengths_size.borrow())) {
        if (((elem!((*code_lengths.borrow()), (*symbol.borrow())).read()) as i32) != 0) {
            let __rhs = ((*symbol.borrow()) as u16);
            elem!(
                (*sorted.borrow()),
                (*offset.borrow_mut())
                    [(elem!((*code_lengths.borrow()), (*symbol.borrow())).read()) as usize]
                    .postfix_inc()
            )
            .write(__rhs);
        }
        (*symbol.borrow_mut()).postfix_inc();
    }
    (*table.borrow_mut()) = (*root_table.borrow()).clone();
    (*table_bits.borrow_mut()) = (*root_bits.borrow());
    (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
    (*total_size.borrow_mut()) = (*table_size.borrow());
    if (((*offset.borrow())[(kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize] as i32) == 1) {
        (*code.borrow_mut()).bits = 0_u8;
        (*code.borrow_mut()).value = { (elem!((*sorted.borrow()), 0).read()) };
        (*key.borrow_mut()) = 0;
        'loop_: while ((*key.borrow()) < (*total_size.borrow())) {
            elem!((*table.borrow()), (*key.borrow())).write({ (*code.borrow()).clone() });
            (*key.borrow_mut()).prefix_inc();
        }
        return ((*total_size.borrow()) as u32);
    }
    if ((*table_bits.borrow()) > (*max_length.borrow())) {
        (*table_bits.borrow_mut()) = (*max_length.borrow());
        (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
    }
    (*key.borrow_mut()) = 0;
    (*symbol.borrow_mut()) = 0_usize;
    (*code.borrow_mut()).bits = 1_u8;
    (*step.borrow_mut()) = 2;
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*code.borrow_mut()).bits.prefix_inc() as usize) <= (*table_bits.borrow()))
    {
        __do_while = false;
        'loop_: while (((elem!((*count.borrow()), { (*code.borrow()).bits }).read()) as i32) != 0) {
            let __rhs = (elem!((*sorted.borrow()), (*symbol.borrow_mut()).postfix_inc()).read());
            (*code.borrow_mut()).value = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> =
                    ((*table.borrow()).offset((*key.borrow()) as isize));
                let _code: brunsli_HuffmanCode = (*code.borrow()).clone();
                ReplicateValue_222(_table, (*step.borrow()), (*table_size.borrow()), _code)
            });
            let __rhs = ({ GetNextKey_221((*key.borrow()), ({ (*code.borrow()).bits } as usize)) });
            (*key.borrow_mut()) = __rhs;
            elem!((*count.borrow()), { (*code.borrow()).bits }).with_mut(|__v| __v.prefix_dec());
        }
        (*step.borrow_mut()) <<= 1;
    }
    'loop_: while ((*total_size.borrow()) != (*table_size.borrow())) {
        {
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &(((*table.borrow()).offset((0) as isize)) as Ptr<brunsli_HuffmanCode>)
                        .to_any(),
                    ((*table_size.borrow()) as usize).wrapping_mul((4usize as usize)) as usize,
                );
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
        };
        (*table_size.borrow_mut()) <<= 1;
    }
    (*mask.borrow_mut()) = ((*total_size.borrow()) - 1);
    (*low.borrow_mut()) = -1_i32;
    {
        (*len.borrow_mut()) = (*root_bits.borrow()).wrapping_add(1_usize);
        (*step.borrow_mut()) = 2
    };
    'loop_: while ((*len.borrow()) <= (*max_length.borrow())) {
        'loop_: while (((elem!((*count.borrow()), (*len.borrow())).read()) as i32) != 0) {
            if (((*key.borrow()) & (*mask.borrow())) != (*low.borrow())) {
                (*table.borrow_mut()) += (*table_size.borrow());
                (*table_bits.borrow_mut()) = ({
                    NextTableBitSize_223(
                        (*count.borrow()).clone(),
                        (*len.borrow()),
                        (*root_bits.borrow()),
                    )
                });
                (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
                (*total_size.borrow_mut()) += (*table_size.borrow());
                (*low.borrow_mut()) = ((*key.borrow()) & (*mask.borrow()));
                field!(elem!((*root_table.borrow()), (*low.borrow())), bits)
                    .write((((*table_bits.borrow()).wrapping_add((*root_bits.borrow()))) as u8));
                field!(elem!((*root_table.borrow()), (*low.borrow())), value).write({
                    (({ (((*table.borrow()).clone() - (*root_table.borrow()).clone()) as i64) } - {
                        ((*low.borrow()) as i64)
                    }) as u16)
                });
            }
            (*code.borrow_mut()).bits =
                (((*len.borrow()).wrapping_sub((*root_bits.borrow()))) as u8);
            let __rhs = (elem!((*sorted.borrow()), (*symbol.borrow_mut()).postfix_inc()).read());
            (*code.borrow_mut()).value = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> =
                    ((*table.borrow()).offset(((*key.borrow()) >> (*root_bits.borrow())) as isize));
                let _code: brunsli_HuffmanCode = (*code.borrow()).clone();
                ReplicateValue_222(_table, (*step.borrow()), (*table_size.borrow()), _code)
            });
            let __rhs = ({ GetNextKey_221((*key.borrow()), (*len.borrow())) });
            (*key.borrow_mut()) = __rhs;
            elem!((*count.borrow()), (*len.borrow())).with_mut(|__v| __v.prefix_dec());
        }
        {
            (*len.borrow_mut()).prefix_inc();
            (*step.borrow_mut()) <<= 1
        };
    }
    return ((*total_size.borrow()) as u32);
}
impl brunsli_internal_dec_OutputChunk {
    pub fn new_4(bytes: Ptr<Vec<u8>>) -> Self {
        let __this: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(Self {
            next: Ptr::<u8>::null(),
            len: (*bytes.upgrade().deref()).len(),
            buffer: None,
        }));
        let this: Ptr<brunsli_internal_dec_OutputChunk> = __this.as_pointer();
        let src: Value<AnyPtr> = Rc::new(RefCell::new(
            ((Ptr::<Vec<u8>>::decay(&(bytes)) as Ptr<u8>) as Ptr<u8>).to_any(),
        ));
        field!(this, next).write((*src.borrow()).reinterpret_cast::<u8>());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
thread_local!(
    pub static kJpegPrecision_224: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kBitWriterChunkSize_225: Value<usize> = Rc::new(RefCell::new(16384_usize));
);
pub fn DivCeil_226(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((((*a.borrow()) + (*b.borrow())) - 1) / (*b.borrow()));
}
pub fn HasZeroByte_227(x: u64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return ((((*x.borrow()).wrapping_sub((72340172838076673_u64 as u64))) & !(*x.borrow()))
        & 9259542123273814144_u64);
}
pub fn BitWriterInit_228(
    bw: Ptr<brunsli_internal_dec_BitWriter>,
    output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let output_queue: Value<Ptr<Vec<brunsli_internal_dec_OutputChunk>>> =
        Rc::new(RefCell::new(output_queue));
    field!((*bw.borrow()), output).write((*output_queue.borrow()).clone());
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> =
            Rc::new(RefCell::new(brunsli_internal_dec_OutputChunk::new_2({
                Some(kBitWriterChunkSize_225.with(|rc| *rc.borrow()))
            })));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!((*bw.borrow()), chunk),
            _arg0.as_pointer(),
        )
    });
    field!((*bw.borrow()), pos).write(0_usize);
    field!((*bw.borrow()), put_buffer).write(0_u64);
    field!((*bw.borrow()), put_bits).write(64);
    field!((*bw.borrow()), healthy).write(true);
    let __rhs = (Ptr::<Vec<u8>>::decay(
        &((*bw.borrow())
            .with(|__s| __s.chunk.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    field!((*bw.borrow()), data).write(__rhs);
}
pub fn SwapBuffer_229(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    field!(field!((*bw.borrow()), chunk), len).write({ (*bw.borrow()).with(|__s| __s.pos) });
    {
        let __init =
            brunsli_internal_dec_OutputChunk::move_from({ field_ptr!((*bw.borrow()), chunk) });
        (*bw.borrow())
            .with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.push(__init))
    };
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> =
            Rc::new(RefCell::new(brunsli_internal_dec_OutputChunk::new_2({
                Some(kBitWriterChunkSize_225.with(|rc| *rc.borrow()))
            })));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!((*bw.borrow()), chunk),
            _arg0.as_pointer(),
        )
    });
    let __rhs = (Ptr::<Vec<u8>>::decay(
        &((*bw.borrow())
            .with(|__s| __s.chunk.buffer.clone())
            .as_pointer()),
    ) as Ptr<u8>);
    field!((*bw.borrow()), data).write(__rhs);
    field!((*bw.borrow()), pos).write(0_usize);
}
pub fn Reserve_230(bw: Ptr<brunsli_internal_dec_BitWriter>, n_bytes: usize) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let n_bytes: Value<usize> = Rc::new(RefCell::new(n_bytes));
    if ((({ (((*bw.borrow()).with(|__s| __s.pos)).wrapping_add((*n_bytes.borrow()))) } > {
        kBitWriterChunkSize_225.with(|rc| *rc.borrow())
    }) as i64)
        != 0)
    {
        ({ SwapBuffer_229((*bw.borrow()).clone()) });
    }
}
pub fn EmitByte_231(bw: Ptr<brunsli_internal_dec_BitWriter>, byte: i32) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let byte: Value<i32> = Rc::new(RefCell::new(byte));
    let __rhs = ((*byte.borrow()) as u8);
    elem!(
        (*bw.borrow()).with(|__s| __s.data.clone()),
        field!((*bw.borrow()), pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    if ((*byte.borrow()) == 255) {
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            field!((*bw.borrow()), pos).with_mut(|__v| __v.postfix_inc())
        )
        .write(0_u8);
    }
}
pub fn DischargeBitBuffer_232(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    ({ Reserve_230((*bw.borrow()).clone(), 12_usize) });
    if (({ HasZeroByte_227((!(*bw.borrow()).with(|__s| __s.put_buffer) | 65535_u64)) }) != 0) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 56) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 48) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 40) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 32) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 24) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 16) & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
    } else {
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            (*bw.borrow()).with(|__s| __s.pos)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 56) & 255_u64) as u8) });
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(1_usize)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 48) & 255_u64) as u8) });
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(2_usize)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 40) & 255_u64) as u8) });
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(3_usize)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 32) & 255_u64) as u8) });
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(4_usize)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 24) & 255_u64) as u8) });
        elem!(
            (*bw.borrow()).with(|__s| __s.data.clone()),
            ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(5_usize)
        )
        .write({ ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 16) & 255_u64) as u8) });
        field!((*bw.borrow()), pos)
            .write({ ((*bw.borrow()).with(|__s| __s.pos)).wrapping_add(6_usize) });
    }
    {
        let _ptr = field!((*bw.borrow()), put_buffer);
        _ptr.write(_ptr.read() << 48)
    };
    {
        let _ptr = field!((*bw.borrow()), put_bits);
        _ptr.write(_ptr.read() + 48)
    };
}
pub fn WriteBits_233(bw: Ptr<brunsli_internal_dec_BitWriter>, nbits: i32, bits: u64) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
    let bits: Value<u64> = Rc::new(RefCell::new(bits));
    if ((*nbits.borrow()) == 0) {
        field!((*bw.borrow()), healthy).write(false);
        return;
    }
    {
        let _ptr = field!((*bw.borrow()), put_bits);
        _ptr.write(_ptr.read() - (*nbits.borrow()))
    };
    {
        let _ptr = field!((*bw.borrow()), put_buffer);
        _ptr.write(
            _ptr.read() | { ({ (*bits.borrow()) } << { (*bw.borrow()).with(|__s| __s.put_bits) }) },
        )
    };
    if ((*bw.borrow()).with(|__s| __s.put_bits) <= 16) {
        ({ DischargeBitBuffer_232((*bw.borrow()).clone()) });
    }
}
pub fn EmitMarker_234(bw: Ptr<brunsli_internal_dec_BitWriter>, marker: i32) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let marker: Value<i32> = Rc::new(RefCell::new(marker));
    ({ Reserve_230((*bw.borrow()).clone(), 2_usize) });
    if !((*marker.borrow()) != 255) {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"jpeg_data_writer.cc"),
                133,
                Ptr::<u8>::from_string_literal(b"EmitMarker"),
            )
        });
        'loop_: while true {}
    };
    elem!(
        (*bw.borrow()).with(|__s| __s.data.clone()),
        field!((*bw.borrow()), pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(255_u8);
    let __rhs = ((*marker.borrow()) as u8);
    elem!(
        (*bw.borrow()).with(|__s| __s.data.clone()),
        field!((*bw.borrow()), pos).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
}
pub fn JumpToByteBoundary_235(
    bw: Ptr<brunsli_internal_dec_BitWriter>,
    pad_bits: Ptr<Ptr<i32>>,
    pad_bits_end: Ptr<i32>,
) -> bool {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let pad_bits: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(pad_bits));
    let pad_bits_end: Value<Ptr<i32>> = Rc::new(RefCell::new(pad_bits_end));
    let n_bits: Value<usize> = Rc::new(RefCell::new(
        ((((*bw.borrow()).with(|__s| __s.put_bits) as u32) & 7_u32) as usize),
    ));
    let pad_pattern: Value<u8> = Rc::new(RefCell::new(0_u8));
    if ((*pad_bits.borrow()).read()).is_null() {
        (*pad_pattern.borrow_mut()) = (((1_u32 << (*n_bits.borrow())).wrapping_sub(1_u32)) as u8);
    } else {
        (*pad_pattern.borrow_mut()) = 0_u8;
        let src: Value<Ptr<i32>> = Rc::new(RefCell::new(((*pad_bits.borrow()).read()).clone()));
        'loop_: while ((*n_bits.borrow_mut()).postfix_dec() != 0) {
            (*pad_pattern.borrow_mut()) = { (((*pad_pattern.borrow()) as i32) << 1) as u8 };
            if ({ (*src.borrow()).clone() } >= { (*pad_bits_end.borrow()).clone() }) {
                return false;
            }
            {
                let rhs_0 = (((*pad_pattern.borrow()) as i32)
                    | (!(!((((*src.borrow_mut()).postfix_inc()).read()) != 0)) as i32))
                    as u8;
                (*pad_pattern.borrow_mut()) = rhs_0
            };
        }
        (*pad_bits.borrow()).write({ (*src.borrow()).clone() });
    }
    ({ Reserve_230((*bw.borrow()).clone(), 16_usize) });
    'loop_: while ((*bw.borrow()).with(|__s| __s.put_bits) <= 56) {
        let c: Value<i32> = Rc::new(RefCell::new(
            ((((*bw.borrow()).with(|__s| __s.put_buffer) >> 56) & 255_u64) as i32),
        ));
        ({ EmitByte_231((*bw.borrow()).clone(), (*c.borrow())) });
        {
            let _ptr = field!((*bw.borrow()), put_buffer);
            _ptr.write(_ptr.read() << 8)
        };
        {
            let _ptr = field!((*bw.borrow()), put_bits);
            _ptr.write(_ptr.read() + 8)
        };
    }
    if ((*bw.borrow()).with(|__s| __s.put_bits) < 64) {
        let pad_mask: Value<i32> = Rc::new(RefCell::new(
            ((255_u32 >> (64 - (*bw.borrow()).with(|__s| __s.put_bits))) as i32),
        ));
        let c: Value<i32> = Rc::new(RefCell::new(
            (({
                ({ ((*bw.borrow()).with(|__s| __s.put_buffer) >> 56) } & {
                    (!(*pad_mask.borrow()) as u64)
                })
            } | { ((*pad_pattern.borrow()) as u64) }) as i32),
        ));
        ({ EmitByte_231((*bw.borrow()).clone(), (*c.borrow())) });
    }
    field!((*bw.borrow()), put_buffer).write(0_u64);
    field!((*bw.borrow()), put_bits).write(64);
    return true;
}
pub fn BitWriterFinish_236(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*bw.borrow()).with(|__s| __s.pos) == 0_usize) {
        return;
    }
    field!(field!((*bw.borrow()), chunk), len).write({ (*bw.borrow()).with(|__s| __s.pos) });
    {
        let __init =
            brunsli_internal_dec_OutputChunk::move_from({ field_ptr!((*bw.borrow()), chunk) });
        (*bw.borrow())
            .with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.push(__init))
    };
    ({
        let _arg0: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(
            brunsli_internal_dec_OutputChunk::new_1({ Ptr::<u8>::null() }, { 0_usize }),
        ));
        brunsli_internal_dec_OutputChunkImpl::move_assign(
            &field_ptr!((*bw.borrow()), chunk),
            _arg0.as_pointer(),
        )
    });
    field!((*bw.borrow()), data).write(Ptr::<u8>::null());
    field!((*bw.borrow()), pos).write(0_usize);
}
pub fn DCTCodingStateInit_237(s: Ptr<brunsli_internal_dec_DCTCodingState>) {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    field!((*s.borrow()), eob_run_).write(0);
    field!((*s.borrow()), cur_ac_huff_).write(Ptr::<brunsli_HuffmanCodeTable>::null());
    (*(*s.borrow())
        .with(|__s| __s.refinement_bits_.clone())
        .borrow_mut())
    .clear();
    if 64_usize as usize
        > (*(*s.borrow())
            .with(|__s| __s.refinement_bits_.clone())
            .borrow())
        .capacity() as usize
    {
        let len_0 = (*(*s.borrow())
            .with(|__s| __s.refinement_bits_.clone())
            .borrow())
        .len();
        (*(*s.borrow())
            .with(|__s| __s.refinement_bits_.clone())
            .borrow_mut())
        .reserve_exact(64_usize as usize - len_0 as usize);
    };
    field!((*s.borrow()), refinement_bits_count_).write(0_usize);
}
pub fn Flush_238(
    s: Ptr<brunsli_internal_dec_DCTCodingState>,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*s.borrow()).with(|__s| __s.eob_run_) > 0) {
        let nbits: Value<i32> = Rc::new(RefCell::new(
            ({ Log2FloorNonZero_74(((*s.borrow()).with(|__s| __s.eob_run_) as u32)) }),
        ));
        let symbol: Value<i32> = Rc::new(RefCell::new(((*nbits.borrow()) << 4_u32)));
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!((*s.borrow()).with(|__s| __s.cur_ac_huff_.clone()), depth)
                    as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!((*s.borrow()).with(|__s| __s.cur_ac_huff_.clone()), code)
                    as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        if ((*nbits.borrow()) > 0) {
            ({
                let _nbits: i32 = (*nbits.borrow());
                let _bits: u64 = (({ (*s.borrow()).with(|__s| __s.eob_run_) } & {
                    ((1 << (*nbits.borrow())) - 1)
                }) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
        }
        field!((*s.borrow()), eob_run_).write(0);
    }
    let num_words: Value<usize> = Rc::new(RefCell::new(
        ((*s.borrow()).with(|__s| __s.refinement_bits_count_) >> 4),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_words.borrow())) {
        ({
            WriteBits_233(
                (*bw.borrow()).clone(),
                16,
                ((elem!(
                    ((*s.borrow()).with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>),
                    (*i.borrow())
                )
                .read()) as u64),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let tail: Value<usize> = Rc::new(RefCell::new(
        ((*s.borrow()).with(|__s| __s.refinement_bits_count_) & 15_usize),
    ));
    if ((*tail.borrow()) != 0) {
        ({
            WriteBits_233(
                (*bw.borrow()).clone(),
                ((*tail.borrow()) as i32),
                ((((*s.borrow()).with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                    .to_last()
                    .read()) as u64),
            )
        });
    }
    (*(*s.borrow())
        .with(|__s| __s.refinement_bits_.clone())
        .borrow_mut())
    .clear();
    field!((*s.borrow()), refinement_bits_count_).write(0_usize);
}
pub fn BufferEndOfBand_239(
    s: Ptr<brunsli_internal_dec_DCTCodingState>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    new_bits_array: Ptr<i32>,
    new_bits_count: usize,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    let ac_huff: Value<Ptr<brunsli_HuffmanCodeTable>> = Rc::new(RefCell::new(ac_huff));
    let new_bits_array: Value<Ptr<i32>> = Rc::new(RefCell::new(new_bits_array));
    let new_bits_count: Value<usize> = Rc::new(RefCell::new(new_bits_count));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*s.borrow()).with(|__s| __s.eob_run_) == 0) {
        field!((*s.borrow()), cur_ac_huff_).write((*ac_huff.borrow()).clone());
    }
    field!((*s.borrow()), eob_run_).with_mut(|__v| __v.prefix_inc());
    if ((*new_bits_count.borrow()) != 0) {
        let new_bits: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*new_bits_count.borrow())) {
            (*new_bits.borrow_mut()) = {
                ({ ((*new_bits.borrow()) << 1) } | {
                    ((elem!((*new_bits_array.borrow()), (*i.borrow())).read()) as u64)
                })
            };
            (*i.borrow_mut()).prefix_inc();
        }
        let tail: Value<usize> = Rc::new(RefCell::new(
            ((*s.borrow()).with(|__s| __s.refinement_bits_count_) & 15_usize),
        ));
        if ((*tail.borrow()) != 0) {
            let stuff_bits_count: Value<usize> = Rc::new(RefCell::new(
                ({
                    let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                        ((16_usize).wrapping_sub((*tail.borrow())) as u64),
                    ));
                    let __tmp_1: Value<u64> =
                        Rc::new(RefCell::new(((*new_bits_count.borrow()) as u64)));
                    (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                        __tmp_0.as_pointer()
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                } as usize),
            ));
            let stuff_bits: Value<u16> = Rc::new(RefCell::new(
                (((*new_bits.borrow())
                    >> ((*new_bits_count.borrow()).wrapping_sub((*stuff_bits_count.borrow()))))
                    as u16),
            ));
            (*stuff_bits.borrow_mut()) = {
                (((*stuff_bits.borrow()) as u32)
                    & ((1_u32 << (*stuff_bits_count.borrow())).wrapping_sub(1_u32)))
                    as u16
            };
            let __rhs = (({
                ({
                    ((((*s.borrow()).with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                        .to_last()
                        .read()) as i32)
                } << { (*stuff_bits_count.borrow()) })
            } | { ((*stuff_bits.borrow()) as i32) }) as u16);
            ((*s.borrow()).with(|__s| __s.refinement_bits_.as_pointer()) as Ptr<u16>)
                .to_last()
                .write(__rhs);
            (*new_bits_count.borrow_mut()) =
                { (*new_bits_count.borrow()).wrapping_sub((*stuff_bits_count.borrow())) };
            field!((*s.borrow()), refinement_bits_count_).write({
                ((*s.borrow()).with(|__s| __s.refinement_bits_count_))
                    .wrapping_add((*stuff_bits_count.borrow()))
            });
        }
        'loop_: while ((*new_bits_count.borrow()) >= 16_usize) {
            {
                let __a1 = (((*new_bits.borrow())
                    >> ((*new_bits_count.borrow()).wrapping_sub(16_usize)))
                    as u16);
                (*(*s.borrow())
                    .with(|__s| __s.refinement_bits_.clone())
                    .borrow_mut())
                .push(__a1)
            };
            (*new_bits_count.borrow_mut()) = { (*new_bits_count.borrow()).wrapping_sub(16_usize) };
            field!((*s.borrow()), refinement_bits_count_).write({
                ((*s.borrow()).with(|__s| __s.refinement_bits_count_)).wrapping_add(16_usize)
            });
        }
        if ((*new_bits_count.borrow()) != 0) {
            {
                let __a1 = (((*new_bits.borrow())
                    & (((1_u32 << (*new_bits_count.borrow())).wrapping_sub(1_u32)) as u64))
                    as u16);
                (*(*s.borrow())
                    .with(|__s| __s.refinement_bits_.clone())
                    .borrow_mut())
                .push(__a1)
            };
            field!((*s.borrow()), refinement_bits_count_).write({
                ((*s.borrow()).with(|__s| __s.refinement_bits_count_))
                    .wrapping_add((*new_bits_count.borrow()))
            });
        }
    }
    if ({ (*s.borrow()).with(|__s| __s.refinement_bits_count_) } > {
        ((32767 * (kDCTBlockSize_3.with(|rc| *rc.borrow()) - 1)) as usize)
    }) {
        return false;
    }
    if ((*s.borrow()).with(|__s| __s.eob_run_) == 32767) {
        ({ Flush_238((*s.borrow()).clone(), (*bw.borrow()).clone()) });
    }
    return true;
}
pub fn BuildHuffmanCodeTable_240(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    table: Ptr<brunsli_HuffmanCodeTable>,
) -> bool {
    let table: Value<Ptr<brunsli_HuffmanCodeTable>> = Rc::new(RefCell::new(table));
    let huff_code: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..256).map(|_| 0_i32).collect::<Box<[i32]>>(),
    ));
    let huff_size: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..257).map(|_| 0_u32).collect::<Box<[u32]>>(),
    ));
    let p: Value<i32> = Rc::new(RefCell::new(0));
    let l: Value<usize> = Rc::new(RefCell::new(1_usize));
    'loop_: while ((*l.borrow()) <= (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize)) {
        let i: Value<i32> = Rc::new(RefCell::new(
            (elem!(
                (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                (*l.borrow())
            )
            .read()),
        ));
        if (((*p.borrow()) + (*i.borrow()))
            > (kJpegHuffmanAlphabetSize_8.with(|rc| *rc.borrow()) + 1))
        {
            return false;
        }
        'loop_: while ((*i.borrow_mut()).postfix_dec() != 0) {
            (*huff_size.borrow_mut())[((*p.borrow_mut()).postfix_inc()) as usize] =
                ((*l.borrow()) as u32);
        }
        (*l.borrow_mut()).prefix_inc();
    }
    if ((*p.borrow()) == 0) {
        return true;
    }
    let last_p: Value<i32> = Rc::new(RefCell::new(((*p.borrow()) - 1)));
    (*huff_size.borrow_mut())[(*last_p.borrow()) as usize] = 0_u32;
    let code: Value<i32> = Rc::new(RefCell::new(0));
    let si: Value<u32> = Rc::new(RefCell::new((*huff_size.borrow())[(0) as usize]));
    (*p.borrow_mut()) = 0;
    'loop_: while ((*huff_size.borrow())[(*p.borrow()) as usize] != 0) {
        'loop_: while (((*huff_size.borrow())[(*p.borrow()) as usize]) == (*si.borrow())) {
            (*huff_code.borrow_mut())[((*p.borrow_mut()).postfix_inc()) as usize] =
                (*code.borrow());
            (*code.borrow_mut()).postfix_inc();
        }
        (*code.borrow_mut()) <<= 1;
        (*si.borrow_mut()).postfix_inc();
    }
    (*p.borrow_mut()) = 0;
    'loop_: while ((*p.borrow()) < (*last_p.borrow())) {
        let i: Value<i32> = Rc::new(RefCell::new(
            (elem!(
                (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                ((*p.borrow()) as usize)
            )
            .read()),
        ));
        elem!(
            (array_field_ptr!((*table.borrow()), depth) as Ptr::<i32>),
            (*i.borrow())
        )
        .write(((*huff_size.borrow())[(*p.borrow()) as usize] as i32));
        elem!(
            (array_field_ptr!((*table.borrow()), code) as Ptr::<i32>),
            (*i.borrow())
        )
        .write((*huff_code.borrow())[(*p.borrow()) as usize]);
        (*p.borrow_mut()).postfix_inc();
    }
    return true;
}
pub fn EncodeSOI_241(state: Ptr<brunsli_internal_dec_SerializationState>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::new_3({
        vec![255_u8, 216_u8]
    }));
    return true;
}
pub fn EncodeEOI_242(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::new_3({
        vec![255_u8, 217_u8]
    }));
    {
        let __init =
            brunsli_internal_dec_OutputChunk::new_4({ jpg.with(|__s| __s.tail_data.as_pointer()) });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    return true;
}
pub fn EncodeSOF_243(
    jpg: Ptr<brunsli_JPEGData>,
    marker: u8,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    if (((*marker.borrow()) as i32) <= 194) {
        field!((*state.borrow()), is_progressive).write((((*marker.borrow()) as i32) == 194));
    }
    let n_comps: Value<usize> = Rc::new(RefCell::new(
        (*jpg.with(|__s| __s.components.clone()).borrow()).len(),
    ));
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (8_usize).wrapping_add((3_usize).wrapping_mul((*n_comps.borrow()))),
    ));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_2({
            Some((*marker_len.borrow()).wrapping_add(2_usize))
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u8>>::decay(
            &(((*state.borrow()).with(|__s| __s.output_queue.as_pointer())
                as Ptr<brunsli_internal_dec_OutputChunk>)
                .to_last()
                .with(|__s| __s.buffer.clone())
                .as_pointer()),
        ) as Ptr<u8>),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(255_u8);
    let __rhs = (*marker.borrow());
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = (kJpegPrecision_224.with(|rc| *rc.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((jpg.with(|__s| __s.height) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = (((jpg.with(|__s| __s.height) as u32) & 255_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((jpg.with(|__s| __s.width) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = (((jpg.with(|__s| __s.width) as u32) & 255_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((*n_comps.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*n_comps.borrow())) {
        let __rhs = ({
            (*elem!(
                (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                (*i.borrow())
            )
            .upgrade()
            .deref())
            .id
        } as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        let __rhs = (({
            ({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .h_samp_factor
            } << 4_u32)
        } | {
            ({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .v_samp_factor
            })
        }) as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        let quant_idx: Value<usize> = Rc::new(RefCell::new(
            ({
                (*elem!(
                    (jpg.with(|__s| __s.components.as_pointer()) as Ptr<brunsli_JPEGComponent>),
                    (*i.borrow())
                )
                .upgrade()
                .deref())
                .quant_idx
            } as usize),
        ));
        if ({ (*quant_idx.borrow()) } >= { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            return false;
        }
        let __rhs = ({
            (*elem!(
                (jpg.with(|__s| __s.quant.as_pointer()) as Ptr<brunsli_JPEGQuantTable>),
                (*quant_idx.borrow())
            )
            .upgrade()
            .deref())
            .index
        } as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeSOS_244(
    jpg: Ptr<brunsli_JPEGData>,
    scan_info: Ptr<brunsli_JPEGScanInfo>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let n_scans: Value<usize> = Rc::new(RefCell::new(scan_info.with(|__s| __s.num_components)));
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (6_usize).wrapping_add((2_usize).wrapping_mul((*n_scans.borrow()))),
    ));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_2({
            Some((*marker_len.borrow()).wrapping_add(2_usize))
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u8>>::decay(
            &(((*state.borrow()).with(|__s| __s.output_queue.as_pointer())
                as Ptr<brunsli_internal_dec_OutputChunk>)
                .to_last()
                .with(|__s| __s.buffer.clone())
                .as_pointer()),
        ) as Ptr<u8>),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(255_u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(218_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((*n_scans.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*n_scans.borrow())) {
        let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
            .with(|__s| __s.components.as_pointer())
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset((*i.borrow()));
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
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        let __rhs =
            (({ (si.with(|__s| __s.dc_tbl_idx) << 4_u32) } + { si.with(|__s| __s.ac_tbl_idx) })
                as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = (scan_info.with(|__s| __s.Ss) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = (scan_info.with(|__s| __s.Se) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs =
        (({ (scan_info.with(|__s| __s.Ah) << 4_u32) } | { (scan_info.with(|__s| __s.Al)) }) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    return true;
}
pub fn EncodeDHT_245(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let huffman_code: Ptr<Vec<brunsli_JPEGHuffmanCode>> =
        jpg.with(|__s| __s.huffman_code.as_pointer());
    let marker_len: Value<usize> = Rc::new(RefCell::new(2_usize));
    let i: Value<usize> = Rc::new(RefCell::new(
        ((*state.borrow()).with(|__s| __s.dht_index) as usize),
    ));
    'loop_: while ({ (*i.borrow()) } < { (*huffman_code.upgrade().deref()).len() }) {
        let huff: Ptr<brunsli_JPEGHuffmanCode> =
            (Ptr::<Vec<brunsli_JPEGHuffmanCode>>::decay(&(huffman_code))
                as Ptr<brunsli_JPEGHuffmanCode>)
                .offset((*i.borrow()));
        (*marker_len.borrow_mut()) = {
            (*marker_len.borrow())
                .wrapping_add((kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize))
        };
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*j.borrow()) } < {
            (*huff.with(|__s| __s.counts.clone()).borrow()).len()
        }) {
            {
                let rhs_0 = (*marker_len.borrow()).wrapping_add(
                    ((elem!(
                        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                        (*j.borrow())
                    )
                    .read()) as usize),
                );
                (*marker_len.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).prefix_inc();
        }
        if huff.with(|__s| __s.is_last) {
            break;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    {
        let __init = brunsli_internal_dec_OutputChunk::new_2({
            Some((*marker_len.borrow()).wrapping_add(2_usize))
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u8>>::decay(
            &(((*state.borrow()).with(|__s| __s.output_queue.as_pointer())
                as Ptr<brunsli_internal_dec_OutputChunk>)
                .to_last()
                .with(|__s| __s.buffer.clone())
                .as_pointer()),
        ) as Ptr<u8>),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(255_u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(196_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    'loop_: while true {
        let huffman_code_index: Value<usize> = Rc::new(RefCell::new(
            (field!((*state.borrow()), dht_index).with_mut(|__v| __v.postfix_inc()) as usize),
        ));
        if ({ (*huffman_code_index.borrow()) } >= { (*huffman_code.upgrade().deref()).len() }) {
            return false;
        }
        let huff: Ptr<brunsli_JPEGHuffmanCode> =
            (Ptr::<Vec<brunsli_JPEGHuffmanCode>>::decay(&(huffman_code))
                as Ptr<brunsli_JPEGHuffmanCode>)
                .offset((*huffman_code_index.borrow()));
        let index: Value<usize> = Rc::new(RefCell::new((huff.with(|__s| __s.slot_id) as usize)));
        let huff_table: Value<Ptr<brunsli_HuffmanCodeTable>> =
            Rc::new(RefCell::new(Ptr::<brunsli_HuffmanCodeTable>::null()));
        if (((*index.borrow()) & 16_usize) != 0) {
            (*index.borrow_mut()) = { (*index.borrow()).wrapping_sub(16_usize) };
            (*huff_table.borrow_mut()) = (((*state.borrow())
                .with(|__s| __s.ac_huff_table.as_pointer())
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset((*index.borrow())));
        } else {
            (*huff_table.borrow_mut()) = (((*state.borrow())
                .with(|__s| __s.dc_huff_table.as_pointer())
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset((*index.borrow())));
        }
        if !({
            let _huff: Ptr<brunsli_JPEGHuffmanCode> = (huff).clone();
            let _table: Ptr<brunsli_HuffmanCodeTable> = (*huff_table.borrow()).clone();
            BuildHuffmanCodeTable_240(_huff, _table)
        }) {
            return false;
        }
        let total_count: Value<usize> = Rc::new(RefCell::new(0_usize));
        let max_length: Value<usize> = Rc::new(RefCell::new(0_usize));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*i.borrow()) } < {
            (*huff.with(|__s| __s.counts.clone()).borrow()).len()
        }) {
            if ((elem!(
                (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                (*i.borrow())
            )
            .read())
                != 0)
            {
                (*max_length.borrow_mut()) = (*i.borrow());
            }
            {
                let rhs_0 = (*total_count.borrow()).wrapping_add(
                    ((elem!(
                        (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                        (*i.borrow())
                    )
                    .read()) as usize),
                );
                (*total_count.borrow_mut()) = rhs_0
            };
            (*i.borrow_mut()).prefix_inc();
        }
        (*total_count.borrow_mut()).prefix_dec();
        let __rhs = (huff.with(|__s| __s.slot_id) as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        let i: Value<usize> = Rc::new(RefCell::new(1_usize));
        'loop_: while ((*i.borrow())
            <= (kJpegHuffmanMaxBitLength_7.with(|rc| *rc.borrow()) as usize))
        {
            let __rhs = ((if ((*i.borrow()) == (*max_length.borrow())) {
                ((elem!(
                    (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                    (*i.borrow())
                )
                .read())
                    - 1)
            } else {
                (elem!(
                    (huff.with(|__s| __s.counts.as_pointer()) as Ptr<i32>),
                    (*i.borrow())
                )
                .read())
            }) as u8);
            elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*total_count.borrow())) {
            let __rhs = ((elem!(
                (huff.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                (*i.borrow())
            )
            .read()) as u8);
            elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        if huff.with(|__s| __s.is_last) {
            break;
        }
    }
    return true;
}
pub fn EncodeDQT_246(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let marker_len: Value<i32> = Rc::new(RefCell::new(2));
    let i: Value<usize> = Rc::new(RefCell::new(
        ((*state.borrow()).with(|__s| __s.dqt_index) as usize),
    ));
    'loop_: while ({ (*i.borrow()) } < { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
        let table: Ptr<brunsli_JPEGQuantTable> = (jpg.with(|__s| __s.quant.as_pointer())
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*i.borrow()));
        (*marker_len.borrow_mut()) += (1
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
        (*i.borrow_mut()).prefix_inc();
    }
    {
        let __init = brunsli_internal_dec_OutputChunk::new_2({
            Some((((*marker_len.borrow()) + 2) as usize))
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<Vec<u8>>::decay(
            &(((*state.borrow()).with(|__s| __s.output_queue.as_pointer())
                as Ptr<brunsli_internal_dec_OutputChunk>)
                .to_last()
                .with(|__s| __s.buffer.clone())
                .as_pointer()),
        ) as Ptr<u8>),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(255_u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(219_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    let __rhs = ((((*marker_len.borrow()) as u32) & 255_u32) as u8);
    elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
    'loop_: while true {
        let idx: Value<usize> = Rc::new(RefCell::new(
            (field!((*state.borrow()), dqt_index).with_mut(|__v| __v.postfix_inc()) as usize),
        ));
        if ({ (*idx.borrow()) } >= { (*jpg.with(|__s| __s.quant.clone()).borrow()).len() }) {
            return false;
        }
        let table: Ptr<brunsli_JPEGQuantTable> = (jpg.with(|__s| __s.quant.as_pointer())
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*idx.borrow()));
        let __rhs =
            (({ (table.with(|__s| __s.precision) << 4_u32) } + { table.with(|__s| __s.index) })
                as u8);
        elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (kDCTBlockSize_3.with(|rc| *rc.borrow()) as usize)) {
            let val_idx: Value<i32> = Rc::new(RefCell::new(
                (({
                    let __idx = (*i.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                }) as i32),
            ));
            let val: Value<i32> = Rc::new(RefCell::new(
                (elem!(
                    (table.with(|__s| __s.values.as_pointer()) as Ptr<i32>),
                    ((*val_idx.borrow()) as usize)
                )
                .read()),
            ));
            if (table.with(|__s| __s.precision) != 0) {
                let __rhs = (((*val.borrow()) >> 8_u32) as u8);
                elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
            }
            let __rhs = ((((*val.borrow()) as u32) & 255_u32) as u8);
            elem!((*data.borrow()), (*pos.borrow_mut()).postfix_inc()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        if table.with(|__s| __s.is_last) {
            break;
        }
    }
    return true;
}
pub fn EncodeDRI_247(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    field!((*state.borrow()), seen_dri_marker).write(true);
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
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::move_from({
        dri_marker.as_pointer()
    }));
    return true;
}
pub fn EncodeRestart_248(marker: u8, state: Ptr<brunsli_internal_dec_SerializationState>) -> bool {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::new_3({
        vec![255_u8, (*marker.borrow())]
    }));
    return true;
}
pub fn EncodeAPP_249(
    jpg: Ptr<brunsli_JPEGData>,
    marker: u8,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    &(*marker.borrow_mut());
    let app_index: Value<usize> = Rc::new(RefCell::new(
        (field!((*state.borrow()), app_index).with_mut(|__v| __v.postfix_inc()) as usize),
    ));
    if ({ (*app_index.borrow()) } >= { (*jpg.with(|__s| __s.app_data.clone()).borrow()).len() }) {
        return false;
    }
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8] }));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.app_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset((*app_index.borrow()))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    return true;
}
pub fn EncodeCOM_250(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let com_index: Value<usize> = Rc::new(RefCell::new(
        (field!((*state.borrow()), com_index).with_mut(|__v| __v.postfix_inc()) as usize),
    ));
    if ({ (*com_index.borrow()) } >= { (*jpg.with(|__s| __s.com_data.clone()).borrow()).len() }) {
        return false;
    }
    (*(*state.borrow())
        .with(|__s| __s.output_queue.clone())
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::new_3({ vec![255_u8] }));
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.com_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset((*com_index.borrow()))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    return true;
}
pub fn EncodeInterMarkerData_251(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let index: Value<usize> = Rc::new(RefCell::new(
        (field!((*state.borrow()), data_index).with_mut(|__v| __v.postfix_inc()) as usize),
    ));
    if ({ (*index.borrow()) } >= {
        (*jpg.with(|__s| __s.inter_marker_data.clone()).borrow()).len()
    }) {
        return false;
    }
    {
        let __init = brunsli_internal_dec_OutputChunk::new_4({
            ((jpg.with(|__s| __s.inter_marker_data.as_pointer()) as Ptr<Value<Vec<u8>>>)
                .offset((*index.borrow()))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
        });
        (*(*state.borrow())
            .with(|__s| __s.output_queue.clone())
            .borrow_mut())
        .push(__init)
    };
    return true;
}
pub fn EncodeDCTBlockSequential_252(
    coeffs: Ptr<i16>,
    dc_huff: Ptr<brunsli_HuffmanCodeTable>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    num_zero_runs: i32,
    last_dc_coeff: Ptr<i16>,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let num_zero_runs: Value<i32> = Rc::new(RefCell::new(num_zero_runs));
    let last_dc_coeff: Value<Ptr<i16>> = Rc::new(RefCell::new(last_dc_coeff));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let temp2: Value<i16> = Rc::new(RefCell::new(0_i16));
    let temp: Value<i16> = Rc::new(RefCell::new(0_i16));
    (*temp2.borrow_mut()) = { (elem!((*coeffs.borrow()), 0).read()) };
    (*temp.borrow_mut()) = {
        (({ ((*temp2.borrow()) as i32) } - { (((*last_dc_coeff.borrow()).read()) as i32) }) as i16)
    };
    (*last_dc_coeff.borrow()).write({ (*temp2.borrow()) });
    (*temp2.borrow_mut()) = (*temp.borrow());
    if (((*temp.borrow()) as i32) < 0) {
        (*temp.borrow_mut()) = { (-((*temp.borrow()) as i32) as i16) };
        (*temp2.borrow_mut()).postfix_dec();
    }
    let dc_nbits: Value<i32> = Rc::new(RefCell::new(if (((*temp.borrow()) as i32) == 0) {
        0
    } else {
        (({ Log2FloorNonZero_74(((*temp.borrow()) as u32)) }) + 1)
    }));
    ({
        let _nbits: i32 = (elem!(
            (array_field_ptr!(dc_huff, depth) as Ptr::<i32>),
            (*dc_nbits.borrow())
        )
        .read());
        let _bits: u64 = ((elem!(
            (array_field_ptr!(dc_huff, code) as Ptr::<i32>),
            (*dc_nbits.borrow())
        )
        .read()) as u64);
        WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
    });
    if ((*dc_nbits.borrow()) > 0) {
        ({
            let _nbits: i32 = (*dc_nbits.borrow());
            let _bits: u64 = ((((*temp2.borrow()) as u32)
                & ((1_u32 << (*dc_nbits.borrow())).wrapping_sub(1_u32)))
                as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
    }
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*k.borrow()) < 64) {
        if ((({
            (*temp.borrow_mut()) = {
                (elem!(
                    (*coeffs.borrow()),
                    ({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read())
            };
            (*temp.borrow())
        }) as i32)
            == 0)
        {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        if (((*temp.borrow()) as i32) < 0) {
            (*temp.borrow_mut()) = { (-((*temp.borrow()) as i32) as i16) };
            (*temp2.borrow_mut()) = (!((*temp.borrow()) as i32) as i16);
        } else {
            (*temp2.borrow_mut()) = (*temp.borrow());
        }
        'loop_: while ((*r.borrow()) > 15) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
        }
        let ac_nbits: Value<i32> = Rc::new(RefCell::new(
            (({ Log2FloorNonZero_74(((*temp.borrow()) as u32)) }) + 1),
        ));
        let symbol: Value<i32> = Rc::new(RefCell::new(
            (((*r.borrow()) << 4_u32) + (*ac_nbits.borrow())),
        ));
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!(ac_huff, depth) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!(ac_huff, code) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        ({
            let _nbits: i32 = (*ac_nbits.borrow());
            let _bits: u64 =
                ((((*temp2.borrow()) as i32) & ((1 << (*ac_nbits.borrow())) - 1)) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*num_zero_runs.borrow())) {
        ({
            let _nbits: i32 = (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        (*r.borrow_mut()) -= 16;
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*r.borrow()) > 0) {
        ({
            let _nbits: i32 = (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 0).read());
            let _bits: u64 =
                ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 0).read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
    }
    return true;
}
pub fn EncodeDCTBlockProgressive_253(
    coeffs: Ptr<i16>,
    dc_huff: Ptr<brunsli_HuffmanCodeTable>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    Ss: i32,
    Se: i32,
    Al: i32,
    num_zero_runs: i32,
    coding_state: Ptr<brunsli_internal_dec_DCTCodingState>,
    last_dc_coeff: Ptr<i16>,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let Ss: Value<i32> = Rc::new(RefCell::new(Ss));
    let Se: Value<i32> = Rc::new(RefCell::new(Se));
    let Al: Value<i32> = Rc::new(RefCell::new(Al));
    let num_zero_runs: Value<i32> = Rc::new(RefCell::new(num_zero_runs));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> =
        Rc::new(RefCell::new(coding_state));
    let last_dc_coeff: Value<Ptr<i16>> = Rc::new(RefCell::new(last_dc_coeff));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let eob_run_allowed: Value<bool> = Rc::new(RefCell::new(((*Ss.borrow()) > 0)));
    let temp2: Value<i16> = Rc::new(RefCell::new(0_i16));
    let temp: Value<i16> = Rc::new(RefCell::new(0_i16));
    if ((*Ss.borrow()) == 0) {
        (*temp2.borrow_mut()) =
            { (({ ((elem!((*coeffs.borrow()), 0).read()) as i32) } >> { (*Al.borrow()) }) as i16) };
        (*temp.borrow_mut()) = {
            (({ ((*temp2.borrow()) as i32) } - { (((*last_dc_coeff.borrow()).read()) as i32) })
                as i16)
        };
        (*last_dc_coeff.borrow()).write({ (*temp2.borrow()) });
        (*temp2.borrow_mut()) = (*temp.borrow());
        if (((*temp.borrow()) as i32) < 0) {
            (*temp.borrow_mut()) = { (-((*temp.borrow()) as i32) as i16) };
            (*temp2.borrow_mut()).postfix_dec();
        }
        let nbits: Value<i32> = Rc::new(RefCell::new(if (((*temp.borrow()) as i32) == 0) {
            0
        } else {
            (({ Log2FloorNonZero_74(((*temp.borrow()) as u32)) }) + 1)
        }));
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!(dc_huff, depth) as Ptr::<i32>),
                (*nbits.borrow())
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!(dc_huff, code) as Ptr::<i32>),
                (*nbits.borrow())
            )
            .read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        if ((*nbits.borrow()) > 0) {
            ({
                let _nbits: i32 = (*nbits.borrow());
                let _bits: u64 =
                    ((((*temp2.borrow()) as i32) & ((1 << (*nbits.borrow())) - 1)) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
        }
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        if ((({
            (*temp.borrow_mut()) = {
                (elem!(
                    (*coeffs.borrow()),
                    ({
                        let __idx = (*k.borrow()) as usize;
                        kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                    })
                )
                .read())
            };
            (*temp.borrow())
        }) as i32)
            == 0)
        {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        if (((*temp.borrow()) as i32) < 0) {
            (*temp.borrow_mut()) = { (-((*temp.borrow()) as i32) as i16) };
            (*temp.borrow_mut()) = { (((*temp.borrow()) as i32) >> (*Al.borrow())) as i16 };
            (*temp2.borrow_mut()) = (!((*temp.borrow()) as i32) as i16);
        } else {
            (*temp.borrow_mut()) = { (((*temp.borrow()) as i32) >> (*Al.borrow())) as i16 };
            (*temp2.borrow_mut()) = (*temp.borrow());
        }
        if (((*temp.borrow()) as i32) == 0) {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
        'loop_: while ((*r.borrow()) > 15) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
        }
        let nbits: Value<i32> = Rc::new(RefCell::new(
            (({ Log2FloorNonZero_74(((*temp.borrow()) as u32)) }) + 1),
        ));
        let symbol: Value<i32> =
            Rc::new(RefCell::new((((*r.borrow()) << 4_u32) + (*nbits.borrow()))));
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!(ac_huff, depth) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!(ac_huff, code) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        ({
            let _nbits: i32 = (*nbits.borrow());
            let _bits: u64 = ((((*temp2.borrow()) as i32) & ((1 << (*nbits.borrow())) - 1)) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).prefix_inc();
    }
    if ((*num_zero_runs.borrow()) > 0) {
        ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*num_zero_runs.borrow())) {
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if ((*r.borrow()) > 0) {
        ({
            BufferEndOfBand_239(
                (*coding_state.borrow()).clone(),
                (ac_huff).clone(),
                Ptr::<i32>::null(),
                0_usize,
                (*bw.borrow()).clone(),
            )
        });
        if !(*eob_run_allowed.borrow()) {
            ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
        }
    }
    return true;
}
pub fn EncodeRefinementBits_254(
    coeffs: Ptr<i16>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    Ss: i32,
    Se: i32,
    Al: i32,
    coding_state: Ptr<brunsli_internal_dec_DCTCodingState>,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let Ss: Value<i32> = Rc::new(RefCell::new(Ss));
    let Se: Value<i32> = Rc::new(RefCell::new(Se));
    let Al: Value<i32> = Rc::new(RefCell::new(Al));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> =
        Rc::new(RefCell::new(coding_state));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let eob_run_allowed: Value<bool> = Rc::new(RefCell::new(((*Ss.borrow()) > 0)));
    if ((*Ss.borrow()) == 0) {
        ({
            WriteBits_233(
                (*bw.borrow()).clone(),
                1,
                ((({ ((elem!((*coeffs.borrow()), 0).read()) as i32) } >> { (*Al.borrow()) }) & 1)
                    as u64),
            )
        });
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    let abs_values: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let eob: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        let abs_val: Value<i16> = Rc::new(RefCell::new(
            (((elem!(
                (*coeffs.borrow()),
                ({
                    let __idx = (*k.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                })
            )
            .read()) as i32)
                .abs() as i16),
        ));
        (*abs_values.borrow_mut())[(*k.borrow()) as usize] =
            (((*abs_val.borrow()) as i32) >> (*Al.borrow()));
        if ((*abs_values.borrow())[(*k.borrow()) as usize] == 1) {
            (*eob.borrow_mut()) = (*k.borrow());
        }
        (*k.borrow_mut()).postfix_inc();
    }
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let refinement_bits: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let refinement_bits_count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        if ((*abs_values.borrow())[(*k.borrow()) as usize] == 0) {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        'loop_: while ((*r.borrow()) > 15) && ((*k.borrow()) <= (*eob.borrow())) {
            ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
            ({
                let _nbits: i32 =
                    (elem!((array_field_ptr!(ac_huff, depth) as Ptr::<i32>), 240).read());
                let _bits: u64 =
                    ((elem!((array_field_ptr!(ac_huff, code) as Ptr::<i32>), 240).read()) as u64);
                WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ((*i.borrow()) < (*refinement_bits_count.borrow())) {
                ({
                    WriteBits_233(
                        (*bw.borrow()).clone(),
                        1,
                        ((*refinement_bits.borrow())[(*i.borrow()) as usize] as u64),
                    )
                });
                (*i.borrow_mut()).prefix_inc();
            }
            (*refinement_bits_count.borrow_mut()) = 0_usize;
        }
        if ((*abs_values.borrow())[(*k.borrow()) as usize] > 1) {
            (*refinement_bits.borrow_mut())
                [((*refinement_bits_count.borrow_mut()).postfix_inc()) as usize] =
                ((((*abs_values.borrow())[(*k.borrow()) as usize] as u32) & 1_u32) as i32);
            (*k.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
        let symbol: Value<i32> = Rc::new(RefCell::new((((*r.borrow()) << 4_u32) + 1)));
        let new_non_zero_bit: Value<i32> = Rc::new(RefCell::new(
            if (((elem!(
                (*coeffs.borrow()),
                ({
                    let __idx = (*k.borrow()) as usize;
                    kJPEGNaturalOrder_13.with(|rc| rc.borrow()[__idx])
                })
            )
            .read()) as i32)
                < 0)
            {
                0
            } else {
                1
            },
        ));
        ({
            let _nbits: i32 = (elem!(
                (array_field_ptr!(ac_huff, depth) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read());
            let _bits: u64 = ((elem!(
                (array_field_ptr!(ac_huff, code) as Ptr::<i32>),
                (*symbol.borrow())
            )
            .read()) as u64);
            WriteBits_233((*bw.borrow()).clone(), _nbits, _bits)
        });
        ({
            WriteBits_233(
                (*bw.borrow()).clone(),
                1,
                ((*new_non_zero_bit.borrow()) as u64),
            )
        });
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*refinement_bits_count.borrow())) {
            ({
                WriteBits_233(
                    (*bw.borrow()).clone(),
                    1,
                    ((*refinement_bits.borrow())[(*i.borrow()) as usize] as u64),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
        (*refinement_bits_count.borrow_mut()) = 0_usize;
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).postfix_inc();
    }
    if ((*r.borrow()) > 0) || ((*refinement_bits_count.borrow()) != 0) {
        if !({
            BufferEndOfBand_239(
                (*coding_state.borrow()).clone(),
                (ac_huff).clone(),
                (refinement_bits.as_pointer() as Ptr<i32>),
                (*refinement_bits_count.borrow()),
                (*bw.borrow()).clone(),
            )
        }) {
            return false;
        }
        if !(*eob_run_allowed.borrow()) {
            ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
        }
    }
    return true;
}
pub fn DoEncodeScan_255(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*state.borrow()).with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!((*state.borrow()), scan_state);
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*state.borrow()).with(|__s| __s.seen_dri_marker) {
            jpg.with(|__s| __s.restart_interval)
        } else {
            0
        },
    ));
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
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                ((*state.borrow()).with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write((*restart_interval.borrow()));
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
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new((field_ptr!(ss, bw))));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> =
        Rc::new(RefCell::new((field_ptr!(ss, coding_state))));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<u8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        (scan_info.with(|__s| __s.num_components) > 1_usize),
    ));
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
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.width) } * { (*h_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.height) } * { (*v_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*state.borrow()).with(|__s| __s.is_progressive),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Al)
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Ss)
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Se)
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                HasSection_194(
                    (parsing_state).clone(),
                    (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                )
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new((*has_ac.borrow())));
    let complete: Value<bool> = Rc::new(RefCell::new(if (*want_ac.borrow()) {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    }));
    let last_mcu_y: Value<i32> = Rc::new(RefCell::new(if (*complete.borrow()) {
        (*MCU_rows.borrow())
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
        } * { (*v_group.borrow()) })
    }));
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { (*last_mcu_y.borrow()) }) {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
                    let _pad_bits_end: Ptr<i32> =
                        (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({
                    EmitMarker_234(
                        (*bw.borrow()).clone(),
                        (208 + ss.with(|__s| __s.next_restart_marker)),
                    )
                });
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() + 1)
                };
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() & 7)
                };
                field!(ss, restarts_to_go).write((*restart_interval.borrow()));
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ({ (*i.borrow()) } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset((*i.borrow()));
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new(
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { (*n_blocks_y.borrow()) }) } + {
                                (*iy.borrow())
                            }),
                        ));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({
                                Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone())
                            });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            (*num_zero_runs.borrow_mut()) = {
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
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            ((c.with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as usize))),
                        ));
                        let ok: Value<bool> = Rc::new(RefCell::new(false));
                        if (0 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                        if !(*ok.borrow()) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { (*MCU_rows.borrow()) }) {
        if !((*bw.borrow()).with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
        let _pad_bits_end: Ptr<i32> = (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((*bw.borrow()).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!((*state.borrow()), scan_index).with_mut(|__v| __v.postfix_inc());
    if !((*bw.borrow()).with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn DoEncodeScan_256(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*state.borrow()).with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!((*state.borrow()), scan_state);
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*state.borrow()).with(|__s| __s.seen_dri_marker) {
            jpg.with(|__s| __s.restart_interval)
        } else {
            0
        },
    ));
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
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                ((*state.borrow()).with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write((*restart_interval.borrow()));
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
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new((field_ptr!(ss, bw))));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> =
        Rc::new(RefCell::new((field_ptr!(ss, coding_state))));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<u8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        (scan_info.with(|__s| __s.num_components) > 1_usize),
    ));
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
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.width) } * { (*h_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.height) } * { (*v_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*state.borrow()).with(|__s| __s.is_progressive),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Al)
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Ss)
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Se)
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                HasSection_194(
                    (parsing_state).clone(),
                    (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                )
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new((*has_ac.borrow())));
    let complete: Value<bool> = Rc::new(RefCell::new(if (*want_ac.borrow()) {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    }));
    let last_mcu_y: Value<i32> = Rc::new(RefCell::new(if (*complete.borrow()) {
        (*MCU_rows.borrow())
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
        } * { (*v_group.borrow()) })
    }));
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { (*last_mcu_y.borrow()) }) {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
                    let _pad_bits_end: Ptr<i32> =
                        (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({
                    EmitMarker_234(
                        (*bw.borrow()).clone(),
                        (208 + ss.with(|__s| __s.next_restart_marker)),
                    )
                });
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() + 1)
                };
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() & 7)
                };
                field!(ss, restarts_to_go).write((*restart_interval.borrow()));
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ({ (*i.borrow()) } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset((*i.borrow()));
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new(
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { (*n_blocks_y.borrow()) }) } + {
                                (*iy.borrow())
                            }),
                        ));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({
                                Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone())
                            });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            (*num_zero_runs.borrow_mut()) = {
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
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            ((c.with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as usize))),
                        ));
                        let ok: Value<bool> = Rc::new(RefCell::new(false));
                        if (1 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                        if !(*ok.borrow()) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { (*MCU_rows.borrow()) }) {
        if !((*bw.borrow()).with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
        let _pad_bits_end: Ptr<i32> = (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((*bw.borrow()).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!((*state.borrow()), scan_index).with_mut(|__v| __v.postfix_inc());
    if !((*bw.borrow()).with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn DoEncodeScan_257(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*state.borrow()).with(|__s| __s.scan_index) as usize));
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = field_ptr!((*state.borrow()), scan_state);
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*state.borrow()).with(|__s| __s.seen_dri_marker) {
            jpg.with(|__s| __s.restart_interval)
        } else {
            0
        },
    ));
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
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        ({
            BitWriterInit_228(
                (field_ptr!(ss, bw)),
                ((*state.borrow()).with(|__s| __s.output_queue.as_pointer())),
            )
        });
        ({ DCTCodingStateInit_237((field_ptr!(ss, coding_state))) });
        field!(ss, restarts_to_go).write((*restart_interval.borrow()));
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
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new((field_ptr!(ss, bw))));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> =
        Rc::new(RefCell::new((field_ptr!(ss, coding_state))));
    if !((ss.with(|__s| __s.stage) as i32)
        == (brunsli_internal_dec_EncodeScanState_Stage_BODY as i32))
    {
        ({
            BrunsliDumpAndAbort_79(
                Ptr::<u8>::from_string_literal(b"jpeg_data_writer.cc"),
                741,
                Ptr::<u8>::from_string_literal(b"DoEncodeScan"),
            )
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        (scan_info.with(|__s| __s.num_components) > 1_usize),
    ));
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
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.h_samp_factor)
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        base_component.with(|__s| __s.v_samp_factor)
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.width) } * { (*h_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_h_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = ({ jpg.with(|__s| __s.height) } * { (*v_group.borrow()) });
            let _b: i32 = (8 * jpg.with(|__s| __s.max_v_samp_factor));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*state.borrow()).with(|__s| __s.is_progressive),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Al)
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Ss)
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Se)
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        (parsing_state.with(|__s| __s.stage) == brunsli_internal_dec_Stage_DONE),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                HasSection_194(
                    (parsing_state).clone(),
                    (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                )
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    let complete_dc: Value<bool> = Rc::new(RefCell::new((*has_ac.borrow())));
    let complete: Value<bool> = Rc::new(RefCell::new(if (*want_ac.borrow()) {
        (*complete_ac.borrow())
    } else {
        (*complete_dc.borrow())
    }));
    let last_mcu_y: Value<i32> = Rc::new(RefCell::new(if (*complete.borrow()) {
        (*MCU_rows.borrow())
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
        } * { (*v_group.borrow()) })
    }));
    'loop_: while ({ ss.with(|__s| __s.mcu_y) } < { (*last_mcu_y.borrow()) }) {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0) && (ss.with(|__s| __s.restarts_to_go) == 0) {
                ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
                if !({
                    let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
                    let _pad_bits_end: Ptr<i32> =
                        (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
                    JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus_ERROR;
                }
                ({
                    EmitMarker_234(
                        (*bw.borrow()).clone(),
                        (208 + ss.with(|__s| __s.next_restart_marker)),
                    )
                });
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() + 1)
                };
                {
                    let _ptr = field!(ss, next_restart_marker);
                    _ptr.write(_ptr.read() & 7)
                };
                field!(ss, restarts_to_go).write((*restart_interval.borrow()));
                {
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                    ((array_field_ptr!(ss, last_dc_coeff) as Ptr<i16>) as Ptr<i16>).to_any()
                };
            }
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ({ (*i.borrow()) } < { scan_info.with(|__s| __s.num_components) }) {
                let si: Ptr<brunsli_JPEGComponentScanInfo> = (scan_info
                    .with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset((*i.borrow()));
                let c: Ptr<brunsli_JPEGComponent> = (jpg.with(|__s| __s.components.as_pointer())
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((si.with(|__s| __s.comp_idx) as usize));
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.dc_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.dc_tbl_idx) as usize));
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> = ((*state.borrow())
                    .with(|__s| __s.ac_huff_table.as_pointer())
                    as Ptr<brunsli_HuffmanCodeTable>)
                    .offset((si.with(|__s| __s.ac_tbl_idx) as usize));
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.v_samp_factor)
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    c.with(|__s| __s.h_samp_factor)
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new(
                            ({ ({ ss.with(|__s| __s.mcu_y) } * { (*n_blocks_y.borrow()) }) } + {
                                (*iy.borrow())
                            }),
                        ));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul(c.with(|__s| __s.width_in_blocks)))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_reset_point)
                        }) {
                            ({
                                Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone())
                            });
                            field!(ss, next_reset_point)
                                .write(({ (*get_next_reset_point.borrow()).call() }).clone());
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if ({ ss.with(|__s| __s.block_scan_index) } == {
                            ss.with(|__s| __s.next_extra_zero_run_index)
                        }) {
                            (*num_zero_runs.borrow_mut()) = {
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
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            ((c.with(|__s| __s.coeffs.as_pointer()) as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as usize))),
                        ));
                        let ok: Value<bool> = Rc::new(RefCell::new(false));
                        if (2 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _last_dc_coeff: Ptr<i16> = (array_field_ptr!(ss, last_dc_coeff)
                                    as Ptr<i16>)
                                    .offset((si.with(|__s| __s.comp_idx) as i32) as isize);
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _coding_state: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
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
                        if !(*ok.borrow()) {
                            return brunsli_internal_dec_SerializationStatus_ERROR;
                        }
                        field!(ss, block_scan_index).with_mut(|__v| __v.prefix_inc());
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            field!(ss, restarts_to_go).with_mut(|__v| __v.prefix_dec());
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        field!(ss, mcu_y).with_mut(|__v| __v.prefix_inc());
    }
    if ({ ss.with(|__s| __s.mcu_y) } < { (*MCU_rows.borrow()) }) {
        if !((*bw.borrow()).with(|__s| __s.healthy)) {
            return brunsli_internal_dec_SerializationStatus_ERROR;
        }
        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
    }
    ({ Flush_238((*coding_state.borrow()).clone(), (*bw.borrow()).clone()) });
    if !({
        let _pad_bits: Ptr<Ptr<i32>> = (field_ptr!((*state.borrow()), pad_bits));
        let _pad_bits_end: Ptr<i32> = (*state.borrow()).with(|__s| __s.pad_bits_end.clone());
        JumpToByteBoundary_235((*bw.borrow()).clone(), _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    ({ BitWriterFinish_236((*bw.borrow()).clone()) });
    field!(ss, stage).write(brunsli_internal_dec_EncodeScanState_Stage_HEAD);
    field!((*state.borrow()), scan_index).with_mut(|__v| __v.postfix_inc());
    if !((*bw.borrow()).with(|__s| __s.healthy)) {
        return brunsli_internal_dec_SerializationStatus_ERROR;
    }
    return brunsli_internal_dec_SerializationStatus_DONE;
}
pub fn EncodeScan_258(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = (jpg.with(|__s| __s.scan_info.as_pointer())
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*state.borrow()).with(|__s| __s.scan_index) as usize));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*state.borrow()).with(|__s| __s.is_progressive),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Al)
    } else {
        0
    }));
    let Ah: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Ah)
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Ss)
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        scan_info.with(|__s| __s.Se)
    } else {
        63
    }));
    let need_sequential: Value<bool> = Rc::new(RefCell::new(
        (!(*is_progressive.borrow()))
            || (((((*Ah.borrow()) == 0) && ((*Al.borrow()) == 0)) && ((*Ss.borrow()) == 0))
                && ((*Se.borrow()) == 63)),
    ));
    if (*need_sequential.borrow()) {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            DoEncodeScan_255(_jpg, _parsing_state, _state)
        });
    } else if ((*Ah.borrow()) == 0) {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            DoEncodeScan_256(_jpg, _parsing_state, _state)
        });
    } else {
        return ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            DoEncodeScan_257(_jpg, _parsing_state, _state)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn SerializeSection_259(
    marker: u8,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_SerializationStatus {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let to_status: Value<FnPtr<fn(bool) -> brunsli_internal_dec_SerializationStatus>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(bool) -> brunsli_internal_dec_SerializationStatus,
        >::new(
            |result: bool| -> brunsli_internal_dec_SerializationStatus {
                {
                    let result: Value<bool> = Rc::new(RefCell::new(result));
                    return if (*result.borrow()) {
                        brunsli_internal_dec_SerializationStatus_DONE
                    } else {
                        brunsli_internal_dec_SerializationStatus_ERROR
                    };
                }
            },
        )));
    'switch: {
        match { ((*marker.borrow()) as i32) } {
            __v if __v == 192 || __v == 193 || __v == 194 || __v == 201 || __v == 202 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _marker: u8 = (*marker.borrow());
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                return ({
                    (*to_status.borrow()).call(
                        ({ EncodeRestart_248((*marker.borrow()), (*state.borrow()).clone()) }),
                    )
                });
            }
            __v if __v == 217 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
                            EncodeEOI_242(_jpg, _state)
                        }),
                    )
                });
            }
            __v if __v == 218 => {
                return ({
                    let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                    let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                    let _state: Ptr<brunsli_internal_dec_SerializationState> =
                        (*state.borrow()).clone();
                    EncodeScan_258(_jpg, _parsing_state, _state)
                });
            }
            __v if __v == 219 => {
                return ({
                    (*to_status.borrow()).call(
                        ({
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                            let _marker: u8 = (*marker.borrow());
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
                            let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                (*state.borrow()).clone();
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
    in_: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
    available_out: Ptr<usize>,
    next_out: Ptr<Ptr<u8>>,
) {
    let in_: Value<Ptr<Vec<brunsli_internal_dec_OutputChunk>>> = Rc::new(RefCell::new(in_));
    let available_out: Value<Ptr<usize>> = Rc::new(RefCell::new(available_out));
    let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
    'loop_: while (((*available_out.borrow()).read()) > 0_usize) {
        if (*(*in_.borrow()).upgrade().deref()).is_empty() {
            return;
        }
        let chunk: Ptr<brunsli_internal_dec_OutputChunk> =
            (Ptr::<Vec<brunsli_internal_dec_OutputChunk>>::decay(&(*in_.borrow()))
                as Ptr<brunsli_internal_dec_OutputChunk>);
        let to_copy: Value<usize> = Rc::new(RefCell::new(
            ({
                let __tmp_0: Value<u64> =
                    Rc::new(RefCell::new((((*available_out.borrow()).read()) as u64)));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new((chunk.with(|__s| __s.len) as u64)));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            } as usize),
        ));
        if ((*to_copy.borrow()) > 0_usize) {
            {
                ((*next_out.borrow()).read()).to_any().memcpy(
                    &(chunk.with(|__s| __s.next.clone()) as Ptr<u8>).to_any(),
                    (*to_copy.borrow()) as usize,
                );
                ((*next_out.borrow()).read()).to_any()
            };
            let __rhs = (*to_copy.borrow());
            {
                let _ptr = (*next_out.borrow()).clone();
                _ptr.write(_ptr.read() + __rhs)
            };
            (*available_out.borrow())
                .write({ ((*available_out.borrow()).read()).wrapping_sub((*to_copy.borrow())) });
            {
                let _ptr = field!(chunk, next);
                _ptr.write(_ptr.read() + (*to_copy.borrow()))
            };
            field!(chunk, len)
                .write({ (chunk.with(|__s| __s.len)).wrapping_sub((*to_copy.borrow())) });
        }
        if (chunk.with(|__s| __s.len) == 0_usize) {
            (*in_.borrow())
                .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.remove(0));
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
        let status: Value<brunsli_internal_dec_SerializationStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (state.as_pointer());
                let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                let _available_out: Ptr<usize> = (available_out.as_pointer());
                let _next_out: Ptr<Ptr<u8>> = (next_out.as_pointer());
                SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
            }),
        ));
        if ((*status.borrow()) != brunsli_internal_dec_SerializationStatus_DONE)
            && ((*status.borrow()) != brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT)
        {
            return false;
        }
        let to_write: Value<usize> = Rc::new(RefCell::new(
            (((*buffer.borrow()).len() as u64).wrapping_sub(((*available_out.borrow()) as u64))
                as usize),
        ));
        if !({
            brunsli_JPEGOutputImpl::Write(
                &out.as_pointer(),
                (buffer.as_pointer() as Ptr<u8>),
                (*to_write.borrow()),
            )
        }) {
            return false;
        }
        if ((*status.borrow()) == brunsli_internal_dec_SerializationStatus_DONE) {
            return true;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn SerializeJpeg_206(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
    available_out: Ptr<usize>,
    next_out: Ptr<Ptr<u8>>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available_out: Value<Ptr<usize>> = Rc::new(RefCell::new(available_out));
    let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
    let ss: Ptr<brunsli_internal_dec_SerializationState> = field_ptr!(
        (*state.borrow())
            .with(|__s| __s.internal.clone())
            .as_pointer(),
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
                        (available_out.read()).clone(),
                        (next_out.read()).clone(),
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
                    let can_start_serialization: Value<bool> = Rc::new(RefCell::new(
                        ((*state.borrow()).with(|__s| __s.stage)
                            == brunsli_internal_dec_Stage_DONE),
                    ));
                    if ({
                        HasSection_194(
                            (*state.borrow()).clone(),
                            (kBrunsliDCDataTag_36.with(|rc| *rc.borrow()) as u32),
                        )
                    }) || ({
                        HasSection_194(
                            (*state.borrow()).clone(),
                            (kBrunsliACDataTag_37.with(|rc| *rc.borrow()) as u32),
                        )
                    }) {
                        (*can_start_serialization.borrow_mut()) = true;
                    }
                    if !(*can_start_serialization.borrow()) {
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
                    let marker: Value<u8> = Rc::new(RefCell::new(
                        (elem!(
                            (jpg.with(|__s| __s.marker_order.as_pointer()) as Ptr<u8>),
                            ss.with(|__s| __s.section_index)
                        )
                        .read()),
                    ));
                    let status: Value<brunsli_internal_dec_SerializationStatus> =
                        Rc::new(RefCell::new(
                            ({
                                let _marker: u8 = (*marker.borrow());
                                let _parsing_state: Ptr<brunsli_internal_dec_State> =
                                    (*state.borrow()).clone();
                                let _state: Ptr<brunsli_internal_dec_SerializationState> =
                                    (ss).clone();
                                let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                                SerializeSection_259(_marker, _parsing_state, _state, _jpg)
                            }),
                        ));
                    if ((*status.borrow()) == brunsli_internal_dec_SerializationStatus_ERROR) {
                        if true {
                        } else {
                            write!(libcc2rs::cerr(), "Failed to encode marker ",);
                            libcc2rs::cerr()
                                .write_all(&([(&[(*marker.borrow()) as u8] as &[u8])].concat()));
                            write!(libcc2rs::cerr(), "\n",);
                        }
                        field!(ss, stage)
                            .write(brunsli_internal_dec_SerializationState_Stage_ERROR);
                        break;
                    }
                    ({ (*maybe_push_output.borrow()).call() });
                    if ((*status.borrow())
                        == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT)
                    {
                        return brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT;
                    } else if ((*status.borrow()) != brunsli_internal_dec_SerializationStatus_DONE)
                    {
                        if !(false) {
                            ({
                                BrunsliDumpAndAbort_79(
                                    Ptr::<u8>::from_string_literal(b"jpeg_data_writer.cc"),
                                    1073,
                                    Ptr::<u8>::from_string_literal(b"SerializeJpeg"),
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
        let __this: Value<brunsli_internal_dec_State> = Rc::new(RefCell::new(Self {
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
        }));
        let this: Ptr<brunsli_internal_dec_State> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl brunsli_internal_dec_State {}
impl brunsli_internal_dec_State {}
pub fn HasSection_194(state: Ptr<brunsli_internal_dec_State>, tag: u32) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let tag: Value<u32> = Rc::new(RefCell::new(tag));
    return (({
        {
            (*(*state.borrow())
                .with(|__s| __s.internal.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .section
            .tags_met
        }
    } & { (1_u32 << (*tag.borrow())) })
        != 0);
}
pub fn StringWriter_262(data: AnyPtr, buf: Ptr<u8>, count: usize) -> usize {
    let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let count: Value<usize> = Rc::new(RefCell::new(count));
    let output: Value<Ptr<Vec<u8>>> =
        Rc::new(RefCell::new((*data.borrow()).reinterpret_cast::<Vec<u8>>()));
    {
        ((*output.borrow()).clone() as Ptr<Vec<u8>>).with_mut(|__v: &mut Vec<u8>| {
            __v.pop();
            __v.extend(
                (*buf.borrow())
                    .reinterpret_cast::<u8>()
                    .map(|c| c.read())
                    .take((*count.borrow()) as usize),
            );
            __v.push(0);
        });
        ((*output.borrow()).clone() as Ptr<Vec<u8>>)
    };
    return (*count.borrow());
}
pub fn ReadFileInternal_263(file: Ptr<CFile>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(file));
    let content: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(content));
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
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| __v.pop());
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| {
            __v.resize(((*input_size.borrow()) as usize) as usize, 0)
        });
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(0))
    };
    let read_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*read_pos.borrow()) } < {
        ((*(*content.borrow()).upgrade().deref()).len() - 1)
    }) {
        let bytes_read: Value<usize> = Rc::new(RefCell::new({
            let __a0 = ((if (*read_pos.borrow()) as usize
                >= (*((*content.borrow()).clone() as Ptr<Vec<u8>>)
                    .upgrade()
                    .deref())
                .len()
                .saturating_sub(1)
            {
                panic!("out of bounds access")
            } else {
                ((*content.borrow()).clone() as Ptr<Vec<u8>>)
                    .decay()
                    .offset((*read_pos.borrow()) as isize)
            }) as Ptr<u8>)
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
pub fn ReadFile_264(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let content: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(content));
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(Ptr::<Vec<u8>>::decay(&(file_name)) as Ptr<u8>).to_rust_string(),
            &Ptr::<u8>::from_string_literal(b"rb").to_rust_string(),
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
        ({ ReadFileInternal_263((*file.borrow()).clone(), (*content.borrow()).clone()) }),
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
pub fn WriteFileInternal_265(file: Ptr<CFile>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(file));
    let write_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*write_pos.borrow()) } < { ((*content.upgrade().deref()).len() - 1) }) {
        let bytes_written: Value<usize> = Rc::new(RefCell::new({
            let __a0 = (((Ptr::<Vec<u8>>::decay(&(content)) as Ptr<u8>)
                .offset((*write_pos.borrow()))) as Ptr<u8>)
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
pub fn WriteFile_266(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(Ptr::<Vec<u8>>::decay(&(file_name)) as Ptr<u8>).to_rust_string(),
            &Ptr::<u8>::from_string_literal(b"wb").to_rust_string(),
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
            let _content: Ptr<Vec<u8>> = (content).clone();
            WriteFileInternal_265(_file, _content)
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
pub fn ProcessFile_267(file_name: Ptr<Vec<u8>>, outfile_name: Ptr<Vec<u8>>) -> bool {
    let input: Value<Vec<u8>> = Rc::new(RefCell::new(vec![0]));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file_name: Ptr<Vec<u8>> = (file_name).clone();
            let _content: Ptr<Vec<u8>> = (input.as_pointer());
            ReadFile_264(_file_name, _content)
        }),
    ));
    if !(*ok.borrow()) {
        return false;
    }
    let output: Value<Vec<u8>> = Rc::new(RefCell::new(vec![0]));
    {
        let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::new()));
        let input_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
            (input.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
        ));
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                BrunsliDecodeJpeg_204(
                    (*input_data.borrow()).clone(),
                    ((*input.borrow()).len() - 1),
                    (jpg.as_pointer()),
                )
            }),
        ));
        (*ok.borrow_mut()) =
            (((*status.borrow()) as i32) == (brunsli_BrunsliStatus_BRUNSLI_OK as i32));
        if ({ (*jpg.borrow()).version } != kFallbackVersion_2.with(|rc| *rc.borrow())) {
            {
                (*input.borrow_mut()).clear();
                (*input.borrow_mut()).push(0)
            };
            (*input.borrow_mut()).shrink_to_fit();
        }
        if !(*ok.borrow()) {
            eprintln!("Failed to parse Brunsli input.");
            return false;
        }
        let writer: Value<brunsli_JPEGOutput> = Rc::new(RefCell::new(brunsli_JPEGOutput::new(
            { FnPtr::<fn(AnyPtr, Ptr<u8>, usize) -> usize>::new(StringWriter_262) },
            { ((output.as_pointer()) as Ptr<Vec<u8>>).to_any() },
        )));
        (*ok.borrow_mut()) = ({ WriteJpeg_261(jpg.as_pointer(), (*writer.borrow()).clone()) });
        if !(*ok.borrow()) {
            eprintln!("Failed to serialize JPEG data.");
            return false;
        }
    }
    (*ok.borrow_mut()) = ({
        let _file_name: Ptr<Vec<u8>> = (outfile_name).clone();
        let _content: Ptr<Vec<u8>> = output.as_pointer();
        WriteFile_266(_file_name, _content)
    });
    return (*ok.borrow());
}
pub fn main() {
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
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
fn main_0(argc: i32, argv: Ptr<Ptr<u8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(argv));
    if ((*argc.borrow()) != 2) && ((*argc.borrow()) != 3) {
        eprintln!("Usage: dbrunsli FILE [OUTPUT_FILE, default=FILE.jpg]");
        return 1;
    }
    let file_name: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __bytes = (elem!((*argv.borrow()), 1).read()).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    if (*file_name.borrow()).len() <= 1 {
        eprintln!("Empty input file name.");
        return 1;
    }
    let outfile_name: Value<Vec<u8>> = Rc::new(RefCell::new(if ((*argc.borrow()) == 2) {
        {
            let mut r = (*file_name.borrow()).clone();
            r.pop();
            Ptr::<u8>::from_string_literal(b".jpg").with_c_str(|__s| r.extend_from_slice(__s));
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
        ({ ProcessFile_267(file_name.as_pointer(), outfile_name.as_pointer()) }),
    ));
    return if (*ok.borrow()) { 0 } else { 1 };
}
pub trait brunsli_ANSDecoderImpl {
    fn Init(&self, in_: Ptr<brunsli_WordSource>);
    fn ReadSymbol(&self, code: Ptr<brunsli_ANSDecodingData>, in_: Ptr<brunsli_WordSource>) -> i32;
    fn CheckCRC(&self) -> bool;
}
impl brunsli_ANSDecoderImpl for Ptr<brunsli_ANSDecoder> {
    fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        field!((*self), state_)
            .write((({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32));
        let __rhs = ({ ((*self).with(|__s| __s.state_) << 16_u32) } | {
            (({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32)
        });
        field!((*self), state_).write(__rhs);
    }
    fn ReadSymbol(&self, code: Ptr<brunsli_ANSDecodingData>, in_: Ptr<brunsli_WordSource>) -> i32 {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        let res: Value<u32> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.state_)
                & ((BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow()) - 1) as u32)),
        ));
        let s: Ptr<brunsli_ANSSymbolInfo> = (array_field_ptr!(code, map_)
            as Ptr<brunsli_ANSSymbolInfo>)
            .offset((*res.borrow()) as isize);
        field!((*self), state_).write({
            ((s.with(|__s| __s.freq_) as u32).wrapping_mul(
                ((*self).with(|__s| __s.state_)
                    >> BRUNSLI_ANS_LOG_TAB_SIZE_0.with(|rc| *rc.borrow())),
            ))
            .wrapping_add((s.with(|__s| __s.offset_) as u32))
        });
        if ((*self).with(|__s| __s.state_) < (1_u32 << 16_u32)) {
            let __rhs = ({ ((*self).with(|__s| __s.state_) << 16_u32) } | {
                (({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32)
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
        let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ({ (*i.borrow()) } < { (*counts.upgrade().deref()).len() }) {
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ({ (*j.borrow()) } < {
                ((elem!(
                    (Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>),
                    (*i.borrow())
                )
                .read()) as usize)
            }) {
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        (*pos.borrow())
                    ),
                    symbol_
                )
                .write(((*i.borrow()) as u8));
                let __rhs = ((elem!(
                    (Ptr::<Vec<u32>>::decay(&(counts)) as Ptr<u32>),
                    (*i.borrow())
                )
                .read()) as u16);
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        (*pos.borrow())
                    ),
                    freq_
                )
                .write(__rhs);
                field!(
                    elem!(
                        (array_field_ptr!((*self), map_) as Ptr<brunsli_ANSSymbolInfo>),
                        (*pos.borrow())
                    ),
                    offset_
                )
                .write(((*j.borrow()) as u16));
                {
                    (*j.borrow_mut()).prefix_inc();
                    (*pos.borrow_mut()).prefix_inc()
                };
            }
            (*i.borrow_mut()).prefix_inc();
        }
        return ((*pos.borrow()) == (BRUNSLI_ANS_TAB_SIZE_1.with(|rc| *rc.borrow()) as usize));
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
    fn reserve(&self, limit: usize) {
        let limit: Value<usize> = Rc::new(RefCell::new(limit));
        if ((*self).with(|__s| __s.capacity) < (*limit.borrow())) {
            field!((*self), capacity).write((*limit.borrow()));
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
    fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        field!((*self), low_).write(0_u32);
        field!((*self), high_).write(!0_u32);
        field!((*self), value_)
            .write((({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32));
        let __rhs = ({ ((*self).with(|__s| __s.value_) << 16_u32) } | {
            (({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32)
        });
        field!((*self), value_).write(__rhs);
    }
    fn ReadBit(&self, prob: i32, in_: Ptr<brunsli_WordSource>) -> i32 {
        let prob: Value<i32> = Rc::new(RefCell::new(prob));
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        let diff: Value<u32> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.high_)).wrapping_sub((*self).with(|__s| __s.low_)),
        ));
        let split: Value<u32> = Rc::new(RefCell::new(
            ((((*self).with(|__s| __s.low_) as u64).wrapping_add(
                ((((*diff.borrow()) as u64).wrapping_mul(((*prob.borrow()) as u64))) >> 8_u32),
            )) as u32),
        ));
        let bit: Value<i32> = Rc::new(RefCell::new(0_i32));
        if ((*self).with(|__s| __s.value_) > (*split.borrow())) {
            field!((*self), low_).write((*split.borrow()).wrapping_add(1_u32));
            (*bit.borrow_mut()) = 1;
        } else {
            field!((*self), high_).write((*split.borrow()));
            (*bit.borrow_mut()) = 0;
        }
        if ((((*self).with(|__s| __s.low_) ^ (*self).with(|__s| __s.high_)) >> 16_u32) == 0_u32) {
            let __rhs = ({ ((*self).with(|__s| __s.value_) << 16_u32) } | {
                (({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32)
            });
            field!((*self), value_).write(__rhs);
            {
                let _ptr = field!((*self), low_);
                _ptr.write(_ptr.read() << 16_u32)
            };
            {
                let _ptr = field!((*self), high_);
                _ptr.write(_ptr.read() << 16_u32)
            };
            {
                let _ptr = field!((*self), high_);
                _ptr.write(_ptr.read() | 65535_u32)
            };
        }
        return (*bit.borrow());
    }
}
pub trait brunsli_BitSourceImpl {
    fn Init(&self, in_: Ptr<brunsli_WordSource>);
    fn ReadBits(&self, nbits: i32, in_: Ptr<brunsli_WordSource>) -> u32;
    fn Finish(&self) -> bool;
}
impl brunsli_BitSourceImpl for Ptr<brunsli_BitSource> {
    fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        field!((*self), val_)
            .write((({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32));
        field!((*self), bit_pos_).write(0);
    }
    fn ReadBits(&self, nbits: i32, in_: Ptr<brunsli_WordSource>) -> u32 {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        if (((*self).with(|__s| __s.bit_pos_) + (*nbits.borrow())) > 16) {
            let new_bits: Value<u32> = Rc::new(RefCell::new(
                (({ brunsli_WordSourceImpl::GetNextWord(&(*in_.borrow())) }) as u32),
            ));
            {
                let _ptr = field!((*self), val_);
                _ptr.write(_ptr.read() | ((*new_bits.borrow()) << 16))
            };
        }
        let result: Value<u32> = Rc::new(RefCell::new(
            (((*self).with(|__s| __s.val_) >> (*self).with(|__s| __s.bit_pos_))
                & (({
                    let __idx = (*nbits.borrow()) as usize;
                    kBitMask_120.with(|rc| rc.borrow()[__idx])
                }) as u32)),
        ));
        {
            let _ptr = field!((*self), bit_pos_);
            _ptr.write(_ptr.read() + (*nbits.borrow()))
        };
        if ((*self).with(|__s| __s.bit_pos_) > 16) {
            {
                let _ptr = field!((*self), bit_pos_);
                _ptr.write(_ptr.read() - 16)
            };
            {
                let _ptr = field!((*self), val_);
                _ptr.write(_ptr.read() >> 16)
            };
        }
        return (*result.borrow());
    }
    fn Finish(&self) -> bool {
        let n_bits: Value<usize> = Rc::new(RefCell::new(
            ((16 - (*self).with(|__s| __s.bit_pos_)) as usize),
        ));
        if ((*n_bits.borrow()) > 0_usize) {
            let padding_bits: Value<i32> = Rc::new(RefCell::new(
                ((((*self).with(|__s| __s.val_) >> (*self).with(|__s| __s.bit_pos_))
                    & (({
                        let __idx = (*n_bits.borrow()) as usize;
                        kBitMask_120.with(|rc| rc.borrow()[__idx])
                    }) as u32)) as i32),
            ));
            if ((*padding_bits.borrow()) != 0) {
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
        available_in: Ptr<usize>,
        next_in: Ptr<Ptr<u8>>,
        available_out: Ptr<usize>,
        next_out: Ptr<Ptr<u8>>,
    ) -> brunsli_BrunsliDecoder_Status {
        let available_in: Value<Ptr<usize>> = Rc::new(RefCell::new(available_in));
        let next_in: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_in));
        let available_out: Value<Ptr<usize>> = Rc::new(RefCell::new(available_out));
        let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
        let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(
            (*self).with(|__s| __s.jpg_.clone()).as_pointer(),
        ));
        if !(!(*jpg.borrow()).is_null()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2511,
                    Ptr::<u8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(
            (*self).with(|__s| __s.state_.clone()).as_pointer(),
        ));
        if !(!(*state.borrow()).is_null()) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2513,
                    Ptr::<u8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        field!((*state.borrow()), data).write({ ((*next_in.borrow()).read()).clone() });
        field!((*state.borrow()), pos).write(0_usize);
        field!((*state.borrow()), len).write({ ((*available_in.borrow()).read()) });
        let parse_status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({ ProcessJpeg_203((*state.borrow()).clone(), (*jpg.borrow()).clone()) }),
        ));
        let consumed_bytes: Value<usize> =
            Rc::new(RefCell::new((*state.borrow()).with(|__s| __s.pos)));
        (*available_in.borrow())
            .write({ ((*available_in.borrow()).read()).wrapping_sub((*consumed_bytes.borrow())) });
        let __rhs = (*consumed_bytes.borrow());
        {
            let _ptr = (*next_in.borrow()).clone();
            _ptr.write(_ptr.read() + __rhs)
        };
        if (((*parse_status.borrow()) as i32) != (brunsli_BrunsliStatus_BRUNSLI_OK as i32))
            && (((*parse_status.borrow()) as i32)
                != (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            return brunsli_BrunsliDecoder_Status_ERROR;
        }
        if !(((*available_in.borrow()).read()) == 0_usize) {
            ({
                BrunsliDumpAndAbort_79(
                    Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                    2529,
                    Ptr::<u8>::from_string_literal(b"Decode"),
                )
            });
            'loop_: while true {}
        };
        let serialization_status: Value<brunsli_internal_dec_SerializationStatus> =
            Rc::new(RefCell::new(
                ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    let _available_out: Ptr<usize> = (*available_out.borrow()).clone();
                    let _next_out: Ptr<Ptr<u8>> = (*next_out.borrow()).clone();
                    SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
                }),
            ));
        if ((*serialization_status.borrow()) == brunsli_internal_dec_SerializationStatus_ERROR) {
            return brunsli_BrunsliDecoder_Status_ERROR;
        }
        'switch: {
            match { (*serialization_status.borrow()) } {
                __v if __v == brunsli_internal_dec_SerializationStatus_DONE => {
                    if !(((*parse_status.borrow()) as i32)
                        == (brunsli_BrunsliStatus_BRUNSLI_OK as i32))
                    {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                2540,
                                Ptr::<u8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_DONE;
                }
                __v if __v == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_INPUT => {
                    if !(((*parse_status.borrow()) as i32)
                        == (brunsli_BrunsliStatus_BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                2545,
                                Ptr::<u8>::from_string_literal(b"Decode"),
                            )
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status_NEEDS_MORE_INPUT;
                }
                __v if __v == brunsli_internal_dec_SerializationStatus_NEEDS_MORE_OUTPUT => {
                    if !(((*available_out.borrow()).read()) == 0_usize) {
                        ({
                            BrunsliDumpAndAbort_79(
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                2551,
                                Ptr::<u8>::from_string_literal(b"Decode"),
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
                                Ptr::<u8>::from_string_literal(b"brunsli_decode.cc"),
                                2559,
                                Ptr::<u8>::from_string_literal(b"Decode"),
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
                            Ptr::<u8>::from_string_literal(b"context.cc"),
                            227,
                            Ptr::<u8>::from_string_literal(b"InitAll"),
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
pub trait brunsli_HuffmanDecodingDataImpl {
    fn ReadFromBitStream(
        &self,
        alphabet_size: usize,
        br: Ptr<brunsli_BrunsliBitReader>,
        arena: Option<Ptr<brunsli_Arena_brunsli_HuffmanCode_>>,
    ) -> bool {
        unimplemented!()
    }
    fn ReadSymbol(&self, br: Ptr<brunsli_BrunsliBitReader>) -> u16 {
        unimplemented!()
    }
}
impl brunsli_HuffmanDecodingDataImpl for Ptr<brunsli_HuffmanDecodingData> {
    fn ReadFromBitStream(
        &self,
        alphabet_size: usize,
        br: Ptr<brunsli_BrunsliBitReader>,
        arena: Option<Ptr<brunsli_Arena_brunsli_HuffmanCode_>>,
    ) -> bool {
        let alphabet_size: Value<usize> = Rc::new(RefCell::new(alphabet_size));
        let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
        let arena: Value<Ptr<brunsli_Arena_brunsli_HuffmanCode_>> = Rc::new(RefCell::new(
            arena.unwrap_or(Ptr::<brunsli_Arena_brunsli_HuffmanCode_>::null()),
        ));
        let local_arena: Value<brunsli_Arena_brunsli_HuffmanCode_> =
            Rc::new(RefCell::new(<brunsli_Arena_brunsli_HuffmanCode_>::default()));
        if (*arena.borrow()).is_null() {
            (*arena.borrow_mut()) = (local_arena.as_pointer());
        }
        if ((*alphabet_size.borrow())
            > ((1 << kMaxHuffmanBits_22.with(|rc| *rc.borrow())) as usize))
        {
            return false;
        }
        let code_lengths: Value<Vec<u8>> =
            Rc::new(RefCell::new(vec![0_u8; (*alphabet_size.borrow()) as usize]));
        let simple_code_or_skip: Value<u32> = Rc::new(RefCell::new(
            ({ BrunsliBitReaderRead_126((*br.borrow()).clone(), 2_u32) }),
        ));
        if ((*simple_code_or_skip.borrow()) == 1_u32) {
            {
                let __a0 =
                    ((1_u32 << kHuffmanTableBits_21.with(|rc| *rc.borrow())) as usize) as usize;
                (*(*self).with(|__s| __s.table_.clone()).borrow_mut())
                    .resize_with(__a0, || <brunsli_HuffmanCode>::default())
            };
            return ({
                ReadSimpleCode_219(
                    ((*alphabet_size.borrow()) as u16),
                    (*br.borrow()).clone(),
                    ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<brunsli_HuffmanCode>),
                )
            });
        }
        let code_length_code_lengths: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8,
        ])));
        let space: Value<i32> = Rc::new(RefCell::new(32));
        let num_codes: Value<i32> = Rc::new(RefCell::new(0));
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
        let i: Value<usize> = Rc::new(RefCell::new(((*simple_code_or_skip.borrow()) as usize)));
        'loop_: while ((*i.borrow()) < (kCodeLengthCodes_213.with(|rc| *rc.borrow()) as usize))
            && ((*space.borrow()) > 0)
        {
            let code_len_idx: Value<i32> = Rc::new(RefCell::new(
                (({
                    let __idx = (*i.borrow()) as usize;
                    kCodeLengthCodeOrder_214.with(|rc| rc.borrow()[__idx])
                }) as i32),
            ));
            let p: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
                (huff_220.with(|v| v.as_pointer()) as Ptr<brunsli_HuffmanCode>),
            ));
            let v: Value<u8> = Rc::new(RefCell::new(0_u8));
            (*p.borrow_mut()) += ({ BrunsliBitReaderGet_124((*br.borrow()).clone(), 4_u32) });
            ({
                BrunsliBitReaderDrop_125(
                    (*br.borrow()).clone(),
                    ((*p.borrow()).with(|__s| __s.bits) as u32),
                )
            });
            (*v.borrow_mut()) = ((*p.borrow()).with(|__s| __s.value) as u8);
            (*code_length_code_lengths.borrow_mut())[(*code_len_idx.borrow()) as usize] =
                (*v.borrow());
            if (((*v.borrow()) as i32) != 0) {
                (*space.borrow_mut()) = {
                    (((*space.borrow()) as u32).wrapping_sub((32_u32 >> ((*v.borrow()) as i32))))
                        as i32
                };
                (*num_codes.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let ok: Value<bool> = Rc::new(RefCell::new(
            (((*num_codes.borrow()) == 1) || ((*space.borrow()) == 0))
                && ({
                    ReadHuffmanCodeLengths_217(
                        (code_length_code_lengths.as_pointer() as Ptr<u8>),
                        (*alphabet_size.borrow()),
                        ((code_lengths.as_pointer() as Ptr<u8>).offset(0_usize)),
                        (*br.borrow()).clone(),
                    )
                }),
        ));
        if (!(*ok.borrow())) || (!({ BrunsliBitReaderIsHealthy_132((*br.borrow()).clone()) })) {
            return false;
        }
        let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16,
        ])));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*alphabet_size.borrow())) {
            (*counts.borrow_mut())
                [(elem!((code_lengths.as_pointer() as Ptr<u8>), (*i.borrow())).read()) as usize]
                .prefix_inc();
            (*i.borrow_mut()).prefix_inc();
        }
        ({
            brunsli_Arena_brunsli_HuffmanCode_Impl::reserve(
                &(*arena.borrow()),
                (*alphabet_size.borrow()).wrapping_add(376_usize),
            )
        });
        let table_size: Value<u32> = Rc::new(RefCell::new(
            ({
                BuildHuffmanTable_218(
                    ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&(*arena.borrow())) }),
                    (kHuffmanTableBits_21.with(|rc| *rc.borrow()) as usize),
                    ((code_lengths.as_pointer() as Ptr<u8>).offset(0_usize)),
                    (*alphabet_size.borrow()),
                    ((counts.as_pointer() as Ptr<u16>).offset(0)),
                )
            }),
        ));
        ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<Vec<brunsli_HuffmanCode>>).write({
            let __count = ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&(*arena.borrow())) })
                .offset((*table_size.borrow()) as isize)
                .get_offset()
                - ({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&(*arena.borrow())) })
                    .get_offset();
            PtrValueIter::new(
                &({ brunsli_Arena_brunsli_HuffmanCode_Impl::data(&(*arena.borrow())) }),
                __count,
            )
            .map(|item| brunsli_HuffmanCode::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
        });
        return ((*table_size.borrow()) > 0_u32);
    }
    fn ReadSymbol(&self, br: Ptr<brunsli_BrunsliBitReader>) -> u16 {
        let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
        let n_bits: Value<u32> = Rc::new(RefCell::new(0_u32));
        let table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.table_.as_pointer()) as Ptr<brunsli_HuffmanCode>),
        ));
        (*table.borrow_mut()) += ({
            BrunsliBitReaderGet_124(
                (*br.borrow()).clone(),
                kHuffmanTableBits_21.with(|rc| *rc.borrow()),
            )
        });
        (*n_bits.borrow_mut()) = ((*table.borrow()).with(|__s| __s.bits) as u32);
        if ((*n_bits.borrow()) > kHuffmanTableBits_21.with(|rc| *rc.borrow())) {
            ({
                BrunsliBitReaderDrop_125(
                    (*br.borrow()).clone(),
                    kHuffmanTableBits_21.with(|rc| *rc.borrow()),
                )
            });
            (*n_bits.borrow_mut()) =
                { (*n_bits.borrow()).wrapping_sub(kHuffmanTableBits_21.with(|rc| *rc.borrow())) };
            let __rhs = ((*table.borrow()).with(|__s| __s.value) as i32);
            (*table.borrow_mut()) += __rhs;
            (*table.borrow_mut()) +=
                ({ BrunsliBitReaderGet_124((*br.borrow()).clone(), (*n_bits.borrow())) });
        }
        ({
            BrunsliBitReaderDrop_125(
                (*br.borrow()).clone(),
                ((*table.borrow()).with(|__s| __s.bits) as u32),
            )
        });
        return (*table.borrow()).with(|__s| __s.value);
    }
}
pub trait brunsli_JPEGOutputImpl {
    fn Write(&self, buf: Ptr<u8>, len: usize) -> bool;
}
impl brunsli_JPEGOutputImpl for Ptr<brunsli_JPEGOutput> {
    fn Write(&self, buf: Ptr<u8>, len: usize) -> bool {
        let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        if ((*len.borrow()) == 0_usize) {
            return true;
        }
        let bytes_written: Value<usize> = Rc::new(RefCell::new(
            ({
                let _arg0: AnyPtr = (*self).with(|__s| __s.data.clone());
                (*self).with(|__s| __s.cb.clone()).call(
                    _arg0,
                    (*buf.borrow()).clone(),
                    (*len.borrow()),
                )
            }),
        ));
        return ((*bytes_written.borrow()) == (*len.borrow()));
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
                    Ptr::<u8>::from_string_literal(b"lehmer_code.cc"),
                    51,
                    Ptr::<u8>::from_string_literal(b"num_bits"),
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
pub trait brunsli_WordSourceImpl {
    fn GetNextWord(&self) -> u16;
    fn CanRead(&self, n: usize) -> bool;
}
impl brunsli_WordSourceImpl for Ptr<brunsli_WordSource> {
    fn GetNextWord(&self) -> u16 {
        let val: Value<u16> = Rc::new(RefCell::new(0_u16));
        if ((*self).with(|__s| __s.pos_) < (*self).with(|__s| __s.len_)) {
            (*val.borrow_mut()) = ({
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
        return (*val.borrow());
    }
    fn CanRead(&self, n: usize) -> bool {
        let n: Value<usize> = Rc::new(RefCell::new(n));
        if (*self).with(|__s| __s.optimistic_) {
            return true;
        }
        let delta: Value<usize> = Rc::new(RefCell::new((2_usize).wrapping_mul((*n.borrow()))));
        let projected_end: Value<usize> = Rc::new(RefCell::new(
            ((*self).with(|__s| __s.pos_)).wrapping_add((*delta.borrow())),
        ));
        if ((*projected_end.borrow()) < (*self).with(|__s| __s.pos_)) {
            return false;
        }
        return ((*projected_end.borrow()) <= (*self).with(|__s| __s.len_));
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
