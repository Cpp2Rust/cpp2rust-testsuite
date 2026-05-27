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
        (1 << (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow())),
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
#[derive()]
pub struct brunsli_JPEGQuantTable {
    pub values: Value<Vec<i32>>,
    pub precision: Value<i32>,
    pub index: Value<i32>,
    pub is_last: Value<bool>,
}
impl Clone for brunsli_JPEGQuantTable {
    fn clone(&self) -> Self {
        let mut this = Self {
            values: Rc::new(RefCell::new((*self.values.borrow()).clone())),
            precision: Rc::new(RefCell::new((*self.precision.borrow()))),
            index: Rc::new(RefCell::new((*self.index.borrow()))),
            is_last: Rc::new(RefCell::new((*self.is_last.borrow()))),
        };
        this
    }
}
impl Default for brunsli_JPEGQuantTable {
    fn default() -> Self {
        brunsli_JPEGQuantTable {
            values: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
            precision: <Value<i32>>::default(),
            index: <Value<i32>>::default(),
            is_last: <Value<bool>>::default(),
        }
    }
}
impl ByteRepr for brunsli_JPEGQuantTable {}
#[derive()]
pub struct brunsli_JPEGHuffmanCode {
    pub counts: Value<Vec<i32>>,
    pub values: Value<Vec<i32>>,
    pub slot_id: Value<i32>,
    pub is_last: Value<bool>,
}
impl Clone for brunsli_JPEGHuffmanCode {
    fn clone(&self) -> Self {
        let mut this = Self {
            counts: Rc::new(RefCell::new((*self.counts.borrow()).clone())),
            values: Rc::new(RefCell::new((*self.values.borrow()).clone())),
            slot_id: Rc::new(RefCell::new((*self.slot_id.borrow()))),
            is_last: Rc::new(RefCell::new((*self.is_last.borrow()))),
        };
        this
    }
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
            slot_id: <Value<i32>>::default(),
            is_last: <Value<bool>>::default(),
        }
    }
}
impl ByteRepr for brunsli_JPEGHuffmanCode {}
#[derive(Default)]
pub struct brunsli_JPEGComponentScanInfo {
    pub comp_idx: Value<u8>,
    pub dc_tbl_idx: Value<i32>,
    pub ac_tbl_idx: Value<i32>,
}
impl Clone for brunsli_JPEGComponentScanInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            comp_idx: Rc::new(RefCell::new((*self.comp_idx.borrow()))),
            dc_tbl_idx: Rc::new(RefCell::new((*self.dc_tbl_idx.borrow()))),
            ac_tbl_idx: Rc::new(RefCell::new((*self.ac_tbl_idx.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_JPEGComponentScanInfo {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.comp_idx.borrow()).to_bytes(&mut buf[0..1]);
        (*self.dc_tbl_idx.borrow()).to_bytes(&mut buf[4..8]);
        (*self.ac_tbl_idx.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            comp_idx: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
            dc_tbl_idx: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            ac_tbl_idx: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct brunsli_JPEGScanInfo_ExtraZeroRunInfo {
    pub block_idx: Value<i32>,
    pub num_extra_zero_runs: Value<i32>,
}
impl Clone for brunsli_JPEGScanInfo_ExtraZeroRunInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            block_idx: Rc::new(RefCell::new((*self.block_idx.borrow()))),
            num_extra_zero_runs: Rc::new(RefCell::new((*self.num_extra_zero_runs.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_JPEGScanInfo_ExtraZeroRunInfo {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.block_idx.borrow()).to_bytes(&mut buf[0..4]);
        (*self.num_extra_zero_runs.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            block_idx: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            num_extra_zero_runs: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive()]
pub struct brunsli_JPEGScanInfo {
    pub Ss: Value<i32>,
    pub Se: Value<i32>,
    pub Ah: Value<i32>,
    pub Al: Value<i32>,
    pub num_components: Value<u64>,
    pub components: Value<Vec<brunsli_JPEGComponentScanInfo>>,
    pub reset_points: Value<Vec<i32>>,
    pub extra_zero_runs: Value<Vec<brunsli_JPEGScanInfo_ExtraZeroRunInfo>>,
}
impl Clone for brunsli_JPEGScanInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            Ss: Rc::new(RefCell::new((*self.Ss.borrow()))),
            Se: Rc::new(RefCell::new((*self.Se.borrow()))),
            Ah: Rc::new(RefCell::new((*self.Ah.borrow()))),
            Al: Rc::new(RefCell::new((*self.Al.borrow()))),
            num_components: Rc::new(RefCell::new((*self.num_components.borrow()))),
            components: Rc::new(RefCell::new((*self.components.borrow()).clone())),
            reset_points: Rc::new(RefCell::new((*self.reset_points.borrow()).clone())),
            extra_zero_runs: Rc::new(RefCell::new((*self.extra_zero_runs.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_JPEGScanInfo {
    fn default() -> Self {
        brunsli_JPEGScanInfo {
            Ss: <Value<i32>>::default(),
            Se: <Value<i32>>::default(),
            Ah: <Value<i32>>::default(),
            Al: <Value<i32>>::default(),
            num_components: <Value<u64>>::default(),
            components: Rc::new(RefCell::new(
                std::array::from_fn::<_, 4, _>(|_| Default::default()).to_vec(),
            )),
            reset_points: Rc::new(RefCell::new(Default::default())),
            extra_zero_runs: Rc::new(RefCell::new(Default::default())),
        }
    }
}
impl ByteRepr for brunsli_JPEGScanInfo {}
#[derive()]
pub struct brunsli_JPEGComponent {
    pub id: Value<i32>,
    pub h_samp_factor: Value<i32>,
    pub v_samp_factor: Value<i32>,
    pub quant_idx: Value<u8>,
    pub width_in_blocks: Value<u32>,
    pub height_in_blocks: Value<u32>,
    pub num_blocks: Value<u32>,
    pub coeffs: Value<Vec<i16>>,
}
impl brunsli_JPEGComponent {
    pub fn brunsli_JPEGComponent() -> Self {
        let mut this = Self {
            id: Rc::new(RefCell::new(0)),
            h_samp_factor: Rc::new(RefCell::new(1)),
            v_samp_factor: Rc::new(RefCell::new(1)),
            quant_idx: Rc::new(RefCell::new(0_u8)),
            width_in_blocks: Rc::new(RefCell::new(0_u32)),
            height_in_blocks: Rc::new(RefCell::new(0_u32)),
            num_blocks: <Value<u32>>::default(),
            coeffs: Rc::new(RefCell::new(Vec::new())),
        };
        this
    }
}
impl Clone for brunsli_JPEGComponent {
    fn clone(&self) -> Self {
        let mut this = Self {
            id: Rc::new(RefCell::new((*self.id.borrow()))),
            h_samp_factor: Rc::new(RefCell::new((*self.h_samp_factor.borrow()))),
            v_samp_factor: Rc::new(RefCell::new((*self.v_samp_factor.borrow()))),
            quant_idx: Rc::new(RefCell::new((*self.quant_idx.borrow()))),
            width_in_blocks: Rc::new(RefCell::new((*self.width_in_blocks.borrow()))),
            height_in_blocks: Rc::new(RefCell::new((*self.height_in_blocks.borrow()))),
            num_blocks: Rc::new(RefCell::new((*self.num_blocks.borrow()))),
            coeffs: Rc::new(RefCell::new((*self.coeffs.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_JPEGComponent {
    fn default() -> Self {
        {
            brunsli_JPEGComponent::brunsli_JPEGComponent()
        }
    }
}
impl ByteRepr for brunsli_JPEGComponent {}
#[derive()]
pub struct brunsli_JPEGData {
    pub width: Value<i32>,
    pub height: Value<i32>,
    pub version: Value<i32>,
    pub max_h_samp_factor: Value<i32>,
    pub max_v_samp_factor: Value<i32>,
    pub MCU_rows: Value<i32>,
    pub MCU_cols: Value<i32>,
    pub restart_interval: Value<i32>,
    pub app_data: Value<Vec<Value<Vec<u8>>>>,
    pub com_data: Value<Vec<Value<Vec<u8>>>>,
    pub quant: Value<Vec<brunsli_JPEGQuantTable>>,
    pub huffman_code: Value<Vec<brunsli_JPEGHuffmanCode>>,
    pub components: Value<Vec<brunsli_JPEGComponent>>,
    pub scan_info: Value<Vec<brunsli_JPEGScanInfo>>,
    pub marker_order: Value<Vec<u8>>,
    pub inter_marker_data: Value<Vec<Value<Vec<u8>>>>,
    pub tail_data: Value<Vec<u8>>,
    pub original_jpg: Value<Ptr<u8>>,
    pub original_jpg_size: Value<u64>,
    pub error: Value<brunsli_JPEGReadError>,
    pub has_zero_padding_bit: Value<bool>,
    pub padding_bits: Value<Vec<i32>>,
}
impl brunsli_JPEGData {
    pub fn brunsli_JPEGData() -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new(0)),
            height: Rc::new(RefCell::new(0)),
            version: Rc::new(RefCell::new(2)),
            max_h_samp_factor: Rc::new(RefCell::new(1)),
            max_v_samp_factor: Rc::new(RefCell::new(1)),
            MCU_rows: Rc::new(RefCell::new(0)),
            MCU_cols: Rc::new(RefCell::new(0)),
            restart_interval: Rc::new(RefCell::new(0)),
            app_data: Rc::new(RefCell::new(Vec::new())),
            com_data: Rc::new(RefCell::new(Vec::new())),
            quant: Rc::new(RefCell::new(Vec::new())),
            huffman_code: Rc::new(RefCell::new(Vec::new())),
            components: Rc::new(RefCell::new(Vec::new())),
            scan_info: Rc::new(RefCell::new(Vec::new())),
            marker_order: Rc::new(RefCell::new(Vec::new())),
            inter_marker_data: Rc::new(RefCell::new(Vec::new())),
            tail_data: Rc::new(RefCell::new(Vec::new())),
            original_jpg: Rc::new(RefCell::new(Ptr::<u8>::null())),
            original_jpg_size: Rc::new(RefCell::new(0_u64)),
            error: Rc::new(RefCell::new(brunsli_JPEGReadError::OK)),
            has_zero_padding_bit: Rc::new(RefCell::new(false)),
            padding_bits: Rc::new(RefCell::new(Vec::new())),
        };
        this
    }
}
impl Clone for brunsli_JPEGData {
    fn clone(&self) -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new((*self.width.borrow()))),
            height: Rc::new(RefCell::new((*self.height.borrow()))),
            version: Rc::new(RefCell::new((*self.version.borrow()))),
            max_h_samp_factor: Rc::new(RefCell::new((*self.max_h_samp_factor.borrow()))),
            max_v_samp_factor: Rc::new(RefCell::new((*self.max_v_samp_factor.borrow()))),
            MCU_rows: Rc::new(RefCell::new((*self.MCU_rows.borrow()))),
            MCU_cols: Rc::new(RefCell::new((*self.MCU_cols.borrow()))),
            restart_interval: Rc::new(RefCell::new((*self.restart_interval.borrow()))),
            app_data: Rc::new(RefCell::new(
                (*self.app_data.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
            com_data: Rc::new(RefCell::new(
                (*self.com_data.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
            quant: Rc::new(RefCell::new((*self.quant.borrow()).clone())),
            huffman_code: Rc::new(RefCell::new((*self.huffman_code.borrow()).clone())),
            components: Rc::new(RefCell::new((*self.components.borrow()).clone())),
            scan_info: Rc::new(RefCell::new((*self.scan_info.borrow()).clone())),
            marker_order: Rc::new(RefCell::new((*self.marker_order.borrow()).clone())),
            inter_marker_data: Rc::new(RefCell::new(
                (*self.inter_marker_data.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
            tail_data: Rc::new(RefCell::new((*self.tail_data.borrow()).clone())),
            original_jpg: Rc::new(RefCell::new((*self.original_jpg.borrow()).clone())),
            original_jpg_size: Rc::new(RefCell::new((*self.original_jpg_size.borrow()))),
            error: Rc::new(RefCell::new((*self.error.borrow()).clone())),
            has_zero_padding_bit: Rc::new(RefCell::new((*self.has_zero_padding_bit.borrow()))),
            padding_bits: Rc::new(RefCell::new((*self.padding_bits.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_JPEGData {
    fn default() -> Self {
        {
            brunsli_JPEGData::brunsli_JPEGData()
        }
    }
}
impl ByteRepr for brunsli_JPEGData {}
pub fn JPEGDataIs420_15(jpg: Ptr<brunsli_JPEGData>) -> bool {
    return ((((((((((*(*jpg.upgrade().deref()).components.borrow()).len() as u64 == 3_u64)
        && ((*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()) == 2))
        && ((*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()) == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1));
}
pub fn JPEGDataIs444_16(jpg: Ptr<brunsli_JPEGData>) -> bool {
    return ((((((((((*(*jpg.upgrade().deref()).components.borrow()).len() as u64 == 3_u64)
        && ((*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()) == 1))
        && ((*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()) == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1));
}
pub fn PaddingBitsLimit_17(jpg: Ptr<brunsli_JPEGData>) -> u64 {
    let num_blocks: Value<u64> = Rc::new(RefCell::new(
        ((((*(*jpg.upgrade().deref()).width.borrow()) as u64).wrapping_add(15_u64)) >> 3_u32)
            .wrapping_mul(
                ((((*(*jpg.upgrade().deref()).height.borrow()) as u64).wrapping_add(15_u64))
                    >> 3_u32),
            ),
    ));
    return (((7_u64).wrapping_mul((*num_blocks.borrow())))
        .wrapping_mul((*(*jpg.upgrade().deref()).components.borrow()).len() as u64))
    .wrapping_add(256_u64);
}
thread_local!(
    pub static kBrunsliMaxNumBlocks_18: Value<u64> = Rc::new(RefCell::new((1_u64 << 21)));
);
thread_local!(
    pub static kBrunsliMaxDCAbsVal_19: Value<i32> = Rc::new(RefCell::new(2054));
);
thread_local!(
    pub static kMaxContextMapAlphabetSize_20: Value<u64> = Rc::new(RefCell::new(272_u64));
);
thread_local!(
    pub static kHuffmanTableBits_21: Value<u32> = Rc::new(RefCell::new(8_u32));
);
thread_local!(
    pub static kMaxHuffmanBits_22: Value<u64> = Rc::new(RefCell::new(15_u64));
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
        | ((*kBrunsliWiringTypeVarint_25.with(Value::clone).borrow()) as i32)) as u8);
}
pub fn SectionMarker_29(tag: u8) -> u8 {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    return (((((*tag.borrow()) as i32) << 3)
        | ((*kBrunsliWiringTypeLengthDelimited_26
            .with(Value::clone)
            .borrow()) as i32)) as u8);
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
    pub static kBrunsliSignatureSize_43: Value<u64> = Rc::new(RefCell::new(6_u64));
);
thread_local!();
thread_local!(
    pub static kMaxApp0Densities_45: Value<u64> = Rc::new(RefCell::new(8_u64));
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
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    pub static kBrunsliSignature_44: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ({
            let _tag: u8 = (*kBrunsliSignatureTag_30.with(Value::clone).borrow());
            SectionMarker_29(_tag)
        }),
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
    let t: Value<u16> = <Value<u16>>::default();
    {
        ((t.as_pointer()) as Ptr<u16>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u16>() as u64 as usize);
        ((t.as_pointer()) as Ptr<u16>).to_any().clone()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite16_67(p: AnyPtr, v: u16) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let v: Value<u16> = Rc::new(RefCell::new(v));
    {
        (*p.borrow()).memcpy(
            &((v.as_pointer()) as Ptr<u16>).to_any(),
            ::std::mem::size_of::<u16>() as u64 as usize,
        );
        (*p.borrow()).clone()
    };
}
pub fn BrunsliUnalignedRead32_68(p: AnyPtr) -> u32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let t: Value<u32> = <Value<u32>>::default();
    {
        ((t.as_pointer()) as Ptr<u32>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u32>() as u64 as usize);
        ((t.as_pointer()) as Ptr<u32>).to_any().clone()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedRead64_69(p: AnyPtr) -> u64 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let t: Value<u64> = <Value<u64>>::default();
    {
        ((t.as_pointer()) as Ptr<u64>)
            .to_any()
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u64>() as u64 as usize);
        ((t.as_pointer()) as Ptr<u64>).to_any().clone()
    };
    return (*t.borrow());
}
pub fn BrunsliUnalignedWrite64_70(p: AnyPtr, v: u64) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let v: Value<u64> = Rc::new(RefCell::new(v));
    {
        (*p.borrow()).memcpy(
            &((v.as_pointer()) as Ptr<u64>).to_any(),
            ::std::mem::size_of::<u64>() as u64 as usize,
        );
        (*p.borrow()).clone()
    };
}
pub fn Append_71(dst: Ptr<Vec<u8>>, begin: Ptr<u8>, end: Ptr<u8>) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new(begin));
    let end: Value<Ptr<u8>> = Rc::new(RefCell::new(end));
    {
        let start_idx = ((*dst.borrow()).to_strong().as_pointer() as Ptr<u8>)
            .to_end()
            .clone()
            .get_offset();
        let count = (*end.borrow()).get_offset() - (*begin.borrow()).get_offset();
        let temp_vec: Vec<u8> = PtrValueIter::new(&(*begin.borrow()), count).collect();
        ((*dst.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>).with_mut(|v: &mut Vec<u8>| {
            v.splice(start_idx..start_idx, temp_vec);
        });
        ((*dst.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>) + start_idx
    };
}
pub fn Append_72(dst: Ptr<Vec<u8>>, begin: Ptr<u8>, length: u64) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new(begin));
    let length: Value<u64> = Rc::new(RefCell::new(length));
    ({
        let _dst: Ptr<Vec<u8>> = (*dst.borrow()).clone();
        let _begin: Ptr<u8> = (*begin.borrow()).clone();
        let _end: Ptr<u8> = (*begin.borrow()).offset((*length.borrow()) as isize);
        Append_71(_dst, _begin, _end)
    });
}
pub fn Append_73(dst: Ptr<Vec<u8>>, src: Ptr<Vec<u8>>) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    ({
        let _dst: Ptr<Vec<u8>> = (*dst.borrow()).clone();
        let _begin: Ptr<u8> = (src.to_strong().as_pointer() as Ptr<u8>);
        let _length: u64 = (*src.upgrade().deref()).len() as u64;
        Append_72(_dst, _begin, _length)
    });
}
pub fn Log2FloorNonZero_74(n: u32) -> i32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return (31 ^ (*n.borrow()).leading_zeros() as i32);
}
pub fn BrunsliSuppressUnusedFunctions_75() {
    ((FnPtr::<fn(Ptr<Vec<u8>>, Ptr<Vec<u8>>)>::new(Append_73))
        .cast::<fn(Ptr<Vec<u8>>, Ptr<Vec<u8>>)>(None));
    (FnPtr::<fn()>::new(BrunsliSuppressUnusedFunctions_75));
    (FnPtr::<fn(AnyPtr) -> u16>::new(BrunsliUnalignedRead16_66));
    (FnPtr::<fn(AnyPtr, u16)>::new(BrunsliUnalignedWrite16_67));
    (FnPtr::<fn(AnyPtr) -> u32>::new(BrunsliUnalignedRead32_68));
    (FnPtr::<fn(AnyPtr) -> u64>::new(BrunsliUnalignedRead64_69));
    (FnPtr::<fn(AnyPtr, u64)>::new(BrunsliUnalignedWrite64_70));
    (FnPtr::<fn(AnyPtr) -> u16>::new(BrunsliUnalignedRead16_66));
    (FnPtr::<fn(AnyPtr, u16)>::new(BrunsliUnalignedWrite16_67));
    (FnPtr::<fn(AnyPtr) -> u32>::new(BrunsliUnalignedRead32_68));
    (FnPtr::<fn(AnyPtr) -> u64>::new(BrunsliUnalignedRead64_69));
    (FnPtr::<fn(AnyPtr, u64)>::new(BrunsliUnalignedWrite64_70));
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
            ((*kDivLut17_77.with(Value::clone).borrow())[(*denominator.borrow()) as usize] as u32),
        )) >> 17),
    ));
    if !((*result.borrow()) < 256_u32) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("context.cc");
            let _l: i32 = 55;
            let _fn: Ptr<u8> = Ptr::from_string_literal("FastDivide");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
#[derive()]
pub struct brunsli_Prob {
    prob8: Value<u8>,
    total: Value<u8>,
    count: Value<u16>,
}
impl brunsli_Prob {
    pub fn brunsli_Prob() -> Self {
        let mut this = Self {
            prob8: Rc::new(RefCell::new((*kInitProb_80.with(Value::clone).borrow()))),
            total: Rc::new(RefCell::new(
                (*kInitProbCount_81.with(Value::clone).borrow()),
            )),
            count: Rc::new(RefCell::new(
                ((((*kInitProb_80.with(Value::clone).borrow()) as i32)
                    * ((*kInitProbCount_81.with(Value::clone).borrow()) as i32))
                    as u16),
            )),
        };
        this
    }
    pub fn Init(&self, probability: u8) {
        let probability: Value<u8> = Rc::new(RefCell::new(probability));
        (*self.prob8.borrow_mut()) = (*probability.borrow());
        (*self.total.borrow_mut()) = (*kInitProbCount_81.with(Value::clone).borrow());
        (*self.count.borrow_mut()) = ((((*kInitProbCount_81.with(Value::clone).borrow()) as i32)
            * ((*probability.borrow()) as i32)) as u16);
    }
    pub fn Add(&self, val: i32) {
        let val: Value<i32> = Rc::new(RefCell::new(val));
        (*self.total.borrow_mut()).prefix_inc();
        if ((*val.borrow()) == 0) {
            let rhs_0 = (((*self.count.borrow()) as i32) + 256) as u16;
            (*self.count.borrow_mut()) = rhs_0;
        } else {
            (*self.count.borrow_mut()).prefix_inc();
        }
        (*self.prob8.borrow_mut()) = ({
            let _numerator: u32 = ((*self.count.borrow()) as u32);
            let _denominator: u8 = (*self.total.borrow());
            FastDivide_78(_numerator, _denominator)
        });
        if (((*self.total.borrow()) as i32)
            == ((*kNormalizeThreshold_76.with(Value::clone).borrow()) as i32))
        {
            let rhs_0 = (((*self.count.borrow()) as i32) >> 1) as u16;
            (*self.count.borrow_mut()) = rhs_0;
            (*self.total.borrow_mut()) =
                ((((*kNormalizeThreshold_76.with(Value::clone).borrow()) as i32) >> 1) as u8);
        }
    }
    pub fn get_proba(&self) -> u8 {
        return (*self.prob8.borrow());
    }
}
impl Clone for brunsli_Prob {
    fn clone(&self) -> Self {
        let mut this = Self {
            prob8: Rc::new(RefCell::new((*self.prob8.borrow()))),
            total: Rc::new(RefCell::new((*self.total.borrow()))),
            count: Rc::new(RefCell::new((*self.count.borrow()))),
        };
        this
    }
}
impl Default for brunsli_Prob {
    fn default() -> Self {
        {
            brunsli_Prob::brunsli_Prob()
        }
    }
}
impl ByteRepr for brunsli_Prob {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.prob8.borrow()).to_bytes(&mut buf[0..1]);
        (*self.total.borrow()).to_bytes(&mut buf[1..2]);
        (*self.count.borrow()).to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            prob8: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
            total: Rc::new(RefCell::new(<u8>::from_bytes(&buf[1..2]))),
            count: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
        }
    }
}
thread_local!(
    pub static kMaxAverageContext_82: Value<u64> = Rc::new(RefCell::new(8_u64));
);
thread_local!(
    pub static kNumAvrgContexts_83: Value<u64> = Rc::new(RefCell::new(
        (*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(1_u64),
    ));
);
thread_local!(
    pub static kNumNonZeroBits_84: Value<u64> = Rc::new(RefCell::new(6_u64));
);
thread_local!(
    pub static kNumNonZeroTreeSize_85: Value<u64> = Rc::new(RefCell::new(
        ((((1_u32 << (*kNumNonZeroBits_84.with(Value::clone).borrow())) as u32)
            .wrapping_sub(1_u32 as u32)) as u64),
    ));
);
thread_local!(
    pub static kNumNonZeroQuant_86: Value<u64> = Rc::new(RefCell::new(2_u64));
);
thread_local!(
    pub static kNumNonZeroContextMax_87: Value<u64> = Rc::new(RefCell::new(
        (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())
            .wrapping_div((*kNumNonZeroQuant_86.with(Value::clone).borrow())),
    ));
);
thread_local!(
    pub static kNumNonZeroContextCount_88: Value<u64> = Rc::new(RefCell::new(
        (*kNumNonZeroContextMax_87.with(Value::clone).borrow()).wrapping_add(1_u64),
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
            0_u8,
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
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
pub fn ZeroDensityContext_96(nonzeros_left: u64, k: u64, bits: u64) -> u16 {
    let nonzeros_left: Value<u64> = Rc::new(RefCell::new(nonzeros_left));
    let k: Value<u64> = Rc::new(RefCell::new(k));
    let bits: Value<u64> = Rc::new(RefCell::new(bits));
    return ((((*kNumNonzeroContext_93.with(Value::clone).borrow())[(*bits.borrow()) as usize]
        .borrow()[(*nonzeros_left.borrow()) as usize] as i32)
        + ((*kFreqContext_92.with(Value::clone).borrow())[(*bits.borrow()) as usize].borrow()
            [(*k.borrow()) as usize] as i32)) as u16);
}
pub fn WeightedAverageContextDC_97(vals: Ptr<i32>, x: i32) -> i32 {
    let vals: Value<Ptr<i32>> = Rc::new(RefCell::new(vals));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((((1 + ((*vals.borrow()).offset(((*x.borrow()) - 2) as isize).read()))
            + ((*vals.borrow()).offset(((*x.borrow()) - 1) as isize).read()))
            + ((*vals.borrow()).offset((*x.borrow()) as isize).read()))
            + ((*vals.borrow()).offset(((*x.borrow()) + 1) as isize).read())),
    ));
    if (((*sum.borrow()) >> (*kMaxAverageContext_82.with(Value::clone).borrow())) != 0) {
        return ((*kMaxAverageContext_82.with(Value::clone).borrow()) as i32);
    }
    return ({
        let _n: u32 = ((*sum.borrow()) as u32);
        Log2FloorNonZero_74(_n)
    });
}
pub fn WeightedAverageContext_98(vals: Ptr<i32>, prev_row_delta: i32) -> i32 {
    let vals: Value<Ptr<i32>> = Rc::new(RefCell::new(vals));
    let prev_row_delta: Value<i32> = Rc::new(RefCell::new(prev_row_delta));
    let sum: Value<i32> = Rc::new(RefCell::new(
        (({
            let _lhs = {
                let _lhs = (4 + ((*vals.borrow()).offset((0) as isize).read()));
                _lhs + (({
                    let _lhs = ((*vals.borrow())
                        .offset((-(*kDCTBlockSize_3.with(Value::clone).borrow())) as isize)
                        .read());
                    _lhs + ((*vals.borrow())
                        .offset((*prev_row_delta.borrow()) as isize)
                        .read())
                }) * 2)
            };
            _lhs + ((*vals.borrow())
                .offset((-2_i32 * (*kDCTBlockSize_3.with(Value::clone).borrow())) as isize)
                .read())
        } + ((*vals.borrow())
            .offset(
                ((*prev_row_delta.borrow()) - (*kDCTBlockSize_3.with(Value::clone).borrow()))
                    as isize,
            )
            .read()))
            + ((*vals.borrow())
                .offset(
                    ((*prev_row_delta.borrow()) + (*kDCTBlockSize_3.with(Value::clone).borrow()))
                        as isize,
                )
                .read())),
    ));
    if (((*sum.borrow())
        >> ((*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(2_u64)))
        != 0)
    {
        return ((*kMaxAverageContext_82.with(Value::clone).borrow()) as i32);
    }
    return (({
        let _n: u32 = ((*sum.borrow()) as u32);
        Log2FloorNonZero_74(_n)
    }) - 2);
}
thread_local!(
    pub static kACPredictPrecisionBits_99: Value<i32> = Rc::new(RefCell::new(13));
);
thread_local!(
    pub static kACPredictPrecision_100: Value<i32> = Rc::new(RefCell::new(
        (1 << (*kACPredictPrecisionBits_99.with(Value::clone).borrow())),
    ));
);
pub fn ACPredictContext_101(p: i64, avg_ctx: Ptr<u64>, sgn: Ptr<u64>) {
    let p: Value<i64> = Rc::new(RefCell::new(p));
    let avg_ctx: Value<Ptr<u64>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<u64>> = Rc::new(RefCell::new(sgn));
    let multiplier: Value<i32> = <Value<i32>>::default();
    if ((*p.borrow()) >= 0_i64) {
        (*multiplier.borrow_mut()) = 1;
    } else {
        (*multiplier.borrow_mut()) = -1_i32;
        let __rhs = -(*p.borrow());
        (*p.borrow_mut()) = __rhs;
    }
    let ctx: Value<u64> = <Value<u64>>::default();
    if ((*p.borrow()) >= ((1_u32 << (*kMaxAverageContext_82.with(Value::clone).borrow())) as i64)) {
        (*ctx.borrow_mut()) = (*kMaxAverageContext_82.with(Value::clone).borrow());
    } else {
        (*ctx.borrow_mut()) = (({
            let _n: u32 = ((2_u32).wrapping_mul(((*p.borrow()) as u32))).wrapping_add(1_u32);
            Log2FloorNonZero_74(_n)
        }) as u64);
    }
    let __rhs = (*ctx.borrow());
    (*avg_ctx.borrow()).write(__rhs);
    let __rhs = (*kMaxAverageContext_82.with(Value::clone).borrow())
        .wrapping_add(((*multiplier.borrow()) as u64).wrapping_mul((*ctx.borrow())));
    (*sgn.borrow()).write(__rhs);
}
pub fn ACPredictContextCol_102(
    prev: Ptr<i16>,
    cur: Ptr<i16>,
    mult: Ptr<i32>,
    avg_ctx: Ptr<u64>,
    sgn: Ptr<u64>,
) {
    let prev: Value<Ptr<i16>> = Rc::new(RefCell::new(prev));
    let cur: Value<Ptr<i16>> = Rc::new(RefCell::new(cur));
    let mult: Value<Ptr<i32>> = Rc::new(RefCell::new(mult));
    let avg_ctx: Value<Ptr<u64>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<u64>> = Rc::new(RefCell::new(sgn));
    let terms: Value<Box<[i16]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <i16>::default()).collect::<Box<[i16]>>(),
    ));
    (*terms.borrow_mut())[(0) as usize] = 0_i16;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((1) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((1) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(1) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((2) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((2) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(2) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((3) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((3) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(3) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((4) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((4) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(4) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((5) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((5) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(5) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((6) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((6) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(6) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((7) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((7) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(7) as usize] = __rhs;
    let delta: Value<i64> = Rc::new(RefCell::new(
        ((((((({
            let _lhs = ((*terms.borrow())[(0) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((0) as isize).read()) as i64)
        } + {
            let _lhs = ((*terms.borrow())[(1) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((1) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(2) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((2) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(3) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((3) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(4) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((4) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(5) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((5) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(6) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((6) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(7) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((7) as isize).read()) as i64)
        }),
    ));
    ({
        let _p: i64 = {
            let _lhs = (((*prev.borrow()).offset((0) as isize).read()) as i64);
            _lhs - ((*delta.borrow())
                / ((*kACPredictPrecision_100.with(Value::clone).borrow()) as i64))
        };
        let _avg_ctx: Ptr<u64> = (*avg_ctx.borrow()).clone();
        let _sgn: Ptr<u64> = (*sgn.borrow()).clone();
        ACPredictContext_101(_p, _avg_ctx, _sgn)
    });
}
pub fn ACPredictContextRow_103(
    prev: Ptr<i16>,
    cur: Ptr<i16>,
    mult: Ptr<i32>,
    avg_ctx: Ptr<u64>,
    sgn: Ptr<u64>,
) {
    let prev: Value<Ptr<i16>> = Rc::new(RefCell::new(prev));
    let cur: Value<Ptr<i16>> = Rc::new(RefCell::new(cur));
    let mult: Value<Ptr<i32>> = Rc::new(RefCell::new(mult));
    let avg_ctx: Value<Ptr<u64>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<u64>> = Rc::new(RefCell::new(sgn));
    let terms: Value<Box<[i16]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <i16>::default()).collect::<Box<[i16]>>(),
    ));
    (*terms.borrow_mut())[(0) as usize] = 0_i16;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((8) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((8) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(1) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((16) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((16) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(2) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((24) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((24) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(3) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((32) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((32) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(4) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((40) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((40) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(5) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((48) as isize).read()) as i32);
        _lhs - (((*prev.borrow()).offset((48) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(6) as usize] = __rhs;
    let __rhs = (({
        let _lhs = (((*cur.borrow()).offset((56) as isize).read()) as i32);
        _lhs + (((*prev.borrow()).offset((56) as isize).read()) as i32)
    }) as i16);
    (*terms.borrow_mut())[(7) as usize] = __rhs;
    let delta: Value<i64> = Rc::new(RefCell::new(
        ((((((({
            let _lhs = ((*terms.borrow())[(0) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((0) as isize).read()) as i64)
        } + {
            let _lhs = ((*terms.borrow())[(1) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((1) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(2) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((2) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(3) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((3) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(4) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((4) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(5) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((5) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(6) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((6) as isize).read()) as i64)
        }) + {
            let _lhs = ((*terms.borrow())[(7) as usize] as i64);
            _lhs * (((*mult.borrow()).offset((7) as isize).read()) as i64)
        }),
    ));
    ({
        let _p: i64 = {
            let _lhs = (((*prev.borrow()).offset((0) as isize).read()) as i64);
            _lhs - ((*delta.borrow())
                / ((*kACPredictPrecision_100.with(Value::clone).borrow()) as i64))
        };
        let _avg_ctx: Ptr<u64> = (*avg_ctx.borrow()).clone();
        let _sgn: Ptr<u64> = (*sgn.borrow()).clone();
        ACPredictContext_101(_p, _avg_ctx, _sgn)
    });
}
pub fn NumNonzerosContext_104(prev: Ptr<u8>, x: i32, y: i32) -> u8 {
    let prev: Value<Ptr<u8>> = Rc::new(RefCell::new(prev));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    let prediction: Value<u64> = <Value<u64>>::default();
    if ((*y.borrow()) == 0) {
        if ((*x.borrow()) == 0) {
            (*prediction.borrow_mut()) = 0_u64;
        } else {
            (*prediction.borrow_mut()) =
                (((*prev.borrow()).offset(((*x.borrow()) - 1) as isize).read()) as u64);
        }
    } else if ((*x.borrow()) == 0) {
        (*prediction.borrow_mut()) =
            (((*prev.borrow()).offset((*x.borrow()) as isize).read()) as u64);
    } else {
        (*prediction.borrow_mut()) =
            (((((((*prev.borrow()).offset(((*x.borrow()) - 1) as isize).read()) as i32)
                + (((*prev.borrow()).offset((*x.borrow()) as isize).read()) as i32))
                + 1)
                / 2) as u64);
    }
    if !((*prediction.borrow()) <= (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("context.cc");
            let _l: i32 = 305;
            let _fn: Ptr<u8> = Ptr::from_string_literal("NumNonzerosContext");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return (((*prediction.borrow())
        .wrapping_div((*kNumNonZeroQuant_86.with(Value::clone).borrow()))) as u8);
}
thread_local!(
    pub static kNumIsEmptyBlockContexts_105: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn IsEmptyBlockContext_106(prev: Ptr<i32>, x: i32) -> i32 {
    let prev: Value<Ptr<i32>> = Rc::new(RefCell::new(prev));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (((*prev.borrow()).offset(((*x.borrow()) - 1) as isize).read())
        + ((*prev.borrow()).offset((*x.borrow()) as isize).read()));
}
#[derive()]
pub struct brunsli_ComponentStateDC {
    pub width: Value<i32>,
    pub is_zero_prob: Value<brunsli_Prob>,
    pub is_empty_block_prob: Value<Vec<brunsli_Prob>>,
    pub sign_prob: Value<Vec<brunsli_Prob>>,
    pub first_extra_bit_prob: Value<Vec<brunsli_Prob>>,
    pub prev_is_nonempty: Value<Vec<i32>>,
    pub prev_abs_coeff: Value<Vec<i32>>,
    pub prev_sign: Value<Vec<i32>>,
}
impl brunsli_ComponentStateDC {
    pub fn brunsli_ComponentStateDC() -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new(0)),
            is_zero_prob: Rc::new(RefCell::new(brunsli_Prob::brunsli_Prob())),
            is_empty_block_prob: Rc::new(RefCell::new(
                (0..((*kNumIsEmptyBlockContexts_105.with(Value::clone).borrow()) as u64) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            sign_prob: Rc::new(RefCell::new(
                (0..(9_u64) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (0..(10_u64) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new(Vec::new())),
            prev_abs_coeff: Rc::new(RefCell::new(Vec::new())),
            prev_sign: Rc::new(RefCell::new(Vec::new())),
        };
        ({ this.InitAll() });
        this
    }
    pub fn SetWidth(&self, w: i32) {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        (*self.width.borrow_mut()) = (*w.borrow());
        {
            let __a0 = (((*w.borrow()) + 1) as u64) as usize;
            (*self.prev_is_nonempty.borrow_mut()).resize(__a0, 1)
        };
        {
            let __a0 = (((*w.borrow()) + 3) as u64) as usize;
            (*self.prev_abs_coeff.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((*w.borrow()) + 1) as u64) as usize;
            (*self.prev_sign.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
    }
}
impl Clone for brunsli_ComponentStateDC {
    fn clone(&self) -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new((*self.width.borrow()))),
            is_zero_prob: Rc::new(RefCell::new((*self.is_zero_prob.borrow()).clone())),
            is_empty_block_prob: Rc::new(RefCell::new(
                (*self.is_empty_block_prob.borrow()).clone(),
            )),
            sign_prob: Rc::new(RefCell::new((*self.sign_prob.borrow()).clone())),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (*self.first_extra_bit_prob.borrow()).clone(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new((*self.prev_is_nonempty.borrow()).clone())),
            prev_abs_coeff: Rc::new(RefCell::new((*self.prev_abs_coeff.borrow()).clone())),
            prev_sign: Rc::new(RefCell::new((*self.prev_sign.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_ComponentStateDC {
    fn default() -> Self {
        {
            brunsli_ComponentStateDC::brunsli_ComponentStateDC()
        }
    }
}
impl ByteRepr for brunsli_ComponentStateDC {}
#[derive()]
pub struct brunsli_ComponentState {
    pub width: Value<i32>,
    pub context_offset: Value<i32>,
    pub order: Value<Box<[u32]>>,
    pub mult_row: Value<Box<[i32]>>,
    pub mult_col: Value<Box<[i32]>>,
    pub is_zero_prob: Value<Vec<brunsli_Prob>>,
    pub sign_prob: Value<Vec<brunsli_Prob>>,
    pub num_nonzero_prob: Value<Box<[brunsli_Prob]>>,
    pub first_extra_bit_prob: Value<Vec<brunsli_Prob>>,
    pub prev_is_nonempty: Value<Vec<i32>>,
    pub prev_num_nonzeros: Value<Vec<u8>>,
    pub prev_abs_coeff: Value<Vec<i32>>,
    pub prev_sign: Value<Vec<i32>>,
}
impl brunsli_ComponentState {
    pub fn brunsli_ComponentState() -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new(0)),
            context_offset: <Value<i32>>::default(),
            order: Rc::new(RefCell::new(
                (0..64).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
            )),
            mult_row: Rc::new(RefCell::new(
                (0..64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            mult_col: Rc::new(RefCell::new(
                (0..64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            is_zero_prob: Rc::new(RefCell::new(
                (0..((((*kNumNonzeroBuckets_90.with(Value::clone).borrow()) as i32)
                    * (*kDCTBlockSize_3.with(Value::clone).borrow())) as u64)
                    as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            sign_prob: Rc::new(RefCell::new(
                (0..(((((2_u64).wrapping_mul((*kMaxAverageContext_82.with(Value::clone).borrow()))
                    as u64)
                    .wrapping_add(1_u64)) as u64)
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
                    as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            num_nonzero_prob: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2016, _>(
                |_| brunsli_Prob::brunsli_Prob(),
            )))),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (0..((10 * (*kDCTBlockSize_3.with(Value::clone).borrow())) as u64) as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new(Vec::new())),
            prev_num_nonzeros: Rc::new(RefCell::new(Vec::new())),
            prev_abs_coeff: Rc::new(RefCell::new(Vec::new())),
            prev_sign: Rc::new(RefCell::new(Vec::new())),
        };
        ({ this.InitAll() });
        this
    }
    pub fn SetWidth(&self, w: i32) {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        (*self.width.borrow_mut()) = (*w.borrow());
        {
            let __a0 = (((*w.borrow()) + 1) as u64) as usize;
            (*self.prev_is_nonempty.borrow_mut()).resize(__a0, 1)
        };
        {
            let __a0 = ((*w.borrow()) as u64) as usize;
            (*self.prev_num_nonzeros.borrow_mut()).resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 = ((((*kDCTBlockSize_3.with(Value::clone).borrow()) * 2) * ((*w.borrow()) + 3))
                as u64) as usize;
            (*self.prev_abs_coeff.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((*kDCTBlockSize_3.with(Value::clone).borrow()) * ((*w.borrow()) + 1))
                as u64) as usize;
            (*self.prev_sign.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
    }
    pub fn SizeInBytes(w: i32) -> u64 {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        return ((((4
            + ((10 + (3 * (*w.borrow()))) * (*kDCTBlockSize_3.with(Value::clone).borrow())))
            + (2 * (*w.borrow()))) as u64)
            .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64))
        .wrapping_add(
            ((((((((*kNumNonzeroBuckets_90.with(Value::clone).borrow()) as u64).wrapping_add(
                (2_u64).wrapping_mul((*kMaxAverageContext_82.with(Value::clone).borrow())) as u64,
            ) as u64)
                .wrapping_add(11_u64)) as u64)
                .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64))
                as u64)
                .wrapping_add(
                    (*kNumNonZeroContextCount_88.with(Value::clone).borrow())
                        .wrapping_mul((*kNumNonZeroTreeSize_85.with(Value::clone).borrow()))
                        as u64,
                )) as u64)
                .wrapping_mul(::std::mem::size_of::<brunsli_Prob>() as u64 as u64)
                as u64,
        );
    }
}
impl Clone for brunsli_ComponentState {
    fn clone(&self) -> Self {
        let mut this = Self {
            width: Rc::new(RefCell::new((*self.width.borrow()))),
            context_offset: Rc::new(RefCell::new((*self.context_offset.borrow()))),
            order: Rc::new(RefCell::new((*self.order.borrow()).clone())),
            mult_row: Rc::new(RefCell::new((*self.mult_row.borrow()).clone())),
            mult_col: Rc::new(RefCell::new((*self.mult_col.borrow()).clone())),
            is_zero_prob: Rc::new(RefCell::new((*self.is_zero_prob.borrow()).clone())),
            sign_prob: Rc::new(RefCell::new((*self.sign_prob.borrow()).clone())),
            num_nonzero_prob: Rc::new(RefCell::new((*self.num_nonzero_prob.borrow()).clone())),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (*self.first_extra_bit_prob.borrow()).clone(),
            )),
            prev_is_nonempty: Rc::new(RefCell::new((*self.prev_is_nonempty.borrow()).clone())),
            prev_num_nonzeros: Rc::new(RefCell::new((*self.prev_num_nonzeros.borrow()).clone())),
            prev_abs_coeff: Rc::new(RefCell::new((*self.prev_abs_coeff.borrow()).clone())),
            prev_sign: Rc::new(RefCell::new((*self.prev_sign.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_ComponentState {
    fn default() -> Self {
        {
            brunsli_ComponentState::brunsli_ComponentState()
        }
    }
}
impl ByteRepr for brunsli_ComponentState {}
thread_local!(
    pub static kSqrt2_107: Value<f64> = Rc::new(RefCell::new(1.414213562E+0));
);
thread_local!(
    pub static kSqrt2FixedPoint_108: Value<i32> = Rc::new(RefCell::new(
        (((*kSqrt2_107.with(Value::clone).borrow())
            * ((*kACPredictPrecision_100.with(Value::clone).borrow()) as f64)) as i32),
    ));
);
pub fn ComputeACPredictMultipliers_109(quant: Ptr<i32>, mult_row: Ptr<i32>, mult_col: Ptr<i32>) {
    let quant: Value<Ptr<i32>> = Rc::new(RefCell::new(quant));
    let mult_row: Value<Ptr<i32>> = Rc::new(RefCell::new(mult_row));
    let mult_col: Value<Ptr<i32>> = Rc::new(RefCell::new(mult_col));
    let y: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*y.borrow()) < 8_u64) {
        let x: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*x.borrow()) < 8_u64) {
            let __rhs = {
                let _lhs = ({
                    let _lhs = ((*quant.borrow())
                        .offset(
                            ((*x.borrow()).wrapping_add((8_u64).wrapping_mul((*y.borrow()))))
                                as isize,
                        )
                        .read());
                    _lhs * (*kSqrt2FixedPoint_108.with(Value::clone).borrow())
                });
                _lhs / ((*quant.borrow())
                    .offset(((*y.borrow()).wrapping_mul(8_u64)) as isize)
                    .read())
            };
            (*mult_row.borrow())
                .offset(((*x.borrow()).wrapping_add((8_u64).wrapping_mul((*y.borrow())))) as isize)
                .write(__rhs);
            let __rhs = {
                let _lhs = ({
                    let _lhs = ((*quant.borrow())
                        .offset(
                            ((*x.borrow()).wrapping_add((8_u64).wrapping_mul((*y.borrow()))))
                                as isize,
                        )
                        .read());
                    _lhs * (*kSqrt2FixedPoint_108.with(Value::clone).borrow())
                });
                _lhs / ((*quant.borrow()).offset((*x.borrow()) as isize).read())
            };
            (*mult_col.borrow())
                .offset((((*x.borrow()).wrapping_mul(8_u64)).wrapping_add((*y.borrow()))) as isize)
                .write(__rhs);
            (*x.borrow_mut()).prefix_inc();
        }
        (*y.borrow_mut()).prefix_inc();
    }
}
impl brunsli_ComponentStateDC {
    fn InitAll(&self) {
        ({
            let _probability: u8 = 135_u8;
            (*self.is_zero_prob.borrow()).Init(_probability)
        });
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*self.sign_prob.borrow()).len() as u64) {
            ({
                let _probability: u8 = 128_u8;
                (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(_probability)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*self.is_empty_block_prob.borrow()).len() as u64) {
            ({
                let _probability: u8 = 74_u8;
                (*(self.is_empty_block_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(_probability)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*self.first_extra_bit_prob.borrow()).len() as u64) {
            ({
                let _probability: u8 = 150_u8;
                (*(self.first_extra_bit_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(_probability)
            });
            (*i.borrow_mut()).prefix_inc();
        }
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
impl brunsli_ComponentState {
    fn InitAll(&self) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow())
            < ((*kNumNonzeroBuckets_90.with(Value::clone).borrow()) as i32))
        {
            let k: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
                let v: Value<i32> = Rc::new(RefCell::new(
                    (((*kInitProb_110.with(Value::clone).borrow())[(*k.borrow()) as usize] as i32)
                        + (9 * ((*i.borrow()) - 7))),
                ));
                if !((*v.borrow()) <= 255) {
                    ({
                        let _f: Ptr<u8> = Ptr::from_string_literal("context.cc");
                        let _l: i32 = 227;
                        let _fn: Ptr<u8> = Ptr::from_string_literal("InitAll");
                        BrunsliDumpAndAbort_79(_f, _l, _fn)
                    });
                    'loop_: while true {}
                };
                ({
                    let _probability: u8 = ((*v.borrow()) as u8);
                    (*(self.is_zero_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset(
                            ((((*i.borrow()) * (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                + (*k.borrow())) as u64) as isize,
                        )
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
                (*k.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*self.sign_prob.borrow()).len() as u64) {
            if ((*i.borrow())
                < (*kMaxAverageContext_82.with(Value::clone).borrow())
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
            {
                ({
                    let _probability: u8 = 108_u8;
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
            } else if ((*i.borrow())
                < (((*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(1_u64))
                    as u64)
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
            {
                ({
                    let _probability: u8 = 128_u8;
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
            } else {
                ({
                    let _probability: u8 = 148_u8;
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*self.first_extra_bit_prob.borrow()).len() as u64) {
            ({
                let _probability: u8 = 158_u8;
                (*(self.first_extra_bit_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(_probability)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*kNumNonZeroContextCount_88.with(Value::clone).borrow())) {
            let non_zero_probs: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                (self.num_nonzero_prob.as_pointer() as Ptr<brunsli_Prob>).offset(
                    ((*i.borrow())
                        .wrapping_mul((*kNumNonZeroTreeSize_85.with(Value::clone).borrow())))
                        as isize,
                ),
            ));
            let j: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while ((*j.borrow()) < (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())) {
                ({
                    let _probability: u8 = (*kInitProbNonzero_111.with(Value::clone).borrow())
                        [(*i.borrow()) as usize]
                        .borrow()[(*j.borrow()) as usize];
                    (*(*non_zero_probs.borrow())
                        .offset((*j.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
#[derive()]
pub struct brunsli_PermutationCoder {
    values_: Value<Vec<u8>>,
}
impl brunsli_PermutationCoder {
    pub fn brunsli_PermutationCoder() -> Self {
        let mut this = Self {
            values_: Rc::new(RefCell::new(Vec::new())),
        };
        this
    }
    pub fn Init(&self, values: Vec<u8>) {
        let values: Value<Vec<u8>> = Rc::new(RefCell::new(values));
        (self.values_.as_pointer() as Ptr<Vec<u8>>)
            .write(std::mem::take(&mut (*values.borrow_mut())));
    }
    pub fn Clear(&self) {
        std::mem::swap(&mut Vec::new(), &mut (*self.values_.borrow_mut()));
    }
    pub fn num_bits(&self) -> i32 {
        let num_values: Value<u32> =
            Rc::new(RefCell::new(((*self.values_.borrow()).len() as u64 as u32)));
        if !((*num_values.borrow()) > 0_u32) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("lehmer_code.cc");
                let _l: i32 = 51;
                let _fn: Ptr<u8> = Ptr::from_string_literal("num_bits");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if ((*num_values.borrow()) <= 1_u32) {
            return 0;
        }
        return (({
            let _n: u32 = (*num_values.borrow()).wrapping_sub(1_u32);
            Log2FloorNonZero_74(_n)
        }) + 1);
    }
    pub fn Remove(&self, code: u64, value: Ptr<u8>) -> bool {
        let code: Value<u64> = Rc::new(RefCell::new(code));
        let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
        if ((*code.borrow()) >= (*self.values_.borrow()).len() as u64) {
            return false;
        }
        let __rhs = ((self.values_.as_pointer() as Ptr<u8>)
            .offset((*code.borrow()) as isize)
            .read());
        (*value.borrow()).write(__rhs);
        {
            let idx = (self.values_.as_pointer() as Ptr<u8>)
                .offset(((*code.borrow()) as i64) as isize)
                .clone()
                .get_offset();
            (self.values_.as_pointer() as Ptr<Vec<u8>>)
                .with_mut(|__v: &mut Vec<u8>| __v.remove(idx));
            (self.values_.as_pointer() as Ptr<Vec<u8>>)
                .to_strong()
                .as_pointer() as Ptr<u8>
        };
        return true;
    }
    pub fn RemoveValue(&self, value: u8, code: Ptr<i32>, nbits: Ptr<i32>) -> bool {
        let value: Value<u8> = Rc::new(RefCell::new(value));
        let code: Value<Ptr<i32>> = Rc::new(RefCell::new(code));
        let nbits: Value<Ptr<i32>> = Rc::new(RefCell::new(nbits));
        let it: Value<Ptr<u8>> =
            Rc::new(RefCell::new(
                (self.values_.as_pointer() as Ptr<u8>).offset(
                    (self.values_.as_pointer() as Ptr<u8>)
                        .clone()
                        .into_iter()
                        .enumerate()
                        .position(|(index_0, value_0)| {
                            index_0
                                < (self.values_.as_pointer() as Ptr<u8>).to_end().get_offset()
                                    as usize
                                && value_0.read() == (*value.borrow())
                        })
                        .unwrap_or(
                            (self.values_.as_pointer() as Ptr<u8>).to_end().get_offset() as usize
                        ) as isize,
                ),
            ));
        if (*it.borrow()) == (self.values_.as_pointer() as Ptr<u8>).to_end() {
            return false;
        }
        let __rhs = ((((*it.borrow()).get_offset() as isize)
            - ((self.values_.as_pointer() as Ptr<u8>).get_offset() as isize))
            as i32);
        (*code.borrow()).write(__rhs);
        let __rhs = ({ self.num_bits() });
        (*nbits.borrow()).write(__rhs);
        {
            let idx = (*it.borrow()).clone().get_offset();
            (self.values_.as_pointer() as Ptr<Vec<u8>>)
                .with_mut(|__v: &mut Vec<u8>| __v.remove(idx));
            (self.values_.as_pointer() as Ptr<Vec<u8>>)
                .to_strong()
                .as_pointer() as Ptr<u8>
        };
        return true;
    }
}
impl Clone for brunsli_PermutationCoder {
    fn clone(&self) -> Self {
        let mut this = Self {
            values_: Rc::new(RefCell::new((*self.values_.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_PermutationCoder {
    fn default() -> Self {
        {
            brunsli_PermutationCoder::brunsli_PermutationCoder()
        }
    }
}
impl ByteRepr for brunsli_PermutationCoder {}
pub fn ComputeLehmerCode_112(sigma: Ptr<u32>, len: u64, code: Ptr<u32>) {
    let sigma: Value<Ptr<u32>> = Rc::new(RefCell::new(sigma));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let code: Value<Ptr<u32>> = Rc::new(RefCell::new(code));
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(*len.borrow()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let __rhs = ((*i.borrow()) as u32);
        (items.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let it: Value<Ptr<u32>> = Rc::new(RefCell::new(
            (items.as_pointer() as Ptr<u32>).offset(
                (items.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize
                            && value_0.read()
                                == ((*sigma.borrow()).offset((*i.borrow()) as isize).read())
                    })
                    .unwrap_or((items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize)
                    as isize,
            ),
        ));
        if !((*it.borrow()) != (items.as_pointer() as Ptr<u32>).to_end()) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("lehmer_code.cc");
                let _l: i32 = 21;
                let _fn: Ptr<u8> = Ptr::from_string_literal("ComputeLehmerCode");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let __rhs = ((((*it.borrow()).get_offset() as isize)
            - ((items.as_pointer() as Ptr<u32>).get_offset() as isize))
            as u32);
        (*code.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        {
            let idx = (*it.borrow()).clone().get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>)
                .to_strong()
                .as_pointer() as Ptr<u32>
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn DecodeLehmerCode_113(code: Ptr<u32>, len: u64, sigma: Ptr<u32>) -> bool {
    let code: Value<Ptr<u32>> = Rc::new(RefCell::new(code));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let sigma: Value<Ptr<u32>> = Rc::new(RefCell::new(sigma));
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(*len.borrow()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let __rhs = ((*i.borrow()) as u32);
        (items.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let index: Value<u32> = Rc::new(RefCell::new(
            ((*code.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        if (((*index.borrow()) as u64) >= (*items.borrow()).len() as u64) {
            return false;
        }
        let value: Value<u32> = Rc::new(RefCell::new(
            ((items.as_pointer() as Ptr<u32>)
                .offset(((*index.borrow()) as u64) as isize)
                .read()),
        ));
        {
            let idx = (items.as_pointer() as Ptr<u32>)
                .offset(((*index.borrow()) as i64) as isize)
                .clone()
                .get_offset();
            (items.as_pointer() as Ptr<Vec<u32>>).with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
            (items.as_pointer() as Ptr<Vec<u32>>)
                .to_strong()
                .as_pointer() as Ptr<u32>
        };
        let __rhs = (*value.borrow());
        (*sigma.borrow())
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn BrunsliDumpAndAbort_79(f: Ptr<u8>, l: i32, fn_: Ptr<u8>) {
    let f: Value<Ptr<u8>> = Rc::new(RefCell::new(f));
    let l: Value<i32> = Rc::new(RefCell::new(l));
    let fn_: Value<Ptr<u8>> = Rc::new(RefCell::new(fn_));
    eprintln!("{}:{} ({})", (*f.borrow()), (*l.borrow()), (*fn_.borrow()));
    if !libcc2rs::cerr().is_null() {
        match libcc2rs::cerr().with_mut(|v| v.sync_all()) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    };
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
    let offset1: Value<i32> = Rc::new(RefCell::new(
        -(*kDCTBlockSize_3.with(Value::clone).borrow()),
    ));
    let offset2: Value<i32> = Rc::new(RefCell::new(-(*stride.borrow())));
    let offset3: Value<i32> = Rc::new(RefCell::new(((*offset2.borrow()) + (*offset1.borrow()))));
    if ((*y.borrow()) != 0) {
        if ((*x.borrow()) != 0) {
            return ({
                let _w: i32 = (((*coeffs.borrow())
                    .offset((*offset1.borrow()) as isize)
                    .read()) as i32);
                let _n: i32 = (((*coeffs.borrow())
                    .offset((*offset2.borrow()) as isize)
                    .read()) as i32);
                let _nw: i32 = (((*coeffs.borrow())
                    .offset((*offset3.borrow()) as isize)
                    .read()) as i32);
                AdaptiveMedian_114(_w, _n, _nw)
            });
        } else {
            return (((*coeffs.borrow())
                .offset((*offset2.borrow()) as isize)
                .read()) as i32);
        }
    } else {
        return if ((*x.borrow()) != 0) {
            (((*coeffs.borrow())
                .offset((*offset1.borrow()) as isize)
                .read()) as i32)
        } else {
            0
        };
    }
    panic!("ub: non-void function does not return a value")
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    pub static kQFactorBits_116: Value<u64> = Rc::new(RefCell::new(6_u64));
);
thread_local!(
    pub static kQFactorLimit_117: Value<u64> = Rc::new(RefCell::new(
        ((1_u32 << (*kQFactorBits_116.with(Value::clone).borrow())) as u64),
    ));
);
pub fn FillQuantMatrix_118(is_chroma: bool, q: u32, dst: Ptr<u8>) {
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let q: Value<u32> = Rc::new(RefCell::new(q));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    if !(((*q.borrow()) >= 0_u32)
        && (((*q.borrow()) as u64) < (*kQFactorLimit_117.with(Value::clone).borrow())))
    {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("quant_matrix.cc");
            let _l: i32 = 18;
            let _fn: Ptr<u8> = Ptr::from_string_literal("FillQuantMatrix");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let in_: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((kDefaultQuantMatrix_12.with(Value::clone).as_pointer() as Ptr<Value<Box<[u8]>>>)
            .offset((*is_chroma.borrow()) as isize)
            .read()
            .as_pointer()) as Ptr<u8>),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        let v: Value<u32> = Rc::new(RefCell::new(
            ((((((*in_.borrow()).offset((*i.borrow()) as isize).read()) as u32)
                .wrapping_mul((*q.borrow())))
            .wrapping_add(32_u32))
                >> 6),
        ));
        let __rhs = (if ((*v.borrow()) < 1_u32) {
            1_u32
        } else {
            if ((*v.borrow()) > 255_u32) {
                255_u32
            } else {
                (*v.borrow())
            }
        } as u8);
        (*dst.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn FindBestMatrix_119(src: Ptr<i32>, is_chroma: bool, dst: Ptr<u8>) -> u32 {
    let src: Value<Ptr<i32>> = Rc::new(RefCell::new(src));
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let best_q: Value<u32> = Rc::new(RefCell::new(0_u32));
    let kMaxDiffCost: Value<u64> = Rc::new(RefCell::new(33_u64));
    let kWorstLen: Value<u64> = Rc::new(RefCell::new(
        (((*kDCTBlockSize_3.with(Value::clone).borrow()) + 1) as u64)
            .wrapping_mul(((*kMaxDiffCost.borrow()).wrapping_add(1_u64)) as u64),
    ));
    let best_len: Value<u64> = Rc::new(RefCell::new((*kWorstLen.borrow())));
    let q: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*q.borrow()) as u64) < (*kQFactorLimit_117.with(Value::clone).borrow())) {
        ({
            let _is_chroma: bool = (*is_chroma.borrow());
            let _q: u32 = (*q.borrow());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            FillQuantMatrix_118(_is_chroma, _q, _dst)
        });
        let last_diff: Value<i32> = Rc::new(RefCell::new(0));
        let len: Value<u64> = Rc::new(RefCell::new(0_u64));
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
            let j: Value<i32> = Rc::new(RefCell::new(
                ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize]
                    as i32),
            ));
            let new_diff: Value<i32> = Rc::new(RefCell::new({
                let _lhs = ((*src.borrow()).offset((*j.borrow()) as isize).read());
                _lhs - (((*dst.borrow()).offset((*j.borrow()) as isize).read()) as i32)
            }));
            let diff: Value<i32> =
                Rc::new(RefCell::new(((*new_diff.borrow()) - (*last_diff.borrow()))));
            (*last_diff.borrow_mut()) = (*new_diff.borrow());
            if ((*diff.borrow()) != 0) {
                let rhs_0 = (*len.borrow()).wrapping_add(1_u64);
                (*len.borrow_mut()) = rhs_0;
                if ((*diff.borrow()) < 0) {
                    let __rhs = -(*diff.borrow());
                    (*diff.borrow_mut()) = __rhs;
                }
                (*diff.borrow_mut()) -= 1;
                if ((*diff.borrow()) == 0) {
                    (*len.borrow_mut()).postfix_inc();
                } else if ((*diff.borrow()) > 65535) {
                    (*len.borrow_mut()) = (*kWorstLen.borrow());
                    break;
                } else {
                    let diff_len: Value<u32> = Rc::new(RefCell::new(
                        ((({
                            let _n: u32 = ((*diff.borrow()) as u32);
                            Log2FloorNonZero_74(_n)
                        }) + 1) as u32),
                    ));
                    if ((*diff_len.borrow()) == 16_u32) {
                        (*diff_len.borrow_mut()).postfix_dec();
                    }
                    let rhs_0 = (*len.borrow()).wrapping_add(
                        ((((2_u32).wrapping_mul((*diff_len.borrow()))).wrapping_add(1_u32)) as u64),
                    );
                    (*len.borrow_mut()) = rhs_0;
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
        let _is_chroma: bool = (*is_chroma.borrow());
        let _q: u32 = (*best_q.borrow());
        let _dst: Ptr<u8> = (*dst.borrow()).clone();
        FillQuantMatrix_118(_is_chroma, _q, _dst)
    });
    return (*best_q.borrow());
}
thread_local!(
    pub static kBitMask_120: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535,
    ])));
);
#[derive(Default)]
pub struct brunsli_WordSource {
    pub data_: Value<Ptr<u8>>,
    pub len_: Value<u64>,
    pub pos_: Value<u64>,
    pub error_: Value<bool>,
    pub optimistic_: Value<bool>,
}
impl brunsli_WordSource {
    pub fn brunsli_WordSource(data: Ptr<u8>, len: u64, optimistic: bool) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<u64> = Rc::new(RefCell::new(len));
        let optimistic: Value<bool> = Rc::new(RefCell::new(optimistic));
        let mut this = Self {
            data_: Rc::new(RefCell::new((*data.borrow()).clone())),
            len_: Rc::new(RefCell::new(((*len.borrow()) & (!1 as u64)))),
            pos_: Rc::new(RefCell::new(0_u64)),
            error_: Rc::new(RefCell::new(false)),
            optimistic_: Rc::new(RefCell::new((*optimistic.borrow()))),
        };
        this
    }
    pub fn GetNextWord(&self) -> u16 {
        let val: Value<u16> = Rc::new(RefCell::new(0_u16));
        if ((*self.pos_.borrow()) < (*self.len_.borrow())) {
            (*val.borrow_mut()) = ({
                let _p: AnyPtr = ((*self.data_.borrow()).offset((*self.pos_.borrow()) as isize)
                    as Ptr<u8>)
                    .to_any();
                BrunsliUnalignedRead16_66(_p)
            });
        } else {
            (*self.error_.borrow_mut()) = true;
        }
        let rhs_0 = (*self.pos_.borrow()).wrapping_add(2_u64);
        (*self.pos_.borrow_mut()) = rhs_0;
        return (*val.borrow());
    }
    pub fn CanRead(&self, n: u64) -> bool {
        let n: Value<u64> = Rc::new(RefCell::new(n));
        if (*self.optimistic_.borrow()) {
            return true;
        }
        let delta: Value<u64> = Rc::new(RefCell::new((2_u64).wrapping_mul((*n.borrow()))));
        let projected_end: Value<u64> = Rc::new(RefCell::new(
            (*self.pos_.borrow()).wrapping_add((*delta.borrow())),
        ));
        if ((*projected_end.borrow()) < (*self.pos_.borrow())) {
            return false;
        }
        return ((*projected_end.borrow()) <= (*self.len_.borrow()));
    }
}
impl Clone for brunsli_WordSource {
    fn clone(&self) -> Self {
        let mut this = Self {
            data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
            len_: Rc::new(RefCell::new((*self.len_.borrow()))),
            pos_: Rc::new(RefCell::new((*self.pos_.borrow()))),
            error_: Rc::new(RefCell::new((*self.error_.borrow()))),
            optimistic_: Rc::new(RefCell::new((*self.optimistic_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_WordSource {}
#[derive()]
pub struct brunsli_BitSource {
    pub val_: Value<u32>,
    pub bit_pos_: Value<i32>,
}
impl brunsli_BitSource {
    pub fn brunsli_BitSource() -> Self {
        let mut this = Self {
            val_: <Value<u32>>::default(),
            bit_pos_: <Value<i32>>::default(),
        };
        this
    }
    pub fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        (*self.val_.borrow_mut()) =
            (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32);
        (*self.bit_pos_.borrow_mut()) = 0;
    }
    pub fn ReadBits(&self, nbits: i32, in_: Ptr<brunsli_WordSource>) -> u32 {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        if (((*self.bit_pos_.borrow()) + (*nbits.borrow())) > 16) {
            let new_bits: Value<u32> = Rc::new(RefCell::new(
                (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32),
            ));
            (*self.val_.borrow_mut()) |= ((*new_bits.borrow()) << 16);
        }
        let result: Value<u32> = Rc::new(RefCell::new(
            (((*self.val_.borrow()) >> (*self.bit_pos_.borrow()))
                & ((*kBitMask_120.with(Value::clone).borrow())[(*nbits.borrow()) as usize] as u32)),
        ));
        (*self.bit_pos_.borrow_mut()) += (*nbits.borrow());
        if ((*self.bit_pos_.borrow()) > 16) {
            (*self.bit_pos_.borrow_mut()) -= 16;
            (*self.val_.borrow_mut()) >>= 16;
        }
        return (*result.borrow());
    }
    pub fn Finish(&self) -> bool {
        let n_bits: Value<u64> = Rc::new(RefCell::new(((16 - (*self.bit_pos_.borrow())) as u64)));
        if ((*n_bits.borrow()) > 0_u64) {
            let padding_bits: Value<i32> = Rc::new(RefCell::new(
                ((((*self.val_.borrow()) >> (*self.bit_pos_.borrow()))
                    & ((*kBitMask_120.with(Value::clone).borrow())[(*n_bits.borrow()) as usize]
                        as u32)) as i32),
            ));
            if ((*padding_bits.borrow()) != 0) {
                return false;
            }
        }
        return true;
    }
}
impl Clone for brunsli_BitSource {
    fn clone(&self) -> Self {
        let mut this = Self {
            val_: Rc::new(RefCell::new((*self.val_.borrow()))),
            bit_pos_: Rc::new(RefCell::new((*self.bit_pos_.borrow()))),
        };
        this
    }
}
impl Default for brunsli_BitSource {
    fn default() -> Self {
        {
            brunsli_BitSource::brunsli_BitSource()
        }
    }
}
impl ByteRepr for brunsli_BitSource {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.val_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.bit_pos_.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            bit_pos_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct brunsli_ANSSymbolInfo {
    pub offset_: Value<u16>,
    pub freq_: Value<u16>,
    pub symbol_: Value<u8>,
}
impl Clone for brunsli_ANSSymbolInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            offset_: Rc::new(RefCell::new((*self.offset_.borrow()))),
            freq_: Rc::new(RefCell::new((*self.freq_.borrow()))),
            symbol_: Rc::new(RefCell::new((*self.symbol_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_ANSSymbolInfo {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.offset_.borrow()).to_bytes(&mut buf[0..2]);
        (*self.freq_.borrow()).to_bytes(&mut buf[2..4]);
        (*self.symbol_.borrow()).to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            offset_: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            freq_: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            symbol_: Rc::new(RefCell::new(<u8>::from_bytes(&buf[4..5]))),
        }
    }
}
#[derive()]
pub struct brunsli_ANSDecodingData {
    pub map_: Value<Box<[brunsli_ANSSymbolInfo]>>,
}
impl brunsli_ANSDecodingData {
    pub fn brunsli_ANSDecodingData() -> Self {
        let mut this = Self {
            map_: Rc::new(RefCell::new(
                (0..1024)
                    .map(|_| <brunsli_ANSSymbolInfo>::default())
                    .collect::<Box<[brunsli_ANSSymbolInfo]>>(),
            )),
        };
        this
    }
}
impl Clone for brunsli_ANSDecodingData {
    fn clone(&self) -> Self {
        let mut this = Self {
            map_: Rc::new(RefCell::new((*self.map_.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_ANSDecodingData {
    fn default() -> Self {
        {
            brunsli_ANSDecodingData::brunsli_ANSDecodingData()
        }
    }
}
impl ByteRepr for brunsli_ANSDecodingData {}
#[derive()]
pub struct brunsli_ANSDecoder {
    state_: Value<u32>,
}
impl brunsli_ANSDecoder {
    pub fn brunsli_ANSDecoder() -> Self {
        let mut this = Self {
            state_: <Value<u32>>::default(),
        };
        this
    }
    pub fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        (*self.state_.borrow_mut()) =
            (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32);
        let __rhs = {
            let _lhs = ((*self.state_.borrow()) << 16_u32);
            _lhs | (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32)
        };
        (*self.state_.borrow_mut()) = __rhs;
    }
    pub fn ReadSymbol(
        &self,
        code: Ptr<brunsli_ANSDecodingData>,
        in_: Ptr<brunsli_WordSource>,
    ) -> i32 {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        let res: Value<u32> = Rc::new(RefCell::new(
            ((*self.state_.borrow())
                & (((*BRUNSLI_ANS_TAB_SIZE_1.with(Value::clone).borrow()) - 1) as u32)),
        ));
        let s: Ptr<brunsli_ANSSymbolInfo> = ((*code.upgrade().deref()).map_.as_pointer()
            as Ptr<brunsli_ANSSymbolInfo>)
            .offset((*res.borrow()) as isize);
        let __rhs = (((*(*s.upgrade().deref()).freq_.borrow()) as u32).wrapping_mul(
            ((*self.state_.borrow()) >> (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow())),
        ))
        .wrapping_add(((*(*s.upgrade().deref()).offset_.borrow()) as u32));
        (*self.state_.borrow_mut()) = __rhs;
        if ((*self.state_.borrow()) < (1_u32 << 16_u32)) {
            let __rhs = {
                let _lhs = ((*self.state_.borrow()) << 16_u32);
                _lhs | (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32)
            };
            (*self.state_.borrow_mut()) = __rhs;
        }
        return ((*(*s.upgrade().deref()).symbol_.borrow()) as i32);
    }
    pub fn CheckCRC(&self) -> bool {
        return ((*self.state_.borrow()) == (19_u32 << 16_u32));
    }
}
impl Clone for brunsli_ANSDecoder {
    fn clone(&self) -> Self {
        let mut this = Self {
            state_: Rc::new(RefCell::new((*self.state_.borrow()))),
        };
        this
    }
}
impl Default for brunsli_ANSDecoder {
    fn default() -> Self {
        {
            brunsli_ANSDecoder::brunsli_ANSDecoder()
        }
    }
}
impl ByteRepr for brunsli_ANSDecoder {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.state_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            state_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
impl brunsli_ANSDecodingData {
    pub fn Init(&self, counts: Ptr<Vec<u32>>) -> bool {
        let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*counts.upgrade().deref()).len() as u64
        } {
            let j: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while {
                let _lhs = (*j.borrow());
                _lhs < (((counts.to_strong().as_pointer() as Ptr<u32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as u64)
            } {
                (*(*self.map_.borrow())[(*pos.borrow()) as usize]
                    .symbol_
                    .borrow_mut()) = ((*i.borrow()) as u8);
                (*(*self.map_.borrow())[(*pos.borrow()) as usize]
                    .freq_
                    .borrow_mut()) = (((counts.to_strong().as_pointer() as Ptr<u32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as u16);
                (*(*self.map_.borrow())[(*pos.borrow()) as usize]
                    .offset_
                    .borrow_mut()) = ((*j.borrow()) as u16);
                (*j.borrow_mut()).prefix_inc();
                (*pos.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        return ((*pos.borrow()) == ((*BRUNSLI_ANS_TAB_SIZE_1.with(Value::clone).borrow()) as u64));
    }
}
#[derive(Default)]
pub struct brunsli_BrunsliBitReader {
    pub next_: Value<Ptr<u8>>,
    pub end_: Value<Ptr<u8>>,
    pub num_bits_: Value<u32>,
    pub bits_: Value<u32>,
    pub num_debt_bytes_: Value<u32>,
    pub is_healthy_: Value<bool>,
    pub is_optimistic_: Value<bool>,
}
impl Clone for brunsli_BrunsliBitReader {
    fn clone(&self) -> Self {
        let mut this = Self {
            next_: Rc::new(RefCell::new((*self.next_.borrow()).clone())),
            end_: Rc::new(RefCell::new((*self.end_.borrow()).clone())),
            num_bits_: Rc::new(RefCell::new((*self.num_bits_.borrow()))),
            bits_: Rc::new(RefCell::new((*self.bits_.borrow()))),
            num_debt_bytes_: Rc::new(RefCell::new((*self.num_debt_bytes_.borrow()))),
            is_healthy_: Rc::new(RefCell::new((*self.is_healthy_.borrow()))),
            is_optimistic_: Rc::new(RefCell::new((*self.is_optimistic_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_BrunsliBitReader {}
pub fn BrunsliBitReaderBitMask_121(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return !((4294967295_u32) << (*n.borrow()));
}
pub fn BrunsliBitReaderOweByte_122(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let rhs_0 = (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()).wrapping_add(8_u32);
    (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = rhs_0;
    (*(*(*br.borrow()).upgrade().deref())
        .num_debt_bytes_
        .borrow_mut())
    .postfix_inc();
}
pub fn BrunsliBitReaderMaybeFetchByte_123(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if {
        let _lhs = (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow());
        _lhs < (*n_bits.borrow())
    } {
        if ((({
            let _lhs = (*(*(*br.borrow()).upgrade().deref()).next_.borrow()).clone();
            _lhs >= (*(*(*br.borrow()).upgrade().deref()).end_.borrow()).clone()
        }) as i64)
            != 0)
        {
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                BrunsliBitReaderOweByte_122(_br)
            });
        } else {
            let __rhs = {
                let _lhs = (((*(*(*br.borrow()).upgrade().deref()).next_.borrow()).read()) as u32);
                _lhs << (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow())
            };
            (*(*(*br.borrow()).upgrade().deref()).bits_.borrow_mut()) |= __rhs;
            let rhs_0 =
                (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()).wrapping_add(8_u32);
            (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = rhs_0;
            (*(*(*br.borrow()).upgrade().deref()).next_.borrow_mut()).postfix_inc();
        }
    }
}
pub fn BrunsliBitReaderGet_124(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) -> u32 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if !((*n_bits.borrow()) <= 24_u32) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("bit_reader.cc");
            let _l: i32 = 110;
            let _fn: Ptr<u8> = Ptr::from_string_literal("BrunsliBitReaderGet");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _n_bits: u32 = (*n_bits.borrow());
        BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
    });
    if ((*n_bits.borrow()) > 8_u32) {
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = (*n_bits.borrow());
            BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
        });
        if ((*n_bits.borrow()) > 16_u32) {
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*n_bits.borrow());
                BrunsliBitReaderMaybeFetchByte_123(_br, _n_bits)
            });
        }
    }
    return {
        let _lhs = (*(*(*br.borrow()).upgrade().deref()).bits_.borrow());
        _lhs & ({
            let _n: u32 = (*n_bits.borrow());
            BrunsliBitReaderBitMask_121(_n)
        })
    };
}
pub fn BrunsliBitReaderDrop_125(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    if !({
        let _lhs = (*n_bits.borrow());
        _lhs <= (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow())
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("bit_reader.cc");
            let _l: i32 = 121;
            let _fn: Ptr<u8> = Ptr::from_string_literal("BrunsliBitReaderDrop");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (*(*(*br.borrow()).upgrade().deref()).bits_.borrow_mut()) >>= (*n_bits.borrow());
    let rhs_0 =
        (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()).wrapping_sub((*n_bits.borrow()));
    (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = rhs_0;
}
pub fn BrunsliBitReaderRead_126(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u32) -> u32 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(n_bits));
    let result: Value<u32> = Rc::new(RefCell::new(
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = (*n_bits.borrow());
            BrunsliBitReaderGet_124(_br, _n_bits)
        }),
    ));
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _n_bits: u32 = (*n_bits.borrow());
        BrunsliBitReaderDrop_125(_br, _n_bits)
    });
    return (*result.borrow());
}
pub fn BrunsliBitReaderInit_127(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = 0_u32;
    (*(*(*br.borrow()).upgrade().deref()).bits_.borrow_mut()) = 0_u32;
    (*(*(*br.borrow()).upgrade().deref())
        .num_debt_bytes_
        .borrow_mut()) = 0_u32;
    (*(*(*br.borrow()).upgrade().deref()).is_healthy_.borrow_mut()) = true;
    (*(*(*br.borrow()).upgrade().deref())
        .is_optimistic_
        .borrow_mut()) = false;
}
pub fn BrunsliBitReaderResume_128(br: Ptr<brunsli_BrunsliBitReader>, buffer: Ptr<u8>, length: u64) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let buffer: Value<Ptr<u8>> = Rc::new(RefCell::new(buffer));
    let length: Value<u64> = Rc::new(RefCell::new(length));
    (*(*(*br.borrow()).upgrade().deref()).next_.borrow_mut()) = (*buffer.borrow()).clone();
    (*(*(*br.borrow()).upgrade().deref()).end_.borrow_mut()) =
        (*buffer.borrow()).offset((*length.borrow()) as isize);
    (*(*(*br.borrow()).upgrade().deref())
        .is_optimistic_
        .borrow_mut()) = false;
}
pub fn BrunsliBitReaderUnload_129(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    'loop_: while ((*(*(*br.borrow()).upgrade().deref()).num_debt_bytes_.borrow()) > 0_u32)
        && ((*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()) >= 8_u32)
    {
        (*(*(*br.borrow()).upgrade().deref())
            .num_debt_bytes_
            .borrow_mut())
        .postfix_dec();
        let rhs_0 = (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()).wrapping_sub(8_u32);
        (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = rhs_0;
    }
    'loop_: while ((*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()) >= 8_u32) {
        (*(*(*br.borrow()).upgrade().deref()).next_.borrow_mut()).postfix_dec();
        let rhs_0 = (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()).wrapping_sub(8_u32);
        (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow_mut()) = rhs_0;
    }
    let __rhs = ({
        let _n: u32 = (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow());
        BrunsliBitReaderBitMask_121(_n)
    });
    (*(*(*br.borrow()).upgrade().deref()).bits_.borrow_mut()) &= __rhs;
}
pub fn BrunsliBitReaderSuspend_130(br: Ptr<brunsli_BrunsliBitReader>) -> u64 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderUnload_129(_br)
    });
    let unused_bytes: Value<u64> = Rc::new(RefCell::new(
        ((((*(*(*br.borrow()).upgrade().deref()).end_.borrow()).clone()
            - (*(*(*br.borrow()).upgrade().deref()).next_.borrow()).clone()) as i64)
            as u64),
    ));
    (*(*(*br.borrow()).upgrade().deref()).next_.borrow_mut()) = Ptr::<u8>::null();
    (*(*(*br.borrow()).upgrade().deref()).end_.borrow_mut()) = Ptr::<u8>::null();
    return (*unused_bytes.borrow());
}
pub fn BrunsliBitReaderFinish_131(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u32> = Rc::new(RefCell::new(
        (*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()),
    ));
    if ((*n_bits.borrow()) >= 8_u32) {
        (*(*(*br.borrow()).upgrade().deref()).is_healthy_.borrow_mut()) = false;
        return;
    }
    if ((*n_bits.borrow()) > 0_u32) {
        let padding_bits: Value<u32> = Rc::new(RefCell::new(
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*n_bits.borrow());
                BrunsliBitReaderRead_126(_br, _n_bits)
            }),
        ));
        if ((*padding_bits.borrow()) != 0_u32) {
            (*(*(*br.borrow()).upgrade().deref()).is_healthy_.borrow_mut()) = false;
        }
    }
}
pub fn BrunsliBitReaderIsHealthy_132(br: Ptr<brunsli_BrunsliBitReader>) -> bool {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderUnload_129(_br)
    });
    return ((*(*(*br.borrow()).upgrade().deref()).num_debt_bytes_.borrow()) == 0_u32)
        && (*(*(*br.borrow()).upgrade().deref()).is_healthy_.borrow());
}
pub fn BrunsliBitReaderSetOptimistic_133(br: Ptr<brunsli_BrunsliBitReader>) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    (*(*(*br.borrow()).upgrade().deref())
        .is_optimistic_
        .borrow_mut()) = true;
}
pub fn BrunsliBitReaderCanRead_134(br: Ptr<brunsli_BrunsliBitReader>, n_bits: u64) -> bool {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let n_bits: Value<u64> = Rc::new(RefCell::new(n_bits));
    if (*(*(*br.borrow()).upgrade().deref()).is_optimistic_.borrow()) {
        return true;
    }
    if ((*(*(*br.borrow()).upgrade().deref()).num_debt_bytes_.borrow()) != 0_u32) {
        return false;
    }
    if {
        let _lhs = ((*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()) as u64);
        _lhs >= (*n_bits.borrow())
    } {
        return true;
    }
    let num_extra_bytes: Value<u64> = Rc::new(RefCell::new(
        ((((*n_bits.borrow())
            .wrapping_sub(((*(*(*br.borrow()).upgrade().deref()).num_bits_.borrow()) as u64)))
        .wrapping_add(7_u64))
            >> 3),
    ));
    return ({
        let _lhs = (*(*(*br.borrow()).upgrade().deref()).next_.borrow())
            .offset((*num_extra_bytes.borrow()) as isize);
        _lhs <= (*(*(*br.borrow()).upgrade().deref()).end_.borrow()).clone()
    });
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
#[derive()]
pub struct brunsli_BrunsliDecoder {
    jpg_: Value<Option<Value<brunsli_JPEGData>>>,
    state_: Value<Option<Value<brunsli_internal_dec_State>>>,
}
impl brunsli_BrunsliDecoder {
    pub fn brunsli_BrunsliDecoder() -> Self {
        let mut this = Self {
            jpg_: Rc::new(RefCell::new(None)),
            state_: Rc::new(RefCell::new(None)),
        };
        {
            let _p: Ptr<_> = Ptr::alloc(brunsli_JPEGData::brunsli_JPEGData());
            (*this.jpg_.borrow_mut()) = _p.to_owned_opt()
        };
        {
            let _p: Ptr<_> = Ptr::alloc(brunsli_internal_dec_State::brunsli_internal_dec_State());
            (*this.state_.borrow_mut()) = _p.to_owned_opt()
        };
        this
    }
}
impl Default for brunsli_BrunsliDecoder {
    fn default() -> Self {
        {
            brunsli_BrunsliDecoder::brunsli_BrunsliDecoder()
        }
    }
}
impl ByteRepr for brunsli_BrunsliDecoder {}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
#[derive()]
pub struct brunsli_BinaryArithmeticDecoder {
    low_: Value<u32>,
    high_: Value<u32>,
    value_: Value<u32>,
}
impl brunsli_BinaryArithmeticDecoder {
    pub fn brunsli_BinaryArithmeticDecoder() -> Self {
        let mut this = Self {
            low_: <Value<u32>>::default(),
            high_: <Value<u32>>::default(),
            value_: <Value<u32>>::default(),
        };
        this
    }
    pub fn Init(&self, in_: Ptr<brunsli_WordSource>) {
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        (*self.low_.borrow_mut()) = 0_u32;
        (*self.high_.borrow_mut()) = !0_u32;
        (*self.value_.borrow_mut()) =
            (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32);
        let __rhs = {
            let _lhs = ((*self.value_.borrow()) << 16_u32);
            _lhs | (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32)
        };
        (*self.value_.borrow_mut()) = __rhs;
    }
    pub fn ReadBit(&self, prob: i32, in_: Ptr<brunsli_WordSource>) -> i32 {
        let prob: Value<i32> = Rc::new(RefCell::new(prob));
        let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
        let diff: Value<u32> = Rc::new(RefCell::new(
            (*self.high_.borrow()).wrapping_sub((*self.low_.borrow())),
        ));
        let split: Value<u32> = Rc::new(RefCell::new(
            ((((*self.low_.borrow()) as u64).wrapping_add(
                ((((*diff.borrow()) as u64).wrapping_mul(((*prob.borrow()) as u64))) >> 8_u32),
            )) as u32),
        ));
        let bit: Value<i32> = <Value<i32>>::default();
        if ((*self.value_.borrow()) > (*split.borrow())) {
            (*self.low_.borrow_mut()) = (*split.borrow()).wrapping_add(1_u32);
            (*bit.borrow_mut()) = 1;
        } else {
            (*self.high_.borrow_mut()) = (*split.borrow());
            (*bit.borrow_mut()) = 0;
        }
        if ((((*self.low_.borrow()) ^ (*self.high_.borrow())) >> 16_u32) == 0_u32) {
            let __rhs = {
                let _lhs = ((*self.value_.borrow()) << 16_u32);
                _lhs | (({ (*(*in_.borrow()).upgrade().deref()).GetNextWord() }) as u32)
            };
            (*self.value_.borrow_mut()) = __rhs;
            (*self.low_.borrow_mut()) <<= 16_u32;
            (*self.high_.borrow_mut()) <<= 16_u32;
            let rhs_0 = (((*self.high_.borrow()) as u32) | 65535_u32) as u32;
            (*self.high_.borrow_mut()) = rhs_0;
        }
        return (*bit.borrow());
    }
}
impl Clone for brunsli_BinaryArithmeticDecoder {
    fn clone(&self) -> Self {
        let mut this = Self {
            low_: Rc::new(RefCell::new((*self.low_.borrow()))),
            high_: Rc::new(RefCell::new((*self.high_.borrow()))),
            value_: Rc::new(RefCell::new((*self.value_.borrow()))),
        };
        this
    }
}
impl Default for brunsli_BinaryArithmeticDecoder {
    fn default() -> Self {
        {
            brunsli_BinaryArithmeticDecoder::brunsli_BinaryArithmeticDecoder()
        }
    }
}
impl ByteRepr for brunsli_BinaryArithmeticDecoder {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.low_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.high_.borrow()).to_bytes(&mut buf[4..8]);
        (*self.value_.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            low_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            high_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            value_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct brunsli_HuffmanCode {
    pub bits: Value<u8>,
    pub value: Value<u16>,
}
impl Clone for brunsli_HuffmanCode {
    fn clone(&self) -> Self {
        let mut this = Self {
            bits: Rc::new(RefCell::new((*self.bits.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_HuffmanCode {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.bits.borrow()).to_bytes(&mut buf[0..1]);
        (*self.value.borrow()).to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bits: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
            value: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
        }
    }
}
#[derive()]
pub struct brunsli_JPEGOutput {
    cb: Value<FnPtr<fn(AnyPtr, Ptr<u8>, u64) -> u64>>,
    data: Value<AnyPtr>,
}
impl brunsli_JPEGOutput {
    pub fn brunsli_JPEGOutput(cb: FnPtr<fn(AnyPtr, Ptr<u8>, u64) -> u64>, data: AnyPtr) -> Self {
        let cb: Value<FnPtr<fn(AnyPtr, Ptr<u8>, u64) -> u64>> = Rc::new(RefCell::new(cb));
        let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
        let mut this = Self {
            cb: Rc::new(RefCell::new((*cb.borrow()).clone())),
            data: Rc::new(RefCell::new((*data.borrow()).clone())),
        };
        this
    }
    pub fn Write(&self, buf: Ptr<u8>, len: u64) -> bool {
        let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
        let len: Value<u64> = Rc::new(RefCell::new(len));
        if ((*len.borrow()) == 0_u64) {
            return true;
        }
        let bytes_written: Value<u64> = Rc::new(RefCell::new(
            ({
                let _arg0: AnyPtr = (*self.data.borrow()).clone();
                let _arg1: Ptr<u8> = (*buf.borrow()).clone();
                let _arg2: u64 = (*len.borrow());
                (*(*self.cb.borrow()))(_arg0, _arg1, _arg2)
            }),
        ));
        return ((*bytes_written.borrow()) == (*len.borrow()));
    }
}
impl Clone for brunsli_JPEGOutput {
    fn clone(&self) -> Self {
        let mut this = Self {
            cb: Rc::new(RefCell::new((*self.cb.borrow()).clone())),
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_JPEGOutput {
    fn default() -> Self {
        brunsli_JPEGOutput {
            cb: Rc::new(RefCell::new(FnPtr::null())),
            data: Rc::new(RefCell::new(AnyPtr::default())),
        }
    }
}
impl ByteRepr for brunsli_JPEGOutput {}
#[derive()]
pub struct brunsli_internal_dec_ComponentMeta {
    pub context_offset: Value<u64>,
    pub h_samp: Value<i32>,
    pub v_samp: Value<i32>,
    pub context_bits: Value<u64>,
    pub ac_stride: Value<i32>,
    pub b_stride: Value<i32>,
    pub width_in_blocks: Value<i32>,
    pub height_in_blocks: Value<i32>,
    pub ac_coeffs: Value<Ptr<i16>>,
    pub block_state: Value<Ptr<u8>>,
    pub quant: Value<Vec<i32>>,
}
impl Clone for brunsli_internal_dec_ComponentMeta {
    fn clone(&self) -> Self {
        let mut this = Self {
            context_offset: Rc::new(RefCell::new((*self.context_offset.borrow()))),
            h_samp: Rc::new(RefCell::new((*self.h_samp.borrow()))),
            v_samp: Rc::new(RefCell::new((*self.v_samp.borrow()))),
            context_bits: Rc::new(RefCell::new((*self.context_bits.borrow()))),
            ac_stride: Rc::new(RefCell::new((*self.ac_stride.borrow()))),
            b_stride: Rc::new(RefCell::new((*self.b_stride.borrow()))),
            width_in_blocks: Rc::new(RefCell::new((*self.width_in_blocks.borrow()))),
            height_in_blocks: Rc::new(RefCell::new((*self.height_in_blocks.borrow()))),
            ac_coeffs: Rc::new(RefCell::new((*self.ac_coeffs.borrow()).clone())),
            block_state: Rc::new(RefCell::new((*self.block_state.borrow()).clone())),
            quant: Rc::new(RefCell::new((*self.quant.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_internal_dec_ComponentMeta {
    fn default() -> Self {
        brunsli_internal_dec_ComponentMeta {
            context_offset: <Value<u64>>::default(),
            h_samp: <Value<i32>>::default(),
            v_samp: <Value<i32>>::default(),
            context_bits: <Value<u64>>::default(),
            ac_stride: <Value<i32>>::default(),
            b_stride: <Value<i32>>::default(),
            width_in_blocks: <Value<i32>>::default(),
            height_in_blocks: <Value<i32>>::default(),
            ac_coeffs: Rc::new(RefCell::new(Ptr::<i16>::null())),
            block_state: Rc::new(RefCell::new(Ptr::<u8>::null())),
            quant: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
impl ByteRepr for brunsli_internal_dec_ComponentMeta {}
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
#[derive(Default)]
pub struct brunsli_Arena {
    pub capacity: Value<u64>,
    pub storage: Value<Option<Value<Box<[brunsli_HuffmanCode]>>>>,
}
impl brunsli_Arena {
    pub fn reserve(&self, limit: u64) {
        let limit: Value<u64> = Rc::new(RefCell::new(limit));
        if ((*self.capacity.borrow()) < (*limit.borrow())) {
            (*self.capacity.borrow_mut()) = (*limit.borrow());
            (*self.storage.borrow_mut()) = Ptr::alloc_array(
                (0..(*self.capacity.borrow()))
                    .map(|_| <brunsli_HuffmanCode>::default())
                    .collect::<Box<[brunsli_HuffmanCode]>>(),
            )
            .to_owned_opt();
        }
    }
    pub fn data(&self) -> Ptr<brunsli_HuffmanCode> {
        return (*self.storage.borrow()).as_pointer();
    }
    pub fn reset(&self) {
        (*self.capacity.borrow_mut()) = 0_u64;
        (*self.storage.borrow_mut()) = None;
    }
}
impl ByteRepr for brunsli_Arena {}
#[derive()]
pub struct brunsli_internal_dec_OutputChunk {
    pub next: Value<Ptr<u8>>,
    pub len: Value<u64>,
    pub buffer: Value<Option<Value<Vec<u8>>>>,
}
impl brunsli_internal_dec_OutputChunk {
    pub fn brunsli_internal_dec_OutputChunk1(data: Ptr<u8>, size: u64) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let size: Value<u64> = Rc::new(RefCell::new(size));
        let mut this = Self {
            next: Rc::new(RefCell::new((*data.borrow()).clone())),
            len: Rc::new(RefCell::new((*size.borrow()))),
            buffer: Rc::new(RefCell::new(None)),
        };
        this
    }
    pub fn brunsli_internal_dec_OutputChunk2(size: Option<u64>) -> Self {
        let size: Value<u64> = Rc::new(RefCell::new(size.unwrap_or(0_u64)));
        let mut this = Self {
            next: Rc::new(RefCell::new(Ptr::<u8>::null())),
            len: <Value<u64>>::default(),
            buffer: Rc::new(RefCell::new(None)),
        };
        {
            let _p: Ptr<_> = Ptr::alloc(
                (0..(*size.borrow()) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            (*this.buffer.borrow_mut()) = _p.to_owned_opt()
        };
        (*this.next.borrow_mut()) = ((*this.buffer.borrow())
            .as_pointer()
            .to_strong()
            .as_pointer() as Ptr<u8>);
        (*this.len.borrow_mut()) = (*size.borrow());
        this
    }
    pub fn brunsli_internal_dec_OutputChunk3(bytes: Vec<u8>) -> Self {
        let bytes: Value<Vec<u8>> = Rc::new(RefCell::new(bytes));
        let mut this = Self {
            next: Rc::new(RefCell::new(Ptr::<u8>::null())),
            len: <Value<u64>>::default(),
            buffer: Rc::new(RefCell::new(None)),
        };
        {
            let _p: Ptr<_> = Ptr::alloc((*bytes.borrow()).clone());
            (*this.buffer.borrow_mut()) = _p.to_owned_opt()
        };
        (*this.next.borrow_mut()) = ((*this.buffer.borrow())
            .as_pointer()
            .to_strong()
            .as_pointer() as Ptr<u8>);
        (*this.len.borrow_mut()) = (*bytes.borrow()).len() as u64;
        this
    }
}
impl Default for brunsli_internal_dec_OutputChunk {
    fn default() -> Self {
        {
            brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(None)
        }
    }
}
impl ByteRepr for brunsli_internal_dec_OutputChunk {}
#[derive()]
pub struct brunsli_HuffmanCodeTable {
    pub depth: Value<Box<[i32]>>,
    pub code: Value<Box<[i32]>>,
}
impl Clone for brunsli_HuffmanCodeTable {
    fn clone(&self) -> Self {
        let mut this = Self {
            depth: Rc::new(RefCell::new((*self.depth.borrow()).clone())),
            code: Rc::new(RefCell::new((*self.code.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_HuffmanCodeTable {
    fn default() -> Self {
        brunsli_HuffmanCodeTable {
            depth: Rc::new(RefCell::new(
                (0..256).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            code: Rc::new(RefCell::new(
                (0..256).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for brunsli_HuffmanCodeTable {}
#[derive(Default)]
pub struct brunsli_internal_dec_BitWriter {
    pub healthy: Value<bool>,
    pub output: Value<Ptr<Vec<brunsli_internal_dec_OutputChunk>>>,
    pub chunk: Value<brunsli_internal_dec_OutputChunk>,
    pub data: Value<Ptr<u8>>,
    pub pos: Value<u64>,
    pub put_buffer: Value<u64>,
    pub put_bits: Value<i32>,
}
impl ByteRepr for brunsli_internal_dec_BitWriter {}
#[derive(Default)]
pub struct brunsli_internal_dec_DCTCodingState {
    pub eob_run_: Value<i32>,
    pub cur_ac_huff_: Value<Ptr<brunsli_HuffmanCodeTable>>,
    pub refinement_bits_: Value<Vec<u16>>,
    pub refinement_bits_count_: Value<u64>,
}
impl Clone for brunsli_internal_dec_DCTCodingState {
    fn clone(&self) -> Self {
        let mut this = Self {
            eob_run_: Rc::new(RefCell::new((*self.eob_run_.borrow()))),
            cur_ac_huff_: Rc::new(RefCell::new((*self.cur_ac_huff_.borrow()).clone())),
            refinement_bits_: Rc::new(RefCell::new((*self.refinement_bits_.borrow()).clone())),
            refinement_bits_count_: Rc::new(RefCell::new((*self.refinement_bits_count_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_DCTCodingState {}
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
#[derive()]
pub struct brunsli_internal_dec_EncodeScanState {
    pub stage: Value<brunsli_internal_dec_EncodeScanState_Stage>,
    pub mcu_y: Value<i32>,
    pub bw: Value<brunsli_internal_dec_BitWriter>,
    pub last_dc_coeff: Value<Box<[i16]>>,
    pub restarts_to_go: Value<i32>,
    pub next_restart_marker: Value<i32>,
    pub block_scan_index: Value<i32>,
    pub coding_state: Value<brunsli_internal_dec_DCTCodingState>,
    pub extra_zero_runs_pos: Value<u64>,
    pub next_extra_zero_run_index: Value<i32>,
    pub next_reset_point_pos: Value<u64>,
    pub next_reset_point: Value<i32>,
}
impl Default for brunsli_internal_dec_EncodeScanState {
    fn default() -> Self {
        brunsli_internal_dec_EncodeScanState {
            stage: <Value<brunsli_internal_dec_EncodeScanState_Stage>>::default(),
            mcu_y: <Value<i32>>::default(),
            bw: <Value<brunsli_internal_dec_BitWriter>>::default(),
            last_dc_coeff: Rc::new(RefCell::new(
                (0..4).map(|_| <i16>::default()).collect::<Box<[i16]>>(),
            )),
            restarts_to_go: <Value<i32>>::default(),
            next_restart_marker: <Value<i32>>::default(),
            block_scan_index: <Value<i32>>::default(),
            coding_state: <Value<brunsli_internal_dec_DCTCodingState>>::default(),
            extra_zero_runs_pos: <Value<u64>>::default(),
            next_extra_zero_run_index: <Value<i32>>::default(),
            next_reset_point_pos: <Value<u64>>::default(),
            next_reset_point: <Value<i32>>::default(),
        }
    }
}
impl ByteRepr for brunsli_internal_dec_EncodeScanState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_SerializationState {
    pub stage: Value<brunsli_internal_dec_SerializationState_Stage>,
    pub output_queue: Value<Vec<brunsli_internal_dec_OutputChunk>>,
    pub section_index: Value<u64>,
    pub dht_index: Value<i32>,
    pub dqt_index: Value<i32>,
    pub app_index: Value<i32>,
    pub com_index: Value<i32>,
    pub data_index: Value<i32>,
    pub scan_index: Value<i32>,
    pub dc_huff_table: Value<Vec<brunsli_HuffmanCodeTable>>,
    pub ac_huff_table: Value<Vec<brunsli_HuffmanCodeTable>>,
    pub pad_bits: Value<Ptr<i32>>,
    pub pad_bits_end: Value<Ptr<i32>>,
    pub seen_dri_marker: Value<bool>,
    pub is_progressive: Value<bool>,
    pub scan_state: Value<brunsli_internal_dec_EncodeScanState>,
}
impl ByteRepr for brunsli_internal_dec_SerializationState {}
#[derive(Default)]
pub struct brunsli_internal_dec_AcDcState {
    pub next_mcu_y: Value<i32>,
    pub next_component: Value<u64>,
    pub next_iy: Value<i32>,
    pub next_x: Value<i32>,
    pub ac_coeffs_order_decoded: Value<bool>,
    pub ac: Value<Vec<brunsli_ComponentState>>,
    pub dc: Value<Vec<brunsli_ComponentStateDC>>,
}
impl Clone for brunsli_internal_dec_AcDcState {
    fn clone(&self) -> Self {
        let mut this = Self {
            next_mcu_y: Rc::new(RefCell::new((*self.next_mcu_y.borrow()))),
            next_component: Rc::new(RefCell::new((*self.next_component.borrow()))),
            next_iy: Rc::new(RefCell::new((*self.next_iy.borrow()))),
            next_x: Rc::new(RefCell::new((*self.next_x.borrow()))),
            ac_coeffs_order_decoded: Rc::new(RefCell::new(
                (*self.ac_coeffs_order_decoded.borrow()),
            )),
            ac: Rc::new(RefCell::new((*self.ac.borrow()).clone())),
            dc: Rc::new(RefCell::new((*self.dc.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_AcDcState {}
#[derive(Default)]
pub struct brunsli_internal_dec_SectionState {
    pub tag: Value<u64>,
    pub is_active: Value<bool>,
    pub is_section: Value<bool>,
    pub tags_met: Value<u32>,
    pub remaining: Value<u64>,
    pub milestone: Value<u64>,
    pub projected_end: Value<u64>,
}
impl Clone for brunsli_internal_dec_SectionState {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            is_active: Rc::new(RefCell::new((*self.is_active.borrow()))),
            is_section: Rc::new(RefCell::new((*self.is_section.borrow()))),
            tags_met: Rc::new(RefCell::new((*self.tags_met.borrow()))),
            remaining: Rc::new(RefCell::new((*self.remaining.borrow()))),
            milestone: Rc::new(RefCell::new((*self.milestone.borrow()))),
            projected_end: Rc::new(RefCell::new((*self.projected_end.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_SectionState {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..8]);
        (*self.is_active.borrow()).to_bytes(&mut buf[8..9]);
        (*self.is_section.borrow()).to_bytes(&mut buf[9..10]);
        (*self.tags_met.borrow()).to_bytes(&mut buf[12..16]);
        (*self.remaining.borrow()).to_bytes(&mut buf[16..24]);
        (*self.milestone.borrow()).to_bytes(&mut buf[24..32]);
        (*self.projected_end.borrow()).to_bytes(&mut buf[32..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            is_active: Rc::new(RefCell::new(<bool>::from_bytes(&buf[8..9]))),
            is_section: Rc::new(RefCell::new(<bool>::from_bytes(&buf[9..10]))),
            tags_met: Rc::new(RefCell::new(<u32>::from_bytes(&buf[12..16]))),
            remaining: Rc::new(RefCell::new(<u64>::from_bytes(&buf[16..24]))),
            milestone: Rc::new(RefCell::new(<u64>::from_bytes(&buf[24..32]))),
            projected_end: Rc::new(RefCell::new(<u64>::from_bytes(&buf[32..40]))),
        }
    }
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
#[derive()]
pub struct brunsli_internal_dec_HeaderState {
    pub stage: Value<u64>,
    pub section: Value<brunsli_internal_dec_SectionState>,
    pub remaining_skip_length: Value<u64>,
    pub varint_values: Value<Vec<u64>>,
}
impl Clone for brunsli_internal_dec_HeaderState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()))),
            section: Rc::new(RefCell::new((*self.section.borrow()).clone())),
            remaining_skip_length: Rc::new(RefCell::new((*self.remaining_skip_length.borrow()))),
            varint_values: Rc::new(RefCell::new((*self.varint_values.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_internal_dec_HeaderState {
    fn default() -> Self {
        brunsli_internal_dec_HeaderState {
            stage: <Value<u64>>::default(),
            section: <Value<brunsli_internal_dec_SectionState>>::default(),
            remaining_skip_length: <Value<u64>>::default(),
            varint_values: Rc::new(RefCell::new(
                std::array::from_fn::<_, 16, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
impl ByteRepr for brunsli_internal_dec_HeaderState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_FallbackState {
    pub stage: Value<u64>,
    pub storage: Value<Vec<u8>>,
}
impl Clone for brunsli_internal_dec_FallbackState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()))),
            storage: Rc::new(RefCell::new((*self.storage.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_FallbackState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_SectionHeaderState {
    pub stage: Value<u64>,
}
impl Clone for brunsli_internal_dec_SectionHeaderState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_SectionHeaderState {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.stage.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            stage: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
        }
    }
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
#[derive(Default)]
pub struct brunsli_internal_dec_VarintState {
    pub stage: Value<brunsli_internal_dec_VarintState_Stage>,
    pub value: Value<u64>,
    pub i: Value<u64>,
}
impl Clone for brunsli_internal_dec_VarintState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()).clone())),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
            i: Rc::new(RefCell::new((*self.i.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_VarintState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_JpegInternalsState {
    pub stage: Value<brunsli_internal_dec_JpegInternalsState_Stage>,
    pub have_dri: Value<bool>,
    pub num_scans: Value<u64>,
    pub dht_count: Value<u64>,
    pub br: Value<brunsli_BrunsliBitReader>,
    pub is_known_last_huffman_code: Value<u64>,
    pub terminal_huffman_code_count: Value<u64>,
    pub is_dc_table: Value<bool>,
    pub total_count: Value<u64>,
    pub space: Value<u64>,
    pub max_len: Value<u64>,
    pub max_count: Value<u64>,
    pub i: Value<u64>,
    pub p: Value<brunsli_PermutationCoder>,
    pub varint: Value<brunsli_internal_dec_VarintState>,
    pub j: Value<u64>,
    pub last_block_idx: Value<i32>,
    pub last_num: Value<i32>,
    pub num_padding_bits: Value<u64>,
    pub intermarker_length: Value<u64>,
}
impl Clone for brunsli_internal_dec_JpegInternalsState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()).clone())),
            have_dri: Rc::new(RefCell::new((*self.have_dri.borrow()))),
            num_scans: Rc::new(RefCell::new((*self.num_scans.borrow()))),
            dht_count: Rc::new(RefCell::new((*self.dht_count.borrow()))),
            br: Rc::new(RefCell::new((*self.br.borrow()).clone())),
            is_known_last_huffman_code: Rc::new(RefCell::new(
                (*self.is_known_last_huffman_code.borrow()),
            )),
            terminal_huffman_code_count: Rc::new(RefCell::new(
                (*self.terminal_huffman_code_count.borrow()),
            )),
            is_dc_table: Rc::new(RefCell::new((*self.is_dc_table.borrow()))),
            total_count: Rc::new(RefCell::new((*self.total_count.borrow()))),
            space: Rc::new(RefCell::new((*self.space.borrow()))),
            max_len: Rc::new(RefCell::new((*self.max_len.borrow()))),
            max_count: Rc::new(RefCell::new((*self.max_count.borrow()))),
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
            varint: Rc::new(RefCell::new((*self.varint.borrow()).clone())),
            j: Rc::new(RefCell::new((*self.j.borrow()))),
            last_block_idx: Rc::new(RefCell::new((*self.last_block_idx.borrow()))),
            last_num: Rc::new(RefCell::new((*self.last_num.borrow()))),
            num_padding_bits: Rc::new(RefCell::new((*self.num_padding_bits.borrow()))),
            intermarker_length: Rc::new(RefCell::new((*self.intermarker_length.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_JpegInternalsState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_QuantDataState {
    pub stage: Value<brunsli_internal_dec_QuantDataState_Stage>,
    pub br: Value<brunsli_BrunsliBitReader>,
    pub i: Value<u64>,
    pub j: Value<u64>,
    pub data_precision: Value<u8>,
    pub vs: Value<brunsli_internal_dec_VarintState>,
    pub delta: Value<i32>,
    pub sign: Value<i32>,
    pub predictor: Value<Vec<u8>>,
}
impl Clone for brunsli_internal_dec_QuantDataState {
    fn clone(&self) -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new((*self.stage.borrow()).clone())),
            br: Rc::new(RefCell::new((*self.br.borrow()).clone())),
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            j: Rc::new(RefCell::new((*self.j.borrow()))),
            data_precision: Rc::new(RefCell::new((*self.data_precision.borrow()))),
            vs: Rc::new(RefCell::new((*self.vs.borrow()).clone())),
            delta: Rc::new(RefCell::new((*self.delta.borrow()))),
            sign: Rc::new(RefCell::new((*self.sign.borrow()))),
            predictor: Rc::new(RefCell::new((*self.predictor.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_QuantDataState {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_HistogramDataState {
    pub stage: Value<brunsli_internal_dec_HistogramDataState_Stage>,
    pub br: Value<brunsli_BrunsliBitReader>,
    pub max_run_length_prefix: Value<u64>,
    pub entropy: Value<Option<Value<brunsli_HuffmanDecodingData>>>,
    pub i: Value<u64>,
    pub counts: Value<Vec<u32>>,
    pub arena: Value<brunsli_Arena>,
}
impl ByteRepr for brunsli_internal_dec_HistogramDataState {}
#[derive(Default)]
pub struct brunsli_internal_dec_Buffer {
    pub data_len: Value<u64>,
    pub borrowed_len: Value<u64>,
    pub data: Value<Vec<u8>>,
    pub external_data: Value<Ptr<u8>>,
    pub external_pos: Value<u64>,
    pub external_len: Value<u64>,
}
impl Clone for brunsli_internal_dec_Buffer {
    fn clone(&self) -> Self {
        let mut this = Self {
            data_len: Rc::new(RefCell::new((*self.data_len.borrow()))),
            borrowed_len: Rc::new(RefCell::new((*self.borrowed_len.borrow()))),
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            external_data: Rc::new(RefCell::new((*self.external_data.borrow()).clone())),
            external_pos: Rc::new(RefCell::new((*self.external_pos.borrow()))),
            external_len: Rc::new(RefCell::new((*self.external_len.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_dec_Buffer {}
#[derive(Default)]
pub struct brunsli_internal_dec_InternalState {
    pub ac_dc: Value<brunsli_internal_dec_AcDcState>,
    pub section: Value<brunsli_internal_dec_SectionState>,
    pub header: Value<brunsli_internal_dec_HeaderState>,
    pub fallback: Value<brunsli_internal_dec_FallbackState>,
    pub section_header: Value<brunsli_internal_dec_SectionHeaderState>,
    pub metadata: Value<brunsli_internal_dec_MetadataState>,
    pub internals: Value<brunsli_internal_dec_JpegInternalsState>,
    pub quant: Value<brunsli_internal_dec_QuantDataState>,
    pub histogram: Value<brunsli_internal_dec_HistogramDataState>,
    pub context_map_: Value<Vec<u8>>,
    pub entropy_codes_: Value<Vec<brunsli_ANSDecodingData>>,
    pub block_state_: Value<Vec<Value<Vec<u8>>>>,
    pub is_meta_warm: Value<bool>,
    pub shallow_histograms: Value<bool>,
    pub num_contexts: Value<u64>,
    pub num_histograms: Value<u64>,
    pub subdecoders_initialized: Value<bool>,
    pub ans_decoder: Value<brunsli_ANSDecoder>,
    pub bit_reader: Value<brunsli_BitSource>,
    pub arith_decoder: Value<brunsli_BinaryArithmeticDecoder>,
    pub result: Value<brunsli_BrunsliStatus>,
    pub last_stage: Value<brunsli_internal_dec_Stage>,
    pub buffer: Value<brunsli_internal_dec_Buffer>,
    pub serialization: Value<brunsli_internal_dec_SerializationState>,
}
impl ByteRepr for brunsli_internal_dec_InternalState {}
thread_local!(
    pub static kNumDirectCodes_135: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kCoeffAlphabetSize_136: Value<i32> = Rc::new(RefCell::new(
        ((*kNumDirectCodes_135.with(Value::clone).borrow()) + 10),
    ));
);
thread_local!(
    pub static kKnownSectionTags_137: Value<u32> = Rc::new(RefCell::new(
        (((((((((1_u32 << ((*kBrunsliSignatureTag_30.with(Value::clone).borrow()) as i32))
            | (1_u32 << ((*kBrunsliHeaderTag_31.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliMetaDataTag_32.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliQuantDataTag_34.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliHistogramDataTag_35.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliDCDataTag_36.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliOriginalJpgTag_38.with(Value::clone).borrow()) as i32))),
    ));
);
thread_local!(
    pub static kKnownHeaderVarintTags_138: Value<u32> = Rc::new(RefCell::new(
        ((((1_u32 << ((*kBrunsliHeaderWidthTag_39.with(Value::clone).borrow()) as i32))
            | (1_u32 << ((*kBrunsliHeaderHeightTag_40.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliHeaderVersionCompTag_41.with(Value::clone).borrow()) as i32)))
            | (1_u32 << ((*kBrunsliHeaderSubsamplingTag_42.with(Value::clone).borrow()) as i32))),
    ));
);
pub fn IsBrunsli_139(data: Ptr<u8>, len: u64) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    thread_local!(
        static kSignature_140: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            10_u8, 4_u8, 66_u8, 210_u8, 213_u8, 78_u8,
        ])));
    );
    thread_local!(
        static kSignatureLen_141: Value<u64> =
            Rc::new(RefCell::new(::std::mem::size_of::<[u8; 6]>() as u64));
    );
    if ((*len.borrow()) < (*kSignatureLen_141.with(Value::clone).borrow())) {
        return false;
    }
    return (((kSignature_140.with(Value::clone).as_pointer() as Ptr<u8>) as Ptr<u8>)
        .to_any()
        .memcmp(
            &((*data.borrow()).clone() as Ptr<u8>).to_any(),
            (*kSignatureLen_141.with(Value::clone).borrow()) as usize,
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
    if (({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _n_bits: u32 = 1_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    }) != 0)
    {
        let nbits: Value<u32> = Rc::new(RefCell::new(
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 3_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }),
        ));
        if ((*nbits.borrow()) == 0_u32) {
            return 1_u32;
        } else {
            return ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*nbits.borrow());
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add((1_u32 << (*nbits.borrow())));
        }
    }
    return 0_u32;
}
pub fn DecodeVarint_144(
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_bits: u64,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_bits: Value<u64> = Rc::new(RefCell::new(max_bits));
    if {
        let _lhs = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_VarintState_Stage::INIT as i32)
    } {
        (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if {
                        let _lhs = (*(*(*s.borrow()).upgrade().deref()).i.borrow());
                        _lhs >= (*max_bits.borrow())
                    } {
                        (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                            brunsli_internal_dec_VarintState_Stage::INIT;
                        return true;
                    }
                    if {
                        let _lhs =
                            (*(*(*s.borrow()).upgrade().deref()).i.borrow()).wrapping_add(1_u64);
                        _lhs != (*max_bits.borrow())
                    } {
                        if !({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if !(({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                                brunsli_internal_dec_VarintState_Stage::INIT;
                            return true;
                        }
                    }
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_VarintState_Stage::READ_DATA;
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = 1_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let next_bit: Value<u64> = Rc::new(RefCell::new(
                        (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as u64),
                    ));
                    let __rhs = {
                        let _lhs = (*next_bit.borrow());
                        _lhs << (*(*(*s.borrow()).upgrade().deref()).i.borrow())
                    };
                    (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) |= __rhs;
                    (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()).prefix_inc();
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 132;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeVarint");
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
pub fn DecodeLimitedVarint_145(
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_symbols: u64,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_symbols: Value<u64> = Rc::new(RefCell::new(max_symbols));
    if {
        let _lhs = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_VarintState_Stage::INIT as i32)
    } {
        (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if {
                        let _lhs = (*(*(*s.borrow()).upgrade().deref()).i.borrow());
                        _lhs < (*max_symbols.borrow())
                    } {
                        if !({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                                brunsli_internal_dec_VarintState_Stage::READ_DATA;
                            continue 'loop_;
                        }
                    }
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                    return true;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = 2_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let next_bits: Value<u64> = Rc::new(RefCell::new(
                        (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = (2_u64 as u32);
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as u64),
                    ));
                    let __rhs = {
                        let _lhs = (*next_bits.borrow());
                        _lhs << ((*(*(*s.borrow()).upgrade().deref()).i.borrow())
                            .wrapping_mul(2_u64 as u64))
                    };
                    (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) |= __rhs;
                    (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()).prefix_inc();
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 169;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeLimitedVarint");
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
pub fn DecodeLimitedVarint_146(
    s: Ptr<brunsli_internal_dec_VarintState>,
    br: Ptr<brunsli_BrunsliBitReader>,
    max_symbols: u64,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_VarintState>> = Rc::new(RefCell::new(s));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let max_symbols: Value<u64> = Rc::new(RefCell::new(max_symbols));
    if {
        let _lhs = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_VarintState_Stage::INIT as i32)
    } {
        (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
    }
    'loop_: while true {
        'switch: {
            let __match_cond = ((*(*(*s.borrow()).upgrade().deref()).stage.borrow()) as i32);
            match __match_cond {
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION as i32) => {
                    if {
                        let _lhs = (*(*(*s.borrow()).upgrade().deref()).i.borrow());
                        _lhs < (*max_symbols.borrow())
                    } {
                        if !({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u64 = 1_u64;
                            BrunsliBitReaderCanRead_134(_br, _n_bits)
                        }) {
                            return false;
                        }
                        if (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) != 0)
                        {
                            (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                                brunsli_internal_dec_VarintState_Stage::READ_DATA;
                            continue 'loop_;
                        }
                    }
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_VarintState_Stage::INIT).clone();
                    return true;
                }
                v if v == (brunsli_internal_dec_VarintState_Stage::READ_DATA as i32) => {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = 8_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return false;
                    }
                    let next_bits: Value<u64> = Rc::new(RefCell::new(
                        (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = (8_u64 as u32);
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as u64),
                    ));
                    let __rhs = {
                        let _lhs = (*next_bits.borrow());
                        _lhs << ((*(*(*s.borrow()).upgrade().deref()).i.borrow())
                            .wrapping_mul(8_u64 as u64))
                    };
                    (*(*(*s.borrow()).upgrade().deref()).value.borrow_mut()) |= __rhs;
                    (*(*(*s.borrow()).upgrade().deref()).i.borrow_mut()).prefix_inc();
                    (*(*(*s.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_VarintState_Stage::READ_CONTINUATION;
                    continue 'loop_;
                }
                _ => {
                    if !(false) {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 169;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeLimitedVarint");
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
pub fn GenerateApp0Marker_147(app0_status: u8) -> Vec<u8> {
    let app0_status: Value<u8> = Rc::new(RefCell::new(app0_status));
    let app0_marker: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (AppData_0xe0_62.with(Value::clone).as_pointer() as Ptr<u8>)
            .offset((17) as isize)
            .get_offset()
            - (AppData_0xe0_62.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
        PtrValueIter::new(
            &(AppData_0xe0_62.with(Value::clone).as_pointer() as Ptr<u8>),
            __count,
        )
        .collect::<Vec<_>>()
    }));
    (app0_marker.as_pointer() as Ptr<u8>)
        .offset(9_u64 as isize)
        .write(
            (if ((((*app0_status.borrow()) as u32) & 1_u32) != 0) {
                2
            } else {
                1
            } as u8),
        );
    let rhs_0 = (((*app0_status.borrow()) as i32) >> 1_u32) as u8;
    (*app0_status.borrow_mut()) = rhs_0;
    (app0_marker.as_pointer() as Ptr<u8>)
        .offset(10_u64 as isize)
        .write(((((*app0_status.borrow()) as u32) & 3_u32) as u8));
    let rhs_0 = (((*app0_status.borrow()) as i32) >> 2_u32) as u8;
    (*app0_status.borrow_mut()) = rhs_0;
    let x_dens: Value<u16> = Rc::new(RefCell::new(
        (*kApp0Densities_46.with(Value::clone).borrow())[(*app0_status.borrow()) as usize],
    ));
    let __rhs = {
        (app0_marker.as_pointer() as Ptr<u8>)
            .offset(13_u64 as isize)
            .write(((((*x_dens.borrow()) as i32) >> 8_u32) as u8));
        ((app0_marker.as_pointer() as Ptr<u8>)
            .offset(13_u64 as isize)
            .read())
    };
    (app0_marker.as_pointer() as Ptr<u8>)
        .offset(11_u64 as isize)
        .write(__rhs);
    let __rhs = {
        (app0_marker.as_pointer() as Ptr<u8>)
            .offset(14_u64 as isize)
            .write(((((*x_dens.borrow()) as u32) & 255_u32) as u8));
        ((app0_marker.as_pointer() as Ptr<u8>)
            .offset(14_u64 as isize)
            .read())
    };
    (app0_marker.as_pointer() as Ptr<u8>)
        .offset(12_u64 as isize)
        .write(__rhs);
    return (*app0_marker.borrow_mut()).clone();
}
pub fn GenerateAppMarker_148(marker: u8, code: u8) -> Vec<u8> {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let code: Value<u8> = Rc::new(RefCell::new(code));
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if (((*marker.borrow()) as i32) == 128) {
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xe2_63.with(Value::clone).as_pointer() as Ptr<u8>)
                .offset((3161) as isize)
                .get_offset()
                - (AppData_0xe2_63.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xe2_63.with(Value::clone).as_pointer() as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        (s.as_pointer() as Ptr<u8>)
            .offset(84_u64 as isize)
            .write((*code.borrow()));
    } else if (((*marker.borrow()) as i32) == 129) {
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xec_64.with(Value::clone).as_pointer() as Ptr<u8>)
                .offset((18) as isize)
                .get_offset()
                - (AppData_0xec_64.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xec_64.with(Value::clone).as_pointer() as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        (s.as_pointer() as Ptr<u8>)
            .offset(15_u64 as isize)
            .write((*code.borrow()));
    } else {
        if !(((*marker.borrow()) as i32) == 130) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 197;
                let _fn: Ptr<u8> = Ptr::from_string_literal("GenerateAppMarker");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        (s.as_pointer() as Ptr<Vec<u8>>).write({
            let __count = (AppData_0xee_65.with(Value::clone).as_pointer() as Ptr<u8>)
                .offset((15) as isize)
                .get_offset()
                - (AppData_0xee_65.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
            PtrValueIter::new(
                &(AppData_0xee_65.with(Value::clone).as_pointer() as Ptr<u8>),
                __count,
            )
            .collect::<Vec<_>>()
        });
        (s.as_pointer() as Ptr<u8>)
            .offset(10_u64 as isize)
            .write((*code.borrow()));
    }
    return (*s.borrow_mut()).clone();
}
pub fn ProcessMetaData_149(
    data: Ptr<u8>,
    len: u64,
    state: Ptr<brunsli_internal_dec_MetadataState>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let state: Value<Ptr<brunsli_internal_dec_MetadataState>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*pos.borrow()) < (*len.borrow())) {
        'switch: {
            let __match_cond = (*(*(*state.borrow()).upgrade().deref()).stage.borrow());
            match __match_cond {
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64) => {
                    let __rhs = ((*data.borrow())
                        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                        .read());
                    (*(*(*state.borrow()).upgrade().deref()).marker.borrow_mut()) = __rhs;
                    if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32) == 217) {
                        ((*(*jpg.borrow()).upgrade().deref()).tail_data.as_pointer()
                            as Ptr<Vec<u8>>)
                            .write(Vec::new());
                        (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64);
                        continue 'loop_;
                    } else if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                        < 64)
                    {
                        (*(*(*state.borrow()).upgrade().deref())
                            .short_marker_count
                            .borrow_mut())
                        .postfix_inc();
                        if {
                            let _lhs = (*(*(*state.borrow()).upgrade().deref())
                                .short_marker_count
                                .borrow());
                            _lhs > ((*kBrunsliShortMarkerLimit_23.with(Value::clone).borrow())
                                as u64)
                        } {
                            return false;
                        }
                        ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer()
                            as Ptr<Vec<Value<Vec<u8>>>>)
                            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                                __v.push(Rc::new(RefCell::new(
                                    ({
                                        let _app0_status: u8 =
                                            (*(*(*state.borrow()).upgrade().deref())
                                                .marker
                                                .borrow());
                                        GenerateApp0Marker_147(_app0_status)
                                    })
                                    .clone(),
                                )))
                            });
                        continue 'loop_;
                    } else if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                        >= 128)
                        && (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                            <= 130)
                    {
                        (*(*(*state.borrow()).upgrade().deref())
                            .short_marker_count
                            .borrow_mut())
                        .postfix_inc();
                        if {
                            let _lhs = (*(*(*state.borrow()).upgrade().deref())
                                .short_marker_count
                                .borrow());
                            _lhs > ((*kBrunsliShortMarkerLimit_23.with(Value::clone).borrow())
                                as u64)
                        } {
                            return false;
                        }
                        (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_MetadataState_Stage::READ_CODE as u64);
                        continue 'loop_;
                    }
                    if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32) != 254)
                        && ((((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                            >> 4_u32)
                            != 14)
                    {
                        return false;
                    }
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_HI as u64);
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64) => {
                    ({
                        let _dst: Ptr<Vec<u8>> =
                            ((*(*jpg.borrow()).upgrade().deref()).tail_data.as_pointer());
                        let _begin: Ptr<u8> = (*data.borrow()).offset((*pos.borrow()) as isize);
                        let _end: Ptr<u8> = (*data.borrow()).offset((*len.borrow()) as isize);
                        Append_71(_dst, _begin, _end)
                    });
                    (*pos.borrow_mut()) = (*len.borrow());
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_CODE as u64) => {
                    let code: Value<u8> = Rc::new(RefCell::new(
                        ((*data.borrow())
                            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                            .read()),
                    ));
                    ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer()
                        as Ptr<Vec<Value<Vec<u8>>>>)
                        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                            __v.push(Rc::new(RefCell::new(
                                ({
                                    let _marker: u8 =
                                        (*(*(*state.borrow()).upgrade().deref()).marker.borrow());
                                    let _code: u8 = (*code.borrow());
                                    GenerateAppMarker_148(_marker, _code)
                                })
                                .clone(),
                            )))
                        });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64);
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_HI as u64) => {
                    let __rhs = ((*data.borrow())
                        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                        .read());
                    (*(*(*state.borrow()).upgrade().deref())
                        .length_hi
                        .borrow_mut()) = __rhs;
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_LO as u64);
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_LENGTH_LO as u64) => {
                    let lo: Value<u8> = Rc::new(RefCell::new(
                        ((*data.borrow())
                            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                            .read()),
                    ));
                    let marker_len: Value<u64> = Rc::new(RefCell::new(
                        (({
                            let _lhs = (((*(*(*state.borrow()).upgrade().deref())
                                .length_hi
                                .borrow()) as i32)
                                << 8_u32);
                            _lhs + ((*lo.borrow()) as i32)
                        }) as u64),
                    ));
                    if ((*marker_len.borrow()) < 2_u64) {
                        return false;
                    }
                    (*(*(*state.borrow()).upgrade().deref())
                        .remaining_multibyte_length
                        .borrow_mut()) = (*marker_len.borrow()).wrapping_sub(2_u64);
                    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
                        (*(*(*state.borrow()).upgrade().deref()).marker.borrow()),
                        (*(*(*state.borrow()).upgrade().deref()).length_hi.borrow()),
                        (*lo.borrow()),
                    ])));
                    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> = Rc::new(RefCell::new(
                        if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                            == 254)
                        {
                            ((*(*jpg.borrow()).upgrade().deref()).com_data.as_pointer())
                        } else {
                            ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer())
                        },
                    ));
                    let delta: Value<u64> = Rc::new(RefCell::new(
                        if (((*(*(*state.borrow()).upgrade().deref()).marker.borrow()) as i32)
                            == 254)
                        {
                            0_u64
                        } else {
                            (*(*(*state.borrow()).upgrade().deref())
                                .short_marker_count
                                .borrow())
                        },
                    ));
                    if {
                        let _lhs = ((*(*dest.borrow()).upgrade().deref()).len() as u64)
                            .wrapping_sub((*delta.borrow()));
                        _lhs >= ((*kBrunsliMultibyteMarkerLimit_24.with(Value::clone).borrow())
                            as u64)
                    } {
                        return false;
                    }
                    (*dest.borrow()).to_strong().as_pointer().with_mut(
                        |__v: &mut Vec<Value<Vec<u8>>>| {
                            __v.push(Rc::new(RefCell::new({
                                let __count = (head.as_pointer() as Ptr<u8>)
                                    .offset((3) as isize)
                                    .get_offset()
                                    - (head.as_pointer() as Ptr<u8>).get_offset();
                                PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                                    .map(|item| u8::try_from(item).ok().unwrap())
                                    .collect::<Vec<_>>()
                            })))
                        },
                    );
                    (*(*(*state.borrow()).upgrade().deref())
                        .multibyte_sink
                        .borrow_mut()) = ((*(*dest.borrow()).upgrade().deref())
                        [(*(*dest.borrow()).upgrade().deref()).len() - 1]
                        .as_pointer());
                    let __rhs = (if ((*(*(*state.borrow()).upgrade().deref())
                        .remaining_multibyte_length
                        .borrow())
                        > 0_u64)
                    {
                        brunsli_internal_dec_MetadataState_Stage::READ_MULTIBYTE
                    } else {
                        brunsli_internal_dec_MetadataState_Stage::READ_MARKER
                    } as u64);
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_MetadataState_Stage::READ_MULTIBYTE as u64) => {
                    let chunk_size: Value<u64> = Rc::new(RefCell::new({
                        let __tmp_1: Value<u64> =
                            Rc::new(RefCell::new((*len.borrow()).wrapping_sub((*pos.borrow()))));
                        (if (*(*state.borrow()).upgrade().deref())
                            .remaining_multibyte_length
                            .as_pointer()
                            .read()
                            <= __tmp_1.as_pointer().read()
                        {
                            (*(*state.borrow()).upgrade().deref())
                                .remaining_multibyte_length
                                .as_pointer()
                        } else {
                            __tmp_1.as_pointer()
                        }
                        .read())
                    }));
                    ({
                        let _dst: Ptr<Vec<u8>> = (*(*(*state.borrow()).upgrade().deref())
                            .multibyte_sink
                            .borrow())
                        .clone();
                        let _begin: Ptr<u8> = (*data.borrow()).offset((*pos.borrow()) as isize);
                        let _length: u64 = (*chunk_size.borrow());
                        Append_72(_dst, _begin, _length)
                    });
                    let rhs_0 = (*(*(*state.borrow()).upgrade().deref())
                        .remaining_multibyte_length
                        .borrow())
                    .wrapping_sub((*chunk_size.borrow()));
                    (*(*(*state.borrow()).upgrade().deref())
                        .remaining_multibyte_length
                        .borrow_mut()) = rhs_0;
                    let rhs_0 = (*pos.borrow()).wrapping_add((*chunk_size.borrow()));
                    (*pos.borrow_mut()) = rhs_0;
                    if ((*(*(*state.borrow()).upgrade().deref())
                        .remaining_multibyte_length
                        .borrow())
                        == 0_u64)
                    {
                        (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64);
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
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> =
        (*s.upgrade().deref()).internals.as_pointer();
    let br: Value<Ptr<brunsli_BrunsliBitReader>> =
        Rc::new(RefCell::new(((*js.upgrade().deref()).br.as_pointer())));
    'loop_: while true {
        'switch: {
            let __match_cond = ((*(*js.upgrade().deref()).stage.borrow()) as i32);
            match __match_cond {
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST
                        as i32) =>
                {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = 1_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    (*(*js.upgrade().deref())
                        .is_known_last_huffman_code
                        .borrow_mut()) = (({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as u64);
                    (*(*jpg.borrow()).upgrade().deref())
                        .huffman_code
                        .as_pointer()
                        .with_mut(|__v: &mut Vec<brunsli_JPEGHuffmanCode>| {
                            __v.push(<brunsli_JPEGHuffmanCode>::default())
                        });
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_SIMPLE;
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_SIMPLE
                        as i32) =>
                {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = ((5
                            + (!((*(*js.upgrade().deref()).is_known_last_huffman_code.borrow())
                                != 0) as i32)) as u64);
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*(*jpg.borrow()).upgrade().deref())
                            .huffman_code
                            .as_pointer()
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    (*(*(*huff.borrow()).upgrade().deref()).slot_id.borrow_mut()) = (({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 2_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    })
                        as i32);
                    (*(*js.upgrade().deref()).is_dc_table.borrow_mut()) = (({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) == 0_u32);
                    (*(*(*huff.borrow()).upgrade().deref()).slot_id.borrow_mut()) +=
                        if (*(*js.upgrade().deref()).is_dc_table.borrow()) {
                            0
                        } else {
                            16
                        };
                    (*(*(*huff.borrow()).upgrade().deref()).is_last.borrow_mut()) =
                        ((*(*js.upgrade().deref()).is_known_last_huffman_code.borrow()) != 0)
                            || (({
                                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                                let _n_bits: u32 = 1_u32;
                                BrunsliBitReaderRead_126(_br, _n_bits)
                            }) != 0);
                    ((*(*huff.borrow()).upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                        .offset(0_u64 as isize)
                        .write(0);
                    let found_match: Value<i32> = Rc::new(RefCell::new(
                        (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as i32),
                    ));
                    if ((*found_match.borrow()) != 0) {
                        if (*(*js.upgrade().deref()).is_dc_table.borrow()) {
                            let huff_table_idx: Value<i32> = Rc::new(RefCell::new(
                                (({
                                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                                    let _n_bits: u32 = 1_u32;
                                    BrunsliBitReaderRead_126(_br, _n_bits)
                                }) as i32),
                            ));
                            {
                                ((((*(*huff.borrow()).upgrade().deref()).counts.as_pointer()
                                    as Ptr<i32>)
                                    .offset(1_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeCounts_54
                                            .with(Value::clone)
                                            .as_pointer()
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()) as isize)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as u64 as usize,
                                    );
                                ((((*(*huff.borrow()).upgrade().deref()).counts.as_pointer()
                                    as Ptr<i32>)
                                    .offset(1_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .clone()
                            };
                            {
                                ((((*(*huff.borrow()).upgrade().deref()).values.as_pointer()
                                    as Ptr<i32>)
                                    .offset(0_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockDCHuffmanCodeValues_55
                                            .with(Value::clone)
                                            .as_pointer()
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()) as isize)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 13]>() as u64 as usize,
                                    );
                                ((((*(*huff.borrow()).upgrade().deref()).values.as_pointer()
                                    as Ptr<i32>)
                                    .offset(0_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .clone()
                            };
                        } else {
                            let huff_table_idx: Value<i32> = Rc::new(RefCell::new(
                                (({
                                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                                    let _n_bits: u32 = 1_u32;
                                    BrunsliBitReaderRead_126(_br, _n_bits)
                                }) as i32),
                            ));
                            {
                                ((((*(*huff.borrow()).upgrade().deref()).counts.as_pointer()
                                    as Ptr<i32>)
                                    .offset(1_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeCounts_57
                                            .with(Value::clone)
                                            .as_pointer()
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()) as isize)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 16]>() as u64 as usize,
                                    );
                                ((((*(*huff.borrow()).upgrade().deref()).counts.as_pointer()
                                    as Ptr<i32>)
                                    .offset(1_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .clone()
                            };
                            {
                                ((((*(*huff.borrow()).upgrade().deref()).values.as_pointer()
                                    as Ptr<i32>)
                                    .offset(0_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .memcpy(
                                        &((((kStockACHuffmanCodeValues_59
                                            .with(Value::clone)
                                            .as_pointer()
                                            as Ptr<Value<Box<[i32]>>>)
                                            .offset((*huff_table_idx.borrow()) as isize)
                                            .read()
                                            .as_pointer())
                                            as Ptr<i32>)
                                            as Ptr<i32>)
                                            .to_any(),
                                        ::std::mem::size_of::<[i32; 163]>() as u64 as usize,
                                    );
                                ((((*(*huff.borrow()).upgrade().deref()).values.as_pointer()
                                    as Ptr<i32>)
                                    .offset(0_u64 as isize))
                                    as Ptr<i32>)
                                    .to_any()
                                    .clone()
                            };
                        }
                        (*(*js.upgrade().deref()).stage.borrow_mut()) =
                            brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE;
                    } else {
                        ({
                            let _values: Vec<u8> = if (*(*js.upgrade().deref())
                                .is_dc_table
                                .borrow())
                            {
                                {
                                    let __count =
                                        (kDefaultDCValues_60.with(Value::clone).as_pointer()
                                            as Ptr<u8>)
                                            .to_end()
                                            .get_offset()
                                            - (kDefaultDCValues_60.with(Value::clone).as_pointer()
                                                as Ptr<u8>)
                                                .get_offset();
                                    PtrValueIter::new(
                                        &(kDefaultDCValues_60.with(Value::clone).as_pointer()
                                            as Ptr<u8>),
                                        __count,
                                    )
                                    .collect::<Vec<_>>()
                                }
                            } else {
                                {
                                    let __count =
                                        (kDefaultACValues_61.with(Value::clone).as_pointer()
                                            as Ptr<u8>)
                                            .to_end()
                                            .get_offset()
                                            - (kDefaultACValues_61.with(Value::clone).as_pointer()
                                                as Ptr<u8>)
                                                .get_offset();
                                    PtrValueIter::new(
                                        &(kDefaultACValues_61.with(Value::clone).as_pointer()
                                            as Ptr<u8>),
                                        __count,
                                    )
                                    .collect::<Vec<_>>()
                                }
                            };
                            (*(*js.upgrade().deref()).p.borrow()).Init(_values)
                        });
                        (*(*js.upgrade().deref()).stage.borrow_mut()) =
                            brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_MAX_LEN;
                    };
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_MAX_LEN
                        as i32) =>
                {
                    if !({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u64 = 4_u64;
                        BrunsliBitReaderCanRead_134(_br, _n_bits)
                    }) {
                        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    }
                    (*(*js.upgrade().deref()).max_len.borrow_mut()) = ((({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 4_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    })
                    .wrapping_add(1_u32))
                        as u64);
                    (*(*js.upgrade().deref()).total_count.borrow_mut()) = 0_u64;
                    let __rhs = (if (*(*js.upgrade().deref()).is_dc_table.borrow()) {
                        (*kJpegDCAlphabetSize_9.with(Value::clone).borrow())
                    } else {
                        (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow())
                    } as u64);
                    (*(*js.upgrade().deref()).max_count.borrow_mut()) = __rhs;
                    let __rhs = ((((1_u32
                        << (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()))
                        as u32)
                        .wrapping_sub(
                            (1_u32
                                << (((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())
                                    as u64)
                                    .wrapping_sub((*(*js.upgrade().deref()).max_len.borrow())))),
                        )) as u64);
                    (*(*js.upgrade().deref()).space.borrow_mut()) = __rhs;
                    (*(*js.upgrade().deref()).i.borrow_mut()) = 1_u64;
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_COUNT;
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_COUNT
                        as i32) =>
                {
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*(*jpg.borrow()).upgrade().deref())
                            .huffman_code
                            .as_pointer()
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    if {
                        let _lhs = (*(*js.upgrade().deref()).i.borrow());
                        _lhs <= (*(*js.upgrade().deref()).max_len.borrow())
                    } {
                        let shift: Value<u64> = Rc::new(RefCell::new(
                            ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) as u64)
                                .wrapping_sub((*(*js.upgrade().deref()).i.borrow())),
                        ));
                        let count_limit: Value<u64> = Rc::new(RefCell::new({
                            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                                (*(*js.upgrade().deref()).max_count.borrow())
                                    .wrapping_sub((*(*js.upgrade().deref()).total_count.borrow())),
                            ));
                            let __tmp_1: Value<u64> = Rc::new(RefCell::new({
                                let _lhs = (*(*js.upgrade().deref()).space.borrow());
                                _lhs >> (*shift.borrow())
                            }));
                            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                                __tmp_0.as_pointer()
                            } else {
                                __tmp_1.as_pointer()
                            }
                            .read())
                        }));
                        if ((*count_limit.borrow()) > 0_u64) {
                            let nbits: Value<i32> = Rc::new(RefCell::new(
                                (({
                                    let _n: u32 = ((*count_limit.borrow()) as u32);
                                    Log2FloorNonZero_74(_n)
                                }) + 1),
                            ));
                            if !({
                                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                                let _n_bits: u64 = ((*nbits.borrow()) as u64);
                                BrunsliBitReaderCanRead_134(_br, _n_bits)
                            }) {
                                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            }
                            let count: Value<u64> = Rc::new(RefCell::new(
                                (({
                                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                                    let _n_bits: u32 = ((*nbits.borrow()) as u32);
                                    BrunsliBitReaderRead_126(_br, _n_bits)
                                }) as u64),
                            ));
                            if ((*count.borrow()) > (*count_limit.borrow())) {
                                return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            }
                            ((*(*huff.borrow()).upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                                .offset((*(*js.upgrade().deref()).i.borrow()) as isize)
                                .write(((*count.borrow()) as i32));
                            let rhs_0 = (*(*js.upgrade().deref()).total_count.borrow())
                                .wrapping_add((*count.borrow()));
                            (*(*js.upgrade().deref()).total_count.borrow_mut()) = rhs_0;
                            let rhs_0 = (*(*js.upgrade().deref()).space.borrow()).wrapping_sub(
                                (*count.borrow()).wrapping_mul((1_u64 << (*shift.borrow()))),
                            );
                            (*(*js.upgrade().deref()).space.borrow_mut()) = rhs_0;
                        }
                        (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
                        continue 'loop_;
                    }
                    ((*(*huff.borrow()).upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                        .offset((*(*js.upgrade().deref()).max_len.borrow()) as isize)
                        .with_mut(|__v| __v.prefix_inc());
                    (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_PERMUTATION;
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_PERMUTATION
                        as i32) =>
                {
                    let huff: Value<Ptr<brunsli_JPEGHuffmanCode>> = Rc::new(RefCell::new(
                        (((*(*jpg.borrow()).upgrade().deref())
                            .huffman_code
                            .as_pointer()
                            as Ptr<brunsli_JPEGHuffmanCode>)
                            .to_last()),
                    ));
                    if {
                        let _lhs = (*(*js.upgrade().deref()).i.borrow());
                        _lhs < (*(*js.upgrade().deref()).total_count.borrow())
                    } {
                        let nbits: Value<i32> = Rc::new(RefCell::new(
                            ({ (*(*js.upgrade().deref()).p.borrow()).num_bits() }),
                        ));
                        if !({
                            let _s: Ptr<brunsli_internal_dec_VarintState> =
                                ((*js.upgrade().deref()).varint.as_pointer());
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _max_symbols: u64 = ((((*nbits.borrow()) + 1) >> 1_u32) as u64);
                            DecodeLimitedVarint_145(_s, _br, _max_symbols)
                        }) {
                            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let value: Value<u8> = <Value<u8>>::default();
                        if !({
                            let _code: u64 =
                                (*(*(*js.upgrade().deref()).varint.borrow()).value.borrow());
                            let _value: Ptr<u8> = (value.as_pointer());
                            (*(*js.upgrade().deref()).p.borrow()).Remove(_code, _value)
                        }) {
                            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        }
                        ((*(*huff.borrow()).upgrade().deref()).values.as_pointer() as Ptr<i32>)
                            .offset((*(*js.upgrade().deref()).i.borrow()) as isize)
                            .write(((*value.borrow()) as i32));
                        (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
                        continue 'loop_;
                    }
                    ((*(*huff.borrow()).upgrade().deref()).values.as_pointer() as Ptr<i32>)
                        .offset((*(*js.upgrade().deref()).total_count.borrow()) as isize)
                        .write((*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow()));
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE;
                    continue 'loop_;
                }
                v if v
                    == (brunsli_internal_dec_JpegInternalsState_Stage::HUFFMAN_UPDATE as i32) =>
                {
                    if (*(*((*(*jpg.borrow()).upgrade().deref())
                        .huffman_code
                        .as_pointer() as Ptr<brunsli_JPEGHuffmanCode>)
                        .to_last()
                        .upgrade()
                        .deref())
                    .is_last
                    .borrow())
                    {
                        (*(*js.upgrade().deref())
                            .terminal_huffman_code_count
                            .borrow_mut())
                        .postfix_inc();
                    }
                    if ((*(*js.upgrade().deref()).is_known_last_huffman_code.borrow()) != 0) {
                        ({ (*(*js.upgrade().deref()).p.borrow()).Clear() });
                        return brunsli_BrunsliStatus::BRUNSLI_OK;
                    }
                    if {
                        let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).huffman_code.borrow())
                            .len() as u64;
                        _lhs >= ((*kMaxDHTMarkers_10.with(Value::clone).borrow()) as u64)
                    } {
                        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    }
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST;
                    continue 'loop_;
                }
                _ => {
                    return (brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR).clone();
                }
            }
        };
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn DecodeScanInfo_151(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> =
        (*s.upgrade().deref()).internals.as_pointer();
    let br: Value<Ptr<brunsli_BrunsliBitReader>> =
        Rc::new(RefCell::new(((*js.upgrade().deref()).br.as_pointer())));
    let maybe_add_zero_run: Value<_> = Rc::new(RefCell::new(
        (|| {
            if ((*(*js.upgrade().deref()).last_num.borrow()) > 0) {
                let info: Value<brunsli_JPEGScanInfo_ExtraZeroRunInfo> = Rc::new(RefCell::new(
                    <brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default(),
                ));
                (*(*info.borrow()).block_idx.borrow_mut()) =
                    (*(*js.upgrade().deref()).last_block_idx.borrow());
                (*(*info.borrow()).num_extra_zero_runs.borrow_mut()) =
                    (*(*js.upgrade().deref()).last_num.borrow());
                {
                    let a0_clone = (*info.borrow()).clone();
                    (*(*((*(*jpg.borrow()).upgrade().deref()).scan_info.as_pointer()
                        as Ptr<brunsli_JPEGScanInfo>)
                        .offset((*(*js.upgrade().deref()).i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .extra_zero_runs
                    .borrow_mut())
                    .push(a0_clone)
                };
                (*(*js.upgrade().deref()).last_num.borrow_mut()) = 0;
            }
        }),
    ));
    'loop_: while true {
        'switch: {
            let __match_cond = ((*(*js.upgrade().deref()).stage.borrow()) as i32);
            match __match_cond { v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON as i32 ) )  => { let  si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*(*jpg.borrow()) .upgrade().deref()) . scan_info  .as_pointer()  as Ptr<brunsli_JPEGScanInfo>).offset((*(*js.upgrade().deref()) . i .borrow())  as isize)  ) )) ;
 ;
 ;
 if ! (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u64   = 22_u64  ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } (*(*(*si.borrow()) .upgrade().deref()) . Ss .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 6_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*(*(*si.borrow()) .upgrade().deref()) . Se .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 6_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*(*(*si.borrow()) .upgrade().deref()) . Ah .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 4_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*(*(*si.borrow()) .upgrade().deref()) . Al .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 4_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*(*(*si.borrow()) .upgrade().deref()) . num_components .borrow_mut())  = ( ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 2_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  ) . wrapping_add ( 1_u32 ) ) as u64 )  ;
 (*(*js.upgrade().deref()) . j .borrow_mut())  = 0_u64  ;
 (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMPONENT  ;
 ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMPONENT as i32 ) )  => { let  si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*(*jpg.borrow()) .upgrade().deref()) . scan_info  .as_pointer()  as Ptr<brunsli_JPEGScanInfo>).offset((*(*js.upgrade().deref()) . i .borrow())  as isize)  ) )) ;
 ;
 ;
 if { let _lhs = (*(*js.upgrade().deref()) . j .borrow()) ; _lhs < (*(*(*si.borrow()) .upgrade().deref()) . num_components .borrow())  } { if ! (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u64   = 6_u64  ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } (*( * ((*(*si.borrow()) .upgrade().deref()) . components  .as_pointer()  as Ptr<brunsli_JPEGComponentScanInfo>).offset((*(*js.upgrade().deref()) . j .borrow())  as isize) .upgrade().deref() ) . comp_idx .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 2_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as u8 ) )  ;
 (*( * ((*(*si.borrow()) .upgrade().deref()) . components  .as_pointer()  as Ptr<brunsli_JPEGComponentScanInfo>).offset((*(*js.upgrade().deref()) . j .borrow())  as isize) .upgrade().deref() ) . dc_tbl_idx .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 2_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*( * ((*(*si.borrow()) .upgrade().deref()) . components  .as_pointer()  as Ptr<brunsli_JPEGComponentScanInfo>).offset((*(*js.upgrade().deref()) . j .borrow())  as isize) .upgrade().deref() ) . ac_tbl_idx .borrow_mut())  = ( ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 2_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  as i32 ) )  ;
 (*(*js.upgrade().deref()) . j .borrow_mut())  . postfix_inc () ;
 } else { (*(*js.upgrade().deref()) . last_block_idx .borrow_mut())  = - 1_i32  ;
 (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION  ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION as i32 ) )  => { if ! (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u64   = 1_u64  ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } if ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 1_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  != 0 ) { (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_DATA  ;
 } else { (*(*js.upgrade().deref()) . last_block_idx .borrow_mut())  = 0  ;
 (*(*js.upgrade().deref()) . last_num .borrow_mut())  = 0  ;
 (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION  ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_DATA as i32 ) )  => { let  si : Value<Ptr<brunsli_JPEGScanInfo> > = Rc::new(RefCell::new(( ((*(*jpg.borrow()) .upgrade().deref()) . scan_info  .as_pointer()  as Ptr<brunsli_JPEGScanInfo>).offset((*(*js.upgrade().deref()) . i .borrow())  as isize)  ) )) ;
 ;
 ;
 if ! (  { let _s: Ptr<brunsli_internal_dec_VarintState>   = ( (*js.upgrade().deref()) . varint  .as_pointer()  )  ; let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _max_bits: u64   = 28_u64  ; DecodeVarint_144 ( _s , _br , _max_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } let  block_idx : Value<i32 > = Rc::new(RefCell::new(( { let _lhs = (*(*js.upgrade().deref()) . last_block_idx .borrow()) ; _lhs + ( ( (*(*(*js.upgrade().deref()) . varint .borrow()) . value .borrow()) as i32 ) )  } + 1 ) )) ;
 ;
 ;
 (*(*si.borrow()) .upgrade().deref()) . reset_points  .as_pointer()  .with_mut(|__v: &mut  Vec<i32>   | __v.push( (*block_idx.borrow_mut()) as i32 ))  ;
 (*(*js.upgrade().deref()) . last_block_idx .borrow_mut())  = (*block_idx.borrow())  ;
 if ( (*(*js.upgrade().deref()) . last_block_idx .borrow()) > ( ( 1 << 30 ) ) ) { return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN  ;
 } (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_RESET_POINT_CONTINUATION  ;
 ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION as i32 ) )  => { if ! (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u64   = 1_u64  ; BrunsliBitReaderCanRead_134 ( _br , _n_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } if ( (  { let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _n_bits: u32   = 1_u32  ; BrunsliBitReaderRead_126 ( _br , _n_bits , ) } )  != 0 ) { (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_DATA  ;
 } else { (  { (*maybe_add_zero_run.borrow_mut()) ( ) } ) ;
 (*(*js.upgrade().deref()) . i .borrow_mut())  . prefix_inc () ;
 if { let _lhs = (*(*js.upgrade().deref()) . i .borrow()) ; _lhs < (*(*js.upgrade().deref()) . num_scans .borrow())  } { (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON  ;
 ;
 continue 'loop_ ;
 } return (brunsli_BrunsliStatus::BRUNSLI_OK ).clone() ;
 } ;
 continue 'loop_ ;
 }, v if v ==  ( ( brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_DATA as i32 ) )  => { if ! (  { let _s: Ptr<brunsli_internal_dec_VarintState>   = ( (*js.upgrade().deref()) . varint  .as_pointer()  )  ; let _br: Ptr<brunsli_BrunsliBitReader>   = ((*br.borrow()) ).clone() ; let _max_bits: u64   = 28_u64  ; DecodeVarint_144 ( _s , _br , _max_bits , ) } )  { return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA  ;
 } let  block_idx : Value<i32 > = Rc::new(RefCell::new({ let _lhs = (*(*js.upgrade().deref()) . last_block_idx .borrow()) ; _lhs + ( ( (*(*(*js.upgrade().deref()) . varint .borrow()) . value .borrow()) as i32 ) )  } )) ;
 ;
 ;
 if { let _lhs = (*block_idx.borrow()) ; _lhs > (*(*js.upgrade().deref()) . last_block_idx .borrow())  } { (  { (*maybe_add_zero_run.borrow_mut()) ( ) } ) ;
 } (*(*js.upgrade().deref()) . last_num .borrow_mut())  . prefix_inc () ;
 (*(*js.upgrade().deref()) . last_block_idx .borrow_mut())  = (*block_idx.borrow())  ;
 if ( (*(*js.upgrade().deref()) . last_block_idx .borrow()) > ( ( 1 << 30 ) ) ) { return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN  ;
 } (*(*js.upgrade().deref()) . stage .borrow_mut())  = brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_ZERO_RUN_CONTINUATION  ;
 ;
 continue 'loop_ ;
 }, _ => { return (brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR ).clone() ;
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
        0_u32,
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
    ])));
    thread_local!(
        static kSpan_153: Value<i32> = Rc::new(RefCell::new(16));
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        if !(({
            let _nbits: i32 = 1;
            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
            (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits, _in)
        }) != 0)
        {
            (*i.borrow_mut()) += (*kSpan_153.with(Value::clone).borrow());
            continue 'loop_;
        }
        let start: Value<i32> = Rc::new(RefCell::new(if ((*i.borrow()) > 0) {
            (*i.borrow())
        } else {
            1
        }));
        let end: Value<i32> = Rc::new(RefCell::new(
            ((*i.borrow()) + (*kSpan_153.with(Value::clone).borrow())),
        ));
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            let v: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*v.borrow()) <= ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32))
            {
                let bits: Value<u32> = Rc::new(RefCell::new(
                    ({
                        let _nbits: i32 = 3;
                        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                        (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits, _in)
                    }),
                ));
                let rhs_0 = (*v.borrow()).wrapping_add((*bits.borrow()));
                (*v.borrow_mut()) = rhs_0;
                if ((*bits.borrow()) < 7_u32) {
                    break;
                }
            }
            if ((*v.borrow()) > ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32)) {
                return false;
            }
            (*lehmer.borrow_mut())[(*j.borrow()) as usize] = (*v.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()) += (*kSpan_153.with(Value::clone).borrow());
    }
    let end: Value<i32> = Rc::new(RefCell::new(
        ((*kDCTBlockSize_3.with(Value::clone).borrow()) - 1),
    ));
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
        let _code: Ptr<u32> = (lehmer.as_pointer() as Ptr<u32>);
        let _len: u64 = ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64);
        let _sigma: Ptr<u32> = (*order.borrow()).clone();
        DecodeLehmerCode_113(_code, _len, _sigma)
    }) {
        return false;
    }
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        let __rhs = (*kJPEGNaturalOrder_13.with(Value::clone).borrow())
            [((*order.borrow()).offset((*k.borrow()) as isize).read()) as usize];
        (*order.borrow())
            .offset((*k.borrow()) as isize)
            .write(__rhs);
        (*k.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn DecodeNumNonzeros_154(
    p: Ptr<brunsli_Prob>,
    ac: Ptr<brunsli_BinaryArithmeticDecoder>,
    in_: Ptr<brunsli_WordSource>,
) -> u64 {
    let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
    let ac: Value<Ptr<brunsli_BinaryArithmeticDecoder>> = Rc::new(RefCell::new(ac));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let bst: Value<Ptr<brunsli_Prob>> =
        Rc::new(RefCell::new((*p.borrow()).offset(-((1) as isize))));
    let ctx: Value<u64> = Rc::new(RefCell::new(1_u64));
    let b: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*b.borrow()) < (*kNumNonZeroBits_84.with(Value::clone).borrow())) {
        let bit: Value<i32> = Rc::new(RefCell::new(
            ({
                let _prob: i32 = (({
                    (*(*bst.borrow())
                        .offset((*ctx.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .get_proba()
                }) as i32);
                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                (*(*ac.borrow()).upgrade().deref()).ReadBit(_prob, _in)
            }),
        ));
        ({
            let _val: i32 = (*bit.borrow());
            (*(*bst.borrow())
                .offset((*ctx.borrow()) as isize)
                .upgrade()
                .deref())
            .Add(_val)
        });
        let __rhs = ((2_u64).wrapping_mul((*ctx.borrow()))).wrapping_add(((*bit.borrow()) as u64));
        (*ctx.borrow_mut()) = __rhs;
        (*b.borrow_mut()).prefix_inc();
    }
    let val: Value<u64> = Rc::new(RefCell::new(
        (*ctx.borrow())
            .wrapping_sub(((1_u32 << (*kNumNonZeroBits_84.with(Value::clone).borrow())) as u64)),
    ));
    if !((*val.borrow()) <= (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 593;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeNumNonzeros");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    if !(*(*s.upgrade().deref()).subdecoders_initialized.borrow()) {
        ({
            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
            (*(*s.upgrade().deref()).ans_decoder.borrow()).Init(_in)
        });
        ({
            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
            (*(*s.upgrade().deref()).bit_reader.borrow()).Init(_in)
        });
        ({
            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
            (*(*s.upgrade().deref()).arith_decoder.borrow()).Init(_in)
        });
        (*(*s.upgrade().deref()).subdecoders_initialized.borrow_mut()) = true;
    }
}
pub fn FinalizeSubdecoders_156(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    if !({ (*(*s.upgrade().deref()).ans_decoder.borrow()).CheckCRC() }) {
        return false;
    }
    if !({ (*(*s.upgrade().deref()).bit_reader.borrow()).Finish() }) {
        return false;
    }
    (*(*s.upgrade().deref()).subdecoders_initialized.borrow_mut()) = false;
    return true;
}
pub fn DecodeDC_157(
    state: Ptr<brunsli_internal_dec_State>,
    in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<u64> = Rc::new(RefCell::new((*meta.upgrade().deref()).len() as u64));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_dec_ComponentMeta>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .height_in_blocks
        .borrow());
        _lhs / (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_dec_ComponentMeta>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .v_samp
        .borrow())
    }));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> =
        (*s.upgrade().deref()).ac_dc.as_pointer();
    let comps: Ptr<Vec<brunsli_ComponentStateDC>> =
        (*ac_dc_state.upgrade().deref()).dc.as_pointer();
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = (*num_components.borrow()) as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| {
                __v.resize_with(__a0, || <brunsli_ComponentStateDC>::default())
            })
        };
        let c: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            ({
                let _w: i32 = (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .width_in_blocks
                .borrow());
                (*(comps.to_strong().as_pointer() as Ptr<brunsli_ComponentStateDC>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .SetWidth(_w)
            });
            (*c.borrow_mut()).prefix_inc();
        }
    }
    if !({
        let _n: u64 = 5_u64;
        (*(*in_.borrow()).upgrade().deref()).CanRead(_n)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
        EnsureSubdecodersInitialized_155(_state, _in)
    });
    let ans: Value<brunsli_ANSDecoder> = Rc::new(RefCell::new(
        (*(*s.upgrade().deref()).ans_decoder.borrow()).clone(),
    ));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new(
        (*(*s.upgrade().deref()).bit_reader.borrow()).clone(),
    ));
    let ac: Value<brunsli_BinaryArithmeticDecoder> = Rc::new(RefCell::new(
        (*(*s.upgrade().deref()).arith_decoder.borrow()).clone(),
    ));
    let mcu_y: Value<i32> = Rc::new(RefCell::new(
        (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow()),
    ));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<u64> = Rc::new(RefCell::new(
            (*(*ac_dc_state.upgrade().deref()).next_component.borrow()),
        ));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
                ((comps.to_strong().as_pointer() as Ptr<brunsli_ComponentStateDC>)
                    .offset((*i.borrow()) as isize)),
            ));
            let m: Ptr<brunsli_internal_dec_ComponentMeta> = (meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*i.borrow()) as isize);
            let context_map: Value<Ptr<u8>> = Rc::new(RefCell::new(
                (*(*(*state.borrow()).upgrade().deref()).context_map.borrow()).offset(
                    ((*i.borrow()).wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow())))
                        as isize,
                ),
            ));
            let ac_stride: Value<i32> = Rc::new(RefCell::new(
                ((*(*m.upgrade().deref()).ac_stride.borrow()) as i32),
            ));
            let b_stride: Value<u64> = Rc::new(RefCell::new(
                ((*(*m.upgrade().deref()).b_stride.borrow()) as u64),
            ));
            let width: Value<i32> = Rc::new(RefCell::new(
                (*(*m.upgrade().deref()).width_in_blocks.borrow()),
            ));
            let y: Value<i32> = Rc::new(RefCell::new({
                let _lhs = {
                    let _lhs = (*mcu_y.borrow());
                    _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
                };
                _lhs + (*(*ac_dc_state.upgrade().deref()).next_iy.borrow())
            }));
            let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*(*c.borrow()).upgrade().deref()).prev_sign.as_pointer() as Ptr<i32>)
                    .offset(1_u64 as isize)),
            ));
            let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*(*c.borrow()).upgrade().deref())
                    .prev_abs_coeff
                    .as_pointer() as Ptr<i32>)
                    .offset(2_u64 as isize)),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(
                (*(*ac_dc_state.upgrade().deref()).next_iy.borrow()),
            ));
            'loop_: while {
                let _lhs = (*iy.borrow());
                _lhs < (*(*m.upgrade().deref()).v_samp.borrow())
            } {
                let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).ac_coeffs.borrow())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize)
                        .offset(
                            ({
                                let _lhs = (*(*ac_dc_state.upgrade().deref()).next_x.borrow());
                                _lhs * (*kDCTBlockSize_3.with(Value::clone).borrow())
                            }) as isize,
                        ),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).block_state.borrow())
                        .offset(
                            (((*y.borrow()) as u64).wrapping_mul((*b_stride.borrow()))) as isize,
                        )
                        .offset((*(*ac_dc_state.upgrade().deref()).next_x.borrow()) as isize),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(
                    (*(*ac_dc_state.upgrade().deref()).next_x.borrow()),
                ));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    if ((!({
                        let _n: u64 = 6_u64;
                        (*(*in_.borrow()).upgrade().deref()).CanRead(_n)
                    }) as i64)
                        != 0)
                    {
                        (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow_mut()) =
                            (*mcu_y.borrow());
                        (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) =
                            (*i.borrow());
                        (*(*ac_dc_state.upgrade().deref()).next_iy.borrow_mut()) = (*iy.borrow());
                        (*(*ac_dc_state.upgrade().deref()).next_x.borrow_mut()) = (*x.borrow());
                        (*(*s.upgrade().deref()).ans_decoder.borrow_mut()) =
                            (*ans.borrow()).clone();
                        (*(*s.upgrade().deref()).bit_reader.borrow_mut()) = (*br.borrow()).clone();
                        (*(*s.upgrade().deref()).arith_decoder.borrow_mut()) =
                            (*ac.borrow()).clone();
                        return (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA).clone();
                    }
                    let is_empty_ctx: Value<i32> = Rc::new(RefCell::new(
                        ({
                            let _prev: Ptr<i32> = (((*(*c.borrow()).upgrade().deref())
                                .prev_is_nonempty
                                .as_pointer()
                                as Ptr<i32>)
                                .offset(1_u64 as isize));
                            let _x: i32 = (*x.borrow());
                            IsEmptyBlockContext_106(_prev, _x)
                        }),
                    ));
                    let is_empty_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                        (((*(*c.borrow()).upgrade().deref())
                            .is_empty_block_prob
                            .as_pointer() as Ptr<brunsli_Prob>)
                            .offset(((*is_empty_ctx.borrow()) as u64) as isize)),
                    ));
                    let is_empty_block: Value<bool> = Rc::new(RefCell::new(
                        !(({
                            let _prob: i32 =
                                (({ (*(*is_empty_p.borrow()).upgrade().deref()).get_proba() })
                                    as i32);
                            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                            (*ac.borrow()).ReadBit(_prob, _in)
                        }) != 0),
                    ));
                    ({
                        let _val: i32 = (!(*is_empty_block.borrow()) as i32);
                        (*(*is_empty_p.borrow()).upgrade().deref()).Add(_val)
                    });
                    ((*(*c.borrow()).upgrade().deref())
                        .prev_is_nonempty
                        .as_pointer() as Ptr<i32>)
                        .offset((((*x.borrow()) + 1) as u64) as isize)
                        .write((!(*is_empty_block.borrow()) as i32));
                    let __rhs = ((*is_empty_block.borrow()) as u8);
                    (*block_state.borrow()).write(__rhs);
                    let abs_val: Value<i32> = Rc::new(RefCell::new(0));
                    let sign: Value<i32> = Rc::new(RefCell::new(0));
                    if !(*is_empty_block.borrow()) {
                        let p_is_zero: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                            ((*(*c.borrow()).upgrade().deref()).is_zero_prob.as_pointer()),
                        ));
                        let is_zero: Value<i32> = Rc::new(RefCell::new(
                            ({
                                let _prob: i32 =
                                    (({ (*(*p_is_zero.borrow()).upgrade().deref()).get_proba() })
                                        as i32);
                                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                (*ac.borrow()).ReadBit(_prob, _in)
                            }),
                        ));
                        ({
                            let _val: i32 = (*is_zero.borrow());
                            (*(*p_is_zero.borrow()).upgrade().deref()).Add(_val)
                        });
                        if !((*is_zero.borrow()) != 0) {
                            let avg_ctx: Value<i32> = Rc::new(RefCell::new(
                                ({
                                    let _vals: Ptr<i32> = (*prev_abs.borrow()).clone();
                                    let _x: i32 = (*x.borrow());
                                    WeightedAverageContextDC_97(_vals, _x)
                                }),
                            ));
                            let sign_ctx: Value<i32> = Rc::new(RefCell::new(
                                ((((*prev_sgn.borrow()).offset((*x.borrow()) as isize).read())
                                    * 3)
                                    + ((*prev_sgn.borrow())
                                        .offset(((*x.borrow()) - 1) as isize)
                                        .read())),
                            ));
                            let sign_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*(*c.borrow()).upgrade().deref()).sign_prob.as_pointer()
                                    as Ptr<brunsli_Prob>)
                                    .offset(((*sign_ctx.borrow()) as u64) as isize)),
                            ));
                            (*sign.borrow_mut()) = ({
                                let _prob: i32 =
                                    (({ (*(*sign_p.borrow()).upgrade().deref()).get_proba() })
                                        as i32);
                                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                (*ac.borrow()).ReadBit(_prob, _in)
                            });
                            ({
                                let _val: i32 = (*sign.borrow());
                                (*(*sign_p.borrow()).upgrade().deref()).Add(_val)
                            });
                            let entropy_ix: Value<i32> = Rc::new(RefCell::new(
                                (((*context_map.borrow())
                                    .offset((*avg_ctx.borrow()) as isize)
                                    .read()) as i32),
                            ));
                            let code: Value<i32> = Rc::new(RefCell::new(
                                ({
                                    let _code: Ptr<brunsli_ANSDecodingData> =
                                        (*(*(*state.borrow()).upgrade().deref())
                                            .entropy_codes
                                            .borrow())
                                        .offset((*entropy_ix.borrow()) as isize);
                                    let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                    (*ans.borrow()).ReadSymbol(_code, _in)
                                }),
                            ));
                            if ((*code.borrow())
                                < (*kNumDirectCodes_135.with(Value::clone).borrow()))
                            {
                                (*abs_val.borrow_mut()) = ((*code.borrow()) + 1);
                            } else {
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    ((*code.borrow())
                                        - (*kNumDirectCodes_135.with(Value::clone).borrow())),
                                ));
                                let p_first_extra_bit: Value<Ptr<brunsli_Prob>> =
                                    Rc::new(RefCell::new(
                                        (((*(*c.borrow()).upgrade().deref())
                                            .first_extra_bit_prob
                                            .as_pointer()
                                            as Ptr<brunsli_Prob>)
                                            .offset(((*nbits.borrow()) as u64) as isize)),
                                    ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    ({
                                        let _prob: i32 = (({
                                            (*(*p_first_extra_bit.borrow()).upgrade().deref())
                                                .get_proba()
                                        })
                                            as i32);
                                        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                        (*ac.borrow()).ReadBit(_prob, _in)
                                    }),
                                ));
                                ({
                                    let _val: i32 = (*first_extra_bit.borrow());
                                    (*(*p_first_extra_bit.borrow()).upgrade().deref()).Add(_val)
                                });
                                let extra_bits_val: Value<i32> = Rc::new(RefCell::new(
                                    ((*first_extra_bit.borrow()) << (*nbits.borrow())),
                                ));
                                if ((*nbits.borrow()) > 0) {
                                    (*extra_bits_val.borrow_mut()) |= (({
                                        let _nbits: i32 = (*nbits.borrow());
                                        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                                        (*br.borrow()).ReadBits(_nbits, _in)
                                    })
                                        as i32);
                                }
                                (*abs_val.borrow_mut()) =
                                    ((((*kNumDirectCodes_135.with(Value::clone).borrow()) - 1)
                                        + (2 << (*nbits.borrow())))
                                        + (*extra_bits_val.borrow()));
                            }
                        }
                    }
                    let __rhs = (*abs_val.borrow());
                    (*prev_abs.borrow())
                        .offset((*x.borrow()) as isize)
                        .write(__rhs);
                    let __rhs = if ((*abs_val.borrow()) != 0) {
                        ((*sign.borrow()) + 1)
                    } else {
                        0
                    };
                    (*prev_sgn.borrow())
                        .offset((*x.borrow()) as isize)
                        .write(__rhs);
                    let __rhs = (({
                        let _lhs = ((1 - (2 * (*sign.borrow()))) * (*abs_val.borrow()));
                        _lhs + ({
                            let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                            let _x: i32 = (*x.borrow());
                            let _y: i32 = (*y.borrow());
                            let _stride: i32 = (*ac_stride.borrow());
                            PredictWithAdaptiveMedian_115(_coeffs, _x, _y, _stride)
                        })
                    }) as i16);
                    (*coeffs.borrow()).offset((0) as isize).write(__rhs);
                    (*block_state.borrow_mut()).postfix_inc();
                    (*coeffs.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                (*(*ac_dc_state.upgrade().deref()).next_x.borrow_mut()) = 0;
                (*iy.borrow_mut()).prefix_inc();
                (*y.borrow_mut()).prefix_inc();
            }
            (*(*ac_dc_state.upgrade().deref()).next_iy.borrow_mut()) = 0;
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) = 0_u64;
        (*mcu_y.borrow_mut()).prefix_inc();
    }
    (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow_mut()) = 0;
    (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) = 0_u64;
    (*(*ac_dc_state.upgrade().deref()).next_iy.borrow_mut()) = 0;
    (*(*ac_dc_state.upgrade().deref()).next_x.borrow_mut()) = 0;
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| __v.clear());
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentStateDC>| __v.shrink_to_fit());
    (*(*s.upgrade().deref()).ans_decoder.borrow_mut()) = (*ans.borrow()).clone();
    (*(*s.upgrade().deref()).bit_reader.borrow_mut()) = (*br.borrow()).clone();
    (*(*s.upgrade().deref()).arith_decoder.borrow_mut()) = (*ac.borrow()).clone();
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        FinalizeSubdecoders_156(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn DecodeEmptyAcBlock_158(prev_sgn: Ptr<i32>, prev_abs: Ptr<i32>) {
    let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(prev_sgn));
    let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(prev_abs));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        (*prev_sgn.borrow()).offset((*k.borrow()) as isize).write(0);
        (*prev_abs.borrow()).offset((*k.borrow()) as isize).write(0);
        (*k.borrow_mut()).prefix_inc();
    }
}
#[derive(Default)]
pub struct brunsli_AcBlockCookie {
    pub x: Value<i32>,
    pub y: Value<i32>,
    pub prev_num_nonzeros: Value<Ptr<u8>>,
    pub prev_sgn: Value<Ptr<i32>>,
    pub prev_abs: Value<Ptr<i32>>,
    pub num_nonzero_prob: Value<Ptr<brunsli_Prob>>,
    pub ac: Value<Ptr<brunsli_BinaryArithmeticDecoder>>,
    pub in_: Value<Ptr<brunsli_WordSource>>,
    pub ans: Value<Ptr<brunsli_ANSDecoder>>,
    pub br: Value<Ptr<brunsli_BitSource>>,
    pub coeffs: Value<Ptr<i16>>,
    pub prev_row_coeffs: Value<Ptr<i16>>,
    pub prev_col_coeffs: Value<Ptr<i16>>,
    pub is_zero_prob: Value<Ptr<brunsli_Prob>>,
    pub order: Value<Ptr<u32>>,
    pub context_modes: Value<Ptr<u8>>,
    pub mult_col: Value<Ptr<i32>>,
    pub mult_row: Value<Ptr<i32>>,
    pub prev_row_delta: Value<i32>,
    pub sign_prob: Value<Ptr<brunsli_Prob>>,
    pub context_bits: Value<u64>,
    pub context_map: Value<Ptr<u8>>,
    pub entropy_codes: Value<Ptr<brunsli_ANSDecodingData>>,
    pub first_extra_bit_prob: Value<Ptr<brunsli_Prob>>,
}
impl Clone for brunsli_AcBlockCookie {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
            prev_num_nonzeros: Rc::new(RefCell::new((*self.prev_num_nonzeros.borrow()).clone())),
            prev_sgn: Rc::new(RefCell::new((*self.prev_sgn.borrow()).clone())),
            prev_abs: Rc::new(RefCell::new((*self.prev_abs.borrow()).clone())),
            num_nonzero_prob: Rc::new(RefCell::new((*self.num_nonzero_prob.borrow()).clone())),
            ac: Rc::new(RefCell::new((*self.ac.borrow()).clone())),
            in_: Rc::new(RefCell::new((*self.in_.borrow()).clone())),
            ans: Rc::new(RefCell::new((*self.ans.borrow()).clone())),
            br: Rc::new(RefCell::new((*self.br.borrow()).clone())),
            coeffs: Rc::new(RefCell::new((*self.coeffs.borrow()).clone())),
            prev_row_coeffs: Rc::new(RefCell::new((*self.prev_row_coeffs.borrow()).clone())),
            prev_col_coeffs: Rc::new(RefCell::new((*self.prev_col_coeffs.borrow()).clone())),
            is_zero_prob: Rc::new(RefCell::new((*self.is_zero_prob.borrow()).clone())),
            order: Rc::new(RefCell::new((*self.order.borrow()).clone())),
            context_modes: Rc::new(RefCell::new((*self.context_modes.borrow()).clone())),
            mult_col: Rc::new(RefCell::new((*self.mult_col.borrow()).clone())),
            mult_row: Rc::new(RefCell::new((*self.mult_row.borrow()).clone())),
            prev_row_delta: Rc::new(RefCell::new((*self.prev_row_delta.borrow()))),
            sign_prob: Rc::new(RefCell::new((*self.sign_prob.borrow()).clone())),
            context_bits: Rc::new(RefCell::new((*self.context_bits.borrow()))),
            context_map: Rc::new(RefCell::new((*self.context_map.borrow()).clone())),
            entropy_codes: Rc::new(RefCell::new((*self.entropy_codes.borrow()).clone())),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (*self.first_extra_bit_prob.borrow()).clone(),
            )),
        };
        this
    }
}
impl ByteRepr for brunsli_AcBlockCookie {}
pub fn DecodeAcBlock_159(cookie: Ptr<brunsli_AcBlockCookie>) -> u64 {
    let c: Value<brunsli_AcBlockCookie> =
        Rc::new(RefCell::new((*cookie.upgrade().deref()).clone()));
    let ac: Value<brunsli_BinaryArithmeticDecoder> = Rc::new(RefCell::new(
        (*(*(*c.borrow()).ac.borrow()).upgrade().deref()).clone(),
    ));
    let in_: Value<Ptr<brunsli_WordSource>> =
        Rc::new(RefCell::new((*(*c.borrow()).in_.borrow()).clone()));
    let ans: Value<brunsli_ANSDecoder> = Rc::new(RefCell::new(
        (*(*(*c.borrow()).ans.borrow()).upgrade().deref()).clone(),
    ));
    let br: Value<brunsli_BitSource> = Rc::new(RefCell::new(
        (*(*(*c.borrow()).br.borrow()).upgrade().deref()).clone(),
    ));
    let num_nonzeros: Value<u64> = Rc::new(RefCell::new(0_u64));
    let nonzero_ctx: Value<u8> = Rc::new(RefCell::new(
        ({
            let _prev: Ptr<u8> = (*(*c.borrow()).prev_num_nonzeros.borrow()).clone();
            let _x: i32 = (*(*c.borrow()).x.borrow());
            let _y: i32 = (*(*c.borrow()).y.borrow());
            NumNonzerosContext_104(_prev, _x, _y)
        }),
    ));
    let last_nz: Value<u64> = Rc::new(RefCell::new(
        ({
            let _p: Ptr<brunsli_Prob> = (*(*c.borrow()).num_nonzero_prob.borrow()).offset(
                ((*kNumNonZeroTreeSize_85.with(Value::clone).borrow())
                    .wrapping_mul(((*nonzero_ctx.borrow()) as u64))) as isize,
            );
            let _ac: Ptr<brunsli_BinaryArithmeticDecoder> = (ac.as_pointer());
            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
            DecodeNumNonzeros_154(_p, _ac, _in)
        }),
    ));
    let k: Value<u64> = Rc::new(RefCell::new((*last_nz.borrow()).wrapping_add(1_u64)));
    'loop_: while ((*k.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)) {
        (*(*c.borrow()).prev_sgn.borrow())
            .offset((*k.borrow()) as isize)
            .write(0);
        (*(*c.borrow()).prev_abs.borrow())
            .offset((*k.borrow()) as isize)
            .write(0);
        (*k.borrow_mut()).prefix_inc();
    }
    let k: Value<u64> = Rc::new(RefCell::new((*last_nz.borrow())));
    'loop_: while ((*k.borrow()) > 0_u64) {
        let is_zero: Value<i32> = Rc::new(RefCell::new(0));
        if ((*k.borrow()) < (*last_nz.borrow())) {
            let bucket: Value<u64> = Rc::new(RefCell::new(
                ((*kNonzeroBuckets_89.with(Value::clone).borrow())
                    [((*num_nonzeros.borrow()).wrapping_sub(1_u64)) as usize]
                    as u64),
            ));
            let is_zero_ctx: Value<u64> = Rc::new(RefCell::new(
                ((*bucket.borrow())
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
                .wrapping_add((*k.borrow())),
            ));
            let p: Ptr<brunsli_Prob> =
                (*(*c.borrow()).is_zero_prob.borrow()).offset((*is_zero_ctx.borrow()) as isize);
            (*is_zero.borrow_mut()) = ({
                let _prob: i32 = (({ (*p.upgrade().deref()).get_proba() }) as i32);
                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                (*ac.borrow()).ReadBit(_prob, _in)
            });
            ({
                let _val: i32 = (*is_zero.borrow());
                (*p.upgrade().deref()).Add(_val)
            });
        }
        let abs_val: Value<i32> = Rc::new(RefCell::new(0));
        let sign: Value<i32> = Rc::new(RefCell::new(1));
        let k_nat: Value<i32> = Rc::new(RefCell::new(
            (((*(*c.borrow()).order.borrow())
                .offset((*k.borrow()) as isize)
                .read()) as i32),
        ));
        if !((*is_zero.borrow()) != 0) {
            let context_type: Value<u64> = Rc::new(RefCell::new(
                (((*(*c.borrow()).context_modes.borrow())
                    .offset((*k_nat.borrow()) as isize)
                    .read()) as u64),
            ));
            let avg_ctx: Value<u64> = Rc::new(RefCell::new(0_u64));
            let sign_ctx: Value<u64> = Rc::new(RefCell::new(
                (*kMaxAverageContext_82.with(Value::clone).borrow()),
            ));
            if (((*context_type.borrow()) & 1_u64) != 0) && ((*(*c.borrow()).y.borrow()) > 0) {
                let offset: Value<u64> = Rc::new(RefCell::new((((*k_nat.borrow()) & 7) as u64)));
                ({
                    let _prev: Ptr<i16> = (*(*c.borrow()).prev_row_coeffs.borrow())
                        .offset((*offset.borrow()) as isize);
                    let _cur: Ptr<i16> =
                        (*(*c.borrow()).coeffs.borrow()).offset((*offset.borrow()) as isize);
                    let _mult: Ptr<i32> = (*(*c.borrow()).mult_col.borrow())
                        .offset(((*offset.borrow()).wrapping_mul(8_u64)) as isize);
                    let _avg_ctx: Ptr<u64> = (avg_ctx.as_pointer());
                    let _sgn: Ptr<u64> = (sign_ctx.as_pointer());
                    ACPredictContextRow_103(_prev, _cur, _mult, _avg_ctx, _sgn)
                });
            } else if (((*context_type.borrow()) & 2_u64) != 0) && ((*(*c.borrow()).x.borrow()) > 0)
            {
                let offset: Value<u64> = Rc::new(RefCell::new((((*k_nat.borrow()) & !7) as u64)));
                ({
                    let _prev: Ptr<i16> = (*(*c.borrow()).prev_col_coeffs.borrow())
                        .offset((*offset.borrow()) as isize);
                    let _cur: Ptr<i16> =
                        (*(*c.borrow()).coeffs.borrow()).offset((*offset.borrow()) as isize);
                    let _mult: Ptr<i32> =
                        (*(*c.borrow()).mult_row.borrow()).offset((*offset.borrow()) as isize);
                    let _avg_ctx: Ptr<u64> = (avg_ctx.as_pointer());
                    let _sgn: Ptr<u64> = (sign_ctx.as_pointer());
                    ACPredictContextCol_102(_prev, _cur, _mult, _avg_ctx, _sgn)
                });
            } else if !((*context_type.borrow()) != 0) {
                (*avg_ctx.borrow_mut()) = (({
                    let _vals: Ptr<i32> =
                        (*(*c.borrow()).prev_abs.borrow()).offset((*k.borrow()) as isize);
                    let _prev_row_delta: i32 = (*(*c.borrow()).prev_row_delta.borrow());
                    WeightedAverageContext_98(_vals, _prev_row_delta)
                }) as u64);
                (*sign_ctx.borrow_mut()) = (({
                    let _lhs = (((*(*c.borrow()).prev_sgn.borrow())
                        .offset((*k.borrow()) as isize)
                        .read())
                        * 3);
                    _lhs + ((*(*c.borrow()).prev_sgn.borrow())
                        .offset(
                            (((*k.borrow()) as i32)
                                - (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                as isize,
                        )
                        .read())
                }) as u64);
            }
            let __rhs = ((*sign_ctx.borrow())
                .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
            .wrapping_add((*k.borrow()));
            (*sign_ctx.borrow_mut()) = __rhs;
            let sign_p: Ptr<brunsli_Prob> =
                (*(*c.borrow()).sign_prob.borrow()).offset((*sign_ctx.borrow()) as isize);
            (*sign.borrow_mut()) = ({
                let _prob: i32 = (({ (*sign_p.upgrade().deref()).get_proba() }) as i32);
                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                (*ac.borrow()).ReadBit(_prob, _in)
            });
            ({
                let _val: i32 = (*sign.borrow());
                (*sign_p.upgrade().deref()).Add(_val)
            });
            let __rhs = ((*sign.borrow()) + 1);
            (*(*c.borrow()).prev_sgn.borrow())
                .offset((*k.borrow()) as isize)
                .write(__rhs);
            let __rhs = (1 - (2 * (*sign.borrow())));
            (*sign.borrow_mut()) = __rhs;
            let z_dens_ctx: Value<u64> = Rc::new(RefCell::new(
                (({
                    let _nonzeros_left: u64 = (*num_nonzeros.borrow());
                    let _k: u64 = (*k.borrow());
                    let _bits: u64 = (*(*c.borrow()).context_bits.borrow());
                    ZeroDensityContext_96(_nonzeros_left, _k, _bits)
                }) as u64),
            ));
            let histo_ix: Value<u64> = Rc::new(RefCell::new(
                ((*z_dens_ctx.borrow())
                    .wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow())))
                .wrapping_add((*avg_ctx.borrow())),
            ));
            let entropy_ix: Value<u64> = Rc::new(RefCell::new(
                (((*(*c.borrow()).context_map.borrow())
                    .offset((*histo_ix.borrow()) as isize)
                    .read()) as u64),
            ));
            let code: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _code: Ptr<brunsli_ANSDecodingData> =
                        (*(*c.borrow()).entropy_codes.borrow())
                            .offset((*entropy_ix.borrow()) as isize);
                    let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                    (*ans.borrow()).ReadSymbol(_code, _in)
                }),
            ));
            if ((*code.borrow()) < (*kNumDirectCodes_135.with(Value::clone).borrow())) {
                (*abs_val.borrow_mut()) = ((*code.borrow()) + 1);
            } else {
                let nbits: Value<i32> = Rc::new(RefCell::new(
                    ((*code.borrow()) - (*kNumDirectCodes_135.with(Value::clone).borrow())),
                ));
                let p: Ptr<brunsli_Prob> = (*(*c.borrow()).first_extra_bit_prob.borrow()).offset(
                    (((*k.borrow()).wrapping_mul(10_u64)).wrapping_add(((*nbits.borrow()) as u64)))
                        as isize,
                );
                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                    ({
                        let _prob: i32 = (({ (*p.upgrade().deref()).get_proba() }) as i32);
                        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                        (*ac.borrow()).ReadBit(_prob, _in)
                    }),
                ));
                ({
                    let _val: i32 = (*first_extra_bit.borrow());
                    (*p.upgrade().deref()).Add(_val)
                });
                let extra_bits_val: Value<i32> = Rc::new(RefCell::new(
                    ((*first_extra_bit.borrow()) << (*nbits.borrow())),
                ));
                if ((*nbits.borrow()) > 0) {
                    let rhs_0 = (((*extra_bits_val.borrow()) as u32)
                        | ({
                            let _nbits: i32 = (*nbits.borrow());
                            let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                            (*br.borrow()).ReadBits(_nbits, _in)
                        })) as i32;
                    (*extra_bits_val.borrow_mut()) = rhs_0;
                }
                (*abs_val.borrow_mut()) = ((((((*kNumDirectCodes_135.with(Value::clone).borrow())
                    - 1) as u32)
                    .wrapping_add((2_u32 << (*nbits.borrow()))))
                .wrapping_add(((*extra_bits_val.borrow()) as u32)))
                    as i32);
            }
            (*num_nonzeros.borrow_mut()).prefix_inc();
        } else {
            (*(*c.borrow()).prev_sgn.borrow())
                .offset((*k.borrow()) as isize)
                .write(0);
        }
        let coeff: Value<i32> = Rc::new(RefCell::new(((*sign.borrow()) * (*abs_val.borrow()))));
        let __rhs = ((*coeff.borrow()) as i16);
        (*(*c.borrow()).coeffs.borrow())
            .offset((*k_nat.borrow()) as isize)
            .write(__rhs);
        let __rhs = (*abs_val.borrow());
        (*(*c.borrow()).prev_abs.borrow())
            .offset((*k.borrow()) as isize)
            .write(__rhs);
        (*k.borrow_mut()).prefix_dec();
    }
    let __rhs = (*ans.borrow()).clone();
    (*(*c.borrow()).ans.borrow()).write(__rhs);
    let __rhs = (*br.borrow()).clone();
    (*(*c.borrow()).br.borrow()).write(__rhs);
    let __rhs = (*ac.borrow()).clone();
    (*(*c.borrow()).ac.borrow()).write(__rhs);
    return (*num_nonzeros.borrow());
}
pub fn DecodeAC_160(
    state: Ptr<brunsli_internal_dec_State>,
    in_: Ptr<brunsli_WordSource>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let in_: Value<Ptr<brunsli_WordSource>> = Rc::new(RefCell::new(in_));
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<u64> = Rc::new(RefCell::new((*meta.upgrade().deref()).len() as u64));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_dec_ComponentMeta>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .height_in_blocks
        .borrow());
        _lhs / (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_dec_ComponentMeta>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .v_samp
        .borrow())
    }));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let ac_dc_state: Ptr<brunsli_internal_dec_AcDcState> =
        (*s.upgrade().deref()).ac_dc.as_pointer();
    let comps: Ptr<Vec<brunsli_ComponentState>> = (*ac_dc_state.upgrade().deref()).ac.as_pointer();
    if (*comps.upgrade().deref()).is_empty() {
        {
            let __a0 = (*num_components.borrow()) as usize;
            comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| {
                __v.resize_with(__a0, || <brunsli_ComponentState>::default())
            })
        };
        let c: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            ({
                let _w: i32 = (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .width_in_blocks
                .borrow());
                (*(comps.to_strong().as_pointer() as Ptr<brunsli_ComponentState>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .SetWidth(_w)
            });
            ({
                let _quant: Ptr<i32> = (((*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .quant
                .as_pointer() as Ptr<i32>)
                    .offset(0_u64 as isize));
                let _mult_row: Ptr<i32> = ((*(comps.to_strong().as_pointer()
                    as Ptr<brunsli_ComponentState>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .mult_row
                .as_pointer() as Ptr<i32>);
                let _mult_col: Ptr<i32> = ((*(comps.to_strong().as_pointer()
                    as Ptr<brunsli_ComponentState>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .mult_col
                .as_pointer() as Ptr<i32>);
                ComputeACPredictMultipliers_109(_quant, _mult_row, _mult_col)
            });
            (*c.borrow_mut()).prefix_inc();
        }
    }
    if !({
        let _n: u64 = 5_u64;
        (*(*in_.borrow()).upgrade().deref()).CanRead(_n)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
        EnsureSubdecodersInitialized_155(_state, _in)
    });
    if !(*(*ac_dc_state.upgrade().deref())
        .ac_coeffs_order_decoded
        .borrow())
    {
        'loop_: while {
            let _lhs = (*(*ac_dc_state.upgrade().deref()).next_component.borrow());
            _lhs < (*num_components.borrow())
        } {
            if !({
                let _n: u64 = 121_u64;
                (*(*in_.borrow()).upgrade().deref()).CanRead(_n)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            }
            if !({
                let _order: Ptr<u32> = ((*(comps.to_strong().as_pointer()
                    as Ptr<brunsli_ComponentState>)
                    .offset((*(*ac_dc_state.upgrade().deref()).next_component.borrow()) as isize)
                    .upgrade()
                    .deref())
                .order
                .as_pointer() as Ptr<u32>);
                let _br: Ptr<brunsli_BitSource> = ((*s.upgrade().deref()).bit_reader.as_pointer());
                let _in: Ptr<brunsli_WordSource> = (*in_.borrow()).clone();
                DecodeCoeffOrder_152(_order, _br, _in)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            }
            (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()).postfix_inc();
        }
        (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) = 0_u64;
        (*(*ac_dc_state.upgrade().deref())
            .ac_coeffs_order_decoded
            .borrow_mut()) = true;
    }
    let c: Value<brunsli_AcBlockCookie> = Rc::new(RefCell::new(<brunsli_AcBlockCookie>::default()));
    (*(*c.borrow()).ac.borrow_mut()) = ((*s.upgrade().deref()).arith_decoder.as_pointer());
    (*(*c.borrow()).in_.borrow_mut()) = (*in_.borrow()).clone();
    (*(*c.borrow()).ans.borrow_mut()) = ((*s.upgrade().deref()).ans_decoder.as_pointer());
    (*(*c.borrow()).br.borrow_mut()) = ((*s.upgrade().deref()).bit_reader.as_pointer());
    (*(*c.borrow()).entropy_codes.borrow_mut()) = (*(*(*state.borrow()).upgrade().deref())
        .entropy_codes
        .borrow())
    .clone();
    (*(*c.borrow()).context_modes.borrow_mut()) =
        (kContextAlgorithm_95.with(Value::clone).as_pointer() as Ptr<u8>).offset(
            (if (*(*(*state.borrow()).upgrade().deref())
                .use_legacy_context_model
                .borrow())
            {
                64
            } else {
                0
            }) as isize,
        );
    let mcu_y: Value<i32> = Rc::new(RefCell::new(
        (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow()),
    ));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<u64> = Rc::new(RefCell::new(
            (*(*ac_dc_state.upgrade().deref()).next_component.borrow()),
        ));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let cst: Ptr<brunsli_ComponentState> = (comps.to_strong().as_pointer()
                as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize);
            (*(*c.borrow()).prev_num_nonzeros.borrow_mut()) =
                ((*cst.upgrade().deref()).prev_num_nonzeros.as_pointer() as Ptr<u8>);
            (*(*c.borrow()).num_nonzero_prob.borrow_mut()) =
                ((*cst.upgrade().deref()).num_nonzero_prob.as_pointer() as Ptr<brunsli_Prob>);
            (*(*c.borrow()).is_zero_prob.borrow_mut()) =
                ((*cst.upgrade().deref()).is_zero_prob.as_pointer() as Ptr<brunsli_Prob>);
            (*(*c.borrow()).order.borrow_mut()) =
                ((*cst.upgrade().deref()).order.as_pointer() as Ptr<u32>);
            (*(*c.borrow()).mult_col.borrow_mut()) =
                ((*cst.upgrade().deref()).mult_col.as_pointer() as Ptr<i32>);
            (*(*c.borrow()).mult_row.borrow_mut()) =
                ((*cst.upgrade().deref()).mult_row.as_pointer() as Ptr<i32>);
            (*(*c.borrow()).sign_prob.borrow_mut()) =
                ((*cst.upgrade().deref()).sign_prob.as_pointer() as Ptr<brunsli_Prob>);
            (*(*c.borrow()).first_extra_bit_prob.borrow_mut()) =
                ((*cst.upgrade().deref()).first_extra_bit_prob.as_pointer() as Ptr<brunsli_Prob>);
            let m: Ptr<brunsli_internal_dec_ComponentMeta> = (meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*i.borrow()) as isize);
            (*(*c.borrow()).context_map.borrow_mut()) =
                (*(*(*state.borrow()).upgrade().deref()).context_map.borrow()).offset(
                    ((*(*m.upgrade().deref()).context_offset.borrow())
                        .wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow())))
                        as isize,
                );
            (*(*c.borrow()).context_bits.borrow_mut()) =
                (*(*m.upgrade().deref()).context_bits.borrow());
            let width: Value<i32> = Rc::new(RefCell::new(
                (*(*m.upgrade().deref()).width_in_blocks.borrow()),
            ));
            let ac_stride: Value<u64> = Rc::new(RefCell::new(
                ((*(*m.upgrade().deref()).ac_stride.borrow()) as u64),
            ));
            let b_stride: Value<u64> = Rc::new(RefCell::new(
                ((*(*m.upgrade().deref()).b_stride.borrow()) as u64),
            ));
            let next_iy: Value<i32> = Rc::new(RefCell::new(
                (*(*ac_dc_state.upgrade().deref()).next_iy.borrow()),
            ));
            (*(*c.borrow()).y.borrow_mut()) = {
                let _lhs = {
                    let _lhs = (*mcu_y.borrow());
                    _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
                };
                _lhs + (*next_iy.borrow())
            };
            let __rhs = (((((1_u32).wrapping_sub(
                (2_u32).wrapping_mul((((*(*c.borrow()).y.borrow()) as u32) & 1_u32)),
            ))
            .wrapping_mul((((*width.borrow()) + 3) as u32)))
            .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32)))
                as i32);
            (*(*c.borrow()).prev_row_delta.borrow_mut()) = __rhs;
            let iy: Value<i32> = Rc::new(RefCell::new((*next_iy.borrow())));
            'loop_: while {
                let _lhs = (*iy.borrow());
                _lhs < (*(*m.upgrade().deref()).v_samp.borrow())
            } {
                let next_x: Value<i32> = Rc::new(RefCell::new(
                    (*(*ac_dc_state.upgrade().deref()).next_x.borrow()),
                ));
                let block_offset: Value<u64> = Rc::new(RefCell::new(
                    (((*next_x.borrow()) * (*kDCTBlockSize_3.with(Value::clone).borrow())) as u64),
                ));
                let __rhs = (*(*m.upgrade().deref()).ac_coeffs.borrow())
                    .offset(
                        (((*(*c.borrow()).y.borrow()) as u64).wrapping_mul((*ac_stride.borrow())))
                            as isize,
                    )
                    .offset((*block_offset.borrow()) as isize);
                (*(*c.borrow()).coeffs.borrow_mut()) = __rhs;
                let __rhs =
                    (*(*c.borrow()).coeffs.borrow()).offset(-((*ac_stride.borrow()) as isize));
                (*(*c.borrow()).prev_row_coeffs.borrow_mut()) = __rhs;
                let __rhs = (*(*c.borrow()).coeffs.borrow())
                    .offset(-((*kDCTBlockSize_3.with(Value::clone).borrow()) as isize));
                (*(*c.borrow()).prev_col_coeffs.borrow_mut()) = __rhs;
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).block_state.borrow())
                        .offset(
                            (((*(*c.borrow()).y.borrow()) as u64)
                                .wrapping_mul((*b_stride.borrow())))
                                as isize,
                        )
                        .offset((*next_x.borrow()) as isize),
                ));
                (*(*c.borrow()).prev_sgn.borrow_mut()) =
                    (((*cst.upgrade().deref()).prev_sign.as_pointer() as Ptr<i32>)
                        .offset(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64) as isize))
                    .offset((*block_offset.borrow()) as isize);
                let __rhs = (((*cst.upgrade().deref()).prev_abs_coeff.as_pointer() as Ptr<i32>)
                    .offset(
                        (((((((*(*c.borrow()).y.borrow()) as u32) & 1_u32)
                            .wrapping_mul((((*width.borrow()) + 3) as u32)))
                        .wrapping_add(2_u32))
                        .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32)))
                            as u64) as isize,
                    ))
                .offset((*block_offset.borrow()) as isize);
                (*(*c.borrow()).prev_abs.borrow_mut()) = __rhs;
                (*(*c.borrow()).x.borrow_mut()) = (*next_x.borrow());
                'loop_: while ((*(*c.borrow()).x.borrow()) < (*width.borrow())) {
                    let is_empty: Value<bool> = Rc::new(RefCell::new(
                        ((((*block_state.borrow_mut()).postfix_inc()).read()) != 0),
                    ));
                    if !(*is_empty.borrow()) {
                        if ((!({
                            let _n: u64 = 297_u64;
                            (*(*in_.borrow()).upgrade().deref()).CanRead(_n)
                        }) as i64)
                            != 0)
                        {
                            (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow_mut()) =
                                (*mcu_y.borrow());
                            (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) =
                                (*i.borrow());
                            (*(*ac_dc_state.upgrade().deref()).next_iy.borrow_mut()) =
                                (*iy.borrow());
                            (*(*ac_dc_state.upgrade().deref()).next_x.borrow_mut()) =
                                (*(*c.borrow()).x.borrow());
                            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        }
                        let num_nonzeros: Value<u64> = Rc::new(RefCell::new(
                            ({
                                let _cookie: Ptr<brunsli_AcBlockCookie> = c.as_pointer();
                                DecodeAcBlock_159(_cookie)
                            }),
                        ));
                        if !((*num_nonzeros.borrow())
                            <= (*kNumNonZeroTreeSize_85.with(Value::clone).borrow()))
                        {
                            ({
                                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                                let _l: i32 = 949;
                                let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeAC");
                                BrunsliDumpAndAbort_79(_f, _l, _fn)
                            });
                            'loop_: while true {}
                        };
                        let __rhs = ((*num_nonzeros.borrow()) as u8);
                        (*(*c.borrow()).prev_num_nonzeros.borrow())
                            .offset((*(*c.borrow()).x.borrow()) as isize)
                            .write(__rhs);
                    } else {
                        ({
                            let _prev_sgn: Ptr<i32> = (*(*c.borrow()).prev_sgn.borrow()).clone();
                            let _prev_abs: Ptr<i32> = (*(*c.borrow()).prev_abs.borrow()).clone();
                            DecodeEmptyAcBlock_158(_prev_sgn, _prev_abs)
                        });
                        (*(*c.borrow()).prev_num_nonzeros.borrow())
                            .offset((*(*c.borrow()).x.borrow()) as isize)
                            .write(0_u8);
                    }
                    (*(*c.borrow()).coeffs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*(*c.borrow()).prev_sgn.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*(*c.borrow()).prev_abs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*(*c.borrow()).prev_row_coeffs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*(*c.borrow()).prev_col_coeffs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*(*c.borrow()).x.borrow_mut()).prefix_inc();
                }
                (*(*c.borrow()).prev_row_delta.borrow_mut()) *= -1_i32;
                (*(*ac_dc_state.upgrade().deref()).next_x.borrow_mut()) = 0;
                (*iy.borrow_mut()).prefix_inc();
                (*(*c.borrow()).y.borrow_mut()).prefix_inc();
            }
            (*(*ac_dc_state.upgrade().deref()).next_iy.borrow_mut()) = 0;
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*ac_dc_state.upgrade().deref()).next_component.borrow_mut()) = 0_u64;
        (*mcu_y.borrow_mut()).prefix_inc();
    }
    (*(*ac_dc_state.upgrade().deref()).next_mcu_y.borrow_mut()) = 0;
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.clear());
    comps.with_mut(|__v: &mut Vec<brunsli_ComponentState>| __v.shrink_to_fit());
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        FinalizeSubdecoders_156(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn CheckCanRead_161(state: Ptr<brunsli_internal_dec_State>, required: u64) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let required: Value<u64> = Rc::new(RefCell::new(required));
    let available: Value<u64> = Rc::new(RefCell::new(
        (*(*(*state.borrow()).upgrade().deref()).len.borrow())
            .wrapping_sub((*(*(*state.borrow()).upgrade().deref()).pos.borrow())),
    ));
    return ((*required.borrow()) <= (*available.borrow()));
}
pub fn CheckCanReadByte_162(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        _lhs != (*(*(*state.borrow()).upgrade().deref()).len.borrow())
    };
}
pub fn ReadByte_163(state: Ptr<brunsli_internal_dec_State>) -> u8 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return ((*(*(*state.borrow()).upgrade().deref()).data.borrow())
        .offset(((*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()).postfix_inc()) as isize)
        .read());
}
pub fn PeekByte_164(state: Ptr<brunsli_internal_dec_State>, offset: u64) -> u8 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let offset: Value<u64> = Rc::new(RefCell::new(offset));
    return ((*(*(*state.borrow()).upgrade().deref()).data.borrow())
        .offset(
            ((*(*(*state.borrow()).upgrade().deref()).pos.borrow())
                .wrapping_add((*offset.borrow()))) as isize,
        )
        .read());
}
pub fn SkipBytes_165(state: Ptr<brunsli_internal_dec_State>, len: u64) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let rhs_0 =
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow()).wrapping_add((*len.borrow()));
    (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
}
pub fn GetBytesAvailable_166(state: Ptr<brunsli_internal_dec_State>) -> u64 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return (*(*(*state.borrow()).upgrade().deref()).len.borrow())
        .wrapping_sub((*(*(*state.borrow()).upgrade().deref()).pos.borrow()));
}
pub fn SkipAvailableBytes_167(state: Ptr<brunsli_internal_dec_State>, len: u64) -> u64 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let available: Value<u64> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            GetBytesAvailable_166(_state)
        }),
    ));
    let skip_bytes: Value<u64> = Rc::new(RefCell::new(
        (if available.as_pointer().read() <= len.as_pointer().read() {
            available.as_pointer()
        } else {
            len.as_pointer()
        }
        .read()),
    ));
    let rhs_0 =
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow()).wrapping_add((*skip_bytes.borrow()));
    (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
    return (*skip_bytes.borrow());
}
pub fn DecodeBase128_168(
    state: Ptr<brunsli_internal_dec_State>,
    val: Ptr<u64>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let val: Value<Ptr<u64>> = Rc::new(RefCell::new(val));
    (*val.borrow()).write(0_u64);
    let b: Value<u64> = Rc::new(RefCell::new(128_u64));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < 9_u64) && (((*b.borrow()) & 128_u64) != 0) {
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _required: u64 = (*i.borrow()).wrapping_add(1_u64);
            CheckCanRead_161(_state, _required)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        (*b.borrow_mut()) = (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _offset: u64 = (*i.borrow());
            PeekByte_164(_state, _offset)
        }) as u64);
        let rhs_0 = ((((*val.borrow()).read()) as u64)
            | (((*b.borrow()) & 127_u64) << ((*i.borrow()).wrapping_mul(7_u64))))
            as u64;
        (*val.borrow()).write(rhs_0);
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _len: u64 = (*i.borrow());
        SkipBytes_165(_state, _len)
    });
    return if (((*b.borrow()) & 128_u64) == 0_u64) {
        brunsli_BrunsliStatus::BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
    };
}
pub fn Fail_169(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    (*(*s.upgrade().deref()).result.borrow_mut()) = (*result.borrow()).clone();
    (*(*s.upgrade().deref()).last_stage.borrow_mut()) =
        (*(*(*state.borrow()).upgrade().deref()).stage.borrow()).clone();
    return (brunsli_internal_dec_Stage::ERROR).clone();
}
pub fn ReadTag_170(
    state: Ptr<brunsli_internal_dec_State>,
    section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        CheckCanReadByte_162(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
    }
    let marker: Value<u8> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            ReadByte_163(_state)
        }),
    ));
    let tag: Value<u64> = Rc::new(RefCell::new(
        ((((*marker.borrow()) as i32) >> 3_u32) as u64),
    ));
    if ((*tag.borrow()) == 0_u64) || ((*tag.borrow()) > 15_u64) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*(*(*section.borrow()).upgrade().deref()).tag.borrow_mut()) = (*tag.borrow());
    let wiring_type: Value<u64> =
        Rc::new(RefCell::new(((((*marker.borrow()) as u32) & 7_u32) as u64)));
    if ((*wiring_type.borrow())
        != ((*kBrunsliWiringTypeVarint_25.with(Value::clone).borrow()) as u64))
        && ((*wiring_type.borrow())
            != ((*kBrunsliWiringTypeLengthDelimited_26
                .with(Value::clone)
                .borrow()) as u64))
    {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*(*(*section.borrow()).upgrade().deref())
        .is_section
        .borrow_mut()) = ((*wiring_type.borrow())
        == ((*kBrunsliWiringTypeLengthDelimited_26
            .with(Value::clone)
            .borrow()) as u64));
    let tag_bit: Value<u32> = Rc::new(RefCell::new((1_u32 << (*tag.borrow()))));
    if ({
        let _lhs = (*(*(*section.borrow()).upgrade().deref()).tags_met.borrow());
        _lhs & (*tag_bit.borrow())
    } != 0)
    {
        write!(
            libcc2rs::cerr(),
            "Duplicate marker {:x}\n",
            ((*marker.borrow()) as i32),
        );
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    (*(*(*section.borrow()).upgrade().deref())
        .tags_met
        .borrow_mut()) |= (*tag_bit.borrow());
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn EnterSection_171(
    state: Ptr<brunsli_internal_dec_State>,
    section: Ptr<brunsli_internal_dec_SectionState>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    let section_size: Value<u64> = <Value<u64>>::default();
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _val: Ptr<u64> = (section_size.as_pointer());
            DecodeBase128_168(_state, _val)
        }),
    ));
    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
        return (*status.borrow()).clone();
    }
    (*(*(*section.borrow()).upgrade().deref())
        .is_active
        .borrow_mut()) = true;
    (*(*(*section.borrow()).upgrade().deref())
        .remaining
        .borrow_mut()) = (*section_size.borrow());
    (*(*(*section.borrow()).upgrade().deref())
        .milestone
        .borrow_mut()) = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
    let __rhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow())
        .wrapping_add((*(*(*section.borrow()).upgrade().deref()).remaining.borrow()));
    (*(*(*section.borrow()).upgrade().deref())
        .projected_end
        .borrow_mut()) = __rhs;
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn LeaveSection_172(section: Ptr<brunsli_internal_dec_SectionState>) {
    let section: Value<Ptr<brunsli_internal_dec_SectionState>> = Rc::new(RefCell::new(section));
    (*(*(*section.borrow()).upgrade().deref())
        .is_active
        .borrow_mut()) = false;
}
pub fn IsOutOfSectionBounds_173(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        _lhs > (*(*(*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
            .as_ref()
            .unwrap()
            .borrow())
        .section
        .borrow())
        .projected_end
        .borrow())
    };
}
pub fn RemainingSectionLength_174(state: Ptr<brunsli_internal_dec_State>) -> u64 {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    if ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        IsOutOfSectionBounds_173(_state)
    }) {
        return 0_u64;
    }
    return (*(*(*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
        .as_ref()
        .unwrap()
        .borrow())
    .section
    .borrow())
    .projected_end
    .borrow())
    .wrapping_sub((*(*(*state.borrow()).upgrade().deref()).pos.borrow()));
}
pub fn IsAtSectionBoundary_175(state: Ptr<brunsli_internal_dec_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    return {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        _lhs == (*(*(*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
            .as_ref()
            .unwrap()
            .borrow())
        .section
        .borrow())
        .projected_end
        .borrow())
    };
}
pub fn VerifySignature_176(state: Ptr<brunsli_internal_dec_State>) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _required: u64 = (*kBrunsliSignatureSize_43.with(Value::clone).borrow());
        CheckCanRead_161(_state, _required)
    }) {
        return ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            Fail_169(_state, _result)
        });
    }
    let is_signature_ok: Value<bool> = Rc::new(RefCell::new(
        (((*(*(*state.borrow()).upgrade().deref()).data.borrow())
            .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize)
            as Ptr<u8>)
            .to_any()
            .memcmp(
                &((kBrunsliSignature_44.with(Value::clone).as_pointer() as Ptr<u8>) as Ptr<u8>)
                    .to_any(),
                (*kBrunsliSignatureSize_43.with(Value::clone).borrow()) as usize,
            )
            != 0),
    ));
    let rhs_0 = (*(*(*state.borrow()).upgrade().deref()).pos.borrow())
        .wrapping_add((*kBrunsliSignatureSize_43.with(Value::clone).borrow()));
    (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
    let rhs_0 = (((*(*(*s.upgrade().deref()).section.borrow()).tags_met.borrow()) as u32)
        | (1_u32 << ((*kBrunsliSignatureTag_30.with(Value::clone).borrow()) as i32)))
        as u32;
    (*(*(*s.upgrade().deref()).section.borrow())
        .tags_met
        .borrow_mut()) = rhs_0;
    if (*is_signature_ok.borrow()) {
        return ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            Fail_169(_state, _result)
        });
    }
    return brunsli_internal_dec_Stage::HEADER;
}
pub fn DecodeHeader_177(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let hs: Ptr<brunsli_internal_dec_HeaderState> = (*s.upgrade().deref()).header.as_pointer();
    'loop_: while {
        let _lhs = (*(*hs.upgrade().deref()).stage.borrow());
        _lhs != (brunsli_internal_dec_HeaderState_Stage::DONE as u64)
    } {
        'switch: {
            let __match_cond = (*(*hs.upgrade().deref()).stage.borrow());
            match __match_cond {
                v if v == (brunsli_internal_dec_HeaderState_Stage::READ_TAG as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            ReadTag_170(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    if ({
                        let _lhs = (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow());
                        _lhs != ((*kBrunsliHeaderTag_31.with(Value::clone).borrow()) as u64)
                    }) || (!(*(*(*s.upgrade().deref()).section.borrow())
                        .is_section
                        .borrow()))
                    {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ENTER_SECTION as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ENTER_SECTION as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            EnterSection_171(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64) => {
                    if ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        IsAtSectionBoundary_175(_state)
                    }) {
                        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_HeaderState_Stage::FINALE as u64);
                        break 'switch;
                    }
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*hs.upgrade().deref()).section.as_pointer());
                            ReadTag_170(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    let tag_bit: Value<u32> = Rc::new(RefCell::new(
                        (1_u32 << (*(*(*hs.upgrade().deref()).section.borrow()).tag.borrow())),
                    ));
                    if (*(*(*hs.upgrade().deref()).section.borrow())
                        .is_section
                        .borrow())
                    {
                        if (((*kKnownHeaderVarintTags_138.with(Value::clone).borrow())
                            & (*tag_bit.borrow()))
                            != 0)
                        {
                            ({
                                let _state: Ptr<brunsli_internal_dec_State> =
                                    (*state.borrow()).clone();
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_HeaderState_Stage::ITEM_ENTER_SECTION as u64);
                        break 'switch;
                    }
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_VALUE as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_ENTER_SECTION as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _val: Ptr<u64> =
                                ((*hs.upgrade().deref()).remaining_skip_length.as_pointer());
                            DecodeBase128_168(_state, _val)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_SKIP_CONTENTS as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_SKIP_CONTENTS as u64) => {
                    let bytes_skipped: Value<u64> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _len: u64 =
                                (*(*hs.upgrade().deref()).remaining_skip_length.borrow());
                            SkipAvailableBytes_167(_state, _len)
                        }),
                    ));
                    let rhs_0 = (*(*hs.upgrade().deref()).remaining_skip_length.borrow())
                        .wrapping_sub((*bytes_skipped.borrow()));
                    (*(*hs.upgrade().deref()).remaining_skip_length.borrow_mut()) = rhs_0;
                    if ((*(*hs.upgrade().deref()).remaining_skip_length.borrow()) > 0_u64) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_VALUE as u64) => {
                    let value: Value<u64> = <Value<u64>>::default();
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _val: Ptr<u64> = (value.as_pointer());
                            DecodeBase128_168(_state, _val)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    ((*hs.upgrade().deref()).varint_values.as_pointer() as Ptr<u64>)
                        .offset(
                            (*(*(*hs.upgrade().deref()).section.borrow()).tag.borrow()) as isize,
                        )
                        .write((*value.borrow()));
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::ITEM_READ_TAG as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_HeaderState_Stage::FINALE as u64) => {
                    let has_version: Value<bool> = Rc::new(RefCell::new(
                        ({
                            let _lhs = (*(*(*hs.upgrade().deref()).section.borrow())
                                .tags_met
                                .borrow());
                            _lhs & (1_u32
                                << ((*kBrunsliHeaderVersionCompTag_41.with(Value::clone).borrow())
                                    as i32))
                        } != 0),
                    ));
                    if !(*has_version.borrow()) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let version_and_comp_count: Value<u64> = Rc::new(RefCell::new(
                        (((*hs.upgrade().deref()).varint_values.as_pointer() as Ptr<u64>)
                            .offset(
                                ((*kBrunsliHeaderVersionCompTag_41.with(Value::clone).borrow())
                                    as u64) as isize,
                            )
                            .read()),
                    ));
                    let version: Value<u64> =
                        Rc::new(RefCell::new(((*version_and_comp_count.borrow()) >> 2_u32)));
                    (*(*(*jpg.borrow()).upgrade().deref()).version.borrow_mut()) =
                        ((*version.borrow()) as i32);
                    if ((*version.borrow()) == 1_u64) {
                        (*(*(*jpg.borrow()).upgrade().deref()).width.borrow_mut()) = 0;
                        (*(*(*jpg.borrow()).upgrade().deref()).height.borrow_mut()) = 0;
                        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_HeaderState_Stage::DONE as u64);
                        break 'switch;
                    }
                    if (((*version.borrow()) & 1_u64) != 0_u64) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    if (((*version.borrow()) & (!7_u32 as u64)) != 0_u64) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*(*state.borrow()).upgrade().deref())
                        .use_legacy_context_model
                        .borrow_mut()) = !(((*version.borrow()) & 2_u64) != 0);
                    let rhs_0 = (((*(*(*s.upgrade().deref()).section.borrow()).tags_met.borrow())
                        as u32)
                        | (1_u32
                            << ((*kBrunsliOriginalJpgTag_38.with(Value::clone).borrow()) as i32)))
                        as u32;
                    (*(*(*s.upgrade().deref()).section.borrow())
                        .tags_met
                        .borrow_mut()) = rhs_0;
                    let has_width: Value<bool> = Rc::new(RefCell::new(
                        ({
                            let _lhs = (*(*(*hs.upgrade().deref()).section.borrow())
                                .tags_met
                                .borrow());
                            _lhs & (1_u32
                                << ((*kBrunsliHeaderWidthTag_39.with(Value::clone).borrow())
                                    as i32))
                        } != 0),
                    ));
                    if !(*has_width.borrow()) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let width: Value<u64> = Rc::new(RefCell::new(
                        (((*hs.upgrade().deref()).varint_values.as_pointer() as Ptr<u64>)
                            .offset(
                                ((*kBrunsliHeaderWidthTag_39.with(Value::clone).borrow()) as u64)
                                    as isize,
                            )
                            .read()),
                    ));
                    let has_height: Value<bool> = Rc::new(RefCell::new(
                        ({
                            let _lhs = (*(*(*hs.upgrade().deref()).section.borrow())
                                .tags_met
                                .borrow());
                            _lhs & (1_u32
                                << ((*kBrunsliHeaderHeightTag_40.with(Value::clone).borrow())
                                    as i32))
                        } != 0),
                    ));
                    if !(*has_height.borrow()) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let height: Value<u64> = Rc::new(RefCell::new(
                        (((*hs.upgrade().deref()).varint_values.as_pointer() as Ptr<u64>)
                            .offset(
                                ((*kBrunsliHeaderHeightTag_40.with(Value::clone).borrow()) as u64)
                                    as isize,
                            )
                            .read()),
                    ));
                    if ((*width.borrow()) == 0_u64) || ((*height.borrow()) == 0_u64) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    if ((*width.borrow())
                        > ((*kMaxDimPixels_11.with(Value::clone).borrow()) as u64))
                        || ((*height.borrow())
                            > ((*kMaxDimPixels_11.with(Value::clone).borrow()) as u64))
                    {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*(*jpg.borrow()).upgrade().deref()).width.borrow_mut()) =
                        ((*width.borrow()) as i32);
                    (*(*(*jpg.borrow()).upgrade().deref()).height.borrow_mut()) =
                        ((*height.borrow()) as i32);
                    let num_components: Value<u64> = Rc::new(RefCell::new(
                        ((*version_and_comp_count.borrow()) & 3_u64).wrapping_add(1_u64),
                    ));
                    {
                        let __a0 = (*num_components.borrow()) as usize;
                        (*(*(*jpg.borrow()).upgrade().deref()).components.borrow_mut())
                            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
                    };
                    let has_subsampling: Value<bool> = Rc::new(RefCell::new(
                        ({
                            let _lhs = (*(*(*hs.upgrade().deref()).section.borrow())
                                .tags_met
                                .borrow());
                            _lhs & (1_u32
                                << ((*kBrunsliHeaderSubsamplingTag_42.with(Value::clone).borrow())
                                    as i32))
                        } != 0),
                    ));
                    if !(*has_subsampling.borrow()) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    let subsampling_code: Value<u64> = Rc::new(RefCell::new(
                        (((*hs.upgrade().deref()).varint_values.as_pointer() as Ptr<u64>)
                            .offset(
                                ((*kBrunsliHeaderSubsamplingTag_42.with(Value::clone).borrow())
                                    as u64) as isize,
                            )
                            .read()),
                    ));
                    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
                    'loop_: while {
                        let _lhs = (*i.borrow());
                        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
                            as u64
                    } {
                        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
                            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                                as Ptr<brunsli_JPEGComponent>)
                                .offset((*i.borrow()) as isize)),
                        ));
                        (*(*(*c.borrow()).upgrade().deref())
                            .v_samp_factor
                            .borrow_mut()) =
                            ((((*subsampling_code.borrow()) & 15_u64).wrapping_add(1_u64)) as i32);
                        (*subsampling_code.borrow_mut()) >>= 4_u32;
                        (*(*(*c.borrow()).upgrade().deref())
                            .h_samp_factor
                            .borrow_mut()) =
                            ((((*subsampling_code.borrow()) & 15_u64).wrapping_add(1_u64)) as i32);
                        (*subsampling_code.borrow_mut()) >>= 4_u32;
                        if {
                            let _lhs = (*(*(*c.borrow()).upgrade().deref()).v_samp_factor.borrow());
                            _lhs > (*kBrunsliMaxSampling_27.with(Value::clone).borrow())
                        } {
                            return ({
                                let _state: Ptr<brunsli_internal_dec_State> =
                                    (*state.borrow()).clone();
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        if {
                            let _lhs = (*(*(*c.borrow()).upgrade().deref()).h_samp_factor.borrow());
                            _lhs > (*kBrunsliMaxSampling_27.with(Value::clone).borrow())
                        } {
                            return ({
                                let _state: Ptr<brunsli_internal_dec_State> =
                                    (*state.borrow()).clone();
                                let _result: brunsli_BrunsliStatus =
                                    brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                                Fail_169(_state, _result)
                            });
                        }
                        (*i.borrow_mut()).prefix_inc();
                    }
                    if !({
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        UpdateSubsamplingDerivatives_178(_jpg)
                    }) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    ({
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        PrepareMeta_179(_jpg, _state)
                    });
                    (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_HeaderState_Stage::DONE as u64);
                    break 'switch;
                }
                _ => {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    ({
        let _section: Ptr<brunsli_internal_dec_SectionState> =
            ((*s.upgrade().deref()).section.as_pointer());
        LeaveSection_172(_section)
    });
    return if ({
        let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).version.borrow());
        _lhs == (*kFallbackVersion_2.with(Value::clone).borrow())
    }) {
        brunsli_internal_dec_Stage::FALLBACK
    } else {
        brunsli_internal_dec_Stage::SECTION
    };
}
pub fn DecodeMetaDataSection_180(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let ms: Ptr<brunsli_internal_dec_MetadataState> = (*s.upgrade().deref()).metadata.as_pointer();
    if {
        let _lhs = (*(*ms.upgrade().deref()).decompression_stage.borrow()).clone();
        _lhs == brunsli_internal_dec_MetadataDecompressionStage::DONE
    } {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if {
        let _lhs = (*(*ms.upgrade().deref()).decompression_stage.borrow()).clone();
        _lhs == brunsli_internal_dec_MetadataDecompressionStage::INITIAL
    } {
        if ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsAtSectionBoundary_175(_state)
        }) {
            (*(*ms.upgrade().deref()).decompression_stage.borrow_mut()) =
                brunsli_internal_dec_MetadataDecompressionStage::DONE;
            return (brunsli_BrunsliStatus::BRUNSLI_OK).clone();
        }
        if (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        }) == 1_u64)
        {
            if !({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                CheckCanReadByte_162(_state)
            }) {
                return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
            }
            let data: Value<Box<[u8]>> = Rc::new(RefCell::new(
                (0..1).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            ));
            (*data.borrow_mut())[(0) as usize] = ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                ReadByte_163(_state)
            });
            let ok: Value<bool> = Rc::new(RefCell::new(
                ({
                    let _data: Ptr<u8> = (data.as_pointer() as Ptr<u8>);
                    let _len: u64 = 1_u64;
                    let _state: Ptr<brunsli_internal_dec_MetadataState> = (ms).clone();
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    ProcessMetaData_149(_data, _len, _state, _jpg)
                }) && ({ (*ms.upgrade().deref()).CanFinish() }),
            ));
            (*(*ms.upgrade().deref()).decompression_stage.borrow_mut()) =
                brunsli_internal_dec_MetadataDecompressionStage::DONE;
            return if (*ok.borrow()) {
                brunsli_BrunsliStatus::BRUNSLI_OK
            } else {
                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
            };
        }
        (*(*ms.upgrade().deref()).decompression_stage.borrow_mut()) =
            brunsli_internal_dec_MetadataDecompressionStage::READ_LENGTH;
    }
    if {
        let _lhs = (*(*ms.upgrade().deref()).decompression_stage.borrow()).clone();
        _lhs == brunsli_internal_dec_MetadataDecompressionStage::READ_LENGTH
    } {
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _val: Ptr<u64> = ((*ms.upgrade().deref()).metadata_size.as_pointer());
                DecodeBase128_168(_state, _val)
            }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return (*status.borrow()).clone();
        }
        if ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsOutOfSectionBounds_173(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        if (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        }) == 0_u64)
        {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*(*ms.upgrade().deref()).brotli.borrow_mut()) =
            unsafe { ::brotli_sys::BrotliDecoderCreateInstance(None, None, std::ptr::null_mut()) };
        if (*(*ms.upgrade().deref()).brotli.borrow()).is_null() {
            return (brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR).clone();
        }
        (*(*ms.upgrade().deref()).decompression_stage.borrow_mut()) =
            brunsli_internal_dec_MetadataDecompressionStage::DECOMPRESSING;
    }
    if {
        let _lhs = (*(*ms.upgrade().deref()).decompression_stage.borrow()).clone();
        _lhs == brunsli_internal_dec_MetadataDecompressionStage::DECOMPRESSING
    } {
        let finish_decompression: Value<_> = Rc::new(RefCell::new(
            (|result: brunsli_BrunsliStatus| {
                let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
                if !(!((*(*ms.upgrade().deref()).brotli.borrow()).is_null())) {
                    ({
                        let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                        let _l: i32 = 1312;
                        let _fn: Ptr<u8> = Ptr::from_string_literal("operator()");
                        BrunsliDumpAndAbort_79(_f, _l, _fn)
                    });
                    'loop_: while true {}
                };
                unsafe {
                    ::brotli_sys::BrotliDecoderDestroyInstance(
                        (*(*ms.upgrade().deref()).brotli.borrow()),
                    )
                };
                (*(*ms.upgrade().deref()).brotli.borrow_mut()) = std::ptr::null_mut();
                (*(*ms.upgrade().deref()).decompression_stage.borrow_mut()) =
                    (brunsli_internal_dec_MetadataDecompressionStage::DONE).clone();
                return (*result.borrow()).clone();
            }),
        ));
        'loop_: while true {
            let available_bytes: Value<u64> = Rc::new(RefCell::new({
                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        GetBytesAvailable_166(_state)
                    }),
                ));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        RemainingSectionLength_174(_state)
                    }),
                ));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            }));
            let available_in: Value<u64> = Rc::new(RefCell::new((*available_bytes.borrow())));
            let next_in: Value<Ptr<u8>> = Rc::new(RefCell::new(
                (*(*(*state.borrow()).upgrade().deref()).data.borrow())
                    .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize),
            ));
            let available_out: Value<u64> = Rc::new(RefCell::new(0_u64));
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
                                (*(*ms.upgrade().deref()).brotli.borrow()),
                                _v1 as *mut u64 as *mut usize,
                                _v2 as *mut *const u8,
                                _v3 as *mut u64 as *mut usize,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                            )
                        })
                    })
                })
            }));
            if (((*result.borrow()) as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_ERROR as i32)) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        (brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN).clone();
                    (*finish_decompression.borrow_mut())(_result)
                })
                .clone();
            }
            let chunk_size: Value<u64> = Rc::new(RefCell::new(0_u64));
            let chunk_data: Value<Ptr<u8>> = Rc::new(RefCell::new(unsafe {
                (chunk_size.as_pointer()).with_mut(|_v1| {
                    let output: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(
                        (*(*ms.upgrade().deref()).brotli.borrow()),
                        _v1 as *mut u64 as *mut usize,
                    );
                    let slice = std::slice::from_raw_parts(output, *_v1 as usize);
                    let result: Ptr<Vec<u8>> = Ptr::alloc(slice.to_vec());
                    (result.to_strong().as_pointer() as Ptr<u8>).clone()
                })
            }));
            let rhs_0 = (*(*ms.upgrade().deref()).decompressed_size.borrow())
                .wrapping_add((*chunk_size.borrow()));
            (*(*ms.upgrade().deref()).decompressed_size.borrow_mut()) = rhs_0;
            if {
                let _lhs = (*(*ms.upgrade().deref()).decompressed_size.borrow());
                _lhs > (*(*ms.upgrade().deref()).metadata_size.borrow())
            } {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*finish_decompression.borrow_mut())(_result)
                })
                .clone();
            }
            let consumed_bytes: Value<u64> = Rc::new(RefCell::new(
                (*available_bytes.borrow()).wrapping_sub((*available_in.borrow())),
            ));
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _len: u64 = (*consumed_bytes.borrow());
                SkipBytes_165(_state, _len)
            });
            let chunk_ok: Value<bool> = Rc::new(RefCell::new(
                ({
                    let _data: Ptr<u8> = (*chunk_data.borrow()).clone();
                    let _len: u64 = (*chunk_size.borrow());
                    let _state: Ptr<brunsli_internal_dec_MetadataState> = (ms).clone();
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    ProcessMetaData_149(_data, _len, _state, _jpg)
                }),
            ));
            if !(*chunk_ok.borrow()) {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*finish_decompression.borrow_mut())(_result)
                })
                .clone();
            }
            if (((*result.borrow()) as i32) == (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32))
            {
                if (({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    RemainingSectionLength_174(_state)
                }) != 0_u64)
                {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*finish_decompression.borrow_mut())(_result)
                    })
                    .clone();
                }
                if {
                    let _lhs = (*(*ms.upgrade().deref()).decompressed_size.borrow());
                    _lhs != (*(*ms.upgrade().deref()).metadata_size.borrow())
                } {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*finish_decompression.borrow_mut())(_result)
                    })
                    .clone();
                }
                if !({ (*ms.upgrade().deref()).CanFinish() }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*finish_decompression.borrow_mut())(_result)
                    })
                    .clone();
                }
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
                    (*finish_decompression.borrow_mut())(_result)
                })
                .clone();
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
                    let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                    let _l: i32 = 1352;
                    let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeMetaDataSection");
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            if (({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                RemainingSectionLength_174(_state)
            }) == 0_u64)
            {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*finish_decompression.borrow_mut())(_result)
                })
                .clone();
            }
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
    }
    if !(false) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1361;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeMetaDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
}
pub fn CheckBoundary_181(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    if (((*result.borrow()) as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        let last: Value<bool> = Rc::new(RefCell::new(
            ({
                let _lhs = ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    RemainingSectionLength_174(_state)
                });
                _lhs <= ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    GetBytesAvailable_166(_state)
                })
            }),
        ));
        return if (*last.borrow()) {
            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
        } else {
            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA
        };
    } else {
        return (*result.borrow()).clone();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn PrepareBitReader_182(
    br: Ptr<brunsli_BrunsliBitReader>,
    state: Ptr<brunsli_internal_dec_State>,
) {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let chunk_len: Value<u64> = Rc::new(RefCell::new({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                GetBytesAvailable_166(_state)
            }),
        ));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                RemainingSectionLength_174(_state)
            }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _buffer: Ptr<u8> = (*(*(*state.borrow()).upgrade().deref()).data.borrow())
            .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize);
        let _length: u64 = (*chunk_len.borrow());
        BrunsliBitReaderResume_128(_br, _buffer, _length)
    });
    if !({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1384;
            let _fn: Ptr<u8> = Ptr::from_string_literal("PrepareBitReader");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
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
    let chunk_len: Value<u64> = Rc::new(RefCell::new({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                GetBytesAvailable_166(_state)
            }),
        ));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                RemainingSectionLength_174(_state)
            }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let unused_bytes: Value<u64> = Rc::new(RefCell::new(
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderSuspend_130(_br)
        }),
    ));
    let consumed_bytes: Value<u64> = Rc::new(RefCell::new(
        (*chunk_len.borrow()).wrapping_sub((*unused_bytes.borrow())),
    ));
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _len: u64 = (*consumed_bytes.borrow());
        SkipBytes_165(_state, _len)
    });
    let __rhs = ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _result: brunsli_BrunsliStatus = (*result.borrow()).clone();
        CheckBoundary_181(_state, _result)
    });
    (*result.borrow_mut()) = __rhs;
    if !(({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    }) || ((((*result.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32))
        && (((*result.borrow()) as i32)
            != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))))
    {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1401;
            let _fn: Ptr<u8> = Ptr::from_string_literal("SuspendBitReader");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return (*result.borrow()).clone();
}
pub fn DecodeJPEGInternalsSection_184(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let js: Ptr<brunsli_internal_dec_JpegInternalsState> =
        (*s.upgrade().deref()).internals.as_pointer();
    let br: Value<Ptr<brunsli_BrunsliBitReader>> =
        Rc::new(RefCell::new(((*js.upgrade().deref()).br.as_pointer())));
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::INIT as i32)
    } {
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderInit_127(_br)
        });
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_MARKERS;
    }
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        PrepareBitReader_182(_br, _state)
    });
    let suspend_bit_reader: Value<_> = Rc::new(RefCell::new(
        (|result: brunsli_BrunsliStatus| {
            let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
            return ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _result: brunsli_BrunsliStatus = (*result.borrow()).clone();
                SuspendBitReader_183(_br, _state, _result)
            });
        }),
    ));
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_MARKERS as i32)
    } {
        'loop_: while true {
            if !({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u64 = 6_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            let marker: Value<u8> = Rc::new(RefCell::new(
                (((192_u32).wrapping_add(
                    ({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 6_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }),
                )) as u8),
            ));
            {
                let a0_clone = (*marker.borrow()).clone();
                (*(*(*jpg.borrow()).upgrade().deref())
                    .marker_order
                    .borrow_mut())
                .push(a0_clone)
            };
            if (((*marker.borrow()) as i32) == 196) {
                (*(*js.upgrade().deref()).dht_count.borrow_mut()).prefix_inc();
            }
            if (((*marker.borrow()) as i32) == 221) {
                (*(*js.upgrade().deref()).have_dri.borrow_mut()) = true;
            }
            if (((*marker.borrow()) as i32) == 218) {
                (*(*js.upgrade().deref()).num_scans.borrow_mut()).prefix_inc();
            }
            if (((*marker.borrow()) as i32) == 217) {
                break;
            }
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_DRI;
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_DRI as i32)
    } {
        if (*(*js.upgrade().deref()).have_dri.borrow()) {
            if !({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u64 = 16_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            (*(*(*jpg.borrow()).upgrade().deref())
                .restart_interval
                .borrow_mut()) = (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 16_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as i32);
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_HUFFMAN_LAST;
    }
    if ({
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs & (brunsli_internal_dec_JpegInternalsState_Stage::DECODE_HUFFMAN_MASK as i32)
    } != 0)
    {
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                DecodeHuffmanCode_150(_state, _jpg)
            }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return ({
                let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::PREPARE_READ_SCANS;
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::PREPARE_READ_SCANS as i32)
    } {
        if {
            let _lhs = (*(*js.upgrade().deref()).dht_count.borrow());
            _lhs != (*(*js.upgrade().deref()).terminal_huffman_code_count.borrow())
        } {
            write!(libcc2rs::cerr(), "Invalid number of DHT markers\n",);
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        if ((*(*js.upgrade().deref()).num_scans.borrow()) > 0_u64) {
            {
                let __a0 = (*(*js.upgrade().deref()).num_scans.borrow()) as usize;
                (*(*(*jpg.borrow()).upgrade().deref()).scan_info.borrow_mut())
                    .resize_with(__a0, || <brunsli_JPEGScanInfo>::default())
            };
            (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
            (*(*js.upgrade().deref()).stage.borrow_mut()) =
                brunsli_internal_dec_JpegInternalsState_Stage::READ_SCAN_COMMON;
        } else {
            (*(*js.upgrade().deref()).stage.borrow_mut()) =
                (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT).clone();
        }
    }
    if ({
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs & (brunsli_internal_dec_JpegInternalsState_Stage::DECODE_SCAN_MASK as i32)
    } != 0)
    {
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                DecodeScanInfo_151(_state, _jpg)
            }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return ({
                let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT;
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_QUANT as i32)
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let num_quant_tables: Value<i32> = Rc::new(RefCell::new(
            ((({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 2_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(1_u32)) as i32),
        ));
        {
            let __a0 = ((*num_quant_tables.borrow()) as u64) as usize;
            (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow_mut())
                .resize_with(__a0, || <brunsli_JPEGQuantTable>::default())
        };
        (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_QUANT;
    }
    'loop_: while {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_QUANT as i32)
    } {
        if {
            let _lhs = (*(*js.upgrade().deref()).i.borrow());
            _lhs >= (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len() as u64
        } {
            (*(*js.upgrade().deref()).stage.borrow_mut()) =
                brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID_SCHEME;
            break;
        }
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 7_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let q: Value<Ptr<brunsli_JPEGQuantTable>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                as Ptr<brunsli_JPEGQuantTable>)
                .offset((*(*js.upgrade().deref()).i.borrow()) as isize)),
        ));
        (*(*(*q.borrow()).upgrade().deref()).index.borrow_mut()) = (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32);
        (*(*(*q.borrow()).upgrade().deref()).is_last.borrow_mut()) = ({
            let _lhs = (*(*js.upgrade().deref()).i.borrow());
            _lhs == ((*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len() as u64)
                .wrapping_sub(1_u64)
        }) || (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 1_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) != 0);
        (*(*(*q.borrow()).upgrade().deref()).precision.borrow_mut()) = (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 4_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32);
        if ((*(*(*q.borrow()).upgrade().deref()).precision.borrow()) > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quantization table precision: {:}\n",
                (*(*(*q.borrow()).upgrade().deref()).precision.borrow()),
            );
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID_SCHEME as i32)
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let comp_ids: Value<i32> = Rc::new(RefCell::new(
            (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 2_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as i32),
        ));
        thread_local!(
            static kMinRequiredComponents_185: Value<Box<[u64]>> =
                Rc::new(RefCell::new(Box::new([3_u64, 1_u64, 3_u64, 0_u64])));
        );
        if {
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64;
            _lhs < (*kMinRequiredComponents_185.with(Value::clone).borrow())
                [(*comp_ids.borrow()) as usize]
        } {
            write!(
                libcc2rs::cerr(),
                "Insufficient number of components for ColorId #{:}\n",
                (*comp_ids.borrow()),
            );
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS;
        if ((*comp_ids.borrow()) == (*kComponentIds123_49.with(Value::clone).borrow())) {
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = 1;
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(1_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = 2;
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(2_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = 3;
        } else if ((*comp_ids.borrow()) == (*kComponentIdsGray_50.with(Value::clone).borrow())) {
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = 1;
        } else if ((*comp_ids.borrow()) == (*kComponentIdsRGB_51.with(Value::clone).borrow())) {
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = (('R' as u8) as i32);
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(1_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = (('G' as u8) as i32);
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset(2_u64 as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = (('B' as u8) as i32);
        } else {
            if !((*comp_ids.borrow()) == (*kComponentIdsCustom_52.with(Value::clone).borrow())) {
                ({
                    let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                    let _l: i32 = 1529;
                    let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeJPEGInternalsSection");
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
            (*(*js.upgrade().deref()).stage.borrow_mut()) =
                brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID;
        }
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_COMP_ID as i32)
    } {
        'loop_: while {
            let _lhs = (*(*js.upgrade().deref()).i.borrow());
            _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64
        } {
            if !({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u64 = 8_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*(*js.upgrade().deref()).i.borrow()) as isize)
                .upgrade()
                .deref())
            .id
            .borrow_mut()) = (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 8_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as i32);
            (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
        }
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS;
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_NUM_PADDING_BITS as i32)
    } {
        if !({
            let _s: Ptr<brunsli_internal_dec_VarintState> =
                ((*js.upgrade().deref()).varint.as_pointer());
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _max_symbols: u64 = 4_u64;
            DecodeLimitedVarint_146(_s, _br, _max_symbols)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let __rhs = (*(*(*js.upgrade().deref()).varint.borrow()).value.borrow());
        (*(*js.upgrade().deref()).num_padding_bits.borrow_mut()) = __rhs;
        (*(*(*jpg.borrow()).upgrade().deref())
            .has_zero_padding_bit
            .borrow_mut()) = ((*(*js.upgrade().deref()).num_padding_bits.borrow()) > 0_u64);
        if {
            let _lhs = (*(*js.upgrade().deref()).num_padding_bits.borrow());
            _lhs > ({
                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                PaddingBitsLimit_17(_jpg)
            })
        } {
            write!(
                libcc2rs::cerr(),
                "Suspicious number of padding bits {:}\n",
                (*(*js.upgrade().deref()).num_padding_bits.borrow()),
            );
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::READ_PADDING_BITS;
    }
    if {
        let _lhs = ((*(*js.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_JpegInternalsState_Stage::READ_PADDING_BITS as i32)
    } {
        'loop_: while {
            let _lhs = (*(*js.upgrade().deref()).i.borrow());
            _lhs < (*(*js.upgrade().deref()).num_padding_bits.borrow())
        } {
            if !({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u64 = 1_u64;
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            (*(*jpg.borrow()).upgrade().deref())
                .padding_bits
                .as_pointer()
                .with_mut(|__v: &mut Vec<i32>| {
                    __v.push(
                        ({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 1_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as i32,
                    )
                });
            (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
        }
        ({
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (*suspend_bit_reader.borrow_mut())(_result)
        });
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderFinish_131(_br)
        });
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*(*js.upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*js.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS;
    } else {
        ({
            let _result: brunsli_BrunsliStatus = (brunsli_BrunsliStatus::BRUNSLI_OK).clone();
            (*suspend_bit_reader.borrow_mut())(_result)
        });
    }
    'loop_: while true {
        switch!(match ((*(*js.upgrade().deref()).stage.borrow()) as i32) {
            v if v == (brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS as i32) => {
                if {
                    let _lhs = (*(*js.upgrade().deref()).i.borrow());
                    _lhs >= (*(*(*jpg.borrow()).upgrade().deref()).marker_order.borrow()).len()
                        as u64
                } {
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::DONE;
                } else if (((((*(*jpg.borrow()).upgrade().deref())
                    .marker_order
                    .as_pointer() as Ptr<u8>)
                    .offset((*(*js.upgrade().deref()).i.borrow()) as isize)
                    .read()) as i32)
                    == 255)
                {
                    (*(*js.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_LENGTH;
                } else {
                    (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
                };
                continue 'loop_;
            }
            v if v
                == (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_LENGTH
                    as i32) =>
            {
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _val: Ptr<u64> =
                            ((*js.upgrade().deref()).intermarker_length.as_pointer());
                        DecodeBase128_168(_state, _val)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        CheckBoundary_181(_state, _result)
                    });
                }
                if {
                    let _lhs = (*(*js.upgrade().deref()).intermarker_length.borrow());
                    _lhs > ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        RemainingSectionLength_174(_state)
                    })
                } {
                    return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                }
                (*(*jpg.borrow()).upgrade().deref())
                    .inter_marker_data
                    .as_pointer()
                    .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                        __v.push(Rc::new(RefCell::new(Vec::new())))
                    });
                (*(*js.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_DATA;
                continue 'loop_;
            }
            v if v
                == (brunsli_internal_dec_JpegInternalsState_Stage::READ_INTERMARKER_DATA
                    as i32) =>
            {
                let dest: Ptr<Vec<u8>> = (*(*(*jpg.borrow()).upgrade().deref())
                    .inter_marker_data
                    .borrow())[(*(*(*jpg.borrow()).upgrade().deref())
                    .inter_marker_data
                    .borrow())
                .len()
                    - 1]
                .as_pointer();
                let piece_limit: Value<u64> = Rc::new(RefCell::new(
                    (*(*js.upgrade().deref()).intermarker_length.borrow())
                        .wrapping_sub((*dest.upgrade().deref()).len() as u64),
                ));
                let piece_size: Value<u64> = Rc::new(RefCell::new({
                    let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            GetBytesAvailable_166(_state)
                        }),
                    ));
                    (if piece_limit.as_pointer().read() <= __tmp_1.as_pointer().read() {
                        piece_limit.as_pointer()
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                }));
                ({
                    let _dst: Ptr<Vec<u8>> = (dest).clone();
                    let _begin: Ptr<u8> = (*(*(*state.borrow()).upgrade().deref()).data.borrow())
                        .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize);
                    let _length: u64 = (*piece_size.borrow());
                    Append_72(_dst, _begin, _length)
                });
                ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _len: u64 = (*piece_size.borrow());
                    SkipBytes_165(_state, _len)
                });
                if {
                    let _lhs = (*dest.upgrade().deref()).len() as u64;
                    _lhs < (*(*js.upgrade().deref()).intermarker_length.borrow())
                } {
                    if !(({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        GetBytesAvailable_166(_state)
                    }) == 0_u64)
                    {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 1613;
                            let _fn: Ptr<u8> =
                                Ptr::from_string_literal("DecodeJPEGInternalsSection");
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    if !(({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        RemainingSectionLength_174(_state)
                    }) > 0_u64)
                    {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 1614;
                            let _fn: Ptr<u8> =
                                Ptr::from_string_literal("DecodeJPEGInternalsSection");
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                }
                (*(*js.upgrade().deref()).i.borrow_mut()).prefix_inc();
                (*(*js.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_JpegInternalsState_Stage::ITERATE_MARKERS;
                continue 'loop_;
            }
            _ => {}
        });
        break;
    }
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        IsAtSectionBoundary_175(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn DecodeQuantDataSection_186(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let qs: Ptr<brunsli_internal_dec_QuantDataState> = (*s.upgrade().deref()).quant.as_pointer();
    let br: Value<Ptr<brunsli_BrunsliBitReader>> =
        Rc::new(RefCell::new(((*qs.upgrade().deref()).br.as_pointer())));
    if {
        let _lhs = ((*(*qs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_QuantDataState_Stage::INIT as i32)
    } {
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderInit_127(_br)
        });
        (*(*qs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_QuantDataState_Stage::READ_NUM_QUANT;
    }
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        PrepareBitReader_182(_br, _state)
    });
    let suspend_bit_reader: Value<_> = Rc::new(RefCell::new(
        (|result: brunsli_BrunsliStatus| {
            let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
            return ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _result: brunsli_BrunsliStatus = (*result.borrow()).clone();
                SuspendBitReader_183(_br, _state, _result)
            });
        }),
    ));
    if {
        let _lhs = ((*(*qs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_QuantDataState_Stage::READ_NUM_QUANT as i32)
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let num_quant_tables: Value<u64> = Rc::new(RefCell::new(
            ((({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 2_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(1_u32)) as u64),
        ));
        if {
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len() as u64;
            _lhs != (*num_quant_tables.borrow())
        } {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        {
            let __a0 = ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64) as usize;
            (*(*qs.upgrade().deref()).predictor.borrow_mut()).resize_with(__a0, || <u8>::default())
        };
        (*(*qs.upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*qs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_QuantDataState_Stage::READ_STOCK;
    }
    'loop_: while true {
        switch!(match ((*(*qs.upgrade().deref()).stage.borrow()) as i32) {
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_STOCK as i32) => {
                if {
                    let _lhs = (*(*qs.upgrade().deref()).i.borrow());
                    _lhs >= (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len() as u64
                } {
                    std::mem::swap(
                        &mut Vec::new(),
                        &mut (*(*qs.upgrade().deref()).predictor.borrow_mut()),
                    );
                    (*(*qs.upgrade().deref()).i.borrow_mut()) = 0_u64;
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_QuantDataState_Stage::READ_QUANT_IDX;
                    continue 'loop_;
                }
                if !({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u64 = 4_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                (*(*qs.upgrade().deref()).data_precision.borrow_mut()) = 0_u8;
                let is_short: Value<bool> = Rc::new(RefCell::new(
                    !(({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 1_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) != 0),
                ));
                if (*is_short.borrow()) {
                    let short_code: Value<u64> = Rc::new(RefCell::new(
                        (({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = 3_u32;
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        }) as u64),
                    ));
                    let table: Value<Ptr<i32>> = Rc::new(RefCell::new(
                        ((*((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                            as Ptr<brunsli_JPEGQuantTable>)
                            .offset((*(*qs.upgrade().deref()).i.borrow()) as isize)
                            .upgrade()
                            .deref())
                        .values
                        .as_pointer() as Ptr<i32>),
                    ));
                    let selector: Value<u64> = Rc::new(RefCell::new(
                        (if ((*(*qs.upgrade().deref()).i.borrow()) > 0_u64) {
                            1
                        } else {
                            0
                        } as u64),
                    ));
                    let k: Value<u64> = Rc::new(RefCell::new(0_u64));
                    'loop_: while ((*k.borrow())
                        < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64))
                    {
                        let __rhs = ((*kStockQuantizationTables_48.with(Value::clone).borrow())
                            [(*selector.borrow()) as usize]
                            .borrow()[(*short_code.borrow()) as usize]
                            .borrow()[(*k.borrow()) as usize]
                            as i32);
                        (*table.borrow())
                            .offset((*k.borrow()) as isize)
                            .write(__rhs);
                        (*k.borrow_mut()).prefix_inc();
                    }
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_QuantDataState_Stage::UPDATE;
                } else {
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_QuantDataState_Stage::READ_Q_FACTOR).clone();
                };
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_Q_FACTOR as i32) => {
                if !({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u64 = 6_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                let q_factor: Value<u32> = Rc::new(RefCell::new(
                    ({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = 6_u32;
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }),
                ));
                ({
                    let _is_chroma: bool = ((*(*qs.upgrade().deref()).i.borrow()) > 0_u64);
                    let _q: u32 = (*q_factor.borrow());
                    let _dst: Ptr<u8> = ((*qs.upgrade().deref()).predictor.as_pointer() as Ptr<u8>);
                    FillQuantMatrix_118(_is_chroma, _q, _dst)
                });
                (*(*qs.upgrade().deref()).j.borrow_mut()) = 0_u64;
                (*(*qs.upgrade().deref()).delta.borrow_mut()) = 0;
                (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO;
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO as i32) => {
                if {
                    let _lhs = (*(*qs.upgrade().deref()).j.borrow());
                    _lhs >= ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)
                } {
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_QuantDataState_Stage::UPDATE;
                    continue 'loop_;
                }
                if !({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u64 = 1_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                if (({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) != 0)
                {
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_SIGN;
                } else {
                    (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF).clone();
                };
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_SIGN as i32) => {
                if !({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u64 = 1_u64;
                    BrunsliBitReaderCanRead_134(_br, _n_bits)
                }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                (*(*qs.upgrade().deref()).sign.borrow_mut()) = if (({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) != 0)
                {
                    -1_i32
                } else {
                    1
                };
                (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_QuantDataState_Stage::READ_DIFF;
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::READ_DIFF as i32) => {
                if !({
                    let _s: Ptr<brunsli_internal_dec_VarintState> =
                        ((*qs.upgrade().deref()).vs.as_pointer());
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _max_bits: u64 = 16_u64;
                    DecodeVarint_144(_s, _br, _max_bits)
                }) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                let diff: Value<i32> = Rc::new(RefCell::new(
                    (((*(*(*qs.upgrade().deref()).vs.borrow()).value.borrow()) as i32) + 1),
                ));
                let __rhs = {
                    let _lhs = (*(*qs.upgrade().deref()).sign.borrow());
                    _lhs * (*diff.borrow())
                };
                (*(*qs.upgrade().deref()).delta.borrow_mut()) += __rhs;
                (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF;
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::APPLY_DIFF as i32) => {
                let k: Value<i32> = Rc::new(RefCell::new(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                        [(*(*qs.upgrade().deref()).j.borrow()) as usize]
                        as i32),
                ));
                let quant_value: Value<i32> = Rc::new(RefCell::new({
                    let _lhs = ((((*qs.upgrade().deref()).predictor.as_pointer() as Ptr<u8>)
                        .offset(((*k.borrow()) as u64) as isize)
                        .read()) as i32);
                    _lhs + (*(*qs.upgrade().deref()).delta.borrow())
                }));
                ((*((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                    as Ptr<brunsli_JPEGQuantTable>)
                    .offset((*(*qs.upgrade().deref()).i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .values
                .as_pointer() as Ptr<i32>)
                    .offset(((*k.borrow()) as u64) as isize)
                    .write((*quant_value.borrow()));
                if ((*quant_value.borrow()) <= 0) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                if ((*quant_value.borrow()) >= 256) {
                    (*(*qs.upgrade().deref()).data_precision.borrow_mut()) = 1_u8;
                }
                if ((*quant_value.borrow()) >= 65536) {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                (*(*qs.upgrade().deref()).j.borrow_mut()).prefix_inc();
                (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_QuantDataState_Stage::READ_DIFF_IS_ZERO;
                continue 'loop_;
            }
            v if v == (brunsli_internal_dec_QuantDataState_Stage::UPDATE as i32) => {
                if {
                    let _lhs = (*(*((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                        as Ptr<brunsli_JPEGQuantTable>)
                        .offset((*(*qs.upgrade().deref()).i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .precision
                    .borrow());
                    _lhs < ((*(*qs.upgrade().deref()).data_precision.borrow()) as i32)
                } {
                    return ({
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        (*suspend_bit_reader.borrow_mut())(_result)
                    })
                    .clone();
                }
                (*(*qs.upgrade().deref()).i.borrow_mut()).prefix_inc();
                (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_QuantDataState_Stage::READ_STOCK;
                continue 'loop_;
            }
            _ => {}
        });
        break;
    }
    'loop_: while {
        let _lhs = ((*(*qs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_QuantDataState_Stage::READ_QUANT_IDX as i32)
    } {
        if {
            let _lhs = (*(*qs.upgrade().deref()).i.borrow());
            _lhs >= (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64
        } {
            (*(*qs.upgrade().deref()).stage.borrow_mut()) =
                brunsli_internal_dec_QuantDataState_Stage::FINISH;
            continue 'loop_;
        }
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*(*qs.upgrade().deref()).i.borrow()) as isize)),
        ));
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 2_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*(*c.borrow()).upgrade().deref()).quant_idx.borrow_mut()) = (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as u8);
        if {
            let _lhs = ((*(*(*c.borrow()).upgrade().deref()).quant_idx.borrow()) as u64);
            _lhs >= (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len() as u64
        } {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*qs.upgrade().deref()).i.borrow_mut()).prefix_inc();
    }
    if !({
        let _lhs = ((*(*qs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_QuantDataState_Stage::FINISH as i32)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1787;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeQuantDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    ({
        let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
        (*suspend_bit_reader.borrow_mut())(_result)
    });
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderFinish_131(_br)
    });
    if !({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        IsAtSectionBoundary_175(_state)
    }) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn DecodeHistogramDataSection_187(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let hs: Ptr<brunsli_internal_dec_HistogramDataState> =
        (*s.upgrade().deref()).histogram.as_pointer();
    let br: Value<Ptr<brunsli_BrunsliBitReader>> =
        Rc::new(RefCell::new(((*hs.upgrade().deref()).br.as_pointer())));
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::INIT as i32)
    } {
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderInit_127(_br)
        });
        if !(!(*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).is_empty()) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 1802;
                let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeHistogramDataSection");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        (*(*s.upgrade().deref()).num_contexts.borrow_mut()) =
            (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64;
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::READ_SCHEME;
        ({
            let _limit: u64 = 648_u64;
            (*(*hs.upgrade().deref()).arena.borrow()).reserve(_limit)
        });
    }
    ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        PrepareBitReader_182(_br, _state)
    });
    if {
        let _lhs = ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        });
        _lhs <= ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            GetBytesAvailable_166(_state)
        })
    } {
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderSetOptimistic_133(_br)
        });
    }
    let suspend_bit_reader: Value<_> = Rc::new(RefCell::new(
        (|result: brunsli_BrunsliStatus| {
            let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
            return ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _result: brunsli_BrunsliStatus = (*result.borrow()).clone();
                SuspendBitReader_183(_br, _state, _result)
            });
        }),
    ));
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::READ_SCHEME as i32)
    } {
        let num_components: Value<u64> = Rc::new(RefCell::new(
            (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64,
        ));
        if !((*num_components.borrow()) <= 4_u64) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 1822;
                let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeHistogramDataSection");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = (3_u64).wrapping_mul((*num_components.borrow()));
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let scheme: Value<u64> = Rc::new(RefCell::new(
                (({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = 3_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }) as u64),
            ));
            if ((*scheme.borrow()) >= ((*kNumSchemes_91.with(Value::clone).borrow()) as u64)) {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            let m: Ptr<brunsli_internal_dec_ComponentMeta> =
                ((*(*state.borrow()).upgrade().deref()).meta.as_pointer()
                    as Ptr<brunsli_internal_dec_ComponentMeta>)
                    .offset((*i.borrow()) as isize);
            (*(*m.upgrade().deref()).context_bits.borrow_mut()) = (*scheme.borrow());
            (*(*m.upgrade().deref()).context_offset.borrow_mut()) =
                (*(*s.upgrade().deref()).num_contexts.borrow());
            let rhs_0 = (*(*s.upgrade().deref()).num_contexts.borrow()).wrapping_add(
                ((*kNumNonzeroContextSkip_94.with(Value::clone).borrow())
                    [(*scheme.borrow()) as usize] as u64),
            );
            (*(*s.upgrade().deref()).num_contexts.borrow_mut()) = rhs_0;
            (*i.borrow_mut()).prefix_inc();
        }
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::READ_NUM_HISTOGRAMS;
    }
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::READ_NUM_HISTOGRAMS as i32)
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = 11_u64;
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*s.upgrade().deref()).num_histograms.borrow_mut()) = ((({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            DecodeVarLenUint8_143(_br)
        })
        .wrapping_add(1_u32))
            as u64);
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        if (*(*s.upgrade().deref()).shallow_histograms.borrow()) {
            (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                brunsli_internal_dec_HistogramDataState_Stage::SKIP_CONTENT;
        } else {
            {
                let __a0 = (*(*s.upgrade().deref()).num_contexts.borrow())
                    .wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow()))
                    as usize;
                (*(*s.upgrade().deref()).context_map_.borrow_mut())
                    .resize_with(__a0, || <u8>::default())
            };
            (*(*(*state.borrow()).upgrade().deref())
                .context_map
                .borrow_mut()) = ((*s.upgrade().deref()).context_map_.as_pointer() as Ptr<u8>);
            {
                let __a0 = (*(*s.upgrade().deref()).num_histograms.borrow()) as usize;
                (*(*s.upgrade().deref()).entropy_codes_.borrow_mut())
                    .resize_with(__a0, || <brunsli_ANSDecodingData>::default())
            };
            (*(*(*state.borrow()).upgrade().deref())
                .entropy_codes
                .borrow_mut()) = ((*s.upgrade().deref()).entropy_codes_.as_pointer()
                as Ptr<brunsli_ANSDecodingData>);
            if ((*(*s.upgrade().deref()).num_histograms.borrow()) > 1_u64) {
                (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP_CODE;
            } else {
                (*(*hs.upgrade().deref()).i.borrow_mut()) = 0_u64;
                {
                    let __a0 =
                        ((*kCoeffAlphabetSize_136.with(Value::clone).borrow()) as u64) as usize;
                    (*(*hs.upgrade().deref()).counts.borrow_mut())
                        .resize_with(__a0, || <u32>::default())
                };
                (*(*hs.upgrade().deref()).stage.borrow_mut()) =
                    brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS;
            }
        }
    }
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::SKIP_CONTENT as i32)
    } {
        ({
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (*suspend_bit_reader.borrow_mut())(_result)
        });
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _len: u64 = ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                RemainingSectionLength_174(_state)
            });
            SkipAvailableBytes_167(_state, _len)
        });
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::DONE;
    }
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP_CODE as i32)
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 = (207_u64).wrapping_add(
                (*(*s.upgrade().deref()).num_histograms.borrow()).wrapping_mul(8_u64),
            );
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*hs.upgrade().deref()).max_run_length_prefix.borrow_mut()) = 0_u64;
        let use_rle_for_zeros: Value<bool> = Rc::new(RefCell::new(
            !!(({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 1_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) != 0),
        ));
        if (*use_rle_for_zeros.borrow()) {
            (*(*hs.upgrade().deref()).max_run_length_prefix.borrow_mut()) = ((({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 4_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(1_u32))
                as u64);
        }
        let alphabet_size: Value<u64> = Rc::new(RefCell::new(
            (*(*s.upgrade().deref()).num_histograms.borrow())
                .wrapping_add((*(*hs.upgrade().deref()).max_run_length_prefix.borrow())),
        ));
        {
            let _p: Ptr<_> = Ptr::alloc(<brunsli_HuffmanDecodingData>::default());
            (*(*hs.upgrade().deref()).entropy.borrow_mut()) = _p.to_owned_opt()
        };
        if !({
            let _alphabet_size: u64 = (*alphabet_size.borrow());
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _arena: Ptr<brunsli_Arena> = ((*hs.upgrade().deref()).arena.as_pointer());
            (*(*(*hs.upgrade().deref()).entropy.borrow())
                .as_ref()
                .unwrap()
                .borrow())
            .ReadFromBitStream(_alphabet_size, _br, Some(_arena))
        }) {
            return ({
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*hs.upgrade().deref()).i.borrow_mut()) = 0_u64;
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP;
    }
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::READ_CONTEXT_MAP as i32)
    } {
        let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                let _entropy: Ptr<brunsli_HuffmanDecodingData> =
                    (*(*hs.upgrade().deref()).entropy.borrow()).as_pointer();
                let _max_run_length_prefix: u64 =
                    (*(*hs.upgrade().deref()).max_run_length_prefix.borrow());
                let _index: Ptr<u64> = ((*hs.upgrade().deref()).i.as_pointer());
                let _context_map: Ptr<Vec<u8>> = ((*s.upgrade().deref()).context_map_.as_pointer());
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                DecodeContextMap_188(_entropy, _max_run_length_prefix, _index, _context_map, _br)
            }),
        ));
        if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
            return ({
                let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                (*suspend_bit_reader.borrow_mut())(_result)
            })
            .clone();
        }
        (*(*hs.upgrade().deref()).i.borrow_mut()) = 0_u64;
        {
            let __a0 = ((*kCoeffAlphabetSize_136.with(Value::clone).borrow()) as u64) as usize;
            (*(*hs.upgrade().deref()).counts.borrow_mut()).resize_with(__a0, || <u32>::default())
        };
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS;
    }
    if {
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::READ_HISTOGRAMS as i32)
    } {
        'loop_: while {
            let _lhs = (*(*hs.upgrade().deref()).i.borrow());
            _lhs < (*(*s.upgrade().deref()).num_histograms.borrow())
        } {
            if !({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u64 =
                    ((9 + ((*kCoeffAlphabetSize_136.with(Value::clone).borrow()) * 11)) as u64);
                BrunsliBitReaderCanRead_134(_br, _n_bits)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus =
                        brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            if !({
                let _precision_bits: u32 =
                    ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as u32);
                let _counts: Ptr<Vec<u32>> = ((*hs.upgrade().deref()).counts.as_pointer());
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                ReadHistogram_189(_precision_bits, _counts, _br)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            if !({
                let _counts: Ptr<Vec<u32>> = (*hs.upgrade().deref()).counts.as_pointer();
                (*((*s.upgrade().deref()).entropy_codes_.as_pointer()
                    as Ptr<brunsli_ANSDecodingData>)
                    .offset((*(*hs.upgrade().deref()).i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(_counts)
            }) {
                return ({
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    (*suspend_bit_reader.borrow_mut())(_result)
                })
                .clone();
            }
            (*(*hs.upgrade().deref()).i.borrow_mut()).prefix_inc();
        }
        {
            let _p: Ptr<_> = Default::default();
            (*(*hs.upgrade().deref()).entropy.borrow_mut()) = _p.to_owned_opt()
        };
        std::mem::swap(
            &mut Vec::new(),
            &mut (*(*hs.upgrade().deref()).counts.borrow_mut()),
        );
        ({
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_OK;
            (*suspend_bit_reader.borrow_mut())(_result)
        });
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderFinish_131(_br)
        });
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            BrunsliBitReaderIsHealthy_132(_br)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
        (*(*hs.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_HistogramDataState_Stage::DONE;
    }
    ({ (*(*hs.upgrade().deref()).arena.borrow()).reset() });
    if !({
        let _lhs = ((*(*hs.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_HistogramDataState_Stage::DONE as i32)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1925;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeHistogramDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return brunsli_BrunsliStatus::BRUNSLI_OK;
}
pub fn DecodeDCDataSection_190(state: Ptr<brunsli_internal_dec_State>) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available: Value<u64> = Rc::new(RefCell::new(
        (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            GetBytesAvailable_166(_state)
        }) & (!1 as u64)),
    ));
    let limit: Value<u64> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        }),
    ));
    if !(((*limit.borrow()) & 1_u64) == 0_u64) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1932;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeDCDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let chunk_len: Value<u64> = Rc::new(RefCell::new(
        (if available.as_pointer().read() <= limit.as_pointer().read() {
            available.as_pointer()
        } else {
            limit.as_pointer()
        }
        .read()),
    ));
    let is_last_chunk: Value<bool> =
        Rc::new(RefCell::new(((*chunk_len.borrow()) == (*limit.borrow()))));
    let in_: Value<brunsli_WordSource> =
        Rc::new(RefCell::new(brunsli_WordSource::brunsli_WordSource(
            (*(*(*state.borrow()).upgrade().deref()).data.borrow())
                .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize),
            (*chunk_len.borrow()),
            (*is_last_chunk.borrow()),
        )));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _in: Ptr<brunsli_WordSource> = (in_.as_pointer());
            DecodeDC_157(_state, _in)
        }),
    ));
    if !(((*(*in_.borrow()).pos_.borrow()) & 1_u64) == 0_u64) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1941;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeDCDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if (*(*in_.borrow()).error_.borrow()) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !((*(*in_.borrow()).pos_.borrow()) <= (*chunk_len.borrow())) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1943;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeDCDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _len: u64 = (*(*in_.borrow()).pos_.borrow());
        SkipBytes_165(_state, _len)
    });
    if (*is_last_chunk.borrow()) {
        if !(((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 1946;
                let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeDCDataSection");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
    }
    return (*status.borrow()).clone();
}
pub fn DecodeACDataSection_191(state: Ptr<brunsli_internal_dec_State>) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available: Value<u64> = Rc::new(RefCell::new(
        (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            GetBytesAvailable_166(_state)
        }) & (!1 as u64)),
    ));
    let limit: Value<u64> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        }),
    ));
    if !(((*limit.borrow()) & 1_u64) == 0_u64) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1955;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeACDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let chunk_len: Value<u64> = Rc::new(RefCell::new(
        (if available.as_pointer().read() <= limit.as_pointer().read() {
            available.as_pointer()
        } else {
            limit.as_pointer()
        }
        .read()),
    ));
    let is_last_chunk: Value<bool> =
        Rc::new(RefCell::new(((*chunk_len.borrow()) == (*limit.borrow()))));
    let in_: Value<brunsli_WordSource> =
        Rc::new(RefCell::new(brunsli_WordSource::brunsli_WordSource(
            (*(*(*state.borrow()).upgrade().deref()).data.borrow())
                .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize),
            (*chunk_len.borrow()),
            (*is_last_chunk.borrow()),
        )));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _in: Ptr<brunsli_WordSource> = (in_.as_pointer());
            DecodeAC_160(_state, _in)
        }),
    ));
    if !(((*(*in_.borrow()).pos_.borrow()) & 1_u64) == 0_u64) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1964;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeACDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if (*(*in_.borrow()).error_.borrow()) {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
    }
    if !((*(*in_.borrow()).pos_.borrow()) <= (*chunk_len.borrow())) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 1966;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeACDataSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        let _len: u64 = (*(*in_.borrow()).pos_.borrow());
        SkipBytes_165(_state, _len)
    });
    if (*is_last_chunk.borrow()) {
        if !(((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 1969;
                let _fn: Ptr<u8> = Ptr::from_string_literal("DecodeACDataSection");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            IsAtSectionBoundary_175(_state)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
        }
    }
    return (*status.borrow()).clone();
}
pub fn DecodeOriginalJpg_192(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let fs: Ptr<brunsli_internal_dec_FallbackState> = (*s.upgrade().deref()).fallback.as_pointer();
    'loop_: while {
        let _lhs = (*(*fs.upgrade().deref()).stage.borrow());
        _lhs != (brunsli_internal_dec_FallbackState_Stage::DONE as u64)
    } {
        'switch: {
            let __match_cond = (*(*fs.upgrade().deref()).stage.borrow());
            match __match_cond {
                v if v == (brunsli_internal_dec_FallbackState_Stage::READ_TAG as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            ReadTag_170(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    if ({
                        let _lhs = (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow());
                        _lhs != ((*kBrunsliOriginalJpgTag_38.with(Value::clone).borrow()) as u64)
                    }) || (!(*(*(*s.upgrade().deref()).section.borrow())
                        .is_section
                        .borrow()))
                    {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*fs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_FallbackState_Stage::ENTER_SECTION as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_FallbackState_Stage::ENTER_SECTION as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            EnterSection_171(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*(*jpg.borrow()).upgrade().deref())
                        .original_jpg_size
                        .borrow_mut()) = (*(*(*s.upgrade().deref()).section.borrow())
                        .remaining
                        .borrow());
                    if ((*(*(*jpg.borrow()).upgrade().deref())
                        .original_jpg_size
                        .borrow())
                        == 0_u64)
                    {
                        (*(*(*jpg.borrow()).upgrade().deref())
                            .original_jpg
                            .borrow_mut()) = Ptr::<u8>::null();
                        (*(*fs.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_FallbackState_Stage::DONE as u64).clone();
                        break 'switch;
                    }
                    (*(*fs.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_FallbackState_Stage::READ_CONTENTS as u64);
                    break 'switch;
                }
                v if v == (brunsli_internal_dec_FallbackState_Stage::READ_CONTENTS as u64) => {
                    let chunk_size: Value<u64> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            GetBytesAvailable_166(_state)
                        }),
                    ));
                    if ((*chunk_size.borrow()) == 0_u64) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                            Fail_169(_state, _result)
                        });
                    }
                    let src: Value<Ptr<u8>> = Rc::new(RefCell::new(
                        (*(*(*state.borrow()).upgrade().deref()).data.borrow()).offset(
                            (*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize,
                        ),
                    ));
                    if (*(*fs.upgrade().deref()).storage.borrow()).is_empty() {
                        if {
                            let _lhs = (*chunk_size.borrow());
                            _lhs >= (*(*(*jpg.borrow()).upgrade().deref())
                                .original_jpg_size
                                .borrow())
                        } {
                            (*(*(*jpg.borrow()).upgrade().deref())
                                .original_jpg
                                .borrow_mut()) = (*src.borrow()).clone();
                            ({
                                let _state: Ptr<brunsli_internal_dec_State> =
                                    (*state.borrow()).clone();
                                let _len: u64 = (*(*(*jpg.borrow()).upgrade().deref())
                                    .original_jpg_size
                                    .borrow());
                                SkipBytes_165(_state, _len)
                            });
                            (*(*fs.upgrade().deref()).stage.borrow_mut()) =
                                (brunsli_internal_dec_FallbackState_Stage::DONE as u64);
                            break 'switch;
                        }
                    }
                    let remaining: Value<u64> = Rc::new(RefCell::new(
                        (*(*(*jpg.borrow()).upgrade().deref())
                            .original_jpg_size
                            .borrow())
                        .wrapping_sub((*(*fs.upgrade().deref()).storage.borrow()).len() as u64),
                    ));
                    let to_copy: Value<u64> = Rc::new(RefCell::new(
                        (if chunk_size.as_pointer().read() <= remaining.as_pointer().read() {
                            chunk_size.as_pointer()
                        } else {
                            remaining.as_pointer()
                        }
                        .read()),
                    ));
                    {
                        let start_idx = ((*fs.upgrade().deref()).storage.as_pointer() as Ptr<u8>)
                            .to_end()
                            .get_offset();
                        let count = (*src.borrow())
                            .offset((*to_copy.borrow()) as isize)
                            .get_offset()
                            - (*src.borrow()).get_offset();
                        let temp_vec: Vec<u8> =
                            PtrValueIter::new(&(*src.borrow()), count).collect();
                        ((*fs.upgrade().deref()).storage.as_pointer() as Ptr<Vec<u8>>).with_mut(
                            |v: &mut Vec<u8>| {
                                v.splice(start_idx..start_idx, temp_vec);
                            },
                        );
                        ((*fs.upgrade().deref()).storage.as_pointer() as Ptr<Vec<u8>>) + start_idx
                    };
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _len: u64 = (*to_copy.borrow());
                        SkipBytes_165(_state, _len)
                    });
                    if {
                        let _lhs = (*(*fs.upgrade().deref()).storage.borrow()).len() as u64;
                        _lhs == (*(*(*jpg.borrow()).upgrade().deref())
                            .original_jpg_size
                            .borrow())
                    } {
                        (*(*(*jpg.borrow()).upgrade().deref())
                            .original_jpg
                            .borrow_mut()) =
                            ((*fs.upgrade().deref()).storage.as_pointer() as Ptr<u8>);
                        (*(*fs.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_FallbackState_Stage::DONE as u64).clone();
                        break 'switch;
                    }
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                        Fail_169(_state, _result)
                    });
                }
                _ => {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    ({
        let _section: Ptr<brunsli_internal_dec_SectionState> =
            ((*s.upgrade().deref()).section.as_pointer());
        LeaveSection_172(_section)
    });
    return brunsli_internal_dec_Stage::DONE;
}
pub fn ParseSection_193(state: Ptr<brunsli_internal_dec_State>) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let sh: Ptr<brunsli_internal_dec_SectionHeaderState> =
        (*s.upgrade().deref()).section_header.as_pointer();
    let result: Value<brunsli_internal_dec_Stage> =
        Rc::new(RefCell::new(brunsli_internal_dec_Stage::ERROR));
    'loop_: while {
        let _lhs = (*(*sh.upgrade().deref()).stage.borrow());
        _lhs != (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64)
    } {
        'switch: {
            let __match_cond = (*(*sh.upgrade().deref()).stage.borrow());
            match __match_cond {
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::READ_TAG as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            ReadTag_170(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32)
                        == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        if ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _tag: u32 =
                                ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u32);
                            HasSection_194(_state, _tag)
                        }) {
                            return brunsli_internal_dec_Stage::DONE;
                        }
                    }
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    if (*(*(*s.upgrade().deref()).section.borrow())
                        .is_section
                        .borrow())
                    {
                        (*(*sh.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_SectionHeaderState_Stage::ENTER_SECTION as u64);
                        continue 'loop_;
                    }
                    let tag_bit: Value<u32> = Rc::new(RefCell::new(
                        (1_u32 << (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow())),
                    ));
                    let is_known_section_tag: Value<bool> = Rc::new(RefCell::new(
                        (((*kKnownSectionTags_137.with(Value::clone).borrow())
                            & (*tag_bit.borrow()))
                            != 0),
                    ));
                    if (*is_known_section_tag.borrow()) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                    }
                    (*(*sh.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_SectionHeaderState_Stage::READ_VALUE as u64);
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::READ_VALUE as u64) => {
                    let sink: Value<u64> = <Value<u64>>::default();
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _val: Ptr<u64> = (sink.as_pointer());
                            DecodeBase128_168(_state, _val)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    (*result.borrow_mut()) = brunsli_internal_dec_Stage::SECTION;
                    (*(*sh.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64).clone();
                    continue 'loop_;
                }
                v if v == (brunsli_internal_dec_SectionHeaderState_Stage::ENTER_SECTION as u64) => {
                    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                        ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _section: Ptr<brunsli_internal_dec_SectionState> =
                                ((*s.upgrade().deref()).section.as_pointer());
                            EnterSection_171(_state, _section)
                        }),
                    ));
                    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                        return ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                            Fail_169(_state, _result)
                        });
                    }
                    (*result.borrow_mut()) = brunsli_internal_dec_Stage::SECTION_BODY;
                    (*(*sh.upgrade().deref()).stage.borrow_mut()) =
                        (brunsli_internal_dec_SectionHeaderState_Stage::DONE as u64).clone();
                    continue 'loop_;
                }
                _ => {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                }
            }
        };
    }
    (*(*sh.upgrade().deref()).stage.borrow_mut()) =
        (brunsli_internal_dec_SectionHeaderState_Stage::READ_TAG as u64);
    if !((*result.borrow()) != brunsli_internal_dec_Stage::ERROR) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 2091;
            let _fn: Ptr<u8> = Ptr::from_string_literal("ParseSection");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return (*result.borrow()).clone();
}
pub fn ProcessSection_195(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_internal_dec_Stage {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let tag_bit: Value<i32> = Rc::new(RefCell::new(
        ((1_u32 << (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow())) as i32),
    ));
    let is_known_section_tag: Value<bool> = Rc::new(RefCell::new(
        (((*kKnownSectionTags_137.with(Value::clone).borrow()) & ((*tag_bit.borrow()) as u32))
            != 0),
    ));
    let skip_section: Value<bool> = Rc::new(RefCell::new(
        (!(*is_known_section_tag.borrow()))
            || (({
                let _lhs = (*(*(*state.borrow()).upgrade().deref()).skip_tags.borrow());
                _lhs & ((*tag_bit.borrow()) as u32)
            }) != 0),
    ));
    if (*skip_section.borrow()) {
        let to_skip: Value<u64> = Rc::new(RefCell::new({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    GetBytesAvailable_166(_state)
                }),
            ));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    RemainingSectionLength_174(_state)
                }),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let rhs_0 = (*(*(*state.borrow()).upgrade().deref()).pos.borrow())
            .wrapping_add((*to_skip.borrow()));
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
        if (({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            RemainingSectionLength_174(_state)
        }) != 0_u64)
        {
            if !(({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                GetBytesAvailable_166(_state)
            }) == 0_u64)
            {
                ({
                    let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                    let _l: i32 = 2110;
                    let _fn: Ptr<u8> = Ptr::from_string_literal("ProcessSection");
                    BrunsliDumpAndAbort_79(_f, _l, _fn)
                });
                'loop_: while true {}
            };
            return ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
                Fail_169(_state, _result)
            });
        }
        return brunsli_internal_dec_Stage::SECTION;
    }
    'switch: {
        let __match_cond = (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow());
        match __match_cond {
            v if v == ((*kBrunsliMetaDataTag_32.with(Value::clone).borrow()) as u64) => {
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeMetaDataSection_180(_state, _jpg)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == ((*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow()) as u64) => {
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeJPEGInternalsSection_184(_state, _jpg)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == ((*kBrunsliQuantDataTag_34.with(Value::clone).borrow()) as u64) => {
                if !({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 =
                        ((*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeQuantDataSection_186(_state, _jpg)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == ((*kBrunsliHistogramDataTag_35.with(Value::clone).borrow()) as u64) => {
                if !({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 =
                        ((*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeHistogramDataSection_187(_state, _jpg)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == ((*kBrunsliDCDataTag_36.with(Value::clone).borrow()) as u64) => {
                if !({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 =
                        ((*kBrunsliHistogramDataTag_35.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if !({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 = ((*kBrunsliQuantDataTag_34.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if ((({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    RemainingSectionLength_174(_state)
                }) & 1_u64)
                    != 0_u64)
                {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                ({
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    WarmupMeta_196(_jpg, _state)
                });
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        DecodeDCDataSection_190(_state)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            v if v == ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u64) => {
                if !({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 = ((*kBrunsliDCDataTag_36.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                if ((({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    RemainingSectionLength_174(_state)
                }) & 1_u64)
                    != 0_u64)
                {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                        Fail_169(_state, _result)
                    });
                }
                ({
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    WarmupMeta_196(_jpg, _state)
                });
                let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
                    ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        DecodeACDataSection_191(_state)
                    }),
                ));
                if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
                    return ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus = (*status.borrow()).clone();
                        Fail_169(_state, _result)
                    });
                }
                break 'switch;
            }
            _ => {
                return ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                    Fail_169(_state, _result)
                });
            }
        }
    };
    if !({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        IsAtSectionBoundary_175(_state)
    }) {
        return ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _result: brunsli_BrunsliStatus = brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
            Fail_169(_state, _result)
        });
    }
    if {
        let _lhs = (*(*(*s.upgrade().deref()).section.borrow()).tag.borrow());
        _lhs == ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u64)
    } {
        return brunsli_internal_dec_Stage::DONE;
    }
    return brunsli_internal_dec_Stage::SECTION;
}
pub fn UpdateSubsamplingDerivatives_178(jpg: Ptr<brunsli_JPEGData>) -> bool {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64
    } {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)),
        ));
        let __rhs = (if (*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .as_pointer()
            .read()
            >= (*(*c.borrow()).upgrade().deref())
                .h_samp_factor
                .as_pointer()
                .read()
        {
            (*(*jpg.borrow()).upgrade().deref())
                .max_h_samp_factor
                .as_pointer()
        } else {
            (*(*c.borrow()).upgrade().deref())
                .h_samp_factor
                .as_pointer()
        }
        .read());
        (*(*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .borrow_mut()) = __rhs;
        let __rhs = (if (*(*jpg.borrow()).upgrade().deref())
            .max_v_samp_factor
            .as_pointer()
            .read()
            >= (*(*c.borrow()).upgrade().deref())
                .v_samp_factor
                .as_pointer()
                .read()
        {
            (*(*jpg.borrow()).upgrade().deref())
                .max_v_samp_factor
                .as_pointer()
        } else {
            (*(*c.borrow()).upgrade().deref())
                .v_samp_factor
                .as_pointer()
        }
        .read());
        (*(*(*jpg.borrow()).upgrade().deref())
            .max_v_samp_factor
            .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ({
        let _a: i32 = (*(*(*jpg.borrow()).upgrade().deref()).height.borrow());
        let _b: i32 = ((*(*(*jpg.borrow()).upgrade().deref())
            .max_v_samp_factor
            .borrow())
            * 8);
        DivCeil_142(_a, _b)
    });
    (*(*(*jpg.borrow()).upgrade().deref()).MCU_rows.borrow_mut()) = __rhs;
    let __rhs = ({
        let _a: i32 = (*(*(*jpg.borrow()).upgrade().deref()).width.borrow());
        let _b: i32 = ((*(*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .borrow())
            * 8);
        DivCeil_142(_a, _b)
    });
    (*(*(*jpg.borrow()).upgrade().deref()).MCU_cols.borrow_mut()) = __rhs;
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64
    } {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)),
        ));
        let __rhs = (({
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).MCU_cols.borrow());
            _lhs * (*(*(*c.borrow()).upgrade().deref()).h_samp_factor.borrow())
        }) as u32);
        (*(*(*c.borrow()).upgrade().deref())
            .width_in_blocks
            .borrow_mut()) = __rhs;
        let __rhs = (({
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).MCU_rows.borrow());
            _lhs * (*(*(*c.borrow()).upgrade().deref()).v_samp_factor.borrow())
        }) as u32);
        (*(*(*c.borrow()).upgrade().deref())
            .height_in_blocks
            .borrow_mut()) = __rhs;
        if !((*(*(*c.borrow()).upgrade().deref()).width_in_blocks.borrow()) <= 8205_u32) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2211;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UpdateSubsamplingDerivatives");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !((*(*(*c.borrow()).upgrade().deref()).height_in_blocks.borrow()) <= 8205_u32) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2212;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UpdateSubsamplingDerivatives");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let num_blocks: Value<u32> = Rc::new(RefCell::new(
            (*(*(*c.borrow()).upgrade().deref()).width_in_blocks.borrow())
                .wrapping_mul((*(*(*c.borrow()).upgrade().deref()).height_in_blocks.borrow())),
        ));
        if (((*num_blocks.borrow()) as u64)
            > (*kBrunsliMaxNumBlocks_18.with(Value::clone).borrow()))
        {
            return false;
        }
        (*(*(*c.borrow()).upgrade().deref()).num_blocks.borrow_mut()) = (*num_blocks.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn PrepareMeta_179(jpg: Ptr<brunsli_JPEGData>, state: Ptr<brunsli_internal_dec_State>) {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let num_components: Value<u64> = Rc::new(RefCell::new(
        (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len() as u64,
    ));
    {
        let _a0 = (*num_components.borrow()) as usize;
        ((*s.upgrade().deref()).block_state_.as_pointer() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<u8>>>| __v.resize_with(_a0, <Value<Vec<u8>>>::default),
        )
    };
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    {
        let __a0 = (*num_components.borrow()) as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_dec_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_dec_ComponentMeta>::default())
        })
    };
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let c: Ptr<brunsli_JPEGComponent> =
            ((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize);
        let m: Ptr<brunsli_internal_dec_ComponentMeta> = (meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_dec_ComponentMeta>)
            .offset((*i.borrow()) as isize);
        (*(*m.upgrade().deref()).h_samp.borrow_mut()) =
            (*(*c.upgrade().deref()).h_samp_factor.borrow());
        (*(*m.upgrade().deref()).v_samp.borrow_mut()) =
            (*(*c.upgrade().deref()).v_samp_factor.borrow());
        let __rhs = {
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).MCU_cols.borrow());
            _lhs * (*(*m.upgrade().deref()).h_samp.borrow())
        };
        (*(*m.upgrade().deref()).width_in_blocks.borrow_mut()) = __rhs;
        let __rhs = {
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).MCU_rows.borrow());
            _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
        };
        (*(*m.upgrade().deref()).height_in_blocks.borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn WarmupMeta_196(jpg: Ptr<brunsli_JPEGData>, state: Ptr<brunsli_internal_dec_State>) {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let meta: Ptr<Vec<brunsli_internal_dec_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<u64> = Rc::new(RefCell::new((*meta.upgrade().deref()).len() as u64));
    if !(*(*(*state.borrow()).upgrade().deref())
        .is_storage_allocated
        .borrow())
    {
        (*(*(*state.borrow()).upgrade().deref())
            .is_storage_allocated
            .borrow_mut()) = true;
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let num_blocks: Value<u64> = Rc::new(RefCell::new(
                (({
                    let _lhs = (*(*(meta.to_strong().as_pointer()
                        as Ptr<brunsli_internal_dec_ComponentMeta>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .width_in_blocks
                    .borrow());
                    _lhs * (*(*(meta.to_strong().as_pointer()
                        as Ptr<brunsli_internal_dec_ComponentMeta>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .height_in_blocks
                    .borrow())
                }) as u64),
            ));
            {
                let __a0 = (*num_blocks.borrow())
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64))
                    as usize;
                (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .coeffs
                .borrow_mut())
                .resize_with(__a0, || <i16>::default())
            };
            {
                let __a0 = (*num_blocks.borrow()) as usize;
                ((*s.upgrade().deref()).block_state_.as_pointer() as Ptr<Value<Vec<u8>>>)
                    .offset((*i.borrow()) as isize)
                    .with_mut(|__v: &mut Value<Vec<u8>>| {
                        (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                    })
            };
            let __rhs = (((*s.upgrade().deref()).block_state_.as_pointer() as Ptr<Value<Vec<u8>>>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>);
            (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .block_state
            .borrow_mut()) = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if !(*(*s.upgrade().deref()).is_meta_warm.borrow()) {
        (*(*s.upgrade().deref()).is_meta_warm.borrow_mut()) = true;
        let c: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*c.borrow()) < (*num_components.borrow())) {
            let m: Ptr<brunsli_internal_dec_ComponentMeta> = (meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_dec_ComponentMeta>)
                .offset((*c.borrow()) as isize);
            let q: Ptr<brunsli_JPEGQuantTable> =
                ((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                    as Ptr<brunsli_JPEGQuantTable>)
                    .offset(
                        ((*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                            as Ptr<brunsli_JPEGComponent>)
                            .offset((*c.borrow()) as isize)
                            .upgrade()
                            .deref())
                        .quant_idx
                        .borrow()) as u64) as isize,
                    );
            (*(*m.upgrade().deref()).ac_coeffs.borrow_mut()) =
                ((*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((*c.borrow()) as isize)
                    .upgrade()
                    .deref())
                .coeffs
                .as_pointer() as Ptr<i16>);
            let __rhs = {
                let _lhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
                _lhs * (*kDCTBlockSize_3.with(Value::clone).borrow())
            };
            (*(*m.upgrade().deref()).ac_stride.borrow_mut()) = __rhs;
            let __rhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
            (*(*m.upgrade().deref()).b_stride.borrow_mut()) = __rhs;
            {
                (((*m.upgrade().deref()).quant.as_pointer() as Ptr<i32>) as Ptr<i32>)
                    .to_any()
                    .memcpy(
                        &(((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>) as Ptr<i32>)
                            .to_any(),
                        ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)
                            .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64)
                            as usize,
                    );
                (((*m.upgrade().deref()).quant.as_pointer() as Ptr<i32>) as Ptr<i32>)
                    .to_any()
                    .clone()
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
            let __match_cond = (*(*(*state.borrow()).upgrade().deref()).stage.borrow());
            match __match_cond {
                v if v == brunsli_internal_dec_Stage::SIGNATURE => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        VerifySignature_176(_state)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::HEADER => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeHeader_177(_state, _jpg)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::FALLBACK => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        DecodeOriginalJpg_192(_state, _jpg)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::SECTION => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        ParseSection_193(_state)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::SECTION_BODY => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessSection_195(_state, _jpg)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
                v if v == brunsli_internal_dec_Stage::DONE => {
                    if {
                        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
                        _lhs != (*(*(*state.borrow()).upgrade().deref()).len.borrow())
                    } {
                        let __rhs = ({
                            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                            let _result: brunsli_BrunsliStatus =
                                brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                            Fail_169(_state, _result)
                        });
                        (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                        break 'switch;
                    }
                    return (brunsli_BrunsliStatus::BRUNSLI_OK).clone();
                }
                v if v == brunsli_internal_dec_Stage::ERROR => {
                    return (*(*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
                        .as_ref()
                        .unwrap()
                        .borrow())
                    .result
                    .borrow())
                    .clone();
                }
                _ => {
                    let __rhs = ({
                        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                        let _result: brunsli_BrunsliStatus =
                            brunsli_BrunsliStatus::BRUNSLI_DECOMPRESSION_ERROR;
                        Fail_169(_state, _result)
                    });
                    (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) = __rhs;
                    break 'switch;
                }
            }
        };
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ChargeBuffer_198(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = (*s.upgrade().deref()).buffer.as_pointer();
    (*(*b.upgrade().deref()).borrowed_len.borrow_mut()) = 0_u64;
    (*(*b.upgrade().deref()).external_data.borrow_mut()) =
        (*(*(*state.borrow()).upgrade().deref()).data.borrow()).clone();
    (*(*b.upgrade().deref()).external_pos.borrow_mut()) =
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
    (*(*b.upgrade().deref()).external_len.borrow_mut()) =
        (*(*(*state.borrow()).upgrade().deref()).len.borrow());
}
thread_local!(
    pub static kBufferMaxReadAhead_199: Value<u64> = Rc::new(RefCell::new(600_u64));
);
pub fn LoadInput_200(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = (*s.upgrade().deref()).buffer.as_pointer();
    if ((*(*b.upgrade().deref()).data_len.borrow()) == 0_u64) {
        (*(*(*state.borrow()).upgrade().deref()).data.borrow_mut()) =
            (*(*b.upgrade().deref()).external_data.borrow()).clone();
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) =
            (*(*b.upgrade().deref()).external_pos.borrow());
        (*(*(*state.borrow()).upgrade().deref()).len.borrow_mut()) =
            (*(*b.upgrade().deref()).external_len.borrow());
        return;
    }
    if !({
        let _lhs = (*(*b.upgrade().deref()).data_len.borrow());
        _lhs <= (*kBufferMaxReadAhead_199.with(Value::clone).borrow())
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 2337;
            let _fn: Ptr<u8> = Ptr::from_string_literal("LoadInput");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let available: Value<u64> = Rc::new(RefCell::new(
        (*(*b.upgrade().deref()).external_len.borrow())
            .wrapping_sub((*(*b.upgrade().deref()).external_pos.borrow())),
    ));
    (*(*b.upgrade().deref()).borrowed_len.borrow_mut()) = (if kBufferMaxReadAhead_199
        .with(Value::clone)
        .as_pointer()
        .read()
        <= available.as_pointer().read()
    {
        kBufferMaxReadAhead_199.with(Value::clone).as_pointer()
    } else {
        available.as_pointer()
    }
    .read());
    {
        (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>)
            .offset((*(*b.upgrade().deref()).data_len.borrow()) as isize) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((*(*b.upgrade().deref()).external_data.borrow())
                    .offset((*(*b.upgrade().deref()).external_pos.borrow()) as isize)
                    as Ptr<u8>)
                    .to_any(),
                (*(*b.upgrade().deref()).borrowed_len.borrow()) as usize,
            );
        (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>)
            .offset((*(*b.upgrade().deref()).data_len.borrow()) as isize) as Ptr<u8>)
            .to_any()
            .clone()
    };
    (*(*(*state.borrow()).upgrade().deref()).data.borrow_mut()) =
        ((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>);
    (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = 0_u64;
    (*(*(*state.borrow()).upgrade().deref()).len.borrow_mut()) =
        (*(*b.upgrade().deref()).data_len.borrow())
            .wrapping_add((*(*b.upgrade().deref()).borrowed_len.borrow()));
}
pub fn UnloadInput_201(
    state: Ptr<brunsli_internal_dec_State>,
    result: brunsli_BrunsliStatus,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let result: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(result));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = (*s.upgrade().deref()).buffer.as_pointer();
    if {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).data.borrow()).clone();
        _lhs == (*(*b.upgrade().deref()).external_data.borrow()).clone()
    } {
        (*(*b.upgrade().deref()).external_pos.borrow_mut()) =
            (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        if !({
            let _lhs = (*(*b.upgrade().deref()).external_pos.borrow());
            _lhs <= (*(*b.upgrade().deref()).external_len.borrow())
        }) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2364;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if (((*result.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            return true;
        }
        if !((*(*b.upgrade().deref()).data_len.borrow()) == 0_u64) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2366;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let available: Value<u64> = Rc::new(RefCell::new(
            (*(*b.upgrade().deref()).external_len.borrow())
                .wrapping_sub((*(*b.upgrade().deref()).external_pos.borrow())),
        ));
        if !((*available.borrow()) < (*kBufferMaxReadAhead_199.with(Value::clone).borrow())) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2368;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if (*(*b.upgrade().deref()).data.borrow()).is_empty() {
            {
                let __a0 = (2_u64)
                    .wrapping_mul((*kBufferMaxReadAhead_199.with(Value::clone).borrow()))
                    as usize;
                (*(*b.upgrade().deref()).data.borrow_mut()).resize_with(__a0, || <u8>::default())
            };
        }
        (*(*b.upgrade().deref()).data_len.borrow_mut()) = (*available.borrow());
        {
            (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &((*(*b.upgrade().deref()).external_data.borrow())
                        .offset((*(*b.upgrade().deref()).external_pos.borrow()) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    (*(*b.upgrade().deref()).data_len.borrow()) as usize,
                );
            (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .clone()
        };
        let rhs_0 =
            (*(*b.upgrade().deref()).external_pos.borrow()).wrapping_add((*available.borrow()));
        (*(*b.upgrade().deref()).external_pos.borrow_mut()) = rhs_0;
        return false;
    }
    if {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        _lhs >= (*(*b.upgrade().deref()).data_len.borrow())
    } {
        let used_borrowed_bytes: Value<u64> = Rc::new(RefCell::new(
            (*(*(*state.borrow()).upgrade().deref()).pos.borrow())
                .wrapping_sub((*(*b.upgrade().deref()).data_len.borrow())),
        ));
        (*(*b.upgrade().deref()).data_len.borrow_mut()) = 0_u64;
        let rhs_0 = (*(*b.upgrade().deref()).external_pos.borrow())
            .wrapping_add((*used_borrowed_bytes.borrow()));
        (*(*b.upgrade().deref()).external_pos.borrow_mut()) = rhs_0;
        return true;
    }
    let rhs_0 = (*(*b.upgrade().deref()).data_len.borrow())
        .wrapping_sub((*(*(*state.borrow()).upgrade().deref()).pos.borrow()));
    (*(*b.upgrade().deref()).data_len.borrow_mut()) = rhs_0;
    if (((*result.borrow()) as i32) == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)) {
        if !({
            let _lhs = (*(*b.upgrade().deref()).external_pos.borrow())
                .wrapping_add((*(*b.upgrade().deref()).borrowed_len.borrow()));
            _lhs == (*(*b.upgrade().deref()).external_len.borrow())
        }) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2389;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        if !({
            let _lhs = (*(*b.upgrade().deref()).data_len.borrow())
                .wrapping_add((*(*b.upgrade().deref()).borrowed_len.borrow()));
            _lhs < (*kBufferMaxReadAhead_199.with(Value::clone).borrow())
        }) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2391;
                let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let rhs_0 = (*(*b.upgrade().deref()).data_len.borrow())
            .wrapping_add((*(*b.upgrade().deref()).borrowed_len.borrow()));
        (*(*b.upgrade().deref()).data_len.borrow_mut()) = rhs_0;
        let rhs_0 = (*(*b.upgrade().deref()).external_pos.borrow())
            .wrapping_add((*(*b.upgrade().deref()).borrowed_len.borrow()));
        (*(*b.upgrade().deref()).external_pos.borrow_mut()) = rhs_0;
    }
    if !(!(*(*b.upgrade().deref()).data.borrow()).is_empty()) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 2395;
            let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    if ((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) > 0_u64)
        && ((*(*b.upgrade().deref()).data_len.borrow()) > 0_u64)
    {
        {
            (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &(((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>)
                        .offset((*(*(*state.borrow()).upgrade().deref()).pos.borrow()) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    (*(*b.upgrade().deref()).data_len.borrow()) as usize,
                );
            (((*b.upgrade().deref()).data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .clone()
        };
    }
    if !({
        let _lhs = (*(*b.upgrade().deref()).data_len.borrow());
        _lhs <= (*kBufferMaxReadAhead_199.with(Value::clone).borrow())
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
            let _l: i32 = 2399;
            let _fn: Ptr<u8> = Ptr::from_string_literal("UnloadInput");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    return (((*result.borrow()) as i32)
        != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        .clone();
}
pub fn UnchargeBuffer_202(state: Ptr<brunsli_internal_dec_State>) {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    let b: Ptr<brunsli_internal_dec_Buffer> = (*s.upgrade().deref()).buffer.as_pointer();
    (*(*(*state.borrow()).upgrade().deref()).data.borrow_mut()) =
        (*(*b.upgrade().deref()).external_data.borrow()).clone();
    (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) =
        (*(*b.upgrade().deref()).external_pos.borrow());
    (*(*(*state.borrow()).upgrade().deref()).len.borrow_mut()) =
        (*(*b.upgrade().deref()).external_len.borrow());
}
pub fn ProcessJpeg_203(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*(*state.borrow()).upgrade().deref()).internal.borrow()).as_pointer();
    if {
        let _lhs = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
        _lhs > (*(*(*state.borrow()).upgrade().deref()).len.borrow())
    } {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM;
    }
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        ChargeBuffer_198(_state)
    });
    let result: Value<brunsli_BrunsliStatus> =
        Rc::new(RefCell::new(brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA));
    'loop_: while (((*result.borrow()) as i32)
        == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
    {
        if {
            let _lhs = (*(*(*state.borrow()).upgrade().deref()).stage.borrow()).clone();
            _lhs == brunsli_internal_dec_Stage::ERROR
        } {
            if {
                let _lhs = ((*(*s.upgrade().deref()).result.borrow()) as i32).clone();
                _lhs != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32)
            } {
                return (*(*s.upgrade().deref()).result.borrow()).clone();
            }
            (*(*s.upgrade().deref()).result.borrow_mut()) = brunsli_BrunsliStatus::BRUNSLI_OK;
            (*(*(*state.borrow()).upgrade().deref()).stage.borrow_mut()) =
                (*(*s.upgrade().deref()).last_stage.borrow()).clone();
            (*(*s.upgrade().deref()).last_stage.borrow_mut()) =
                (brunsli_internal_dec_Stage::ERROR).clone();
        }
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            LoadInput_200(_state)
        });
        if (*(*(*s.upgrade().deref()).section.borrow())
            .is_active
            .borrow())
        {
            (*(*(*s.upgrade().deref()).section.borrow())
                .milestone
                .borrow_mut()) = (*(*(*state.borrow()).upgrade().deref()).pos.borrow());
            let __rhs = (*(*(*s.upgrade().deref()).section.borrow())
                .milestone
                .borrow())
            .wrapping_add(
                (*(*(*s.upgrade().deref()).section.borrow())
                    .remaining
                    .borrow()),
            );
            (*(*(*s.upgrade().deref()).section.borrow())
                .projected_end
                .borrow_mut()) = __rhs;
        }
        (*(*(*s.upgrade().deref()).section.borrow())
            .tags_met
            .borrow_mut()) |= (*(*(*state.borrow()).upgrade().deref()).tags_met.borrow());
        (*result.borrow_mut()) = ({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
            DoProcessJpeg_197(_state, _jpg)
        });
        if (*(*(*s.upgrade().deref()).section.borrow())
            .is_active
            .borrow())
        {
            let processed_len: Value<u64> = Rc::new(RefCell::new(
                (*(*(*state.borrow()).upgrade().deref()).pos.borrow()).wrapping_sub(
                    (*(*(*s.upgrade().deref()).section.borrow())
                        .milestone
                        .borrow()),
                ),
            ));
            let rhs_0 = (*(*(*s.upgrade().deref()).section.borrow())
                .remaining
                .borrow())
            .wrapping_sub((*processed_len.borrow()));
            (*(*(*s.upgrade().deref()).section.borrow())
                .remaining
                .borrow_mut()) = rhs_0;
        }
        if !({
            let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
            let _result: brunsli_BrunsliStatus = (*result.borrow()).clone();
            UnloadInput_201(_state, _result)
        }) {
            break;
        }
    }
    ({
        let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
        UnchargeBuffer_202(_state)
    });
    return (*result.borrow()).clone();
}
pub fn BrunsliDecodeJpeg_204(
    data: Ptr<u8>,
    len: u64,
    jpg: Ptr<brunsli_JPEGData>,
) -> brunsli_BrunsliStatus {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if !!(*data.borrow()).is_null() {
        return brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM;
    }
    let state: Value<brunsli_internal_dec_State> = Rc::new(RefCell::new(
        brunsli_internal_dec_State::brunsli_internal_dec_State(),
    ));
    (*(*state.borrow()).data.borrow_mut()) = (*data.borrow()).clone();
    (*(*state.borrow()).len.borrow_mut()) = (*len.borrow());
    return ({
        let _state: Ptr<brunsli_internal_dec_State> = (state.as_pointer());
        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
        ProcessJpeg_203(_state, _jpg)
    });
}
pub fn BrunsliEstimateDecoderPeakMemoryUsage_205(data: Ptr<u8>, len: u64) -> u64 {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    if !!(*data.borrow()).is_null() {
        return (brunsli_BrunsliStatus::BRUNSLI_INVALID_PARAM as u64);
    }
    let state: Value<brunsli_internal_dec_State> = Rc::new(RefCell::new(
        brunsli_internal_dec_State::brunsli_internal_dec_State(),
    ));
    (*(*state.borrow()).data.borrow_mut()) = (*data.borrow()).clone();
    (*(*state.borrow()).len.borrow_mut()) = (*len.borrow());
    (*(*state.borrow()).skip_tags.borrow_mut()) =
        !(1_u32 << ((*kBrunsliHistogramDataTag_35.with(Value::clone).borrow()) as i32));
    let s: Ptr<brunsli_internal_dec_InternalState> =
        (*(*state.borrow()).internal.borrow()).as_pointer();
    (*(*s.upgrade().deref()).shallow_histograms.borrow_mut()) = true;
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::brunsli_JPEGData()));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({
            let _state: Ptr<brunsli_internal_dec_State> = (state.as_pointer());
            let _jpg: Ptr<brunsli_JPEGData> = (jpg.as_pointer());
            ProcessJpeg_203(_state, _jpg)
        }),
    ));
    if (((*status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32)) {
        return 0_u64;
    }
    let out_size: Value<u64> = Rc::new(RefCell::new((2_u64).wrapping_mul((*len.borrow()))));
    let total_num_blocks: Value<u64> = Rc::new(RefCell::new(0_u64));
    let component_state_size: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*(*jpg.borrow()).components.borrow()).len() as u64) {
        let c: Ptr<brunsli_JPEGComponent> = ((*jpg.borrow()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize);
        let rhs_0 = (*total_num_blocks.borrow())
            .wrapping_add(((*(*c.upgrade().deref()).num_blocks.borrow()) as u64));
        (*total_num_blocks.borrow_mut()) = rhs_0;
        let rhs_0 = (*component_state_size.borrow()).wrapping_add(
            ({
                let _w: i32 = ((*(*c.upgrade().deref()).width_in_blocks.borrow()) as i32);
                brunsli_ComponentState::SizeInBytes(_w)
            }),
        );
        (*component_state_size.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    let jpeg_data_size: Value<u64> = Rc::new(RefCell::new(
        ((*total_num_blocks.borrow())
            .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)))
        .wrapping_mul(::std::mem::size_of::<i16>() as u64 as u64),
    ));
    let context_map_size: Value<u64> = Rc::new(RefCell::new(
        ((*(*s.upgrade().deref()).num_contexts.borrow())
            .wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow())))
        .wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64),
    ));
    let histogram_size: Value<u64> = Rc::new(RefCell::new(
        (*(*s.upgrade().deref()).num_histograms.borrow())
            .wrapping_mul(::std::mem::size_of::<brunsli_ANSDecodingData>() as u64 as u64),
    ));
    let decode_peak: Value<u64> = Rc::new(RefCell::new(
        ((*context_map_size.borrow()).wrapping_add((*histogram_size.borrow())))
            .wrapping_add((*component_state_size.borrow())),
    ));
    let jpeg_writer_size: Value<u64> =
        Rc::new(RefCell::new(((1_u32 << 17_u32) as u64).wrapping_add(
            ((1_u32 << 16_u32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64 as u64)
                as u64,
        )));
    return (((*out_size.borrow()).wrapping_add((*jpeg_data_size.borrow()))).wrapping_add(
        (if decode_peak.as_pointer().read() >= jpeg_writer_size.as_pointer().read() {
            decode_peak.as_pointer()
        } else {
            jpeg_writer_size.as_pointer()
        }
        .read()),
    ));
}
impl brunsli_BrunsliDecoder {}
impl brunsli_BrunsliDecoder {
    pub fn Decode(
        &self,
        available_in: Ptr<u64>,
        next_in: Ptr<Ptr<u8>>,
        available_out: Ptr<u64>,
        next_out: Ptr<Ptr<u8>>,
    ) -> brunsli_BrunsliDecoder_Status {
        let available_in: Value<Ptr<u64>> = Rc::new(RefCell::new(available_in));
        let next_in: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_in));
        let available_out: Value<Ptr<u64>> = Rc::new(RefCell::new(available_out));
        let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
        let jpg: Value<Ptr<brunsli_JPEGData>> =
            Rc::new(RefCell::new((*self.jpg_.borrow()).as_pointer()));
        if !!(*jpg.borrow()).is_null() {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2511;
                let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let state: Value<Ptr<brunsli_internal_dec_State>> =
            Rc::new(RefCell::new((*self.state_.borrow()).as_pointer()));
        if !!(*state.borrow()).is_null() {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2513;
                let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let __rhs = ((*next_in.borrow()).read()).clone();
        (*(*(*state.borrow()).upgrade().deref()).data.borrow_mut()) = __rhs;
        (*(*(*state.borrow()).upgrade().deref()).pos.borrow_mut()) = 0_u64;
        let __rhs = ((*available_in.borrow()).read());
        (*(*(*state.borrow()).upgrade().deref()).len.borrow_mut()) = __rhs;
        let parse_status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                ProcessJpeg_203(_state, _jpg)
            }),
        ));
        let consumed_bytes: Value<u64> = Rc::new(RefCell::new(
            (*(*(*state.borrow()).upgrade().deref()).pos.borrow()),
        ));
        let rhs_0 = ((*available_in.borrow()).read()).wrapping_sub((*consumed_bytes.borrow()));
        (*available_in.borrow()).write(rhs_0);
        let __rhs = (*consumed_bytes.borrow());
        {
            let _ptr = (*next_in.borrow()).clone();
            _ptr.write(_ptr.read() + __rhs)
        };
        if (((*parse_status.borrow()) as i32) != (brunsli_BrunsliStatus::BRUNSLI_OK as i32))
            && (((*parse_status.borrow()) as i32)
                != (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
        {
            return (brunsli_BrunsliDecoder_Status::ERROR).clone();
        }
        if !(((*available_in.borrow()).read()) == 0_u64) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                let _l: i32 = 2529;
                let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let serialization_status: Value<brunsli_internal_dec_SerializationStatus> =
            Rc::new(RefCell::new(
                ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                    let _available_out: Ptr<u64> = (*available_out.borrow()).clone();
                    let _next_out: Ptr<Ptr<u8>> = (*next_out.borrow()).clone();
                    SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
                }),
            ));
        if ((*serialization_status.borrow()) == brunsli_internal_dec_SerializationStatus::ERROR) {
            return (brunsli_BrunsliDecoder_Status::ERROR).clone();
        }
        'switch: {
            let __match_cond = (*serialization_status.borrow());
            match __match_cond {
                v if v == brunsli_internal_dec_SerializationStatus::DONE => {
                    if !(((*parse_status.borrow()) as i32)
                        == (brunsli_BrunsliStatus::BRUNSLI_OK as i32))
                    {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 2540;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::DONE;
                }
                v if v == brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT => {
                    if !(((*parse_status.borrow()) as i32)
                        == (brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA as i32))
                    {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 2545;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    return brunsli_BrunsliDecoder_Status::NEEDS_MORE_INPUT;
                }
                v if v == brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT => {
                    if !(((*available_out.borrow()).read()) == 0_u64) {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 2551;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
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
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("brunsli_decode.cc");
                            let _l: i32 = 2559;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("Decode");
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
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
pub fn MoveToFront_207(v: Ptr<u8>, index: u8) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let index: Value<u8> = Rc::new(RefCell::new(index));
    let value: Value<u8> = Rc::new(RefCell::new(
        ((*v.borrow()).offset((*index.borrow()) as isize).read()),
    ));
    let i: Value<u8> = Rc::new(RefCell::new((*index.borrow())));
    'loop_: while ((*i.borrow()) != 0) {
        let __rhs = ((*v.borrow())
            .offset((((*i.borrow()) as i32) - 1) as isize)
            .read());
        (*v.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_dec();
    }
    let __rhs = (*value.borrow());
    (*v.borrow()).offset((0) as isize).write(__rhs);
}
pub fn InverseMoveToFrontTransform_208(v: Ptr<u8>, v_len: u64) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let v_len: Value<u64> = Rc::new(RefCell::new(v_len));
    let mtf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < 256_u64) {
        let __rhs = ((*i.borrow()) as u8);
        (*mtf.borrow_mut())[(*i.borrow()) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*v_len.borrow())) {
        let index: Value<u8> = Rc::new(RefCell::new(
            ((*v.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        let __rhs = (*mtf.borrow())[(*index.borrow()) as usize];
        (*v.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        if ((*index.borrow()) != 0) {
            ({
                let _v: Ptr<u8> = (mtf.as_pointer() as Ptr<u8>);
                let _index: u8 = (*index.borrow());
                MoveToFront_207(_v, _index)
            });
        }
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn DecodeContextMap_188(
    entropy: Ptr<brunsli_HuffmanDecodingData>,
    max_run_length_prefix: u64,
    index: Ptr<u64>,
    context_map: Ptr<Vec<u8>>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> brunsli_BrunsliStatus {
    let max_run_length_prefix: Value<u64> = Rc::new(RefCell::new(max_run_length_prefix));
    let index: Value<Ptr<u64>> = Rc::new(RefCell::new(index));
    let context_map: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(context_map));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let i: Ptr<u64> = (*index.borrow()).clone();
    let map: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*context_map.borrow()).to_strong().as_pointer() as Ptr<u8>),
    ));
    let length: Value<u64> = Rc::new(RefCell::new(
        (*(*context_map.borrow()).upgrade().deref()).len() as u64,
    ));
    'loop_: while {
        let _lhs = (i.read());
        _lhs < (*length.borrow())
    } {
        if !({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u64 =
                ((15_u64).wrapping_add((*max_run_length_prefix.borrow()))).wrapping_add(1_u64);
            BrunsliBitReaderCanRead_134(_br, _n_bits)
        }) {
            return brunsli_BrunsliStatus::BRUNSLI_NOT_ENOUGH_DATA;
        }
        let code: Value<u32> = Rc::new(RefCell::new(
            (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                (*entropy.upgrade().deref()).ReadSymbol(_br)
            }) as u32),
        ));
        if ((*code.borrow()) == 0_u32) {
            (*map.borrow()).offset((i.read()) as isize).write(0_u8);
            i.with_mut(|__v| __v.prefix_inc());
        } else if (((*code.borrow()) as u64) <= (*max_run_length_prefix.borrow())) {
            let reps: Value<u64> = Rc::new(RefCell::new(
                ((((1_u32 as u32).wrapping_add((1_u32 << (*code.borrow())))).wrapping_add(
                    ((({
                        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                        let _n_bits: u32 = (*code.borrow());
                        BrunsliBitReaderRead_126(_br, _n_bits)
                    }) as i32) as u32),
                )) as u64),
            ));
            'loop_: while ((*reps.borrow_mut()).prefix_dec() != 0) {
                if {
                    let _lhs = (i.read());
                    _lhs >= (*length.borrow())
                } {
                    return brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN;
                }
                (*map.borrow()).offset((i.read()) as isize).write(0_u8);
                i.with_mut(|__v| __v.prefix_inc());
            }
        } else {
            let __rhs =
                ((((*code.borrow()) as u64).wrapping_sub((*max_run_length_prefix.borrow()))) as u8);
            (*map.borrow()).offset((i.read()) as isize).write(__rhs);
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    if (({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        let _n_bits: u32 = 1_u32;
        BrunsliBitReaderRead_126(_br, _n_bits)
    }) != 0)
    {
        ({
            let _v: Ptr<u8> = (*map.borrow()).clone();
            let _v_len: u64 = (*length.borrow());
            InverseMoveToFrontTransform_208(_v, _v_len)
        });
    }
    return if ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    }) {
        brunsli_BrunsliStatus::BRUNSLI_OK
    } else {
        brunsli_BrunsliStatus::BRUNSLI_INVALID_BRN
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
pub fn ReadShortHuffmanCode_212(br: Ptr<brunsli_BrunsliBitReader>, tree: Ptr<i8>) -> u64 {
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let tree: Value<Ptr<i8>> = Rc::new(RefCell::new(tree));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    let delta: Value<i8> = Rc::new(RefCell::new(1_i8));
    'loop_: while (((*delta.borrow()) as i32) > 0) {
        let rhs_0 = (*pos.borrow()).wrapping_add(
            ((((*delta.borrow()) as u32).wrapping_add(
                ({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = 1_u32;
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }),
            )) as u64),
        );
        (*pos.borrow_mut()) = rhs_0;
        let __rhs = ((*tree.borrow()).offset((*pos.borrow()) as isize).read());
        (*delta.borrow_mut()) = __rhs;
    }
    return (-((*delta.borrow()) as i32) as u64);
}
pub fn ReadHistogram_189(
    precision_bits: u32,
    counts: Ptr<Vec<u32>>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    let precision_bits: Value<u32> = Rc::new(RefCell::new(precision_bits));
    let counts: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(counts));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    if !(!(*(*counts.borrow()).upgrade().deref()).is_empty()) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("histogram_decode.cc");
            let _l: i32 = 41;
            let _fn: Ptr<u8> = Ptr::from_string_literal("ReadHistogram");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let space: Value<u32> = Rc::new(RefCell::new((1_u32 << (*precision_bits.borrow()))));
    let length: Value<u64> = Rc::new(RefCell::new(
        (*(*counts.borrow()).upgrade().deref()).len() as u64
    ));
    {
        let mut __a0 = ((*counts.borrow()).to_strong().as_pointer() as Ptr<u32>).clone();
        while __a0 != ((*counts.borrow()).to_strong().as_pointer() as Ptr<u32>).to_end() {
            let v = 0.clone();
            __a0.write(v);
            __a0 += 1;
        }
    };
    let histogram: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*counts.borrow()).to_strong().as_pointer() as Ptr<u32>),
    ));
    let simple_code: Value<i32> = Rc::new(RefCell::new(
        (({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 1_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        }) as i32),
    ));
    if ((*simple_code.borrow()) == 1) {
        let max_bits_counter: Value<u64> =
            Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_u64)));
        let max_bits: Value<u32> = Rc::new(RefCell::new(0_u32));
        let symbols: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, <i32>::default()])));
        let num_symbols: Value<u64> = Rc::new(RefCell::new(
            ((({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 1_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            })
            .wrapping_add(1_u32 as u32)) as u64),
        ));
        'loop_: while ((*max_bits_counter.borrow()) != 0) {
            (*max_bits_counter.borrow_mut()) >>= 1;
            (*max_bits.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
            (*symbols.borrow_mut())[(*i.borrow()) as usize] = (((({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*max_bits.borrow());
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u64)
                .wrapping_rem((*length.borrow())))
                as i32);
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*num_symbols.borrow()) == 1_u64) {
            let __rhs = (*space.borrow());
            (*histogram.borrow())
                .offset(((*symbols.borrow())[(0) as usize]) as isize)
                .write(__rhs);
        } else {
            if ((*symbols.borrow())[(0) as usize] == (*symbols.borrow())[(1) as usize]) {
                return false;
            }
            let value: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = (*precision_bits.borrow());
                    BrunsliBitReaderRead_126(_br, _n_bits)
                }),
            ));
            let __rhs = (*value.borrow());
            (*histogram.borrow())
                .offset(((*symbols.borrow())[(0) as usize]) as isize)
                .write(__rhs);
            let __rhs = (*space.borrow()).wrapping_sub((*value.borrow()));
            (*histogram.borrow())
                .offset(((*symbols.borrow())[(1) as usize]) as isize)
                .write(__rhs);
        }
    } else {
        let real_length: Value<u64> = Rc::new(RefCell::new(
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _tree: Ptr<i8> = (kLengthTree_210.with(Value::clone).as_pointer() as Ptr<i8>);
                ReadShortHuffmanCode_212(_br, _tree)
            }),
        ));
        let total_count: Value<u32> = Rc::new(RefCell::new(0_u32));
        let log_counts: Value<Box<[u32]>> = Rc::new(RefCell::new(
            (0..18).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
        ));
        let omit_pos: Value<u64> = Rc::new(RefCell::new(0_u64));
        if !((*real_length.borrow()) > 2_u64) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("histogram_decode.cc");
                let _l: i32 = 74;
                let _fn: Ptr<u8> = Ptr::from_string_literal("ReadHistogram");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*real_length.borrow())) {
            (*log_counts.borrow_mut())[(*i.borrow()) as usize] = (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _tree: Ptr<i8> = (kLogCountTree_211.with(Value::clone).as_pointer() as Ptr<i8>);
                ReadShortHuffmanCode_212(_br, _tree)
            }) as u32);
            if ((*log_counts.borrow())[(*i.borrow()) as usize]
                > (*log_counts.borrow())[(*omit_pos.borrow()) as usize])
            {
                (*omit_pos.borrow_mut()) = (*i.borrow());
            }
            (*i.borrow_mut()).prefix_inc();
        }
        if !((*omit_pos.borrow()) >= 0_u64) {
            ({
                let _f: Ptr<u8> = Ptr::from_string_literal("histogram_decode.cc");
                let _l: i32 = 80;
                let _fn: Ptr<u8> = Ptr::from_string_literal("ReadHistogram");
                BrunsliDumpAndAbort_79(_f, _l, _fn)
            });
            'loop_: while true {}
        };
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
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
                (*histogram.borrow())
                    .offset((*i.borrow()) as isize)
                    .write(1_u32);
            } else {
                let bit_count: Value<u32> = Rc::new(RefCell::new(
                    ({
                        let _logcount: u32 = (*code.borrow()).wrapping_sub(1_u32);
                        GetPopulationCountPrecision_209(_logcount)
                    }),
                ));
                let __rhs = (1_u32 << ((*code.borrow()).wrapping_sub(1_u32))).wrapping_add(
                    ({
                        let _lhs = ({
                            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                            let _n_bits: u32 = (*bit_count.borrow());
                            BrunsliBitReaderRead_126(_br, _n_bits)
                        });
                        _lhs << (((*code.borrow()).wrapping_sub(1_u32))
                            .wrapping_sub((*bit_count.borrow())))
                    }),
                );
                (*histogram.borrow())
                    .offset((*i.borrow()) as isize)
                    .write(__rhs);
            }
            let rhs_0 = (*total_count.borrow())
                .wrapping_add(((*histogram.borrow()).offset((*i.borrow()) as isize).read()));
            (*total_count.borrow_mut()) = rhs_0;
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*total_count.borrow()) >= (*space.borrow())) {
            return false;
        }
        let __rhs = (*space.borrow()).wrapping_sub((*total_count.borrow()));
        (*histogram.borrow())
            .offset((*omit_pos.borrow()) as isize)
            .write(__rhs);
    }
    return ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    });
}
#[derive(Default)]
pub struct brunsli_HuffmanDecodingData {
    pub table_: Value<Vec<brunsli_HuffmanCode>>,
}
impl Clone for brunsli_HuffmanDecodingData {
    fn clone(&self) -> Self {
        let mut this = Self {
            table_: Rc::new(RefCell::new((*self.table_.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for brunsli_HuffmanDecodingData {}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
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
    num_symbols: u64,
    code_lengths: Ptr<u8>,
    br: Ptr<brunsli_BrunsliBitReader>,
) -> bool {
    let code_length_code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_length_code_lengths));
    let num_symbols: Value<u64> = Rc::new(RefCell::new(num_symbols));
    let code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_lengths));
    let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
    let symbol: Value<u64> = Rc::new(RefCell::new(0_u64));
    let prev_code_len: Value<u8> = Rc::new(RefCell::new(
        (*kDefaultCodeLength_215.with(Value::clone).borrow()),
    ));
    let repeat: Value<u64> = Rc::new(RefCell::new(0_u64));
    let repeat_code_len: Value<u8> = Rc::new(RefCell::new(0_u8));
    let kFullSpace: Value<i32> = Rc::new(RefCell::new((1 << 15)));
    let space: Value<i32> = Rc::new(RefCell::new((*kFullSpace.borrow())));
    let table: Value<Box<[brunsli_HuffmanCode]>> = Rc::new(RefCell::new(
        (0..32)
            .map(|_| <brunsli_HuffmanCode>::default())
            .collect::<Box<[brunsli_HuffmanCode]>>(),
    ));
    let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16,
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
    ])));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kCodeLengthCodes_213.with(Value::clone).borrow())) {
        (*counts.borrow_mut())[((*code_length_code_lengths.borrow())
            .offset((*i.borrow()) as isize)
            .read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    if !(({
        let _root_table: Ptr<brunsli_HuffmanCode> =
            (table.as_pointer() as Ptr<brunsli_HuffmanCode>);
        let _root_bits: u64 = 5_u64;
        let _code_lengths: Ptr<u8> = (*code_length_code_lengths.borrow()).clone();
        let _code_lengths_size: u64 = ((*kCodeLengthCodes_213.with(Value::clone).borrow()) as u64);
        let _count: Ptr<u16> = ((counts.as_pointer() as Ptr<u16>).offset(0 as isize));
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
    'loop_: while ((*symbol.borrow()) < (*num_symbols.borrow())) && ((*space.borrow()) > 0) {
        let p: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
            (table.as_pointer() as Ptr<brunsli_HuffmanCode>),
        ));
        let code_len: Value<u8> = <Value<u8>>::default();
        (*p.borrow_mut()) += ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 5_u32;
            BrunsliBitReaderGet_124(_br, _n_bits)
        });
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = ((*(*(*p.borrow()).upgrade().deref()).bits.borrow()) as u32);
            BrunsliBitReaderDrop_125(_br, _n_bits)
        });
        (*code_len.borrow_mut()) = ((*(*(*p.borrow()).upgrade().deref()).value.borrow()) as u8);
        if (((*code_len.borrow()) as i32)
            < ((*kCodeLengthRepeatCode_216.with(Value::clone).borrow()) as i32))
        {
            (*repeat.borrow_mut()) = 0_u64;
            let __rhs = (*code_len.borrow());
            (*code_lengths.borrow())
                .offset(((*symbol.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
            if (((*code_len.borrow()) as i32) != 0) {
                (*prev_code_len.borrow_mut()) = (*code_len.borrow());
                (*space.borrow_mut()) -= ((*kFullSpace.borrow()) >> ((*code_len.borrow()) as i32));
            }
        } else {
            let extra_bits: Value<u32> =
                Rc::new(RefCell::new(((((*code_len.borrow()) as i32) - 14) as u32)));
            let old_repeat: Value<u64> = <Value<u64>>::default();
            let repeat_delta: Value<u64> = <Value<u64>>::default();
            let new_len: Value<u8> = Rc::new(RefCell::new(0_u8));
            if (((*code_len.borrow()) as i32)
                == ((*kCodeLengthRepeatCode_216.with(Value::clone).borrow()) as i32))
            {
                (*new_len.borrow_mut()) = (*prev_code_len.borrow());
            }
            if (((*repeat_code_len.borrow()) as i32) != ((*new_len.borrow()) as i32)) {
                (*repeat.borrow_mut()) = 0_u64;
                (*repeat_code_len.borrow_mut()) = (*new_len.borrow());
            }
            (*old_repeat.borrow_mut()) = (*repeat.borrow());
            if ((*repeat.borrow()) > 0_u64) {
                let rhs_0 = (*repeat.borrow()).wrapping_sub(2_u64);
                (*repeat.borrow_mut()) = rhs_0;
                (*repeat.borrow_mut()) <<= (*extra_bits.borrow());
            }
            let rhs_0 = (*repeat.borrow()).wrapping_add(
                ((({
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    let _n_bits: u32 = (*extra_bits.borrow());
                    BrunsliBitReaderRead_126(_br, _n_bits)
                })
                .wrapping_add(3_u32 as u32)) as u64),
            );
            (*repeat.borrow_mut()) = rhs_0;
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
                (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>)
                    .to_any()
                    .clone()
            };
            let rhs_0 = (*symbol.borrow()).wrapping_add((*repeat_delta.borrow()));
            (*symbol.borrow_mut()) = rhs_0;
            if (((*repeat_code_len.borrow()) as i32) != 0) {
                (*space.borrow_mut()) -= ((((*repeat_delta.borrow())
                    .wrapping_mul(((*kFullSpace.borrow()) as u64)))
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
        (((*code_lengths.borrow()).offset((*symbol.borrow()) as isize)) as Ptr<u8>)
            .to_any()
            .clone()
    };
    return ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    });
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
                let _n: u32 = ((*alphabet_size.borrow()) as u32).wrapping_sub(1_u32 as u32);
                Log2FloorNonZero_74(_n)
            }) + 1)
        } else {
            0
        } as u32),
    ));
    let num_symbols: Value<u64> = Rc::new(RefCell::new(
        ((({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = 2_u32;
            BrunsliBitReaderRead_126(_br, _n_bits)
        })
        .wrapping_add(1_u32)) as u64),
    ));
    let symbols: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16,
        <u16>::default(),
        <u16>::default(),
        <u16>::default(),
    ])));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
        let symbol: Value<u16> = Rc::new(RefCell::new(
            (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*max_bits.borrow());
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u16),
        ));
        if (((*symbol.borrow()) as i32) >= ((*alphabet_size.borrow()) as i32)) {
            return false;
        }
        (*symbols.borrow_mut())[(*i.borrow()) as usize] = (*symbol.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow()).wrapping_sub(1_u64)) {
        let j: Value<u64> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_u64)));
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
    if ((*num_symbols.borrow()) == 4_u64) {
        let rhs_0 = (*num_symbols.borrow()).wrapping_add(
            (({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 1_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }) as u64),
        );
        (*num_symbols.borrow_mut()) = rhs_0;
    }
    let swap_symbols: Value<_> = Rc::new(RefCell::new(
        (|i: u64, j: u64| {
            let i: Value<u64> = Rc::new(RefCell::new(i));
            let j: Value<u64> = Rc::new(RefCell::new(j));
            let t: Value<u16> = Rc::new(RefCell::new((*symbols.borrow())[(*j.borrow()) as usize]));
            let __rhs = (*symbols.borrow())[(*i.borrow()) as usize];
            (*symbols.borrow_mut())[(*j.borrow()) as usize] = __rhs;
            (*symbols.borrow_mut())[(*i.borrow()) as usize] = (*t.borrow());
        }),
    ));
    let table_size: Value<u64> = Rc::new(RefCell::new(1_u64));
    'switch: {
        let __match_cond = (*num_symbols.borrow());
        match __match_cond {
            v if v == 1_u64 => {
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(0_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((0) as isize).write(__rhs);
                break 'switch;
            }
            v if v == 2_u64 => {
                if (((*symbols.borrow())[(0) as usize] as i32)
                    > ((*symbols.borrow())[(1) as usize] as i32))
                {
                    ({
                        let _i: u64 = 0_u64;
                        let _j: u64 = 1_u64;
                        (*swap_symbols.borrow_mut())(_i, _j)
                    });
                }
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((0) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(1) as usize])),
                };
                (*table.borrow()).offset((1) as isize).write(__rhs);
                (*table_size.borrow_mut()) = 2_u64;
                break 'switch;
            }
            v if v == 3_u64 => {
                if (((*symbols.borrow())[(1) as usize] as i32)
                    > ((*symbols.borrow())[(2) as usize] as i32))
                {
                    ({
                        let _i: u64 = 1_u64;
                        let _j: u64 = 2_u64;
                        (*swap_symbols.borrow_mut())(_i, _j)
                    });
                }
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((0) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((2) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(1) as usize])),
                };
                (*table.borrow()).offset((1) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(2) as usize])),
                };
                (*table.borrow()).offset((3) as isize).write(__rhs);
                (*table_size.borrow_mut()) = 4_u64;
                break 'switch;
            }
            v if v == 4_u64 => {
                let i: Value<u64> = Rc::new(RefCell::new(0_u64));
                'loop_: while ((*i.borrow()) < 3_u64) {
                    let j: Value<u64> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_u64)));
                    'loop_: while ((*j.borrow()) < 4_u64) {
                        if (((*symbols.borrow())[(*i.borrow()) as usize] as i32)
                            > ((*symbols.borrow())[(*j.borrow()) as usize] as i32))
                        {
                            ({
                                let _i: u64 = (*i.borrow());
                                let _j: u64 = (*j.borrow());
                                (*swap_symbols.borrow_mut())(_i, _j)
                            });
                        }
                        (*j.borrow_mut()).prefix_inc();
                    }
                    (*i.borrow_mut()).prefix_inc();
                }
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((0) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(1) as usize])),
                };
                (*table.borrow()).offset((2) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(2) as usize])),
                };
                (*table.borrow()).offset((1) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(3) as usize])),
                };
                (*table.borrow()).offset((3) as isize).write(__rhs);
                (*table_size.borrow_mut()) = 4_u64;
                break 'switch;
            }
            v if v == 5_u64 => {
                if (((*symbols.borrow())[(2) as usize] as i32)
                    > ((*symbols.borrow())[(3) as usize] as i32))
                {
                    ({
                        let _i: u64 = 2_u64;
                        let _j: u64 = 3_u64;
                        (*swap_symbols.borrow_mut())(_i, _j)
                    });
                }
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((0) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(1) as usize])),
                };
                (*table.borrow()).offset((1) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((2) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(3_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(2) as usize])),
                };
                (*table.borrow()).offset((3) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((4) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(1) as usize])),
                };
                (*table.borrow()).offset((5) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(1_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(0) as usize])),
                };
                (*table.borrow()).offset((6) as isize).write(__rhs);
                let __rhs = brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(3_u8)),
                    value: Rc::new(RefCell::new((*symbols.borrow())[(3) as usize])),
                };
                (*table.borrow()).offset((7) as isize).write(__rhs);
                (*table_size.borrow_mut()) = 8_u64;
                break 'switch;
            }
            _ => {
                return false;
            }
        }
    };
    let goal_size: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << (*kHuffmanTableBits_21.with(Value::clone).borrow())),
    ));
    'loop_: while ((*table_size.borrow()) != ((*goal_size.borrow()) as u64)) {
        {
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &(((*table.borrow()).offset((0) as isize)) as Ptr<brunsli_HuffmanCode>)
                        .to_any(),
                    (*table_size.borrow())
                        .wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                        as usize,
                );
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .clone()
        };
        (*table_size.borrow_mut()) <<= 1;
    }
    return ({
        let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
        BrunsliBitReaderIsHealthy_132(_br)
    });
}
impl brunsli_HuffmanDecodingData {
    pub fn ReadFromBitStream(
        &self,
        alphabet_size: u64,
        br: Ptr<brunsli_BrunsliBitReader>,
        arena: Option<Ptr<brunsli_Arena>>,
    ) -> bool {
        let alphabet_size: Value<u64> = Rc::new(RefCell::new(alphabet_size));
        let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
        let arena: Value<Ptr<brunsli_Arena>> =
            Rc::new(RefCell::new(arena.unwrap_or(Ptr::<brunsli_Arena>::null())));
        let local_arena: Value<brunsli_Arena> = Rc::new(RefCell::new(<brunsli_Arena>::default()));
        if (*arena.borrow()).is_null() {
            (*arena.borrow_mut()) = (local_arena.as_pointer());
        }
        if ((*alphabet_size.borrow())
            > ((1 << (*kMaxHuffmanBits_22.with(Value::clone).borrow())) as u64))
        {
            return false;
        }
        let code_lengths: Value<Vec<u8>> =
            Rc::new(RefCell::new(vec![0_u8; (*alphabet_size.borrow()) as usize]));
        let simple_code_or_skip: Value<u32> = Rc::new(RefCell::new(
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 2_u32;
                BrunsliBitReaderRead_126(_br, _n_bits)
            }),
        ));
        if ((*simple_code_or_skip.borrow()) == 1_u32) {
            {
                let __a0 = ((1_u32 << (*kHuffmanTableBits_21.with(Value::clone).borrow())) as u64)
                    as usize;
                (*self.table_.borrow_mut()).resize_with(__a0, || <brunsli_HuffmanCode>::default())
            };
            return ({
                let _alphabet_size: u16 = ((*alphabet_size.borrow()) as u16);
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _table: Ptr<brunsli_HuffmanCode> =
                    (self.table_.as_pointer() as Ptr<brunsli_HuffmanCode>);
                ReadSimpleCode_219(_alphabet_size, _br, _table)
            });
        }
        let code_length_code_lengths: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            0_u8,
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
            <u8>::default(),
        ])));
        let space: Value<i32> = Rc::new(RefCell::new(32));
        let num_codes: Value<i32> = Rc::new(RefCell::new(0));
        thread_local!(
            static huff_220: Value<Box<[brunsli_HuffmanCode]>> = Rc::new(RefCell::new(Box::new([
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(0_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(4_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(3_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(3_u8)),
                    value: Rc::new(RefCell::new(2_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(0_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(4_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(3_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(4_u8)),
                    value: Rc::new(RefCell::new(1_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(0_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(4_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(3_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(3_u8)),
                    value: Rc::new(RefCell::new(2_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(0_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(4_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(2_u8)),
                    value: Rc::new(RefCell::new(3_u16)),
                },
                brunsli_HuffmanCode {
                    bits: Rc::new(RefCell::new(4_u8)),
                    value: Rc::new(RefCell::new(5_u16)),
                },
            ])));
        );
        let i: Value<u64> = Rc::new(RefCell::new(((*simple_code_or_skip.borrow()) as u64)));
        'loop_: while ((*i.borrow()) < ((*kCodeLengthCodes_213.with(Value::clone).borrow()) as u64))
            && ((*space.borrow()) > 0)
        {
            let code_len_idx: Value<i32> = Rc::new(RefCell::new(
                ((*kCodeLengthCodeOrder_214.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    as i32),
            ));
            let p: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
                (huff_220.with(Value::clone).as_pointer() as Ptr<brunsli_HuffmanCode>),
            ));
            let v: Value<u8> = <Value<u8>>::default();
            (*p.borrow_mut()) += ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = 4_u32;
                BrunsliBitReaderGet_124(_br, _n_bits)
            });
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = ((*(*(*p.borrow()).upgrade().deref()).bits.borrow()) as u32);
                BrunsliBitReaderDrop_125(_br, _n_bits)
            });
            (*v.borrow_mut()) = ((*(*(*p.borrow()).upgrade().deref()).value.borrow()) as u8);
            (*code_length_code_lengths.borrow_mut())[(*code_len_idx.borrow()) as usize] =
                (*v.borrow());
            if (((*v.borrow()) as i32) != 0) {
                let rhs_0 = (((*space.borrow()) as u32)
                    .wrapping_sub((32_u32 >> ((*v.borrow()) as i32))))
                    as i32;
                (*space.borrow_mut()) = rhs_0;
                (*num_codes.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let ok: Value<bool> = Rc::new(RefCell::new(
            (((*num_codes.borrow()) == 1) || ((*space.borrow()) == 0))
                && ({
                    let _code_length_code_lengths: Ptr<u8> =
                        (code_length_code_lengths.as_pointer() as Ptr<u8>);
                    let _num_symbols: u64 = (*alphabet_size.borrow());
                    let _code_lengths: Ptr<u8> =
                        ((code_lengths.as_pointer() as Ptr<u8>).offset(0_u64 as isize));
                    let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                    ReadHuffmanCodeLengths_217(
                        _code_length_code_lengths,
                        _num_symbols,
                        _code_lengths,
                        _br,
                    )
                }),
        ));
        if (!(*ok.borrow()))
            || (!({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                BrunsliBitReaderIsHealthy_132(_br)
            }))
        {
            return false;
        }
        let counts: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
            0_u16,
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
        ])));
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*alphabet_size.borrow())) {
            (*counts.borrow_mut())[((code_lengths.as_pointer() as Ptr<u8>)
                .offset((*i.borrow()) as isize)
                .read()) as usize]
                .prefix_inc();
            (*i.borrow_mut()).prefix_inc();
        }
        ({
            let _limit: u64 = (*alphabet_size.borrow()).wrapping_add(376_u64);
            (*(*arena.borrow()).upgrade().deref()).reserve(_limit)
        });
        let table_size: Value<u32> = Rc::new(RefCell::new(
            ({
                let _root_table: Ptr<brunsli_HuffmanCode> =
                    ({ (*(*arena.borrow()).upgrade().deref()).data() });
                let _root_bits: u64 = ((*kHuffmanTableBits_21.with(Value::clone).borrow()) as u64);
                let _code_lengths: Ptr<u8> =
                    ((code_lengths.as_pointer() as Ptr<u8>).offset(0_u64 as isize));
                let _code_lengths_size: u64 = (*alphabet_size.borrow());
                let _count: Ptr<u16> = ((counts.as_pointer() as Ptr<u16>).offset(0 as isize));
                BuildHuffmanTable_218(
                    _root_table,
                    _root_bits,
                    _code_lengths,
                    _code_lengths_size,
                    _count,
                )
            }),
        ));
        (self.table_.as_pointer() as Ptr<Vec<brunsli_HuffmanCode>>).write({
            let __count = ({ (*(*arena.borrow()).upgrade().deref()).data() })
                .offset((*table_size.borrow()) as isize)
                .get_offset()
                - ({ (*(*arena.borrow()).upgrade().deref()).data() }).get_offset();
            PtrValueIter::new(
                &({ (*(*arena.borrow()).upgrade().deref()).data() }),
                __count,
            )
            .map(|item| brunsli_HuffmanCode::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
        });
        return ((*table_size.borrow()) > 0_u32);
    }
}
impl brunsli_HuffmanDecodingData {
    pub fn ReadSymbol(&self, br: Ptr<brunsli_BrunsliBitReader>) -> u16 {
        let br: Value<Ptr<brunsli_BrunsliBitReader>> = Rc::new(RefCell::new(br));
        let n_bits: Value<u32> = <Value<u32>>::default();
        let table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(
            (self.table_.as_pointer() as Ptr<brunsli_HuffmanCode>),
        ));
        (*table.borrow_mut()) += ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = (*kHuffmanTableBits_21.with(Value::clone).borrow());
            BrunsliBitReaderGet_124(_br, _n_bits)
        });
        (*n_bits.borrow_mut()) = ((*(*(*table.borrow()).upgrade().deref()).bits.borrow()) as u32);
        if ((*n_bits.borrow()) > (*kHuffmanTableBits_21.with(Value::clone).borrow())) {
            ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*kHuffmanTableBits_21.with(Value::clone).borrow());
                BrunsliBitReaderDrop_125(_br, _n_bits)
            });
            let rhs_0 = (*n_bits.borrow())
                .wrapping_sub((*kHuffmanTableBits_21.with(Value::clone).borrow()));
            (*n_bits.borrow_mut()) = rhs_0;
            let __rhs = ((*(*(*table.borrow()).upgrade().deref()).value.borrow()) as i32);
            (*table.borrow_mut()) += __rhs;
            (*table.borrow_mut()) += ({
                let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
                let _n_bits: u32 = (*n_bits.borrow());
                BrunsliBitReaderGet_124(_br, _n_bits)
            });
        }
        ({
            let _br: Ptr<brunsli_BrunsliBitReader> = (*br.borrow()).clone();
            let _n_bits: u32 = ((*(*(*table.borrow()).upgrade().deref()).bits.borrow()) as u32);
            BrunsliBitReaderDrop_125(_br, _n_bits)
        });
        return (*(*(*table.borrow()).upgrade().deref()).value.borrow());
    }
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
pub fn GetNextKey_221(key: i32, len: u64) -> i32 {
    let key: Value<i32> = Rc::new(RefCell::new(key));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let step: Value<i32> = Rc::new(RefCell::new(
        ((1_u32 << ((*len.borrow()).wrapping_sub(1_u64))) as i32),
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
    'loop_: loop {
        (*end.borrow_mut()) -= (*step.borrow());
        let __rhs = (*code.borrow()).clone();
        (*table.borrow())
            .offset((*end.borrow()) as isize)
            .write(__rhs);
        if !((*end.borrow()) > 0) {
            break;
        }
    }
}
pub fn NextTableBitSize_223(count: Ptr<u16>, len: u64, root_bits: u64) -> u64 {
    let count: Value<Ptr<u16>> = Rc::new(RefCell::new(count));
    let len: Value<u64> = Rc::new(RefCell::new(len));
    let root_bits: Value<u64> = Rc::new(RefCell::new(root_bits));
    let left: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << ((*len.borrow()).wrapping_sub((*root_bits.borrow())))),
    ));
    'loop_: while ((*len.borrow()) < (*kMaxHuffmanBits_22.with(Value::clone).borrow())) {
        if {
            let _lhs = (*left.borrow());
            _lhs <= (((*count.borrow()).offset((*len.borrow()) as isize).read()) as u64)
        } {
            break;
        }
        let rhs_0 = (*left.borrow())
            .wrapping_sub((((*count.borrow()).offset((*len.borrow()) as isize).read()) as u64));
        (*left.borrow_mut()) = rhs_0;
        (*len.borrow_mut()).prefix_inc();
        (*left.borrow_mut()) <<= 1;
    }
    return (*len.borrow()).wrapping_sub((*root_bits.borrow()));
}
pub fn BuildHuffmanTable_218(
    root_table: Ptr<brunsli_HuffmanCode>,
    root_bits: u64,
    code_lengths: Ptr<u8>,
    code_lengths_size: u64,
    count: Ptr<u16>,
) -> u32 {
    let root_table: Value<Ptr<brunsli_HuffmanCode>> = Rc::new(RefCell::new(root_table));
    let root_bits: Value<u64> = Rc::new(RefCell::new(root_bits));
    let code_lengths: Value<Ptr<u8>> = Rc::new(RefCell::new(code_lengths));
    let code_lengths_size: Value<u64> = Rc::new(RefCell::new(code_lengths_size));
    let count: Value<Ptr<u16>> = Rc::new(RefCell::new(count));
    let code: Value<brunsli_HuffmanCode> = Rc::new(RefCell::new(<brunsli_HuffmanCode>::default()));
    let table: Value<Ptr<brunsli_HuffmanCode>> =
        Rc::new(RefCell::new(Ptr::<brunsli_HuffmanCode>::null()));
    let len: Value<u64> = <Value<u64>>::default();
    let symbol: Value<u64> = <Value<u64>>::default();
    let key: Value<i32> = <Value<i32>>::default();
    let step: Value<i32> = <Value<i32>>::default();
    let low: Value<i32> = <Value<i32>>::default();
    let mask: Value<i32> = <Value<i32>>::default();
    let table_bits: Value<u64> = <Value<u64>>::default();
    let table_size: Value<i32> = <Value<i32>>::default();
    let total_size: Value<i32> = <Value<i32>>::default();
    let offset: Value<Box<[u16]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u16>::default()).collect::<Box<[u16]>>(),
    ));
    let max_length: Value<u64> = Rc::new(RefCell::new(1_u64));
    if ((*code_lengths_size.borrow())
        > ((1_u32 << (*kMaxHuffmanBits_22.with(Value::clone).borrow())) as u64))
    {
        return 0_u32;
    }
    let sorted_storage: Value<Vec<u16>> = Rc::new(RefCell::new(
        (0..(*code_lengths_size.borrow()) as usize)
            .map(|_| <u16>::default())
            .collect::<Vec<_>>(),
    ));
    let sorted: Value<Ptr<u16>> = Rc::new(RefCell::new((sorted_storage.as_pointer() as Ptr<u16>)));
    let sum: Value<u16> = Rc::new(RefCell::new(0_u16));
    (*len.borrow_mut()) = 1_u64;
    'loop_: while ((*len.borrow()) <= (*kMaxHuffmanBits_22.with(Value::clone).borrow())) {
        (*offset.borrow_mut())[(*len.borrow()) as usize] = (*sum.borrow());
        if (((*count.borrow()).offset((*len.borrow()) as isize).read()) != 0) {
            let __rhs = (({
                let _lhs = ((*sum.borrow()) as i32);
                _lhs + (((*count.borrow()).offset((*len.borrow()) as isize).read()) as i32)
            }) as u16);
            (*sum.borrow_mut()) = __rhs;
            (*max_length.borrow_mut()) = (*len.borrow());
        }
        (*len.borrow_mut()).postfix_inc();
    }
    (*symbol.borrow_mut()) = 0_u64;
    'loop_: while ((*symbol.borrow()) < (*code_lengths_size.borrow())) {
        if ((((*code_lengths.borrow())
            .offset((*symbol.borrow()) as isize)
            .read()) as i32)
            != 0)
        {
            let __rhs = ((*symbol.borrow()) as u16);
            (*sorted.borrow())
                .offset(
                    ((*offset.borrow_mut())[((*code_lengths.borrow())
                        .offset((*symbol.borrow()) as isize)
                        .read()) as usize]
                        .postfix_inc()) as isize,
                )
                .write(__rhs);
        }
        (*symbol.borrow_mut()).postfix_inc();
    }
    (*table.borrow_mut()) = (*root_table.borrow()).clone();
    (*table_bits.borrow_mut()) = (*root_bits.borrow());
    (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
    (*total_size.borrow_mut()) = (*table_size.borrow());
    if (((*offset.borrow())[(*kMaxHuffmanBits_22.with(Value::clone).borrow()) as usize] as i32)
        == 1)
    {
        (*(*code.borrow()).bits.borrow_mut()) = 0_u8;
        let __rhs = ((*sorted.borrow()).offset((0) as isize).read());
        (*(*code.borrow()).value.borrow_mut()) = __rhs;
        (*key.borrow_mut()) = 0;
        'loop_: while ((*key.borrow()) < (*total_size.borrow())) {
            let __rhs = (*code.borrow()).clone();
            (*table.borrow())
                .offset((*key.borrow()) as isize)
                .write(__rhs);
            (*key.borrow_mut()).prefix_inc();
        }
        return ((*total_size.borrow()) as u32);
    }
    if ((*table_bits.borrow()) > (*max_length.borrow())) {
        (*table_bits.borrow_mut()) = (*max_length.borrow());
        (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
    }
    (*key.borrow_mut()) = 0;
    (*symbol.borrow_mut()) = 0_u64;
    (*(*code.borrow()).bits.borrow_mut()) = 1_u8;
    (*step.borrow_mut()) = 2;
    'loop_: loop {
        'loop_: while ((((*count.borrow())
            .offset((*(*code.borrow()).bits.borrow()) as isize)
            .read()) as i32)
            != 0)
        {
            let __rhs = ((*sorted.borrow())
                .offset(((*symbol.borrow_mut()).postfix_inc()) as isize)
                .read());
            (*(*code.borrow()).value.borrow_mut()) = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> =
                    ((*table.borrow()).offset((*key.borrow()) as isize));
                let _step: i32 = (*step.borrow());
                let _end: i32 = (*table_size.borrow());
                let _code: brunsli_HuffmanCode = (*code.borrow()).clone();
                ReplicateValue_222(_table, _step, _end, _code)
            });
            let __rhs = ({
                let _key: i32 = (*key.borrow());
                let _len: u64 = ((*(*code.borrow()).bits.borrow()) as u64);
                GetNextKey_221(_key, _len)
            });
            (*key.borrow_mut()) = __rhs;
            (*count.borrow())
                .offset((*(*code.borrow()).bits.borrow()) as isize)
                .with_mut(|__v| __v.prefix_dec());
        }
        (*step.borrow_mut()) <<= 1;
        if !(((*(*code.borrow()).bits.borrow_mut()).prefix_inc() as u64) <= (*table_bits.borrow()))
        {
            break;
        }
    }
    'loop_: while ((*total_size.borrow()) != (*table_size.borrow())) {
        {
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .memcpy(
                    &(((*table.borrow()).offset((0) as isize)) as Ptr<brunsli_HuffmanCode>)
                        .to_any(),
                    ((*table_size.borrow()) as u64)
                        .wrapping_mul(::std::mem::size_of::<brunsli_HuffmanCode>() as u64 as u64)
                        as usize,
                );
            (((*table.borrow()).offset((*table_size.borrow()) as isize))
                as Ptr<brunsli_HuffmanCode>)
                .to_any()
                .clone()
        };
        (*table_size.borrow_mut()) <<= 1;
    }
    (*mask.borrow_mut()) = ((*total_size.borrow()) - 1);
    (*low.borrow_mut()) = -1_i32;
    (*len.borrow_mut()) = (*root_bits.borrow()).wrapping_add(1_u64);
    (*step.borrow_mut()) = 2;
    'loop_: while ((*len.borrow()) <= (*max_length.borrow())) {
        'loop_: while ((((*count.borrow()).offset((*len.borrow()) as isize).read()) as i32) != 0) {
            if (((*key.borrow()) & (*mask.borrow())) != (*low.borrow())) {
                (*table.borrow_mut()) += (*table_size.borrow());
                (*table_bits.borrow_mut()) = ({
                    let _count: Ptr<u16> = (*count.borrow()).clone();
                    let _len: u64 = (*len.borrow());
                    let _root_bits: u64 = (*root_bits.borrow());
                    NextTableBitSize_223(_count, _len, _root_bits)
                });
                (*table_size.borrow_mut()) = ((1_u32 << (*table_bits.borrow())) as i32);
                (*total_size.borrow_mut()) += (*table_size.borrow());
                (*low.borrow_mut()) = ((*key.borrow()) & (*mask.borrow()));
                (*(*(*root_table.borrow())
                    .offset((*low.borrow()) as isize)
                    .upgrade()
                    .deref())
                .bits
                .borrow_mut()) =
                    (((*table_bits.borrow()).wrapping_add((*root_bits.borrow()))) as u8);
                let __rhs = (({
                    let _lhs =
                        (((*table.borrow()).clone() - (*root_table.borrow()).clone()) as i64);
                    _lhs - ((*low.borrow()) as i64)
                }) as u16);
                (*(*(*root_table.borrow())
                    .offset((*low.borrow()) as isize)
                    .upgrade()
                    .deref())
                .value
                .borrow_mut()) = __rhs;
            }
            (*(*code.borrow()).bits.borrow_mut()) =
                (((*len.borrow()).wrapping_sub((*root_bits.borrow()))) as u8);
            let __rhs = ((*sorted.borrow())
                .offset(((*symbol.borrow_mut()).postfix_inc()) as isize)
                .read());
            (*(*code.borrow()).value.borrow_mut()) = __rhs;
            ({
                let _table: Ptr<brunsli_HuffmanCode> =
                    ((*table.borrow()).offset(((*key.borrow()) >> (*root_bits.borrow())) as isize));
                let _step: i32 = (*step.borrow());
                let _end: i32 = (*table_size.borrow());
                let _code: brunsli_HuffmanCode = (*code.borrow()).clone();
                ReplicateValue_222(_table, _step, _end, _code)
            });
            let __rhs = ({
                let _key: i32 = (*key.borrow());
                let _len: u64 = (*len.borrow());
                GetNextKey_221(_key, _len)
            });
            (*key.borrow_mut()) = __rhs;
            (*count.borrow())
                .offset((*len.borrow()) as isize)
                .with_mut(|__v| __v.prefix_dec());
        }
        (*len.borrow_mut()).prefix_inc();
        (*step.borrow_mut()) <<= 1;
    }
    return ((*total_size.borrow()) as u32);
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
impl brunsli_internal_dec_OutputChunk {
    pub fn brunsli_internal_dec_OutputChunk4(bytes: Ptr<Vec<u8>>) -> Self {
        let mut this = Self {
            next: Rc::new(RefCell::new(Ptr::<u8>::null())),
            len: Rc::new(RefCell::new((*bytes.upgrade().deref()).len() as u64)),
            buffer: Rc::new(RefCell::new(None)),
        };
        let src: Value<AnyPtr> = Rc::new(RefCell::new(
            ((bytes.to_strong().as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
        ));
        (*this.next.borrow_mut()) = ((*src.borrow()).reinterpret_cast::<u8>()).clone();
        this
    }
}
thread_local!(
    pub static kJpegPrecision_224: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kBitWriterChunkSize_225: Value<u64> = Rc::new(RefCell::new(16384_u64));
);
pub fn DivCeil_226(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((((*a.borrow()) + (*b.borrow())) - 1) / (*b.borrow()));
}
pub fn HasZeroByte_227(x: u64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return ((((*x.borrow()).wrapping_sub(72340172838076673_u64 as u64)) & !(*x.borrow()))
        & 9259542123273814144_u64);
}
pub fn BitWriterInit_228(
    bw: Ptr<brunsli_internal_dec_BitWriter>,
    output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let output_queue: Value<Ptr<Vec<brunsli_internal_dec_OutputChunk>>> =
        Rc::new(RefCell::new(output_queue));
    (*(*(*bw.borrow()).upgrade().deref()).output.borrow_mut()) = (*output_queue.borrow()).clone();
    (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()) =
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (*kBitWriterChunkSize_225.with(Value::clone).borrow()),
        ));
    (*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()) = 0_u64;
    (*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow_mut()) = 0_u64;
    (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow_mut()) = 64;
    (*(*(*bw.borrow()).upgrade().deref()).healthy.borrow_mut()) = true;
    let __rhs = ((*(*(*(*bw.borrow()).upgrade().deref()).chunk.borrow())
        .buffer
        .borrow())
    .as_pointer()
    .to_strong()
    .as_pointer() as Ptr<u8>);
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow_mut()) = __rhs;
}
pub fn SwapBuffer_229(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let __rhs = (*(*(*bw.borrow()).upgrade().deref()).pos.borrow());
    (*(*(*(*bw.borrow()).upgrade().deref()).chunk.borrow())
        .len
        .borrow_mut()) = __rhs;
    (*(*(*bw.borrow()).upgrade().deref()).output.borrow())
        .to_strong()
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(std::mem::take(
                &mut (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()),
            ))
        });
    (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()) =
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
            (*kBitWriterChunkSize_225.with(Value::clone).borrow()),
        ));
    let __rhs = ((*(*(*(*bw.borrow()).upgrade().deref()).chunk.borrow())
        .buffer
        .borrow())
    .as_pointer()
    .to_strong()
    .as_pointer() as Ptr<u8>);
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow_mut()) = __rhs;
    (*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()) = 0_u64;
}
pub fn Reserve_230(bw: Ptr<brunsli_internal_dec_BitWriter>, n_bytes: u64) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let n_bytes: Value<u64> = Rc::new(RefCell::new(n_bytes));
    if ((({
        let _lhs =
            ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add((*n_bytes.borrow())));
        _lhs > (*kBitWriterChunkSize_225.with(Value::clone).borrow())
    }) as i64)
        != 0)
    {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            SwapBuffer_229(_bw)
        });
    }
}
pub fn EmitByte_231(bw: Ptr<brunsli_internal_dec_BitWriter>, byte: i32) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let byte: Value<i32> = Rc::new(RefCell::new(byte));
    let __rhs = ((*byte.borrow()) as u8);
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
        .offset(((*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    if ((*byte.borrow()) == 255) {
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()).postfix_inc()) as isize,
            )
            .write(0_u8);
    }
}
pub fn DischargeBitBuffer_232(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _n_bytes: u64 = 12_u64;
        Reserve_230(_bw, _n_bytes)
    });
    if (({
        let _x: u64 = (!(*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) | 65535_u64);
        HasZeroByte_227(_x)
    }) != 0)
    {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 56)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 48)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 40)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 32)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 24)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 16)
                & 255_u64) as i32);
            EmitByte_231(_bw, _byte)
        });
    } else {
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 56) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()) as isize)
            .write(__rhs);
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 48) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(1_u64)) as isize,
            )
            .write(__rhs);
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 40) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(2_u64)) as isize,
            )
            .write(__rhs);
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 32) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(3_u64)) as isize,
            )
            .write(__rhs);
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 24) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(4_u64)) as isize,
            )
            .write(__rhs);
        let __rhs =
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 16) & 255_u64) as u8);
        (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
            .offset(
                ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(5_u64)) as isize,
            )
            .write(__rhs);
        let rhs_0 = (*(*(*bw.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(6_u64);
        (*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
    }
    (*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow_mut()) <<= 48;
    (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow_mut()) += 48;
}
pub fn WriteBits_233(bw: Ptr<brunsli_internal_dec_BitWriter>, nbits: i32, bits: u64) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
    let bits: Value<u64> = Rc::new(RefCell::new(bits));
    if ((*nbits.borrow()) == 0) {
        (*(*(*bw.borrow()).upgrade().deref()).healthy.borrow_mut()) = false;
        return;
    }
    (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow_mut()) -= (*nbits.borrow());
    let __rhs = ({
        let _lhs = (*bits.borrow());
        _lhs << (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow())
    });
    (*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow_mut()) |= __rhs;
    if ((*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow()) <= 16) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            DischargeBitBuffer_232(_bw)
        });
    }
}
pub fn EmitMarker_234(bw: Ptr<brunsli_internal_dec_BitWriter>, marker: i32) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    let marker: Value<i32> = Rc::new(RefCell::new(marker));
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _n_bytes: u64 = 2_u64;
        Reserve_230(_bw, _n_bytes)
    });
    if !((*marker.borrow()) != 255) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("jpeg_data_writer.cc");
            let _l: i32 = 133;
            let _fn: Ptr<u8> = Ptr::from_string_literal("EmitMarker");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
        .offset(((*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()).postfix_inc()) as isize)
        .write(255_u8);
    let __rhs = ((*marker.borrow()) as u8);
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow())
        .offset(((*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()).postfix_inc()) as isize)
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
    let n_bits: Value<u64> = Rc::new(RefCell::new(
        ((((*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow()) as u32) & 7_u32) as u64),
    ));
    let pad_pattern: Value<u8> = <Value<u8>>::default();
    if ((*pad_bits.borrow()).read()).is_null() {
        (*pad_pattern.borrow_mut()) = (((1_u32 << (*n_bits.borrow())).wrapping_sub(1_u32)) as u8);
    } else {
        (*pad_pattern.borrow_mut()) = 0_u8;
        let src: Value<Ptr<i32>> = Rc::new(RefCell::new(((*pad_bits.borrow()).read()).clone()));
        'loop_: while ((*n_bits.borrow_mut()).postfix_dec() != 0) {
            let rhs_0 = (((*pad_pattern.borrow()) as i32) << 1) as u8;
            (*pad_pattern.borrow_mut()) = rhs_0;
            if {
                let _lhs = (*src.borrow()).clone();
                _lhs >= (*pad_bits_end.borrow()).clone()
            } {
                return false;
            }
            let rhs_0 = (((*pad_pattern.borrow()) as i32)
                | (!!((((*src.borrow_mut()).postfix_inc()).read()) != 0) as i32))
                as u8;
            (*pad_pattern.borrow_mut()) = rhs_0;
        }
        let __rhs = (*src.borrow()).clone();
        (*pad_bits.borrow()).write(__rhs);
    }
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _n_bytes: u64 = 16_u64;
        Reserve_230(_bw, _n_bytes)
    });
    'loop_: while ((*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow()) <= 56) {
        let c: Value<i32> = Rc::new(RefCell::new(
            ((((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 56) & 255_u64) as i32),
        ));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = (*c.borrow());
            EmitByte_231(_bw, _byte)
        });
        (*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow_mut()) <<= 8;
        (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow_mut()) += 8;
    }
    if ((*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow()) < 64) {
        let pad_mask: Value<i32> = Rc::new(RefCell::new(
            ((255_u32 >> (64 - (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow()))) as i32),
        ));
        let c: Value<i32> = Rc::new(RefCell::new(
            (({
                let _lhs = ({
                    let _lhs = ((*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow()) >> 56);
                    _lhs & (!(*pad_mask.borrow()) as u64)
                });
                _lhs | ((*pad_pattern.borrow()) as u64)
            }) as i32),
        ));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _byte: i32 = (*c.borrow());
            EmitByte_231(_bw, _byte)
        });
    }
    (*(*(*bw.borrow()).upgrade().deref()).put_buffer.borrow_mut()) = 0_u64;
    (*(*(*bw.borrow()).upgrade().deref()).put_bits.borrow_mut()) = 64;
    return true;
}
pub fn BitWriterFinish_236(bw: Ptr<brunsli_internal_dec_BitWriter>) {
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*(*(*bw.borrow()).upgrade().deref()).pos.borrow()) == 0_u64) {
        return;
    }
    let __rhs = (*(*(*bw.borrow()).upgrade().deref()).pos.borrow());
    (*(*(*(*bw.borrow()).upgrade().deref()).chunk.borrow())
        .len
        .borrow_mut()) = __rhs;
    (*(*(*bw.borrow()).upgrade().deref()).output.borrow())
        .to_strong()
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(std::mem::take(
                &mut (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()),
            ))
        });
    (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()) =
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk1(
            Ptr::<u8>::null(),
            0_u64,
        );
    (*(*(*bw.borrow()).upgrade().deref()).data.borrow_mut()) = Ptr::<u8>::null();
    (*(*(*bw.borrow()).upgrade().deref()).pos.borrow_mut()) = 0_u64;
}
pub fn DCTCodingStateInit_237(s: Ptr<brunsli_internal_dec_DCTCodingState>) {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    (*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow_mut()) = 0;
    (*(*(*s.borrow()).upgrade().deref()).cur_ac_huff_.borrow_mut()) =
        Ptr::<brunsli_HuffmanCodeTable>::null();
    (*(*(*s.borrow()).upgrade().deref())
        .refinement_bits_
        .borrow_mut())
    .clear();
    if 64_u64 as usize
        > (*(*(*s.borrow()).upgrade().deref()).refinement_bits_.borrow()).capacity() as usize
    {
        let len_0 = (*(*(*s.borrow()).upgrade().deref()).refinement_bits_.borrow()).len();
        (*(*(*s.borrow()).upgrade().deref())
            .refinement_bits_
            .borrow_mut())
        .reserve_exact(64_u64 as usize - len_0 as usize);
    };
    (*(*(*s.borrow()).upgrade().deref())
        .refinement_bits_count_
        .borrow_mut()) = 0_u64;
}
pub fn Flush_238(
    s: Ptr<brunsli_internal_dec_DCTCodingState>,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow()) > 0) {
        let nbits: Value<i32> = Rc::new(RefCell::new(
            ({
                let _n: u32 = ((*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            }),
        ));
        let symbol: Value<i32> = Rc::new(RefCell::new(((*nbits.borrow()) << 4_u32)));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*(*(*(*(*s.borrow()).upgrade().deref()).cur_ac_huff_.borrow())
                .upgrade()
                .deref())
            .depth
            .borrow())[(*symbol.borrow()) as usize];
            let _bits: u64 = ((*(*(*(*(*s.borrow()).upgrade().deref()).cur_ac_huff_.borrow())
                .upgrade()
                .deref())
            .code
            .borrow())[(*symbol.borrow()) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        if ((*nbits.borrow()) > 0) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*nbits.borrow());
                let _bits: u64 = (({
                    let _lhs = (*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow());
                    _lhs & ((1 << (*nbits.borrow())) - 1)
                }) as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
        }
        (*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow_mut()) = 0;
    }
    let num_words: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*s.borrow()).upgrade().deref())
            .refinement_bits_count_
            .borrow())
            >> 4),
    ));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*num_words.borrow())) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = 16;
            let _bits: u64 = ((((*(*s.borrow()).upgrade().deref())
                .refinement_bits_
                .as_pointer() as Ptr<u16>)
                .offset((*i.borrow()) as isize)
                .read()) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let tail: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*s.borrow()).upgrade().deref())
            .refinement_bits_count_
            .borrow())
            & 15_u64),
    ));
    if ((*tail.borrow()) != 0) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = ((*tail.borrow()) as i32);
            let _bits: u64 = ((((*(*s.borrow()).upgrade().deref())
                .refinement_bits_
                .as_pointer() as Ptr<u16>)
                .to_last()
                .read()) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
    }
    (*(*(*s.borrow()).upgrade().deref())
        .refinement_bits_
        .borrow_mut())
    .clear();
    (*(*(*s.borrow()).upgrade().deref())
        .refinement_bits_count_
        .borrow_mut()) = 0_u64;
}
pub fn BufferEndOfBand_239(
    s: Ptr<brunsli_internal_dec_DCTCodingState>,
    ac_huff: Ptr<brunsli_HuffmanCodeTable>,
    new_bits_array: Ptr<i32>,
    new_bits_count: u64,
    bw: Ptr<brunsli_internal_dec_BitWriter>,
) -> bool {
    let s: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(s));
    let ac_huff: Value<Ptr<brunsli_HuffmanCodeTable>> = Rc::new(RefCell::new(ac_huff));
    let new_bits_array: Value<Ptr<i32>> = Rc::new(RefCell::new(new_bits_array));
    let new_bits_count: Value<u64> = Rc::new(RefCell::new(new_bits_count));
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> = Rc::new(RefCell::new(bw));
    if ((*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow()) == 0) {
        (*(*(*s.borrow()).upgrade().deref()).cur_ac_huff_.borrow_mut()) =
            (*ac_huff.borrow()).clone();
    }
    (*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow_mut()).prefix_inc();
    if ((*new_bits_count.borrow()) != 0) {
        let new_bits: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*new_bits_count.borrow())) {
            let __rhs = {
                let _lhs = ((*new_bits.borrow()) << 1);
                _lhs | (((*new_bits_array.borrow())
                    .offset((*i.borrow()) as isize)
                    .read()) as u64)
            };
            (*new_bits.borrow_mut()) = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        let tail: Value<u64> = Rc::new(RefCell::new(
            ((*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow())
                & 15_u64),
        ));
        if ((*tail.borrow()) != 0) {
            let stuff_bits_count: Value<u64> = Rc::new(RefCell::new({
                let __tmp_0: Value<u64> =
                    Rc::new(RefCell::new((16_u64).wrapping_sub((*tail.borrow()))));
                (if __tmp_0.as_pointer().read() <= new_bits_count.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    new_bits_count.as_pointer()
                }
                .read())
            }));
            let stuff_bits: Value<u16> = Rc::new(RefCell::new(
                (((*new_bits.borrow())
                    >> ((*new_bits_count.borrow()).wrapping_sub((*stuff_bits_count.borrow()))))
                    as u16),
            ));
            let rhs_0 = (((*stuff_bits.borrow()) as u32)
                & ((1_u32 << (*stuff_bits_count.borrow())).wrapping_sub(1_u32)))
                as u16;
            (*stuff_bits.borrow_mut()) = rhs_0;
            let __rhs = (({
                let _lhs = ({
                    let _lhs = ((((*(*s.borrow()).upgrade().deref())
                        .refinement_bits_
                        .as_pointer() as Ptr<u16>)
                        .to_last()
                        .read()) as i32);
                    _lhs << (*stuff_bits_count.borrow())
                });
                _lhs | ((*stuff_bits.borrow()) as i32)
            }) as u16);
            ((*(*s.borrow()).upgrade().deref())
                .refinement_bits_
                .as_pointer() as Ptr<u16>)
                .to_last()
                .write(__rhs);
            let rhs_0 = (*new_bits_count.borrow()).wrapping_sub((*stuff_bits_count.borrow()));
            (*new_bits_count.borrow_mut()) = rhs_0;
            let rhs_0 = (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow())
            .wrapping_add((*stuff_bits_count.borrow()));
            (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow_mut()) = rhs_0;
        }
        'loop_: while ((*new_bits_count.borrow()) >= 16_u64) {
            (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_
                .borrow_mut())
            .push(
                (((*new_bits.borrow()) >> ((*new_bits_count.borrow()).wrapping_sub(16_u64)))
                    as u16),
            );
            let rhs_0 = (*new_bits_count.borrow()).wrapping_sub(16_u64);
            (*new_bits_count.borrow_mut()) = rhs_0;
            let rhs_0 = (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow())
            .wrapping_add(16_u64);
            (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow_mut()) = rhs_0;
        }
        if ((*new_bits_count.borrow()) != 0) {
            (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_
                .borrow_mut())
            .push(
                (((*new_bits.borrow())
                    & (((1_u32 << (*new_bits_count.borrow())).wrapping_sub(1_u32)) as u64))
                    as u16),
            );
            let rhs_0 = (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow())
            .wrapping_add((*new_bits_count.borrow()));
            (*(*(*s.borrow()).upgrade().deref())
                .refinement_bits_count_
                .borrow_mut()) = rhs_0;
        }
    }
    if {
        let _lhs = (*(*(*s.borrow()).upgrade().deref())
            .refinement_bits_count_
            .borrow());
        _lhs > ((32767 * ((*kDCTBlockSize_3.with(Value::clone).borrow()) - 1)) as u64)
    } {
        return false;
    }
    if ((*(*(*s.borrow()).upgrade().deref()).eob_run_.borrow()) == 32767) {
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*s.borrow()).clone();
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            Flush_238(_s, _bw)
        });
    }
    return true;
}
pub fn BuildHuffmanCodeTable_240(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    table: Ptr<brunsli_HuffmanCodeTable>,
) -> bool {
    let table: Value<Ptr<brunsli_HuffmanCodeTable>> = Rc::new(RefCell::new(table));
    let huff_code: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..256).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    let huff_size: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..257).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
    ));
    let p: Value<i32> = Rc::new(RefCell::new(0));
    let l: Value<u64> = Rc::new(RefCell::new(1_u64));
    'loop_: while ((*l.borrow())
        <= ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) as u64))
    {
        let i: Value<i32> = Rc::new(RefCell::new(
            (((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                .offset((*l.borrow()) as isize)
                .read()),
        ));
        if (((*p.borrow()) + (*i.borrow()))
            > ((*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow()) + 1))
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
            (((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                .offset(((*p.borrow()) as u64) as isize)
                .read()),
        ));
        (*(*(*table.borrow()).upgrade().deref()).depth.borrow_mut())[(*i.borrow()) as usize] =
            ((*huff_size.borrow())[(*p.borrow()) as usize] as i32);
        (*(*(*table.borrow()).upgrade().deref()).code.borrow_mut())[(*i.borrow()) as usize] =
            (*huff_code.borrow())[(*p.borrow()) as usize];
        (*p.borrow_mut()).postfix_inc();
    }
    return true;
}
pub fn EncodeSOI_241(state: Ptr<brunsli_internal_dec_SerializationState>) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8, 216_u8]),
    );
    return true;
}
pub fn EncodeEOI_242(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8, 217_u8]),
    );
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
                    (*jpg.upgrade().deref()).tail_data.as_pointer(),
                ),
            )
        });
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
        (*(*(*state.borrow()).upgrade().deref())
            .is_progressive
            .borrow_mut()) = (((*marker.borrow()) as i32) == 194);
    }
    let n_comps: Value<u64> = Rc::new(RefCell::new(
        (*(*jpg.upgrade().deref()).components.borrow()).len() as u64,
    ));
    let marker_len: Value<u64> = Rc::new(RefCell::new(
        (8_u64).wrapping_add((3_u64).wrapping_mul((*n_comps.borrow()))),
    ));
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
                    (*marker_len.borrow()).wrapping_add(2_u64),
                )),
            )
        });
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*((*(*state.borrow()).upgrade().deref())
            .output_queue
            .as_pointer() as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .upgrade()
            .deref())
        .buffer
        .borrow())
        .as_pointer()
        .to_strong()
        .as_pointer() as Ptr<u8>),
    ));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(255_u8);
    let __rhs = (*marker.borrow());
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*kJpegPrecision_224.with(Value::clone).borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = (((*(*jpg.upgrade().deref()).height.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((((*(*jpg.upgrade().deref()).height.borrow()) as u32) & 255_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = (((*(*jpg.upgrade().deref()).width.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((((*(*jpg.upgrade().deref()).width.borrow()) as u32) & 255_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*n_comps.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*n_comps.borrow())) {
        let __rhs = ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .id
        .borrow()) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        let __rhs = (({
            let _lhs = ((*(*((*jpg.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .h_samp_factor
            .borrow())
                << 4_u32);
            _lhs | (*(*((*jpg.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .v_samp_factor
            .borrow())
        }) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        let quant_idx: Value<u64> = Rc::new(RefCell::new(
            ((*(*((*jpg.upgrade().deref()).components.as_pointer() as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .quant_idx
            .borrow()) as u64),
        ));
        if {
            let _lhs = (*quant_idx.borrow());
            _lhs >= (*(*jpg.upgrade().deref()).quant.borrow()).len() as u64
        } {
            return false;
        }
        let __rhs = ((*(*((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*quant_idx.borrow()) as isize)
            .upgrade()
            .deref())
        .index
        .borrow()) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
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
    let n_scans: Value<u64> = Rc::new(RefCell::new(
        (*(*scan_info.upgrade().deref()).num_components.borrow()),
    ));
    let marker_len: Value<u64> = Rc::new(RefCell::new(
        (6_u64).wrapping_add((2_u64).wrapping_mul((*n_scans.borrow()))),
    ));
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
                    (*marker_len.borrow()).wrapping_add(2_u64),
                )),
            )
        });
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*((*(*state.borrow()).upgrade().deref())
            .output_queue
            .as_pointer() as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .upgrade()
            .deref())
        .buffer
        .borrow())
        .as_pointer()
        .to_strong()
        .as_pointer() as Ptr<u8>),
    ));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(255_u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(218_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*n_scans.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*n_scans.borrow())) {
        let si: Ptr<brunsli_JPEGComponentScanInfo> =
            ((*scan_info.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset((*i.borrow()) as isize);
        if {
            let _lhs = ((*(*si.upgrade().deref()).comp_idx.borrow()) as u64);
            _lhs >= (*(*jpg.upgrade().deref()).components.borrow()).len() as u64
        } {
            return false;
        }
        let __rhs = ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(((*(*si.upgrade().deref()).comp_idx.borrow()) as u64) as isize)
            .upgrade()
            .deref())
        .id
        .borrow()) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        let __rhs = (({
            let _lhs = ((*(*si.upgrade().deref()).dc_tbl_idx.borrow()) << 4_u32);
            _lhs + (*(*si.upgrade().deref()).ac_tbl_idx.borrow())
        }) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ((*(*scan_info.upgrade().deref()).Ss.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*(*scan_info.upgrade().deref()).Se.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = (({
        let _lhs = ((*(*scan_info.upgrade().deref()).Ah.borrow()) << 4_u32);
        _lhs | (*(*scan_info.upgrade().deref()).Al.borrow())
    }) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    return true;
}
pub fn EncodeDHT_245(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let huffman_code: Ptr<Vec<brunsli_JPEGHuffmanCode>> =
        (*jpg.upgrade().deref()).huffman_code.as_pointer();
    let marker_len: Value<u64> = Rc::new(RefCell::new(2_u64));
    let i: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*state.borrow()).upgrade().deref()).dht_index.borrow()) as u64),
    ));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*huffman_code.upgrade().deref()).len() as u64
    } {
        let huff: Ptr<brunsli_JPEGHuffmanCode> = (huffman_code.to_strong().as_pointer()
            as Ptr<brunsli_JPEGHuffmanCode>)
            .offset((*i.borrow()) as isize);
        let rhs_0 = (*marker_len.borrow())
            .wrapping_add(((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) as u64));
        (*marker_len.borrow_mut()) = rhs_0;
        let j: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while {
            let _lhs = (*j.borrow());
            _lhs < (*(*huff.upgrade().deref()).counts.borrow()).len() as u64
        } {
            let rhs_0 = (*marker_len.borrow()).wrapping_add(
                ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                    .offset((*j.borrow()) as isize)
                    .read()) as u64),
            );
            (*marker_len.borrow_mut()) = rhs_0;
            (*j.borrow_mut()).prefix_inc();
        }
        if (*(*huff.upgrade().deref()).is_last.borrow()) {
            break;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
                    (*marker_len.borrow()).wrapping_add(2_u64),
                )),
            )
        });
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*((*(*state.borrow()).upgrade().deref())
            .output_queue
            .as_pointer() as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .upgrade()
            .deref())
        .buffer
        .borrow())
        .as_pointer()
        .to_strong()
        .as_pointer() as Ptr<u8>),
    ));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(255_u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(196_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((*marker_len.borrow()) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    'loop_: while true {
        let huffman_code_index: Value<u64> = Rc::new(RefCell::new(
            ((*(*(*state.borrow()).upgrade().deref())
                .dht_index
                .borrow_mut())
            .postfix_inc() as u64),
        ));
        if {
            let _lhs = (*huffman_code_index.borrow());
            _lhs >= (*huffman_code.upgrade().deref()).len() as u64
        } {
            return false;
        }
        let huff: Ptr<brunsli_JPEGHuffmanCode> = (huffman_code.to_strong().as_pointer()
            as Ptr<brunsli_JPEGHuffmanCode>)
            .offset((*huffman_code_index.borrow()) as isize);
        let index: Value<u64> = Rc::new(RefCell::new(
            ((*(*huff.upgrade().deref()).slot_id.borrow()) as u64),
        ));
        let huff_table: Value<Ptr<brunsli_HuffmanCodeTable>> =
            Rc::new(RefCell::new(Ptr::<brunsli_HuffmanCodeTable>::null()));
        if (((*index.borrow()) & 16_u64) != 0) {
            let rhs_0 = (*index.borrow()).wrapping_sub(16_u64);
            (*index.borrow_mut()) = rhs_0;
            (*huff_table.borrow_mut()) = (((*(*state.borrow()).upgrade().deref())
                .ac_huff_table
                .as_pointer()
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset((*index.borrow()) as isize));
        } else {
            (*huff_table.borrow_mut()) = (((*(*state.borrow()).upgrade().deref())
                .dc_huff_table
                .as_pointer()
                as Ptr<brunsli_HuffmanCodeTable>)
                .offset((*index.borrow()) as isize));
        }
        if !({
            let _huff: Ptr<brunsli_JPEGHuffmanCode> = (huff).clone();
            let _table: Ptr<brunsli_HuffmanCodeTable> = (*huff_table.borrow()).clone();
            BuildHuffmanCodeTable_240(_huff, _table)
        }) {
            return false;
        }
        let total_count: Value<u64> = Rc::new(RefCell::new(0_u64));
        let max_length: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*huff.upgrade().deref()).counts.borrow()).len() as u64
        } {
            if ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                .offset((*i.borrow()) as isize)
                .read())
                != 0)
            {
                (*max_length.borrow_mut()) = (*i.borrow());
            }
            let rhs_0 = (*total_count.borrow()).wrapping_add(
                ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as u64),
            );
            (*total_count.borrow_mut()) = rhs_0;
            (*i.borrow_mut()).prefix_inc();
        }
        (*total_count.borrow_mut()).prefix_dec();
        let __rhs = ((*(*huff.upgrade().deref()).slot_id.borrow()) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        let i: Value<u64> = Rc::new(RefCell::new(1_u64));
        'loop_: while ((*i.borrow())
            <= ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) as u64))
        {
            let __rhs = ((if ((*i.borrow()) == (*max_length.borrow())) {
                ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                    .offset((*i.borrow()) as isize)
                    .read())
                    - 1)
            } else {
                (((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                    .offset((*i.borrow()) as isize)
                    .read())
            }) as u8);
            (*data.borrow())
                .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*total_count.borrow())) {
            let __rhs = ((((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                .offset((*i.borrow()) as isize)
                .read()) as u8);
            (*data.borrow())
                .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        if (*(*huff.upgrade().deref()).is_last.borrow()) {
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
    let i: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*state.borrow()).upgrade().deref()).dqt_index.borrow()) as u64),
    ));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).quant.borrow()).len() as u64
    } {
        let table: Ptr<brunsli_JPEGQuantTable> = ((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*i.borrow()) as isize);
        (*marker_len.borrow_mut()) += (1 + {
            let _lhs = (if ((*(*table.upgrade().deref()).precision.borrow()) != 0) {
                2
            } else {
                1
            });
            _lhs * (*kDCTBlockSize_3.with(Value::clone).borrow())
        });
        if (*(*table.upgrade().deref()).is_last.borrow()) {
            break;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk2(Some(
                    (((*marker_len.borrow()) + 2) as u64),
                )),
            )
        });
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*((*(*state.borrow()).upgrade().deref())
            .output_queue
            .as_pointer() as Ptr<brunsli_internal_dec_OutputChunk>)
            .to_last()
            .upgrade()
            .deref())
        .buffer
        .borrow())
        .as_pointer()
        .to_strong()
        .as_pointer() as Ptr<u8>),
    ));
    let pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(255_u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(219_u8);
    let __rhs = (((*marker_len.borrow()) >> 8_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    let __rhs = ((((*marker_len.borrow()) as u32) & 255_u32) as u8);
    (*data.borrow())
        .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
        .write(__rhs);
    'loop_: while true {
        let idx: Value<u64> = Rc::new(RefCell::new(
            ((*(*(*state.borrow()).upgrade().deref())
                .dqt_index
                .borrow_mut())
            .postfix_inc() as u64),
        ));
        if {
            let _lhs = (*idx.borrow());
            _lhs >= (*(*jpg.upgrade().deref()).quant.borrow()).len() as u64
        } {
            return false;
        }
        let table: Ptr<brunsli_JPEGQuantTable> = ((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*idx.borrow()) as isize);
        let __rhs = (({
            let _lhs = ((*(*table.upgrade().deref()).precision.borrow()) << 4_u32);
            _lhs + (*(*table.upgrade().deref()).index.borrow())
        }) as u8);
        (*data.borrow())
            .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u64)) {
            let val_idx: Value<i32> = Rc::new(RefCell::new(
                ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    as i32),
            ));
            let val: Value<i32> = Rc::new(RefCell::new(
                (((*table.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                    .offset(((*val_idx.borrow()) as u64) as isize)
                    .read()),
            ));
            if ((*(*table.upgrade().deref()).precision.borrow()) != 0) {
                let __rhs = (((*val.borrow()) >> 8_u32) as u8);
                (*data.borrow())
                    .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            }
            let __rhs = ((((*val.borrow()) as u32) & 255_u32) as u8);
            (*data.borrow())
                .offset(((*pos.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        if (*(*table.upgrade().deref()).is_last.borrow()) {
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
    (*(*(*state.borrow()).upgrade().deref())
        .seen_dri_marker
        .borrow_mut()) = true;
    let dri_marker: Value<brunsli_internal_dec_OutputChunk> = Rc::new(RefCell::new(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![
            255_u8,
            221_u8,
            0_u8,
            4_u8,
            (((*(*jpg.upgrade().deref()).restart_interval.borrow()) >> 8) as u8),
            (((*(*jpg.upgrade().deref()).restart_interval.borrow()) & 255) as u8),
        ]),
    ));
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(std::mem::take(&mut (*dri_marker.borrow_mut())));
    return true;
}
pub fn EncodeRestart_248(marker: u8, state: Ptr<brunsli_internal_dec_SerializationState>) -> bool {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(
        brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![
            255_u8,
            (*marker.borrow()),
        ]),
    );
    return true;
}
pub fn EncodeAPP_249(
    jpg: Ptr<brunsli_JPEGData>,
    marker: u8,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let marker: Value<u8> = Rc::new(RefCell::new(marker));
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    (*marker.borrow_mut());
    let app_index: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*state.borrow()).upgrade().deref())
            .app_index
            .borrow_mut())
        .postfix_inc() as u64),
    ));
    if {
        let _lhs = (*app_index.borrow());
        _lhs >= (*(*jpg.upgrade().deref()).app_data.borrow()).len() as u64
    } {
        return false;
    }
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8]));
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
                    (((*jpg.upgrade().deref()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
                        .offset((*app_index.borrow()) as isize)
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>),
                ),
            )
        });
    return true;
}
pub fn EncodeCOM_250(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let com_index: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*state.borrow()).upgrade().deref())
            .com_index
            .borrow_mut())
        .postfix_inc() as u64),
    ));
    if {
        let _lhs = (*com_index.borrow());
        _lhs >= (*(*jpg.upgrade().deref()).com_data.borrow()).len() as u64
    } {
        return false;
    }
    (*(*(*state.borrow()).upgrade().deref())
        .output_queue
        .borrow_mut())
    .push(brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk3(vec![255_u8]));
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
                    (((*jpg.upgrade().deref()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
                        .offset((*com_index.borrow()) as isize)
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>),
                ),
            )
        });
    return true;
}
pub fn EncodeInterMarkerData_251(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let index: Value<u64> = Rc::new(RefCell::new(
        ((*(*(*state.borrow()).upgrade().deref())
            .data_index
            .borrow_mut())
        .postfix_inc() as u64),
    ));
    if {
        let _lhs = (*index.borrow());
        _lhs >= (*(*jpg.upgrade().deref()).inter_marker_data.borrow()).len() as u64
    } {
        return false;
    }
    (*(*state.borrow()).upgrade().deref())
        .output_queue
        .as_pointer()
        .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
            __v.push(
                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk4(
                    (((*jpg.upgrade().deref()).inter_marker_data.as_pointer()
                        as Ptr<Value<Vec<u8>>>)
                        .offset((*index.borrow()) as isize)
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>),
                ),
            )
        });
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
    let temp2: Value<i16> = <Value<i16>>::default();
    let temp: Value<i16> = <Value<i16>>::default();
    let __rhs = ((*coeffs.borrow()).offset((0) as isize).read());
    (*temp2.borrow_mut()) = __rhs;
    let __rhs = (({
        let _lhs = ((*temp2.borrow()) as i32);
        _lhs - (((*last_dc_coeff.borrow()).read()) as i32)
    }) as i16);
    (*temp.borrow_mut()) = __rhs;
    let __rhs = (*temp2.borrow());
    (*last_dc_coeff.borrow()).write(__rhs);
    (*temp2.borrow_mut()) = (*temp.borrow());
    if (((*temp.borrow()) as i32) < 0) {
        let __rhs = (-((*temp.borrow()) as i32) as i16);
        (*temp.borrow_mut()) = __rhs;
        (*temp2.borrow_mut()).postfix_dec();
    }
    let dc_nbits: Value<i32> = Rc::new(RefCell::new(if (((*temp.borrow()) as i32) == 0) {
        0
    } else {
        (({
            let _n: u32 = ((*temp.borrow()) as u32);
            Log2FloorNonZero_74(_n)
        }) + 1)
    }));
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _nbits: i32 =
            (*(*dc_huff.upgrade().deref()).depth.borrow())[(*dc_nbits.borrow()) as usize];
        let _bits: u64 =
            ((*(*dc_huff.upgrade().deref()).code.borrow())[(*dc_nbits.borrow()) as usize] as u64);
        WriteBits_233(_bw, _nbits, _bits)
    });
    if ((*dc_nbits.borrow()) > 0) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*dc_nbits.borrow());
            let _bits: u64 = ((((*temp2.borrow()) as u32)
                & ((1_u32 << (*dc_nbits.borrow())).wrapping_sub(1_u32)))
                as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
    }
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*k.borrow()) < 64) {
        if ((({
            let __rhs = ((*coeffs.borrow())
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize])
                        as isize,
                )
                .read());
            (*temp.borrow_mut()) = __rhs;
            (*temp.borrow())
        }) as i32)
            == 0)
        {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        if (((*temp.borrow()) as i32) < 0) {
            let __rhs = (-((*temp.borrow()) as i32) as i16);
            (*temp.borrow_mut()) = __rhs;
            (*temp2.borrow_mut()) = (!((*temp.borrow()) as i32) as i16);
        } else {
            (*temp2.borrow_mut()) = (*temp.borrow());
        }
        'loop_: while ((*r.borrow()) > 15) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(240) as usize];
                let _bits: u64 =
                    ((*(*ac_huff.upgrade().deref()).code.borrow())[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
        }
        let ac_nbits: Value<i32> = Rc::new(RefCell::new(
            (({
                let _n: u32 = ((*temp.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            }) + 1),
        ));
        let symbol: Value<i32> = Rc::new(RefCell::new(
            (((*r.borrow()) << 4_u32) + (*ac_nbits.borrow())),
        ));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 =
                (*(*ac_huff.upgrade().deref()).depth.borrow())[(*symbol.borrow()) as usize];
            let _bits: u64 =
                ((*(*ac_huff.upgrade().deref()).code.borrow())[(*symbol.borrow()) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*ac_nbits.borrow());
            let _bits: u64 =
                ((((*temp2.borrow()) as i32) & ((1 << (*ac_nbits.borrow())) - 1)) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*num_zero_runs.borrow())) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(240) as usize];
            let _bits: u64 = ((*(*ac_huff.upgrade().deref()).code.borrow())[(240) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (*r.borrow_mut()) -= 16;
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*r.borrow()) > 0) {
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(0) as usize];
            let _bits: u64 = ((*(*ac_huff.upgrade().deref()).code.borrow())[(0) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
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
    let temp2: Value<i16> = <Value<i16>>::default();
    let temp: Value<i16> = <Value<i16>>::default();
    if ((*Ss.borrow()) == 0) {
        let __rhs = (({
            let _lhs = (((*coeffs.borrow()).offset((0) as isize).read()) as i32);
            _lhs >> (*Al.borrow())
        }) as i16);
        (*temp2.borrow_mut()) = __rhs;
        let __rhs = (({
            let _lhs = ((*temp2.borrow()) as i32);
            _lhs - (((*last_dc_coeff.borrow()).read()) as i32)
        }) as i16);
        (*temp.borrow_mut()) = __rhs;
        let __rhs = (*temp2.borrow());
        (*last_dc_coeff.borrow()).write(__rhs);
        (*temp2.borrow_mut()) = (*temp.borrow());
        if (((*temp.borrow()) as i32) < 0) {
            let __rhs = (-((*temp.borrow()) as i32) as i16);
            (*temp.borrow_mut()) = __rhs;
            (*temp2.borrow_mut()).postfix_dec();
        }
        let nbits: Value<i32> = Rc::new(RefCell::new(if (((*temp.borrow()) as i32) == 0) {
            0
        } else {
            (({
                let _n: u32 = ((*temp.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            }) + 1)
        }));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 =
                (*(*dc_huff.upgrade().deref()).depth.borrow())[(*nbits.borrow()) as usize];
            let _bits: u64 =
                ((*(*dc_huff.upgrade().deref()).code.borrow())[(*nbits.borrow()) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        if ((*nbits.borrow()) > 0) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*nbits.borrow());
                let _bits: u64 =
                    ((((*temp2.borrow()) as i32) & ((1 << (*nbits.borrow())) - 1)) as u64);
                WriteBits_233(_bw, _nbits, _bits)
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
            let __rhs = ((*coeffs.borrow())
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize])
                        as isize,
                )
                .read());
            (*temp.borrow_mut()) = __rhs;
            (*temp.borrow())
        }) as i32)
            == 0)
        {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        if (((*temp.borrow()) as i32) < 0) {
            let __rhs = (-((*temp.borrow()) as i32) as i16);
            (*temp.borrow_mut()) = __rhs;
            let rhs_0 = (((*temp.borrow()) as i32) >> (*Al.borrow())) as i16;
            (*temp.borrow_mut()) = rhs_0;
            (*temp2.borrow_mut()) = (!((*temp.borrow()) as i32) as i16);
        } else {
            let rhs_0 = (((*temp.borrow()) as i32) >> (*Al.borrow())) as i16;
            (*temp.borrow_mut()) = rhs_0;
            (*temp2.borrow_mut()) = (*temp.borrow());
        }
        if (((*temp.borrow()) as i32) == 0) {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            Flush_238(_s, _bw)
        });
        'loop_: while ((*r.borrow()) > 15) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(240) as usize];
                let _bits: u64 =
                    ((*(*ac_huff.upgrade().deref()).code.borrow())[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
        }
        let nbits: Value<i32> = Rc::new(RefCell::new(
            (({
                let _n: u32 = ((*temp.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            }) + 1),
        ));
        let symbol: Value<i32> =
            Rc::new(RefCell::new((((*r.borrow()) << 4_u32) + (*nbits.borrow()))));
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 =
                (*(*ac_huff.upgrade().deref()).depth.borrow())[(*symbol.borrow()) as usize];
            let _bits: u64 =
                ((*(*ac_huff.upgrade().deref()).code.borrow())[(*symbol.borrow()) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = (*nbits.borrow());
            let _bits: u64 = ((((*temp2.borrow()) as i32) & ((1 << (*nbits.borrow())) - 1)) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).prefix_inc();
    }
    if ((*num_zero_runs.borrow()) > 0) {
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            Flush_238(_s, _bw)
        });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*num_zero_runs.borrow())) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(240) as usize];
                let _bits: u64 =
                    ((*(*ac_huff.upgrade().deref()).code.borrow())[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if ((*r.borrow()) > 0) {
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
            let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
            let _new_bits_array: Ptr<i32> = Ptr::<i32>::null();
            let _new_bits_count: u64 = 0_u64;
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            BufferEndOfBand_239(_s, _ac_huff, _new_bits_array, _new_bits_count, _bw)
        });
        if !(*eob_run_allowed.borrow()) {
            ({
                let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                Flush_238(_s, _bw)
            });
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
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = 1;
            let _bits: u64 = ((({
                let _lhs = (((*coeffs.borrow()).offset((0) as isize).read()) as i32);
                _lhs >> (*Al.borrow())
            }) & 1) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    let abs_values: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    let eob: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        let abs_val: Value<i16> = Rc::new(RefCell::new(
            ((((*coeffs.borrow())
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize])
                        as isize,
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
    let refinement_bits: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    let refinement_bits_count: Value<u64> = Rc::new(RefCell::new(0_u64));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
        if ((*abs_values.borrow())[(*k.borrow()) as usize] == 0) {
            (*r.borrow_mut()).postfix_inc();
            (*k.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        'loop_: while ((*r.borrow()) > 15) && ((*k.borrow()) <= (*eob.borrow())) {
            ({
                let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                Flush_238(_s, _bw)
            });
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = (*(*ac_huff.upgrade().deref()).depth.borrow())[(240) as usize];
                let _bits: u64 =
                    ((*(*ac_huff.upgrade().deref()).code.borrow())[(240) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            (*r.borrow_mut()) -= 16;
            let i: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while ((*i.borrow()) < (*refinement_bits_count.borrow())) {
                ({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _nbits: i32 = 1;
                    let _bits: u64 = ((*refinement_bits.borrow())[(*i.borrow()) as usize] as u64);
                    WriteBits_233(_bw, _nbits, _bits)
                });
                (*i.borrow_mut()).prefix_inc();
            }
            (*refinement_bits_count.borrow_mut()) = 0_u64;
        }
        if ((*abs_values.borrow())[(*k.borrow()) as usize] > 1) {
            (*refinement_bits.borrow_mut())
                [((*refinement_bits_count.borrow_mut()).postfix_inc()) as usize] =
                ((((*abs_values.borrow())[(*k.borrow()) as usize] as u32) & 1_u32) as i32);
            (*k.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            Flush_238(_s, _bw)
        });
        let symbol: Value<i32> = Rc::new(RefCell::new((((*r.borrow()) << 4_u32) + 1)));
        let new_non_zero_bit: Value<i32> = Rc::new(RefCell::new(
            if ((((*coeffs.borrow())
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize])
                        as isize,
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
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 =
                (*(*ac_huff.upgrade().deref()).depth.borrow())[(*symbol.borrow()) as usize];
            let _bits: u64 =
                ((*(*ac_huff.upgrade().deref()).code.borrow())[(*symbol.borrow()) as usize] as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            let _nbits: i32 = 1;
            let _bits: u64 = ((*new_non_zero_bit.borrow()) as u64);
            WriteBits_233(_bw, _nbits, _bits)
        });
        let i: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow()) < (*refinement_bits_count.borrow())) {
            ({
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                let _nbits: i32 = 1;
                let _bits: u64 = ((*refinement_bits.borrow())[(*i.borrow()) as usize] as u64);
                WriteBits_233(_bw, _nbits, _bits)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        (*refinement_bits_count.borrow_mut()) = 0_u64;
        (*r.borrow_mut()) = 0;
        (*k.borrow_mut()).postfix_inc();
    }
    if ((*r.borrow()) > 0) || ((*refinement_bits_count.borrow()) != 0) {
        if !({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
            let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
            let _new_bits_array: Ptr<i32> = (refinement_bits.as_pointer() as Ptr<i32>);
            let _new_bits_count: u64 = (*refinement_bits_count.borrow());
            let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
            BufferEndOfBand_239(_s, _ac_huff, _new_bits_array, _new_bits_count, _bw)
        }) {
            return false;
        }
        if !(*eob_run_allowed.borrow()) {
            ({
                let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
                let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                Flush_238(_s, _bw)
            });
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
    let scan_info: Ptr<brunsli_JPEGScanInfo> = ((*jpg.upgrade().deref()).scan_info.as_pointer()
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*(*(*state.borrow()).upgrade().deref()).scan_index.borrow()) as u64) as isize);
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (*(*state.borrow()).upgrade().deref())
        .scan_state
        .as_pointer();
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*(*(*state.borrow()).upgrade().deref())
            .seen_dri_marker
            .borrow())
        {
            (*(*jpg.upgrade().deref()).restart_interval.borrow())
        } else {
            0
        },
    ));
    let get_next_extra_zero_run_index: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).extra_zero_runs.borrow()).len() as u64
            } {
                return (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                    .offset((*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow()) as isize)
                    .upgrade()
                    .deref())
                .block_idx
                .borrow());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    let get_next_reset_point: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).next_reset_point_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).reset_points.borrow()).len() as u64
            } {
                return (((*scan_info.upgrade().deref()).reset_points.as_pointer() as Ptr<i32>)
                    .offset(
                        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()).postfix_inc()
                            as isize,
                    )
                    .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    if {
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)
    } {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                ((*ss.upgrade().deref()).bw.as_pointer());
            let _output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>> =
                ((*(*state.borrow()).upgrade().deref())
                    .output_queue
                    .as_pointer());
            BitWriterInit_228(_bw, _output_queue)
        });
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                ((*ss.upgrade().deref()).coding_state.as_pointer());
            DCTCodingStateInit_237(_s)
        });
        (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) = (*restart_interval.borrow());
        (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref())
            .next_extra_zero_run_index
            .borrow_mut()) = ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
            ({ (*get_next_reset_point.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()) = 0;
        {
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .clone()
        };
        (*(*ss.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_EncodeScanState_Stage::BODY;
    }
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new(((*ss.upgrade().deref()).bw.as_pointer())));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(
        ((*ss.upgrade().deref()).coding_state.as_pointer()),
    ));
    if !({
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("jpeg_data_writer.cc");
            let _l: i32 = 741;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DoEncodeScan");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        ((*(*scan_info.upgrade().deref()).num_components.borrow()) > 1_u64),
    ));
    let base_component: Ptr<brunsli_JPEGComponent> =
        ((*jpg.upgrade().deref()).components.as_pointer() as Ptr<brunsli_JPEGComponent>).offset(
            ((*(*((*scan_info.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .comp_idx
            .borrow()) as u64) as isize,
        );
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).h_samp_factor.borrow())
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).v_samp_factor.borrow())
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).width.borrow());
                _lhs * (*h_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).height.borrow());
                _lhs * (*v_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*(*(*state.borrow()).upgrade().deref())
            .is_progressive
            .borrow()),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Al.borrow())
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Ss.borrow())
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Se.borrow())
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        ({
            let _lhs = (*(*parsing_state.upgrade().deref()).stage.borrow()).clone();
            _lhs == brunsli_internal_dec_Stage::DONE
        }),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                let _state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                let _tag: u32 = ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u32);
                HasSection_194(_state, _tag)
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
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
        {
            let _lhs = (*(*(*(*(*parsing_state.upgrade().deref()).internal.borrow())
                .as_ref()
                .unwrap()
                .borrow())
            .ac_dc
            .borrow())
            .next_mcu_y
            .borrow());
            _lhs * (*v_group.borrow())
        }
    }));
    'loop_: while {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*last_mcu_y.borrow())
    } {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0)
                && ((*(*ss.upgrade().deref()).restarts_to_go.borrow()) == 0)
            {
                ({
                    let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                        (*coding_state.borrow()).clone();
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    Flush_238(_s, _bw)
                });
                if !({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _pad_bits: Ptr<Ptr<i32>> =
                        ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
                    let _pad_bits_end: Ptr<i32> =
                        (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                ({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _marker: i32 =
                        (208 + (*(*ss.upgrade().deref()).next_restart_marker.borrow()));
                    EmitMarker_234(_bw, _marker)
                });
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) += 1;
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) &= 7;
                (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) =
                    (*restart_interval.borrow());
                {
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .clone()
                };
            }
            let i: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while {
                let _lhs = (*i.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).num_components.borrow())
            } {
                let si: Ptr<brunsli_JPEGComponentScanInfo> =
                    ((*scan_info.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponentScanInfo>)
                        .offset((*i.borrow()) as isize);
                let c: Ptr<brunsli_JPEGComponent> =
                    ((*jpg.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponent>)
                        .offset(((*(*si.upgrade().deref()).comp_idx.borrow()) as u64) as isize);
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .dc_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).dc_tbl_idx.borrow()) as u64) as isize);
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .ac_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).ac_tbl_idx.borrow()) as u64) as isize);
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).v_samp_factor.borrow())
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).h_samp_factor.borrow())
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new({
                            let _lhs = {
                                let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
                                _lhs * (*n_blocks_y.borrow())
                            };
                            _lhs + (*iy.borrow())
                        }));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul((*(*c.upgrade().deref()).width_in_blocks.borrow())))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_reset_point.borrow())
                        } {
                            ({
                                let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
                                Flush_238(_s, _bw)
                            });
                            (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
                                ({ (*get_next_reset_point.borrow_mut())() }).clone();
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_extra_zero_run_index.borrow())
                        } {
                            (*num_zero_runs.borrow_mut()) =
                                (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                                    .offset(
                                        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow())
                                            as isize,
                                    )
                                    .upgrade()
                                    .deref())
                                .num_extra_zero_runs
                                .borrow());
                            (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut())
                                .prefix_inc();
                            (*(*ss.upgrade().deref())
                                .next_extra_zero_run_index
                                .borrow_mut()) =
                                ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
                        }
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            (((*c.upgrade().deref()).coeffs.as_pointer() as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as u64) as isize)),
                        ));
                        let ok: Value<bool> = <Value<bool>>::default();
                        if (0 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()).prefix_inc();
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()).prefix_dec();
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*MCU_rows.borrow())
    } {
        if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    ({
        let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        Flush_238(_s, _bw)
    });
    if !({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _pad_bits: Ptr<Ptr<i32>> =
            ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
        let _pad_bits_end: Ptr<i32> =
            (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        BitWriterFinish_236(_bw)
    });
    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
        brunsli_internal_dec_EncodeScanState_Stage::HEAD;
    (*(*(*state.borrow()).upgrade().deref())
        .scan_index
        .borrow_mut())
    .postfix_inc();
    if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub fn DoEncodeScan_256(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = ((*jpg.upgrade().deref()).scan_info.as_pointer()
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*(*(*state.borrow()).upgrade().deref()).scan_index.borrow()) as u64) as isize);
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (*(*state.borrow()).upgrade().deref())
        .scan_state
        .as_pointer();
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*(*(*state.borrow()).upgrade().deref())
            .seen_dri_marker
            .borrow())
        {
            (*(*jpg.upgrade().deref()).restart_interval.borrow())
        } else {
            0
        },
    ));
    let get_next_extra_zero_run_index: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).extra_zero_runs.borrow()).len() as u64
            } {
                return (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                    .offset((*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow()) as isize)
                    .upgrade()
                    .deref())
                .block_idx
                .borrow());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    let get_next_reset_point: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).next_reset_point_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).reset_points.borrow()).len() as u64
            } {
                return (((*scan_info.upgrade().deref()).reset_points.as_pointer() as Ptr<i32>)
                    .offset(
                        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()).postfix_inc()
                            as isize,
                    )
                    .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    if {
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)
    } {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                ((*ss.upgrade().deref()).bw.as_pointer());
            let _output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>> =
                ((*(*state.borrow()).upgrade().deref())
                    .output_queue
                    .as_pointer());
            BitWriterInit_228(_bw, _output_queue)
        });
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                ((*ss.upgrade().deref()).coding_state.as_pointer());
            DCTCodingStateInit_237(_s)
        });
        (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) = (*restart_interval.borrow());
        (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref())
            .next_extra_zero_run_index
            .borrow_mut()) = ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
            ({ (*get_next_reset_point.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()) = 0;
        {
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .clone()
        };
        (*(*ss.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_EncodeScanState_Stage::BODY;
    }
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new(((*ss.upgrade().deref()).bw.as_pointer())));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(
        ((*ss.upgrade().deref()).coding_state.as_pointer()),
    ));
    if !({
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("jpeg_data_writer.cc");
            let _l: i32 = 741;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DoEncodeScan");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        ((*(*scan_info.upgrade().deref()).num_components.borrow()) > 1_u64),
    ));
    let base_component: Ptr<brunsli_JPEGComponent> =
        ((*jpg.upgrade().deref()).components.as_pointer() as Ptr<brunsli_JPEGComponent>).offset(
            ((*(*((*scan_info.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .comp_idx
            .borrow()) as u64) as isize,
        );
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).h_samp_factor.borrow())
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).v_samp_factor.borrow())
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).width.borrow());
                _lhs * (*h_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).height.borrow());
                _lhs * (*v_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*(*(*state.borrow()).upgrade().deref())
            .is_progressive
            .borrow()),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Al.borrow())
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Ss.borrow())
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Se.borrow())
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        ({
            let _lhs = (*(*parsing_state.upgrade().deref()).stage.borrow()).clone();
            _lhs == brunsli_internal_dec_Stage::DONE
        }),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                let _state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                let _tag: u32 = ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u32);
                HasSection_194(_state, _tag)
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
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
        {
            let _lhs = (*(*(*(*(*parsing_state.upgrade().deref()).internal.borrow())
                .as_ref()
                .unwrap()
                .borrow())
            .ac_dc
            .borrow())
            .next_mcu_y
            .borrow());
            _lhs * (*v_group.borrow())
        }
    }));
    'loop_: while {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*last_mcu_y.borrow())
    } {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0)
                && ((*(*ss.upgrade().deref()).restarts_to_go.borrow()) == 0)
            {
                ({
                    let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                        (*coding_state.borrow()).clone();
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    Flush_238(_s, _bw)
                });
                if !({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _pad_bits: Ptr<Ptr<i32>> =
                        ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
                    let _pad_bits_end: Ptr<i32> =
                        (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                ({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _marker: i32 =
                        (208 + (*(*ss.upgrade().deref()).next_restart_marker.borrow()));
                    EmitMarker_234(_bw, _marker)
                });
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) += 1;
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) &= 7;
                (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) =
                    (*restart_interval.borrow());
                {
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .clone()
                };
            }
            let i: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while {
                let _lhs = (*i.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).num_components.borrow())
            } {
                let si: Ptr<brunsli_JPEGComponentScanInfo> =
                    ((*scan_info.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponentScanInfo>)
                        .offset((*i.borrow()) as isize);
                let c: Ptr<brunsli_JPEGComponent> =
                    ((*jpg.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponent>)
                        .offset(((*(*si.upgrade().deref()).comp_idx.borrow()) as u64) as isize);
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .dc_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).dc_tbl_idx.borrow()) as u64) as isize);
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .ac_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).ac_tbl_idx.borrow()) as u64) as isize);
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).v_samp_factor.borrow())
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).h_samp_factor.borrow())
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new({
                            let _lhs = {
                                let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
                                _lhs * (*n_blocks_y.borrow())
                            };
                            _lhs + (*iy.borrow())
                        }));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul((*(*c.upgrade().deref()).width_in_blocks.borrow())))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_reset_point.borrow())
                        } {
                            ({
                                let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
                                Flush_238(_s, _bw)
                            });
                            (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
                                ({ (*get_next_reset_point.borrow_mut())() }).clone();
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_extra_zero_run_index.borrow())
                        } {
                            (*num_zero_runs.borrow_mut()) =
                                (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                                    .offset(
                                        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow())
                                            as isize,
                                    )
                                    .upgrade()
                                    .deref())
                                .num_extra_zero_runs
                                .borrow());
                            (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut())
                                .prefix_inc();
                            (*(*ss.upgrade().deref())
                                .next_extra_zero_run_index
                                .borrow_mut()) =
                                ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
                        }
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            (((*c.upgrade().deref()).coeffs.as_pointer() as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as u64) as isize)),
                        ));
                        let ok: Value<bool> = <Value<bool>>::default();
                        if (1 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()).prefix_inc();
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()).prefix_dec();
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*MCU_rows.borrow())
    } {
        if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    ({
        let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        Flush_238(_s, _bw)
    });
    if !({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _pad_bits: Ptr<Ptr<i32>> =
            ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
        let _pad_bits_end: Ptr<i32> =
            (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        BitWriterFinish_236(_bw)
    });
    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
        brunsli_internal_dec_EncodeScanState_Stage::HEAD;
    (*(*(*state.borrow()).upgrade().deref())
        .scan_index
        .borrow_mut())
    .postfix_inc();
    if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub fn DoEncodeScan_257(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = ((*jpg.upgrade().deref()).scan_info.as_pointer()
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*(*(*state.borrow()).upgrade().deref()).scan_index.borrow()) as u64) as isize);
    let ss: Ptr<brunsli_internal_dec_EncodeScanState> = (*(*state.borrow()).upgrade().deref())
        .scan_state
        .as_pointer();
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        if (*(*(*state.borrow()).upgrade().deref())
            .seen_dri_marker
            .borrow())
        {
            (*(*jpg.upgrade().deref()).restart_interval.borrow())
        } else {
            0
        },
    ));
    let get_next_extra_zero_run_index: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).extra_zero_runs.borrow()).len() as u64
            } {
                return (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                    .offset((*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow()) as isize)
                    .upgrade()
                    .deref())
                .block_idx
                .borrow());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    let get_next_reset_point: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = (*(*ss.upgrade().deref()).next_reset_point_pos.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).reset_points.borrow()).len() as u64
            } {
                return (((*scan_info.upgrade().deref()).reset_points.as_pointer() as Ptr<i32>)
                    .offset(
                        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()).postfix_inc()
                            as isize,
                    )
                    .read());
            } else {
                return -1_i32;
            }
            panic!("ub: non-void function does not return a value")
        }),
    ));
    if {
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::HEAD as i32)
    } {
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            let _scan_info: Ptr<brunsli_JPEGScanInfo> = (scan_info).clone();
            let _state: Ptr<brunsli_internal_dec_SerializationState> = (*state.borrow()).clone();
            EncodeSOS_244(_jpg, _scan_info, _state)
        }) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        ({
            let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                ((*ss.upgrade().deref()).bw.as_pointer());
            let _output_queue: Ptr<Vec<brunsli_internal_dec_OutputChunk>> =
                ((*(*state.borrow()).upgrade().deref())
                    .output_queue
                    .as_pointer());
            BitWriterInit_228(_bw, _output_queue)
        });
        ({
            let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                ((*ss.upgrade().deref()).coding_state.as_pointer());
            DCTCodingStateInit_237(_s)
        });
        (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) = (*restart_interval.borrow());
        (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()) = 0;
        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref())
            .next_extra_zero_run_index
            .borrow_mut()) = ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).next_reset_point_pos.borrow_mut()) = 0_u64;
        (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
            ({ (*get_next_reset_point.borrow_mut())() }).clone();
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()) = 0;
        {
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
            (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                .to_any()
                .clone()
        };
        (*(*ss.upgrade().deref()).stage.borrow_mut()) =
            brunsli_internal_dec_EncodeScanState_Stage::BODY;
    }
    let bw: Value<Ptr<brunsli_internal_dec_BitWriter>> =
        Rc::new(RefCell::new(((*ss.upgrade().deref()).bw.as_pointer())));
    let coding_state: Value<Ptr<brunsli_internal_dec_DCTCodingState>> = Rc::new(RefCell::new(
        ((*ss.upgrade().deref()).coding_state.as_pointer()),
    ));
    if !({
        let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
        _lhs == (brunsli_internal_dec_EncodeScanState_Stage::BODY as i32)
    }) {
        ({
            let _f: Ptr<u8> = Ptr::from_string_literal("jpeg_data_writer.cc");
            let _l: i32 = 741;
            let _fn: Ptr<u8> = Ptr::from_string_literal("DoEncodeScan");
            BrunsliDumpAndAbort_79(_f, _l, _fn)
        });
        'loop_: while true {}
    };
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        ((*(*scan_info.upgrade().deref()).num_components.borrow()) > 1_u64),
    ));
    let base_component: Ptr<brunsli_JPEGComponent> =
        ((*jpg.upgrade().deref()).components.as_pointer() as Ptr<brunsli_JPEGComponent>).offset(
            ((*(*((*scan_info.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset(0_u64 as isize)
                .upgrade()
                .deref())
            .comp_idx
            .borrow()) as u64) as isize,
        );
    let h_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).h_samp_factor.borrow())
    }));
    let v_group: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
        1
    } else {
        (*(*base_component.upgrade().deref()).v_samp_factor.borrow())
    }));
    let MCUs_per_row: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).width.borrow());
                _lhs * (*h_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let MCU_rows: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: i32 = {
                let _lhs = (*(*jpg.upgrade().deref()).height.borrow());
                _lhs * (*v_group.borrow())
            };
            let _b: i32 = (8 * (*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()));
            DivCeil_226(_a, _b)
        }),
    ));
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*(*(*state.borrow()).upgrade().deref())
            .is_progressive
            .borrow()),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Al.borrow())
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Ss.borrow())
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Se.borrow())
    } else {
        63
    }));
    let want_ac: Value<bool> = Rc::new(RefCell::new(
        (((*Ss.borrow()) != 0) || ((*Se.borrow()) != 0)),
    ));
    let complete_ac: Value<bool> = Rc::new(RefCell::new(
        ({
            let _lhs = (*(*parsing_state.upgrade().deref()).stage.borrow()).clone();
            _lhs == brunsli_internal_dec_Stage::DONE
        }),
    ));
    let has_ac: Value<bool> = Rc::new(RefCell::new(
        (*complete_ac.borrow())
            || ({
                let _state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                let _tag: u32 = ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u32);
                HasSection_194(_state, _tag)
            }),
    ));
    if (*want_ac.borrow()) && (!(*has_ac.borrow())) {
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
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
        {
            let _lhs = (*(*(*(*(*parsing_state.upgrade().deref()).internal.borrow())
                .as_ref()
                .unwrap()
                .borrow())
            .ac_dc
            .borrow())
            .next_mcu_y
            .borrow());
            _lhs * (*v_group.borrow())
        }
    }));
    'loop_: while {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*last_mcu_y.borrow())
    } {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*restart_interval.borrow()) > 0)
                && ((*(*ss.upgrade().deref()).restarts_to_go.borrow()) == 0)
            {
                ({
                    let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                        (*coding_state.borrow()).clone();
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    Flush_238(_s, _bw)
                });
                if !({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _pad_bits: Ptr<Ptr<i32>> =
                        ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
                    let _pad_bits_end: Ptr<i32> =
                        (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
                    JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
                }) {
                    return brunsli_internal_dec_SerializationStatus::ERROR;
                }
                ({
                    let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
                    let _marker: i32 =
                        (208 + (*(*ss.upgrade().deref()).next_restart_marker.borrow()));
                    EmitMarker_234(_bw, _marker)
                });
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) += 1;
                (*(*ss.upgrade().deref()).next_restart_marker.borrow_mut()) &= 7;
                (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()) =
                    (*restart_interval.borrow());
                {
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as u64 as usize);
                    (((*ss.upgrade().deref()).last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                        .to_any()
                        .clone()
                };
            }
            let i: Value<u64> = Rc::new(RefCell::new(0_u64));
            'loop_: while {
                let _lhs = (*i.borrow());
                _lhs < (*(*scan_info.upgrade().deref()).num_components.borrow())
            } {
                let si: Ptr<brunsli_JPEGComponentScanInfo> =
                    ((*scan_info.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponentScanInfo>)
                        .offset((*i.borrow()) as isize);
                let c: Ptr<brunsli_JPEGComponent> =
                    ((*jpg.upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponent>)
                        .offset(((*(*si.upgrade().deref()).comp_idx.borrow()) as u64) as isize);
                let dc_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .dc_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).dc_tbl_idx.borrow()) as u64) as isize);
                let ac_huff: Ptr<brunsli_HuffmanCodeTable> =
                    ((*(*state.borrow()).upgrade().deref())
                        .ac_huff_table
                        .as_pointer() as Ptr<brunsli_HuffmanCodeTable>)
                        .offset(((*(*si.upgrade().deref()).ac_tbl_idx.borrow()) as u64) as isize);
                let n_blocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).v_samp_factor.borrow())
                } else {
                    1
                }));
                let n_blocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*c.upgrade().deref()).h_samp_factor.borrow())
                } else {
                    1
                }));
                let iy: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*iy.borrow()) < (*n_blocks_y.borrow())) {
                    let ix: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*ix.borrow()) < (*n_blocks_x.borrow())) {
                        let block_y: Value<i32> = Rc::new(RefCell::new({
                            let _lhs = {
                                let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
                                _lhs * (*n_blocks_y.borrow())
                            };
                            _lhs + (*iy.borrow())
                        }));
                        let block_x: Value<i32> = Rc::new(RefCell::new(
                            (((*mcu_x.borrow()) * (*n_blocks_x.borrow())) + (*ix.borrow())),
                        ));
                        let block_idx: Value<i32> = Rc::new(RefCell::new(
                            (((((*block_y.borrow()) as u32)
                                .wrapping_mul((*(*c.upgrade().deref()).width_in_blocks.borrow())))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_reset_point.borrow())
                        } {
                            ({
                                let _s: Ptr<brunsli_internal_dec_DCTCodingState> =
                                    (*coding_state.borrow()).clone();
                                let _bw: Ptr<brunsli_internal_dec_BitWriter> =
                                    (*bw.borrow()).clone();
                                Flush_238(_s, _bw)
                            });
                            (*(*ss.upgrade().deref()).next_reset_point.borrow_mut()) =
                                ({ (*get_next_reset_point.borrow_mut())() }).clone();
                        }
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        if {
                            let _lhs = (*(*ss.upgrade().deref()).block_scan_index.borrow());
                            _lhs == (*(*ss.upgrade().deref()).next_extra_zero_run_index.borrow())
                        } {
                            (*num_zero_runs.borrow_mut()) =
                                (*(*((*scan_info.upgrade().deref()).extra_zero_runs.as_pointer()
                                    as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                                    .offset(
                                        (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow())
                                            as isize,
                                    )
                                    .upgrade()
                                    .deref())
                                .num_extra_zero_runs
                                .borrow());
                            (*(*ss.upgrade().deref()).extra_zero_runs_pos.borrow_mut())
                                .prefix_inc();
                            (*(*ss.upgrade().deref())
                                .next_extra_zero_run_index
                                .borrow_mut()) =
                                ({ (*get_next_extra_zero_run_index.borrow_mut())() }).clone();
                        }
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            (((*c.upgrade().deref()).coeffs.as_pointer() as Ptr<i16>)
                                .offset((((*block_idx.borrow()) << 6) as u64) as isize)),
                        ));
                        let ok: Value<bool> = <Value<bool>>::default();
                        if (2 == 0) {
                            (*ok.borrow_mut()) = ({
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                let _dc_huff: Ptr<brunsli_HuffmanCodeTable> = (dc_huff).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanCodeTable> = (ac_huff).clone();
                                let _num_zero_runs: i32 = (*num_zero_runs.borrow());
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                                let _last_dc_coeff: Ptr<i16> =
                                    ((*ss.upgrade().deref()).last_dc_coeff.as_pointer()
                                        as Ptr<i16>)
                                        .offset(
                                            ((*(*si.upgrade().deref()).comp_idx.borrow()) as i32)
                                                as isize,
                                        );
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
                            return brunsli_internal_dec_SerializationStatus::ERROR;
                        }
                        (*(*ss.upgrade().deref()).block_scan_index.borrow_mut()).prefix_inc();
                        (*ix.borrow_mut()).prefix_inc();
                    }
                    (*iy.borrow_mut()).prefix_inc();
                }
                (*i.borrow_mut()).prefix_inc();
            }
            (*(*ss.upgrade().deref()).restarts_to_go.borrow_mut()).prefix_dec();
            (*mcu_x.borrow_mut()).prefix_inc();
        }
        (*(*ss.upgrade().deref()).mcu_y.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (*(*ss.upgrade().deref()).mcu_y.borrow());
        _lhs < (*MCU_rows.borrow())
    } {
        if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
            return brunsli_internal_dec_SerializationStatus::ERROR;
        }
        return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
    }
    ({
        let _s: Ptr<brunsli_internal_dec_DCTCodingState> = (*coding_state.borrow()).clone();
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        Flush_238(_s, _bw)
    });
    if !({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        let _pad_bits: Ptr<Ptr<i32>> =
            ((*(*state.borrow()).upgrade().deref()).pad_bits.as_pointer());
        let _pad_bits_end: Ptr<i32> =
            (*(*(*state.borrow()).upgrade().deref()).pad_bits_end.borrow()).clone();
        JumpToByteBoundary_235(_bw, _pad_bits, _pad_bits_end)
    }) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    ({
        let _bw: Ptr<brunsli_internal_dec_BitWriter> = (*bw.borrow()).clone();
        BitWriterFinish_236(_bw)
    });
    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
        brunsli_internal_dec_EncodeScanState_Stage::HEAD;
    (*(*(*state.borrow()).upgrade().deref())
        .scan_index
        .borrow_mut())
    .postfix_inc();
    if !(*(*(*bw.borrow()).upgrade().deref()).healthy.borrow()) {
        return brunsli_internal_dec_SerializationStatus::ERROR;
    }
    return brunsli_internal_dec_SerializationStatus::DONE;
}
pub fn EncodeScan_258(
    jpg: Ptr<brunsli_JPEGData>,
    parsing_state: Ptr<brunsli_internal_dec_State>,
    state: Ptr<brunsli_internal_dec_SerializationState>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_SerializationState>> = Rc::new(RefCell::new(state));
    let scan_info: Ptr<brunsli_JPEGScanInfo> = ((*jpg.upgrade().deref()).scan_info.as_pointer()
        as Ptr<brunsli_JPEGScanInfo>)
        .offset(((*(*(*state.borrow()).upgrade().deref()).scan_index.borrow()) as u64) as isize);
    let is_progressive: Value<bool> = Rc::new(RefCell::new(
        (*(*(*state.borrow()).upgrade().deref())
            .is_progressive
            .borrow()),
    ));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Al.borrow())
    } else {
        0
    }));
    let Ah: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Ah.borrow())
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Ss.borrow())
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*scan_info.upgrade().deref()).Se.borrow())
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
    let to_status: Value<_> = Rc::new(RefCell::new(
        (|result: bool| {
            let result: Value<bool> = Rc::new(RefCell::new(result));
            return if (*result.borrow()) {
                brunsli_internal_dec_SerializationStatus::DONE
            } else {
                brunsli_internal_dec_SerializationStatus::ERROR
            };
        }),
    ));
    'switch: {
        let __match_cond = ((*marker.borrow()) as i32);
        match __match_cond {
            v if v == 192 || v == 193 || v == 194 || v == 201 || v == 202 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _marker: u8 = (*marker.borrow());
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeSOF_243(_jpg, _marker, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 196 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeDHT_245(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
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
                return ({
                    let _result: bool = ({
                        let _marker: u8 = (*marker.borrow());
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeRestart_248(_marker, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 217 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeEOI_242(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 218 => {
                return ({
                    let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                    let _parsing_state: Ptr<brunsli_internal_dec_State> = (parsing_state).clone();
                    let _state: Ptr<brunsli_internal_dec_SerializationState> =
                        (*state.borrow()).clone();
                    EncodeScan_258(_jpg, _parsing_state, _state)
                });
            }
            v if v == 219 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeDQT_246(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 221 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeDRI_247(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
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
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _marker: u8 = (*marker.borrow());
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeAPP_249(_jpg, _marker, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 254 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeCOM_250(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            v if v == 255 => {
                return ({
                    let _result: bool = ({
                        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                        let _state: Ptr<brunsli_internal_dec_SerializationState> =
                            (*state.borrow()).clone();
                        EncodeInterMarkerData_251(_jpg, _state)
                    });
                    (*to_status.borrow_mut())(_result)
                })
                .clone();
            }
            _ => {
                return brunsli_internal_dec_SerializationStatus::ERROR;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn PushOutput_260(
    in_: Ptr<Vec<brunsli_internal_dec_OutputChunk>>,
    available_out: Ptr<u64>,
    next_out: Ptr<Ptr<u8>>,
) {
    let in_: Value<Ptr<Vec<brunsli_internal_dec_OutputChunk>>> = Rc::new(RefCell::new(in_));
    let available_out: Value<Ptr<u64>> = Rc::new(RefCell::new(available_out));
    let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
    'loop_: while (((*available_out.borrow()).read()) > 0_u64) {
        if (*(*in_.borrow()).upgrade().deref()).is_empty() {
            return;
        }
        let chunk: Ptr<brunsli_internal_dec_OutputChunk> =
            ((*in_.borrow()).to_strong().as_pointer() as Ptr<brunsli_internal_dec_OutputChunk>);
        let to_copy: Value<u64> = Rc::new(RefCell::new(
            (if (*available_out.borrow()).clone().read()
                <= (*chunk.upgrade().deref()).len.as_pointer().read()
            {
                (*available_out.borrow()).clone()
            } else {
                (*chunk.upgrade().deref()).len.as_pointer()
            }
            .read()),
        ));
        if ((*to_copy.borrow()) > 0_u64) {
            {
                (((*next_out.borrow()).read()).clone() as Ptr<u8>)
                    .to_any()
                    .memcpy(
                        &((*(*chunk.upgrade().deref()).next.borrow()).clone() as Ptr<u8>).to_any(),
                        (*to_copy.borrow()) as usize,
                    );
                (((*next_out.borrow()).read()).clone() as Ptr<u8>)
                    .to_any()
                    .clone()
            };
            let __rhs = (*to_copy.borrow());
            {
                let _ptr = (*next_out.borrow()).clone();
                _ptr.write(_ptr.read() + __rhs)
            };
            let rhs_0 = ((*available_out.borrow()).read()).wrapping_sub((*to_copy.borrow()));
            (*available_out.borrow()).write(rhs_0);
            (*(*chunk.upgrade().deref()).next.borrow_mut()) += (*to_copy.borrow());
            let rhs_0 =
                (*(*chunk.upgrade().deref()).len.borrow()).wrapping_sub((*to_copy.borrow()));
            (*(*chunk.upgrade().deref()).len.borrow_mut()) = rhs_0;
        }
        if ((*(*chunk.upgrade().deref()).len.borrow()) == 0_u64) {
            (*in_.borrow())
                .with_mut(|__v: &mut Vec<brunsli_internal_dec_OutputChunk>| __v.remove(0));
        }
    }
}
pub fn WriteJpeg_261(jpg: Ptr<brunsli_JPEGData>, out: brunsli_JPEGOutput) -> bool {
    let out: Value<brunsli_JPEGOutput> = Rc::new(RefCell::new(out));
    let state: Value<brunsli_internal_dec_State> = Rc::new(RefCell::new(
        brunsli_internal_dec_State::brunsli_internal_dec_State(),
    ));
    (*(*state.borrow()).stage.borrow_mut()) = brunsli_internal_dec_Stage::DONE;
    let buffer: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(16384_u64) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    'loop_: while true {
        let next_out: Value<Ptr<u8>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<u8>)));
        let available_out: Value<u64> = Rc::new(RefCell::new((*buffer.borrow()).len() as u64));
        let status: Value<brunsli_internal_dec_SerializationStatus> = Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_dec_State> = (state.as_pointer());
                let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                let _available_out: Ptr<u64> = (available_out.as_pointer());
                let _next_out: Ptr<Ptr<u8>> = (next_out.as_pointer());
                SerializeJpeg_206(_state, _jpg, _available_out, _next_out)
            }),
        ));
        if ((*status.borrow()) != brunsli_internal_dec_SerializationStatus::DONE)
            && ((*status.borrow()) != brunsli_internal_dec_SerializationStatus::NEEDS_MORE_OUTPUT)
        {
            return false;
        }
        let to_write: Value<u64> = Rc::new(RefCell::new(
            ((*buffer.borrow()).len() as u64).wrapping_sub((*available_out.borrow())),
        ));
        if !({
            let _buf: Ptr<u8> = (buffer.as_pointer() as Ptr<u8>);
            let _len: u64 = (*to_write.borrow());
            (*out.borrow()).Write(_buf, _len)
        }) {
            return false;
        }
        if ((*status.borrow()) == brunsli_internal_dec_SerializationStatus::DONE) {
            return true;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn SerializeJpeg_206(
    state: Ptr<brunsli_internal_dec_State>,
    jpg: Ptr<brunsli_JPEGData>,
    available_out: Ptr<u64>,
    next_out: Ptr<Ptr<u8>>,
) -> brunsli_internal_dec_SerializationStatus {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let available_out: Value<Ptr<u64>> = Rc::new(RefCell::new(available_out));
    let next_out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(next_out));
    let ss: Ptr<brunsli_internal_dec_SerializationState> =
        (*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
            .as_ref()
            .unwrap()
            .borrow())
        .serialization
        .as_pointer();
    let maybe_push_output: Value<_> = Rc::new(RefCell::new(
        (|| {
            if {
                let _lhs = ((*(*ss.upgrade().deref()).stage.borrow()) as i32).clone();
                _lhs != (brunsli_internal_dec_SerializationState_Stage::ERROR as i32)
            } {
                ({
                    let _in: Ptr<Vec<brunsli_internal_dec_OutputChunk>> =
                        ((*ss.upgrade().deref()).output_queue.as_pointer());
                    let _available_out: Ptr<u64> = (*available_out.borrow()).clone();
                    let _next_out: Ptr<Ptr<u8>> = (*next_out.borrow()).clone();
                    PushOutput_260(_in, _available_out, _next_out)
                });
            }
        }),
    ));
    ({ (*maybe_push_output.borrow_mut())() });
    'loop_: while true {
        switch!(match ((*(*ss.upgrade().deref()).stage.borrow()) as i32) {
            v if v == (brunsli_internal_dec_SerializationState_Stage::INIT as i32) => {
                let can_start_serialization: Value<bool> = Rc::new(RefCell::new(
                    ({
                        let _lhs = (*(*(*state.borrow()).upgrade().deref()).stage.borrow()).clone();
                        _lhs == brunsli_internal_dec_Stage::DONE
                    }),
                ));
                if ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 = ((*kBrunsliDCDataTag_36.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) || ({
                    let _state: Ptr<brunsli_internal_dec_State> = (*state.borrow()).clone();
                    let _tag: u32 = ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as u32);
                    HasSection_194(_state, _tag)
                }) {
                    (*can_start_serialization.borrow_mut()) = true;
                }
                if !(*can_start_serialization.borrow()) {
                    return brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT;
                }
                if {
                    let _lhs = (*(*jpg.upgrade().deref()).version.borrow());
                    _lhs == (*kFallbackVersion_2.with(Value::clone).borrow())
                } {
                    if (*(*jpg.upgrade().deref()).original_jpg.borrow()).is_null() {
                        (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                            (brunsli_internal_dec_SerializationState_Stage::ERROR).clone();
                        break;
                    }
                    (*ss.upgrade().deref()).output_queue.as_pointer().with_mut(
                        |__v: &mut Vec<brunsli_internal_dec_OutputChunk>| {
                            __v.push(
                                brunsli_internal_dec_OutputChunk::brunsli_internal_dec_OutputChunk1(
                                    (*(*jpg.upgrade().deref()).original_jpg.borrow()).clone(),
                                    (*(*jpg.upgrade().deref()).original_jpg_size.borrow()),
                                ),
                            )
                        },
                    );
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::DONE;
                    break;
                }
                if {
                    let _lhs = ((*(*jpg.upgrade().deref()).version.borrow()) & 1);
                    _lhs == (*kFallbackVersion_2.with(Value::clone).borrow())
                } {
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::ERROR;
                    break;
                }
                if (*(*jpg.upgrade().deref()).marker_order.borrow()).is_empty() {
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::ERROR;
                    break;
                }
                {
                    let __a0 = ((*kMaxHuffmanTables_6.with(Value::clone).borrow()) as u64) as usize;
                    (*(*ss.upgrade().deref()).dc_huff_table.borrow_mut())
                        .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                };
                {
                    let __a0 = ((*kMaxHuffmanTables_6.with(Value::clone).borrow()) as u64) as usize;
                    (*(*ss.upgrade().deref()).ac_huff_table.borrow_mut())
                        .resize_with(__a0, || <brunsli_HuffmanCodeTable>::default())
                };
                if (*(*jpg.upgrade().deref()).has_zero_padding_bit.borrow()) {
                    (*(*ss.upgrade().deref()).pad_bits.borrow_mut()) =
                        ((*jpg.upgrade().deref()).padding_bits.as_pointer() as Ptr<i32>);
                    let __rhs = (*(*ss.upgrade().deref()).pad_bits.borrow()).offset(
                        ((*(*jpg.upgrade().deref()).padding_bits.borrow()).len() as u64) as isize,
                    );
                    (*(*ss.upgrade().deref()).pad_bits_end.borrow_mut()) = __rhs;
                }
                ({
                    let _state: Ptr<brunsli_internal_dec_SerializationState> = (ss).clone();
                    EncodeSOI_241(_state)
                });
                ({ (*maybe_push_output.borrow_mut())() });
                (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                    (brunsli_internal_dec_SerializationState_Stage::SERIALIZE_SECTION).clone();
                break;
            }
            v if v == (brunsli_internal_dec_SerializationState_Stage::SERIALIZE_SECTION as i32) => {
                if {
                    let _lhs = (*(*ss.upgrade().deref()).section_index.borrow());
                    _lhs >= (*(*jpg.upgrade().deref()).marker_order.borrow()).len() as u64
                } {
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::DONE;
                    break;
                }
                let marker: Value<u8> = Rc::new(RefCell::new(
                    (((*jpg.upgrade().deref()).marker_order.as_pointer() as Ptr<u8>)
                        .offset((*(*ss.upgrade().deref()).section_index.borrow()) as isize)
                        .read()),
                ));
                let status: Value<brunsli_internal_dec_SerializationStatus> =
                    Rc::new(RefCell::new(
                        ({
                            let _marker: u8 = (*marker.borrow());
                            let _parsing_state: Ptr<brunsli_internal_dec_State> =
                                (*state.borrow()).clone();
                            let _state: Ptr<brunsli_internal_dec_SerializationState> = (ss).clone();
                            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                            SerializeSection_259(_marker, _parsing_state, _state, _jpg)
                        }),
                    ));
                if ((*status.borrow()) == brunsli_internal_dec_SerializationStatus::ERROR) {
                    if true {
                    } else {
                        write!(libcc2rs::cerr(), "Failed to encode marker ",);
                        libcc2rs::cerr().write_all(&([(&[(*marker.borrow())] as &[u8])].concat()));
                        write!(libcc2rs::cerr(), "\n",);
                    }
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::ERROR;
                    break;
                }
                ({ (*maybe_push_output.borrow_mut())() });
                if ((*status.borrow())
                    == brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT)
                {
                    return (brunsli_internal_dec_SerializationStatus::NEEDS_MORE_INPUT).clone();
                } else if ((*status.borrow()) != brunsli_internal_dec_SerializationStatus::DONE) {
                    if !(false) {
                        ({
                            let _f: Ptr<u8> = Ptr::from_string_literal("jpeg_data_writer.cc");
                            let _l: i32 = 1073;
                            let _fn: Ptr<u8> = Ptr::from_string_literal("SerializeJpeg");
                            BrunsliDumpAndAbort_79(_f, _l, _fn)
                        });
                        'loop_: while true {}
                    };
                    (*(*ss.upgrade().deref()).stage.borrow_mut()) =
                        brunsli_internal_dec_SerializationState_Stage::ERROR;
                    break;
                }
                (*(*ss.upgrade().deref()).section_index.borrow_mut()).prefix_inc();
                break;
            }
            v if v == (brunsli_internal_dec_SerializationState_Stage::DONE as i32) => {
                if !(*(*ss.upgrade().deref()).output_queue.borrow()).is_empty() {
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
#[derive()]
pub struct brunsli_internal_dec_State {
    pub stage: Value<brunsli_internal_dec_Stage>,
    pub tags_met: Value<u32>,
    pub skip_tags: Value<u32>,
    pub data: Value<Ptr<u8>>,
    pub len: Value<u64>,
    pub pos: Value<u64>,
    pub context_map: Value<Ptr<u8>>,
    pub entropy_codes: Value<Ptr<brunsli_ANSDecodingData>>,
    pub use_legacy_context_model: Value<bool>,
    pub is_storage_allocated: Value<bool>,
    pub meta: Value<Vec<brunsli_internal_dec_ComponentMeta>>,
    pub internal: Value<Option<Value<brunsli_internal_dec_InternalState>>>,
}
impl brunsli_internal_dec_State {
    pub fn brunsli_internal_dec_State() -> Self {
        let mut this = Self {
            stage: Rc::new(RefCell::new(brunsli_internal_dec_Stage::SIGNATURE)),
            tags_met: Rc::new(RefCell::new(0_u32)),
            skip_tags: Rc::new(RefCell::new(0_u32)),
            data: Rc::new(RefCell::new(Ptr::<u8>::null())),
            len: Rc::new(RefCell::new(0_u64)),
            pos: Rc::new(RefCell::new(0_u64)),
            context_map: Rc::new(RefCell::new(Ptr::<u8>::null())),
            entropy_codes: Rc::new(RefCell::new(Ptr::<brunsli_ANSDecodingData>::null())),
            use_legacy_context_model: Rc::new(RefCell::new(false)),
            is_storage_allocated: Rc::new(RefCell::new(false)),
            meta: Rc::new(RefCell::new(Vec::new())),
            internal: Rc::new(RefCell::new(
                Ptr::alloc(<brunsli_internal_dec_InternalState>::default()).to_owned_opt(),
            )),
        };
        this
    }
}
impl Default for brunsli_internal_dec_State {
    fn default() -> Self {
        {
            brunsli_internal_dec_State::brunsli_internal_dec_State()
        }
    }
}
impl ByteRepr for brunsli_internal_dec_State {}
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
#[derive(Default)]
pub struct brunsli_internal_dec_MetadataState {
    pub short_marker_count: Value<u64>,
    pub marker: Value<u8>,
    pub length_hi: Value<u8>,
    pub remaining_multibyte_length: Value<u64>,
    pub multibyte_sink: Value<Ptr<Vec<u8>>>,
    pub stage: Value<u64>,
    pub brotli: Value<*mut ::brotli_sys::BrotliDecoderState>,
    pub metadata_size: Value<u64>,
    pub decompressed_size: Value<u64>,
    pub result: Value<brunsli_BrunsliStatus>,
    pub decompression_stage: Value<brunsli_internal_dec_MetadataDecompressionStage>,
}
impl brunsli_internal_dec_MetadataState {
    pub fn CanFinish(&self) -> bool {
        return ((*self.stage.borrow())
            == (brunsli_internal_dec_MetadataState_Stage::READ_MARKER as u64))
            || ((*self.stage.borrow())
                == (brunsli_internal_dec_MetadataState_Stage::READ_TAIL as u64));
    }
}
impl Clone for brunsli_internal_dec_MetadataState {
    fn clone(&self) -> Self {
        let mut this = Self {
            short_marker_count: Rc::new(RefCell::new((*self.short_marker_count.borrow()))),
            marker: Rc::new(RefCell::new((*self.marker.borrow()))),
            length_hi: Rc::new(RefCell::new((*self.length_hi.borrow()))),
            remaining_multibyte_length: Rc::new(RefCell::new(
                (*self.remaining_multibyte_length.borrow()),
            )),
            multibyte_sink: Rc::new(RefCell::new((*self.multibyte_sink.borrow()).clone())),
            stage: Rc::new(RefCell::new((*self.stage.borrow()))),
            brotli: Rc::new(RefCell::new((*self.brotli.borrow()).clone())),
            metadata_size: Rc::new(RefCell::new((*self.metadata_size.borrow()))),
            decompressed_size: Rc::new(RefCell::new((*self.decompressed_size.borrow()))),
            result: Rc::new(RefCell::new((*self.result.borrow()).clone())),
            decompression_stage: Rc::new(RefCell::new(
                (*self.decompression_stage.borrow()).clone(),
            )),
        };
        this
    }
}
impl Drop for brunsli_internal_dec_MetadataState {
    fn drop(&mut self) {
        if !((*self.brotli.borrow()).is_null()) {
            unsafe { ::brotli_sys::BrotliDecoderDestroyInstance((*self.brotli.borrow())) };
            (*self.brotli.borrow_mut()) = std::ptr::null_mut();
        }
    }
}
impl ByteRepr for brunsli_internal_dec_MetadataState {}
impl brunsli_internal_dec_State {}
impl brunsli_internal_dec_State {}
pub fn HasSection_194(state: Ptr<brunsli_internal_dec_State>, tag: u32) -> bool {
    let state: Value<Ptr<brunsli_internal_dec_State>> = Rc::new(RefCell::new(state));
    let tag: Value<u32> = Rc::new(RefCell::new(tag));
    return ({
        let _lhs = (*(*(*(*(*(*state.borrow()).upgrade().deref()).internal.borrow())
            .as_ref()
            .unwrap()
            .borrow())
        .section
        .borrow())
        .tags_met
        .borrow());
        _lhs & (1_u32 << (*tag.borrow()))
    } != 0);
}
pub fn StringWriter_262(data: AnyPtr, buf: Ptr<u8>, count: u64) -> u64 {
    let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let count: Value<u64> = Rc::new(RefCell::new(count));
    let output: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(
        ((*data.borrow()).reinterpret_cast::<Vec<u8>>()).clone(),
    ));
    {
        ((*output.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>).with_mut(
            |__v: &mut Vec<u8>| {
                __v.pop();
                __v.extend(
                    (*buf.borrow())
                        .reinterpret_cast::<u8>()
                        .map(|c| c.read())
                        .take((*count.borrow()) as usize),
                );
                __v.push(0);
            },
        );
        ((*output.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
    };
    return (*count.borrow());
}
pub fn ReadFileInternal_263(file: Ptr<::std::fs::File>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(file));
    let content: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(content));
    if (if (match 2 {
        0 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::Start(0_i64 as u64))),
        1 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::Current(0_i64))),
        2 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::End(0_i64))),
        _ => Err(std::io::Error::other("unsupported whence for fseek.")),
    })
    .is_ok()
    {
        0
    } else {
        -1
    } != 0)
    {
        eprintln!("Failed to seek end of input file.");
        return false;
    }
    let input_size: Value<i32> = Rc::new(RefCell::new(
        ((*file.borrow()).with_mut(|v| match v.stream_position() {
            Ok(pos) => pos as i64,
            Err(_) => -1,
        }) as i32),
    ));
    if ((*input_size.borrow()) == 0) {
        eprintln!("Input file is empty.");
        return false;
    }
    if (if (match 0 {
        0 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::Start(0_i64 as u64))),
        1 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::Current(0_i64))),
        2 => (*file.borrow())
            .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::End(0_i64))),
        _ => Err(std::io::Error::other("unsupported whence for fseek.")),
    })
    .is_ok()
    {
        0
    } else {
        -1
    } != 0)
    {
        eprintln!("Failed to rewind input file to the beginning.");
        return false;
    }
    {
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| __v.pop());
        (*content.borrow())
            .with_mut(|__v: &mut Vec<u8>| __v.resize(((*input_size.borrow()) as u64) as usize, 0));
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(0))
    };
    let read_pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while {
        let _lhs = (*read_pos.borrow());
        _lhs < ((*(*content.borrow()).upgrade().deref()).len() - 1) as u64
    } {
        let bytes_read: Value<u64> = Rc::new(RefCell::new({
            let __a0 = ((if (*read_pos.borrow()) as usize
                >= (*((*content.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
                    .upgrade()
                    .deref())
                .len()
                    - 1
            {
                panic!("out of bounds access")
            } else {
                (((*content.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
                    .to_strong()
                    .as_pointer() as Ptr<u8>)
                    .offset((*read_pos.borrow()) as isize)
            }) as Ptr<u8>)
                .to_any();
            let __a1 = 1_u64;
            let __a2 = (((*(*content.borrow()).upgrade().deref()).len() - 1) as u64)
                .wrapping_sub((*read_pos.borrow()));
            let __a3 = (*file.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        }));
        if ((*bytes_read.borrow()) == 0_u64) {
            eprintln!("Failed to read input file");
            return false;
        }
        let rhs_0 = (*read_pos.borrow()).wrapping_add((*bytes_read.borrow()));
        (*read_pos.borrow_mut()) = rhs_0;
    }
    return true;
}
pub fn ReadFile_264(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let content: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(content));
    let file: Value<Ptr<::std::fs::File>> =
        Rc::new(RefCell::new(
            match Ptr::from_string_literal("rb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open((file_name.to_strong().as_pointer() as Ptr<u8>).to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open((file_name.to_strong().as_pointer() as Ptr<u8>).to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
    if (*file.borrow()).is_null() {
        eprintln!("Failed to open input file.");
        return false;
    }
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file: Ptr<::std::fs::File> = (*file.borrow()).clone();
            let _content: Ptr<Vec<u8>> = (*content.borrow()).clone();
            ReadFileInternal_263(_file, _content)
        }),
    ));
    if ({
        (*file.borrow()).delete();
        0
    } != 0)
    {
        if (*ok.borrow()) {
            eprintln!("Failed to close input file.");
        }
        return false;
    }
    return (*ok.borrow());
}
pub fn WriteFileInternal_265(file: Ptr<::std::fs::File>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(file));
    let write_pos: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while {
        let _lhs = (*write_pos.borrow());
        _lhs < ((*content.upgrade().deref()).len() - 1) as u64
    } {
        let bytes_written: Value<u64> = Rc::new(RefCell::new({
            let __a0 = (((content.to_strong().as_pointer() as Ptr<u8>)
                .offset((*write_pos.borrow()) as isize)) as Ptr<u8>)
                .to_any();
            let __a1 = 1_u64;
            let __a2 = (((*content.upgrade().deref()).len() - 1) as u64)
                .wrapping_sub((*write_pos.borrow()));
            let __a3 = (*file.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        }));
        if ((*bytes_written.borrow()) == 0_u64) {
            eprintln!("Failed to write output.");
            return false;
        }
        let rhs_0 = (*write_pos.borrow()).wrapping_add((*bytes_written.borrow()));
        (*write_pos.borrow_mut()) = rhs_0;
    }
    return true;
}
pub fn WriteFile_266(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<::std::fs::File>> =
        Rc::new(RefCell::new(
            match Ptr::from_string_literal("wb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open((file_name.to_strong().as_pointer() as Ptr<u8>).to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open((file_name.to_strong().as_pointer() as Ptr<u8>).to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
    if (*file.borrow()).is_null() {
        eprintln!("Failed to open file for writing.");
        return false;
    }
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file: Ptr<::std::fs::File> = (*file.borrow()).clone();
            let _content: Ptr<Vec<u8>> = (content).clone();
            WriteFileInternal_265(_file, _content)
        }),
    ));
    if ({
        (*file.borrow()).delete();
        0
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
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::brunsli_JPEGData()));
    let input_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (input.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
    ));
    let status: Value<brunsli_BrunsliStatus> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*input_data.borrow()).clone();
            let _len: u64 = ((*input.borrow()).len() - 1) as u64;
            let _jpg: Ptr<brunsli_JPEGData> = (jpg.as_pointer());
            BrunsliDecodeJpeg_204(_data, _len, _jpg)
        }),
    ));
    (*ok.borrow_mut()) =
        (((*status.borrow()) as i32) == (brunsli_BrunsliStatus::BRUNSLI_OK as i32)).clone();
    if ((*(*jpg.borrow()).version.borrow()) != (*kFallbackVersion_2.with(Value::clone).borrow())) {
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
    let writer: Value<brunsli_JPEGOutput> =
        Rc::new(RefCell::new(brunsli_JPEGOutput::brunsli_JPEGOutput(
            FnPtr::<fn(AnyPtr, Ptr<u8>, u64) -> u64>::new(StringWriter_262),
            ((output.as_pointer()) as Ptr<Vec<u8>>).to_any(),
        )));
    (*ok.borrow_mut()) = ({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _out: brunsli_JPEGOutput = (*writer.borrow()).clone();
        WriteJpeg_261(_jpg, _out)
    });
    if !(*ok.borrow()) {
        eprintln!("Failed to serialize JPEG data.");
        return false;
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
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<u8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(argv));
    if ((*argc.borrow()) != 2) && ((*argc.borrow()) != 3) {
        eprintln!("Usage: dbrunsli FILE [OUTPUT_FILE, default=FILE.jpg]");
        return 1;
    }
    let file_name: Value<Vec<u8>> = Rc::new(RefCell::new(
        ((*argv.borrow()).offset((1) as isize).read())
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    if (*file_name.borrow()).len() <= 1 {
        eprintln!("Empty input file name.");
        return 1;
    }
    let outfile_name: Value<Vec<u8>> = Rc::new(RefCell::new(if ((*argc.borrow()) == 2) {
        {
            let mut r = (*file_name.borrow()).clone();
            r.pop();
            r.extend(Ptr::from_string_literal(".jpg").to_c_string_iterator());
            r.push(0);
            r
        }
    } else {
        ((*argv.borrow()).offset((2) as isize).read())
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>()
    }));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file_name: Ptr<Vec<u8>> = file_name.as_pointer();
            let _outfile_name: Ptr<Vec<u8>> = outfile_name.as_pointer();
            ProcessFile_267(_file_name, _outfile_name)
        }),
    ));
    return if (*ok.borrow()) { 0 } else { 1 };
}
