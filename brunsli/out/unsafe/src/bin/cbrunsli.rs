extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut BRUNSLI_ANS_LOG_TAB_SIZE_0: i32 = unsafe { 10 };
pub static mut BRUNSLI_ANS_TAB_SIZE_1: i32 = unsafe { ((1) << (BRUNSLI_ANS_LOG_TAB_SIZE_0)) };
pub static mut kFallbackVersion_2: i32 = unsafe { 1 };
pub static mut kDCTBlockSize_3: i32 = unsafe { 64 };
pub static mut kMaxComponents_4: i32 = unsafe { 4 };
pub static mut kMaxQuantTables_5: i32 = unsafe { 4 };
pub static mut kMaxHuffmanTables_6: i32 = unsafe { 4 };
pub static mut kJpegHuffmanMaxBitLength_7: i32 = unsafe { 16 };
pub static mut kJpegHuffmanAlphabetSize_8: i32 = unsafe { 256 };
pub static mut kJpegDCAlphabetSize_9: i32 = unsafe { 12 };
pub static mut kMaxDHTMarkers_10: i32 = unsafe { 512 };
pub static mut kMaxDimPixels_11: i32 = unsafe { 65535 };
pub static mut kDefaultQuantMatrix_12: [[u8; 64]; 2] = unsafe {
    [
        [
            16_u8, 11_u8, 10_u8, 16_u8, 24_u8, 40_u8, 51_u8, 61_u8, 12_u8, 12_u8, 14_u8, 19_u8,
            26_u8, 58_u8, 60_u8, 55_u8, 14_u8, 13_u8, 16_u8, 24_u8, 40_u8, 57_u8, 69_u8, 56_u8,
            14_u8, 17_u8, 22_u8, 29_u8, 51_u8, 87_u8, 80_u8, 62_u8, 18_u8, 22_u8, 37_u8, 56_u8,
            68_u8, 109_u8, 103_u8, 77_u8, 24_u8, 35_u8, 55_u8, 64_u8, 81_u8, 104_u8, 113_u8, 92_u8,
            49_u8, 64_u8, 78_u8, 87_u8, 103_u8, 121_u8, 120_u8, 101_u8, 72_u8, 92_u8, 95_u8, 98_u8,
            112_u8, 100_u8, 103_u8, 99_u8,
        ],
        [
            17_u8, 18_u8, 24_u8, 47_u8, 99_u8, 99_u8, 99_u8, 99_u8, 18_u8, 21_u8, 26_u8, 66_u8,
            99_u8, 99_u8, 99_u8, 99_u8, 24_u8, 26_u8, 56_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
            47_u8, 66_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
            99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
            99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8, 99_u8,
            99_u8, 99_u8, 99_u8, 99_u8,
        ],
    ]
};
pub static mut kJPEGNaturalOrder_13: [u32; 80] = unsafe {
    [
        0_u32, 1_u32, 8_u32, 16_u32, 9_u32, 2_u32, 3_u32, 10_u32, 17_u32, 24_u32, 32_u32, 25_u32,
        18_u32, 11_u32, 4_u32, 5_u32, 12_u32, 19_u32, 26_u32, 33_u32, 40_u32, 48_u32, 41_u32,
        34_u32, 27_u32, 20_u32, 13_u32, 6_u32, 7_u32, 14_u32, 21_u32, 28_u32, 35_u32, 42_u32,
        49_u32, 56_u32, 57_u32, 50_u32, 43_u32, 36_u32, 29_u32, 22_u32, 15_u32, 23_u32, 30_u32,
        37_u32, 44_u32, 51_u32, 58_u32, 59_u32, 52_u32, 45_u32, 38_u32, 31_u32, 39_u32, 46_u32,
        53_u32, 60_u32, 61_u32, 54_u32, 47_u32, 55_u32, 62_u32, 63_u32, 63_u32, 63_u32, 63_u32,
        63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32, 63_u32,
        63_u32, 63_u32,
    ]
};
pub static mut kJPEGZigZagOrder_14: [u32; 64] = unsafe {
    [
        0_u32, 1_u32, 5_u32, 6_u32, 14_u32, 15_u32, 27_u32, 28_u32, 2_u32, 4_u32, 7_u32, 13_u32,
        16_u32, 26_u32, 29_u32, 42_u32, 3_u32, 8_u32, 12_u32, 17_u32, 25_u32, 30_u32, 41_u32,
        43_u32, 9_u32, 11_u32, 18_u32, 24_u32, 31_u32, 40_u32, 44_u32, 53_u32, 10_u32, 19_u32,
        23_u32, 32_u32, 39_u32, 45_u32, 52_u32, 54_u32, 20_u32, 22_u32, 33_u32, 38_u32, 46_u32,
        51_u32, 55_u32, 60_u32, 21_u32, 34_u32, 37_u32, 47_u32, 50_u32, 56_u32, 59_u32, 61_u32,
        35_u32, 36_u32, 48_u32, 49_u32, 57_u32, 58_u32, 62_u32, 63_u32,
    ]
};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_JPEGReadError {
    #[default]
    OK = 0,
    SOI_NOT_FOUND = 1,
    SOF_NOT_FOUND = 2,
    UNEXPECTED_EOF = 3,
    MARKER_BYTE_NOT_FOUND = 4,
    UNSUPPORTED_MARKER = 5,
    WRONG_MARKER_SIZE = 6,
    INVALID_PRECISION = 7,
    INVALID_WIDTH = 8,
    INVALID_HEIGHT = 9,
    INVALID_NUMCOMP = 10,
    INVALID_SAMP_FACTOR = 11,
    INVALID_START_OF_SCAN = 12,
    INVALID_END_OF_SCAN = 13,
    INVALID_SCAN_BIT_POSITION = 14,
    INVALID_COMPS_IN_SCAN = 15,
    INVALID_HUFFMAN_INDEX = 16,
    INVALID_QUANT_TBL_INDEX = 17,
    INVALID_QUANT_VAL = 18,
    INVALID_MARKER_LEN = 19,
    INVALID_SAMPLING_FACTORS = 20,
    INVALID_HUFFMAN_CODE = 21,
    INVALID_SYMBOL = 22,
    NON_REPRESENTABLE_DC_COEFF = 23,
    NON_REPRESENTABLE_AC_COEFF = 24,
    INVALID_SCAN = 25,
    OVERLAPPING_SCANS = 26,
    INVALID_SCAN_ORDER = 27,
    EXTRA_ZERO_RUN = 28,
    DUPLICATE_DRI = 29,
    DUPLICATE_SOF = 30,
    WRONG_RESTART_MARKER = 31,
    DUPLICATE_COMPONENT_ID = 32,
    COMPONENT_NOT_FOUND = 33,
    HUFFMAN_TABLE_NOT_FOUND = 34,
    HUFFMAN_TABLE_ERROR = 35,
    QUANT_TABLE_NOT_FOUND = 36,
    EMPTY_DHT = 37,
    EMPTY_DQT = 38,
    OUT_OF_BAND_COEFF = 39,
    EOB_RUN_TOO_LONG = 40,
    IMAGE_TOO_LARGE = 41,
    INVALID_QUANT_TBL_PRECISION = 42,
}
impl From<i32> for brunsli_JPEGReadError {
    fn from(n: i32) -> brunsli_JPEGReadError {
        match n {
            0 => brunsli_JPEGReadError::OK,
            1 => brunsli_JPEGReadError::SOI_NOT_FOUND,
            2 => brunsli_JPEGReadError::SOF_NOT_FOUND,
            3 => brunsli_JPEGReadError::UNEXPECTED_EOF,
            4 => brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND,
            5 => brunsli_JPEGReadError::UNSUPPORTED_MARKER,
            6 => brunsli_JPEGReadError::WRONG_MARKER_SIZE,
            7 => brunsli_JPEGReadError::INVALID_PRECISION,
            8 => brunsli_JPEGReadError::INVALID_WIDTH,
            9 => brunsli_JPEGReadError::INVALID_HEIGHT,
            10 => brunsli_JPEGReadError::INVALID_NUMCOMP,
            11 => brunsli_JPEGReadError::INVALID_SAMP_FACTOR,
            12 => brunsli_JPEGReadError::INVALID_START_OF_SCAN,
            13 => brunsli_JPEGReadError::INVALID_END_OF_SCAN,
            14 => brunsli_JPEGReadError::INVALID_SCAN_BIT_POSITION,
            15 => brunsli_JPEGReadError::INVALID_COMPS_IN_SCAN,
            16 => brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX,
            17 => brunsli_JPEGReadError::INVALID_QUANT_TBL_INDEX,
            18 => brunsli_JPEGReadError::INVALID_QUANT_VAL,
            19 => brunsli_JPEGReadError::INVALID_MARKER_LEN,
            20 => brunsli_JPEGReadError::INVALID_SAMPLING_FACTORS,
            21 => brunsli_JPEGReadError::INVALID_HUFFMAN_CODE,
            22 => brunsli_JPEGReadError::INVALID_SYMBOL,
            23 => brunsli_JPEGReadError::NON_REPRESENTABLE_DC_COEFF,
            24 => brunsli_JPEGReadError::NON_REPRESENTABLE_AC_COEFF,
            25 => brunsli_JPEGReadError::INVALID_SCAN,
            26 => brunsli_JPEGReadError::OVERLAPPING_SCANS,
            27 => brunsli_JPEGReadError::INVALID_SCAN_ORDER,
            28 => brunsli_JPEGReadError::EXTRA_ZERO_RUN,
            29 => brunsli_JPEGReadError::DUPLICATE_DRI,
            30 => brunsli_JPEGReadError::DUPLICATE_SOF,
            31 => brunsli_JPEGReadError::WRONG_RESTART_MARKER,
            32 => brunsli_JPEGReadError::DUPLICATE_COMPONENT_ID,
            33 => brunsli_JPEGReadError::COMPONENT_NOT_FOUND,
            34 => brunsli_JPEGReadError::HUFFMAN_TABLE_NOT_FOUND,
            35 => brunsli_JPEGReadError::HUFFMAN_TABLE_ERROR,
            36 => brunsli_JPEGReadError::QUANT_TABLE_NOT_FOUND,
            37 => brunsli_JPEGReadError::EMPTY_DHT,
            38 => brunsli_JPEGReadError::EMPTY_DQT,
            39 => brunsli_JPEGReadError::OUT_OF_BAND_COEFF,
            40 => brunsli_JPEGReadError::EOB_RUN_TOO_LONG,
            41 => brunsli_JPEGReadError::IMAGE_TOO_LARGE,
            42 => brunsli_JPEGReadError::INVALID_QUANT_TBL_PRECISION,
            _ => panic!("invalid brunsli_JPEGReadError value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_JPEGReadError);
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_JPEGQuantTable {
    pub values: Vec<i32>,
    pub precision: i32,
    pub index: i32,
    pub is_last: bool,
}
impl Default for brunsli_JPEGQuantTable {
    fn default() -> Self {
        brunsli_JPEGQuantTable {
            values: std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            precision: 0_i32,
            index: 0_i32,
            is_last: false,
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_JPEGHuffmanCode {
    pub counts: Vec<i32>,
    pub values: Vec<i32>,
    pub slot_id: i32,
    pub is_last: bool,
}
impl Default for brunsli_JPEGHuffmanCode {
    fn default() -> Self {
        brunsli_JPEGHuffmanCode {
            counts: std::array::from_fn::<_, 17, _>(|_| Default::default()).to_vec(),
            values: std::array::from_fn::<_, 257, _>(|_| Default::default()).to_vec(),
            slot_id: 0_i32,
            is_last: false,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_JPEGComponentScanInfo {
    pub comp_idx: u8,
    pub dc_tbl_idx: i32,
    pub ac_tbl_idx: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_JPEGScanInfo_ExtraZeroRunInfo {
    pub block_idx: i32,
    pub num_extra_zero_runs: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_JPEGScanInfo {
    pub Ss: i32,
    pub Se: i32,
    pub Ah: i32,
    pub Al: i32,
    pub num_components: usize,
    pub components: Vec<brunsli_JPEGComponentScanInfo>,
    pub reset_points: Vec<i32>,
    pub extra_zero_runs: Vec<brunsli_JPEGScanInfo_ExtraZeroRunInfo>,
}
impl Default for brunsli_JPEGScanInfo {
    fn default() -> Self {
        brunsli_JPEGScanInfo {
            Ss: 0_i32,
            Se: 0_i32,
            Ah: 0_i32,
            Al: 0_i32,
            num_components: 0_usize,
            components: std::array::from_fn::<_, 4, _>(|_| Default::default()).to_vec(),
            reset_points: Default::default(),
            extra_zero_runs: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_JPEGComponent {
    pub id: i32,
    pub h_samp_factor: i32,
    pub v_samp_factor: i32,
    pub quant_idx: u8,
    pub width_in_blocks: u32,
    pub height_in_blocks: u32,
    pub num_blocks: u32,
    pub coeffs: Vec<i16>,
}
impl brunsli_JPEGComponent {
    pub unsafe fn brunsli_JPEGComponent() -> Self {
        let mut this = Self {
            id: 0,
            h_samp_factor: 1,
            v_samp_factor: 1,
            quant_idx: 0_u8,
            width_in_blocks: 0_u32,
            height_in_blocks: 0_u32,
            num_blocks: 0_u32,
            coeffs: Vec::new(),
        };
        this
    }
}
impl Default for brunsli_JPEGComponent {
    fn default() -> Self {
        unsafe { brunsli_JPEGComponent::brunsli_JPEGComponent() }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_JPEGData {
    pub width: i32,
    pub height: i32,
    pub version: i32,
    pub max_h_samp_factor: i32,
    pub max_v_samp_factor: i32,
    pub MCU_rows: i32,
    pub MCU_cols: i32,
    pub restart_interval: i32,
    pub app_data: Vec<Vec<u8>>,
    pub com_data: Vec<Vec<u8>>,
    pub quant: Vec<brunsli_JPEGQuantTable>,
    pub huffman_code: Vec<brunsli_JPEGHuffmanCode>,
    pub components: Vec<brunsli_JPEGComponent>,
    pub scan_info: Vec<brunsli_JPEGScanInfo>,
    pub marker_order: Vec<u8>,
    pub inter_marker_data: Vec<Vec<u8>>,
    pub tail_data: Vec<u8>,
    pub original_jpg: *const u8,
    pub original_jpg_size: usize,
    pub error: brunsli_JPEGReadError,
    pub has_zero_padding_bit: bool,
    pub padding_bits: Vec<i32>,
}
impl brunsli_JPEGData {
    pub unsafe fn brunsli_JPEGData() -> Self {
        let mut this = Self {
            width: 0,
            height: 0,
            version: 2,
            max_h_samp_factor: 1,
            max_v_samp_factor: 1,
            MCU_rows: 0,
            MCU_cols: 0,
            restart_interval: 0,
            app_data: Vec::new(),
            com_data: Vec::new(),
            quant: Vec::new(),
            huffman_code: Vec::new(),
            components: Vec::new(),
            scan_info: Vec::new(),
            marker_order: Vec::new(),
            inter_marker_data: Vec::new(),
            tail_data: Vec::new(),
            original_jpg: std::ptr::null(),
            original_jpg_size: 0_usize,
            error: brunsli_JPEGReadError::OK,
            has_zero_padding_bit: false,
            padding_bits: Vec::new(),
        };
        this
    }
}
impl Default for brunsli_JPEGData {
    fn default() -> Self {
        unsafe { brunsli_JPEGData::brunsli_JPEGData() }
    }
}
pub unsafe fn JPEGDataIs420_15(jpg: *const brunsli_JPEGData) -> bool {
    return (((((((((((*jpg).components.len()) == (3_usize))
        && (((*jpg).max_h_samp_factor) == (2)))
        && (((*jpg).max_v_samp_factor) == (2)))
        && (((&(*jpg)).components[(0_usize)].h_samp_factor) == (2)))
        && (((&(*jpg)).components[(0_usize)].v_samp_factor) == (2)))
        && (((&(*jpg)).components[(1_usize)].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_usize)].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_usize)].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_usize)].v_samp_factor) == (1)));
}
pub unsafe fn JPEGDataIs444_16(jpg: *const brunsli_JPEGData) -> bool {
    return (((((((((((*jpg).components.len()) == (3_usize))
        && (((*jpg).max_h_samp_factor) == (1)))
        && (((*jpg).max_v_samp_factor) == (1)))
        && (((&(*jpg)).components[(0_usize)].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(0_usize)].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_usize)].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_usize)].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_usize)].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_usize)].v_samp_factor) == (1)));
}
pub unsafe fn PaddingBitsLimit_17(jpg: *const brunsli_JPEGData) -> u64 {
    let num_blocks: u64 = ((((*jpg).width as u64).wrapping_add(15_u64)) >> (3_u32))
        .wrapping_mul(((((*jpg).height as u64).wrapping_add(15_u64)) >> (3_u32)));
    return (((7_u64).wrapping_mul(num_blocks)).wrapping_mul(((*jpg).components.len() as u64)))
        .wrapping_add(256_u64);
}
pub static mut kBrunsliMaxNumBlocks_18: usize = unsafe { (((1_u64) << (21)) as usize) };
pub static mut kBrunsliMaxDCAbsVal_19: i32 = unsafe { 2054 };
pub static mut kMaxContextMapAlphabetSize_20: usize = unsafe { 272_usize };
pub static mut kHuffmanTableBits_21: u32 = unsafe { 8_u32 };
pub static mut kMaxHuffmanBits_22: usize = unsafe { 15_usize };
pub static mut kBrunsliShortMarkerLimit_23: i32 = unsafe { ((64) + ((3) * (256))) };
pub static mut kBrunsliMultibyteMarkerLimit_24: i32 = unsafe { 1024 };
pub static mut kBrunsliWiringTypeVarint_25: u8 = unsafe { 0_u8 };
pub static mut kBrunsliWiringTypeLengthDelimited_26: u8 = unsafe { 2_u8 };
pub static mut kBrunsliMaxSampling_27: i32 = unsafe { 15 };
pub const unsafe fn ValueMarker_28(mut tag: u8) -> u8 {
    return ((((tag as i32) << (3)) | (kBrunsliWiringTypeVarint_25 as i32)) as u8);
}
pub const unsafe fn SectionMarker_29(mut tag: u8) -> u8 {
    return ((((tag as i32) << (3)) | (kBrunsliWiringTypeLengthDelimited_26 as i32)) as u8);
}
pub static mut kBrunsliSignatureTag_30: u8 = unsafe { 1_u8 };
pub static mut kBrunsliHeaderTag_31: u8 = unsafe { 2_u8 };
pub static mut kBrunsliMetaDataTag_32: u8 = unsafe { 3_u8 };
pub static mut kBrunsliJPEGInternalsTag_33: u8 = unsafe { 4_u8 };
pub static mut kBrunsliQuantDataTag_34: u8 = unsafe { 5_u8 };
pub static mut kBrunsliHistogramDataTag_35: u8 = unsafe { 6_u8 };
pub static mut kBrunsliDCDataTag_36: u8 = unsafe { 7_u8 };
pub static mut kBrunsliACDataTag_37: u8 = unsafe { 8_u8 };
pub static mut kBrunsliOriginalJpgTag_38: u8 = unsafe { 9_u8 };
pub static mut kBrunsliHeaderWidthTag_39: u8 = unsafe { 1_u8 };
pub static mut kBrunsliHeaderHeightTag_40: u8 = unsafe { 2_u8 };
pub static mut kBrunsliHeaderVersionCompTag_41: u8 = unsafe { 3_u8 };
pub static mut kBrunsliHeaderSubsamplingTag_42: u8 = unsafe { 4_u8 };
pub static mut kBrunsliSignatureSize_43: usize = unsafe { 6_usize };
pub static mut kMaxApp0Densities_45: usize = unsafe { 8_usize };
pub static mut kApp0Densities_46: [u16; 8] = unsafe {
    [
        1_u16, 72_u16, 96_u16, 100_u16, 150_u16, 180_u16, 240_u16, 300_u16,
    ]
};
pub static mut kNumStockQuantTables_47: i32 = unsafe { 8 };
pub static mut kStockQuantizationTables_48: [[[u8; 64]; 8]; 2] = unsafe {
    [
        [
            [
                3_u8, 2_u8, 2_u8, 3_u8, 5_u8, 8_u8, 10_u8, 12_u8, 2_u8, 2_u8, 3_u8, 4_u8, 5_u8,
                12_u8, 12_u8, 11_u8, 3_u8, 3_u8, 3_u8, 5_u8, 8_u8, 11_u8, 14_u8, 11_u8, 3_u8, 3_u8,
                4_u8, 6_u8, 10_u8, 17_u8, 16_u8, 12_u8, 4_u8, 4_u8, 7_u8, 11_u8, 14_u8, 22_u8,
                21_u8, 15_u8, 5_u8, 7_u8, 11_u8, 13_u8, 16_u8, 21_u8, 23_u8, 18_u8, 10_u8, 13_u8,
                16_u8, 17_u8, 21_u8, 24_u8, 24_u8, 20_u8, 14_u8, 18_u8, 19_u8, 20_u8, 22_u8, 20_u8,
                21_u8, 20_u8,
            ],
            [
                8_u8, 6_u8, 5_u8, 8_u8, 12_u8, 20_u8, 26_u8, 31_u8, 6_u8, 6_u8, 7_u8, 10_u8, 13_u8,
                29_u8, 30_u8, 28_u8, 7_u8, 7_u8, 8_u8, 12_u8, 20_u8, 29_u8, 35_u8, 28_u8, 7_u8,
                9_u8, 11_u8, 15_u8, 26_u8, 44_u8, 40_u8, 31_u8, 9_u8, 11_u8, 19_u8, 28_u8, 34_u8,
                55_u8, 52_u8, 39_u8, 12_u8, 18_u8, 28_u8, 32_u8, 41_u8, 52_u8, 57_u8, 46_u8, 25_u8,
                32_u8, 39_u8, 44_u8, 52_u8, 61_u8, 60_u8, 51_u8, 36_u8, 46_u8, 48_u8, 49_u8, 56_u8,
                50_u8, 52_u8, 50_u8,
            ],
            [
                6_u8, 4_u8, 4_u8, 6_u8, 10_u8, 16_u8, 20_u8, 24_u8, 5_u8, 5_u8, 6_u8, 8_u8, 10_u8,
                23_u8, 24_u8, 22_u8, 6_u8, 5_u8, 6_u8, 10_u8, 16_u8, 23_u8, 28_u8, 22_u8, 6_u8,
                7_u8, 9_u8, 12_u8, 20_u8, 35_u8, 32_u8, 25_u8, 7_u8, 9_u8, 15_u8, 22_u8, 27_u8,
                44_u8, 41_u8, 31_u8, 10_u8, 14_u8, 22_u8, 26_u8, 32_u8, 42_u8, 45_u8, 37_u8, 20_u8,
                26_u8, 31_u8, 35_u8, 41_u8, 48_u8, 48_u8, 40_u8, 29_u8, 37_u8, 38_u8, 39_u8, 45_u8,
                40_u8, 41_u8, 40_u8,
            ],
            [
                5_u8, 3_u8, 3_u8, 5_u8, 7_u8, 12_u8, 15_u8, 18_u8, 4_u8, 4_u8, 4_u8, 6_u8, 8_u8,
                17_u8, 18_u8, 17_u8, 4_u8, 4_u8, 5_u8, 7_u8, 12_u8, 17_u8, 21_u8, 17_u8, 4_u8,
                5_u8, 7_u8, 9_u8, 15_u8, 26_u8, 24_u8, 19_u8, 5_u8, 7_u8, 11_u8, 17_u8, 20_u8,
                33_u8, 31_u8, 23_u8, 7_u8, 11_u8, 17_u8, 19_u8, 24_u8, 31_u8, 34_u8, 28_u8, 15_u8,
                19_u8, 23_u8, 26_u8, 31_u8, 36_u8, 36_u8, 30_u8, 22_u8, 28_u8, 29_u8, 29_u8, 34_u8,
                30_u8, 31_u8, 30_u8,
            ],
            [
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            ],
            [
                2_u8, 1_u8, 1_u8, 2_u8, 2_u8, 4_u8, 5_u8, 6_u8, 1_u8, 1_u8, 1_u8, 2_u8, 3_u8, 6_u8,
                6_u8, 6_u8, 1_u8, 1_u8, 2_u8, 2_u8, 4_u8, 6_u8, 7_u8, 6_u8, 1_u8, 2_u8, 2_u8, 3_u8,
                5_u8, 9_u8, 8_u8, 6_u8, 2_u8, 2_u8, 4_u8, 6_u8, 7_u8, 11_u8, 10_u8, 8_u8, 2_u8,
                4_u8, 6_u8, 6_u8, 8_u8, 10_u8, 11_u8, 9_u8, 5_u8, 6_u8, 8_u8, 9_u8, 10_u8, 12_u8,
                12_u8, 10_u8, 7_u8, 9_u8, 10_u8, 10_u8, 11_u8, 10_u8, 10_u8, 10_u8,
            ],
            [
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 2_u8, 2_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 3_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 3_u8, 3_u8,
                1_u8, 1_u8, 2_u8, 2_u8, 3_u8, 3_u8, 3_u8, 3_u8,
            ],
            [
                10_u8, 7_u8, 6_u8, 10_u8, 14_u8, 24_u8, 31_u8, 37_u8, 7_u8, 7_u8, 8_u8, 11_u8,
                16_u8, 35_u8, 36_u8, 33_u8, 8_u8, 8_u8, 10_u8, 14_u8, 24_u8, 34_u8, 41_u8, 34_u8,
                8_u8, 10_u8, 13_u8, 17_u8, 31_u8, 52_u8, 48_u8, 37_u8, 11_u8, 13_u8, 22_u8, 34_u8,
                41_u8, 65_u8, 62_u8, 46_u8, 14_u8, 21_u8, 33_u8, 38_u8, 49_u8, 62_u8, 68_u8, 55_u8,
                29_u8, 38_u8, 47_u8, 52_u8, 62_u8, 73_u8, 72_u8, 61_u8, 43_u8, 55_u8, 57_u8, 59_u8,
                67_u8, 60_u8, 62_u8, 59_u8,
            ],
        ],
        [
            [
                9_u8, 9_u8, 9_u8, 12_u8, 11_u8, 12_u8, 24_u8, 13_u8, 13_u8, 24_u8, 50_u8, 33_u8,
                28_u8, 33_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8,
            ],
            [
                3_u8, 4_u8, 5_u8, 9_u8, 20_u8, 20_u8, 20_u8, 20_u8, 4_u8, 4_u8, 5_u8, 13_u8, 20_u8,
                20_u8, 20_u8, 20_u8, 5_u8, 5_u8, 11_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 9_u8,
                13_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8, 20_u8,
                20_u8, 20_u8, 20_u8,
            ],
            [
                9_u8, 9_u8, 12_u8, 24_u8, 50_u8, 50_u8, 50_u8, 50_u8, 9_u8, 11_u8, 13_u8, 33_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 12_u8, 13_u8, 28_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                24_u8, 33_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8, 50_u8,
                50_u8, 50_u8, 50_u8, 50_u8,
            ],
            [
                5_u8, 5_u8, 7_u8, 14_u8, 30_u8, 30_u8, 30_u8, 30_u8, 5_u8, 6_u8, 8_u8, 20_u8,
                30_u8, 30_u8, 30_u8, 30_u8, 7_u8, 8_u8, 17_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                14_u8, 20_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8, 30_u8,
                30_u8, 30_u8, 30_u8, 30_u8,
            ],
            [
                7_u8, 7_u8, 10_u8, 19_u8, 40_u8, 40_u8, 40_u8, 40_u8, 7_u8, 8_u8, 10_u8, 26_u8,
                40_u8, 40_u8, 40_u8, 40_u8, 10_u8, 10_u8, 22_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                19_u8, 26_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8, 40_u8,
                40_u8, 40_u8, 40_u8, 40_u8,
            ],
            [
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
                1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            ],
            [
                2_u8, 2_u8, 2_u8, 5_u8, 10_u8, 10_u8, 10_u8, 10_u8, 2_u8, 2_u8, 3_u8, 7_u8, 10_u8,
                10_u8, 10_u8, 10_u8, 2_u8, 3_u8, 6_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 5_u8,
                7_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
                10_u8, 10_u8, 10_u8,
            ],
            [
                10_u8, 11_u8, 14_u8, 28_u8, 59_u8, 59_u8, 59_u8, 59_u8, 11_u8, 13_u8, 16_u8, 40_u8,
                59_u8, 59_u8, 59_u8, 59_u8, 14_u8, 16_u8, 34_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                28_u8, 40_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8, 59_u8,
                59_u8, 59_u8, 59_u8, 59_u8,
            ],
        ],
    ]
};
pub static mut kComponentIds123_49: i32 = unsafe { 0 };
pub static mut kComponentIdsGray_50: i32 = unsafe { 1 };
pub static mut kComponentIdsRGB_51: i32 = unsafe { 2 };
pub static mut kComponentIdsCustom_52: i32 = unsafe { 3 };
pub static mut kNumStockDCHuffmanCodes_53: i32 = unsafe { 2 };
pub static mut kStockDCHuffmanCodeCounts_54: [[i32; 16]; 2] = unsafe {
    [
        [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0],
        [0, 1, 5, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0],
    ]
};
pub static mut kStockDCHuffmanCodeValues_55: [[i32; 13]; 2] = unsafe {
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 256],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 256],
    ]
};
pub static mut kNumStockACHuffmanCodes_56: i32 = unsafe { 2 };
pub static mut kStockACHuffmanCodeCounts_57: [[i32; 16]; 2] = unsafe {
    [
        [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 126],
        [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 120],
    ]
};
pub static mut kStockACHuffmanCodeTotalCount_58: i32 = unsafe { 163 };
pub static mut kStockACHuffmanCodeValues_59: [[i32; 163]; 2] = unsafe {
    [
        [
            1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50, 129, 145, 161,
            8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130, 9, 10, 22, 23, 24, 25, 26,
            37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83,
            84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118,
            119, 120, 121, 122, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150,
            151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180, 181,
            182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210, 211, 212,
            213, 214, 215, 216, 217, 218, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 241,
            242, 243, 244, 245, 246, 247, 248, 249, 250, 256,
        ],
        [
            0, 1, 2, 3, 17, 4, 5, 33, 49, 6, 18, 65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20, 66,
            145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114, 209, 10, 22, 36, 52, 225, 37, 241,
            23, 24, 25, 26, 38, 39, 40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73,
            74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116,
            117, 118, 119, 120, 121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147,
            148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178,
            179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202,
            210, 211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233,
            234, 242, 243, 244, 245, 246, 247, 248, 249, 250, 256,
        ],
    ]
};
pub static mut kDefaultDCValues_60: [u8; 16] = unsafe {
    [
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
        14_u8, 15_u8,
    ]
};
pub static mut kDefaultACValues_61: [u8; 256] = unsafe {
    [
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
    ]
};
pub static mut kBrunsliSignature_44: [u8; 6] = unsafe {
    [
        (unsafe { SectionMarker_29(kBrunsliSignatureTag_30) }),
        4_u8,
        (('B' as libc::c_char) as u8),
        210_u8,
        213_u8,
        (('N' as libc::c_char) as u8),
    ]
};
pub static mut AppData_0xe0_62: [u8; 17] = unsafe {
    [
        224_u8,
        0_u8,
        16_u8,
        (('J' as libc::c_char) as u8),
        (('F' as libc::c_char) as u8),
        (('I' as libc::c_char) as u8),
        (('F' as libc::c_char) as u8),
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
    ]
};
pub static mut AppData_0xec_64: [u8; 18] = unsafe {
    [
        236_u8,
        0_u8,
        17_u8,
        (('D' as libc::c_char) as u8),
        (('u' as libc::c_char) as u8),
        (('c' as libc::c_char) as u8),
        (('k' as libc::c_char) as u8),
        (('y' as libc::c_char) as u8),
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
    ]
};
pub static mut AppData_0xee_65: [u8; 15] = unsafe {
    [
        238_u8,
        0_u8,
        14_u8,
        (('A' as libc::c_char) as u8),
        (('d' as libc::c_char) as u8),
        (('o' as libc::c_char) as u8),
        (('b' as libc::c_char) as u8),
        (('e' as libc::c_char) as u8),
        0_u8,
        100_u8,
        0_u8,
        0_u8,
        0_u8,
        0_u8,
        1_u8,
    ]
};
pub static mut AppData_0xe2_63: [u8; 3161] = unsafe {
    [
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
    ]
};
pub unsafe fn BrunsliUnalignedRead16_66(mut p: *const ::libc::c_void) -> u16 {
    let mut t: u16 = 0_u16;
    {
        if ::std::mem::size_of::<u16>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u16) as *mut u16 as *mut ::libc::c_void),
                ::std::mem::size_of::<u16>() as usize,
            )
        }
        ((&mut t as *mut u16) as *mut u16 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedWrite16_67(mut p: *mut ::libc::c_void, mut v: u16) {
    {
        if ::std::mem::size_of::<u16>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut v as *mut u16) as *const u16 as *const ::libc::c_void),
                p,
                ::std::mem::size_of::<u16>() as usize,
            )
        }
        p
    };
}
pub unsafe fn BrunsliUnalignedRead32_68(mut p: *const ::libc::c_void) -> u32 {
    let mut t: u32 = 0_u32;
    {
        if ::std::mem::size_of::<u32>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u32) as *mut u32 as *mut ::libc::c_void),
                ::std::mem::size_of::<u32>() as usize,
            )
        }
        ((&mut t as *mut u32) as *mut u32 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedRead64_69(mut p: *const ::libc::c_void) -> u64 {
    let mut t: u64 = 0_u64;
    {
        if ::std::mem::size_of::<u64>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u64) as *mut u64 as *mut ::libc::c_void),
                ::std::mem::size_of::<u64>() as usize,
            )
        }
        ((&mut t as *mut u64) as *mut u64 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedWrite64_70(mut p: *mut ::libc::c_void, mut v: u64) {
    {
        if ::std::mem::size_of::<u64>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut v as *mut u64) as *const u64 as *const ::libc::c_void),
                p,
                ::std::mem::size_of::<u64>() as usize,
            )
        }
        p
    };
}
pub unsafe fn Append_71(mut dst: *mut Vec<u8>, mut begin: *const u8, mut end: *const u8) {
    {
        let __off = (*dst)
            .as_mut_ptr()
            .add((*dst).len())
            .offset_from((*dst).as_ptr()) as usize;
        let count = end.offset_from(begin) as usize;
        (*dst).splice(
            __off..__off,
            std::slice::from_raw_parts(begin, count).iter().cloned(),
        );
        (*dst).as_mut_ptr().add(__off)
    };
}
pub unsafe fn Append_72(mut dst: *mut Vec<u8>, mut begin: *const u8, mut length: usize) {
    (unsafe {
        let _begin: *const u8 = begin;
        let _end: *const u8 = begin.offset((length) as isize);
        Append_71(dst, _begin, _end)
    });
}
pub unsafe fn Append_73(mut dst: *mut Vec<u8>, src: *const Vec<u8>) {
    (unsafe {
        let _begin: *const u8 = (*src).as_ptr();
        let _length: usize = (*src).len();
        Append_72(dst, _begin, _length)
    });
}
pub unsafe fn Log2FloorNonZero_74(mut n: u32) -> i32 {
    return ((31) ^ (n.leading_zeros() as i32));
}
pub unsafe fn BrunsliSuppressUnusedFunctions_75() {
    &(std::mem::transmute::<
        Option<unsafe fn(*mut Vec<u8>, *const Vec<u8>)>,
        Option<unsafe fn(*mut Vec<u8>, *const Vec<u8>)>,
    >((Some(Append_73))));
    &(Some(BrunsliSuppressUnusedFunctions_75));
    &(Some(BrunsliUnalignedRead16_66));
    &(Some(BrunsliUnalignedWrite16_67));
    &(Some(BrunsliUnalignedRead32_68));
    &(Some(BrunsliUnalignedRead64_69));
    &(Some(BrunsliUnalignedWrite64_70));
    &(Some(BrunsliUnalignedRead16_66));
    &(Some(BrunsliUnalignedWrite16_67));
    &(Some(BrunsliUnalignedRead32_68));
    &(Some(BrunsliUnalignedRead64_69));
    &(Some(BrunsliUnalignedWrite64_70));
}
pub static mut kNormalizeThreshold_76: u8 = unsafe { 254_u8 };
pub static mut kDivLut17_77: [u16; 255] = unsafe {
    [
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
    ]
};
pub unsafe fn FastDivide_78(mut numerator: u32, mut denominator: u8) -> u8 {
    let mut result: u32 =
        (((numerator).wrapping_mul((kDivLut17_77[(denominator) as usize] as u32))) >> (17));
    if !((result) < (256_u32)) {
        (unsafe { BrunsliDumpAndAbort_79(c"context.cc".as_ptr(), 55, c"FastDivide".as_ptr()) });
        'loop_: while true {}
    };
    return (result as u8);
}
pub static mut kInitProb_80: u8 = unsafe { 134_u8 };
pub static mut kInitProbCount_81: u8 = unsafe { 3_u8 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_Prob {
    prob8: u8,
    total: u8,
    count: u16,
}
impl brunsli_Prob {
    pub unsafe fn brunsli_Prob() -> Self {
        let mut this = Self {
            prob8: kInitProb_80,
            total: kInitProbCount_81,
            count: (((kInitProb_80 as i32) * (kInitProbCount_81 as i32)) as u16),
        };
        this
    }
    pub unsafe fn Init(&mut self, mut probability: u8) {
        self.prob8 = probability;
        self.total = kInitProbCount_81;
        self.count = (((kInitProbCount_81 as i32) * (probability as i32)) as u16);
    }
    pub unsafe fn Add(&mut self, mut val: i32) {
        self.total.prefix_inc();
        if ((val) == (0)) {
            self.count = ((self.count as i32) + 256) as u16;
        } else {
            self.count.prefix_inc();
        }
        self.prob8 = (unsafe {
            let _numerator: u32 = (self.count as u32);
            let _denominator: u8 = self.total;
            FastDivide_78(_numerator, _denominator)
        });
        if ((self.total as i32) == (kNormalizeThreshold_76 as i32)) {
            self.count = ((self.count as i32) >> 1) as u16;
            self.total = (((kNormalizeThreshold_76 as i32) >> (1)) as u8);
        }
    }
    pub unsafe fn get_proba(&self) -> u8 {
        return self.prob8;
    }
}
impl Default for brunsli_Prob {
    fn default() -> Self {
        unsafe { brunsli_Prob::brunsli_Prob() }
    }
}
pub static mut kMaxAverageContext_82: usize = unsafe { 8_usize };
pub static mut kNumAvrgContexts_83: usize =
    unsafe { (kMaxAverageContext_82).wrapping_add(1_usize) };
pub static mut kNumNonZeroBits_84: usize = unsafe { 6_usize };
pub static mut kNumNonZeroTreeSize_85: usize =
    unsafe { (((((1_u32) << (kNumNonZeroBits_84)) as u32).wrapping_sub((1_u32 as u32))) as usize) };
pub static mut kNumNonZeroQuant_86: usize = unsafe { 2_usize };
pub static mut kNumNonZeroContextMax_87: usize =
    unsafe { (kNumNonZeroTreeSize_85).wrapping_div(kNumNonZeroQuant_86) };
pub static mut kNumNonZeroContextCount_88: usize =
    unsafe { (kNumNonZeroContextMax_87).wrapping_add(1_usize) };
pub static mut kNonzeroBuckets_89: [u8; 64] = unsafe {
    [
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 4_u8, 5_u8, 5_u8, 5_u8, 6_u8, 6_u8, 6_u8, 6_u8, 7_u8, 7_u8,
        7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8, 8_u8,
        8_u8, 8_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8, 9_u8,
        10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
        10_u8, 10_u8, 10_u8, 10_u8, 10_u8, 10_u8,
    ]
};
pub static mut kNumNonzeroBuckets_90: u8 = unsafe { 11_u8 };
pub static mut kNumSchemes_91: i32 = unsafe { 7 };
pub static mut kFreqContext_92: [[u8; 64]; 7] = unsafe {
    [
        [
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        ],
        [
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8,
        ],
        [
            0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 2_u8, 2_u8, 2_u8,
            2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 2_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8,
            3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 1_u8, 1_u8, 1_u8,
        ],
        [
            0_u8, 1_u8, 1_u8, 2_u8, 2_u8, 2_u8, 3_u8, 3_u8, 3_u8, 3_u8, 4_u8, 4_u8, 4_u8, 4_u8,
            4_u8, 4_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 5_u8, 6_u8, 6_u8, 6_u8, 6_u8,
            6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 6_u8, 7_u8, 7_u8,
            7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 7_u8,
            7_u8, 7_u8, 7_u8, 7_u8, 7_u8, 2_u8, 2_u8, 2_u8,
        ],
        [
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 4_u8, 5_u8, 5_u8, 6_u8, 6_u8, 7_u8, 7_u8, 8_u8, 8_u8,
            8_u8, 8_u8, 9_u8, 9_u8, 9_u8, 9_u8, 10_u8, 10_u8, 10_u8, 10_u8, 11_u8, 11_u8, 11_u8,
            11_u8, 12_u8, 12_u8, 12_u8, 12_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8, 13_u8,
            13_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 14_u8, 15_u8, 15_u8, 15_u8,
            15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8, 15_u8,
            15_u8,
        ],
        [
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
            14_u8, 15_u8, 16_u8, 16_u8, 17_u8, 17_u8, 18_u8, 18_u8, 19_u8, 19_u8, 20_u8, 20_u8,
            21_u8, 21_u8, 22_u8, 22_u8, 23_u8, 23_u8, 24_u8, 24_u8, 24_u8, 24_u8, 25_u8, 25_u8,
            25_u8, 25_u8, 26_u8, 26_u8, 26_u8, 26_u8, 27_u8, 27_u8, 27_u8, 27_u8, 28_u8, 28_u8,
            28_u8, 28_u8, 29_u8, 29_u8, 29_u8, 29_u8, 30_u8, 30_u8, 30_u8, 30_u8, 31_u8, 31_u8,
            31_u8, 31_u8,
        ],
        [
            0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8,
            14_u8, 15_u8, 16_u8, 17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8,
            26_u8, 27_u8, 28_u8, 29_u8, 30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8,
            38_u8, 39_u8, 40_u8, 41_u8, 42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8,
            50_u8, 51_u8, 52_u8, 53_u8, 54_u8, 55_u8, 56_u8, 57_u8, 58_u8, 59_u8, 60_u8, 61_u8,
            62_u8, 63_u8,
        ],
    ]
};
pub static mut kNumNonzeroContext_93: [[u16; 64]; 7] = unsafe {
    [
        [
            0_u16, 1_u16, 1_u16, 2_u16, 2_u16, 2_u16, 3_u16, 3_u16, 3_u16, 3_u16, 4_u16, 4_u16,
            4_u16, 4_u16, 4_u16, 4_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16, 5_u16,
            6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16, 6_u16,
            6_u16, 6_u16, 6_u16, 6_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16,
            7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16, 7_u16,
            7_u16, 7_u16, 7_u16, 7_u16,
        ],
        [
            0_u16, 2_u16, 2_u16, 4_u16, 4_u16, 4_u16, 6_u16, 6_u16, 6_u16, 6_u16, 8_u16, 8_u16,
            8_u16, 8_u16, 8_u16, 8_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16, 10_u16,
            10_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16,
            12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 12_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16,
            14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16,
            14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16, 14_u16,
        ],
        [
            0_u16, 4_u16, 4_u16, 8_u16, 8_u16, 8_u16, 12_u16, 12_u16, 12_u16, 12_u16, 16_u16,
            16_u16, 16_u16, 16_u16, 16_u16, 16_u16, 20_u16, 20_u16, 20_u16, 20_u16, 20_u16, 20_u16,
            20_u16, 20_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16,
            24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 24_u16, 28_u16, 28_u16, 28_u16, 28_u16,
            28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16,
            28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16, 28_u16,
        ],
        [
            0_u16, 8_u16, 8_u16, 16_u16, 16_u16, 16_u16, 24_u16, 24_u16, 24_u16, 24_u16, 32_u16,
            32_u16, 32_u16, 32_u16, 32_u16, 32_u16, 40_u16, 40_u16, 40_u16, 40_u16, 40_u16, 40_u16,
            40_u16, 40_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16,
            48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 48_u16, 55_u16, 55_u16, 55_u16, 55_u16,
            55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16,
            55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16, 55_u16,
        ],
        [
            0_u16, 16_u16, 16_u16, 32_u16, 32_u16, 32_u16, 48_u16, 48_u16, 48_u16, 48_u16, 64_u16,
            64_u16, 64_u16, 64_u16, 64_u16, 64_u16, 80_u16, 80_u16, 80_u16, 80_u16, 80_u16, 80_u16,
            80_u16, 80_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16,
            95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 95_u16, 109_u16, 109_u16, 109_u16,
            109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16,
            109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16, 109_u16,
            109_u16, 109_u16, 109_u16,
        ],
        [
            0_u16, 32_u16, 32_u16, 64_u16, 64_u16, 64_u16, 96_u16, 96_u16, 96_u16, 96_u16, 127_u16,
            127_u16, 127_u16, 127_u16, 127_u16, 127_u16, 157_u16, 157_u16, 157_u16, 157_u16,
            157_u16, 157_u16, 157_u16, 157_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16,
            185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16, 185_u16,
            185_u16, 185_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
            211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
            211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16, 211_u16,
        ],
        [
            0_u16, 64_u16, 64_u16, 127_u16, 127_u16, 127_u16, 188_u16, 188_u16, 188_u16, 188_u16,
            246_u16, 246_u16, 246_u16, 246_u16, 246_u16, 246_u16, 300_u16, 300_u16, 300_u16,
            300_u16, 300_u16, 300_u16, 300_u16, 300_u16, 348_u16, 348_u16, 348_u16, 348_u16,
            348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16, 348_u16,
            348_u16, 348_u16, 348_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
            388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
            388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16, 388_u16,
        ],
    ]
};
pub static mut kNumNonzeroContextSkip_94: [u16; 7] =
    unsafe { [8_u16, 15_u16, 31_u16, 61_u16, 120_u16, 231_u16, 412_u16] };
pub static mut kContextAlgorithm_95: [u8; 128] = unsafe {
    [
        0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 2_u8, 3_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8,
        0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 2_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 2_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8,
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        2_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
    ]
};
pub unsafe fn ZeroDensityContext_96(
    mut nonzeros_left: usize,
    mut k: usize,
    mut bits: usize,
) -> u16 {
    return (((kNumNonzeroContext_93[(bits)][(nonzeros_left)] as i32)
        + (kFreqContext_92[(bits)][(k)] as i32)) as u16);
}
pub unsafe fn WeightedAverageContextDC_97(mut vals: *const i32, mut x: i32) -> i32 {
    let mut sum: i32 = (((((1) + (*vals.offset(((x) - (2)) as isize)))
        + (*vals.offset(((x) - (1)) as isize)))
        + (*vals.offset((x) as isize)))
        + (*vals.offset(((x) + (1)) as isize)));
    if (((sum) >> (kMaxAverageContext_82)) != (0)) {
        return (kMaxAverageContext_82 as i32);
    }
    return (unsafe { Log2FloorNonZero_74((sum as u32)) });
}
pub unsafe fn WeightedAverageContext_98(mut vals: *const i32, mut prev_row_delta: i32) -> i32 {
    let mut sum: i32 = ((((((4) + (*vals.offset((0) as isize)))
        + (((*vals.offset((-kDCTBlockSize_3) as isize))
            + (*vals.offset((prev_row_delta) as isize)))
            * (2)))
        + (*vals.offset(((-2_i32) * (kDCTBlockSize_3)) as isize)))
        + (*vals.offset(((prev_row_delta) - (kDCTBlockSize_3)) as isize)))
        + (*vals.offset(((prev_row_delta) + (kDCTBlockSize_3)) as isize)));
    if (((sum) >> ((kMaxAverageContext_82).wrapping_add(2_usize))) != (0)) {
        return (kMaxAverageContext_82 as i32);
    }
    return ((unsafe { Log2FloorNonZero_74((sum as u32)) }) - (2));
}
pub static mut kACPredictPrecisionBits_99: i32 = unsafe { 13 };
pub static mut kACPredictPrecision_100: i32 = unsafe { ((1) << (kACPredictPrecisionBits_99)) };
pub unsafe fn ACPredictContext_101(mut p: i64, mut avg_ctx: *mut usize, mut sgn: *mut usize) {
    let mut multiplier: i32 = 0_i32;
    if ((p) >= (0_i64)) {
        multiplier = 1;
    } else {
        multiplier = -1_i32;
        p = -p;
    }
    let mut ctx: usize = 0_usize;
    if ((p) >= (((1_u32) << (kMaxAverageContext_82)) as i64)) {
        ctx = kMaxAverageContext_82;
    } else {
        ctx = ((unsafe {
            Log2FloorNonZero_74(((2_u32).wrapping_mul((p as u32))).wrapping_add(1_u32))
        }) as usize);
    }
    (*avg_ctx) = ctx;
    (*sgn) = (kMaxAverageContext_82).wrapping_add((multiplier as usize).wrapping_mul(ctx));
}
pub unsafe fn ACPredictContextCol_102(
    mut prev: *const i16,
    mut cur: *const i16,
    mut mult: *const i32,
    mut avg_ctx: *mut usize,
    mut sgn: *mut usize,
) {
    let mut terms: [i16; 8] = [0_i16; 8];
    terms[(0) as usize] = 0_i16;
    terms[(1) as usize] =
        ((((*cur.offset((1) as isize)) as i32) + ((*prev.offset((1) as isize)) as i32)) as i16);
    terms[(2) as usize] =
        ((((*cur.offset((2) as isize)) as i32) - ((*prev.offset((2) as isize)) as i32)) as i16);
    terms[(3) as usize] =
        ((((*cur.offset((3) as isize)) as i32) + ((*prev.offset((3) as isize)) as i32)) as i16);
    terms[(4) as usize] =
        ((((*cur.offset((4) as isize)) as i32) - ((*prev.offset((4) as isize)) as i32)) as i16);
    terms[(5) as usize] =
        ((((*cur.offset((5) as isize)) as i32) + ((*prev.offset((5) as isize)) as i32)) as i16);
    terms[(6) as usize] =
        ((((*cur.offset((6) as isize)) as i32) - ((*prev.offset((6) as isize)) as i32)) as i16);
    terms[(7) as usize] =
        ((((*cur.offset((7) as isize)) as i32) + ((*prev.offset((7) as isize)) as i32)) as i16);
    let mut delta: i64 = (((((((((terms[(0) as usize] as i64)
        * ((*mult.offset((0) as isize)) as i64))
        + ((terms[(1) as usize] as i64) * ((*mult.offset((1) as isize)) as i64)))
        + ((terms[(2) as usize] as i64) * ((*mult.offset((2) as isize)) as i64)))
        + ((terms[(3) as usize] as i64) * ((*mult.offset((3) as isize)) as i64)))
        + ((terms[(4) as usize] as i64) * ((*mult.offset((4) as isize)) as i64)))
        + ((terms[(5) as usize] as i64) * ((*mult.offset((5) as isize)) as i64)))
        + ((terms[(6) as usize] as i64) * ((*mult.offset((6) as isize)) as i64)))
        + ((terms[(7) as usize] as i64) * ((*mult.offset((7) as isize)) as i64)));
    (unsafe {
        ACPredictContext_101(
            (((*prev.offset((0) as isize)) as i64) - ((delta) / (kACPredictPrecision_100 as i64))),
            avg_ctx,
            sgn,
        )
    });
}
pub unsafe fn ACPredictContextRow_103(
    mut prev: *const i16,
    mut cur: *const i16,
    mut mult: *const i32,
    mut avg_ctx: *mut usize,
    mut sgn: *mut usize,
) {
    let mut terms: [i16; 8] = [0_i16; 8];
    terms[(0) as usize] = 0_i16;
    terms[(1) as usize] =
        ((((*cur.offset((8) as isize)) as i32) + ((*prev.offset((8) as isize)) as i32)) as i16);
    terms[(2) as usize] =
        ((((*cur.offset((16) as isize)) as i32) - ((*prev.offset((16) as isize)) as i32)) as i16);
    terms[(3) as usize] =
        ((((*cur.offset((24) as isize)) as i32) + ((*prev.offset((24) as isize)) as i32)) as i16);
    terms[(4) as usize] =
        ((((*cur.offset((32) as isize)) as i32) - ((*prev.offset((32) as isize)) as i32)) as i16);
    terms[(5) as usize] =
        ((((*cur.offset((40) as isize)) as i32) + ((*prev.offset((40) as isize)) as i32)) as i16);
    terms[(6) as usize] =
        ((((*cur.offset((48) as isize)) as i32) - ((*prev.offset((48) as isize)) as i32)) as i16);
    terms[(7) as usize] =
        ((((*cur.offset((56) as isize)) as i32) + ((*prev.offset((56) as isize)) as i32)) as i16);
    let mut delta: i64 = (((((((((terms[(0) as usize] as i64)
        * ((*mult.offset((0) as isize)) as i64))
        + ((terms[(1) as usize] as i64) * ((*mult.offset((1) as isize)) as i64)))
        + ((terms[(2) as usize] as i64) * ((*mult.offset((2) as isize)) as i64)))
        + ((terms[(3) as usize] as i64) * ((*mult.offset((3) as isize)) as i64)))
        + ((terms[(4) as usize] as i64) * ((*mult.offset((4) as isize)) as i64)))
        + ((terms[(5) as usize] as i64) * ((*mult.offset((5) as isize)) as i64)))
        + ((terms[(6) as usize] as i64) * ((*mult.offset((6) as isize)) as i64)))
        + ((terms[(7) as usize] as i64) * ((*mult.offset((7) as isize)) as i64)));
    (unsafe {
        ACPredictContext_101(
            (((*prev.offset((0) as isize)) as i64) - ((delta) / (kACPredictPrecision_100 as i64))),
            avg_ctx,
            sgn,
        )
    });
}
pub unsafe fn NumNonzerosContext_104(mut prev: *const u8, mut x: i32, mut y: i32) -> u8 {
    let mut prediction: usize = 0_usize;
    if ((y) == (0)) {
        if ((x) == (0)) {
            prediction = 0_usize;
        } else {
            prediction = ((*prev.offset(((x) - (1)) as isize)) as usize);
        }
    } else if ((x) == (0)) {
        prediction = ((*prev.offset((x) as isize)) as usize);
    } else {
        prediction = ((((((*prev.offset(((x) - (1)) as isize)) as i32)
            + ((*prev.offset((x) as isize)) as i32))
            + (1))
            / (2)) as usize);
    }
    if !((prediction) <= (kNumNonZeroTreeSize_85)) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"context.cc".as_ptr(), 305, c"NumNonzerosContext".as_ptr())
        });
        'loop_: while true {}
    };
    return (((prediction).wrapping_div(kNumNonZeroQuant_86)) as u8);
}
pub static mut kNumIsEmptyBlockContexts_105: i32 = unsafe { 3 };
pub unsafe fn IsEmptyBlockContext_106(mut prev: *const i32, mut x: i32) -> i32 {
    return ((*prev.offset(((x) - (1)) as isize)) + (*prev.offset((x) as isize)));
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_ComponentStateDC {
    pub width: i32,
    pub is_zero_prob: brunsli_Prob,
    pub is_empty_block_prob: Vec<brunsli_Prob>,
    pub sign_prob: Vec<brunsli_Prob>,
    pub first_extra_bit_prob: Vec<brunsli_Prob>,
    pub prev_is_nonempty: Vec<i32>,
    pub prev_abs_coeff: Vec<i32>,
    pub prev_sign: Vec<i32>,
}
impl brunsli_ComponentStateDC {
    pub unsafe fn brunsli_ComponentStateDC() -> Self {
        let mut this = Self {
            width: 0,
            is_zero_prob: brunsli_Prob::brunsli_Prob(),
            is_empty_block_prob: (0..(kNumIsEmptyBlockContexts_105 as usize) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            sign_prob: (0..(9_usize) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            first_extra_bit_prob: (0..(10_usize) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            prev_is_nonempty: Vec::new(),
            prev_abs_coeff: Vec::new(),
            prev_sign: Vec::new(),
        };
        (unsafe { this.InitAll() });
        this
    }
    pub unsafe fn SetWidth(&mut self, mut w: i32) {
        self.width = w;
        {
            let __a0 = (((w) + (1)) as usize) as usize;
            self.prev_is_nonempty.resize(__a0, 1)
        };
        {
            let __a0 = (((w) + (3)) as usize) as usize;
            self.prev_abs_coeff.resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((w) + (1)) as usize) as usize;
            self.prev_sign.resize_with(__a0, || <i32>::default())
        };
    }
}
impl Default for brunsli_ComponentStateDC {
    fn default() -> Self {
        unsafe { brunsli_ComponentStateDC::brunsli_ComponentStateDC() }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_ComponentState {
    pub width: i32,
    pub context_offset: i32,
    pub order: [u32; 64],
    pub mult_row: [i32; 64],
    pub mult_col: [i32; 64],
    pub is_zero_prob: Vec<brunsli_Prob>,
    pub sign_prob: Vec<brunsli_Prob>,
    pub num_nonzero_prob: [brunsli_Prob; 2016],
    pub first_extra_bit_prob: Vec<brunsli_Prob>,
    pub prev_is_nonempty: Vec<i32>,
    pub prev_num_nonzeros: Vec<u8>,
    pub prev_abs_coeff: Vec<i32>,
    pub prev_sign: Vec<i32>,
}
impl brunsli_ComponentState {
    pub unsafe fn brunsli_ComponentState() -> Self {
        let mut this = Self {
            width: 0,
            context_offset: 0_i32,
            order: [0_u32; 64],
            mult_row: [0_i32; 64],
            mult_col: [0_i32; 64],
            is_zero_prob: (0..(((kNumNonzeroBuckets_90 as i32) * (kDCTBlockSize_3)) as usize)
                as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            sign_prob: (0..(((((2_usize).wrapping_mul(kMaxAverageContext_82) as usize)
                .wrapping_add(1_usize)) as usize)
                .wrapping_mul((kDCTBlockSize_3 as usize))) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            num_nonzero_prob: std::array::from_fn::<_, 2016, _>(|_| brunsli_Prob::brunsli_Prob()),
            first_extra_bit_prob: (0..(((10) * (kDCTBlockSize_3)) as usize) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            prev_is_nonempty: Vec::new(),
            prev_num_nonzeros: Vec::new(),
            prev_abs_coeff: Vec::new(),
            prev_sign: Vec::new(),
        };
        (unsafe { this.InitAll() });
        this
    }
    pub unsafe fn SetWidth(&mut self, mut w: i32) {
        self.width = w;
        {
            let __a0 = (((w) + (1)) as usize) as usize;
            self.prev_is_nonempty.resize(__a0, 1)
        };
        {
            let __a0 = (w as usize) as usize;
            self.prev_num_nonzeros.resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 = ((((kDCTBlockSize_3) * (2)) * ((w) + (3))) as usize) as usize;
            self.prev_abs_coeff.resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((kDCTBlockSize_3) * ((w) + (1))) as usize) as usize;
            self.prev_sign.resize_with(__a0, || <i32>::default())
        };
    }
    pub unsafe fn SizeInBytes(mut w: i32) -> usize {
        return ((((((4) + (((10) + ((3) * (w))) * (kDCTBlockSize_3))) + ((2) * (w))) as usize)
            .wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            .wrapping_add(
                ((((((((kNumNonzeroBuckets_90 as usize)
                    .wrapping_add(((2_usize).wrapping_mul(kMaxAverageContext_82) as usize))
                    as usize)
                    .wrapping_add(11_usize)) as usize)
                    .wrapping_mul((kDCTBlockSize_3 as usize)) as usize)
                    .wrapping_add(
                        ((kNumNonZeroContextCount_88).wrapping_mul(kNumNonZeroTreeSize_85)
                            as usize),
                    )) as u64)
                    .wrapping_mul((::std::mem::size_of::<brunsli_Prob>() as u64))
                    as u64),
            ) as usize);
    }
}
impl Default for brunsli_ComponentState {
    fn default() -> Self {
        unsafe { brunsli_ComponentState::brunsli_ComponentState() }
    }
}
pub static mut kSqrt2_107: f64 = unsafe { 1.414213562E+0 };
pub static mut kSqrt2FixedPoint_108: i32 =
    unsafe { (((kSqrt2_107) * (kACPredictPrecision_100 as f64)) as i32) };
pub unsafe fn ComputeACPredictMultipliers_109(
    mut quant: *const i32,
    mut mult_row: *mut i32,
    mut mult_col: *mut i32,
) {
    let mut y: usize = 0_usize;
    'loop_: while ((y) < (8_usize)) {
        let mut x: usize = 0_usize;
        'loop_: while ((x) < (8_usize)) {
            (*mult_row.offset(((x).wrapping_add((8_usize).wrapping_mul(y))) as isize)) = (((*quant
                .offset(((x).wrapping_add((8_usize).wrapping_mul(y))) as isize))
                * (kSqrt2FixedPoint_108))
                / (*quant.offset(((y).wrapping_mul(8_usize)) as isize)));
            (*mult_col.offset((((x).wrapping_mul(8_usize)).wrapping_add(y)) as isize)) = (((*quant
                .offset(((x).wrapping_add((8_usize).wrapping_mul(y))) as isize))
                * (kSqrt2FixedPoint_108))
                / (*quant.offset((x) as isize)));
            x.prefix_inc();
        }
        y.prefix_inc();
    }
}
impl brunsli_ComponentStateDC {
    unsafe fn InitAll(&mut self) {
        (unsafe { self.is_zero_prob.Init(135_u8) });
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.sign_prob.len())) {
            (unsafe { self.sign_prob[(i)].Init(128_u8) });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.is_empty_block_prob.len())) {
            (unsafe { self.is_empty_block_prob[(i)].Init(74_u8) });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.first_extra_bit_prob.len())) {
            (unsafe { self.first_extra_bit_prob[(i)].Init(150_u8) });
            i.prefix_inc();
        }
    }
}
pub static mut kInitProb_110: [u8; 64] = unsafe {
    [
        228_u8, 216_u8, 216_u8, 195_u8, 192_u8, 189_u8, 182_u8, 184_u8, 179_u8, 176_u8, 171_u8,
        168_u8, 166_u8, 159_u8, 156_u8, 151_u8, 151_u8, 150_u8, 150_u8, 146_u8, 144_u8, 138_u8,
        138_u8, 137_u8, 135_u8, 131_u8, 127_u8, 126_u8, 124_u8, 123_u8, 124_u8, 123_u8, 122_u8,
        121_u8, 118_u8, 117_u8, 114_u8, 115_u8, 116_u8, 116_u8, 115_u8, 115_u8, 114_u8, 111_u8,
        111_u8, 111_u8, 112_u8, 111_u8, 110_u8, 110_u8, 110_u8, 111_u8, 111_u8, 114_u8, 110_u8,
        111_u8, 112_u8, 113_u8, 116_u8, 120_u8, 126_u8, 131_u8, 147_u8, 160_u8,
    ]
};
pub static mut kInitProbNonzero_111: [[u8; 63]; 32] = unsafe {
    [
        [
            251_u8, 252_u8, 117_u8, 249_u8, 161_u8, 136_u8, 83_u8, 238_u8, 184_u8, 126_u8, 137_u8,
            129_u8, 140_u8, 119_u8, 70_u8, 213_u8, 160_u8, 175_u8, 174_u8, 130_u8, 166_u8, 134_u8,
            122_u8, 125_u8, 131_u8, 144_u8, 136_u8, 133_u8, 139_u8, 123_u8, 79_u8, 216_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            254_u8, 252_u8, 174_u8, 232_u8, 189_u8, 155_u8, 122_u8, 177_u8, 204_u8, 173_u8, 146_u8,
            149_u8, 141_u8, 133_u8, 103_u8, 109_u8, 167_u8, 187_u8, 168_u8, 142_u8, 154_u8, 147_u8,
            125_u8, 139_u8, 144_u8, 138_u8, 138_u8, 153_u8, 141_u8, 133_u8, 90_u8, 121_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            251_u8, 240_u8, 197_u8, 176_u8, 184_u8, 177_u8, 114_u8, 89_u8, 194_u8, 165_u8, 153_u8,
            161_u8, 158_u8, 136_u8, 92_u8, 95_u8, 123_u8, 171_u8, 160_u8, 140_u8, 148_u8, 136_u8,
            129_u8, 139_u8, 145_u8, 136_u8, 143_u8, 134_u8, 138_u8, 124_u8, 92_u8, 154_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            247_u8, 220_u8, 201_u8, 110_u8, 194_u8, 176_u8, 147_u8, 59_u8, 175_u8, 171_u8, 156_u8,
            157_u8, 152_u8, 146_u8, 115_u8, 114_u8, 88_u8, 151_u8, 164_u8, 141_u8, 153_u8, 135_u8,
            141_u8, 131_u8, 146_u8, 139_u8, 140_u8, 145_u8, 138_u8, 137_u8, 112_u8, 184_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            238_u8, 179_u8, 203_u8, 63_u8, 194_u8, 173_u8, 149_u8, 71_u8, 139_u8, 169_u8, 154_u8,
            159_u8, 150_u8, 146_u8, 117_u8, 143_u8, 78_u8, 122_u8, 152_u8, 137_u8, 149_u8, 138_u8,
            138_u8, 133_u8, 134_u8, 142_u8, 142_u8, 142_u8, 148_u8, 128_u8, 118_u8, 199_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            227_u8, 127_u8, 200_u8, 44_u8, 192_u8, 170_u8, 148_u8, 100_u8, 102_u8, 161_u8, 156_u8,
            153_u8, 148_u8, 149_u8, 124_u8, 160_u8, 88_u8, 101_u8, 134_u8, 132_u8, 149_u8, 145_u8,
            134_u8, 134_u8, 136_u8, 141_u8, 138_u8, 142_u8, 144_u8, 137_u8, 116_u8, 208_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            214_u8, 86_u8, 195_u8, 44_u8, 187_u8, 163_u8, 148_u8, 126_u8, 81_u8, 147_u8, 156_u8,
            152_u8, 150_u8, 144_u8, 121_u8, 172_u8, 96_u8, 95_u8, 117_u8, 122_u8, 145_u8, 152_u8,
            136_u8, 133_u8, 135_u8, 135_u8, 131_u8, 142_u8, 141_u8, 135_u8, 114_u8, 217_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            198_u8, 56_u8, 191_u8, 54_u8, 171_u8, 162_u8, 147_u8, 144_u8, 74_u8, 128_u8, 152_u8,
            149_u8, 150_u8, 142_u8, 119_u8, 177_u8, 101_u8, 100_u8, 106_u8, 111_u8, 135_u8, 154_u8,
            136_u8, 137_u8, 136_u8, 132_u8, 133_u8, 142_u8, 144_u8, 130_u8, 117_u8, 222_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            176_u8, 40_u8, 189_u8, 73_u8, 147_u8, 159_u8, 148_u8, 152_u8, 79_u8, 106_u8, 147_u8,
            149_u8, 151_u8, 139_u8, 123_u8, 188_u8, 108_u8, 110_u8, 106_u8, 97_u8, 125_u8, 151_u8,
            137_u8, 138_u8, 135_u8, 135_u8, 134_u8, 136_u8, 140_u8, 131_u8, 116_u8, 221_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            148_u8, 33_u8, 185_u8, 88_u8, 117_u8, 158_u8, 145_u8, 163_u8, 95_u8, 91_u8, 137_u8,
            146_u8, 150_u8, 140_u8, 120_u8, 197_u8, 115_u8, 116_u8, 114_u8, 92_u8, 114_u8, 144_u8,
            130_u8, 133_u8, 132_u8, 133_u8, 129_u8, 140_u8, 138_u8, 130_u8, 111_u8, 224_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            117_u8, 31_u8, 180_u8, 104_u8, 93_u8, 150_u8, 143_u8, 166_u8, 99_u8, 85_u8, 124_u8,
            139_u8, 148_u8, 142_u8, 118_u8, 201_u8, 105_u8, 120_u8, 120_u8, 90_u8, 107_u8, 135_u8,
            127_u8, 130_u8, 131_u8, 131_u8, 132_u8, 140_u8, 142_u8, 133_u8, 114_u8, 229_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            87_u8, 35_u8, 170_u8, 110_u8, 78_u8, 141_u8, 144_u8, 176_u8, 106_u8, 90_u8, 112_u8,
            132_u8, 143_u8, 138_u8, 119_u8, 204_u8, 111_u8, 121_u8, 125_u8, 90_u8, 105_u8, 131_u8,
            124_u8, 122_u8, 129_u8, 128_u8, 129_u8, 137_u8, 138_u8, 133_u8, 114_u8, 227_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            63_u8, 42_u8, 159_u8, 123_u8, 73_u8, 127_u8, 142_u8, 191_u8, 105_u8, 91_u8, 105_u8,
            123_u8, 139_u8, 137_u8, 120_u8, 209_u8, 117_u8, 110_u8, 122_u8, 98_u8, 110_u8, 125_u8,
            115_u8, 123_u8, 122_u8, 126_u8, 128_u8, 134_u8, 141_u8, 129_u8, 113_u8, 229_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            45_u8, 53_u8, 146_u8, 135_u8, 71_u8, 114_u8, 138_u8, 193_u8, 100_u8, 98_u8, 98_u8,
            113_u8, 133_u8, 135_u8, 118_u8, 222_u8, 113_u8, 111_u8, 139_u8, 103_u8, 107_u8, 126_u8,
            111_u8, 119_u8, 121_u8, 122_u8, 127_u8, 135_u8, 141_u8, 128_u8, 114_u8, 242_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            33_u8, 60_u8, 132_u8, 138_u8, 75_u8, 100_u8, 134_u8, 203_u8, 112_u8, 99_u8, 98_u8,
            105_u8, 126_u8, 131_u8, 115_u8, 229_u8, 107_u8, 93_u8, 121_u8, 106_u8, 108_u8, 122_u8,
            106_u8, 109_u8, 114_u8, 116_u8, 127_u8, 133_u8, 143_u8, 128_u8, 110_u8, 242_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            24_u8, 70_u8, 118_u8, 134_u8, 76_u8, 87_u8, 130_u8, 201_u8, 110_u8, 96_u8, 99_u8,
            97_u8, 119_u8, 130_u8, 111_u8, 229_u8, 97_u8, 104_u8, 125_u8, 102_u8, 112_u8, 125_u8,
            101_u8, 109_u8, 113_u8, 114_u8, 125_u8, 129_u8, 142_u8, 127_u8, 112_u8, 241_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            17_u8, 65_u8, 100_u8, 121_u8, 80_u8, 75_u8, 124_u8, 174_u8, 117_u8, 100_u8, 94_u8,
            93_u8, 114_u8, 128_u8, 110_u8, 216_u8, 103_u8, 94_u8, 113_u8, 122_u8, 118_u8, 126_u8,
            113_u8, 108_u8, 105_u8, 108_u8, 122_u8, 128_u8, 141_u8, 125_u8, 113_u8, 238_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            12_u8, 70_u8, 82_u8, 132_u8, 78_u8, 65_u8, 118_u8, 155_u8, 136_u8, 103_u8, 97_u8,
            89_u8, 106_u8, 124_u8, 111_u8, 215_u8, 115_u8, 123_u8, 129_u8, 99_u8, 104_u8, 127_u8,
            110_u8, 108_u8, 101_u8, 109_u8, 118_u8, 126_u8, 136_u8, 123_u8, 110_u8, 233_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            8_u8, 66_u8, 61_u8, 117_u8, 91_u8, 59_u8, 108_u8, 195_u8, 101_u8, 112_u8, 99_u8, 99_u8,
            99_u8, 116_u8, 106_u8, 230_u8, 127_u8, 99_u8, 144_u8, 101_u8, 118_u8, 137_u8, 117_u8,
            111_u8, 106_u8, 104_u8, 116_u8, 121_u8, 134_u8, 122_u8, 110_u8, 223_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            6_u8, 78_u8, 42_u8, 146_u8, 101_u8, 54_u8, 94_u8, 201_u8, 116_u8, 102_u8, 110_u8,
            94_u8, 92_u8, 108_u8, 103_u8, 214_u8, 108_u8, 111_u8, 127_u8, 102_u8, 121_u8, 132_u8,
            120_u8, 121_u8, 95_u8, 98_u8, 110_u8, 121_u8, 129_u8, 117_u8, 107_u8, 235_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            5_u8, 93_u8, 29_u8, 145_u8, 102_u8, 52_u8, 77_u8, 216_u8, 108_u8, 115_u8, 108_u8,
            102_u8, 89_u8, 97_u8, 94_u8, 229_u8, 89_u8, 103_u8, 139_u8, 120_u8, 103_u8, 151_u8,
            102_u8, 100_u8, 97_u8, 96_u8, 99_u8, 111_u8, 125_u8, 116_u8, 104_u8, 242_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            4_u8, 105_u8, 21_u8, 145_u8, 100_u8, 54_u8, 64_u8, 217_u8, 100_u8, 122_u8, 128_u8,
            87_u8, 88_u8, 91_u8, 87_u8, 230_u8, 112_u8, 80_u8, 148_u8, 95_u8, 146_u8, 123_u8,
            96_u8, 140_u8, 90_u8, 91_u8, 98_u8, 106_u8, 122_u8, 111_u8, 100_u8, 249_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            4_u8, 130_u8, 14_u8, 142_u8, 104_u8, 56_u8, 51_u8, 208_u8, 116_u8, 135_u8, 100_u8,
            89_u8, 82_u8, 84_u8, 75_u8, 239_u8, 85_u8, 85_u8, 122_u8, 125_u8, 94_u8, 144_u8,
            151_u8, 136_u8, 92_u8, 97_u8, 104_u8, 109_u8, 113_u8, 110_u8, 91_u8, 246_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            3_u8, 126_u8, 9_u8, 172_u8, 105_u8, 57_u8, 39_u8, 219_u8, 95_u8, 120_u8, 118_u8, 96_u8,
            93_u8, 75_u8, 66_u8, 241_u8, 102_u8, 134_u8, 96_u8, 156_u8, 146_u8, 162_u8, 130_u8,
            112_u8, 82_u8, 89_u8, 97_u8, 101_u8, 116_u8, 103_u8, 82_u8, 254_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            3_u8, 149_u8, 7_u8, 182_u8, 122_u8, 54_u8, 29_u8, 224_u8, 103_u8, 100_u8, 113_u8,
            96_u8, 90_u8, 74_u8, 55_u8, 250_u8, 127_u8, 94_u8, 118_u8, 93_u8, 135_u8, 160_u8,
            113_u8, 130_u8, 95_u8, 117_u8, 106_u8, 96_u8, 111_u8, 97_u8, 77_u8, 242_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            3_u8, 150_u8, 4_u8, 170_u8, 138_u8, 59_u8, 20_u8, 229_u8, 91_u8, 150_u8, 107_u8, 98_u8,
            92_u8, 68_u8, 48_u8, 245_u8, 113_u8, 64_u8, 114_u8, 111_u8, 134_u8, 127_u8, 102_u8,
            104_u8, 85_u8, 118_u8, 103_u8, 107_u8, 102_u8, 91_u8, 72_u8, 245_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            3_u8, 171_u8, 3_u8, 165_u8, 137_u8, 62_u8, 14_u8, 211_u8, 96_u8, 127_u8, 132_u8,
            121_u8, 95_u8, 62_u8, 37_u8, 248_u8, 102_u8, 57_u8, 144_u8, 85_u8, 127_u8, 191_u8,
            102_u8, 97_u8, 127_u8, 104_u8, 91_u8, 102_u8, 107_u8, 81_u8, 64_u8, 254_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            2_u8, 166_u8, 2_u8, 196_u8, 122_u8, 65_u8, 10_u8, 243_u8, 102_u8, 93_u8, 117_u8, 92_u8,
            96_u8, 63_u8, 29_u8, 251_u8, 169_u8, 159_u8, 149_u8, 96_u8, 91_u8, 139_u8, 157_u8,
            40_u8, 100_u8, 89_u8, 120_u8, 92_u8, 109_u8, 79_u8, 58_u8, 247_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            2_u8, 176_u8, 2_u8, 189_u8, 118_u8, 48_u8, 7_u8, 219_u8, 68_u8, 43_u8, 109_u8, 96_u8,
            129_u8, 75_u8, 19_u8, 254_u8, 2_u8, 3_u8, 185_u8, 6_u8, 102_u8, 127_u8, 127_u8, 127_u8,
            1_u8, 131_u8, 83_u8, 99_u8, 107_u8, 80_u8, 45_u8, 254_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            1_u8, 205_u8, 2_u8, 208_u8, 64_u8, 89_u8, 4_u8, 223_u8, 29_u8, 169_u8, 29_u8, 123_u8,
            118_u8, 76_u8, 11_u8, 240_u8, 202_u8, 243_u8, 65_u8, 6_u8, 12_u8, 243_u8, 96_u8, 55_u8,
            102_u8, 102_u8, 114_u8, 102_u8, 107_u8, 74_u8, 31_u8, 247_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            1_u8, 216_u8, 1_u8, 214_u8, 127_u8, 94_u8, 2_u8, 234_u8, 145_u8, 3_u8, 127_u8, 106_u8,
            155_u8, 80_u8, 4_u8, 247_u8, 4_u8, 65_u8, 86_u8, 127_u8, 127_u8, 127_u8, 127_u8,
            102_u8, 127_u8, 143_u8, 143_u8, 108_u8, 113_u8, 80_u8, 16_u8, 216_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
        [
            2_u8, 199_u8, 1_u8, 222_u8, 93_u8, 94_u8, 1_u8, 232_u8, 2_u8, 65_u8, 74_u8, 139_u8,
            201_u8, 48_u8, 2_u8, 254_u8, 169_u8, 127_u8, 52_u8, 243_u8, 251_u8, 249_u8, 102_u8,
            86_u8, 202_u8, 153_u8, 65_u8, 65_u8, 146_u8, 69_u8, 8_u8, 238_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
            128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8, 128_u8,
        ],
    ]
};
impl brunsli_ComponentState {
    unsafe fn InitAll(&mut self) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (kNumNonzeroBuckets_90 as i32)) {
            let mut k: i32 = 0;
            'loop_: while ((k) < (kDCTBlockSize_3)) {
                let v: i32 = ((kInitProb_110[(k) as usize] as i32) + ((9) * ((i) - (7))));
                if !((v) <= (255)) {
                    (unsafe {
                        BrunsliDumpAndAbort_79(c"context.cc".as_ptr(), 227, c"InitAll".as_ptr())
                    });
                    'loop_: while true {}
                };
                (unsafe {
                    self.is_zero_prob[((((i) * (kDCTBlockSize_3)) + (k)) as usize)].Init((v as u8))
                });
                k.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.sign_prob.len())) {
            if ((i) < ((kMaxAverageContext_82).wrapping_mul((kDCTBlockSize_3 as usize)))) {
                (unsafe { self.sign_prob[(i)].Init(108_u8) });
            } else if ((i)
                < ((((kMaxAverageContext_82).wrapping_add(1_usize)) as usize)
                    .wrapping_mul((kDCTBlockSize_3 as usize))))
            {
                (unsafe { self.sign_prob[(i)].Init(128_u8) });
            } else {
                (unsafe { self.sign_prob[(i)].Init(148_u8) });
            }
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.first_extra_bit_prob.len())) {
            (unsafe { self.first_extra_bit_prob[(i)].Init(158_u8) });
            i.prefix_inc();
        }
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (kNumNonZeroContextCount_88)) {
            let mut non_zero_probs: *mut brunsli_Prob = self
                .num_nonzero_prob
                .as_mut_ptr()
                .offset(((i).wrapping_mul(kNumNonZeroTreeSize_85)) as isize);
            let mut j: usize = 0_usize;
            'loop_: while ((j) < (kNumNonZeroTreeSize_85)) {
                (unsafe {
                    let _probability: u8 = kInitProbNonzero_111[(i)][(j)];
                    (*non_zero_probs.offset((j) as isize)).Init(_probability)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_PermutationCoder {
    values_: Vec<u8>,
}
impl brunsli_PermutationCoder {
    pub unsafe fn brunsli_PermutationCoder() -> Self {
        let mut this = Self {
            values_: Vec::new(),
        };
        this
    }
    pub unsafe fn Init(&mut self, mut values: Vec<u8>) {
        self.values_ = std::mem::take(&mut values);
    }
    pub unsafe fn Clear(&mut self) {
        std::mem::swap(&mut Vec::new(), &mut self.values_);
    }
    pub unsafe fn num_bits(&self) -> i32 {
        let mut num_values: u32 = (self.values_.len() as u32);
        if !((num_values) > (0_u32)) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"lehmer_code.cc".as_ptr(), 51, c"num_bits".as_ptr())
            });
            'loop_: while true {}
        };
        if ((num_values) <= (1_u32)) {
            return 0;
        }
        return ((unsafe { Log2FloorNonZero_74((num_values).wrapping_sub(1_u32)) }) + (1));
    }
    pub unsafe fn Remove(&mut self, mut code: usize, mut value: *mut u8) -> bool {
        if ((code) >= (self.values_.len())) {
            return false;
        }
        (*value) = self.values_[(code)];
        {
            let pos = self
                .values_
                .as_mut_ptr()
                .add((code as i64) as usize)
                .offset_from(self.values_.as_ptr()) as usize;
            self.values_.remove(pos);
            self.values_.as_mut_ptr().add((code as i64) as usize)
        };
        return true;
    }
    pub unsafe fn RemoveValue(
        &mut self,
        mut value: u8,
        mut code: *mut i32,
        mut nbits: *mut i32,
    ) -> bool {
        let mut it: *mut u8 = {
            let mut it = self.values_.as_mut_ptr();
            while it != self.values_.as_mut_ptr().add(self.values_.len()) && *it != value {
                it = it.add(1);
            }
            it
        };
        if it == self.values_.as_mut_ptr().add(self.values_.len()) {
            return false;
        }
        (*code) = (it.offset_from(self.values_.as_mut_ptr()) as i32).clone();
        (*nbits) = (unsafe { self.num_bits() }).clone();
        {
            let pos = it.offset_from(self.values_.as_ptr()) as usize;
            self.values_.remove(pos);
            it
        };
        return true;
    }
}
impl Default for brunsli_PermutationCoder {
    fn default() -> Self {
        unsafe { brunsli_PermutationCoder::brunsli_PermutationCoder() }
    }
}
pub unsafe fn ComputeLehmerCode_112(mut sigma: *const u32, len: usize, mut code: *mut u32) {
    let mut items: Vec<u32> = (0..(len) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        items[(i)] = (i as u32);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        let mut it: *mut u32 = {
            let mut it = items.as_mut_ptr();
            while it != items.as_mut_ptr().add(items.len()) && *it != (*sigma.offset((i) as isize))
            {
                it = it.add(1);
            }
            it
        };
        if !(it != items.as_mut_ptr().add(items.len())) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"lehmer_code.cc".as_ptr(),
                    21,
                    c"ComputeLehmerCode".as_ptr(),
                )
            });
            'loop_: while true {}
        };
        (*code.offset((i) as isize)) = (it.offset_from(items.as_mut_ptr()) as u32);
        {
            let pos = it.offset_from(items.as_ptr()) as usize;
            items.remove(pos);
            it
        };
        i.prefix_inc();
    }
}
pub unsafe fn DecodeLehmerCode_113(
    mut code: *const u32,
    mut len: usize,
    mut sigma: *mut u32,
) -> bool {
    let mut items: Vec<u32> = (0..(len) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        items[(i)] = (i as u32);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        let mut index: u32 = (*code.offset((i) as isize));
        if ((index as usize) >= (items.len())) {
            return false;
        }
        let value: u32 = items[(index as usize)];
        {
            let pos = items
                .as_mut_ptr()
                .add((index as i64) as usize)
                .offset_from(items.as_ptr()) as usize;
            items.remove(pos);
            items.as_mut_ptr().add((index as i64) as usize)
        };
        (*sigma.offset((i) as isize)) = value;
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn BrunsliDumpAndAbort_79(
    mut f: *const libc::c_char,
    mut l: i32,
    mut fn_: *const libc::c_char,
) {
    printf(c"%s:%d (%s)\n".as_ptr() as *const i8, f, l, fn_);
    libc::fflush(libcc2rs::stderr_unsafe());
    std::process::abort();
}
pub unsafe fn AdaptiveMedian_114(mut w: i32, mut n: i32, mut nw: i32) -> i32 {
    let mx: i32 = if ((w) > (n)) { w } else { n };
    let mn: i32 = (((w) + (n)) - (mx));
    if ((nw) > (mx)) {
        return mn;
    } else if ((nw) < (mn)) {
        return mx;
    } else {
        return (((n) + (w)) - (nw));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn PredictWithAdaptiveMedian_115(
    mut coeffs: *const i16,
    mut x: i32,
    mut y: i32,
    mut stride: i32,
) -> i32 {
    let offset1: i32 = -kDCTBlockSize_3;
    let offset2: i32 = -stride;
    let offset3: i32 = ((offset2) + (offset1));
    if ((y) != (0)) {
        if ((x) != (0)) {
            return (unsafe {
                let _w: i32 = ((*coeffs.offset((offset1) as isize)) as i32);
                let _n: i32 = ((*coeffs.offset((offset2) as isize)) as i32);
                let _nw: i32 = ((*coeffs.offset((offset3) as isize)) as i32);
                AdaptiveMedian_114(_w, _n, _nw)
            });
        } else {
            return ((*coeffs.offset((offset2) as isize)) as i32);
        }
    } else {
        return if (x != 0) {
            ((*coeffs.offset((offset1) as isize)) as i32)
        } else {
            0
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub static mut kQFactorBits_116: usize = unsafe { 6_usize };
pub static mut kQFactorLimit_117: usize = unsafe { (((1_u32) << (kQFactorBits_116)) as usize) };
pub unsafe fn FillQuantMatrix_118(mut is_chroma: bool, mut q: u32, mut dst: *mut u8) {
    if !(((q) >= (0_u32)) && ((q as usize) < (kQFactorLimit_117))) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"quant_matrix.cc".as_ptr(), 18, c"FillQuantMatrix".as_ptr())
        });
        'loop_: while true {}
    };
    let in_: *const u8 = kDefaultQuantMatrix_12[(is_chroma) as usize].as_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (kDCTBlockSize_3)) {
        let v: u32 =
            (((((*in_.offset((i) as isize)) as u32).wrapping_mul(q)).wrapping_add(32_u32)) >> (6));
        (*dst.offset((i) as isize)) = (if ((v) < (1_u32)) {
            1_u32
        } else {
            if ((v) > (255_u32)) { 255_u32 } else { v }
        } as u8);
        i.prefix_inc();
    }
}
pub unsafe fn FindBestMatrix_119(
    mut src: *const i32,
    mut is_chroma: bool,
    mut dst: *mut u8,
) -> u32 {
    let mut best_q: u32 = 0_u32;
    let kMaxDiffCost: usize = 33_usize;
    let kWorstLen: usize = (((kDCTBlockSize_3) + (1)) as usize)
        .wrapping_mul((((kMaxDiffCost).wrapping_add(1_usize)) as usize));
    let mut best_len: usize = kWorstLen;
    let mut q: u32 = 0_u32;
    'loop_: while ((q as usize) < (kQFactorLimit_117)) {
        (unsafe { FillQuantMatrix_118(is_chroma, q, dst) });
        let mut last_diff: i32 = 0;
        let mut len: usize = 0_usize;
        let mut k: i32 = 0;
        'loop_: while ((k) < (kDCTBlockSize_3)) {
            let j: i32 = (kJPEGNaturalOrder_13[(k) as usize] as i32);
            let new_diff: i32 =
                ((*src.offset((j) as isize)) - ((*dst.offset((j) as isize)) as i32));
            let mut diff: i32 = ((new_diff) - (last_diff));
            last_diff = new_diff;
            if ((diff) != (0)) {
                len = (len).wrapping_add(1_usize);
                if ((diff) < (0)) {
                    diff = -diff;
                }
                diff -= 1;
                if ((diff) == (0)) {
                    len.postfix_inc();
                } else if ((diff) > (65535)) {
                    len = kWorstLen;
                    break;
                } else {
                    let mut diff_len: u32 =
                        (((unsafe { Log2FloorNonZero_74((diff as u32)) }) + (1)) as u32);
                    if ((diff_len) == (16_u32)) {
                        diff_len.postfix_dec();
                    }
                    len = (len).wrapping_add(
                        ((((2_u32).wrapping_mul(diff_len)).wrapping_add(1_u32)) as usize),
                    );
                }
            }
            k.prefix_inc();
        }
        if ((len) < (best_len)) {
            best_len = len;
            best_q = q;
        }
        q.prefix_inc();
    }
    (unsafe { FillQuantMatrix_118(is_chroma, best_q, dst) });
    return best_q;
}
pub unsafe fn WriteBits_120(mut n_bits: usize, mut bits: u64, mut storage: *mut brunsli_Storage) {
    if true {
    } else {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "WriteBits {:2} {:16x} {:10}\n",
            n_bits,
            bits,
            (*storage).pos,
        );
    }
    if !(((bits) >> (n_bits)) == (0_u64)) {
        (unsafe { BrunsliDumpAndAbort_79(c"ans_encode.cc".as_ptr(), 58, c"WriteBits".as_ptr()) });
        'loop_: while true {}
    };
    if !((n_bits) <= (56_usize)) {
        (unsafe { BrunsliDumpAndAbort_79(c"ans_encode.cc".as_ptr(), 59, c"WriteBits".as_ptr()) });
        'loop_: while true {}
    };
    if !((((((*storage).pos).wrapping_add(n_bits)) >> (3)).wrapping_add(7_usize))
        < ((*storage).length))
    {
        (unsafe { BrunsliDumpAndAbort_79(c"ans_encode.cc".as_ptr(), 61, c"WriteBits".as_ptr()) });
        'loop_: while true {}
    };
    let mut p: *mut u8 = (*storage).data.offset((((*storage).pos) >> (3)) as isize);
    let mut v: u64 = ((*p) as u64);
    v |= ((bits) << (((*storage).pos) & (7_usize)));
    (unsafe { BrunsliUnalignedWrite64_70((p as *mut u8 as *mut ::libc::c_void), v) });
    (*storage).pos = ((*storage).pos).wrapping_add(n_bits);
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_ANSEncSymbolInfo {
    pub freq_: u16,
    pub start_: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_ANSTable {
    pub info_: [brunsli_ANSEncSymbolInfo; 18],
}
impl Default for brunsli_ANSTable {
    fn default() -> Self {
        brunsli_ANSTable {
            info_: [<brunsli_ANSEncSymbolInfo>::default(); 18],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_ANSCoder {
    state_: u32,
}
impl brunsli_ANSCoder {
    pub unsafe fn brunsli_ANSCoder() -> Self {
        let mut this = Self {
            state_: ((19_u32) << (16)),
        };
        this
    }
    pub unsafe fn PutSymbol(&mut self, t: brunsli_ANSEncSymbolInfo, mut nbits: *mut u8) -> u32 {
        let mut bits: u32 = 0_u32;
        (*nbits) = 0_u8;
        if (((self.state_) >> ((32) - (BRUNSLI_ANS_LOG_TAB_SIZE_0))) >= (t.freq_ as u32)) {
            bits = ((self.state_) & (65535_u32));
            self.state_ >>= 16;
            (*nbits) = 16_u8;
        }
        self.state_ = ((((self.state_).wrapping_div((t.freq_ as u32)))
            << (BRUNSLI_ANS_LOG_TAB_SIZE_0))
            .wrapping_add(((self.state_).wrapping_rem((t.freq_ as u32)))))
        .wrapping_add((t.start_ as u32));
        return bits;
    }
    pub unsafe fn GetState(&self) -> u32 {
        return self.state_;
    }
}
impl Default for brunsli_ANSCoder {
    fn default() -> Self {
        unsafe { brunsli_ANSCoder::brunsli_ANSCoder() }
    }
}
pub static mut kMaxNumSymbolsForSmallCode_121: i32 = unsafe { 4 };
pub unsafe fn ANSBuildInfoTable_122(
    mut counts: *const i32,
    mut alphabet_size: i32,
    mut info: *mut brunsli_ANSEncSymbolInfo,
) {
    let mut total: i32 = 0;
    let mut s: i32 = 0;
    'loop_: while ((s) < (alphabet_size)) {
        let freq: u32 = ((*counts.offset((s) as isize)) as u32);
        (*info.offset((s) as isize)).freq_ = ((*counts.offset((s) as isize)) as u16);
        (*info.offset((s) as isize)).start_ = (total as u16);
        total = ((total as u32).wrapping_add(freq)) as i32;
        s.prefix_inc();
    }
}
pub unsafe fn BuildAndStoreANSEncodingData_123(
    mut histogram: *const i32,
    mut table: *mut brunsli_ANSTable,
    mut storage: *mut brunsli_Storage,
) {
    let mut num_symbols: i32 = 0_i32;
    let mut symbols: [i32; 4] = [0, 0_i32, 0_i32, 0_i32];
    let mut counts: Vec<i32> = core::slice::from_raw_parts(
        histogram,
        (histogram.offset((18) as isize)).offset_from(histogram) as usize,
    )
    .to_vec();
    let mut omit_pos: i32 = 0;
    (unsafe {
        NormalizeCounts_124(
            (&mut counts[(0_usize)] as *mut i32),
            (&mut omit_pos as *mut i32),
            18,
            BRUNSLI_ANS_LOG_TAB_SIZE_0,
            (&mut num_symbols as *mut i32),
            symbols.as_mut_ptr(),
        )
    });
    (unsafe {
        ANSBuildInfoTable_122(
            (&mut counts[(0_usize)] as *mut i32).cast_const(),
            18,
            (*table).info_.as_mut_ptr(),
        )
    });
    (unsafe {
        EncodeCounts_125(
            (&mut counts[(0_usize)] as *mut i32).cast_const(),
            omit_pos,
            num_symbols,
            (symbols.as_mut_ptr()).cast_const(),
            storage,
        )
    });
}
pub static mut kLog2Table_126: [f32; 256] = unsafe {
    [
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
    ]
};
pub unsafe fn FastLog2_127(mut v: i32) -> f64 {
    if ((v)
        < (((::std::mem::size_of::<[f32; 256]>() as usize)
            .wrapping_div((::std::mem::size_of::<f32>() as usize))) as i32))
    {
        return (kLog2Table_126[(v) as usize] as f64);
    }
    return (v as f64).log2();
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_HistogramPair {
    pub idx1: usize,
    pub idx2: usize,
    pub cost_combo: f64,
    pub cost_diff: f64,
}
pub unsafe fn lt(p1: *const brunsli_HistogramPair, p2: *const brunsli_HistogramPair) -> bool {
    if (((*p1).cost_diff) != ((*p2).cost_diff)) {
        return (((*p1).cost_diff) > ((*p2).cost_diff));
    }
    if !(((*p1).idx1) < ((*p1).idx2)) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 35, c"operator<".as_ptr())
        });
        'loop_: while true {}
    };
    if !(((*p2).idx1) < ((*p2).idx2)) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 36, c"operator<".as_ptr())
        });
        'loop_: while true {}
    };
    return ((((*p1).idx2).wrapping_sub((*p1).idx1)) > (((*p2).idx2).wrapping_sub((*p2).idx1)));
}
impl Ord for brunsli_HistogramPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if lt(self, other) {
                std::cmp::Ordering::Less
            } else if lt(other, self) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl PartialOrd for brunsli_HistogramPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for brunsli_HistogramPair {
    fn eq(&self, other: &Self) -> bool {
        unsafe { !(lt(self, other)) && !(lt(other, self)) }
    }
}
impl Eq for brunsli_HistogramPair {}
pub unsafe fn ClusterCostDiff_128(mut size_a: i32, mut size_b: i32) -> f64 {
    let mut size_c: i32 = ((size_a) + (size_b));
    return ((((size_a as f64) * (unsafe { FastLog2_127(size_a) }))
        + ((size_b as f64) * (unsafe { FastLog2_127(size_b) })))
        - ((size_c as f64) * (unsafe { FastLog2_127(size_c) })));
}
pub unsafe fn PopulationCost_129(h: *const brunsli_internal_enc_Histogram) -> f64 {
    return (unsafe {
        let _data: *const i32 = (&(*h).data_[(0) as usize] as *const i32);
        let _total_count: i32 = (*h).total_count_;
        PopulationCost_130(_data, _total_count)
    });
}
pub unsafe fn CompareAndPushToQueue_131(
    mut out: *const brunsli_internal_enc_Histogram,
    mut cluster_size: *const i32,
    mut idx1: i32,
    mut idx2: i32,
    mut pairs: *mut Vec<brunsli_HistogramPair>,
) {
    if ((idx1) == (idx2)) {
        return;
    };
    if ((idx2) < (idx1)) {
        std::mem::swap(&mut idx1, &mut idx2);
    }
    let mut store_pair: bool = false;
    let mut p: brunsli_HistogramPair = <brunsli_HistogramPair>::default();
    p.idx1 = (idx1 as usize);
    p.idx2 = (idx2 as usize);
    p.cost_diff = ((5.0E-1)
        * (unsafe {
            let _size_a: i32 = (*cluster_size.offset((idx1) as isize));
            let _size_b: i32 = (*cluster_size.offset((idx2) as isize));
            ClusterCostDiff_128(_size_a, _size_b)
        }));
    p.cost_diff -= (*out.offset((idx1) as isize)).bit_cost_;
    p.cost_diff -= (*out.offset((idx2) as isize)).bit_cost_;
    if (((*out.offset((idx1) as isize)).total_count_) == (0)) {
        p.cost_combo = (*out.offset((idx2) as isize)).bit_cost_;
        store_pair = true;
    } else if (((*out.offset((idx2) as isize)).total_count_) == (0)) {
        p.cost_combo = (*out.offset((idx1) as isize)).bit_cost_;
        store_pair = true;
    } else {
        let mut threshold: f64 = if (*(pairs).cast_const()).is_empty() {
            1.0E+99
        } else {
            {
                let mut __tmp_0 = 0.0E+0;
                (*if *&mut __tmp_0 >= *&(&mut (*pairs))[(0_usize)].cost_diff {
                    (&mut __tmp_0) as *const _
                } else {
                    (&(&mut (*pairs))[(0_usize)].cost_diff) as *const _
                })
            }
        };
        let mut combo: brunsli_internal_enc_Histogram = (*out.offset((idx1) as isize)).clone();
        (unsafe {
            let _other: *const brunsli_internal_enc_Histogram =
                &(*out.offset((idx2) as isize)) as *const brunsli_internal_enc_Histogram;
            combo.AddHistogram(_other)
        });
        let mut cost_combo: f64 =
            (unsafe { PopulationCost_129(&combo as *const brunsli_internal_enc_Histogram) });
        if ((cost_combo) < ((threshold) - (p.cost_diff))) {
            p.cost_combo = cost_combo;
            store_pair = true;
        }
    }
    if store_pair {
        p.cost_diff += p.cost_combo;
        if (!(*(pairs).cast_const()).is_empty()) && (lt(((*pairs).first_mut().unwrap()), &mut p)) {
            {
                let a0_clone = (*((*pairs).first_mut().unwrap())).clone();
                (*pairs).push(a0_clone)
            };
            (*((*pairs).first_mut().unwrap())) = (p).clone();
        } else {
            {
                let a0_clone = p.clone();
                (*pairs).push(a0_clone)
            };
        }
    }
}
pub unsafe fn HistogramCombine_132(
    mut out: *mut brunsli_internal_enc_Histogram,
    mut cluster_size: *mut i32,
    mut symbols: *mut u32,
    mut symbols_size: usize,
    mut max_clusters: usize,
) -> usize {
    let mut cost_diff_threshold: f64 = 0.0E+0;
    let mut min_cluster_size: usize = 1_usize;
    let mut clusters: Vec<u64> = core::slice::from_raw_parts(
        symbols,
        (symbols.offset((symbols_size) as isize)).offset_from(symbols) as usize,
    )
    .iter()
    .map(|x| u64::try_from(x.clone()).ok().unwrap())
    .collect();
    {
        let len = clusters
            .as_mut_ptr()
            .add(clusters.len())
            .offset_from(clusters.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(clusters.as_mut_ptr(), len).sort()
    };
    {
        let __a0 = (if clusters.as_mut_ptr() == clusters.as_mut_ptr().add(clusters.len()) {
            clusters.as_mut_ptr()
        } else {
            let mut write = clusters.as_mut_ptr();
            let mut prev = clusters.as_mut_ptr();
            let mut it = clusters.as_mut_ptr();
            it = it.add(1);

            while it != clusters.as_mut_ptr().add(clusters.len()) {
                if *prev != *it {
                    write = write.add(1);
                    *write = (*it).clone();
                    prev = write;
                }
                it = it.add(1);
            }

            write = write.add(1);
            write
        }
        .offset_from(clusters.as_mut_ptr()) as usize) as usize;
        clusters.resize_with(__a0, || <u64>::default())
    };
    let mut pairs: Vec<brunsli_HistogramPair> = Vec::new();
    if ((clusters.len()).wrapping_mul(((clusters.len()).wrapping_add(1_usize))))
        .wrapping_div(2_usize) as usize
        > pairs.capacity() as usize
    {
        let len_0 = pairs.len();
        pairs.reserve_exact(
            ((clusters.len()).wrapping_mul(((clusters.len()).wrapping_add(1_usize))))
                .wrapping_div(2_usize) as usize
                - len_0 as usize,
        );
    };
    let mut idx1: usize = 0_usize;
    'loop_: while ((idx1) < (clusters.len())) {
        let mut idx2: usize = (idx1).wrapping_add(1_usize);
        'loop_: while ((idx2) < (clusters.len())) {
            (unsafe {
                let _cluster_size: *const i32 = (cluster_size).cast_const();
                let _idx1: i32 = (clusters[(idx1)] as i32);
                let _idx2: i32 = (clusters[(idx2)] as i32);
                CompareAndPushToQueue_131(
                    (out).cast_const(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (&mut pairs as *mut Vec<brunsli_HistogramPair>),
                )
            });
            idx2.prefix_inc();
        }
        idx1.prefix_inc();
    }
    'loop_: while ((clusters.len()) > (min_cluster_size)) {
        if ((pairs[(0_usize)].cost_diff) >= (cost_diff_threshold)) {
            cost_diff_threshold = 1.0E+99;
            min_cluster_size = max_clusters;
            continue 'loop_;
        }
        let mut best_idx1: usize = pairs[(0_usize)].idx1;
        let mut best_idx2: usize = pairs[(0_usize)].idx2;
        (unsafe {
            let _other: *const brunsli_internal_enc_Histogram =
                &(*out.offset((best_idx2) as isize)) as *const brunsli_internal_enc_Histogram;
            (*out.offset((best_idx1) as isize)).AddHistogram(_other)
        });
        (*out.offset((best_idx1) as isize)).bit_cost_ = pairs[(0_usize)].cost_combo;
        (*cluster_size.offset((best_idx1) as isize)) +=
            (*cluster_size.offset((best_idx2) as isize));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (symbols_size)) {
            if (((*symbols.offset((i) as isize)) as usize) == (best_idx2)) {
                (*symbols.offset((i) as isize)) = (best_idx1 as u32);
            }
            i.prefix_inc();
        }
        let mut cluster: *mut u64 = clusters.as_mut_ptr();
        'loop_: while cluster != clusters.as_mut_ptr().add(clusters.len()) {
            if (((*cluster) as usize) >= (best_idx2)) {
                {
                    let pos = cluster.offset_from(clusters.as_ptr()) as usize;
                    clusters.remove(pos);
                    cluster
                };
                break;
            }
            cluster.prefix_inc();
        }
        let mut copy_to: *mut brunsli_HistogramPair = pairs.as_mut_ptr();
        'loop_: for p in 0..(pairs.len()) {
            let mut p = pairs.as_ptr().add(p);
            if (((((*p).idx1) == (best_idx1)) || (((*p).idx2) == (best_idx1)))
                || (((*p).idx1) == (best_idx2)))
                || (((*p).idx2) == (best_idx2))
            {
                continue 'loop_;
            }
            if lt(((pairs).first_mut().unwrap()), p) {
                let mut front: brunsli_HistogramPair = (*((pairs).first_mut().unwrap())).clone();
                (*((pairs).first_mut().unwrap())) = (*p);
                (*copy_to) = front;
            } else {
                (*copy_to) = (*p);
            }
            copy_to.prefix_inc();
        }
        {
            let __a0 = (copy_to.offset_from(pairs.as_mut_ptr()) as usize) as usize;
            pairs.resize_with(__a0, || <brunsli_HistogramPair>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (clusters.len())) {
            (unsafe {
                let _cluster_size: *const i32 = (cluster_size).cast_const();
                let _idx1: i32 = (best_idx1 as i32);
                let _idx2: i32 = (clusters[(i)] as i32);
                CompareAndPushToQueue_131(
                    (out).cast_const(),
                    _cluster_size,
                    _idx1,
                    _idx2,
                    (&mut pairs as *mut Vec<brunsli_HistogramPair>),
                )
            });
            i.prefix_inc();
        }
    }
    return clusters.len();
}
pub unsafe fn HistogramBitCostDistance_133(
    histogram: *const brunsli_internal_enc_Histogram,
    candidate: *const brunsli_internal_enc_Histogram,
) -> f64 {
    if (((*histogram).total_count_) == (0)) {
        return 0.0E+0;
    }
    let mut tmp: brunsli_internal_enc_Histogram = (*histogram).clone();
    (unsafe {
        let _other: *const brunsli_internal_enc_Histogram = candidate;
        tmp.AddHistogram(_other)
    });
    return ((unsafe { PopulationCost_129(&tmp as *const brunsli_internal_enc_Histogram) })
        - ((*candidate).bit_cost_));
}
pub unsafe fn HistogramRemap_134(
    mut in_: *const brunsli_internal_enc_Histogram,
    mut in_size: usize,
    mut out: *mut brunsli_internal_enc_Histogram,
    mut symbols: *mut u32,
) {
    let mut all_symbols: Vec<i32> = core::slice::from_raw_parts(
        symbols,
        (symbols.offset((in_size) as isize)).offset_from(symbols) as usize,
    )
    .iter()
    .map(|x| i32::try_from(x.clone()).ok().unwrap())
    .collect();
    {
        let len = all_symbols
            .as_mut_ptr()
            .add(all_symbols.len())
            .offset_from(all_symbols.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(all_symbols.as_mut_ptr(), len).sort()
    };
    {
        let __a0 = (if all_symbols.as_mut_ptr() == all_symbols.as_mut_ptr().add(all_symbols.len()) {
            all_symbols.as_mut_ptr()
        } else {
            let mut write = all_symbols.as_mut_ptr();
            let mut prev = all_symbols.as_mut_ptr();
            let mut it = all_symbols.as_mut_ptr();
            it = it.add(1);

            while it != all_symbols.as_mut_ptr().add(all_symbols.len()) {
                if *prev != *it {
                    write = write.add(1);
                    *write = (*it).clone();
                    prev = write;
                }
                it = it.add(1);
            }

            write = write.add(1);
            write
        }
        .offset_from(all_symbols.as_mut_ptr()) as usize) as usize;
        all_symbols.resize_with(__a0, || <i32>::default())
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (in_size)) {
        let mut best_out: i32 = (if ((i) == (0_usize)) {
            (*symbols.offset((0) as isize))
        } else {
            (*symbols.offset(((i).wrapping_sub(1_usize)) as isize))
        } as i32);
        let mut best_bits: f64 = (unsafe {
            let _histogram: *const brunsli_internal_enc_Histogram =
                &(*in_.offset((i) as isize)) as *const brunsli_internal_enc_Histogram;
            let _candidate: *const brunsli_internal_enc_Histogram =
                &(*out.offset((best_out) as isize)) as *const brunsli_internal_enc_Histogram;
            HistogramBitCostDistance_133(_histogram, _candidate)
        });
        'loop_: for k in 0..(all_symbols.len()) {
            let mut k = all_symbols[k].clone();
            let cur_bits: f64 = (unsafe {
                let _histogram: *const brunsli_internal_enc_Histogram =
                    &(*in_.offset((i) as isize)) as *const brunsli_internal_enc_Histogram;
                let _candidate: *const brunsli_internal_enc_Histogram =
                    &(*out.offset((k) as isize)) as *const brunsli_internal_enc_Histogram;
                HistogramBitCostDistance_133(_histogram, _candidate)
            });
            if ((cur_bits) < (best_bits)) {
                best_bits = cur_bits;
                best_out = k;
            }
        }
        (*symbols.offset((i) as isize)) = (best_out as u32);
        i.prefix_inc();
    }
    'loop_: for k in 0..(all_symbols.len()) {
        let mut k = all_symbols[k].clone();
        (unsafe { (*out.offset((k) as isize)).Clear() });
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (in_size)) {
        (unsafe {
            let _other: *const brunsli_internal_enc_Histogram =
                &(*in_.offset((i) as isize)) as *const brunsli_internal_enc_Histogram;
            (*out.offset((*symbols.offset((i) as isize)) as isize)).AddHistogram(_other)
        });
        i.prefix_inc();
    }
}
pub unsafe fn HistogramReindex_135(
    mut out: *mut Vec<brunsli_internal_enc_Histogram>,
    mut symbols: *mut Vec<u32>,
) {
    let mut tmp: Vec<brunsli_internal_enc_Histogram> = (*out).clone();
    let mut new_index: BTreeMap<i32, Box<i32>> = BTreeMap::new();
    let mut next_index: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*(symbols).cast_const()).len())) {
        if UnsafeMapIterator::find_key(
            &new_index as *const BTreeMap<i32, Box<i32>>,
            &((&mut (*symbols))[(i)] as i32),
        ) == UnsafeMapIterator::end(&new_index as *const BTreeMap<i32, Box<i32>>)
        {
            (*new_index
                .entry(((&mut (*symbols))[(i)] as i32))
                .or_default()
                .as_mut()) = next_index;
            (&mut (*out))[(next_index as usize)] = tmp[((&mut (*symbols))[(i)] as usize)];
            next_index.prefix_inc();
        }
        i.prefix_inc();
    }
    {
        let __a0 = (next_index as usize) as usize;
        (*out).resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*(symbols).cast_const()).len())) {
        (&mut (*symbols))[(i)] = ((*new_index
            .entry(((&mut (*symbols))[(i)] as i32))
            .or_default()
            .as_mut()) as u32);
        i.prefix_inc();
    }
}
pub unsafe fn ClusterHistograms_136(
    in_: *const Vec<brunsli_internal_enc_Histogram>,
    mut num_contexts: usize,
    mut num_blocks: usize,
    block_group_offsets: Vec<u64>,
    mut max_histograms: usize,
    mut out: *mut Vec<brunsli_internal_enc_Histogram>,
    mut histogram_symbols: *mut Vec<u32>,
) {
    let in_size: usize = (num_contexts).wrapping_mul(num_blocks);
    let mut cluster_size: Vec<i32> = vec![1; in_size as usize];
    {
        let __a0 = in_size as usize;
        (*out).resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
    };
    {
        let __a0 = in_size as usize;
        (*histogram_symbols).resize_with(__a0, || <u32>::default())
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (in_size)) {
        (&mut (*out))[(i)] = (&(*in_))[(i)];
        (&mut (*out))[(i)].bit_cost_ = (unsafe {
            PopulationCost_129(&(&(*in_))[(i)] as *const brunsli_internal_enc_Histogram)
        });
        (&mut (*histogram_symbols))[(i)] = (i as u32);
        i.prefix_inc();
    }
    if ((num_contexts) > (1_usize)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (num_blocks)) {
            (unsafe {
                let _symbols: *mut u32 = (&mut (&mut (*histogram_symbols))
                    [((i).wrapping_mul(num_contexts))]
                    as *mut u32);
                let _symbols_size: usize = num_contexts;
                HistogramCombine_132(
                    (&mut (&mut (*out))[(0_usize)] as *mut brunsli_internal_enc_Histogram),
                    (&mut cluster_size[(0_usize)] as *mut i32),
                    _symbols,
                    _symbols_size,
                    max_histograms,
                )
            });
            i.prefix_inc();
        }
    }
    static mut kMinClustersForHistogramRemap_137: usize = unsafe { 24_usize };;
    let mut num_clusters: usize = 0_usize;
    if ((block_group_offsets.len()) > (1_usize)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (block_group_offsets.len())) {
            let mut offset: usize =
                ((block_group_offsets[(i)]).wrapping_mul((num_contexts as u64)) as usize);
            let mut next_offset: usize =
                (if (((i).wrapping_add(1_usize)) < (block_group_offsets.len())) {
                    (block_group_offsets[((i).wrapping_add(1_usize))])
                        .wrapping_mul((num_contexts as u64))
                } else {
                    (in_size as u64)
                } as usize);
            let mut length: usize = (next_offset).wrapping_sub(offset);
            let mut nclusters: usize = (unsafe {
                HistogramCombine_132(
                    (&mut (&mut (*out))[(0_usize)] as *mut brunsli_internal_enc_Histogram),
                    (&mut cluster_size[(0_usize)] as *mut i32),
                    (&mut (&mut (*histogram_symbols))[(offset)] as *mut u32),
                    length,
                    max_histograms,
                )
            });
            if ((nclusters) >= (2_usize)) && ((nclusters) < (kMinClustersForHistogramRemap_137)) {
                (unsafe {
                    let _in: *const brunsli_internal_enc_Histogram =
                        (&(&(*in_))[(offset)] as *const brunsli_internal_enc_Histogram);
                    let _symbols: *mut u32 =
                        (&mut (&mut (*histogram_symbols))[(offset)] as *mut u32);
                    HistogramRemap_134(
                        _in,
                        length,
                        (&mut (&mut (*out))[(0_usize)] as *mut brunsli_internal_enc_Histogram),
                        _symbols,
                    )
                });
            }
            num_clusters = (num_clusters).wrapping_add(nclusters);
            i.prefix_inc();
        }
    }
    if ((block_group_offsets.len()) <= (1_usize)) || ((num_clusters) > (max_histograms)) {
        num_clusters = (unsafe {
            HistogramCombine_132(
                (&mut (&mut (*out))[(0_usize)] as *mut brunsli_internal_enc_Histogram),
                (&mut cluster_size[(0_usize)] as *mut i32),
                (&mut (&mut (*histogram_symbols))[(0_usize)] as *mut u32),
                in_size,
                max_histograms,
            )
        });
        if ((num_clusters) >= (2_usize)) && ((num_clusters) < (kMinClustersForHistogramRemap_137)) {
            (unsafe {
                HistogramRemap_134(
                    (&(&(*in_))[(0_usize)] as *const brunsli_internal_enc_Histogram),
                    in_size,
                    (&mut (&mut (*out))[(0_usize)] as *mut brunsli_internal_enc_Histogram),
                    (&mut (&mut (*histogram_symbols))[(0_usize)] as *mut u32),
                )
            });
        }
    }
    (unsafe { HistogramReindex_135(out, histogram_symbols) });
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_JpegReadMode {
    #[default]
    JPEG_READ_HEADER = 0,
    JPEG_READ_TABLES = 1,
    JPEG_READ_ALL = 2,
}
impl From<i32> for brunsli_JpegReadMode {
    fn from(n: i32) -> brunsli_JpegReadMode {
        match n {
            0 => brunsli_JpegReadMode::JPEG_READ_HEADER,
            1 => brunsli_JpegReadMode::JPEG_READ_TABLES,
            2 => brunsli_JpegReadMode::JPEG_READ_ALL,
            _ => panic!("invalid brunsli_JpegReadMode value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_JpegReadMode);
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_internal_enc_ComponentMeta {
    pub context_offset: usize,
    pub approx_total_nonzeros: usize,
    pub h_samp: i32,
    pub v_samp: i32,
    pub context_bits: i32,
    pub ac_stride: i32,
    pub dc_stride: i32,
    pub b_stride: i32,
    pub width_in_blocks: i32,
    pub height_in_blocks: i32,
    pub ac_coeffs: *const i16,
    pub dc_prediction_errors: *mut i16,
    pub block_state: *mut u8,
    pub num_zeros: Vec<i32>,
    pub quant: Vec<i32>,
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
            ac_coeffs: std::ptr::null(),
            dc_prediction_errors: std::ptr::null_mut(),
            block_state: std::ptr::null_mut(),
            num_zeros: std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            quant: std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_internal_enc_Histogram {
    pub data_: [i32; 18],
    pub total_count_: i32,
    pub bit_cost_: f64,
}
impl brunsli_internal_enc_Histogram {
    pub unsafe fn brunsli_internal_enc_Histogram() -> Self {
        let mut this = Self {
            data_: [0_i32; 18],
            total_count_: 0_i32,
            bit_cost_: 0.0_f64,
        };
        (unsafe { this.Clear() });
        this
    }
}
impl Default for brunsli_internal_enc_Histogram {
    fn default() -> Self {
        unsafe { brunsli_internal_enc_Histogram::brunsli_internal_enc_Histogram() }
    }
}
static mut kMaxNumberOfHistograms_138: usize = unsafe { 256_usize };
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_enc_EntropyCodes {
    clustered_: Vec<brunsli_internal_enc_Histogram>,
    context_map_: Vec<u32>,
    ans_tables_: Vec<brunsli_ANSTable>,
}
impl brunsli_internal_enc_EntropyCodes {
    pub unsafe fn brunsli_internal_enc_EntropyCodes(
        histograms: *const Vec<brunsli_internal_enc_Histogram>,
        mut num_bands: usize,
        offsets: *const Vec<u64>,
    ) -> Self {
        let mut this = Self {
            clustered_: Vec::new(),
            context_map_: Vec::new(),
            ans_tables_: Vec::new(),
        };
        (unsafe {
            let _in: *const Vec<brunsli_internal_enc_Histogram> = histograms;
            let _num_contexts: usize = kNumAvrgContexts_83;
            let _num_blocks: usize = num_bands;
            let _block_group_offsets: Vec<u64> = (*offsets).clone();
            let _max_histograms: usize = kMaxNumberOfHistograms_138;
            let _out: *mut Vec<brunsli_internal_enc_Histogram> =
                (&mut this.clustered_ as *mut Vec<brunsli_internal_enc_Histogram>);
            let _histogram_symbols: *mut Vec<u32> = (&mut this.context_map_ as *mut Vec<u32>);
            ClusterHistograms_136(
                _in,
                _num_contexts,
                _num_blocks,
                _block_group_offsets,
                _max_histograms,
                _out,
                _histogram_symbols,
            )
        });
        this
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_internal_enc_EntropySource {
    num_bands_: usize,
    histograms_: Vec<brunsli_internal_enc_Histogram>,
}
impl brunsli_internal_enc_EntropySource {
    pub unsafe fn brunsli_internal_enc_EntropySource() -> Self {
        let mut this = Self {
            num_bands_: 0_usize,
            histograms_: Vec::new(),
        };
        this
    }
}
impl Default for brunsli_internal_enc_EntropySource {
    fn default() -> Self {
        unsafe { brunsli_internal_enc_EntropySource::brunsli_internal_enc_EntropySource() }
    }
}
static mut kSlackForOneBlock_139: usize = unsafe { 1024_usize };
#[repr(C)]
#[derive(Copy, Clone)]
struct brunsli_internal_enc_DataStream_CodeWord {
    pub context: u32,
    pub value: u16,
    pub code: u8,
    pub nbits: u8,
}
impl brunsli_internal_enc_DataStream_CodeWord {
    pub unsafe fn brunsli_internal_enc_DataStream_CodeWord() -> Self {
        let mut this = Self {
            context: 0_u32,
            value: 0_u16,
            code: 0_u8,
            nbits: 0_u8,
        };
        this
    }
}
impl Default for brunsli_internal_enc_DataStream_CodeWord {
    fn default() -> Self {
        unsafe {
            brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord()
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_internal_enc_DataStream {
    pos_: i32,
    bw_pos_: i32,
    ac_pos0_: i32,
    ac_pos1_: i32,
    low_: u32,
    high_: u32,
    bw_val_: u32,
    bw_bitpos_: i32,
    code_words_: Vec<brunsli_internal_enc_DataStream_CodeWord>,
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn brunsli_internal_enc_DataStream() -> Self {
        let mut this = Self {
            pos_: 3,
            bw_pos_: 0,
            ac_pos0_: 1,
            ac_pos1_: 2,
            low_: 0_u32,
            high_: (!0 as u32),
            bw_val_: 0_u32,
            bw_bitpos_: 0,
            code_words_: Vec::new(),
        };
        this
    }
}
impl Default for brunsli_internal_enc_DataStream {
    fn default() -> Self {
        unsafe { brunsli_internal_enc_DataStream::brunsli_internal_enc_DataStream() }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_enc_State {
    pub entropy_source: brunsli_internal_enc_EntropySource,
    pub entropy_codes: *mut brunsli_internal_enc_EntropyCodes,
    pub data_stream_dc: brunsli_internal_enc_DataStream,
    pub data_stream_ac: brunsli_internal_enc_DataStream,
    pub meta: Vec<brunsli_internal_enc_ComponentMeta>,
    pub num_contexts: usize,
    pub use_legacy_context_model: bool,
}
pub static mut kNumDirectCodes_140: i32 = unsafe { 8 };
pub static mut kBrotliQuality_141: i32 = unsafe { 6 };
pub static mut kBrotliWindowBits_142: i32 = unsafe { 18 };
pub unsafe fn EstimateAuxDataSize_143(jpg: *const brunsli_JPEGData) -> usize {
    let mut size: usize = (((((*jpg).marker_order.len())
        .wrapping_add((272_usize).wrapping_mul((*jpg).huffman_code.len())))
    .wrapping_add((7_usize).wrapping_mul((*jpg).scan_info.len())))
    .wrapping_add(16_usize));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).scan_info.len())) {
        size = ((size as u64).wrapping_add(
            ((7_usize).wrapping_mul((&(*jpg)).scan_info[(i)].reset_points.len()) as u64),
        )) as usize;
        size = ((size as u64).wrapping_add(
            ((7_usize).wrapping_mul((&(*jpg)).scan_info[(i)].extra_zero_runs.len()) as u64),
        )) as usize;
        i.prefix_inc();
    }
    let mut nsize: usize = if (*jpg).has_zero_padding_bit {
        (*jpg).padding_bits.len()
    } else {
        0_usize
    };
    size = (size).wrapping_add((((nsize).wrapping_add(43_usize)) >> (3)));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).inter_marker_data.len())) {
        size = ((size as u64)
            .wrapping_add(((5_usize).wrapping_add((&(*jpg)).inter_marker_data[(i)].len()) as u64)))
            as usize;
        i.prefix_inc();
    }
    return size;
}
pub unsafe fn GetMaximumBrunsliEncodedSize_144(jpg: *const brunsli_JPEGData) -> usize {
    let mut hdr_size: usize = (((1) << (20)) as usize);
    hdr_size = (hdr_size).wrapping_add((unsafe { EstimateAuxDataSize_143(jpg) }));
    'loop_: for data in 0..((*jpg).app_data.len()) {
        let mut data = (*jpg).app_data.as_ptr().add(data);
        hdr_size = ((hdr_size as u64).wrapping_add(((*data).len() as u64))) as usize;
    }
    'loop_: for data in 0..((*jpg).com_data.len()) {
        let mut data = (*jpg).com_data.as_ptr().add(data);
        hdr_size = ((hdr_size as u64).wrapping_add(((*data).len() as u64))) as usize;
    }
    hdr_size = ((hdr_size as u64).wrapping_add(((*jpg).tail_data.len() as u64))) as usize;
    let mut num_pixels: usize =
        ((((*jpg).width) * ((*jpg).height)) as usize).wrapping_mul((*jpg).components.len());
    return (((num_pixels as f64) * (1.2E+0)) as usize).wrapping_add(hdr_size);
}
pub unsafe fn Base128Size_145(mut val: usize) -> usize {
    let mut size: usize = 1_usize;
    'loop_: while ((val) >= (128_usize)) {
        size.prefix_inc();
        val >>= 7;
    }
    return size;
}
pub unsafe fn EncodeBase128_146(mut val: usize, mut data: *mut u8) -> usize {
    let mut len: usize = 0_usize;
    let mut __do_while = true;
    'loop_: while __do_while || ((val) > (0_usize)) {
        __do_while = false;
        (*data.offset((len.postfix_inc()) as isize)) = ((((val) & (127_usize))
            | ((if ((val) >= (128_usize)) { 128 } else { 0 }) as usize))
            as u8);
        val >>= 7;
    }
    return len;
}
pub unsafe fn EncodeBase128Fix_147(mut val: usize, mut len: usize, mut data: *mut u8) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        (*(data.postfix_inc())) = ((((val) & (127_usize))
            | ((if (((i).wrapping_add(1_usize)) < (len)) {
                128
            } else {
                0
            }) as usize)) as u8);
        val >>= 7;
        i.prefix_inc();
    }
}
pub unsafe fn TransformApp0Marker_148(s: *const Vec<u8>, mut out: *mut Vec<u8>) -> bool {
    if (((*s).len()) != (17_usize)) {
        return false;
    }
    if (({
        let sa = core::slice::from_raw_parts(
            ((*s).as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
            9_usize as usize,
        );
        let sb = core::slice::from_raw_parts(
            (AppData_0xe0_62.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
            9_usize as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) != (0))
    {
        return false;
    }
    if ((((((&(*s))[(9_usize)] as i32) == (1)) || (((&(*s))[(9_usize)] as i32) == (2)))
        && (((&(*s))[(10_usize)] as i32) < (4)))
        && (((&(*s))[(15_usize)] as i32) == (0)))
        && (((&(*s))[(16_usize)] as i32) == (0))
    {
        let x_dens_hi: u8 = (&(*s))[(11_usize)];
        let x_dens_lo: u8 = (&(*s))[(12_usize)];
        let mut x_dens: i32 = (((x_dens_hi as i32) << (8)) + (x_dens_lo as i32));
        let y_dens_hi: u8 = (&(*s))[(13_usize)];
        let y_dens_lo: u8 = (&(*s))[(14_usize)];
        let mut y_dens: i32 = (((y_dens_hi as i32) << (8)) + (y_dens_lo as i32));
        let mut density_ix: i32 = -1_i32;
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (kMaxApp0Densities_45)) {
            if ((x_dens) == (kApp0Densities_46[(k)] as i32)) && ((y_dens) == (x_dens)) {
                density_ix = (k as i32);
            }
            k.prefix_inc();
        }
        if ((density_ix) >= (0)) {
            let mut app0_status: u8 = ((((((&(*s))[(9_usize)] as i32) - (1))
                | (((&(*s))[(10_usize)] as i32) << (1)))
                | ((density_ix) << (3))) as u8);
            (*out) = (0..(1_usize) as usize)
                .map(|_| <u8>::default())
                .collect::<Vec<_>>();
            (*&mut (&mut (*out))[0_usize as usize]) = app0_status;
            return true;
        }
    }
    return false;
}
pub unsafe fn TransformApp2Marker_149(s: *const Vec<u8>, mut out: *mut Vec<u8>) -> bool {
    if ((((*s).len()) == (3161_usize))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((*s).as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                84_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xe2_63.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                84_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0)))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((*s).as_ptr().offset((85) as isize) as *const u8 as *const ::libc::c_void)
                    as *const u8,
                (((3161) - (85)) as usize) as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xe2_63.as_ptr().offset((85) as isize) as *const u8
                    as *const ::libc::c_void) as *const u8,
                (((3161) - (85)) as usize) as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[(0_usize)] = 128_u8;
        code[(1_usize)] = (&(*s))[(84_usize)];
        (*out) = code.clone();
        return true;
    }
    return false;
}
pub unsafe fn TransformApp12Marker_150(s: *const Vec<u8>, mut out: *mut Vec<u8>) -> bool {
    if ((((*s).len()) == (18_usize))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((*s).as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                15_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xec_64.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                15_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0)))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((*s).as_ptr().offset((16) as isize) as *const u8 as *const ::libc::c_void)
                    as *const u8,
                (((18) - (16)) as usize) as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xec_64.as_ptr().offset((16) as isize) as *const u8
                    as *const ::libc::c_void) as *const u8,
                (((18) - (16)) as usize) as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[(0_usize)] = 129_u8;
        code[(1_usize)] = (&(*s))[(15_usize)];
        (*out) = code.clone();
        return true;
    }
    return false;
}
pub unsafe fn TransformApp14Marker_151(s: *const Vec<u8>, mut out: *mut Vec<u8>) -> bool {
    if ((((*s).len()) == (15_usize))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((&(&(*s))[(0_usize)] as *const u8) as *const u8 as *const ::libc::c_void)
                    as *const u8,
                10_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xee_65.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                10_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0)))
        && (!({
            let sa = core::slice::from_raw_parts(
                ((&(&(*s))[(11_usize)] as *const u8) as *const u8 as *const ::libc::c_void)
                    as *const u8,
                (((15) - (11)) as usize) as usize,
            );
            let sb = core::slice::from_raw_parts(
                (AppData_0xee_65.as_ptr().offset((11) as isize) as *const u8
                    as *const ::libc::c_void) as *const u8,
                (((15) - (11)) as usize) as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        } != 0))
    {
        let mut code: Vec<u8> = (0..(2_usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>();
        code[(0_usize)] = 130_u8;
        code[(1_usize)] = (&(*s))[(10_usize)];
        (*out) = code.clone();
        return true;
    }
    return false;
}
pub unsafe fn TransformAppMarker_152(
    s: *const Vec<u8>,
    mut transformed_marker_count: *mut usize,
) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    if (unsafe {
        let _s: *const Vec<u8> = s;
        let _out: *mut Vec<u8> = (&mut out as *mut Vec<u8>);
        TransformApp0Marker_148(_s, _out)
    }) {
        (*transformed_marker_count).postfix_inc();
        return out;
    }
    if (unsafe {
        let _s: *const Vec<u8> = s;
        let _out: *mut Vec<u8> = (&mut out as *mut Vec<u8>);
        TransformApp2Marker_149(_s, _out)
    }) {
        (*transformed_marker_count).postfix_inc();
        return out;
    }
    if (unsafe {
        let _s: *const Vec<u8> = s;
        let _out: *mut Vec<u8> = (&mut out as *mut Vec<u8>);
        TransformApp12Marker_150(_s, _out)
    }) {
        (*transformed_marker_count).postfix_inc();
        return out;
    }
    if (unsafe {
        let _s: *const Vec<u8> = s;
        let _out: *mut Vec<u8> = (&mut out as *mut Vec<u8>);
        TransformApp14Marker_151(_s, _out)
    }) {
        (*transformed_marker_count).postfix_inc();
        return out;
    }
    return (*s).clone();
}
pub unsafe fn GetQuantTableId_153(
    q: *const brunsli_JPEGQuantTable,
    mut is_chroma: bool,
    mut dst: *mut u8,
) -> i32 {
    let mut j: i32 = 0;
    'loop_: while ((j) < (kNumStockQuantTables_47)) {
        let mut match_found: bool = true;
        let mut k: i32 = 0;
        'loop_: while (match_found) && ((k) < (kDCTBlockSize_3)) {
            if (((&(*q)).values[(k as usize)])
                != (kStockQuantizationTables_48[(is_chroma) as usize][(j) as usize][(k) as usize]
                    as i32))
            {
                match_found = false;
            }
            k.prefix_inc();
        }
        if match_found {
            return j;
        }
        j.prefix_inc();
    }
    return (((kNumStockQuantTables_47 as u32).wrapping_add(
        (unsafe { FindBestMatrix_119((&(&(*q)).values[(0_usize)] as *const i32), is_chroma, dst) }),
    )) as i32);
}
pub unsafe fn EncodeVarint_154(mut n: i32, mut max_bits: i32, mut storage: *mut brunsli_Storage) {
    let mut b: i32 = 0_i32;
    if !((n) < ((1) << (max_bits))) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 215, c"EncodeVarint".as_ptr())
        });
        'loop_: while true {}
    };
    b = 0;
    'loop_: while ((n) != (0)) && ((b) < (max_bits)) {
        if (((b) + (1)) != (max_bits)) {
            (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
        }
        (unsafe { WriteBits_120(1_usize, (((n) & (1)) as u64), storage) });
        n >>= 1;
        b.prefix_inc();
    }
    if ((b) < (max_bits)) {
        (unsafe { WriteBits_120(1_usize, 0_u64, storage) });
    }
}
pub unsafe fn EncodeLimitedVarint_155(
    mut bits: usize,
    mut nbits: i32,
    mut max_symbols: i32,
    mut storage: *mut brunsli_Storage,
) {
    let mask: usize = ((1_usize) << (nbits)).wrapping_sub(1_usize);
    let mut b: i32 = 0;
    'loop_: while ((b) < (max_symbols)) {
        (unsafe { WriteBits_120(1_usize, (((bits) != (0_usize)) as u64), storage) });
        if ((bits) == (0_usize)) {
            break;
        }
        (unsafe { WriteBits_120((nbits as usize), (((bits) & (mask)) as u64), storage) });
        bits >>= nbits;
        b.prefix_inc();
    }
}
pub unsafe fn EncodeQuantTables_156(
    jpg: *const brunsli_JPEGData,
    mut storage: *mut brunsli_Storage,
) -> bool {
    if ((*jpg).quant.is_empty()) || (((*jpg).quant.len()) > (4_usize)) {
        return false;
    }
    (unsafe {
        WriteBits_120(
            2_usize,
            (((*jpg).quant.len()).wrapping_sub(1_usize) as u64),
            storage,
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).quant.len())) {
        let q: *const brunsli_JPEGQuantTable =
            &(&(*jpg)).quant[(i)] as *const brunsli_JPEGQuantTable;
        let mut k: i32 = 0;
        'loop_: while ((k) < (kDCTBlockSize_3)) {
            let j: i32 = (kJPEGNaturalOrder_13[(k) as usize] as i32);
            if (((&(*q)).values[(j as usize)]) == (0)) {
                return false;
            }
            k.prefix_inc();
        }
        let mut quant_approx: [u8; 64] = [0_u8; 64];
        let code: i32 = (unsafe {
            let _q: *const brunsli_JPEGQuantTable = q;
            let _is_chroma: bool = ((i) > (0_usize));
            let _dst: *mut u8 = quant_approx.as_mut_ptr();
            GetQuantTableId_153(_q, _is_chroma, _dst)
        });
        (unsafe {
            WriteBits_120(
                1_usize,
                (((code) >= (kNumStockQuantTables_47)) as u64),
                storage,
            )
        });
        if ((code) < (kNumStockQuantTables_47)) {
            (unsafe { WriteBits_120(3_usize, (code as u64), storage) });
        } else {
            let mut q_factor: usize = (((code) - (kNumStockQuantTables_47)) as usize);
            if !((q_factor) < (kQFactorLimit_117)) {
                (unsafe {
                    BrunsliDumpAndAbort_79(
                        c"brunsli_encode.cc".as_ptr(),
                        264,
                        c"EncodeQuantTables".as_ptr(),
                    )
                });
                'loop_: while true {}
            };
            (unsafe { WriteBits_120(kQFactorBits_116, (q_factor as u64), storage) });
            let mut last_diff: i32 = 0;
            let mut k: i32 = 0;
            'loop_: while ((k) < (kDCTBlockSize_3)) {
                let j: i32 = (kJPEGNaturalOrder_13[(k) as usize] as i32);
                let new_diff: i32 =
                    (((&(*q)).values[(j as usize)]) - (quant_approx[(j) as usize] as i32));
                let mut diff: i32 = ((new_diff) - (last_diff));
                last_diff = new_diff;
                (unsafe { WriteBits_120(1_usize, (((diff) != (0)) as u64), storage) });
                if (diff != 0) {
                    (unsafe { WriteBits_120(1_usize, (((diff) < (0)) as u64), storage) });
                    if ((diff) < (0)) {
                        diff = -diff;
                    }
                    diff -= 1;
                    if ((diff) > (65535)) {
                        return false;
                    }
                    (unsafe { EncodeVarint_154(diff, 16, storage) });
                }
                k.prefix_inc();
            }
        }
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) {
        (unsafe {
            WriteBits_120(
                2_usize,
                ((&(*jpg)).components[(i)].quant_idx as u64),
                storage,
            )
        });
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn EncodeHuffmanCode_157(
    huff: *const brunsli_JPEGHuffmanCode,
    mut is_known_last: bool,
    mut storage: *mut brunsli_Storage,
) -> bool {
    (unsafe { WriteBits_120(2_usize, ((((*huff).slot_id) & (15)) as u64), storage) });
    (unsafe { WriteBits_120(1_usize, ((((*huff).slot_id) >> (4)) as u64), storage) });
    if !is_known_last {
        (unsafe { WriteBits_120(1_usize, ((*huff).is_last as u64), storage) });
    } else if !(*huff).is_last {
        return false;
    }
    let mut is_dc_table: i32 = (((((*huff).slot_id) >> (4)) == (0)) as i32);
    let mut total_count: i32 = 0;
    let mut space: i32 = ((1) << (kJpegHuffmanMaxBitLength_7));
    let mut max_len: i32 = kJpegHuffmanMaxBitLength_7;
    let mut max_count: i32 = if (is_dc_table != 0) {
        kJpegDCAlphabetSize_9
    } else {
        kJpegHuffmanAlphabetSize_8
    };
    let mut found_match: i32 = 0;
    let mut stock_table_idx: i32 = 0;
    if (is_dc_table != 0) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (kNumStockDCHuffmanCodes_53)) && (!(found_match != 0)) {
            if (({
                let sa = core::slice::from_raw_parts(
                    ((&(&(*huff)).counts[(1_usize)] as *const i32) as *const i32
                        as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<[i32; 16]>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    (kStockDCHuffmanCodeCounts_54[(i) as usize].as_ptr() as *const i32
                        as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<[i32; 16]>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
                && (({
                    let sa = core::slice::from_raw_parts(
                        ((&(&(*huff)).values[(0_usize)] as *const i32) as *const i32
                            as *const ::libc::c_void) as *const u8,
                        ::std::mem::size_of::<[i32; 13]>() as usize,
                    );
                    let sb = core::slice::from_raw_parts(
                        (kStockDCHuffmanCodeValues_55[(i) as usize].as_ptr() as *const i32
                            as *const ::libc::c_void) as *const u8,
                        ::std::mem::size_of::<[i32; 13]>() as usize,
                    );
                    let mut diff = 0_i32;
                    for (x, y) in sa.iter().zip(sb.iter()) {
                        if x != y {
                            diff = (*x as i32) - (*y as i32);
                            break;
                        }
                    }
                    diff
                }) == (0))
            {
                found_match = 1;
                stock_table_idx = i;
            }
            i.prefix_inc();
        }
    } else {
        let mut i: i32 = 0;
        'loop_: while ((i) < (kNumStockACHuffmanCodes_56)) && (!(found_match != 0)) {
            if (({
                let sa = core::slice::from_raw_parts(
                    ((&(&(*huff)).counts[(1_usize)] as *const i32) as *const i32
                        as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<[i32; 16]>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    (kStockACHuffmanCodeCounts_57[(i) as usize].as_ptr() as *const i32
                        as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<[i32; 16]>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
                && (({
                    let sa = core::slice::from_raw_parts(
                        ((&(&(*huff)).values[(0_usize)] as *const i32) as *const i32
                            as *const ::libc::c_void) as *const u8,
                        ::std::mem::size_of::<[i32; 163]>() as usize,
                    );
                    let sb = core::slice::from_raw_parts(
                        (kStockACHuffmanCodeValues_59[(i) as usize].as_ptr() as *const i32
                            as *const ::libc::c_void) as *const u8,
                        ::std::mem::size_of::<[i32; 163]>() as usize,
                    );
                    let mut diff = 0_i32;
                    for (x, y) in sa.iter().zip(sb.iter()) {
                        if x != y {
                            diff = (*x as i32) - (*y as i32);
                            break;
                        }
                    }
                    diff
                }) == (0))
            {
                found_match = 1;
                stock_table_idx = i;
            }
            i.prefix_inc();
        }
    }
    (unsafe { WriteBits_120(1_usize, (found_match as u64), storage) });
    if (found_match != 0) {
        (unsafe { WriteBits_120(1_usize, (stock_table_idx as u64), storage) });
        return true;
    }
    'loop_: while ((max_len) > (0)) && (((&(*huff)).counts[(max_len as usize)]) == (0)) {
        max_len.prefix_dec();
    }
    if (((&(*huff)).counts[(0_usize)]) != (0)) || ((max_len) == (0)) {
        return false;
    }
    (unsafe { WriteBits_120(4_usize, (((max_len) - (1)) as u64), storage) });
    space -= ((1) << ((kJpegHuffmanMaxBitLength_7) - (max_len)));
    let mut i: i32 = 1;
    'loop_: while ((i) <= (max_len)) {
        let mut count: i32 =
            (((&(*huff)).counts[(i as usize)]) - (if ((i) == (max_len)) { 1 } else { 0 }));
        let mut count_limit: i32 = {
            let mut __tmp_0 = ((max_count) - (total_count));
            let mut __tmp_1 = ((space) >> ((kJpegHuffmanMaxBitLength_7) - (i)));
            (*if *&mut __tmp_0 <= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        };
        if ((count) > (count_limit)) {
            if true {
            } else {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "len = {:} count = {:} limit = {:} space = {:} total = {:}\n",
                    i,
                    count,
                    count_limit,
                    space,
                    total_count,
                );
            }
            return false;
        }
        if ((count_limit) > (0)) {
            let mut nbits: i32 = ((unsafe { Log2FloorNonZero_74((count_limit as u32)) }) + (1));
            (unsafe { WriteBits_120((nbits as usize), (count as u64), storage) });
            total_count += count;
            space -= ((count) * ((1) << ((kJpegHuffmanMaxBitLength_7) - (i))));
        }
        i.prefix_inc();
    }
    if (((&(*huff)).values[(total_count as usize)]) != (kJpegHuffmanAlphabetSize_8)) {
        return false;
    }
    let mut p: brunsli_PermutationCoder = brunsli_PermutationCoder::brunsli_PermutationCoder();
    (unsafe {
        p.Init(if (is_dc_table != 0) {
            core::slice::from_raw_parts(
                kDefaultDCValues_60.as_ptr(),
                (kDefaultDCValues_60.as_ptr().add(kDefaultDCValues_60.len()))
                    .offset_from(kDefaultDCValues_60.as_ptr()) as usize,
            )
            .to_vec()
        } else {
            core::slice::from_raw_parts(
                kDefaultACValues_61.as_ptr(),
                (kDefaultACValues_61.as_ptr().add(kDefaultACValues_61.len()))
                    .offset_from(kDefaultACValues_61.as_ptr()) as usize,
            )
            .to_vec()
        })
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (total_count)) {
        let val: i32 = (&(*huff)).values[(i as usize)];
        let mut code: i32 = 0_i32;
        let mut nbits: i32 = 0_i32;
        if !(unsafe {
            p.RemoveValue(
                (val as u8),
                (&mut code as *mut i32),
                (&mut nbits as *mut i32),
            )
        }) {
            return false;
        }
        (unsafe { EncodeLimitedVarint_155((code as usize), 2, (((nbits) + (1)) >> (1)), storage) });
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn EncodeScanInfo_158(
    si: *const brunsli_JPEGScanInfo,
    mut storage: *mut brunsli_Storage,
) -> bool {
    (unsafe { WriteBits_120(6_usize, ((*si).Ss as u64), storage) });
    (unsafe { WriteBits_120(6_usize, ((*si).Se as u64), storage) });
    (unsafe { WriteBits_120(4_usize, ((*si).Ah as u64), storage) });
    (unsafe { WriteBits_120(4_usize, ((*si).Al as u64), storage) });
    (unsafe {
        WriteBits_120(
            2_usize,
            (((*si).num_components).wrapping_sub(1_usize) as u64),
            storage,
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*si).num_components)) {
        let csi: *const brunsli_JPEGComponentScanInfo =
            &(&(*si)).components[(i)] as *const brunsli_JPEGComponentScanInfo;
        (unsafe { WriteBits_120(2_usize, ((*csi).comp_idx as u64), storage) });
        (unsafe { WriteBits_120(2_usize, ((*csi).dc_tbl_idx as u64), storage) });
        (unsafe { WriteBits_120(2_usize, ((*csi).ac_tbl_idx as u64), storage) });
        i.prefix_inc();
    }
    let mut last_block_idx: i32 = -1_i32;
    'loop_: for block_idx in 0..((*si).reset_points.len()) {
        let mut block_idx = (*si).reset_points.as_ptr().add(block_idx);
        (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
        if !((*block_idx) >= ((last_block_idx) + (1))) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"brunsli_encode.cc".as_ptr(),
                    391,
                    c"EncodeScanInfo".as_ptr(),
                )
            });
            'loop_: while true {}
        };
        (unsafe {
            let _n: i32 = (((*block_idx) - (last_block_idx)) - (1));
            let _storage: *mut brunsli_Storage = storage;
            EncodeVarint_154(_n, 28, _storage)
        });
        last_block_idx = (*block_idx);
    }
    (unsafe { WriteBits_120(1_usize, 0_u64, storage) });
    last_block_idx = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*si).extra_zero_runs.len())) {
        let mut block_idx: i32 = (&(*si)).extra_zero_runs[(i)].block_idx;
        let mut num: i32 = (&(*si)).extra_zero_runs[(i)].num_extra_zero_runs;
        if !((block_idx) >= (last_block_idx)) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"brunsli_encode.cc".as_ptr(),
                    401,
                    c"EncodeScanInfo".as_ptr(),
                )
            });
            'loop_: while true {}
        };
        let mut j: i32 = 0;
        'loop_: while ((j) < (num)) {
            (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
            (unsafe { EncodeVarint_154(((block_idx) - (last_block_idx)), 28, storage) });
            last_block_idx = block_idx;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (unsafe { WriteBits_120(1_usize, 0_u64, storage) });
    return true;
}
pub unsafe fn MatchComponentIds_159(comps: *const Vec<brunsli_JPEGComponent>) -> i32 {
    if (((*comps).len()) == (1_usize)) && (((&(*comps))[(0_usize)].id) == (1)) {
        return kComponentIdsGray_50;
    }
    if (((*comps).len()) == (3_usize)) {
        if ((((&(*comps))[(0_usize)].id) == (1)) && (((&(*comps))[(1_usize)].id) == (2)))
            && (((&(*comps))[(2_usize)].id) == (3))
        {
            return kComponentIds123_49;
        } else if ((((&(*comps))[(0_usize)].id) == (('R' as libc::c_char) as i32))
            && (((&(*comps))[(1_usize)].id) == (('G' as libc::c_char) as i32)))
            && (((&(*comps))[(2_usize)].id) == (('B' as libc::c_char) as i32))
        {
            return kComponentIdsRGB_51;
        }
    }
    return kComponentIdsCustom_52;
}
pub unsafe fn JumpToByteBoundary_160(mut storage: *mut brunsli_Storage) {
    let mut nbits: i32 = ((((*storage).pos) & (7_usize)) as i32);
    if ((nbits) > (0)) {
        (unsafe { WriteBits_120((((8) - (nbits)) as usize), 0_u64, storage) });
    }
}
pub unsafe fn EncodeAuxData_161(
    jpg: *const brunsli_JPEGData,
    mut storage: *mut brunsli_Storage,
) -> bool {
    if ((*jpg).marker_order.is_empty())
        || (((*(((*jpg).marker_order).last().unwrap())) as i32) != (217))
    {
        return false;
    }
    let mut have_dri: bool = false;
    let mut num_scans: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).marker_order.len())) {
        let mut marker: u8 = (&(*jpg)).marker_order[(i)];
        if ((marker as i32) < (192)) {
            return false;
        }
        (unsafe { WriteBits_120(6_usize, (((marker as i32) - (192)) as u64), storage) });
        if ((marker as i32) == (221)) {
            have_dri = true;
        }
        if ((marker as i32) == (218)) {
            num_scans.prefix_inc();
        }
        i.prefix_inc();
    }
    if have_dri {
        (unsafe { WriteBits_120(16_usize, ((*jpg).restart_interval as u64), storage) });
    }
    if !(((*jpg).huffman_code.len()) < (kMaxDHTMarkers_10 as usize)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"brunsli_encode.cc".as_ptr(),
                453,
                c"EncodeAuxData".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).huffman_code.len())) {
        let is_known_last: bool = (((i).wrapping_add(1_usize)) == ((*jpg).huffman_code.len()));
        (unsafe { WriteBits_120(1_usize, (is_known_last as u64), storage) });
        if !(unsafe {
            EncodeHuffmanCode_157(
                &(&(*jpg)).huffman_code[(i)] as *const brunsli_JPEGHuffmanCode,
                is_known_last,
                storage,
            )
        }) {
            return false;
        }
        i.prefix_inc();
    }
    if ((num_scans) != ((*jpg).scan_info.len())) {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).scan_info.len())) {
        if !(unsafe {
            EncodeScanInfo_158(
                &(&(*jpg)).scan_info[(i)] as *const brunsli_JPEGScanInfo,
                storage,
            )
        }) {
            return false;
        }
        i.prefix_inc();
    }
    (unsafe {
        WriteBits_120(
            2_usize,
            (((*jpg).quant.len()).wrapping_sub(1_usize) as u64),
            storage,
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).quant.len())) {
        (unsafe { WriteBits_120(2_usize, ((&(*jpg)).quant[(i)].index as u64), storage) });
        if ((i) != (((*jpg).quant.len()).wrapping_sub(1_usize))) {
            (unsafe { WriteBits_120(1_usize, ((&(*jpg)).quant[(i)].is_last as u64), storage) });
        } else if !(&(*jpg)).quant[(i)].is_last {
            return false;
        }
        (unsafe { WriteBits_120(4_usize, ((&(*jpg)).quant[(i)].precision as u64), storage) });
        i.prefix_inc();
    }
    let mut comp_ids: i32 =
        (unsafe { MatchComponentIds_159(&(*jpg).components as *const Vec<brunsli_JPEGComponent>) });
    (unsafe { WriteBits_120(2_usize, (comp_ids as u64), storage) });
    if ((comp_ids) == (kComponentIdsCustom_52)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < ((*jpg).components.len())) {
            (unsafe { WriteBits_120(8_usize, ((&(*jpg)).components[(i)].id as u64), storage) });
            i.prefix_inc();
        }
    }
    let mut nsize: usize = if (*jpg).has_zero_padding_bit {
        (*jpg).padding_bits.len()
    } else {
        0_usize
    };
    if ((nsize) > ((unsafe { PaddingBitsLimit_17(jpg) }) as usize)) {
        return false;
    }
    (unsafe { EncodeLimitedVarint_155(nsize, 8, 4, storage) });
    if ((nsize) > (0_usize)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (nsize)) {
            (unsafe { WriteBits_120(1_usize, ((&(*jpg)).padding_bits[(i)] as u64), storage) });
            i.prefix_inc();
        }
    }
    (unsafe { JumpToByteBoundary_160(storage) });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).inter_marker_data.len())) {
        let s: *const Vec<u8> = &(&(*jpg)).inter_marker_data[(i)] as *const Vec<u8>;
        let mut buffer: [u8; 10] = [0_u8; 10];
        let mut len: usize = (unsafe { EncodeBase128_146((*s).len(), buffer.as_mut_ptr()) });
        (unsafe { (*storage).AppendBytes((buffer.as_mut_ptr()).cast_const(), len) });
        (unsafe {
            let _src: *const u8 = (*s).as_ptr();
            let _len: usize = (*s).len();
            (*storage).AppendBytes(_src, _len)
        });
        i.prefix_inc();
    }
    return true;
}
impl brunsli_internal_enc_Histogram {}
impl brunsli_internal_enc_Histogram {
    pub unsafe fn Clear(&mut self) {
        {
            let byte_0 = (self.data_.as_mut_ptr() as *mut i32 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[i32; 18]>() {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            (self.data_.as_mut_ptr() as *mut i32 as *mut ::libc::c_void)
        };
        self.total_count_ = 0;
    }
}
impl brunsli_internal_enc_Histogram {
    pub unsafe fn AddHistogram(&mut self, other: *const brunsli_internal_enc_Histogram) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (18)) {
            self.data_[(i) as usize] += (*other).data_[(i) as usize];
            i.prefix_inc();
        }
        self.total_count_ += (*other).total_count_;
    }
}
impl brunsli_internal_enc_Histogram {
    pub unsafe fn Add(&mut self, mut val: usize) {
        if !((val) < (18_usize)) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 522, c"Add".as_ptr())
            });
            'loop_: while true {}
        };
        self.data_[(val)].prefix_inc();
        self.total_count_.prefix_inc();
    }
}
impl brunsli_internal_enc_Histogram {
    pub unsafe fn Merge(&mut self, other: *const brunsli_internal_enc_Histogram) {
        if (((*other).total_count_) == (0)) {
            return;
        }
        self.total_count_ += (*other).total_count_;
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (18_usize)) {
            self.data_[(i)] += (*other).data_[(i)];
            i.prefix_inc();
        }
    }
}
pub unsafe fn ComputeCoeffOrder_162(num_zeros: *const Vec<i32>, mut order: *mut u32) {
    let mut pos_and_val: Vec<(i32, i32)> = (0..(kDCTBlockSize_3 as usize) as usize)
        .map(|_| <(i32, i32)>::default())
        .collect::<Vec<_>>();
    let mut i: i32 = 0;
    'loop_: while ((i) < (kDCTBlockSize_3)) {
        pos_and_val[(i as usize)].0 = i;
        pos_and_val[(i as usize)].1 =
            (&(*num_zeros))[(kJPEGNaturalOrder_13[(i) as usize] as usize)];
        i.prefix_inc();
    }
    {
        let len = pos_and_val
            .as_mut_ptr()
            .add(pos_and_val.len())
            .offset_from(pos_and_val.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(pos_and_val.as_mut_ptr(), len).sort_by(|x, y| {
            if (|a: *const (i32, i32), b: *const (i32, i32)| {
                return (((*a).1) < ((*b).1));
            })(x, y)
            {
                std::cmp::Ordering::Less
            } else if (|a: *const (i32, i32), b: *const (i32, i32)| {
                return (((*a).1) < ((*b).1));
            })(y, x)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (kDCTBlockSize_3 as usize)) {
        (*order.offset((i) as isize)) = kJPEGNaturalOrder_13[(pos_and_val[(i)].0) as usize];
        i.prefix_inc();
    }
}
impl brunsli_internal_enc_EntropySource {
    pub unsafe fn Resize(&mut self, mut num_bands: usize) {
        self.num_bands_ = num_bands;
        {
            let __a0 = (num_bands).wrapping_mul(kNumAvrgContexts_83) as usize;
            self.histograms_
                .resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        };
    }
}
impl brunsli_internal_enc_EntropySource {
    pub unsafe fn AddCode(&mut self, mut code: usize, mut histo_ix: usize) {
        (unsafe { self.histograms_[(histo_ix)].Add(code) });
    }
}
impl brunsli_internal_enc_EntropySource {
    pub unsafe fn Finish(
        &mut self,
        offsets: *const Vec<u64>,
    ) -> Option<Box<brunsli_internal_enc_EntropyCodes>> {
        let mut histograms: Vec<brunsli_internal_enc_Histogram> = Vec::new();
        std::mem::swap(&mut histograms, &mut self.histograms_);
        return Some(Box::from_raw(
            (Box::leak(Box::new(
                brunsli_internal_enc_EntropyCodes::brunsli_internal_enc_EntropyCodes(
                    &histograms as *const Vec<brunsli_internal_enc_Histogram>,
                    self.num_bands_,
                    offsets,
                ),
            )) as *mut brunsli_internal_enc_EntropyCodes),
        ));
    }
}
impl brunsli_internal_enc_EntropySource {
    pub unsafe fn Merge(&mut self, other: *const brunsli_internal_enc_EntropySource) {
        if !((self.histograms_.len()) >= ((*other).histograms_.len())) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 568, c"Merge".as_ptr())
            });
            'loop_: while true {}
        };
        let mut i: usize = 0_usize;
        'loop_: while ((i) < ((*other).histograms_.len())) {
            (unsafe {
                let _other: *const brunsli_internal_enc_Histogram =
                    &(&(*other)).histograms_[(i)] as *const brunsli_internal_enc_Histogram;
                self.histograms_[(i)].Merge(_other)
            });
            i.prefix_inc();
        }
    }
}
impl brunsli_internal_enc_EntropyCodes {}
impl brunsli_internal_enc_EntropyCodes {
    pub unsafe fn EncodeContextMap(&self, mut storage: *mut brunsli_Storage) {
        (unsafe {
            let _context_map: *const Vec<u32> = &self.context_map_ as *const Vec<u32>;
            let _num_clusters: usize = self.clustered_.len();
            EncodeContextMap_163(_context_map, _num_clusters, storage)
        });
    }
}
impl brunsli_internal_enc_EntropyCodes {
    pub unsafe fn BuildAndStoreEntropyCodes(&mut self, mut storage: *mut brunsli_Storage) {
        {
            let __a0 = self.clustered_.len() as usize;
            self.ans_tables_
                .resize_with(__a0, || <brunsli_ANSTable>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (self.clustered_.len())) {
            (unsafe {
                let _histogram: *const i32 =
                    (&mut self.clustered_[(i)].data_[(0) as usize] as *mut i32).cast_const();
                let _table: *mut brunsli_ANSTable =
                    (&mut self.ans_tables_[(i)] as *mut brunsli_ANSTable);
                BuildAndStoreANSEncodingData_123(_histogram, _table, storage)
            });
            i.prefix_inc();
        }
    }
}
impl brunsli_internal_enc_EntropyCodes {
    pub unsafe fn GetANSTable(&self, mut context: i32) -> *const brunsli_ANSTable {
        let entropy_ix: i32 = (self.context_map_[(context as usize)] as i32);
        return (&self.ans_tables_[(entropy_ix as usize)] as *const brunsli_ANSTable);
    }
}
impl brunsli_internal_enc_DataStream {}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn Resize(&mut self, mut max_num_code_words: usize) {
        {
            let __a0 = max_num_code_words as usize;
            self.code_words_.resize_with(__a0, || {
                <brunsli_internal_enc_DataStream_CodeWord>::default()
            })
        };
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn ResizeForBlock(&mut self) {
        if (((self.pos_ as usize).wrapping_add(kSlackForOneBlock_139)) > (self.code_words_.len())) {
            static mut kGrowMult_164: f64 = unsafe { 1.2E+0 };;
            let new_size: usize = (((kGrowMult_164) * (self.code_words_.capacity() as f64))
                as usize)
                .wrapping_add(kSlackForOneBlock_139);
            {
                let __a0 = new_size as usize;
                self.code_words_.resize_with(__a0, || {
                    <brunsli_internal_enc_DataStream_CodeWord>::default()
                })
            };
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn AddCode(
        &mut self,
        mut code: usize,
        mut band: usize,
        mut context: usize,
        mut s: *mut brunsli_internal_enc_EntropySource,
    ) {
        let mut histo_ix: usize = ((band).wrapping_mul(kNumAvrgContexts_83)).wrapping_add(context);
        let mut word: brunsli_internal_enc_DataStream_CodeWord =
            brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord();
        word.context = (histo_ix as u32);
        word.code = ((code as u32) as u8);
        word.nbits = 0_u8;
        word.value = 0_u16;
        if !((self.pos_ as usize) < (self.code_words_.len())) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"brunsli_encode.cc".as_ptr(), 631, c"AddCode".as_ptr())
            });
            'loop_: while true {}
        };
        self.code_words_[(self.pos_.postfix_inc() as usize)] = word;
        (unsafe { (*s).AddCode(code, histo_ix) });
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn AddBits(&mut self, mut nbits: i32, mut bits: i32) {
        self.bw_val_ |= (((bits) << (self.bw_bitpos_)) as u32);
        self.bw_bitpos_ += nbits;
        if ((self.bw_bitpos_) > (16)) {
            let mut word: brunsli_internal_enc_DataStream_CodeWord =
                brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord(
                );
            word.context = 0_u32;
            word.code = 0_u8;
            word.nbits = 16_u8;
            word.value = (((self.bw_val_) & (65535_u32)) as u16);
            self.code_words_[(self.bw_pos_ as usize)] = word;
            self.bw_pos_ = self.pos_;
            self.pos_.prefix_inc();
            self.bw_val_ >>= 16;
            self.bw_bitpos_ -= 16;
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn FlushArithmeticCoder(&mut self) {
        self.code_words_[(self.ac_pos0_ as usize)].value = (((self.high_) >> (16)) as u16);
        self.code_words_[(self.ac_pos1_ as usize)].value = (((self.high_) & (65535_u32)) as u16);
        self.code_words_[(self.ac_pos0_ as usize)].nbits = 16_u8;
        self.code_words_[(self.ac_pos1_ as usize)].nbits = 16_u8;
        self.low_ = 0_u32;
        self.high_ = (!0 as u32);
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn FlushBitWriter(&mut self) {
        self.code_words_[(self.bw_pos_ as usize)].nbits = 16_u8;
        self.code_words_[(self.bw_pos_ as usize)].value = (((self.bw_val_) & (65535_u32)) as u16);
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn AddBit(&mut self, p: *mut brunsli_Prob, mut bit: i32) {
        let prob: u8 = (unsafe { (*(p).cast_const()).get_proba() });
        (unsafe { (*p).Add(bit) });
        let diff: u32 = (self.high_).wrapping_sub(self.low_);
        let split: u32 = (((self.low_ as u64)
            .wrapping_add((((diff as u64).wrapping_mul((prob as u64))) >> (8))))
            as u32);
        if (bit != 0) {
            self.low_ = (split).wrapping_add(1_u32);
        } else {
            self.high_ = split;
        }
        if ((((self.low_) ^ (self.high_)) >> (16)) == (0_u32)) {
            self.code_words_[(self.ac_pos0_ as usize)].value = (((self.high_) >> (16)) as u16);
            self.code_words_[(self.ac_pos0_ as usize)].nbits = 16_u8;
            self.ac_pos0_ = self.ac_pos1_;
            self.ac_pos1_ = self.pos_;
            self.pos_.prefix_inc();
            self.low_ <<= 16;
            self.high_ <<= 16;
            self.high_ |= 65535_u32;
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub unsafe fn EncodeCodeWords(
        &mut self,
        mut s: *mut brunsli_internal_enc_EntropyCodes,
        mut storage: *mut brunsli_Storage,
    ) {
        (unsafe { self.FlushBitWriter() });
        (unsafe { self.FlushArithmeticCoder() });
        let mut ans: brunsli_ANSCoder = brunsli_ANSCoder::brunsli_ANSCoder();
        let mut i: i32 = ((self.pos_) - (1));
        'loop_: while ((i) >= (0)) {
            let word: *mut brunsli_internal_enc_DataStream_CodeWord = (&mut self.code_words_
                [(i as usize)]
                as *mut brunsli_internal_enc_DataStream_CodeWord);
            if (((*word).nbits as i32) == (0)) {
                let info: brunsli_ANSEncSymbolInfo =
                    (*(unsafe { (*(s).cast_const()).GetANSTable(((*word).context as i32)) })).info_
                        [((*word).code) as usize]
                        .clone();
                (*word).value =
                    ((unsafe { ans.PutSymbol(info.clone(), (&mut (*word).nbits as *mut u8)) })
                        as u16)
                        .clone();
            }
            i.prefix_dec();
        }
        let state: u32 = (unsafe { ans.GetState() });
        let mut out: *mut u16 = ((*storage).data as *mut u16);
        let mut out_start: *const u16 = (out).cast_const();
        (unsafe {
            let _p: *mut ::libc::c_void = (out.postfix_inc() as *mut u16 as *mut ::libc::c_void);
            let _v: u16 = (((state) >> (16)) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        (unsafe {
            let _p: *mut ::libc::c_void = (out.postfix_inc() as *mut u16 as *mut ::libc::c_void);
            let _v: u16 = (state as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (self.pos_)) {
            let word: *const brunsli_internal_enc_DataStream_CodeWord =
                &self.code_words_[(i as usize)] as *const brunsli_internal_enc_DataStream_CodeWord;
            if ((*word).nbits != 0) {
                (unsafe {
                    let _p: *mut ::libc::c_void =
                        (out.postfix_inc() as *mut u16 as *mut ::libc::c_void);
                    let _v: u16 = (*word).value;
                    BrunsliUnalignedWrite16_67(_p, _v)
                });
            }
            i.prefix_inc();
        }
        (*storage).pos = ((*storage).pos).wrapping_add(
            (((((out as usize - out_start as usize) / ::std::mem::size_of::<u16>()) as i64)
                * (16_i64)) as usize),
        );
    }
}
pub unsafe fn EncodeNumNonzeros_165(
    mut val: usize,
    mut p: *mut brunsli_Prob,
    mut data_stream: *mut brunsli_internal_enc_DataStream,
) {
    if !((val) < (((1_u32) << (kNumNonZeroBits_84)) as usize)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"brunsli_encode.cc".as_ptr(),
                719,
                c"EncodeNumNonzeros".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let mut bst: *mut brunsli_Prob = p.offset(-((1) as isize));
    let mut ctx: usize = 1_usize;
    let mut mask: usize = (((1) << ((kNumNonZeroBits_84).wrapping_sub(1_usize))) as usize);
    'loop_: while ((mask) != (0_usize)) {
        let bit: i32 = ((((val) & (mask)) != (0_usize)) as i32);
        (unsafe { (*data_stream).AddBit(bst.offset((ctx) as isize), bit) });
        ctx = ((2_usize).wrapping_mul(ctx)).wrapping_add((bit as usize));
        mask >>= 1;
    }
}
pub unsafe fn CollectAllCoeffs_166(mut coeffs: *const i16) -> i16 {
    let mut all_coeffs: i16 = 0_i16;
    let mut k: i32 = 1;
    'loop_: while ((all_coeffs as i32) == (0)) && ((k) < (kDCTBlockSize_3)) {
        all_coeffs = ((all_coeffs as i32) | ((*coeffs.offset((k) as isize)) as i32)) as i16;
        k.prefix_inc();
    }
    return all_coeffs;
}
pub unsafe fn EncodeCoeffOrder_167(
    mut order: *const u32,
    mut data_stream: *mut brunsli_internal_enc_DataStream,
) {
    let mut order_zigzag: [u32; 64] = [0_u32; 64];
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (kDCTBlockSize_3 as usize)) {
        order_zigzag[(i)] = kJPEGZigZagOrder_14[(*order.offset((i) as isize)) as usize];
        i.prefix_inc();
    }
    let mut lehmer: [u32; 64] = [0_u32; 64];
    (unsafe {
        ComputeLehmerCode_112(
            (order_zigzag.as_mut_ptr()).cast_const(),
            (kDCTBlockSize_3 as usize),
            lehmer.as_mut_ptr(),
        )
    });
    let mut tail: i32 = ((kDCTBlockSize_3) - (1));
    'loop_: while ((tail) >= (1)) && ((lehmer[(tail) as usize]) == (0_u32)) {
        tail.prefix_dec();
    }
    let mut i: i32 = 1;
    'loop_: while ((i) <= (tail)) {
        lehmer[(i) as usize].prefix_inc();
        i.prefix_inc();
    }
    static mut kSpan_168: i32 = unsafe { 16 };;
    let mut i: i32 = 0;
    'loop_: while ((i) < (kDCTBlockSize_3)) {
        let start: i32 = if ((i) > (0)) { i } else { 1 };
        let end: i32 = ((i) + (kSpan_168));
        let mut has_non_zero: i32 = 0;
        let mut j: i32 = start;
        'loop_: while ((j) < (end)) {
            has_non_zero = ((has_non_zero as u32) | lehmer[(j) as usize]) as i32;
            j.prefix_inc();
        }
        if !(has_non_zero != 0) {
            (unsafe { (*data_stream).AddBits(1, 0) });
            i += kSpan_168;
            continue 'loop_;
        } else {
            (unsafe { (*data_stream).AddBits(1, 1) });
        }
        let mut j: i32 = start;
        'loop_: while ((j) < (end)) {
            let mut v: i32 = 0_i32;
            if !((lehmer[(j) as usize]) <= (kDCTBlockSize_3 as u32)) {
                (unsafe {
                    BrunsliDumpAndAbort_79(
                        c"brunsli_encode.cc".as_ptr(),
                        769,
                        c"EncodeCoeffOrder".as_ptr(),
                    )
                });
                'loop_: while true {}
            };
            v = (lehmer[(j) as usize] as i32);
            'loop_: while ((v) >= (7)) {
                (unsafe { (*data_stream).AddBits(3, 7) });
                v -= 7;
            }
            (unsafe { (*data_stream).AddBits(3, v) });
            j.prefix_inc();
        }
        i += kSpan_168;
    }
}
pub unsafe fn FrameTypeCode_169(jpg: *const brunsli_JPEGData) -> u32 {
    let mut code: u32 = 0_u32;
    let mut shift: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) && ((i) < (4_usize)) {
        let mut h_samp: u32 = ((((&(*jpg)).components[(i)].h_samp_factor) - (1)) as u32);
        let mut v_samp: u32 = ((((&(*jpg)).components[(i)].v_samp_factor) - (1)) as u32);
        code |= (((h_samp) << ((shift) + (4))) | ((v_samp) << (shift)));
        shift += 8;
        i.prefix_inc();
    }
    return code;
}
pub unsafe fn EncodeSignature_170(mut len: usize, mut data: *mut u8, mut pos: *mut usize) -> bool {
    if ((len) < (kBrunsliSignatureSize_43))
        || ((*pos) > ((len).wrapping_sub(kBrunsliSignatureSize_43)))
    {
        return false;
    }
    {
        if kBrunsliSignatureSize_43 != 0 {
            ::std::ptr::copy_nonoverlapping(
                (kBrunsliSignature_44.as_ptr() as *const u8 as *const ::libc::c_void),
                ((&mut (*data.offset((*pos) as isize)) as *mut u8) as *mut u8
                    as *mut ::libc::c_void),
                kBrunsliSignatureSize_43 as usize,
            )
        }
        ((&mut (*data.offset((*pos) as isize)) as *mut u8) as *mut u8 as *mut ::libc::c_void)
    };
    (*pos) = (*pos).wrapping_add(kBrunsliSignatureSize_43);
    return true;
}
pub unsafe fn EncodeValue_171(
    mut tag: u8,
    mut value: usize,
    mut data: *mut u8,
    mut pos: *mut usize,
) {
    (*data.offset(((*pos).postfix_inc()) as isize)) = (unsafe { ValueMarker_28(tag) });
    (*pos) = (*pos).wrapping_add(
        (unsafe {
            let _val: usize = value;
            let _data: *mut u8 = data.offset((*pos) as isize);
            EncodeBase128_146(_val, _data)
        }),
    );
}
pub unsafe fn EncodeHeader_172(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(state);
    let mut version: usize = ((*jpg).version as usize);
    let mut is_fallback: bool = (((version) & (1_usize)) == (kFallbackVersion_2 as usize));
    if (is_fallback) && ((version) != (kFallbackVersion_2 as usize)) {
        return false;
    }
    if (((!is_fallback) && ((((*jpg).width) == (0)) || (((*jpg).height) == (0))))
        || ((*jpg).components.is_empty()))
        || (((*jpg).components.len()) > (kMaxComponents_4 as usize))
    {
        return false;
    }
    if (((version) & (!7_u32 as usize)) != 0) {
        return false;
    }
    let mut version_comp: usize = ((((((*jpg).components.len()).wrapping_sub(1_usize)) as u64)
        | (((version) << (2)) as u64)) as usize);
    let mut subsampling: usize = ((unsafe { FrameTypeCode_169(jpg) }) as usize);
    let mut pos: usize = 0_usize;
    (unsafe {
        let _tag: u8 = kBrunsliHeaderWidthTag_39;
        let _data: *mut u8 = data;
        EncodeValue_171(
            _tag,
            ((*jpg).width as usize),
            _data,
            (&mut pos as *mut usize),
        )
    });
    (unsafe {
        let _tag: u8 = kBrunsliHeaderHeightTag_40;
        let _data: *mut u8 = data;
        EncodeValue_171(
            _tag,
            ((*jpg).height as usize),
            _data,
            (&mut pos as *mut usize),
        )
    });
    (unsafe {
        let _tag: u8 = kBrunsliHeaderVersionCompTag_41;
        let _data: *mut u8 = data;
        EncodeValue_171(_tag, version_comp, _data, (&mut pos as *mut usize))
    });
    (unsafe {
        let _tag: u8 = kBrunsliHeaderSubsamplingTag_42;
        let _data: *mut u8 = data;
        EncodeValue_171(_tag, subsampling, _data, (&mut pos as *mut usize))
    });
    (*len) = pos;
    return true;
}
pub unsafe fn EncodeMetaData_173(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(state);
    let mut metadata: Vec<u8> = Vec::new();
    let mut transformed_marker_count: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).app_data.len())) {
        let s: *const Vec<u8> = &(&(*jpg)).app_data[(i)] as *const Vec<u8>;
        (unsafe {
            let _dst: *mut Vec<u8> = (&mut metadata as *mut Vec<u8>);
            let mut _src = (unsafe {
                let _s: *const Vec<u8> = s;
                let _transformed_marker_count: *mut usize =
                    (&mut transformed_marker_count as *mut usize);
                TransformAppMarker_152(_s, _transformed_marker_count)
            });
            Append_73(_dst, &mut _src)
        });
        i.prefix_inc();
    }
    if ((transformed_marker_count) > (kBrunsliShortMarkerLimit_23 as usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Too many short markers: {:}\n",
            transformed_marker_count,
        );
        return false;
    }
    let mut other_app_count: usize =
        (((*jpg).app_data.len() as u64).wrapping_sub((transformed_marker_count as u64)) as usize);
    if ((other_app_count) > (kBrunsliMultibyteMarkerLimit_24 as usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Too many app markers: {:}\n",
            other_app_count,
        );
        return false;
    }
    let mut com_count: usize = (*jpg).com_data.len();
    if ((com_count) > (kBrunsliMultibyteMarkerLimit_24 as usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Too many com markers: {:}\n",
            com_count,
        );
        return false;
    }
    'loop_: for s in 0..((*jpg).com_data.len()) {
        let mut s = (*jpg).com_data.as_ptr().add(s);
        (unsafe {
            let _dst: *mut Vec<u8> = (&mut metadata as *mut Vec<u8>);
            let _src: *const Vec<u8> = s;
            Append_73(_dst, _src)
        });
    }
    if !(*jpg).tail_data.is_empty() {
        let marker: [u8; 1] = [217_u8];
        (unsafe { Append_72((&mut metadata as *mut Vec<u8>), marker.as_ptr(), 1_usize) });
        (unsafe {
            Append_73(
                (&mut metadata as *mut Vec<u8>),
                &(*jpg).tail_data as *const Vec<u8>,
            )
        });
    }
    if metadata.is_empty() {
        (*len) = 0_usize;
        return true;
    } else if ((metadata.len()) == (1_usize)) {
        (*len) = 1_usize;
        (*data.offset((0) as isize)) = metadata[(0_usize)];
        return true;
    }
    let mut pos: usize = (unsafe { EncodeBase128_146(metadata.len(), data) });
    let mut compressed_size: usize = (*len).wrapping_sub(pos);
    if !(::brotli_sys::BrotliEncoderCompress(
        kBrotliQuality_141,
        kBrotliWindowBits_142,
        ::brotli_sys::BROTLI_MODE_GENERIC,
        metadata.len(),
        (metadata.as_mut_ptr()).cast_const(),
        (&mut compressed_size as *mut usize),
        (&mut (*data.offset((pos) as isize)) as *mut u8),
    ) != 0)
    {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Brotli compression failed: input size = {:} pos = {:} len = {:}\n",
            metadata.len(),
            pos,
            (*len),
        );
        return false;
    }
    pos = (pos).wrapping_add(compressed_size);
    (*len) = pos;
    return true;
}
pub unsafe fn EncodeJPEGInternals_174(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(state);
    let mut storage: brunsli_Storage = brunsli_Storage::brunsli_Storage(data, (*len));
    if !(unsafe {
        let _jpg: *const brunsli_JPEGData = jpg;
        let _storage: *mut brunsli_Storage = (&mut storage as *mut brunsli_Storage);
        EncodeAuxData_161(_jpg, _storage)
    }) {
        return false;
    }
    (*len) = (unsafe { storage.GetBytesUsed() }).clone();
    return true;
}
pub unsafe fn EncodeQuantData_175(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(state);
    let mut storage: brunsli_Storage = brunsli_Storage::brunsli_Storage(data, (*len));
    if !(unsafe {
        let _jpg: *const brunsli_JPEGData = jpg;
        let _storage: *mut brunsli_Storage = (&mut storage as *mut brunsli_Storage);
        EncodeQuantTables_156(_jpg, _storage)
    }) {
        return false;
    }
    (*len) = (unsafe { storage.GetBytesUsed() }).clone();
    return true;
}
pub unsafe fn EncodeHistogramData_176(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    let mut storage: brunsli_Storage = brunsli_Storage::brunsli_Storage(data, (*len));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) {
        (unsafe {
            WriteBits_120(
                3_usize,
                ((&mut (*state)).meta[(i)].context_bits as u64),
                (&mut storage as *mut brunsli_Storage),
            )
        });
        i.prefix_inc();
    }
    (unsafe {
        (*((*state).entropy_codes).cast_const())
            .EncodeContextMap((&mut storage as *mut brunsli_Storage))
    });
    (unsafe {
        (*(*state).entropy_codes).BuildAndStoreEntropyCodes((&mut storage as *mut brunsli_Storage))
    });
    (*len) = (unsafe { storage.GetBytesUsed() }).clone();
    return true;
}
pub unsafe fn EncodeDCData_177(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(*jpg);
    let mut storage: brunsli_Storage = brunsli_Storage::brunsli_Storage(data, (*len));
    (unsafe {
        let _s: *mut brunsli_internal_enc_EntropyCodes = (*state).entropy_codes;
        (*state)
            .data_stream_dc
            .EncodeCodeWords(_s, (&mut storage as *mut brunsli_Storage))
    });
    (*len) = (unsafe { storage.GetBytesUsed() }).clone();
    return true;
}
pub unsafe fn EncodeACData_178(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(*jpg);
    let mut storage: brunsli_Storage = brunsli_Storage::brunsli_Storage(data, (*len));
    (unsafe {
        let _s: *mut brunsli_internal_enc_EntropyCodes = (*state).entropy_codes;
        (*state)
            .data_stream_ac
            .EncodeCodeWords(_s, (&mut storage as *mut brunsli_Storage))
    });
    (*len) = (unsafe { storage.GetBytesUsed() }).clone();
    return true;
}
pub unsafe fn EncodeSection_179(
    jpg: *const brunsli_JPEGData,
    mut s: *mut brunsli_internal_enc_State,
    mut tag: u8,
    mut write_section: Option<
        unsafe fn(
            *const brunsli_JPEGData,
            *mut brunsli_internal_enc_State,
            *mut u8,
            *mut usize,
        ) -> bool,
    >,
    mut section_size_bytes: usize,
    mut len: usize,
    mut data: *mut u8,
    mut pos: *mut usize,
) -> bool {
    let pos_start: usize = (*pos);
    let marker: u8 = (unsafe { SectionMarker_29(tag) });
    (*data.offset(((*pos).postfix_inc()) as isize)) = marker;
    (*pos) = (*pos).wrapping_add(section_size_bytes);
    let mut section_size: usize = (len).wrapping_sub((*pos));
    if !(unsafe {
        let _arg0: *const brunsli_JPEGData = jpg;
        let _arg1: *mut brunsli_internal_enc_State = s;
        let _arg2: *mut u8 = (&mut (*data.offset((*pos) as isize)) as *mut u8);
        let _arg3: *mut usize = (&mut section_size as *mut usize);
        (write_section).unwrap()(_arg0, _arg1, _arg2, _arg3)
    }) {
        return false;
    }
    (*pos) = (*pos).wrapping_add(section_size);
    if (((section_size) >> ((7_usize).wrapping_mul(section_size_bytes))) > (0_usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Section 0x",
        );
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        )
        .write_all(&([(&[marker as u8] as &[u8]), (b" size " as &[u8])].concat()));
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "{:} too large for {:} bytes base128 number.\n",
            section_size,
            section_size_bytes,
        );
        return false;
    }
    (unsafe {
        EncodeBase128Fix_147(
            section_size,
            section_size_bytes,
            (&mut (*data.offset(((pos_start).wrapping_add(1_usize)) as isize)) as *mut u8),
        )
    });
    return true;
}
pub unsafe fn SampleNumNonZeros_180(mut m: *mut brunsli_internal_enc_ComponentMeta) -> usize {
    let mut num_blocks: usize = ((((*m).width_in_blocks) * ((*m).height_in_blocks)) as usize);
    if ((num_blocks) < (((32) * (32)) as usize)) {
        return (kDCTBlockSize_3 as usize).wrapping_mul(num_blocks);
    }
    let mut coeffs: *const i16 = (*m).ac_coeffs;
    let mut stride: usize = ((*m).ac_stride as usize);
    let mut width_in_blocks: usize = ((*m).width_in_blocks as usize);
    let num_zeros: *mut Vec<i32> = &mut (*m).num_zeros as *mut Vec<i32>;
    static mut kStride_181: i32 = unsafe { 5 };;
    let mut total_nonzeros: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_blocks)) {
        let mut x: usize = (i).wrapping_rem(width_in_blocks);
        let mut y: usize = (i).wrapping_div(width_in_blocks);
        let mut block: *const i16 = coeffs
            .offset(((x).wrapping_mul((kDCTBlockSize_3 as usize))) as isize)
            .offset(((y).wrapping_mul(stride)) as isize);
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (kDCTBlockSize_3 as usize)) {
            if (((*block.offset((k) as isize)) as i32) == (0)) {
                (&mut (*num_zeros))[(k)].prefix_inc();
            }
            k.prefix_inc();
        }
        total_nonzeros = (total_nonzeros).wrapping_add((kDCTBlockSize_3 as usize));
        i = (i).wrapping_add((kStride_181 as usize));
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (kDCTBlockSize_3 as usize)) {
        total_nonzeros = (total_nonzeros).wrapping_sub(((&mut (*num_zeros))[(i)] as usize));
        i.prefix_inc();
    }
    (&mut (*num_zeros))[(0_usize)] = 0;
    return (total_nonzeros).wrapping_mul((kStride_181 as usize));
}
pub unsafe fn SelectContextBits_182(mut num_symbols: usize) -> i32 {
    static mut kContextBits_183: [i32; 33] = unsafe {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6,
            6, 6, 6, 6,
        ]
    };;
    let mut log2_size: usize = ((unsafe { Log2FloorNonZero_74((num_symbols as u32)) }) as usize);
    let mut scheme: i32 = kContextBits_183[(log2_size)];
    if !((scheme) < (kNumSchemes_91)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"brunsli_encode.cc".as_ptr(),
                1029,
                c"SelectContextBits".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    return scheme;
}
pub unsafe fn PredictDCCoeffs_184(mut state: *mut brunsli_internal_enc_State) -> bool {
    let meta: *mut Vec<brunsli_internal_enc_ComponentMeta> =
        &mut (*state).meta as *mut Vec<brunsli_internal_enc_ComponentMeta>;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*meta).len())) {
        let m: *mut brunsli_internal_enc_ComponentMeta =
            &mut (&mut (*meta))[(i)] as *mut brunsli_internal_enc_ComponentMeta;
        let width: i32 = (*m).width_in_blocks;
        let height: i32 = (*m).height_in_blocks;
        let ac_stride: i32 = (*m).ac_stride;
        let dc_stride: i32 = (*m).dc_stride;
        let mut y: i32 = 0;
        'loop_: while ((y) < (height)) {
            let mut coeffs: *const i16 = (*m).ac_coeffs.offset(((ac_stride) * (y)) as isize);
            let mut pred_errors: *mut i16 = (*m)
                .dc_prediction_errors
                .offset(((dc_stride) * (y)) as isize);
            let mut x: i32 = 0;
            'loop_: while ((x) < (width)) {
                let mut err: i32 = (((*coeffs.offset((0) as isize)) as i32)
                    - (unsafe { PredictWithAdaptiveMedian_115(coeffs, x, y, ac_stride) }));
                if ((err.abs()) > (kBrunsliMaxDCAbsVal_19)) {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "Invalid DC coefficient: {:} after prediction: {:}\n",
                        (*coeffs.offset((0) as isize)),
                        err,
                    );
                    return false;
                }
                coeffs = (coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                (*(pred_errors.postfix_inc())) = (err as i16);
                x.prefix_inc();
            }
            y.prefix_inc();
        }
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn CalculateMeta_185(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
) -> bool {
    let num_components: usize = (*jpg).components.len();
    let meta: *mut Vec<brunsli_internal_enc_ComponentMeta> =
        &mut (*state).meta as *mut Vec<brunsli_internal_enc_ComponentMeta>;
    {
        let __a0 = num_components as usize;
        (*meta).resize_with(__a0, || <brunsli_internal_enc_ComponentMeta>::default())
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        let c: *const brunsli_JPEGComponent =
            &(&(*jpg)).components[(i)] as *const brunsli_JPEGComponent;
        let m: *mut brunsli_internal_enc_ComponentMeta =
            &mut (&mut (*meta))[(i)] as *mut brunsli_internal_enc_ComponentMeta;
        if (((*c).quant_idx as usize) >= ((*jpg).quant.len())) {
            return false;
        }
        let q: *const brunsli_JPEGQuantTable =
            &(&(*jpg)).quant[((*c).quant_idx as usize)] as *const brunsli_JPEGQuantTable;
        (*m).h_samp = (*c).h_samp_factor;
        (*m).v_samp = (*c).v_samp_factor;
        (*m).width_in_blocks = (((*jpg).MCU_cols) * ((*m).h_samp));
        (*m).height_in_blocks = (((*jpg).MCU_rows) * ((*m).v_samp));
        (*m).ac_coeffs = (&(&(*c)).coeffs[(0_usize)] as *const i16);
        (*m).ac_stride = (((*m).width_in_blocks) * (kDCTBlockSize_3));
        (*m).dc_stride = (*m).width_in_blocks;
        (*m).b_stride = (*m).width_in_blocks;
        {
            if (kDCTBlockSize_3 as usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) != 0
            {
                ::std::ptr::copy_nonoverlapping(
                    ((&(&(*q)).values[(0_usize)] as *const i32) as *const i32
                        as *const ::libc::c_void),
                    ((*m).quant.as_mut_ptr() as *mut i32 as *mut ::libc::c_void),
                    (kDCTBlockSize_3 as usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))
                        as usize,
                )
            }
            ((*m).quant.as_mut_ptr() as *mut i32 as *mut ::libc::c_void)
        };
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn EncodeDC_186(mut state: *mut brunsli_internal_enc_State) {
    let meta: *const Vec<brunsli_internal_enc_ComponentMeta> =
        &(*state).meta as *const Vec<brunsli_internal_enc_ComponentMeta>;
    let num_components: usize = (*meta).len();
    let mcu_rows: i32 = (((&(*meta))[(0_usize)].height_in_blocks) / ((&(*meta))[(0_usize)].v_samp));
    let entropy_source: *mut brunsli_internal_enc_EntropySource =
        &mut (*state).entropy_source as *mut brunsli_internal_enc_EntropySource;
    let data_stream: *mut brunsli_internal_enc_DataStream =
        &mut (*state).data_stream_dc as *mut brunsli_internal_enc_DataStream;
    let mut comps: Vec<brunsli_ComponentStateDC> = (0..(num_components) as usize)
        .map(|_| <brunsli_ComponentStateDC>::default())
        .collect::<Vec<_>>();
    let mut total_num_blocks: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        let m: *const brunsli_internal_enc_ComponentMeta =
            &(&(*meta))[(i)] as *const brunsli_internal_enc_ComponentMeta;
        (unsafe { comps[(i)].SetWidth((*m).width_in_blocks) });
        total_num_blocks = (total_num_blocks)
            .wrapping_add(((((*m).width_in_blocks) * ((*m).height_in_blocks)) as usize));
        i.prefix_inc();
    }
    (unsafe {
        let _num_bands: usize = num_components;
        (*entropy_source).Resize(_num_bands)
    });
    (unsafe {
        let _max_num_code_words: usize =
            ((3_usize).wrapping_mul(total_num_blocks)).wrapping_add(128_usize);
        (*data_stream).Resize(_max_num_code_words)
    });
    let mut mcu_y: i32 = 0;
    'loop_: while ((mcu_y) < (mcu_rows)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (num_components)) {
            let mut c: *mut brunsli_ComponentStateDC =
                (&mut comps[(i)] as *mut brunsli_ComponentStateDC);
            let m: *const brunsli_internal_enc_ComponentMeta =
                &(&(*meta))[(i)] as *const brunsli_internal_enc_ComponentMeta;
            let width: i32 = (*c).width;
            let ac_stride: i32 = (*m).ac_stride;
            let dc_stride: i32 = (*m).dc_stride;
            let b_stride: i32 = (*m).b_stride;
            let mut y: i32 = ((mcu_y) * ((*m).v_samp));
            let mut prev_sgn: *mut i32 = (&mut (&mut (*c)).prev_sign[(1_usize)] as *mut i32);
            let mut prev_abs: *mut i32 = (&mut (&mut (*c)).prev_abs_coeff[(2_usize)] as *mut i32);
            let mut iy: i32 = 0;
            'loop_: while ((iy) < ((*m).v_samp)) {
                let mut dc_coeffs_in: *const i16 = ((*m)
                    .dc_prediction_errors
                    .offset(((y) * (dc_stride)) as isize))
                .cast_const();
                let mut ac_coeffs_in: *const i16 =
                    (*m).ac_coeffs.offset(((y) * (ac_stride)) as isize);
                let mut block_state: *mut u8 = (*m).block_state.offset(((y) * (b_stride)) as isize);
                let mut x: i32 = 0;
                'loop_: while ((x) < (width)) {
                    (unsafe { (*data_stream).ResizeForBlock() });
                    let coeff: i16 = (*dc_coeffs_in.offset((0) as isize));
                    let sign: i32 = if ((coeff as i32) > (0)) {
                        1
                    } else {
                        if ((coeff as i32) < (0)) { 2 } else { 0 }
                    };
                    let absval: i32 = if ((sign) == (2)) {
                        -(coeff as i32)
                    } else {
                        (coeff as i32)
                    };
                    let all_coeffs: i16 = (((coeff as i32)
                        | ((unsafe { CollectAllCoeffs_166(ac_coeffs_in) }) as i32))
                        as i16);
                    let is_empty_block: bool = ((all_coeffs as i32) == (0));
                    let is_empty_ctx: i32 = (unsafe {
                        IsEmptyBlockContext_106(
                            (&mut (&mut (*c)).prev_is_nonempty[(1_usize)] as *mut i32).cast_const(),
                            x,
                        )
                    });
                    (unsafe {
                        let _p: *mut brunsli_Prob = (&mut (&mut (*c)).is_empty_block_prob
                            [(is_empty_ctx as usize)]
                            as *mut brunsli_Prob);
                        let _bit: i32 = (!is_empty_block as i32);
                        (*data_stream).AddBit(_p, _bit)
                    });
                    (&mut (*c)).prev_is_nonempty[(((x) + (1)) as usize)] = (!is_empty_block as i32);
                    (*block_state) = (is_empty_block as u8);
                    if !is_empty_block {
                        let is_zero: i32 = (((coeff as i32) == (0)) as i32);
                        (unsafe {
                            let _p: *mut brunsli_Prob =
                                (&mut (*c).is_zero_prob as *mut brunsli_Prob);
                            let _bit: i32 = is_zero;
                            (*data_stream).AddBit(_p, _bit)
                        });
                        if !(is_zero != 0) {
                            let avrg_ctx: i32 = (unsafe {
                                let _vals: *const i32 = (prev_abs).cast_const();
                                let _x: i32 = x;
                                WeightedAverageContextDC_97(_vals, _x)
                            });
                            let sign_ctx: i32 = (((*prev_sgn.offset((x) as isize)) * (3))
                                + (*prev_sgn.offset(((x) - (1)) as isize)));
                            (unsafe {
                                let _p: *mut brunsli_Prob = (&mut (&mut (*c)).sign_prob
                                    [(sign_ctx as usize)]
                                    as *mut brunsli_Prob);
                                let _bit: i32 = ((sign) - (1));
                                (*data_stream).AddBit(_p, _bit)
                            });
                            let zdens_ctx: usize = i;
                            if ((absval) <= (kNumDirectCodes_140)) {
                                (unsafe {
                                    let _code: usize = (((absval) - (1)) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = ((avrg_ctx as u32) as usize);
                                    let _s: *mut brunsli_internal_enc_EntropySource =
                                        (entropy_source);
                                    (*data_stream).AddCode(_code, _band, _context, _s)
                                });
                            } else {
                                let mut nbits: i32 = ((unsafe {
                                    Log2FloorNonZero_74(
                                        ((((absval) - (kNumDirectCodes_140)) + (1)) as u32),
                                    )
                                }) - (1));
                                (unsafe {
                                    let _code: usize = (((kNumDirectCodes_140) + (nbits)) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = (avrg_ctx as usize);
                                    let _s: *mut brunsli_internal_enc_EntropySource =
                                        (entropy_source);
                                    (*data_stream).AddCode(_code, _band, _context, _s)
                                });
                                let mut extra_bits: i32 =
                                    ((absval) - (((kNumDirectCodes_140) - (1)) + ((2) << (nbits))));
                                let mut first_extra_bit: i32 = (((extra_bits) >> (nbits)) & (1));
                                (unsafe {
                                    let _p: *mut brunsli_Prob =
                                        (&mut (&mut (*c)).first_extra_bit_prob[(nbits as usize)]
                                            as *mut brunsli_Prob);
                                    let _bit: i32 = first_extra_bit;
                                    (*data_stream).AddBit(_p, _bit)
                                });
                                if ((nbits) > (0)) {
                                    extra_bits &= (((1) << (nbits)) - (1));
                                    (unsafe {
                                        let _nbits: i32 = nbits;
                                        let _bits: i32 = extra_bits;
                                        (*data_stream).AddBits(_nbits, _bits)
                                    });
                                }
                            }
                        }
                    }
                    (*prev_sgn.offset((x) as isize)) = sign;
                    (*prev_abs.offset((x) as isize)) = absval;
                    block_state.prefix_inc();
                    dc_coeffs_in.prefix_inc();
                    ac_coeffs_in = (ac_coeffs_in).wrapping_add(kDCTBlockSize_3 as usize);
                    x.prefix_inc();
                }
                iy.prefix_inc();
                y.prefix_inc();
            }
            i.prefix_inc();
        }
        mcu_y.prefix_inc();
    }
}
pub unsafe fn EncodeAC_187(mut state: *mut brunsli_internal_enc_State) {
    let meta: *const Vec<brunsli_internal_enc_ComponentMeta> =
        &(*state).meta as *const Vec<brunsli_internal_enc_ComponentMeta>;
    let num_components: usize = (*meta).len();
    let mcu_rows: i32 = (((&(*meta))[(0_usize)].height_in_blocks) / ((&(*meta))[(0_usize)].v_samp));
    let entropy_source: *mut brunsli_internal_enc_EntropySource =
        &mut (*state).entropy_source as *mut brunsli_internal_enc_EntropySource;
    let data_stream: *mut brunsli_internal_enc_DataStream =
        &mut (*state).data_stream_ac as *mut brunsli_internal_enc_DataStream;
    let mut context_modes: *const u8 = kContextAlgorithm_95.as_ptr().offset(
        (if (*state).use_legacy_context_model {
            64
        } else {
            0
        }) as isize,
    );
    let mut num_code_words: usize = 0_usize;
    let mut comps: Vec<brunsli_ComponentState> = (0..(num_components) as usize)
        .map(|_| <brunsli_ComponentState>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        let m: *const brunsli_internal_enc_ComponentMeta =
            &(&(*meta))[(i)] as *const brunsli_internal_enc_ComponentMeta;
        let num_blocks: usize = ((((*m).width_in_blocks) * ((*m).height_in_blocks)) as usize);
        num_code_words = (num_code_words).wrapping_add(
            (((2_usize).wrapping_mul((*m).approx_total_nonzeros)).wrapping_add(1024_usize))
                .wrapping_add((3_usize).wrapping_mul(num_blocks)),
        );
        (unsafe {
            ComputeCoeffOrder_162(
                &(*m).num_zeros as *const Vec<i32>,
                (&mut comps[(i)].order[(0) as usize] as *mut u32),
            )
        });
        (unsafe {
            let _mult_row: *mut i32 = (&mut comps[(i)].mult_row[(0) as usize] as *mut i32);
            let _mult_col: *mut i32 = (&mut comps[(i)].mult_col[(0) as usize] as *mut i32);
            ComputeACPredictMultipliers_109((*m).quant.as_ptr(), _mult_row, _mult_col)
        });
        (unsafe { comps[(i)].SetWidth((*m).width_in_blocks) });
        i.prefix_inc();
    }
    (unsafe {
        let _num_bands: usize = (*state).num_contexts;
        (*entropy_source).Resize(_num_bands)
    });
    (unsafe {
        let _max_num_code_words: usize = num_code_words;
        (*data_stream).Resize(_max_num_code_words)
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        (unsafe {
            EncodeCoeffOrder_167(
                (&mut comps[(i)].order[(0) as usize] as *mut u32).cast_const(),
                (data_stream),
            )
        });
        i.prefix_inc();
    }
    let mut mcu_y: i32 = 0;
    'loop_: while ((mcu_y) < (mcu_rows)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (num_components)) {
            let c: *mut brunsli_ComponentState = (&mut comps[(i)] as *mut brunsli_ComponentState);
            let m: *const brunsli_internal_enc_ComponentMeta =
                &(&(*meta))[(i)] as *const brunsli_internal_enc_ComponentMeta;
            let cur_ctx_bits: i32 = (*m).context_bits;
            let mut cur_order: *const u32 = ((*c).order.as_mut_ptr()).cast_const();
            let width: i32 = (*c).width;
            let mut y: i32 = ((mcu_y) * ((*m).v_samp));
            let ac_stride: i32 = (*m).ac_stride;
            let b_stride: i32 = (*m).b_stride;
            let mut prev_row_delta: i32 =
                ((((1) - ((2) * ((y) & (1)))) * ((width) + (3))) * (kDCTBlockSize_3));
            let mut iy: i32 = 0;
            'loop_: while ((iy) < ((*m).v_samp)) {
                let mut coeffs_in: *const i16 = (*m).ac_coeffs.offset(((y) * (ac_stride)) as isize);
                let mut block_state: *const u8 =
                    ((*m).block_state.offset(((y) * (b_stride)) as isize)).cast_const();
                let mut prev_row_coeffs: *const i16 = coeffs_in.offset(-((ac_stride) as isize));
                let mut prev_col_coeffs: *const i16 =
                    coeffs_in.offset(-((kDCTBlockSize_3) as isize));
                let mut prev_sgn: *mut i32 =
                    (&mut (&mut (*c)).prev_sign[(kDCTBlockSize_3 as usize)] as *mut i32);
                let mut prev_abs: *mut i32 = (&mut (&mut (*c)).prev_abs_coeff
                    [((((((y) & (1)) * ((width) + (3))) + (2)) * (kDCTBlockSize_3)) as usize)]
                    as *mut i32);
                let mut x: i32 = 0;
                'loop_: while ((x) < (width)) {
                    (unsafe { (*data_stream).ResizeForBlock() });
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
                    let is_empty_block: bool = ((*block_state) != 0);
                    if !is_empty_block {
                        let mut k: i32 = 1;
                        'loop_: while ((k) < (kDCTBlockSize_3)) {
                            let k_nat: i32 = ((*cur_order.offset((k) as isize)) as i32);
                            coeffs[(k) as usize] = (*coeffs_in.offset((k_nat) as isize));
                            if (coeffs[(k) as usize] != 0) {
                                last_nz = k;
                            }
                            k.prefix_inc();
                        }
                        let nzero_context: u8 = (unsafe {
                            NumNonzerosContext_104(
                                ((*c).prev_num_nonzeros.as_mut_ptr()).cast_const(),
                                x,
                                y,
                            )
                        });
                        (unsafe {
                            EncodeNumNonzeros_165(
                                (last_nz as usize),
                                (*c).num_nonzero_prob.as_mut_ptr().offset(
                                    ((kNumNonZeroTreeSize_85)
                                        .wrapping_mul((nzero_context as usize)))
                                        as isize,
                                ),
                                (data_stream),
                            )
                        });
                    }
                    let mut k: i32 = ((kDCTBlockSize_3) - (1));
                    'loop_: while ((k) > (last_nz)) {
                        (*prev_sgn.offset((k) as isize)) = 0;
                        (*prev_abs.offset((k) as isize)) = 0;
                        k.prefix_dec();
                    }
                    let mut num_nzeros: usize = 0_usize;
                    let mut encoded_coeffs: [i16; 64] = [
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16, 0_i16,
                        0_i16, 0_i16, 0_i16, 0_i16,
                    ];
                    let mut k: i32 = last_nz;
                    'loop_: while ((k) >= (1)) {
                        let mut coeff: i16 = coeffs[(k) as usize];
                        let is_zero: i32 = (((coeff as i32) == (0)) as i32);
                        if ((k) < (last_nz)) {
                            let bucket: i32 =
                                (kNonzeroBuckets_89[((num_nzeros).wrapping_sub(1_usize))] as i32);
                            let is_zero_ctx: i32 = (((bucket) * (kDCTBlockSize_3)) + (k));
                            let p: *mut brunsli_Prob = (&mut (&mut (*c)).is_zero_prob
                                [(is_zero_ctx as usize)]
                                as *mut brunsli_Prob);
                            (unsafe {
                                let _p: *mut brunsli_Prob = p;
                                let _bit: i32 = is_zero;
                                (*data_stream).AddBit(_p, _bit)
                            });
                        }
                        if !(is_zero != 0) {
                            let sign: i32 = (if ((coeff as i32) > (0)) { 0 } else { 1 });
                            let absval: i32 = if (sign != 0) {
                                -(coeff as i32)
                            } else {
                                (coeff as i32)
                            };
                            let k_nat: i32 = ((*cur_order.offset((k) as isize)) as i32);
                            let mut context_type: usize =
                                ((*context_modes.offset((k_nat) as isize)) as usize);
                            let mut avg_ctx: usize = 0_usize;
                            let mut sign_ctx: usize = kMaxAverageContext_82;
                            if (((context_type) & (1_usize)) != 0) && ((y) > (0)) {
                                if ((y) > (0)) {
                                    let mut offset: usize = (((k_nat) & (7)) as usize);
                                    (unsafe {
                                        let _prev: *const i16 =
                                            prev_row_coeffs.offset((offset) as isize);
                                        let _cur: *const i16 =
                                            (encoded_coeffs.as_mut_ptr().offset((offset) as isize))
                                                .cast_const();
                                        let _mult: *const i32 = (&mut (*c).mult_col
                                            [((offset).wrapping_mul(8_usize))]
                                            as *mut i32)
                                            .cast_const();
                                        ACPredictContextRow_103(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (&mut avg_ctx as *mut usize),
                                            (&mut sign_ctx as *mut usize),
                                        )
                                    });
                                }
                            } else if (((context_type) & (2_usize)) != 0) && ((x) > (0)) {
                                if ((x) > (0)) {
                                    let mut offset: usize = (((k_nat) & (!7)) as usize);
                                    (unsafe {
                                        let _prev: *const i16 =
                                            prev_col_coeffs.offset((offset) as isize);
                                        let _cur: *const i16 =
                                            (encoded_coeffs.as_mut_ptr().offset((offset) as isize))
                                                .cast_const();
                                        let _mult: *const i32 =
                                            (&mut (*c).mult_row[(offset)] as *mut i32).cast_const();
                                        ACPredictContextCol_102(
                                            _prev,
                                            _cur,
                                            _mult,
                                            (&mut avg_ctx as *mut usize),
                                            (&mut sign_ctx as *mut usize),
                                        )
                                    });
                                }
                            } else if !(context_type != 0) {
                                avg_ctx = ((unsafe {
                                    let _vals: *const i32 =
                                        (prev_abs.offset((k) as isize)).cast_const();
                                    let _prev_row_delta: i32 = prev_row_delta;
                                    WeightedAverageContext_98(_vals, _prev_row_delta)
                                }) as usize);
                                sign_ctx = ((((*prev_sgn.offset((k) as isize)) * (3))
                                    + (*prev_sgn.offset(((k) - (kDCTBlockSize_3)) as isize)))
                                    as usize);
                            }
                            sign_ctx = ((sign_ctx).wrapping_mul((kDCTBlockSize_3 as usize)))
                                .wrapping_add((k as usize));
                            let sign_p: *mut brunsli_Prob =
                                (&mut (&mut (*c)).sign_prob[(sign_ctx)] as *mut brunsli_Prob);
                            (unsafe {
                                let _p: *mut brunsli_Prob = sign_p;
                                let _bit: i32 = sign;
                                (*data_stream).AddBit(_p, _bit)
                            });
                            (*prev_sgn.offset((k) as isize)) = ((sign) + (1));
                            let zdens_ctx: usize = ((*m).context_offset).wrapping_add(
                                ((unsafe {
                                    ZeroDensityContext_96(
                                        num_nzeros,
                                        (k as usize),
                                        (cur_ctx_bits as usize),
                                    )
                                }) as usize),
                            );
                            if ((absval) <= (kNumDirectCodes_140)) {
                                (unsafe {
                                    let _code: usize = (((absval) - (1)) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = avg_ctx;
                                    let _s: *mut brunsli_internal_enc_EntropySource =
                                        (entropy_source);
                                    (*data_stream).AddCode(_code, _band, _context, _s)
                                });
                            } else {
                                let base_code: i32 = (((absval) - (kNumDirectCodes_140)) + (1));
                                let nbits: i32 =
                                    ((unsafe { Log2FloorNonZero_74((base_code as u32)) }) - (1));
                                (unsafe {
                                    let _code: usize = (((kNumDirectCodes_140) + (nbits)) as usize);
                                    let _band: usize = zdens_ctx;
                                    let _context: usize = ((avg_ctx as u32) as usize);
                                    let _s: *mut brunsli_internal_enc_EntropySource =
                                        (entropy_source);
                                    (*data_stream).AddCode(_code, _band, _context, _s)
                                });
                                let extra_bits: i32 = ((base_code) - ((2) << (nbits)));
                                let first_extra_bit: i32 = (((extra_bits) >> (nbits)) & (1));
                                let p: *mut brunsli_Prob = (&mut (&mut (*c)).first_extra_bit_prob
                                    [((((k) * (10)) + (nbits)) as usize)]
                                    as *mut brunsli_Prob);
                                (unsafe {
                                    let _p: *mut brunsli_Prob = p;
                                    let _bit: i32 = first_extra_bit;
                                    (*data_stream).AddBit(_p, _bit)
                                });
                                if ((nbits) > (0)) {
                                    let left_over_bits: i32 =
                                        ((extra_bits) & (((1) << (nbits)) - (1)));
                                    (unsafe {
                                        let _nbits: i32 = nbits;
                                        let _bits: i32 = left_over_bits;
                                        (*data_stream).AddBits(_nbits, _bits)
                                    });
                                }
                            }
                            num_nzeros.prefix_inc();
                            encoded_coeffs[(k_nat) as usize] = coeff;
                            (*prev_abs.offset((k) as isize)) = absval;
                        } else {
                            (*prev_sgn.offset((k) as isize)) = 0;
                            (*prev_abs.offset((k) as isize)) = 0;
                        }
                        k.prefix_dec();
                    }
                    if !((num_nzeros) <= (kNumNonZeroTreeSize_85)) {
                        (unsafe {
                            BrunsliDumpAndAbort_79(
                                c"brunsli_encode.cc".as_ptr(),
                                1329,
                                c"EncodeAC".as_ptr(),
                            )
                        });
                        'loop_: while true {}
                    };
                    (&mut (*c)).prev_num_nonzeros[(x as usize)] = (num_nzeros as u8);
                    block_state.prefix_inc();
                    coeffs_in = (coeffs_in).wrapping_add(kDCTBlockSize_3 as usize);
                    prev_sgn = (prev_sgn).wrapping_add(kDCTBlockSize_3 as usize);
                    prev_abs = (prev_abs).wrapping_add(kDCTBlockSize_3 as usize);
                    prev_row_coeffs = (prev_row_coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    prev_col_coeffs = (prev_col_coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    x.prefix_inc();
                }
                prev_row_delta *= -1_i32;
                iy.prefix_inc();
                y.prefix_inc();
            }
            i.prefix_inc();
        }
        mcu_y.prefix_inc();
    }
}
pub unsafe fn PrepareEntropyCodes_188(
    mut state: *mut brunsli_internal_enc_State,
) -> Option<Box<brunsli_internal_enc_EntropyCodes>> {
    let meta: *mut Vec<brunsli_internal_enc_ComponentMeta> =
        &mut (*state).meta as *mut Vec<brunsli_internal_enc_ComponentMeta>;
    let num_components: usize = (*meta).len();
    let mut group_context_offsets: Vec<u64> = (0..((1_usize).wrapping_add(num_components))
        as usize)
        .map(|_| <u64>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        group_context_offsets[((i).wrapping_add(1_usize))] =
            ((&mut (*meta))[(i)].context_offset as u64);
        i.prefix_inc();
    }
    return (unsafe {
        (*state)
            .entropy_source
            .Finish(&group_context_offsets as *const Vec<u64>)
    });
}
pub unsafe fn BrunsliSerialize_189(
    mut state: *mut brunsli_internal_enc_State,
    jpg: *const brunsli_JPEGData,
    mut skip_sections: u32,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    let mut pos: usize = 0_usize;
    let mut ok: bool = true;
    if !(((skip_sections) & ((1_u32) << (kBrunsliSignatureTag_30 as i32))) != 0) {
        ok = (unsafe {
            let _len: usize = (*len);
            let _data: *mut u8 = data;
            let _pos: *mut usize = (&mut pos as *mut usize);
            EncodeSignature_170(_len, _data, _pos)
        })
        .clone();
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliHeaderTag_31 as i32))) != 0) {
        ok = (unsafe {
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(kBrunsliHeaderTag_31, Some(EncodeHeader_172), 1_usize)
        });
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliJPEGInternalsTag_33 as i32))) != 0) {
        ok = (unsafe {
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(
                kBrunsliJPEGInternalsTag_33,
                Some(EncodeJPEGInternals_174),
                (unsafe { Base128Size_145((unsafe { EstimateAuxDataSize_143(jpg) })) }),
            )
        });
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliMetaDataTag_32 as i32))) != 0) {
        ok = (unsafe {
            let _tag: u8 = kBrunsliMetaDataTag_32;
            let _fn: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            > = Some(EncodeMetaData_173);
            let _size: usize = (unsafe { Base128Size_145((*len).wrapping_sub(pos)) });
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(_tag, _fn, _size)
        })
        .clone();
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliQuantDataTag_34 as i32))) != 0) {
        ok = (unsafe {
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(kBrunsliQuantDataTag_34, Some(EncodeQuantData_175), 2_usize)
        });
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliHistogramDataTag_35 as i32))) != 0) {
        ok = (unsafe {
            let _tag: u8 = kBrunsliHistogramDataTag_35;
            let _fn: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            > = Some(EncodeHistogramData_176);
            let _size: usize = (unsafe { Base128Size_145((*len).wrapping_sub(pos)) });
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(_tag, _fn, _size)
        })
        .clone();
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliDCDataTag_36 as i32))) != 0) {
        ok = (unsafe {
            let _tag: u8 = kBrunsliDCDataTag_36;
            let _fn: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            > = Some(EncodeDCData_177);
            let _size: usize = (unsafe { Base128Size_145((*len).wrapping_sub(pos)) });
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(_tag, _fn, _size)
        })
        .clone();
        if !ok {
            return false;
        }
    }
    if !(((skip_sections) & ((1_u32) << (kBrunsliACDataTag_37 as i32))) != 0) {
        ok = (unsafe {
            let _tag: u8 = kBrunsliACDataTag_37;
            let _fn: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            > = Some(EncodeACData_178);
            let _size: usize = (unsafe { Base128Size_145((*len).wrapping_sub(pos)) });
            (|tag: u8,
              fn_: Option<
                unsafe fn(
                    *const brunsli_JPEGData,
                    *mut brunsli_internal_enc_State,
                    *mut u8,
                    *mut usize,
                ) -> bool,
            >,
              size: usize| {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _s: *mut brunsli_internal_enc_State = state;
                    let _tag: u8 = tag;
                    let _write_section: Option<
                        unsafe fn(
                            *const brunsli_JPEGData,
                            *mut brunsli_internal_enc_State,
                            *mut u8,
                            *mut usize,
                        ) -> bool,
                    > = fn_;
                    let _section_size_bytes: usize = size;
                    let _len: usize = (*len);
                    let _data: *mut u8 = data;
                    let _pos: *mut usize = (&mut pos as *mut usize);
                    EncodeSection_179(
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
            })(_tag, _fn, _size)
        })
        .clone();
        if !ok {
            return false;
        }
    }
    (*len) = pos;
    return true;
}
pub unsafe fn BrunsliEncodeJpeg_190(
    jpg: *const brunsli_JPEGData,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    let mut state: brunsli_internal_enc_State = <brunsli_internal_enc_State>::default();
    let meta: *mut Vec<brunsli_internal_enc_ComponentMeta> =
        &mut state.meta as *mut Vec<brunsli_internal_enc_ComponentMeta>;
    let mut num_components: usize = (*jpg).components.len();
    state.use_legacy_context_model = !((((*jpg).version) & (2)) != 0);
    if !(unsafe {
        let _jpg: *const brunsli_JPEGData = jpg;
        let _state: *mut brunsli_internal_enc_State =
            (&mut state as *mut brunsli_internal_enc_State);
        CalculateMeta_185(_jpg, _state)
    }) {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        (&mut (*meta))[(i)].approx_total_nonzeros = (unsafe {
            SampleNumNonZeros_180((&mut state.meta[(i)] as *mut brunsli_internal_enc_ComponentMeta))
        });
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        (&mut (*meta))[(i)].context_bits = (unsafe {
            SelectContextBits_182(((&mut (*meta))[(i)].approx_total_nonzeros).wrapping_add(1_usize))
        });
        i.prefix_inc();
    }
    let mut num_contexts: usize = num_components;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        (&mut (*meta))[(i)].context_offset = num_contexts;
        num_contexts = (num_contexts).wrapping_add(
            (kNumNonzeroContextSkip_94[((&mut (*meta))[(i)].context_bits) as usize] as usize),
        );
        i.prefix_inc();
    }
    state.num_contexts = num_contexts;
    let mut dc_prediction_errors: Vec<Vec<i16>> = (0..(num_components) as usize)
        .map(|_| <Vec<i16>>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        {
            let __a0 = ((((&mut (*meta))[(i)].width_in_blocks)
                * ((&mut (*meta))[(i)].height_in_blocks)) as usize) as usize;
            dc_prediction_errors[(i)].resize_with(__a0, || <i16>::default())
        };
        (&mut (*meta))[(i)].dc_prediction_errors = dc_prediction_errors[(i)].as_mut_ptr();
        i.prefix_inc();
    }
    if !(unsafe { PredictDCCoeffs_184((&mut state as *mut brunsli_internal_enc_State)) }) {
        return false;
    }
    let mut block_state: Vec<Vec<u8>> = (0..(num_components) as usize)
        .map(|_| <Vec<u8>>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_components)) {
        {
            let __a0 = ((((&mut (*meta))[(i)].width_in_blocks)
                * ((&mut (*meta))[(i)].height_in_blocks)) as usize) as usize;
            block_state[(i)].resize_with(__a0, || <u8>::default())
        };
        (&mut (*meta))[(i)].block_state = block_state[(i)].as_mut_ptr();
        i.prefix_inc();
    }
    (unsafe { EncodeDC_186((&mut state as *mut brunsli_internal_enc_State)) });
    (unsafe { EncodeAC_187((&mut state as *mut brunsli_internal_enc_State)) });
    let mut entropy_codes: Option<Box<brunsli_internal_enc_EntropyCodes>> =
        (unsafe { PrepareEntropyCodes_188((&mut state as *mut brunsli_internal_enc_State)) });
    state.entropy_codes = entropy_codes
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| {
            v as *mut brunsli_internal_enc_EntropyCodes
        });
    return (unsafe {
        let _state: *mut brunsli_internal_enc_State =
            (&mut state as *mut brunsli_internal_enc_State);
        let _jpg: *const brunsli_JPEGData = jpg;
        let _data: *mut u8 = data;
        let _len: *mut usize = len;
        BrunsliSerialize_189(_state, _jpg, 0_u32, _data, _len)
    });
}
pub static mut kMaxBypassHeaderSize_191: usize = unsafe { (((5) * (6)) as usize) };
pub unsafe fn GetBrunsliBypassSize_192(mut jpg_size: usize) -> usize {
    return ((jpg_size).wrapping_add(kBrunsliSignatureSize_43))
        .wrapping_add(kMaxBypassHeaderSize_191);
}
pub unsafe fn EncodeOriginalJpg_193(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_enc_State,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    &(state);
    if (((*jpg).original_jpg).is_null()) || (((*jpg).original_jpg_size) > (*len)) {
        return false;
    }
    {
        if (*jpg).original_jpg_size != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((*jpg).original_jpg as *const u8 as *const ::libc::c_void),
                (data as *mut u8 as *mut ::libc::c_void),
                (*jpg).original_jpg_size as usize,
            )
        }
        (data as *mut u8 as *mut ::libc::c_void)
    };
    (*len) = (*jpg).original_jpg_size;
    return true;
}
pub unsafe fn BrunsliEncodeJpegBypass_194(
    mut jpg_data: *const u8,
    mut jpg_data_len: usize,
    mut data: *mut u8,
    mut len: *mut usize,
) -> bool {
    let mut pos: usize = 0_usize;
    if !(unsafe {
        let _len: usize = (*len);
        let _data: *mut u8 = data;
        let _pos: *mut usize = (&mut pos as *mut usize);
        EncodeSignature_170(_len, _data, _pos)
    }) {
        return false;
    }
    let mut jpg: brunsli_JPEGData = brunsli_JPEGData::brunsli_JPEGData();
    if !(unsafe {
        ReadJpeg_195(
            jpg_data,
            jpg_data_len,
            brunsli_JpegReadMode::JPEG_READ_HEADER,
            (&mut jpg as *mut brunsli_JPEGData),
        )
    }) {
        jpg.width = 0;
        jpg.height = 0;
        {
            let __a0 = 1_usize as usize;
            jpg.components
                .resize_with(__a0, || <brunsli_JPEGComponent>::default())
        };
        jpg.components[(0_usize)].h_samp_factor = 1;
        jpg.components[(0_usize)].v_samp_factor = 1;
    }
    jpg.version = kFallbackVersion_2;
    jpg.original_jpg = jpg_data;
    jpg.original_jpg_size = jpg_data_len;
    let mut state: brunsli_internal_enc_State = <brunsli_internal_enc_State>::default();
    if !(unsafe {
        let _jpg: *const brunsli_JPEGData = &jpg as *const brunsli_JPEGData;
        let _s: *mut brunsli_internal_enc_State = (&mut state as *mut brunsli_internal_enc_State);
        let _tag: u8 = kBrunsliHeaderTag_31;
        let _write_section: Option<
            unsafe fn(
                *const brunsli_JPEGData,
                *mut brunsli_internal_enc_State,
                *mut u8,
                *mut usize,
            ) -> bool,
        > = Some(EncodeHeader_172);
        let _len: usize = (*len);
        let _data: *mut u8 = data;
        let _pos: *mut usize = (&mut pos as *mut usize);
        EncodeSection_179(_jpg, _s, _tag, _write_section, 1_usize, _len, _data, _pos)
    }) {
        return false;
    }
    if !(unsafe {
        let _jpg: *const brunsli_JPEGData = &jpg as *const brunsli_JPEGData;
        let _s: *mut brunsli_internal_enc_State = (&mut state as *mut brunsli_internal_enc_State);
        let _tag: u8 = kBrunsliOriginalJpgTag_38;
        let _write_section: Option<
            unsafe fn(
                *const brunsli_JPEGData,
                *mut brunsli_internal_enc_State,
                *mut u8,
                *mut usize,
            ) -> bool,
        > = Some(EncodeOriginalJpg_193);
        let _section_size_bytes: usize = (unsafe { Base128Size_145(jpg_data_len) });
        let _len: usize = (*len);
        let _data: *mut u8 = data;
        let _pos: *mut usize = (&mut pos as *mut usize);
        EncodeSection_179(
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
    (*len) = pos;
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_HuffmanTree {
    pub total_count: u32,
    pub index_left: i16,
    pub index_right_or_value: i16,
}
impl brunsli_HuffmanTree {
    pub unsafe fn brunsli_HuffmanTree(mut count: u32, mut left: i16, mut right: i16) -> Self {
        let mut this = Self {
            total_count: count,
            index_left: left,
            index_right_or_value: right,
        };
        this
    }
}
pub unsafe fn StoreVarLenUint8_196(mut n: usize, mut storage: *mut brunsli_Storage) {
    if ((n) == (0_usize)) {
        (unsafe { WriteBits_120(1_usize, 0_u64, storage) });
    } else {
        (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
        let mut nbits: usize = ((unsafe { Log2FloorNonZero_74((n as u32)) }) as usize);
        (unsafe { WriteBits_120(3_usize, (nbits as u64), storage) });
        (unsafe {
            let _n_bits: usize = nbits;
            let _bits: u64 = ((n).wrapping_sub(((1_usize) << (nbits))) as u64);
            WriteBits_120(_n_bits, _bits, storage)
        });
    }
}
pub unsafe fn IndexOf_197(v: *const Vec<u32>, mut value: u32) -> usize {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v).len())) {
        if (((&(*v))[(i)]) == (value)) {
            return i;
        }
        i.prefix_inc();
    }
    return i;
}
pub unsafe fn MoveToFront_198(mut v: *mut Vec<u32>, mut index: usize) {
    let mut value: u32 = (&mut (*v))[(index)];
    let mut i: usize = index;
    'loop_: while ((i) != (0_usize)) {
        (&mut (*v))[(i)] = (&mut (*v))[((i).wrapping_sub(1_usize))];
        i.prefix_dec();
    }
    (&mut (*v))[(0_usize)] = value;
}
pub unsafe fn MoveToFrontTransform_199(v: *const Vec<u32>) -> Vec<u32> {
    if (*v).is_empty() {
        return (*v).clone();
    }
    let mut max_value: u32 = (*core::slice::from_raw_parts(
        (*v).as_ptr(),
        ((*v).as_ptr().add((*v).len())).offset_from((*v).as_ptr()) as usize,
    )
    .iter()
    .max()
    .unwrap());
    let mut mtf: Vec<u32> = (0..(((max_value).wrapping_add(1_u32)) as usize) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (max_value)) {
        mtf[(i as usize)] = i;
        i.prefix_inc();
    }
    let mut result: Vec<u32> = (0..((*v).len()) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v).len())) {
        let mut index: usize = (unsafe {
            let _v: *const Vec<u32> = &mtf as *const Vec<u32>;
            let _value: u32 = (&(*v))[(i)];
            IndexOf_197(_v, _value)
        });
        if !((index) < (mtf.len())) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"context_map_encode.cc".as_ptr(),
                    60,
                    c"MoveToFrontTransform".as_ptr(),
                )
            });
            'loop_: while true {}
        };
        result[(i)] = (index as u32);
        (unsafe { MoveToFront_198((&mut mtf as *mut Vec<u32>), index) });
        i.prefix_inc();
    }
    return result;
}
pub unsafe fn RunLengthCodeZeros_200(
    v_in: *const Vec<u32>,
    mut max_run_length_prefix: *mut u32,
    mut v_out: *mut Vec<u32>,
    mut extra_bits: *mut Vec<u32>,
) {
    let mut max_reps: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v_in).len())) {
        'loop_: while ((i) < ((*v_in).len())) && (((&(*v_in))[(i)]) != (0_u32)) {
            i.prefix_inc();
        }
        let mut i0: usize = i;
        'loop_: while ((i) < ((*v_in).len())) && (((&(*v_in))[(i)]) == (0_u32)) {
            i.prefix_inc();
        }
        max_reps = ({
            let mut __tmp_0 = ((i).wrapping_sub(i0) as u64);
            let mut __tmp_1 = (max_reps as u64);
            (*if *&mut __tmp_0 >= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        } as usize);
    }
    let mut max_prefix: u32 = (if ((max_reps) > (0_usize)) {
        (unsafe { Log2FloorNonZero_74((max_reps as u32)) })
    } else {
        0
    } as u32);
    max_prefix = (*if *&mut max_prefix <= *max_run_length_prefix {
        (&mut max_prefix) as *const _
    } else {
        (max_run_length_prefix) as *const _
    });
    (*max_run_length_prefix) = max_prefix;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v_in).len())) {
        if (((&(*v_in))[(i)]) != (0_u32)) {
            (*v_out).push(((&(*v_in))[(i)]).wrapping_add((*max_run_length_prefix)));
            (*extra_bits).push(0_u32);
            i.prefix_inc();
        } else {
            let mut reps: u32 = 1_u32;
            let mut k: usize = (i).wrapping_add(1_usize);
            'loop_: while ((k) < ((*v_in).len())) && (((&(*v_in))[(k)]) == (0_u32)) {
                reps.prefix_inc();
                k.prefix_inc();
            }
            i = (i).wrapping_add((reps as usize));
            'loop_: while ((reps) != (0_u32)) {
                if ((reps) < ((2_u32) << (max_prefix))) {
                    let mut run_length_prefix: u32 =
                        ((unsafe { Log2FloorNonZero_74(reps) }) as u32);
                    {
                        let a0_clone = run_length_prefix.clone();
                        (*v_out).push(a0_clone)
                    };
                    (*extra_bits).push((reps).wrapping_sub(((1_u32) << (run_length_prefix))));
                    break;
                } else {
                    {
                        let a0_clone = max_prefix.clone();
                        (*v_out).push(a0_clone)
                    };
                    (*extra_bits).push(((1_u32) << (max_prefix)).wrapping_sub((1_u32 as u32)));
                    reps =
                        (reps).wrapping_sub(((2_u32) << (max_prefix)).wrapping_sub((1_u32 as u32)));
                }
            }
        };
    }
}
pub unsafe fn EncodeContextMap_163(
    context_map: *const Vec<u32>,
    mut num_clusters: usize,
    mut storage: *mut brunsli_Storage,
) {
    (unsafe { StoreVarLenUint8_196((num_clusters).wrapping_sub(1_usize), storage) });
    if ((num_clusters) == (1_usize)) {
        return;
    }
    let mut transformed_symbols: Vec<u32> = (unsafe { MoveToFrontTransform_199(context_map) });
    let mut rle_symbols: Vec<u32> = Vec::new();
    let mut extra_bits: Vec<u32> = Vec::new();
    let mut max_run_length_prefix: u32 = 6_u32;
    (unsafe {
        RunLengthCodeZeros_200(
            &transformed_symbols as *const Vec<u32>,
            (&mut max_run_length_prefix as *mut u32),
            (&mut rle_symbols as *mut Vec<u32>),
            (&mut extra_bits as *mut Vec<u32>),
        )
    });
    let mut symbol_histogram: [u32; 272] = [0_u32; 272];
    {
        let byte_0 = (symbol_histogram.as_mut_ptr() as *mut u32 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u32; 272]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (symbol_histogram.as_mut_ptr() as *mut u32 as *mut ::libc::c_void)
    };
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (rle_symbols.len())) {
        symbol_histogram[(rle_symbols[(i)]) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut use_rle: bool = ((max_run_length_prefix) > (0_u32));
    (unsafe { WriteBits_120(1_usize, (use_rle as u64), storage) });
    if use_rle {
        (unsafe {
            WriteBits_120(
                4_usize,
                (((max_run_length_prefix).wrapping_sub(1_u32)) as u64),
                storage,
            )
        });
    }
    let mut bit_depths: [u8; 272] = [0_u8; 272];
    let mut bit_codes: [u16; 272] = [0_u16; 272];
    {
        let byte_0 = (bit_depths.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u8; 272]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (bit_depths.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
    };
    {
        let byte_0 = (bit_codes.as_mut_ptr() as *mut u16 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u16; 272]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (bit_codes.as_mut_ptr() as *mut u16 as *mut ::libc::c_void)
    };
    (unsafe {
        BuildAndStoreHuffmanTree_201(
            (symbol_histogram.as_mut_ptr()).cast_const(),
            (num_clusters).wrapping_add((max_run_length_prefix as usize)),
            bit_depths.as_mut_ptr(),
            bit_codes.as_mut_ptr(),
            storage,
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (rle_symbols.len())) {
        (unsafe {
            let _n_bits: usize = (bit_depths[(rle_symbols[(i)]) as usize] as usize);
            let _bits: u64 = (bit_codes[(rle_symbols[(i)]) as usize] as u64);
            WriteBits_120(_n_bits, _bits, storage)
        });
        if ((rle_symbols[(i)]) > (0_u32)) && ((rle_symbols[(i)]) <= (max_run_length_prefix)) {
            (unsafe {
                let _n_bits: usize = (rle_symbols[(i)] as usize);
                let _bits: u64 = (extra_bits[(i)] as u64);
                WriteBits_120(_n_bits, _bits, storage)
            });
        }
        i.prefix_inc();
    }
    (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
}
pub unsafe fn GetPopulationCountPrecision_202(mut logcount: u32) -> u32 {
    return (((logcount).wrapping_add(1_u32)) >> (1));
}
pub static mut kHistogramLengthBitLengths_203: [u8; 16] = unsafe {
    [
        8_u8, 8_u8, 6_u8, 6_u8, 6_u8, 5_u8, 4_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 4_u8, 5_u8,
        7_u8,
    ]
};
pub static mut kHistogramLengthSymbols_204: [u16; 16] = unsafe {
    [
        127_u16, 255_u16, 15_u16, 47_u16, 31_u16, 7_u16, 3_u16, 0_u16, 4_u16, 2_u16, 6_u16, 1_u16,
        5_u16, 11_u16, 23_u16, 63_u16,
    ]
};
pub static mut kLogCountBitLengths_205: [u8; 11] = unsafe {
    [
        5_u8, 4_u8, 4_u8, 4_u8, 3_u8, 3_u8, 2_u8, 3_u8, 3_u8, 6_u8, 6_u8,
    ]
};
pub static mut kLogCountSymbols_206: [u16; 11] = unsafe {
    [
        15_u16, 3_u16, 11_u16, 7_u16, 2_u16, 6_u16, 0_u16, 1_u16, 5_u16, 31_u16, 63_u16,
    ]
};
pub unsafe fn SmallestIncrement_207(mut count: i32) -> i32 {
    if !((count) > (0)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                39,
                c"SmallestIncrement".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let mut bits: i32 = (unsafe { Log2FloorNonZero_74((count as u32)) });
    let mut drop_bits: i32 = (((bits as u32)
        .wrapping_sub((unsafe { GetPopulationCountPrecision_202((bits as u32)) })))
        as i32);
    return ((1) << (drop_bits));
}
pub unsafe fn RebalanceHistogram_208(
    mut targets: *const f32,
    mut max_symbol: i32,
    mut table_size: i32,
    mut omit_pos: *mut i32,
    mut counts: *mut i32,
) -> bool {
    if !((table_size) >= (2)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                48,
                c"RebalanceHistogram".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let mut sum: i32 = 0;
    let mut sum_nonrounded: f32 = (0.0E+0 as f32);
    let mut remainder_pos: i32 = -1_i32;
    let mut remainder_log: i32 = -1_i32;
    let mut n: i32 = 0;
    'loop_: while ((n) < (max_symbol)) {
        if ((*targets.offset((n) as isize)) > (0_f32)) {
            sum_nonrounded += (*targets.offset((n) as isize));
            (*counts.offset((n) as isize)) =
                (((((*targets.offset((n) as isize)) as f64) + (5.0E-1)) as u32) as i32);
            if ((*counts.offset((n) as isize)) == (0)) {
                (*counts.offset((n) as isize)) = 1;
            }
            if ((*counts.offset((n) as isize)) == (table_size)) {
                (*counts.offset((n) as isize)) = ((table_size) - (1));
            }
            let mut inc: i32 = (unsafe { SmallestIncrement_207((*counts.offset((n) as isize))) });
            (*counts.offset((n) as isize)) -= ((*counts.offset((n) as isize)) & ((inc) - (1)));
            let target: f32 = if false {
                ((sum_nonrounded) - (sum as f32))
            } else {
                (*targets.offset((n) as isize))
            };
            if ((*counts.offset((n) as isize)) == (0))
                || (((target) > (((*counts.offset((n) as isize)) + ((inc) / (2))) as f32))
                    && (((*counts.offset((n) as isize)) + (inc)) < (table_size)))
            {
                (*counts.offset((n) as isize)) += inc;
            }
            sum += (*counts.offset((n) as isize));
            let count_log: i32 =
                (unsafe { Log2FloorNonZero_74(((*counts.offset((n) as isize)) as u32)) });
            if ((count_log) > (remainder_log)) {
                remainder_pos = n;
                remainder_log = count_log;
            }
        }
        n.prefix_inc();
    }
    if !((remainder_pos) != (-1_i32)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                81,
                c"RebalanceHistogram".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    (*counts.offset((remainder_pos) as isize)) -= ((sum) - (table_size));
    (*omit_pos) = remainder_pos;
    return ((*counts.offset((remainder_pos) as isize)) > (0));
}
pub unsafe fn RebalanceHistogram_209(
    mut targets: *const f32,
    mut max_symbol: i32,
    mut table_size: i32,
    mut omit_pos: *mut i32,
    mut counts: *mut i32,
) -> bool {
    if !((table_size) >= (2)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                48,
                c"RebalanceHistogram".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let mut sum: i32 = 0;
    let mut sum_nonrounded: f32 = (0.0E+0 as f32);
    let mut remainder_pos: i32 = -1_i32;
    let mut remainder_log: i32 = -1_i32;
    let mut n: i32 = 0;
    'loop_: while ((n) < (max_symbol)) {
        if ((*targets.offset((n) as isize)) > (0_f32)) {
            sum_nonrounded += (*targets.offset((n) as isize));
            (*counts.offset((n) as isize)) =
                (((((*targets.offset((n) as isize)) as f64) + (5.0E-1)) as u32) as i32);
            if ((*counts.offset((n) as isize)) == (0)) {
                (*counts.offset((n) as isize)) = 1;
            }
            if ((*counts.offset((n) as isize)) == (table_size)) {
                (*counts.offset((n) as isize)) = ((table_size) - (1));
            }
            let mut inc: i32 = (unsafe { SmallestIncrement_207((*counts.offset((n) as isize))) });
            (*counts.offset((n) as isize)) -= ((*counts.offset((n) as isize)) & ((inc) - (1)));
            let target: f32 = if true {
                ((sum_nonrounded) - (sum as f32))
            } else {
                (*targets.offset((n) as isize))
            };
            if ((*counts.offset((n) as isize)) == (0))
                || (((target) > (((*counts.offset((n) as isize)) + ((inc) / (2))) as f32))
                    && (((*counts.offset((n) as isize)) + (inc)) < (table_size)))
            {
                (*counts.offset((n) as isize)) += inc;
            }
            sum += (*counts.offset((n) as isize));
            let count_log: i32 =
                (unsafe { Log2FloorNonZero_74(((*counts.offset((n) as isize)) as u32)) });
            if ((count_log) > (remainder_log)) {
                remainder_pos = n;
                remainder_log = count_log;
            }
        }
        n.prefix_inc();
    }
    if !((remainder_pos) != (-1_i32)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                81,
                c"RebalanceHistogram".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    (*counts.offset((remainder_pos) as isize)) -= ((sum) - (table_size));
    (*omit_pos) = remainder_pos;
    return ((*counts.offset((remainder_pos) as isize)) > (0));
}
pub unsafe fn NormalizeCounts_124(
    mut counts: *mut i32,
    mut omit_pos: *mut i32,
    length: i32,
    precision_bits: i32,
    mut num_symbols: *mut i32,
    mut symbols: *mut i32,
) {
    if !((precision_bits) > (0)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                89,
                c"NormalizeCounts".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let table_size: i32 = ((1) << (precision_bits));
    let mut total: u64 = 0_u64;
    let mut max_symbol: i32 = 0;
    let mut symbol_count: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while ((n) < (length)) {
        total = (total).wrapping_add(((*counts.offset((n) as isize)) as u64));
        if ((*counts.offset((n) as isize)) > (0)) {
            if ((symbol_count) < (kMaxNumSymbolsForSmallCode_121)) {
                (*symbols.offset((symbol_count) as isize)) = n;
            }
            symbol_count.prefix_inc();
            max_symbol = ((n) + (1));
        }
        n.prefix_inc();
    }
    (*num_symbols) = symbol_count;
    if ((symbol_count) == (0)) {
        return;
    }
    if ((symbol_count) == (1)) {
        (*counts.offset((*symbols.offset((0) as isize)) as isize)) = table_size;
        return;
    }
    if !((symbol_count) <= (table_size)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"histogram_encode.cc".as_ptr(),
                112,
                c"NormalizeCounts".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    let norm: f32 = (((1.0E+0) * (table_size as f32)) / (total as f32));
    let mut targets: [f32; 18] = [0.0_f32; 18];
    let mut n: i32 = 0;
    'loop_: while ((n) < (max_symbol)) {
        targets[(n) as usize] = ((norm) * ((*counts.offset((n) as isize)) as f32));
        n.prefix_inc();
    }
    if !(unsafe {
        let _max_symbol: i32 = max_symbol;
        let _table_size: i32 = table_size;
        let _omit_pos: *mut i32 = omit_pos;
        let _counts: *mut i32 = counts;
        RebalanceHistogram_208(
            (targets.as_mut_ptr()).cast_const(),
            _max_symbol,
            _table_size,
            _omit_pos,
            _counts,
        )
    }) {
        if !(unsafe {
            let _max_symbol: i32 = max_symbol;
            let _table_size: i32 = table_size;
            let _omit_pos: *mut i32 = omit_pos;
            let _counts: *mut i32 = counts;
            RebalanceHistogram_209(
                (targets.as_mut_ptr()).cast_const(),
                _max_symbol,
                _table_size,
                _omit_pos,
                _counts,
            )
        }) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"histogram_encode.cc".as_ptr(),
                    126,
                    c"NormalizeCounts".as_ptr(),
                )
            });
            'loop_: while true {}
        };
    }
}
pub unsafe fn EncodeCounts_125(
    mut counts: *const i32,
    omit_pos: i32,
    num_symbols: i32,
    mut symbols: *const i32,
    mut storage: *mut brunsli_Storage,
) {
    let mut max_bits: i32 = 5;
    if ((num_symbols) <= (2)) {
        (unsafe { WriteBits_120(1_usize, 1_u64, storage) });
        if ((num_symbols) == (0)) {
            (unsafe { WriteBits_120((((max_bits) + (1)) as usize), 0_u64, storage) });
        } else {
            (unsafe { WriteBits_120(1_usize, (((num_symbols) - (1)) as u64), storage) });
            let mut i: i32 = 0;
            'loop_: while ((i) < (num_symbols)) {
                (unsafe {
                    WriteBits_120(
                        (max_bits as usize),
                        ((*symbols.offset((i) as isize)) as u64),
                        storage,
                    )
                });
                i.prefix_inc();
            }
        }
        if ((num_symbols) == (2)) {
            (unsafe {
                WriteBits_120(
                    (BRUNSLI_ANS_LOG_TAB_SIZE_0 as usize),
                    ((*counts.offset((*symbols.offset((0) as isize)) as isize)) as u64),
                    storage,
                )
            });
        }
    } else {
        (unsafe { WriteBits_120(1_usize, 0_u64, storage) });
        let mut length: i32 = 0;
        let mut logcounts: [i32; 18] = [
            0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ];
        let mut omit_log: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (18)) {
            if !((*counts.offset((i) as isize)) <= (BRUNSLI_ANS_TAB_SIZE_1)) {
                (unsafe {
                    BrunsliDumpAndAbort_79(
                        c"histogram_encode.cc".as_ptr(),
                        155,
                        c"EncodeCounts".as_ptr(),
                    )
                });
                'loop_: while true {}
            };
            if !((*counts.offset((i) as isize)) >= (0)) {
                (unsafe {
                    BrunsliDumpAndAbort_79(
                        c"histogram_encode.cc".as_ptr(),
                        156,
                        c"EncodeCounts".as_ptr(),
                    )
                });
                'loop_: while true {}
            };
            if ((i) == (omit_pos)) {
                length = ((i) + (1));
            } else if ((*counts.offset((i) as isize)) > (0)) {
                logcounts[(i) as usize] =
                    ((unsafe { Log2FloorNonZero_74(((*counts.offset((i) as isize)) as u32)) })
                        + (1));
                length = ((i) + (1));
                if ((i) < (omit_pos)) {
                    omit_log = {
                        let mut __tmp_1 = ((logcounts[(i) as usize]) + (1));
                        (*if *&mut omit_log >= *&mut __tmp_1 {
                            (&mut omit_log) as *const _
                        } else {
                            (&mut __tmp_1) as *const _
                        })
                    };
                } else {
                    omit_log = (*if *&mut omit_log >= *&mut logcounts[(i) as usize] {
                        (&mut omit_log) as *const _
                    } else {
                        (&mut logcounts[(i) as usize]) as *const _
                    });
                }
            }
            i.prefix_inc();
        }
        logcounts[(omit_pos) as usize] = omit_log;
        (unsafe {
            let _n_bits: usize =
                (kHistogramLengthBitLengths_203[((length) - (3)) as usize] as usize);
            let _bits: u64 = (kHistogramLengthSymbols_204[((length) - (3)) as usize] as u64);
            WriteBits_120(_n_bits, _bits, storage)
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (length)) {
            (unsafe {
                let _n_bits: usize =
                    (kLogCountBitLengths_205[(logcounts[(i) as usize]) as usize] as usize);
                let _bits: u64 = (kLogCountSymbols_206[(logcounts[(i) as usize]) as usize] as u64);
                WriteBits_120(_n_bits, _bits, storage)
            });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (length)) {
            if ((logcounts[(i) as usize]) > (1)) && ((i) != (omit_pos)) {
                let mut bitcount: i32 = ((unsafe {
                    GetPopulationCountPrecision_202((((logcounts[(i) as usize]) - (1)) as u32))
                }) as i32);
                let mut drop_bits: i32 = (((logcounts[(i) as usize]) - (1)) - (bitcount));
                if !(((*counts.offset((i) as isize)) & (((1) << (drop_bits)) - (1))) == (0)) {
                    (unsafe {
                        BrunsliDumpAndAbort_79(
                            c"histogram_encode.cc".as_ptr(),
                            184,
                            c"EncodeCounts".as_ptr(),
                        )
                    });
                    'loop_: while true {}
                };
                (unsafe {
                    let _n_bits: usize = (bitcount as usize);
                    let _bits: u64 = ((((*counts.offset((i) as isize)) >> (drop_bits))
                        - ((1) << (bitcount))) as u64);
                    WriteBits_120(_n_bits, _bits, storage)
                });
            }
            i.prefix_inc();
        }
    }
}
pub unsafe fn PopulationCost_130(mut data: *const i32, mut total_count: i32) -> f64 {
    if ((total_count) == (0)) {
        return 7_f64;
    }
    let mut entropy_bits: f64 = (((total_count) * (BRUNSLI_ANS_LOG_TAB_SIZE_0)) as f64);
    let mut histogram_bits: i32 = 0;
    let mut count: i32 = 0;
    let mut length: i32 = 0;
    if ((total_count) > (BRUNSLI_ANS_TAB_SIZE_1)) {
        let mut total: u64 = (total_count as u64);
        let mut i: i32 = 0;
        'loop_: while ((i) < (18)) {
            if ((*data.offset((i) as isize)) > (0)) {
                count.prefix_inc();
                length = i;
            }
            i.prefix_inc();
        }
        if ((count) == (1)) {
            return 7_f64;
        }
        length.prefix_inc();
        let max0: u64 =
            (((total).wrapping_mul((length as u64))) >> (BRUNSLI_ANS_LOG_TAB_SIZE_0 as u64));
        let max1: u64 =
            (((max0).wrapping_mul((length as u64))) >> (BRUNSLI_ANS_LOG_TAB_SIZE_0 as u64));
        let min_base: u64 = ((((total).wrapping_add(max0)).wrapping_add(max1))
            >> (BRUNSLI_ANS_LOG_TAB_SIZE_0 as u64));
        total = (total).wrapping_add((min_base).wrapping_mul((count as u64)));
        let kFixBits: i64 = 32_i64;
        let kFixOne: i64 = ((1_i64) << (kFixBits));
        let kDescaleBits: i64 = ((kFixBits) - (BRUNSLI_ANS_LOG_TAB_SIZE_0 as i64));
        let kDescaleOne: i64 = ((1_i64) << (kDescaleBits));
        let kDescaleMask: i64 = ((kDescaleOne) - (1_i64));
        let mult: u32 = (((kFixOne as u64).wrapping_div(total)) as u32);
        let error: u32 = (((kFixOne as u64).wrapping_rem(total)) as u32);
        let mut cumul: u32 = error;
        if ((error as i64) < (kDescaleOne)) {
            cumul = ((cumul as i64) + (((kDescaleOne) - (error as i64)) >> (1))) as u32;
        }
        if ((*data.offset((0) as isize)) > (0)) {
            let mut c: u64 = ((((*data.offset((0) as isize)) as u64).wrapping_add(min_base))
                .wrapping_mul((mult as u64)))
            .wrapping_add((cumul as u64));
            let mut c_descaled: u64 = ((c) >> (kDescaleBits));
            if !((c_descaled) < ((1_u64) << (31))) {
                (unsafe {
                    BrunsliDumpAndAbort_79(
                        c"histogram_encode.cc".as_ptr(),
                        236,
                        c"PopulationCost".as_ptr(),
                    )
                });
                'loop_: while true {}
            };
            let mut log2count: f64 = (unsafe { FastLog2_127((c_descaled as i32)) });
            entropy_bits -= (((*data.offset((0) as isize)) as f64) * (log2count));
            cumul = (((c) & (kDescaleMask as u64)) as u32);
        }
        let mut i: i32 = 1;
        'loop_: while ((i) < (length)) {
            if ((*data.offset((i) as isize)) > (0)) {
                let mut c: u64 = ((((*data.offset((i) as isize)) as u64).wrapping_add(min_base))
                    .wrapping_mul((mult as u64)))
                .wrapping_add((cumul as u64));
                let mut c_descaled: u64 = ((c) >> (kDescaleBits));
                if !((c_descaled) < ((1_u64) << (31))) {
                    (unsafe {
                        BrunsliDumpAndAbort_79(
                            c"histogram_encode.cc".as_ptr(),
                            245,
                            c"PopulationCost".as_ptr(),
                        )
                    });
                    'loop_: while true {}
                };
                let mut log2count: f64 = (unsafe { FastLog2_127((c_descaled as i32)) });
                let mut log2floor: i32 = (log2count as i32);
                entropy_bits -= (((*data.offset((i) as isize)) as f64) * (log2count));
                histogram_bits += log2floor;
                histogram_bits += (kLogCountBitLengths_205[((log2floor) + (1)) as usize] as i32);
                cumul = (((c) & (kDescaleMask as u64)) as u32);
            } else {
                histogram_bits += (kLogCountBitLengths_205[(0) as usize] as i32);
            }
            i.prefix_inc();
        }
    } else {
        let mut log2norm: f64 =
            ((BRUNSLI_ANS_LOG_TAB_SIZE_0 as f64) - (unsafe { FastLog2_127(total_count) }));
        if ((*data.offset((0) as isize)) > (0)) {
            let mut log2count: f64 =
                ((unsafe { FastLog2_127((*data.offset((0) as isize))) }) + (log2norm));
            entropy_bits -= (((*data.offset((0) as isize)) as f64) * (log2count));
            length = 0;
            count.prefix_inc();
        }
        let mut i: i32 = 1;
        'loop_: while ((i) < (18)) {
            if ((*data.offset((i) as isize)) > (0)) {
                let mut log2count: f64 =
                    ((unsafe { FastLog2_127((*data.offset((i) as isize))) }) + (log2norm));
                let mut log2floor: i32 = (log2count as i32);
                entropy_bits -= (((*data.offset((i) as isize)) as f64) * (log2count));
                if ((log2floor) >= (BRUNSLI_ANS_LOG_TAB_SIZE_0)) {
                    log2floor = ((BRUNSLI_ANS_LOG_TAB_SIZE_0) - (1));
                }
                histogram_bits = ((histogram_bits as u32)
                    .wrapping_add((unsafe { GetPopulationCountPrecision_202((log2floor as u32)) })))
                    as i32;
                histogram_bits += (kLogCountBitLengths_205[((log2floor) + (1)) as usize] as i32);
                length = i;
                count.prefix_inc();
            } else {
                histogram_bits += (kLogCountBitLengths_205[(0) as usize] as i32);
            }
            i.prefix_inc();
        }
        length.prefix_inc();
    }
    if ((count) == (1)) {
        return 7_f64;
    }
    if ((count) == (2)) {
        return (((((entropy_bits as i32) + (1)) + (12)) + (BRUNSLI_ANS_LOG_TAB_SIZE_0)) as f64);
    }
    histogram_bits += (kHistogramLengthBitLengths_203[((length) - (3)) as usize] as i32);
    return ((((histogram_bits) + (entropy_bits as i32)) + (1)) as f64);
}
pub static mut kCodeLengthCodes_210: i32 = unsafe { 18 };
pub unsafe fn StoreHuffmanTreeOfHuffmanTreeToBitMask_211(
    num_codes: i32,
    mut code_length_bitdepth: *const u8,
    mut storage: *mut brunsli_Storage,
) {
    static mut kStorageOrder_212: [u8; 18] = unsafe {
        [
            1_u8, 2_u8, 3_u8, 4_u8, 0_u8, 5_u8, 17_u8, 6_u8, 16_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
            12_u8, 13_u8, 14_u8, 15_u8,
        ]
    };;
    static mut kHuffmanBitLengthHuffmanCodeSymbols_213: [u8; 6] =
        unsafe { [0_u8, 7_u8, 3_u8, 2_u8, 1_u8, 15_u8] };;
    static mut kHuffmanBitLengthHuffmanCodeBitLengths_214: [u8; 6] =
        unsafe { [2_u8, 4_u8, 3_u8, 2_u8, 2_u8, 4_u8] };;
    let mut codes_to_store: usize = (kCodeLengthCodes_210 as usize);
    if ((num_codes) > (1)) {
        'loop_: while ((codes_to_store) > (0_usize)) {
            if (((*code_length_bitdepth
                .offset((kStorageOrder_212[((codes_to_store).wrapping_sub(1_usize))]) as isize))
                as i32)
                != (0))
            {
                break;
            }
            codes_to_store.prefix_dec();
        }
    }
    let mut skip_some: usize = 0_usize;
    if (((*code_length_bitdepth.offset((kStorageOrder_212[(0) as usize]) as isize)) as i32) == (0))
        && (((*code_length_bitdepth.offset((kStorageOrder_212[(1) as usize]) as isize)) as i32)
            == (0))
    {
        skip_some = 2_usize;
        if (((*code_length_bitdepth.offset((kStorageOrder_212[(2) as usize]) as isize)) as i32)
            == (0))
        {
            skip_some = 3_usize;
        }
    }
    (unsafe { WriteBits_120(2_usize, (skip_some as u64), storage) });
    let mut i: usize = skip_some;
    'loop_: while ((i) < (codes_to_store)) {
        let mut l: usize =
            ((*code_length_bitdepth.offset((kStorageOrder_212[(i)]) as isize)) as usize);
        (unsafe {
            let _n_bits: usize = (kHuffmanBitLengthHuffmanCodeBitLengths_214[(l)] as usize);
            let _bits: u64 = (kHuffmanBitLengthHuffmanCodeSymbols_213[(l)] as u64);
            WriteBits_120(_n_bits, _bits, storage)
        });
        i.prefix_inc();
    }
}
pub unsafe fn StoreHuffmanTreeToBitMask_215(
    huffman_tree_size: usize,
    mut huffman_tree: *const u8,
    mut huffman_tree_extra_bits: *const u8,
    mut code_length_bitdepth: *const u8,
    mut code_length_bitdepth_symbols: *const u16,
    mut storage: *mut brunsli_Storage,
) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (huffman_tree_size)) {
        let mut ix: usize = ((*huffman_tree.offset((i) as isize)) as usize);
        (unsafe {
            let _n_bits: usize = ((*code_length_bitdepth.offset((ix) as isize)) as usize);
            let _bits: u64 = ((*code_length_bitdepth_symbols.offset((ix) as isize)) as u64);
            WriteBits_120(_n_bits, _bits, storage)
        });
        'switch: {
            let __match_cond = ix;
            match __match_cond {
                __v if __v == 16_usize => {
                    (unsafe {
                        WriteBits_120(
                            2_usize,
                            ((*huffman_tree_extra_bits.offset((i) as isize)) as u64),
                            storage,
                        )
                    });
                    break 'switch;
                }
                __v if __v == 17_usize => {
                    (unsafe {
                        WriteBits_120(
                            3_usize,
                            ((*huffman_tree_extra_bits.offset((i) as isize)) as u64),
                            storage,
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
pub unsafe fn StoreSimpleHuffmanTree_216(
    mut depths: *const u8,
    mut symbols: *mut usize,
    mut num_symbols: usize,
    mut max_bits: usize,
    mut storage: *mut brunsli_Storage,
) {
    (unsafe { WriteBits_120(2_usize, 1_u64, storage) });
    (unsafe {
        WriteBits_120(
            2_usize,
            ((num_symbols).wrapping_sub(1_usize) as u64),
            storage,
        )
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_symbols)) {
        let mut j: usize = (i).wrapping_add(1_usize);
        'loop_: while ((j) < (num_symbols)) {
            if (((*depths.offset((*symbols.offset((j) as isize)) as isize)) as i32)
                < ((*depths.offset((*symbols.offset((i) as isize)) as isize)) as i32))
            {
                std::mem::swap(
                    &mut (*symbols.offset((j) as isize)),
                    &mut (*symbols.offset((i) as isize)),
                );
            }
            j.postfix_inc();
        }
        i.postfix_inc();
    }
    if ((num_symbols) == (2_usize)) {
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((0) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((1) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else if ((num_symbols) == (3_usize)) {
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((0) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((1) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((2) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else {
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((0) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((1) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((2) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _n_bits: usize = max_bits;
            let _bits: u64 = ((*symbols.offset((3) as isize)) as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (unsafe {
            let _bits: u64 =
                (if (((*depths.offset((*symbols.offset((0) as isize)) as isize)) as i32) == (1)) {
                    1
                } else {
                    0
                } as u64);
            let _storage: *mut brunsli_Storage = storage;
            WriteBits_120(1_usize, _bits, _storage)
        });
    }
}
pub unsafe fn StoreHuffmanTree_217(
    mut depths: *const u8,
    mut num: usize,
    mut storage: *mut brunsli_Storage,
) {
    let mut arena: Option<Box<[u8]>> = Some(Box::from_raw(Box::leak(
        (0..(2_usize).wrapping_mul(num))
            .map(|_| 0_u8)
            .collect::<Box<[u8]>>(),
    )));
    let mut huffman_tree: *mut u8 = arena
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    let mut huffman_tree_extra_bits: *mut u8 = arena
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
        .offset((num) as isize);
    let mut huffman_tree_size: usize = 0_usize;
    (unsafe {
        WriteHuffmanTree_218(
            depths,
            num,
            (&mut huffman_tree_size as *mut usize),
            huffman_tree,
            huffman_tree_extra_bits,
        )
    });
    let mut huffman_tree_histogram: [u32; 18] = [
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ];
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (huffman_tree_size)) {
        huffman_tree_histogram[(*huffman_tree.offset((i) as isize)) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut num_codes: i32 = 0;
    let mut code: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (kCodeLengthCodes_210)) {
        if (huffman_tree_histogram[(i) as usize] != 0) {
            if ((num_codes) == (0)) {
                code = i;
                num_codes = 1;
            } else if ((num_codes) == (1)) {
                num_codes = 2;
                break;
            }
        }
        i.prefix_inc();
    }
    let mut code_length_bitdepth: [u8; 18] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8, 0_u8, 0_u8,
    ];
    let mut code_length_bitdepth_symbols: [u16; 18] = [
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
    ];
    (unsafe {
        CreateHuffmanTree_219(
            (&mut huffman_tree_histogram[(0) as usize] as *mut u32).cast_const(),
            (kCodeLengthCodes_210 as usize),
            5,
            (&mut code_length_bitdepth[(0) as usize] as *mut u8),
        )
    });
    (unsafe {
        ConvertBitDepthsToSymbols_220(
            (code_length_bitdepth.as_mut_ptr()).cast_const(),
            (kCodeLengthCodes_210 as usize),
            (&mut code_length_bitdepth_symbols[(0) as usize] as *mut u16),
        )
    });
    (unsafe {
        StoreHuffmanTreeOfHuffmanTreeToBitMask_211(
            num_codes,
            (code_length_bitdepth.as_mut_ptr()).cast_const(),
            storage,
        )
    });
    if ((num_codes) == (1)) {
        code_length_bitdepth[(code) as usize] = 0_u8;
    }
    (unsafe {
        StoreHuffmanTreeToBitMask_215(
            huffman_tree_size,
            (huffman_tree).cast_const(),
            (huffman_tree_extra_bits).cast_const(),
            (&mut code_length_bitdepth[(0) as usize] as *mut u8).cast_const(),
            (code_length_bitdepth_symbols.as_mut_ptr()).cast_const(),
            storage,
        )
    });
}
pub unsafe fn BuildAndStoreHuffmanTree_201(
    mut histogram: *const u32,
    length: usize,
    mut depth: *mut u8,
    mut bits: *mut u16,
    mut storage: *mut brunsli_Storage,
) {
    let mut count: usize = 0_usize;
    let mut s4: [usize; 4] = [0_usize, 0_usize, 0_usize, 0_usize];
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (length)) {
        if ((*histogram.offset((i) as isize)) != 0) {
            if ((count) < (4_usize)) {
                s4[(count)] = i;
            } else if ((count) > (4_usize)) {
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
    if ((count) <= (1_usize)) {
        (unsafe { WriteBits_120(4_usize, 1_u64, storage) });
        (unsafe { WriteBits_120(max_bits, (s4[(0) as usize] as u64), storage) });
        return;
    }
    (unsafe { CreateHuffmanTree_219(histogram, length, 15, depth) });
    (unsafe { ConvertBitDepthsToSymbols_220((depth).cast_const(), length, bits) });
    if ((count) <= (4_usize)) {
        (unsafe {
            StoreSimpleHuffmanTree_216(
                (depth).cast_const(),
                s4.as_mut_ptr(),
                count,
                max_bits,
                storage,
            )
        });
    } else {
        (unsafe { StoreHuffmanTree_217((depth).cast_const(), length, storage) });
    }
}
pub unsafe fn SetDepth_221(
    p: *const brunsli_HuffmanTree,
    mut pool: *mut brunsli_HuffmanTree,
    mut depth: *mut u8,
    mut level: u8,
) {
    if (((*p).index_left as i32) >= (0)) {
        level.prefix_inc();
        (unsafe {
            let _p: *const brunsli_HuffmanTree =
                &(*pool.offset(((*p).index_left) as isize)) as *const brunsli_HuffmanTree;
            let _pool: *mut brunsli_HuffmanTree = pool;
            let _depth: *mut u8 = depth;
            let _level: u8 = level;
            SetDepth_221(_p, _pool, _depth, _level)
        });
        (unsafe {
            let _p: *const brunsli_HuffmanTree =
                &(*pool.offset(((*p).index_right_or_value) as isize)) as *const brunsli_HuffmanTree;
            let _pool: *mut brunsli_HuffmanTree = pool;
            let _depth: *mut u8 = depth;
            let _level: u8 = level;
            SetDepth_221(_p, _pool, _depth, _level)
        });
    } else {
        (*depth.offset(((*p).index_right_or_value) as isize)) = level;
    }
}
pub unsafe fn Compare_222(v0: *const brunsli_HuffmanTree, v1: *const brunsli_HuffmanTree) -> bool {
    return (((*v0).total_count) < ((*v1).total_count));
}
pub unsafe fn CreateHuffmanTree_219(
    mut data: *const u32,
    length: usize,
    tree_limit: i32,
    mut depth: *mut u8,
) {
    let mut count_limit: u32 = 1_u32;
    'loop_: while true {
        let mut tree: Vec<brunsli_HuffmanTree> = Vec::new();
        if ((2_usize).wrapping_mul(length)).wrapping_add(1_usize) as usize
            > tree.capacity() as usize
        {
            let len_0 = tree.len();
            tree.reserve_exact(
                ((2_usize).wrapping_mul(length)).wrapping_add(1_usize) as usize - len_0 as usize,
            );
        };
        let mut i: usize = length;
        'loop_: while ((i) != (0_usize)) {
            i.prefix_dec();
            if ((*data.offset((i) as isize)) != 0) {
                let count: u32 = {
                    let mut __tmp_1 = (count_limit).wrapping_sub(1_u32);
                    (*if *data.offset((i) as isize) >= *&mut __tmp_1 {
                        (data.offset((i) as isize)) as *const _
                    } else {
                        (&mut __tmp_1) as *const _
                    })
                };
                tree.push(brunsli_HuffmanTree::brunsli_HuffmanTree(
                    count,
                    (-1_i32 as i16),
                    (i as i16),
                ));
            };
        }
        let n: usize = tree.len();
        if ((n) == (1_usize)) {
            (*depth.offset((tree[(0_usize)].index_right_or_value) as isize)) = 1_u8;
            break;
        }
        {
            let len = tree
                .as_mut_ptr()
                .add(tree.len())
                .offset_from(tree.as_mut_ptr()) as usize;
            ::std::slice::from_raw_parts_mut(tree.as_mut_ptr(), len).sort_by(|x, y| {
                if (Compare_222)(x, y) {
                    std::cmp::Ordering::Less
                } else if (Compare_222)(y, x) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        };
        let sentinel: brunsli_HuffmanTree =
            brunsli_HuffmanTree::brunsli_HuffmanTree(<u32>::MAX, (-1_i32 as i16), (-1_i32 as i16));
        {
            let a0_clone = sentinel.clone();
            tree.push(a0_clone)
        };
        {
            let a0_clone = sentinel.clone();
            tree.push(a0_clone)
        };
        let mut i: usize = 0_usize;
        let mut j: usize = (n).wrapping_add(1_usize);
        let mut k: usize = (n).wrapping_sub(1_usize);
        'loop_: while ((k) != (0_usize)) {
            let mut left: usize = 0_usize;
            let mut right: usize = 0_usize;
            if ((tree[(i)].total_count) <= (tree[(j)].total_count)) {
                left = i;
                i.prefix_inc();
            } else {
                left = j;
                j.prefix_inc();
            }
            if ((tree[(i)].total_count) <= (tree[(j)].total_count)) {
                right = i;
                i.prefix_inc();
            } else {
                right = j;
                j.prefix_inc();
            }
            let mut j_end: usize = (tree.len()).wrapping_sub(1_usize);
            tree[(j_end)].total_count =
                (tree[(left)].total_count).wrapping_add(tree[(right)].total_count);
            tree[(j_end)].index_left = (left as i16);
            tree[(j_end)].index_right_or_value = (right as i16);
            {
                let a0_clone = sentinel.clone();
                tree.push(a0_clone)
            };
            k.prefix_dec();
        }
        if !((tree.len()) == (((2_usize).wrapping_mul(n)).wrapping_add(1_usize))) {
            (unsafe {
                BrunsliDumpAndAbort_79(
                    c"huffman_tree.cc".as_ptr(),
                    121,
                    c"CreateHuffmanTree".as_ptr(),
                )
            });
            'loop_: while true {}
        };
        (unsafe {
            let _p: *const brunsli_HuffmanTree = &tree
                [(((2_usize).wrapping_mul(n)).wrapping_sub(1_usize))]
                as *const brunsli_HuffmanTree;
            let _pool: *mut brunsli_HuffmanTree =
                (&mut tree[(0_usize)] as *mut brunsli_HuffmanTree);
            let _depth: *mut u8 = depth;
            SetDepth_221(_p, _pool, _depth, 0_u8)
        });
        if (((*{
            let count = (&mut (*depth.offset((length) as isize)) as *mut u8)
                .offset_from((&mut (*depth.offset((0) as isize)) as *mut u8))
                as usize;
            std::slice::from_raw_parts((&mut (*depth.offset((0) as isize)) as *mut u8), count)
                .iter()
                .enumerate()
                .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| (&mut (*depth.offset((0) as isize)) as *mut u8).add(i))
                .unwrap_or((&mut (*depth.offset((0) as isize)) as *mut u8))
        }) as i32)
            <= (tree_limit))
        {
            break;
        }
        count_limit = (count_limit).wrapping_mul(2_u32);
    }
}
pub unsafe fn Reverse_223(mut v: *mut u8, mut start: usize, mut end: usize) {
    end.prefix_dec();
    'loop_: while ((start) < (end)) {
        let mut tmp: u8 = (*v.offset((start) as isize));
        (*v.offset((start) as isize)) = (*v.offset((end) as isize));
        (*v.offset((end) as isize)) = tmp;
        start.prefix_inc();
        end.prefix_dec();
    }
}
pub unsafe fn WriteHuffmanTreeRepetitions_224(
    previous_value: u8,
    value: u8,
    mut repetitions: usize,
    mut tree_size: *mut usize,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    if !((repetitions) > (0_usize)) {
        (unsafe {
            BrunsliDumpAndAbort_79(
                c"huffman_tree.cc".as_ptr(),
                151,
                c"WriteHuffmanTreeRepetitions".as_ptr(),
            )
        });
        'loop_: while true {}
    };
    if ((previous_value as i32) != (value as i32)) {
        (*tree.offset((*tree_size) as isize)) = value;
        (*extra_bits_data.offset((*tree_size) as isize)) = 0_u8;
        (*tree_size).prefix_inc();
        repetitions.prefix_dec();
    }
    if ((repetitions) == (7_usize)) {
        (*tree.offset((*tree_size) as isize)) = value;
        (*extra_bits_data.offset((*tree_size) as isize)) = 0_u8;
        (*tree_size).prefix_inc();
        repetitions.prefix_dec();
    }
    if ((repetitions) < (3_usize)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (repetitions)) {
            (*tree.offset((*tree_size) as isize)) = value;
            (*extra_bits_data.offset((*tree_size) as isize)) = 0_u8;
            (*tree_size).prefix_inc();
            i.prefix_inc();
        }
    } else {
        repetitions = (repetitions).wrapping_sub(3_usize);
        let mut start: usize = (*tree_size);
        'loop_: while true {
            (*tree.offset((*tree_size) as isize)) = 16_u8;
            (*extra_bits_data.offset((*tree_size) as isize)) = (((repetitions) & (3_usize)) as u8);
            (*tree_size).prefix_inc();
            repetitions >>= 2;
            if ((repetitions) == (0_usize)) {
                break;
            }
            repetitions.prefix_dec();
        }
        (unsafe {
            let _v: *mut u8 = tree;
            let _start: usize = start;
            let _end: usize = (*tree_size);
            Reverse_223(_v, _start, _end)
        });
        (unsafe {
            let _v: *mut u8 = extra_bits_data;
            let _start: usize = start;
            let _end: usize = (*tree_size);
            Reverse_223(_v, _start, _end)
        });
    }
}
pub unsafe fn WriteHuffmanTreeRepetitionsZeros_225(
    mut repetitions: usize,
    mut tree_size: *mut usize,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    if ((repetitions) == (11_usize)) {
        (*tree.offset((*tree_size) as isize)) = 0_u8;
        (*extra_bits_data.offset((*tree_size) as isize)) = 0_u8;
        (*tree_size).prefix_inc();
        repetitions.prefix_dec();
    }
    if ((repetitions) < (3_usize)) {
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (repetitions)) {
            (*tree.offset((*tree_size) as isize)) = 0_u8;
            (*extra_bits_data.offset((*tree_size) as isize)) = 0_u8;
            (*tree_size).prefix_inc();
            i.prefix_inc();
        }
    } else {
        repetitions = (repetitions).wrapping_sub(3_usize);
        let mut start: usize = (*tree_size);
        'loop_: while true {
            (*tree.offset((*tree_size) as isize)) = 17_u8;
            (*extra_bits_data.offset((*tree_size) as isize)) = (((repetitions) & (7_usize)) as u8);
            (*tree_size).prefix_inc();
            repetitions >>= 3;
            if ((repetitions) == (0_usize)) {
                break;
            }
            repetitions.prefix_dec();
        }
        (unsafe {
            let _v: *mut u8 = tree;
            let _start: usize = start;
            let _end: usize = (*tree_size);
            Reverse_223(_v, _start, _end)
        });
        (unsafe {
            let _v: *mut u8 = extra_bits_data;
            let _start: usize = start;
            let _end: usize = (*tree_size);
            Reverse_223(_v, _start, _end)
        });
    }
}
pub unsafe fn DecideOverRleUse_226(
    mut depth: *const u8,
    length: usize,
    mut use_rle_for_non_zero: *mut bool,
    mut use_rle_for_zero: *mut bool,
) {
    let mut total_reps_zero: usize = 0_usize;
    let mut total_reps_non_zero: usize = 0_usize;
    let mut count_reps_zero: usize = 1_usize;
    let mut count_reps_non_zero: usize = 1_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (length)) {
        let value: u8 = (*depth.offset((i) as isize));
        let mut reps: usize = 1_usize;
        let mut k: usize = (i).wrapping_add(1_usize);
        'loop_: while ((k) < (length)) && (((*depth.offset((k) as isize)) as i32) == (value as i32))
        {
            reps.prefix_inc();
            k.prefix_inc();
        }
        if ((reps) >= (3_usize)) && ((value as i32) == (0)) {
            total_reps_zero = (total_reps_zero).wrapping_add(reps);
            count_reps_zero.prefix_inc();
        }
        if ((reps) >= (4_usize)) && ((value as i32) != (0)) {
            total_reps_non_zero = (total_reps_non_zero).wrapping_add(reps);
            count_reps_non_zero.prefix_inc();
        }
        i = (i).wrapping_add(reps);
    }
    (*use_rle_for_non_zero) =
        ((total_reps_non_zero) > ((count_reps_non_zero).wrapping_mul(2_usize)));
    (*use_rle_for_zero) = ((total_reps_zero) > ((count_reps_zero).wrapping_mul(2_usize)));
}
pub unsafe fn WriteHuffmanTree_218(
    mut depth: *const u8,
    mut length: usize,
    mut tree_size: *mut usize,
    mut tree: *mut u8,
    mut extra_bits_data: *mut u8,
) {
    let mut previous_value: u8 = 8_u8;
    let mut new_length: usize = length;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (length)) {
        if (((*depth.offset((((length).wrapping_sub(i)).wrapping_sub(1_usize)) as isize)) as i32)
            == (0))
        {
            new_length.prefix_dec();
        } else {
            break;
        }
        i.prefix_inc();
    }
    let mut use_rle_for_non_zero: bool = false;
    let mut use_rle_for_zero: bool = false;
    if ((length) > (50_usize)) {
        (unsafe {
            DecideOverRleUse_226(
                depth,
                new_length,
                (&mut use_rle_for_non_zero as *mut bool),
                (&mut use_rle_for_zero as *mut bool),
            )
        });
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (new_length)) {
        let value: u8 = (*depth.offset((i) as isize));
        let mut reps: usize = 1_usize;
        if (((value as i32) != (0)) && (use_rle_for_non_zero))
            || (((value as i32) == (0)) && (use_rle_for_zero))
        {
            let mut k: usize = (i).wrapping_add(1_usize);
            'loop_: while ((k) < (new_length))
                && (((*depth.offset((k) as isize)) as i32) == (value as i32))
            {
                reps.prefix_inc();
                k.prefix_inc();
            }
        }
        if ((value as i32) == (0)) {
            (unsafe {
                let _repetitions: usize = reps;
                let _tree_size: *mut usize = tree_size;
                WriteHuffmanTreeRepetitionsZeros_225(
                    _repetitions,
                    _tree_size,
                    tree,
                    extra_bits_data,
                )
            });
        } else {
            (unsafe {
                let _previous_value: u8 = previous_value;
                let _value: u8 = value;
                let _repetitions: usize = reps;
                let _tree_size: *mut usize = tree_size;
                let _tree: *mut u8 = tree;
                let _extra_bits_data: *mut u8 = extra_bits_data;
                WriteHuffmanTreeRepetitions_224(
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
        i = (i).wrapping_add(reps);
    }
}
pub unsafe fn ReverseBits_227(mut num_bits: i32, mut bits: u16) -> u16 {
    static mut kLut_228: [usize; 16] = unsafe {
        [
            0_usize, 8_usize, 4_usize, 12_usize, 2_usize, 10_usize, 6_usize, 14_usize, 1_usize,
            9_usize, 5_usize, 13_usize, 3_usize, 11_usize, 7_usize, 15_usize,
        ]
    };;
    let mut retval: usize = kLut_228[((bits as i32) & (15)) as usize];
    let mut i: i32 = 4;
    'loop_: while ((i) < (num_bits)) {
        retval <<= 4;
        bits = (((bits as i32) >> (4)) as u16);
        retval |= kLut_228[((bits as i32) & (15)) as usize];
        i += 4;
    }
    retval >>= ((-num_bits) & (3));
    return (retval as u16);
}
pub unsafe fn ConvertBitDepthsToSymbols_220(
    mut depth: *const u8,
    mut len: usize,
    mut bits: *mut u16,
) {
    let kMaxBits: i32 = 16;
    let mut bl_count: [u16; 16] = [
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ];
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        bl_count[(*depth.offset((i) as isize)) as usize].prefix_inc();
        i.prefix_inc();
    }
    bl_count[(0) as usize] = 0_u16;
    let mut next_code: [u16; 16] = [0_u16; 16];
    next_code[(0) as usize] = 0_u16;
    let mut code: i32 = 0;
    let mut i: usize = 1_usize;
    'loop_: while ((i) < (kMaxBits as usize)) {
        code = (((code) + (bl_count[((i).wrapping_sub(1_usize))] as i32)) << (1));
        next_code[(i)] = (code as u16);
        i.prefix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        if ((*depth.offset((i) as isize)) != 0) {
            (*bits.offset((i) as isize)) = (unsafe {
                let _num_bits: i32 = ((*depth.offset((i) as isize)) as i32);
                let _bits: u16 = next_code[(*depth.offset((i) as isize)) as usize].postfix_inc();
                ReverseBits_227(_num_bits, _bits)
            });
        }
        i.prefix_inc();
    }
}
pub static mut kJpegHuffmanRootTableBits_229: i32 = unsafe { 8 };
pub static mut kJpegHuffmanLutSize_230: i32 = unsafe { 1024 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_HuffmanTableEntry {
    pub bits: u8,
    pub value: u16,
}
impl brunsli_HuffmanTableEntry {
    pub unsafe fn brunsli_HuffmanTableEntry() -> Self {
        let mut this = Self {
            bits: 0_u8,
            value: 65535_u16,
        };
        this
    }
}
impl Default for brunsli_HuffmanTableEntry {
    fn default() -> Self {
        unsafe { brunsli_HuffmanTableEntry::brunsli_HuffmanTableEntry() }
    }
}
pub unsafe fn DivCeil_231(mut a: i32, mut b: i32) -> i32 {
    return ((((a) + (b)) - (1)) / (b));
}
pub unsafe fn ReadUint8_232(mut data: *const u8, mut pos: *mut usize) -> i32 {
    return ((*data.offset(((*pos).postfix_inc()) as isize)) as i32);
}
pub unsafe fn ReadUint16_233(mut data: *const u8, mut pos: *mut usize) -> i32 {
    let mut v: i32 = ((((*data.offset((*pos) as isize)) as i32) << (8))
        + ((*data.offset(((*pos).wrapping_add(1_usize)) as isize)) as i32));
    (*pos) = (*pos).wrapping_add(2_usize);
    return v;
}
pub unsafe fn ProcessSOF_234(
    mut data: *const u8,
    len: usize,
    mut mode: brunsli_JpegReadMode,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    if (((*jpg).width) != (0)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Duplicate SOF marker.\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::DUPLICATE_SOF).clone();
        return false;
    }
    let start_pos: usize = (*pos);
    if (((*pos).wrapping_add(((8) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (8),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    let mut precision: i32 = (unsafe { ReadUint8_232(data, pos) });
    let mut height: i32 = (unsafe { ReadUint16_233(data, pos) });
    let mut width: i32 = (unsafe { ReadUint16_233(data, pos) });
    let mut num_components: i32 = (unsafe { ReadUint8_232(data, pos) });
    if ((precision) < (8)) || ((precision) > (8)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid precision: {:}\n",
            precision,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_PRECISION).clone();
        return false;
    };
    if ((height) < (1)) || ((height) > (kMaxDimPixels_11)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid height: {:}\n",
            height,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_HEIGHT).clone();
        return false;
    };
    if ((width) < (1)) || ((width) > (kMaxDimPixels_11)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid width: {:}\n",
            width,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_WIDTH).clone();
        return false;
    };
    if ((num_components) < (1)) || ((num_components) > (kMaxComponents_4)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid num_components: {:}\n",
            num_components,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_NUMCOMP).clone();
        return false;
    };
    if (((*pos).wrapping_add((((3) * (num_components)) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            ((3) * (num_components)),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    (*jpg).height = height;
    (*jpg).width = width;
    {
        let __a0 = (num_components as usize) as usize;
        (*jpg)
            .components
            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
    };
    let mut ids_seen: Vec<bool> = (0..(256_usize) as usize)
        .map(|_| false)
        .collect::<Vec<bool>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) {
        let id: i32 = (unsafe { ReadUint8_232(data, pos) });
        if (ids_seen[(id as usize)] as bool) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Duplicate ID {:} in SOF.\n",
                id,
            );
            (*jpg).error = (brunsli_JPEGReadError::DUPLICATE_COMPONENT_ID).clone();
            return false;
        }
        ids_seen[(id as usize)] = true;
        (&mut (*jpg)).components[(i)].id = id;
        let mut factor: i32 = (unsafe { ReadUint8_232(data, pos) });
        let mut h_samp_factor: i32 = ((factor) >> (4));
        let mut v_samp_factor: i32 = ((factor) & (15));
        if ((h_samp_factor) < (1)) || ((h_samp_factor) > (kBrunsliMaxSampling_27)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid h_samp_factor: {:}\n",
                h_samp_factor,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_SAMP_FACTOR).clone();
            return false;
        };
        if ((v_samp_factor) < (1)) || ((v_samp_factor) > (kBrunsliMaxSampling_27)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid v_samp_factor: {:}\n",
                v_samp_factor,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_SAMP_FACTOR).clone();
            return false;
        };
        (&mut (*jpg)).components[(i)].h_samp_factor = h_samp_factor;
        (&mut (*jpg)).components[(i)].v_samp_factor = v_samp_factor;
        (&mut (*jpg)).components[(i)].quant_idx =
            ((unsafe { ReadUint8_232(data, pos) }) as u8).clone();
        (*jpg).max_h_samp_factor = (*if *&(*jpg).max_h_samp_factor >= *&mut h_samp_factor {
            (&(*jpg).max_h_samp_factor) as *const _
        } else {
            (&mut h_samp_factor) as *const _
        });
        (*jpg).max_v_samp_factor = (*if *&(*jpg).max_v_samp_factor >= *&mut v_samp_factor {
            (&(*jpg).max_v_samp_factor) as *const _
        } else {
            (&mut v_samp_factor) as *const _
        });
        i.prefix_inc();
    }
    (*jpg).MCU_rows = (unsafe {
        let _a: i32 = (*jpg).height;
        let _b: i32 = (((*jpg).max_v_samp_factor) * (8));
        DivCeil_231(_a, _b)
    });
    (*jpg).MCU_cols = (unsafe {
        let _a: i32 = (*jpg).width;
        let _b: i32 = (((*jpg).max_h_samp_factor) * (8));
        DivCeil_231(_a, _b)
    });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) {
        let mut c: *mut brunsli_JPEGComponent =
            (&mut (&mut (*jpg)).components[(i)] as *mut brunsli_JPEGComponent);
        if ((((*jpg).max_h_samp_factor) % ((*c).h_samp_factor)) != (0))
            || ((((*jpg).max_v_samp_factor) % ((*c).v_samp_factor)) != (0))
        {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Non-integral subsampling ratios.\n",
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_SAMPLING_FACTORS).clone();
            return false;
        }
        (*c).width_in_blocks = ((((*jpg).MCU_cols) * ((*c).h_samp_factor)) as u32);
        (*c).height_in_blocks = ((((*jpg).MCU_rows) * ((*c).v_samp_factor)) as u32);
        let num_blocks: u64 =
            ((*c).width_in_blocks as u64).wrapping_mul(((*c).height_in_blocks as u64));
        if ((num_blocks as usize) > (kBrunsliMaxNumBlocks_18)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Image too large.\n",
            );
            (*jpg).error = (brunsli_JPEGReadError::IMAGE_TOO_LARGE).clone();
            return false;
        }
        (*c).num_blocks = ((num_blocks as i32) as u32);
        if ((mode as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
            {
                let __a0 =
                    ((((*c).num_blocks).wrapping_mul((kDCTBlockSize_3 as u32))) as usize) as usize;
                (*c).coeffs.resize_with(__a0, || <i16>::default())
            };
        }
        i.prefix_inc();
    }
    if (((start_pos).wrapping_add(marker_len)) != (*pos)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((*pos).wrapping_sub(start_pos)),
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_MARKER_SIZE).clone();
        return false;
    };
    return true;
}
pub unsafe fn ProcessSOS_235(
    mut data: *const u8,
    len: usize,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let start_pos: usize = (*pos);
    if (((*pos).wrapping_add(((3) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (3),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    let mut comps_in_scan: i32 = (unsafe { ReadUint8_232(data, pos) });
    if ((comps_in_scan as usize) < (1_usize))
        || ((comps_in_scan as usize) > ((*jpg).components.len()))
    {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid static_cast<size_t>(comps_in_scan): {:}\n",
            (comps_in_scan as usize),
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_COMPS_IN_SCAN).clone();
        return false;
    };
    let mut scan_info: brunsli_JPEGScanInfo = <brunsli_JPEGScanInfo>::default();
    scan_info.num_components = (comps_in_scan as usize);
    if (((*pos).wrapping_add((((2) * (comps_in_scan)) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            ((2) * (comps_in_scan)),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut ids_seen: Vec<bool> = (0..(256_usize) as usize)
        .map(|_| false)
        .collect::<Vec<bool>>();
    let mut i: i32 = 0;
    'loop_: while ((i) < (comps_in_scan)) {
        let mut id: i32 = (unsafe { ReadUint8_232(data, pos) });
        if (ids_seen[(id as usize)] as bool) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Duplicate ID {:} in SOS.\n",
                id,
            );
            (*jpg).error = (brunsli_JPEGReadError::DUPLICATE_COMPONENT_ID).clone();
            return false;
        }
        ids_seen[(id as usize)] = true;
        let mut found_index: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ((j) < ((*jpg).components.len())) {
            if (((&mut (*jpg)).components[(j)].id) == (id)) {
                scan_info.components[(i as usize)].comp_idx = (j as u8);
                found_index = true;
            }
            j.prefix_inc();
        }
        if !found_index {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "SOS marker: Could not find component with id {:}\n",
                id,
            );
            (*jpg).error = (brunsli_JPEGReadError::COMPONENT_NOT_FOUND).clone();
            return false;
        }
        let mut c: i32 = (unsafe { ReadUint8_232(data, pos) });
        let mut dc_tbl_idx: i32 = ((c) >> (4));
        let mut ac_tbl_idx: i32 = ((c) & (15));
        if ((dc_tbl_idx) < (0)) || ((dc_tbl_idx) > (3)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid dc_tbl_idx: {:}\n",
                dc_tbl_idx,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX).clone();
            return false;
        };
        if ((ac_tbl_idx) < (0)) || ((ac_tbl_idx) > (3)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid ac_tbl_idx: {:}\n",
                ac_tbl_idx,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX).clone();
            return false;
        };
        scan_info.components[(i as usize)].dc_tbl_idx = dc_tbl_idx;
        scan_info.components[(i as usize)].ac_tbl_idx = ac_tbl_idx;
        i.prefix_inc();
    }
    if (((*pos).wrapping_add(((3) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (3),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    scan_info.Ss = (unsafe { ReadUint8_232(data, pos) }).clone();
    scan_info.Se = (unsafe { ReadUint8_232(data, pos) }).clone();
    if ((scan_info.Ss) < (0)) || ((scan_info.Ss) > (63)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid scan_info.Ss: {:}\n",
            scan_info.Ss,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_START_OF_SCAN).clone();
        return false;
    };
    if ((scan_info.Se) < (scan_info.Ss)) || ((scan_info.Se) > (63)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid scan_info.Se: {:}\n",
            scan_info.Se,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_END_OF_SCAN).clone();
        return false;
    };
    let mut c: i32 = (unsafe { ReadUint8_232(data, pos) });
    scan_info.Ah = ((c) >> (4));
    scan_info.Al = ((c) & (15));
    if ((scan_info.Ah) != (0)) && ((scan_info.Al) != ((scan_info.Ah) - (1))) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid progressive parameters:  Al = {:} Ah = {:}\n",
            scan_info.Al,
            scan_info.Ah,
        );
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (comps_in_scan)) {
        let mut found_dc_table: bool = false;
        let mut found_ac_table: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ((j) < ((*jpg).huffman_code.len())) {
            let mut slot_id: i32 = (&mut (*jpg)).huffman_code[(j)].slot_id;
            if ((slot_id) == (scan_info.components[(i as usize)].dc_tbl_idx)) {
                found_dc_table = true;
            } else if ((slot_id) == ((scan_info.components[(i as usize)].ac_tbl_idx) + (16))) {
                found_ac_table = true;
            }
            j.prefix_inc();
        }
        if ((scan_info.Ss) == (0)) && (!found_dc_table) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "SOS marker: Could not find DC Huffman table with index {:}\n",
                scan_info.components[(i as usize)].dc_tbl_idx,
            );
            (*jpg).error = (brunsli_JPEGReadError::HUFFMAN_TABLE_NOT_FOUND).clone();
            return false;
        }
        if ((scan_info.Se) > (0)) && (!found_ac_table) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "SOS marker: Could not find AC Huffman table with index {:}\n",
                scan_info.components[(i as usize)].ac_tbl_idx,
            );
            (*jpg).error = (brunsli_JPEGReadError::HUFFMAN_TABLE_NOT_FOUND).clone();
            return false;
        }
        i.prefix_inc();
    }
    {
        let a0_clone = scan_info.clone();
        (*jpg).scan_info.push(a0_clone)
    };
    if (((start_pos).wrapping_add(marker_len)) != (*pos)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((*pos).wrapping_sub(start_pos)),
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_MARKER_SIZE).clone();
        return false;
    };
    return true;
}
pub unsafe fn ProcessDHT_236(
    mut data: *const u8,
    len: usize,
    mut mode: brunsli_JpegReadMode,
    mut dc_huff_lut: *mut Vec<brunsli_HuffmanTableEntry>,
    mut ac_huff_lut: *mut Vec<brunsli_HuffmanTableEntry>,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let start_pos: usize = (*pos);
    if (((*pos).wrapping_add(((2) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (2),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    if ((marker_len) == (2_usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "DHT marker: no Huffman table found\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::EMPTY_DHT).clone();
        return false;
    }
    'loop_: while ((*pos) < ((start_pos).wrapping_add(marker_len))) {
        if (((*pos).wrapping_add((((1) + (kJpegHuffmanMaxBitLength_7)) as usize))) > (len)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (*pos),
                ((1) + (kJpegHuffmanMaxBitLength_7)),
                len,
            );
            (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
            return false;
        };
        let mut huff: brunsli_JPEGHuffmanCode = <brunsli_JPEGHuffmanCode>::default();
        huff.slot_id = (unsafe { ReadUint8_232(data, pos) }).clone();
        let mut huffman_index: i32 = huff.slot_id;
        let mut is_ac_table: i32 = ((((huff.slot_id) & (16)) != (0)) as i32);
        let mut huff_lut: *mut brunsli_HuffmanTableEntry = std::ptr::null_mut();
        if (is_ac_table != 0) {
            huffman_index -= 16;
            if ((huffman_index) < (0)) || ((huffman_index) > (3)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid huffman_index: {:}\n",
                    huffman_index,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX).clone();
                return false;
            };
            huff_lut = (&mut (&mut (*ac_huff_lut))
                [(((huffman_index) * (kJpegHuffmanLutSize_230)) as usize)]
                as *mut brunsli_HuffmanTableEntry);
        } else {
            if ((huffman_index) < (0)) || ((huffman_index) > (3)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid huffman_index: {:}\n",
                    huffman_index,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX).clone();
                return false;
            };
            huff_lut = (&mut (&mut (*dc_huff_lut))
                [(((huffman_index) * (kJpegHuffmanLutSize_230)) as usize)]
                as *mut brunsli_HuffmanTableEntry);
        }
        huff.counts[(0_usize)] = 0;
        let mut total_count: i32 = 0;
        let mut space: i32 = ((1) << (kJpegHuffmanMaxBitLength_7));
        let mut max_depth: i32 = 1;
        let mut i: i32 = 1;
        'loop_: while ((i) <= (kJpegHuffmanMaxBitLength_7)) {
            let mut count: i32 = (unsafe { ReadUint8_232(data, pos) });
            if ((count) != (0)) {
                max_depth = i;
            }
            huff.counts[(i as usize)] = count;
            total_count += count;
            space -= ((count) * ((1) << ((kJpegHuffmanMaxBitLength_7) - (i))));
            i.prefix_inc();
        }
        if (is_ac_table != 0) {
            if ((total_count) < (0)) || ((total_count) > (kJpegHuffmanAlphabetSize_8)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid total_count: {:}\n",
                    total_count,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_CODE).clone();
                return false;
            };
        } else {
            if ((total_count) < (0)) || ((total_count) > (kJpegDCAlphabetSize_9)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid total_count: {:}\n",
                    total_count,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_CODE).clone();
                return false;
            };
        }
        if (((*pos).wrapping_add(((total_count) as usize))) > (len)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (*pos),
                (total_count),
                len,
            );
            (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
            return false;
        };
        let mut values_seen: Vec<bool> = (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>();
        let mut i: i32 = 0;
        'loop_: while ((i) < (total_count)) {
            let mut value: u8 = ((unsafe { ReadUint8_232(data, pos) }) as u8);
            if !(is_ac_table != 0) {
                if ((value as i32) < (0)) || ((value as i32) > ((kJpegDCAlphabetSize_9) - (1))) {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "Invalid value: ",
                    );
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    )
                    .write_all(&([(&[value as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()));
                    (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_CODE).clone();
                    return false;
                };
            }
            if (values_seen[(value as usize)] as bool) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Duplicate Huffman code value ",
                );
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                )
                .write_all(&([(&[value as u8] as &[u8]), (&[b'\n'] as &[u8])].concat()));
                (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_CODE).clone();
                return false;
            }
            values_seen[(value as usize)] = true;
            huff.values[(i as usize)] = (value as i32);
            i.prefix_inc();
        }
        huff.counts[(max_depth as usize)].prefix_inc();
        huff.values[(total_count as usize)] = kJpegHuffmanAlphabetSize_8;
        space -= ((1) << ((kJpegHuffmanMaxBitLength_7) - (max_depth)));
        if ((space) < (0)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid Huffman code lengths.\n",
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_HUFFMAN_CODE).clone();
            return false;
        } else if ((space) > (0)) && (((*huff_lut.offset((0) as isize)).value as i32) != (65535)) {
            let mut i: i32 = 0;
            'loop_: while ((i) < (kJpegHuffmanLutSize_230)) {
                (*huff_lut.offset((i) as isize)).bits = 0_u8;
                (*huff_lut.offset((i) as isize)).value = 65535_u16;
                i.prefix_inc();
            }
        }
        huff.is_last = ((*pos) == ((start_pos).wrapping_add(marker_len)));
        if ((mode as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
            (unsafe {
                let _counts: *const i32 = (&mut huff.counts[(0_usize)] as *mut i32).cast_const();
                let _symbols: *const i32 = (&mut huff.values[(0_usize)] as *mut i32).cast_const();
                BuildJpegHuffmanTable_237(_counts, _symbols, huff_lut)
            });
        }
        {
            let a0_clone = huff.clone();
            (*jpg).huffman_code.push(a0_clone)
        };
    }
    if (((start_pos).wrapping_add(marker_len)) != (*pos)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((*pos).wrapping_sub(start_pos)),
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_MARKER_SIZE).clone();
        return false;
    };
    return true;
}
pub unsafe fn ProcessDQT_238(
    mut data: *const u8,
    len: usize,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let start_pos: usize = (*pos);
    if (((*pos).wrapping_add(((2) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (2),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    if ((marker_len) == (2_usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "DQT marker: no quantization table found\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::EMPTY_DQT).clone();
        return false;
    }
    'loop_: while ((*pos) < ((start_pos).wrapping_add(marker_len)))
        && (((*jpg).quant.len()) < (kMaxQuantTables_5 as usize))
    {
        if (((*pos).wrapping_add(((1) as usize))) > (len)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (*pos),
                (1),
                len,
            );
            (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
            return false;
        };
        let mut quant_table_index: i32 = (unsafe { ReadUint8_232(data, pos) });
        let mut quant_table_precision: i32 = ((quant_table_index) >> (4));
        if ((quant_table_precision) < (0)) || ((quant_table_precision) > (1)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid quant_table_precision: {:}\n",
                quant_table_precision,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_QUANT_TBL_PRECISION).clone();
            return false;
        };
        quant_table_index &= 15;
        if ((quant_table_index) < (0)) || ((quant_table_index) > (3)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid quant_table_index: {:}\n",
                quant_table_index,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_QUANT_TBL_INDEX).clone();
            return false;
        };
        if (((*pos).wrapping_add(((((quant_table_precision) + (1)) * (kDCTBlockSize_3)) as usize)))
            > (len))
        {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                (*pos),
                (((quant_table_precision) + (1)) * (kDCTBlockSize_3)),
                len,
            );
            (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
            return false;
        };
        let mut table: brunsli_JPEGQuantTable = <brunsli_JPEGQuantTable>::default();
        table.index = quant_table_index;
        table.precision = quant_table_precision;
        let mut i: i32 = 0;
        'loop_: while ((i) < (kDCTBlockSize_3)) {
            let mut quant_val: i32 = if (quant_table_precision != 0) {
                (unsafe { ReadUint16_233(data, pos) })
            } else {
                (unsafe { ReadUint8_232(data, pos) })
            };
            if ((quant_val) < (1)) || ((quant_val) > (65535)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid quant_val: {:}\n",
                    quant_val,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_QUANT_VAL).clone();
                return false;
            };
            table.values[(kJPEGNaturalOrder_13[(i) as usize] as usize)] = quant_val;
            i.prefix_inc();
        }
        table.is_last = ((*pos) == ((start_pos).wrapping_add(marker_len)));
        {
            let a0_clone = table.clone();
            (*jpg).quant.push(a0_clone)
        };
    }
    if (((start_pos).wrapping_add(marker_len)) != (*pos)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((*pos).wrapping_sub(start_pos)),
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_MARKER_SIZE).clone();
        return false;
    };
    return true;
}
pub unsafe fn ProcessDRI_239(
    mut data: *const u8,
    len: usize,
    mut pos: *mut usize,
    mut found_dri: *mut bool,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    if (*found_dri) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Duplicate DRI marker.\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::DUPLICATE_DRI).clone();
        return false;
    }
    (*found_dri) = true;
    let start_pos: usize = (*pos);
    if (((*pos).wrapping_add(((4) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (4),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    let mut restart_interval: i32 = (unsafe { ReadUint16_233(data, pos) });
    (*jpg).restart_interval = restart_interval;
    if (((start_pos).wrapping_add(marker_len)) != (*pos)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker length: declared={:} actual={:}\n",
            marker_len,
            ((*pos).wrapping_sub(start_pos)),
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_MARKER_SIZE).clone();
        return false;
    };
    return true;
}
pub unsafe fn ProcessAPP_240(
    mut data: *const u8,
    len: usize,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    if (((*pos).wrapping_add(((2) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (2),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    if ((marker_len) < (2_usize)) || ((marker_len) > (65535_usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker_len: {:}\n",
            marker_len,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_MARKER_LEN).clone();
        return false;
    };
    if (((*pos).wrapping_add(((marker_len).wrapping_sub(2_usize)))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            ((marker_len).wrapping_sub(2_usize)),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut app_str_start: *const u8 = data.offset((*pos) as isize).offset(-((3) as isize));
    let mut app_str: Vec<u8> = core::slice::from_raw_parts(
        app_str_start,
        (app_str_start
            .offset((marker_len) as isize)
            .offset((1) as isize))
        .offset_from(app_str_start) as usize,
    )
    .to_vec();
    (*pos) = (*pos).wrapping_add((marker_len).wrapping_sub(2_usize));
    (*jpg).app_data.push(app_str.clone());
    return true;
}
pub unsafe fn ProcessCOM_241(
    mut data: *const u8,
    len: usize,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    if (((*pos).wrapping_add(((2) as usize))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            (2),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut marker_len: usize = ((unsafe { ReadUint16_233(data, pos) }) as usize);
    if ((marker_len) < (2_usize)) || ((marker_len) > (65535_usize)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Invalid marker_len: {:}\n",
            marker_len,
        );
        (*jpg).error = (brunsli_JPEGReadError::INVALID_MARKER_LEN).clone();
        return false;
    };
    if (((*pos).wrapping_add(((marker_len).wrapping_sub(2_usize)))) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            (*pos),
            ((marker_len).wrapping_sub(2_usize)),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    };
    let mut com_str_start: *const u8 = data.offset((*pos) as isize).offset(-((3) as isize));
    let mut com_str: Vec<u8> = core::slice::from_raw_parts(
        com_str_start,
        (com_str_start
            .offset((marker_len) as isize)
            .offset((1) as isize))
        .offset_from(com_str_start) as usize,
    )
    .to_vec();
    (*pos) = (*pos).wrapping_add((marker_len).wrapping_sub(2_usize));
    (*jpg).com_data.push(com_str.clone());
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_BitReaderState {
    pub data_: *const u8,
    pub len_: usize,
    pub pos_: usize,
    pub val_: u64,
    pub bits_left_: i32,
    pub next_marker_pos_: usize,
}
impl brunsli_BitReaderState {
    pub unsafe fn brunsli_BitReaderState(mut data: *const u8, len: usize, mut pos: usize) -> Self {
        let mut this = Self {
            data_: data,
            len_: len,
            pos_: 0_usize,
            val_: 0_u64,
            bits_left_: 0_i32,
            next_marker_pos_: 0_usize,
        };
        (unsafe { this.Reset(pos) });
        this
    }
    pub unsafe fn Reset(&mut self, mut pos: usize) {
        self.pos_ = pos;
        self.val_ = 0_u64;
        self.bits_left_ = 0;
        self.next_marker_pos_ = (self.len_).wrapping_sub(2_usize);
        (unsafe { self.FillBitWindow() });
    }
    pub unsafe fn GetNextByte(&mut self) -> u8 {
        if ((self.pos_) >= (self.next_marker_pos_)) {
            self.pos_.prefix_inc();
            return 0_u8;
        }
        let mut c: u8 = (*self.data_.offset((self.pos_.postfix_inc()) as isize));
        if ((c as i32) == (255)) {
            let mut escape: u8 = (*self.data_.offset((self.pos_) as isize));
            if ((escape as i32) == (0)) {
                self.pos_.prefix_inc();
            } else {
                self.next_marker_pos_ = (self.pos_).wrapping_sub(1_usize);
            }
        }
        return c;
    }
    pub unsafe fn FillBitWindow(&mut self) {
        if ((self.bits_left_) <= (16)) {
            'loop_: while ((self.bits_left_) <= (56)) {
                self.val_ <<= 8;
                self.val_ |= ((unsafe { self.GetNextByte() }) as u64);
                self.bits_left_ += 8;
            }
        }
    }
    pub unsafe fn ReadBits(&mut self, mut nbits: i32) -> i32 {
        (unsafe { self.FillBitWindow() });
        let mut val: u64 = (((self.val_) >> ((self.bits_left_) - (nbits)))
            & (((1_u64) << (nbits)).wrapping_sub(1_u64)));
        self.bits_left_ -= nbits;
        if !((val) < (((1_u32) << (31)) as u64)) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"jpeg_data_reader.cc".as_ptr(), 471, c"ReadBits".as_ptr())
            });
            'loop_: while true {}
        };
        return (val as i32);
    }
    pub unsafe fn IsUnhealthy(&mut self) -> bool {
        return ((self.pos_) > ((self.next_marker_pos_).wrapping_add(32_usize)));
    }
    pub unsafe fn FinishStream(
        &mut self,
        mut jpg: *mut brunsli_JPEGData,
        mut pos: *mut usize,
    ) -> bool {
        let mut npadbits: i32 = ((self.bits_left_) & (7));
        if ((npadbits) > (0)) {
            let mut padmask: u64 = ((1_u64) << (npadbits)).wrapping_sub(1_u64);
            let mut padbits: u64 = (((self.val_) >> ((self.bits_left_) - (npadbits))) & (padmask));
            if ((padbits) != (padmask)) {
                (*jpg).has_zero_padding_bit = true;
            }
            let mut i: i32 = ((npadbits) - (1));
            'loop_: while ((i) >= (0)) {
                (*jpg)
                    .padding_bits
                    .push(((((padbits) >> (i)) & (1_u64)) as i32));
                i.prefix_dec();
            }
        }
        let mut unused_bytes_left: i32 = ((self.bits_left_) >> (3));
        'loop_: while ((unused_bytes_left.postfix_dec()) > (0)) {
            self.pos_.prefix_dec();
            if (((self.pos_) < (self.next_marker_pos_))
                && (((*self.data_.offset((self.pos_) as isize)) as i32) == (0)))
                && (((*self
                    .data_
                    .offset(((self.pos_).wrapping_sub(1_usize)) as isize))
                    as i32)
                    == (255))
            {
                self.pos_.prefix_dec();
            }
        }
        if ((self.pos_) > (self.next_marker_pos_)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Unexpected end of scan.\n",
            );
            return false;
        }
        (*pos) = self.pos_;
        return true;
    }
}
pub unsafe fn ReadSymbol_242(
    mut table: *const brunsli_HuffmanTableEntry,
    mut br: *mut brunsli_BitReaderState,
) -> i32 {
    let mut nbits: i32 = 0_i32;
    (unsafe { (*br).FillBitWindow() });
    let mut val: i32 = (((((*br).val_) >> (((*br).bits_left_) - (8))) & (255_u64)) as i32);
    table = (table).wrapping_add(val as usize);
    nbits = (((*table).bits as i32) - (8));
    if ((nbits) > (0)) {
        (*br).bits_left_ -= 8;
        table = (table).wrapping_add(((*table).value as i32) as usize);
        val = (((((*br).val_) >> (((*br).bits_left_) - (nbits)))
            & ((((1) << (nbits)) - (1)) as u64)) as i32);
        table = (table).wrapping_add(val as usize);
    }
    (*br).bits_left_ -= ((*table).bits as i32);
    return ((*table).value as i32);
}
pub unsafe fn HuffExtend_243(mut x: i32, mut s: i32) -> i32 {
    if !((s) >= (1)) {
        (unsafe {
            BrunsliDumpAndAbort_79(c"jpeg_data_reader.cc".as_ptr(), 575, c"HuffExtend".as_ptr())
        });
        'loop_: while true {}
    };
    let mut half: i32 = ((1) << ((s) - (1)));
    if ((x) >= (half)) {
        if !((x) < ((1) << (s))) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"jpeg_data_reader.cc".as_ptr(), 578, c"HuffExtend".as_ptr())
            });
            'loop_: while true {}
        };
        return x;
    } else {
        return (((x) - ((1) << (s))) + (1));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn DecodeDCTBlock_244(
    mut dc_huff: *const brunsli_HuffmanTableEntry,
    mut ac_huff: *const brunsli_HuffmanTableEntry,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut eobrun: *mut i32,
    mut reset_state: *mut bool,
    mut num_zero_runs: *mut i32,
    mut br: *mut brunsli_BitReaderState,
    mut jpg: *mut brunsli_JPEGData,
    mut last_dc_coeff: *mut i16,
    mut coeffs: *mut i16,
) -> bool {
    let mut Am: i32 = ((1) << (Al));
    let mut eobrun_allowed: bool = ((Ss) > (0));
    if ((Ss) == (0)) {
        let mut s: i32 = (unsafe { ReadSymbol_242(dc_huff, br) });
        if ((s) >= (kJpegDCAlphabetSize_9)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid Huffman symbol {:} for DC coefficient.\n",
                s,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_SYMBOL).clone();
            return false;
        }
        let mut diff: i32 = 0;
        if ((s) > (0)) {
            let mut bits: i32 = (unsafe { (*br).ReadBits(s) });
            diff = (unsafe { HuffExtend_243(bits, s) });
        }
        let mut coeff: i32 = ((diff) + ((*last_dc_coeff) as i32));
        let dc_coeff: i32 = ((coeff) * (Am));
        (*coeffs.offset((0) as isize)) = (dc_coeff as i16);
        if ((dc_coeff) != ((*coeffs.offset((0) as isize)) as i32)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid DC coefficient {:}\n",
                dc_coeff,
            );
            (*jpg).error = (brunsli_JPEGReadError::NON_REPRESENTABLE_DC_COEFF).clone();
            return false;
        }
        (*last_dc_coeff) = (coeff as i16);
        Ss.prefix_inc();
    }
    if ((Ss) > (Se)) {
        return true;
    }
    if ((*eobrun) > (0)) {
        (*eobrun).prefix_dec();
        return true;
    }
    (*num_zero_runs) = 0;
    let mut k: i32 = Ss;
    'loop_: while ((k) <= (Se)) {
        let mut sr: i32 = (unsafe { ReadSymbol_242(ac_huff, br) });
        if ((sr) >= (kJpegHuffmanAlphabetSize_8)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                sr,
                k,
            );
            (*jpg).error = (brunsli_JPEGReadError::INVALID_SYMBOL).clone();
            return false;
        }
        let mut r: i32 = ((sr) >> (4));
        let mut s: i32 = ((sr) & (15));
        if ((s) > (0)) {
            k += r;
            if ((k) > (Se)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Out-of-band coefficient {:} band was {:}-{:}\n",
                    k,
                    Ss,
                    Se,
                );
                (*jpg).error = (brunsli_JPEGReadError::OUT_OF_BAND_COEFF).clone();
                return false;
            }
            if (((s) + (Al)) >= (kJpegDCAlphabetSize_9)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Out of range AC coefficient value: s = {:} Al = {:} k = {:}\n",
                    s,
                    Al,
                    k,
                );
                (*jpg).error = (brunsli_JPEGReadError::NON_REPRESENTABLE_AC_COEFF).clone();
                return false;
            }
            let mut bits: i32 = (unsafe { (*br).ReadBits(s) });
            let mut coeff: i32 = (unsafe { HuffExtend_243(bits, s) });
            (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) =
                (((coeff) * (Am)) as i16);
            (*num_zero_runs) = 0;
        } else if ((r) == (15)) {
            k += 15;
            (*num_zero_runs).prefix_inc();
        } else {
            if ((eobrun_allowed) && ((k) == (Ss))) && ((*eobrun) == (0)) {
                (*reset_state) = true;
            }
            (*eobrun) = ((1) << (r));
            if ((r) > (0)) {
                if !eobrun_allowed {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "End-of-block run crossing DC coeff.\n",
                    );
                    (*jpg).error = (brunsli_JPEGReadError::EOB_RUN_TOO_LONG).clone();
                    return false;
                }
                (*eobrun) += (unsafe { (*br).ReadBits(r) });
            }
            break;
        }
        k.postfix_inc();
    }
    (*eobrun).prefix_dec();
    return true;
}
pub unsafe fn RefineDCTBlock_245(
    mut ac_huff: *const brunsli_HuffmanTableEntry,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut eobrun: *mut i32,
    mut reset_state: *mut bool,
    mut br: *mut brunsli_BitReaderState,
    mut jpg: *mut brunsli_JPEGData,
    mut coeffs: *mut i16,
) -> bool {
    let mut Am: i32 = ((1) << (Al));
    let mut eobrun_allowed: bool = ((Ss) > (0));
    if ((Ss) == (0)) {
        let mut s: i32 = (unsafe { (*br).ReadBits(1) });
        let mut dc_coeff: i16 = (*coeffs.offset((0) as isize));
        dc_coeff = ((dc_coeff as i32) | ((s) * (Am))) as i16;
        (*coeffs.offset((0) as isize)) = dc_coeff;
        Ss.prefix_inc();
    }
    if ((Ss) > (Se)) {
        return true;
    }
    let mut p1: i32 = Am;
    let mut m1: i32 = -Am;
    let mut k: i32 = Ss;
    let mut r: i32 = 0_i32;
    let mut s: i32 = 0_i32;
    let mut in_zero_run: bool = false;
    if ((*eobrun) <= (0)) {
        'loop_: while ((k) <= (Se)) {
            s = (unsafe { ReadSymbol_242(ac_huff, br) }).clone();
            if ((s) >= (kJpegHuffmanAlphabetSize_8)) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                    s,
                    k,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_SYMBOL).clone();
                return false;
            }
            r = ((s) >> (4));
            s &= 15;
            if (s != 0) {
                if ((s) != (1)) {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                        s,
                        k,
                    );
                    (*jpg).error = (brunsli_JPEGReadError::INVALID_SYMBOL).clone();
                    return false;
                }
                s = if ((unsafe { (*br).ReadBits(1) }) != 0) {
                    p1
                } else {
                    m1
                };
                in_zero_run = false;
            } else {
                if ((r) != (15)) {
                    if ((eobrun_allowed) && ((k) == (Ss))) && ((*eobrun) == (0)) {
                        (*reset_state) = true;
                    }
                    (*eobrun) = ((1) << (r));
                    if ((r) > (0)) {
                        if !eobrun_allowed {
                            write!(
                                std::fs::File::from_raw_fd(
                                    std::io::stderr()
                                        .as_fd()
                                        .try_clone_to_owned()
                                        .unwrap()
                                        .into_raw_fd(),
                                ),
                                "End-of-block run crossing DC coeff.\n",
                            );
                            (*jpg).error = (brunsli_JPEGReadError::EOB_RUN_TOO_LONG).clone();
                            return false;
                        }
                        (*eobrun) += (unsafe { (*br).ReadBits(r) });
                    }
                    break;
                }
                in_zero_run = true;
            }
            let mut __do_while = true;
            'loop_: while __do_while || ((k) <= (Se)) {
                __do_while = false;
                let mut thiscoef: i16 =
                    (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize));
                if ((thiscoef as i32) != (0)) {
                    if ((unsafe { (*br).ReadBits(1) }) != 0) {
                        if (((thiscoef as i32) & (p1)) == (0)) {
                            if ((thiscoef as i32) >= (0)) {
                                thiscoef = ((thiscoef as i32) + p1) as i16;
                            } else {
                                thiscoef = ((thiscoef as i32) + m1) as i16;
                            }
                        }
                    }
                    (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) = thiscoef;
                } else {
                    if ((r.prefix_dec()) < (0)) {
                        break;
                    }
                }
                k.postfix_inc();
            }
            if (s != 0) {
                if ((k) > (Se)) {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "Out-of-band coefficient {:} band was {:}-{:}\n",
                        k,
                        Ss,
                        Se,
                    );
                    (*jpg).error = (brunsli_JPEGReadError::OUT_OF_BAND_COEFF).clone();
                    return false;
                }
                (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) = (s as i16);
            }
            k.postfix_inc();
        }
    }
    if in_zero_run {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Extra zero run before end-of-block.\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::EXTRA_ZERO_RUN).clone();
        return false;
    }
    if ((*eobrun) > (0)) {
        'loop_: while ((k) <= (Se)) {
            let mut thiscoef: i16 = (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize));
            if ((thiscoef as i32) != (0)) {
                if ((unsafe { (*br).ReadBits(1) }) != 0) {
                    if (((thiscoef as i32) & (p1)) == (0)) {
                        if ((thiscoef as i32) >= (0)) {
                            thiscoef = ((thiscoef as i32) + p1) as i16;
                        } else {
                            thiscoef = ((thiscoef as i32) + m1) as i16;
                        }
                    }
                }
                (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) = thiscoef;
            }
            k.postfix_inc();
        }
    }
    (*eobrun).prefix_dec();
    return true;
}
pub unsafe fn ProcessRestart_246(
    mut data: *const u8,
    len: usize,
    mut next_restart_marker: *mut i32,
    mut br: *mut brunsli_BitReaderState,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let mut pos: usize = 0_usize;
    if !(unsafe { (*br).FinishStream(jpg, (&mut pos as *mut usize)) }) {
        (*jpg).error = (brunsli_JPEGReadError::INVALID_SCAN).clone();
        return false;
    }
    let mut expected_marker: i32 = ((208) + (*next_restart_marker));
    if (((pos).wrapping_add(2_usize)) > (len)) || (((*data.offset((pos) as isize)) as i32) != (255))
    {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((pos) < (len)) {
                ((*data.offset((pos) as isize)) as i32)
            } else {
                0
            }),
            pos,
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND).clone();
        return false;
    };
    let mut marker: i32 = ((*data.offset(((pos).wrapping_add(1_usize)) as isize)) as i32);
    if ((marker) != (expected_marker)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Did not find expected restart marker {:} actual={:}\n",
            expected_marker,
            marker,
        );
        (*jpg).error = (brunsli_JPEGReadError::WRONG_RESTART_MARKER).clone();
        return false;
    }
    (unsafe { (*br).Reset((pos).wrapping_add(2_usize)) });
    (*next_restart_marker) += 1;
    (*next_restart_marker) &= 7;
    return true;
}
pub unsafe fn ProcessScan_247(
    mut data: *const u8,
    len: usize,
    dc_huff_lut: *const Vec<brunsli_HuffmanTableEntry>,
    ac_huff_lut: *const Vec<brunsli_HuffmanTableEntry>,
    mut scan_progression: *mut [u16; 64],
    mut is_progressive: bool,
    mut pos: *mut usize,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    if !(unsafe {
        let _len: usize = len;
        let _pos: *mut usize = pos;
        ProcessSOS_235(data, _len, _pos, jpg)
    }) {
        return false;
    }
    let mut scan_info: *mut brunsli_JPEGScanInfo = (((*jpg).scan_info).last_mut().unwrap());
    let mut is_interleaved: bool = (((*scan_info).num_components) > (1_usize));
    let mut MCUs_per_row: i32 = 0_i32;
    let mut MCU_rows: i32 = 0_i32;
    if is_interleaved {
        MCUs_per_row = (*jpg).MCU_cols;
        MCU_rows = (*jpg).MCU_rows;
    } else {
        let c: *const brunsli_JPEGComponent = &(&mut (*jpg)).components
            [((&mut (*scan_info)).components[(0_usize)].comp_idx as usize)]
            as *const brunsli_JPEGComponent;
        MCUs_per_row = (unsafe {
            let _a: i32 = (((*jpg).width) * ((*c).h_samp_factor));
            let _b: i32 = ((8) * ((*jpg).max_h_samp_factor));
            DivCeil_231(_a, _b)
        });
        MCU_rows = (unsafe {
            let _a: i32 = (((*jpg).height) * ((*c).v_samp_factor));
            let _b: i32 = ((8) * ((*jpg).max_v_samp_factor));
            DivCeil_231(_a, _b)
        });
    }
    let mut last_dc_coeff: [i16; 4] = [0_i16, 0_i16, 0_i16, 0_i16];
    let mut br: brunsli_BitReaderState =
        brunsli_BitReaderState::brunsli_BitReaderState(data, len, (*pos));
    let mut restarts_to_go: i32 = (*jpg).restart_interval;
    let mut next_restart_marker: i32 = 0;
    let mut eobrun: i32 = -1_i32;
    let mut block_scan_index: i32 = 0;
    let Al: i32 = if is_progressive { (*scan_info).Al } else { 0 };
    let Ah: i32 = if is_progressive { (*scan_info).Ah } else { 0 };
    let Ss: i32 = if is_progressive { (*scan_info).Ss } else { 0 };
    let Se: i32 = if is_progressive { (*scan_info).Se } else { 63 };
    let scan_bitmask: u16 = (if ((Ah) == (0)) {
        (((65535) << (Al)) as u32)
    } else {
        ((1_u32) << (Al))
    } as u16);
    let refinement_bitmask: u16 = ((((1) << (Al)) - (1)) as u16);
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*scan_info).num_components)) {
        let mut comp_idx: i32 = ((&mut (*scan_info)).components[(i)].comp_idx as i32);
        let mut k: i32 = Ss;
        'loop_: while ((k) <= (Se)) {
            if ((((*scan_progression.offset((comp_idx) as isize))[(k) as usize] as i32)
                & (scan_bitmask as i32))
                != 0)
            {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Overlapping scans: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    comp_idx,
                    k,
                    (*scan_progression.offset((i) as isize))[(k) as usize],
                    scan_bitmask,
                );
                (*jpg).error = (brunsli_JPEGReadError::OVERLAPPING_SCANS).clone();
                return false;
            }
            if ((((*scan_progression.offset((comp_idx) as isize))[(k) as usize] as i32)
                & (refinement_bitmask as i32))
                != 0)
            {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Invalid scan order, a more refined scan was already done: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    comp_idx,
                    k,
                    (*scan_progression.offset((i) as isize))[(k) as usize],
                    scan_bitmask,
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_SCAN_ORDER).clone();
                return false;
            }
            (*scan_progression.offset((comp_idx) as isize))[(k) as usize] =
                (((*scan_progression.offset((comp_idx) as isize))[(k) as usize] as i32)
                    | (scan_bitmask as i32)) as u16;
            k.prefix_inc();
        }
        i.prefix_inc();
    }
    if ((Al) > (10)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Scan parameter Al = {:} is not supported in brunsli.\n",
            Al,
        );
        (*jpg).error = (brunsli_JPEGReadError::NON_REPRESENTABLE_AC_COEFF).clone();
        return false;
    }
    let mut mcu_y: i32 = 0;
    'loop_: while ((mcu_y) < (MCU_rows)) {
        let mut mcu_x: i32 = 0;
        'loop_: while ((mcu_x) < (MCUs_per_row)) {
            if (((*jpg).restart_interval) > (0)) {
                if ((restarts_to_go) == (0)) {
                    if (unsafe {
                        ProcessRestart_246(
                            data,
                            len,
                            (&mut next_restart_marker as *mut i32),
                            (&mut br as *mut brunsli_BitReaderState),
                            jpg,
                        )
                    }) {
                        restarts_to_go = (*jpg).restart_interval;
                        {
                            let byte_0 = (last_dc_coeff.as_mut_ptr() as *mut i16
                                as *mut ::libc::c_void)
                                as *mut u8;
                            for offset in 0..::std::mem::size_of::<[i16; 4]>() {
                                *byte_0.offset(offset as isize) = 0 as u8;
                            }
                            (last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
                        };
                        if ((eobrun) > (0)) {
                            write!(
                                std::fs::File::from_raw_fd(
                                    std::io::stderr()
                                        .as_fd()
                                        .try_clone_to_owned()
                                        .unwrap()
                                        .into_raw_fd(),
                                ),
                                "End-of-block run too long.\n",
                            );
                            (*jpg).error = (brunsli_JPEGReadError::EOB_RUN_TOO_LONG).clone();
                            return false;
                        }
                        eobrun = -1_i32;
                    } else {
                        return false;
                    }
                }
                restarts_to_go.prefix_dec();
            }
            if (unsafe { br.IsUnhealthy() }) {
                write!(
                    std::fs::File::from_raw_fd(
                        std::io::stderr()
                            .as_fd()
                            .try_clone_to_owned()
                            .unwrap()
                            .into_raw_fd(),
                    ),
                    "Unexpected end of scan.\n",
                );
                (*jpg).error = (brunsli_JPEGReadError::INVALID_SCAN).clone();
                return false;
            }
            let mut i: usize = 0_usize;
            'loop_: while ((i) < ((*scan_info).num_components)) {
                let mut si: *mut brunsli_JPEGComponentScanInfo = (&mut (&mut (*scan_info))
                    .components[(i)]
                    as *mut brunsli_JPEGComponentScanInfo);
                let mut c: *mut brunsli_JPEGComponent = (&mut (&mut (*jpg)).components
                    [((*si).comp_idx as usize)]
                    as *mut brunsli_JPEGComponent);
                let mut dc_lut: *const brunsli_HuffmanTableEntry = (&(&(*dc_huff_lut))
                    [((((*si).dc_tbl_idx) * (kJpegHuffmanLutSize_230)) as usize)]
                    as *const brunsli_HuffmanTableEntry);
                let mut ac_lut: *const brunsli_HuffmanTableEntry = (&(&(*ac_huff_lut))
                    [((((*si).ac_tbl_idx) * (kJpegHuffmanLutSize_230)) as usize)]
                    as *const brunsli_HuffmanTableEntry);
                let mut nblocks_y: i32 = if is_interleaved {
                    (*c).v_samp_factor
                } else {
                    1
                };
                let mut nblocks_x: i32 = if is_interleaved {
                    (*c).h_samp_factor
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while ((iy) < (nblocks_y)) {
                    let mut ix: i32 = 0;
                    'loop_: while ((ix) < (nblocks_x)) {
                        let mut block_y: i32 = (((mcu_y) * (nblocks_y)) + (iy));
                        let mut block_x: i32 = (((mcu_x) * (nblocks_x)) + (ix));
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul((*c).width_in_blocks))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        let mut reset_state: bool = false;
                        let mut num_zero_runs: i32 = 0;
                        let mut coeffs: *mut i16 = (&mut (&mut (*c)).coeffs
                            [(((block_idx) * (kDCTBlockSize_3)) as usize)]
                            as *mut i16);
                        if ((Ah) == (0)) {
                            if !(unsafe {
                                DecodeDCTBlock_244(
                                    dc_lut,
                                    ac_lut,
                                    Ss,
                                    Se,
                                    Al,
                                    (&mut eobrun as *mut i32),
                                    (&mut reset_state as *mut bool),
                                    (&mut num_zero_runs as *mut i32),
                                    (&mut br as *mut brunsli_BitReaderState),
                                    jpg,
                                    (&mut last_dc_coeff[((*si).comp_idx) as usize] as *mut i16),
                                    coeffs,
                                )
                            }) {
                                return false;
                            }
                        } else {
                            if !(unsafe {
                                RefineDCTBlock_245(
                                    ac_lut,
                                    Ss,
                                    Se,
                                    Al,
                                    (&mut eobrun as *mut i32),
                                    (&mut reset_state as *mut bool),
                                    (&mut br as *mut brunsli_BitReaderState),
                                    jpg,
                                    coeffs,
                                )
                            }) {
                                return false;
                            }
                        }
                        if reset_state {
                            (*scan_info).reset_points.push(block_scan_index as i32);
                        }
                        if ((num_zero_runs) > (0)) {
                            let mut info: brunsli_JPEGScanInfo_ExtraZeroRunInfo =
                                <brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default();
                            info.block_idx = block_scan_index;
                            info.num_extra_zero_runs = num_zero_runs;
                            {
                                let a0_clone = info.clone();
                                (*scan_info).extra_zero_runs.push(a0_clone)
                            };
                        }
                        block_scan_index.prefix_inc();
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
    if ((eobrun) > (0)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "End-of-block run too long.\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::EOB_RUN_TOO_LONG).clone();
        return false;
    }
    if !(unsafe { br.FinishStream(jpg, pos) }) {
        (*jpg).error = (brunsli_JPEGReadError::INVALID_SCAN).clone();
        return false;
    }
    if ((*pos) > (len)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Unexpected end of file during scan. pos={:} len={:}\n",
            (*pos),
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::UNEXPECTED_EOF).clone();
        return false;
    }
    return true;
}
pub unsafe fn FixupIndexes_248(mut jpg: *mut brunsli_JPEGData) -> bool {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*jpg).components.len())) {
        let mut c: *mut brunsli_JPEGComponent =
            (&mut (&mut (*jpg)).components[(i)] as *mut brunsli_JPEGComponent);
        let mut found_index: bool = false;
        let mut j: usize = 0_usize;
        'loop_: while ((j) < ((*jpg).quant.len())) {
            if (((&mut (*jpg)).quant[(j)].index) == ((*c).quant_idx as i32)) {
                (*c).quant_idx = (j as u8);
                found_index = true;
                break;
            }
            j.prefix_inc();
        }
        if !found_index {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Quantization table with index ",
            );
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            )
            .write_all(
                &([
                    (&[(*c).quant_idx as u8] as &[u8]),
                    (b" not found." as &[u8]),
                    (&[b'\n'] as &[u8]),
                ]
                .concat()),
            );
            (*jpg).error = (brunsli_JPEGReadError::QUANT_TABLE_NOT_FOUND).clone();
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn FindNextMarker_249(mut data: *const u8, len: usize, mut pos: usize) -> usize {
    static mut kIsValidMarker_250: [u8; 64] = unsafe {
        [
            1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8,
        ]
    };;
    let mut num_skipped: usize = 0_usize;
    'loop_: while (((pos).wrapping_add(1_usize)) < (len))
        && (((((*data.offset((pos) as isize)) as i32) != (255))
            || (((*data.offset(((pos).wrapping_add(1_usize)) as isize)) as i32) < (192)))
            || (!(kIsValidMarker_250[(((*data.offset(((pos).wrapping_add(1_usize)) as isize))
                as i32)
                - (192)) as usize]
                != 0)))
    {
        pos.prefix_inc();
        num_skipped.prefix_inc();
    }
    return num_skipped;
}
pub unsafe fn ReadJpeg_195(
    mut data: *const u8,
    len: usize,
    mut mode: brunsli_JpegReadMode,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let mut pos: usize = 0_usize;
    if (((pos).wrapping_add(2_usize)) > (len)) || (((*data.offset((pos) as isize)) as i32) != (255))
    {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((pos) < (len)) {
                ((*data.offset((pos) as isize)) as i32)
            } else {
                0
            }),
            pos,
            len,
        );
        (*jpg).error = (brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND).clone();
        return false;
    };
    let mut marker: i32 = ((*data.offset(((pos).wrapping_add(1_usize)) as isize)) as i32);
    pos = (pos).wrapping_add(2_usize);
    if ((marker) != (216)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Did not find expected SOI marker, actual={:}\n",
            marker,
        );
        (*jpg).error = (brunsli_JPEGReadError::SOI_NOT_FOUND).clone();
        return false;
    }
    let mut lut_size: i32 = ((kMaxHuffmanTables_6) * (kJpegHuffmanLutSize_230));
    let mut dc_huff_lut: Vec<brunsli_HuffmanTableEntry> = (0..(lut_size as usize) as usize)
        .map(|_| <brunsli_HuffmanTableEntry>::default())
        .collect::<Vec<_>>();
    let mut ac_huff_lut: Vec<brunsli_HuffmanTableEntry> = (0..(lut_size as usize) as usize)
        .map(|_| <brunsli_HuffmanTableEntry>::default())
        .collect::<Vec<_>>();
    let mut found_sof: bool = false;
    let mut found_dri: bool = false;
    let mut scan_progression: [[u16; 64]; 4] = [
        [
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16,
        ],
        [0; 64],
        [0; 64],
        [0; 64],
    ];
    {
        let __a0 = 0_usize as usize;
        (*jpg).padding_bits.resize_with(__a0, || <i32>::default())
    };
    let mut is_progressive: bool = false;
    let mut __do_while = true;
    'loop_: while __do_while || ((marker) != (217)) {
        __do_while = false;
        let mut num_skipped: usize = (unsafe { FindNextMarker_249(data, len, pos) });
        if ((num_skipped) > (0_usize)) {
            (*jpg).marker_order.push(255_u8);
            (*jpg).inter_marker_data.push(
                core::slice::from_raw_parts(
                    data.offset((pos) as isize),
                    (data.offset((pos) as isize).offset((num_skipped) as isize))
                        .offset_from(data.offset((pos) as isize)) as usize,
                )
                .to_vec(),
            );
            pos = (pos).wrapping_add(num_skipped);
        }
        if (((pos).wrapping_add(2_usize)) > (len))
            || (((*data.offset((pos) as isize)) as i32) != (255))
        {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
                (if ((pos) < (len)) {
                    ((*data.offset((pos) as isize)) as i32)
                } else {
                    0
                }),
                pos,
                len,
            );
            (*jpg).error = (brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND).clone();
            return false;
        };
        marker = ((*data.offset(((pos).wrapping_add(1_usize)) as isize)) as i32);
        pos = (pos).wrapping_add(2_usize);
        let mut ok: bool = true;
        'switch: {
            let __match_cond = marker;
            match __match_cond {
                __v if __v == 192 || __v == 193 || __v == 194 => {
                    is_progressive = ((marker) == (194));
                    ok =
                        (unsafe { ProcessSOF_234(data, len, mode, (&mut pos as *mut usize), jpg) })
                            .clone();
                    found_sof = true;
                    break 'switch;
                }
                __v if __v == 196 => {
                    ok = (unsafe {
                        ProcessDHT_236(
                            data,
                            len,
                            mode,
                            (&mut dc_huff_lut as *mut Vec<brunsli_HuffmanTableEntry>),
                            (&mut ac_huff_lut as *mut Vec<brunsli_HuffmanTableEntry>),
                            (&mut pos as *mut usize),
                            jpg,
                        )
                    })
                    .clone();
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
                    if ((mode as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
                        ok = (unsafe {
                            ProcessScan_247(
                                data,
                                len,
                                &dc_huff_lut as *const Vec<brunsli_HuffmanTableEntry>,
                                &ac_huff_lut as *const Vec<brunsli_HuffmanTableEntry>,
                                scan_progression.as_mut_ptr(),
                                is_progressive,
                                (&mut pos as *mut usize),
                                jpg,
                            )
                        })
                        .clone();
                    }
                    break 'switch;
                }
                __v if __v == 219 => {
                    ok = (unsafe { ProcessDQT_238(data, len, (&mut pos as *mut usize), jpg) })
                        .clone();
                    break 'switch;
                }
                __v if __v == 221 => {
                    ok = (unsafe {
                        ProcessDRI_239(
                            data,
                            len,
                            (&mut pos as *mut usize),
                            (&mut found_dri as *mut bool),
                            jpg,
                        )
                    })
                    .clone();
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
                    if ((mode as i32) != (brunsli_JpegReadMode::JPEG_READ_TABLES as i32)) {
                        ok = (unsafe { ProcessAPP_240(data, len, (&mut pos as *mut usize), jpg) })
                            .clone();
                    }
                    break 'switch;
                }
                __v if __v == 254 => {
                    if ((mode as i32) != (brunsli_JpegReadMode::JPEG_READ_TABLES as i32)) {
                        ok = (unsafe { ProcessCOM_241(data, len, (&mut pos as *mut usize), jpg) })
                            .clone();
                    }
                    break 'switch;
                }
                _ => {
                    write!(
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        ),
                        "Unsupported marker: {:} pos={:} len={:}\n",
                        marker,
                        pos,
                        len,
                    );
                    (*jpg).error = (brunsli_JPEGReadError::UNSUPPORTED_MARKER).clone();
                    ok = false;
                    break 'switch;
                }
            }
        };
        if !ok {
            return false;
        }
        (*jpg).marker_order.push((marker as u8));
        if ((mode as i32) == (brunsli_JpegReadMode::JPEG_READ_HEADER as i32)) && (found_sof) {
            break;
        }
    }
    if !found_sof {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Missing SOF marker.\n",
        );
        (*jpg).error = (brunsli_JPEGReadError::SOF_NOT_FOUND).clone();
        return false;
    }
    if ((mode as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
        if ((pos) < (len)) {
            (*jpg).tail_data = core::slice::from_raw_parts(
                data.offset((pos) as isize),
                (data.offset((len) as isize)).offset_from(data.offset((pos) as isize)) as usize,
            )
            .to_vec();
        }
        if !(unsafe { FixupIndexes_248(jpg) }) {
            return false;
        }
        if (*jpg).huffman_code.is_empty() {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Need at least one Huffman code table.\n",
            );
            (*jpg).error = (brunsli_JPEGReadError::HUFFMAN_TABLE_ERROR).clone();
            return false;
        }
        if (((*jpg).huffman_code.len()) >= (kMaxDHTMarkers_10 as usize)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Too many Huffman tables.\n",
            );
            (*jpg).error = (brunsli_JPEGReadError::HUFFMAN_TABLE_ERROR).clone();
            return false;
        }
    }
    return true;
}
pub unsafe fn NextTableBitSize_251(mut count: *const i32, mut len: i32) -> i32 {
    let mut left: i32 = ((1) << ((len) - (kJpegHuffmanRootTableBits_229)));
    'loop_: while ((len) < (kJpegHuffmanMaxBitLength_7)) {
        left -= (*count.offset((len) as isize));
        if ((left) <= (0)) {
            break;
        }
        len.prefix_inc();
        left <<= 1;
    }
    return ((len) - (kJpegHuffmanRootTableBits_229));
}
pub unsafe fn BuildJpegHuffmanTable_237(
    mut count: *const i32,
    mut symbols: *const i32,
    mut lut: *mut brunsli_HuffmanTableEntry,
) {
    let mut code: brunsli_HuffmanTableEntry =
        brunsli_HuffmanTableEntry::brunsli_HuffmanTableEntry();
    let mut table: *mut brunsli_HuffmanTableEntry = std::ptr::null_mut();
    let mut len: i32 = 0_i32;
    let mut idx: i32 = 0_i32;
    let mut key: i32 = 0_i32;
    let mut reps: i32 = 0_i32;
    let mut low: i32 = 0_i32;
    let mut table_bits: i32 = 0_i32;
    let mut table_size: i32 = 0_i32;
    let mut tmp_count: [i32; 17] = [
        0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32,
    ];
    let mut total_count: i32 = 0;
    len = 1;
    'loop_: while ((len) <= (kJpegHuffmanMaxBitLength_7)) {
        tmp_count[(len) as usize] = (*count.offset((len) as isize));
        total_count += tmp_count[(len) as usize];
        len.prefix_inc();
    }
    table = lut;
    table_bits = kJpegHuffmanRootTableBits_229;
    table_size = ((1) << (table_bits));
    if ((total_count) == (1)) {
        code.bits = 0_u8;
        code.value = ((*symbols.offset((0) as isize)) as u16);
        key = 0;
        'loop_: while ((key) < (table_size)) {
            (*table.offset((key) as isize)) = code;
            key.prefix_inc();
        }
        return;
    }
    key = 0;
    idx = 0;
    len = 1;
    'loop_: while ((len) <= (kJpegHuffmanRootTableBits_229)) {
        'loop_: while ((tmp_count[(len) as usize]) > (0)) {
            code.bits = (len as u8);
            code.value = ((*symbols.offset((idx.postfix_inc()) as isize)) as u16);
            reps = ((1) << ((kJpegHuffmanRootTableBits_229) - (len)));
            'loop_: while (reps.postfix_dec() != 0) {
                (*table.offset((key.postfix_inc()) as isize)) = code;
            }
            tmp_count[(len) as usize].prefix_dec();
        }
        len.prefix_inc();
    }
    table = (table).wrapping_add(table_size as usize);
    table_size = 0;
    low = 0;
    len = ((kJpegHuffmanRootTableBits_229) + (1));
    'loop_: while ((len) <= (kJpegHuffmanMaxBitLength_7)) {
        'loop_: while ((tmp_count[(len) as usize]) > (0)) {
            if ((low) >= (table_size)) {
                table = (table).wrapping_add(table_size as usize);
                table_bits =
                    (unsafe { NextTableBitSize_251((tmp_count.as_mut_ptr()).cast_const(), len) });
                table_size = ((1) << (table_bits));
                low = 0;
                (*lut.offset((key) as isize)).bits =
                    (((table_bits) + (kJpegHuffmanRootTableBits_229)) as u8);
                (*lut.offset((key) as isize)).value = (((((table as usize - lut as usize)
                    / ::std::mem::size_of::<brunsli_HuffmanTableEntry>())
                    as i64)
                    - (key as i64)) as u16);
                key.prefix_inc();
            }
            code.bits = (((len) - (kJpegHuffmanRootTableBits_229)) as u8);
            code.value = ((*symbols.offset((idx.postfix_inc()) as isize)) as u16);
            reps = ((1) << ((table_bits) - (code.bits as i32)));
            'loop_: while (reps.postfix_dec() != 0) {
                (*table.offset((low.postfix_inc()) as isize)) = code;
            }
            tmp_count[(len) as usize].prefix_dec();
        }
        len.prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_Storage {
    pub data: *mut u8,
    pub length: usize,
    pub pos: usize,
}
impl brunsli_Storage {
    pub unsafe fn brunsli_Storage(mut data: *mut u8, mut length: usize) -> Self {
        let mut this = Self {
            data: data,
            length: length,
            pos: 0_usize,
        };
        if !((length) > (0_usize)) {
            (unsafe { BrunsliDumpAndAbort_79(c"write_bits.cc".as_ptr(), 14, c"Storage".as_ptr()) });
            'loop_: while true {}
        };
        (*data.offset((0) as isize)) = 0_u8;
        this
    }
    pub unsafe fn GetBytesUsed(&self) -> usize {
        return (((self.pos).wrapping_add(7_usize)) >> (3));
    }
}
impl brunsli_Storage {}
impl brunsli_Storage {
    pub unsafe fn AppendBytes(&mut self, mut src: *const u8, mut len: usize) {
        if !(((self.pos) & (7_usize)) == (0_usize)) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"write_bits.cc".as_ptr(), 19, c"AppendBytes".as_ptr())
            });
            'loop_: while true {}
        };
        if !(((unsafe { self.GetBytesUsed() }).wrapping_add(len)) <= (self.length)) {
            (unsafe {
                BrunsliDumpAndAbort_79(c"write_bits.cc".as_ptr(), 20, c"AppendBytes".as_ptr())
            });
            'loop_: while true {}
        };
        {
            if len != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (src as *const u8 as *const ::libc::c_void),
                    (self.data.offset(((self.pos) >> (3)) as isize) as *mut u8
                        as *mut ::libc::c_void),
                    len as usize,
                )
            }
            (self.data.offset(((self.pos) >> (3)) as isize) as *mut u8 as *mut ::libc::c_void)
        };
        self.pos = (self.pos).wrapping_add((8_usize).wrapping_mul(len));
    }
}
pub unsafe fn ReadFileInternal_252(
    mut file: *mut ::libc::FILE,
    mut content: *mut Vec<libc::c_char>,
) -> bool {
    if ((libc::fseek(file, 0_i64 as ::libc::c_long, ::libc::SEEK_END)) != (0)) {
        printf(c"Failed to seek end of input file.\n".as_ptr() as *const i8);
        return false;
    }
    let mut input_size: i32 = (libc::ftell(file) as i64 as i32);
    if ((input_size) == (0)) {
        printf(c"Input file is empty.\n".as_ptr() as *const i8);
        return false;
    }
    if ((libc::fseek(file, 0_i64 as ::libc::c_long, ::libc::SEEK_SET)) != (0)) {
        printf(c"Failed to rewind input file to the beginning.\n".as_ptr() as *const i8);
        return false;
    }
    {
        (*content).pop();
        (*content).resize((input_size as usize) as usize, 0);
        (*content).push(0)
    };
    let mut read_pos: usize = 0_usize;
    'loop_: while ((read_pos) < ((*(content).cast_const()).len() - 1)) {
        let bytes_read: usize = libcc2rs::fread_unsafe(
            ((if read_pos as usize >= (*content).len() - 1 {
                panic!("out of bounds access")
            } else {
                &mut (&mut (*content))[read_pos as usize]
            }) as *mut libc::c_char as *mut ::libc::c_void),
            1_usize,
            ((((*(content).cast_const()).len() - 1) as u64).wrapping_sub((read_pos as u64))
                as usize),
            file,
        );
        if ((bytes_read) == (0_usize)) {
            printf(c"Failed to read input file\n".as_ptr() as *const i8);
            return false;
        }
        read_pos = (read_pos).wrapping_add(bytes_read);
    }
    return true;
}
pub unsafe fn ReadFile_253(
    file_name: *const Vec<libc::c_char>,
    mut content: *mut Vec<libc::c_char>,
) -> bool {
    let mut file: *mut ::libc::FILE = libc::fopen((*file_name).as_ptr(), c"rb".as_ptr());
    if (file).is_null() {
        printf(c"Failed to open input file.\n".as_ptr() as *const i8);
        return false;
    }
    let mut ok: bool = (unsafe { ReadFileInternal_252(file, content) });
    if ((libc::fclose(file)) != (0)) {
        if ok {
            printf(c"Failed to close input file.\n".as_ptr() as *const i8);
        }
        return false;
    }
    return ok;
}
pub unsafe fn WriteFileInternal_254(
    mut file: *mut ::libc::FILE,
    content: *const Vec<libc::c_char>,
) -> bool {
    let mut write_pos: usize = 0_usize;
    'loop_: while ((write_pos) < ((*content).len() - 1)) {
        let bytes_written: usize = libcc2rs::fwrite_unsafe(
            ((&(&(*content))[(write_pos)] as *const libc::c_char) as *const libc::c_char
                as *const ::libc::c_void),
            1_usize,
            ((((*content).len() - 1) as u64).wrapping_sub((write_pos as u64)) as usize),
            file,
        );
        if ((bytes_written) == (0_usize)) {
            printf(c"Failed to write output.\n".as_ptr() as *const i8);
            return false;
        }
        write_pos = (write_pos).wrapping_add(bytes_written);
    }
    return true;
}
pub unsafe fn WriteFile_255(
    file_name: *const Vec<libc::c_char>,
    content: *const Vec<libc::c_char>,
) -> bool {
    let mut file: *mut ::libc::FILE = libc::fopen((*file_name).as_ptr(), c"wb".as_ptr());
    if (file).is_null() {
        printf(c"Failed to open file for writing.\n".as_ptr() as *const i8);
        return false;
    }
    let mut ok: bool = (unsafe {
        let _file: *mut ::libc::FILE = file;
        let _content: *const Vec<libc::c_char> = content;
        WriteFileInternal_254(_file, _content)
    });
    if ((libc::fclose(file)) != (0)) {
        if ok {
            printf(c"Failed to close output file.\n".as_ptr() as *const i8);
        }
        return false;
    }
    return ok;
}
pub unsafe fn ProcessFile_256(
    file_name: *const Vec<libc::c_char>,
    outfile_name: *const Vec<libc::c_char>,
) -> bool {
    let mut input: Vec<libc::c_char> = vec![0];
    let mut ok: bool = (unsafe {
        let _file_name: *const Vec<libc::c_char> = file_name;
        let _content: *mut Vec<libc::c_char> = (&mut input as *mut Vec<libc::c_char>);
        ReadFile_253(_file_name, _content)
    });
    if !ok {
        return false;
    }
    let mut output: Vec<libc::c_char> = vec![0];
    let mut jpg: brunsli_JPEGData = brunsli_JPEGData::brunsli_JPEGData();
    let mut input_data: *const u8 = (input.as_ptr() as *const u8);
    ok = (unsafe {
        ReadJpeg_195(
            input_data,
            (input.len() - 1),
            brunsli_JpegReadMode::JPEG_READ_ALL,
            (&mut jpg as *mut brunsli_JPEGData),
        )
    })
    .clone();
    {
        input.clear();
        input.push(0)
    };
    input.shrink_to_fit();
    if !ok {
        printf(c"Failed to parse JPEG input.\n".as_ptr() as *const i8);
        return false;
    }
    let mut output_size: usize =
        (unsafe { GetMaximumBrunsliEncodedSize_144(&jpg as *const brunsli_JPEGData) });
    {
        output.pop();
        output.resize((output_size) as usize, 0);
        output.push(0)
    };
    let mut output_data: *mut u8 =
        ((&mut output[(0_usize)] as *mut libc::c_char) as *mut libc::c_char as *mut u8);
    ok = (unsafe {
        BrunsliEncodeJpeg_190(
            &jpg as *const brunsli_JPEGData,
            output_data,
            (&mut output_size as *mut usize),
        )
    })
    .clone();
    if !ok {
        printf(c"Failed to transform JPEG to Brunsli\n".as_ptr() as *const i8);
        return false;
    }
    {
        output.pop();
        output.resize((output_size) as usize, 0);
        output.push(0)
    };
    ok = (unsafe {
        let _file_name: *const Vec<libc::c_char> = outfile_name;
        let _content: *const Vec<libc::c_char> = &output as *const Vec<libc::c_char>;
        WriteFile_255(_file_name, _content)
    });
    return ok;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32) }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut libc::c_char) -> i32 {
    if ((argc) != (2)) && ((argc) != (3)) {
        printf(c"Usage: cbrunsli FILE [OUTPUT_FILE, default=FILE.brn]\n".as_ptr() as *const i8);
        return 1;
    }
    let file_name: Vec<libc::c_char> = {
        let s = (*argv.offset((1) as isize)).cast_const();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    if file_name.len() <= 1 {
        printf(c"Empty input file name.\n".as_ptr() as *const i8);
        return 1;
    }
    let outfile_name: Vec<libc::c_char> = if ((argc) == (2)) {
        {
            let mut __tmp2 = file_name.clone();
            __tmp2.pop();
            let __from = c".brn".as_ptr();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                __from,
                (0..).position(|i| *__from.add(i) == 0).unwrap(),
            ));
            __tmp2.push(0);
            __tmp2
        }
    } else {
        {
            let s = (*argv.offset((2) as isize)).cast_const();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    };
    let mut ok: bool = (unsafe {
        ProcessFile_256(
            &file_name as *const Vec<libc::c_char>,
            &outfile_name as *const Vec<libc::c_char>,
        )
    });
    return if ok { 0 } else { 1 };
}
