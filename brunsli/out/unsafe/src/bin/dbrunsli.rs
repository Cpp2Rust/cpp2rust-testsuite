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
    pub num_components: u64,
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
            num_components: 0_u64,
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
    pub original_jpg_size: u64,
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
            original_jpg_size: 0_u64,
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
    return (((((((((((*jpg).components.len() as u64) == (3_u64))
        && (((*jpg).max_h_samp_factor) == (2)))
        && (((*jpg).max_v_samp_factor) == (2)))
        && (((&(*jpg)).components[(0_u64) as usize].h_samp_factor) == (2)))
        && (((&(*jpg)).components[(0_u64) as usize].v_samp_factor) == (2)))
        && (((&(*jpg)).components[(1_u64) as usize].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_u64) as usize].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_u64) as usize].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_u64) as usize].v_samp_factor) == (1)));
}
pub unsafe fn JPEGDataIs444_16(jpg: *const brunsli_JPEGData) -> bool {
    return (((((((((((*jpg).components.len() as u64) == (3_u64))
        && (((*jpg).max_h_samp_factor) == (1)))
        && (((*jpg).max_v_samp_factor) == (1)))
        && (((&(*jpg)).components[(0_u64) as usize].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(0_u64) as usize].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_u64) as usize].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(1_u64) as usize].v_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_u64) as usize].h_samp_factor) == (1)))
        && (((&(*jpg)).components[(2_u64) as usize].v_samp_factor) == (1)));
}
pub unsafe fn PaddingBitsLimit_17(jpg: *const brunsli_JPEGData) -> u64 {
    let num_blocks: u64 = ((((*jpg).width as u64).wrapping_add(15_u64)) >> (3_u32))
        .wrapping_mul(((((*jpg).height as u64).wrapping_add(15_u64)) >> (3_u32)));
    return (((7_u64).wrapping_mul(num_blocks)).wrapping_mul((*jpg).components.len() as u64))
        .wrapping_add(256_u64);
}
pub static mut kBrunsliMaxNumBlocks_18: u64 = unsafe { ((1_u64) << (21)) };
pub static mut kBrunsliMaxDCAbsVal_19: i32 = unsafe { 2054 };
pub static mut kMaxContextMapAlphabetSize_20: u64 = unsafe { 272_u64 };
pub static mut kHuffmanTableBits_21: u32 = unsafe { 8_u32 };
pub static mut kMaxHuffmanBits_22: u64 = unsafe { 15_u64 };
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
pub static mut kBrunsliSignatureSize_43: u64 = unsafe { 6_u64 };
pub static mut kMaxApp0Densities_45: u64 = unsafe { 8_u64 };
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
        (unsafe {
            let _tag: u8 = kBrunsliSignatureTag_30;
            SectionMarker_29(_tag)
        }),
        4_u8,
        ('B' as u8),
        210_u8,
        213_u8,
        ('N' as u8),
    ]
};
pub static mut AppData_0xe0_62: [u8; 17] = unsafe {
    [
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
    ]
};
pub static mut AppData_0xec_64: [u8; 18] = unsafe {
    [
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
    ]
};
pub static mut AppData_0xee_65: [u8; 15] = unsafe {
    [
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
        if ::std::mem::size_of::<u16>() as u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u16) as *mut u16 as *mut ::libc::c_void),
                ::std::mem::size_of::<u16>() as u64 as usize,
            )
        }
        ((&mut t as *mut u16) as *mut u16 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedWrite16_67(mut p: *mut ::libc::c_void, mut v: u16) {
    {
        if ::std::mem::size_of::<u16>() as u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut v as *mut u16) as *const u16 as *const ::libc::c_void),
                p,
                ::std::mem::size_of::<u16>() as u64 as usize,
            )
        }
        p
    };
}
pub unsafe fn BrunsliUnalignedRead32_68(mut p: *const ::libc::c_void) -> u32 {
    let mut t: u32 = 0_u32;
    {
        if ::std::mem::size_of::<u32>() as u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u32) as *mut u32 as *mut ::libc::c_void),
                ::std::mem::size_of::<u32>() as u64 as usize,
            )
        }
        ((&mut t as *mut u32) as *mut u32 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedRead64_69(mut p: *const ::libc::c_void) -> u64 {
    let mut t: u64 = 0_u64;
    {
        if ::std::mem::size_of::<u64>() as u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                p,
                ((&mut t as *mut u64) as *mut u64 as *mut ::libc::c_void),
                ::std::mem::size_of::<u64>() as u64 as usize,
            )
        }
        ((&mut t as *mut u64) as *mut u64 as *mut ::libc::c_void)
    };
    return t;
}
pub unsafe fn BrunsliUnalignedWrite64_70(mut p: *mut ::libc::c_void, mut v: u64) {
    {
        if ::std::mem::size_of::<u64>() as u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut v as *mut u64) as *const u64 as *const ::libc::c_void),
                p,
                ::std::mem::size_of::<u64>() as u64 as usize,
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
pub unsafe fn Append_72(mut dst: *mut Vec<u8>, mut begin: *const u8, mut length: u64) {
    (unsafe {
        let _dst: *mut Vec<u8> = dst;
        let _begin: *const u8 = begin;
        let _end: *const u8 = begin.offset((length) as isize);
        Append_71(_dst, _begin, _end)
    });
}
pub unsafe fn Append_73(mut dst: *mut Vec<u8>, src: *const Vec<u8>) {
    (unsafe {
        let _dst: *mut Vec<u8> = dst;
        let _begin: *const u8 = (*src).as_ptr();
        let _length: u64 = (*src).len() as u64;
        Append_72(_dst, _begin, _length)
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
        (unsafe {
            let _f: *const u8 = b"context.cc\0".as_ptr();
            let _l: i32 = 55;
            let _fn: *const u8 = b"FastDivide\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
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
pub static mut kMaxAverageContext_82: u64 = unsafe { 8_u64 };
pub static mut kNumAvrgContexts_83: u64 = unsafe { (kMaxAverageContext_82).wrapping_add(1_u64) };
pub static mut kNumNonZeroBits_84: u64 = unsafe { 6_u64 };
pub static mut kNumNonZeroTreeSize_85: u64 =
    unsafe { (((((1_u32) << (kNumNonZeroBits_84)) as u32).wrapping_sub(1_u32 as u32)) as u64) };
pub static mut kNumNonZeroQuant_86: u64 = unsafe { 2_u64 };
pub static mut kNumNonZeroContextMax_87: u64 =
    unsafe { (kNumNonZeroTreeSize_85).wrapping_div(kNumNonZeroQuant_86) };
pub static mut kNumNonZeroContextCount_88: u64 =
    unsafe { (kNumNonZeroContextMax_87).wrapping_add(1_u64) };
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
pub unsafe fn ZeroDensityContext_96(mut nonzeros_left: u64, mut k: u64, mut bits: u64) -> u16 {
    return (((kNumNonzeroContext_93[(bits) as usize][(nonzeros_left) as usize] as i32)
        + (kFreqContext_92[(bits) as usize][(k) as usize] as i32)) as u16);
}
pub unsafe fn WeightedAverageContextDC_97(mut vals: *const i32, mut x: i32) -> i32 {
    let mut sum: i32 = (((((1) + (*vals.offset(((x) - (2)) as isize)))
        + (*vals.offset(((x) - (1)) as isize)))
        + (*vals.offset((x) as isize)))
        + (*vals.offset(((x) + (1)) as isize)));
    if (((sum) >> (kMaxAverageContext_82)) != (0)) {
        return (kMaxAverageContext_82 as i32);
    }
    return (unsafe {
        let _n: u32 = (sum as u32);
        Log2FloorNonZero_74(_n)
    });
}
pub unsafe fn WeightedAverageContext_98(mut vals: *const i32, mut prev_row_delta: i32) -> i32 {
    let mut sum: i32 = ((((((4) + (*vals.offset((0) as isize)))
        + (((*vals.offset((-kDCTBlockSize_3) as isize))
            + (*vals.offset((prev_row_delta) as isize)))
            * (2)))
        + (*vals.offset(((-2_i32) * (kDCTBlockSize_3)) as isize)))
        + (*vals.offset(((prev_row_delta) - (kDCTBlockSize_3)) as isize)))
        + (*vals.offset(((prev_row_delta) + (kDCTBlockSize_3)) as isize)));
    if (((sum) >> ((kMaxAverageContext_82).wrapping_add(2_u64))) != (0)) {
        return (kMaxAverageContext_82 as i32);
    }
    return ((unsafe {
        let _n: u32 = (sum as u32);
        Log2FloorNonZero_74(_n)
    }) - (2));
}
pub static mut kACPredictPrecisionBits_99: i32 = unsafe { 13 };
pub static mut kACPredictPrecision_100: i32 = unsafe { ((1) << (kACPredictPrecisionBits_99)) };
pub unsafe fn ACPredictContext_101(mut p: i64, mut avg_ctx: *mut u64, mut sgn: *mut u64) {
    let mut multiplier: i32 = 0_i32;
    if ((p) >= (0_i64)) {
        multiplier = 1;
    } else {
        multiplier = -1_i32;
        p = -p;
    }
    let mut ctx: u64 = 0_u64;
    if ((p) >= (((1_u32) << (kMaxAverageContext_82)) as i64)) {
        ctx = kMaxAverageContext_82;
    } else {
        ctx = ((unsafe {
            let _n: u32 = ((2_u32).wrapping_mul((p as u32))).wrapping_add(1_u32);
            Log2FloorNonZero_74(_n)
        }) as u64);
    }
    (*avg_ctx) = ctx;
    (*sgn) = (kMaxAverageContext_82).wrapping_add((multiplier as u64).wrapping_mul(ctx));
}
pub unsafe fn ACPredictContextCol_102(
    mut prev: *const i16,
    mut cur: *const i16,
    mut mult: *const i32,
    mut avg_ctx: *mut u64,
    mut sgn: *mut u64,
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
        let _p: i64 =
            (((*prev.offset((0) as isize)) as i64) - ((delta) / (kACPredictPrecision_100 as i64)));
        let _avg_ctx: *mut u64 = avg_ctx;
        let _sgn: *mut u64 = sgn;
        ACPredictContext_101(_p, _avg_ctx, _sgn)
    });
}
pub unsafe fn ACPredictContextRow_103(
    mut prev: *const i16,
    mut cur: *const i16,
    mut mult: *const i32,
    mut avg_ctx: *mut u64,
    mut sgn: *mut u64,
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
        let _p: i64 =
            (((*prev.offset((0) as isize)) as i64) - ((delta) / (kACPredictPrecision_100 as i64)));
        let _avg_ctx: *mut u64 = avg_ctx;
        let _sgn: *mut u64 = sgn;
        ACPredictContext_101(_p, _avg_ctx, _sgn)
    });
}
pub unsafe fn NumNonzerosContext_104(mut prev: *const u8, mut x: i32, mut y: i32) -> u8 {
    let mut prediction: u64 = 0_u64;
    if ((y) == (0)) {
        if ((x) == (0)) {
            prediction = 0_u64;
        } else {
            prediction = ((*prev.offset(((x) - (1)) as isize)) as u64);
        }
    } else if ((x) == (0)) {
        prediction = ((*prev.offset((x) as isize)) as u64);
    } else {
        prediction = ((((((*prev.offset(((x) - (1)) as isize)) as i32)
            + ((*prev.offset((x) as isize)) as i32))
            + (1))
            / (2)) as u64);
    }
    if !((prediction) <= (kNumNonZeroTreeSize_85)) {
        (unsafe {
            let _f: *const u8 = b"context.cc\0".as_ptr();
            let _l: i32 = 305;
            let _fn: *const u8 = b"NumNonzerosContext\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
            is_empty_block_prob: (0..(kNumIsEmptyBlockContexts_105 as u64) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            sign_prob: (0..(9_u64) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            first_extra_bit_prob: (0..(10_u64) as usize)
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
            let __a0 = (((w) + (1)) as u64) as usize;
            self.prev_is_nonempty.resize(__a0, 1)
        };
        {
            let __a0 = (((w) + (3)) as u64) as usize;
            self.prev_abs_coeff.resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((w) + (1)) as u64) as usize;
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
            is_zero_prob: (0..(((kNumNonzeroBuckets_90 as i32) * (kDCTBlockSize_3)) as u64)
                as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            sign_prob: (0..(((((2_u64).wrapping_mul(kMaxAverageContext_82) as u64)
                .wrapping_add(1_u64)) as u64)
                .wrapping_mul((kDCTBlockSize_3 as u64))) as usize)
                .map(|_| <brunsli_Prob>::default())
                .collect::<Vec<_>>(),
            num_nonzero_prob: std::array::from_fn::<_, 2016, _>(|_| brunsli_Prob::brunsli_Prob()),
            first_extra_bit_prob: (0..(((10) * (kDCTBlockSize_3)) as u64) as usize)
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
            let __a0 = (((w) + (1)) as u64) as usize;
            self.prev_is_nonempty.resize(__a0, 1)
        };
        {
            let __a0 = (w as u64) as usize;
            self.prev_num_nonzeros.resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 = ((((kDCTBlockSize_3) * (2)) * ((w) + (3))) as u64) as usize;
            self.prev_abs_coeff.resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((kDCTBlockSize_3) * ((w) + (1))) as u64) as usize;
            self.prev_sign.resize_with(__a0, || <i32>::default())
        };
    }
    pub unsafe fn SizeInBytes(mut w: i32) -> u64 {
        return (((((4) + (((10) + ((3) * (w))) * (kDCTBlockSize_3))) + ((2) * (w))) as u64)
            .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64))
        .wrapping_add(
            (((((((kNumNonzeroBuckets_90 as u64)
                .wrapping_add((2_u64).wrapping_mul(kMaxAverageContext_82) as u64)
                as u64)
                .wrapping_add(11_u64)) as u64)
                .wrapping_mul((kDCTBlockSize_3 as u64)) as u64)
                .wrapping_add(
                    (kNumNonZeroContextCount_88).wrapping_mul(kNumNonZeroTreeSize_85) as u64,
                )) as u64)
                .wrapping_mul(::std::mem::size_of::<brunsli_Prob>() as u64 as u64)
                as u64,
        );
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
    let mut y: u64 = 0_u64;
    'loop_: while ((y) < (8_u64)) {
        let mut x: u64 = 0_u64;
        'loop_: while ((x) < (8_u64)) {
            (*mult_row.offset(((x).wrapping_add((8_u64).wrapping_mul(y))) as isize)) = (((*quant
                .offset(((x).wrapping_add((8_u64).wrapping_mul(y))) as isize))
                * (kSqrt2FixedPoint_108))
                / (*quant.offset(((y).wrapping_mul(8_u64)) as isize)));
            (*mult_col.offset((((x).wrapping_mul(8_u64)).wrapping_add(y)) as isize)) = (((*quant
                .offset(((x).wrapping_add((8_u64).wrapping_mul(y))) as isize))
                * (kSqrt2FixedPoint_108))
                / (*quant.offset((x) as isize)));
            x.prefix_inc();
        }
        y.prefix_inc();
    }
}
impl brunsli_ComponentStateDC {
    unsafe fn InitAll(&mut self) {
        (unsafe {
            let _probability: u8 = 135_u8;
            self.is_zero_prob.Init(_probability)
        });
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (self.sign_prob.len() as u64)) {
            (unsafe {
                let _probability: u8 = 128_u8;
                self.sign_prob[(i) as usize].Init(_probability)
            });
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (self.is_empty_block_prob.len() as u64)) {
            (unsafe {
                let _probability: u8 = 74_u8;
                self.is_empty_block_prob[(i) as usize].Init(_probability)
            });
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (self.first_extra_bit_prob.len() as u64)) {
            (unsafe {
                let _probability: u8 = 150_u8;
                self.first_extra_bit_prob[(i) as usize].Init(_probability)
            });
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
                        let _f: *const u8 = b"context.cc\0".as_ptr();
                        let _l: i32 = 227;
                        let _fn: *const u8 = b"InitAll\0".as_ptr();
                        BrunsliDumpAndAbort_79(_f, _l, _fn)
                    });
                    'loop_: while true {}
                };
                (unsafe {
                    let _probability: u8 = (v as u8);
                    self.is_zero_prob[((((i) * (kDCTBlockSize_3)) + (k)) as u64) as usize]
                        .Init(_probability)
                });
                k.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (self.sign_prob.len() as u64)) {
            if ((i) < ((kMaxAverageContext_82).wrapping_mul((kDCTBlockSize_3 as u64)))) {
                (unsafe {
                    let _probability: u8 = 108_u8;
                    self.sign_prob[(i) as usize].Init(_probability)
                });
            } else if ((i)
                < ((((kMaxAverageContext_82).wrapping_add(1_u64)) as u64)
                    .wrapping_mul((kDCTBlockSize_3 as u64))))
            {
                (unsafe {
                    let _probability: u8 = 128_u8;
                    self.sign_prob[(i) as usize].Init(_probability)
                });
            } else {
                (unsafe {
                    let _probability: u8 = 148_u8;
                    self.sign_prob[(i) as usize].Init(_probability)
                });
            }
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (self.first_extra_bit_prob.len() as u64)) {
            (unsafe {
                let _probability: u8 = 158_u8;
                self.first_extra_bit_prob[(i) as usize].Init(_probability)
            });
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (kNumNonZeroContextCount_88)) {
            let mut non_zero_probs: *mut brunsli_Prob = self
                .num_nonzero_prob
                .as_mut_ptr()
                .offset(((i).wrapping_mul(kNumNonZeroTreeSize_85)) as isize);
            let mut j: u64 = 0_u64;
            'loop_: while ((j) < (kNumNonZeroTreeSize_85)) {
                (unsafe {
                    let _probability: u8 = kInitProbNonzero_111[(i) as usize][(j) as usize];
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
        let mut num_values: u32 = (self.values_.len() as u64 as u32);
        if !((num_values) > (0_u32)) {
            (unsafe {
                let _f: *const u8 = b"lehmer_code.cc\0".as_ptr();
                let _l: i32 = 51;
                let _fn: *const u8 = b"num_bits\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if ((num_values) <= (1_u32)) {
            return 0;
        }
        return ((unsafe {
            let _n: u32 = (num_values).wrapping_sub(1_u32);
            Log2FloorNonZero_74(_n)
        }) + (1));
    }
    pub unsafe fn Remove(&mut self, mut code: u64, mut value: *mut u8) -> bool {
        if ((code) >= (self.values_.len() as u64)) {
            return false;
        }
        (*value) = self.values_[(code) as usize];
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
pub unsafe fn ComputeLehmerCode_112(mut sigma: *const u32, len: u64, mut code: *mut u32) {
    let mut items: Vec<u32> = (0..(len) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (len)) {
        items[(i) as usize] = (i as u32);
        i.prefix_inc();
    }
    let mut i: u64 = 0_u64;
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
                let _f: *const u8 = b"lehmer_code.cc\0".as_ptr();
                let _l: i32 = 21;
                let _fn: *const u8 = b"ComputeLehmerCode\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
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
    mut len: u64,
    mut sigma: *mut u32,
) -> bool {
    let mut items: Vec<u32> = (0..(len) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (len)) {
        items[(i) as usize] = (i as u32);
        i.prefix_inc();
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (len)) {
        let mut index: u32 = (*code.offset((i) as isize));
        if ((index as u64) >= (items.len() as u64)) {
            return false;
        }
        let value: u32 = items[(index as u64) as usize];
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
pub unsafe fn BrunsliDumpAndAbort_79(mut f: *const u8, mut l: i32, mut fn_: *const u8) {
    printf(b"%s:%d (%s)\n\0".as_ptr() as *const i8, f, l, fn_);
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
pub static mut kQFactorBits_116: u64 = unsafe { 6_u64 };
pub static mut kQFactorLimit_117: u64 = unsafe { (((1_u32) << (kQFactorBits_116)) as u64) };
pub unsafe fn FillQuantMatrix_118(mut is_chroma: bool, mut q: u32, mut dst: *mut u8) {
    if !(((q) >= (0_u32)) && ((q as u64) < (kQFactorLimit_117))) {
        (unsafe {
            let _f: *const u8 = b"quant_matrix.cc\0".as_ptr();
            let _l: i32 = 18;
            let _fn: *const u8 = b"FillQuantMatrix\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
            if ((v) > (255_u32)) {
                255_u32
            } else {
                v
            }
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
    let kMaxDiffCost: u64 = 33_u64;
    let kWorstLen: u64 = (((kDCTBlockSize_3) + (1)) as u64)
        .wrapping_mul(((kMaxDiffCost).wrapping_add(1_u64)) as u64);
    let mut best_len: u64 = kWorstLen;
    let mut q: u32 = 0_u32;
    'loop_: while ((q as u64) < (kQFactorLimit_117)) {
        (unsafe {
            let _is_chroma: bool = is_chroma;
            let _q: u32 = q;
            let _dst: *mut u8 = dst;
            FillQuantMatrix_118(_is_chroma, _q, _dst)
        });
        let mut last_diff: i32 = 0;
        let mut len: u64 = 0_u64;
        let mut k: i32 = 0;
        'loop_: while ((k) < (kDCTBlockSize_3)) {
            let j: i32 = (kJPEGNaturalOrder_13[(k) as usize] as i32);
            let new_diff: i32 =
                ((*src.offset((j) as isize)) - ((*dst.offset((j) as isize)) as i32));
            let mut diff: i32 = ((new_diff) - (last_diff));
            last_diff = new_diff;
            if ((diff) != (0)) {
                len = (len).wrapping_add(1_u64);
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
                    let mut diff_len: u32 = (((unsafe {
                        let _n: u32 = (diff as u32);
                        Log2FloorNonZero_74(_n)
                    }) + (1)) as u32);
                    if ((diff_len) == (16_u32)) {
                        diff_len.postfix_dec();
                    }
                    len = (len).wrapping_add(
                        ((((2_u32).wrapping_mul(diff_len)).wrapping_add(1_u32)) as u64),
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
    (unsafe {
        let _is_chroma: bool = is_chroma;
        let _q: u32 = best_q;
        let _dst: *mut u8 = dst;
        FillQuantMatrix_118(_is_chroma, _q, _dst)
    });
    return best_q;
}
pub static mut kBitMask_120: [i32; 17] = unsafe {
    [
        0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535,
    ]
};
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_WordSource {
    pub data_: *const u8,
    pub len_: u64,
    pub pos_: u64,
    pub error_: bool,
    pub optimistic_: bool,
}
impl brunsli_WordSource {
    pub unsafe fn brunsli_WordSource(
        mut data: *const u8,
        mut len: u64,
        mut optimistic: bool,
    ) -> Self {
        let mut this = Self {
            data_: data,
            len_: ((len) & (!1 as u64)),
            pos_: 0_u64,
            error_: false,
            optimistic_: optimistic,
        };
        this
    }
    pub unsafe fn GetNextWord(&mut self) -> u16 {
        let mut val: u16 = 0_u16;
        if ((self.pos_) < (self.len_)) {
            val = (unsafe {
                let _p: *const ::libc::c_void =
                    (self.data_.offset((self.pos_) as isize) as *const u8 as *const ::libc::c_void);
                BrunsliUnalignedRead16_66(_p)
            });
        } else {
            self.error_ = true;
        }
        self.pos_ = (self.pos_).wrapping_add(2_u64);
        return val;
    }
    pub unsafe fn CanRead(&mut self, mut n: u64) -> bool {
        if self.optimistic_ {
            return true;
        }
        let mut delta: u64 = (2_u64).wrapping_mul(n);
        let mut projected_end: u64 = (self.pos_).wrapping_add(delta);
        if ((projected_end) < (self.pos_)) {
            return false;
        }
        return ((projected_end) <= (self.len_));
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_BitSource {
    pub val_: u32,
    pub bit_pos_: i32,
}
impl brunsli_BitSource {
    pub unsafe fn brunsli_BitSource() -> Self {
        let mut this = Self {
            val_: 0_u32,
            bit_pos_: 0_i32,
        };
        this
    }
    pub unsafe fn Init(&mut self, mut in_: *mut brunsli_WordSource) {
        self.val_ = ((unsafe { (*in_).GetNextWord() }) as u32).clone();
        self.bit_pos_ = 0;
    }
    pub unsafe fn ReadBits(&mut self, mut nbits: i32, mut in_: *mut brunsli_WordSource) -> u32 {
        if (((self.bit_pos_) + (nbits)) > (16)) {
            let mut new_bits: u32 = ((unsafe { (*in_).GetNextWord() }) as u32);
            self.val_ |= ((new_bits) << (16));
        }
        let mut result: u32 =
            (((self.val_) >> (self.bit_pos_)) & (kBitMask_120[(nbits) as usize] as u32));
        self.bit_pos_ += nbits;
        if ((self.bit_pos_) > (16)) {
            self.bit_pos_ -= 16;
            self.val_ >>= 16;
        }
        return result;
    }
    pub unsafe fn Finish(&mut self) -> bool {
        let mut n_bits: u64 = (((16) - (self.bit_pos_)) as u64);
        if ((n_bits) > (0_u64)) {
            let mut padding_bits: i32 = ((((self.val_) >> (self.bit_pos_))
                & (kBitMask_120[(n_bits) as usize] as u32))
                as i32);
            if ((padding_bits) != (0)) {
                return false;
            }
        }
        return true;
    }
}
impl Default for brunsli_BitSource {
    fn default() -> Self {
        unsafe { brunsli_BitSource::brunsli_BitSource() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_ANSSymbolInfo {
    pub offset_: u16,
    pub freq_: u16,
    pub symbol_: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_ANSDecodingData {
    pub map_: [brunsli_ANSSymbolInfo; 1024],
}
impl brunsli_ANSDecodingData {
    pub unsafe fn brunsli_ANSDecodingData() -> Self {
        let mut this = Self {
            map_: [<brunsli_ANSSymbolInfo>::default(); 1024],
        };
        this
    }
}
impl Default for brunsli_ANSDecodingData {
    fn default() -> Self {
        unsafe { brunsli_ANSDecodingData::brunsli_ANSDecodingData() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_ANSDecoder {
    state_: u32,
}
impl brunsli_ANSDecoder {
    pub unsafe fn brunsli_ANSDecoder() -> Self {
        let mut this = Self { state_: 0_u32 };
        this
    }
    pub unsafe fn Init(&mut self, mut in_: *mut brunsli_WordSource) {
        self.state_ = ((unsafe { (*in_).GetNextWord() }) as u32).clone();
        self.state_ =
            (((self.state_) << (16_u32)) | ((unsafe { (*in_).GetNextWord() }) as u32)).clone();
    }
    pub unsafe fn ReadSymbol(
        &mut self,
        code: *const brunsli_ANSDecodingData,
        mut in_: *mut brunsli_WordSource,
    ) -> i32 {
        let res: u32 = ((self.state_) & (((BRUNSLI_ANS_TAB_SIZE_1) - (1)) as u32));
        let s: *const brunsli_ANSSymbolInfo =
            &(*code).map_[(res) as usize] as *const brunsli_ANSSymbolInfo;
        self.state_ = (((*s).freq_ as u32)
            .wrapping_mul(((self.state_) >> (BRUNSLI_ANS_LOG_TAB_SIZE_0))))
        .wrapping_add(((*s).offset_ as u32));
        if ((self.state_) < ((1_u32) << (16_u32))) {
            self.state_ =
                (((self.state_) << (16_u32)) | ((unsafe { (*in_).GetNextWord() }) as u32)).clone();
        }
        return ((*s).symbol_ as i32);
    }
    pub unsafe fn CheckCRC(&self) -> bool {
        return ((self.state_) == ((19_u32) << (16_u32)));
    }
}
impl Default for brunsli_ANSDecoder {
    fn default() -> Self {
        unsafe { brunsli_ANSDecoder::brunsli_ANSDecoder() }
    }
}
impl brunsli_ANSDecodingData {
    pub unsafe fn Init(&mut self, counts: *const Vec<u32>) -> bool {
        let mut pos: u64 = 0_u64;
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < ((*counts).len() as u64)) {
            let mut j: u64 = 0_u64;
            'loop_: while ((j) < ((&(*counts))[(i) as usize] as u64)) {
                self.map_[(pos) as usize].symbol_ = (i as u8);
                self.map_[(pos) as usize].freq_ = ((&(*counts))[(i) as usize] as u16);
                self.map_[(pos) as usize].offset_ = (j as u16);
                j.prefix_inc();
                pos.prefix_inc();
            }
            i.prefix_inc();
        }
        return ((pos) == (BRUNSLI_ANS_TAB_SIZE_1 as u64));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_BrunsliBitReader {
    pub next_: *const u8,
    pub end_: *const u8,
    pub num_bits_: u32,
    pub bits_: u32,
    pub num_debt_bytes_: u32,
    pub is_healthy_: bool,
    pub is_optimistic_: bool,
}
pub unsafe fn BrunsliBitReaderBitMask_121(mut n: u32) -> u32 {
    return !((4294967295_u32) << (n));
}
pub unsafe fn BrunsliBitReaderOweByte_122(mut br: *mut brunsli_BrunsliBitReader) {
    (*br).num_bits_ = ((*br).num_bits_).wrapping_add(8_u32);
    (*br).num_debt_bytes_.postfix_inc();
}
pub unsafe fn BrunsliBitReaderMaybeFetchByte_123(
    mut br: *mut brunsli_BrunsliBitReader,
    mut n_bits: u32,
) {
    if (((*br).num_bits_) < (n_bits)) {
        if (((((*br).next_) >= ((*br).end_)) as i64) != 0) {
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                BrunsliBitReaderOweByte_122(_br)
            });
        } else {
            (*br).bits_ |= (((*(*br).next_) as u32) << ((*br).num_bits_));
            (*br).num_bits_ = ((*br).num_bits_).wrapping_add(8_u32);
            (*br).next_.postfix_inc();
        }
    }
}
pub unsafe fn BrunsliBitReaderGet_124(
    mut br: *mut brunsli_BrunsliBitReader,
    mut n_bits: u32,
) -> u32 {
    if !((n_bits) <= (24_u32)) {
        (unsafe {
            let _f: *const u8 = b"bit_reader.cc\0".as_ptr();
            let _l: i32 = 110;
            let _fn: *const u8 = b"BrunsliBitReaderGet\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = n_bits;
        BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
    });
    if ((n_bits) > (8_u32)) {
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = n_bits;
            BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
        });
        if ((n_bits) > (16_u32)) {
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = n_bits;
                BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
            });
        }
    }
    return (((*br).bits_)
        & (unsafe {
            let _n: u32 = n_bits;
            BrunsliBitReaderBitMask_121(_n)
        }));
}
pub unsafe fn BrunsliBitReaderDrop_125(mut br: *mut brunsli_BrunsliBitReader, mut n_bits: u32) {
    if !((n_bits) <= ((*br).num_bits_)) {
        (unsafe {
            let _f: *const u8 = b"bit_reader.cc\0".as_ptr();
            let _l: i32 = 121;
            let _fn: *const u8 = b"BrunsliBitReaderDrop\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (*br).bits_ >>= n_bits;
    (*br).num_bits_ = ((*br).num_bits_).wrapping_sub(n_bits);
}
pub unsafe fn BrunsliBitReaderRead_126(
    mut br: *mut brunsli_BrunsliBitReader,
    mut n_bits: u32,
) -> u32 {
    let mut result: u32 = (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = n_bits;
        BrunsliBitReaderGet_124(_br, _n_bits)
    });
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = n_bits;
        BrunsliBitReaderDrop_125(_br, _n_bits)
    });
    return result;
}
pub unsafe fn BrunsliBitReaderInit_127(mut br: *mut brunsli_BrunsliBitReader) {
    (*br).num_bits_ = 0_u32;
    (*br).bits_ = 0_u32;
    (*br).num_debt_bytes_ = 0_u32;
    (*br).is_healthy_ = true;
    (*br).is_optimistic_ = false;
}
pub unsafe fn BrunsliBitReaderResume_128(
    mut br: *mut brunsli_BrunsliBitReader,
    mut buffer: *const u8,
    mut length: u64,
) {
    (*br).next_ = buffer;
    (*br).end_ = buffer.offset((length) as isize);
    (*br).is_optimistic_ = false;
}
pub unsafe fn BrunsliBitReaderUnload_129(mut br: *mut brunsli_BrunsliBitReader) {
    'loop_: while (((*br).num_debt_bytes_) > (0_u32)) && (((*br).num_bits_) >= (8_u32)) {
        (*br).num_debt_bytes_.postfix_dec();
        (*br).num_bits_ = ((*br).num_bits_).wrapping_sub(8_u32);
    }
    'loop_: while (((*br).num_bits_) >= (8_u32)) {
        (*br).next_.postfix_dec();
        (*br).num_bits_ = ((*br).num_bits_).wrapping_sub(8_u32);
    }
    (*br).bits_ &= (unsafe {
        let _n: u32 = (*br).num_bits_;
        BrunsliBitReaderBitMask_121(_n)
    });
}
pub unsafe fn BrunsliBitReaderSuspend_130(mut br: *mut brunsli_BrunsliBitReader) -> u64 {
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderUnload_129(_br)
    });
    let mut unused_bytes: u64 = (((((*br).end_ as usize - (*br).next_ as usize)
        / ::std::mem::size_of::<u8>()) as i64) as u64);
    (*br).next_ = std::ptr::null();
    (*br).end_ = std::ptr::null();
    return unused_bytes;
}
pub unsafe fn BrunsliBitReaderFinish_131(mut br: *mut brunsli_BrunsliBitReader) {
    let mut n_bits: u32 = (*br).num_bits_;
    if ((n_bits) >= (8_u32)) {
        (*br).is_healthy_ = false;
        return;
    }
    if ((n_bits) > (0_u32)) {
        let mut padding_bits: u32 = (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = n_bits;
            BrunsliBitReaderRead_126(_br, _n_bits)
        });
        if ((padding_bits) != (0_u32)) {
            (*br).is_healthy_ = false;
        }
    }
}
pub unsafe fn BrunsliBitReaderIsHealthy_132(mut br: *mut brunsli_BrunsliBitReader) -> bool {
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderUnload_129(_br)
    });
    return (((*br).num_debt_bytes_) == (0_u32)) && ((*br).is_healthy_);
}
pub unsafe fn BrunsliBitReaderSetOptimistic_133(mut br: *mut brunsli_BrunsliBitReader) {
    (*br).is_optimistic_ = true;
}
pub unsafe fn BrunsliBitReaderCanRead_134(
    mut br: *mut brunsli_BrunsliBitReader,
    mut n_bits: u64,
) -> bool {
    if (*br).is_optimistic_ {
        return true;
    }
    if (((*br).num_debt_bytes_) != (0_u32)) {
        return false;
    }
    if (((*br).num_bits_ as u64) >= (n_bits)) {
        return true;
    }
    let mut num_extra_bytes: u64 =
        ((((n_bits).wrapping_sub(((*br).num_bits_ as u64))).wrapping_add(7_u64)) >> (3));
    return (((*br).next_.offset((num_extra_bytes) as isize)) <= ((*br).end_));
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_BrunsliStatus {
    #[default]
    BRUNSLI_OK = 0,
    BRUNSLI_NON_REPRESENTABLE = 1,
    BRUNSLI_MEMORY_ERROR = 2,
    BRUNSLI_INVALID_PARAM = 3,
    BRUNSLI_COMPRESSION_ERROR = 4,
    BRUNSLI_INVALID_BRN = 5,
    BRUNSLI_DECOMPRESSION_ERROR = 6,
    BRUNSLI_NOT_ENOUGH_DATA = 7,
}
impl From<i32> for brunsli_BrunsliStatus {
    fn from(n: i32) -> brunsli_BrunsliStatus {
        match n {
            0 => brunsli_BrunsliStatus::BRUNSLI_OK,
            1 => brunsli_BrunsliStatus::BRUNSLI_NON_REPRESENTABLE,
            2 => brunsli_BrunsliStatus::BRUNSLI_MEMORY_ERROR,
            3 => brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM,
            4 => brunsli_BrunsliStatus::BRUNSLI_COMPRESSION_ERROR,
            5 => brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN,
            6 => brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR,
            7 => brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA,
            _ => panic!("invalid brunsli_BrunsliStatus value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_BrunsliStatus);
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_BrunsliDecoder_Status {
    #[default]
    NEEDS_MORE_INPUT = 0,
    NEEDS_MORE_OUTPUT = 1,
    ERROR = 2,
    DONE = 3,
}
impl From<i32> for brunsli_BrunsliDecoder_Status {
    fn from(n: i32) -> brunsli_BrunsliDecoder_Status {
        match n {
            0 => brunsli_BrunsliDecoder_Status::NEEDS_MORE_INPUT,
            1 => brunsli_BrunsliDecoder_Status::NEEDS_MORE_OUTPUT,
            2 => brunsli_BrunsliDecoder_Status::ERROR,
            3 => brunsli_BrunsliDecoder_Status::DONE,
            _ => panic!("invalid brunsli_BrunsliDecoder_Status value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_BrunsliDecoder_Status);
#[repr(C)]
#[derive()]
pub struct brunsli_BrunsliDecoder {
    jpg_: Option<Box<brunsli_JPEGData>>,
    state_: Option<Box<brunsli_internal_dec_State>>,
}
impl brunsli_BrunsliDecoder {
    pub unsafe fn brunsli_BrunsliDecoder() -> Self {
        let mut this = Self {
            jpg_: None,
            state_: None,
        };
        {
            let _a0: *mut brunsli_JPEGData =
                (Box::leak(Box::new(brunsli_JPEGData::brunsli_JPEGData()))
                    as *mut brunsli_JPEGData);
            this.jpg_ = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        {
            let _a0: *mut brunsli_internal_dec_State = (Box::leak(Box::new(
                brunsli_internal_dec_State::brunsli_internal_dec_State(),
            ))
                as *mut brunsli_internal_dec_State);
            this.state_ = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        this
    }
}
impl Default for brunsli_BrunsliDecoder {
    fn default() -> Self {
        unsafe { brunsli_BrunsliDecoder::brunsli_BrunsliDecoder() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_BinaryArithmeticDecoder {
    low_: u32,
    high_: u32,
    value_: u32,
}
impl brunsli_BinaryArithmeticDecoder {
    pub unsafe fn brunsli_BinaryArithmeticDecoder() -> Self {
        let mut this = Self {
            low_: 0_u32,
            high_: 0_u32,
            value_: 0_u32,
        };
        this
    }
    pub unsafe fn Init(&mut self, mut in_: *mut brunsli_WordSource) {
        self.low_ = 0_u32;
        self.high_ = !0_u32;
        self.value_ = ((unsafe { (*in_).GetNextWord() }) as u32).clone();
        self.value_ =
            (((self.value_) << (16_u32)) | ((unsafe { (*in_).GetNextWord() }) as u32)).clone();
    }
    pub unsafe fn ReadBit(&mut self, mut prob: i32, mut in_: *mut brunsli_WordSource) -> i32 {
        let diff: u32 = (self.high_).wrapping_sub(self.low_);
        let split: u32 = (((self.low_ as u64)
            .wrapping_add((((diff as u64).wrapping_mul((prob as u64))) >> (8_u32))))
            as u32);
        let mut bit: i32 = 0_i32;
        if ((self.value_) > (split)) {
            self.low_ = (split).wrapping_add(1_u32);
            bit = 1;
        } else {
            self.high_ = split;
            bit = 0;
        }
        if ((((self.low_) ^ (self.high_)) >> (16_u32)) == (0_u32)) {
            self.value_ =
                (((self.value_) << (16_u32)) | ((unsafe { (*in_).GetNextWord() }) as u32)).clone();
            self.low_ <<= 16_u32;
            self.high_ <<= 16_u32;
            self.high_ = ((self.high_ as u32) | 65535_u32) as u32;
        }
        return bit;
    }
}
impl Default for brunsli_BinaryArithmeticDecoder {
    fn default() -> Self {
        unsafe { brunsli_BinaryArithmeticDecoder::brunsli_BinaryArithmeticDecoder() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_HuffmanCode {
    pub bits: u8,
    pub value: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_JPEGOutput {
    cb: Option<unsafe fn(*mut ::libc::c_void, *const u8, u64) -> u64>,
    data: *mut ::libc::c_void,
}
impl brunsli_JPEGOutput {
    pub unsafe fn brunsli_JPEGOutput(
        mut cb: Option<unsafe fn(*mut ::libc::c_void, *const u8, u64) -> u64>,
        mut data: *mut ::libc::c_void,
    ) -> Self {
        let mut this = Self { cb: cb, data: data };
        this
    }
    pub unsafe fn Write(&self, mut buf: *const u8, mut len: u64) -> bool {
        if ((len) == (0_u64)) {
            return true;
        }
        let mut bytes_written: u64 = (unsafe {
            let _arg0: *mut ::libc::c_void = self.data;
            let _arg1: *const u8 = buf;
            let _arg2: u64 = len;
            (self.cb).unwrap()(_arg0, _arg1, _arg2)
        });
        return ((bytes_written) == (len));
    }
}
impl Default for brunsli_JPEGOutput {
    fn default() -> Self {
        brunsli_JPEGOutput {
            cb: None,
            data: std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_internal_dec_ComponentMeta {
    pub context_offset: u64,
    pub h_samp: i32,
    pub v_samp: i32,
    pub context_bits: u64,
    pub ac_stride: i32,
    pub b_stride: i32,
    pub width_in_blocks: i32,
    pub height_in_blocks: i32,
    pub ac_coeffs: *mut i16,
    pub block_state: *mut u8,
    pub quant: Vec<i32>,
}
impl Default for brunsli_internal_dec_ComponentMeta {
    fn default() -> Self {
        brunsli_internal_dec_ComponentMeta {
            context_offset: 0_u64,
            h_samp: 0_i32,
            v_samp: 0_i32,
            context_bits: 0_u64,
            ac_stride: 0_i32,
            b_stride: 0_i32,
            width_in_blocks: 0_i32,
            height_in_blocks: 0_i32,
            ac_coeffs: std::ptr::null_mut(),
            block_state: std::ptr::null_mut(),
            quant: std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_Stage {
    #[default]
    SIGNATURE = 0,
    HEADER = 1,
    FALLBACK = 2,
    SECTION = 3,
    SECTION_BODY = 4,
    DONE = 5,
    ERROR = 6,
}
impl From<i32> for brunsli_internal_dec_Stage {
    fn from(n: i32) -> brunsli_internal_dec_Stage {
        match n {
            0 => brunsli_internal_dec_Stage::SIGNATURE,
            1 => brunsli_internal_dec_Stage::HEADER,
            2 => brunsli_internal_dec_Stage::FALLBACK,
            3 => brunsli_internal_dec_Stage::SECTION,
            4 => brunsli_internal_dec_Stage::SECTION_BODY,
            5 => brunsli_internal_dec_Stage::DONE,
            6 => brunsli_internal_dec_Stage::ERROR,
            _ => panic!("invalid brunsli_internal_dec_Stage value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_Stage);
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_SerializationStatus {
    #[default]
    NEEDS_MORE_INPUT = 0,
    NEEDS_MORE_OUTPUT = 1,
    ERROR = 2,
    DONE = 3,
}
impl From<i32> for brunsli_internal_dec_SerializationStatus {
    fn from(n: i32) -> brunsli_internal_dec_SerializationStatus {
        match n {
            0 => brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT,
            1 => brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT,
            2 => brunsli_internal_dec_SerializationStatus::ERROR,
            3 => brunsli_internal_dec_SerializationStatus::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_SerializationStatus value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_SerializationStatus);
#[repr(C)]
#[derive(Default)]
pub struct brunsli_Arena {
    pub capacity: u64,
    pub storage: Option<Box<[brunsli_HuffmanCode]>>,
}
impl brunsli_Arena {
    pub unsafe fn reserve(&mut self, mut limit: u64) {
        if ((self.capacity) < (limit)) {
            self.capacity = limit;
            self.storage = Some(Box::from_raw(Box::leak(
                (0..self.capacity)
                    .map(|_| <brunsli_HuffmanCode>::default())
                    .collect::<Box<[brunsli_HuffmanCode]>>(),
            )));
        }
    }
    pub unsafe fn data(&mut self) -> *mut brunsli_HuffmanCode {
        return self
            .storage
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    }
    pub unsafe fn reset(&mut self) {
        self.capacity = 0_u64;
        self.storage = None;
    }
}
#[repr(C)]
#[derive()]
pub struct brunsli_internal_dec_OutputChunk {
    pub next: *const u8,
    pub len: u64,
    pub buffer: Option<Box<Vec<u8>>>,
}
impl brunsli_internal_dec_OutputChunk {
    pub unsafe fn brunsli_internal_dec_OutputChunk1(mut data: *const u8, mut size: u64) -> Self {
        let mut this = Self {
            next: data,
            len: size,
            buffer: None,
        };
        this
    }
    pub unsafe fn brunsli_internal_dec_OutputChunk2(mut size: Option<u64>) -> Self {
        let mut size: u64 = size.unwrap_or(0_u64);
        let mut this = Self {
            next: std::ptr::null(),
            len: 0_u64,
            buffer: None,
        };
        {
            let _a0: *mut Vec<u8> = (Box::leak(Box::new(
                (0..(size) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            )) as *mut Vec<u8>);
            this.buffer = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        this.next = ((*this.buffer.as_deref_mut().unwrap()).as_mut_ptr()).cast_const();
        this.len = size;
        this
    }
    pub unsafe fn brunsli_internal_dec_OutputChunk3(mut bytes: Vec<u8>) -> Self {
        let mut this = Self {
            next: std::ptr::null(),
            len: 0_u64,
            buffer: None,
        };
        {
            let _a0: *mut Vec<u8> = (Box::leak(Box::new(bytes.clone())) as *mut Vec<u8>);
            this.buffer = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        this.next = ((*this.buffer.as_deref_mut().unwrap()).as_mut_ptr()).cast_const();
        this.len = bytes.len() as u64;
        this
    }
}
impl Default for brunsli_internal_dec_OutputChunk {
    fn default() -> Self {
        unsafe { brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brunsli_HuffmanCodeTable {
    pub depth: [i32; 256],
    pub code: [i32; 256],
}
impl Default for brunsli_HuffmanCodeTable {
    fn default() -> Self {
        brunsli_HuffmanCodeTable {
            depth: [0_i32; 256],
            code: [0_i32; 256],
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct brunsli_internal_dec_BitWriter {
    pub healthy: bool,
    pub output: *mut Vec<brunsli_internal_dec_OutputChunk>,
    pub chunk: brunsli_internal_dec_OutputChunk,
    pub data: *mut u8,
    pub pos: u64,
    pub put_buffer: u64,
    pub put_bits: i32,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_DCTCodingState {
    pub eob_run_: i32,
    pub cur_ac_huff_: *const brunsli_HuffmanCodeTable,
    pub refinement_bits_: Vec<u16>,
    pub refinement_bits_count_: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_EncodeScanState_Stage {
    #[default]
    HEAD = 0,
    BODY = 1,
}
impl From<i32> for brunsli_internal_dec_EncodeScanState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_EncodeScanState_Stage {
        match n {
            0 => brunsli_internal_dec_EncodeScanState_Stage::HEAD,
            1 => brunsli_internal_dec_EncodeScanState_Stage::BODY,
            _ => panic!(
                "invalid brunsli_internal_dec_EncodeScanState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_EncodeScanState_Stage);
#[repr(C)]
#[derive()]
pub struct brunsli_internal_dec_EncodeScanState {
    pub stage: brunsli_internal_dec_EncodeScanState_Stage,
    pub mcu_y: i32,
    pub bw: brunsli_internal_dec_BitWriter,
    pub last_dc_coeff: [i16; 4],
    pub restarts_to_go: i32,
    pub next_restart_marker: i32,
    pub block_scan_index: i32,
    pub coding_state: brunsli_internal_dec_DCTCodingState,
    pub extra_zero_runs_pos: u64,
    pub next_extra_zero_run_index: i32,
    pub next_reset_point_pos: u64,
    pub next_reset_point: i32,
}
impl Default for brunsli_internal_dec_EncodeScanState {
    fn default() -> Self {
        brunsli_internal_dec_EncodeScanState {
            stage: brunsli_internal_dec_EncodeScanState_Stage::HEAD,
            mcu_y: 0_i32,
            bw: <brunsli_internal_dec_BitWriter>::default(),
            last_dc_coeff: [0_i16; 4],
            restarts_to_go: 0_i32,
            next_restart_marker: 0_i32,
            block_scan_index: 0_i32,
            coding_state: <brunsli_internal_dec_DCTCodingState>::default(),
            extra_zero_runs_pos: 0_u64,
            next_extra_zero_run_index: 0_i32,
            next_reset_point_pos: 0_u64,
            next_reset_point: 0_i32,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_SerializationState_Stage {
    #[default]
    INIT = 0,
    SERIALIZE_SECTION = 1,
    DONE = 2,
    ERROR = 3,
}
impl From<i32> for brunsli_internal_dec_SerializationState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_SerializationState_Stage {
        match n {
            0 => brunsli_internal_dec_SerializationState_Stage::INIT,
            1 => brunsli_internal_dec_SerializationState_Stage::SERIALIZE_SECTION,
            2 => brunsli_internal_dec_SerializationState_Stage::DONE,
            3 => brunsli_internal_dec_SerializationState_Stage::ERROR,
            _ => panic!(
                "invalid brunsli_internal_dec_SerializationState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_SerializationState_Stage);
#[repr(C)]
#[derive(Default)]
pub struct brunsli_internal_dec_SerializationState {
    pub stage: brunsli_internal_dec_SerializationState_Stage,
    pub output_queue: Vec<brunsli_internal_dec_OutputChunk>,
    pub section_index: u64,
    pub dht_index: i32,
    pub dqt_index: i32,
    pub app_index: i32,
    pub com_index: i32,
    pub data_index: i32,
    pub scan_index: i32,
    pub dc_huff_table: Vec<brunsli_HuffmanCodeTable>,
    pub ac_huff_table: Vec<brunsli_HuffmanCodeTable>,
    pub pad_bits: *const i32,
    pub pad_bits_end: *const i32,
    pub seen_dri_marker: bool,
    pub is_progressive: bool,
    pub scan_state: brunsli_internal_dec_EncodeScanState,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_AcDcState {
    pub next_mcu_y: i32,
    pub next_component: u64,
    pub next_iy: i32,
    pub next_x: i32,
    pub ac_coeffs_order_decoded: bool,
    pub ac: Vec<brunsli_ComponentState>,
    pub dc: Vec<brunsli_ComponentStateDC>,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_internal_dec_SectionState {
    pub tag: u64,
    pub is_active: bool,
    pub is_section: bool,
    pub tags_met: u32,
    pub remaining: u64,
    pub milestone: u64,
    pub projected_end: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_HeaderState_Stage {
    #[default]
    READ_TAG = 0,
    ENTER_SECTION = 1,
    ITEM_READ_TAG = 2,
    ITEM_ENTER_SECTION = 3,
    ITEM_SKIP_CONTENTS = 4,
    ITEM_READ_VALUE = 5,
    FINALE = 6,
    DONE = 7,
}
impl From<i32> for brunsli_internal_dec_HeaderState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_HeaderState_Stage {
        match n {
            0 => brunsli_internal_dec_HeaderState_Stage::READ_TAG,
            1 => brunsli_internal_dec_HeaderState_Stage::ENTER_SECTION,
            2 => brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG,
            3 => brunsli_internal_dec_HeaderState_Stage::ITEM_ENTER_SECTION,
            4 => brunsli_internal_dec_HeaderState_Stage::ITEM_SKIP_CONTENTS,
            5 => brunsli_internal_dec_HeaderState_Stage::ITEM_READ_VALUE,
            6 => brunsli_internal_dec_HeaderState_Stage::FINALE,
            7 => brunsli_internal_dec_HeaderState_Stage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_HeaderState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_HeaderState_Stage);
#[repr(C)]
#[derive(Clone)]
pub struct brunsli_internal_dec_HeaderState {
    pub stage: u64,
    pub section: brunsli_internal_dec_SectionState,
    pub remaining_skip_length: u64,
    pub varint_values: Vec<u64>,
}
impl Default for brunsli_internal_dec_HeaderState {
    fn default() -> Self {
        brunsli_internal_dec_HeaderState {
            stage: 0_u64,
            section: <brunsli_internal_dec_SectionState>::default(),
            remaining_skip_length: 0_u64,
            varint_values: std::array::from_fn::<_, 16, _>(|_| Default::default()).to_vec(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_FallbackState_Stage {
    #[default]
    READ_TAG = 0,
    ENTER_SECTION = 1,
    READ_CONTENTS = 2,
    DONE = 3,
}
impl From<i32> for brunsli_internal_dec_FallbackState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_FallbackState_Stage {
        match n {
            0 => brunsli_internal_dec_FallbackState_Stage::READ_TAG,
            1 => brunsli_internal_dec_FallbackState_Stage::ENTER_SECTION,
            2 => brunsli_internal_dec_FallbackState_Stage::READ_CONTENTS,
            3 => brunsli_internal_dec_FallbackState_Stage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_FallbackState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_FallbackState_Stage);
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_FallbackState {
    pub stage: u64,
    pub storage: Vec<u8>,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_SectionHeaderState_Stage {
    #[default]
    READ_TAG = 0,
    READ_VALUE = 1,
    ENTER_SECTION = 2,
    DONE = 3,
}
impl From<i32> for brunsli_internal_dec_SectionHeaderState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_SectionHeaderState_Stage {
        match n {
            0 => brunsli_internal_dec_SectionHeaderState_Stage::READ_TAG,
            1 => brunsli_internal_dec_SectionHeaderState_Stage::READ_VALUE,
            2 => brunsli_internal_dec_SectionHeaderState_Stage::ENTER_SECTION,
            3 => brunsli_internal_dec_SectionHeaderState_Stage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_SectionHeaderState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_SectionHeaderState_Stage);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_internal_dec_SectionHeaderState {
    pub stage: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_MetadataDecompressionStage {
    #[default]
    INITIAL = 0,
    READ_LENGTH = 1,
    DECOMPRESSING = 2,
    DONE = 3,
}
impl From<i32> for brunsli_internal_dec_MetadataDecompressionStage {
    fn from(n: i32) -> brunsli_internal_dec_MetadataDecompressionStage {
        match n {
            0 => brunsli_internal_dec_MetadataDecompressionStage::INITIAL,
            1 => brunsli_internal_dec_MetadataDecompressionStage::READ_LENGTH,
            2 => brunsli_internal_dec_MetadataDecompressionStage::DECOMPRESSING,
            3 => brunsli_internal_dec_MetadataDecompressionStage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_MetadataDecompressionStage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_MetadataDecompressionStage);
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_VarintState_Stage {
    #[default]
    INIT = 0,
    READ_CONTINUATION = 1,
    READ_DATA = 2,
}
impl From<i32> for brunsli_internal_dec_VarintState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_VarintState_Stage {
        match n {
            0 => brunsli_internal_dec_VarintState_Stage::INIT,
            1 => brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION,
            2 => brunsli_internal_dec_VarintState_Stage::READ_DATA,
            _ => panic!(
                "invalid brunsli_internal_dec_VarintState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_VarintState_Stage);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_internal_dec_VarintState {
    pub stage: brunsli_internal_dec_VarintState_Stage,
    pub value: u64,
    pub i: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_JpegInternalsState_Stage {
    #[default]
    INIT = 0,
    READ_MARKERS = 1,
    READ_DRI = 2,
    DECODE_HUFFMAN_MASK = 16,
    READ_HUFFMAN_LAST = 17,
    READ_HUFFMAN_SIMPLE = 18,
    READ_HUFFMAN_MAX_LEN = 19,
    READ_HUFFMAN_COUNT = 20,
    READ_HUFFMAN_PERMUTATION = 21,
    HUFFMAN_UPDATE = 22,
    PREPARE_READ_SCANS = 32,
    DECODE_SCAN_MASK = 64,
    READ_SCAN_COMMON = 65,
    READ_SCAN_COMPONENT = 66,
    READ_SCAN_RESET_POINT_CONTINUATION = 67,
    READ_SCAN_RESET_POINT_DATA = 68,
    READ_SCAN_ZERO_RUN_CONTINUATION = 69,
    READ_SCAN_ZERO_RUN_DATA = 70,
    READ_NUM_QUANT = 128,
    READ_QUANT = 129,
    READ_COMP_ID_SCHEME = 130,
    READ_COMP_ID = 131,
    READ_NUM_PADDING_BITS = 132,
    READ_PADDING_BITS = 133,
    ITERATE_MARKERS = 134,
    READ_INTERMARKER_LENGTH = 135,
    READ_INTERMARKER_DATA = 136,
    DONE = 137,
}
impl From<i32> for brunsli_internal_dec_JpegInternalsState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_JpegInternalsState_Stage {
        match n {
            0 => brunsli_internal_dec_JpegInternalsState_Stage::INIT,
            1 => brunsli_internal_dec_JpegInternalsState_Stage::READ_MARKERS,
            2 => brunsli_internal_dec_JpegInternalsState_Stage::READ_DRI,
            16 => brunsli_internal_dec_JpegInternalsState_Stage::DECODE_HUFFMAN_MASK,
            17 => brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST,
            18 => brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_SIMPLE,
            19 => brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_MAX_LEN,
            20 => brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_COUNT,
            21 => brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_PERMUTATION,
            22 => brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE,
            32 => brunsli_internal_dec_JpegInternalsState_Stage::PREPARE_READ_SCANS,
            64 => brunsli_internal_dec_JpegInternalsState_Stage::DECODE_SCAN_MASK,
            65 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON,
            66 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMPONENT,
            67 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION,
            68 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_DATA,
            69 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION,
            70 => brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_DATA,
            128 => brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT,
            129 => brunsli_internal_dec_JpegInternalsState_Stage::READ_QUANT,
            130 => brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID_SCHEME,
            131 => brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID,
            132 => brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS,
            133 => brunsli_internal_dec_JpegInternalsState_Stage::READ_PADDING_BITS,
            134 => brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS,
            135 => brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_LENGTH,
            136 => brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_DATA,
            137 => brunsli_internal_dec_JpegInternalsState_Stage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_JpegInternalsState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_JpegInternalsState_Stage);
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_JpegInternalsState {
    pub stage: brunsli_internal_dec_JpegInternalsState_Stage,
    pub have_dri: bool,
    pub num_scans: u64,
    pub dht_count: u64,
    pub br: brunsli_BrunsliBitReader,
    pub is_known_last_huffman_code: u64,
    pub terminal_huffman_code_count: u64,
    pub is_dc_table: bool,
    pub total_count: u64,
    pub space: u64,
    pub max_len: u64,
    pub max_count: u64,
    pub i: u64,
    pub p: brunsli_PermutationCoder,
    pub varint: brunsli_internal_dec_VarintState,
    pub j: u64,
    pub last_block_idx: i32,
    pub last_num: i32,
    pub num_padding_bits: u64,
    pub intermarker_length: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_QuantDataState_Stage {
    #[default]
    INIT = 0,
    READ_NUM_QUANT = 1,
    READ_STOCK = 2,
    READ_Q_FACTOR = 3,
    READ_DIFF_IS_ZERO = 4,
    READ_DIFF_SIGN = 5,
    READ_DIFF = 6,
    APPLY_DIFF = 7,
    UPDATE = 8,
    READ_QUANT_IDX = 9,
    FINISH = 10,
}
impl From<i32> for brunsli_internal_dec_QuantDataState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_QuantDataState_Stage {
        match n {
            0 => brunsli_internal_dec_QuantDataState_Stage::INIT,
            1 => brunsli_internal_dec_QuantDataState_Stage::READ_NUM_QUANT,
            2 => brunsli_internal_dec_QuantDataState_Stage::READ_STOCK,
            3 => brunsli_internal_dec_QuantDataState_Stage::READ_Q_FACTOR,
            4 => brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO,
            5 => brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_SIGN,
            6 => brunsli_internal_dec_QuantDataState_Stage::READ_DIFF,
            7 => brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF,
            8 => brunsli_internal_dec_QuantDataState_Stage::UPDATE,
            9 => brunsli_internal_dec_QuantDataState_Stage::READ_QUANT_IDX,
            10 => brunsli_internal_dec_QuantDataState_Stage::FINISH,
            _ => panic!(
                "invalid brunsli_internal_dec_QuantDataState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_QuantDataState_Stage);
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_QuantDataState {
    pub stage: brunsli_internal_dec_QuantDataState_Stage,
    pub br: brunsli_BrunsliBitReader,
    pub i: u64,
    pub j: u64,
    pub data_precision: u8,
    pub vs: brunsli_internal_dec_VarintState,
    pub delta: i32,
    pub sign: i32,
    pub predictor: Vec<u8>,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_HistogramDataState_Stage {
    #[default]
    INIT = 0,
    READ_SCHEME = 1,
    READ_NUM_HISTOGRAMS = 2,
    READ_CONTEXT_MAP_CODE = 3,
    READ_CONTEXT_MAP = 4,
    READ_HISTOGRAMS = 5,
    SKIP_CONTENT = 6,
    DONE = 7,
}
impl From<i32> for brunsli_internal_dec_HistogramDataState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_HistogramDataState_Stage {
        match n {
            0 => brunsli_internal_dec_HistogramDataState_Stage::INIT,
            1 => brunsli_internal_dec_HistogramDataState_Stage::READ_SCHEME,
            2 => brunsli_internal_dec_HistogramDataState_Stage::READ_NUM_HISTOGRAMS,
            3 => brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP_CODE,
            4 => brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP,
            5 => brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS,
            6 => brunsli_internal_dec_HistogramDataState_Stage::SKIP_CONTENT,
            7 => brunsli_internal_dec_HistogramDataState_Stage::DONE,
            _ => panic!(
                "invalid brunsli_internal_dec_HistogramDataState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_HistogramDataState_Stage);
#[repr(C)]
#[derive(Default)]
pub struct brunsli_internal_dec_HistogramDataState {
    pub stage: brunsli_internal_dec_HistogramDataState_Stage,
    pub br: brunsli_BrunsliBitReader,
    pub max_run_length_prefix: u64,
    pub entropy: Option<Box<brunsli_HuffmanDecodingData>>,
    pub i: u64,
    pub counts: Vec<u32>,
    pub arena: brunsli_Arena,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_internal_dec_Buffer {
    pub data_len: u64,
    pub borrowed_len: u64,
    pub data: Vec<u8>,
    pub external_data: *const u8,
    pub external_pos: u64,
    pub external_len: u64,
}
#[repr(C)]
#[derive(Default)]
pub struct brunsli_internal_dec_InternalState {
    pub ac_dc: brunsli_internal_dec_AcDcState,
    pub section: brunsli_internal_dec_SectionState,
    pub header: brunsli_internal_dec_HeaderState,
    pub fallback: brunsli_internal_dec_FallbackState,
    pub section_header: brunsli_internal_dec_SectionHeaderState,
    pub metadata: brunsli_internal_dec_MetadataState,
    pub internals: brunsli_internal_dec_JpegInternalsState,
    pub quant: brunsli_internal_dec_QuantDataState,
    pub histogram: brunsli_internal_dec_HistogramDataState,
    pub context_map_: Vec<u8>,
    pub entropy_codes_: Vec<brunsli_ANSDecodingData>,
    pub block_state_: Vec<Vec<u8>>,
    pub is_meta_warm: bool,
    pub shallow_histograms: bool,
    pub num_contexts: u64,
    pub num_histograms: u64,
    pub subdecoders_initialized: bool,
    pub ans_decoder: brunsli_ANSDecoder,
    pub bit_reader: brunsli_BitSource,
    pub arith_decoder: brunsli_BinaryArithmeticDecoder,
    pub result: brunsli_BrunsliStatus,
    pub last_stage: brunsli_internal_dec_Stage,
    pub buffer: brunsli_internal_dec_Buffer,
    pub serialization: brunsli_internal_dec_SerializationState,
}
pub static mut kNumDirectCodes_135: i32 = unsafe { 8 };
pub static mut kCoeffAlphabetSize_136: i32 = unsafe { ((kNumDirectCodes_135) + (10)) };
pub static mut kKnownSectionTags_137: u32 = unsafe {
    ((((((((((1_u32) << (kBrunsliSignatureTag_30 as i32))
        | ((1_u32) << (kBrunsliHeaderTag_31 as i32)))
        | ((1_u32) << (kBrunsliMetaDataTag_32 as i32)))
        | ((1_u32) << (kBrunsliJPEGInternalsTag_33 as i32)))
        | ((1_u32) << (kBrunsliQuantDataTag_34 as i32)))
        | ((1_u32) << (kBrunsliHistogramDataTag_35 as i32)))
        | ((1_u32) << (kBrunsliDCDataTag_36 as i32)))
        | ((1_u32) << (kBrunsliACDataTag_37 as i32)))
        | ((1_u32) << (kBrunsliOriginalJpgTag_38 as i32)))
};
pub static mut kKnownHeaderVarintTags_138: u32 = unsafe {
    (((((1_u32) << (kBrunsliHeaderWidthTag_39 as i32))
        | ((1_u32) << (kBrunsliHeaderHeightTag_40 as i32)))
        | ((1_u32) << (kBrunsliHeaderVersionCompTag_41 as i32)))
        | ((1_u32) << (kBrunsliHeaderSubsamplingTag_42 as i32)))
};
pub unsafe fn IsBrunsli_139(mut data: *const u8, len: u64) -> bool {
    static mut kSignature_140: [u8; 6] = unsafe { [10_u8, 4_u8, 66_u8, 210_u8, 213_u8, 78_u8] };;
    static mut kSignatureLen_141: u64 = unsafe { ::std::mem::size_of::<[u8; 6]>() as u64 };;
    if ((len) < (kSignatureLen_141)) {
        return false;
    }
    return (({
        let sa = core::slice::from_raw_parts(
            (kSignature_140.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
            kSignatureLen_141 as usize,
        );
        let sb = core::slice::from_raw_parts(
            (data as *const u8 as *const ::libc::c_void) as *const u8,
            kSignatureLen_141 as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn DivCeil_142(mut a: i32, mut b: i32) -> i32 {
    return ((((a) + (b)) - (1)) / (b));
}
pub unsafe fn DecodeVarLenUint8_143(mut br: *mut brunsli_BrunsliBitReader) -> u32 {
    if ((unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = 1_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    }) != 0)
    {
        let mut nbits: u32 = (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 3_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        });
        if ((nbits) == (0_u32)) {
            return 1_u32;
        } else {
            return (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = nbits;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(((1_u32) << (nbits)));
        }
    }
    return 0_u32;
}
pub unsafe fn DecodeVarint_144(
    mut s: *mut brunsli_internal_dec_VarintState,
    mut br: *mut brunsli_BrunsliBitReader,
    mut max_bits: u64,
) -> bool {
    if (((*s).stage as i32) == (brunsli_internal_dec_VarintState_Stage::INIT as i32)) {
        (*s).value = 0_u64;
        (*s).i = 0_u64;
        (*s).stage = (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*s).stage as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if (((*s).i) >= (max_bits)) {
                        (*s).stage = (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                        return true;
                    }
                    if ((((*s).i).wrapping_add(1_u64)) != (max_bits)) {
                        if !(unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if !((unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*s).stage = (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                            return true;
                        }
                    }
                    (*s).stage = (brunsli_internal_dec_VarintState_Stage::READ_DATA).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 = 1_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let mut next_bit: u64 = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    (*s).value |= ((next_bit) << ((*s).i));
                    (*s).i.prefix_inc();
                    (*s).stage =
                        (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 132;
                            let _fn: *const u8 = b"DecodeVarint\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
pub unsafe fn DecodeLimitedVarint_145(
    mut s: *mut brunsli_internal_dec_VarintState,
    mut br: *mut brunsli_BrunsliBitReader,
    mut max_symbols: u64,
) -> bool {
    if (((*s).stage as i32) == (brunsli_internal_dec_VarintState_Stage::INIT as i32)) {
        (*s).value = 0_u64;
        (*s).i = 0_u64;
        (*s).stage = (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*s).stage as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if (((*s).i) < (max_symbols)) {
                        if !(unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if ((unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*s).stage =
                                (brunsli_internal_dec_VarintState_Stage::READ_DATA).clone();
                            continue 'loop_;
                        }
                    }
                    (*s).stage = (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                    return true;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 = 2_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let mut next_bits: u64 = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = (2_u64 as u32);
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    (*s).value |= ((next_bits) << (((*s).i).wrapping_mul(2_u64 as u64)));
                    (*s).i.prefix_inc();
                    (*s).stage =
                        (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 169;
                            let _fn: *const u8 = b"DecodeLimitedVarint\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
pub unsafe fn DecodeLimitedVarint_146(
    mut s: *mut brunsli_internal_dec_VarintState,
    mut br: *mut brunsli_BrunsliBitReader,
    mut max_symbols: u64,
) -> bool {
    if (((*s).stage as i32) == (brunsli_internal_dec_VarintState_Stage::INIT as i32)) {
        (*s).value = 0_u64;
        (*s).i = 0_u64;
        (*s).stage = (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*s).stage as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if (((*s).i) < (max_symbols)) {
                        if !(unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if ((unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*s).stage =
                                (brunsli_internal_dec_VarintState_Stage::READ_DATA).clone();
                            continue 'loop_;
                        }
                    }
                    (*s).stage = (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                    return true;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 = 8_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let mut next_bits: u64 = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = (8_u64 as u32);
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    (*s).value |= ((next_bits) << (((*s).i).wrapping_mul(8_u64 as u64)));
                    (*s).i.prefix_inc();
                    (*s).stage =
                        (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION).clone();
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 169;
                            let _fn: *const u8 = b"DecodeLimitedVarint\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
pub unsafe fn GenerateApp0Marker_147(mut app0_status: u8) -> Vec<u8> {
    let mut app0_marker: Vec<u8> = core::slice::from_raw_parts(
        AppData_0xe0_62.as_ptr(),
        (AppData_0xe0_62.as_ptr().offset((17) as isize)).offset_from(AppData_0xe0_62.as_ptr())
            as usize,
    )
    .to_vec();
    app0_marker[(9_u64) as usize] = (if (((app0_status as u32) & (1_u32)) != 0) {
        2
    } else {
        1
    } as u8);
    app0_status = ((app0_status as i32) >> 1_u32) as u8;
    app0_marker[(10_u64) as usize] = (((app0_status as u32) & (3_u32)) as u8);
    app0_status = ((app0_status as i32) >> 2_u32) as u8;
    let mut x_dens: u16 = kApp0Densities_46[(app0_status) as usize];
    app0_marker[(11_u64) as usize] = {
        app0_marker[(13_u64) as usize] = (((x_dens as i32) >> (8_u32)) as u8);
        app0_marker[(13_u64) as usize]
    };
    app0_marker[(12_u64) as usize] = {
        app0_marker[(14_u64) as usize] = (((x_dens as u32) & (255_u32)) as u8);
        app0_marker[(14_u64) as usize]
    };
    return app0_marker;
}
pub unsafe fn GenerateAppMarker_148(mut marker: u8, mut code: u8) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::new();
    if ((marker as i32) == (128)) {
        s = core::slice::from_raw_parts(
            AppData_0xe2_63.as_ptr(),
            (AppData_0xe2_63.as_ptr().offset((3161) as isize)).offset_from(AppData_0xe2_63.as_ptr())
                as usize,
        )
        .to_vec();
        s[(84_u64) as usize] = code;
    } else if ((marker as i32) == (129)) {
        s = core::slice::from_raw_parts(
            AppData_0xec_64.as_ptr(),
            (AppData_0xec_64.as_ptr().offset((18) as isize)).offset_from(AppData_0xec_64.as_ptr())
                as usize,
        )
        .to_vec();
        s[(15_u64) as usize] = code;
    } else {
        if !((marker as i32) == (130)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 197;
                let _fn: *const u8 = b"GenerateAppMarker\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        s = core::slice::from_raw_parts(
            AppData_0xee_65.as_ptr(),
            (AppData_0xee_65.as_ptr().offset((15) as isize)).offset_from(AppData_0xee_65.as_ptr())
                as usize,
        )
        .to_vec();
        s[(10_u64) as usize] = code;
    }
    return s;
}
pub unsafe fn ProcessMetaData_149(
    mut data: *const u8,
    mut len: u64,
    mut state: *mut brunsli_internal_dec_MetadataState,
    mut jpg: *mut brunsli_JPEGData,
) -> bool {
    let mut pos: u64 = 0_u64;
    'loop_: while ((pos) < (len)) {
        'switch: {
            let __match_cond = (*state).stage;
            match __match_cond {
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64) => {
                    (*state).marker = (*data.offset((pos.postfix_inc()) as isize));
                    if (((*state).marker as i32) == (217)) {
                        (*jpg).tail_data = Vec::new();
                        (*state).stage =
                            (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64).clone();
                        continue 'loop_;
                    } else if (((*state).marker as i32) < (64)) {
                        (*state).short_marker_count.postfix_inc();
                        if (((*state).short_marker_count) > (kBrunsliShortMarkerLimit_23 as u64)) {
                            return false;
                        }
                        (*jpg).app_data.push(
                            (unsafe {
                                let _app0_status: u8 = (*state).marker;
                                GenerateApp0Marker_147(_app0_status)
                            }),
                        );
                        continue 'loop_;
                    } else if (((*state).marker as i32) >= (128))
                        && (((*state).marker as i32) <= (130))
                    {
                        (*state).short_marker_count.postfix_inc();
                        if (((*state).short_marker_count) > (kBrunsliShortMarkerLimit_23 as u64)) {
                            return false;
                        }
                        (*state).stage =
                            (brunsli_internal_dec_MetadataState_Stage::READ_CODE as u64).clone();
                        continue 'loop_;
                    }
                    if (((*state).marker as i32) != (254))
                        && ((((*state).marker as i32) >> (4_u32)) != (14))
                    {
                        return false;
                    }
                    (*state).stage =
                        (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_HI as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64) => {
                    (unsafe {
                        let _dst: *mut Vec<u8> = (&mut (*jpg).tail_data as *mut Vec<u8>);
                        let _begin: *const u8 = data.offset((pos) as isize);
                        let _end: *const u8 = data.offset((len) as isize);
                        Append_71(_dst, _begin, _end)
                    });
                    pos = len;
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_CODE as u64) => {
                    let code: u8 = (*data.offset((pos.postfix_inc()) as isize));
                    (*jpg).app_data.push(
                        (unsafe {
                            let _marker: u8 = (*state).marker;
                            let _code: u8 = code;
                            GenerateAppMarker_148(_marker, _code)
                        }),
                    );
                    (*state).stage =
                        (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_HI as u64) => {
                    (*state).length_hi = (*data.offset((pos.postfix_inc()) as isize));
                    (*state).stage =
                        (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_LO as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_LO as u64) => {
                    let lo: u8 = (*data.offset((pos.postfix_inc()) as isize));
                    let mut marker_len: u64 =
                        (((((*state).length_hi as i32) << (8_u32)) + (lo as i32)) as u64);
                    if ((marker_len) < (2_u64)) {
                        return false;
                    }
                    (*state).remaining_multibyte_length = (marker_len).wrapping_sub(2_u64);
                    let mut head: [u8; 3] = [(*state).marker, (*state).length_hi, lo];
                    let mut dest: *mut Vec<Vec<u8>> = if (((*state).marker as i32) == (254)) {
                        (&mut (*jpg).com_data as *mut Vec<Vec<u8>>)
                    } else {
                        (&mut (*jpg).app_data as *mut Vec<Vec<u8>>)
                    };
                    let mut delta: u64 = if (((*state).marker as i32) == (254)) {
                        0_u64
                    } else {
                        (*state).short_marker_count
                    };
                    if ((((*(dest).cast_const()).len() as u64).wrapping_sub(delta))
                        >= (kBrunsliMultibyteMarkerLimit_24 as u64))
                    {
                        return false;
                    }
                    (*dest).push(
                        core::slice::from_raw_parts(
                            head.as_mut_ptr(),
                            (head.as_mut_ptr().offset((3) as isize)).offset_from(head.as_mut_ptr())
                                as usize,
                        )
                        .iter()
                        .map(|x| u8::try_from(x.clone()).ok().unwrap())
                        .collect(),
                    );
                    (*state).multibyte_sink = ((*dest).last_mut().unwrap());
                    (*state).stage = (if (((*state).remaining_multibyte_length) > (0_u64)) {
                        brunsli_internal_dec_MetadataState_Stage::READ_MULTIBYTE
                    } else {
                        brunsli_internal_dec_MetadataState_Stage::READ_MARKER
                    } as u64);
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_MULTIBYTE as u64) => {
                    let mut chunk_size: u64 = {
                        let mut __tmp_1 = (len).wrapping_sub(pos);
                        (*if *&(*state).remaining_multibyte_length <= *&mut __tmp_1 {
                            (&(*state).remaining_multibyte_length) as *const _
                        } else {
                            (&mut __tmp_1) as *const _
                        })
                    };
                    (unsafe {
                        let _dst: *mut Vec<u8> = (*state).multibyte_sink;
                        let _begin: *const u8 = data.offset((pos) as isize);
                        let _length: u64 = chunk_size;
                        Append_72(_dst, _begin, _length)
                    });
                    (*state).remaining_multibyte_length =
                        ((*state).remaining_multibyte_length).wrapping_sub(chunk_size);
                    pos = (pos).wrapping_add(chunk_size);
                    if (((*state).remaining_multibyte_length) == (0_u64)) {
                        (*state).stage =
                            (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64).clone();
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
pub unsafe fn DecodeHuffmanCode_150(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let js: *mut brunsli_internal_dec_JpegInternalsState =
        &mut (*s).internals as *mut brunsli_internal_dec_JpegInternalsState;
    let mut br: *mut brunsli_BrunsliBitReader = (&mut (*js).br as *mut brunsli_BrunsliBitReader);
    'loop_: while true {
        'switch: {
            let __match_cond = ((*js).stage as i32);
            match __match_cond {
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST
                        as i32) =>
                {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 = 1_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    (*js).is_known_last_huffman_code = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    (*jpg)
                        .huffman_code
                        .push(<brunsli_JPEGHuffmanCode>::default());
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_SIMPLE)
                            .clone();
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_SIMPLE
                        as i32) =>
                {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 =
                            (((5) + (!((*js).is_known_last_huffman_code != 0) as i32)) as u64);
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let mut huff: *mut brunsli_JPEGHuffmanCode =
                        (((*jpg).huffman_code).last_mut().unwrap());
                    (*huff).slot_id = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 2_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as i32);
                    (*js).is_dc_table = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) == (0_u32));
                    (*huff).slot_id += if (*js).is_dc_table { 0 } else { 16 };
                    (*huff).is_last = ((*js).is_known_last_huffman_code != 0)
                        || ((unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0);
                    (&mut (*huff)).counts[(0_u64) as usize] = 0;
                    let mut found_match: i32 = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as i32);
                    if (found_match != 0) {
                        if (*js).is_dc_table {
                            let mut huff_table_idx: i32 = ((unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _n_bits: u32 = 1_u32;
                                BrunsliBitReaderRead_126(_br, _n_bits)
                            }) as i32);
                            {
                                if ::std::mem::size_of::<[i32; 16]>() as u64 != 0 {
                                    ::std::ptr::copy_nonoverlapping(
                                        (kStockDCHuffmanCodeCounts_54[(huff_table_idx) as usize]
                                            .as_ptr()
                                            as *const i32
                                            as *const ::libc::c_void),
                                        ((&mut (&mut (*huff)).counts[(1_u64) as usize] as *mut i32)
                                            as *mut i32
                                            as *mut ::libc::c_void),
                                        ::std::mem::size_of::<[i32; 16]>() as u64 as usize,
                                    )
                                }
                                ((&mut (&mut (*huff)).counts[(1_u64) as usize] as *mut i32)
                                    as *mut i32
                                    as *mut ::libc::c_void)
                            };
                            {
                                if ::std::mem::size_of::<[i32; 13]>() as u64 != 0 {
                                    ::std::ptr::copy_nonoverlapping(
                                        (kStockDCHuffmanCodeValues_55[(huff_table_idx) as usize]
                                            .as_ptr()
                                            as *const i32
                                            as *const ::libc::c_void),
                                        ((&mut (&mut (*huff)).values[(0_u64) as usize] as *mut i32)
                                            as *mut i32
                                            as *mut ::libc::c_void),
                                        ::std::mem::size_of::<[i32; 13]>() as u64 as usize,
                                    )
                                }
                                ((&mut (&mut (*huff)).values[(0_u64) as usize] as *mut i32)
                                    as *mut i32
                                    as *mut ::libc::c_void)
                            };
                        } else {
                            let mut huff_table_idx: i32 = ((unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _n_bits: u32 = 1_u32;
                                BrunsliBitReaderRead_126(_br, _n_bits)
                            }) as i32);
                            {
                                if ::std::mem::size_of::<[i32; 16]>() as u64 != 0 {
                                    ::std::ptr::copy_nonoverlapping(
                                        (kStockACHuffmanCodeCounts_57[(huff_table_idx) as usize]
                                            .as_ptr()
                                            as *const i32
                                            as *const ::libc::c_void),
                                        ((&mut (&mut (*huff)).counts[(1_u64) as usize] as *mut i32)
                                            as *mut i32
                                            as *mut ::libc::c_void),
                                        ::std::mem::size_of::<[i32; 16]>() as u64 as usize,
                                    )
                                }
                                ((&mut (&mut (*huff)).counts[(1_u64) as usize] as *mut i32)
                                    as *mut i32
                                    as *mut ::libc::c_void)
                            };
                            {
                                if ::std::mem::size_of::<[i32; 163]>() as u64 != 0 {
                                    ::std::ptr::copy_nonoverlapping(
                                        (kStockACHuffmanCodeValues_59[(huff_table_idx) as usize]
                                            .as_ptr()
                                            as *const i32
                                            as *const ::libc::c_void),
                                        ((&mut (&mut (*huff)).values[(0_u64) as usize] as *mut i32)
                                            as *mut i32
                                            as *mut ::libc::c_void),
                                        ::std::mem::size_of::<[i32; 163]>() as u64 as usize,
                                    )
                                }
                                ((&mut (&mut (*huff)).values[(0_u64) as usize] as *mut i32)
                                    as *mut i32
                                    as *mut ::libc::c_void)
                            };
                        }
                        (*js).stage =
                            (brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE).clone();
                    } else {
                        (unsafe {
                            let _values: Vec<u8> = if (*js).is_dc_table {
                                core::slice::from_raw_parts(
                                    kDefaultDCValues_60.as_ptr(),
                                    (kDefaultDCValues_60.as_ptr().add(kDefaultDCValues_60.len()))
                                        .offset_from(kDefaultDCValues_60.as_ptr())
                                        as usize,
                                )
                                .to_vec()
                            } else {
                                core::slice::from_raw_parts(
                                    kDefaultACValues_61.as_ptr(),
                                    (kDefaultACValues_61.as_ptr().add(kDefaultACValues_61.len()))
                                        .offset_from(kDefaultACValues_61.as_ptr())
                                        as usize,
                                )
                                .to_vec()
                            };
                            (*js).p.Init(_values)
                        });
                        (*js).stage =
                            (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_MAX_LEN)
                                .clone();
                    };
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_MAX_LEN
                        as i32) =>
                {
                    if !(unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u64 = 4_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    (*js).max_len = (((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 4_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    })
                    .wrapping_add(1_u32)) as u64);
                    (*js).total_count = 0_u64;
                    (*js).max_count = (if (*js).is_dc_table {
                        kJpegDCAlphabetSize_9
                    } else {
                        kJpegHuffmanAlphabetSize_8
                    } as u64);
                    (*js).space = (((((1_u32) << (kJpegHuffmanMaxBitLength_7)) as u32)
                        .wrapping_sub(
                            ((1_u32)
                                << ((kJpegHuffmanMaxBitLength_7 as u64)
                                    .wrapping_sub((*js).max_len))),
                        )) as u64);
                    (*js).i = 1_u64;
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_COUNT).clone();
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_COUNT
                        as i32) =>
                {
                    let mut huff: *mut brunsli_JPEGHuffmanCode =
                        (((*jpg).huffman_code).last_mut().unwrap());
                    if (((*js).i) <= ((*js).max_len)) {
                        let mut shift: u64 =
                            (kJpegHuffmanMaxBitLength_7 as u64).wrapping_sub((*js).i);
                        let mut count_limit: u64 = {
                            let mut __tmp_0 = ((*js).max_count).wrapping_sub((*js).total_count);
                            let mut __tmp_1 = (((*js).space) >> (shift));
                            (*if *&mut __tmp_0 <= *&mut __tmp_1 {
                                (&mut __tmp_0) as *const _
                            } else {
                                (&mut __tmp_1) as *const _
                            })
                        };
                        if ((count_limit) > (0_u64)) {
                            let mut nbits: i32 = ((unsafe {
                                let _n: u32 = (count_limit as u32);
                                Log2FloorNonZero_74(_n)
                            }) + (1));
                            if !(unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _n_bits: u64 = (nbits as u64);
                                BrunsliBitReaderCanRead_134(_br, _n_bits)
                            }) {
                                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            }
                            let mut count: u64 = ((unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _n_bits: u32 = (nbits as u32);
                                BrunsliBitReaderRead_126(_br, _n_bits)
                            }) as u64);
                            if ((count) > (count_limit)) {
                                return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            }
                            (&mut (*huff)).counts[((*js).i) as usize] = (count as i32);
                            (*js).total_count = ((*js).total_count).wrapping_add(count);
                            (*js).space = ((*js).space)
                                .wrapping_sub((count).wrapping_mul(((1_u64) << (shift))));
                        }
                        (*js).i.prefix_inc();
                        continue 'loop_;
                    }
                    (&mut (*huff)).counts[((*js).max_len) as usize].prefix_inc();
                    (*js).i = 0_u64;
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_PERMUTATION)
                            .clone();
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_PERMUTATION
                        as i32) =>
                {
                    let mut huff: *mut brunsli_JPEGHuffmanCode =
                        (((*jpg).huffman_code).last_mut().unwrap());
                    if (((*js).i) < ((*js).total_count)) {
                        let nbits: i32 = (unsafe { (*js).p.num_bits() });
                        if !(unsafe {
                            let _s: *mut brunsli_internal_dec_VarintState =
                                (&mut (*js).varint as *mut brunsli_internal_dec_VarintState);
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _max_symbols: u64 = ((((nbits) + (1)) >> (1_u32)) as u64);
                            DecodeLimitedVarint_145(_s, _br, _max_symbols)
                        }) {
                            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let mut value: u8 = 0_u8;
                        if !(unsafe {
                            let _code: u64 = (*js).varint.value;
                            let _value: *mut u8 = (&mut value as *mut u8);
                            (*js).p.Remove(_code, _value)
                        }) {
                            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        }
                        (&mut (*huff)).values[((*js).i) as usize] = (value as i32);
                        (*js).i.prefix_inc();
                        continue 'loop_;
                    }
                    (&mut (*huff)).values[((*js).total_count) as usize] =
                        kJpegHuffmanAlphabetSize_8;
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE).clone();
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE as i32) =>
                {
                    if (*(((*jpg).huffman_code).last_mut().unwrap())).is_last {
                        (*js).terminal_huffman_code_count.postfix_inc();
                    }
                    if ((*js).is_known_last_huffman_code != 0) {
                        (unsafe { (*js).p.Clear() });
                        return brunsli_BrunsliStatus::BRUNSLI_OK;
                    }
                    if (((*jpg).huffman_code.len() as u64) >= (kMaxDHTMarkers_10 as u64)) {
                        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    }
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST).clone();
                    continue 'loop_;
                }
                _ => {
                    return brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                }
            }
        };
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn DecodeScanInfo_151(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let js: *mut brunsli_internal_dec_JpegInternalsState =
        &mut (*s).internals as *mut brunsli_internal_dec_JpegInternalsState;
    let mut br: *mut brunsli_BrunsliBitReader = (&mut (*js).br as *mut brunsli_BrunsliBitReader);
    'loop_: while true {
        'switch: {
            let __match_cond = ((*js).stage as i32);
            match __match_cond { v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON as i32 ) )  => { let mut si : *mut brunsli_JPEGScanInfo = ( & mut ( &mut ( * jpg  ) ) . scan_info  [ ( ( * js ) . i  ) as usize ] as *mut brunsli_JPEGScanInfo ) ;
 ;
 ;
 if ! ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u64  = 22_u64 ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } ( * si  ) . Ss   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 6_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( * si  ) . Se   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 6_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( * si  ) . Ah   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 4_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( * si  ) . Al   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 4_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( * si  ) . num_components   = ( ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 2_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  ) . wrapping_add ( 1_u32 ) ) as u64 )  ;
 ( * js ) . j   = 0_u64  ;
 ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMPONENT ).clone() ;
 ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMPONENT as i32 ) )  => { let mut si : *mut brunsli_JPEGScanInfo = ( & mut ( &mut ( * jpg  ) ) . scan_info  [ ( ( * js ) . i  ) as usize ] as *mut brunsli_JPEGScanInfo ) ;
 ;
 ;
 if ( ( ( * js ) . j  ) < ( ( * si  ) . num_components  ) ) { if ! ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u64  = 6_u64 ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } ( &mut ( * si  ) ) . components  [ ( ( * js ) . j  ) as usize ] . comp_idx   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 2_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as u8 ) )  ;
 ( &mut ( * si  ) ) . components  [ ( ( * js ) . j  ) as usize ] . dc_tbl_idx   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 2_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( &mut ( * si  ) ) . components  [ ( ( * js ) . j  ) as usize ] . ac_tbl_idx   = ( ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 2_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 ( * js ) . j  .postfix_inc() ;
 } else { ( * js ) . last_block_idx   = - 1_i32  ;
 ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION ).clone() ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION as i32 ) )  => { if ! ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u64  = 1_u64 ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } if ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 1_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  != 0 ) { ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_DATA ).clone() ;
 } else { ( * js ) . last_block_idx   = 0  ;
 ( * js ) . last_num   = 0  ;
 ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION ).clone() ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_DATA as i32 ) )  => { let mut si : *mut brunsli_JPEGScanInfo = ( & mut ( &mut ( * jpg  ) ) . scan_info  [ ( ( * js ) . i  ) as usize ] as *mut brunsli_JPEGScanInfo ) ;
 ;
 ;
 if ! ( unsafe { let _s: *mut brunsli_internal_dec_VarintState  = ( & mut ( * js ) . varint  as *mut brunsli_internal_dec_VarintState ) ; let _br: *mut brunsli_BrunsliBitReader  = br ; let _max_bits: u64  = 28_u64 ; DecodeVarint_144 ( _s , _br , _max_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } let mut block_idx : i32 = ( ( ( ( ( * js ) . last_block_idx  ) + ( ( ( ( * js ) . varint  . value  as i32 ) ) ) ) ) + ( 1 ) ) ;
 ;
 ;
 ( * si  ) . reset_points  . push   ( block_idx as i32 )  ;
 ( * js ) . last_block_idx   = block_idx  ;
 if ( ( ( * js ) . last_block_idx  ) > ( ( ( ( 1 ) << ( 30 ) ) ) ) ) { return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN ;
 } ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION ).clone() ;
 ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION as i32 ) )  => { if ! ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u64  = 1_u64 ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } if ( ( unsafe { let _br: *mut brunsli_BrunsliBitReader  = br ; let _n_bits: u32  = 1_u32 ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  != 0 ) { ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_DATA ).clone() ;
 } else { ( unsafe { ( ( | | { if ( ( ( * js ) . last_num  ) > ( 0 ) ) { let mut info : brunsli_JPEGScanInfo_ExtraZeroRunInfo = <brunsli_JPEGScanInfo_ExtraZeroRunInfo >::default() ;
 ;
 ;
 info . block_idx   = ( * js ) . last_block_idx   ;
 info . num_extra_zero_runs   = ( * js ) . last_num   ;
 {let a0_clone = info .clone();
    ( &mut ( * jpg  ) ) . scan_info  [ ( ( * js ) . i  ) as usize ] . extra_zero_runs  .push(a0_clone)} ;
 ( * js ) . last_num   = 0  ;
 } } ) ) ( ) } ) ;
 ( * js ) . i  .prefix_inc() ;
 if ( ( ( * js ) . i  ) < ( ( * js ) . num_scans  ) ) { ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON ).clone() ;
 ;
 continue 'loop_ ;
 } return brunsli_BrunsliStatus::BRUNSLI_OK ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_DATA as i32 ) )  => { if ! ( unsafe { let _s: *mut brunsli_internal_dec_VarintState  = ( & mut ( * js ) . varint  as *mut brunsli_internal_dec_VarintState ) ; let _br: *mut brunsli_BrunsliBitReader  = br ; let _max_bits: u64  = 28_u64 ; DecodeVarint_144 ( _s , _br , _max_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA ;
 } let mut block_idx : i32 = ( ( ( * js ) . last_block_idx  ) + ( ( ( ( * js ) . varint  . value  as i32 ) ) ) ) ;
 ;
 ;
 if ( ( block_idx ) > ( ( * js ) . last_block_idx  ) ) { ( unsafe { ( ( | | { if ( ( ( * js ) . last_num  ) > ( 0 ) ) { let mut info : brunsli_JPEGScanInfo_ExtraZeroRunInfo = <brunsli_JPEGScanInfo_ExtraZeroRunInfo >::default() ;
 ;
 ;
 info . block_idx   = ( * js ) . last_block_idx   ;
 info . num_extra_zero_runs   = ( * js ) . last_num   ;
 {let a0_clone = info .clone();
    ( &mut ( * jpg  ) ) . scan_info  [ ( ( * js ) . i  ) as usize ] . extra_zero_runs  .push(a0_clone)} ;
 ( * js ) . last_num   = 0  ;
 } } ) ) ( ) } ) ;
 } ( * js ) . last_num  .prefix_inc() ;
 ( * js ) . last_block_idx   = block_idx  ;
 if ( ( ( * js ) . last_block_idx  ) > ( ( ( ( 1 ) << ( 30 ) ) ) ) ) { return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN ;
 } ( * js ) . stage   = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION ).clone() ;
 ;
 continue 'loop_ ;
 }, _ => { return brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR ;
 }, }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn DecodeCoeffOrder_152(
    mut order: *mut u32,
    mut br: *mut brunsli_BitSource,
    mut in_: *mut brunsli_WordSource,
) -> bool {
    let mut lehmer: [u32; 64] = [
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ];
    static mut kSpan_153: i32 = unsafe { 16 };;
    let mut i: i32 = 0;
    'loop_: while ((i) < (kDCTBlockSize_3)) {
        if !((unsafe {
            let _nbits: i32 = 1;
            let _in: *mut brunsli_WordSource = in_;
            (*br).ReadBits(_nbits, _in)
        }) != 0)
        {
            i += kSpan_153;
            continue 'loop_;
        }
        let start: i32 = if ((i) > (0)) { i } else { 1 };
        let end: i32 = ((i) + (kSpan_153));
        let mut j: i32 = start;
        'loop_: while ((j) < (end)) {
            let mut v: u32 = 0_u32;
            'loop_: while ((v) <= (kDCTBlockSize_3 as u32)) {
                let bits: u32 = (unsafe {
                    let _nbits: i32 = 3;
                    let _in: *mut brunsli_WordSource = in_;
                    (*br).ReadBits(_nbits, _in)
                });
                v = (v).wrapping_add(bits);
                if ((bits) < (7_u32)) {
                    break;
                }
            }
            if ((v) > (kDCTBlockSize_3 as u32)) {
                return false;
            }
            lehmer[(j) as usize] = v;
            j.prefix_inc();
        }
        i += kSpan_153;
    }
    let mut end: i32 = ((kDCTBlockSize_3) - (1));
    'loop_: while ((end) >= (1)) && ((lehmer[(end) as usize]) == (0_u32)) {
        end.prefix_dec();
    }
    if ((lehmer[(end) as usize]) == (1_u32)) {
        return false;
    }
    let mut i: i32 = 1;
    'loop_: while ((i) <= (end)) {
        if ((lehmer[(i) as usize]) == (0_u32)) {
            return false;
        }
        lehmer[(i) as usize].prefix_dec();
        i.prefix_inc();
    }
    if !(unsafe {
        let _code: *const u32 = (lehmer.as_mut_ptr()).cast_const();
        let _len: u64 = (kDCTBlockSize_3 as u64);
        let _sigma: *mut u32 = order;
        DecodeLehmerCode_113(_code, _len, _sigma)
    }) {
        return false;
    }
    let mut k: i32 = 0;
    'loop_: while ((k) < (kDCTBlockSize_3)) {
        (*order.offset((k) as isize)) =
            kJPEGNaturalOrder_13[(*order.offset((k) as isize)) as usize];
        k.prefix_inc();
    }
    return true;
}
pub unsafe fn DecodeNumNonzeros_154(
    mut p: *mut brunsli_Prob,
    mut ac: *mut brunsli_BinaryArithmeticDecoder,
    mut in_: *mut brunsli_WordSource,
) -> u64 {
    let mut bst: *mut brunsli_Prob = p.offset(-((1) as isize));
    let mut ctx: u64 = 1_u64;
    let mut b: u64 = 0_u64;
    'loop_: while ((b) < (kNumNonZeroBits_84)) {
        let bit: i32 = (unsafe {
            let _prob: i32 = ((unsafe { (*bst.offset((ctx) as isize)).get_proba() }) as i32);
            let _in: *mut brunsli_WordSource = in_;
            (*ac).ReadBit(_prob, _in)
        });
        (unsafe {
            let _val: i32 = bit;
            (*bst.offset((ctx) as isize)).Add(_val)
        });
        ctx = ((2_u64).wrapping_mul(ctx)).wrapping_add((bit as u64));
        b.prefix_inc();
    }
    let mut val: u64 = (ctx).wrapping_sub((((1_u32) << (kNumNonZeroBits_84)) as u64));
    if !((val) <= (kNumNonZeroTreeSize_85)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 593;
            let _fn: *const u8 = b"DecodeNumNonzeros\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return val;
}
pub unsafe fn EnsureSubdecodersInitialized_155(
    mut state: *mut brunsli_internal_dec_State,
    mut in_: *mut brunsli_WordSource,
) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    if !(*s).subdecoders_initialized {
        (unsafe {
            let _in: *mut brunsli_WordSource = in_;
            (*s).ans_decoder.Init(_in)
        });
        (unsafe {
            let _in: *mut brunsli_WordSource = in_;
            (*s).bit_reader.Init(_in)
        });
        (unsafe {
            let _in: *mut brunsli_WordSource = in_;
            (*s).arith_decoder.Init(_in)
        });
        (*s).subdecoders_initialized = true;
    }
}
pub unsafe fn FinalizeSubdecoders_156(mut state: *mut brunsli_internal_dec_State) -> bool {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    if !(unsafe { (*s).ans_decoder.CheckCRC() }) {
        return false;
    }
    if !(unsafe { (*s).bit_reader.Finish() }) {
        return false;
    }
    (*s).subdecoders_initialized = false;
    return true;
}
pub unsafe fn DecodeDC_157(
    mut state: *mut brunsli_internal_dec_State,
    mut in_: *mut brunsli_WordSource,
) -> brunsli_BrunsliStatus {
    let meta: *const Vec<brunsli_internal_dec_ComponentMeta> =
        &(*state).meta as *const Vec<brunsli_internal_dec_ComponentMeta>;
    let num_components: u64 = (*meta).len() as u64;
    let mcu_rows: i32 =
        (((&(*meta))[(0_u64) as usize].height_in_blocks) / ((&(*meta))[(0_u64) as usize].v_samp));
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let ac_dc_state: *mut brunsli_internal_dec_AcDcState =
        &mut (*s).ac_dc as *mut brunsli_internal_dec_AcDcState;
    let comps: *mut Vec<brunsli_ComponentStateDC> =
        &mut (*ac_dc_state).dc as *mut Vec<brunsli_ComponentStateDC>;
    if (*comps).is_empty() {
        {
            let __a0 = num_components as usize;
            (*comps).resize_with(__a0, || <brunsli_ComponentStateDC>::default())
        };
        let mut c: u64 = 0_u64;
        'loop_: while ((c) < (num_components)) {
            (unsafe {
                let _w: i32 = (&(*meta))[(c) as usize].width_in_blocks;
                (&mut (*comps))[(c) as usize].SetWidth(_w)
            });
            c.prefix_inc();
        }
    }
    if !(unsafe {
        let _n: u64 = 5_u64;
        (*in_).CanRead(_n)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _in: *mut brunsli_WordSource = in_;
        EnsureSubdecodersInitialized_155(_state, _in)
    });
    let mut ans: brunsli_ANSDecoder = (*s).ans_decoder.clone();
    let mut br: brunsli_BitSource = (*s).bit_reader.clone();
    let mut ac: brunsli_BinaryArithmeticDecoder = (*s).arith_decoder.clone();
    let mut mcu_y: i32 = (*ac_dc_state).next_mcu_y;
    'loop_: while ((mcu_y) < (mcu_rows)) {
        let mut i: u64 = (*ac_dc_state).next_component;
        'loop_: while ((i) < (num_components)) {
            let mut c: *mut brunsli_ComponentStateDC =
                (&mut (&mut (*comps))[(i) as usize] as *mut brunsli_ComponentStateDC);
            let m: *const brunsli_internal_dec_ComponentMeta =
                &(&(*meta))[(i) as usize] as *const brunsli_internal_dec_ComponentMeta;
            let mut context_map: *const u8 = (*state)
                .context_map
                .offset(((i).wrapping_mul(kNumAvrgContexts_83)) as isize);
            let ac_stride: i32 = ((*m).ac_stride as i32);
            let b_stride: u64 = ((*m).b_stride as u64);
            let width: i32 = (*m).width_in_blocks;
            let mut y: i32 = (((mcu_y) * ((*m).v_samp)) + ((*ac_dc_state).next_iy));
            let prev_sgn: *mut i32 = (&mut (&mut (*c)).prev_sign[(1_u64) as usize] as *mut i32);
            let prev_abs: *mut i32 =
                (&mut (&mut (*c)).prev_abs_coeff[(2_u64) as usize] as *mut i32);
            let mut iy: i32 = (*ac_dc_state).next_iy;
            'loop_: while ((iy) < ((*m).v_samp)) {
                let mut coeffs: *mut i16 = (*m)
                    .ac_coeffs
                    .offset(((y) * (ac_stride)) as isize)
                    .offset((((*ac_dc_state).next_x) * (kDCTBlockSize_3)) as isize);
                let mut block_state: *mut u8 = (*m)
                    .block_state
                    .offset(((y as u64).wrapping_mul(b_stride)) as isize)
                    .offset(((*ac_dc_state).next_x) as isize);
                let mut x: i32 = (*ac_dc_state).next_x;
                'loop_: while ((x) < (width)) {
                    if ((!(unsafe {
                        let _n: u64 = 6_u64;
                        (*in_).CanRead(_n)
                    }) as i64)
                        != 0)
                    {
                        (*ac_dc_state).next_mcu_y = mcu_y;
                        (*ac_dc_state).next_component = i;
                        (*ac_dc_state).next_iy = iy;
                        (*ac_dc_state).next_x = x;
                        (*s).ans_decoder = (ans).clone();
                        (*s).bit_reader = (br).clone();
                        (*s).arith_decoder = (ac).clone();
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let is_empty_ctx: i32 = (unsafe {
                        let _prev: *const i32 =
                            (&mut (&mut (*c)).prev_is_nonempty[(1_u64) as usize] as *mut i32)
                                .cast_const();
                        let _x: i32 = x;
                        IsEmptyBlockContext_106(_prev, _x)
                    });
                    let mut is_empty_p: *mut brunsli_Prob = (&mut (&mut (*c)).is_empty_block_prob
                        [(is_empty_ctx as u64) as usize]
                        as *mut brunsli_Prob);
                    let is_empty_block: bool = !((unsafe {
                        let _prob: i32 =
                            ((unsafe { (*(is_empty_p).cast_const()).get_proba() }) as i32);
                        let _in: *mut brunsli_WordSource = in_;
                        ac.ReadBit(_prob, _in)
                    }) != 0);
                    (unsafe {
                        let _val: i32 = (!is_empty_block as i32);
                        (*is_empty_p).Add(_val)
                    });
                    (&mut (*c)).prev_is_nonempty[(((x) + (1)) as u64) as usize] =
                        (!is_empty_block as i32);
                    (*block_state) = (is_empty_block as u8);
                    let mut abs_val: i32 = 0;
                    let mut sign: i32 = 0;
                    if !is_empty_block {
                        let mut p_is_zero: *mut brunsli_Prob =
                            (&mut (*c).is_zero_prob as *mut brunsli_Prob);
                        let mut is_zero: i32 = (unsafe {
                            let _prob: i32 =
                                ((unsafe { (*(p_is_zero).cast_const()).get_proba() }) as i32);
                            let _in: *mut brunsli_WordSource = in_;
                            ac.ReadBit(_prob, _in)
                        });
                        (unsafe {
                            let _val: i32 = is_zero;
                            (*p_is_zero).Add(_val)
                        });
                        if !(is_zero != 0) {
                            let avg_ctx: i32 = (unsafe {
                                let _vals: *const i32 = (prev_abs).cast_const();
                                let _x: i32 = x;
                                WeightedAverageContextDC_97(_vals, _x)
                            });
                            let sign_ctx: i32 = (((*prev_sgn.offset((x) as isize)) * (3))
                                + (*prev_sgn.offset(((x) - (1)) as isize)));
                            let mut sign_p: *mut brunsli_Prob = (&mut (&mut (*c)).sign_prob
                                [(sign_ctx as u64) as usize]
                                as *mut brunsli_Prob);
                            sign = (unsafe {
                                let _prob: i32 =
                                    ((unsafe { (*(sign_p).cast_const()).get_proba() }) as i32);
                                let _in: *mut brunsli_WordSource = in_;
                                ac.ReadBit(_prob, _in)
                            })
                            .clone();
                            (unsafe {
                                let _val: i32 = sign;
                                (*sign_p).Add(_val)
                            });
                            let entropy_ix: i32 =
                                ((*context_map.offset((avg_ctx) as isize)) as i32);
                            let mut code: i32 = (unsafe {
                                let _code: *const brunsli_ANSDecodingData =
                                    &(*(*state).entropy_codes.offset((entropy_ix) as isize))
                                        as *const brunsli_ANSDecodingData;
                                let _in: *mut brunsli_WordSource = in_;
                                ans.ReadSymbol(_code, _in)
                            });
                            if ((code) < (kNumDirectCodes_135)) {
                                abs_val = ((code) + (1));
                            } else {
                                let mut nbits: i32 = ((code) - (kNumDirectCodes_135));
                                let mut p_first_extra_bit: *mut brunsli_Prob =
                                    (&mut (&mut (*c)).first_extra_bit_prob[(nbits as u64) as usize]
                                        as *mut brunsli_Prob);
                                let mut first_extra_bit: i32 = (unsafe {
                                    let _prob: i32 = ((unsafe {
                                        (*(p_first_extra_bit).cast_const()).get_proba()
                                    })
                                        as i32);
                                    let _in: *mut brunsli_WordSource = in_;
                                    ac.ReadBit(_prob, _in)
                                });
                                (unsafe {
                                    let _val: i32 = first_extra_bit;
                                    (*p_first_extra_bit).Add(_val)
                                });
                                let mut extra_bits_val: i32 = ((first_extra_bit) << (nbits));
                                if ((nbits) > (0)) {
                                    extra_bits_val |= ((unsafe {
                                        let _nbits: i32 = nbits;
                                        let _in: *mut brunsli_WordSource = in_;
                                        br.ReadBits(_nbits, _in)
                                    })
                                        as i32)
                                        .clone();
                                }
                                abs_val = ((((kNumDirectCodes_135) - (1)) + ((2) << (nbits)))
                                    + (extra_bits_val));
                            }
                        }
                    }
                    (*prev_abs.offset((x) as isize)) = abs_val;
                    (*prev_sgn.offset((x) as isize)) =
                        if (abs_val != 0) { ((sign) + (1)) } else { 0 };
                    (*coeffs.offset((0) as isize)) = (((((1) - ((2) * (sign))) * (abs_val))
                        + (unsafe {
                            let _coeffs: *const i16 = (coeffs).cast_const();
                            let _x: i32 = x;
                            let _y: i32 = y;
                            let _stride: i32 = ac_stride;
                            PredictWithAdaptiveMedian_115(_coeffs, _x, _y, _stride)
                        })) as i16);
                    block_state.postfix_inc();
                    coeffs = (coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    x.prefix_inc();
                }
                (*ac_dc_state).next_x = 0;
                iy.prefix_inc();
                y.prefix_inc();
            }
            (*ac_dc_state).next_iy = 0;
            i.prefix_inc();
        }
        (*ac_dc_state).next_component = 0_u64;
        mcu_y.prefix_inc();
    }
    (*ac_dc_state).next_mcu_y = 0;
    (*ac_dc_state).next_component = 0_u64;
    (*ac_dc_state).next_iy = 0;
    (*ac_dc_state).next_x = 0;
    (*comps).clear();
    (*comps).shrink_to_fit();
    (*s).ans_decoder = (ans).clone();
    (*s).bit_reader = (br).clone();
    (*s).arith_decoder = (ac).clone();
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        FinalizeSubdecoders_156(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn DecodeEmptyAcBlock_158(mut prev_sgn: *mut i32, mut prev_abs: *mut i32) {
    let mut k: i32 = 1;
    'loop_: while ((k) < (kDCTBlockSize_3)) {
        (*prev_sgn.offset((k) as isize)) = 0;
        (*prev_abs.offset((k) as isize)) = 0;
        k.prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_AcBlockCookie {
    pub x: i32,
    pub y: i32,
    pub prev_num_nonzeros: *mut u8,
    pub prev_sgn: *mut i32,
    pub prev_abs: *mut i32,
    pub num_nonzero_prob: *mut brunsli_Prob,
    pub ac: *mut brunsli_BinaryArithmeticDecoder,
    pub in_: *mut brunsli_WordSource,
    pub ans: *mut brunsli_ANSDecoder,
    pub br: *mut brunsli_BitSource,
    pub coeffs: *mut i16,
    pub prev_row_coeffs: *const i16,
    pub prev_col_coeffs: *const i16,
    pub is_zero_prob: *mut brunsli_Prob,
    pub order: *const u32,
    pub context_modes: *const u8,
    pub mult_col: *const i32,
    pub mult_row: *const i32,
    pub prev_row_delta: i32,
    pub sign_prob: *mut brunsli_Prob,
    pub context_bits: u64,
    pub context_map: *const u8,
    pub entropy_codes: *const brunsli_ANSDecodingData,
    pub first_extra_bit_prob: *mut brunsli_Prob,
}
pub unsafe fn DecodeAcBlock_159(cookie: *const brunsli_AcBlockCookie) -> u64 {
    let mut c: brunsli_AcBlockCookie = (*cookie).clone();
    let mut ac: brunsli_BinaryArithmeticDecoder = (*c.ac).clone();
    let mut in_: *mut brunsli_WordSource = c.in_;
    let mut ans: brunsli_ANSDecoder = (*c.ans).clone();
    let mut br: brunsli_BitSource = (*c.br).clone();
    let mut num_nonzeros: u64 = 0_u64;
    let nonzero_ctx: u8 = (unsafe {
        let _prev: *const u8 = (c.prev_num_nonzeros).cast_const();
        let _x: i32 = c.x;
        let _y: i32 = c.y;
        NumNonzerosContext_104(_prev, _x, _y)
    });
    let mut last_nz: u64 = (unsafe {
        let _p: *mut brunsli_Prob = c
            .num_nonzero_prob
            .offset(((kNumNonZeroTreeSize_85).wrapping_mul((nonzero_ctx as u64))) as isize);
        let _ac: *mut brunsli_BinaryArithmeticDecoder =
            (&mut ac as *mut brunsli_BinaryArithmeticDecoder);
        let _in: *mut brunsli_WordSource = in_;
        DecodeNumNonzeros_154(_p, _ac, _in)
    });
    let mut k: u64 = (last_nz).wrapping_add(1_u64);
    'loop_: while ((k) < (kDCTBlockSize_3 as u64)) {
        (*c.prev_sgn.offset((k) as isize)) = 0;
        (*c.prev_abs.offset((k) as isize)) = 0;
        k.prefix_inc();
    }
    let mut k: u64 = last_nz;
    'loop_: while ((k) > (0_u64)) {
        let mut is_zero: i32 = 0;
        if ((k) < (last_nz)) {
            let mut bucket: u64 =
                (kNonzeroBuckets_89[((num_nonzeros).wrapping_sub(1_u64)) as usize] as u64);
            let mut is_zero_ctx: u64 =
                ((bucket).wrapping_mul((kDCTBlockSize_3 as u64))).wrapping_add(k);
            let p: *mut brunsli_Prob =
                &mut (*c.is_zero_prob.offset((is_zero_ctx) as isize)) as *mut brunsli_Prob;
            is_zero = (unsafe {
                let _prob: i32 = ((unsafe { (*p).get_proba() }) as i32);
                let _in: *mut brunsli_WordSource = in_;
                ac.ReadBit(_prob, _in)
            })
            .clone();
            (unsafe {
                let _val: i32 = is_zero;
                (*p).Add(_val)
            });
        }
        let mut abs_val: i32 = 0;
        let mut sign: i32 = 1;
        let k_nat: i32 = ((*c.order.offset((k) as isize)) as i32);
        if !(is_zero != 0) {
            let mut context_type: u64 = ((*c.context_modes.offset((k_nat) as isize)) as u64);
            let mut avg_ctx: u64 = 0_u64;
            let mut sign_ctx: u64 = kMaxAverageContext_82;
            if (((context_type) & (1_u64)) != 0) && ((c.y) > (0)) {
                let mut offset: u64 = (((k_nat) & (7)) as u64);
                (unsafe {
                    let _prev: *const i16 = c.prev_row_coeffs.offset((offset) as isize);
                    let _cur: *const i16 = (c.coeffs.offset((offset) as isize)).cast_const();
                    let _mult: *const i32 =
                        c.mult_col.offset(((offset).wrapping_mul(8_u64)) as isize);
                    let _avg_ctx: *mut u64 = (&mut avg_ctx as *mut u64);
                    let _sgn: *mut u64 = (&mut sign_ctx as *mut u64);
                    ACPredictContextRow_103(_prev, _cur, _mult, _avg_ctx, _sgn)
                });
            } else if (((context_type) & (2_u64)) != 0) && ((c.x) > (0)) {
                let mut offset: u64 = (((k_nat) & (!7)) as u64);
                (unsafe {
                    let _prev: *const i16 = c.prev_col_coeffs.offset((offset) as isize);
                    let _cur: *const i16 = (c.coeffs.offset((offset) as isize)).cast_const();
                    let _mult: *const i32 = c.mult_row.offset((offset) as isize);
                    let _avg_ctx: *mut u64 = (&mut avg_ctx as *mut u64);
                    let _sgn: *mut u64 = (&mut sign_ctx as *mut u64);
                    ACPredictContextCol_102(_prev, _cur, _mult, _avg_ctx, _sgn)
                });
            } else if !(context_type != 0) {
                avg_ctx = ((unsafe {
                    let _vals: *const i32 = (c.prev_abs.offset((k) as isize)).cast_const();
                    let _prev_row_delta: i32 = c.prev_row_delta;
                    WeightedAverageContext_98(_vals, _prev_row_delta)
                }) as u64);
                sign_ctx = ((((*c.prev_sgn.offset((k) as isize)) * (3))
                    + (*c.prev_sgn.offset(((k as i32) - (kDCTBlockSize_3)) as isize)))
                    as u64);
            }
            sign_ctx = ((sign_ctx).wrapping_mul((kDCTBlockSize_3 as u64))).wrapping_add(k);
            let sign_p: *mut brunsli_Prob =
                &mut (*c.sign_prob.offset((sign_ctx) as isize)) as *mut brunsli_Prob;
            sign = (unsafe {
                let _prob: i32 = ((unsafe { (*sign_p).get_proba() }) as i32);
                let _in: *mut brunsli_WordSource = in_;
                ac.ReadBit(_prob, _in)
            })
            .clone();
            (unsafe {
                let _val: i32 = sign;
                (*sign_p).Add(_val)
            });
            (*c.prev_sgn.offset((k) as isize)) = ((sign) + (1));
            sign = ((1) - ((2) * (sign)));
            let z_dens_ctx: u64 = ((unsafe {
                let _nonzeros_left: u64 = num_nonzeros;
                let _k: u64 = k;
                let _bits: u64 = c.context_bits;
                ZeroDensityContext_96(_nonzeros_left, _k, _bits)
            }) as u64);
            let mut histo_ix: u64 =
                ((z_dens_ctx).wrapping_mul(kNumAvrgContexts_83)).wrapping_add(avg_ctx);
            let mut entropy_ix: u64 = ((*c.context_map.offset((histo_ix) as isize)) as u64);
            let mut code: i32 = (unsafe {
                let _code: *const brunsli_ANSDecodingData =
                    &(*c.entropy_codes.offset((entropy_ix) as isize))
                        as *const brunsli_ANSDecodingData;
                let _in: *mut brunsli_WordSource = in_;
                ans.ReadSymbol(_code, _in)
            });
            if ((code) < (kNumDirectCodes_135)) {
                abs_val = ((code) + (1));
            } else {
                let mut nbits: i32 = ((code) - (kNumDirectCodes_135));
                let p: *mut brunsli_Prob = &mut (*c
                    .first_extra_bit_prob
                    .offset((((k).wrapping_mul(10_u64)).wrapping_add((nbits as u64))) as isize))
                    as *mut brunsli_Prob;
                let mut first_extra_bit: i32 = (unsafe {
                    let _prob: i32 = ((unsafe { (*p).get_proba() }) as i32);
                    let _in: *mut brunsli_WordSource = in_;
                    ac.ReadBit(_prob, _in)
                });
                (unsafe {
                    let _val: i32 = first_extra_bit;
                    (*p).Add(_val)
                });
                let mut extra_bits_val: i32 = ((first_extra_bit) << (nbits));
                if ((nbits) > (0)) {
                    extra_bits_val = ((extra_bits_val as u32)
                        | (unsafe {
                            let _nbits: i32 = nbits;
                            let _in: *mut brunsli_WordSource = in_;
                            br.ReadBits(_nbits, _in)
                        })) as i32;
                }
                abs_val = ((((((kNumDirectCodes_135) - (1)) as u32)
                    .wrapping_add(((2_u32) << (nbits))))
                .wrapping_add((extra_bits_val as u32))) as i32);
            }
            num_nonzeros.prefix_inc();
        } else {
            (*c.prev_sgn.offset((k) as isize)) = 0;
        }
        let mut coeff: i32 = ((sign) * (abs_val));
        (*c.coeffs.offset((k_nat) as isize)) = (coeff as i16);
        (*c.prev_abs.offset((k) as isize)) = abs_val;
        k.prefix_dec();
    }
    (*c.ans) = (ans).clone();
    (*c.br) = (br).clone();
    (*c.ac) = (ac).clone();
    return num_nonzeros;
}
pub unsafe fn DecodeAC_160(
    mut state: *mut brunsli_internal_dec_State,
    mut in_: *mut brunsli_WordSource,
) -> brunsli_BrunsliStatus {
    let meta: *const Vec<brunsli_internal_dec_ComponentMeta> =
        &(*state).meta as *const Vec<brunsli_internal_dec_ComponentMeta>;
    let num_components: u64 = (*meta).len() as u64;
    let mcu_rows: i32 =
        (((&(*meta))[(0_u64) as usize].height_in_blocks) / ((&(*meta))[(0_u64) as usize].v_samp));
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let ac_dc_state: *mut brunsli_internal_dec_AcDcState =
        &mut (*s).ac_dc as *mut brunsli_internal_dec_AcDcState;
    let comps: *mut Vec<brunsli_ComponentState> =
        &mut (*ac_dc_state).ac as *mut Vec<brunsli_ComponentState>;
    if (*comps).is_empty() {
        {
            let __a0 = num_components as usize;
            (*comps).resize_with(__a0, || <brunsli_ComponentState>::default())
        };
        let mut c: u64 = 0_u64;
        'loop_: while ((c) < (num_components)) {
            (unsafe {
                let _w: i32 = (&(*meta))[(c) as usize].width_in_blocks;
                (&mut (*comps))[(c) as usize].SetWidth(_w)
            });
            (unsafe {
                let _quant: *const i32 =
                    (&(&(*meta))[(c) as usize].quant[(0_u64) as usize] as *const i32);
                let _mult_row: *mut i32 = (&mut (*comps))[(c) as usize].mult_row.as_mut_ptr();
                let _mult_col: *mut i32 = (&mut (*comps))[(c) as usize].mult_col.as_mut_ptr();
                ComputeACPredictMultipliers_109(_quant, _mult_row, _mult_col)
            });
            c.prefix_inc();
        }
    }
    if !(unsafe {
        let _n: u64 = 5_u64;
        (*in_).CanRead(_n)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _in: *mut brunsli_WordSource = in_;
        EnsureSubdecodersInitialized_155(_state, _in)
    });
    if !(*ac_dc_state).ac_coeffs_order_decoded {
        'loop_: while (((*ac_dc_state).next_component) < (num_components)) {
            if !(unsafe {
                let _n: u64 = 121_u64;
                (*in_).CanRead(_n)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            }
            if !(unsafe {
                let _order: *mut u32 = (&mut (*comps))[((*ac_dc_state).next_component) as usize]
                    .order
                    .as_mut_ptr();
                let _br: *mut brunsli_BitSource = (&mut (*s).bit_reader as *mut brunsli_BitSource);
                let _in: *mut brunsli_WordSource = in_;
                DecodeCoeffOrder_152(_order, _br, _in)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            }
            (*ac_dc_state).next_component.postfix_inc();
        }
        (*ac_dc_state).next_component = 0_u64;
        (*ac_dc_state).ac_coeffs_order_decoded = true;
    }
    let mut c: brunsli_AcBlockCookie = <brunsli_AcBlockCookie>::default();
    c.ac = (&mut (*s).arith_decoder as *mut brunsli_BinaryArithmeticDecoder);
    c.in_ = in_;
    c.ans = (&mut (*s).ans_decoder as *mut brunsli_ANSDecoder);
    c.br = (&mut (*s).bit_reader as *mut brunsli_BitSource);
    c.entropy_codes = (*state).entropy_codes;
    c.context_modes = kContextAlgorithm_95.as_ptr().offset(
        (if (*state).use_legacy_context_model {
            64
        } else {
            0
        }) as isize,
    );
    let mut mcu_y: i32 = (*ac_dc_state).next_mcu_y;
    'loop_: while ((mcu_y) < (mcu_rows)) {
        let mut i: u64 = (*ac_dc_state).next_component;
        'loop_: while ((i) < (num_components)) {
            let cst: *mut brunsli_ComponentState =
                &mut (&mut (*comps))[(i) as usize] as *mut brunsli_ComponentState;
            c.prev_num_nonzeros = (*cst).prev_num_nonzeros.as_mut_ptr();
            c.num_nonzero_prob = (*cst).num_nonzero_prob.as_mut_ptr();
            c.is_zero_prob = (*cst).is_zero_prob.as_mut_ptr();
            c.order = ((*cst).order.as_mut_ptr()).cast_const();
            c.mult_col = ((*cst).mult_col.as_mut_ptr()).cast_const();
            c.mult_row = ((*cst).mult_row.as_mut_ptr()).cast_const();
            c.sign_prob = (*cst).sign_prob.as_mut_ptr();
            c.first_extra_bit_prob = (*cst).first_extra_bit_prob.as_mut_ptr();
            let m: *const brunsli_internal_dec_ComponentMeta =
                &(&(*meta))[(i) as usize] as *const brunsli_internal_dec_ComponentMeta;
            c.context_map = (*state)
                .context_map
                .offset((((*m).context_offset).wrapping_mul(kNumAvrgContexts_83)) as isize);
            c.context_bits = (*m).context_bits;
            let width: i32 = (*m).width_in_blocks;
            let ac_stride: u64 = ((*m).ac_stride as u64);
            let b_stride: u64 = ((*m).b_stride as u64);
            let next_iy: i32 = (*ac_dc_state).next_iy;
            c.y = (((mcu_y) * ((*m).v_samp)) + (next_iy));
            c.prev_row_delta = (((((1_u32)
                .wrapping_sub((2_u32).wrapping_mul(((c.y as u32) & (1_u32)))))
            .wrapping_mul((((width) + (3)) as u32)))
            .wrapping_mul((kDCTBlockSize_3 as u32))) as i32);
            let mut iy: i32 = next_iy;
            'loop_: while ((iy) < ((*m).v_samp)) {
                let next_x: i32 = (*ac_dc_state).next_x;
                let block_offset: u64 = (((next_x) * (kDCTBlockSize_3)) as u64);
                c.coeffs = (*m)
                    .ac_coeffs
                    .offset(((c.y as u64).wrapping_mul(ac_stride)) as isize)
                    .offset((block_offset) as isize);
                c.prev_row_coeffs = (c.coeffs.offset(-((ac_stride) as isize))).cast_const();
                c.prev_col_coeffs = (c.coeffs.offset(-((kDCTBlockSize_3) as isize))).cast_const();
                let mut block_state: *const u8 = ((*m)
                    .block_state
                    .offset(((c.y as u64).wrapping_mul(b_stride)) as isize)
                    .offset((next_x) as isize))
                .cast_const();
                c.prev_sgn = (&mut (&mut (*cst)).prev_sign[(kDCTBlockSize_3 as u64) as usize]
                    as *mut i32)
                    .offset((block_offset) as isize);
                c.prev_abs = (&mut (&mut (*cst)).prev_abs_coeff[((((((c.y as u32) & (1_u32))
                    .wrapping_mul((((width) + (3)) as u32)))
                .wrapping_add(2_u32))
                .wrapping_mul((kDCTBlockSize_3 as u32)))
                    as u64)
                    as usize] as *mut i32)
                    .offset((block_offset) as isize);
                c.x = next_x;
                'loop_: while ((c.x) < (width)) {
                    let mut is_empty: bool = ((*(block_state.postfix_inc())) != 0);
                    if !is_empty {
                        if ((!(unsafe {
                            let _n: u64 = 297_u64;
                            (*in_).CanRead(_n)
                        }) as i64)
                            != 0)
                        {
                            (*ac_dc_state).next_mcu_y = mcu_y;
                            (*ac_dc_state).next_component = i;
                            (*ac_dc_state).next_iy = iy;
                            (*ac_dc_state).next_x = c.x;
                            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let mut num_nonzeros: u64 = (unsafe {
                            let _cookie: *const brunsli_AcBlockCookie =
                                &c as *const brunsli_AcBlockCookie;
                            DecodeAcBlock_159(_cookie)
                        });
                        if !((num_nonzeros) <= (kNumNonZeroTreeSize_85)) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 949;
                                let _fn: *const u8 = b"DecodeAC\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        (*c.prev_num_nonzeros.offset((c.x) as isize)) = (num_nonzeros as u8);
                    } else {
                        (unsafe {
                            let _prev_sgn: *mut i32 = c.prev_sgn;
                            let _prev_abs: *mut i32 = c.prev_abs;
                            DecodeEmptyAcBlock_158(_prev_sgn, _prev_abs)
                        });
                        (*c.prev_num_nonzeros.offset((c.x) as isize)) = 0_u8;
                    }
                    c.coeffs = (c.coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    c.prev_sgn = (c.prev_sgn).wrapping_add(kDCTBlockSize_3 as usize);
                    c.prev_abs = (c.prev_abs).wrapping_add(kDCTBlockSize_3 as usize);
                    c.prev_row_coeffs = (c.prev_row_coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    c.prev_col_coeffs = (c.prev_col_coeffs).wrapping_add(kDCTBlockSize_3 as usize);
                    c.x.prefix_inc();
                }
                c.prev_row_delta *= -1_i32;
                (*ac_dc_state).next_x = 0;
                iy.prefix_inc();
                c.y.prefix_inc();
            }
            (*ac_dc_state).next_iy = 0;
            i.prefix_inc();
        }
        (*ac_dc_state).next_component = 0_u64;
        mcu_y.prefix_inc();
    }
    (*ac_dc_state).next_mcu_y = 0;
    (*comps).clear();
    (*comps).shrink_to_fit();
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        FinalizeSubdecoders_156(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn CheckCanRead_161(
    mut state: *mut brunsli_internal_dec_State,
    mut required: u64,
) -> bool {
    let mut available: u64 = ((*state).len).wrapping_sub((*state).pos);
    return ((required) <= (available));
}
pub unsafe fn CheckCanReadByte_162(mut state: *mut brunsli_internal_dec_State) -> bool {
    return (((*state).pos) != ((*state).len));
}
pub unsafe fn ReadByte_163(mut state: *mut brunsli_internal_dec_State) -> u8 {
    return (*(*state).data.offset(((*state).pos.postfix_inc()) as isize));
}
pub unsafe fn PeekByte_164(mut state: *mut brunsli_internal_dec_State, mut offset: u64) -> u8 {
    return (*(*state)
        .data
        .offset((((*state).pos).wrapping_add(offset)) as isize));
}
pub unsafe fn SkipBytes_165(mut state: *mut brunsli_internal_dec_State, mut len: u64) {
    (*state).pos = ((*state).pos).wrapping_add(len);
}
pub unsafe fn GetBytesAvailable_166(mut state: *mut brunsli_internal_dec_State) -> u64 {
    return ((*state).len).wrapping_sub((*state).pos);
}
pub unsafe fn SkipAvailableBytes_167(
    mut state: *mut brunsli_internal_dec_State,
    mut len: u64,
) -> u64 {
    let mut available: u64 = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        GetBytesAvailable_166(_state)
    });
    let mut skip_bytes: u64 = (*if *&mut available <= *&mut len {
        (&mut available) as *const _
    } else {
        (&mut len) as *const _
    });
    (*state).pos = ((*state).pos).wrapping_add(skip_bytes);
    return skip_bytes;
}
pub unsafe fn DecodeBase128_168(
    mut state: *mut brunsli_internal_dec_State,
    mut val: *mut u64,
) -> brunsli_BrunsliStatus {
    (*val) = 0_u64;
    let mut b: u64 = 128_u64;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (9_u64)) && (((b) & (128_u64)) != 0) {
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _required: u64 = (i).wrapping_add(1_u64);
            CheckCanRead_161(_state, _required)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        b = ((unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _offset: u64 = i;
            PeekByte_164(_state, _offset)
        }) as u64);
        (*val) = (((*val) as u64) | (((b) & (127_u64)) << ((i).wrapping_mul(7_u64)))) as u64;
        i.prefix_inc();
    }
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _len: u64 = i;
        SkipBytes_165(_state, _len)
    });
    return if (((b) & (128_u64)) == (0_u64)) {
        brunsli_BrunsliStatus::BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
    };
}
pub unsafe fn Fail_169(
    mut state: *mut brunsli_internal_dec_State,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    (*s).result = (result).clone();
    (*s).last_stage = ((*state).stage).clone();
    return brunsli_internal_dec_Stage::ERROR;
}
pub unsafe fn ReadTag_170(
    mut state: *mut brunsli_internal_dec_State,
    mut section: *mut brunsli_internal_dec_SectionState,
) -> brunsli_BrunsliStatus {
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        CheckCanReadByte_162(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    let marker: u8 = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        ReadByte_163(_state)
    });
    let tag: u64 = (((marker as i32) >> (3_u32)) as u64);
    if ((tag) == (0_u64)) || ((tag) > (15_u64)) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*section).tag = tag;
    let wiring_type: u64 = (((marker as u32) & (7_u32)) as u64);
    if ((wiring_type) != (kBrunsliWiringTypeVarint_25 as u64))
        && ((wiring_type) != (kBrunsliWiringTypeLengthDelimited_26 as u64))
    {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*section).is_section = ((wiring_type) == (kBrunsliWiringTypeLengthDelimited_26 as u64));
    let tag_bit: u32 = ((1_u32) << (tag));
    if ((((*section).tags_met) & (tag_bit)) != 0) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "Duplicate marker {:x}\n",
            (marker as i32),
        );
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*section).tags_met |= tag_bit;
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn EnterSection_171(
    mut state: *mut brunsli_internal_dec_State,
    mut section: *mut brunsli_internal_dec_SectionState,
) -> brunsli_BrunsliStatus {
    let mut section_size: u64 = 0_u64;
    let mut status: brunsli_BrunsliStatus = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _val: *mut u64 = (&mut section_size as *mut u64);
        DecodeBase128_168(_state, _val)
    });
    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
        return status;
    }
    (*section).is_active = true;
    (*section).remaining = section_size;
    (*section).milestone = (*state).pos;
    (*section).projected_end = ((*state).pos).wrapping_add((*section).remaining);
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn LeaveSection_172(mut section: *mut brunsli_internal_dec_SectionState) {
    (*section).is_active = false;
}
pub unsafe fn IsOutOfSectionBounds_173(mut state: *mut brunsli_internal_dec_State) -> bool {
    return (((*state).pos)
        > ((*(*state).internal.as_deref_mut().unwrap())
            .section
            .projected_end));
}
pub unsafe fn RemainingSectionLength_174(mut state: *mut brunsli_internal_dec_State) -> u64 {
    if (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        IsOutOfSectionBounds_173(_state)
    }) {
        return 0_u64;
    }
    return ((*(*state).internal.as_deref_mut().unwrap())
        .section
        .projected_end)
        .wrapping_sub((*state).pos);
}
pub unsafe fn IsAtSectionBoundary_175(mut state: *mut brunsli_internal_dec_State) -> bool {
    return (((*state).pos)
        == ((*(*state).internal.as_deref_mut().unwrap())
            .section
            .projected_end));
}
pub unsafe fn VerifySignature_176(
    mut state: *mut brunsli_internal_dec_State,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _required: u64 = kBrunsliSignatureSize_43;
        CheckCanRead_161(_state, _required)
    }) {
        return (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            Fail_169(_state, _result)
        });
    }
    let is_signature_ok: bool = (({
        let sa = core::slice::from_raw_parts(
            ((*state).data.offset(((*state).pos) as isize) as *const u8 as *const ::libc::c_void)
                as *const u8,
            kBrunsliSignatureSize_43 as usize,
        );
        let sb = core::slice::from_raw_parts(
            (kBrunsliSignature_44.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
            kBrunsliSignatureSize_43 as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) != (0));
    (*state).pos = ((*state).pos).wrapping_add(kBrunsliSignatureSize_43);
    (*s).section.tags_met =
        (((*s).section.tags_met as u32) | ((1_u32) << (kBrunsliSignatureTag_30 as i32))) as u32;
    if is_signature_ok {
        return (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            Fail_169(_state, _result)
        });
    }
    return brunsli_internal_dec_Stage::HEADER;
}
pub unsafe fn DecodeHeader_177(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let hs: *mut brunsli_internal_dec_HeaderState =
        &mut (*s).header as *mut brunsli_internal_dec_HeaderState;
    'loop_: while (((*hs).stage) != (brunsli_internal_dec_HeaderState_Stage::DONE as u64)) {
        'switch: {
            let __match_cond = (*hs).stage;
            match __match_cond {
                v if v == (brunsli_internal_dec_HeaderState_Stage::READ_TAG as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        ReadTag_170(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    if (((*s).section.tag) != (kBrunsliHeaderTag_31 as u64))
                        || (!(*s).section.is_section)
                    {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ENTER_SECTION as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ENTER_SECTION as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        EnterSection_171(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64) => {
                    if (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        IsAtSectionBoundary_175(_state)
                    }) {
                        (*hs).stage =
                            (brunsli_internal_dec_HeaderState_Stage::FINALE as u64).clone();
                        break 'switch;
                    }
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*hs).section as *mut brunsli_internal_dec_SectionState);
                        ReadTag_170(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    let tag_bit: u32 = ((1_u32) << ((*hs).section.tag));
                    if (*hs).section.is_section {
                        if (((kKnownHeaderVarintTags_138) & (tag_bit)) != 0) {
                            (unsafe {
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        (*hs).stage = (brunsli_internal_dec_HeaderState_Stage::ITEM_ENTER_SECTION
                            as u64)
                            .clone();
                        break 'switch;
                    }
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_VALUE as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_ENTER_SECTION as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _val: *mut u64 = (&mut (*hs).remaining_skip_length as *mut u64);
                        DecodeBase128_168(_state, _val)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_SKIP_CONTENTS as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_SKIP_CONTENTS as u64) => {
                    let mut bytes_skipped: u64 = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _len: u64 = (*hs).remaining_skip_length;
                        SkipAvailableBytes_167(_state, _len)
                    });
                    (*hs).remaining_skip_length =
                        ((*hs).remaining_skip_length).wrapping_sub(bytes_skipped);
                    if (((*hs).remaining_skip_length) > (0_u64)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            Fail_169(_state, _result)
                        });
                    }
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_VALUE as u64) => {
                    let mut value: u64 = 0_u64;
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _val: *mut u64 = (&mut value as *mut u64);
                        DecodeBase128_168(_state, _val)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    (&mut (*hs)).varint_values[((*hs).section.tag) as usize] = value;
                    (*hs).stage =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::FINALE as u64) => {
                    let has_version: bool = ((((*hs).section.tags_met)
                        & ((1_u32) << (kBrunsliHeaderVersionCompTag_41 as i32)))
                        != 0);
                    if !has_version {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let version_and_comp_count: u64 = (&mut (*hs)).varint_values
                        [(kBrunsliHeaderVersionCompTag_41 as u64) as usize];
                    let version: u64 = ((version_and_comp_count) >> (2_u32));
                    (*jpg).version = (version as i32);
                    if ((version) == (1_u64)) {
                        (*jpg).width = 0;
                        (*jpg).height = 0;
                        (*hs).stage = (brunsli_internal_dec_HeaderState_Stage::DONE as u64).clone();
                        break 'switch;
                    }
                    if (((version) & (1_u64)) != (0_u64)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    if (((version) & (!7_u32 as u64)) != (0_u64)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*state).use_legacy_context_model = !(((version) & (2_u64)) != 0);
                    (*s).section.tags_met = (((*s).section.tags_met as u32)
                        | ((1_u32) << (kBrunsliOriginalJpgTag_38 as i32)))
                        as u32;
                    let has_width: bool = ((((*hs).section.tags_met)
                        & ((1_u32) << (kBrunsliHeaderWidthTag_39 as i32)))
                        != 0);
                    if !has_width {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let width: u64 =
                        (&mut (*hs)).varint_values[(kBrunsliHeaderWidthTag_39 as u64) as usize];
                    let has_height: bool = ((((*hs).section.tags_met)
                        & ((1_u32) << (kBrunsliHeaderHeightTag_40 as i32)))
                        != 0);
                    if !has_height {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let height: u64 =
                        (&mut (*hs)).varint_values[(kBrunsliHeaderHeightTag_40 as u64) as usize];
                    if ((width) == (0_u64)) || ((height) == (0_u64)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    if ((width) > (kMaxDimPixels_11 as u64))
                        || ((height) > (kMaxDimPixels_11 as u64))
                    {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*jpg).width = (width as i32);
                    (*jpg).height = (height as i32);
                    let num_components: u64 =
                        ((version_and_comp_count) & (3_u64)).wrapping_add(1_u64);
                    {
                        let __a0 = num_components as usize;
                        (*jpg)
                            .components
                            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
                    };
                    let has_subsampling: bool = ((((*hs).section.tags_met)
                        & ((1_u32) << (kBrunsliHeaderSubsamplingTag_42 as i32)))
                        != 0);
                    if !has_subsampling {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let mut subsampling_code: u64 = (&mut (*hs)).varint_values
                        [(kBrunsliHeaderSubsamplingTag_42 as u64) as usize];
                    let mut i: u64 = 0_u64;
                    'loop_: while ((i) < ((*jpg).components.len() as u64)) {
                        let mut c: *mut brunsli_JPEGComponent = (&mut (&mut (*jpg)).components
                            [(i) as usize]
                            as *mut brunsli_JPEGComponent);
                        (*c).v_samp_factor =
                            ((((subsampling_code) & (15_u64)).wrapping_add(1_u64)) as i32);
                        subsampling_code >>= 4_u32;
                        (*c).h_samp_factor =
                            ((((subsampling_code) & (15_u64)).wrapping_add(1_u64)) as i32);
                        subsampling_code >>= 4_u32;
                        if (((*c).v_samp_factor) > (kBrunsliMaxSampling_27)) {
                            return (unsafe {
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        if (((*c).h_samp_factor) > (kBrunsliMaxSampling_27)) {
                            return (unsafe {
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        i.prefix_inc();
                    }
                    if !(unsafe {
                        let _jpg: *mut brunsli_JPEGData = jpg;
                        UpdateSubsamplingDerivatives_178(_jpg)
                    }) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (unsafe {
                        let _jpg: *const brunsli_JPEGData = (jpg).cast_const();
                        let _state: *mut brunsli_internal_dec_State = state;
                        PrepareMeta_179(_jpg, _state)
                    });
                    (*hs).stage = (brunsli_internal_dec_HeaderState_Stage::DONE as u64).clone();
                    break 'switch;
                }
                _ => {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    (unsafe {
        let _section: *mut brunsli_internal_dec_SectionState =
            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
        LeaveSection_172(_section)
    });
    return if (((*jpg).version) == (kFallbackVersion_2)) {
        brunsli_internal_dec_Stage::FALLBACK
    } else {
        brunsli_internal_dec_Stage::SECTION
    };
}
pub unsafe fn DecodeMetaDataSection_180(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let ms: *mut brunsli_internal_dec_MetadataState =
        &mut (*s).metadata as *mut brunsli_internal_dec_MetadataState;
    if (((*ms).decompression_stage) == (brunsli_internal_dec_MetadataDecompressionStage::DONE)) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if (((*ms).decompression_stage) == (brunsli_internal_dec_MetadataDecompressionStage::INITIAL)) {
        if (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsAtSectionBoundary_175(_state)
        }) {
            (*ms).decompression_stage =
                (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
            return brunsli_BrunsliStatus::BRUNSLI_OK;
        }
        if ((unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        }) == (1_u64))
        {
            if !(unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                CheckCanReadByte_162(_state)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            }
            let mut data: [u8; 1] = [0_u8; 1];
            data[(0) as usize] = (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                ReadByte_163(_state)
            })
            .clone();
            let mut ok: bool = (unsafe {
                let _data: *const u8 = (data.as_mut_ptr()).cast_const();
                let _len: u64 = 1_u64;
                let _state: *mut brunsli_internal_dec_MetadataState = (ms);
                let _jpg: *mut brunsli_JPEGData = jpg;
                ProcessMetaData_149(_data, _len, _state, _jpg)
            }) && (unsafe { (*ms).CanFinish() });
            (*ms).decompression_stage =
                (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
            return if ok {
                brunsli_BrunsliStatus::BRUNSLI_OK
            } else {
                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
            };
        }
        (*ms).decompression_stage =
            (brunsli_internal_dec_MetadataDecompressionStage::READ_LENGTH).clone();
    }
    if (((*ms).decompression_stage)
        == (brunsli_internal_dec_MetadataDecompressionStage::READ_LENGTH))
    {
        let mut status: brunsli_BrunsliStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _val: *mut u64 = (&mut (*ms).metadata_size as *mut u64);
            DecodeBase128_168(_state, _val)
        });
        if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return status;
        }
        if (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsOutOfSectionBounds_173(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        if ((unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        }) == (0_u64))
        {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*ms).brotli = ::brotli_sys::BrotliDecoderCreateInstance(None, None, std::ptr::null_mut());
        if ((*ms).brotli).is_null() {
            return brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
        }
        (*ms).decompression_stage =
            (brunsli_internal_dec_MetadataDecompressionStage::DECOMPRESSING).clone();
    }
    if (((*ms).decompression_stage)
        == (brunsli_internal_dec_MetadataDecompressionStage::DECOMPRESSING))
    {
        'loop_: while true {
            let mut available_bytes: u64 = {
                let mut __tmp_0 = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    GetBytesAvailable_166(_state)
                });
                let mut __tmp_1 = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    RemainingSectionLength_174(_state)
                });
                (*if *&mut __tmp_0 <= *&mut __tmp_1 {
                    (&mut __tmp_0) as *const _
                } else {
                    (&mut __tmp_1) as *const _
                })
            };
            let mut available_in: u64 = available_bytes;
            let mut next_in: *const u8 = (*state).data.offset(((*state).pos) as isize);
            let mut available_out: u64 = 0_u64;
            let mut result: ::brotli_sys::BrotliDecoderResult =
                ::brotli_sys::BrotliDecoderDecompressStream(
                    (*ms).brotli,
                    (&mut available_in as *mut u64) as *mut usize,
                    (&mut next_in as *mut *const u8),
                    (&mut available_out as *mut u64) as *mut usize,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_ERROR as i32)) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        if !(!(((*ms).brotli).is_null())) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 1312;
                                let _fn: *const u8 = b"operator()\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                        (*ms).brotli = std::ptr::null_mut();
                        (*ms).decompression_stage =
                            (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                        return result;
                    })(_result)
                });
            }
            let mut chunk_size: u64 = 0_u64;
            let mut chunk_data: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(
                (*ms).brotli,
                (&mut chunk_size as *mut u64) as *mut usize,
            );
            (*ms).decompressed_size = ((*ms).decompressed_size).wrapping_add(chunk_size);
            if (((*ms).decompressed_size) > ((*ms).metadata_size)) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        if !(!(((*ms).brotli).is_null())) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 1312;
                                let _fn: *const u8 = b"operator()\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                        (*ms).brotli = std::ptr::null_mut();
                        (*ms).decompression_stage =
                            (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                        return result;
                    })(_result)
                });
            }
            let mut consumed_bytes: u64 = (available_bytes).wrapping_sub(available_in);
            (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                let _len: u64 = consumed_bytes;
                SkipBytes_165(_state, _len)
            });
            let mut chunk_ok: bool = (unsafe {
                let _data: *const u8 = chunk_data;
                let _len: u64 = chunk_size;
                let _state: *mut brunsli_internal_dec_MetadataState = (ms);
                let _jpg: *mut brunsli_JPEGData = jpg;
                ProcessMetaData_149(_data, _len, _state, _jpg)
            });
            if !chunk_ok {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        if !(!(((*ms).brotli).is_null())) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 1312;
                                let _fn: *const u8 = b"operator()\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                        (*ms).brotli = std::ptr::null_mut();
                        (*ms).decompression_stage =
                            (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                        return result;
                    })(_result)
                });
            }
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32)) {
                if ((unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    RemainingSectionLength_174(_state)
                }) != (0_u64))
                {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            if !(!(((*ms).brotli).is_null())) {
                                (unsafe {
                                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                    let _l: i32 = 1312;
                                    let _fn: *const u8 = b"operator()\0".as_ptr();
                                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                                });
                                'loop_: while true {}
                            };
                            ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                            (*ms).brotli = std::ptr::null_mut();
                            (*ms).decompression_stage =
                                (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                            return result;
                        })(_result)
                    });
                }
                if (((*ms).decompressed_size) != ((*ms).metadata_size)) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            if !(!(((*ms).brotli).is_null())) {
                                (unsafe {
                                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                    let _l: i32 = 1312;
                                    let _fn: *const u8 = b"operator()\0".as_ptr();
                                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                                });
                                'loop_: while true {}
                            };
                            ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                            (*ms).brotli = std::ptr::null_mut();
                            (*ms).decompression_stage =
                                (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                            return result;
                        })(_result)
                    });
                }
                if !(unsafe { (*ms).CanFinish() }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            if !(!(((*ms).brotli).is_null())) {
                                (unsafe {
                                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                    let _l: i32 = 1312;
                                    let _fn: *const u8 = b"operator()\0".as_ptr();
                                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                                });
                                'loop_: while true {}
                            };
                            ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                            (*ms).brotli = std::ptr::null_mut();
                            (*ms).decompression_stage =
                                (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                            return result;
                        })(_result)
                    });
                }
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
                    (|result: brunsli_BrunsliStatus| {
                        if !(!(((*ms).brotli).is_null())) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 1312;
                                let _fn: *const u8 = b"operator()\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                        (*ms).brotli = std::ptr::null_mut();
                        (*ms).decompression_stage =
                            (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                        return result;
                    })(_result)
                });
            }
            if ((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as i32)) {
                continue 'loop_;
            }
            if !((result as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as i32)) {
                (unsafe {
                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                    let _l: i32 = 1352;
                    let _fn: *const u8 = b"DecodeMetaDataSection\0".as_ptr();
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            if ((unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                RemainingSectionLength_174(_state)
            }) == (0_u64))
            {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        if !(!(((*ms).brotli).is_null())) {
                            (unsafe {
                                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                                let _l: i32 = 1312;
                                let _fn: *const u8 = b"operator()\0".as_ptr();
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        ::brotli_sys::BrotliDecoderDestroyInstance((*ms).brotli);
                        (*ms).brotli = std::ptr::null_mut();
                        (*ms).decompression_stage =
                            (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                        return result;
                    })(_result)
                });
            }
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
    }
    if !(false) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1361;
            let _fn: *const u8 = b"DecodeMetaDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
}
pub unsafe fn CheckBoundary_181(
    mut state: *mut brunsli_internal_dec_State,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    if ((result as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        let mut last: bool = ((unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        }) <= (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            GetBytesAvailable_166(_state)
        }));
        return if last {
            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
        } else {
            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA
        };
    } else {
        return result;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn PrepareBitReader_182(
    mut br: *mut brunsli_BrunsliBitReader,
    mut state: *mut brunsli_internal_dec_State,
) {
    let mut chunk_len: u64 = {
        let mut __tmp_0 = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            GetBytesAvailable_166(_state)
        });
        let mut __tmp_1 = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        });
        (*if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    };
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _buffer: *const u8 = (*state).data.offset(((*state).pos) as isize);
        let _length: u64 = chunk_len;
        BrunsliBitReaderResume_128(_br, _buffer, _length)
    });
    if !(unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1384;
            let _fn: *const u8 = b"PrepareBitReader\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
}
pub unsafe fn SuspendBitReader_183(
    mut br: *mut brunsli_BrunsliBitReader,
    mut state: *mut brunsli_internal_dec_State,
    mut result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    let mut chunk_len: u64 = {
        let mut __tmp_0 = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            GetBytesAvailable_166(_state)
        });
        let mut __tmp_1 = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        });
        (*if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    };
    let mut unused_bytes: u64 = (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderSuspend_130(_br)
    });
    let mut consumed_bytes: u64 = (chunk_len).wrapping_sub(unused_bytes);
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _len: u64 = consumed_bytes;
        SkipBytes_165(_state, _len)
    });
    result = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _result: brunsli_BrunsliStatus = result;
        CheckBoundary_181(_state, _result)
    })
    .clone();
    if !((unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    }) || (((result as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32))
        && ((result as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))))
    {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1401;
            let _fn: *const u8 = b"SuspendBitReader\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return result;
}
pub unsafe fn DecodeJPEGInternalsSection_184(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let js: *mut brunsli_internal_dec_JpegInternalsState =
        &mut (*s).internals as *mut brunsli_internal_dec_JpegInternalsState;
    let mut br: *mut brunsli_BrunsliBitReader = (&mut (*js).br as *mut brunsli_BrunsliBitReader);
    if (((*js).stage as i32) == (brunsli_internal_dec_JpegInternalsState_Stage::INIT as i32)) {
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderInit_127(_br)
        });
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_MARKERS).clone();
    }
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _state: *mut brunsli_internal_dec_State = state;
        PrepareBitReader_182(_br, _state)
    });
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_MARKERS as i32))
    {
        'loop_: while true {
            if !(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u64 = 6_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            let mut marker: u8 = (((192_u32).wrapping_add(
                (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 6_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }),
            )) as u8);
            {
                let a0_clone = marker.clone();
                (*jpg).marker_order.push(a0_clone)
            };
            if ((marker as i32) == (196)) {
                (*js).dht_count.prefix_inc();
            }
            if ((marker as i32) == (221)) {
                (*js).have_dri = true;
            }
            if ((marker as i32) == (218)) {
                (*js).num_scans.prefix_inc();
            }
            if ((marker as i32) == (217)) {
                break;
            }
        }
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_DRI).clone();
    }
    if (((*js).stage as i32) == (brunsli_internal_dec_JpegInternalsState_Stage::READ_DRI as i32)) {
        if (*js).have_dri {
            if !(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u64 = 16_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            (*jpg).restart_interval = ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 16_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as i32);
        }
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST).clone();
    }
    if ((((*js).stage as i32)
        & (brunsli_internal_dec_JpegInternalsState_Stage::DECODE_HUFFMAN_MASK as i32))
        != 0)
    {
        let mut status: brunsli_BrunsliStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _jpg: *mut brunsli_JPEGData = jpg;
            DecodeHuffmanCode_150(_state, _jpg)
        });
        if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = status;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::PREPARE_READ_SCANS).clone();
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::PREPARE_READ_SCANS as i32))
    {
        if (((*js).dht_count) != ((*js).terminal_huffman_code_count)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid number of DHT markers\n",
            );
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        if (((*js).num_scans) > (0_u64)) {
            {
                let __a0 = (*js).num_scans as usize;
                (*jpg)
                    .scan_info
                    .resize_with(__a0, || <brunsli_JPEGScanInfo>::default())
            };
            (*js).i = 0_u64;
            (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON).clone();
        } else {
            (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT).clone();
        }
    }
    if ((((*js).stage as i32)
        & (brunsli_internal_dec_JpegInternalsState_Stage::DECODE_SCAN_MASK as i32))
        != 0)
    {
        let mut status: brunsli_BrunsliStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _jpg: *mut brunsli_JPEGData = jpg;
            DecodeScanInfo_151(_state, _jpg)
        });
        if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = status;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT).clone();
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT as i32))
    {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        let mut num_quant_tables: i32 = (((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        })
        .wrapping_add(1_u32)) as i32);
        {
            let __a0 = (num_quant_tables as u64) as usize;
            (*jpg)
                .quant
                .resize_with(__a0, || <brunsli_JPEGQuantTable>::default())
        };
        (*js).i = 0_u64;
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_QUANT).clone();
    }
    'loop_: while (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_QUANT as i32))
    {
        if (((*js).i) >= ((*jpg).quant.len() as u64)) {
            (*js).stage =
                (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID_SCHEME).clone();
            break;
        }
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 7_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        let mut q: *mut brunsli_JPEGQuantTable =
            (&mut (&mut (*jpg)).quant[((*js).i) as usize] as *mut brunsli_JPEGQuantTable);
        (*q).index = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32);
        (*q).is_last = (((*js).i) == (((*jpg).quant.len() as u64).wrapping_sub(1_u64)))
            || ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 1_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) != 0);
        (*q).precision = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 4_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32);
        if (((*q).precision) > (1)) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Invalid quantization table precision: {:}\n",
                (*q).precision,
            );
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).i.prefix_inc();
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID_SCHEME as i32))
    {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        let mut comp_ids: i32 = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32);
        static mut kMinRequiredComponents_185: [u64; 4] = unsafe { [3_u64, 1_u64, 3_u64, 0_u64] };;
        if (((*jpg).components.len() as u64) < (kMinRequiredComponents_185[(comp_ids) as usize])) {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Insufficient number of components for ColorId #{:}\n",
                comp_ids,
            );
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).stage =
            (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS).clone();
        if ((comp_ids) == (kComponentIds123_49)) {
            (&mut (*jpg)).components[(0_u64) as usize].id = 1;
            (&mut (*jpg)).components[(1_u64) as usize].id = 2;
            (&mut (*jpg)).components[(2_u64) as usize].id = 3;
        } else if ((comp_ids) == (kComponentIdsGray_50)) {
            (&mut (*jpg)).components[(0_u64) as usize].id = 1;
        } else if ((comp_ids) == (kComponentIdsRGB_51)) {
            (&mut (*jpg)).components[(0_u64) as usize].id = (('R' as u8) as i32);
            (&mut (*jpg)).components[(1_u64) as usize].id = (('G' as u8) as i32);
            (&mut (*jpg)).components[(2_u64) as usize].id = (('B' as u8) as i32);
        } else {
            if !((comp_ids) == (kComponentIdsCustom_52)) {
                (unsafe {
                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                    let _l: i32 = 1529;
                    let _fn: *const u8 = b"DecodeJPEGInternalsSection\0".as_ptr();
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            (*js).i = 0_u64;
            (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID).clone();
        }
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID as i32))
    {
        'loop_: while (((*js).i) < ((*jpg).components.len() as u64)) {
            if !(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u64 = 8_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            (&mut (*jpg)).components[((*js).i) as usize].id = ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 8_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as i32);
            (*js).i.prefix_inc();
        }
        (*js).stage =
            (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS).clone();
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS as i32))
    {
        if !(unsafe {
            let _s: *mut brunsli_internal_dec_VarintState =
                (&mut (*js).varint as *mut brunsli_internal_dec_VarintState);
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _max_symbols: u64 = 4_u64;
            DecodeLimitedVarint_146(_s, _br, _max_symbols)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).num_padding_bits = (*js).varint.value;
        (*jpg).has_zero_padding_bit = (((*js).num_padding_bits) > (0_u64));
        if (((*js).num_padding_bits)
            > (unsafe {
                let _jpg: *const brunsli_JPEGData = &(*jpg) as *const brunsli_JPEGData;
                PaddingBitsLimit_17(_jpg)
            }))
        {
            write!(
                std::fs::File::from_raw_fd(
                    std::io::stderr()
                        .as_fd()
                        .try_clone_to_owned()
                        .unwrap()
                        .into_raw_fd(),
                ),
                "Suspicious number of padding bits {:}\n",
                (*js).num_padding_bits,
            );
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*js).i = 0_u64;
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::READ_PADDING_BITS).clone();
    }
    if (((*js).stage as i32)
        == (brunsli_internal_dec_JpegInternalsState_Stage::READ_PADDING_BITS as i32))
    {
        'loop_: while (((*js).i) < ((*js).num_padding_bits)) {
            if !(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u64 = 1_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            (*jpg).padding_bits.push(
                (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) as i32,
            );
            (*js).i.prefix_inc();
        }
        (unsafe {
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (|result: brunsli_BrunsliStatus| {
                return (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _result: brunsli_BrunsliStatus = result;
                    SuspendBitReader_183(_br, _state, _result)
                });
            })(_result)
        });
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderFinish_131(_br)
        });
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*js).i = 0_u64;
        (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS).clone();
    } else {
        (unsafe {
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (|result: brunsli_BrunsliStatus| {
                return (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _result: brunsli_BrunsliStatus = result;
                    SuspendBitReader_183(_br, _state, _result)
                });
            })(_result)
        });
    }
    'loop_: while true {
        switch!(match ((*js).stage as i32) {
            v if v == (brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS as i32) => {
                if (((*js).i) >= ((*jpg).marker_order.len() as u64)) {
                    (*js).stage = (brunsli_internal_dec_JpegInternalsState_Stage::DONE).clone();
                } else if (((&mut (*jpg)).marker_order[((*js).i) as usize] as i32) == (255)) {
                    (*js).stage =
                        (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_LENGTH)
                            .clone();
                } else {
                    (*js).i.prefix_inc();
                };
                continue 'loop_;
            }
            v if v
                == (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_LENGTH
                    as i32) =>
            {
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _val: *mut u64 = (&mut (*js).intermarker_length as *mut u64);
                    DecodeBase128_168(_state, _val)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        CheckBoundary_181(_state, _result)
                    });
                }
                if (((*js).intermarker_length)
                    > (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        RemainingSectionLength_174(_state)
                    }))
                {
                    return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                }
                (*jpg).inter_marker_data.push(Vec::new());
                (*js).stage =
                    (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_DATA).clone();
                continue 'loop_;
            }
            v if v
                == (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_DATA
                    as i32) =>
            {
                let dest: *mut Vec<u8> = (((*jpg).inter_marker_data).last_mut().unwrap());
                let mut piece_limit: u64 =
                    ((*js).intermarker_length).wrapping_sub((*dest).len() as u64);
                let mut piece_size: u64 = {
                    let mut __tmp_1 = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        GetBytesAvailable_166(_state)
                    });
                    (*if *&mut piece_limit <= *&mut __tmp_1 {
                        (&mut piece_limit) as *const _
                    } else {
                        (&mut __tmp_1) as *const _
                    })
                };
                (unsafe {
                    let _dst: *mut Vec<u8> = (dest);
                    let _begin: *const u8 = (*state).data.offset(((*state).pos) as isize);
                    let _length: u64 = piece_size;
                    Append_72(_dst, _begin, _length)
                });
                (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _len: u64 = piece_size;
                    SkipBytes_165(_state, _len)
                });
                if (((*dest).len() as u64) < ((*js).intermarker_length)) {
                    if !((unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        GetBytesAvailable_166(_state)
                    }) == (0_u64))
                    {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 1613;
                            let _fn: *const u8 = b"DecodeJPEGInternalsSection\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    if !((unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        RemainingSectionLength_174(_state)
                    }) > (0_u64))
                    {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 1614;
                            let _fn: *const u8 = b"DecodeJPEGInternalsSection\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                }
                (*js).i.prefix_inc();
                (*js).stage =
                    (brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS).clone();
                continue 'loop_;
            }
            _ => {}
        });
        break;
    }
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        IsAtSectionBoundary_175(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn DecodeQuantDataSection_186(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let qs: *mut brunsli_internal_dec_QuantDataState =
        &mut (*s).quant as *mut brunsli_internal_dec_QuantDataState;
    let mut br: *mut brunsli_BrunsliBitReader = (&mut (*qs).br as *mut brunsli_BrunsliBitReader);
    if (((*qs).stage as i32) == (brunsli_internal_dec_QuantDataState_Stage::INIT as i32)) {
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderInit_127(_br)
        });
        (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::READ_NUM_QUANT).clone();
    }
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _state: *mut brunsli_internal_dec_State = state;
        PrepareBitReader_182(_br, _state)
    });
    if (((*qs).stage as i32) == (brunsli_internal_dec_QuantDataState_Stage::READ_NUM_QUANT as i32))
    {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        let mut num_quant_tables: u64 = (((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        })
        .wrapping_add(1_u32)) as u64);
        if (((*jpg).quant.len() as u64) != (num_quant_tables)) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        {
            let __a0 = (kDCTBlockSize_3 as u64) as usize;
            (*qs).predictor.resize_with(__a0, || <u8>::default())
        };
        (*qs).i = 0_u64;
        (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::READ_STOCK).clone();
    }
    'loop_: while true {
        switch!(match ((*qs).stage as i32) {
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_STOCK as i32) => {
                if (((*qs).i) >= ((*jpg).quant.len() as u64)) {
                    std::mem::swap(&mut Vec::new(), &mut (*qs).predictor);
                    (*qs).i = 0_u64;
                    (*qs).stage =
                        (brunsli_internal_dec_QuantDataState_Stage::READ_QUANT_IDX).clone();
                    continue 'loop_;
                }
                if !(unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u64 = 4_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                (*qs).data_precision = 0_u8;
                let mut is_short: bool = !((unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) != 0);
                if is_short {
                    let short_code: u64 = ((unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _n_bits: u32 = 3_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    let mut table: *mut i32 =
                        (&mut (*jpg)).quant[((*qs).i) as usize].values.as_mut_ptr();
                    let mut selector: u64 = (if (((*qs).i) > (0_u64)) { 1 } else { 0 } as u64);
                    let mut k: u64 = 0_u64;
                    'loop_: while ((k) < (kDCTBlockSize_3 as u64)) {
                        (*table.offset((k) as isize)) = (kStockQuantizationTables_48
                            [(selector) as usize][(short_code) as usize][(k) as usize]
                            as i32);
                        k.prefix_inc();
                    }
                    (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::UPDATE).clone();
                } else {
                    (*qs).stage =
                        (brunsli_internal_dec_QuantDataState_Stage::READ_Q_FACTOR).clone();
                };
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_Q_FACTOR as i32) => {
                if !(unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u64 = 6_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                let q_factor: u32 = (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 6_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                });
                (unsafe {
                    let _is_chroma: bool = (((*qs).i) > (0_u64));
                    let _q: u32 = q_factor;
                    let _dst: *mut u8 = (*qs).predictor.as_mut_ptr();
                    FillQuantMatrix_118(_is_chroma, _q, _dst)
                });
                (*qs).j = 0_u64;
                (*qs).delta = 0;
                (*qs).stage =
                    (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO).clone();
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO as i32) => {
                if (((*qs).j) >= (kDCTBlockSize_3 as u64)) {
                    (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::UPDATE).clone();
                    continue 'loop_;
                }
                if !(unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u64 = 1_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                if ((unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) != 0)
                {
                    (*qs).stage =
                        (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_SIGN).clone();
                } else {
                    (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF).clone();
                };
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_SIGN as i32) => {
                if !(unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u64 = 1_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                (*qs).sign = if ((unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) != 0)
                {
                    -1_i32
                } else {
                    1
                };
                (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF).clone();
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF as i32) => {
                if !(unsafe {
                    let _s: *mut brunsli_internal_dec_VarintState =
                        (&mut (*qs).vs as *mut brunsli_internal_dec_VarintState);
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _max_bits: u64 = 16_u64;
                    DecodeVarint_144(_s, _br, _max_bits)
                }) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                let mut diff: i32 = (((*qs).vs.value as i32) + (1));
                (*qs).delta += (((*qs).sign) * (diff));
                (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF).clone();
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF as i32) => {
                let k: i32 = (kJPEGNaturalOrder_13[((*qs).j) as usize] as i32);
                let quant_value: i32 =
                    (((&mut (*qs)).predictor[(k as u64) as usize] as i32) + ((*qs).delta));
                (&mut (*jpg)).quant[((*qs).i) as usize].values[(k as u64) as usize] = quant_value;
                if ((quant_value) <= (0)) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                if ((quant_value) >= (256)) {
                    (*qs).data_precision = 1_u8;
                }
                if ((quant_value) >= (65536)) {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                (*qs).j.prefix_inc();
                (*qs).stage =
                    (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO).clone();
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::UPDATE as i32) => {
                if (((&mut (*jpg)).quant[((*qs).i) as usize].precision)
                    < ((*qs).data_precision as i32))
                {
                    return (unsafe {
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (|result: brunsli_BrunsliStatus| {
                            return (unsafe {
                                let _br: *mut brunsli_BrunsliBitReader = br;
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _result: brunsli_BrunsliStatus = result;
                                SuspendBitReader_183(_br, _state, _result)
                            });
                        })(_result)
                    });
                }
                (*qs).i.prefix_inc();
                (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::READ_STOCK).clone();
                continue 'loop_;
            }
            _ => {}
        });
        break;
    }
    'loop_: while (((*qs).stage as i32)
        == (brunsli_internal_dec_QuantDataState_Stage::READ_QUANT_IDX as i32))
    {
        if (((*qs).i) >= ((*jpg).components.len() as u64)) {
            (*qs).stage = (brunsli_internal_dec_QuantDataState_Stage::FINISH).clone();
            continue 'loop_;
        }
        let mut c: *mut brunsli_JPEGComponent =
            (&mut (&mut (*jpg)).components[((*qs).i) as usize] as *mut brunsli_JPEGComponent);
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*c).quant_idx = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as u8);
        if (((*c).quant_idx as u64) >= ((*jpg).quant.len() as u64)) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*qs).i.prefix_inc();
    }
    if !(((*qs).stage as i32) == (brunsli_internal_dec_QuantDataState_Stage::FINISH as i32)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1787;
            let _fn: *const u8 = b"DecodeQuantDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (unsafe {
        let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
        (|result: brunsli_BrunsliStatus| {
            return (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _state: *mut brunsli_internal_dec_State = state;
                let _result: brunsli_BrunsliStatus = result;
                SuspendBitReader_183(_br, _state, _result)
            });
        })(_result)
    });
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderFinish_131(_br)
    });
    if !(unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        IsAtSectionBoundary_175(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn DecodeHistogramDataSection_187(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let hs: *mut brunsli_internal_dec_HistogramDataState =
        &mut (*s).histogram as *mut brunsli_internal_dec_HistogramDataState;
    let mut br: *mut brunsli_BrunsliBitReader = (&mut (*hs).br as *mut brunsli_BrunsliBitReader);
    if (((*hs).stage as i32) == (brunsli_internal_dec_HistogramDataState_Stage::INIT as i32)) {
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderInit_127(_br)
        });
        if !(!(*jpg).components.is_empty()) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 1802;
                let _fn: *const u8 = b"DecodeHistogramDataSection\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        (*s).num_contexts = ((*jpg).components.len() as u64).clone();
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::READ_SCHEME).clone();
        (unsafe {
            let _limit: u64 = 648_u64;
            (*hs).arena.reserve(_limit)
        });
    }
    (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _state: *mut brunsli_internal_dec_State = state;
        PrepareBitReader_182(_br, _state)
    });
    if ((unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        RemainingSectionLength_174(_state)
    }) <= (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        GetBytesAvailable_166(_state)
    })) {
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderSetOptimistic_133(_br)
        });
    };
    if (((*hs).stage as i32) == (brunsli_internal_dec_HistogramDataState_Stage::READ_SCHEME as i32))
    {
        let num_components: u64 = (*jpg).components.len() as u64;
        if !((num_components) <= (4_u64)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 1822;
                let _fn: *const u8 = b"DecodeHistogramDataSection\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = (3_u64).wrapping_mul(num_components);
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (num_components)) {
            let mut scheme: u64 = ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 3_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u64);
            if ((scheme) >= (kNumSchemes_91 as u64)) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            let m: *mut brunsli_internal_dec_ComponentMeta =
                &mut (&mut (*state)).meta[(i) as usize] as *mut brunsli_internal_dec_ComponentMeta;
            (*m).context_bits = scheme;
            (*m).context_offset = (*s).num_contexts;
            (*s).num_contexts = ((*s).num_contexts)
                .wrapping_add((kNumNonzeroContextSkip_94[(scheme) as usize] as u64));
            i.prefix_inc();
        }
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::READ_NUM_HISTOGRAMS).clone();
    }
    if (((*hs).stage as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage::READ_NUM_HISTOGRAMS as i32))
    {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = 11_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*s).num_histograms = (((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            DecodeVarLenUint8_143(_br)
        })
        .wrapping_add(1_u32)) as u64);
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        if (*s).shallow_histograms {
            (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::SKIP_CONTENT).clone();
        } else {
            {
                let __a0 = ((*s).num_contexts).wrapping_mul(kNumAvrgContexts_83) as usize;
                (*s).context_map_.resize_with(__a0, || <u8>::default())
            };
            (*state).context_map = ((*s).context_map_.as_mut_ptr()).cast_const();
            {
                let __a0 = (*s).num_histograms as usize;
                (*s).entropy_codes_
                    .resize_with(__a0, || <brunsli_ANSDecodingData>::default())
            };
            (*state).entropy_codes = ((*s).entropy_codes_.as_mut_ptr()).cast_const();
            if (((*s).num_histograms) > (1_u64)) {
                (*hs).stage =
                    (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP_CODE).clone();
            } else {
                (*hs).i = 0_u64;
                {
                    let __a0 = (kCoeffAlphabetSize_136 as u64) as usize;
                    (*hs).counts.resize_with(__a0, || <u32>::default())
                };
                (*hs).stage =
                    (brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS).clone();
            }
        }
    }
    if (((*hs).stage as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage::SKIP_CONTENT as i32))
    {
        (unsafe {
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (|result: brunsli_BrunsliStatus| {
                return (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _result: brunsli_BrunsliStatus = result;
                    SuspendBitReader_183(_br, _state, _result)
                });
            })(_result)
        });
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _len: u64 = (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                RemainingSectionLength_174(_state)
            });
            SkipAvailableBytes_167(_state, _len)
        });
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::DONE).clone();
    }
    if (((*hs).stage as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP_CODE as i32))
    {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = (207_u64).wrapping_add(((*s).num_histograms).wrapping_mul(8_u64));
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*hs).max_run_length_prefix = 0_u64;
        let mut use_rle_for_zeros: bool = !!((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 1_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) != 0);
        if use_rle_for_zeros {
            (*hs).max_run_length_prefix = (((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 4_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(1_u32)) as u64);
        }
        let mut alphabet_size: u64 =
            ((*s).num_histograms).wrapping_add((*hs).max_run_length_prefix);
        {
            let _a0: *mut brunsli_HuffmanDecodingData =
                (Box::leak(Box::new(<brunsli_HuffmanDecodingData>::default()))
                    as *mut brunsli_HuffmanDecodingData);
            (*hs).entropy = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        if !(unsafe {
            let _alphabet_size: u64 = alphabet_size;
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _arena: *mut brunsli_Arena = (&mut (*hs).arena as *mut brunsli_Arena);
            (*(*hs).entropy.as_deref_mut().unwrap()).ReadFromBitStream(
                _alphabet_size,
                _br,
                Some(_arena),
            )
        }) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*hs).i = 0_u64;
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP).clone();
    }
    if (((*hs).stage as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP as i32))
    {
        let mut status: brunsli_BrunsliStatus = (unsafe {
            let _entropy: *const brunsli_HuffmanDecodingData =
                &(*(*hs).entropy.as_deref_mut().unwrap()) as *const brunsli_HuffmanDecodingData;
            let _max_run_length_prefix: u64 = (*hs).max_run_length_prefix;
            let _index: *mut u64 = (&mut (*hs).i as *mut u64);
            let _context_map: *mut Vec<u8> = (&mut (*s).context_map_ as *mut Vec<u8>);
            let _br: *mut brunsli_BrunsliBitReader = br;
            DecodeContextMap_188(_entropy, _max_run_length_prefix, _index, _context_map, _br)
        });
        if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return (unsafe {
                let _result: brunsli_BrunsliStatus = status;
                (|result: brunsli_BrunsliStatus| {
                    return (unsafe {
                        let _br: *mut brunsli_BrunsliBitReader = br;
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = result;
                        SuspendBitReader_183(_br, _state, _result)
                    });
                })(_result)
            });
        }
        (*hs).i = 0_u64;
        {
            let __a0 = (kCoeffAlphabetSize_136 as u64) as usize;
            (*hs).counts.resize_with(__a0, || <u32>::default())
        };
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS).clone();
    }
    if (((*hs).stage as i32)
        == (brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS as i32))
    {
        'loop_: while (((*hs).i) < ((*s).num_histograms)) {
            if !(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u64 = (((9) + ((kCoeffAlphabetSize_136) * (11))) as u64);
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            if !(unsafe {
                let _precision_bits: u32 = (BRUNSLI_ANS_LOG_TAB_SIZE_0 as u32);
                let _counts: *mut Vec<u32> = (&mut (*hs).counts as *mut Vec<u32>);
                let _br: *mut brunsli_BrunsliBitReader = br;
                ReadHistogram_189(_precision_bits, _counts, _br)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            if !(unsafe {
                let _counts: *const Vec<u32> = &(*hs).counts as *const Vec<u32>;
                (&mut (*s)).entropy_codes_[((*hs).i) as usize].Init(_counts)
            }) {
                return (unsafe {
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (|result: brunsli_BrunsliStatus| {
                        return (unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = result;
                            SuspendBitReader_183(_br, _state, _result)
                        });
                    })(_result)
                });
            }
            (*hs).i.prefix_inc();
        }
        {
            let _a0: *mut brunsli_HuffmanDecodingData = Default::default();
            (*hs).entropy = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        std::mem::swap(&mut Vec::new(), &mut (*hs).counts);
        (unsafe {
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (|result: brunsli_BrunsliStatus| {
                return (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _result: brunsli_BrunsliStatus = result;
                    SuspendBitReader_183(_br, _state, _result)
                });
            })(_result)
        });
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderFinish_131(_br)
        });
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*hs).stage = (brunsli_internal_dec_HistogramDataState_Stage::DONE).clone();
    }
    (unsafe { (*hs).arena.reset() });
    if !(((*hs).stage as i32) == (brunsli_internal_dec_HistogramDataState_Stage::DONE as i32)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1925;
            let _fn: *const u8 = b"DecodeHistogramDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub unsafe fn DecodeDCDataSection_190(
    mut state: *mut brunsli_internal_dec_State,
) -> brunsli_BrunsliStatus {
    let mut available: u64 = ((unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        GetBytesAvailable_166(_state)
    }) & (!1 as u64));
    let mut limit: u64 = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        RemainingSectionLength_174(_state)
    });
    if !(((limit) & (1_u64)) == (0_u64)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1932;
            let _fn: *const u8 = b"DecodeDCDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let mut chunk_len: u64 = (*if *&mut available <= *&mut limit {
        (&mut available) as *const _
    } else {
        (&mut limit) as *const _
    });
    let mut is_last_chunk: bool = ((chunk_len) == (limit));
    let mut in_: brunsli_WordSource = brunsli_WordSource::brunsli_WordSource(
        (*state).data.offset(((*state).pos) as isize),
        chunk_len,
        is_last_chunk,
    );
    let mut status: brunsli_BrunsliStatus = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _in: *mut brunsli_WordSource = (&mut in_ as *mut brunsli_WordSource);
        DecodeDC_157(_state, _in)
    });
    if !(((in_.pos_) & (1_u64)) == (0_u64)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1941;
            let _fn: *const u8 = b"DecodeDCDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if in_.error_ {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !((in_.pos_) <= (chunk_len)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1943;
            let _fn: *const u8 = b"DecodeDCDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _len: u64 = in_.pos_;
        SkipBytes_165(_state, _len)
    });
    if is_last_chunk {
        if !((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 1946;
                let _fn: *const u8 = b"DecodeDCDataSection\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
    }
    return status;
}
pub unsafe fn DecodeACDataSection_191(
    mut state: *mut brunsli_internal_dec_State,
) -> brunsli_BrunsliStatus {
    let mut available: u64 = ((unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        GetBytesAvailable_166(_state)
    }) & (!1 as u64));
    let mut limit: u64 = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        RemainingSectionLength_174(_state)
    });
    if !(((limit) & (1_u64)) == (0_u64)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1955;
            let _fn: *const u8 = b"DecodeACDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let mut chunk_len: u64 = (*if *&mut available <= *&mut limit {
        (&mut available) as *const _
    } else {
        (&mut limit) as *const _
    });
    let mut is_last_chunk: bool = ((chunk_len) == (limit));
    let mut in_: brunsli_WordSource = brunsli_WordSource::brunsli_WordSource(
        (*state).data.offset(((*state).pos) as isize),
        chunk_len,
        is_last_chunk,
    );
    let mut status: brunsli_BrunsliStatus = (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _in: *mut brunsli_WordSource = (&mut in_ as *mut brunsli_WordSource);
        DecodeAC_160(_state, _in)
    });
    if !(((in_.pos_) & (1_u64)) == (0_u64)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1964;
            let _fn: *const u8 = b"DecodeACDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if in_.error_ {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !((in_.pos_) <= (chunk_len)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 1966;
            let _fn: *const u8 = b"DecodeACDataSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        let _len: u64 = in_.pos_;
        SkipBytes_165(_state, _len)
    });
    if is_last_chunk {
        if !((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 1969;
                let _fn: *const u8 = b"DecodeACDataSection\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
    }
    return status;
}
pub unsafe fn DecodeOriginalJpg_192(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let fs: *mut brunsli_internal_dec_FallbackState =
        &mut (*s).fallback as *mut brunsli_internal_dec_FallbackState;
    'loop_: while (((*fs).stage) != (brunsli_internal_dec_FallbackState_Stage::DONE as u64)) {
        'switch: {
            let __match_cond = (*fs).stage;
            match __match_cond {
                v if v == (brunsli_internal_dec_FallbackState_Stage::READ_TAG as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        ReadTag_170(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    if (((*s).section.tag) != (kBrunsliOriginalJpgTag_38 as u64))
                        || (!(*s).section.is_section)
                    {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*fs).stage =
                        (brunsli_internal_dec_FallbackState_Stage::ENTER_SECTION as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_FallbackState_Stage::ENTER_SECTION as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        EnterSection_171(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    (*jpg).original_jpg_size = (*s).section.remaining;
                    if (((*jpg).original_jpg_size) == (0_u64)) {
                        (*jpg).original_jpg = std::ptr::null();
                        (*fs).stage =
                            (brunsli_internal_dec_FallbackState_Stage::DONE as u64).clone();
                        break 'switch;
                    }
                    (*fs).stage =
                        (brunsli_internal_dec_FallbackState_Stage::READ_CONTENTS as u64).clone();
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_FallbackState_Stage::READ_CONTENTS as u64) => {
                    let mut chunk_size: u64 = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        GetBytesAvailable_166(_state)
                    });
                    if ((chunk_size) == (0_u64)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            Fail_169(_state, _result)
                        });
                    }
                    let mut src: *const u8 = (*state).data.offset(((*state).pos) as isize);
                    if (*fs).storage.is_empty() {
                        if ((chunk_size) >= ((*jpg).original_jpg_size)) {
                            (*jpg).original_jpg = src;
                            (unsafe {
                                let _state: *mut brunsli_internal_dec_State = state;
                                let _len: u64 = (*jpg).original_jpg_size;
                                SkipBytes_165(_state, _len)
                            });
                            (*fs).stage =
                                (brunsli_internal_dec_FallbackState_Stage::DONE as u64).clone();
                            break 'switch;
                        }
                    }
                    let mut remaining: u64 =
                        ((*jpg).original_jpg_size).wrapping_sub((*fs).storage.len() as u64);
                    let mut to_copy: u64 = (*if *&mut chunk_size <= *&mut remaining {
                        (&mut chunk_size) as *const _
                    } else {
                        (&mut remaining) as *const _
                    });
                    {
                        let __off = (*fs)
                            .storage
                            .as_ptr()
                            .add((*fs).storage.len())
                            .offset_from((*fs).storage.as_ptr())
                            as usize;
                        let count = src.offset((to_copy) as isize).offset_from(src) as usize;
                        (*fs).storage.splice(
                            __off..__off,
                            std::slice::from_raw_parts(src, count).iter().cloned(),
                        );
                        (*fs).storage.as_mut_ptr().add(__off)
                    };
                    (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _len: u64 = to_copy;
                        SkipBytes_165(_state, _len)
                    });
                    if (((*fs).storage.len() as u64) == ((*jpg).original_jpg_size)) {
                        (*jpg).original_jpg = ((*fs).storage.as_mut_ptr()).cast_const();
                        (*fs).stage =
                            (brunsli_internal_dec_FallbackState_Stage::DONE as u64).clone();
                        break 'switch;
                    }
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        Fail_169(_state, _result)
                    });
                }
                _ => {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    (unsafe {
        let _section: *mut brunsli_internal_dec_SectionState =
            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
        LeaveSection_172(_section)
    });
    return brunsli_internal_dec_Stage::DONE;
}
pub unsafe fn ParseSection_193(
    mut state: *mut brunsli_internal_dec_State,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let sh: *mut brunsli_internal_dec_SectionHeaderState =
        &mut (*s).section_header as *mut brunsli_internal_dec_SectionHeaderState;
    let mut result: brunsli_internal_dec_Stage = brunsli_internal_dec_Stage::ERROR;
    'loop_: while (((*sh).stage) != (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64)) {
        'switch: {
            let __match_cond = (*sh).stage;
            match __match_cond {
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::READ_TAG as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        ReadTag_170(_state, _section)
                    });
                    if ((status as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        if (unsafe {
                            let _state: *const brunsli_internal_dec_State = (state).cast_const();
                            let _tag: u32 = (kBrunsliACDataTag_37 as u32);
                            HasSection_194(_state, _tag)
                        }) {
                            return brunsli_internal_dec_Stage::DONE;
                        }
                    }
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    if (*s).section.is_section {
                        (*sh).stage = (brunsli_internal_dec_SectionHeaderState_Stage::ENTER_SECTION
                            as u64)
                            .clone();
                        continue 'loop_;
                    }
                    let tag_bit: u32 = ((1_u32) << ((*s).section.tag));
                    let is_known_section_tag: bool = (((kKnownSectionTags_137) & (tag_bit)) != 0);
                    if is_known_section_tag {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*sh).stage =
                        (brunsli_internal_dec_SectionHeaderState_Stage::READ_VALUE as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::READ_VALUE as u64) => {
                    let mut sink: u64 = 0_u64;
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _val: *mut u64 = (&mut sink as *mut u64);
                        DecodeBase128_168(_state, _val)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    result = (brunsli_internal_dec_Stage::SECTION).clone();
                    (*sh).stage =
                        (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::ENTER_SECTION as u64) => {
                    let mut status: brunsli_BrunsliStatus = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _section: *mut brunsli_internal_dec_SectionState =
                            (&mut (*s).section as *mut brunsli_internal_dec_SectionState);
                        EnterSection_171(_state, _section)
                    });
                    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus = status;
                            Fail_169(_state, _result)
                        });
                    }
                    result = (brunsli_internal_dec_Stage::SECTION_BODY).clone();
                    (*sh).stage =
                        (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64).clone();
                    continue 'loop_;
                }
                _ => {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    (*sh).stage = (brunsli_internal_dec_SectionHeaderState_Stage::READ_TAG as u64).clone();
    if !((result) != (brunsli_internal_dec_Stage::ERROR)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 2091;
            let _fn: *const u8 = b"ParseSection\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return result;
}
pub unsafe fn ProcessSection_195(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_internal_dec_Stage {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let tag_bit: i32 = (((1_u32) << ((*s).section.tag)) as i32);
    let is_known_section_tag: bool = (((kKnownSectionTags_137) & (tag_bit as u32)) != 0);
    let skip_section: bool =
        (!is_known_section_tag) || ((((*state).skip_tags) & (tag_bit as u32)) != 0);
    if skip_section {
        let mut to_skip: u64 = {
            let mut __tmp_0 = (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                GetBytesAvailable_166(_state)
            });
            let mut __tmp_1 = (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                RemainingSectionLength_174(_state)
            });
            (*if *&mut __tmp_0 <= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        };
        (*state).pos = ((*state).pos).wrapping_add(to_skip);
        if ((unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            RemainingSectionLength_174(_state)
        }) != (0_u64))
        {
            if !((unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                GetBytesAvailable_166(_state)
            }) == (0_u64))
            {
                (unsafe {
                    let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                    let _l: i32 = 2110;
                    let _fn: *const u8 = b"ProcessSection\0".as_ptr();
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            return (unsafe {
                let _state: *mut brunsli_internal_dec_State = state;
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                Fail_169(_state, _result)
            });
        }
        return brunsli_internal_dec_Stage::SECTION;
    }
    'switch: {
        let __match_cond = (*s).section.tag;
        match __match_cond {
            v if v == (kBrunsliMetaDataTag_32 as u64) => {
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    DecodeMetaDataSection_180(_state, _jpg)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == (kBrunsliJPEGInternalsTag_33 as u64) => {
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    DecodeJPEGInternalsSection_184(_state, _jpg)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == (kBrunsliQuantDataTag_34 as u64) => {
                if !(unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliJPEGInternalsTag_33 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    DecodeQuantDataSection_186(_state, _jpg)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == (kBrunsliHistogramDataTag_35 as u64) => {
                if !(unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliJPEGInternalsTag_33 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    DecodeHistogramDataSection_187(_state, _jpg)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == (kBrunsliDCDataTag_36 as u64) => {
                if !(unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliHistogramDataTag_35 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if !(unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliQuantDataTag_34 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if (((unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    RemainingSectionLength_174(_state)
                }) & (1_u64))
                    != (0_u64))
                {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                (unsafe {
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    let _state: *mut brunsli_internal_dec_State = state;
                    WarmupMeta_196(_jpg, _state)
                });
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    DecodeDCDataSection_190(_state)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == (kBrunsliACDataTag_37 as u64) => {
                if !(unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliDCDataTag_36 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if (((unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    RemainingSectionLength_174(_state)
                }) & (1_u64))
                    != (0_u64))
                {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                (unsafe {
                    let _jpg: *mut brunsli_JPEGData = jpg;
                    let _state: *mut brunsli_internal_dec_State = state;
                    WarmupMeta_196(_jpg, _state)
                });
                let mut status: brunsli_BrunsliStatus = (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    DecodeACDataSection_191(_state)
                });
                if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus = status;
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            _ => {
                return (unsafe {
                    let _state: *mut brunsli_internal_dec_State = state;
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    Fail_169(_state, _result)
                });
            }
        }
    };
    if !(unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        IsAtSectionBoundary_175(_state)
    }) {
        return (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            Fail_169(_state, _result)
        });
    }
    if (((*s).section.tag) == (kBrunsliACDataTag_37 as u64)) {
        return brunsli_internal_dec_Stage::DONE;
    }
    return brunsli_internal_dec_Stage::SECTION;
}
pub unsafe fn UpdateSubsamplingDerivatives_178(mut jpg: *mut brunsli_JPEGData) -> bool {
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*jpg).components.len() as u64)) {
        let mut c: *mut brunsli_JPEGComponent =
            (&mut (&mut (*jpg)).components[(i) as usize] as *mut brunsli_JPEGComponent);
        (*jpg).max_h_samp_factor = (*if *&(*jpg).max_h_samp_factor >= *&(*c).h_samp_factor {
            (&(*jpg).max_h_samp_factor) as *const _
        } else {
            (&(*c).h_samp_factor) as *const _
        });
        (*jpg).max_v_samp_factor = (*if *&(*jpg).max_v_samp_factor >= *&(*c).v_samp_factor {
            (&(*jpg).max_v_samp_factor) as *const _
        } else {
            (&(*c).v_samp_factor) as *const _
        });
        i.prefix_inc();
    }
    (*jpg).MCU_rows = (unsafe {
        let _a: i32 = (*jpg).height;
        let _b: i32 = (((*jpg).max_v_samp_factor) * (8));
        DivCeil_142(_a, _b)
    });
    (*jpg).MCU_cols = (unsafe {
        let _a: i32 = (*jpg).width;
        let _b: i32 = (((*jpg).max_h_samp_factor) * (8));
        DivCeil_142(_a, _b)
    });
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*jpg).components.len() as u64)) {
        let mut c: *mut brunsli_JPEGComponent =
            (&mut (&mut (*jpg)).components[(i) as usize] as *mut brunsli_JPEGComponent);
        (*c).width_in_blocks = ((((*jpg).MCU_cols) * ((*c).h_samp_factor)) as u32);
        (*c).height_in_blocks = ((((*jpg).MCU_rows) * ((*c).v_samp_factor)) as u32);
        if !(((*c).width_in_blocks) <= (8205_u32)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2211;
                let _fn: *const u8 = b"UpdateSubsamplingDerivatives\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !(((*c).height_in_blocks) <= (8205_u32)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2212;
                let _fn: *const u8 = b"UpdateSubsamplingDerivatives\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut num_blocks: u32 = ((*c).width_in_blocks).wrapping_mul((*c).height_in_blocks);
        if ((num_blocks as u64) > (kBrunsliMaxNumBlocks_18)) {
            return false;
        }
        (*c).num_blocks = num_blocks;
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn PrepareMeta_179(
    mut jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_State,
) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let mut num_components: u64 = (*jpg).components.len() as u64;
    (*s).block_state_
        .resize_with(num_components as usize, || <Vec<u8>>::default());
    let meta: *mut Vec<brunsli_internal_dec_ComponentMeta> =
        &mut (*state).meta as *mut Vec<brunsli_internal_dec_ComponentMeta>;
    {
        let __a0 = num_components as usize;
        (*meta).resize_with(__a0, || <brunsli_internal_dec_ComponentMeta>::default())
    };
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (num_components)) {
        let c: *const brunsli_JPEGComponent =
            &(&(*jpg)).components[(i) as usize] as *const brunsli_JPEGComponent;
        let m: *mut brunsli_internal_dec_ComponentMeta =
            &mut (&mut (*meta))[(i) as usize] as *mut brunsli_internal_dec_ComponentMeta;
        (*m).h_samp = (*c).h_samp_factor;
        (*m).v_samp = (*c).v_samp_factor;
        (*m).width_in_blocks = (((*jpg).MCU_cols) * ((*m).h_samp));
        (*m).height_in_blocks = (((*jpg).MCU_rows) * ((*m).v_samp));
        i.prefix_inc();
    }
}
pub unsafe fn WarmupMeta_196(
    mut jpg: *mut brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_State,
) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let meta: *mut Vec<brunsli_internal_dec_ComponentMeta> =
        &mut (*state).meta as *mut Vec<brunsli_internal_dec_ComponentMeta>;
    let num_components: u64 = (*meta).len() as u64;
    if !(*state).is_storage_allocated {
        (*state).is_storage_allocated = true;
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (num_components)) {
            let mut num_blocks: u64 = ((((&mut (*meta))[(i) as usize].width_in_blocks)
                * ((&mut (*meta))[(i) as usize].height_in_blocks))
                as u64);
            {
                let __a0 = (num_blocks).wrapping_mul((kDCTBlockSize_3 as u64)) as usize;
                (&mut (*jpg)).components[(i) as usize]
                    .coeffs
                    .resize_with(__a0, || <i16>::default())
            };
            {
                let __a0 = num_blocks as usize;
                (&mut (*s)).block_state_[(i) as usize].resize_with(__a0, || <u8>::default())
            };
            (&mut (*meta))[(i) as usize].block_state =
                (&mut (*s)).block_state_[(i) as usize].as_mut_ptr();
            i.prefix_inc();
        }
    }
    if !(*s).is_meta_warm {
        (*s).is_meta_warm = true;
        let mut c: u64 = 0_u64;
        'loop_: while ((c) < (num_components)) {
            let m: *mut brunsli_internal_dec_ComponentMeta =
                &mut (&mut (*meta))[(c) as usize] as *mut brunsli_internal_dec_ComponentMeta;
            let q: *const brunsli_JPEGQuantTable = &(&mut (*jpg)).quant
                [((&mut (*jpg)).components[(c) as usize].quant_idx as u64) as usize]
                as *const brunsli_JPEGQuantTable;
            (*m).ac_coeffs = (&mut (*jpg)).components[(c) as usize].coeffs.as_mut_ptr();
            (*m).ac_stride = (((*m).width_in_blocks) * (kDCTBlockSize_3));
            (*m).b_stride = (*m).width_in_blocks;
            {
                if (kDCTBlockSize_3 as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64)
                    != 0
                {
                    ::std::ptr::copy_nonoverlapping(
                        ((*q).values.as_ptr() as *const i32 as *const ::libc::c_void),
                        ((*m).quant.as_mut_ptr() as *mut i32 as *mut ::libc::c_void),
                        (kDCTBlockSize_3 as u64)
                            .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64)
                            as usize,
                    )
                }
                ((*m).quant.as_mut_ptr() as *mut i32 as *mut ::libc::c_void)
            };
            c.prefix_inc();
        }
    }
}
pub unsafe fn DoProcessJpeg_197(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    'loop_: while true {
        'switch: {
            let __match_cond = (*state).stage;
            match __match_cond {
                v if v == brunsli_internal_dec_Stage::SIGNATURE => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        VerifySignature_176(_state)
                    })
                    .clone();
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::HEADER => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _jpg: *mut brunsli_JPEGData = jpg;
                        DecodeHeader_177(_state, _jpg)
                    })
                    .clone();
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::FALLBACK => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _jpg: *mut brunsli_JPEGData = jpg;
                        DecodeOriginalJpg_192(_state, _jpg)
                    })
                    .clone();
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::SECTION => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        ParseSection_193(_state)
                    })
                    .clone();
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::SECTION_BODY => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _jpg: *mut brunsli_JPEGData = jpg;
                        ProcessSection_195(_state, _jpg)
                    })
                    .clone();
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::DONE => {
                    if (((*state).pos) != ((*state).len)) {
                        (*state).stage = (unsafe {
                            let _state: *mut brunsli_internal_dec_State = state;
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        })
                        .clone();
                        break 'switch;
                    }
                    return brunsli_BrunsliStatus::BRUNSLI_OK;
                }
                v if v == brunsli_internal_dec_Stage::ERROR => {
                    return (*(*state).internal.as_deref_mut().unwrap()).result;
                }
                _ => {
                    (*state).stage = (unsafe {
                        let _state: *mut brunsli_internal_dec_State = state;
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    })
                    .clone();
                    break 'switch;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn ChargeBuffer_198(mut state: *mut brunsli_internal_dec_State) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let b: *mut brunsli_internal_dec_Buffer = &mut (*s).buffer as *mut brunsli_internal_dec_Buffer;
    (*b).borrowed_len = 0_u64;
    (*b).external_data = (*state).data;
    (*b).external_pos = (*state).pos;
    (*b).external_len = (*state).len;
}
pub static mut kBufferMaxReadAhead_199: u64 = unsafe { 600_u64 };
pub unsafe fn LoadInput_200(mut state: *mut brunsli_internal_dec_State) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let b: *mut brunsli_internal_dec_Buffer = &mut (*s).buffer as *mut brunsli_internal_dec_Buffer;
    if (((*b).data_len) == (0_u64)) {
        (*state).data = (*b).external_data;
        (*state).pos = (*b).external_pos;
        (*state).len = (*b).external_len;
        return;
    }
    if !(((*b).data_len) <= (kBufferMaxReadAhead_199)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 2337;
            let _fn: *const u8 = b"LoadInput\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let mut available: u64 = ((*b).external_len).wrapping_sub((*b).external_pos);
    (*b).borrowed_len = (*if *&kBufferMaxReadAhead_199 <= *&mut available {
        (&kBufferMaxReadAhead_199) as *const _
    } else {
        (&mut available) as *const _
    });
    {
        if (*b).borrowed_len != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((*b).external_data.offset(((*b).external_pos) as isize) as *const u8
                    as *const ::libc::c_void),
                ((*b).data.as_mut_ptr().offset(((*b).data_len) as isize) as *mut u8
                    as *mut ::libc::c_void),
                (*b).borrowed_len as usize,
            )
        }
        ((*b).data.as_mut_ptr().offset(((*b).data_len) as isize) as *mut u8 as *mut ::libc::c_void)
    };
    (*state).data = ((*b).data.as_mut_ptr()).cast_const();
    (*state).pos = 0_u64;
    (*state).len = ((*b).data_len).wrapping_add((*b).borrowed_len);
}
pub unsafe fn UnloadInput_201(
    mut state: *mut brunsli_internal_dec_State,
    mut result: brunsli_BrunsliStatus,
) -> bool {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let b: *mut brunsli_internal_dec_Buffer = &mut (*s).buffer as *mut brunsli_internal_dec_Buffer;
    if (((*state).data) == ((*b).external_data)) {
        (*b).external_pos = (*state).pos;
        if !(((*b).external_pos) <= ((*b).external_len)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2364;
                let _fn: *const u8 = b"UnloadInput\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if ((result as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
            return true;
        }
        if !(((*b).data_len) == (0_u64)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2366;
                let _fn: *const u8 = b"UnloadInput\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut available: u64 = ((*b).external_len).wrapping_sub((*b).external_pos);
        if !((available) < (kBufferMaxReadAhead_199)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2368;
                let _fn: *const u8 = b"UnloadInput\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if (*b).data.is_empty() {
            {
                let __a0 = (2_u64).wrapping_mul(kBufferMaxReadAhead_199) as usize;
                (*b).data.resize_with(__a0, || <u8>::default())
            };
        }
        (*b).data_len = available;
        {
            if (*b).data_len != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((*b).external_data.offset(((*b).external_pos) as isize) as *const u8
                        as *const ::libc::c_void),
                    ((*b).data.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
                    (*b).data_len as usize,
                )
            }
            ((*b).data.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
        };
        (*b).external_pos = ((*b).external_pos).wrapping_add(available);
        return false;
    }
    if (((*state).pos) >= ((*b).data_len)) {
        let mut used_borrowed_bytes: u64 = ((*state).pos).wrapping_sub((*b).data_len);
        (*b).data_len = 0_u64;
        (*b).external_pos = ((*b).external_pos).wrapping_add(used_borrowed_bytes);
        return true;
    }
    (*b).data_len = ((*b).data_len).wrapping_sub((*state).pos);
    if ((result as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if !((((*b).external_pos).wrapping_add((*b).borrowed_len)) == ((*b).external_len)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2389;
                let _fn: *const u8 = b"UnloadInput\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !((((*b).data_len).wrapping_add((*b).borrowed_len)) < (kBufferMaxReadAhead_199)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2391;
                let _fn: *const u8 = b"UnloadInput\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        (*b).data_len = ((*b).data_len).wrapping_add((*b).borrowed_len);
        (*b).external_pos = ((*b).external_pos).wrapping_add((*b).borrowed_len);
    }
    if !(!(*b).data.is_empty()) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 2395;
            let _fn: *const u8 = b"UnloadInput\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if (((*state).pos) > (0_u64)) && (((*b).data_len) > (0_u64)) {
        {
            if (*b).data_len != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((*b).data.as_mut_ptr().offset(((*state).pos) as isize) as *const u8
                        as *const ::libc::c_void),
                    ((*b).data.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
                    (*b).data_len as usize,
                )
            }
            ((*b).data.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
        };
    }
    if !(((*b).data_len) <= (kBufferMaxReadAhead_199)) {
        (unsafe {
            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
            let _l: i32 = 2399;
            let _fn: *const u8 = b"UnloadInput\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return ((result as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32));
}
pub unsafe fn UnchargeBuffer_202(mut state: *mut brunsli_internal_dec_State) {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    let b: *mut brunsli_internal_dec_Buffer = &mut (*s).buffer as *mut brunsli_internal_dec_Buffer;
    (*state).data = (*b).external_data;
    (*state).pos = (*b).external_pos;
    (*state).len = (*b).external_len;
}
pub unsafe fn ProcessJpeg_203(
    mut state: *mut brunsli_internal_dec_State,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*(*state).internal.as_deref_mut().unwrap())
            as *mut brunsli_internal_dec_InternalState;
    if (((*state).pos) > ((*state).len)) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM;
    }
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        ChargeBuffer_198(_state)
    });
    let mut result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    'loop_: while ((result as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if (((*state).stage) == (brunsli_internal_dec_Stage::ERROR)) {
            if (((*s).result as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
                return (*s).result;
            }
            (*s).result = (brunsli_BrunsliStatus::BRUNSLI_OK).clone();
            (*state).stage = ((*s).last_stage).clone();
            (*s).last_stage = (brunsli_internal_dec_Stage::ERROR).clone();
        }
        (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            LoadInput_200(_state)
        });
        if (*s).section.is_active {
            (*s).section.milestone = (*state).pos;
            (*s).section.projected_end =
                ((*s).section.milestone).wrapping_add((*s).section.remaining);
        }
        (*s).section.tags_met |= (*state).tags_met;
        result = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _jpg: *mut brunsli_JPEGData = jpg;
            DoProcessJpeg_197(_state, _jpg)
        })
        .clone();
        if (*s).section.is_active {
            let mut processed_len: u64 = ((*state).pos).wrapping_sub((*s).section.milestone);
            (*s).section.remaining = ((*s).section.remaining).wrapping_sub(processed_len);
        }
        if !(unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _result: brunsli_BrunsliStatus = result;
            UnloadInput_201(_state, _result)
        }) {
            break;
        }
    }
    (unsafe {
        let _state: *mut brunsli_internal_dec_State = state;
        UnchargeBuffer_202(_state)
    });
    return result;
}
pub unsafe fn BrunsliDecodeJpeg_204(
    mut data: *const u8,
    len: u64,
    mut jpg: *mut brunsli_JPEGData,
) -> brunsli_BrunsliStatus {
    if !!(data).is_null() {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM;
    }
    let mut state: brunsli_internal_dec_State =
        brunsli_internal_dec_State::brunsli_internal_dec_State();
    state.data = data;
    state.len = len;
    return (unsafe {
        let _state: *mut brunsli_internal_dec_State =
            (&mut state as *mut brunsli_internal_dec_State);
        let _jpg: *mut brunsli_JPEGData = jpg;
        ProcessJpeg_203(_state, _jpg)
    });
}
pub unsafe fn BrunsliEstimateDecoderPeakMemoryUsage_205(mut data: *const u8, len: u64) -> u64 {
    if !!(data).is_null() {
        return (brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM as u64);
    }
    let mut state: brunsli_internal_dec_State =
        brunsli_internal_dec_State::brunsli_internal_dec_State();
    state.data = data;
    state.len = len;
    state.skip_tags = !((1_u32) << (kBrunsliHistogramDataTag_35 as i32));
    let s: *mut brunsli_internal_dec_InternalState =
        &mut (*state.internal.as_deref_mut().unwrap()) as *mut brunsli_internal_dec_InternalState;
    (*s).shallow_histograms = true;
    let mut jpg: brunsli_JPEGData = brunsli_JPEGData::brunsli_JPEGData();
    let mut status: brunsli_BrunsliStatus = (unsafe {
        let _state: *mut brunsli_internal_dec_State =
            (&mut state as *mut brunsli_internal_dec_State);
        let _jpg: *mut brunsli_JPEGData = (&mut jpg as *mut brunsli_JPEGData);
        ProcessJpeg_203(_state, _jpg)
    });
    if ((status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
        return 0_u64;
    }
    let mut out_size: u64 = (2_u64).wrapping_mul(len);
    let mut total_num_blocks: u64 = 0_u64;
    let mut component_state_size: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (jpg.components.len() as u64)) {
        let c: *const brunsli_JPEGComponent =
            &jpg.components[(i) as usize] as *const brunsli_JPEGComponent;
        total_num_blocks = (total_num_blocks).wrapping_add(((*c).num_blocks as u64));
        component_state_size = (component_state_size).wrapping_add(
            (unsafe {
                let _w: i32 = ((*c).width_in_blocks as i32);
                brunsli_ComponentState::SizeInBytes(_w)
            }),
        );
        i.prefix_inc();
    }
    let mut jpeg_data_size: u64 = ((total_num_blocks).wrapping_mul((kDCTBlockSize_3 as u64)))
        .wrapping_mul(::std::mem::size_of::<i16>() as u64 as u64);
    let mut context_map_size: u64 = (((*s).num_contexts).wrapping_mul(kNumAvrgContexts_83))
        .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64);
    let mut histogram_size: u64 = ((*s).num_histograms)
        .wrapping_mul(::std::mem::size_of::<brunsli_ANSDecodingData>() as u64 as u64);
    let mut decode_peak: u64 =
        ((context_map_size).wrapping_add(histogram_size)).wrapping_add(component_state_size);
    let mut jpeg_writer_size: u64 = (((1_u32) << (17_u32)) as u64).wrapping_add(
        (((1_u32) << (16_u32)) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64)
            as u64,
    );
    return (((out_size).wrapping_add(jpeg_data_size)).wrapping_add(
        (*if *&mut decode_peak >= *&mut jpeg_writer_size {
            (&mut decode_peak) as *const _
        } else {
            (&mut jpeg_writer_size) as *const _
        }),
    ));
}
impl brunsli_BrunsliDecoder {}
impl brunsli_BrunsliDecoder {
    pub unsafe fn Decode(
        &mut self,
        mut available_in: *mut u64,
        mut next_in: *mut *const u8,
        mut available_out: *mut u64,
        mut next_out: *mut *mut u8,
    ) -> brunsli_BrunsliDecoder_Status {
        let mut jpg: *mut brunsli_JPEGData = self
            .jpg_
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut brunsli_JPEGData);
        if !!(jpg).is_null() {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2511;
                let _fn: *const u8 = b"Decode\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut state: *mut brunsli_internal_dec_State = self
            .state_
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| {
                v as *mut brunsli_internal_dec_State
            });
        if !!(state).is_null() {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2513;
                let _fn: *const u8 = b"Decode\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        (*state).data = (*next_in);
        (*state).pos = 0_u64;
        (*state).len = (*available_in);
        let mut parse_status: brunsli_BrunsliStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _jpg: *mut brunsli_JPEGData = jpg;
            ProcessJpeg_203(_state, _jpg)
        });
        let mut consumed_bytes: u64 = (*state).pos;
        (*available_in) = (*available_in).wrapping_sub(consumed_bytes);
        (*next_in) = (*next_in).wrapping_add(consumed_bytes as usize);
        if ((parse_status as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32))
            && ((parse_status as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            return brunsli_BrunsliDecoder_Status::ERROR;
        }
        if !((*available_in) == (0_u64)) {
            (unsafe {
                let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                let _l: i32 = 2529;
                let _fn: *const u8 = b"Decode\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut serialization_status: brunsli_internal_dec_SerializationStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State = state;
            let _jpg: *const brunsli_JPEGData = &(*jpg) as *const brunsli_JPEGData;
            let _available_out: *mut u64 = available_out;
            let _next_out: *mut *mut u8 = next_out;
            SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
        });
        if ((serialization_status) == (brunsli_internal_dec_SerializationStatus::ERROR)) {
            return brunsli_BrunsliDecoder_Status::ERROR;
        }
        'switch: {
            let __match_cond = serialization_status;
            match __match_cond {
                v if v == brunsli_internal_dec_SerializationStatus::DONE => {
                    if !((parse_status as i32) == (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 2540;
                            let _fn: *const u8 = b"Decode\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::DONE;
                }
                v if v == brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT => {
                    if !((parse_status as i32)
                        == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 2545;
                            let _fn: *const u8 = b"Decode\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::NEEDS_MORE_INPUT;
                }
                v if v == brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT => {
                    if !((*available_out) == (0_u64)) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 2551;
                            let _fn: *const u8 = b"Decode\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::NEEDS_MORE_OUTPUT;
                }
                v if v == brunsli_internal_dec_SerializationStatus::ERROR => {
                    return brunsli_BrunsliDecoder_Status::ERROR;
                }
                _ => {
                    if !(false) {
                        (unsafe {
                            let _f: *const u8 = b"brunsli_decode.cc\0".as_ptr();
                            let _l: i32 = 2559;
                            let _fn: *const u8 = b"Decode\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::ERROR;
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
}
pub unsafe fn MoveToFront_207(mut v: *mut u8, mut index: u8) {
    let mut value: u8 = (*v.offset((index) as isize));
    let mut i: u8 = index;
    'loop_: while (i != 0) {
        (*v.offset((i) as isize)) = (*v.offset(((i as i32) - (1)) as isize));
        i.prefix_dec();
    }
    (*v.offset((0) as isize)) = value;
}
pub unsafe fn InverseMoveToFrontTransform_208(mut v: *mut u8, mut v_len: u64) {
    let mut mtf: [u8; 256] = [0_u8; 256];
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (256_u64)) {
        mtf[(i) as usize] = (i as u8);
        i.prefix_inc();
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (v_len)) {
        let mut index: u8 = (*v.offset((i) as isize));
        (*v.offset((i) as isize)) = mtf[(index) as usize];
        if (index != 0) {
            (unsafe {
                let _v: *mut u8 = mtf.as_mut_ptr();
                let _index: u8 = index;
                MoveToFront_207(_v, _index)
            });
        }
        i.prefix_inc();
    }
}
pub unsafe fn DecodeContextMap_188(
    entropy: *const brunsli_HuffmanDecodingData,
    mut max_run_length_prefix: u64,
    mut index: *mut u64,
    mut context_map: *mut Vec<u8>,
    mut br: *mut brunsli_BrunsliBitReader,
) -> brunsli_BrunsliStatus {
    let i: *mut u64 = &mut (*index) as *mut u64;
    let mut map: *mut u8 = (*context_map).as_mut_ptr();
    let length: u64 = (*(context_map).cast_const()).len() as u64;
    'loop_: while ((*i) < (length)) {
        if !(unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u64 = ((15_u64).wrapping_add(max_run_length_prefix)).wrapping_add(1_u64);
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        let mut code: u32 = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            (*entropy).ReadSymbol(_br)
        }) as u32);
        if ((code) == (0_u32)) {
            (*map.offset((*i) as isize)) = 0_u8;
            (*i).prefix_inc();
        } else if ((code as u64) <= (max_run_length_prefix)) {
            let mut reps: u64 = ((((1_u32 as u32).wrapping_add(((1_u32) << (code)))).wrapping_add(
                (((unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = code;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) as i32) as u32),
            )) as u64);
            'loop_: while (reps.prefix_dec() != 0) {
                if ((*i) >= (length)) {
                    return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                }
                (*map.offset((*i) as isize)) = 0_u8;
                (*i).prefix_inc();
            }
        } else {
            (*map.offset((*i) as isize)) =
                (((code as u64).wrapping_sub(max_run_length_prefix)) as u8);
            (*i).prefix_inc();
        }
    }
    if ((unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = 1_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    }) != 0)
    {
        (unsafe {
            let _v: *mut u8 = map;
            let _v_len: u64 = length;
            InverseMoveToFrontTransform_208(_v, _v_len)
        });
    }
    return if (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        brunsli_BrunsliStatus::BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
    };
}
pub unsafe fn GetPopulationCountPrecision_209(mut logcount: u32) -> u32 {
    return (((logcount).wrapping_add(1_u32)) >> (1));
}
pub static mut kLengthTree_210: [i8; 31] = unsafe {
    [
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
    ]
};
pub static mut kLogCountTree_211: [i8; 21] = unsafe {
    [
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
    ]
};
pub unsafe fn ReadShortHuffmanCode_212(
    mut br: *mut brunsli_BrunsliBitReader,
    mut tree: *const i8,
) -> u64 {
    let mut pos: u64 = 0_u64;
    let mut delta: i8 = 1_i8;
    'loop_: while ((delta as i32) > (0)) {
        pos = (pos).wrapping_add(
            (((delta as u32).wrapping_add(
                (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }),
            )) as u64),
        );
        delta = (*tree.offset((pos) as isize));
    }
    return (-(delta as i32) as u64);
}
pub unsafe fn ReadHistogram_189(
    mut precision_bits: u32,
    mut counts: *mut Vec<u32>,
    mut br: *mut brunsli_BrunsliBitReader,
) -> bool {
    if !(!(*(counts).cast_const()).is_empty()) {
        (unsafe {
            let _f: *const u8 = b"histogram_decode.cc\0".as_ptr();
            let _l: i32 = 41;
            let _fn: *const u8 = b"ReadHistogram\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let mut space: u32 = ((1_u32) << (precision_bits));
    let length: u64 = (*(counts).cast_const()).len() as u64;
    {
        let count = (*counts)
            .as_mut_ptr()
            .add((*counts).len())
            .offset_from((*counts).as_mut_ptr()) as usize;
        std::slice::from_raw_parts_mut((*counts).as_mut_ptr(), count).fill(0)
    };
    let mut histogram: *mut u32 = (*counts).as_mut_ptr();
    let mut simple_code: i32 = ((unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = 1_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    }) as i32);
    if ((simple_code) == (1)) {
        let mut max_bits_counter: u64 = (length).wrapping_sub(1_u64);
        let mut max_bits: u32 = 0_u32;
        let mut symbols: [i32; 2] = [0, 0_i32];
        let num_symbols: u64 = (((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 1_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        })
        .wrapping_add(1_u32 as u32)) as u64);
        'loop_: while (max_bits_counter != 0) {
            max_bits_counter >>= 1;
            max_bits.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (num_symbols)) {
            symbols[(i) as usize] = ((((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = max_bits;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u64)
                .wrapping_rem(length)) as i32);
            i.prefix_inc();
        }
        if ((num_symbols) == (1_u64)) {
            (*histogram.offset((symbols[(0) as usize]) as isize)) = space;
        } else {
            if ((symbols[(0) as usize]) == (symbols[(1) as usize])) {
                return false;
            }
            let mut value: u32 = (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = precision_bits;
                BrunsliBitReaderRead_126(_br, _n_bits)
            });
            (*histogram.offset((symbols[(0) as usize]) as isize)) = value;
            (*histogram.offset((symbols[(1) as usize]) as isize)) = (space).wrapping_sub(value);
        }
    } else {
        let mut real_length: u64 = (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _tree: *const i8 = kLengthTree_210.as_ptr();
            ReadShortHuffmanCode_212(_br, _tree)
        });
        let mut total_count: u32 = 0_u32;
        let mut log_counts: [u32; 18] = [0_u32; 18];
        let mut omit_pos: u64 = 0_u64;
        if !((real_length) > (2_u64)) {
            (unsafe {
                let _f: *const u8 = b"histogram_decode.cc\0".as_ptr();
                let _l: i32 = 74;
                let _fn: *const u8 = b"ReadHistogram\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (real_length)) {
            log_counts[(i) as usize] = ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _tree: *const i8 = kLogCountTree_211.as_ptr();
                ReadShortHuffmanCode_212(_br, _tree)
            }) as u32)
                .clone();
            if ((log_counts[(i) as usize]) > (log_counts[(omit_pos) as usize])) {
                omit_pos = i;
            }
            i.prefix_inc();
        }
        if !((omit_pos) >= (0_u64)) {
            (unsafe {
                let _f: *const u8 = b"histogram_decode.cc\0".as_ptr();
                let _l: i32 = 80;
                let _fn: *const u8 = b"ReadHistogram\0".as_ptr();
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (real_length)) {
            let mut code: u32 = log_counts[(i) as usize];
            if ((i) == (omit_pos)) {
                i.prefix_inc();
                continue 'loop_;
            } else if ((code) == (0_u32)) {
                i.prefix_inc();
                continue 'loop_;
            } else if ((code) == (1_u32)) {
                (*histogram.offset((i) as isize)) = 1_u32;
            } else {
                let mut bit_count: u32 = (unsafe {
                    let _logcount: u32 = (code).wrapping_sub(1_u32);
                    GetPopulationCountPrecision_209(_logcount)
                });
                (*histogram.offset((i) as isize)) = ((1_u32) << ((code).wrapping_sub(1_u32)))
                    .wrapping_add(
                        ((unsafe {
                            let _br: *mut brunsli_BrunsliBitReader = br;
                            let _n_bits: u32 = bit_count;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) << (((code).wrapping_sub(1_u32)).wrapping_sub(bit_count))),
                    );
            }
            total_count = (total_count).wrapping_add((*histogram.offset((i) as isize)));
            i.prefix_inc();
        }
        if ((total_count) >= (space)) {
            return false;
        }
        (*histogram.offset((omit_pos) as isize)) = (space).wrapping_sub(total_count);
    }
    return (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    });
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct brunsli_HuffmanDecodingData {
    pub table_: Vec<brunsli_HuffmanCode>,
}
pub static mut kCodeLengthCodes_213: i32 = unsafe { 18 };
pub static mut kCodeLengthCodeOrder_214: [u8; 18] = unsafe {
    [
        1_u8, 2_u8, 3_u8, 4_u8, 0_u8, 5_u8, 17_u8, 6_u8, 16_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        12_u8, 13_u8, 14_u8, 15_u8,
    ]
};
pub static mut kDefaultCodeLength_215: u8 = unsafe { 8_u8 };
pub static mut kCodeLengthRepeatCode_216: u8 = unsafe { 16_u8 };
pub unsafe fn ReadHuffmanCodeLengths_217(
    mut code_length_code_lengths: *const u8,
    mut num_symbols: u64,
    mut code_lengths: *mut u8,
    mut br: *mut brunsli_BrunsliBitReader,
) -> bool {
    let mut symbol: u64 = 0_u64;
    let mut prev_code_len: u8 = kDefaultCodeLength_215;
    let mut repeat: u64 = 0_u64;
    let mut repeat_code_len: u8 = 0_u8;
    let kFullSpace: i32 = ((1) << (15));
    let mut space: i32 = kFullSpace;
    let mut table: [brunsli_HuffmanCode; 32] = [<brunsli_HuffmanCode>::default(); 32];
    let mut counts: [u16; 16] = [
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
        0_u16, 0_u16, 0_u16,
    ];
    let mut i: i32 = 0;
    'loop_: while ((i) < (kCodeLengthCodes_213)) {
        counts[(*code_length_code_lengths.offset((i) as isize)) as usize].prefix_inc();
        i.prefix_inc();
    }
    if !((unsafe {
        let _root_table: *mut brunsli_HuffmanCode = table.as_mut_ptr();
        let _root_bits: u64 = 5_u64;
        let _code_lengths: *const u8 = code_length_code_lengths;
        let _code_lengths_size: u64 = (kCodeLengthCodes_213 as u64);
        let _count: *mut u16 = (&mut counts[(0) as usize] as *mut u16);
        BuildHuffmanTable_218(
            _root_table,
            _root_bits,
            _code_lengths,
            _code_lengths_size,
            _count,
        )
    }) != 0)
    {
        return false;
    }
    'loop_: while ((symbol) < (num_symbols)) && ((space) > (0)) {
        let mut p: *const brunsli_HuffmanCode = (table.as_mut_ptr()).cast_const();
        let mut code_len: u8 = 0_u8;
        p = (p).wrapping_add(
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 5_u32;
                BrunsliBitReaderGet_124(_br, _n_bits)
            }) as usize,
        );
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = ((*p).bits as u32);
            BrunsliBitReaderDrop_125(_br, _n_bits)
        });
        code_len = ((*p).value as u8);
        if ((code_len as i32) < (kCodeLengthRepeatCode_216 as i32)) {
            repeat = 0_u64;
            (*code_lengths.offset((symbol.postfix_inc()) as isize)) = code_len;
            if ((code_len as i32) != (0)) {
                prev_code_len = code_len;
                space -= ((kFullSpace) >> (code_len as i32));
            }
        } else {
            let mut extra_bits: u32 = (((code_len as i32) - (14)) as u32);
            let mut old_repeat: u64 = 0_u64;
            let mut repeat_delta: u64 = 0_u64;
            let mut new_len: u8 = 0_u8;
            if ((code_len as i32) == (kCodeLengthRepeatCode_216 as i32)) {
                new_len = prev_code_len;
            }
            if ((repeat_code_len as i32) != (new_len as i32)) {
                repeat = 0_u64;
                repeat_code_len = new_len;
            }
            old_repeat = repeat;
            if ((repeat) > (0_u64)) {
                repeat = (repeat).wrapping_sub(2_u64);
                repeat <<= extra_bits;
            }
            repeat = (repeat).wrapping_add(
                (((unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = extra_bits;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                })
                .wrapping_add(3_u32 as u32)) as u64),
            );
            repeat_delta = (repeat).wrapping_sub(old_repeat);
            if (((symbol).wrapping_add(repeat_delta)) > (num_symbols)) {
                return false;
            }
            {
                let byte_0 = ((&mut (*code_lengths.offset((symbol) as isize)) as *mut u8) as *mut u8
                    as *mut ::libc::c_void) as *mut u8;
                for offset in 0..repeat_delta {
                    *byte_0.offset(offset as isize) = (repeat_code_len as i32) as u8;
                }
                ((&mut (*code_lengths.offset((symbol) as isize)) as *mut u8) as *mut u8
                    as *mut ::libc::c_void)
            };
            symbol = (symbol).wrapping_add(repeat_delta);
            if ((repeat_code_len as i32) != (0)) {
                space -= ((((repeat_delta).wrapping_mul((kFullSpace as u64))) as i32)
                    >> (repeat_code_len as i32));
            }
        }
    }
    if ((space) != (0)) {
        return false;
    }
    {
        let byte_0 = ((&mut (*code_lengths.offset((symbol) as isize)) as *mut u8) as *mut u8
            as *mut ::libc::c_void) as *mut u8;
        for offset in 0..((num_symbols).wrapping_sub(symbol)) {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut (*code_lengths.offset((symbol) as isize)) as *mut u8) as *mut u8
            as *mut ::libc::c_void)
    };
    return (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    });
}
pub unsafe fn ReadSimpleCode_219(
    mut alphabet_size: u16,
    mut br: *mut brunsli_BrunsliBitReader,
    mut table: *mut brunsli_HuffmanCode,
) -> bool {
    let mut max_bits: u32 = (if ((alphabet_size as u32) > (1_u32)) {
        ((unsafe {
            let _n: u32 = (alphabet_size as u32).wrapping_sub(1_u32 as u32);
            Log2FloorNonZero_74(_n)
        }) + (1))
    } else {
        0
    } as u32);
    let mut num_symbols: u64 = (((unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        let _n_bits: u32 = 2_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    })
    .wrapping_add(1_u32)) as u64);
    let mut symbols: [u16; 4] = [0_u16, 0_u16, 0_u16, 0_u16];
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (num_symbols)) {
        let mut symbol: u16 = ((unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = max_bits;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as u16);
        if ((symbol as i32) >= (alphabet_size as i32)) {
            return false;
        }
        symbols[(i) as usize] = symbol;
        i.prefix_inc();
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((num_symbols).wrapping_sub(1_u64))) {
        let mut j: u64 = (i).wrapping_add(1_u64);
        'loop_: while ((j) < (num_symbols)) {
            if ((symbols[(i) as usize] as i32) == (symbols[(j) as usize] as i32)) {
                return false;
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    if ((num_symbols) == (4_u64)) {
        num_symbols = (num_symbols).wrapping_add(
            ((unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = 1_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u64),
        );
    };
    let mut table_size: u64 = 1_u64;
    'switch: {
        let __match_cond = num_symbols;
        match __match_cond {
            v if v == 1_u64 => {
                (*table.offset((0) as isize)) = brunsli_HuffmanCode {
                    bits: 0_u8,
                    value: symbols[(0) as usize],
                };
                break 'switch;
            }
            v if v == 2_u64 => {
                if ((symbols[(0) as usize] as i32) > (symbols[(1) as usize] as i32)) {
                    (unsafe {
                        let _i: u64 = 0_u64;
                        let _j: u64 = 1_u64;
                        (|i: u64, j: u64| {
                            let mut t: u16 = symbols[(j) as usize];
                            symbols[(j) as usize] = symbols[(i) as usize];
                            symbols[(i) as usize] = t;
                        })(_i, _j)
                    });
                }
                (*table.offset((0) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((1) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(1) as usize],
                };
                table_size = 2_u64;
                break 'switch;
            }
            v if v == 3_u64 => {
                if ((symbols[(1) as usize] as i32) > (symbols[(2) as usize] as i32)) {
                    (unsafe {
                        let _i: u64 = 1_u64;
                        let _j: u64 = 2_u64;
                        (|i: u64, j: u64| {
                            let mut t: u16 = symbols[(j) as usize];
                            symbols[(j) as usize] = symbols[(i) as usize];
                            symbols[(i) as usize] = t;
                        })(_i, _j)
                    });
                }
                (*table.offset((0) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((2) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((1) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(1) as usize],
                };
                (*table.offset((3) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(2) as usize],
                };
                table_size = 4_u64;
                break 'switch;
            }
            v if v == 4_u64 => {
                let mut i: u64 = 0_u64;
                'loop_: while ((i) < (3_u64)) {
                    let mut j: u64 = (i).wrapping_add(1_u64);
                    'loop_: while ((j) < (4_u64)) {
                        if ((symbols[(i) as usize] as i32) > (symbols[(j) as usize] as i32)) {
                            (unsafe {
                                let _i: u64 = i;
                                let _j: u64 = j;
                                (|i: u64, j: u64| {
                                    let mut t: u16 = symbols[(j) as usize];
                                    symbols[(j) as usize] = symbols[(i) as usize];
                                    symbols[(i) as usize] = t;
                                })(_i, _j)
                            });
                        }
                        j.prefix_inc();
                    }
                    i.prefix_inc();
                }
                (*table.offset((0) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((2) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(1) as usize],
                };
                (*table.offset((1) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(2) as usize],
                };
                (*table.offset((3) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(3) as usize],
                };
                table_size = 4_u64;
                break 'switch;
            }
            v if v == 5_u64 => {
                if ((symbols[(2) as usize] as i32) > (symbols[(3) as usize] as i32)) {
                    (unsafe {
                        let _i: u64 = 2_u64;
                        let _j: u64 = 3_u64;
                        (|i: u64, j: u64| {
                            let mut t: u16 = symbols[(j) as usize];
                            symbols[(j) as usize] = symbols[(i) as usize];
                            symbols[(i) as usize] = t;
                        })(_i, _j)
                    });
                }
                (*table.offset((0) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((1) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(1) as usize],
                };
                (*table.offset((2) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((3) as isize)) = brunsli_HuffmanCode {
                    bits: 3_u8,
                    value: symbols[(2) as usize],
                };
                (*table.offset((4) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((5) as isize)) = brunsli_HuffmanCode {
                    bits: 2_u8,
                    value: symbols[(1) as usize],
                };
                (*table.offset((6) as isize)) = brunsli_HuffmanCode {
                    bits: 1_u8,
                    value: symbols[(0) as usize],
                };
                (*table.offset((7) as isize)) = brunsli_HuffmanCode {
                    bits: 3_u8,
                    value: symbols[(3) as usize],
                };
                table_size = 8_u64;
                break 'switch;
            }
            _ => {
                return false;
            }
        }
    };
    let goal_size: u32 = ((1_u32) << (kHuffmanTableBits_21));
    'loop_: while ((table_size) != (goal_size as u64)) {
        {
            if (table_size).wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                != 0
            {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*table.offset((0) as isize)) as *mut brunsli_HuffmanCode)
                        as *const brunsli_HuffmanCode
                        as *const ::libc::c_void),
                    ((&mut (*table.offset((table_size) as isize)) as *mut brunsli_HuffmanCode)
                        as *mut brunsli_HuffmanCode as *mut ::libc::c_void),
                    (table_size)
                        .wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                        as usize,
                )
            }
            ((&mut (*table.offset((table_size) as isize)) as *mut brunsli_HuffmanCode)
                as *mut brunsli_HuffmanCode as *mut ::libc::c_void)
        };
        table_size <<= 1;
    }
    return (unsafe {
        let _br: *mut brunsli_BrunsliBitReader = br;
        BrunsliBitReaderIsHealthy_132(_br)
    });
}
impl brunsli_HuffmanDecodingData {
    pub unsafe fn ReadFromBitStream(
        &mut self,
        mut alphabet_size: u64,
        mut br: *mut brunsli_BrunsliBitReader,
        mut arena: Option<*mut brunsli_Arena>,
    ) -> bool {
        let mut arena: *mut brunsli_Arena = arena.unwrap_or(std::ptr::null_mut());
        let mut local_arena: brunsli_Arena = <brunsli_Arena>::default();
        if (arena).is_null() {
            arena = (&mut local_arena as *mut brunsli_Arena);
        }
        if ((alphabet_size) > (((1) << (kMaxHuffmanBits_22)) as u64)) {
            return false;
        }
        let mut code_lengths: Vec<u8> = vec![0_u8; alphabet_size as usize];
        let mut simple_code_or_skip: u32 = (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        });
        if ((simple_code_or_skip) == (1_u32)) {
            {
                let __a0 = (((1_u32) << (kHuffmanTableBits_21)) as u64) as usize;
                self.table_
                    .resize_with(__a0, || <brunsli_HuffmanCode>::default())
            };
            return (unsafe {
                let _alphabet_size: u16 = (alphabet_size as u16);
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _table: *mut brunsli_HuffmanCode = self.table_.as_mut_ptr();
                ReadSimpleCode_219(_alphabet_size, _br, _table)
            });
        }
        let mut code_length_code_lengths: [u8; 18] = [
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8,
        ];
        let mut space: i32 = 32;
        let mut num_codes: i32 = 0;
        static mut huff_220: [brunsli_HuffmanCode; 16] = unsafe {
            [
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
            ]
        };;
        let mut i: u64 = (simple_code_or_skip as u64);
        'loop_: while ((i) < (kCodeLengthCodes_213 as u64)) && ((space) > (0)) {
            let code_len_idx: i32 = (kCodeLengthCodeOrder_214[(i) as usize] as i32);
            let mut p: *const brunsli_HuffmanCode = huff_220.as_ptr();
            let mut v: u8 = 0_u8;
            p = (p).wrapping_add(
                (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = 4_u32;
                    BrunsliBitReaderGet_124(_br, _n_bits)
                }) as usize,
            );
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = ((*p).bits as u32);
                BrunsliBitReaderDrop_125(_br, _n_bits)
            });
            v = ((*p).value as u8);
            code_length_code_lengths[(code_len_idx) as usize] = v;
            if ((v as i32) != (0)) {
                space = ((space as u32).wrapping_sub(((32_u32) >> (v as i32)))) as i32;
                num_codes.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut ok: bool = (((num_codes) == (1)) || ((space) == (0)))
            && (unsafe {
                let _code_length_code_lengths: *const u8 =
                    (code_length_code_lengths.as_mut_ptr()).cast_const();
                let _num_symbols: u64 = alphabet_size;
                let _code_lengths: *mut u8 = (&mut code_lengths[(0_u64) as usize] as *mut u8);
                let _br: *mut brunsli_BrunsliBitReader = br;
                ReadHuffmanCodeLengths_217(
                    _code_length_code_lengths,
                    _num_symbols,
                    _code_lengths,
                    _br,
                )
            });
        if (!ok)
            || (!(unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                BrunsliBitReaderIsHealthy_132(_br)
            }))
        {
            return false;
        }
        let mut counts: [u16; 16] = [
            0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
            0_u16, 0_u16, 0_u16, 0_u16,
        ];
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (alphabet_size)) {
            counts[(code_lengths[(i) as usize]) as usize].prefix_inc();
            i.prefix_inc();
        }
        (unsafe {
            let _limit: u64 = (alphabet_size).wrapping_add(376_u64);
            (*arena).reserve(_limit)
        });
        let mut table_size: u32 = (unsafe {
            let _root_table: *mut brunsli_HuffmanCode = (unsafe { (*arena).data() });
            let _root_bits: u64 = (kHuffmanTableBits_21 as u64);
            let _code_lengths: *const u8 =
                (&mut code_lengths[(0_u64) as usize] as *mut u8).cast_const();
            let _code_lengths_size: u64 = alphabet_size;
            let _count: *mut u16 = (&mut counts[(0) as usize] as *mut u16);
            BuildHuffmanTable_218(
                _root_table,
                _root_bits,
                _code_lengths,
                _code_lengths_size,
                _count,
            )
        });
        self.table_ = core::slice::from_raw_parts(
            (unsafe { (*arena).data() }),
            ((unsafe { (*arena).data() }).offset((table_size) as isize))
                .offset_from((unsafe { (*arena).data() })) as usize,
        )
        .iter()
        .map(|x| brunsli_HuffmanCode::try_from(x.clone()).ok().unwrap())
        .collect();
        return ((table_size) > (0_u32));
    }
}
impl brunsli_HuffmanDecodingData {
    pub unsafe fn ReadSymbol(&self, mut br: *mut brunsli_BrunsliBitReader) -> u16 {
        let mut n_bits: u32 = 0_u32;
        let mut table: *const brunsli_HuffmanCode = self.table_.as_ptr();
        table = (table).wrapping_add(
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = kHuffmanTableBits_21;
                BrunsliBitReaderGet_124(_br, _n_bits)
            }) as usize,
        );
        n_bits = ((*table).bits as u32);
        if ((n_bits) > (kHuffmanTableBits_21)) {
            (unsafe {
                let _br: *mut brunsli_BrunsliBitReader = br;
                let _n_bits: u32 = kHuffmanTableBits_21;
                BrunsliBitReaderDrop_125(_br, _n_bits)
            });
            n_bits = (n_bits).wrapping_sub(kHuffmanTableBits_21);
            table = (table).wrapping_add(((*table).value as i32) as usize);
            table = (table).wrapping_add(
                (unsafe {
                    let _br: *mut brunsli_BrunsliBitReader = br;
                    let _n_bits: u32 = n_bits;
                    BrunsliBitReaderGet_124(_br, _n_bits)
                }) as usize,
            );
        }
        (unsafe {
            let _br: *mut brunsli_BrunsliBitReader = br;
            let _n_bits: u32 = ((*table).bits as u32);
            BrunsliBitReaderDrop_125(_br, _n_bits)
        });
        return (*table).value;
    }
}
pub unsafe fn GetNextKey_221(mut key: i32, mut len: u64) -> i32 {
    let mut step: i32 = (((1_u32) << ((len).wrapping_sub(1_u64))) as i32);
    'loop_: while (((key) & (step)) != 0) {
        step >>= 1;
    }
    return (((key) & ((step) - (1))) + (step));
}
pub unsafe fn ReplicateValue_222(
    mut table: *mut brunsli_HuffmanCode,
    mut step: i32,
    mut end: i32,
    mut code: brunsli_HuffmanCode,
) {
    let mut __do_while = true;
    'loop_: while __do_while || ((end) > (0)) {
        __do_while = false;
        end -= step;
        (*table.offset((end) as isize)) = code;
    }
}
pub unsafe fn NextTableBitSize_223(count: *const u16, mut len: u64, mut root_bits: u64) -> u64 {
    let mut left: u64 = ((1_u64) << ((len).wrapping_sub(root_bits)));
    'loop_: while ((len) < (kMaxHuffmanBits_22)) {
        if ((left) <= ((*count.offset((len) as isize)) as u64)) {
            break;
        }
        left = (left).wrapping_sub(((*count.offset((len) as isize)) as u64));
        len.prefix_inc();
        left <<= 1;
    }
    return (len).wrapping_sub(root_bits);
}
pub unsafe fn BuildHuffmanTable_218(
    mut root_table: *mut brunsli_HuffmanCode,
    mut root_bits: u64,
    code_lengths: *const u8,
    mut code_lengths_size: u64,
    mut count: *mut u16,
) -> u32 {
    let mut code: brunsli_HuffmanCode = <brunsli_HuffmanCode>::default();
    let mut table: *mut brunsli_HuffmanCode = std::ptr::null_mut();
    let mut len: u64 = 0_u64;
    let mut symbol: u64 = 0_u64;
    let mut key: i32 = 0_i32;
    let mut step: i32 = 0_i32;
    let mut low: i32 = 0_i32;
    let mut mask: i32 = 0_i32;
    let mut table_bits: u64 = 0_u64;
    let mut table_size: i32 = 0_i32;
    let mut total_size: i32 = 0_i32;
    let mut offset: [u16; 16] = [0_u16; 16];
    let mut max_length: u64 = 1_u64;
    if ((code_lengths_size) > (((1_u32) << (kMaxHuffmanBits_22)) as u64)) {
        return 0_u32;
    }
    let mut sorted_storage: Vec<u16> = (0..(code_lengths_size) as usize)
        .map(|_| <u16>::default())
        .collect::<Vec<_>>();
    let mut sorted: *mut u16 = sorted_storage.as_mut_ptr();
    let mut sum: u16 = 0_u16;
    len = 1_u64;
    'loop_: while ((len) <= (kMaxHuffmanBits_22)) {
        offset[(len) as usize] = sum;
        if ((*count.offset((len) as isize)) != 0) {
            sum = (((sum as i32) + ((*count.offset((len) as isize)) as i32)) as u16);
            max_length = len;
        }
        len.postfix_inc();
    }
    symbol = 0_u64;
    'loop_: while ((symbol) < (code_lengths_size)) {
        if (((*code_lengths.offset((symbol) as isize)) as i32) != (0)) {
            (*sorted.offset(
                (offset[(*code_lengths.offset((symbol) as isize)) as usize].postfix_inc()) as isize,
            )) = (symbol as u16);
        }
        symbol.postfix_inc();
    }
    table = root_table;
    table_bits = root_bits;
    table_size = (((1_u32) << (table_bits)) as i32);
    total_size = table_size;
    if ((offset[(kMaxHuffmanBits_22) as usize] as i32) == (1)) {
        code.bits = 0_u8;
        code.value = (*sorted.offset((0) as isize));
        key = 0;
        'loop_: while ((key) < (total_size)) {
            (*table.offset((key) as isize)) = code;
            key.prefix_inc();
        }
        return (total_size as u32);
    }
    if ((table_bits) > (max_length)) {
        table_bits = max_length;
        table_size = (((1_u32) << (table_bits)) as i32);
    }
    key = 0;
    symbol = 0_u64;
    code.bits = 1_u8;
    step = 2;
    let mut __do_while = true;
    'loop_: while __do_while || ((code.bits.prefix_inc() as u64) <= (table_bits)) {
        __do_while = false;
        'loop_: while (((*count.offset((code.bits) as isize)) as i32) != (0)) {
            code.value = (*sorted.offset((symbol.postfix_inc()) as isize));
            (unsafe {
                let _table: *mut brunsli_HuffmanCode =
                    (&mut (*table.offset((key) as isize)) as *mut brunsli_HuffmanCode);
                let _step: i32 = step;
                let _end: i32 = table_size;
                let _code: brunsli_HuffmanCode = code.clone();
                ReplicateValue_222(_table, _step, _end, _code)
            });
            key = (unsafe {
                let _key: i32 = key;
                let _len: u64 = (code.bits as u64);
                GetNextKey_221(_key, _len)
            });
            (*count.offset((code.bits) as isize)).prefix_dec();
        }
        step <<= 1;
    }
    'loop_: while ((total_size) != (table_size)) {
        {
            if (table_size as u64)
                .wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                != 0
            {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*table.offset((0) as isize)) as *mut brunsli_HuffmanCode)
                        as *const brunsli_HuffmanCode
                        as *const ::libc::c_void),
                    ((&mut (*table.offset((table_size) as isize)) as *mut brunsli_HuffmanCode)
                        as *mut brunsli_HuffmanCode as *mut ::libc::c_void),
                    (table_size as u64)
                        .wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                        as usize,
                )
            }
            ((&mut (*table.offset((table_size) as isize)) as *mut brunsli_HuffmanCode)
                as *mut brunsli_HuffmanCode as *mut ::libc::c_void)
        };
        table_size <<= 1;
    }
    mask = ((total_size) - (1));
    low = -1_i32;
    len = (root_bits).wrapping_add(1_u64);
    step = 2;
    'loop_: while ((len) <= (max_length)) {
        'loop_: while (((*count.offset((len) as isize)) as i32) != (0)) {
            if (((key) & (mask)) != (low)) {
                table = (table).wrapping_add(table_size as usize);
                table_bits = (unsafe {
                    let _count: *const u16 = (count).cast_const();
                    let _len: u64 = len;
                    let _root_bits: u64 = root_bits;
                    NextTableBitSize_223(_count, _len, _root_bits)
                });
                table_size = (((1_u32) << (table_bits)) as i32);
                total_size += table_size;
                low = ((key) & (mask));
                (*root_table.offset((low) as isize)).bits =
                    (((table_bits).wrapping_add(root_bits)) as u8);
                (*root_table.offset((low) as isize)).value =
                    (((((table as usize - root_table as usize)
                        / ::std::mem::size_of::<brunsli_HuffmanCode>())
                        as i64)
                        - (low as i64)) as u16);
            }
            code.bits = (((len).wrapping_sub(root_bits)) as u8);
            code.value = (*sorted.offset((symbol.postfix_inc()) as isize));
            (unsafe {
                let _table: *mut brunsli_HuffmanCode = (&mut (*table
                    .offset(((key) >> (root_bits)) as isize))
                    as *mut brunsli_HuffmanCode);
                let _step: i32 = step;
                let _end: i32 = table_size;
                let _code: brunsli_HuffmanCode = code.clone();
                ReplicateValue_222(_table, _step, _end, _code)
            });
            key = (unsafe {
                let _key: i32 = key;
                let _len: u64 = len;
                GetNextKey_221(_key, _len)
            });
            (*count.offset((len) as isize)).prefix_dec();
        }
        len.prefix_inc();
        step <<= 1;
    }
    return (total_size as u32);
}
impl brunsli_internal_dec_OutputChunk {
    pub unsafe fn brunsli_internal_dec_OutputChunk4(bytes: *const Vec<u8>) -> Self {
        let mut this = Self {
            next: std::ptr::null(),
            len: (*bytes).len() as u64,
            buffer: None,
        };
        let mut src: *const ::libc::c_void =
            ((*bytes).as_ptr() as *const u8 as *const ::libc::c_void);
        this.next = (src as *const u8);
        this
    }
}
pub static mut kJpegPrecision_224: i32 = unsafe { 8 };
pub static mut kBitWriterChunkSize_225: u64 = unsafe { 16384_u64 };
pub unsafe fn DivCeil_226(mut a: i32, mut b: i32) -> i32 {
    return ((((a) + (b)) - (1)) / (b));
}
pub unsafe fn HasZeroByte_227(mut x: u64) -> u64 {
    return ((((x).wrapping_sub(72340172838076673_u64 as u64)) & (!x)) & (9259542123273814144_u64));
}
pub unsafe fn BitWriterInit_228(
    mut bw: *mut brunsli_internal_dec_BitWriter,
    mut output_queue: *mut Vec<brunsli_internal_dec_OutputChunk>,
) {
    (*bw).output = output_queue;
    (*bw).chunk = brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
        kBitWriterChunkSize_225,
    ));
    (*bw).pos = 0_u64;
    (*bw).put_buffer = 0_u64;
    (*bw).put_bits = 64;
    (*bw).healthy = true;
    (*bw).data = (*(*bw).chunk.buffer.as_deref_mut().unwrap()).as_mut_ptr();
}
pub unsafe fn SwapBuffer_229(mut bw: *mut brunsli_internal_dec_BitWriter) {
    (*bw).chunk.len = (*bw).pos;
    (*(*bw).output).push(std::mem::take(&mut (*bw).chunk));
    (*bw).chunk = brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
        kBitWriterChunkSize_225,
    ));
    (*bw).data = (*(*bw).chunk.buffer.as_deref_mut().unwrap()).as_mut_ptr();
    (*bw).pos = 0_u64;
}
pub unsafe fn Reserve_230(mut bw: *mut brunsli_internal_dec_BitWriter, mut n_bytes: u64) {
    if ((((((*bw).pos).wrapping_add(n_bytes)) > (kBitWriterChunkSize_225)) as i64) != 0) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            SwapBuffer_229(_bw)
        });
    }
}
pub unsafe fn EmitByte_231(mut bw: *mut brunsli_internal_dec_BitWriter, mut byte: i32) {
    (*(*bw).data.offset(((*bw).pos.postfix_inc()) as isize)) = (byte as u8);
    if ((byte) == (255)) {
        (*(*bw).data.offset(((*bw).pos.postfix_inc()) as isize)) = 0_u8;
    }
}
pub unsafe fn DischargeBitBuffer_232(mut bw: *mut brunsli_internal_dec_BitWriter) {
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _n_bytes: u64 = 12_u64;
        Reserve_230(_bw, _n_bytes)
    });
    if ((unsafe {
        let _x: u64 = ((!(*bw).put_buffer) | (65535_u64));
        HasZeroByte_227(_x)
    }) != 0)
    {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (56)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (48)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (40)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (32)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (24)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = (((((*bw).put_buffer) >> (16)) & (255_u64)) as i32);
            EmitByte_231(_bw, _byte)
        });
    } else {
        (*(*bw).data.offset(((*bw).pos) as isize)) =
            (((((*bw).put_buffer) >> (56)) & (255_u64)) as u8);
        (*(*bw)
            .data
            .offset((((*bw).pos).wrapping_add(1_u64)) as isize)) =
            (((((*bw).put_buffer) >> (48)) & (255_u64)) as u8);
        (*(*bw)
            .data
            .offset((((*bw).pos).wrapping_add(2_u64)) as isize)) =
            (((((*bw).put_buffer) >> (40)) & (255_u64)) as u8);
        (*(*bw)
            .data
            .offset((((*bw).pos).wrapping_add(3_u64)) as isize)) =
            (((((*bw).put_buffer) >> (32)) & (255_u64)) as u8);
        (*(*bw)
            .data
            .offset((((*bw).pos).wrapping_add(4_u64)) as isize)) =
            (((((*bw).put_buffer) >> (24)) & (255_u64)) as u8);
        (*(*bw)
            .data
            .offset((((*bw).pos).wrapping_add(5_u64)) as isize)) =
            (((((*bw).put_buffer) >> (16)) & (255_u64)) as u8);
        (*bw).pos = ((*bw).pos).wrapping_add(6_u64);
    }
    (*bw).put_buffer <<= 48;
    (*bw).put_bits += 48;
}
pub unsafe fn WriteBits_233(
    mut bw: *mut brunsli_internal_dec_BitWriter,
    mut nbits: i32,
    mut bits: u64,
) {
    if ((nbits) == (0)) {
        (*bw).healthy = false;
        return;
    }
    (*bw).put_bits -= nbits;
    (*bw).put_buffer |= ((bits) << ((*bw).put_bits));
    if (((*bw).put_bits) <= (16)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            DischargeBitBuffer_232(_bw)
        });
    }
}
pub unsafe fn EmitMarker_234(mut bw: *mut brunsli_internal_dec_BitWriter, mut marker: i32) {
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _n_bytes: u64 = 2_u64;
        Reserve_230(_bw, _n_bytes)
    });
    if !((marker) != (255)) {
        (unsafe {
            let _f: *const u8 = b"jpeg_data_writer.cc\0".as_ptr();
            let _l: i32 = 133;
            let _fn: *const u8 = b"EmitMarker\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (*(*bw).data.offset(((*bw).pos.postfix_inc()) as isize)) = 255_u8;
    (*(*bw).data.offset(((*bw).pos.postfix_inc()) as isize)) = (marker as u8);
}
pub unsafe fn JumpToByteBoundary_235(
    mut bw: *mut brunsli_internal_dec_BitWriter,
    mut pad_bits: *mut *const i32,
    mut pad_bits_end: *const i32,
) -> bool {
    let mut n_bits: u64 = ((((*bw).put_bits as u32) & (7_u32)) as u64);
    let mut pad_pattern: u8 = 0_u8;
    if (*pad_bits).is_null() {
        pad_pattern = ((((1_u32) << (n_bits)).wrapping_sub(1_u32)) as u8);
    } else {
        pad_pattern = 0_u8;
        let mut src: *const i32 = (*pad_bits);
        'loop_: while (n_bits.postfix_dec() != 0) {
            pad_pattern = ((pad_pattern as i32) << 1) as u8;
            if ((src) >= (pad_bits_end)) {
                return false;
            }
            pad_pattern = ((pad_pattern as i32) | (!!((*(src.postfix_inc())) != 0) as i32)) as u8;
        }
        (*pad_bits) = src;
    }
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _n_bytes: u64 = 16_u64;
        Reserve_230(_bw, _n_bytes)
    });
    'loop_: while (((*bw).put_bits) <= (56)) {
        let mut c: i32 = (((((*bw).put_buffer) >> (56)) & (255_u64)) as i32);
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = c;
            EmitByte_231(_bw, _byte)
        });
        (*bw).put_buffer <<= 8;
        (*bw).put_bits += 8;
    }
    if (((*bw).put_bits) < (64)) {
        let mut pad_mask: i32 = (((255_u32) >> ((64) - ((*bw).put_bits))) as i32);
        let mut c: i32 =
            ((((((*bw).put_buffer) >> (56)) & (!pad_mask as u64)) | (pad_pattern as u64)) as i32);
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _byte: i32 = c;
            EmitByte_231(_bw, _byte)
        });
    }
    (*bw).put_buffer = 0_u64;
    (*bw).put_bits = 64;
    return true;
}
pub unsafe fn BitWriterFinish_236(mut bw: *mut brunsli_internal_dec_BitWriter) {
    if (((*bw).pos) == (0_u64)) {
        return;
    }
    (*bw).chunk.len = (*bw).pos;
    (*(*bw).output).push(std::mem::take(&mut (*bw).chunk));
    (*bw).chunk = brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk1(
        std::ptr::null(),
        0_u64,
    );
    (*bw).data = std::ptr::null_mut();
    (*bw).pos = 0_u64;
}
pub unsafe fn DCTCodingStateInit_237(mut s: *mut brunsli_internal_dec_DCTCodingState) {
    (*s).eob_run_ = 0;
    (*s).cur_ac_huff_ = std::ptr::null();
    (*s).refinement_bits_.clear();
    if 64_u64 as usize > (*s).refinement_bits_.capacity() as usize {
        let len_0 = (*s).refinement_bits_.len();
        (*s).refinement_bits_
            .reserve_exact(64_u64 as usize - len_0 as usize);
    };
    (*s).refinement_bits_count_ = 0_u64;
}
pub unsafe fn Flush_238(
    mut s: *mut brunsli_internal_dec_DCTCodingState,
    mut bw: *mut brunsli_internal_dec_BitWriter,
) {
    if (((*s).eob_run_) > (0)) {
        let mut nbits: i32 = (unsafe {
            let _n: u32 = ((*s).eob_run_ as u32);
            Log2FloorNonZero_74(_n)
        });
        let mut symbol: i32 = ((nbits) << (4_u32));
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*(*s).cur_ac_huff_).depth[(symbol) as usize];
            let _bits: u64 = ((*(*s).cur_ac_huff_).code[(symbol) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        if ((nbits) > (0)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = nbits;
                let _bits: u64 = ((((*s).eob_run_) & (((1) << (nbits)) - (1))) as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
        }
        (*s).eob_run_ = 0;
    }
    let mut num_words: u64 = (((*s).refinement_bits_count_) >> (4));
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (num_words)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = 16;
            let _bits: u64 = ((&mut (*s)).refinement_bits_[(i) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        i.prefix_inc();
    }
    let mut tail: u64 = (((*s).refinement_bits_count_) & (15_u64));
    if (tail != 0) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (tail as i32);
            let _bits: u64 = ((*(((*s).refinement_bits_).last_mut().unwrap())) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
    }
    (*s).refinement_bits_.clear();
    (*s).refinement_bits_count_ = 0_u64;
}
pub unsafe fn BufferEndOfBand_239(
    mut s: *mut brunsli_internal_dec_DCTCodingState,
    mut ac_huff: *const brunsli_HuffmanCodeTable,
    mut new_bits_array: *const i32,
    mut new_bits_count: u64,
    mut bw: *mut brunsli_internal_dec_BitWriter,
) -> bool {
    if (((*s).eob_run_) == (0)) {
        (*s).cur_ac_huff_ = ac_huff;
    }
    (*s).eob_run_.prefix_inc();
    if (new_bits_count != 0) {
        let mut new_bits: u64 = 0_u64;
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (new_bits_count)) {
            new_bits = (((new_bits) << (1)) | ((*new_bits_array.offset((i) as isize)) as u64));
            i.prefix_inc();
        }
        let mut tail: u64 = (((*s).refinement_bits_count_) & (15_u64));
        if (tail != 0) {
            let mut stuff_bits_count: u64 = {
                let mut __tmp_0 = (16_u64).wrapping_sub(tail);
                (*if *&mut __tmp_0 <= *&mut new_bits_count {
                    (&mut __tmp_0) as *const _
                } else {
                    (&mut new_bits_count) as *const _
                })
            };
            let mut stuff_bits: u16 =
                (((new_bits) >> ((new_bits_count).wrapping_sub(stuff_bits_count))) as u16);
            stuff_bits = ((stuff_bits as u32)
                & (((1_u32) << (stuff_bits_count)).wrapping_sub(1_u32)))
                as u16;
            (*(((*s).refinement_bits_).last_mut().unwrap())) =
                (((((*(((*s).refinement_bits_).last_mut().unwrap())) as i32) << (stuff_bits_count))
                    | (stuff_bits as i32)) as u16);
            new_bits_count = (new_bits_count).wrapping_sub(stuff_bits_count);
            (*s).refinement_bits_count_ =
                ((*s).refinement_bits_count_).wrapping_add(stuff_bits_count);
        }
        'loop_: while ((new_bits_count) >= (16_u64)) {
            (*s).refinement_bits_
                .push((((new_bits) >> ((new_bits_count).wrapping_sub(16_u64))) as u16));
            new_bits_count = (new_bits_count).wrapping_sub(16_u64);
            (*s).refinement_bits_count_ = ((*s).refinement_bits_count_).wrapping_add(16_u64);
        }
        if (new_bits_count != 0) {
            (*s).refinement_bits_.push(
                (((new_bits) & ((((1_u32) << (new_bits_count)).wrapping_sub(1_u32)) as u64))
                    as u16),
            );
            (*s).refinement_bits_count_ =
                ((*s).refinement_bits_count_).wrapping_add(new_bits_count);
        }
    }
    if (((*s).refinement_bits_count_) > (((32767) * ((kDCTBlockSize_3) - (1))) as u64)) {
        return false;
    }
    if (((*s).eob_run_) == (32767)) {
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = s;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            Flush_238(_s, _bw)
        });
    }
    return true;
}
pub unsafe fn BuildHuffmanCodeTable_240(
    huff: *const brunsli_JPEGHuffmanCode,
    mut table: *mut brunsli_HuffmanCodeTable,
) -> bool {
    let mut huff_code: [i32; 256] = [0_i32; 256];
    let mut huff_size: [u32; 257] = [0_u32; 257];
    let mut p: i32 = 0;
    let mut l: u64 = 1_u64;
    'loop_: while ((l) <= (kJpegHuffmanMaxBitLength_7 as u64)) {
        let mut i: i32 = (&(*huff)).counts[(l) as usize];
        if (((p) + (i)) > ((kJpegHuffmanAlphabetSize_8) + (1))) {
            return false;
        }
        'loop_: while (i.postfix_dec() != 0) {
            huff_size[(p.postfix_inc()) as usize] = (l as u32);
        }
        l.prefix_inc();
    }
    if ((p) == (0)) {
        return true;
    }
    let mut last_p: i32 = ((p) - (1));
    huff_size[(last_p) as usize] = 0_u32;
    let mut code: i32 = 0;
    let mut si: u32 = huff_size[(0) as usize];
    p = 0;
    'loop_: while (huff_size[(p) as usize] != 0) {
        'loop_: while ((huff_size[(p) as usize]) == (si)) {
            huff_code[(p.postfix_inc()) as usize] = code;
            code.postfix_inc();
        }
        code <<= 1;
        si.postfix_inc();
    }
    p = 0;
    'loop_: while ((p) < (last_p)) {
        let mut i: i32 = (&(*huff)).values[(p as u64) as usize];
        (*table).depth[(i) as usize] = (huff_size[(p) as usize] as i32);
        (*table).code[(i) as usize] = huff_code[(p) as usize];
        p.postfix_inc();
    }
    return true;
}
pub unsafe fn EncodeSOI_241(mut state: *mut brunsli_internal_dec_SerializationState) -> bool {
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8, 216_u8]),
    );
    return true;
}
pub unsafe fn EncodeEOI_242(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8, 217_u8]),
    );
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
            &(*jpg).tail_data as *const Vec<u8>,
        ),
    );
    return true;
}
pub unsafe fn EncodeSOF_243(
    jpg: *const brunsli_JPEGData,
    mut marker: u8,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    if ((marker as i32) <= (194)) {
        (*state).is_progressive = ((marker as i32) == (194));
    }
    let n_comps: u64 = (*jpg).components.len() as u64;
    let marker_len: u64 = (8_u64).wrapping_add((3_u64).wrapping_mul(n_comps));
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (marker_len).wrapping_add(2_u64),
        )),
    );
    let mut data: *mut u8 = (*(*((*state).output_queue.last_mut().unwrap()))
        .buffer
        .as_deref_mut()
        .unwrap())
    .as_mut_ptr();
    let mut pos: u64 = 0_u64;
    (*data.offset((pos.postfix_inc()) as isize)) = 255_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = marker;
    (*data.offset((pos.postfix_inc()) as isize)) = (((marker_len) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (marker_len as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (kJpegPrecision_224 as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = ((((*jpg).height) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = ((((*jpg).height as u32) & (255_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = ((((*jpg).width) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = ((((*jpg).width as u32) & (255_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (n_comps as u8);
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (n_comps)) {
        (*data.offset((pos.postfix_inc()) as isize)) =
            ((&(*jpg)).components[(i) as usize].id as u8);
        (*data.offset((pos.postfix_inc()) as isize)) =
            (((((&(*jpg)).components[(i) as usize].h_samp_factor) << (4_u32))
                | ((&(*jpg)).components[(i) as usize].v_samp_factor)) as u8);
        let quant_idx: u64 = ((&(*jpg)).components[(i) as usize].quant_idx as u64);
        if ((quant_idx) >= ((*jpg).quant.len() as u64)) {
            return false;
        }
        (*data.offset((pos.postfix_inc()) as isize)) =
            ((&(*jpg)).quant[(quant_idx) as usize].index as u8);
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn EncodeSOS_244(
    jpg: *const brunsli_JPEGData,
    scan_info: *const brunsli_JPEGScanInfo,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    let n_scans: u64 = (*scan_info).num_components;
    let marker_len: u64 = (6_u64).wrapping_add((2_u64).wrapping_mul(n_scans));
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (marker_len).wrapping_add(2_u64),
        )),
    );
    let mut data: *mut u8 = (*(*((*state).output_queue.last_mut().unwrap()))
        .buffer
        .as_deref_mut()
        .unwrap())
    .as_mut_ptr();
    let mut pos: u64 = 0_u64;
    (*data.offset((pos.postfix_inc()) as isize)) = 255_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = 218_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = (((marker_len) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (marker_len as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (n_scans as u8);
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (n_scans)) {
        let si: *const brunsli_JPEGComponentScanInfo =
            &(&(*scan_info)).components[(i) as usize] as *const brunsli_JPEGComponentScanInfo;
        if (((*si).comp_idx as u64) >= ((*jpg).components.len() as u64)) {
            return false;
        }
        (*data.offset((pos.postfix_inc()) as isize)) =
            ((&(*jpg)).components[((*si).comp_idx as u64) as usize].id as u8);
        (*data.offset((pos.postfix_inc()) as isize)) =
            (((((*si).dc_tbl_idx) << (4_u32)) + ((*si).ac_tbl_idx)) as u8);
        i.prefix_inc();
    }
    (*data.offset((pos.postfix_inc()) as isize)) = ((*scan_info).Ss as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = ((*scan_info).Se as u8);
    (*data.offset((pos.postfix_inc()) as isize)) =
        (((((*scan_info).Ah) << (4_u32)) | ((*scan_info).Al)) as u8);
    return true;
}
pub unsafe fn EncodeDHT_245(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    let huffman_code: *const Vec<brunsli_JPEGHuffmanCode> =
        &(*jpg).huffman_code as *const Vec<brunsli_JPEGHuffmanCode>;
    let mut marker_len: u64 = 2_u64;
    let mut i: u64 = ((*state).dht_index as u64);
    'loop_: while ((i) < ((*huffman_code).len() as u64)) {
        let huff: *const brunsli_JPEGHuffmanCode =
            &(&(*huffman_code))[(i) as usize] as *const brunsli_JPEGHuffmanCode;
        marker_len = (marker_len).wrapping_add((kJpegHuffmanMaxBitLength_7 as u64));
        let mut j: u64 = 0_u64;
        'loop_: while ((j) < ((*huff).counts.len() as u64)) {
            marker_len = (marker_len).wrapping_add(((&(*huff)).counts[(j) as usize] as u64));
            j.prefix_inc();
        }
        if (*huff).is_last {
            break;
        }
        i.prefix_inc();
    }
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (marker_len).wrapping_add(2_u64),
        )),
    );
    let mut data: *mut u8 = (*(*((*state).output_queue.last_mut().unwrap()))
        .buffer
        .as_deref_mut()
        .unwrap())
    .as_mut_ptr();
    let mut pos: u64 = 0_u64;
    (*data.offset((pos.postfix_inc()) as isize)) = 255_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = 196_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = (((marker_len) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (marker_len as u8);
    'loop_: while true {
        let huffman_code_index: u64 = ((*state).dht_index.postfix_inc() as u64);
        if ((huffman_code_index) >= ((*huffman_code).len() as u64)) {
            return false;
        }
        let huff: *const brunsli_JPEGHuffmanCode =
            &(&(*huffman_code))[(huffman_code_index) as usize] as *const brunsli_JPEGHuffmanCode;
        let mut index: u64 = ((*huff).slot_id as u64);
        let mut huff_table: *mut brunsli_HuffmanCodeTable = std::ptr::null_mut();
        if (((index) & (16_u64)) != 0) {
            index = (index).wrapping_sub(16_u64);
            huff_table = (&mut (&mut (*state)).ac_huff_table[(index) as usize]
                as *mut brunsli_HuffmanCodeTable);
        } else {
            huff_table = (&mut (&mut (*state)).dc_huff_table[(index) as usize]
                as *mut brunsli_HuffmanCodeTable);
        }
        if !(unsafe {
            let _huff: *const brunsli_JPEGHuffmanCode = huff;
            let _table: *mut brunsli_HuffmanCodeTable = huff_table;
            BuildHuffmanCodeTable_240(_huff, _table)
        }) {
            return false;
        }
        let mut total_count: u64 = 0_u64;
        let mut max_length: u64 = 0_u64;
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < ((*huff).counts.len() as u64)) {
            if (((&(*huff)).counts[(i) as usize]) != (0)) {
                max_length = i;
            }
            total_count = (total_count).wrapping_add(((&(*huff)).counts[(i) as usize] as u64));
            i.prefix_inc();
        }
        total_count.prefix_dec();
        (*data.offset((pos.postfix_inc()) as isize)) = ((*huff).slot_id as u8);
        let mut i: u64 = 1_u64;
        'loop_: while ((i) <= (kJpegHuffmanMaxBitLength_7 as u64)) {
            (*data.offset((pos.postfix_inc()) as isize)) = ((if ((i) == (max_length)) {
                (((&(*huff)).counts[(i) as usize]) - (1))
            } else {
                (&(*huff)).counts[(i) as usize]
            }) as u8);
            i.prefix_inc();
        }
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (total_count)) {
            (*data.offset((pos.postfix_inc()) as isize)) = ((&(*huff)).values[(i) as usize] as u8);
            i.prefix_inc();
        }
        if (*huff).is_last {
            break;
        }
    }
    return true;
}
pub unsafe fn EncodeDQT_246(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    let mut marker_len: i32 = 2;
    let mut i: u64 = ((*state).dqt_index as u64);
    'loop_: while ((i) < ((*jpg).quant.len() as u64)) {
        let table: *const brunsli_JPEGQuantTable =
            &(&(*jpg)).quant[(i) as usize] as *const brunsli_JPEGQuantTable;
        marker_len += ((1) + ((if ((*table).precision != 0) { 2 } else { 1 }) * (kDCTBlockSize_3)));
        if (*table).is_last {
            break;
        }
        i.prefix_inc();
    }
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (((marker_len) + (2)) as u64),
        )),
    );
    let mut data: *mut u8 = (*(*((*state).output_queue.last_mut().unwrap()))
        .buffer
        .as_deref_mut()
        .unwrap())
    .as_mut_ptr();
    let mut pos: u64 = 0_u64;
    (*data.offset((pos.postfix_inc()) as isize)) = 255_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = 219_u8;
    (*data.offset((pos.postfix_inc()) as isize)) = (((marker_len) >> (8_u32)) as u8);
    (*data.offset((pos.postfix_inc()) as isize)) = (((marker_len as u32) & (255_u32)) as u8);
    'loop_: while true {
        let idx: u64 = ((*state).dqt_index.postfix_inc() as u64);
        if ((idx) >= ((*jpg).quant.len() as u64)) {
            return false;
        }
        let table: *const brunsli_JPEGQuantTable =
            &(&(*jpg)).quant[(idx) as usize] as *const brunsli_JPEGQuantTable;
        (*data.offset((pos.postfix_inc()) as isize)) =
            (((((*table).precision) << (4_u32)) + ((*table).index)) as u8);
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (kDCTBlockSize_3 as u64)) {
            let mut val_idx: i32 = (kJPEGNaturalOrder_13[(i) as usize] as i32);
            let mut val: i32 = (&(*table)).values[(val_idx as u64) as usize];
            if ((*table).precision != 0) {
                (*data.offset((pos.postfix_inc()) as isize)) = (((val) >> (8_u32)) as u8);
            }
            (*data.offset((pos.postfix_inc()) as isize)) = (((val as u32) & (255_u32)) as u8);
            i.prefix_inc();
        }
        if (*table).is_last {
            break;
        }
    }
    return true;
}
pub unsafe fn EncodeDRI_247(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    (*state).seen_dri_marker = true;
    let mut dri_marker: brunsli_internal_dec_OutputChunk =
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![
            255_u8,
            221_u8,
            0_u8,
            4_u8,
            ((((*jpg).restart_interval) >> (8)) as u8),
            ((((*jpg).restart_interval) & (255)) as u8),
        ]);
    (*state).output_queue.push(std::mem::take(&mut dri_marker));
    return true;
}
pub unsafe fn EncodeRestart_248(
    mut marker: u8,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8, marker]),
    );
    return true;
}
pub unsafe fn EncodeAPP_249(
    jpg: *const brunsli_JPEGData,
    mut marker: u8,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    &(marker);
    let mut app_index: u64 = ((*state).app_index.postfix_inc() as u64);
    if ((app_index) >= ((*jpg).app_data.len() as u64)) {
        return false;
    }
    (*state)
        .output_queue
        .push(brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8]));
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
            &(&(*jpg)).app_data[(app_index) as usize] as *const Vec<u8>,
        ),
    );
    return true;
}
pub unsafe fn EncodeCOM_250(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    let mut com_index: u64 = ((*state).com_index.postfix_inc() as u64);
    if ((com_index) >= ((*jpg).com_data.len() as u64)) {
        return false;
    }
    (*state)
        .output_queue
        .push(brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8]));
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
            &(&(*jpg)).com_data[(com_index) as usize] as *const Vec<u8>,
        ),
    );
    return true;
}
pub unsafe fn EncodeInterMarkerData_251(
    jpg: *const brunsli_JPEGData,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> bool {
    let mut index: u64 = ((*state).data_index.postfix_inc() as u64);
    if ((index) >= ((*jpg).inter_marker_data.len() as u64)) {
        return false;
    }
    (*state).output_queue.push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
            &(&(*jpg)).inter_marker_data[(index) as usize] as *const Vec<u8>,
        ),
    );
    return true;
}
pub unsafe fn EncodeDCTBlockSequential_252(
    mut coeffs: *const i16,
    dc_huff: *const brunsli_HuffmanCodeTable,
    ac_huff: *const brunsli_HuffmanCodeTable,
    mut num_zero_runs: i32,
    mut last_dc_coeff: *mut i16,
    mut bw: *mut brunsli_internal_dec_BitWriter,
) -> bool {
    let mut temp2: i16 = 0_i16;
    let mut temp: i16 = 0_i16;
    temp2 = (*coeffs.offset((0) as isize));
    temp = (((temp2 as i32) - ((*last_dc_coeff) as i32)) as i16);
    (*last_dc_coeff) = temp2;
    temp2 = temp;
    if ((temp as i32) < (0)) {
        temp = (-(temp as i32) as i16);
        temp2.postfix_dec();
    }
    let mut dc_nbits: i32 = if ((temp as i32) == (0)) {
        0
    } else {
        ((unsafe {
            let _n: u32 = (temp as u32);
            Log2FloorNonZero_74(_n)
        }) + (1))
    };
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _nbits: i32 = (*dc_huff).depth[(dc_nbits) as usize];
        let _bits: u64 = ((*dc_huff).code[(dc_nbits) as usize] as u64);
        WriteBits_233(_bw, _nbits, _bits)
    });
    if ((dc_nbits) > (0)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = dc_nbits;
            let _bits: u64 =
                (((temp2 as u32) & (((1_u32) << (dc_nbits)).wrapping_sub(1_u32))) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
    }
    let mut r: i32 = 0;
    let mut k: i32 = 1;
    'loop_: while ((k) < (64)) {
        if ((({
            temp = (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize));
            temp
        }) as i32)
            == (0))
        {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        if ((temp as i32) < (0)) {
            temp = (-(temp as i32) as i16);
            temp2 = (!(temp as i32) as i16);
        } else {
            temp2 = temp;
        }
        'loop_: while ((r) > (15)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = (*ac_huff).depth[(240) as usize];
                let _bits: u64 = ((*ac_huff).code[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            r -= 16;
        }
        let mut ac_nbits: i32 = ((unsafe {
            let _n: u32 = (temp as u32);
            Log2FloorNonZero_74(_n)
        }) + (1));
        let mut symbol: i32 = (((r) << (4_u32)) + (ac_nbits));
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*ac_huff).depth[(symbol) as usize];
            let _bits: u64 = ((*ac_huff).code[(symbol) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = ac_nbits;
            let _bits: u64 = (((temp2 as i32) & (((1) << (ac_nbits)) - (1))) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        r = 0;
        k.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_zero_runs)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*ac_huff).depth[(240) as usize];
            let _bits: u64 = ((*ac_huff).code[(240) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        r -= 16;
        i.prefix_inc();
    }
    if ((r) > (0)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*ac_huff).depth[(0) as usize];
            let _bits: u64 = ((*ac_huff).code[(0) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
    }
    return true;
}
pub unsafe fn EncodeDCTBlockProgressive_253(
    mut coeffs: *const i16,
    dc_huff: *const brunsli_HuffmanCodeTable,
    ac_huff: *const brunsli_HuffmanCodeTable,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut num_zero_runs: i32,
    mut coding_state: *mut brunsli_internal_dec_DCTCodingState,
    mut last_dc_coeff: *mut i16,
    mut bw: *mut brunsli_internal_dec_BitWriter,
) -> bool {
    let mut eob_run_allowed: bool = ((Ss) > (0));
    let mut temp2: i16 = 0_i16;
    let mut temp: i16 = 0_i16;
    if ((Ss) == (0)) {
        temp2 = ((((*coeffs.offset((0) as isize)) as i32) >> (Al)) as i16);
        temp = (((temp2 as i32) - ((*last_dc_coeff) as i32)) as i16);
        (*last_dc_coeff) = temp2;
        temp2 = temp;
        if ((temp as i32) < (0)) {
            temp = (-(temp as i32) as i16);
            temp2.postfix_dec();
        }
        let mut nbits: i32 = if ((temp as i32) == (0)) {
            0
        } else {
            ((unsafe {
                let _n: u32 = (temp as u32);
                Log2FloorNonZero_74(_n)
            }) + (1))
        };
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*dc_huff).depth[(nbits) as usize];
            let _bits: u64 = ((*dc_huff).code[(nbits) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        if ((nbits) > (0)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = nbits;
                let _bits: u64 = (((temp2 as i32) & (((1) << (nbits)) - (1))) as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
        }
        Ss.prefix_inc();
    }
    if ((Ss) > (Se)) {
        return true;
    }
    let mut r: i32 = 0;
    let mut k: i32 = Ss;
    'loop_: while ((k) <= (Se)) {
        if ((({
            temp = (*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize));
            temp
        }) as i32)
            == (0))
        {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        if ((temp as i32) < (0)) {
            temp = (-(temp as i32) as i16);
            temp = ((temp as i32) >> Al) as i16;
            temp2 = (!(temp as i32) as i16);
        } else {
            temp = ((temp as i32) >> Al) as i16;
            temp2 = temp;
        }
        if ((temp as i32) == (0)) {
            r.postfix_inc();
            k.prefix_inc();
            continue 'loop_;
        }
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            Flush_238(_s, _bw)
        });
        'loop_: while ((r) > (15)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = (*ac_huff).depth[(240) as usize];
                let _bits: u64 = ((*ac_huff).code[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            r -= 16;
        }
        let mut nbits: i32 = ((unsafe {
            let _n: u32 = (temp as u32);
            Log2FloorNonZero_74(_n)
        }) + (1));
        let mut symbol: i32 = (((r) << (4_u32)) + (nbits));
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*ac_huff).depth[(symbol) as usize];
            let _bits: u64 = ((*ac_huff).code[(symbol) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = nbits;
            let _bits: u64 = (((temp2 as i32) & (((1) << (nbits)) - (1))) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        r = 0;
        k.prefix_inc();
    }
    if ((num_zero_runs) > (0)) {
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            Flush_238(_s, _bw)
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_zero_runs)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = (*ac_huff).depth[(240) as usize];
                let _bits: u64 = ((*ac_huff).code[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            r -= 16;
            i.prefix_inc();
        }
    }
    if ((r) > (0)) {
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
            let _ac_huff: *const brunsli_HuffmanCodeTable = (ac_huff);
            let _new_bits_array: *const i32 = std::ptr::null();
            let _new_bits_count: u64 = 0_u64;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            BufferEndOfBand_239(_s, _ac_huff, _new_bits_array, _new_bits_count, _bw)
        });
        if !eob_run_allowed {
            (unsafe {
                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                Flush_238(_s, _bw)
            });
        }
    }
    return true;
}
pub unsafe fn EncodeRefinementBits_254(
    mut coeffs: *const i16,
    ac_huff: *const brunsli_HuffmanCodeTable,
    mut Ss: i32,
    mut Se: i32,
    mut Al: i32,
    mut coding_state: *mut brunsli_internal_dec_DCTCodingState,
    mut bw: *mut brunsli_internal_dec_BitWriter,
) -> bool {
    let mut eob_run_allowed: bool = ((Ss) > (0));
    if ((Ss) == (0)) {
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = 1;
            let _bits: u64 = (((((*coeffs.offset((0) as isize)) as i32) >> (Al)) & (1)) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        Ss.prefix_inc();
    }
    if ((Ss) > (Se)) {
        return true;
    }
    let mut abs_values: [i32; 64] = [0_i32; 64];
    let mut eob: i32 = 0;
    let mut k: i32 = Ss;
    'loop_: while ((k) <= (Se)) {
        let abs_val: i16 =
            (((*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) as i32).abs() as i16);
        abs_values[(k) as usize] = ((abs_val as i32) >> (Al));
        if ((abs_values[(k) as usize]) == (1)) {
            eob = k;
        }
        k.postfix_inc();
    }
    let mut r: i32 = 0;
    let mut refinement_bits: [i32; 64] = [0_i32; 64];
    let mut refinement_bits_count: u64 = 0_u64;
    let mut k: i32 = Ss;
    'loop_: while ((k) <= (Se)) {
        if ((abs_values[(k) as usize]) == (0)) {
            r.postfix_inc();
            k.postfix_inc();
            continue 'loop_;
        }
        'loop_: while ((r) > (15)) && ((k) <= (eob)) {
            (unsafe {
                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                Flush_238(_s, _bw)
            });
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = (*ac_huff).depth[(240) as usize];
                let _bits: u64 = ((*ac_huff).code[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            r -= 16;
            let mut i: u64 = 0_u64;
            'loop_: while ((i) < (refinement_bits_count)) {
                (unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _nbits: i32 = 1;
                    let _bits: u64 = (refinement_bits[(i) as usize] as u64);
                    WriteBits_233(_bw, _nbits, _bits)
                });
                i.prefix_inc();
            }
            refinement_bits_count = 0_u64;
        }
        if ((abs_values[(k) as usize]) > (1)) {
            refinement_bits[(refinement_bits_count.postfix_inc()) as usize] =
                (((abs_values[(k) as usize] as u32) & (1_u32)) as i32);
            k.postfix_inc();
            continue 'loop_;
        }
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            Flush_238(_s, _bw)
        });
        let mut symbol: i32 = (((r) << (4_u32)) + (1));
        let mut new_non_zero_bit: i32 =
            if (((*coeffs.offset((kJPEGNaturalOrder_13[(k) as usize]) as isize)) as i32) < (0)) {
                0
            } else {
                1
            };
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = (*ac_huff).depth[(symbol) as usize];
            let _bits: u64 = ((*ac_huff).code[(symbol) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            let _nbits: i32 = 1;
            let _bits: u64 = (new_non_zero_bit as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < (refinement_bits_count)) {
            (unsafe {
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                let _nbits: i32 = 1;
                let _bits: u64 = (refinement_bits[(i) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            i.prefix_inc();
        }
        refinement_bits_count = 0_u64;
        r = 0;
        k.postfix_inc();
    }
    if ((r) > (0)) || (refinement_bits_count != 0) {
        if !(unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
            let _ac_huff: *const brunsli_HuffmanCodeTable = (ac_huff);
            let _new_bits_array: *const i32 = (refinement_bits.as_mut_ptr()).cast_const();
            let _new_bits_count: u64 = refinement_bits_count;
            let _bw: *mut brunsli_internal_dec_BitWriter = bw;
            BufferEndOfBand_239(_s, _ac_huff, _new_bits_array, _new_bits_count, _bw)
        }) {
            return false;
        }
        if !eob_run_allowed {
            (unsafe {
                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                Flush_238(_s, _bw)
            });
        }
    }
    return true;
}
pub unsafe fn DoEncodeScan_255(
    jpg: *const brunsli_JPEGData,
    parsing_state: *const brunsli_internal_dec_State,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: *const brunsli_JPEGScanInfo =
        &(&(*jpg)).scan_info[((*state).scan_index as u64) as usize] as *const brunsli_JPEGScanInfo;
    let ss: *mut brunsli_internal_dec_EncodeScanState =
        &mut (*state).scan_state as *mut brunsli_internal_dec_EncodeScanState;
    let restart_interval: i32 = if (*state).seen_dri_marker {
        (*jpg).restart_interval
    } else {
        0
    };
    if (((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)) {
        if !(unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _scan_info: *const brunsli_JPEGScanInfo = scan_info;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter =
                (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
            let _output_queue: *mut Vec<brunsli_internal_dec_OutputChunk> =
                (&mut (*state).output_queue as *mut Vec<brunsli_internal_dec_OutputChunk>);
            BitWriterInit_228(_bw, _output_queue)
        });
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState =
                (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
            DCTCodingStateInit_237(_s)
        });
        (*ss).restarts_to_go = restart_interval;
        (*ss).next_restart_marker = 0;
        (*ss).block_scan_index = 0;
        (*ss).extra_zero_runs_pos = 0_u64;
        (*ss).next_extra_zero_run_index = (unsafe {
            (|| {
                if (((*ss).extra_zero_runs_pos) < ((*scan_info).extra_zero_runs.len() as u64)) {
                    return (&(*scan_info)).extra_zero_runs[((*ss).extra_zero_runs_pos) as usize]
                        .block_idx;
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).next_reset_point_pos = 0_u64;
        (*ss).next_reset_point = (unsafe {
            (|| {
                if (((*ss).next_reset_point_pos) < ((*scan_info).reset_points.len() as u64)) {
                    return (&(*scan_info)).reset_points
                        [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).mcu_y = 0;
        {
            let byte_0 =
                ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
        };
        (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::BODY).clone();
    }
    let mut bw: *mut brunsli_internal_dec_BitWriter =
        (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
    let mut coding_state: *mut brunsli_internal_dec_DCTCodingState =
        (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
    if !(((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)) {
        (unsafe {
            let _f: *const u8 = b"jpeg_data_writer.cc\0".as_ptr();
            let _l: i32 = 741;
            let _fn: *const u8 = b"DoEncodeScan\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: bool = (((*scan_info).num_components) > (1_u64));
    let base_component: *const brunsli_JPEGComponent = &(&(*jpg)).components
        [((&(*scan_info)).components[(0_u64) as usize].comp_idx as u64) as usize]
        as *const brunsli_JPEGComponent;
    let h_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).h_samp_factor
    };
    let v_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).v_samp_factor
    };
    let MCUs_per_row: i32 = (unsafe {
        let _a: i32 = (((*jpg).width) * (h_group));
        let _b: i32 = ((8) * ((*jpg).max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let MCU_rows: i32 = (unsafe {
        let _a: i32 = (((*jpg).height) * (v_group));
        let _b: i32 = ((8) * ((*jpg).max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let is_progressive: bool = (*state).is_progressive;
    let Al: i32 = if is_progressive { (*scan_info).Al } else { 0 };
    let Ss: i32 = if is_progressive { (*scan_info).Ss } else { 0 };
    let Se: i32 = if is_progressive { (*scan_info).Se } else { 63 };
    let want_ac: bool = (((Ss) != (0)) || ((Se) != (0)));
    let complete_ac: bool = (((*parsing_state).stage) == (brunsli_internal_dec_Stage::DONE));
    let has_ac: bool = (complete_ac)
        || (unsafe {
            let _state: *const brunsli_internal_dec_State = (parsing_state);
            let _tag: u32 = (kBrunsliACDataTag_37 as u32);
            HasSection_194(_state, _tag)
        });
    if (want_ac) && (!has_ac) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    let complete_dc: bool = has_ac;
    let complete: bool = if want_ac { complete_ac } else { complete_dc };
    let last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        (((*(*(std::ptr::addr_of!((*parsing_state).internal).cast_mut()))
            .as_deref_mut()
            .unwrap())
        .ac_dc
        .next_mcu_y)
            * (v_group))
    };
    'loop_: while (((*ss).mcu_y) < (last_mcu_y)) {
        let mut mcu_x: i32 = 0;
        'loop_: while ((mcu_x) < (MCUs_per_row)) {
            if ((restart_interval) > (0)) && (((*ss).restarts_to_go) == (0)) {
                (unsafe {
                    let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    Flush_238(_s, _bw)
                });
                if !(unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
                    let _pad_bits_end: *const i32 = (*state).pad_bits_end;
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                (unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _marker: i32 = ((208) + ((*ss).next_restart_marker));
                    EmitMarker_234(_bw, _marker)
                });
                (*ss).next_restart_marker += 1;
                (*ss).next_restart_marker &= 7;
                (*ss).restarts_to_go = restart_interval;
                {
                    let byte_0 = ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16
                        as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
                };
            }
            let mut i: u64 = 0_u64;
            'loop_: while ((i) < ((*scan_info).num_components)) {
                let si: *const brunsli_JPEGComponentScanInfo = &(&(*scan_info)).components
                    [(i) as usize]
                    as *const brunsli_JPEGComponentScanInfo;
                let c: *const brunsli_JPEGComponent = &(&(*jpg)).components
                    [((*si).comp_idx as u64) as usize]
                    as *const brunsli_JPEGComponent;
                let dc_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).dc_huff_table
                    [((*si).dc_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let ac_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).ac_huff_table
                    [((*si).ac_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let mut n_blocks_y: i32 = if is_interleaved {
                    (*c).v_samp_factor
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    (*c).h_samp_factor
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while ((iy) < (n_blocks_y)) {
                    let mut ix: i32 = 0;
                    'loop_: while ((ix) < (n_blocks_x)) {
                        let mut block_y: i32 = ((((*ss).mcu_y) * (n_blocks_y)) + (iy));
                        let mut block_x: i32 = (((mcu_x) * (n_blocks_x)) + (ix));
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul((*c).width_in_blocks))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if (((*ss).block_scan_index) == ((*ss).next_reset_point)) {
                            (unsafe {
                                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                Flush_238(_s, _bw)
                            });
                            (*ss).next_reset_point = (unsafe {
                                (|| {
                                    if (((*ss).next_reset_point_pos)
                                        < ((*scan_info).reset_points.len() as u64))
                                    {
                                        return (&(*scan_info)).reset_points
                                            [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut num_zero_runs: i32 = 0;
                        if (((*ss).block_scan_index) == ((*ss).next_extra_zero_run_index)) {
                            num_zero_runs = (&(*scan_info)).extra_zero_runs
                                [((*ss).extra_zero_runs_pos) as usize]
                                .num_extra_zero_runs;
                            (*ss).extra_zero_runs_pos.prefix_inc();
                            (*ss).next_extra_zero_run_index = (unsafe {
                                (|| {
                                    if (((*ss).extra_zero_runs_pos)
                                        < ((*scan_info).extra_zero_runs.len() as u64))
                                    {
                                        return (&(*scan_info)).extra_zero_runs
                                            [((*ss).extra_zero_runs_pos) as usize]
                                            .block_idx;
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut coeffs: *const i16 =
                            (&(&(*c)).coeffs[(((block_idx) << (6)) as u64) as usize] as *const i16);
                        let mut ok: bool = false;
                        if ((0) == (0)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            })
                            .clone();
                        } else if ((0) == (1)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
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
                            })
                            .clone();
                        } else {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            })
                            .clone();
                        }
                        if !ok {
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*ss).block_scan_index.prefix_inc();
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            (*ss).restarts_to_go.prefix_dec();
            mcu_x.prefix_inc();
        }
        (*ss).mcu_y.prefix_inc();
    }
    if (((*ss).mcu_y) < (MCU_rows)) {
        if !(*bw).healthy {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    (unsafe {
        let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        Flush_238(_s, _bw)
    });
    if !(unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
        let _pad_bits_end: *const i32 = (*state).pad_bits_end;
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        BitWriterFinish_236(_bw)
    });
    (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::HEAD).clone();
    (*state).scan_index.postfix_inc();
    if !(*bw).healthy {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub unsafe fn DoEncodeScan_256(
    jpg: *const brunsli_JPEGData,
    parsing_state: *const brunsli_internal_dec_State,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: *const brunsli_JPEGScanInfo =
        &(&(*jpg)).scan_info[((*state).scan_index as u64) as usize] as *const brunsli_JPEGScanInfo;
    let ss: *mut brunsli_internal_dec_EncodeScanState =
        &mut (*state).scan_state as *mut brunsli_internal_dec_EncodeScanState;
    let restart_interval: i32 = if (*state).seen_dri_marker {
        (*jpg).restart_interval
    } else {
        0
    };
    if (((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)) {
        if !(unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _scan_info: *const brunsli_JPEGScanInfo = scan_info;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter =
                (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
            let _output_queue: *mut Vec<brunsli_internal_dec_OutputChunk> =
                (&mut (*state).output_queue as *mut Vec<brunsli_internal_dec_OutputChunk>);
            BitWriterInit_228(_bw, _output_queue)
        });
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState =
                (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
            DCTCodingStateInit_237(_s)
        });
        (*ss).restarts_to_go = restart_interval;
        (*ss).next_restart_marker = 0;
        (*ss).block_scan_index = 0;
        (*ss).extra_zero_runs_pos = 0_u64;
        (*ss).next_extra_zero_run_index = (unsafe {
            (|| {
                if (((*ss).extra_zero_runs_pos) < ((*scan_info).extra_zero_runs.len() as u64)) {
                    return (&(*scan_info)).extra_zero_runs[((*ss).extra_zero_runs_pos) as usize]
                        .block_idx;
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).next_reset_point_pos = 0_u64;
        (*ss).next_reset_point = (unsafe {
            (|| {
                if (((*ss).next_reset_point_pos) < ((*scan_info).reset_points.len() as u64)) {
                    return (&(*scan_info)).reset_points
                        [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).mcu_y = 0;
        {
            let byte_0 =
                ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
        };
        (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::BODY).clone();
    }
    let mut bw: *mut brunsli_internal_dec_BitWriter =
        (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
    let mut coding_state: *mut brunsli_internal_dec_DCTCodingState =
        (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
    if !(((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)) {
        (unsafe {
            let _f: *const u8 = b"jpeg_data_writer.cc\0".as_ptr();
            let _l: i32 = 741;
            let _fn: *const u8 = b"DoEncodeScan\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: bool = (((*scan_info).num_components) > (1_u64));
    let base_component: *const brunsli_JPEGComponent = &(&(*jpg)).components
        [((&(*scan_info)).components[(0_u64) as usize].comp_idx as u64) as usize]
        as *const brunsli_JPEGComponent;
    let h_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).h_samp_factor
    };
    let v_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).v_samp_factor
    };
    let MCUs_per_row: i32 = (unsafe {
        let _a: i32 = (((*jpg).width) * (h_group));
        let _b: i32 = ((8) * ((*jpg).max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let MCU_rows: i32 = (unsafe {
        let _a: i32 = (((*jpg).height) * (v_group));
        let _b: i32 = ((8) * ((*jpg).max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let is_progressive: bool = (*state).is_progressive;
    let Al: i32 = if is_progressive { (*scan_info).Al } else { 0 };
    let Ss: i32 = if is_progressive { (*scan_info).Ss } else { 0 };
    let Se: i32 = if is_progressive { (*scan_info).Se } else { 63 };
    let want_ac: bool = (((Ss) != (0)) || ((Se) != (0)));
    let complete_ac: bool = (((*parsing_state).stage) == (brunsli_internal_dec_Stage::DONE));
    let has_ac: bool = (complete_ac)
        || (unsafe {
            let _state: *const brunsli_internal_dec_State = (parsing_state);
            let _tag: u32 = (kBrunsliACDataTag_37 as u32);
            HasSection_194(_state, _tag)
        });
    if (want_ac) && (!has_ac) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    let complete_dc: bool = has_ac;
    let complete: bool = if want_ac { complete_ac } else { complete_dc };
    let last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        (((*(*(std::ptr::addr_of!((*parsing_state).internal).cast_mut()))
            .as_deref_mut()
            .unwrap())
        .ac_dc
        .next_mcu_y)
            * (v_group))
    };
    'loop_: while (((*ss).mcu_y) < (last_mcu_y)) {
        let mut mcu_x: i32 = 0;
        'loop_: while ((mcu_x) < (MCUs_per_row)) {
            if ((restart_interval) > (0)) && (((*ss).restarts_to_go) == (0)) {
                (unsafe {
                    let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    Flush_238(_s, _bw)
                });
                if !(unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
                    let _pad_bits_end: *const i32 = (*state).pad_bits_end;
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                (unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _marker: i32 = ((208) + ((*ss).next_restart_marker));
                    EmitMarker_234(_bw, _marker)
                });
                (*ss).next_restart_marker += 1;
                (*ss).next_restart_marker &= 7;
                (*ss).restarts_to_go = restart_interval;
                {
                    let byte_0 = ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16
                        as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
                };
            }
            let mut i: u64 = 0_u64;
            'loop_: while ((i) < ((*scan_info).num_components)) {
                let si: *const brunsli_JPEGComponentScanInfo = &(&(*scan_info)).components
                    [(i) as usize]
                    as *const brunsli_JPEGComponentScanInfo;
                let c: *const brunsli_JPEGComponent = &(&(*jpg)).components
                    [((*si).comp_idx as u64) as usize]
                    as *const brunsli_JPEGComponent;
                let dc_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).dc_huff_table
                    [((*si).dc_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let ac_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).ac_huff_table
                    [((*si).ac_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let mut n_blocks_y: i32 = if is_interleaved {
                    (*c).v_samp_factor
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    (*c).h_samp_factor
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while ((iy) < (n_blocks_y)) {
                    let mut ix: i32 = 0;
                    'loop_: while ((ix) < (n_blocks_x)) {
                        let mut block_y: i32 = ((((*ss).mcu_y) * (n_blocks_y)) + (iy));
                        let mut block_x: i32 = (((mcu_x) * (n_blocks_x)) + (ix));
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul((*c).width_in_blocks))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if (((*ss).block_scan_index) == ((*ss).next_reset_point)) {
                            (unsafe {
                                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                Flush_238(_s, _bw)
                            });
                            (*ss).next_reset_point = (unsafe {
                                (|| {
                                    if (((*ss).next_reset_point_pos)
                                        < ((*scan_info).reset_points.len() as u64))
                                    {
                                        return (&(*scan_info)).reset_points
                                            [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut num_zero_runs: i32 = 0;
                        if (((*ss).block_scan_index) == ((*ss).next_extra_zero_run_index)) {
                            num_zero_runs = (&(*scan_info)).extra_zero_runs
                                [((*ss).extra_zero_runs_pos) as usize]
                                .num_extra_zero_runs;
                            (*ss).extra_zero_runs_pos.prefix_inc();
                            (*ss).next_extra_zero_run_index = (unsafe {
                                (|| {
                                    if (((*ss).extra_zero_runs_pos)
                                        < ((*scan_info).extra_zero_runs.len() as u64))
                                    {
                                        return (&(*scan_info)).extra_zero_runs
                                            [((*ss).extra_zero_runs_pos) as usize]
                                            .block_idx;
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut coeffs: *const i16 =
                            (&(&(*c)).coeffs[(((block_idx) << (6)) as u64) as usize] as *const i16);
                        let mut ok: bool = false;
                        if ((1) == (0)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            })
                            .clone();
                        } else if ((1) == (1)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
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
                            })
                            .clone();
                        } else {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            })
                            .clone();
                        }
                        if !ok {
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*ss).block_scan_index.prefix_inc();
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            (*ss).restarts_to_go.prefix_dec();
            mcu_x.prefix_inc();
        }
        (*ss).mcu_y.prefix_inc();
    }
    if (((*ss).mcu_y) < (MCU_rows)) {
        if !(*bw).healthy {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    (unsafe {
        let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        Flush_238(_s, _bw)
    });
    if !(unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
        let _pad_bits_end: *const i32 = (*state).pad_bits_end;
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        BitWriterFinish_236(_bw)
    });
    (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::HEAD).clone();
    (*state).scan_index.postfix_inc();
    if !(*bw).healthy {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub unsafe fn DoEncodeScan_257(
    jpg: *const brunsli_JPEGData,
    parsing_state: *const brunsli_internal_dec_State,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: *const brunsli_JPEGScanInfo =
        &(&(*jpg)).scan_info[((*state).scan_index as u64) as usize] as *const brunsli_JPEGScanInfo;
    let ss: *mut brunsli_internal_dec_EncodeScanState =
        &mut (*state).scan_state as *mut brunsli_internal_dec_EncodeScanState;
    let restart_interval: i32 = if (*state).seen_dri_marker {
        (*jpg).restart_interval
    } else {
        0
    };
    if (((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)) {
        if !(unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _scan_info: *const brunsli_JPEGScanInfo = scan_info;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        (unsafe {
            let _bw: *mut brunsli_internal_dec_BitWriter =
                (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
            let _output_queue: *mut Vec<brunsli_internal_dec_OutputChunk> =
                (&mut (*state).output_queue as *mut Vec<brunsli_internal_dec_OutputChunk>);
            BitWriterInit_228(_bw, _output_queue)
        });
        (unsafe {
            let _s: *mut brunsli_internal_dec_DCTCodingState =
                (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
            DCTCodingStateInit_237(_s)
        });
        (*ss).restarts_to_go = restart_interval;
        (*ss).next_restart_marker = 0;
        (*ss).block_scan_index = 0;
        (*ss).extra_zero_runs_pos = 0_u64;
        (*ss).next_extra_zero_run_index = (unsafe {
            (|| {
                if (((*ss).extra_zero_runs_pos) < ((*scan_info).extra_zero_runs.len() as u64)) {
                    return (&(*scan_info)).extra_zero_runs[((*ss).extra_zero_runs_pos) as usize]
                        .block_idx;
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).next_reset_point_pos = 0_u64;
        (*ss).next_reset_point = (unsafe {
            (|| {
                if (((*ss).next_reset_point_pos) < ((*scan_info).reset_points.len() as u64)) {
                    return (&(*scan_info)).reset_points
                        [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                } else {
                    return -1_i32;
                }
                panic!("ub: non-void function does not return a value")
            })()
        });
        (*ss).mcu_y = 0;
        {
            let byte_0 =
                ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
        };
        (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::BODY).clone();
    }
    let mut bw: *mut brunsli_internal_dec_BitWriter =
        (&mut (*ss).bw as *mut brunsli_internal_dec_BitWriter);
    let mut coding_state: *mut brunsli_internal_dec_DCTCodingState =
        (&mut (*ss).coding_state as *mut brunsli_internal_dec_DCTCodingState);
    if !(((*ss).stage as i32) == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)) {
        (unsafe {
            let _f: *const u8 = b"jpeg_data_writer.cc\0".as_ptr();
            let _l: i32 = 741;
            let _fn: *const u8 = b"DoEncodeScan\0".as_ptr();
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: bool = (((*scan_info).num_components) > (1_u64));
    let base_component: *const brunsli_JPEGComponent = &(&(*jpg)).components
        [((&(*scan_info)).components[(0_u64) as usize].comp_idx as u64) as usize]
        as *const brunsli_JPEGComponent;
    let h_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).h_samp_factor
    };
    let v_group: i32 = if is_interleaved {
        1
    } else {
        (*base_component).v_samp_factor
    };
    let MCUs_per_row: i32 = (unsafe {
        let _a: i32 = (((*jpg).width) * (h_group));
        let _b: i32 = ((8) * ((*jpg).max_h_samp_factor));
        DivCeil_226(_a, _b)
    });
    let MCU_rows: i32 = (unsafe {
        let _a: i32 = (((*jpg).height) * (v_group));
        let _b: i32 = ((8) * ((*jpg).max_v_samp_factor));
        DivCeil_226(_a, _b)
    });
    let is_progressive: bool = (*state).is_progressive;
    let Al: i32 = if is_progressive { (*scan_info).Al } else { 0 };
    let Ss: i32 = if is_progressive { (*scan_info).Ss } else { 0 };
    let Se: i32 = if is_progressive { (*scan_info).Se } else { 63 };
    let want_ac: bool = (((Ss) != (0)) || ((Se) != (0)));
    let complete_ac: bool = (((*parsing_state).stage) == (brunsli_internal_dec_Stage::DONE));
    let has_ac: bool = (complete_ac)
        || (unsafe {
            let _state: *const brunsli_internal_dec_State = (parsing_state);
            let _tag: u32 = (kBrunsliACDataTag_37 as u32);
            HasSection_194(_state, _tag)
        });
    if (want_ac) && (!has_ac) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    let complete_dc: bool = has_ac;
    let complete: bool = if want_ac { complete_ac } else { complete_dc };
    let last_mcu_y: i32 = if complete {
        MCU_rows
    } else {
        (((*(*(std::ptr::addr_of!((*parsing_state).internal).cast_mut()))
            .as_deref_mut()
            .unwrap())
        .ac_dc
        .next_mcu_y)
            * (v_group))
    };
    'loop_: while (((*ss).mcu_y) < (last_mcu_y)) {
        let mut mcu_x: i32 = 0;
        'loop_: while ((mcu_x) < (MCUs_per_row)) {
            if ((restart_interval) > (0)) && (((*ss).restarts_to_go) == (0)) {
                (unsafe {
                    let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    Flush_238(_s, _bw)
                });
                if !(unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
                    let _pad_bits_end: *const i32 = (*state).pad_bits_end;
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                (unsafe {
                    let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                    let _marker: i32 = ((208) + ((*ss).next_restart_marker));
                    EmitMarker_234(_bw, _marker)
                });
                (*ss).next_restart_marker += 1;
                (*ss).next_restart_marker &= 7;
                (*ss).restarts_to_go = restart_interval;
                {
                    let byte_0 = ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16
                        as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..::std::mem::size_of::<[i16; 4]>() as u64 {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    ((*ss).last_dc_coeff.as_mut_ptr() as *mut i16 as *mut ::libc::c_void)
                };
            }
            let mut i: u64 = 0_u64;
            'loop_: while ((i) < ((*scan_info).num_components)) {
                let si: *const brunsli_JPEGComponentScanInfo = &(&(*scan_info)).components
                    [(i) as usize]
                    as *const brunsli_JPEGComponentScanInfo;
                let c: *const brunsli_JPEGComponent = &(&(*jpg)).components
                    [((*si).comp_idx as u64) as usize]
                    as *const brunsli_JPEGComponent;
                let dc_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).dc_huff_table
                    [((*si).dc_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let ac_huff: *const brunsli_HuffmanCodeTable = &(&mut (*state)).ac_huff_table
                    [((*si).ac_tbl_idx as u64) as usize]
                    as *const brunsli_HuffmanCodeTable;
                let mut n_blocks_y: i32 = if is_interleaved {
                    (*c).v_samp_factor
                } else {
                    1
                };
                let mut n_blocks_x: i32 = if is_interleaved {
                    (*c).h_samp_factor
                } else {
                    1
                };
                let mut iy: i32 = 0;
                'loop_: while ((iy) < (n_blocks_y)) {
                    let mut ix: i32 = 0;
                    'loop_: while ((ix) < (n_blocks_x)) {
                        let mut block_y: i32 = ((((*ss).mcu_y) * (n_blocks_y)) + (iy));
                        let mut block_x: i32 = (((mcu_x) * (n_blocks_x)) + (ix));
                        let mut block_idx: i32 = ((((block_y as u32)
                            .wrapping_mul((*c).width_in_blocks))
                        .wrapping_add((block_x as u32)))
                            as i32);
                        if (((*ss).block_scan_index) == ((*ss).next_reset_point)) {
                            (unsafe {
                                let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                Flush_238(_s, _bw)
                            });
                            (*ss).next_reset_point = (unsafe {
                                (|| {
                                    if (((*ss).next_reset_point_pos)
                                        < ((*scan_info).reset_points.len() as u64))
                                    {
                                        return (&(*scan_info)).reset_points
                                            [((*ss).next_reset_point_pos.postfix_inc()) as usize];
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut num_zero_runs: i32 = 0;
                        if (((*ss).block_scan_index) == ((*ss).next_extra_zero_run_index)) {
                            num_zero_runs = (&(*scan_info)).extra_zero_runs
                                [((*ss).extra_zero_runs_pos) as usize]
                                .num_extra_zero_runs;
                            (*ss).extra_zero_runs_pos.prefix_inc();
                            (*ss).next_extra_zero_run_index = (unsafe {
                                (|| {
                                    if (((*ss).extra_zero_runs_pos)
                                        < ((*scan_info).extra_zero_runs.len() as u64))
                                    {
                                        return (&(*scan_info)).extra_zero_runs
                                            [((*ss).extra_zero_runs_pos) as usize]
                                            .block_idx;
                                    } else {
                                        return -1_i32;
                                    }
                                    panic!("ub: non-void function does not return a value")
                                })()
                            });
                        }
                        let mut coeffs: *const i16 =
                            (&(&(*c)).coeffs[(((block_idx) << (6)) as u64) as usize] as *const i16);
                        let mut ok: bool = false;
                        if ((2) == (0)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeDCTBlockSequential_252(
                                    _coeffs,
                                    _dc_huff,
                                    _ac_huff,
                                    _num_zero_runs,
                                    _last_dc_coeff,
                                    _bw,
                                )
                            })
                            .clone();
                        } else if ((2) == (1)) {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _dc_huff: *const brunsli_HuffmanCodeTable = dc_huff;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _num_zero_runs: i32 = num_zero_runs;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _last_dc_coeff: *mut i16 = (*ss)
                                    .last_dc_coeff
                                    .as_mut_ptr()
                                    .offset(((*si).comp_idx as i32) as isize);
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
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
                            })
                            .clone();
                        } else {
                            ok = (unsafe {
                                let _coeffs: *const i16 = coeffs;
                                let _ac_huff: *const brunsli_HuffmanCodeTable = ac_huff;
                                let _Ss: i32 = Ss;
                                let _Se: i32 = Se;
                                let _Al: i32 = Al;
                                let _coding_state: *mut brunsli_internal_dec_DCTCodingState =
                                    coding_state;
                                let _bw: *mut brunsli_internal_dec_BitWriter = bw;
                                EncodeRefinementBits_254(
                                    _coeffs,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _coding_state,
                                    _bw,
                                )
                            })
                            .clone();
                        }
                        if !ok {
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*ss).block_scan_index.prefix_inc();
                        ix.prefix_inc();
                    }
                    iy.prefix_inc();
                }
                i.prefix_inc();
            }
            (*ss).restarts_to_go.prefix_dec();
            mcu_x.prefix_inc();
        }
        (*ss).mcu_y.prefix_inc();
    }
    if (((*ss).mcu_y) < (MCU_rows)) {
        if !(*bw).healthy {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    (unsafe {
        let _s: *mut brunsli_internal_dec_DCTCodingState = coding_state;
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        Flush_238(_s, _bw)
    });
    if !(unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        let _pad_bits: *mut *const i32 = (&mut (*state).pad_bits as *mut *const i32);
        let _pad_bits_end: *const i32 = (*state).pad_bits_end;
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    (unsafe {
        let _bw: *mut brunsli_internal_dec_BitWriter = bw;
        BitWriterFinish_236(_bw)
    });
    (*ss).stage = (brunsli_internal_dec_EncodeScanState_Stage::HEAD).clone();
    (*state).scan_index.postfix_inc();
    if !(*bw).healthy {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub unsafe fn EncodeScan_258(
    jpg: *const brunsli_JPEGData,
    parsing_state: *const brunsli_internal_dec_State,
    mut state: *mut brunsli_internal_dec_SerializationState,
) -> brunsli_internal_dec_SerializationStatus {
    let scan_info: *const brunsli_JPEGScanInfo =
        &(&(*jpg)).scan_info[((*state).scan_index as u64) as usize] as *const brunsli_JPEGScanInfo;
    let is_progressive: bool = (*state).is_progressive;
    let Al: i32 = if is_progressive { (*scan_info).Al } else { 0 };
    let Ah: i32 = if is_progressive { (*scan_info).Ah } else { 0 };
    let Ss: i32 = if is_progressive { (*scan_info).Ss } else { 0 };
    let Se: i32 = if is_progressive { (*scan_info).Se } else { 63 };
    let need_sequential: bool = (!is_progressive)
        || (((((Ah) == (0)) && ((Al) == (0))) && ((Ss) == (0))) && ((Se) == (63)));
    if need_sequential {
        return (unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _parsing_state: *const brunsli_internal_dec_State = parsing_state;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            DoEncodeScan_255(_jpg, _parsing_state, _state)
        });
    } else if ((Ah) == (0)) {
        return (unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _parsing_state: *const brunsli_internal_dec_State = parsing_state;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            DoEncodeScan_256(_jpg, _parsing_state, _state)
        });
    } else {
        return (unsafe {
            let _jpg: *const brunsli_JPEGData = jpg;
            let _parsing_state: *const brunsli_internal_dec_State = parsing_state;
            let _state: *mut brunsli_internal_dec_SerializationState = state;
            DoEncodeScan_257(_jpg, _parsing_state, _state)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn SerializeSection_259(
    mut marker: u8,
    parsing_state: *const brunsli_internal_dec_State,
    mut state: *mut brunsli_internal_dec_SerializationState,
    jpg: *const brunsli_JPEGData,
) -> brunsli_internal_dec_SerializationStatus {
    'switch: {
        let __match_cond = (marker as i32);
        match __match_cond {
            v if v == 192 || v == 193 || v == 194 || v == 201 || v == 202 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _marker: u8 = marker;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeSOF_243(_jpg, _marker, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 196 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeDHT_245(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 208
                || v == 209
                || v == 210
                || v == 211
                || v == 212
                || v == 213
                || v == 214
                || v == 215 =>
            {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _marker: u8 = marker;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeRestart_248(_marker, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 217 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeEOI_242(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 218 => {
                return (unsafe {
                    let _jpg: *const brunsli_JPEGData = jpg;
                    let _parsing_state: *const brunsli_internal_dec_State = parsing_state;
                    let _state: *mut brunsli_internal_dec_SerializationState = state;
                    EncodeScan_258(_jpg, _parsing_state, _state)
                });
            }
            v if v == 219 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeDQT_246(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 221 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeDRI_247(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 224
                || v == 225
                || v == 226
                || v == 227
                || v == 228
                || v == 229
                || v == 230
                || v == 231
                || v == 232
                || v == 233
                || v == 234
                || v == 235
                || v == 236
                || v == 237
                || v == 238
                || v == 239 =>
            {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _marker: u8 = marker;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeAPP_249(_jpg, _marker, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 254 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeCOM_250(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            v if v == 255 => {
                return (unsafe {
                    let _result: bool = (unsafe {
                        let _jpg: *const brunsli_JPEGData = jpg;
                        let _state: *mut brunsli_internal_dec_SerializationState = state;
                        EncodeInterMarkerData_251(_jpg, _state)
                    });
                    (|result: bool| {
                        return if result {
                            brunsli_internal_dec_SerializationStatus::DONE
                        } else {
                            brunsli_internal_dec_SerializationStatus::ERROR
                        };
                    })(_result)
                });
            }
            _ => {
                return brunsli_internal_dec_SerializationStatus::ERROR;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn PushOutput_260(
    mut in_: *mut Vec<brunsli_internal_dec_OutputChunk>,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
) {
    'loop_: while ((*available_out) > (0_u64)) {
        if (*(in_).cast_const()).is_empty() {
            return;
        }
        let chunk: *mut brunsli_internal_dec_OutputChunk = ((*in_).first_mut().unwrap());
        let mut to_copy: u64 = (*if *available_out <= *&(*chunk).len {
            (available_out) as *const _
        } else {
            (&(*chunk).len) as *const _
        });
        if ((to_copy) > (0_u64)) {
            {
                if to_copy != 0 {
                    ::std::ptr::copy_nonoverlapping(
                        ((*chunk).next as *const u8 as *const ::libc::c_void),
                        ((*next_out) as *mut u8 as *mut ::libc::c_void),
                        to_copy as usize,
                    )
                }
                ((*next_out) as *mut u8 as *mut ::libc::c_void)
            };
            (*next_out) = (*next_out).wrapping_add(to_copy as usize);
            (*available_out) = (*available_out).wrapping_sub(to_copy);
            (*chunk).next = ((*chunk).next).wrapping_add(to_copy as usize);
            (*chunk).len = ((*chunk).len).wrapping_sub(to_copy);
        }
        if (((*chunk).len) == (0_u64)) {
            (*in_).remove(0);
        }
    }
}
pub unsafe fn WriteJpeg_261(jpg: *const brunsli_JPEGData, mut out: brunsli_JPEGOutput) -> bool {
    let mut state: brunsli_internal_dec_State =
        brunsli_internal_dec_State::brunsli_internal_dec_State();
    state.stage = (brunsli_internal_dec_Stage::DONE).clone();
    let mut buffer: Vec<u8> = (0..(16384_u64) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    'loop_: while true {
        let mut next_out: *mut u8 = buffer.as_mut_ptr();
        let mut available_out: u64 = buffer.len() as u64;
        let mut status: brunsli_internal_dec_SerializationStatus = (unsafe {
            let _state: *mut brunsli_internal_dec_State =
                (&mut state as *mut brunsli_internal_dec_State);
            let _jpg: *const brunsli_JPEGData = jpg;
            let _available_out: *mut u64 = (&mut available_out as *mut u64);
            let _next_out: *mut *mut u8 = (&mut next_out as *mut *mut u8);
            SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
        });
        if ((status) != (brunsli_internal_dec_SerializationStatus::DONE))
            && ((status) != (brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT))
        {
            return false;
        }
        let mut to_write: u64 = (buffer.len() as u64).wrapping_sub(available_out);
        if !(unsafe {
            let _buf: *const u8 = (buffer.as_mut_ptr()).cast_const();
            let _len: u64 = to_write;
            out.Write(_buf, _len)
        }) {
            return false;
        }
        if ((status) == (brunsli_internal_dec_SerializationStatus::DONE)) {
            return true;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn SerializeJpeg_206(
    mut state: *mut brunsli_internal_dec_State,
    jpg: *const brunsli_JPEGData,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
) -> brunsli_internal_dec_SerializationStatus {
    let ss: *mut brunsli_internal_dec_SerializationState =
        &mut (*(*state).internal.as_deref_mut().unwrap()).serialization
            as *mut brunsli_internal_dec_SerializationState;
    (unsafe {
        (|| {
            if (((*ss).stage as i32)
                != (brunsli_internal_dec_SerializationState_Stage::ERROR as i32))
            {
                (unsafe {
                    let _in: *mut Vec<brunsli_internal_dec_OutputChunk> =
                        (&mut (*ss).output_queue as *mut Vec<brunsli_internal_dec_OutputChunk>);
                    let _available_out: *mut u64 = available_out;
                    let _next_out: *mut *mut u8 = next_out;
                    PushOutput_260(_in, _available_out, _next_out)
                });
            }
        })()
    });
    'loop_: while true {
        switch!(match ((*ss).stage as i32) {
            v if v == (brunsli_internal_dec_SerializationState_Stage::INIT as i32) => {
                let mut can_start_serialization: bool =
                    (((*state).stage) == (brunsli_internal_dec_Stage::DONE));
                if (unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliDCDataTag_36 as u32);
                    HasSection_194(_state, _tag)
                }) || (unsafe {
                    let _state: *const brunsli_internal_dec_State = (state).cast_const();
                    let _tag: u32 = (kBrunsliACDataTag_37 as u32);
                    HasSection_194(_state, _tag)
                }) {
                    can_start_serialization = true;
                }
                if !can_start_serialization {
                    return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
                }
                if (((*jpg).version) == (kFallbackVersion_2)) {
                    if ((*jpg).original_jpg).is_null() {
                        (*ss).stage =
                            (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                        break;
                    }
                    (*ss).output_queue.push(
                        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk1(
                            (*jpg).original_jpg,
                            (*jpg).original_jpg_size,
                        ),
                    );
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::DONE).clone();
                    break;
                }
                if ((((*jpg).version) & (1)) == (kFallbackVersion_2)) {
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                    break;
                }
                if (*jpg).marker_order.is_empty() {
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                    break;
                }
                {
                    let __a0 = (kMaxHuffmanTables_6 as u64) as usize;
                    (*ss)
                        .dc_huff_table
                        .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                };
                {
                    let __a0 = (kMaxHuffmanTables_6 as u64) as usize;
                    (*ss)
                        .ac_huff_table
                        .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                };
                if (*jpg).has_zero_padding_bit {
                    (*ss).pad_bits = (*jpg).padding_bits.as_ptr();
                    (*ss).pad_bits_end = (*ss)
                        .pad_bits
                        .offset(((*jpg).padding_bits.len() as u64) as isize);
                }
                (unsafe {
                    let _state: *mut brunsli_internal_dec_SerializationState = (ss);
                    EncodeSOI_241(_state)
                });
                (unsafe {
                    (|| {
                        if (((*ss).stage as i32)
                            != (brunsli_internal_dec_SerializationState_Stage::ERROR as i32))
                        {
                            (unsafe {
                                let _in: *mut Vec<brunsli_internal_dec_OutputChunk> = (&mut (*ss)
                                    .output_queue
                                    as *mut Vec<brunsli_internal_dec_OutputChunk>);
                                let _available_out: *mut u64 = available_out;
                                let _next_out: *mut *mut u8 = next_out;
                                PushOutput_260(_in, _available_out, _next_out)
                            });
                        }
                    })()
                });
                (*ss).stage =
                    (brunsli_internal_dec_SerializationState_Stage::SERIALIZE_SECTION).clone();
                break;
            }
            v if v == (brunsli_internal_dec_SerializationState_Stage::SERIALIZE_SECTION as i32) => {
                if (((*ss).section_index) >= ((*jpg).marker_order.len() as u64)) {
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::DONE).clone();
                    break;
                }
                let mut marker: u8 = (&(*jpg)).marker_order[((*ss).section_index) as usize];
                let mut status: brunsli_internal_dec_SerializationStatus = (unsafe {
                    let _marker: u8 = marker;
                    let _parsing_state: *const brunsli_internal_dec_State =
                        &(*state) as *const brunsli_internal_dec_State;
                    let _state: *mut brunsli_internal_dec_SerializationState = (ss);
                    let _jpg: *const brunsli_JPEGData = jpg;
                    SerializeSection_259(_marker, _parsing_state, _state, _jpg)
                });
                if ((status) == (brunsli_internal_dec_SerializationStatus::ERROR)) {
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
                            "Failed to encode marker ",
                        );
                        std::fs::File::from_raw_fd(
                            std::io::stderr()
                                .as_fd()
                                .try_clone_to_owned()
                                .unwrap()
                                .into_raw_fd(),
                        )
                        .write_all(&([(&[marker] as &[u8])].concat()));
                        write!(
                            std::fs::File::from_raw_fd(
                                std::io::stderr()
                                    .as_fd()
                                    .try_clone_to_owned()
                                    .unwrap()
                                    .into_raw_fd(),
                            ),
                            "\n",
                        );
                    }
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                    break;
                }
                (unsafe {
                    (|| {
                        if (((*ss).stage as i32)
                            != (brunsli_internal_dec_SerializationState_Stage::ERROR as i32))
                        {
                            (unsafe {
                                let _in: *mut Vec<brunsli_internal_dec_OutputChunk> = (&mut (*ss)
                                    .output_queue
                                    as *mut Vec<brunsli_internal_dec_OutputChunk>);
                                let _available_out: *mut u64 = available_out;
                                let _next_out: *mut *mut u8 = next_out;
                                PushOutput_260(_in, _available_out, _next_out)
                            });
                        }
                    })()
                });
                if ((status) == (brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT)) {
                    return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
                } else if ((status) != (brunsli_internal_dec_SerializationStatus::DONE)) {
                    if !(false) {
                        (unsafe {
                            let _f: *const u8 = b"jpeg_data_writer.cc\0".as_ptr();
                            let _l: i32 = 1073;
                            let _fn: *const u8 = b"SerializeJpeg\0".as_ptr();
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    (*ss).stage = (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                    break;
                }
                (*ss).section_index.prefix_inc();
                break;
            }
            v if v == (brunsli_internal_dec_SerializationState_Stage::DONE as i32) => {
                if !(*ss).output_queue.is_empty() {
                    return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT;
                } else {
                    return brunsli_internal_dec_SerializationStatus::DONE;
                }
            }
            _ => {
                return brunsli_internal_dec_SerializationStatus::ERROR;
            }
        });
    }
    panic!("ub: non-void function does not return a value")
}
#[repr(C)]
#[derive()]
pub struct brunsli_internal_dec_State {
    pub stage: brunsli_internal_dec_Stage,
    pub tags_met: u32,
    pub skip_tags: u32,
    pub data: *const u8,
    pub len: u64,
    pub pos: u64,
    pub context_map: *const u8,
    pub entropy_codes: *const brunsli_ANSDecodingData,
    pub use_legacy_context_model: bool,
    pub is_storage_allocated: bool,
    pub meta: Vec<brunsli_internal_dec_ComponentMeta>,
    pub internal: Option<Box<brunsli_internal_dec_InternalState>>,
}
impl brunsli_internal_dec_State {
    pub unsafe fn brunsli_internal_dec_State() -> Self {
        let mut this = Self {
            stage: brunsli_internal_dec_Stage::SIGNATURE,
            tags_met: 0_u32,
            skip_tags: 0_u32,
            data: std::ptr::null(),
            len: 0_u64,
            pos: 0_u64,
            context_map: std::ptr::null(),
            entropy_codes: std::ptr::null(),
            use_legacy_context_model: false,
            is_storage_allocated: false,
            meta: Vec::new(),
            internal: Some(Box::from_raw(
                (Box::leak(Box::new(<brunsli_internal_dec_InternalState>::default()))
                    as *mut brunsli_internal_dec_InternalState),
            )),
        };
        this
    }
}
impl Default for brunsli_internal_dec_State {
    fn default() -> Self {
        unsafe { brunsli_internal_dec_State::brunsli_internal_dec_State() }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum brunsli_internal_dec_MetadataState_Stage {
    #[default]
    READ_MARKER = 0,
    READ_TAIL = 1,
    READ_CODE = 2,
    READ_LENGTH_HI = 3,
    READ_LENGTH_LO = 4,
    READ_MULTIBYTE = 5,
}
impl From<i32> for brunsli_internal_dec_MetadataState_Stage {
    fn from(n: i32) -> brunsli_internal_dec_MetadataState_Stage {
        match n {
            0 => brunsli_internal_dec_MetadataState_Stage::READ_MARKER,
            1 => brunsli_internal_dec_MetadataState_Stage::READ_TAIL,
            2 => brunsli_internal_dec_MetadataState_Stage::READ_CODE,
            3 => brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_HI,
            4 => brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_LO,
            5 => brunsli_internal_dec_MetadataState_Stage::READ_MULTIBYTE,
            _ => panic!(
                "invalid brunsli_internal_dec_MetadataState_Stage value: {}",
                n
            ),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(brunsli_internal_dec_MetadataState_Stage);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct brunsli_internal_dec_MetadataState {
    pub short_marker_count: u64,
    pub marker: u8,
    pub length_hi: u8,
    pub remaining_multibyte_length: u64,
    pub multibyte_sink: *mut Vec<u8>,
    pub stage: u64,
    pub brotli: *mut ::brotli_sys::BrotliDecoderState,
    pub metadata_size: u64,
    pub decompressed_size: u64,
    pub result: brunsli_BrunsliStatus,
    pub decompression_stage: brunsli_internal_dec_MetadataDecompressionStage,
}
impl brunsli_internal_dec_MetadataState {
    pub unsafe fn CanFinish(&mut self) -> bool {
        return ((self.stage) == (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64))
            || ((self.stage) == (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64));
    }
}
impl brunsli_internal_dec_State {}
impl brunsli_internal_dec_State {}
pub unsafe fn HasSection_194(mut state: *const brunsli_internal_dec_State, mut tag: u32) -> bool {
    return ((((*(*(std::ptr::addr_of!((*state).internal).cast_mut()))
        .as_deref_mut()
        .unwrap())
    .section
    .tags_met)
        & ((1_u32) << (tag)))
        != 0);
}
pub unsafe fn StringWriter_262(
    mut data: *mut ::libc::c_void,
    mut buf: *const u8,
    mut count: u64,
) -> u64 {
    let mut output: *mut Vec<u8> = (data as *mut Vec<u8>);
    (*output).splice((*output).len().saturating_sub(1)..(*output).len(), {
        let mut v = ::std::slice::from_raw_parts((buf as *const u8), count as usize).to_vec();
        v.push(0);
        v
    });
    return count;
}
pub unsafe fn ReadFileInternal_263(mut file: *mut ::libc::FILE, mut content: *mut Vec<u8>) -> bool {
    if ((libc::fseek(file, 0_i64 as ::libc::c_long, 2)) != (0)) {
        printf(b"Failed to seek end of input file.\n\0".as_ptr() as *const i8);
        return false;
    }
    let mut input_size: i32 = (libc::ftell(file) as i64 as i32);
    if ((input_size) == (0)) {
        printf(b"Input file is empty.\n\0".as_ptr() as *const i8);
        return false;
    }
    if ((libc::fseek(file, 0_i64 as ::libc::c_long, 0)) != (0)) {
        printf(b"Failed to rewind input file to the beginning.\n\0".as_ptr() as *const i8);
        return false;
    }
    {
        (*content).pop();
        (*content).resize((input_size as u64) as usize, 0);
        (*content).push(0)
    };
    let mut read_pos: u64 = 0_u64;
    'loop_: while ((read_pos) < (((*(content).cast_const()).len() - 1) as u64)) {
        let bytes_read: u64 = libcc2rs::fread_unsafe(
            ((if read_pos as usize >= (*content).len() - 1 {
                panic!("out of bounds access")
            } else {
                &mut (&mut (*content))[read_pos as usize]
            }) as *mut u8 as *mut ::libc::c_void),
            1_u64,
            (((*(content).cast_const()).len() - 1) as u64).wrapping_sub(read_pos),
            file,
        );
        if ((bytes_read) == (0_u64)) {
            printf(b"Failed to read input file\n\0".as_ptr() as *const i8);
            return false;
        }
        read_pos = (read_pos).wrapping_add(bytes_read);
    }
    return true;
}
pub unsafe fn ReadFile_264(file_name: *const Vec<u8>, mut content: *mut Vec<u8>) -> bool {
    let mut file: *mut ::libc::FILE = libc::fopen(
        (*file_name).as_ptr() as *const i8,
        b"rb\0".as_ptr() as *const i8,
    );
    if (file).is_null() {
        printf(b"Failed to open input file.\n\0".as_ptr() as *const i8);
        return false;
    }
    let mut ok: bool = (unsafe {
        let _file: *mut ::libc::FILE = file;
        let _content: *mut Vec<u8> = content;
        ReadFileInternal_263(_file, _content)
    });
    if ((libc::fclose(file)) != (0)) {
        if ok {
            printf(b"Failed to close input file.\n\0".as_ptr() as *const i8);
        }
        return false;
    }
    return ok;
}
pub unsafe fn WriteFileInternal_265(mut file: *mut ::libc::FILE, content: *const Vec<u8>) -> bool {
    let mut write_pos: u64 = 0_u64;
    'loop_: while ((write_pos) < (((*content).len() - 1) as u64)) {
        let bytes_written: u64 = libcc2rs::fwrite_unsafe(
            ((&(&(*content))[(write_pos) as usize] as *const u8) as *const u8
                as *const ::libc::c_void),
            1_u64,
            (((*content).len() - 1) as u64).wrapping_sub(write_pos),
            file,
        );
        if ((bytes_written) == (0_u64)) {
            printf(b"Failed to write output.\n\0".as_ptr() as *const i8);
            return false;
        }
        write_pos = (write_pos).wrapping_add(bytes_written);
    }
    return true;
}
pub unsafe fn WriteFile_266(file_name: *const Vec<u8>, content: *const Vec<u8>) -> bool {
    let mut file: *mut ::libc::FILE = libc::fopen(
        (*file_name).as_ptr() as *const i8,
        b"wb\0".as_ptr() as *const i8,
    );
    if (file).is_null() {
        printf(b"Failed to open file for writing.\n\0".as_ptr() as *const i8);
        return false;
    }
    let mut ok: bool = (unsafe {
        let _file: *mut ::libc::FILE = file;
        let _content: *const Vec<u8> = content;
        WriteFileInternal_265(_file, _content)
    });
    if ((libc::fclose(file)) != (0)) {
        if ok {
            printf(b"Failed to close output file.\n\0".as_ptr() as *const i8);
        }
        return false;
    }
    return ok;
}
pub unsafe fn ProcessFile_267(file_name: *const Vec<u8>, outfile_name: *const Vec<u8>) -> bool {
    let mut input: Vec<u8> = vec![0];
    let mut ok: bool = (unsafe {
        let _file_name: *const Vec<u8> = file_name;
        let _content: *mut Vec<u8> = (&mut input as *mut Vec<u8>);
        ReadFile_264(_file_name, _content)
    });
    if !ok {
        return false;
    }
    let mut output: Vec<u8> = vec![0];
    let mut jpg: brunsli_JPEGData = brunsli_JPEGData::brunsli_JPEGData();
    let mut input_data: *const u8 = (input.as_ptr() as *const u8);
    let mut status: brunsli_BrunsliStatus = (unsafe {
        let _data: *const u8 = input_data;
        let _len: u64 = (input.len() - 1) as u64;
        let _jpg: *mut brunsli_JPEGData = (&mut jpg as *mut brunsli_JPEGData);
        BrunsliDecodeJpeg_204(_data, _len, _jpg)
    });
    ok = ((status as i32) == (brunsli_BrunsliStatus::BRUNSLI_OK as i32)).clone();
    if ((jpg.version) != (kFallbackVersion_2)) {
        {
            input.clear();
            input.push(0)
        };
        input.shrink_to_fit();
    }
    if !ok {
        printf(b"Failed to parse Brunsli input.\n\0".as_ptr() as *const i8);
        return false;
    }
    let mut writer: brunsli_JPEGOutput = brunsli_JPEGOutput::brunsli_JPEGOutput(
        Some(StringWriter_262),
        ((&mut output as *mut Vec<u8>) as *mut Vec<u8> as *mut ::libc::c_void),
    );
    ok = (unsafe {
        let _jpg: *const brunsli_JPEGData = &jpg as *const brunsli_JPEGData;
        let _out: brunsli_JPEGOutput = writer.clone();
        WriteJpeg_261(_jpg, _out)
    });
    if !ok {
        printf(b"Failed to serialize JPEG data.\n\0".as_ptr() as *const i8);
        return false;
    }
    ok = (unsafe {
        let _file_name: *const Vec<u8> = outfile_name;
        let _content: *const Vec<u8> = &output as *const Vec<u8>;
        WriteFile_266(_file_name, _content)
    });
    return ok;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut u8> = args.iter().map(|arg| arg.as_ptr() as *mut u8).collect();
    argv.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32) }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    if ((argc) != (2)) && ((argc) != (3)) {
        printf(b"Usage: dbrunsli FILE [OUTPUT_FILE, default=FILE.jpg]\n\0".as_ptr() as *const i8);
        return 1;
    }
    let file_name: Vec<u8> = {
        let s = (*argv.offset((1) as isize)).cast_const();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    if file_name.len() <= 1 {
        printf(b"Empty input file name.\n\0".as_ptr() as *const i8);
        return 1;
    }
    let outfile_name: Vec<u8> = if ((argc) == (2)) {
        {
            let mut __tmp2 = file_name.clone();
            __tmp2.pop();
            let __from = b".jpg\0".as_ptr();
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
        let _file_name: *const Vec<u8> = &file_name as *const Vec<u8>;
        let _outfile_name: *const Vec<u8> = &outfile_name as *const Vec<u8>;
        ProcessFile_267(_file_name, _outfile_name)
    });
    return if ok { 0 } else { 1 };
}
