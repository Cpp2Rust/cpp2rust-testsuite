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
    pub num_components: Value<usize>,
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
            num_components: Rc::new(RefCell::new(0_usize)),
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
    pub original_jpg_size: Value<usize>,
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
            original_jpg_size: Rc::new(RefCell::new(0_usize)),
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
    return ((((((((((*(*jpg.upgrade().deref()).components.borrow()).len() == 3_usize)
        && ((*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()) == 2))
        && ((*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()) == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 2))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1));
}
pub fn JPEGDataIs444_16(jpg: Ptr<brunsli_JPEGData>) -> bool {
    return ((((((((((*(*jpg.upgrade().deref()).components.borrow()).len() == 3_usize)
        && ((*(*jpg.upgrade().deref()).max_h_samp_factor.borrow()) == 1))
        && ((*(*jpg.upgrade().deref()).max_v_samp_factor.borrow()) == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow())
            == 1))
        && ((*(*((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(2_usize as isize)
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
        .wrapping_mul(((*(*jpg.upgrade().deref()).components.borrow()).len() as u64)))
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
    pub static kBrunsliSignatureSize_43: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!();
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
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u16>() as usize);
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
            ::std::mem::size_of::<u16>() as usize,
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
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u32>() as usize);
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
            .memcpy(&(*p.borrow()), ::std::mem::size_of::<u64>() as usize);
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
pub fn Append_72(dst: Ptr<Vec<u8>>, begin: Ptr<u8>, length: usize) {
    let dst: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(dst));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new(begin));
    let length: Value<usize> = Rc::new(RefCell::new(length));
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
        let _length: usize = (*src.upgrade().deref()).len();
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"FastDivide");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"context.cc"), 55, _fn)
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
    pub static kMaxAverageContext_82: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
    pub static kNumAvrgContexts_83: Value<usize> = Rc::new(RefCell::new(
        (*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(1_usize),
    ));
);
thread_local!(
    pub static kNumNonZeroBits_84: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!(
    pub static kNumNonZeroTreeSize_85: Value<usize> = Rc::new(RefCell::new(
        ((((1_u32 << (*kNumNonZeroBits_84.with(Value::clone).borrow())) as u32)
            .wrapping_sub((1_u32 as u32))) as usize),
    ));
);
thread_local!(
    pub static kNumNonZeroQuant_86: Value<usize> = Rc::new(RefCell::new(2_usize));
);
thread_local!(
    pub static kNumNonZeroContextMax_87: Value<usize> = Rc::new(RefCell::new(
        (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())
            .wrapping_div((*kNumNonZeroQuant_86.with(Value::clone).borrow())),
    ));
);
thread_local!(
    pub static kNumNonZeroContextCount_88: Value<usize> = Rc::new(RefCell::new(
        (*kNumNonZeroContextMax_87.with(Value::clone).borrow()).wrapping_add(1_usize),
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
pub fn ZeroDensityContext_96(nonzeros_left: usize, k: usize, bits: usize) -> u16 {
    let nonzeros_left: Value<usize> = Rc::new(RefCell::new(nonzeros_left));
    let k: Value<usize> = Rc::new(RefCell::new(k));
    let bits: Value<usize> = Rc::new(RefCell::new(bits));
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
        >> ((*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(2_usize)))
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
pub fn ACPredictContext_101(p: i64, avg_ctx: Ptr<usize>, sgn: Ptr<usize>) {
    let p: Value<i64> = Rc::new(RefCell::new(p));
    let avg_ctx: Value<Ptr<usize>> = Rc::new(RefCell::new(avg_ctx));
    let sgn: Value<Ptr<usize>> = Rc::new(RefCell::new(sgn));
    let multiplier: Value<i32> = <Value<i32>>::default();
    if ((*p.borrow()) >= 0_i64) {
        (*multiplier.borrow_mut()) = 1;
    } else {
        (*multiplier.borrow_mut()) = -1_i32;
        let __rhs = -(*p.borrow());
        (*p.borrow_mut()) = __rhs;
    }
    let ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*p.borrow()) >= ((1_u32 << (*kMaxAverageContext_82.with(Value::clone).borrow())) as i64)) {
        (*ctx.borrow_mut()) = (*kMaxAverageContext_82.with(Value::clone).borrow());
    } else {
        (*ctx.borrow_mut()) = (({
            let _n: u32 = ((2_u32).wrapping_mul(((*p.borrow()) as u32))).wrapping_add(1_u32);
            Log2FloorNonZero_74(_n)
        }) as usize);
    }
    let __rhs = (*ctx.borrow());
    (*avg_ctx.borrow()).write(__rhs);
    let __rhs = (*kMaxAverageContext_82.with(Value::clone).borrow())
        .wrapping_add(((*multiplier.borrow()) as usize).wrapping_mul((*ctx.borrow())));
    (*sgn.borrow()).write(__rhs);
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
        let _avg_ctx: Ptr<usize> = (*avg_ctx.borrow()).clone();
        let _sgn: Ptr<usize> = (*sgn.borrow()).clone();
        ACPredictContext_101(_p, _avg_ctx, _sgn)
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
        let _avg_ctx: Ptr<usize> = (*avg_ctx.borrow()).clone();
        let _sgn: Ptr<usize> = (*sgn.borrow()).clone();
        ACPredictContext_101(_p, _avg_ctx, _sgn)
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
                (((*prev.borrow()).offset(((*x.borrow()) - 1) as isize).read()) as usize);
        }
    } else if ((*x.borrow()) == 0) {
        (*prediction.borrow_mut()) =
            (((*prev.borrow()).offset((*x.borrow()) as isize).read()) as usize);
    } else {
        (*prediction.borrow_mut()) =
            (((((((*prev.borrow()).offset(((*x.borrow()) - 1) as isize).read()) as i32)
                + (((*prev.borrow()).offset((*x.borrow()) as isize).read()) as i32))
                + 1)
                / 2) as usize);
    }
    if !((*prediction.borrow()) <= (*kNumNonZeroTreeSize_85.with(Value::clone).borrow())) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"NumNonzerosContext");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"context.cc"), 305, _fn)
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
                (0..((*kNumIsEmptyBlockContexts_105.with(Value::clone).borrow()) as usize)
                    as usize)
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
        };
        ({ this.InitAll() });
        this
    }
    pub fn SetWidth(&self, w: i32) {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        (*self.width.borrow_mut()) = (*w.borrow());
        {
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
            (*self.prev_is_nonempty.borrow_mut()).resize(__a0, 1)
        };
        {
            let __a0 = (((*w.borrow()) + 3) as usize) as usize;
            (*self.prev_abs_coeff.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
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
                    * (*kDCTBlockSize_3.with(Value::clone).borrow())) as usize)
                    as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            sign_prob: Rc::new(RefCell::new(
                (0
                    ..(((((2_usize)
                        .wrapping_mul((*kMaxAverageContext_82.with(Value::clone).borrow()))
                        as usize)
                        .wrapping_add(1_usize)) as usize)
                        .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)))
                        as usize)
                    .map(|_| <brunsli_Prob>::default())
                    .collect::<Vec<_>>(),
            )),
            num_nonzero_prob: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2016, _>(
                |_| brunsli_Prob::brunsli_Prob(),
            )))),
            first_extra_bit_prob: Rc::new(RefCell::new(
                (0..((10 * (*kDCTBlockSize_3.with(Value::clone).borrow())) as usize) as usize)
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
            let __a0 = (((*w.borrow()) + 1) as usize) as usize;
            (*self.prev_is_nonempty.borrow_mut()).resize(__a0, 1)
        };
        {
            let __a0 = ((*w.borrow()) as usize) as usize;
            (*self.prev_num_nonzeros.borrow_mut()).resize_with(__a0, || <u8>::default())
        };
        {
            let __a0 = ((((*kDCTBlockSize_3.with(Value::clone).borrow()) * 2) * ((*w.borrow()) + 3))
                as usize) as usize;
            (*self.prev_abs_coeff.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
        {
            let __a0 = (((*kDCTBlockSize_3.with(Value::clone).borrow()) * ((*w.borrow()) + 1))
                as usize) as usize;
            (*self.prev_sign.borrow_mut()).resize_with(__a0, || <i32>::default())
        };
    }
    pub fn SizeInBytes(w: i32) -> usize {
        let w: Value<i32> = Rc::new(RefCell::new(w));
        return ( ( ( ( ( ( ( ( ( 4 + ( ( ( 10 + ( 3 * (*w.borrow()) ) ) ) * (*kDCTBlockSize_3.with(Value::clone).borrow()) ) ) + ( 2 * (*w.borrow()) ) ) ) as usize ) ) ) . wrapping_mul ( ( ::std::mem::size_of::<i32>() as usize ) ) as u64 ) ) . wrapping_add ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( (*kNumNonzeroBuckets_90.with(Value::clone).borrow()) as usize ) ) ) . wrapping_add ( ( ( 2_usize ) . wrapping_mul ( (*kMaxAverageContext_82.with(Value::clone).borrow()) ) as usize ) ) as usize ) ) . wrapping_add ( 11_usize ) ) as usize ) ) . wrapping_mul ( ( ( (*kDCTBlockSize_3.with(Value::clone).borrow()) as usize ) ) ) as usize ) ) . wrapping_add ( ( ( (*kNumNonZeroContextCount_88.with(Value::clone).borrow()) ) . wrapping_mul ( (*kNumNonZeroTreeSize_85.with(Value::clone).borrow()) ) as usize ) ) ) as u64 ) ) . wrapping_mul ( ( ::std::mem::size_of::<brunsli_Prob>() as u64 ) ) as u64 ) ) as usize )  ;
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
    let y: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*y.borrow()) < 8_usize) {
        let x: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*x.borrow()) < 8_usize) {
            let __rhs = {
                let _lhs = ({
                    let _lhs = ((*quant.borrow())
                        .offset(
                            ((*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow()))))
                                as isize,
                        )
                        .read());
                    _lhs * (*kSqrt2FixedPoint_108.with(Value::clone).borrow())
                });
                _lhs / ((*quant.borrow())
                    .offset(((*y.borrow()).wrapping_mul(8_usize)) as isize)
                    .read())
            };
            (*mult_row.borrow())
                .offset(
                    ((*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow())))) as isize,
                )
                .write(__rhs);
            let __rhs = {
                let _lhs = ({
                    let _lhs = ((*quant.borrow())
                        .offset(
                            ((*x.borrow()).wrapping_add((8_usize).wrapping_mul((*y.borrow()))))
                                as isize,
                        )
                        .read());
                    _lhs * (*kSqrt2FixedPoint_108.with(Value::clone).borrow())
                });
                _lhs / ((*quant.borrow()).offset((*x.borrow()) as isize).read())
            };
            (*mult_col.borrow())
                .offset(
                    (((*x.borrow()).wrapping_mul(8_usize)).wrapping_add((*y.borrow()))) as isize,
                )
                .write(__rhs);
            (*x.borrow_mut()).prefix_inc();
        }
        (*y.borrow_mut()).prefix_inc();
    }
}
impl brunsli_ComponentStateDC {
    fn InitAll(&self) {
        ({ (*self.is_zero_prob.borrow()).Init(135_u8) });
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.sign_prob.borrow()).len()) {
            ({
                (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(128_u8)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.is_empty_block_prob.borrow()).len()) {
            ({
                (*(self.is_empty_block_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(74_u8)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.first_extra_bit_prob.borrow()).len()) {
            ({
                (*(self.first_extra_bit_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(150_u8)
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
                        let _fn: Ptr<u8> = Ptr::from_string_literal(b"InitAll");
                        BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"context.cc"), 227, _fn)
                    });
                    'loop_: while true {}
                };
                ({
                    let _probability: u8 = ((*v.borrow()) as u8);
                    (*(self.is_zero_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset(
                            ((((*i.borrow()) * (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                + (*k.borrow())) as usize) as isize,
                        )
                        .upgrade()
                        .deref())
                    .Init(_probability)
                });
                (*k.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.sign_prob.borrow()).len()) {
            if ((*i.borrow())
                < (*kMaxAverageContext_82.with(Value::clone).borrow())
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)))
            {
                ({
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(108_u8)
                });
            } else if ((*i.borrow())
                < (((*kMaxAverageContext_82.with(Value::clone).borrow()).wrapping_add(1_usize))
                    as usize)
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)))
            {
                ({
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(128_u8)
                });
            } else {
                ({
                    (*(self.sign_prob.as_pointer() as Ptr<brunsli_Prob>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref())
                    .Init(148_u8)
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.first_extra_bit_prob.borrow()).len()) {
            ({
                (*(self.first_extra_bit_prob.as_pointer() as Ptr<brunsli_Prob>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Init(158_u8)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*kNumNonZeroContextCount_88.with(Value::clone).borrow())) {
            let non_zero_probs: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                (self.num_nonzero_prob.as_pointer() as Ptr<brunsli_Prob>).offset(
                    ((*i.borrow())
                        .wrapping_mul((*kNumNonZeroTreeSize_85.with(Value::clone).borrow())))
                        as isize,
                ),
            ));
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
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
        let num_values: Value<u32> = Rc::new(RefCell::new(((*self.values_.borrow()).len() as u32)));
        if !((*num_values.borrow()) > 0_u32) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"num_bits");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"lehmer_code.cc"), 51, _fn)
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
    pub fn Remove(&self, code: usize, value: Ptr<u8>) -> bool {
        let code: Value<usize> = Rc::new(RefCell::new(code));
        let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
        if ((*code.borrow()) >= (*self.values_.borrow()).len()) {
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
        (items.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
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
                            && value_0.read()
                                == ((*sigma.borrow()).offset((*i.borrow()) as isize).read())
                    })
                    .unwrap_or((items.as_pointer() as Ptr<u32>).to_end().get_offset() as usize)
                    as isize,
            ),
        ));
        if !((*it.borrow()) != (items.as_pointer() as Ptr<u32>).to_end()) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"ComputeLehmerCode");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"lehmer_code.cc"), 21, _fn)
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
        (items.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        let index: Value<u32> = Rc::new(RefCell::new(
            ((*code.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        if (((*index.borrow()) as usize) >= (*items.borrow()).len()) {
            return false;
        }
        let value: Value<u32> = Rc::new(RefCell::new(
            ((items.as_pointer() as Ptr<u32>)
                .offset(((*index.borrow()) as usize) as isize)
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
    pub static kQFactorBits_116: Value<usize> = Rc::new(RefCell::new(6_usize));
);
thread_local!(
    pub static kQFactorLimit_117: Value<usize> = Rc::new(RefCell::new(
        ((1_u32 << (*kQFactorBits_116.with(Value::clone).borrow())) as usize),
    ));
);
pub fn FillQuantMatrix_118(is_chroma: bool, q: u32, dst: Ptr<u8>) {
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let q: Value<u32> = Rc::new(RefCell::new(q));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    if !(((*q.borrow()) >= 0_u32)
        && (((*q.borrow()) as usize) < (*kQFactorLimit_117.with(Value::clone).borrow())))
    {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"FillQuantMatrix");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"quant_matrix.cc"), 18, _fn)
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
    let kMaxDiffCost: Value<usize> = Rc::new(RefCell::new(33_usize));
    let kWorstLen: Value<usize> = Rc::new(RefCell::new(
        (((*kDCTBlockSize_3.with(Value::clone).borrow()) + 1) as usize)
            .wrapping_mul((((*kMaxDiffCost.borrow()).wrapping_add(1_usize)) as usize)),
    ));
    let best_len: Value<usize> = Rc::new(RefCell::new((*kWorstLen.borrow())));
    let q: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*q.borrow()) as usize) < (*kQFactorLimit_117.with(Value::clone).borrow())) {
        ({
            let _is_chroma: bool = (*is_chroma.borrow());
            let _q: u32 = (*q.borrow());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            FillQuantMatrix_118(_is_chroma, _q, _dst)
        });
        let last_diff: Value<i32> = Rc::new(RefCell::new(0));
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
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
                let rhs_0 = (*len.borrow()).wrapping_add(1_usize);
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
                        ((((2_u32).wrapping_mul((*diff_len.borrow()))).wrapping_add(1_u32))
                            as usize),
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
            (*(*(*storage.borrow()).upgrade().deref()).pos.borrow()),
        );
    }
    if !(((*bits.borrow()) >> (*n_bits.borrow())) == 0_u64) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"WriteBits");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"ans_encode.cc"), 58, _fn)
        });
        'loop_: while true {}
    };
    if !((*n_bits.borrow()) <= 56_usize) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"WriteBits");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"ans_encode.cc"), 59, _fn)
        });
        'loop_: while true {}
    };
    if !({
        let _lhs = (((*(*(*storage.borrow()).upgrade().deref()).pos.borrow())
            .wrapping_add((*n_bits.borrow())))
            >> 3)
            .wrapping_add(7_usize);
        _lhs < (*(*(*storage.borrow()).upgrade().deref()).length.borrow())
    }) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"WriteBits");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"ans_encode.cc"), 61, _fn)
        });
        'loop_: while true {}
    };
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*(*(*storage.borrow()).upgrade().deref()).data.borrow())
            .offset(((*(*(*storage.borrow()).upgrade().deref()).pos.borrow()) >> 3) as isize),
    ));
    let v: Value<u64> = Rc::new(RefCell::new((((*p.borrow()).read()) as u64)));
    (*v.borrow_mut()) |= {
        let _lhs = (*bits.borrow());
        _lhs << ((*(*(*storage.borrow()).upgrade().deref()).pos.borrow()) & 7_usize)
    };
    ({
        let _p: AnyPtr = ((*p.borrow()).clone() as Ptr<u8>).to_any();
        let _v: u64 = (*v.borrow());
        BrunsliUnalignedWrite64_70(_p, _v)
    });
    let rhs_0 =
        (*(*(*storage.borrow()).upgrade().deref()).pos.borrow()).wrapping_add((*n_bits.borrow()));
    (*(*(*storage.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
}
#[derive(Default)]
pub struct brunsli_ANSEncSymbolInfo {
    pub freq_: Value<u16>,
    pub start_: Value<u16>,
}
impl Clone for brunsli_ANSEncSymbolInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            freq_: Rc::new(RefCell::new((*self.freq_.borrow()))),
            start_: Rc::new(RefCell::new((*self.start_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_ANSEncSymbolInfo {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.freq_.borrow()).to_bytes(&mut buf[0..2]);
        (*self.start_.borrow()).to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            freq_: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            start_: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
        }
    }
}
#[derive()]
pub struct brunsli_ANSTable {
    pub info_: Value<Box<[brunsli_ANSEncSymbolInfo]>>,
}
impl Clone for brunsli_ANSTable {
    fn clone(&self) -> Self {
        let mut this = Self {
            info_: Rc::new(RefCell::new((*self.info_.borrow()).clone())),
        };
        this
    }
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
impl ByteRepr for brunsli_ANSTable {}
#[derive()]
pub struct brunsli_ANSCoder {
    state_: Value<u32>,
}
impl brunsli_ANSCoder {
    pub fn brunsli_ANSCoder() -> Self {
        let mut this = Self {
            state_: Rc::new(RefCell::new((19_u32 << 16))),
        };
        this
    }
    pub fn PutSymbol(&self, t: brunsli_ANSEncSymbolInfo, nbits: Ptr<u8>) -> u32 {
        let t: Value<brunsli_ANSEncSymbolInfo> = Rc::new(RefCell::new(t));
        let nbits: Value<Ptr<u8>> = Rc::new(RefCell::new(nbits));
        let bits: Value<u32> = Rc::new(RefCell::new(0_u32));
        (*nbits.borrow()).write(0_u8);
        if (((*self.state_.borrow())
            >> (32 - (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow())))
            >= ((*(*t.borrow()).freq_.borrow()) as u32))
        {
            (*bits.borrow_mut()) = ((*self.state_.borrow()) & 65535_u32);
            (*self.state_.borrow_mut()) >>= 16;
            (*nbits.borrow()).write(16_u8);
        }
        let __rhs = ((((*self.state_.borrow())
            .wrapping_div(((*(*t.borrow()).freq_.borrow()) as u32)))
            << (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()))
        .wrapping_add(
            ((*self.state_.borrow()).wrapping_rem(((*(*t.borrow()).freq_.borrow()) as u32))),
        ))
        .wrapping_add(((*(*t.borrow()).start_.borrow()) as u32));
        (*self.state_.borrow_mut()) = __rhs;
        return (*bits.borrow());
    }
    pub fn GetState(&self) -> u32 {
        return (*self.state_.borrow());
    }
}
impl Clone for brunsli_ANSCoder {
    fn clone(&self) -> Self {
        let mut this = Self {
            state_: Rc::new(RefCell::new((*self.state_.borrow()))),
        };
        this
    }
}
impl Default for brunsli_ANSCoder {
    fn default() -> Self {
        {
            brunsli_ANSCoder::brunsli_ANSCoder()
        }
    }
}
impl ByteRepr for brunsli_ANSCoder {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.state_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            state_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
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
            (((*counts.borrow()).offset((*s.borrow()) as isize).read()) as u32),
        ));
        let __rhs = (((*counts.borrow()).offset((*s.borrow()) as isize).read()) as u16);
        (*(*(*info.borrow())
            .offset((*s.borrow()) as isize)
            .upgrade()
            .deref())
        .freq_
        .borrow_mut()) = __rhs;
        (*(*(*info.borrow())
            .offset((*s.borrow()) as isize)
            .upgrade()
            .deref())
        .start_
        .borrow_mut()) = ((*total.borrow()) as u16);
        let rhs_0 = (((*total.borrow()) as u32).wrapping_add((*freq.borrow()))) as i32;
        (*total.borrow_mut()) = rhs_0;
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
    let num_symbols: Value<i32> = <Value<i32>>::default();
    let symbols: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ])));
    let counts: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = (*histogram.borrow()).offset((18) as isize).get_offset()
            - (*histogram.borrow()).get_offset();
        PtrValueIter::new(&(*histogram.borrow()), __count).collect::<Vec<_>>()
    }));
    let omit_pos: Value<i32> = Rc::new(RefCell::new(0));
    ({
        let _counts: Ptr<i32> = ((counts.as_pointer() as Ptr<i32>).offset(0_usize as isize));
        let _omit_pos: Ptr<i32> = (omit_pos.as_pointer());
        let _precision_bits: i32 = (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow());
        let _num_symbols: Ptr<i32> = (num_symbols.as_pointer());
        let _symbols: Ptr<i32> = (symbols.as_pointer() as Ptr<i32>);
        NormalizeCounts_124(
            _counts,
            _omit_pos,
            18,
            _precision_bits,
            _num_symbols,
            _symbols,
        )
    });
    ({
        let _counts: Ptr<i32> = ((counts.as_pointer() as Ptr<i32>).offset(0_usize as isize));
        let _info: Ptr<brunsli_ANSEncSymbolInfo> =
            ((*(*table.borrow()).upgrade().deref()).info_.as_pointer()
                as Ptr<brunsli_ANSEncSymbolInfo>);
        ANSBuildInfoTable_122(_counts, 18, _info)
    });
    ({
        let _counts: Ptr<i32> = ((counts.as_pointer() as Ptr<i32>).offset(0_usize as isize));
        let _omit_pos: i32 = (*omit_pos.borrow());
        let _num_symbols: i32 = (*num_symbols.borrow());
        let _symbols: Ptr<i32> = (symbols.as_pointer() as Ptr<i32>);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        EncodeCounts_125(_counts, _omit_pos, _num_symbols, _symbols, _storage)
    });
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
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
        return ((*kLog2Table_126.with(Value::clone).borrow())[(*v.borrow()) as usize] as f64);
    }
    return ((*v.borrow()) as f64).log2();
}
#[derive(Default)]
pub struct brunsli_HistogramPair {
    pub idx1: Value<usize>,
    pub idx2: Value<usize>,
    pub cost_combo: Value<f64>,
    pub cost_diff: Value<f64>,
}
impl Clone for brunsli_HistogramPair {
    fn clone(&self) -> Self {
        let mut this = Self {
            idx1: Rc::new(RefCell::new((*self.idx1.borrow()))),
            idx2: Rc::new(RefCell::new((*self.idx2.borrow()))),
            cost_combo: Rc::new(RefCell::new((*self.cost_combo.borrow()))),
            cost_diff: Rc::new(RefCell::new((*self.cost_diff.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_HistogramPair {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.idx1.borrow()).to_bytes(&mut buf[0..8]);
        (*self.idx2.borrow()).to_bytes(&mut buf[8..16]);
        (*self.cost_combo.borrow()).to_bytes(&mut buf[16..24]);
        (*self.cost_diff.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            idx1: Rc::new(RefCell::new(<usize>::from_bytes(&buf[0..8]))),
            idx2: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            cost_combo: Rc::new(RefCell::new(<f64>::from_bytes(&buf[16..24]))),
            cost_diff: Rc::new(RefCell::new(<f64>::from_bytes(&buf[24..32]))),
        }
    }
}
pub fn lt(p1: Ptr<brunsli_HistogramPair>, p2: Ptr<brunsli_HistogramPair>) -> bool {
    if {
        let _lhs = (*(*p1.upgrade().deref()).cost_diff.borrow());
        _lhs != (*(*p2.upgrade().deref()).cost_diff.borrow())
    } {
        return {
            let _lhs = (*(*p1.upgrade().deref()).cost_diff.borrow());
            _lhs > (*(*p2.upgrade().deref()).cost_diff.borrow())
        };
    }
    if !({
        let _lhs = (*(*p1.upgrade().deref()).idx1.borrow());
        _lhs < (*(*p1.upgrade().deref()).idx2.borrow())
    }) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"operator<");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 35, _fn)
        });
        'loop_: while true {}
    };
    if !({
        let _lhs = (*(*p2.upgrade().deref()).idx1.borrow());
        _lhs < (*(*p2.upgrade().deref()).idx2.borrow())
    }) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"operator<");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 36, _fn)
        });
        'loop_: while true {}
    };
    return {
        let _lhs = ((*(*p1.upgrade().deref()).idx2.borrow())
            .wrapping_sub((*(*p1.upgrade().deref()).idx1.borrow())));
        _lhs > ((*(*p2.upgrade().deref()).idx2.borrow())
            .wrapping_sub((*(*p2.upgrade().deref()).idx1.borrow())))
    };
}
impl Ord for brunsli_HistogramPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if lt(
                Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if lt(
                Rc::new(RefCell::new(other.clone())).as_pointer(),
                Rc::new(RefCell::new(self.clone())).as_pointer(),
            ) {
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
        {
            !(lt(
                Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )) && !(lt(
                Rc::new(RefCell::new(other.clone())).as_pointer(),
                Rc::new(RefCell::new(self.clone())).as_pointer(),
            ))
        }
    }
}
impl Eq for brunsli_HistogramPair {}
pub fn ClusterCostDiff_128(size_a: i32, size_b: i32) -> f64 {
    let size_a: Value<i32> = Rc::new(RefCell::new(size_a));
    let size_b: Value<i32> = Rc::new(RefCell::new(size_b));
    let size_c: Value<i32> = Rc::new(RefCell::new(((*size_a.borrow()) + (*size_b.borrow()))));
    return (((((*size_a.borrow()) as f64)
        * ({
            let _v: i32 = (*size_a.borrow());
            FastLog2_127(_v)
        }))
        + (((*size_b.borrow()) as f64)
            * ({
                let _v: i32 = (*size_b.borrow());
                FastLog2_127(_v)
            })))
        - (((*size_c.borrow()) as f64)
            * ({
                let _v: i32 = (*size_c.borrow());
                FastLog2_127(_v)
            })));
}
pub fn PopulationCost_129(h: Ptr<brunsli_internal_enc_Histogram>) -> f64 {
    return ({
        let _data: Ptr<i32> =
            (((*h.upgrade().deref()).data_.as_pointer() as Ptr<i32>).offset(0 as isize));
        let _total_count: i32 = (*(*h.upgrade().deref()).total_count_.borrow());
        PopulationCost_130(_data, _total_count)
    });
}
pub fn CompareAndPushToQueue_131(
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
    (*(*p.borrow()).idx1.borrow_mut()) = ((*idx1.borrow()) as usize);
    (*(*p.borrow()).idx2.borrow_mut()) = ((*idx2.borrow()) as usize);
    (*(*p.borrow()).cost_diff.borrow_mut()) = (5.0E-1
        * ({
            let _size_a: i32 = ((*cluster_size.borrow())
                .offset((*idx1.borrow()) as isize)
                .read());
            let _size_b: i32 = ((*cluster_size.borrow())
                .offset((*idx2.borrow()) as isize)
                .read());
            ClusterCostDiff_128(_size_a, _size_b)
        }));
    (*(*p.borrow()).cost_diff.borrow_mut()) -= (*(*(*out.borrow())
        .offset((*idx1.borrow()) as isize)
        .upgrade()
        .deref())
    .bit_cost_
    .borrow());
    (*(*p.borrow()).cost_diff.borrow_mut()) -= (*(*(*out.borrow())
        .offset((*idx2.borrow()) as isize)
        .upgrade()
        .deref())
    .bit_cost_
    .borrow());
    if ((*(*(*out.borrow())
        .offset((*idx1.borrow()) as isize)
        .upgrade()
        .deref())
    .total_count_
    .borrow())
        == 0)
    {
        (*(*p.borrow()).cost_combo.borrow_mut()) = (*(*(*out.borrow())
            .offset((*idx2.borrow()) as isize)
            .upgrade()
            .deref())
        .bit_cost_
        .borrow());
        (*store_pair.borrow_mut()) = true;
    } else if ((*(*(*out.borrow())
        .offset((*idx2.borrow()) as isize)
        .upgrade()
        .deref())
    .total_count_
    .borrow())
        == 0)
    {
        (*(*p.borrow()).cost_combo.borrow_mut()) = (*(*(*out.borrow())
            .offset((*idx1.borrow()) as isize)
            .upgrade()
            .deref())
        .bit_cost_
        .borrow());
        (*store_pair.borrow_mut()) = true;
    } else {
        let threshold: Value<f64> = Rc::new(RefCell::new(
            if (*(*pairs.borrow()).upgrade().deref()).is_empty() {
                1.0E+99
            } else {
                {
                    let __tmp_0: Value<f64> = Rc::new(RefCell::new(0.0E+0));
                    (if __tmp_0.as_pointer().read()
                        >= (*(((*pairs.borrow()).to_strong().as_pointer())
                            as Ptr<brunsli_HistogramPair>)
                            .offset(0_usize as isize)
                            .upgrade()
                            .deref())
                        .cost_diff
                        .as_pointer()
                        .read()
                    {
                        __tmp_0.as_pointer()
                    } else {
                        (*(((*pairs.borrow()).to_strong().as_pointer())
                            as Ptr<brunsli_HistogramPair>)
                            .offset(0_usize as isize)
                            .upgrade()
                            .deref())
                        .cost_diff
                        .as_pointer()
                    }
                    .read())
                }
            },
        ));
        let combo: Value<brunsli_internal_enc_Histogram> = Rc::new(RefCell::new(
            (*(*out.borrow())
                .offset((*idx1.borrow()) as isize)
                .upgrade()
                .deref())
            .clone(),
        ));
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*out.borrow()).offset((*idx2.borrow()) as isize);
            (*combo.borrow()).AddHistogram(_other)
        });
        let cost_combo: Value<f64> = Rc::new(RefCell::new(
            ({
                let _h: Ptr<brunsli_internal_enc_Histogram> = combo.as_pointer();
                PopulationCost_129(_h)
            }),
        ));
        if ((*cost_combo.borrow()) < ((*threshold.borrow()) - (*(*p.borrow()).cost_diff.borrow())))
        {
            (*(*p.borrow()).cost_combo.borrow_mut()) = (*cost_combo.borrow());
            (*store_pair.borrow_mut()) = true;
        }
    }
    if (*store_pair.borrow()) {
        let __rhs = (*(*p.borrow()).cost_combo.borrow());
        (*(*p.borrow()).cost_diff.borrow_mut()) += __rhs;
        if (!(*(*pairs.borrow()).upgrade().deref()).is_empty())
            && (lt(
                ((*pairs.borrow()).to_strong().as_pointer() as Ptr<brunsli_HistogramPair>),
                p.as_pointer(),
            ))
        {
            {
                let a0_clone = (*((*pairs.borrow()).to_strong().as_pointer()
                    as Ptr<brunsli_HistogramPair>)
                    .upgrade()
                    .deref())
                .clone();
                (*pairs.borrow())
                    .with_mut(|__v: &mut Vec<brunsli_HistogramPair>| __v.push(a0_clone))
            };
            ((*pairs.borrow()).to_strong().as_pointer() as Ptr<brunsli_HistogramPair>)
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
pub fn HistogramCombine_132(
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
                let mut write_ptr = (clusters.as_pointer() as Ptr<u64>).clone();
                let mut iter = PtrValueIter::new(&(clusters.as_pointer() as Ptr<u64>), count);
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
                let _out: Ptr<brunsli_internal_enc_Histogram> = (*out.borrow()).clone();
                let _cluster_size: Ptr<i32> = (*cluster_size.borrow()).clone();
                let _idx1: i32 = (((clusters.as_pointer() as Ptr<u64>)
                    .offset((*idx1.borrow()) as isize)
                    .read()) as i32);
                let _idx2: i32 = (((clusters.as_pointer() as Ptr<u64>)
                    .offset((*idx2.borrow()) as isize)
                    .read()) as i32);
                let _pairs: Ptr<Vec<brunsli_HistogramPair>> = (pairs.as_pointer());
                CompareAndPushToQueue_131(_out, _cluster_size, _idx1, _idx2, _pairs)
            });
            (*idx2.borrow_mut()).prefix_inc();
        }
        (*idx1.borrow_mut()).prefix_inc();
    }
    'loop_: while ((*clusters.borrow()).len() > (*min_cluster_size.borrow())) {
        if ((*(*(pairs.as_pointer() as Ptr<brunsli_HistogramPair>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .cost_diff
        .borrow())
            >= (*cost_diff_threshold.borrow()))
        {
            (*cost_diff_threshold.borrow_mut()) = 1.0E+99;
            (*min_cluster_size.borrow_mut()) = (*max_clusters.borrow());
            continue 'loop_;
        }
        let best_idx1: Value<usize> = Rc::new(RefCell::new(
            (*(*(pairs.as_pointer() as Ptr<brunsli_HistogramPair>)
                .offset(0_usize as isize)
                .upgrade()
                .deref())
            .idx1
            .borrow()),
        ));
        let best_idx2: Value<usize> = Rc::new(RefCell::new(
            (*(*(pairs.as_pointer() as Ptr<brunsli_HistogramPair>)
                .offset(0_usize as isize)
                .upgrade()
                .deref())
            .idx2
            .borrow()),
        ));
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*out.borrow()).offset((*best_idx2.borrow()) as isize);
            (*(*out.borrow())
                .offset((*best_idx1.borrow()) as isize)
                .upgrade()
                .deref())
            .AddHistogram(_other)
        });
        (*(*(*out.borrow())
            .offset((*best_idx1.borrow()) as isize)
            .upgrade()
            .deref())
        .bit_cost_
        .borrow_mut()) = (*(*(pairs.as_pointer() as Ptr<brunsli_HistogramPair>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .cost_combo
        .borrow());
        let __rhs = ((*cluster_size.borrow())
            .offset((*best_idx2.borrow()) as isize)
            .read());
        {
            let _ptr = (*cluster_size.borrow())
                .offset((*best_idx1.borrow()) as isize)
                .clone();
            _ptr.write(_ptr.read() + __rhs)
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*symbols_size.borrow())) {
            if {
                let _lhs = (((*symbols.borrow()).offset((*i.borrow()) as isize).read()) as usize);
                _lhs == (*best_idx2.borrow())
            } {
                let __rhs = ((*best_idx1.borrow()) as u32);
                (*symbols.borrow())
                    .offset((*i.borrow()) as isize)
                    .write(__rhs);
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let cluster: Value<Ptr<u64>> = Rc::new(RefCell::new((clusters.as_pointer() as Ptr<u64>)));
        'loop_: while (*cluster.borrow()) != (clusters.as_pointer() as Ptr<u64>).to_end() {
            if ((((*cluster.borrow()).read()) as usize) >= (*best_idx2.borrow())) {
                {
                    let idx = (*cluster.borrow()).clone().get_offset();
                    (clusters.as_pointer() as Ptr<Vec<u64>>)
                        .with_mut(|__v: &mut Vec<u64>| __v.remove(idx));
                    (clusters.as_pointer() as Ptr<Vec<u64>>)
                        .to_strong()
                        .as_pointer() as Ptr<u64>
                };
                break;
            }
            (*cluster.borrow_mut()).prefix_inc();
        }
        let copy_to: Value<Ptr<brunsli_HistogramPair>> = Rc::new(RefCell::new(
            (pairs.as_pointer() as Ptr<brunsli_HistogramPair>),
        ));
        'loop_: for mut p in pairs.as_pointer() as Ptr<brunsli_HistogramPair> {
            if ((({
                let _lhs = (*(*p.upgrade().deref()).idx1.borrow());
                _lhs == (*best_idx1.borrow())
            }) || ({
                let _lhs = (*(*p.upgrade().deref()).idx2.borrow());
                _lhs == (*best_idx1.borrow())
            })) || ({
                let _lhs = (*(*p.upgrade().deref()).idx1.borrow());
                _lhs == (*best_idx2.borrow())
            })) || ({
                let _lhs = (*(*p.upgrade().deref()).idx2.borrow());
                _lhs == (*best_idx2.borrow())
            }) {
                continue 'loop_;
            }
            if lt(
                (pairs.as_pointer() as Ptr<brunsli_HistogramPair>),
                (p).clone(),
            ) {
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
                let _out: Ptr<brunsli_internal_enc_Histogram> = (*out.borrow()).clone();
                let _cluster_size: Ptr<i32> = (*cluster_size.borrow()).clone();
                let _idx1: i32 = ((*best_idx1.borrow()) as i32);
                let _idx2: i32 = (((clusters.as_pointer() as Ptr<u64>)
                    .offset((*i.borrow()) as isize)
                    .read()) as i32);
                let _pairs: Ptr<Vec<brunsli_HistogramPair>> = (pairs.as_pointer());
                CompareAndPushToQueue_131(_out, _cluster_size, _idx1, _idx2, _pairs)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    return (*clusters.borrow()).len();
}
pub fn HistogramBitCostDistance_133(
    histogram: Ptr<brunsli_internal_enc_Histogram>,
    candidate: Ptr<brunsli_internal_enc_Histogram>,
) -> f64 {
    if ((*(*histogram.upgrade().deref()).total_count_.borrow()) == 0) {
        return 0.0E+0;
    }
    let tmp: Value<brunsli_internal_enc_Histogram> =
        Rc::new(RefCell::new((*histogram.upgrade().deref()).clone()));
    ({
        let _other: Ptr<brunsli_internal_enc_Histogram> = (candidate).clone();
        (*tmp.borrow()).AddHistogram(_other)
    });
    return {
        let _lhs = ({
            let _h: Ptr<brunsli_internal_enc_Histogram> = tmp.as_pointer();
            PopulationCost_129(_h)
        });
        _lhs - (*(*candidate.upgrade().deref()).bit_cost_.borrow())
    };
}
pub fn HistogramRemap_134(
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
                let mut write_ptr = (all_symbols.as_pointer() as Ptr<i32>).clone();
                let mut iter = PtrValueIter::new(&(all_symbols.as_pointer() as Ptr<i32>), count);
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
                ((*symbols.borrow()).offset((0) as isize).read())
            } else {
                ((*symbols.borrow())
                    .offset(((*i.borrow()).wrapping_sub(1_usize)) as isize)
                    .read())
            } as i32),
        ));
        let best_bits: Value<f64> = Rc::new(RefCell::new(
            ({
                let _histogram: Ptr<brunsli_internal_enc_Histogram> =
                    (*in_.borrow()).offset((*i.borrow()) as isize);
                let _candidate: Ptr<brunsli_internal_enc_Histogram> =
                    (*out.borrow()).offset((*best_out.borrow()) as isize);
                HistogramBitCostDistance_133(_histogram, _candidate)
            }),
        ));
        'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
            let k: Value<i32> = Rc::new(RefCell::new(k.read().clone()));
            let cur_bits: Value<f64> = Rc::new(RefCell::new(
                ({
                    let _histogram: Ptr<brunsli_internal_enc_Histogram> =
                        (*in_.borrow()).offset((*i.borrow()) as isize);
                    let _candidate: Ptr<brunsli_internal_enc_Histogram> =
                        (*out.borrow()).offset((*k.borrow()) as isize);
                    HistogramBitCostDistance_133(_histogram, _candidate)
                }),
            ));
            if ((*cur_bits.borrow()) < (*best_bits.borrow())) {
                (*best_bits.borrow_mut()) = (*cur_bits.borrow());
                (*best_out.borrow_mut()) = (*k.borrow());
            }
        }
        let __rhs = ((*best_out.borrow()) as u32);
        (*symbols.borrow())
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    'loop_: for mut k in all_symbols.as_pointer() as Ptr<i32> {
        let k: Value<i32> = Rc::new(RefCell::new(k.read().clone()));
        ({
            (*(*out.borrow())
                .offset((*k.borrow()) as isize)
                .upgrade()
                .deref())
            .Clear()
        });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*in_size.borrow())) {
        ({
            let _other: Ptr<brunsli_internal_enc_Histogram> =
                (*in_.borrow()).offset((*i.borrow()) as isize);
            (*(*out.borrow())
                .offset(((*symbols.borrow()).offset((*i.borrow()) as isize).read()) as isize)
                .upgrade()
                .deref())
            .AddHistogram(_other)
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn HistogramReindex_135(out: Ptr<Vec<brunsli_internal_enc_Histogram>>, symbols: Ptr<Vec<u32>>) {
    let out: Value<Ptr<Vec<brunsli_internal_enc_Histogram>>> = Rc::new(RefCell::new(out));
    let symbols: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(symbols));
    let tmp: Value<Vec<brunsli_internal_enc_Histogram>> =
        Rc::new(RefCell::new((*(*out.borrow()).upgrade().deref()).clone()));
    let new_index: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let next_index: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*symbols.borrow()).upgrade().deref()).len()
    } {
        if RefcountMapIter::find_key(
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
            &(((((*symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read()) as i32),
        ) == RefcountMapIter::end((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>))
        {
            (new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry(
                        (((((*symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                            .offset((*i.borrow()) as isize)
                            .read()) as i32)
                            .clone(),
                    )
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
                })
                .write((*next_index.borrow()));
            let __rhs = (*(tmp.as_pointer() as Ptr<brunsli_internal_enc_Histogram>)
                .offset(
                    (((((*symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                        .offset((*i.borrow()) as isize)
                        .read()) as usize) as isize,
                )
                .upgrade()
                .deref())
            .clone();
            (((*out.borrow()).to_strong().as_pointer()) as Ptr<brunsli_internal_enc_Histogram>)
                .offset(((*next_index.borrow()) as usize) as isize)
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
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*symbols.borrow()).upgrade().deref()).len()
    } {
        let __rhs = (((new_index.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry(
                    (((((*symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                        .offset((*i.borrow()) as isize)
                        .read()) as i32)
                        .clone(),
                )
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
            })
            .read()) as u32);
        (((*symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn ClusterHistograms_136(
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
        let __rhs = (*(in_.to_strong().as_pointer() as Ptr<brunsli_internal_enc_Histogram>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .clone();
        (((*out.borrow()).to_strong().as_pointer()) as Ptr<brunsli_internal_enc_Histogram>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        let __rhs = ({
            let _h: Ptr<brunsli_internal_enc_Histogram> = (in_.to_strong().as_pointer()
                as Ptr<brunsli_internal_enc_Histogram>)
                .offset((*i.borrow()) as isize);
            PopulationCost_129(_h)
        });
        (*(*(((*out.borrow()).to_strong().as_pointer()) as Ptr<brunsli_internal_enc_Histogram>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .bit_cost_
        .borrow_mut()) = __rhs;
        let __rhs = ((*i.borrow()) as u32);
        (((*histogram_symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*num_contexts.borrow()) > 1_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_blocks.borrow())) {
            ({
                let _out: Ptr<brunsli_internal_enc_Histogram> =
                    ((((*out.borrow()).to_strong().as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize as isize));
                let _cluster_size: Ptr<i32> =
                    ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize as isize));
                let _symbols: Ptr<u32> =
                    ((((*histogram_symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                        .offset((*i.borrow()).wrapping_mul((*num_contexts.borrow())) as isize));
                let _symbols_size: usize = (*num_contexts.borrow());
                let _max_clusters: usize = (*max_histograms.borrow());
                HistogramCombine_132(_out, _cluster_size, _symbols, _symbols_size, _max_clusters)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    thread_local!(
        static kMinClustersForHistogramRemap_137: Value<usize> = Rc::new(RefCell::new(24_usize));
    );
    let num_clusters: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*block_group_offsets.borrow()).len() > 1_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*block_group_offsets.borrow()).len()) {
            let offset: Value<usize> = Rc::new(RefCell::new(
                (((block_group_offsets.as_pointer() as Ptr<u64>)
                    .offset((*i.borrow()) as isize)
                    .read())
                .wrapping_mul(((*num_contexts.borrow()) as u64)) as usize),
            ));
            let next_offset: Value<usize> = Rc::new(RefCell::new(
                (if ((*i.borrow()).wrapping_add(1_usize) < (*block_group_offsets.borrow()).len()) {
                    ((block_group_offsets.as_pointer() as Ptr<u64>)
                        .offset((*i.borrow()).wrapping_add(1_usize) as isize)
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
                    let _out: Ptr<brunsli_internal_enc_Histogram> =
                        ((((*out.borrow()).to_strong().as_pointer())
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(0_usize as isize));
                    let _cluster_size: Ptr<i32> =
                        ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize as isize));
                    let _symbols: Ptr<u32> =
                        ((((*histogram_symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                            .offset((*offset.borrow()) as isize));
                    let _symbols_size: usize = (*length.borrow());
                    let _max_clusters: usize = (*max_histograms.borrow());
                    HistogramCombine_132(
                        _out,
                        _cluster_size,
                        _symbols,
                        _symbols_size,
                        _max_clusters,
                    )
                }),
            ));
            if ((*nclusters.borrow()) >= 2_usize)
                && ((*nclusters.borrow())
                    < (*kMinClustersForHistogramRemap_137
                        .with(Value::clone)
                        .borrow()))
            {
                ({
                    let _in: Ptr<brunsli_internal_enc_Histogram> = ((in_.to_strong().as_pointer()
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset((*offset.borrow()) as isize));
                    let _in_size: usize = (*length.borrow());
                    let _out: Ptr<brunsli_internal_enc_Histogram> =
                        ((((*out.borrow()).to_strong().as_pointer())
                            as Ptr<brunsli_internal_enc_Histogram>)
                            .offset(0_usize as isize));
                    let _symbols: Ptr<u32> =
                        ((((*histogram_symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                            .offset((*offset.borrow()) as isize));
                    HistogramRemap_134(_in, _in_size, _out, _symbols)
                });
            }
            let rhs_0 = (*num_clusters.borrow()).wrapping_add((*nclusters.borrow()));
            (*num_clusters.borrow_mut()) = rhs_0;
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if ((*block_group_offsets.borrow()).len() <= 1_usize)
        || ((*num_clusters.borrow()) > (*max_histograms.borrow()))
    {
        (*num_clusters.borrow_mut()) = ({
            let _out: Ptr<brunsli_internal_enc_Histogram> =
                ((((*out.borrow()).to_strong().as_pointer())
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(0_usize as isize));
            let _cluster_size: Ptr<i32> =
                ((cluster_size.as_pointer() as Ptr<i32>).offset(0_usize as isize));
            let _symbols: Ptr<u32> = ((((*histogram_symbols.borrow()).to_strong().as_pointer())
                as Ptr<u32>)
                .offset(0_usize as isize));
            let _symbols_size: usize = (*in_size.borrow());
            let _max_clusters: usize = (*max_histograms.borrow());
            HistogramCombine_132(_out, _cluster_size, _symbols, _symbols_size, _max_clusters)
        });
        if ((*num_clusters.borrow()) >= 2_usize)
            && ((*num_clusters.borrow())
                < (*kMinClustersForHistogramRemap_137
                    .with(Value::clone)
                    .borrow()))
        {
            ({
                let _in: Ptr<brunsli_internal_enc_Histogram> = ((in_.to_strong().as_pointer()
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset(0_usize as isize));
                let _in_size: usize = (*in_size.borrow());
                let _out: Ptr<brunsli_internal_enc_Histogram> =
                    ((((*out.borrow()).to_strong().as_pointer())
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset(0_usize as isize));
                let _symbols: Ptr<u32> =
                    ((((*histogram_symbols.borrow()).to_strong().as_pointer()) as Ptr<u32>)
                        .offset(0_usize as isize));
                HistogramRemap_134(_in, _in_size, _out, _symbols)
            });
        }
    }
    ({
        let _out: Ptr<Vec<brunsli_internal_enc_Histogram>> = (*out.borrow()).clone();
        let _symbols: Ptr<Vec<u32>> = (*histogram_symbols.borrow()).clone();
        HistogramReindex_135(_out, _symbols)
    });
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
#[derive()]
pub struct brunsli_internal_enc_ComponentMeta {
    pub context_offset: Value<usize>,
    pub approx_total_nonzeros: Value<usize>,
    pub h_samp: Value<i32>,
    pub v_samp: Value<i32>,
    pub context_bits: Value<i32>,
    pub ac_stride: Value<i32>,
    pub dc_stride: Value<i32>,
    pub b_stride: Value<i32>,
    pub width_in_blocks: Value<i32>,
    pub height_in_blocks: Value<i32>,
    pub ac_coeffs: Value<Ptr<i16>>,
    pub dc_prediction_errors: Value<Ptr<i16>>,
    pub block_state: Value<Ptr<u8>>,
    pub num_zeros: Value<Vec<i32>>,
    pub quant: Value<Vec<i32>>,
}
impl Clone for brunsli_internal_enc_ComponentMeta {
    fn clone(&self) -> Self {
        let mut this = Self {
            context_offset: Rc::new(RefCell::new((*self.context_offset.borrow()))),
            approx_total_nonzeros: Rc::new(RefCell::new((*self.approx_total_nonzeros.borrow()))),
            h_samp: Rc::new(RefCell::new((*self.h_samp.borrow()))),
            v_samp: Rc::new(RefCell::new((*self.v_samp.borrow()))),
            context_bits: Rc::new(RefCell::new((*self.context_bits.borrow()))),
            ac_stride: Rc::new(RefCell::new((*self.ac_stride.borrow()))),
            dc_stride: Rc::new(RefCell::new((*self.dc_stride.borrow()))),
            b_stride: Rc::new(RefCell::new((*self.b_stride.borrow()))),
            width_in_blocks: Rc::new(RefCell::new((*self.width_in_blocks.borrow()))),
            height_in_blocks: Rc::new(RefCell::new((*self.height_in_blocks.borrow()))),
            ac_coeffs: Rc::new(RefCell::new((*self.ac_coeffs.borrow()).clone())),
            dc_prediction_errors: Rc::new(RefCell::new(
                (*self.dc_prediction_errors.borrow()).clone(),
            )),
            block_state: Rc::new(RefCell::new((*self.block_state.borrow()).clone())),
            num_zeros: Rc::new(RefCell::new((*self.num_zeros.borrow()).clone())),
            quant: Rc::new(RefCell::new((*self.quant.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_internal_enc_ComponentMeta {
    fn default() -> Self {
        brunsli_internal_enc_ComponentMeta {
            context_offset: Rc::new(RefCell::new(0_usize)),
            approx_total_nonzeros: Rc::new(RefCell::new(0_usize)),
            h_samp: <Value<i32>>::default(),
            v_samp: <Value<i32>>::default(),
            context_bits: <Value<i32>>::default(),
            ac_stride: <Value<i32>>::default(),
            dc_stride: <Value<i32>>::default(),
            b_stride: <Value<i32>>::default(),
            width_in_blocks: <Value<i32>>::default(),
            height_in_blocks: <Value<i32>>::default(),
            ac_coeffs: Rc::new(RefCell::new(Ptr::<i16>::null())),
            dc_prediction_errors: Rc::new(RefCell::new(Ptr::<i16>::null())),
            block_state: Rc::new(RefCell::new(Ptr::<u8>::null())),
            num_zeros: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
            quant: Rc::new(RefCell::new(
                std::array::from_fn::<_, 64, _>(|_| Default::default()).to_vec(),
            )),
        }
    }
}
impl ByteRepr for brunsli_internal_enc_ComponentMeta {}
#[derive()]
pub struct brunsli_internal_enc_Histogram {
    pub data_: Value<Box<[i32]>>,
    pub total_count_: Value<i32>,
    pub bit_cost_: Value<f64>,
}
impl brunsli_internal_enc_Histogram {
    pub fn brunsli_internal_enc_Histogram() -> Self {
        let mut this = Self {
            data_: Rc::new(RefCell::new(
                (0..18).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            total_count_: <Value<i32>>::default(),
            bit_cost_: <Value<f64>>::default(),
        };
        ({ this.Clear() });
        this
    }
}
impl Clone for brunsli_internal_enc_Histogram {
    fn clone(&self) -> Self {
        let mut this = Self {
            data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
            total_count_: Rc::new(RefCell::new((*self.total_count_.borrow()))),
            bit_cost_: Rc::new(RefCell::new((*self.bit_cost_.borrow()))),
        };
        this
    }
}
impl Default for brunsli_internal_enc_Histogram {
    fn default() -> Self {
        {
            brunsli_internal_enc_Histogram::brunsli_internal_enc_Histogram()
        }
    }
}
impl ByteRepr for brunsli_internal_enc_Histogram {}
thread_local!(
    static kMaxNumberOfHistograms_138: Value<usize> = Rc::new(RefCell::new(256_usize));
);
#[derive(Default)]
pub struct brunsli_internal_enc_EntropyCodes {
    clustered_: Value<Vec<brunsli_internal_enc_Histogram>>,
    context_map_: Value<Vec<u32>>,
    ans_tables_: Value<Vec<brunsli_ANSTable>>,
}
impl brunsli_internal_enc_EntropyCodes {
    pub fn brunsli_internal_enc_EntropyCodes(
        histograms: Ptr<Vec<brunsli_internal_enc_Histogram>>,
        num_bands: usize,
        offsets: Ptr<Vec<u64>>,
    ) -> Self {
        let num_bands: Value<usize> = Rc::new(RefCell::new(num_bands));
        let mut this = Self {
            clustered_: Rc::new(RefCell::new(Vec::new())),
            context_map_: Rc::new(RefCell::new(Vec::new())),
            ans_tables_: Rc::new(RefCell::new(Vec::new())),
        };
        ({
            let _in: Ptr<Vec<brunsli_internal_enc_Histogram>> = (histograms).clone();
            let _num_contexts: usize = (*kNumAvrgContexts_83.with(Value::clone).borrow());
            let _num_blocks: usize = (*num_bands.borrow());
            let _block_group_offsets: Vec<u64> = (*offsets.upgrade().deref()).clone();
            let _max_histograms: usize = (*kMaxNumberOfHistograms_138.with(Value::clone).borrow());
            let _out: Ptr<Vec<brunsli_internal_enc_Histogram>> = (this.clustered_.as_pointer());
            let _histogram_symbols: Ptr<Vec<u32>> = (this.context_map_.as_pointer());
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
impl Clone for brunsli_internal_enc_EntropyCodes {
    fn clone(&self) -> Self {
        let mut this = Self {
            clustered_: Rc::new(RefCell::new((*self.clustered_.borrow()).clone())),
            context_map_: Rc::new(RefCell::new((*self.context_map_.borrow()).clone())),
            ans_tables_: Rc::new(RefCell::new((*self.ans_tables_.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_enc_EntropyCodes {}
#[derive()]
pub struct brunsli_internal_enc_EntropySource {
    num_bands_: Value<usize>,
    histograms_: Value<Vec<brunsli_internal_enc_Histogram>>,
}
impl brunsli_internal_enc_EntropySource {
    pub fn brunsli_internal_enc_EntropySource() -> Self {
        let mut this = Self {
            num_bands_: Rc::new(RefCell::new(0_usize)),
            histograms_: Rc::new(RefCell::new(Vec::new())),
        };
        this
    }
}
impl Clone for brunsli_internal_enc_EntropySource {
    fn clone(&self) -> Self {
        let mut this = Self {
            num_bands_: Rc::new(RefCell::new((*self.num_bands_.borrow()))),
            histograms_: Rc::new(RefCell::new((*self.histograms_.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_internal_enc_EntropySource {
    fn default() -> Self {
        {
            brunsli_internal_enc_EntropySource::brunsli_internal_enc_EntropySource()
        }
    }
}
impl ByteRepr for brunsli_internal_enc_EntropySource {}
thread_local!(
    static kSlackForOneBlock_139: Value<usize> = Rc::new(RefCell::new(1024_usize));
);
#[derive()]
struct brunsli_internal_enc_DataStream_CodeWord {
    pub context: Value<u32>,
    pub value: Value<u16>,
    pub code: Value<u8>,
    pub nbits: Value<u8>,
}
impl brunsli_internal_enc_DataStream_CodeWord {
    pub fn brunsli_internal_enc_DataStream_CodeWord() -> Self {
        let mut this = Self {
            context: <Value<u32>>::default(),
            value: <Value<u16>>::default(),
            code: <Value<u8>>::default(),
            nbits: <Value<u8>>::default(),
        };
        this
    }
}
impl Clone for brunsli_internal_enc_DataStream_CodeWord {
    fn clone(&self) -> Self {
        let mut this = Self {
            context: Rc::new(RefCell::new((*self.context.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
            code: Rc::new(RefCell::new((*self.code.borrow()))),
            nbits: Rc::new(RefCell::new((*self.nbits.borrow()))),
        };
        this
    }
}
impl Default for brunsli_internal_enc_DataStream_CodeWord {
    fn default() -> Self {
        {
            brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord()
        }
    }
}
impl ByteRepr for brunsli_internal_enc_DataStream_CodeWord {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.context.borrow()).to_bytes(&mut buf[0..4]);
        (*self.value.borrow()).to_bytes(&mut buf[4..6]);
        (*self.code.borrow()).to_bytes(&mut buf[6..7]);
        (*self.nbits.borrow()).to_bytes(&mut buf[7..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            context: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            value: Rc::new(RefCell::new(<u16>::from_bytes(&buf[4..6]))),
            code: Rc::new(RefCell::new(<u8>::from_bytes(&buf[6..7]))),
            nbits: Rc::new(RefCell::new(<u8>::from_bytes(&buf[7..8]))),
        }
    }
}
#[derive()]
pub struct brunsli_internal_enc_DataStream {
    pos_: Value<i32>,
    bw_pos_: Value<i32>,
    ac_pos0_: Value<i32>,
    ac_pos1_: Value<i32>,
    low_: Value<u32>,
    high_: Value<u32>,
    bw_val_: Value<u32>,
    bw_bitpos_: Value<i32>,
    code_words_: Value<Vec<brunsli_internal_enc_DataStream_CodeWord>>,
}
impl brunsli_internal_enc_DataStream {
    pub fn brunsli_internal_enc_DataStream() -> Self {
        let mut this = Self {
            pos_: Rc::new(RefCell::new(3)),
            bw_pos_: Rc::new(RefCell::new(0)),
            ac_pos0_: Rc::new(RefCell::new(1)),
            ac_pos1_: Rc::new(RefCell::new(2)),
            low_: Rc::new(RefCell::new(0_u32)),
            high_: Rc::new(RefCell::new((!0 as u32))),
            bw_val_: Rc::new(RefCell::new(0_u32)),
            bw_bitpos_: Rc::new(RefCell::new(0)),
            code_words_: Rc::new(RefCell::new(Vec::new())),
        };
        this
    }
}
impl Clone for brunsli_internal_enc_DataStream {
    fn clone(&self) -> Self {
        let mut this = Self {
            pos_: Rc::new(RefCell::new((*self.pos_.borrow()))),
            bw_pos_: Rc::new(RefCell::new((*self.bw_pos_.borrow()))),
            ac_pos0_: Rc::new(RefCell::new((*self.ac_pos0_.borrow()))),
            ac_pos1_: Rc::new(RefCell::new((*self.ac_pos1_.borrow()))),
            low_: Rc::new(RefCell::new((*self.low_.borrow()))),
            high_: Rc::new(RefCell::new((*self.high_.borrow()))),
            bw_val_: Rc::new(RefCell::new((*self.bw_val_.borrow()))),
            bw_bitpos_: Rc::new(RefCell::new((*self.bw_bitpos_.borrow()))),
            code_words_: Rc::new(RefCell::new((*self.code_words_.borrow()).clone())),
        };
        this
    }
}
impl Default for brunsli_internal_enc_DataStream {
    fn default() -> Self {
        {
            brunsli_internal_enc_DataStream::brunsli_internal_enc_DataStream()
        }
    }
}
impl ByteRepr for brunsli_internal_enc_DataStream {}
#[derive(Default)]
pub struct brunsli_internal_enc_State {
    pub entropy_source: Value<brunsli_internal_enc_EntropySource>,
    pub entropy_codes: Value<Ptr<brunsli_internal_enc_EntropyCodes>>,
    pub data_stream_dc: Value<brunsli_internal_enc_DataStream>,
    pub data_stream_ac: Value<brunsli_internal_enc_DataStream>,
    pub meta: Value<Vec<brunsli_internal_enc_ComponentMeta>>,
    pub num_contexts: Value<usize>,
    pub use_legacy_context_model: Value<bool>,
}
impl Clone for brunsli_internal_enc_State {
    fn clone(&self) -> Self {
        let mut this = Self {
            entropy_source: Rc::new(RefCell::new((*self.entropy_source.borrow()).clone())),
            entropy_codes: Rc::new(RefCell::new((*self.entropy_codes.borrow()).clone())),
            data_stream_dc: Rc::new(RefCell::new((*self.data_stream_dc.borrow()).clone())),
            data_stream_ac: Rc::new(RefCell::new((*self.data_stream_ac.borrow()).clone())),
            meta: Rc::new(RefCell::new((*self.meta.borrow()).clone())),
            num_contexts: Rc::new(RefCell::new((*self.num_contexts.borrow()))),
            use_legacy_context_model: Rc::new(RefCell::new(
                (*self.use_legacy_context_model.borrow()),
            )),
        };
        this
    }
}
impl ByteRepr for brunsli_internal_enc_State {}
thread_local!(
    pub static kNumDirectCodes_140: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kBrotliQuality_141: Value<i32> = Rc::new(RefCell::new(6));
);
thread_local!(
    pub static kBrotliWindowBits_142: Value<i32> = Rc::new(RefCell::new(18));
);
pub fn EstimateAuxDataSize_143(jpg: Ptr<brunsli_JPEGData>) -> usize {
    let size: Value<usize> = Rc::new(RefCell::new(
        (((((*(*jpg.upgrade().deref()).marker_order.borrow()).len()).wrapping_add(
            (272_usize).wrapping_mul((*(*jpg.upgrade().deref()).huffman_code.borrow()).len()),
        ))
        .wrapping_add(
            (7_usize).wrapping_mul((*(*jpg.upgrade().deref()).scan_info.borrow()).len()),
        ))
        .wrapping_add(16_usize)),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).scan_info.borrow()).len()
    } {
        let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
            ((7_usize).wrapping_mul(
                (*(*((*jpg.upgrade().deref()).scan_info.as_pointer()
                    as Ptr<brunsli_JPEGScanInfo>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .reset_points
                .borrow())
                .len(),
            ) as u64),
        )) as usize;
        (*size.borrow_mut()) = rhs_0;
        let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
            ((7_usize).wrapping_mul(
                (*(*((*jpg.upgrade().deref()).scan_info.as_pointer()
                    as Ptr<brunsli_JPEGScanInfo>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .extra_zero_runs
                .borrow())
                .len(),
            ) as u64),
        )) as usize;
        (*size.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    let nsize: Value<usize> = Rc::new(RefCell::new(
        if (*(*jpg.upgrade().deref()).has_zero_padding_bit.borrow()) {
            (*(*jpg.upgrade().deref()).padding_bits.borrow()).len()
        } else {
            0_usize
        },
    ));
    let rhs_0 = (*size.borrow()).wrapping_add((((*nsize.borrow()).wrapping_add(43_usize)) >> 3));
    (*size.borrow_mut()) = rhs_0;
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).inter_marker_data.borrow()).len()
    } {
        let rhs_0 =
            (((*size.borrow()) as u64).wrapping_add(
                ((5_usize).wrapping_add(
                    (*(((*jpg.upgrade().deref()).inter_marker_data.as_pointer()
                        as Ptr<Value<Vec<u8>>>)
                        .offset((*i.borrow()) as isize)
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<u8>>)
                        .upgrade()
                        .deref())
                    .len(),
                ) as u64),
            )) as usize;
        (*size.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*size.borrow());
}
pub fn GetMaximumBrunsliEncodedSize_144(jpg: Ptr<brunsli_JPEGData>) -> usize {
    let hdr_size: Value<usize> = Rc::new(RefCell::new(((1 << 20) as usize)));
    let rhs_0 = (*hdr_size.borrow()).wrapping_add(
        ({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            EstimateAuxDataSize_143(_jpg)
        }),
    );
    (*hdr_size.borrow_mut()) = rhs_0;
    'loop_: for mut data in (*jpg.upgrade().deref()).app_data.as_pointer() as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        let rhs_0 = (((*hdr_size.borrow()) as u64)
            .wrapping_add(((*data.upgrade().deref()).len() as u64))) as usize;
        (*hdr_size.borrow_mut()) = rhs_0;
    }
    'loop_: for mut data in (*jpg.upgrade().deref()).com_data.as_pointer() as Ptr<Value<Vec<u8>>> {
        let data: Ptr<Vec<u8>> = data.upgrade().deref().as_pointer();
        let rhs_0 = (((*hdr_size.borrow()) as u64)
            .wrapping_add(((*data.upgrade().deref()).len() as u64))) as usize;
        (*hdr_size.borrow_mut()) = rhs_0;
    }
    let rhs_0 = (((*hdr_size.borrow()) as u64)
        .wrapping_add(((*(*jpg.upgrade().deref()).tail_data.borrow()).len() as u64)))
        as usize;
    (*hdr_size.borrow_mut()) = rhs_0;
    let num_pixels: Value<usize> = Rc::new(RefCell::new(
        (({
            let _lhs = (*(*jpg.upgrade().deref()).width.borrow());
            _lhs * (*(*jpg.upgrade().deref()).height.borrow())
        }) as usize)
            .wrapping_mul((*(*jpg.upgrade().deref()).components.borrow()).len()),
    ));
    return ((((*num_pixels.borrow()) as f64) * 1.2E+0) as usize)
        .wrapping_add((*hdr_size.borrow()));
}
pub fn Base128Size_145(val: usize) -> usize {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let size: Value<usize> = Rc::new(RefCell::new(1_usize));
    'loop_: while ((*val.borrow()) >= 128_usize) {
        (*size.borrow_mut()).prefix_inc();
        (*val.borrow_mut()) >>= 7;
    }
    return (*size.borrow());
}
pub fn EncodeBase128_146(val: usize, data: Ptr<u8>) -> usize {
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
        (*data.borrow())
            .offset(((*len.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
        (*val.borrow_mut()) >>= 7;
    }
    return (*len.borrow());
}
pub fn EncodeBase128Fix_147(val: usize, len: usize, data: Ptr<u8>) {
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
pub fn TransformApp0Marker_148(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    if ((*s.upgrade().deref()).len() != 17_usize) {
        return false;
    }
    if (((s.to_strong().as_pointer() as Ptr<u8>) as Ptr<u8>)
        .to_any()
        .memcmp(
            &((AppData_0xe0_62.with(Value::clone).as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            9_usize,
        )
        != 0)
    {
        return false;
    }
    if (((((((s.to_strong().as_pointer() as Ptr<u8>)
        .offset(9_usize as isize)
        .read()) as i32)
        == 1)
        || ((((s.to_strong().as_pointer() as Ptr<u8>)
            .offset(9_usize as isize)
            .read()) as i32)
            == 2))
        && ((((s.to_strong().as_pointer() as Ptr<u8>)
            .offset(10_usize as isize)
            .read()) as i32)
            < 4))
        && ((((s.to_strong().as_pointer() as Ptr<u8>)
            .offset(15_usize as isize)
            .read()) as i32)
            == 0))
        && ((((s.to_strong().as_pointer() as Ptr<u8>)
            .offset(16_usize as isize)
            .read()) as i32)
            == 0)
    {
        let x_dens_hi: Value<u8> = Rc::new(RefCell::new(
            ((s.to_strong().as_pointer() as Ptr<u8>)
                .offset(11_usize as isize)
                .read()),
        ));
        let x_dens_lo: Value<u8> = Rc::new(RefCell::new(
            ((s.to_strong().as_pointer() as Ptr<u8>)
                .offset(12_usize as isize)
                .read()),
        ));
        let x_dens: Value<i32> = Rc::new(RefCell::new(
            ((((*x_dens_hi.borrow()) as i32) << 8) + ((*x_dens_lo.borrow()) as i32)),
        ));
        let y_dens_hi: Value<u8> = Rc::new(RefCell::new(
            ((s.to_strong().as_pointer() as Ptr<u8>)
                .offset(13_usize as isize)
                .read()),
        ));
        let y_dens_lo: Value<u8> = Rc::new(RefCell::new(
            ((s.to_strong().as_pointer() as Ptr<u8>)
                .offset(14_usize as isize)
                .read()),
        ));
        let y_dens: Value<i32> = Rc::new(RefCell::new(
            ((((*y_dens_hi.borrow()) as i32) << 8) + ((*y_dens_lo.borrow()) as i32)),
        ));
        let density_ix: Value<i32> = Rc::new(RefCell::new(-1_i32));
        let k: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*k.borrow()) < (*kMaxApp0Densities_45.with(Value::clone).borrow())) {
            if ((*x_dens.borrow())
                == ((*kApp0Densities_46.with(Value::clone).borrow())[(*k.borrow()) as usize]
                    as i32))
                && ((*y_dens.borrow()) == (*x_dens.borrow()))
            {
                (*density_ix.borrow_mut()) = ((*k.borrow()) as i32);
            }
            (*k.borrow_mut()).prefix_inc();
        }
        if ((*density_ix.borrow()) >= 0) {
            let app0_status: Value<u8> = Rc::new(RefCell::new(
                (({
                    let _lhs = (((((s.to_strong().as_pointer() as Ptr<u8>)
                        .offset(9_usize as isize)
                        .read()) as i32)
                        - 1)
                        | ((((s.to_strong().as_pointer() as Ptr<u8>)
                            .offset(10_usize as isize)
                            .read()) as i32)
                            << 1));
                    _lhs | ((*density_ix.borrow()) << 3)
                }) as u8),
            ));
            ((*out.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>).write(
                (0..(1_usize) as usize)
                    .map(|_| <u8>::default())
                    .collect::<Vec<_>>(),
            );
            ((*out.borrow()).to_strong().as_pointer() as Ptr<u8>)
                .offset(0_usize as isize)
                .write((*app0_status.borrow()));
            return true;
        }
    }
    return false;
}
pub fn TransformApp2Marker_149(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    if ( ( ( (*s.upgrade().deref()) .len()  == 3161_usize ) ) && ( ! ( ((s .to_strong().as_pointer()  as Ptr<u8>)   as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xe2_63.with(Value::clone) .as_pointer()  as Ptr::<u8>   )  as Ptr::<u8>  ).to_any() , 84_usize )  != 0 ) ) ) && ( ! ( ((s .to_strong().as_pointer()  as Ptr<u8>)  . offset ( ( 85 ) as isize )  as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xe2_63.with(Value::clone) .as_pointer()  as Ptr::<u8>   ) . offset ( ( 85 ) as isize )  as Ptr::<u8>  ).to_any() , ( ( ( 3161 - 85 ) ) as usize ) )  != 0 ) ) { let  code : Value<Vec<u8> > = Rc::new(RefCell::new((0..(2_usize ) as usize).map(|_| <u8>::default()).collect::<Vec<_>>() )) ;
 ;
 ;
 (code .as_pointer()  as Ptr<u8>).offset(0_usize  as isize) .write( 128_u8  ) ;
 (code .as_pointer()  as Ptr<u8>).offset(1_usize  as isize) .write( (  (s .to_strong().as_pointer()  as Ptr<u8>).offset(84_usize  as isize) .read() )  ) ;
 ((*out.borrow()) .to_strong().as_pointer()  as Ptr<Vec<u8>>).write((*code.borrow()) .clone())  ;
 return true  ;
 }
    return false;
}
pub fn TransformApp12Marker_150(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    if ( ( ( (*s.upgrade().deref()) .len()  == 18_usize ) ) && ( ! ( ((s .to_strong().as_pointer()  as Ptr<u8>)   as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xec_64.with(Value::clone) .as_pointer()  as Ptr::<u8>   )  as Ptr::<u8>  ).to_any() , 15_usize )  != 0 ) ) ) && ( ! ( ((s .to_strong().as_pointer()  as Ptr<u8>)  . offset ( ( 16 ) as isize )  as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xec_64.with(Value::clone) .as_pointer()  as Ptr::<u8>   ) . offset ( ( 16 ) as isize )  as Ptr::<u8>  ).to_any() , ( ( ( 18 - 16 ) ) as usize ) )  != 0 ) ) { let  code : Value<Vec<u8> > = Rc::new(RefCell::new((0..(2_usize ) as usize).map(|_| <u8>::default()).collect::<Vec<_>>() )) ;
 ;
 ;
 (code .as_pointer()  as Ptr<u8>).offset(0_usize  as isize) .write( 129_u8  ) ;
 (code .as_pointer()  as Ptr<u8>).offset(1_usize  as isize) .write( (  (s .to_strong().as_pointer()  as Ptr<u8>).offset(15_usize  as isize) .read() )  ) ;
 ((*out.borrow()) .to_strong().as_pointer()  as Ptr<Vec<u8>>).write((*code.borrow()) .clone())  ;
 return true  ;
 }
    return false;
}
pub fn TransformApp14Marker_151(s: Ptr<Vec<u8>>, out: Ptr<Vec<u8>>) -> bool {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    if ( ( ( (*s.upgrade().deref()) .len()  == 15_usize ) ) && ( ! ( (( (s .to_strong().as_pointer()  as Ptr<u8>).offset(0_usize  as isize)  )  as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xee_65.with(Value::clone) .as_pointer()  as Ptr::<u8>   )  as Ptr::<u8>  ).to_any() , 10_usize )  != 0 ) ) ) && ( ! ( (( (s .to_strong().as_pointer()  as Ptr<u8>).offset(11_usize  as isize)  )  as Ptr::<u8>  ).to_any() .memcmp(&(( AppData_0xee_65.with(Value::clone) .as_pointer()  as Ptr::<u8>   ) . offset ( ( 11 ) as isize )  as Ptr::<u8>  ).to_any() , ( ( ( 15 - 11 ) ) as usize ) )  != 0 ) ) { let  code : Value<Vec<u8> > = Rc::new(RefCell::new((0..(2_usize ) as usize).map(|_| <u8>::default()).collect::<Vec<_>>() )) ;
 ;
 ;
 (code .as_pointer()  as Ptr<u8>).offset(0_usize  as isize) .write( 130_u8  ) ;
 (code .as_pointer()  as Ptr<u8>).offset(1_usize  as isize) .write( (  (s .to_strong().as_pointer()  as Ptr<u8>).offset(10_usize  as isize) .read() )  ) ;
 ((*out.borrow()) .to_strong().as_pointer()  as Ptr<Vec<u8>>).write((*code.borrow()) .clone())  ;
 return true  ;
 }
    return false;
}
pub fn TransformAppMarker_152(s: Ptr<Vec<u8>>, transformed_marker_count: Ptr<usize>) -> Vec<u8> {
    let transformed_marker_count: Value<Ptr<usize>> =
        Rc::new(RefCell::new(transformed_marker_count));
    let out: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp0Marker_148(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return (*out.borrow_mut()).clone();
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp2Marker_149(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return (*out.borrow_mut()).clone();
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp12Marker_150(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return (*out.borrow_mut()).clone();
    }
    if ({
        let _s: Ptr<Vec<u8>> = (s).clone();
        let _out: Ptr<Vec<u8>> = (out.as_pointer());
        TransformApp14Marker_151(_s, _out)
    }) {
        (*transformed_marker_count.borrow()).with_mut(|__v| __v.postfix_inc());
        return (*out.borrow_mut()).clone();
    }
    return (*s.upgrade().deref()).clone();
}
pub fn GetQuantTableId_153(q: Ptr<brunsli_JPEGQuantTable>, is_chroma: bool, dst: Ptr<u8>) -> i32 {
    let is_chroma: Value<bool> = Rc::new(RefCell::new(is_chroma));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let j: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*j.borrow()) < (*kNumStockQuantTables_47.with(Value::clone).borrow())) {
        let match_found: Value<bool> = Rc::new(RefCell::new(true));
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (*match_found.borrow())
            && ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow()))
        {
            if {
                let _lhs = (((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                    .offset(((*k.borrow()) as usize) as isize)
                    .read());
                _lhs != ((*kStockQuantizationTables_48.with(Value::clone).borrow())
                    [(*is_chroma.borrow()) as usize]
                    .borrow()[(*j.borrow()) as usize]
                    .borrow()[(*k.borrow()) as usize] as i32)
            } {
                (*match_found.borrow_mut()) = false;
            }
            (*k.borrow_mut()).prefix_inc();
        }
        if (*match_found.borrow()) {
            return (*j.borrow());
        }
        (*j.borrow_mut()).prefix_inc();
    }
    return ((((*kNumStockQuantTables_47.with(Value::clone).borrow()) as u32).wrapping_add(
        ({
            let _src: Ptr<i32> =
                (((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>).offset(0_usize as isize));
            let _is_chroma: bool = (*is_chroma.borrow());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            FindBestMatrix_119(_src, _is_chroma, _dst)
        }),
    )) as i32);
}
pub fn EncodeVarint_154(n: i32, max_bits: i32, storage: Ptr<brunsli_Storage>) {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let max_bits: Value<i32> = Rc::new(RefCell::new(max_bits));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let b: Value<i32> = <Value<i32>>::default();
    if !((*n.borrow()) < (1 << (*max_bits.borrow()))) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeVarint");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 215, _fn)
        });
        'loop_: while true {}
    };
    (*b.borrow_mut()) = 0;
    'loop_: while ((*n.borrow()) != 0) && ((*b.borrow()) < (*max_bits.borrow())) {
        if (((*b.borrow()) + 1) != (*max_bits.borrow())) {
            ({
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(1_usize, 1_u64, _storage)
            });
        }
        ({
            let _bits: u64 = (((*n.borrow()) & 1) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
        (*n.borrow_mut()) >>= 1;
        (*b.borrow_mut()).prefix_inc();
    }
    if ((*b.borrow()) < (*max_bits.borrow())) {
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 0_u64, _storage)
        });
    }
}
pub fn EncodeLimitedVarint_155(
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
            let _bits: u64 = (((*bits.borrow()) != 0_usize) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
        if ((*bits.borrow()) == 0_usize) {
            break;
        }
        ({
            let _n_bits: usize = ((*nbits.borrow()) as usize);
            let _bits: u64 = (((*bits.borrow()) & (*mask.borrow())) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (*bits.borrow_mut()) >>= (*nbits.borrow());
        (*b.borrow_mut()).prefix_inc();
    }
}
pub fn EncodeQuantTables_156(jpg: Ptr<brunsli_JPEGData>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*(*jpg.upgrade().deref()).quant.borrow()).is_empty())
        || ((*(*jpg.upgrade().deref()).quant.borrow()).len() > 4_usize)
    {
        return false;
    }
    ({
        let _bits: u64 =
            (((*(*jpg.upgrade().deref()).quant.borrow()).len()).wrapping_sub(1_usize) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).quant.borrow()).len()
    } {
        let q: Ptr<brunsli_JPEGQuantTable> = ((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*i.borrow()) as isize);
        let k: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
            let j: Value<i32> = Rc::new(RefCell::new(
                ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize]
                    as i32),
            ));
            if ((((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                .offset(((*j.borrow()) as usize) as isize)
                .read())
                == 0)
            {
                return false;
            }
            (*k.borrow_mut()).prefix_inc();
        }
        let quant_approx: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        let code: Value<i32> = Rc::new(RefCell::new(
            ({
                let _q: Ptr<brunsli_JPEGQuantTable> = (q).clone();
                let _is_chroma: bool = ((*i.borrow()) > 0_usize);
                let _dst: Ptr<u8> = (quant_approx.as_pointer() as Ptr<u8>);
                GetQuantTableId_153(_q, _is_chroma, _dst)
            }),
        ));
        ({
            let _bits: u64 = (((*code.borrow())
                >= (*kNumStockQuantTables_47.with(Value::clone).borrow()))
                as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
        if ((*code.borrow()) < (*kNumStockQuantTables_47.with(Value::clone).borrow())) {
            ({
                let _bits: u64 = ((*code.borrow()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(3_usize, _bits, _storage)
            });
        } else {
            let q_factor: Value<usize> = Rc::new(RefCell::new(
                (((*code.borrow()) - (*kNumStockQuantTables_47.with(Value::clone).borrow()))
                    as usize),
            ));
            if !((*q_factor.borrow()) < (*kQFactorLimit_117.with(Value::clone).borrow())) {
                ({
                    let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeQuantTables");
                    BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 264, _fn)
                });
                'loop_: while true {}
            };
            ({
                let _n_bits: usize = (*kQFactorBits_116.with(Value::clone).borrow());
                let _bits: u64 = ((*q_factor.borrow()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, _bits, _storage)
            });
            let last_diff: Value<i32> = Rc::new(RefCell::new(0));
            let k: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
                let j: Value<i32> = Rc::new(RefCell::new(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize]
                        as i32),
                ));
                let new_diff: Value<i32> = Rc::new(RefCell::new({
                    let _lhs = (((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                        .offset(((*j.borrow()) as usize) as isize)
                        .read());
                    _lhs - ((*quant_approx.borrow())[(*j.borrow()) as usize] as i32)
                }));
                let diff: Value<i32> =
                    Rc::new(RefCell::new(((*new_diff.borrow()) - (*last_diff.borrow()))));
                (*last_diff.borrow_mut()) = (*new_diff.borrow());
                ({
                    let _bits: u64 = (((*diff.borrow()) != 0) as u64);
                    let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                    WriteBits_120(1_usize, _bits, _storage)
                });
                if ((*diff.borrow()) != 0) {
                    ({
                        let _bits: u64 = (((*diff.borrow()) < 0) as u64);
                        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                        WriteBits_120(1_usize, _bits, _storage)
                    });
                    if ((*diff.borrow()) < 0) {
                        let __rhs = -(*diff.borrow());
                        (*diff.borrow_mut()) = __rhs;
                    }
                    (*diff.borrow_mut()) -= 1;
                    if ((*diff.borrow()) > 65535) {
                        return false;
                    }
                    ({
                        let _n: i32 = (*diff.borrow());
                        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                        EncodeVarint_154(_n, 16, _storage)
                    });
                }
                (*k.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).components.borrow()).len()
    } {
        ({
            let _bits: u64 = ((*(*((*jpg.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .quant_idx
            .borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(2_usize, _bits, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeHuffmanCode_157(
    huff: Ptr<brunsli_JPEGHuffmanCode>,
    is_known_last: bool,
    storage: Ptr<brunsli_Storage>,
) -> bool {
    let is_known_last: Value<bool> = Rc::new(RefCell::new(is_known_last));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        let _bits: u64 = (((*(*huff.upgrade().deref()).slot_id.borrow()) & 15) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    ({
        let _bits: u64 = (((*(*huff.upgrade().deref()).slot_id.borrow()) >> 4) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, _bits, _storage)
    });
    if !(*is_known_last.borrow()) {
        ({
            let _bits: u64 = ((*(*huff.upgrade().deref()).is_last.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
    } else if !(*(*huff.upgrade().deref()).is_last.borrow()) {
        return false;
    }
    let is_dc_table: Value<i32> = Rc::new(RefCell::new(
        ((((*(*huff.upgrade().deref()).slot_id.borrow()) >> 4) == 0) as i32),
    ));
    let total_count: Value<i32> = Rc::new(RefCell::new(0));
    let space: Value<i32> = Rc::new(RefCell::new(
        (1 << (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())),
    ));
    let max_len: Value<i32> = Rc::new(RefCell::new(
        (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()),
    ));
    let max_count: Value<i32> = Rc::new(RefCell::new(if ((*is_dc_table.borrow()) != 0) {
        (*kJpegDCAlphabetSize_9.with(Value::clone).borrow())
    } else {
        (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow())
    }));
    let found_match: Value<i32> = Rc::new(RefCell::new(0));
    let stock_table_idx: Value<i32> = Rc::new(RefCell::new(0));
    if ((*is_dc_table.borrow()) != 0) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*kNumStockDCHuffmanCodes_53.with(Value::clone).borrow()))
            && (!((*found_match.borrow()) != 0))
        {
            if (((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                .offset(1_usize as isize)) as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockDCHuffmanCodeCounts_54.with(Value::clone).as_pointer()
                        as Ptr<Value<Box<[i32]>>>)
                        .offset((*i.borrow()) as isize)
                        .read()
                        .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                        .to_any(),
                    ::std::mem::size_of::<[i32; 16]>(),
                )
                == 0)
                && (((((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                    .offset(0_usize as isize)) as Ptr<i32>)
                    .to_any()
                    .memcmp(
                        &((((kStockDCHuffmanCodeValues_55.with(Value::clone).as_pointer()
                            as Ptr<Value<Box<[i32]>>>)
                            .offset((*i.borrow()) as isize)
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
        'loop_: while ((*i.borrow()) < (*kNumStockACHuffmanCodes_56.with(Value::clone).borrow()))
            && (!((*found_match.borrow()) != 0))
        {
            if (((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                .offset(1_usize as isize)) as Ptr<i32>)
                .to_any()
                .memcmp(
                    &((((kStockACHuffmanCodeCounts_57.with(Value::clone).as_pointer()
                        as Ptr<Value<Box<[i32]>>>)
                        .offset((*i.borrow()) as isize)
                        .read()
                        .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                        .to_any(),
                    ::std::mem::size_of::<[i32; 16]>(),
                )
                == 0)
                && (((((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                    .offset(0_usize as isize)) as Ptr<i32>)
                    .to_any()
                    .memcmp(
                        &((((kStockACHuffmanCodeValues_59.with(Value::clone).as_pointer()
                            as Ptr<Value<Box<[i32]>>>)
                            .offset((*i.borrow()) as isize)
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
        let _bits: u64 = ((*found_match.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, _bits, _storage)
    });
    if ((*found_match.borrow()) != 0) {
        ({
            let _bits: u64 = ((*stock_table_idx.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
        return true;
    }
    'loop_: while ((*max_len.borrow()) > 0)
        && ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
            .offset(((*max_len.borrow()) as usize) as isize)
            .read())
            == 0)
    {
        (*max_len.borrow_mut()).prefix_dec();
    }
    if ((((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
        .offset(0_usize as isize)
        .read())
        != 0)
        || ((*max_len.borrow()) == 0)
    {
        return false;
    }
    ({
        let _bits: u64 = (((*max_len.borrow()) - 1) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(4_usize, _bits, _storage)
    });
    (*space.borrow_mut()) -=
        (1 << ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) - (*max_len.borrow())));
    let i: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while ((*i.borrow()) <= (*max_len.borrow())) {
        let count: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (((*huff.upgrade().deref()).counts.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize) as isize)
                .read());
            _lhs - (if ((*i.borrow()) == (*max_len.borrow())) {
                1
            } else {
                0
            })
        }));
        let count_limit: Value<i32> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(
                ((*max_count.borrow()) - (*total_count.borrow())),
            ));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(
                ((*space.borrow())
                    >> ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) - (*i.borrow()))),
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
                (({
                    let _n: u32 = ((*count_limit.borrow()) as u32);
                    Log2FloorNonZero_74(_n)
                }) + 1),
            ));
            ({
                let _n_bits: usize = ((*nbits.borrow()) as usize);
                let _bits: u64 = ((*count.borrow()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, _bits, _storage)
            });
            (*total_count.borrow_mut()) += (*count.borrow());
            (*space.borrow_mut()) -= ((*count.borrow())
                * (1 << ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())
                    - (*i.borrow()))));
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
            .offset(((*total_count.borrow()) as usize) as isize)
            .read());
        _lhs != (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow())
    } {
        return false;
    }
    let p: Value<brunsli_PermutationCoder> = Rc::new(RefCell::new(
        brunsli_PermutationCoder::brunsli_PermutationCoder(),
    ));
    ({
        let _values: Vec<u8> = if ((*is_dc_table.borrow()) != 0) {
            {
                let __count = (kDefaultDCValues_60.with(Value::clone).as_pointer() as Ptr<u8>)
                    .to_end()
                    .get_offset()
                    - (kDefaultDCValues_60.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
                PtrValueIter::new(
                    &(kDefaultDCValues_60.with(Value::clone).as_pointer() as Ptr<u8>),
                    __count,
                )
                .collect::<Vec<_>>()
            }
        } else {
            {
                let __count = (kDefaultACValues_61.with(Value::clone).as_pointer() as Ptr<u8>)
                    .to_end()
                    .get_offset()
                    - (kDefaultACValues_61.with(Value::clone).as_pointer() as Ptr<u8>).get_offset();
                PtrValueIter::new(
                    &(kDefaultACValues_61.with(Value::clone).as_pointer() as Ptr<u8>),
                    __count,
                )
                .collect::<Vec<_>>()
            }
        };
        (*p.borrow()).Init(_values)
    });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*total_count.borrow())) {
        let val: Value<i32> = Rc::new(RefCell::new(
            (((*huff.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize) as isize)
                .read()),
        ));
        let code: Value<i32> = <Value<i32>>::default();
        let nbits: Value<i32> = <Value<i32>>::default();
        if !({
            let _value: u8 = ((*val.borrow()) as u8);
            let _code: Ptr<i32> = (code.as_pointer());
            let _nbits: Ptr<i32> = (nbits.as_pointer());
            (*p.borrow()).RemoveValue(_value, _code, _nbits)
        }) {
            return false;
        }
        ({
            let _bits: usize = ((*code.borrow()) as usize);
            let _max_symbols: i32 = (((*nbits.borrow()) + 1) >> 1);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeLimitedVarint_155(_bits, 2, _max_symbols, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeScanInfo_158(si: Ptr<brunsli_JPEGScanInfo>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        let _bits: u64 = ((*(*si.upgrade().deref()).Ss.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(6_usize, _bits, _storage)
    });
    ({
        let _bits: u64 = ((*(*si.upgrade().deref()).Se.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(6_usize, _bits, _storage)
    });
    ({
        let _bits: u64 = ((*(*si.upgrade().deref()).Ah.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(4_usize, _bits, _storage)
    });
    ({
        let _bits: u64 = ((*(*si.upgrade().deref()).Al.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(4_usize, _bits, _storage)
    });
    ({
        let _bits: u64 =
            ((*(*si.upgrade().deref()).num_components.borrow()).wrapping_sub(1_usize) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*si.upgrade().deref()).num_components.borrow())
    } {
        let csi: Ptr<brunsli_JPEGComponentScanInfo> =
            ((*si.upgrade().deref()).components.as_pointer() as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset((*i.borrow()) as isize);
        ({
            let _bits: u64 = ((*(*csi.upgrade().deref()).comp_idx.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(2_usize, _bits, _storage)
        });
        ({
            let _bits: u64 = ((*(*csi.upgrade().deref()).dc_tbl_idx.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(2_usize, _bits, _storage)
        });
        ({
            let _bits: u64 = ((*(*csi.upgrade().deref()).ac_tbl_idx.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(2_usize, _bits, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let last_block_idx: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: for mut block_idx in (*si.upgrade().deref()).reset_points.as_pointer() as Ptr<i32> {
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 1_u64, _storage)
        });
        if !({
            let _lhs = (block_idx.read());
            _lhs >= ((*last_block_idx.borrow()) + 1)
        }) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeScanInfo");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 391, _fn)
            });
            'loop_: while true {}
        };
        ({
            let _n: i32 = ({
                let _lhs = (block_idx.read());
                _lhs - (*last_block_idx.borrow())
            } - 1);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeVarint_154(_n, 28, _storage)
        });
        let __rhs = (block_idx.read());
        (*last_block_idx.borrow_mut()) = __rhs;
    }
    ({
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, 0_u64, _storage)
    });
    (*last_block_idx.borrow_mut()) = 0;
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*si.upgrade().deref()).extra_zero_runs.borrow()).len()
    } {
        let block_idx: Value<i32> = Rc::new(RefCell::new(
            (*(*((*si.upgrade().deref()).extra_zero_runs.as_pointer()
                as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .block_idx
            .borrow()),
        ));
        let num: Value<i32> = Rc::new(RefCell::new(
            (*(*((*si.upgrade().deref()).extra_zero_runs.as_pointer()
                as Ptr<brunsli_JPEGScanInfo_ExtraZeroRunInfo>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .num_extra_zero_runs
            .borrow()),
        ));
        if !((*block_idx.borrow()) >= (*last_block_idx.borrow())) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeScanInfo");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 401, _fn)
            });
            'loop_: while true {}
        };
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < (*num.borrow())) {
            ({
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(1_usize, 1_u64, _storage)
            });
            ({
                let _n: i32 = ((*block_idx.borrow()) - (*last_block_idx.borrow()));
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                EncodeVarint_154(_n, 28, _storage)
            });
            (*last_block_idx.borrow_mut()) = (*block_idx.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, 0_u64, _storage)
    });
    return true;
}
pub fn MatchComponentIds_159(comps: Ptr<Vec<brunsli_JPEGComponent>>) -> i32 {
    if ((*comps.upgrade().deref()).len() == 1_usize)
        && ((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .id
        .borrow())
            == 1)
    {
        return (*kComponentIdsGray_50.with(Value::clone).borrow());
    }
    if ((*comps.upgrade().deref()).len() == 3_usize) {
        if (((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .id
        .borrow())
            == 1)
            && ((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
                .offset(1_usize as isize)
                .upgrade()
                .deref())
            .id
            .borrow())
                == 2))
            && ((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
                .offset(2_usize as isize)
                .upgrade()
                .deref())
            .id
            .borrow())
                == 3)
        {
            return (*kComponentIds123_49.with(Value::clone).borrow());
        } else if (((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .id
        .borrow())
            == (('R' as u8) as i32))
            && ((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
                .offset(1_usize as isize)
                .upgrade()
                .deref())
            .id
            .borrow())
                == (('G' as u8) as i32)))
            && ((*(*(comps.to_strong().as_pointer() as Ptr<brunsli_JPEGComponent>)
                .offset(2_usize as isize)
                .upgrade()
                .deref())
            .id
            .borrow())
                == (('B' as u8) as i32))
        {
            return (*kComponentIdsRGB_51.with(Value::clone).borrow());
        }
    }
    return (*kComponentIdsCustom_52.with(Value::clone).borrow());
}
pub fn JumpToByteBoundary_160(storage: Ptr<brunsli_Storage>) {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let nbits: Value<i32> = Rc::new(RefCell::new(
        (((*(*(*storage.borrow()).upgrade().deref()).pos.borrow()) & 7_usize) as i32),
    ));
    if ((*nbits.borrow()) > 0) {
        ({
            let _n_bits: usize = ((8 - (*nbits.borrow())) as usize);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, 0_u64, _storage)
        });
    }
}
pub fn EncodeAuxData_161(jpg: Ptr<brunsli_JPEGData>, storage: Ptr<brunsli_Storage>) -> bool {
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*(*jpg.upgrade().deref()).marker_order.borrow()).is_empty())
        || (((((*jpg.upgrade().deref()).marker_order.as_pointer() as Ptr<u8>)
            .to_last()
            .read()) as i32)
            != 217)
    {
        return false;
    }
    let have_dri: Value<bool> = Rc::new(RefCell::new(false));
    let num_scans: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).marker_order.borrow()).len()
    } {
        let marker: Value<u8> = Rc::new(RefCell::new(
            (((*jpg.upgrade().deref()).marker_order.as_pointer() as Ptr<u8>)
                .offset((*i.borrow()) as isize)
                .read()),
        ));
        if (((*marker.borrow()) as i32) < 192) {
            return false;
        }
        ({
            let _bits: u64 = ((((*marker.borrow()) as i32) - 192) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(6_usize, _bits, _storage)
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
            let _bits: u64 = ((*(*jpg.upgrade().deref()).restart_interval.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(16_usize, _bits, _storage)
        });
    }
    if !({
        let _lhs = (*(*jpg.upgrade().deref()).huffman_code.borrow()).len();
        _lhs < ((*kMaxDHTMarkers_10.with(Value::clone).borrow()) as usize)
    }) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeAuxData");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 453, _fn)
        });
        'loop_: while true {}
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).huffman_code.borrow()).len()
    } {
        let is_known_last: Value<bool> = Rc::new(RefCell::new(
            ({
                let _lhs = ((*i.borrow()).wrapping_add(1_usize));
                _lhs == (*(*jpg.upgrade().deref()).huffman_code.borrow()).len()
            }),
        ));
        ({
            let _bits: u64 = ((*is_known_last.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, _bits, _storage)
        });
        if !({
            let _huff: Ptr<brunsli_JPEGHuffmanCode> =
                ((*jpg.upgrade().deref()).huffman_code.as_pointer()
                    as Ptr<brunsli_JPEGHuffmanCode>)
                    .offset((*i.borrow()) as isize);
            let _is_known_last: bool = (*is_known_last.borrow());
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeHuffmanCode_157(_huff, _is_known_last, _storage)
        }) {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (*num_scans.borrow());
        _lhs != (*(*jpg.upgrade().deref()).scan_info.borrow()).len()
    } {
        return false;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).scan_info.borrow()).len()
    } {
        if !({
            let _si: Ptr<brunsli_JPEGScanInfo> = ((*jpg.upgrade().deref()).scan_info.as_pointer()
                as Ptr<brunsli_JPEGScanInfo>)
                .offset((*i.borrow()) as isize);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeScanInfo_158(_si, _storage)
        }) {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _bits: u64 =
            (((*(*jpg.upgrade().deref()).quant.borrow()).len()).wrapping_sub(1_usize) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).quant.borrow()).len()
    } {
        ({
            let _bits: u64 = ((*(*((*jpg.upgrade().deref()).quant.as_pointer()
                as Ptr<brunsli_JPEGQuantTable>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .index
            .borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(2_usize, _bits, _storage)
        });
        if {
            let _lhs = (*i.borrow());
            _lhs != ((*(*jpg.upgrade().deref()).quant.borrow()).len()).wrapping_sub(1_usize)
        } {
            ({
                let _bits: u64 = ((*(*((*jpg.upgrade().deref()).quant.as_pointer()
                    as Ptr<brunsli_JPEGQuantTable>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .is_last
                .borrow()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(1_usize, _bits, _storage)
            });
        } else if !(*(*((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .is_last
        .borrow())
        {
            return false;
        }
        ({
            let _bits: u64 = ((*(*((*jpg.upgrade().deref()).quant.as_pointer()
                as Ptr<brunsli_JPEGQuantTable>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .precision
            .borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(4_usize, _bits, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let comp_ids: Value<i32> = Rc::new(RefCell::new(
        ({
            let _comps: Ptr<Vec<brunsli_JPEGComponent>> =
                (*jpg.upgrade().deref()).components.as_pointer();
            MatchComponentIds_159(_comps)
        }),
    ));
    ({
        let _bits: u64 = ((*comp_ids.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    if ((*comp_ids.borrow()) == (*kComponentIdsCustom_52.with(Value::clone).borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*jpg.upgrade().deref()).components.borrow()).len()
        } {
            ({
                let _bits: u64 = ((*(*((*jpg.upgrade().deref()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .id
                .borrow()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(8_usize, _bits, _storage)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    let nsize: Value<usize> = Rc::new(RefCell::new(
        if (*(*jpg.upgrade().deref()).has_zero_padding_bit.borrow()) {
            (*(*jpg.upgrade().deref()).padding_bits.borrow()).len()
        } else {
            0_usize
        },
    ));
    if {
        let _lhs = (*nsize.borrow());
        _lhs > (({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            PaddingBitsLimit_17(_jpg)
        }) as usize)
    } {
        return false;
    }
    ({
        let _bits: usize = (*nsize.borrow());
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        EncodeLimitedVarint_155(_bits, 8, 4, _storage)
    });
    if ((*nsize.borrow()) > 0_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*nsize.borrow())) {
            ({
                let _bits: u64 = ((((*jpg.upgrade().deref()).padding_bits.as_pointer() as Ptr<i32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(1_usize, _bits, _storage)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    ({
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        JumpToByteBoundary_160(_storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).inter_marker_data.borrow()).len()
    } {
        let s: Ptr<Vec<u8>> = (((*jpg.upgrade().deref()).inter_marker_data.as_pointer()
            as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>);
        let buffer: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..10).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        let len: Value<usize> = Rc::new(RefCell::new(
            ({
                let _val: usize = (*s.upgrade().deref()).len();
                let _data: Ptr<u8> = (buffer.as_pointer() as Ptr<u8>);
                EncodeBase128_146(_val, _data)
            }),
        ));
        ({
            let _src: Ptr<u8> = (buffer.as_pointer() as Ptr<u8>);
            let _len: usize = (*len.borrow());
            (*(*storage.borrow()).upgrade().deref()).AppendBytes(_src, _len)
        });
        ({
            let _src: Ptr<u8> = (s.to_strong().as_pointer() as Ptr<u8>);
            let _len: usize = (*s.upgrade().deref()).len();
            (*(*storage.borrow()).upgrade().deref()).AppendBytes(_src, _len)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
impl brunsli_internal_enc_Histogram {}
impl brunsli_internal_enc_Histogram {
    pub fn Clear(&self) {
        {
            ((self.data_.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .memset((0) as u8, ::std::mem::size_of::<[i32; 18]>() as usize);
            ((self.data_.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .clone()
        };
        (*self.total_count_.borrow_mut()) = 0;
    }
}
impl brunsli_internal_enc_Histogram {
    pub fn AddHistogram(&self, other: Ptr<brunsli_internal_enc_Histogram>) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            let __rhs = (*(*other.upgrade().deref()).data_.borrow())[(*i.borrow()) as usize];
            (*self.data_.borrow_mut())[(*i.borrow()) as usize] += __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        let __rhs = (*(*other.upgrade().deref()).total_count_.borrow());
        (*self.total_count_.borrow_mut()) += __rhs;
    }
}
impl brunsli_internal_enc_Histogram {
    pub fn Add(&self, val: usize) {
        let val: Value<usize> = Rc::new(RefCell::new(val));
        if !((*val.borrow()) < 18_usize) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"Add");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 522, _fn)
            });
            'loop_: while true {}
        };
        (*self.data_.borrow_mut())[(*val.borrow()) as usize].prefix_inc();
        (*self.total_count_.borrow_mut()).prefix_inc();
    }
}
impl brunsli_internal_enc_Histogram {
    pub fn Merge(&self, other: Ptr<brunsli_internal_enc_Histogram>) {
        if ((*(*other.upgrade().deref()).total_count_.borrow()) == 0) {
            return;
        }
        let __rhs = (*(*other.upgrade().deref()).total_count_.borrow());
        (*self.total_count_.borrow_mut()) += __rhs;
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < 18_usize) {
            let __rhs = (*(*other.upgrade().deref()).data_.borrow())[(*i.borrow()) as usize];
            (*self.data_.borrow_mut())[(*i.borrow()) as usize] += __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub fn ComputeCoeffOrder_162(num_zeros: Ptr<Vec<i32>>, order: Ptr<u32>) {
    let order: Value<Ptr<u32>> = Rc::new(RefCell::new(order));
    let pos_and_val: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize) as usize)
            .map(|_| <(Value<i32>, Value<i32>)>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        let __rhs = (*i.borrow());
        (*(*(pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>)
            .offset(((*i.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .0
        .borrow_mut()) = __rhs;
        let __rhs = ((num_zeros.to_strong().as_pointer() as Ptr<i32>)
            .offset(
                ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    as usize) as isize,
            )
            .read());
        (*(*(pos_and_val.as_pointer() as Ptr<(Value<i32>, Value<i32>)>)
            .offset(((*i.borrow()) as usize) as isize)
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
        (|a: Ptr<(Value<i32>, Value<i32>)>, b: Ptr<(Value<i32>, Value<i32>)>| {
            return {
                let _lhs = (*(*a.upgrade().deref()).1.borrow());
                _lhs < (*(*b.upgrade().deref()).1.borrow())
            };
        }),
    );
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)) {
        let __rhs =
            (*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*(*(pos_and_val.as_pointer()
                as Ptr<(Value<i32>, Value<i32>)>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .0
            .borrow()) as usize];
        (*order.borrow())
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
impl brunsli_internal_enc_EntropySource {
    pub fn Resize(&self, num_bands: usize) {
        let num_bands: Value<usize> = Rc::new(RefCell::new(num_bands));
        (*self.num_bands_.borrow_mut()) = (*num_bands.borrow());
        {
            let __a0 = (*num_bands.borrow())
                .wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow()))
                as usize;
            (*self.histograms_.borrow_mut())
                .resize_with(__a0, || <brunsli_internal_enc_Histogram>::default())
        };
    }
}
impl brunsli_internal_enc_EntropySource {
    pub fn AddCode(&self, code: usize, histo_ix: usize) {
        let code: Value<usize> = Rc::new(RefCell::new(code));
        let histo_ix: Value<usize> = Rc::new(RefCell::new(histo_ix));
        ({
            let _val: usize = (*code.borrow());
            (*(self.histograms_.as_pointer() as Ptr<brunsli_internal_enc_Histogram>)
                .offset((*histo_ix.borrow()) as isize)
                .upgrade()
                .deref())
            .Add(_val)
        });
    }
}
impl brunsli_internal_enc_EntropySource {
    pub fn Finish(
        &self,
        offsets: Ptr<Vec<u64>>,
    ) -> Option<Value<brunsli_internal_enc_EntropyCodes>> {
        let histograms: Value<Vec<brunsli_internal_enc_Histogram>> =
            Rc::new(RefCell::new(Vec::new()));
        std::mem::swap(
            &mut (*histograms.borrow_mut()),
            &mut (*self.histograms_.borrow_mut()),
        );
        return Ptr::alloc(
            brunsli_internal_enc_EntropyCodes::brunsli_internal_enc_EntropyCodes(
                histograms.as_pointer(),
                (*self.num_bands_.borrow()),
                (offsets).clone(),
            ),
        )
        .to_owned_opt();
    }
}
impl brunsli_internal_enc_EntropySource {
    pub fn Merge(&self, other: Ptr<brunsli_internal_enc_EntropySource>) {
        if !({
            let _lhs = (*self.histograms_.borrow()).len();
            _lhs >= (*(*other.upgrade().deref()).histograms_.borrow()).len()
        }) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"Merge");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 568, _fn)
            });
            'loop_: while true {}
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*other.upgrade().deref()).histograms_.borrow()).len()
        } {
            ({
                let _other: Ptr<brunsli_internal_enc_Histogram> =
                    ((*other.upgrade().deref()).histograms_.as_pointer()
                        as Ptr<brunsli_internal_enc_Histogram>)
                        .offset((*i.borrow()) as isize);
                (*(self.histograms_.as_pointer() as Ptr<brunsli_internal_enc_Histogram>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .Merge(_other)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
impl brunsli_internal_enc_EntropyCodes {}
impl brunsli_internal_enc_EntropyCodes {
    pub fn EncodeContextMap(&self, storage: Ptr<brunsli_Storage>) {
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        ({
            let _context_map: Ptr<Vec<u32>> = self.context_map_.as_pointer();
            let _num_clusters: usize = (*self.clustered_.borrow()).len();
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            EncodeContextMap_163(_context_map, _num_clusters, _storage)
        });
    }
}
impl brunsli_internal_enc_EntropyCodes {
    pub fn BuildAndStoreEntropyCodes(&self, storage: Ptr<brunsli_Storage>) {
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        {
            let __a0 = (*self.clustered_.borrow()).len() as usize;
            (*self.ans_tables_.borrow_mut()).resize_with(__a0, || <brunsli_ANSTable>::default())
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*self.clustered_.borrow()).len()) {
            ({
                let _histogram: Ptr<i32> = (((*(self.clustered_.as_pointer()
                    as Ptr<brunsli_internal_enc_Histogram>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .data_
                .as_pointer() as Ptr<i32>)
                    .offset(0 as isize));
                let _table: Ptr<brunsli_ANSTable> = ((self.ans_tables_.as_pointer()
                    as Ptr<brunsli_ANSTable>)
                    .offset((*i.borrow()) as isize));
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                BuildAndStoreANSEncodingData_123(_histogram, _table, _storage)
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
impl brunsli_internal_enc_EntropyCodes {
    pub fn GetANSTable(&self, context: i32) -> Ptr<brunsli_ANSTable> {
        let context: Value<i32> = Rc::new(RefCell::new(context));
        let entropy_ix: Value<i32> = Rc::new(RefCell::new(
            (((self.context_map_.as_pointer() as Ptr<u32>)
                .offset(((*context.borrow()) as usize) as isize)
                .read()) as i32),
        ));
        return ((self.ans_tables_.as_pointer() as Ptr<brunsli_ANSTable>)
            .offset(((*entropy_ix.borrow()) as usize) as isize));
    }
}
impl brunsli_internal_enc_DataStream {}
impl brunsli_internal_enc_DataStream {
    pub fn Resize(&self, max_num_code_words: usize) {
        let max_num_code_words: Value<usize> = Rc::new(RefCell::new(max_num_code_words));
        {
            let __a0 = (*max_num_code_words.borrow()) as usize;
            (*self.code_words_.borrow_mut()).resize_with(__a0, || {
                <brunsli_internal_enc_DataStream_CodeWord>::default()
            })
        };
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn ResizeForBlock(&self) {
        if (((*self.pos_.borrow()) as usize)
            .wrapping_add((*kSlackForOneBlock_139.with(Value::clone).borrow()))
            > (*self.code_words_.borrow()).len())
        {
            thread_local!(
                static kGrowMult_164: Value<f64> = Rc::new(RefCell::new(1.2E+0));
            );
            let new_size: Value<usize> = Rc::new(RefCell::new(
                (((*kGrowMult_164.with(Value::clone).borrow())
                    * ((*self.code_words_.borrow()).capacity() as f64)) as usize)
                    .wrapping_add((*kSlackForOneBlock_139.with(Value::clone).borrow())),
            ));
            {
                let __a0 = (*new_size.borrow()) as usize;
                (*self.code_words_.borrow_mut()).resize_with(__a0, || {
                    <brunsli_internal_enc_DataStream_CodeWord>::default()
                })
            };
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn AddCode(
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
            ((*band.borrow()).wrapping_mul((*kNumAvrgContexts_83.with(Value::clone).borrow())))
                .wrapping_add((*context.borrow())),
        ));
        let word: Value<brunsli_internal_enc_DataStream_CodeWord> = Rc::new(RefCell::new(
            brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord(),
        ));
        (*(*word.borrow()).context.borrow_mut()) = ((*histo_ix.borrow()) as u32);
        (*(*word.borrow()).code.borrow_mut()) = (((*code.borrow()) as u32) as u8);
        (*(*word.borrow()).nbits.borrow_mut()) = 0_u8;
        (*(*word.borrow()).value.borrow_mut()) = 0_u16;
        if !(((*self.pos_.borrow()) as usize) < (*self.code_words_.borrow()).len()) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"AddCode");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 631, _fn)
            });
            'loop_: while true {}
        };
        (self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.pos_.borrow_mut()).postfix_inc() as usize) as isize)
            .write((*word.borrow()).clone());
        ({
            let _code: usize = (*code.borrow());
            let _histo_ix: usize = (*histo_ix.borrow());
            (*(*s.borrow()).upgrade().deref()).AddCode(_code, _histo_ix)
        });
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn AddBits(&self, nbits: i32, bits: i32) {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        let bits: Value<i32> = Rc::new(RefCell::new(bits));
        (*self.bw_val_.borrow_mut()) |= (((*bits.borrow()) << (*self.bw_bitpos_.borrow())) as u32);
        (*self.bw_bitpos_.borrow_mut()) += (*nbits.borrow());
        if ((*self.bw_bitpos_.borrow()) > 16) {
            let word: Value<brunsli_internal_enc_DataStream_CodeWord> = Rc::new(RefCell::new(
                brunsli_internal_enc_DataStream_CodeWord::brunsli_internal_enc_DataStream_CodeWord(
                ),
            ));
            (*(*word.borrow()).context.borrow_mut()) = 0_u32;
            (*(*word.borrow()).code.borrow_mut()) = 0_u8;
            (*(*word.borrow()).nbits.borrow_mut()) = 16_u8;
            (*(*word.borrow()).value.borrow_mut()) =
                (((*self.bw_val_.borrow()) & 65535_u32) as u16);
            (self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset(((*self.bw_pos_.borrow()) as usize) as isize)
                .write((*word.borrow()).clone());
            (*self.bw_pos_.borrow_mut()) = (*self.pos_.borrow());
            (*self.pos_.borrow_mut()).prefix_inc();
            (*self.bw_val_.borrow_mut()) >>= 16;
            (*self.bw_bitpos_.borrow_mut()) -= 16;
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn FlushArithmeticCoder(&self) {
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.ac_pos0_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .value
        .borrow_mut()) = (((*self.high_.borrow()) >> 16) as u16);
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.ac_pos1_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .value
        .borrow_mut()) = (((*self.high_.borrow()) & 65535_u32) as u16);
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.ac_pos0_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .nbits
        .borrow_mut()) = 16_u8;
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.ac_pos1_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .nbits
        .borrow_mut()) = 16_u8;
        (*self.low_.borrow_mut()) = 0_u32;
        (*self.high_.borrow_mut()) = (!0 as u32);
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn FlushBitWriter(&self) {
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.bw_pos_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .nbits
        .borrow_mut()) = 16_u8;
        (*(*(self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
            .offset(((*self.bw_pos_.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .value
        .borrow_mut()) = (((*self.bw_val_.borrow()) & 65535_u32) as u16);
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn AddBit(&self, p: Ptr<brunsli_Prob>, bit: i32) {
        let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
        let bit: Value<i32> = Rc::new(RefCell::new(bit));
        let prob: Value<u8> = Rc::new(RefCell::new(
            ({ (*(*p.borrow()).upgrade().deref()).get_proba() }),
        ));
        ({
            let _val: i32 = (*bit.borrow());
            (*(*p.borrow()).upgrade().deref()).Add(_val)
        });
        let diff: Value<u32> = Rc::new(RefCell::new(
            (*self.high_.borrow()).wrapping_sub((*self.low_.borrow())),
        ));
        let split: Value<u32> = Rc::new(RefCell::new(
            ((((*self.low_.borrow()) as u64).wrapping_add(
                ((((*diff.borrow()) as u64).wrapping_mul(((*prob.borrow()) as u64))) >> 8),
            )) as u32),
        ));
        if ((*bit.borrow()) != 0) {
            (*self.low_.borrow_mut()) = (*split.borrow()).wrapping_add(1_u32);
        } else {
            (*self.high_.borrow_mut()) = (*split.borrow());
        }
        if ((((*self.low_.borrow()) ^ (*self.high_.borrow())) >> 16) == 0_u32) {
            (*(*(self.code_words_.as_pointer()
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset(((*self.ac_pos0_.borrow()) as usize) as isize)
                .upgrade()
                .deref())
            .value
            .borrow_mut()) = (((*self.high_.borrow()) >> 16) as u16);
            (*(*(self.code_words_.as_pointer()
                as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                .offset(((*self.ac_pos0_.borrow()) as usize) as isize)
                .upgrade()
                .deref())
            .nbits
            .borrow_mut()) = 16_u8;
            (*self.ac_pos0_.borrow_mut()) = (*self.ac_pos1_.borrow());
            (*self.ac_pos1_.borrow_mut()) = (*self.pos_.borrow());
            (*self.pos_.borrow_mut()).prefix_inc();
            (*self.low_.borrow_mut()) <<= 16;
            (*self.high_.borrow_mut()) <<= 16;
            (*self.high_.borrow_mut()) |= 65535_u32;
        }
    }
}
impl brunsli_internal_enc_DataStream {
    pub fn EncodeCodeWords(
        &self,
        s: Ptr<brunsli_internal_enc_EntropyCodes>,
        storage: Ptr<brunsli_Storage>,
    ) {
        let s: Value<Ptr<brunsli_internal_enc_EntropyCodes>> = Rc::new(RefCell::new(s));
        let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
        ({ self.FlushBitWriter() });
        ({ self.FlushArithmeticCoder() });
        let ans: Value<brunsli_ANSCoder> =
            Rc::new(RefCell::new(brunsli_ANSCoder::brunsli_ANSCoder()));
        let i: Value<i32> = Rc::new(RefCell::new(((*self.pos_.borrow()) - 1)));
        'loop_: while ((*i.borrow()) >= 0) {
            let word: Value<Ptr<brunsli_internal_enc_DataStream_CodeWord>> = Rc::new(RefCell::new(
                ((self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                    .offset(((*i.borrow()) as usize) as isize)),
            ));
            if (((*(*(*word.borrow()).upgrade().deref()).nbits.borrow()) as i32) == 0) {
                let info: Value<brunsli_ANSEncSymbolInfo> = Rc::new(RefCell::new(
                    ((*(*({
                        let _context: i32 =
                            ((*(*(*word.borrow()).upgrade().deref()).context.borrow()) as i32);
                        (*(*s.borrow()).upgrade().deref()).GetANSTable(_context)
                    })
                    .upgrade()
                    .deref())
                    .info_
                    .borrow())
                        [(*(*(*word.borrow()).upgrade().deref()).code.borrow()) as usize])
                        .clone(),
                ));
                let __rhs = (({
                    let _t: brunsli_ANSEncSymbolInfo = (*info.borrow()).clone();
                    let _nbits: Ptr<u8> =
                        ((*(*word.borrow()).upgrade().deref()).nbits.as_pointer());
                    (*ans.borrow()).PutSymbol(_t, _nbits)
                }) as u16);
                (*(*(*word.borrow()).upgrade().deref()).value.borrow_mut()) = __rhs;
            }
            (*i.borrow_mut()).prefix_dec();
        }
        let state: Value<u32> = Rc::new(RefCell::new(({ (*ans.borrow()).GetState() })));
        let out: Value<Ptr<u16>> = Rc::new(RefCell::new(
            ((*(*(*storage.borrow()).upgrade().deref()).data.borrow()).reinterpret_cast::<u16>())
                .clone(),
        ));
        let out_start: Value<Ptr<u16>> = Rc::new(RefCell::new((*out.borrow()).clone()));
        ({
            let _p: AnyPtr = (((*out.borrow_mut()).postfix_inc()).clone() as Ptr<u16>).to_any();
            let _v: u16 = (((*state.borrow()) >> 16) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        ({
            let _p: AnyPtr = (((*out.borrow_mut()).postfix_inc()).clone() as Ptr<u16>).to_any();
            let _v: u16 = ((*state.borrow()) as u16);
            BrunsliUnalignedWrite16_67(_p, _v)
        });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*self.pos_.borrow())) {
            let word: Ptr<brunsli_internal_enc_DataStream_CodeWord> =
                (self.code_words_.as_pointer() as Ptr<brunsli_internal_enc_DataStream_CodeWord>)
                    .offset(((*i.borrow()) as usize) as isize);
            if ((*(*word.upgrade().deref()).nbits.borrow()) != 0) {
                ({
                    let _p: AnyPtr =
                        (((*out.borrow_mut()).postfix_inc()).clone() as Ptr<u16>).to_any();
                    let _v: u16 = (*(*word.upgrade().deref()).value.borrow());
                    BrunsliUnalignedWrite16_67(_p, _v)
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let rhs_0 = (*(*(*storage.borrow()).upgrade().deref()).pos.borrow()).wrapping_add(
            (((((*out.borrow()).clone() - (*out_start.borrow()).clone()) as i64) * 16_i64)
                as usize),
        );
        (*(*(*storage.borrow()).upgrade().deref()).pos.borrow_mut()) = rhs_0;
    }
}
pub fn EncodeNumNonzeros_165(
    val: usize,
    p: Ptr<brunsli_Prob>,
    data_stream: Ptr<brunsli_internal_enc_DataStream>,
) {
    let val: Value<usize> = Rc::new(RefCell::new(val));
    let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(p));
    let data_stream: Value<Ptr<brunsli_internal_enc_DataStream>> =
        Rc::new(RefCell::new(data_stream));
    if !((*val.borrow()) < ((1_u32 << (*kNumNonZeroBits_84.with(Value::clone).borrow())) as usize))
    {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeNumNonzeros");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 719, _fn)
        });
        'loop_: while true {}
    };
    let bst: Value<Ptr<brunsli_Prob>> =
        Rc::new(RefCell::new((*p.borrow()).offset(-((1) as isize))));
    let ctx: Value<usize> = Rc::new(RefCell::new(1_usize));
    let mask: Value<usize> = Rc::new(RefCell::new(
        ((1 << ((*kNumNonZeroBits_84.with(Value::clone).borrow()).wrapping_sub(1_usize))) as usize),
    ));
    'loop_: while ((*mask.borrow()) != 0_usize) {
        let bit: Value<i32> = Rc::new(RefCell::new(
            ((((*val.borrow()) & (*mask.borrow())) != 0_usize) as i32),
        ));
        ({
            let _p: Ptr<brunsli_Prob> = (*bst.borrow()).offset((*ctx.borrow()) as isize);
            let _bit: i32 = (*bit.borrow());
            (*(*data_stream.borrow()).upgrade().deref()).AddBit(_p, _bit)
        });
        let __rhs =
            ((2_usize).wrapping_mul((*ctx.borrow()))).wrapping_add(((*bit.borrow()) as usize));
        (*ctx.borrow_mut()) = __rhs;
        (*mask.borrow_mut()) >>= 1;
    }
}
pub fn CollectAllCoeffs_166(coeffs: Ptr<i16>) -> i16 {
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(coeffs));
    let all_coeffs: Value<i16> = Rc::new(RefCell::new(0_i16));
    let k: Value<i32> = Rc::new(RefCell::new(1));
    'loop_: while (((*all_coeffs.borrow()) as i32) == 0)
        && ((*k.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow()))
    {
        let rhs_0 = (((*all_coeffs.borrow()) as i32)
            | (((*coeffs.borrow()).offset((*k.borrow()) as isize).read()) as i32))
            as i16;
        (*all_coeffs.borrow_mut()) = rhs_0;
        (*k.borrow_mut()).prefix_inc();
    }
    return (*all_coeffs.borrow());
}
pub fn EncodeCoeffOrder_167(order: Ptr<u32>, data_stream: Ptr<brunsli_internal_enc_DataStream>) {
    let order: Value<Ptr<u32>> = Rc::new(RefCell::new(order));
    let data_stream: Value<Ptr<brunsli_internal_enc_DataStream>> =
        Rc::new(RefCell::new(data_stream));
    let order_zigzag: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)) {
        let __rhs = (*kJPEGZigZagOrder_14.with(Value::clone).borrow())
            [((*order.borrow()).offset((*i.borrow()) as isize).read()) as usize];
        (*order_zigzag.borrow_mut())[(*i.borrow()) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let lehmer: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
    ));
    ({
        let _sigma: Ptr<u32> = (order_zigzag.as_pointer() as Ptr<u32>);
        let _len: usize = ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize);
        let _code: Ptr<u32> = (lehmer.as_pointer() as Ptr<u32>);
        ComputeLehmerCode_112(_sigma, _len, _code)
    });
    let tail: Value<i32> = Rc::new(RefCell::new(
        ((*kDCTBlockSize_3.with(Value::clone).borrow()) - 1),
    ));
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
        static kSpan_168: Value<i32> = Rc::new(RefCell::new(16));
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
        let start: Value<i32> = Rc::new(RefCell::new(if ((*i.borrow()) > 0) {
            (*i.borrow())
        } else {
            1
        }));
        let end: Value<i32> = Rc::new(RefCell::new(
            ((*i.borrow()) + (*kSpan_168.with(Value::clone).borrow())),
        ));
        let has_non_zero: Value<i32> = Rc::new(RefCell::new(0));
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            let rhs_0 = (((*has_non_zero.borrow()) as u32)
                | (*lehmer.borrow())[(*j.borrow()) as usize]) as i32;
            (*has_non_zero.borrow_mut()) = rhs_0;
            (*j.borrow_mut()).prefix_inc();
        }
        if !((*has_non_zero.borrow()) != 0) {
            ({ (*(*data_stream.borrow()).upgrade().deref()).AddBits(1, 0) });
            (*i.borrow_mut()) += (*kSpan_168.with(Value::clone).borrow());
            continue 'loop_;
        } else {
            ({ (*(*data_stream.borrow()).upgrade().deref()).AddBits(1, 1) });
        }
        let j: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        'loop_: while ((*j.borrow()) < (*end.borrow())) {
            let v: Value<i32> = <Value<i32>>::default();
            if !((*lehmer.borrow())[(*j.borrow()) as usize]
                <= ((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32))
            {
                ({
                    let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeCoeffOrder");
                    BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 769, _fn)
                });
                'loop_: while true {}
            };
            (*v.borrow_mut()) = ((*lehmer.borrow())[(*j.borrow()) as usize] as i32);
            'loop_: while ((*v.borrow()) >= 7) {
                ({ (*(*data_stream.borrow()).upgrade().deref()).AddBits(3, 7) });
                (*v.borrow_mut()) -= 7;
            }
            ({
                let _bits: i32 = (*v.borrow());
                (*(*data_stream.borrow()).upgrade().deref()).AddBits(3, _bits)
            });
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()) += (*kSpan_168.with(Value::clone).borrow());
    }
}
pub fn FrameTypeCode_169(jpg: Ptr<brunsli_JPEGData>) -> u32 {
    let code: Value<u32> = Rc::new(RefCell::new(0_u32));
    let shift: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).components.borrow()).len()
    }) && ((*i.borrow()) < 4_usize)
    {
        let h_samp: Value<u32> = Rc::new(RefCell::new(
            (((*(*((*jpg.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .h_samp_factor
            .borrow())
                - 1) as u32),
        ));
        let v_samp: Value<u32> = Rc::new(RefCell::new(
            (((*(*((*jpg.upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .v_samp_factor
            .borrow())
                - 1) as u32),
        ));
        (*code.borrow_mut()) |= (((*h_samp.borrow()) << ((*shift.borrow()) + 4))
            | ((*v_samp.borrow()) << (*shift.borrow())));
        (*shift.borrow_mut()) += 8;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*code.borrow());
}
pub fn EncodeSignature_170(len: usize, data: Ptr<u8>, pos: Ptr<usize>) -> bool {
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    if ((*len.borrow()) < (*kBrunsliSignatureSize_43.with(Value::clone).borrow()))
        || ({
            let _lhs = ((*pos.borrow()).read());
            _lhs > (*len.borrow())
                .wrapping_sub((*kBrunsliSignatureSize_43.with(Value::clone).borrow()))
        })
    {
        return false;
    }
    {
        (((*data.borrow()).offset(((*pos.borrow()).read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((kBrunsliSignature_44.with(Value::clone).as_pointer() as Ptr<u8>) as Ptr<u8>)
                    .to_any(),
                (*kBrunsliSignatureSize_43.with(Value::clone).borrow()) as usize,
            );
        (((*data.borrow()).offset(((*pos.borrow()).read()) as isize)) as Ptr<u8>)
            .to_any()
            .clone()
    };
    let rhs_0 = ((*pos.borrow()).read())
        .wrapping_add((*kBrunsliSignatureSize_43.with(Value::clone).borrow()));
    (*pos.borrow()).write(rhs_0);
    return true;
}
pub fn EncodeValue_171(tag: u8, value: usize, data: Ptr<u8>, pos: Ptr<usize>) {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    let value: Value<usize> = Rc::new(RefCell::new(value));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let __rhs = ({
        let _tag: u8 = (*tag.borrow());
        ValueMarker_28(_tag)
    });
    (*data.borrow())
        .offset(((*pos.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add(
        ({
            let _val: usize = (*value.borrow());
            let _data: Ptr<u8> = (*data.borrow()).offset(((*pos.borrow()).read()) as isize);
            EncodeBase128_146(_val, _data)
        }),
    );
    (*pos.borrow()).write(rhs_0);
}
pub fn EncodeHeader_172(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*state.borrow_mut()).clone();
    let version: Value<usize> = Rc::new(RefCell::new(
        ((*(*jpg.upgrade().deref()).version.borrow()) as usize),
    ));
    let is_fallback: Value<bool> = Rc::new(RefCell::new(
        (((*version.borrow()) & 1_usize)
            == ((*kFallbackVersion_2.with(Value::clone).borrow()) as usize)),
    ));
    if (*is_fallback.borrow())
        && ((*version.borrow()) != ((*kFallbackVersion_2.with(Value::clone).borrow()) as usize))
    {
        return false;
    }
    if (((!(*is_fallback.borrow()))
        && (((*(*jpg.upgrade().deref()).width.borrow()) == 0)
            || ((*(*jpg.upgrade().deref()).height.borrow()) == 0)))
        || ((*(*jpg.upgrade().deref()).components.borrow()).is_empty()))
        || ({
            let _lhs = (*(*jpg.upgrade().deref()).components.borrow()).len();
            _lhs > ((*kMaxComponents_4.with(Value::clone).borrow()) as usize)
        })
    {
        return false;
    }
    if (((*version.borrow()) & (!7_u32 as usize)) != 0) {
        return false;
    }
    let version_comp: Value<usize> = Rc::new(RefCell::new(
        ({
            let _lhs = ((((*(*jpg.upgrade().deref()).components.borrow()).len())
                .wrapping_sub(1_usize)) as u64);
            _lhs | (((*version.borrow()) << 2) as u64)
        } as usize),
    ));
    let subsampling: Value<usize> = Rc::new(RefCell::new(
        (({
            let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
            FrameTypeCode_169(_jpg)
        }) as usize),
    ));
    let pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        let _tag: u8 = (*kBrunsliHeaderWidthTag_39.with(Value::clone).borrow());
        let _value: usize = ((*(*jpg.upgrade().deref()).width.borrow()) as usize);
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeValue_171(_tag, _value, _data, _pos)
    });
    ({
        let _tag: u8 = (*kBrunsliHeaderHeightTag_40.with(Value::clone).borrow());
        let _value: usize = ((*(*jpg.upgrade().deref()).height.borrow()) as usize);
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeValue_171(_tag, _value, _data, _pos)
    });
    ({
        let _tag: u8 = (*kBrunsliHeaderVersionCompTag_41.with(Value::clone).borrow());
        let _value: usize = (*version_comp.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeValue_171(_tag, _value, _data, _pos)
    });
    ({
        let _tag: u8 = (*kBrunsliHeaderSubsamplingTag_42.with(Value::clone).borrow());
        let _value: usize = (*subsampling.borrow());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeValue_171(_tag, _value, _data, _pos)
    });
    let __rhs = (*pos.borrow());
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeMetaData_173(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*state.borrow_mut()).clone();
    let metadata: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    let transformed_marker_count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).app_data.borrow()).len()
    } {
        let s: Ptr<Vec<u8>> = (((*jpg.upgrade().deref()).app_data.as_pointer()
            as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()) as isize)
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
                    TransformAppMarker_152(_s, _transformed_marker_count)
                }),
            ));
            Append_73(_dst, _src.as_pointer())
        });
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*transformed_marker_count.borrow())
        > ((*kBrunsliShortMarkerLimit_23.with(Value::clone).borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many short markers: {:}\n",
            (*transformed_marker_count.borrow()),
        );
        return false;
    }
    let other_app_count: Value<usize> = Rc::new(RefCell::new(
        (((*(*jpg.upgrade().deref()).app_data.borrow()).len() as u64)
            .wrapping_sub(((*transformed_marker_count.borrow()) as u64)) as usize),
    ));
    if ((*other_app_count.borrow())
        > ((*kBrunsliMultibyteMarkerLimit_24.with(Value::clone).borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many app markers: {:}\n",
            (*other_app_count.borrow()),
        );
        return false;
    }
    let com_count: Value<usize> = Rc::new(RefCell::new(
        (*(*jpg.upgrade().deref()).com_data.borrow()).len(),
    ));
    if ((*com_count.borrow())
        > ((*kBrunsliMultibyteMarkerLimit_24.with(Value::clone).borrow()) as usize))
    {
        write!(
            libcc2rs::cerr(),
            "Too many com markers: {:}\n",
            (*com_count.borrow()),
        );
        return false;
    }
    'loop_: for mut s in (*jpg.upgrade().deref()).com_data.as_pointer() as Ptr<Value<Vec<u8>>> {
        let s: Ptr<Vec<u8>> = s.upgrade().deref().as_pointer();
        ({
            let _dst: Ptr<Vec<u8>> = (metadata.as_pointer());
            let _src: Ptr<Vec<u8>> = (s).clone();
            Append_73(_dst, _src)
        });
    }
    if !(*(*jpg.upgrade().deref()).tail_data.borrow()).is_empty() {
        let marker: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([217_u8])));
        ({
            let _dst: Ptr<Vec<u8>> = (metadata.as_pointer());
            let _begin: Ptr<u8> = (marker.as_pointer() as Ptr<u8>);
            Append_72(_dst, _begin, 1_usize)
        });
        ({
            let _dst: Ptr<Vec<u8>> = (metadata.as_pointer());
            let _src: Ptr<Vec<u8>> = (*jpg.upgrade().deref()).tail_data.as_pointer();
            Append_73(_dst, _src)
        });
    }
    if (*metadata.borrow()).is_empty() {
        (*len.borrow()).write(0_usize);
        return true;
    } else if ((*metadata.borrow()).len() == 1_usize) {
        (*len.borrow()).write(1_usize);
        let __rhs = ((metadata.as_pointer() as Ptr<u8>)
            .offset(0_usize as isize)
            .read());
        (*data.borrow()).offset((0) as isize).write(__rhs);
        return true;
    }
    let pos: Value<usize> = Rc::new(RefCell::new(
        ({
            let _val: usize = (*metadata.borrow()).len();
            let _data: Ptr<u8> = (*data.borrow()).clone();
            EncodeBase128_146(_val, _data)
        }),
    ));
    let compressed_size: Value<usize> = Rc::new(RefCell::new(
        ((*len.borrow()).read()).wrapping_sub((*pos.borrow())),
    ));
    if !((compressed_size.as_pointer()).with_mut(|_v5| {
        ((*data.borrow()).offset((*pos.borrow()) as isize)).with_mut(|_v6| unsafe {
            ::brotli_sys::BrotliEncoderCompress(
                (*kBrotliQuality_141.with(Value::clone).borrow()),
                (*kBrotliWindowBits_142.with(Value::clone).borrow()),
                ::brotli_sys::BROTLI_MODE_GENERIC,
                (*metadata.borrow()).len(),
                &*(metadata.as_pointer() as Ptr<u8>).upgrade().deref() as *const u8,
                _v5 as *mut usize,
                _v6,
            )
        })
    }) != 0)
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
    let rhs_0 = (*pos.borrow()).wrapping_add((*compressed_size.borrow()));
    (*pos.borrow_mut()) = rhs_0;
    let __rhs = (*pos.borrow());
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeJPEGInternals_174(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*state.borrow_mut()).clone();
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::brunsli_Storage(
        (*data.borrow()).clone(),
        ((*len.borrow()).read()),
    )));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeAuxData_161(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ (*storage.borrow()).GetBytesUsed() });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeQuantData_175(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*state.borrow_mut()).clone();
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::brunsli_Storage(
        (*data.borrow()).clone(),
        ((*len.borrow()).read()),
    )));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        EncodeQuantTables_156(_jpg, _storage)
    }) {
        return false;
    }
    let __rhs = ({ (*storage.borrow()).GetBytesUsed() });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeHistogramData_176(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::brunsli_Storage(
        (*data.borrow()).clone(),
        ((*len.borrow()).read()),
    )));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*jpg.upgrade().deref()).components.borrow()).len()
    } {
        ({
            let _bits: u64 = ((*(*((*(*state.borrow()).upgrade().deref()).meta.as_pointer()
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .context_bits
            .borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
            WriteBits_120(3_usize, _bits, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        (*(*(*(*state.borrow()).upgrade().deref())
            .entropy_codes
            .borrow())
        .upgrade()
        .deref())
        .EncodeContextMap(_storage)
    });
    ({
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        (*(*(*(*state.borrow()).upgrade().deref())
            .entropy_codes
            .borrow())
        .upgrade()
        .deref())
        .BuildAndStoreEntropyCodes(_storage)
    });
    let __rhs = ({ (*storage.borrow()).GetBytesUsed() });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeDCData_177(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*jpg.upgrade().deref()).clone();
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::brunsli_Storage(
        (*data.borrow()).clone(),
        ((*len.borrow()).read()),
    )));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> = (*(*(*state.borrow()).upgrade().deref())
            .entropy_codes
            .borrow())
        .clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        (*(*(*state.borrow()).upgrade().deref())
            .data_stream_dc
            .borrow())
        .EncodeCodeWords(_s, _storage)
    });
    let __rhs = ({ (*storage.borrow()).GetBytesUsed() });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeACData_178(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*jpg.upgrade().deref()).clone();
    let storage: Value<brunsli_Storage> = Rc::new(RefCell::new(brunsli_Storage::brunsli_Storage(
        (*data.borrow()).clone(),
        ((*len.borrow()).read()),
    )));
    ({
        let _s: Ptr<brunsli_internal_enc_EntropyCodes> = (*(*(*state.borrow()).upgrade().deref())
            .entropy_codes
            .borrow())
        .clone();
        let _storage: Ptr<brunsli_Storage> = (storage.as_pointer());
        (*(*(*state.borrow()).upgrade().deref())
            .data_stream_ac
            .borrow())
        .EncodeCodeWords(_s, _storage)
    });
    let __rhs = ({ (*storage.borrow()).GetBytesUsed() });
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn EncodeSection_179(
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
    let marker: Value<u8> = Rc::new(RefCell::new(
        ({
            let _tag: u8 = (*tag.borrow());
            SectionMarker_29(_tag)
        }),
    ));
    let __rhs = (*marker.borrow());
    (*data.borrow())
        .offset(((*pos.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add((*section_size_bytes.borrow()));
    (*pos.borrow()).write(rhs_0);
    let section_size: Value<usize> = Rc::new(RefCell::new(
        (*len.borrow()).wrapping_sub(((*pos.borrow()).read())),
    ));
    if !({
        let _arg0: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _arg1: Ptr<brunsli_internal_enc_State> = (*s.borrow()).clone();
        let _arg2: Ptr<u8> = ((*data.borrow()).offset(((*pos.borrow()).read()) as isize));
        let _arg3: Ptr<usize> = (section_size.as_pointer());
        (*(*write_section.borrow()))(_arg0, _arg1, _arg2, _arg3)
    }) {
        return false;
    }
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add((*section_size.borrow()));
    (*pos.borrow()).write(rhs_0);
    if (((*section_size.borrow()) >> ((7_usize).wrapping_mul((*section_size_bytes.borrow()))))
        > 0_usize)
    {
        write!(libcc2rs::cerr(), "Section 0x",);
        libcc2rs::cerr()
            .write_all(&([(&[(*marker.borrow())] as &[u8]), (b" size " as &[u8])].concat()));
        write!(
            libcc2rs::cerr(),
            "{:} too large for {:} bytes base128 number.\n",
            (*section_size.borrow()),
            (*section_size_bytes.borrow()),
        );
        return false;
    }
    ({
        let _val: usize = (*section_size.borrow());
        let _len: usize = (*section_size_bytes.borrow());
        let _data: Ptr<u8> =
            ((*data.borrow()).offset(((*pos_start.borrow()).wrapping_add(1_usize)) as isize));
        EncodeBase128Fix_147(_val, _len, _data)
    });
    return true;
}
pub fn SampleNumNonZeros_180(m: Ptr<brunsli_internal_enc_ComponentMeta>) -> usize {
    let m: Value<Ptr<brunsli_internal_enc_ComponentMeta>> = Rc::new(RefCell::new(m));
    let num_blocks: Value<usize> = Rc::new(RefCell::new(
        (({
            let _lhs = (*(*(*m.borrow()).upgrade().deref()).width_in_blocks.borrow());
            _lhs * (*(*(*m.borrow()).upgrade().deref()).height_in_blocks.borrow())
        }) as usize),
    ));
    if ((*num_blocks.borrow()) < ((32 * 32) as usize)) {
        return ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)
            .wrapping_mul((*num_blocks.borrow()));
    }
    let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
        (*(*(*m.borrow()).upgrade().deref()).ac_coeffs.borrow()).clone(),
    ));
    let stride: Value<usize> = Rc::new(RefCell::new(
        ((*(*(*m.borrow()).upgrade().deref()).ac_stride.borrow()) as usize),
    ));
    let width_in_blocks: Value<usize> = Rc::new(RefCell::new(
        ((*(*(*m.borrow()).upgrade().deref()).width_in_blocks.borrow()) as usize),
    ));
    let num_zeros: Ptr<Vec<i32>> = (*(*m.borrow()).upgrade().deref()).num_zeros.as_pointer();
    thread_local!(
        static kStride_181: Value<i32> = Rc::new(RefCell::new(5));
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
                    ((*x.borrow())
                        .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)))
                        as isize,
                )
                .offset(((*y.borrow()).wrapping_mul((*stride.borrow()))) as isize),
        ));
        let k: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*k.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)) {
            if ((((*block.borrow()).offset((*k.borrow()) as isize).read()) as i32) == 0) {
                (num_zeros.to_strong().as_pointer() as Ptr<i32>)
                    .offset((*k.borrow()) as isize)
                    .with_mut(|__v| __v.prefix_inc());
            }
            (*k.borrow_mut()).prefix_inc();
        }
        let rhs_0 = (*total_nonzeros.borrow())
            .wrapping_add(((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize));
        (*total_nonzeros.borrow_mut()) = rhs_0;
        let rhs_0 =
            (*i.borrow()).wrapping_add(((*kStride_181.with(Value::clone).borrow()) as usize));
        (*i.borrow_mut()) = rhs_0;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)) {
        let rhs_0 = (*total_nonzeros.borrow()).wrapping_sub(
            (((num_zeros.to_strong().as_pointer() as Ptr<i32>)
                .offset((*i.borrow()) as isize)
                .read()) as usize),
        );
        (*total_nonzeros.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    (num_zeros.to_strong().as_pointer() as Ptr<i32>)
        .offset(0_usize as isize)
        .write(0);
    return (*total_nonzeros.borrow())
        .wrapping_mul(((*kStride_181.with(Value::clone).borrow()) as usize));
}
pub fn SelectContextBits_182(num_symbols: usize) -> i32 {
    let num_symbols: Value<usize> = Rc::new(RefCell::new(num_symbols));
    thread_local!(
        static kContextBits_183: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6,
            6, 6, 6, 6,
        ])));
    );
    let log2_size: Value<usize> = Rc::new(RefCell::new(
        (({
            let _n: u32 = ((*num_symbols.borrow()) as u32);
            Log2FloorNonZero_74(_n)
        }) as usize),
    ));
    let scheme: Value<i32> = Rc::new(RefCell::new(
        (*kContextBits_183.with(Value::clone).borrow())[(*log2_size.borrow()) as usize],
    ));
    if !((*scheme.borrow()) < (*kNumSchemes_91.with(Value::clone).borrow())) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"SelectContextBits");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"brunsli_encode.cc"), 1029, _fn)
        });
        'loop_: while true {}
    };
    return (*scheme.borrow());
}
pub fn PredictDCCoeffs_184(state: Ptr<brunsli_internal_enc_State>) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*meta.upgrade().deref()).len()
    } {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize);
        let width: Value<i32> = Rc::new(RefCell::new(
            (*(*m.upgrade().deref()).width_in_blocks.borrow()),
        ));
        let height: Value<i32> = Rc::new(RefCell::new(
            (*(*m.upgrade().deref()).height_in_blocks.borrow()),
        ));
        let ac_stride: Value<i32> =
            Rc::new(RefCell::new((*(*m.upgrade().deref()).ac_stride.borrow())));
        let dc_stride: Value<i32> =
            Rc::new(RefCell::new((*(*m.upgrade().deref()).dc_stride.borrow())));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*y.borrow()) < (*height.borrow())) {
            let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                (*(*m.upgrade().deref()).ac_coeffs.borrow())
                    .offset(((*ac_stride.borrow()) * (*y.borrow())) as isize),
            ));
            let pred_errors: Value<Ptr<i16>> = Rc::new(RefCell::new(
                (*(*m.upgrade().deref()).dc_prediction_errors.borrow())
                    .offset(((*dc_stride.borrow()) * (*y.borrow())) as isize),
            ));
            let x: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*x.borrow()) < (*width.borrow())) {
                let err: Value<i32> = Rc::new(RefCell::new({
                    let _lhs = (((*coeffs.borrow()).offset((0) as isize).read()) as i32);
                    _lhs - ({
                        let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                        let _x: i32 = (*x.borrow());
                        let _y: i32 = (*y.borrow());
                        let _stride: i32 = (*ac_stride.borrow());
                        PredictWithAdaptiveMedian_115(_coeffs, _x, _y, _stride)
                    })
                }));
                if ((*err.borrow()).abs() > (*kBrunsliMaxDCAbsVal_19.with(Value::clone).borrow())) {
                    write!(
                        libcc2rs::cerr(),
                        "Invalid DC coefficient: {:} after prediction: {:}\n",
                        ((*coeffs.borrow()).offset((0) as isize).read()),
                        (*err.borrow()),
                    );
                    return false;
                }
                (*coeffs.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
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
pub fn CalculateMeta_185(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let num_components: Value<usize> = Rc::new(RefCell::new(
        (*(*jpg.upgrade().deref()).components.borrow()).len(),
    ));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    {
        let __a0 = (*num_components.borrow()) as usize;
        meta.with_mut(|__v: &mut Vec<brunsli_internal_enc_ComponentMeta>| {
            __v.resize_with(__a0, || <brunsli_internal_enc_ComponentMeta>::default())
        })
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let c: Ptr<brunsli_JPEGComponent> = ((*jpg.upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize);
        let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize);
        if {
            let _lhs = ((*(*c.upgrade().deref()).quant_idx.borrow()) as usize);
            _lhs >= (*(*jpg.upgrade().deref()).quant.borrow()).len()
        } {
            return false;
        }
        let q: Ptr<brunsli_JPEGQuantTable> = ((*jpg.upgrade().deref()).quant.as_pointer()
            as Ptr<brunsli_JPEGQuantTable>)
            .offset(((*(*c.upgrade().deref()).quant_idx.borrow()) as usize) as isize);
        (*(*m.upgrade().deref()).h_samp.borrow_mut()) =
            (*(*c.upgrade().deref()).h_samp_factor.borrow());
        (*(*m.upgrade().deref()).v_samp.borrow_mut()) =
            (*(*c.upgrade().deref()).v_samp_factor.borrow());
        let __rhs = {
            let _lhs = (*(*jpg.upgrade().deref()).MCU_cols.borrow());
            _lhs * (*(*m.upgrade().deref()).h_samp.borrow())
        };
        (*(*m.upgrade().deref()).width_in_blocks.borrow_mut()) = __rhs;
        let __rhs = {
            let _lhs = (*(*jpg.upgrade().deref()).MCU_rows.borrow());
            _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
        };
        (*(*m.upgrade().deref()).height_in_blocks.borrow_mut()) = __rhs;
        (*(*m.upgrade().deref()).ac_coeffs.borrow_mut()) =
            (((*c.upgrade().deref()).coeffs.as_pointer() as Ptr<i16>).offset(0_usize as isize));
        let __rhs = {
            let _lhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
            _lhs * (*kDCTBlockSize_3.with(Value::clone).borrow())
        };
        (*(*m.upgrade().deref()).ac_stride.borrow_mut()) = __rhs;
        let __rhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
        (*(*m.upgrade().deref()).dc_stride.borrow_mut()) = __rhs;
        let __rhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
        (*(*m.upgrade().deref()).b_stride.borrow_mut()) = __rhs;
        {
            (((*m.upgrade().deref()).quant.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((((*q.upgrade().deref()).values.as_pointer() as Ptr<i32>)
                        .offset(0_usize as isize)) as Ptr<i32>)
                        .to_any(),
                    ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize)
                        .wrapping_mul((::std::mem::size_of::<i32>() as usize))
                        as usize,
                );
            (((*m.upgrade().deref()).quant.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .clone()
        };
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn EncodeDC_186(state: Ptr<brunsli_internal_enc_State>) {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .height_in_blocks
        .borrow());
        _lhs / (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .v_samp
        .borrow())
    }));
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> =
        (*(*state.borrow()).upgrade().deref())
            .entropy_source
            .as_pointer();
    let data_stream: Ptr<brunsli_internal_enc_DataStream> = (*(*state.borrow()).upgrade().deref())
        .data_stream_dc
        .as_pointer();
    let comps: Value<Vec<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <brunsli_ComponentStateDC>::default())
            .collect::<Vec<_>>(),
    ));
    let total_num_blocks: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize);
        ({
            let _w: i32 = (*(*m.upgrade().deref()).width_in_blocks.borrow());
            (*(comps.as_pointer() as Ptr<brunsli_ComponentStateDC>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .SetWidth(_w)
        });
        let rhs_0 = (*total_num_blocks.borrow()).wrapping_add(
            (({
                let _lhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
                _lhs * (*(*m.upgrade().deref()).height_in_blocks.borrow())
            }) as usize),
        );
        (*total_num_blocks.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _num_bands: usize = (*num_components.borrow());
        (*entropy_source.upgrade().deref()).Resize(_num_bands)
    });
    ({
        let _max_num_code_words: usize =
            ((3_usize).wrapping_mul((*total_num_blocks.borrow()))).wrapping_add(128_usize);
        (*data_stream.upgrade().deref()).Resize(_max_num_code_words)
    });
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentStateDC>> = Rc::new(RefCell::new(
                ((comps.as_pointer() as Ptr<brunsli_ComponentStateDC>)
                    .offset((*i.borrow()) as isize)),
            ));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()) as isize);
            let width: Value<i32> = Rc::new(RefCell::new(
                (*(*(*c.borrow()).upgrade().deref()).width.borrow()),
            ));
            let ac_stride: Value<i32> =
                Rc::new(RefCell::new((*(*m.upgrade().deref()).ac_stride.borrow())));
            let dc_stride: Value<i32> =
                Rc::new(RefCell::new((*(*m.upgrade().deref()).dc_stride.borrow())));
            let b_stride: Value<i32> =
                Rc::new(RefCell::new((*(*m.upgrade().deref()).b_stride.borrow())));
            let y: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*mcu_y.borrow());
                _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
            }));
            let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*(*c.borrow()).upgrade().deref()).prev_sign.as_pointer() as Ptr<i32>)
                    .offset(1_usize as isize)),
            ));
            let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                (((*(*c.borrow()).upgrade().deref())
                    .prev_abs_coeff
                    .as_pointer() as Ptr<i32>)
                    .offset(2_usize as isize)),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while {
                let _lhs = (*iy.borrow());
                _lhs < (*(*m.upgrade().deref()).v_samp.borrow())
            } {
                let dc_coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).dc_prediction_errors.borrow())
                        .offset(((*y.borrow()) * (*dc_stride.borrow())) as isize),
                ));
                let ac_coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).ac_coeffs.borrow())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).block_state.borrow())
                        .offset(((*y.borrow()) * (*b_stride.borrow())) as isize),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    ({ (*data_stream.upgrade().deref()).ResizeForBlock() });
                    let coeff: Value<i16> = Rc::new(RefCell::new(
                        ((*dc_coeffs_in.borrow()).offset((0) as isize).read()),
                    ));
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
                        (({
                            let _lhs = ((*coeff.borrow()) as i32);
                            _lhs | (({
                                let _coeffs: Ptr<i16> = (*ac_coeffs_in.borrow()).clone();
                                CollectAllCoeffs_166(_coeffs)
                            }) as i32)
                        }) as i16),
                    ));
                    let is_empty_block: Value<bool> =
                        Rc::new(RefCell::new((((*all_coeffs.borrow()) as i32) == 0)));
                    let is_empty_ctx: Value<i32> = Rc::new(RefCell::new(
                        ({
                            let _prev: Ptr<i32> = (((*(*c.borrow()).upgrade().deref())
                                .prev_is_nonempty
                                .as_pointer()
                                as Ptr<i32>)
                                .offset(1_usize as isize));
                            let _x: i32 = (*x.borrow());
                            IsEmptyBlockContext_106(_prev, _x)
                        }),
                    ));
                    ({
                        let _p: Ptr<brunsli_Prob> = (((*(*c.borrow()).upgrade().deref())
                            .is_empty_block_prob
                            .as_pointer()
                            as Ptr<brunsli_Prob>)
                            .offset(((*is_empty_ctx.borrow()) as usize) as isize));
                        let _bit: i32 = (!(*is_empty_block.borrow()) as i32);
                        (*data_stream.upgrade().deref()).AddBit(_p, _bit)
                    });
                    ((*(*c.borrow()).upgrade().deref())
                        .prev_is_nonempty
                        .as_pointer() as Ptr<i32>)
                        .offset((((*x.borrow()) + 1) as usize) as isize)
                        .write((!(*is_empty_block.borrow()) as i32));
                    let __rhs = ((*is_empty_block.borrow()) as u8);
                    (*block_state.borrow()).write(__rhs);
                    if !(*is_empty_block.borrow()) {
                        let is_zero: Value<i32> =
                            Rc::new(RefCell::new(((((*coeff.borrow()) as i32) == 0) as i32)));
                        ({
                            let _p: Ptr<brunsli_Prob> =
                                ((*(*c.borrow()).upgrade().deref()).is_zero_prob.as_pointer());
                            let _bit: i32 = (*is_zero.borrow());
                            (*data_stream.upgrade().deref()).AddBit(_p, _bit)
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
                                ((((*prev_sgn.borrow()).offset((*x.borrow()) as isize).read())
                                    * 3)
                                    + ((*prev_sgn.borrow())
                                        .offset(((*x.borrow()) - 1) as isize)
                                        .read())),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> =
                                    (((*(*c.borrow()).upgrade().deref()).sign_prob.as_pointer()
                                        as Ptr<brunsli_Prob>)
                                        .offset(((*sign_ctx.borrow()) as usize) as isize));
                                let _bit: i32 = ((*sign.borrow()) - 1);
                                (*data_stream.upgrade().deref()).AddBit(_p, _bit)
                            });
                            let zdens_ctx: Value<usize> = Rc::new(RefCell::new((*i.borrow())));
                            if ((*absval.borrow())
                                <= (*kNumDirectCodes_140.with(Value::clone).borrow()))
                            {
                                ({
                                    let _code: usize = (((*absval.borrow()) - 1) as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = (((*avrg_ctx.borrow()) as u32) as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    (*data_stream.upgrade().deref())
                                        .AddCode(_code, _band, _context, _s)
                                });
                            } else {
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    (({
                                        let _n: u32 = ((((*absval.borrow())
                                            - (*kNumDirectCodes_140.with(Value::clone).borrow()))
                                            + 1)
                                            as u32);
                                        Log2FloorNonZero_74(_n)
                                    }) - 1),
                                ));
                                ({
                                    let _code: usize =
                                        (((*kNumDirectCodes_140.with(Value::clone).borrow())
                                            + (*nbits.borrow()))
                                            as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = ((*avrg_ctx.borrow()) as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    (*data_stream.upgrade().deref())
                                        .AddCode(_code, _band, _context, _s)
                                });
                                let extra_bits: Value<i32> = Rc::new(RefCell::new(
                                    ((*absval.borrow())
                                        - (((*kNumDirectCodes_140.with(Value::clone).borrow())
                                            - 1)
                                            + (2 << (*nbits.borrow())))),
                                ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    (((*extra_bits.borrow()) >> (*nbits.borrow())) & 1),
                                ));
                                ({
                                    let _p: Ptr<brunsli_Prob> =
                                        (((*(*c.borrow()).upgrade().deref())
                                            .first_extra_bit_prob
                                            .as_pointer()
                                            as Ptr<brunsli_Prob>)
                                            .offset(((*nbits.borrow()) as usize) as isize));
                                    let _bit: i32 = (*first_extra_bit.borrow());
                                    (*data_stream.upgrade().deref()).AddBit(_p, _bit)
                                });
                                if ((*nbits.borrow()) > 0) {
                                    (*extra_bits.borrow_mut()) &= ((1 << (*nbits.borrow())) - 1);
                                    ({
                                        let _nbits: i32 = (*nbits.borrow());
                                        let _bits: i32 = (*extra_bits.borrow());
                                        (*data_stream.upgrade().deref()).AddBits(_nbits, _bits)
                                    });
                                }
                            }
                        }
                    }
                    let __rhs = (*sign.borrow());
                    (*prev_sgn.borrow())
                        .offset((*x.borrow()) as isize)
                        .write(__rhs);
                    let __rhs = (*absval.borrow());
                    (*prev_abs.borrow())
                        .offset((*x.borrow()) as isize)
                        .write(__rhs);
                    (*block_state.borrow_mut()).prefix_inc();
                    (*dc_coeffs_in.borrow_mut()).prefix_inc();
                    (*ac_coeffs_in.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                (*iy.borrow_mut()).prefix_inc();
                (*y.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*mcu_y.borrow_mut()).prefix_inc();
    }
}
pub fn EncodeAC_187(state: Ptr<brunsli_internal_enc_State>) {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let mcu_rows: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .height_in_blocks
        .borrow());
        _lhs / (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .v_samp
        .borrow())
    }));
    let entropy_source: Ptr<brunsli_internal_enc_EntropySource> =
        (*(*state.borrow()).upgrade().deref())
            .entropy_source
            .as_pointer();
    let data_stream: Ptr<brunsli_internal_enc_DataStream> = (*(*state.borrow()).upgrade().deref())
        .data_stream_ac
        .as_pointer();
    let context_modes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (kContextAlgorithm_95.with(Value::clone).as_pointer() as Ptr<u8>).offset(
            (if (*(*(*state.borrow()).upgrade().deref())
                .use_legacy_context_model
                .borrow())
            {
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
        let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize);
        let num_blocks: Value<usize> = Rc::new(RefCell::new(
            (({
                let _lhs = (*(*m.upgrade().deref()).width_in_blocks.borrow());
                _lhs * (*(*m.upgrade().deref()).height_in_blocks.borrow())
            }) as usize),
        ));
        let rhs_0 = (*num_code_words.borrow()).wrapping_add(
            (((2_usize).wrapping_mul((*(*m.upgrade().deref()).approx_total_nonzeros.borrow())))
                .wrapping_add(1024_usize))
            .wrapping_add((3_usize).wrapping_mul((*num_blocks.borrow()))),
        );
        (*num_code_words.borrow_mut()) = rhs_0;
        ({
            let _num_zeros: Ptr<Vec<i32>> = (*m.upgrade().deref()).num_zeros.as_pointer();
            let _order: Ptr<u32> = (((*(comps.as_pointer() as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .order
            .as_pointer() as Ptr<u32>)
                .offset(0 as isize));
            ComputeCoeffOrder_162(_num_zeros, _order)
        });
        ({
            let _quant: Ptr<i32> = ((*m.upgrade().deref()).quant.as_pointer() as Ptr<i32>);
            let _mult_row: Ptr<i32> = (((*(comps.as_pointer() as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .mult_row
            .as_pointer() as Ptr<i32>)
                .offset(0 as isize));
            let _mult_col: Ptr<i32> = (((*(comps.as_pointer() as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .mult_col
            .as_pointer() as Ptr<i32>)
                .offset(0 as isize));
            ComputeACPredictMultipliers_109(_quant, _mult_row, _mult_col)
        });
        ({
            let _w: i32 = (*(*m.upgrade().deref()).width_in_blocks.borrow());
            (*(comps.as_pointer() as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .SetWidth(_w)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _num_bands: usize = (*(*(*state.borrow()).upgrade().deref()).num_contexts.borrow());
        (*entropy_source.upgrade().deref()).Resize(_num_bands)
    });
    ({
        let _max_num_code_words: usize = (*num_code_words.borrow());
        (*data_stream.upgrade().deref()).Resize(_max_num_code_words)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        ({
            let _order: Ptr<u32> = (((*(comps.as_pointer() as Ptr<brunsli_ComponentState>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .order
            .as_pointer() as Ptr<u32>)
                .offset(0 as isize));
            let _data_stream: Ptr<brunsli_internal_enc_DataStream> = (data_stream).clone();
            EncodeCoeffOrder_167(_order, _data_stream)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*mcu_rows.borrow())) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
            let c: Value<Ptr<brunsli_ComponentState>> = Rc::new(RefCell::new(
                ((comps.as_pointer() as Ptr<brunsli_ComponentState>)
                    .offset((*i.borrow()) as isize)),
            ));
            let m: Ptr<brunsli_internal_enc_ComponentMeta> = (meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()) as isize);
            let cur_ctx_bits: Value<i32> = Rc::new(RefCell::new(
                (*(*m.upgrade().deref()).context_bits.borrow()),
            ));
            let cur_order: Value<Ptr<u32>> = Rc::new(RefCell::new(
                ((*(*c.borrow()).upgrade().deref()).order.as_pointer() as Ptr<u32>),
            ));
            let width: Value<i32> = Rc::new(RefCell::new(
                (*(*(*c.borrow()).upgrade().deref()).width.borrow()),
            ));
            let y: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*mcu_y.borrow());
                _lhs * (*(*m.upgrade().deref()).v_samp.borrow())
            }));
            let ac_stride: Value<i32> =
                Rc::new(RefCell::new((*(*m.upgrade().deref()).ac_stride.borrow())));
            let b_stride: Value<i32> =
                Rc::new(RefCell::new((*(*m.upgrade().deref()).b_stride.borrow())));
            let prev_row_delta: Value<i32> = Rc::new(RefCell::new(
                (((1 - (2 * ((*y.borrow()) & 1))) * ((*width.borrow()) + 3))
                    * (*kDCTBlockSize_3.with(Value::clone).borrow())),
            ));
            let iy: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while {
                let _lhs = (*iy.borrow());
                _lhs < (*(*m.upgrade().deref()).v_samp.borrow())
            } {
                let coeffs_in: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).ac_coeffs.borrow())
                        .offset(((*y.borrow()) * (*ac_stride.borrow())) as isize),
                ));
                let block_state: Value<Ptr<u8>> = Rc::new(RefCell::new(
                    (*(*m.upgrade().deref()).block_state.borrow())
                        .offset(((*y.borrow()) * (*b_stride.borrow())) as isize),
                ));
                let prev_row_coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*coeffs_in.borrow()).offset(-((*ac_stride.borrow()) as isize)),
                ));
                let prev_col_coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                    (*coeffs_in.borrow())
                        .offset(-((*kDCTBlockSize_3.with(Value::clone).borrow()) as isize)),
                ));
                let prev_sgn: Value<Ptr<i32>> = Rc::new(RefCell::new(
                    (((*(*c.borrow()).upgrade().deref()).prev_sign.as_pointer() as Ptr<i32>)
                        .offset(
                            ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize) as isize,
                        )),
                ));
                let prev_abs: Value<Ptr<i32>> = Rc::new(RefCell::new(
                    (((*(*c.borrow()).upgrade().deref())
                        .prev_abs_coeff
                        .as_pointer() as Ptr<i32>)
                        .offset(
                            ((((((*y.borrow()) & 1) * ((*width.borrow()) + 3)) + 2)
                                * (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                as usize) as isize,
                        )),
                ));
                let x: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*x.borrow()) < (*width.borrow())) {
                    ({ (*data_stream.upgrade().deref()).ResizeForBlock() });
                    let coeffs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
                        0_i16,
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                    ])));
                    let last_nz: Value<i32> = Rc::new(RefCell::new(0));
                    let is_empty_block: Value<bool> =
                        Rc::new(RefCell::new((((*block_state.borrow()).read()) != 0)));
                    if !(*is_empty_block.borrow()) {
                        let k: Value<i32> = Rc::new(RefCell::new(1));
                        'loop_: while ((*k.borrow())
                            < (*kDCTBlockSize_3.with(Value::clone).borrow()))
                        {
                            let k_nat: Value<i32> = Rc::new(RefCell::new(
                                (((*cur_order.borrow()).offset((*k.borrow()) as isize).read())
                                    as i32),
                            ));
                            let __rhs = ((*coeffs_in.borrow())
                                .offset((*k_nat.borrow()) as isize)
                                .read());
                            (*coeffs.borrow_mut())[(*k.borrow()) as usize] = __rhs;
                            if ((*coeffs.borrow())[(*k.borrow()) as usize] != 0) {
                                (*last_nz.borrow_mut()) = (*k.borrow());
                            }
                            (*k.borrow_mut()).prefix_inc();
                        }
                        let nzero_context: Value<u8> = Rc::new(RefCell::new(
                            ({
                                let _prev: Ptr<u8> = ((*(*c.borrow()).upgrade().deref())
                                    .prev_num_nonzeros
                                    .as_pointer()
                                    as Ptr<u8>);
                                let _x: i32 = (*x.borrow());
                                let _y: i32 = (*y.borrow());
                                NumNonzerosContext_104(_prev, _x, _y)
                            }),
                        ));
                        ({
                            let _val: usize = ((*last_nz.borrow()) as usize);
                            let _p: Ptr<brunsli_Prob> = ((*(*c.borrow()).upgrade().deref())
                                .num_nonzero_prob
                                .as_pointer()
                                as Ptr<brunsli_Prob>)
                                .offset(
                                    ((*kNumNonZeroTreeSize_85.with(Value::clone).borrow())
                                        .wrapping_mul(((*nzero_context.borrow()) as usize)))
                                        as isize,
                                );
                            let _data_stream: Ptr<brunsli_internal_enc_DataStream> =
                                (data_stream).clone();
                            EncodeNumNonzeros_165(_val, _p, _data_stream)
                        });
                    }
                    let k: Value<i32> = Rc::new(RefCell::new(
                        ((*kDCTBlockSize_3.with(Value::clone).borrow()) - 1),
                    ));
                    'loop_: while ((*k.borrow()) > (*last_nz.borrow())) {
                        (*prev_sgn.borrow()).offset((*k.borrow()) as isize).write(0);
                        (*prev_abs.borrow()).offset((*k.borrow()) as isize).write(0);
                        (*k.borrow_mut()).prefix_dec();
                    }
                    let num_nzeros: Value<usize> = Rc::new(RefCell::new(0_usize));
                    let encoded_coeffs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
                        0_i16,
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                        <i16>::default(),
                    ])));
                    let k: Value<i32> = Rc::new(RefCell::new((*last_nz.borrow())));
                    'loop_: while ((*k.borrow()) >= 1) {
                        let coeff: Value<i16> =
                            Rc::new(RefCell::new((*coeffs.borrow())[(*k.borrow()) as usize]));
                        let is_zero: Value<i32> =
                            Rc::new(RefCell::new(((((*coeff.borrow()) as i32) == 0) as i32)));
                        if ((*k.borrow()) < (*last_nz.borrow())) {
                            let bucket: Value<i32> = Rc::new(RefCell::new(
                                ((*kNonzeroBuckets_89.with(Value::clone).borrow())
                                    [((*num_nzeros.borrow()).wrapping_sub(1_usize)) as usize]
                                    as i32),
                            ));
                            let is_zero_ctx: Value<i32> = Rc::new(RefCell::new(
                                (((*bucket.borrow())
                                    * (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                    + (*k.borrow())),
                            ));
                            let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*(*c.borrow()).upgrade().deref()).is_zero_prob.as_pointer()
                                    as Ptr<brunsli_Prob>)
                                    .offset(((*is_zero_ctx.borrow()) as usize) as isize)),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> = (*p.borrow()).clone();
                                let _bit: i32 = (*is_zero.borrow());
                                (*data_stream.upgrade().deref()).AddBit(_p, _bit)
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
                                (((*cur_order.borrow()).offset((*k.borrow()) as isize).read())
                                    as i32),
                            ));
                            let context_type: Value<usize> = Rc::new(RefCell::new(
                                (((*context_modes.borrow())
                                    .offset((*k_nat.borrow()) as isize)
                                    .read()) as usize),
                            ));
                            let avg_ctx: Value<usize> = Rc::new(RefCell::new(0_usize));
                            let sign_ctx: Value<usize> = Rc::new(RefCell::new(
                                (*kMaxAverageContext_82.with(Value::clone).borrow()),
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
                                        let _mult: Ptr<i32> = (((*(*c.borrow()).upgrade().deref())
                                            .mult_col
                                            .as_pointer()
                                            as Ptr<i32>)
                                            .offset(
                                                (*offset.borrow()).wrapping_mul(8_usize) as isize
                                            ));
                                        let _avg_ctx: Ptr<usize> = (avg_ctx.as_pointer());
                                        let _sgn: Ptr<usize> = (sign_ctx.as_pointer());
                                        ACPredictContextRow_103(_prev, _cur, _mult, _avg_ctx, _sgn)
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
                                        let _mult: Ptr<i32> = (((*(*c.borrow()).upgrade().deref())
                                            .mult_row
                                            .as_pointer()
                                            as Ptr<i32>)
                                            .offset((*offset.borrow()) as isize));
                                        let _avg_ctx: Ptr<usize> = (avg_ctx.as_pointer());
                                        let _sgn: Ptr<usize> = (sign_ctx.as_pointer());
                                        ACPredictContextCol_102(_prev, _cur, _mult, _avg_ctx, _sgn)
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
                                (*sign_ctx.borrow_mut()) = (({
                                    let _lhs = (((*prev_sgn.borrow())
                                        .offset((*k.borrow()) as isize)
                                        .read())
                                        * 3);
                                    _lhs + ((*prev_sgn.borrow())
                                        .offset(
                                            ((*k.borrow())
                                                - (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                                as isize,
                                        )
                                        .read())
                                })
                                    as usize);
                            }
                            let __rhs = ((*sign_ctx.borrow()).wrapping_mul(
                                ((*kDCTBlockSize_3.with(Value::clone).borrow()) as usize),
                            ))
                            .wrapping_add(((*k.borrow()) as usize));
                            (*sign_ctx.borrow_mut()) = __rhs;
                            let sign_p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                (((*(*c.borrow()).upgrade().deref()).sign_prob.as_pointer()
                                    as Ptr<brunsli_Prob>)
                                    .offset((*sign_ctx.borrow()) as isize)),
                            ));
                            ({
                                let _p: Ptr<brunsli_Prob> = (*sign_p.borrow()).clone();
                                let _bit: i32 = (*sign.borrow());
                                (*data_stream.upgrade().deref()).AddBit(_p, _bit)
                            });
                            let __rhs = ((*sign.borrow()) + 1);
                            (*prev_sgn.borrow())
                                .offset((*k.borrow()) as isize)
                                .write(__rhs);
                            let zdens_ctx: Value<usize> = Rc::new(RefCell::new(
                                (*(*m.upgrade().deref()).context_offset.borrow()).wrapping_add(
                                    (({
                                        let _nonzeros_left: usize = (*num_nzeros.borrow());
                                        let _k: usize = ((*k.borrow()) as usize);
                                        let _bits: usize = ((*cur_ctx_bits.borrow()) as usize);
                                        ZeroDensityContext_96(_nonzeros_left, _k, _bits)
                                    }) as usize),
                                ),
                            ));
                            if ((*absval.borrow())
                                <= (*kNumDirectCodes_140.with(Value::clone).borrow()))
                            {
                                ({
                                    let _code: usize = (((*absval.borrow()) - 1) as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = (*avg_ctx.borrow());
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    (*data_stream.upgrade().deref())
                                        .AddCode(_code, _band, _context, _s)
                                });
                            } else {
                                let base_code: Value<i32> = Rc::new(RefCell::new(
                                    (((*absval.borrow())
                                        - (*kNumDirectCodes_140.with(Value::clone).borrow()))
                                        + 1),
                                ));
                                let nbits: Value<i32> = Rc::new(RefCell::new(
                                    (({
                                        let _n: u32 = ((*base_code.borrow()) as u32);
                                        Log2FloorNonZero_74(_n)
                                    }) - 1),
                                ));
                                ({
                                    let _code: usize =
                                        (((*kNumDirectCodes_140.with(Value::clone).borrow())
                                            + (*nbits.borrow()))
                                            as usize);
                                    let _band: usize = (*zdens_ctx.borrow());
                                    let _context: usize = (((*avg_ctx.borrow()) as u32) as usize);
                                    let _s: Ptr<brunsli_internal_enc_EntropySource> =
                                        (entropy_source).clone();
                                    (*data_stream.upgrade().deref())
                                        .AddCode(_code, _band, _context, _s)
                                });
                                let extra_bits: Value<i32> = Rc::new(RefCell::new(
                                    ((*base_code.borrow()) - (2 << (*nbits.borrow()))),
                                ));
                                let first_extra_bit: Value<i32> = Rc::new(RefCell::new(
                                    (((*extra_bits.borrow()) >> (*nbits.borrow())) & 1),
                                ));
                                let p: Value<Ptr<brunsli_Prob>> = Rc::new(RefCell::new(
                                    (((*(*c.borrow()).upgrade().deref())
                                        .first_extra_bit_prob
                                        .as_pointer()
                                        as Ptr<brunsli_Prob>)
                                        .offset(
                                            ((((*k.borrow()) * 10) + (*nbits.borrow())) as usize)
                                                as isize,
                                        )),
                                ));
                                ({
                                    let _p: Ptr<brunsli_Prob> = (*p.borrow()).clone();
                                    let _bit: i32 = (*first_extra_bit.borrow());
                                    (*data_stream.upgrade().deref()).AddBit(_p, _bit)
                                });
                                if ((*nbits.borrow()) > 0) {
                                    let left_over_bits: Value<i32> = Rc::new(RefCell::new(
                                        ((*extra_bits.borrow()) & ((1 << (*nbits.borrow())) - 1)),
                                    ));
                                    ({
                                        let _nbits: i32 = (*nbits.borrow());
                                        let _bits: i32 = (*left_over_bits.borrow());
                                        (*data_stream.upgrade().deref()).AddBits(_nbits, _bits)
                                    });
                                }
                            }
                            (*num_nzeros.borrow_mut()).prefix_inc();
                            (*encoded_coeffs.borrow_mut())[(*k_nat.borrow()) as usize] =
                                (*coeff.borrow());
                            let __rhs = (*absval.borrow());
                            (*prev_abs.borrow())
                                .offset((*k.borrow()) as isize)
                                .write(__rhs);
                        } else {
                            (*prev_sgn.borrow()).offset((*k.borrow()) as isize).write(0);
                            (*prev_abs.borrow()).offset((*k.borrow()) as isize).write(0);
                        }
                        (*k.borrow_mut()).prefix_dec();
                    }
                    if !((*num_nzeros.borrow())
                        <= (*kNumNonZeroTreeSize_85.with(Value::clone).borrow()))
                    {
                        ({
                            let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeAC");
                            BrunsliDumpAndAbort_79(
                                Ptr::from_string_literal(b"brunsli_encode.cc"),
                                1329,
                                _fn,
                            )
                        });
                        'loop_: while true {}
                    };
                    ((*(*c.borrow()).upgrade().deref())
                        .prev_num_nonzeros
                        .as_pointer() as Ptr<u8>)
                        .offset(((*x.borrow()) as usize) as isize)
                        .write(((*num_nzeros.borrow()) as u8));
                    (*block_state.borrow_mut()).prefix_inc();
                    (*coeffs_in.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*prev_sgn.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*prev_abs.borrow_mut()) += (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*prev_row_coeffs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*prev_col_coeffs.borrow_mut()) +=
                        (*kDCTBlockSize_3.with(Value::clone).borrow());
                    (*x.borrow_mut()).prefix_inc();
                }
                (*prev_row_delta.borrow_mut()) *= -1_i32;
                (*iy.borrow_mut()).prefix_inc();
                (*y.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*mcu_y.borrow_mut()).prefix_inc();
    }
}
pub fn PrepareEntropyCodes_188(
    state: Ptr<brunsli_internal_enc_State>,
) -> Option<Value<brunsli_internal_enc_EntropyCodes>> {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> =
        (*(*state.borrow()).upgrade().deref()).meta.as_pointer();
    let num_components: Value<usize> = Rc::new(RefCell::new((*meta.upgrade().deref()).len()));
    let group_context_offsets: Value<Vec<u64>> = Rc::new(RefCell::new(
        (0..((1_usize).wrapping_add((*num_components.borrow()))) as usize)
            .map(|_| <u64>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ((*(*(meta.to_strong().as_pointer()
            as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .context_offset
        .borrow()) as u64);
        (group_context_offsets.as_pointer() as Ptr<u64>)
            .offset((*i.borrow()).wrapping_add(1_usize) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    return ({
        let _offsets: Ptr<Vec<u64>> = group_context_offsets.as_pointer();
        (*(*(*state.borrow()).upgrade().deref())
            .entropy_source
            .borrow())
        .Finish(_offsets)
    });
}
pub fn BrunsliSerialize_189(
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
    let encode_section: Value<_> = Rc::new(RefCell::new(
        (|tag: u8,
          fn_: FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >,
          size: usize| {
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
                let _s: Ptr<brunsli_internal_enc_State> = (*state.borrow()).clone();
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
                let _len: usize = ((*len.borrow()).read());
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (pos.as_pointer());
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
        }),
    ));
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliSignatureTag_30.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _len: usize = ((*len.borrow()).read());
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (pos.as_pointer());
            EncodeSignature_170(_len, _data, _pos)
        });
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliHeaderTag_31.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliHeaderTag_31.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeHeader_172);
            (*encode_section.borrow_mut())(_tag, _fn, 1_usize)
        });
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliJPEGInternalsTag_33.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeJPEGInternals_174);
            let _size: usize = ({
                let _val: usize = ({
                    let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
                    EstimateAuxDataSize_143(_jpg)
                });
                Base128Size_145(_val)
            });
            (*encode_section.borrow_mut())(_tag, _fn, _size)
        })
        .clone();
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliMetaDataTag_32.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliMetaDataTag_32.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeMetaData_173);
            let _size: usize = ({
                let _val: usize = ((*len.borrow()).read()).wrapping_sub((*pos.borrow()));
                Base128Size_145(_val)
            });
            (*encode_section.borrow_mut())(_tag, _fn, _size)
        })
        .clone();
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliQuantDataTag_34.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliQuantDataTag_34.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeQuantData_175);
            (*encode_section.borrow_mut())(_tag, _fn, 2_usize)
        });
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliHistogramDataTag_35.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliHistogramDataTag_35.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeHistogramData_176);
            let _size: usize = ({
                let _val: usize = ((*len.borrow()).read()).wrapping_sub((*pos.borrow()));
                Base128Size_145(_val)
            });
            (*encode_section.borrow_mut())(_tag, _fn, _size)
        })
        .clone();
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliDCDataTag_36.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliDCDataTag_36.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeDCData_177);
            let _size: usize = ({
                let _val: usize = ((*len.borrow()).read()).wrapping_sub((*pos.borrow()));
                Base128Size_145(_val)
            });
            (*encode_section.borrow_mut())(_tag, _fn, _size)
        })
        .clone();
        if !(*ok.borrow()) {
            return false;
        }
    }
    if !(((*skip_sections.borrow())
        & (1_u32 << ((*kBrunsliACDataTag_37.with(Value::clone).borrow()) as i32)))
        != 0)
    {
        (*ok.borrow_mut()) = ({
            let _tag: u8 = (*kBrunsliACDataTag_37.with(Value::clone).borrow());
            let _fn: FnPtr<
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
            >::new(EncodeACData_178);
            let _size: usize = ({
                let _val: usize = ((*len.borrow()).read()).wrapping_sub((*pos.borrow()));
                Base128Size_145(_val)
            });
            (*encode_section.borrow_mut())(_tag, _fn, _size)
        })
        .clone();
        if !(*ok.borrow()) {
            return false;
        }
    }
    let __rhs = (*pos.borrow());
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn BrunsliEncodeJpeg_190(jpg: Ptr<brunsli_JPEGData>, data: Ptr<u8>, len: Ptr<usize>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    let state: Value<brunsli_internal_enc_State> =
        Rc::new(RefCell::new(<brunsli_internal_enc_State>::default()));
    let meta: Ptr<Vec<brunsli_internal_enc_ComponentMeta>> = (*state.borrow()).meta.as_pointer();
    let num_components: Value<usize> = Rc::new(RefCell::new(
        (*(*jpg.upgrade().deref()).components.borrow()).len(),
    ));
    (*(*state.borrow()).use_legacy_context_model.borrow_mut()) =
        !(((*(*jpg.upgrade().deref()).version.borrow()) & 2) != 0);
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        CalculateMeta_185(_jpg, _state)
    }) {
        return false;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ({
            let _m: Ptr<brunsli_internal_enc_ComponentMeta> =
                (((*state.borrow()).meta.as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()) as isize));
            SampleNumNonZeros_180(_m)
        });
        (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .approx_total_nonzeros
        .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        let __rhs = ({
            let _num_symbols: usize = (*(*(meta.to_strong().as_pointer()
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .approx_total_nonzeros
            .borrow())
            .wrapping_add(1_usize);
            SelectContextBits_182(_num_symbols)
        });
        (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .context_bits
        .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let num_contexts: Value<usize> = Rc::new(RefCell::new((*num_components.borrow())));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .context_offset
        .borrow_mut()) = (*num_contexts.borrow());
        let rhs_0 = (*num_contexts.borrow()).wrapping_add(
            ((*kNumNonzeroContextSkip_94.with(Value::clone).borrow())[(*(*(meta
                .to_strong()
                .as_pointer()
                as Ptr<brunsli_internal_enc_ComponentMeta>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .context_bits
            .borrow())
                as usize] as usize),
        );
        (*num_contexts.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    (*(*state.borrow()).num_contexts.borrow_mut()) = (*num_contexts.borrow());
    let dc_prediction_errors: Value<Vec<Value<Vec<i16>>>> = Rc::new(RefCell::new(
        (0..(*num_components.borrow()) as usize)
            .map(|_| <Value<Vec<i16>>>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_components.borrow())) {
        {
            let __a0 = (({
                let _lhs = (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .width_in_blocks
                .borrow());
                _lhs * (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .height_in_blocks
                .borrow())
            }) as usize) as usize;
            (dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>)
                .offset((*i.borrow()) as isize)
                .with_mut(|__v: &mut Value<Vec<i16>>| {
                    (*__v.borrow_mut()).resize_with(__a0, || <i16>::default())
                })
        };
        let __rhs = ((dc_prediction_errors.as_pointer() as Ptr<Value<Vec<i16>>>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<i16>);
        (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .dc_prediction_errors
        .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    if !({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        PredictDCCoeffs_184(_state)
    }) {
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
                let _lhs = (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .width_in_blocks
                .borrow());
                _lhs * (*(*(meta.to_strong().as_pointer()
                    as Ptr<brunsli_internal_enc_ComponentMeta>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .height_in_blocks
                .borrow())
            }) as usize) as usize;
            (block_state.as_pointer() as Ptr<Value<Vec<u8>>>)
                .offset((*i.borrow()) as isize)
                .with_mut(|__v: &mut Value<Vec<u8>>| {
                    (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                })
        };
        let __rhs = ((block_state.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>);
        (*(*(meta.to_strong().as_pointer() as Ptr<brunsli_internal_enc_ComponentMeta>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .block_state
        .borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        EncodeDC_186(_state)
    });
    ({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        EncodeAC_187(_state)
    });
    let entropy_codes: Value<Option<Value<brunsli_internal_enc_EntropyCodes>>> =
        Rc::new(RefCell::new(
            ({
                let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
                PrepareEntropyCodes_188(_state)
            }),
        ));
    (*(*state.borrow()).entropy_codes.borrow_mut()) = (*entropy_codes.borrow()).as_pointer();
    return ({
        let _state: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _jpg: Ptr<brunsli_JPEGData> = (jpg).clone();
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _len: Ptr<usize> = (*len.borrow()).clone();
        BrunsliSerialize_189(_state, _jpg, 0_u32, _data, _len)
    });
}
thread_local!(
    pub static kMaxBypassHeaderSize_191: Value<usize> = Rc::new(RefCell::new(((5 * 6) as usize)));
);
pub fn GetBrunsliBypassSize_192(jpg_size: usize) -> usize {
    let jpg_size: Value<usize> = Rc::new(RefCell::new(jpg_size));
    return ((*jpg_size.borrow())
        .wrapping_add((*kBrunsliSignatureSize_43.with(Value::clone).borrow())))
    .wrapping_add((*kMaxBypassHeaderSize_191.with(Value::clone).borrow()));
}
pub fn EncodeOriginalJpg_193(
    jpg: Ptr<brunsli_JPEGData>,
    state: Ptr<brunsli_internal_enc_State>,
    data: Ptr<u8>,
    len: Ptr<usize>,
) -> bool {
    let state: Value<Ptr<brunsli_internal_enc_State>> = Rc::new(RefCell::new(state));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<Ptr<usize>> = Rc::new(RefCell::new(len));
    (*state.borrow_mut()).clone();
    if ((*(*jpg.upgrade().deref()).original_jpg.borrow()).is_null())
        || ({
            let _lhs = (*(*jpg.upgrade().deref()).original_jpg_size.borrow());
            _lhs > ((*len.borrow()).read())
        })
    {
        return false;
    }
    {
        ((*data.borrow()).clone() as Ptr<u8>).to_any().memcpy(
            &((*(*jpg.upgrade().deref()).original_jpg.borrow()).clone() as Ptr<u8>).to_any(),
            (*(*jpg.upgrade().deref()).original_jpg_size.borrow()) as usize,
        );
        ((*data.borrow()).clone() as Ptr<u8>).to_any().clone()
    };
    let __rhs = (*(*jpg.upgrade().deref()).original_jpg_size.borrow());
    (*len.borrow()).write(__rhs);
    return true;
}
pub fn BrunsliEncodeJpegBypass_194(
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
        EncodeSignature_170(_len, _data, _pos)
    }) {
        return false;
    }
    let jpg: Value<brunsli_JPEGData> = Rc::new(RefCell::new(brunsli_JPEGData::brunsli_JPEGData()));
    if !({
        let _data: Ptr<u8> = (*jpg_data.borrow()).clone();
        let _len: usize = (*jpg_data_len.borrow());
        let _mode: brunsli_JpegReadMode = brunsli_JpegReadMode::JPEG_READ_HEADER;
        let _jpg: Ptr<brunsli_JPEGData> = (jpg.as_pointer());
        ReadJpeg_195(_data, _len, _mode, _jpg)
    }) {
        (*(*jpg.borrow()).width.borrow_mut()) = 0;
        (*(*jpg.borrow()).height.borrow_mut()) = 0;
        {
            let __a0 = 1_usize as usize;
            (*(*jpg.borrow()).components.borrow_mut())
                .resize_with(__a0, || <brunsli_JPEGComponent>::default())
        };
        (*(*((*jpg.borrow()).components.as_pointer() as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow_mut()) = 1;
        (*(*((*jpg.borrow()).components.as_pointer() as Ptr<brunsli_JPEGComponent>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow_mut()) = 1;
    }
    (*(*jpg.borrow()).version.borrow_mut()) = (*kFallbackVersion_2.with(Value::clone).borrow());
    (*(*jpg.borrow()).original_jpg.borrow_mut()) = (*jpg_data.borrow()).clone();
    (*(*jpg.borrow()).original_jpg_size.borrow_mut()) = (*jpg_data_len.borrow());
    let state: Value<brunsli_internal_enc_State> =
        Rc::new(RefCell::new(<brunsli_internal_enc_State>::default()));
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _s: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _tag: u8 = (*kBrunsliHeaderTag_31.with(Value::clone).borrow());
        let _write_section: FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        > = FnPtr::<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >::new(EncodeHeader_172);
        let _len: usize = ((*len.borrow()).read());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        EncodeSection_179(_jpg, _s, _tag, _write_section, 1_usize, _len, _data, _pos)
    }) {
        return false;
    }
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _s: Ptr<brunsli_internal_enc_State> = (state.as_pointer());
        let _tag: u8 = (*kBrunsliOriginalJpgTag_38.with(Value::clone).borrow());
        let _write_section: FnPtr<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        > = FnPtr::<
            fn(Ptr<brunsli_JPEGData>, Ptr<brunsli_internal_enc_State>, Ptr<u8>, Ptr<usize>) -> bool,
        >::new(EncodeOriginalJpg_193);
        let _section_size_bytes: usize = ({
            let _val: usize = (*jpg_data_len.borrow());
            Base128Size_145(_val)
        });
        let _len: usize = ((*len.borrow()).read());
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
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
    let __rhs = (*pos.borrow());
    (*len.borrow()).write(__rhs);
    return true;
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
#[derive(Default)]
pub struct brunsli_HuffmanTree {
    pub total_count: Value<u32>,
    pub index_left: Value<i16>,
    pub index_right_or_value: Value<i16>,
}
impl brunsli_HuffmanTree {
    pub fn brunsli_HuffmanTree(count: u32, left: i16, right: i16) -> Self {
        let count: Value<u32> = Rc::new(RefCell::new(count));
        let left: Value<i16> = Rc::new(RefCell::new(left));
        let right: Value<i16> = Rc::new(RefCell::new(right));
        let mut this = Self {
            total_count: Rc::new(RefCell::new((*count.borrow()))),
            index_left: Rc::new(RefCell::new((*left.borrow()))),
            index_right_or_value: Rc::new(RefCell::new((*right.borrow()))),
        };
        this
    }
}
impl Clone for brunsli_HuffmanTree {
    fn clone(&self) -> Self {
        let mut this = Self {
            total_count: Rc::new(RefCell::new((*self.total_count.borrow()))),
            index_left: Rc::new(RefCell::new((*self.index_left.borrow()))),
            index_right_or_value: Rc::new(RefCell::new((*self.index_right_or_value.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_HuffmanTree {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.total_count.borrow()).to_bytes(&mut buf[0..4]);
        (*self.index_left.borrow()).to_bytes(&mut buf[4..6]);
        (*self.index_right_or_value.borrow()).to_bytes(&mut buf[6..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            total_count: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            index_left: Rc::new(RefCell::new(<i16>::from_bytes(&buf[4..6]))),
            index_right_or_value: Rc::new(RefCell::new(<i16>::from_bytes(&buf[6..8]))),
        }
    }
}
pub fn StoreVarLenUint8_196(n: usize, storage: Ptr<brunsli_Storage>) {
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    if ((*n.borrow()) == 0_usize) {
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 0_u64, _storage)
        });
    } else {
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 1_u64, _storage)
        });
        let nbits: Value<usize> = Rc::new(RefCell::new(
            (({
                let _n: u32 = ((*n.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            }) as usize),
        ));
        ({
            let _bits: u64 = ((*nbits.borrow()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(3_usize, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*nbits.borrow());
            let _bits: u64 = ((*n.borrow()).wrapping_sub((1_usize << (*nbits.borrow()))) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    }
}
pub fn IndexOf_197(v: Ptr<Vec<u32>>, value: u32) -> usize {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*v.upgrade().deref()).len()
    } {
        if {
            let _lhs = ((v.to_strong().as_pointer() as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read());
            _lhs == (*value.borrow())
        } {
            return (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*i.borrow());
}
pub fn MoveToFront_198(v: Ptr<Vec<u32>>, index: usize) {
    let v: Value<Ptr<Vec<u32>>> = Rc::new(RefCell::new(v));
    let index: Value<usize> = Rc::new(RefCell::new(index));
    let value: Value<u32> = Rc::new(RefCell::new(
        ((((*v.borrow()).to_strong().as_pointer()) as Ptr<u32>)
            .offset((*index.borrow()) as isize)
            .read()),
    ));
    let i: Value<usize> = Rc::new(RefCell::new((*index.borrow())));
    'loop_: while ((*i.borrow()) != 0_usize) {
        let __rhs = ((((*v.borrow()).to_strong().as_pointer()) as Ptr<u32>)
            .offset((*i.borrow()).wrapping_sub(1_usize) as isize)
            .read());
        (((*v.borrow()).to_strong().as_pointer()) as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_dec();
    }
    (((*v.borrow()).to_strong().as_pointer()) as Ptr<u32>)
        .offset(0_usize as isize)
        .write((*value.borrow()));
}
pub fn MoveToFrontTransform_199(v: Ptr<Vec<u32>>) -> Vec<u32> {
    if (*v.upgrade().deref()).is_empty() {
        return (*v.upgrade().deref()).clone();
    }
    let max_value: Value<u32> = Rc::new(RefCell::new(
        ({
            let __count = (v.to_strong().as_pointer() as Ptr<u32>)
                .to_end()
                .get_offset()
                - (v.to_strong().as_pointer() as Ptr<u32>).get_offset();
            let max_index = PtrValueIter::new(&(v.to_strong().as_pointer() as Ptr<u32>), __count)
                .enumerate()
                .max_by_key(|&(_, val)| val)
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            (v.to_strong().as_pointer() as Ptr<u32>) + max_index
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
        (mtf.as_pointer() as Ptr<u32>)
            .offset(((*i.borrow()) as usize) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let result: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..((*v.upgrade().deref()).len()) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*v.upgrade().deref()).len()
    } {
        let index: Value<usize> = Rc::new(RefCell::new(
            ({
                let _v: Ptr<Vec<u32>> = mtf.as_pointer();
                let _value: u32 = ((v.to_strong().as_pointer() as Ptr<u32>)
                    .offset((*i.borrow()) as isize)
                    .read());
                IndexOf_197(_v, _value)
            }),
        ));
        if !((*index.borrow()) < (*mtf.borrow()).len()) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"MoveToFrontTransform");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"context_map_encode.cc"), 60, _fn)
            });
            'loop_: while true {}
        };
        (result.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .write(((*index.borrow()) as u32));
        ({
            let _v: Ptr<Vec<u32>> = (mtf.as_pointer());
            let _index: usize = (*index.borrow());
            MoveToFront_198(_v, _index)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    return (*result.borrow_mut()).clone();
}
pub fn RunLengthCodeZeros_200(
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
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*v_in.upgrade().deref()).len()
    } {
        'loop_: while ({
            let _lhs = (*i.borrow());
            _lhs < (*v_in.upgrade().deref()).len()
        }) && (((v_in.to_strong().as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .read())
            != 0_u32)
        {
            (*i.borrow_mut()).prefix_inc();
        }
        let i0: Value<usize> = Rc::new(RefCell::new((*i.borrow())));
        'loop_: while ({
            let _lhs = (*i.borrow());
            _lhs < (*v_in.upgrade().deref()).len()
        }) && (((v_in.to_strong().as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .read())
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
            ({
                let _n: u32 = ((*max_reps.borrow()) as u32);
                Log2FloorNonZero_74(_n)
            })
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
    let __rhs = (*max_prefix.borrow());
    (*max_run_length_prefix.borrow()).write(__rhs);
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*v_in.upgrade().deref()).len()
    } {
        if (((v_in.to_strong().as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .read())
            != 0_u32)
        {
            (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| {
                __v.push(
                    ((v_in.to_strong().as_pointer() as Ptr<u32>)
                        .offset((*i.borrow()) as isize)
                        .read())
                    .wrapping_add(((*max_run_length_prefix.borrow()).read())),
                )
            });
            (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(0_u32));
            (*i.borrow_mut()).prefix_inc();
        } else {
            let reps: Value<u32> = Rc::new(RefCell::new(1_u32));
            let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
            'loop_: while ({
                let _lhs = (*k.borrow());
                _lhs < (*v_in.upgrade().deref()).len()
            }) && (((v_in.to_strong().as_pointer() as Ptr<u32>)
                .offset((*k.borrow()) as isize)
                .read())
                == 0_u32)
            {
                (*reps.borrow_mut()).prefix_inc();
                (*k.borrow_mut()).prefix_inc();
            }
            let rhs_0 = (*i.borrow()).wrapping_add(((*reps.borrow()) as usize));
            (*i.borrow_mut()) = rhs_0;
            'loop_: while ((*reps.borrow()) != 0_u32) {
                if ((*reps.borrow()) < (2_u32 << (*max_prefix.borrow()))) {
                    let run_length_prefix: Value<u32> = Rc::new(RefCell::new(
                        (({
                            let _n: u32 = (*reps.borrow());
                            Log2FloorNonZero_74(_n)
                        }) as u32),
                    ));
                    {
                        let a0_clone = (*run_length_prefix.borrow()).clone();
                        (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| {
                        __v.push(
                            (*reps.borrow()).wrapping_sub((1_u32 << (*run_length_prefix.borrow()))),
                        )
                    });
                    break;
                } else {
                    {
                        let a0_clone = (*max_prefix.borrow()).clone();
                        (*v_out.borrow()).with_mut(|__v: &mut Vec<u32>| __v.push(a0_clone))
                    };
                    (*extra_bits.borrow()).with_mut(|__v: &mut Vec<u32>| {
                        __v.push((1_u32 << (*max_prefix.borrow())).wrapping_sub((1_u32 as u32)))
                    });
                    let rhs_0 = (((*reps.borrow()) as u32).wrapping_sub(
                        (2_u32 << (*max_prefix.borrow())).wrapping_sub((1_u32 as u32)),
                    )) as u32;
                    (*reps.borrow_mut()) = rhs_0;
                }
            }
        };
    }
}
pub fn EncodeContextMap_163(
    context_map: Ptr<Vec<u32>>,
    num_clusters: usize,
    storage: Ptr<brunsli_Storage>,
) {
    let num_clusters: Value<usize> = Rc::new(RefCell::new(num_clusters));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    ({
        let _n: usize = (*num_clusters.borrow()).wrapping_sub(1_usize);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        StoreVarLenUint8_196(_n, _storage)
    });
    if ((*num_clusters.borrow()) == 1_usize) {
        return;
    }
    let transformed_symbols: Value<Vec<u32>> = Rc::new(RefCell::new(
        ({
            let _v: Ptr<Vec<u32>> = (context_map).clone();
            MoveToFrontTransform_199(_v)
        }),
    ));
    let rle_symbols: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let extra_bits: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let max_run_length_prefix: Value<u32> = Rc::new(RefCell::new(6_u32));
    ({
        let _v_in: Ptr<Vec<u32>> = transformed_symbols.as_pointer();
        let _max_run_length_prefix: Ptr<u32> = (max_run_length_prefix.as_pointer());
        let _v_out: Ptr<Vec<u32>> = (rle_symbols.as_pointer());
        let _extra_bits: Ptr<Vec<u32>> = (extra_bits.as_pointer());
        RunLengthCodeZeros_200(_v_in, _max_run_length_prefix, _v_out, _extra_bits)
    });
    let symbol_histogram: Value<Box<[u32]>> = Rc::new(RefCell::new(
        (0..272).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
    ));
    {
        ((symbol_histogram.as_pointer() as Ptr<u32>) as Ptr<u32>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u32; 272]>() as usize);
        ((symbol_histogram.as_pointer() as Ptr<u32>) as Ptr<u32>)
            .to_any()
            .clone()
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*rle_symbols.borrow()).len()) {
        (*symbol_histogram.borrow_mut())[((rle_symbols.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let use_rle: Value<bool> = Rc::new(RefCell::new(((*max_run_length_prefix.borrow()) > 0_u32)));
    ({
        let _bits: u64 = ((*use_rle.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, _bits, _storage)
    });
    if (*use_rle.borrow()) {
        ({
            let _bits: u64 = (((*max_run_length_prefix.borrow()).wrapping_sub(1_u32)) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(4_usize, _bits, _storage)
        });
    }
    let bit_depths: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..272).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let bit_codes: Value<Box<[u16]>> = Rc::new(RefCell::new(
        (0..272).map(|_| <u16>::default()).collect::<Box<[u16]>>(),
    ));
    {
        ((bit_depths.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 272]>() as usize);
        ((bit_depths.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .clone()
    };
    {
        ((bit_codes.as_pointer() as Ptr<u16>) as Ptr<u16>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u16; 272]>() as usize);
        ((bit_codes.as_pointer() as Ptr<u16>) as Ptr<u16>)
            .to_any()
            .clone()
    };
    ({
        let _histogram: Ptr<u32> = (symbol_histogram.as_pointer() as Ptr<u32>);
        let _length: usize =
            (*num_clusters.borrow()).wrapping_add(((*max_run_length_prefix.borrow()) as usize));
        let _depth: Ptr<u8> = (bit_depths.as_pointer() as Ptr<u8>);
        let _bits: Ptr<u16> = (bit_codes.as_pointer() as Ptr<u16>);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        BuildAndStoreHuffmanTree_201(_histogram, _length, _depth, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*rle_symbols.borrow()).len()) {
        ({
            let _n_bits: usize = ((*bit_depths.borrow())[((rle_symbols.as_pointer() as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read()) as usize] as usize);
            let _bits: u64 = ((*bit_codes.borrow())[((rle_symbols.as_pointer() as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read()) as usize] as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        if (((rle_symbols.as_pointer() as Ptr<u32>)
            .offset((*i.borrow()) as isize)
            .read())
            > 0_u32)
            && (((rle_symbols.as_pointer() as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read())
                <= (*max_run_length_prefix.borrow()))
        {
            ({
                let _n_bits: usize = (((rle_symbols.as_pointer() as Ptr<u32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as usize);
                let _bits: u64 = (((extra_bits.as_pointer() as Ptr<u32>)
                    .offset((*i.borrow()) as isize)
                    .read()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, _bits, _storage)
            });
        }
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(1_usize, 1_u64, _storage)
    });
}
pub fn GetPopulationCountPrecision_202(logcount: u32) -> u32 {
    let logcount: Value<u32> = Rc::new(RefCell::new(logcount));
    return (((*logcount.borrow()).wrapping_add(1_u32)) >> 1);
}
thread_local!(
    pub static kHistogramLengthBitLengths_203: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([
            8_u8, 8_u8, 6_u8, 6_u8, 6_u8, 5_u8, 4_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 3_u8, 4_u8,
            5_u8, 7_u8,
        ])));
);
thread_local!(
    pub static kHistogramLengthSymbols_204: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        127_u16, 255_u16, 15_u16, 47_u16, 31_u16, 7_u16, 3_u16, 0_u16, 4_u16, 2_u16, 6_u16, 1_u16,
        5_u16, 11_u16, 23_u16, 63_u16,
    ])));
);
thread_local!(
    pub static kLogCountBitLengths_205: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        5_u8, 4_u8, 4_u8, 4_u8, 3_u8, 3_u8, 2_u8, 3_u8, 3_u8, 6_u8, 6_u8,
    ])));
);
thread_local!(
    pub static kLogCountSymbols_206: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        15_u16, 3_u16, 11_u16, 7_u16, 2_u16, 6_u16, 0_u16, 1_u16, 5_u16, 31_u16, 63_u16,
    ])));
);
pub fn SmallestIncrement_207(count: i32) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    if !((*count.borrow()) > 0) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"SmallestIncrement");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 39, _fn)
        });
        'loop_: while true {}
    };
    let bits: Value<i32> = Rc::new(RefCell::new(
        ({
            let _n: u32 = ((*count.borrow()) as u32);
            Log2FloorNonZero_74(_n)
        }),
    ));
    let drop_bits: Value<i32> = Rc::new(RefCell::new(
        ((((*bits.borrow()) as u32).wrapping_sub(
            ({
                let _logcount: u32 = ((*bits.borrow()) as u32);
                GetPopulationCountPrecision_202(_logcount)
            }),
        )) as i32),
    ));
    return (1 << (*drop_bits.borrow()));
}
pub fn RebalanceHistogram_208(
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"RebalanceHistogram");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 48, _fn)
        });
        'loop_: while true {}
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let sum_nonrounded: Value<f32> = Rc::new(RefCell::new((0.0E+0 as f32)));
    let remainder_pos: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let remainder_log: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        if (((*targets.borrow()).offset((*n.borrow()) as isize).read()) > 0_f32) {
            let __rhs = ((*targets.borrow()).offset((*n.borrow()) as isize).read());
            (*sum_nonrounded.borrow_mut()) += __rhs;
            let __rhs = ((((((*targets.borrow()).offset((*n.borrow()) as isize).read()) as f64)
                + 5.0E-1) as u32) as i32);
            (*counts.borrow())
                .offset((*n.borrow()) as isize)
                .write(__rhs);
            if (((*counts.borrow()).offset((*n.borrow()) as isize).read()) == 0) {
                (*counts.borrow()).offset((*n.borrow()) as isize).write(1);
            }
            if {
                let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                _lhs == (*table_size.borrow())
            } {
                let __rhs = ((*table_size.borrow()) - 1);
                (*counts.borrow())
                    .offset((*n.borrow()) as isize)
                    .write(__rhs);
            }
            let inc: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _count: i32 = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                    SmallestIncrement_207(_count)
                }),
            ));
            let __rhs = {
                let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                _lhs & ((*inc.borrow()) - 1)
            };
            {
                let _ptr = (*counts.borrow()).offset((*n.borrow()) as isize).clone();
                _ptr.write(_ptr.read() - __rhs)
            };
            let target: Value<f32> = Rc::new(RefCell::new(if false {
                ((*sum_nonrounded.borrow()) - ((*sum.borrow()) as f32))
            } else {
                ((*targets.borrow()).offset((*n.borrow()) as isize).read())
            }));
            if (((*counts.borrow()).offset((*n.borrow()) as isize).read()) == 0)
                || (({
                    let _lhs = (*target.borrow());
                    _lhs > (({
                        let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                        _lhs + ((*inc.borrow()) / 2)
                    }) as f32)
                }) && ({
                    let _lhs = {
                        let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                        _lhs + (*inc.borrow())
                    };
                    _lhs < (*table_size.borrow())
                }))
            {
                let __rhs = (*inc.borrow());
                {
                    let _ptr = (*counts.borrow()).offset((*n.borrow()) as isize).clone();
                    _ptr.write(_ptr.read() + __rhs)
                };
            }
            let __rhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
            (*sum.borrow_mut()) += __rhs;
            let count_log: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _n: u32 =
                        (((*counts.borrow()).offset((*n.borrow()) as isize).read()) as u32);
                    Log2FloorNonZero_74(_n)
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"RebalanceHistogram");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 81, _fn)
        });
        'loop_: while true {}
    };
    let __rhs = ((*sum.borrow()) - (*table_size.borrow()));
    {
        let _ptr = (*counts.borrow())
            .offset((*remainder_pos.borrow()) as isize)
            .clone();
        _ptr.write(_ptr.read() - __rhs)
    };
    let __rhs = (*remainder_pos.borrow());
    (*omit_pos.borrow()).write(__rhs);
    return (((*counts.borrow())
        .offset((*remainder_pos.borrow()) as isize)
        .read())
        > 0);
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"RebalanceHistogram");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 48, _fn)
        });
        'loop_: while true {}
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let sum_nonrounded: Value<f32> = Rc::new(RefCell::new((0.0E+0 as f32)));
    let remainder_pos: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let remainder_log: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        if (((*targets.borrow()).offset((*n.borrow()) as isize).read()) > 0_f32) {
            let __rhs = ((*targets.borrow()).offset((*n.borrow()) as isize).read());
            (*sum_nonrounded.borrow_mut()) += __rhs;
            let __rhs = ((((((*targets.borrow()).offset((*n.borrow()) as isize).read()) as f64)
                + 5.0E-1) as u32) as i32);
            (*counts.borrow())
                .offset((*n.borrow()) as isize)
                .write(__rhs);
            if (((*counts.borrow()).offset((*n.borrow()) as isize).read()) == 0) {
                (*counts.borrow()).offset((*n.borrow()) as isize).write(1);
            }
            if {
                let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                _lhs == (*table_size.borrow())
            } {
                let __rhs = ((*table_size.borrow()) - 1);
                (*counts.borrow())
                    .offset((*n.borrow()) as isize)
                    .write(__rhs);
            }
            let inc: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _count: i32 = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                    SmallestIncrement_207(_count)
                }),
            ));
            let __rhs = {
                let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                _lhs & ((*inc.borrow()) - 1)
            };
            {
                let _ptr = (*counts.borrow()).offset((*n.borrow()) as isize).clone();
                _ptr.write(_ptr.read() - __rhs)
            };
            let target: Value<f32> = Rc::new(RefCell::new(if true {
                ((*sum_nonrounded.borrow()) - ((*sum.borrow()) as f32))
            } else {
                ((*targets.borrow()).offset((*n.borrow()) as isize).read())
            }));
            if (((*counts.borrow()).offset((*n.borrow()) as isize).read()) == 0)
                || (({
                    let _lhs = (*target.borrow());
                    _lhs > (({
                        let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                        _lhs + ((*inc.borrow()) / 2)
                    }) as f32)
                }) && ({
                    let _lhs = {
                        let _lhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
                        _lhs + (*inc.borrow())
                    };
                    _lhs < (*table_size.borrow())
                }))
            {
                let __rhs = (*inc.borrow());
                {
                    let _ptr = (*counts.borrow()).offset((*n.borrow()) as isize).clone();
                    _ptr.write(_ptr.read() + __rhs)
                };
            }
            let __rhs = ((*counts.borrow()).offset((*n.borrow()) as isize).read());
            (*sum.borrow_mut()) += __rhs;
            let count_log: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _n: u32 =
                        (((*counts.borrow()).offset((*n.borrow()) as isize).read()) as u32);
                    Log2FloorNonZero_74(_n)
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"RebalanceHistogram");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 81, _fn)
        });
        'loop_: while true {}
    };
    let __rhs = ((*sum.borrow()) - (*table_size.borrow()));
    {
        let _ptr = (*counts.borrow())
            .offset((*remainder_pos.borrow()) as isize)
            .clone();
        _ptr.write(_ptr.read() - __rhs)
    };
    let __rhs = (*remainder_pos.borrow());
    (*omit_pos.borrow()).write(__rhs);
    return (((*counts.borrow())
        .offset((*remainder_pos.borrow()) as isize)
        .read())
        > 0);
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"NormalizeCounts");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 89, _fn)
        });
        'loop_: while true {}
    };
    let table_size: Value<i32> = Rc::new(RefCell::new((1 << (*precision_bits.borrow()))));
    let total: Value<u64> = Rc::new(RefCell::new(0_u64));
    let max_symbol: Value<i32> = Rc::new(RefCell::new(0));
    let symbol_count: Value<i32> = Rc::new(RefCell::new(0));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*length.borrow())) {
        let rhs_0 = (*total.borrow())
            .wrapping_add((((*counts.borrow()).offset((*n.borrow()) as isize).read()) as u64));
        (*total.borrow_mut()) = rhs_0;
        if (((*counts.borrow()).offset((*n.borrow()) as isize).read()) > 0) {
            if ((*symbol_count.borrow())
                < (*kMaxNumSymbolsForSmallCode_121.with(Value::clone).borrow()))
            {
                let __rhs = (*n.borrow());
                (*symbols.borrow())
                    .offset((*symbol_count.borrow()) as isize)
                    .write(__rhs);
            }
            (*symbol_count.borrow_mut()).prefix_inc();
            (*max_symbol.borrow_mut()) = ((*n.borrow()) + 1);
        }
        (*n.borrow_mut()).prefix_inc();
    }
    let __rhs = (*symbol_count.borrow());
    (*num_symbols.borrow()).write(__rhs);
    if ((*symbol_count.borrow()) == 0) {
        return;
    }
    if ((*symbol_count.borrow()) == 1) {
        let __rhs = (*table_size.borrow());
        (*counts.borrow())
            .offset(((*symbols.borrow()).offset((0) as isize).read()) as isize)
            .write(__rhs);
        return;
    }
    if !((*symbol_count.borrow()) <= (*table_size.borrow())) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"NormalizeCounts");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 112, _fn)
        });
        'loop_: while true {}
    };
    let norm: Value<f32> = Rc::new(RefCell::new(
        ((1.0E+0 * ((*table_size.borrow()) as f32)) / ((*total.borrow()) as f32)),
    ));
    let targets: Value<Box<[f32]>> = Rc::new(RefCell::new(
        (0..18).map(|_| <f32>::default()).collect::<Box<[f32]>>(),
    ));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*n.borrow()) < (*max_symbol.borrow())) {
        let __rhs = {
            let _lhs = (*norm.borrow());
            _lhs * (((*counts.borrow()).offset((*n.borrow()) as isize).read()) as f32)
        };
        (*targets.borrow_mut())[(*n.borrow()) as usize] = __rhs;
        (*n.borrow_mut()).prefix_inc();
    }
    if !({
        let _targets: Ptr<f32> = (targets.as_pointer() as Ptr<f32>);
        let _max_symbol: i32 = (*max_symbol.borrow());
        let _table_size: i32 = (*table_size.borrow());
        let _omit_pos: Ptr<i32> = (*omit_pos.borrow()).clone();
        let _counts: Ptr<i32> = (*counts.borrow()).clone();
        RebalanceHistogram_208(_targets, _max_symbol, _table_size, _omit_pos, _counts)
    }) {
        if !({
            let _targets: Ptr<f32> = (targets.as_pointer() as Ptr<f32>);
            let _max_symbol: i32 = (*max_symbol.borrow());
            let _table_size: i32 = (*table_size.borrow());
            let _omit_pos: Ptr<i32> = (*omit_pos.borrow()).clone();
            let _counts: Ptr<i32> = (*counts.borrow()).clone();
            RebalanceHistogram_209(_targets, _max_symbol, _table_size, _omit_pos, _counts)
        }) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"NormalizeCounts");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"histogram_encode.cc"), 126, _fn)
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
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 1_u64, _storage)
        });
        if ((*num_symbols.borrow()) == 0) {
            ({
                let _n_bits: usize = (((*max_bits.borrow()) + 1) as usize);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, 0_u64, _storage)
            });
        } else {
            ({
                let _bits: u64 = (((*num_symbols.borrow()) - 1) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(1_usize, _bits, _storage)
            });
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
                ({
                    let _n_bits: usize = ((*max_bits.borrow()) as usize);
                    let _bits: u64 =
                        (((*symbols.borrow()).offset((*i.borrow()) as isize).read()) as u64);
                    let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                    WriteBits_120(_n_bits, _bits, _storage)
                });
                (*i.borrow_mut()).prefix_inc();
            }
        }
        if ((*num_symbols.borrow()) == 2) {
            ({
                let _n_bits: usize =
                    ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as usize);
                let _bits: u64 = (((*counts.borrow())
                    .offset(((*symbols.borrow()).offset((0) as isize).read()) as isize)
                    .read()) as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, _bits, _storage)
            });
        }
    } else {
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(1_usize, 0_u64, _storage)
        });
        let length: Value<i32> = Rc::new(RefCell::new(0));
        let logcounts: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0,
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
            <i32>::default(),
        ])));
        let omit_log: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            if !({
                let _lhs = ((*counts.borrow()).offset((*i.borrow()) as isize).read());
                _lhs <= (*BRUNSLI_ANS_TAB_SIZE_1.with(Value::clone).borrow())
            }) {
                ({
                    let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeCounts");
                    BrunsliDumpAndAbort_79(
                        Ptr::from_string_literal(b"histogram_encode.cc"),
                        155,
                        _fn,
                    )
                });
                'loop_: while true {}
            };
            if !(((*counts.borrow()).offset((*i.borrow()) as isize).read()) >= 0) {
                ({
                    let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeCounts");
                    BrunsliDumpAndAbort_79(
                        Ptr::from_string_literal(b"histogram_encode.cc"),
                        156,
                        _fn,
                    )
                });
                'loop_: while true {}
            };
            if ((*i.borrow()) == (*omit_pos.borrow())) {
                (*length.borrow_mut()) = ((*i.borrow()) + 1);
            } else if (((*counts.borrow()).offset((*i.borrow()) as isize).read()) > 0) {
                let __rhs = (({
                    let _n: u32 =
                        (((*counts.borrow()).offset((*i.borrow()) as isize).read()) as u32);
                    Log2FloorNonZero_74(_n)
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
                            .offset((*i.borrow()) as isize)
                            .read()
                    {
                        omit_log.as_pointer()
                    } else {
                        (logcounts.as_pointer() as Ptr<i32>).offset((*i.borrow()) as isize)
                    }
                    .read());
                    (*omit_log.borrow_mut()) = __rhs;
                }
            }
            (*i.borrow_mut()).prefix_inc();
        }
        (*logcounts.borrow_mut())[(*omit_pos.borrow()) as usize] = (*omit_log.borrow());
        ({
            let _n_bits: usize = ((*kHistogramLengthBitLengths_203.with(Value::clone).borrow())
                [((*length.borrow()) - 3) as usize] as usize);
            let _bits: u64 = ((*kHistogramLengthSymbols_204.with(Value::clone).borrow())
                [((*length.borrow()) - 3) as usize] as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*length.borrow())) {
            ({
                let _n_bits: usize = ((*kLogCountBitLengths_205.with(Value::clone).borrow())
                    [((*logcounts.borrow())[(*i.borrow()) as usize]) as usize]
                    as usize);
                let _bits: u64 = ((*kLogCountSymbols_206.with(Value::clone).borrow())
                    [((*logcounts.borrow())[(*i.borrow()) as usize]) as usize]
                    as u64);
                let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                WriteBits_120(_n_bits, _bits, _storage)
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
                        let _logcount: u32 =
                            (((*logcounts.borrow())[(*i.borrow()) as usize] - 1) as u32);
                        GetPopulationCountPrecision_202(_logcount)
                    }) as i32),
                ));
                let drop_bits: Value<i32> = Rc::new(RefCell::new(
                    (((*logcounts.borrow())[(*i.borrow()) as usize] - 1) - (*bitcount.borrow())),
                ));
                if !(({
                    let _lhs = ((*counts.borrow()).offset((*i.borrow()) as isize).read());
                    _lhs & ((1 << (*drop_bits.borrow())) - 1)
                }) == 0)
                {
                    ({
                        let _fn: Ptr<u8> = Ptr::from_string_literal(b"EncodeCounts");
                        BrunsliDumpAndAbort_79(
                            Ptr::from_string_literal(b"histogram_encode.cc"),
                            184,
                            _fn,
                        )
                    });
                    'loop_: while true {}
                };
                ({
                    let _n_bits: usize = ((*bitcount.borrow()) as usize);
                    let _bits: u64 = (({
                        let _lhs = ({
                            let _lhs = ((*counts.borrow()).offset((*i.borrow()) as isize).read());
                            _lhs >> (*drop_bits.borrow())
                        });
                        _lhs - (1 << (*bitcount.borrow()))
                    }) as u64);
                    let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                    WriteBits_120(_n_bits, _bits, _storage)
                });
            }
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub fn PopulationCost_130(data: Ptr<i32>, total_count: i32) -> f64 {
    let data: Value<Ptr<i32>> = Rc::new(RefCell::new(data));
    let total_count: Value<i32> = Rc::new(RefCell::new(total_count));
    if ((*total_count.borrow()) == 0) {
        return 7_f64;
    }
    let entropy_bits: Value<f64> = Rc::new(RefCell::new(
        (((*total_count.borrow()) * (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()))
            as f64),
    ));
    let histogram_bits: Value<i32> = Rc::new(RefCell::new(0));
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let length: Value<i32> = Rc::new(RefCell::new(0));
    if ((*total_count.borrow()) > (*BRUNSLI_ANS_TAB_SIZE_1.with(Value::clone).borrow())) {
        let total: Value<u64> = Rc::new(RefCell::new(((*total_count.borrow()) as u64)));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 18) {
            if (((*data.borrow()).offset((*i.borrow()) as isize).read()) > 0) {
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
                >> ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as u64)),
        ));
        let max1: Value<u64> = Rc::new(RefCell::new(
            (((*max0.borrow()).wrapping_mul(((*length.borrow()) as u64)))
                >> ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as u64)),
        ));
        let min_base: Value<u64> = Rc::new(RefCell::new(
            ((((*total.borrow()).wrapping_add((*max0.borrow()))).wrapping_add((*max1.borrow())))
                >> ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as u64)),
        ));
        let rhs_0 = (*total.borrow())
            .wrapping_add((*min_base.borrow()).wrapping_mul(((*count.borrow()) as u64)));
        (*total.borrow_mut()) = rhs_0;
        let kFixBits: Value<i64> = Rc::new(RefCell::new(32_i64));
        let kFixOne: Value<i64> = Rc::new(RefCell::new((1_i64 << (*kFixBits.borrow()))));
        let kDescaleBits: Value<i64> = Rc::new(RefCell::new(
            ((*kFixBits.borrow())
                - ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as i64)),
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
            let rhs_0 = (((*cumul.borrow()) as i64)
                + (((*kDescaleOne.borrow()) - ((*error.borrow()) as i64)) >> 1))
                as u32;
            (*cumul.borrow_mut()) = rhs_0;
        }
        if (((*data.borrow()).offset((0) as isize).read()) > 0) {
            let c: Value<u64> = Rc::new(RefCell::new(
                (((((*data.borrow()).offset((0) as isize).read()) as u64)
                    .wrapping_add((*min_base.borrow())))
                .wrapping_mul(((*mult.borrow()) as u64)))
                .wrapping_add(((*cumul.borrow()) as u64)),
            ));
            let c_descaled: Value<u64> =
                Rc::new(RefCell::new(((*c.borrow()) >> (*kDescaleBits.borrow()))));
            if !((*c_descaled.borrow()) < (1_u64 << 31)) {
                ({
                    let _fn: Ptr<u8> = Ptr::from_string_literal(b"PopulationCost");
                    BrunsliDumpAndAbort_79(
                        Ptr::from_string_literal(b"histogram_encode.cc"),
                        236,
                        _fn,
                    )
                });
                'loop_: while true {}
            };
            let log2count: Value<f64> = Rc::new(RefCell::new(
                ({
                    let _v: i32 = ((*c_descaled.borrow()) as i32);
                    FastLog2_127(_v)
                }),
            ));
            (*entropy_bits.borrow_mut()) -= {
                let _lhs = (((*data.borrow()).offset((0) as isize).read()) as f64);
                _lhs * (*log2count.borrow())
            };
            (*cumul.borrow_mut()) = (((*c.borrow()) & ((*kDescaleMask.borrow()) as u64)) as u32);
        }
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) < (*length.borrow())) {
            if (((*data.borrow()).offset((*i.borrow()) as isize).read()) > 0) {
                let c: Value<u64> = Rc::new(RefCell::new(
                    (((((*data.borrow()).offset((*i.borrow()) as isize).read()) as u64)
                        .wrapping_add((*min_base.borrow())))
                    .wrapping_mul(((*mult.borrow()) as u64)))
                    .wrapping_add(((*cumul.borrow()) as u64)),
                ));
                let c_descaled: Value<u64> =
                    Rc::new(RefCell::new(((*c.borrow()) >> (*kDescaleBits.borrow()))));
                if !((*c_descaled.borrow()) < (1_u64 << 31)) {
                    ({
                        let _fn: Ptr<u8> = Ptr::from_string_literal(b"PopulationCost");
                        BrunsliDumpAndAbort_79(
                            Ptr::from_string_literal(b"histogram_encode.cc"),
                            245,
                            _fn,
                        )
                    });
                    'loop_: while true {}
                };
                let log2count: Value<f64> = Rc::new(RefCell::new(
                    ({
                        let _v: i32 = ((*c_descaled.borrow()) as i32);
                        FastLog2_127(_v)
                    }),
                ));
                let log2floor: Value<i32> = Rc::new(RefCell::new(((*log2count.borrow()) as i32)));
                (*entropy_bits.borrow_mut()) -= {
                    let _lhs = (((*data.borrow()).offset((*i.borrow()) as isize).read()) as f64);
                    _lhs * (*log2count.borrow())
                };
                (*histogram_bits.borrow_mut()) += (*log2floor.borrow());
                (*histogram_bits.borrow_mut()) +=
                    ((*kLogCountBitLengths_205.with(Value::clone).borrow())
                        [((*log2floor.borrow()) + 1) as usize] as i32);
                (*cumul.borrow_mut()) =
                    (((*c.borrow()) & ((*kDescaleMask.borrow()) as u64)) as u32);
            } else {
                (*histogram_bits.borrow_mut()) +=
                    ((*kLogCountBitLengths_205.with(Value::clone).borrow())[(0) as usize] as i32);
            }
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let log2norm: Value<f64> = Rc::new(RefCell::new(
            (((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) as f64)
                - ({
                    let _v: i32 = (*total_count.borrow());
                    FastLog2_127(_v)
                })),
        ));
        if (((*data.borrow()).offset((0) as isize).read()) > 0) {
            let log2count: Value<f64> = Rc::new(RefCell::new({
                let _lhs = ({
                    let _v: i32 = ((*data.borrow()).offset((0) as isize).read());
                    FastLog2_127(_v)
                });
                _lhs + (*log2norm.borrow())
            }));
            (*entropy_bits.borrow_mut()) -= {
                let _lhs = (((*data.borrow()).offset((0) as isize).read()) as f64);
                _lhs * (*log2count.borrow())
            };
            (*length.borrow_mut()) = 0;
            (*count.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) < 18) {
            if (((*data.borrow()).offset((*i.borrow()) as isize).read()) > 0) {
                let log2count: Value<f64> = Rc::new(RefCell::new({
                    let _lhs = ({
                        let _v: i32 = ((*data.borrow()).offset((*i.borrow()) as isize).read());
                        FastLog2_127(_v)
                    });
                    _lhs + (*log2norm.borrow())
                }));
                let log2floor: Value<i32> = Rc::new(RefCell::new(((*log2count.borrow()) as i32)));
                (*entropy_bits.borrow_mut()) -= {
                    let _lhs = (((*data.borrow()).offset((*i.borrow()) as isize).read()) as f64);
                    _lhs * (*log2count.borrow())
                };
                if ((*log2floor.borrow())
                    >= (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()))
                {
                    (*log2floor.borrow_mut()) =
                        ((*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow()) - 1);
                }
                let rhs_0 = (((*histogram_bits.borrow()) as u32).wrapping_add(
                    ({
                        let _logcount: u32 = ((*log2floor.borrow()) as u32);
                        GetPopulationCountPrecision_202(_logcount)
                    }),
                )) as i32;
                (*histogram_bits.borrow_mut()) = rhs_0;
                (*histogram_bits.borrow_mut()) +=
                    ((*kLogCountBitLengths_205.with(Value::clone).borrow())
                        [((*log2floor.borrow()) + 1) as usize] as i32);
                (*length.borrow_mut()) = (*i.borrow());
                (*count.borrow_mut()).prefix_inc();
            } else {
                (*histogram_bits.borrow_mut()) +=
                    ((*kLogCountBitLengths_205.with(Value::clone).borrow())[(0) as usize] as i32);
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
            + (*BRUNSLI_ANS_LOG_TAB_SIZE_0.with(Value::clone).borrow())) as f64);
    }
    (*histogram_bits.borrow_mut()) += ((*kHistogramLengthBitLengths_203.with(Value::clone).borrow())
        [((*length.borrow()) - 3) as usize] as i32);
    return ((((*histogram_bits.borrow()) + ((*entropy_bits.borrow()) as i32)) + 1) as f64);
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    pub static kCodeLengthCodes_210: Value<i32> = Rc::new(RefCell::new(18));
);
pub fn StoreHuffmanTreeOfHuffmanTreeToBitMask_211(
    num_codes: i32,
    code_length_bitdepth: Ptr<u8>,
    storage: Ptr<brunsli_Storage>,
) {
    let num_codes: Value<i32> = Rc::new(RefCell::new(num_codes));
    let code_length_bitdepth: Value<Ptr<u8>> = Rc::new(RefCell::new(code_length_bitdepth));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    thread_local!(
        static kStorageOrder_212: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            1_u8, 2_u8, 3_u8, 4_u8, 0_u8, 5_u8, 17_u8, 6_u8, 16_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
            12_u8, 13_u8, 14_u8, 15_u8,
        ])));
    );
    thread_local!(
        static kHuffmanBitLengthHuffmanCodeSymbols_213: Value<Box<[u8]>> =
            Rc::new(RefCell::new(Box::new([
                0_u8, 7_u8, 3_u8, 2_u8, 1_u8, 15_u8,
            ])));
    );
    thread_local!(
        static kHuffmanBitLengthHuffmanCodeBitLengths_214: Value<Box<[u8]>> =
            Rc::new(RefCell::new(Box::new([2_u8, 4_u8, 3_u8, 2_u8, 2_u8, 4_u8])));
    );
    let codes_to_store: Value<usize> = Rc::new(RefCell::new(
        ((*kCodeLengthCodes_210.with(Value::clone).borrow()) as usize),
    ));
    if ((*num_codes.borrow()) > 1) {
        'loop_: while ((*codes_to_store.borrow()) > 0_usize) {
            if ((((*code_length_bitdepth.borrow())
                .offset(
                    ((*kStorageOrder_212.with(Value::clone).borrow())
                        [((*codes_to_store.borrow()).wrapping_sub(1_usize)) as usize])
                        as isize,
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
    if ((((*code_length_bitdepth.borrow())
        .offset(((*kStorageOrder_212.with(Value::clone).borrow())[(0) as usize]) as isize)
        .read()) as i32)
        == 0)
        && ((((*code_length_bitdepth.borrow())
            .offset(((*kStorageOrder_212.with(Value::clone).borrow())[(1) as usize]) as isize)
            .read()) as i32)
            == 0)
    {
        (*skip_some.borrow_mut()) = 2_usize;
        if ((((*code_length_bitdepth.borrow())
            .offset(((*kStorageOrder_212.with(Value::clone).borrow())[(2) as usize]) as isize)
            .read()) as i32)
            == 0)
        {
            (*skip_some.borrow_mut()) = 3_usize;
        }
    }
    ({
        let _bits: u64 = ((*skip_some.borrow()) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new((*skip_some.borrow())));
    'loop_: while ((*i.borrow()) < (*codes_to_store.borrow())) {
        let l: Value<usize> = Rc::new(RefCell::new(
            (((*code_length_bitdepth.borrow())
                .offset(
                    ((*kStorageOrder_212.with(Value::clone).borrow())[(*i.borrow()) as usize])
                        as isize,
                )
                .read()) as usize),
        ));
        ({
            let _n_bits: usize = ((*kHuffmanBitLengthHuffmanCodeBitLengths_214
                .with(Value::clone)
                .borrow())[(*l.borrow()) as usize] as usize);
            let _bits: u64 = ((*kHuffmanBitLengthHuffmanCodeSymbols_213
                .with(Value::clone)
                .borrow())[(*l.borrow()) as usize] as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn StoreHuffmanTreeToBitMask_215(
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
            (((*huffman_tree.borrow())
                .offset((*i.borrow()) as isize)
                .read()) as usize),
        ));
        ({
            let _n_bits: usize = (((*code_length_bitdepth.borrow())
                .offset((*ix.borrow()) as isize)
                .read()) as usize);
            let _bits: u64 = (((*code_length_bitdepth_symbols.borrow())
                .offset((*ix.borrow()) as isize)
                .read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        'switch: {
            let __match_cond = (*ix.borrow());
            match __match_cond {
                __v if __v == 16_usize => {
                    ({
                        let _bits: u64 = (((*huffman_tree_extra_bits.borrow())
                            .offset((*i.borrow()) as isize)
                            .read()) as u64);
                        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                        WriteBits_120(2_usize, _bits, _storage)
                    });
                    break 'switch;
                }
                __v if __v == 17_usize => {
                    ({
                        let _bits: u64 = (((*huffman_tree_extra_bits.borrow())
                            .offset((*i.borrow()) as isize)
                            .read()) as u64);
                        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
                        WriteBits_120(3_usize, _bits, _storage)
                    });
                    break 'switch;
                }
                _ => {}
            }
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn StoreSimpleHuffmanTree_216(
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
    ({
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, 1_u64, _storage)
    });
    ({
        let _bits: u64 = ((*num_symbols.borrow()).wrapping_sub(1_usize) as u64);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        WriteBits_120(2_usize, _bits, _storage)
    });
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_symbols.borrow())) {
        let j: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*j.borrow()) < (*num_symbols.borrow())) {
            if {
                let _lhs = (((*depths.borrow())
                    .offset(((*symbols.borrow()).offset((*j.borrow()) as isize).read()) as isize)
                    .read()) as i32);
                _lhs < (((*depths.borrow())
                    .offset(((*symbols.borrow()).offset((*i.borrow()) as isize).read()) as isize)
                    .read()) as i32)
            } {
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
            let _bits: u64 = (((*symbols.borrow()).offset((0) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((1) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else if ((*num_symbols.borrow()) == 3_usize) {
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((0) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((1) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((2) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
    } else {
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((0) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((1) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((2) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = (((*symbols.borrow()).offset((3) as isize).read()) as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        ({
            let _bits: u64 = (if ((((*depths.borrow())
                .offset(((*symbols.borrow()).offset((0) as isize).read()) as isize)
                .read()) as i32)
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
pub fn StoreHuffmanTree_217(depths: Ptr<u8>, num: usize, storage: Ptr<brunsli_Storage>) {
    let depths: Value<Ptr<u8>> = Rc::new(RefCell::new(depths));
    let num: Value<usize> = Rc::new(RefCell::new(num));
    let storage: Value<Ptr<brunsli_Storage>> = Rc::new(RefCell::new(storage));
    let arena: Value<Option<Value<Box<[u8]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array(
            (0..(2_usize).wrapping_mul((*num.borrow())))
                .map(|_| <u8>::default())
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
        let _depth: Ptr<u8> = (*depths.borrow()).clone();
        let _length: usize = (*num.borrow());
        let _tree_size: Ptr<usize> = (huffman_tree_size.as_pointer());
        let _tree: Ptr<u8> = (*huffman_tree.borrow()).clone();
        let _extra_bits_data: Ptr<u8> = (*huffman_tree_extra_bits.borrow()).clone();
        WriteHuffmanTree_218(_depth, _length, _tree_size, _tree, _extra_bits_data)
    });
    let huffman_tree_histogram: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
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
    ])));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*huffman_tree_size.borrow())) {
        (*huffman_tree_histogram.borrow_mut())[((*huffman_tree.borrow())
            .offset((*i.borrow()) as isize)
            .read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let num_codes: Value<i32> = Rc::new(RefCell::new(0));
    let code: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kCodeLengthCodes_210.with(Value::clone).borrow())) {
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
    let code_length_bitdepth_symbols: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
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
        <u16>::default(),
        <u16>::default(),
    ])));
    ({
        let _data: Ptr<u32> =
            ((huffman_tree_histogram.as_pointer() as Ptr<u32>).offset(0 as isize));
        let _length: usize = ((*kCodeLengthCodes_210.with(Value::clone).borrow()) as usize);
        let _depth: Ptr<u8> = ((code_length_bitdepth.as_pointer() as Ptr<u8>).offset(0 as isize));
        CreateHuffmanTree_219(_data, _length, 5, _depth)
    });
    ({
        let _depth: Ptr<u8> = (code_length_bitdepth.as_pointer() as Ptr<u8>);
        let _len: usize = ((*kCodeLengthCodes_210.with(Value::clone).borrow()) as usize);
        let _bits: Ptr<u16> =
            ((code_length_bitdepth_symbols.as_pointer() as Ptr<u16>).offset(0 as isize));
        ConvertBitDepthsToSymbols_220(_depth, _len, _bits)
    });
    ({
        let _num_codes: i32 = (*num_codes.borrow());
        let _code_length_bitdepth: Ptr<u8> = (code_length_bitdepth.as_pointer() as Ptr<u8>);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        StoreHuffmanTreeOfHuffmanTreeToBitMask_211(_num_codes, _code_length_bitdepth, _storage)
    });
    if ((*num_codes.borrow()) == 1) {
        (*code_length_bitdepth.borrow_mut())[(*code.borrow()) as usize] = 0_u8;
    }
    ({
        let _huffman_tree_size: usize = (*huffman_tree_size.borrow());
        let _huffman_tree: Ptr<u8> = (*huffman_tree.borrow()).clone();
        let _huffman_tree_extra_bits: Ptr<u8> = (*huffman_tree_extra_bits.borrow()).clone();
        let _code_length_bitdepth: Ptr<u8> =
            ((code_length_bitdepth.as_pointer() as Ptr<u8>).offset(0 as isize));
        let _code_length_bitdepth_symbols: Ptr<u16> =
            (code_length_bitdepth_symbols.as_pointer() as Ptr<u16>);
        let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
        StoreHuffmanTreeToBitMask_215(
            _huffman_tree_size,
            _huffman_tree,
            _huffman_tree_extra_bits,
            _code_length_bitdepth,
            _code_length_bitdepth_symbols,
            _storage,
        )
    });
}
pub fn BuildAndStoreHuffmanTree_201(
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
        if (((*histogram.borrow()).offset((*i.borrow()) as isize).read()) != 0) {
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
        ({
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(4_usize, 1_u64, _storage)
        });
        ({
            let _n_bits: usize = (*max_bits.borrow());
            let _bits: u64 = ((*s4.borrow())[(0) as usize] as u64);
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            WriteBits_120(_n_bits, _bits, _storage)
        });
        return;
    }
    ({
        let _data: Ptr<u32> = (*histogram.borrow()).clone();
        let _length: usize = (*length.borrow());
        let _depth: Ptr<u8> = (*depth.borrow()).clone();
        CreateHuffmanTree_219(_data, _length, 15, _depth)
    });
    ({
        let _depth: Ptr<u8> = (*depth.borrow()).clone();
        let _len: usize = (*length.borrow());
        let _bits: Ptr<u16> = (*bits.borrow()).clone();
        ConvertBitDepthsToSymbols_220(_depth, _len, _bits)
    });
    if ((*count.borrow()) <= 4_usize) {
        ({
            let _depths: Ptr<u8> = (*depth.borrow()).clone();
            let _symbols: Ptr<usize> = (s4.as_pointer() as Ptr<usize>);
            let _num_symbols: usize = (*count.borrow());
            let _max_bits: usize = (*max_bits.borrow());
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            StoreSimpleHuffmanTree_216(_depths, _symbols, _num_symbols, _max_bits, _storage)
        });
    } else {
        ({
            let _depths: Ptr<u8> = (*depth.borrow()).clone();
            let _num: usize = (*length.borrow());
            let _storage: Ptr<brunsli_Storage> = (*storage.borrow()).clone();
            StoreHuffmanTree_217(_depths, _num, _storage)
        });
    }
}
pub fn SetDepth_221(
    p: Ptr<brunsli_HuffmanTree>,
    pool: Ptr<brunsli_HuffmanTree>,
    depth: Ptr<u8>,
    level: u8,
) {
    let pool: Value<Ptr<brunsli_HuffmanTree>> = Rc::new(RefCell::new(pool));
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let level: Value<u8> = Rc::new(RefCell::new(level));
    if (((*(*p.upgrade().deref()).index_left.borrow()) as i32) >= 0) {
        (*level.borrow_mut()).prefix_inc();
        ({
            let _p: Ptr<brunsli_HuffmanTree> =
                (*pool.borrow()).offset((*(*p.upgrade().deref()).index_left.borrow()) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (*pool.borrow()).clone();
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            let _level: u8 = (*level.borrow());
            SetDepth_221(_p, _pool, _depth, _level)
        });
        ({
            let _p: Ptr<brunsli_HuffmanTree> = (*pool.borrow())
                .offset((*(*p.upgrade().deref()).index_right_or_value.borrow()) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> = (*pool.borrow()).clone();
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            let _level: u8 = (*level.borrow());
            SetDepth_221(_p, _pool, _depth, _level)
        });
    } else {
        let __rhs = (*level.borrow());
        (*depth.borrow())
            .offset((*(*p.upgrade().deref()).index_right_or_value.borrow()) as isize)
            .write(__rhs);
    }
}
pub fn Compare_222(v0: Ptr<brunsli_HuffmanTree>, v1: Ptr<brunsli_HuffmanTree>) -> bool {
    return {
        let _lhs = (*(*v0.upgrade().deref()).total_count.borrow());
        _lhs < (*(*v1.upgrade().deref()).total_count.borrow())
    };
}
pub fn CreateHuffmanTree_219(data: Ptr<u32>, length: usize, tree_limit: i32, depth: Ptr<u8>) {
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
            if (((*data.borrow()).offset((*i.borrow()) as isize).read()) != 0) {
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
                (*tree.borrow_mut()).push(brunsli_HuffmanTree::brunsli_HuffmanTree(
                    (*count.borrow()),
                    (-1_i32 as i16),
                    ((*i.borrow()) as i16),
                ));
            };
        }
        let n: Value<usize> = Rc::new(RefCell::new((*tree.borrow()).len()));
        if ((*n.borrow()) == 1_usize) {
            (*depth.borrow())
                .offset(
                    (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                        .offset(0_usize as isize)
                        .upgrade()
                        .deref())
                    .index_right_or_value
                    .borrow()) as isize,
                )
                .write(1_u8);
            break;
        }
        (tree.as_pointer() as Ptr<brunsli_HuffmanTree>).sort_with_cmp(
            (tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .to_end()
                .get_offset(),
            Compare_222,
        );
        let sentinel: Value<brunsli_HuffmanTree> = Rc::new(RefCell::new(
            brunsli_HuffmanTree::brunsli_HuffmanTree(<u32>::MAX, (-1_i32 as i16), (-1_i32 as i16)),
        ));
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
            if ((*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .total_count
            .borrow())
                <= (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                    .offset((*j.borrow()) as isize)
                    .upgrade()
                    .deref())
                .total_count
                .borrow()))
            {
                (*left.borrow_mut()) = (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            } else {
                (*left.borrow_mut()) = (*j.borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            if ((*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .total_count
            .borrow())
                <= (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                    .offset((*j.borrow()) as isize)
                    .upgrade()
                    .deref())
                .total_count
                .borrow()))
            {
                (*right.borrow_mut()) = (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            } else {
                (*right.borrow_mut()) = (*j.borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            let j_end: Value<usize> =
                Rc::new(RefCell::new(((*tree.borrow()).len()).wrapping_sub(1_usize)));
            let __rhs = (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*left.borrow()) as isize)
                .upgrade()
                .deref())
            .total_count
            .borrow())
            .wrapping_add(
                (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                    .offset((*right.borrow()) as isize)
                    .upgrade()
                    .deref())
                .total_count
                .borrow()),
            );
            (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*j_end.borrow()) as isize)
                .upgrade()
                .deref())
            .total_count
            .borrow_mut()) = __rhs;
            (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*j_end.borrow()) as isize)
                .upgrade()
                .deref())
            .index_left
            .borrow_mut()) = ((*left.borrow()) as i16);
            (*(*(tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset((*j_end.borrow()) as isize)
                .upgrade()
                .deref())
            .index_right_or_value
            .borrow_mut()) = ((*right.borrow()) as i16);
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
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"CreateHuffmanTree");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"huffman_tree.cc"), 121, _fn)
            });
            'loop_: while true {}
        };
        ({
            let _p: Ptr<brunsli_HuffmanTree> = (tree.as_pointer() as Ptr<brunsli_HuffmanTree>)
                .offset(((2_usize).wrapping_mul((*n.borrow()))).wrapping_sub(1_usize) as isize);
            let _pool: Ptr<brunsli_HuffmanTree> =
                ((tree.as_pointer() as Ptr<brunsli_HuffmanTree>).offset(0_usize as isize));
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            SetDepth_221(_p, _pool, _depth, 0_u8)
        });
        if {
            let _lhs = (({
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
            .read()) as i32);
            _lhs <= (*tree_limit.borrow())
        } {
            break;
        }
        let rhs_0 = (*count_limit.borrow()).wrapping_mul(2_u32);
        (*count_limit.borrow_mut()) = rhs_0;
    }
}
pub fn Reverse_223(v: Ptr<u8>, start: usize, end: usize) {
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(v));
    let start: Value<usize> = Rc::new(RefCell::new(start));
    let end: Value<usize> = Rc::new(RefCell::new(end));
    (*end.borrow_mut()).prefix_dec();
    'loop_: while ((*start.borrow()) < (*end.borrow())) {
        let tmp: Value<u8> = Rc::new(RefCell::new(
            ((*v.borrow()).offset((*start.borrow()) as isize).read()),
        ));
        let __rhs = ((*v.borrow()).offset((*end.borrow()) as isize).read());
        (*v.borrow())
            .offset((*start.borrow()) as isize)
            .write(__rhs);
        let __rhs = (*tmp.borrow());
        (*v.borrow()).offset((*end.borrow()) as isize).write(__rhs);
        (*start.borrow_mut()).prefix_inc();
        (*end.borrow_mut()).prefix_dec();
    }
}
pub fn WriteHuffmanTreeRepetitions_224(
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
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"WriteHuffmanTreeRepetitions");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"huffman_tree.cc"), 151, _fn)
        });
        'loop_: while true {}
    };
    if (((*previous_value.borrow()) as i32) != ((*value.borrow()) as i32)) {
        let __rhs = (*value.borrow());
        (*tree.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(__rhs);
        (*extra_bits_data.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) == 7_usize) {
        let __rhs = (*value.borrow());
        (*tree.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(__rhs);
        (*extra_bits_data.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) < 3_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*repetitions.borrow())) {
            let __rhs = (*value.borrow());
            (*tree.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(__rhs);
            (*extra_bits_data.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(0_u8);
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let rhs_0 = (*repetitions.borrow()).wrapping_sub(3_usize);
        (*repetitions.borrow_mut()) = rhs_0;
        let start: Value<usize> = Rc::new(RefCell::new(((*tree_size.borrow()).read())));
        'loop_: while true {
            (*tree.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(16_u8);
            let __rhs = (((*repetitions.borrow()) & 3_usize) as u8);
            (*extra_bits_data.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(__rhs);
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
            Reverse_223(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (*extra_bits_data.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_223(_v, _start, _end)
        });
    }
}
pub fn WriteHuffmanTreeRepetitionsZeros_225(
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
        (*tree.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(0_u8);
        (*extra_bits_data.borrow())
            .offset(((*tree_size.borrow()).read()) as isize)
            .write(0_u8);
        (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
        (*repetitions.borrow_mut()).prefix_dec();
    }
    if ((*repetitions.borrow()) < 3_usize) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*repetitions.borrow())) {
            (*tree.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(0_u8);
            (*extra_bits_data.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(0_u8);
            (*tree_size.borrow()).with_mut(|__v| __v.prefix_inc());
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let rhs_0 = (*repetitions.borrow()).wrapping_sub(3_usize);
        (*repetitions.borrow_mut()) = rhs_0;
        let start: Value<usize> = Rc::new(RefCell::new(((*tree_size.borrow()).read())));
        'loop_: while true {
            (*tree.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(17_u8);
            let __rhs = (((*repetitions.borrow()) & 7_usize) as u8);
            (*extra_bits_data.borrow())
                .offset(((*tree_size.borrow()).read()) as isize)
                .write(__rhs);
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
            Reverse_223(_v, _start, _end)
        });
        ({
            let _v: Ptr<u8> = (*extra_bits_data.borrow()).clone();
            let _start: usize = (*start.borrow());
            let _end: usize = ((*tree_size.borrow()).read());
            Reverse_223(_v, _start, _end)
        });
    }
}
pub fn DecideOverRleUse_226(
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
            ((*depth.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        let reps: Value<usize> = Rc::new(RefCell::new(1_usize));
        let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
        'loop_: while ((*k.borrow()) < (*length.borrow()))
            && ({
                let _lhs = (((*depth.borrow()).offset((*k.borrow()) as isize).read()) as i32);
                _lhs == ((*value.borrow()) as i32)
            })
        {
            (*reps.borrow_mut()).prefix_inc();
            (*k.borrow_mut()).prefix_inc();
        }
        if ((*reps.borrow()) >= 3_usize) && (((*value.borrow()) as i32) == 0) {
            let rhs_0 = (*total_reps_zero.borrow()).wrapping_add((*reps.borrow()));
            (*total_reps_zero.borrow_mut()) = rhs_0;
            (*count_reps_zero.borrow_mut()).prefix_inc();
        }
        if ((*reps.borrow()) >= 4_usize) && (((*value.borrow()) as i32) != 0) {
            let rhs_0 = (*total_reps_non_zero.borrow()).wrapping_add((*reps.borrow()));
            (*total_reps_non_zero.borrow_mut()) = rhs_0;
            (*count_reps_non_zero.borrow_mut()).prefix_inc();
        }
        let rhs_0 = (*i.borrow()).wrapping_add((*reps.borrow()));
        (*i.borrow_mut()) = rhs_0;
    }
    let __rhs =
        ((*total_reps_non_zero.borrow()) > (*count_reps_non_zero.borrow()).wrapping_mul(2_usize));
    (*use_rle_for_non_zero.borrow()).write(__rhs);
    let __rhs = ((*total_reps_zero.borrow()) > (*count_reps_zero.borrow()).wrapping_mul(2_usize));
    (*use_rle_for_zero.borrow()).write(__rhs);
}
pub fn WriteHuffmanTree_218(
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
        if ((((*depth.borrow())
            .offset(
                (((*length.borrow()).wrapping_sub((*i.borrow()))).wrapping_sub(1_usize)) as isize,
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
            let _depth: Ptr<u8> = (*depth.borrow()).clone();
            let _length: usize = (*new_length.borrow());
            let _use_rle_for_non_zero: Ptr<bool> = (use_rle_for_non_zero.as_pointer());
            let _use_rle_for_zero: Ptr<bool> = (use_rle_for_zero.as_pointer());
            DecideOverRleUse_226(_depth, _length, _use_rle_for_non_zero, _use_rle_for_zero)
        });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*new_length.borrow())) {
        let value: Value<u8> = Rc::new(RefCell::new(
            ((*depth.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        let reps: Value<usize> = Rc::new(RefCell::new(1_usize));
        if ((((*value.borrow()) as i32) != 0) && (*use_rle_for_non_zero.borrow()))
            || ((((*value.borrow()) as i32) == 0) && (*use_rle_for_zero.borrow()))
        {
            let k: Value<usize> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_usize)));
            'loop_: while ((*k.borrow()) < (*new_length.borrow()))
                && ({
                    let _lhs = (((*depth.borrow()).offset((*k.borrow()) as isize).read()) as i32);
                    _lhs == ((*value.borrow()) as i32)
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
                let _tree: Ptr<u8> = (*tree.borrow()).clone();
                let _extra_bits_data: Ptr<u8> = (*extra_bits_data.borrow()).clone();
                WriteHuffmanTreeRepetitionsZeros_225(
                    _repetitions,
                    _tree_size,
                    _tree,
                    _extra_bits_data,
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
                WriteHuffmanTreeRepetitions_224(
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
        let rhs_0 = (*i.borrow()).wrapping_add((*reps.borrow()));
        (*i.borrow_mut()) = rhs_0;
    }
}
pub fn ReverseBits_227(num_bits: i32, bits: u16) -> u16 {
    let num_bits: Value<i32> = Rc::new(RefCell::new(num_bits));
    let bits: Value<u16> = Rc::new(RefCell::new(bits));
    thread_local!(
        static kLut_228: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
            0_usize, 8_usize, 4_usize, 12_usize, 2_usize, 10_usize, 6_usize, 14_usize, 1_usize,
            9_usize, 5_usize, 13_usize, 3_usize, 11_usize, 7_usize, 15_usize,
        ])));
    );
    let retval: Value<usize> = Rc::new(RefCell::new(
        (*kLut_228.with(Value::clone).borrow())[(((*bits.borrow()) as i32) & 15) as usize],
    ));
    let i: Value<i32> = Rc::new(RefCell::new(4));
    'loop_: while ((*i.borrow()) < (*num_bits.borrow())) {
        (*retval.borrow_mut()) <<= 4;
        let __rhs = ((((*bits.borrow()) as i32) >> 4) as u16);
        (*bits.borrow_mut()) = __rhs;
        (*retval.borrow_mut()) |=
            (*kLut_228.with(Value::clone).borrow())[(((*bits.borrow()) as i32) & 15) as usize];
        (*i.borrow_mut()) += 4;
    }
    (*retval.borrow_mut()) >>= (-(*num_bits.borrow()) & 3);
    return ((*retval.borrow()) as u16);
}
pub fn ConvertBitDepthsToSymbols_220(depth: Ptr<u8>, len: usize, bits: Ptr<u16>) {
    let depth: Value<Ptr<u8>> = Rc::new(RefCell::new(depth));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let bits: Value<Ptr<u16>> = Rc::new(RefCell::new(bits));
    let kMaxBits: Value<i32> = Rc::new(RefCell::new(16));
    let bl_count: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
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
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        (*bl_count.borrow_mut())
            [((*depth.borrow()).offset((*i.borrow()) as isize).read()) as usize]
            .prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    (*bl_count.borrow_mut())[(0) as usize] = 0_u16;
    let next_code: Value<Box<[u16]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u16>::default()).collect::<Box<[u16]>>(),
    ));
    (*next_code.borrow_mut())[(0) as usize] = 0_u16;
    let code: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(1_usize));
    'loop_: while ((*i.borrow()) < ((*kMaxBits.borrow()) as usize)) {
        let __rhs = (((*code.borrow())
            + ((*bl_count.borrow())[((*i.borrow()).wrapping_sub(1_usize)) as usize] as i32))
            << 1);
        (*code.borrow_mut()) = __rhs;
        (*next_code.borrow_mut())[(*i.borrow()) as usize] = ((*code.borrow()) as u16);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        if (((*depth.borrow()).offset((*i.borrow()) as isize).read()) != 0) {
            let __rhs = ({
                let _num_bits: i32 =
                    (((*depth.borrow()).offset((*i.borrow()) as isize).read()) as i32);
                let _bits: u16 = (*next_code.borrow_mut())
                    [((*depth.borrow()).offset((*i.borrow()) as isize).read()) as usize]
                    .postfix_inc();
                ReverseBits_227(_num_bits, _bits)
            });
            (*bits.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        }
        (*i.borrow_mut()).prefix_inc();
    }
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    pub static kJpegHuffmanRootTableBits_229: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kJpegHuffmanLutSize_230: Value<i32> = Rc::new(RefCell::new(1024));
);
#[derive()]
pub struct brunsli_HuffmanTableEntry {
    pub bits: Value<u8>,
    pub value: Value<u16>,
}
impl brunsli_HuffmanTableEntry {
    pub fn brunsli_HuffmanTableEntry() -> Self {
        let mut this = Self {
            bits: Rc::new(RefCell::new(0_u8)),
            value: Rc::new(RefCell::new(65535_u16)),
        };
        this
    }
}
impl Clone for brunsli_HuffmanTableEntry {
    fn clone(&self) -> Self {
        let mut this = Self {
            bits: Rc::new(RefCell::new((*self.bits.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl Default for brunsli_HuffmanTableEntry {
    fn default() -> Self {
        {
            brunsli_HuffmanTableEntry::brunsli_HuffmanTableEntry()
        }
    }
}
impl ByteRepr for brunsli_HuffmanTableEntry {
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
pub fn DivCeil_231(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((((*a.borrow()) + (*b.borrow())) - 1) / (*b.borrow()));
}
pub fn ReadUint8_232(data: Ptr<u8>, pos: Ptr<usize>) -> i32 {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    return (((*data.borrow())
        .offset(((*pos.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .read()) as i32);
}
pub fn ReadUint16_233(data: Ptr<u8>, pos: Ptr<usize>) -> i32 {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let v: Value<i32> = Rc::new(RefCell::new(
        (((((*data.borrow())
            .offset(((*pos.borrow()).read()) as isize)
            .read()) as i32)
            << 8)
            + (((*data.borrow())
                .offset((((*pos.borrow()).read()).wrapping_add(1_usize)) as isize)
                .read()) as i32)),
    ));
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add(2_usize);
    (*pos.borrow()).write(rhs_0);
    return (*v.borrow());
}
pub fn ProcessSOF_234(
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
    if ((*(*(*jpg.borrow()).upgrade().deref()).width.borrow()) != 0) {
        write!(libcc2rs::cerr(), "Duplicate SOF marker.\n",);
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::DUPLICATE_SOF;
        return false;
    }
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((8) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (8),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    let precision: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        }),
    ));
    let height: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }),
    ));
    let width: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }),
    ));
    let num_components: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        }),
    ));
    if ((*precision.borrow()) < 8) || ((*precision.borrow()) > 8) {
        write!(
            libcc2rs::cerr(),
            "Invalid precision: {:}\n",
            (*precision.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_PRECISION;
        return false;
    };
    if ((*height.borrow()) < 1)
        || ((*height.borrow()) > (*kMaxDimPixels_11.with(Value::clone).borrow()))
    {
        write!(
            libcc2rs::cerr(),
            "Invalid height: {:}\n",
            (*height.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_HEIGHT;
        return false;
    };
    if ((*width.borrow()) < 1)
        || ((*width.borrow()) > (*kMaxDimPixels_11.with(Value::clone).borrow()))
    {
        write!(libcc2rs::cerr(), "Invalid width: {:}\n", (*width.borrow()),);
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_WIDTH;
        return false;
    };
    if ((*num_components.borrow()) < 1)
        || ((*num_components.borrow()) > (*kMaxComponents_4.with(Value::clone).borrow()))
    {
        write!(
            libcc2rs::cerr(),
            "Invalid num_components: {:}\n",
            (*num_components.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_NUMCOMP;
        return false;
    };
    if {
        let _lhs =
            ((*pos.borrow()).read()).wrapping_add(((3 * (*num_components.borrow())) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3 * (*num_components.borrow())),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    (*(*(*jpg.borrow()).upgrade().deref()).height.borrow_mut()) = (*height.borrow());
    (*(*(*jpg.borrow()).upgrade().deref()).width.borrow_mut()) = (*width.borrow());
    {
        let __a0 = ((*num_components.borrow()) as usize) as usize;
        (*(*(*jpg.borrow()).upgrade().deref()).components.borrow_mut())
            .resize_with(__a0, || <brunsli_JPEGComponent>::default())
    };
    let ids_seen: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(256_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
    } {
        let id: Value<i32> = Rc::new(RefCell::new(
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (*pos.borrow()).clone();
                ReadUint8_232(_data, _pos)
            }),
        ));
        if ((*(ids_seen.as_pointer() as Ptr<bool>)
            .offset(((*id.borrow()) as usize) as isize)
            .upgrade()
            .deref()) as bool)
        {
            write!(
                libcc2rs::cerr(),
                "Duplicate ID {:} in SOF.\n",
                (*id.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::DUPLICATE_COMPONENT_ID;
            return false;
        }
        (ids_seen.as_pointer() as Ptr<bool>)
            .offset(((*id.borrow()) as usize) as isize)
            .write(true);
        (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .id
        .borrow_mut()) = (*id.borrow());
        let factor: Value<i32> = Rc::new(RefCell::new(
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (*pos.borrow()).clone();
                ReadUint8_232(_data, _pos)
            }),
        ));
        let h_samp_factor: Value<i32> = Rc::new(RefCell::new(((*factor.borrow()) >> 4)));
        let v_samp_factor: Value<i32> = Rc::new(RefCell::new(((*factor.borrow()) & 15)));
        if ((*h_samp_factor.borrow()) < 1)
            || ((*h_samp_factor.borrow()) > (*kBrunsliMaxSampling_27.with(Value::clone).borrow()))
        {
            write!(
                libcc2rs::cerr(),
                "Invalid h_samp_factor: {:}\n",
                (*h_samp_factor.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_SAMP_FACTOR;
            return false;
        };
        if ((*v_samp_factor.borrow()) < 1)
            || ((*v_samp_factor.borrow()) > (*kBrunsliMaxSampling_27.with(Value::clone).borrow()))
        {
            write!(
                libcc2rs::cerr(),
                "Invalid v_samp_factor: {:}\n",
                (*v_samp_factor.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_SAMP_FACTOR;
            return false;
        };
        (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .h_samp_factor
        .borrow_mut()) = (*h_samp_factor.borrow());
        (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .v_samp_factor
        .borrow_mut()) = (*v_samp_factor.borrow());
        let __rhs = (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        }) as u8);
        (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .quant_idx
        .borrow_mut()) = __rhs;
        let __rhs = (if (*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .as_pointer()
            .read()
            >= h_samp_factor.as_pointer().read()
        {
            (*(*jpg.borrow()).upgrade().deref())
                .max_h_samp_factor
                .as_pointer()
        } else {
            h_samp_factor.as_pointer()
        }
        .read());
        (*(*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .borrow_mut()) = __rhs;
        let __rhs = (if (*(*jpg.borrow()).upgrade().deref())
            .max_v_samp_factor
            .as_pointer()
            .read()
            >= v_samp_factor.as_pointer().read()
        {
            (*(*jpg.borrow()).upgrade().deref())
                .max_v_samp_factor
                .as_pointer()
        } else {
            v_samp_factor.as_pointer()
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
        DivCeil_231(_a, _b)
    });
    (*(*(*jpg.borrow()).upgrade().deref()).MCU_rows.borrow_mut()) = __rhs;
    let __rhs = ({
        let _a: i32 = (*(*(*jpg.borrow()).upgrade().deref()).width.borrow());
        let _b: i32 = ((*(*(*jpg.borrow()).upgrade().deref())
            .max_h_samp_factor
            .borrow())
            * 8);
        DivCeil_231(_a, _b)
    });
    (*(*(*jpg.borrow()).upgrade().deref()).MCU_cols.borrow_mut()) = __rhs;
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
    } {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)),
        ));
        if ({
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref())
                .max_h_samp_factor
                .borrow());
            _lhs % (*(*(*c.borrow()).upgrade().deref()).h_samp_factor.borrow())
        } != 0)
            || ({
                let _lhs = (*(*(*jpg.borrow()).upgrade().deref())
                    .max_v_samp_factor
                    .borrow());
                _lhs % (*(*(*c.borrow()).upgrade().deref()).v_samp_factor.borrow())
            } != 0)
        {
            write!(libcc2rs::cerr(), "Non-integral subsampling ratios.\n",);
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_SAMPLING_FACTORS;
            return false;
        }
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
        let num_blocks: Value<u64> = Rc::new(RefCell::new(
            ((*(*(*c.borrow()).upgrade().deref()).width_in_blocks.borrow()) as u64).wrapping_mul(
                ((*(*(*c.borrow()).upgrade().deref()).height_in_blocks.borrow()) as u64),
            ),
        ));
        if (((*num_blocks.borrow()) as usize)
            > (*kBrunsliMaxNumBlocks_18.with(Value::clone).borrow()))
        {
            write!(libcc2rs::cerr(), "Image too large.\n",);
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::IMAGE_TOO_LARGE;
            return false;
        }
        (*(*(*c.borrow()).upgrade().deref()).num_blocks.borrow_mut()) =
            (((*num_blocks.borrow()) as i32) as u32);
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
            {
                let __a0 = (((*(*(*c.borrow()).upgrade().deref()).num_blocks.borrow())
                    .wrapping_mul(((*kDCTBlockSize_3.with(Value::clone).borrow()) as u32)))
                    as usize) as usize;
                (*(*(*c.borrow()).upgrade().deref()).coeffs.borrow_mut())
                    .resize_with(__a0, || <i16>::default())
            };
        }
        (*i.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = (*start_pos.borrow()).wrapping_add((*marker_len.borrow()));
        _lhs != ((*pos.borrow()).read())
    } {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_MARKER_SIZE;
        return false;
    };
    return true;
}
pub fn ProcessSOS_235(
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
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((3) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    let comps_in_scan: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        }),
    ));
    if (((*comps_in_scan.borrow()) as usize) < 1_usize)
        || ({
            let _lhs = ((*comps_in_scan.borrow()) as usize);
            _lhs > (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
        })
    {
        write!(
            libcc2rs::cerr(),
            "Invalid static_cast<size_t>(comps_in_scan): {:}\n",
            ((*comps_in_scan.borrow()) as usize),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_COMPS_IN_SCAN;
        return false;
    };
    let scan_info: Value<brunsli_JPEGScanInfo> =
        Rc::new(RefCell::new(<brunsli_JPEGScanInfo>::default()));
    (*(*scan_info.borrow()).num_components.borrow_mut()) = ((*comps_in_scan.borrow()) as usize);
    if {
        let _lhs =
            ((*pos.borrow()).read()).wrapping_add(((2 * (*comps_in_scan.borrow())) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2 * (*comps_in_scan.borrow())),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
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
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (*pos.borrow()).clone();
                ReadUint8_232(_data, _pos)
            }),
        ));
        if ((*(ids_seen.as_pointer() as Ptr<bool>)
            .offset(((*id.borrow()) as usize) as isize)
            .upgrade()
            .deref()) as bool)
        {
            write!(
                libcc2rs::cerr(),
                "Duplicate ID {:} in SOS.\n",
                (*id.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::DUPLICATE_COMPONENT_ID;
            return false;
        }
        (ids_seen.as_pointer() as Ptr<bool>)
            .offset(((*id.borrow()) as usize) as isize)
            .write(true);
        let found_index: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*j.borrow());
            _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
        } {
            if {
                let _lhs = (*(*((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponent>)
                    .offset((*j.borrow()) as isize)
                    .upgrade()
                    .deref())
                .id
                .borrow());
                _lhs == (*id.borrow())
            } {
                (*(*((*scan_info.borrow()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .upgrade()
                    .deref())
                .comp_idx
                .borrow_mut()) = ((*j.borrow()) as u8);
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
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::COMPONENT_NOT_FOUND;
            return false;
        }
        let c: Value<i32> = Rc::new(RefCell::new(
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (*pos.borrow()).clone();
                ReadUint8_232(_data, _pos)
            }),
        ));
        let dc_tbl_idx: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) >> 4)));
        let ac_tbl_idx: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) & 15)));
        if ((*dc_tbl_idx.borrow()) < 0) || ((*dc_tbl_idx.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid dc_tbl_idx: {:}\n",
                (*dc_tbl_idx.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX;
            return false;
        };
        if ((*ac_tbl_idx.borrow()) < 0) || ((*ac_tbl_idx.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid ac_tbl_idx: {:}\n",
                (*ac_tbl_idx.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX;
            return false;
        };
        (*(*((*scan_info.borrow()).components.as_pointer()
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset(((*i.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .dc_tbl_idx
        .borrow_mut()) = (*dc_tbl_idx.borrow());
        (*(*((*scan_info.borrow()).components.as_pointer()
            as Ptr<brunsli_JPEGComponentScanInfo>)
            .offset(((*i.borrow()) as usize) as isize)
            .upgrade()
            .deref())
        .ac_tbl_idx
        .borrow_mut()) = (*ac_tbl_idx.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((3) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (3),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    (*(*scan_info.borrow()).Ss.borrow_mut()) = ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (*pos.borrow()).clone();
        ReadUint8_232(_data, _pos)
    });
    (*(*scan_info.borrow()).Se.borrow_mut()) = ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _pos: Ptr<usize> = (*pos.borrow()).clone();
        ReadUint8_232(_data, _pos)
    });
    if ((*(*scan_info.borrow()).Ss.borrow()) < 0) || ((*(*scan_info.borrow()).Ss.borrow()) > 63) {
        write!(
            libcc2rs::cerr(),
            "Invalid scan_info.Ss: {:}\n",
            (*(*scan_info.borrow()).Ss.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_START_OF_SCAN;
        return false;
    };
    if ((*(*scan_info.borrow()).Se.borrow()) < (*(*scan_info.borrow()).Ss.borrow()))
        || ((*(*scan_info.borrow()).Se.borrow()) > 63)
    {
        write!(
            libcc2rs::cerr(),
            "Invalid scan_info.Se: {:}\n",
            (*(*scan_info.borrow()).Se.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_END_OF_SCAN;
        return false;
    };
    let c: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        }),
    ));
    (*(*scan_info.borrow()).Ah.borrow_mut()) = ((*c.borrow()) >> 4);
    (*(*scan_info.borrow()).Al.borrow_mut()) = ((*c.borrow()) & 15);
    if ((*(*scan_info.borrow()).Ah.borrow()) != 0)
        && ((*(*scan_info.borrow()).Al.borrow()) != ((*(*scan_info.borrow()).Ah.borrow()) - 1))
    {
        write!(
            libcc2rs::cerr(),
            "Invalid progressive parameters:  Al = {:} Ah = {:}\n",
            (*(*scan_info.borrow()).Al.borrow()),
            (*(*scan_info.borrow()).Ah.borrow()),
        );
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*comps_in_scan.borrow())) {
        let found_dc_table: Value<bool> = Rc::new(RefCell::new(false));
        let found_ac_table: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*j.borrow());
            _lhs < (*(*(*jpg.borrow()).upgrade().deref()).huffman_code.borrow()).len()
        } {
            let slot_id: Value<i32> = Rc::new(RefCell::new(
                (*(*((*(*jpg.borrow()).upgrade().deref())
                    .huffman_code
                    .as_pointer() as Ptr<brunsli_JPEGHuffmanCode>)
                    .offset((*j.borrow()) as isize)
                    .upgrade()
                    .deref())
                .slot_id
                .borrow()),
            ));
            if ((*slot_id.borrow())
                == (*(*((*scan_info.borrow()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .upgrade()
                    .deref())
                .dc_tbl_idx
                .borrow()))
            {
                (*found_dc_table.borrow_mut()) = true;
            } else if ((*slot_id.borrow())
                == ((*(*((*scan_info.borrow()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .upgrade()
                    .deref())
                .ac_tbl_idx
                .borrow())
                    + 16))
            {
                (*found_ac_table.borrow_mut()) = true;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        if ((*(*scan_info.borrow()).Ss.borrow()) == 0) && (!(*found_dc_table.borrow())) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find DC Huffman table with index {:}\n",
                (*(*((*scan_info.borrow()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .upgrade()
                    .deref())
                .dc_tbl_idx
                .borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::HUFFMAN_TABLE_NOT_FOUND;
            return false;
        }
        if ((*(*scan_info.borrow()).Se.borrow()) > 0) && (!(*found_ac_table.borrow())) {
            write!(
                libcc2rs::cerr(),
                "SOS marker: Could not find AC Huffman table with index {:}\n",
                (*(*((*scan_info.borrow()).components.as_pointer()
                    as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .upgrade()
                    .deref())
                .ac_tbl_idx
                .borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::HUFFMAN_TABLE_NOT_FOUND;
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    {
        let a0_clone = (*scan_info.borrow()).clone();
        (*(*(*jpg.borrow()).upgrade().deref()).scan_info.borrow_mut()).push(a0_clone)
    };
    if {
        let _lhs = (*start_pos.borrow()).wrapping_add((*marker_len.borrow()));
        _lhs != ((*pos.borrow()).read())
    } {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_MARKER_SIZE;
        return false;
    };
    return true;
}
pub fn ProcessDHT_236(
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
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((2) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    if ((*marker_len.borrow()) == 2_usize) {
        write!(libcc2rs::cerr(), "DHT marker: no Huffman table found\n",);
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::EMPTY_DHT;
        return false;
    }
    'loop_: while {
        let _lhs = ((*pos.borrow()).read());
        _lhs < (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
    } {
        if {
            let _lhs = ((*pos.borrow()).read()).wrapping_add(
                ((1 + (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())) as usize),
            );
            _lhs > (*len.borrow())
        } {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (1 + (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())),
                (*len.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::UNEXPECTED_EOF;
            return false;
        };
        let huff: Value<brunsli_JPEGHuffmanCode> =
            Rc::new(RefCell::new(<brunsli_JPEGHuffmanCode>::default()));
        (*(*huff.borrow()).slot_id.borrow_mut()) = ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint8_232(_data, _pos)
        });
        let huffman_index: Value<i32> = Rc::new(RefCell::new((*(*huff.borrow()).slot_id.borrow())));
        let is_ac_table: Value<i32> = Rc::new(RefCell::new(
            ((((*(*huff.borrow()).slot_id.borrow()) & 16) != 0) as i32),
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
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX;
                return false;
            };
            (*huff_lut.borrow_mut()) = ((((*ac_huff_lut.borrow()).to_strong().as_pointer())
                as Ptr<brunsli_HuffmanTableEntry>)
                .offset(
                    (((*huffman_index.borrow())
                        * (*kJpegHuffmanLutSize_230.with(Value::clone).borrow()))
                        as usize) as isize,
                ));
        } else {
            if ((*huffman_index.borrow()) < 0) || ((*huffman_index.borrow()) > 3) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid huffman_index: {:}\n",
                    (*huffman_index.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_HUFFMAN_INDEX;
                return false;
            };
            (*huff_lut.borrow_mut()) = ((((*dc_huff_lut.borrow()).to_strong().as_pointer())
                as Ptr<brunsli_HuffmanTableEntry>)
                .offset(
                    (((*huffman_index.borrow())
                        * (*kJpegHuffmanLutSize_230.with(Value::clone).borrow()))
                        as usize) as isize,
                ));
        }
        ((*huff.borrow()).counts.as_pointer() as Ptr<i32>)
            .offset(0_usize as isize)
            .write(0);
        let total_count: Value<i32> = Rc::new(RefCell::new(0));
        let space: Value<i32> = Rc::new(RefCell::new(
            (1 << (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())),
        ));
        let max_depth: Value<i32> = Rc::new(RefCell::new(1));
        let i: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*i.borrow()) <= (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())) {
            let count: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _data: Ptr<u8> = (*data.borrow()).clone();
                    let _pos: Ptr<usize> = (*pos.borrow()).clone();
                    ReadUint8_232(_data, _pos)
                }),
            ));
            if ((*count.borrow()) != 0) {
                (*max_depth.borrow_mut()) = (*i.borrow());
            }
            ((*huff.borrow()).counts.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize) as isize)
                .write((*count.borrow()));
            (*total_count.borrow_mut()) += (*count.borrow());
            (*space.borrow_mut()) -= ((*count.borrow())
                * (1 << ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())
                    - (*i.borrow()))));
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*is_ac_table.borrow()) != 0) {
            if ((*total_count.borrow()) < 0)
                || ((*total_count.borrow())
                    > (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow()))
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid total_count: {:}\n",
                    (*total_count.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_HUFFMAN_CODE;
                return false;
            };
        } else {
            if ((*total_count.borrow()) < 0)
                || ((*total_count.borrow()) > (*kJpegDCAlphabetSize_9.with(Value::clone).borrow()))
            {
                write!(
                    libcc2rs::cerr(),
                    "Invalid total_count: {:}\n",
                    (*total_count.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_HUFFMAN_CODE;
                return false;
            };
        }
        if {
            let _lhs = ((*pos.borrow()).read()).wrapping_add(((*total_count.borrow()) as usize));
            _lhs > (*len.borrow())
        } {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (*total_count.borrow()),
                (*len.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::UNEXPECTED_EOF;
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
                (({
                    let _data: Ptr<u8> = (*data.borrow()).clone();
                    let _pos: Ptr<usize> = (*pos.borrow()).clone();
                    ReadUint8_232(_data, _pos)
                }) as u8),
            ));
            if !((*is_ac_table.borrow()) != 0) {
                if (((*value.borrow()) as i32) < 0)
                    || (((*value.borrow()) as i32)
                        > ((*kJpegDCAlphabetSize_9.with(Value::clone).borrow()) - 1))
                {
                    write!(libcc2rs::cerr(), "Invalid value: ",);
                    libcc2rs::cerr().write_all(
                        &([(&[(*value.borrow())] as &[u8]), (&[b'\n'] as &[u8])].concat()),
                    );
                    (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                        brunsli_JPEGReadError::INVALID_HUFFMAN_CODE;
                    return false;
                };
            }
            if ((*(values_seen.as_pointer() as Ptr<bool>)
                .offset(((*value.borrow()) as usize) as isize)
                .upgrade()
                .deref()) as bool)
            {
                write!(libcc2rs::cerr(), "Duplicate Huffman code value ",);
                libcc2rs::cerr()
                    .write_all(&([(&[(*value.borrow())] as &[u8]), (&[b'\n'] as &[u8])].concat()));
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_HUFFMAN_CODE;
                return false;
            }
            (values_seen.as_pointer() as Ptr<bool>)
                .offset(((*value.borrow()) as usize) as isize)
                .write(true);
            ((*huff.borrow()).values.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize) as isize)
                .write(((*value.borrow()) as i32));
            (*i.borrow_mut()).prefix_inc();
        }
        ((*huff.borrow()).counts.as_pointer() as Ptr<i32>)
            .offset(((*max_depth.borrow()) as usize) as isize)
            .with_mut(|__v| __v.prefix_inc());
        ((*huff.borrow()).values.as_pointer() as Ptr<i32>)
            .offset(((*total_count.borrow()) as usize) as isize)
            .write((*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow()));
        (*space.borrow_mut()) -= (1
            << ((*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow()) - (*max_depth.borrow())));
        if ((*space.borrow()) < 0) {
            write!(libcc2rs::cerr(), "Invalid Huffman code lengths.\n",);
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_HUFFMAN_CODE;
            return false;
        } else if ((*space.borrow()) > 0)
            && (((*(*(*huff_lut.borrow()).offset((0) as isize).upgrade().deref())
                .value
                .borrow()) as i32)
                != 65535)
        {
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*kJpegHuffmanLutSize_230.with(Value::clone).borrow())) {
                (*(*(*huff_lut.borrow())
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .bits
                .borrow_mut()) = 0_u8;
                (*(*(*huff_lut.borrow())
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .value
                .borrow_mut()) = 65535_u16;
                (*i.borrow_mut()).prefix_inc();
            }
        }
        (*(*huff.borrow()).is_last.borrow_mut()) = ({
            let _lhs = ((*pos.borrow()).read());
            _lhs == (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
        });
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
            ({
                let _counts: Ptr<i32> =
                    (((*huff.borrow()).counts.as_pointer() as Ptr<i32>).offset(0_usize as isize));
                let _symbols: Ptr<i32> =
                    (((*huff.borrow()).values.as_pointer() as Ptr<i32>).offset(0_usize as isize));
                let _lut: Ptr<brunsli_HuffmanTableEntry> = (*huff_lut.borrow()).clone();
                BuildJpegHuffmanTable_237(_counts, _symbols, _lut)
            });
        }
        {
            let a0_clone = (*huff.borrow()).clone();
            (*(*(*jpg.borrow()).upgrade().deref())
                .huffman_code
                .borrow_mut())
            .push(a0_clone)
        };
    }
    if {
        let _lhs = (*start_pos.borrow()).wrapping_add((*marker_len.borrow()));
        _lhs != ((*pos.borrow()).read())
    } {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_MARKER_SIZE;
        return false;
    };
    return true;
}
pub fn ProcessDQT_238(
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
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((2) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    if ((*marker_len.borrow()) == 2_usize) {
        write!(
            libcc2rs::cerr(),
            "DQT marker: no quantization table found\n",
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::EMPTY_DQT;
        return false;
    }
    'loop_: while ({
        let _lhs = ((*pos.borrow()).read());
        _lhs < (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
    }) && ({
        let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len();
        _lhs < ((*kMaxQuantTables_5.with(Value::clone).borrow()) as usize)
    }) {
        if {
            let _lhs = ((*pos.borrow()).read()).wrapping_add(((1) as usize));
            _lhs > (*len.borrow())
        } {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (1),
                (*len.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::UNEXPECTED_EOF;
            return false;
        };
        let quant_table_index: Value<i32> = Rc::new(RefCell::new(
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _pos: Ptr<usize> = (*pos.borrow()).clone();
                ReadUint8_232(_data, _pos)
            }),
        ));
        let quant_table_precision: Value<i32> =
            Rc::new(RefCell::new(((*quant_table_index.borrow()) >> 4)));
        if ((*quant_table_precision.borrow()) < 0) || ((*quant_table_precision.borrow()) > 1) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_precision: {:}\n",
                (*quant_table_precision.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_QUANT_TBL_PRECISION;
            return false;
        };
        (*quant_table_index.borrow_mut()) &= 15;
        if ((*quant_table_index.borrow()) < 0) || ((*quant_table_index.borrow()) > 3) {
            write!(
                libcc2rs::cerr(),
                "Invalid quant_table_index: {:}\n",
                (*quant_table_index.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_QUANT_TBL_INDEX;
            return false;
        };
        if {
            let _lhs = ((*pos.borrow()).read()).wrapping_add(
                ((((*quant_table_precision.borrow()) + 1)
                    * (*kDCTBlockSize_3.with(Value::clone).borrow())) as usize),
            );
            _lhs > (*len.borrow())
        } {
            write!(
                libcc2rs::cerr(),
                "Unexpected end of input: pos={:} need={:} len={:}\n",
                ((*pos.borrow()).read()),
                (((*quant_table_precision.borrow()) + 1)
                    * (*kDCTBlockSize_3.with(Value::clone).borrow())),
                (*len.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::UNEXPECTED_EOF;
            return false;
        };
        let table: Value<brunsli_JPEGQuantTable> =
            Rc::new(RefCell::new(<brunsli_JPEGQuantTable>::default()));
        (*(*table.borrow()).index.borrow_mut()) = (*quant_table_index.borrow());
        (*(*table.borrow()).precision.borrow_mut()) = (*quant_table_precision.borrow());
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*kDCTBlockSize_3.with(Value::clone).borrow())) {
            let quant_val: Value<i32> =
                Rc::new(RefCell::new(if ((*quant_table_precision.borrow()) != 0) {
                    ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _pos: Ptr<usize> = (*pos.borrow()).clone();
                        ReadUint16_233(_data, _pos)
                    })
                } else {
                    ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _pos: Ptr<usize> = (*pos.borrow()).clone();
                        ReadUint8_232(_data, _pos)
                    })
                }));
            if ((*quant_val.borrow()) < 1) || ((*quant_val.borrow()) > 65535) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid quant_val: {:}\n",
                    (*quant_val.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_QUANT_VAL;
                return false;
            };
            ((*table.borrow()).values.as_pointer() as Ptr<i32>)
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*i.borrow()) as usize]
                        as usize) as isize,
                )
                .write((*quant_val.borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*table.borrow()).is_last.borrow_mut()) = ({
            let _lhs = ((*pos.borrow()).read());
            _lhs == (*start_pos.borrow()).wrapping_add((*marker_len.borrow()))
        });
        {
            let a0_clone = (*table.borrow()).clone();
            (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow_mut()).push(a0_clone)
        };
    }
    if {
        let _lhs = (*start_pos.borrow()).wrapping_add((*marker_len.borrow()));
        _lhs != ((*pos.borrow()).read())
    } {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_MARKER_SIZE;
        return false;
    };
    return true;
}
pub fn ProcessDRI_239(
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
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::DUPLICATE_DRI;
        return false;
    }
    (*found_dri.borrow()).write(true);
    let start_pos: Value<usize> = Rc::new(RefCell::new(((*pos.borrow()).read())));
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((4) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (4),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    let restart_interval: Value<i32> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }),
    ));
    (*(*(*jpg.borrow()).upgrade().deref())
        .restart_interval
        .borrow_mut()) = (*restart_interval.borrow());
    if {
        let _lhs = (*start_pos.borrow()).wrapping_add((*marker_len.borrow()));
        _lhs != ((*pos.borrow()).read())
    } {
        write!(
            libcc2rs::cerr(),
            "Invalid marker length: declared={:} actual={:}\n",
            (*marker_len.borrow()),
            (((*pos.borrow()).read()).wrapping_sub((*start_pos.borrow()))),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_MARKER_SIZE;
        return false;
    };
    return true;
}
pub fn ProcessAPP_240(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((2) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    if ((*marker_len.borrow()) < 2_usize) || ((*marker_len.borrow()) > 65535_usize) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker_len: {:}\n",
            (*marker_len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_MARKER_LEN;
        return false;
    };
    if {
        let _lhs =
            ((*pos.borrow()).read()).wrapping_add(((*marker_len.borrow()).wrapping_sub(2_usize)));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            ((*marker_len.borrow()).wrapping_sub(2_usize)),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
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
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add((*marker_len.borrow()).wrapping_sub(2_usize));
    (*pos.borrow()).write(rhs_0);
    ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer() as Ptr<Vec<Value<Vec<u8>>>>)
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new((*app_str.borrow()).clone())))
        });
    return true;
}
pub fn ProcessCOM_241(
    data: Ptr<u8>,
    len: usize,
    pos: Ptr<usize>,
    jpg: Ptr<brunsli_JPEGData>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    if {
        let _lhs = ((*pos.borrow()).read()).wrapping_add(((2) as usize));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            (2),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    };
    let marker_len: Value<usize> = Rc::new(RefCell::new(
        (({
            let _data: Ptr<u8> = (*data.borrow()).clone();
            let _pos: Ptr<usize> = (*pos.borrow()).clone();
            ReadUint16_233(_data, _pos)
        }) as usize),
    ));
    if ((*marker_len.borrow()) < 2_usize) || ((*marker_len.borrow()) > 65535_usize) {
        write!(
            libcc2rs::cerr(),
            "Invalid marker_len: {:}\n",
            (*marker_len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_MARKER_LEN;
        return false;
    };
    if {
        let _lhs =
            ((*pos.borrow()).read()).wrapping_add(((*marker_len.borrow()).wrapping_sub(2_usize)));
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of input: pos={:} need={:} len={:}\n",
            ((*pos.borrow()).read()),
            ((*marker_len.borrow()).wrapping_sub(2_usize)),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
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
    let rhs_0 = ((*pos.borrow()).read()).wrapping_add((*marker_len.borrow()).wrapping_sub(2_usize));
    (*pos.borrow()).write(rhs_0);
    ((*(*jpg.borrow()).upgrade().deref()).com_data.as_pointer() as Ptr<Vec<Value<Vec<u8>>>>)
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new((*com_str.borrow()).clone())))
        });
    return true;
}
#[derive(Default)]
pub struct brunsli_BitReaderState {
    pub data_: Value<Ptr<u8>>,
    pub len_: Value<usize>,
    pub pos_: Value<usize>,
    pub val_: Value<u64>,
    pub bits_left_: Value<i32>,
    pub next_marker_pos_: Value<usize>,
}
impl brunsli_BitReaderState {
    pub fn brunsli_BitReaderState(data: Ptr<u8>, len: usize, pos: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        let pos: Value<usize> = Rc::new(RefCell::new(pos));
        let mut this = Self {
            data_: Rc::new(RefCell::new((*data.borrow()).clone())),
            len_: Rc::new(RefCell::new((*len.borrow()))),
            pos_: Rc::new(RefCell::new(0_usize)),
            val_: <Value<u64>>::default(),
            bits_left_: <Value<i32>>::default(),
            next_marker_pos_: Rc::new(RefCell::new(0_usize)),
        };
        ({
            let _pos: usize = (*pos.borrow());
            this.Reset(_pos)
        });
        this
    }
    pub fn Reset(&self, pos: usize) {
        let pos: Value<usize> = Rc::new(RefCell::new(pos));
        (*self.pos_.borrow_mut()) = (*pos.borrow());
        (*self.val_.borrow_mut()) = 0_u64;
        (*self.bits_left_.borrow_mut()) = 0;
        (*self.next_marker_pos_.borrow_mut()) = (*self.len_.borrow()).wrapping_sub(2_usize);
        ({ self.FillBitWindow() });
    }
    pub fn GetNextByte(&self) -> u8 {
        if ((*self.pos_.borrow()) >= (*self.next_marker_pos_.borrow())) {
            (*self.pos_.borrow_mut()).prefix_inc();
            return 0_u8;
        }
        let c: Value<u8> = Rc::new(RefCell::new(
            ((*self.data_.borrow())
                .offset(((*self.pos_.borrow_mut()).postfix_inc()) as isize)
                .read()),
        ));
        if (((*c.borrow()) as i32) == 255) {
            let escape: Value<u8> = Rc::new(RefCell::new(
                ((*self.data_.borrow())
                    .offset((*self.pos_.borrow()) as isize)
                    .read()),
            ));
            if (((*escape.borrow()) as i32) == 0) {
                (*self.pos_.borrow_mut()).prefix_inc();
            } else {
                (*self.next_marker_pos_.borrow_mut()) = (*self.pos_.borrow()).wrapping_sub(1_usize);
            }
        }
        return (*c.borrow());
    }
    pub fn FillBitWindow(&self) {
        if ((*self.bits_left_.borrow()) <= 16) {
            'loop_: while ((*self.bits_left_.borrow()) <= 56) {
                (*self.val_.borrow_mut()) <<= 8;
                (*self.val_.borrow_mut()) |= (({ self.GetNextByte() }) as u64);
                (*self.bits_left_.borrow_mut()) += 8;
            }
        }
    }
    pub fn ReadBits(&self, nbits: i32) -> i32 {
        let nbits: Value<i32> = Rc::new(RefCell::new(nbits));
        ({ self.FillBitWindow() });
        let val: Value<u64> = Rc::new(RefCell::new(
            (((*self.val_.borrow()) >> ((*self.bits_left_.borrow()) - (*nbits.borrow())))
                & ((1_u64 << (*nbits.borrow())).wrapping_sub(1_u64))),
        ));
        (*self.bits_left_.borrow_mut()) -= (*nbits.borrow());
        if !((*val.borrow()) < ((1_u32 << 31) as u64)) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"ReadBits");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"jpeg_data_reader.cc"), 471, _fn)
            });
            'loop_: while true {}
        };
        return ((*val.borrow()) as i32);
    }
    pub fn IsUnhealthy(&self) -> bool {
        return ((*self.pos_.borrow())
            > ((*self.next_marker_pos_.borrow()).wrapping_add(32_usize)));
    }
    pub fn FinishStream(&self, jpg: Ptr<brunsli_JPEGData>, pos: Ptr<usize>) -> bool {
        let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
        let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
        let npadbits: Value<i32> = Rc::new(RefCell::new(((*self.bits_left_.borrow()) & 7)));
        if ((*npadbits.borrow()) > 0) {
            let padmask: Value<u64> = Rc::new(RefCell::new(
                (1_u64 << (*npadbits.borrow())).wrapping_sub(1_u64),
            ));
            let padbits: Value<u64> = Rc::new(RefCell::new(
                (((*self.val_.borrow()) >> ((*self.bits_left_.borrow()) - (*npadbits.borrow())))
                    & (*padmask.borrow())),
            ));
            if ((*padbits.borrow()) != (*padmask.borrow())) {
                (*(*(*jpg.borrow()).upgrade().deref())
                    .has_zero_padding_bit
                    .borrow_mut()) = true;
            }
            let i: Value<i32> = Rc::new(RefCell::new(((*npadbits.borrow()) - 1)));
            'loop_: while ((*i.borrow()) >= 0) {
                (*(*(*jpg.borrow()).upgrade().deref())
                    .padding_bits
                    .borrow_mut())
                .push(((((*padbits.borrow()) >> (*i.borrow())) & 1_u64) as i32));
                (*i.borrow_mut()).prefix_dec();
            }
        }
        let unused_bytes_left: Value<i32> =
            Rc::new(RefCell::new(((*self.bits_left_.borrow()) >> 3)));
        'loop_: while ((*unused_bytes_left.borrow_mut()).postfix_dec() > 0) {
            (*self.pos_.borrow_mut()).prefix_dec();
            if (((*self.pos_.borrow()) < (*self.next_marker_pos_.borrow()))
                && ((((*self.data_.borrow())
                    .offset((*self.pos_.borrow()) as isize)
                    .read()) as i32)
                    == 0))
                && ((((*self.data_.borrow())
                    .offset(((*self.pos_.borrow()).wrapping_sub(1_usize)) as isize)
                    .read()) as i32)
                    == 255)
            {
                (*self.pos_.borrow_mut()).prefix_dec();
            }
        }
        if ((*self.pos_.borrow()) > (*self.next_marker_pos_.borrow())) {
            write!(libcc2rs::cerr(), "Unexpected end of scan.\n",);
            return false;
        }
        let __rhs = (*self.pos_.borrow());
        (*pos.borrow()).write(__rhs);
        return true;
    }
}
impl Clone for brunsli_BitReaderState {
    fn clone(&self) -> Self {
        let mut this = Self {
            data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
            len_: Rc::new(RefCell::new((*self.len_.borrow()))),
            pos_: Rc::new(RefCell::new((*self.pos_.borrow()))),
            val_: Rc::new(RefCell::new((*self.val_.borrow()))),
            bits_left_: Rc::new(RefCell::new((*self.bits_left_.borrow()))),
            next_marker_pos_: Rc::new(RefCell::new((*self.next_marker_pos_.borrow()))),
        };
        this
    }
}
impl ByteRepr for brunsli_BitReaderState {}
pub fn ReadSymbol_242(
    table: Ptr<brunsli_HuffmanTableEntry>,
    br: Ptr<brunsli_BitReaderState>,
) -> i32 {
    let table: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(table));
    let br: Value<Ptr<brunsli_BitReaderState>> = Rc::new(RefCell::new(br));
    let nbits: Value<i32> = <Value<i32>>::default();
    ({ (*(*br.borrow()).upgrade().deref()).FillBitWindow() });
    let val: Value<i32> = Rc::new(RefCell::new(
        ((({
            let _lhs = (*(*(*br.borrow()).upgrade().deref()).val_.borrow());
            _lhs >> ((*(*(*br.borrow()).upgrade().deref()).bits_left_.borrow()) - 8)
        }) & 255_u64) as i32),
    ));
    (*table.borrow_mut()) += (*val.borrow());
    (*nbits.borrow_mut()) = (((*(*(*table.borrow()).upgrade().deref()).bits.borrow()) as i32) - 8);
    if ((*nbits.borrow()) > 0) {
        (*(*(*br.borrow()).upgrade().deref()).bits_left_.borrow_mut()) -= 8;
        let __rhs = ((*(*(*table.borrow()).upgrade().deref()).value.borrow()) as i32);
        (*table.borrow_mut()) += __rhs;
        (*val.borrow_mut()) = (({
            let _lhs = ({
                let _lhs = (*(*(*br.borrow()).upgrade().deref()).val_.borrow());
                _lhs >> ({
                    let _lhs = (*(*(*br.borrow()).upgrade().deref()).bits_left_.borrow());
                    _lhs - (*nbits.borrow())
                })
            });
            _lhs & (((1 << (*nbits.borrow())) - 1) as u64)
        }) as i32);
        (*table.borrow_mut()) += (*val.borrow());
    }
    (*(*(*br.borrow()).upgrade().deref()).bits_left_.borrow_mut()) -=
        ((*(*(*table.borrow()).upgrade().deref()).bits.borrow()) as i32);
    return ((*(*(*table.borrow()).upgrade().deref()).value.borrow()) as i32);
}
pub fn HuffExtend_243(x: i32, s: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let s: Value<i32> = Rc::new(RefCell::new(s));
    if !((*s.borrow()) >= 1) {
        ({
            let _fn: Ptr<u8> = Ptr::from_string_literal(b"HuffExtend");
            BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"jpeg_data_reader.cc"), 575, _fn)
        });
        'loop_: while true {}
    };
    let half: Value<i32> = Rc::new(RefCell::new((1 << ((*s.borrow()) - 1))));
    if ((*x.borrow()) >= (*half.borrow())) {
        if !((*x.borrow()) < (1 << (*s.borrow()))) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"HuffExtend");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"jpeg_data_reader.cc"), 578, _fn)
            });
            'loop_: while true {}
        };
        return (*x.borrow());
    } else {
        return (((*x.borrow()) - (1 << (*s.borrow()))) + 1);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DecodeDCTBlock_244(
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
            ({
                let _table: Ptr<brunsli_HuffmanTableEntry> = (*dc_huff.borrow()).clone();
                let _br: Ptr<brunsli_BitReaderState> = (*br.borrow()).clone();
                ReadSymbol_242(_table, _br)
            }),
        ));
        if ((*s.borrow()) >= (*kJpegDCAlphabetSize_9.with(Value::clone).borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for DC coefficient.\n",
                (*s.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_SYMBOL;
            return false;
        }
        let diff: Value<i32> = Rc::new(RefCell::new(0));
        if ((*s.borrow()) > 0) {
            let bits: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _nbits: i32 = (*s.borrow());
                    (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits)
                }),
            ));
            (*diff.borrow_mut()) = ({
                let _x: i32 = (*bits.borrow());
                let _s: i32 = (*s.borrow());
                HuffExtend_243(_x, _s)
            });
        }
        let coeff: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*diff.borrow());
            _lhs + (((*last_dc_coeff.borrow()).read()) as i32)
        }));
        let dc_coeff: Value<i32> = Rc::new(RefCell::new(((*coeff.borrow()) * (*Am.borrow()))));
        let __rhs = ((*dc_coeff.borrow()) as i16);
        (*coeffs.borrow()).offset((0) as isize).write(__rhs);
        if {
            let _lhs = (*dc_coeff.borrow());
            _lhs != (((*coeffs.borrow()).offset((0) as isize).read()) as i32)
        } {
            write!(
                libcc2rs::cerr(),
                "Invalid DC coefficient {:}\n",
                (*dc_coeff.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::NON_REPRESENTABLE_DC_COEFF;
            return false;
        }
        let __rhs = ((*coeff.borrow()) as i16);
        (*last_dc_coeff.borrow()).write(__rhs);
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
            ({
                let _table: Ptr<brunsli_HuffmanTableEntry> = (*ac_huff.borrow()).clone();
                let _br: Ptr<brunsli_BitReaderState> = (*br.borrow()).clone();
                ReadSymbol_242(_table, _br)
            }),
        ));
        if ((*sr.borrow()) >= (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow())) {
            write!(
                libcc2rs::cerr(),
                "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                (*sr.borrow()),
                (*k.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::INVALID_SYMBOL;
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
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::OUT_OF_BAND_COEFF;
                return false;
            }
            if (((*s.borrow()) + (*Al.borrow()))
                >= (*kJpegDCAlphabetSize_9.with(Value::clone).borrow()))
            {
                write!(
                    libcc2rs::cerr(),
                    "Out of range AC coefficient value: s = {:} Al = {:} k = {:}\n",
                    (*s.borrow()),
                    (*Al.borrow()),
                    (*k.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::NON_REPRESENTABLE_AC_COEFF;
                return false;
            }
            let bits: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _nbits: i32 = (*s.borrow());
                    (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits)
                }),
            ));
            let coeff: Value<i32> = Rc::new(RefCell::new(
                ({
                    let _x: i32 = (*bits.borrow());
                    let _s: i32 = (*s.borrow());
                    HuffExtend_243(_x, _s)
                }),
            ));
            let __rhs = (((*coeff.borrow()) * (*Am.borrow())) as i16);
            (*coeffs.borrow())
                .offset(
                    ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())[(*k.borrow()) as usize])
                        as isize,
                )
                .write(__rhs);
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
            let __rhs = (1 << (*r.borrow()));
            (*eobrun.borrow()).write(__rhs);
            if ((*r.borrow()) > 0) {
                if !(*eobrun_allowed.borrow()) {
                    write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                    (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                        brunsli_JPEGReadError::EOB_RUN_TOO_LONG;
                    return false;
                }
                let __rhs = ({
                    let _nbits: i32 = (*r.borrow());
                    (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits)
                });
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
pub fn RefineDCTBlock_245(
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
            ({ (*(*br.borrow()).upgrade().deref()).ReadBits(1) }),
        ));
        let dc_coeff: Value<i16> = Rc::new(RefCell::new(
            ((*coeffs.borrow()).offset((0) as isize).read()),
        ));
        let rhs_0 = (((*dc_coeff.borrow()) as i32) | ((*s.borrow()) * (*Am.borrow()))) as i16;
        (*dc_coeff.borrow_mut()) = rhs_0;
        let __rhs = (*dc_coeff.borrow());
        (*coeffs.borrow()).offset((0) as isize).write(__rhs);
        (*Ss.borrow_mut()).prefix_inc();
    }
    if ((*Ss.borrow()) > (*Se.borrow())) {
        return true;
    }
    let p1: Value<i32> = Rc::new(RefCell::new((*Am.borrow())));
    let m1: Value<i32> = Rc::new(RefCell::new(-(*Am.borrow())));
    let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
    let r: Value<i32> = <Value<i32>>::default();
    let s: Value<i32> = <Value<i32>>::default();
    let in_zero_run: Value<bool> = Rc::new(RefCell::new(false));
    if (((*eobrun.borrow()).read()) <= 0) {
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            (*s.borrow_mut()) = ({
                let _table: Ptr<brunsli_HuffmanTableEntry> = (*ac_huff.borrow()).clone();
                let _br: Ptr<brunsli_BitReaderState> = (*br.borrow()).clone();
                ReadSymbol_242(_table, _br)
            });
            if ((*s.borrow()) >= (*kJpegHuffmanAlphabetSize_8.with(Value::clone).borrow())) {
                write!(
                    libcc2rs::cerr(),
                    "Invalid Huffman symbol {:} for AC coefficient {:}\n",
                    (*s.borrow()),
                    (*k.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_SYMBOL;
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
                    (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                        brunsli_JPEGReadError::INVALID_SYMBOL;
                    return false;
                }
                (*s.borrow_mut()) = if (({ (*(*br.borrow()).upgrade().deref()).ReadBits(1) }) != 0)
                {
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
                    let __rhs = (1 << (*r.borrow()));
                    (*eobrun.borrow()).write(__rhs);
                    if ((*r.borrow()) > 0) {
                        if !(*eobrun_allowed.borrow()) {
                            write!(libcc2rs::cerr(), "End-of-block run crossing DC coeff.\n",);
                            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                                brunsli_JPEGReadError::EOB_RUN_TOO_LONG;
                            return false;
                        }
                        let __rhs = ({
                            let _nbits: i32 = (*r.borrow());
                            (*(*br.borrow()).upgrade().deref()).ReadBits(_nbits)
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
                    ((*coeffs.borrow())
                        .offset(
                            ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                                [(*k.borrow()) as usize]) as isize,
                        )
                        .read()),
                ));
                if (((*thiscoef.borrow()) as i32) != 0) {
                    if (({ (*(*br.borrow()).upgrade().deref()).ReadBits(1) }) != 0) {
                        if ((((*thiscoef.borrow()) as i32) & (*p1.borrow())) == 0) {
                            if (((*thiscoef.borrow()) as i32) >= 0) {
                                let rhs_0 = (((*thiscoef.borrow()) as i32) + (*p1.borrow())) as i16;
                                (*thiscoef.borrow_mut()) = rhs_0;
                            } else {
                                let rhs_0 = (((*thiscoef.borrow()) as i32) + (*m1.borrow())) as i16;
                                (*thiscoef.borrow_mut()) = rhs_0;
                            }
                        }
                    }
                    let __rhs = (*thiscoef.borrow());
                    (*coeffs.borrow())
                        .offset(
                            ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                                [(*k.borrow()) as usize]) as isize,
                        )
                        .write(__rhs);
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
                    (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                        brunsli_JPEGReadError::OUT_OF_BAND_COEFF;
                    return false;
                }
                let __rhs = ((*s.borrow()) as i16);
                (*coeffs.borrow())
                    .offset(
                        ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                            [(*k.borrow()) as usize]) as isize,
                    )
                    .write(__rhs);
            }
            (*k.borrow_mut()).postfix_inc();
        }
    }
    if (*in_zero_run.borrow()) {
        write!(libcc2rs::cerr(), "Extra zero run before end-of-block.\n",);
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::EXTRA_ZERO_RUN;
        return false;
    }
    if (((*eobrun.borrow()).read()) > 0) {
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            let thiscoef: Value<i16> = Rc::new(RefCell::new(
                ((*coeffs.borrow())
                    .offset(
                        ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                            [(*k.borrow()) as usize]) as isize,
                    )
                    .read()),
            ));
            if (((*thiscoef.borrow()) as i32) != 0) {
                if (({ (*(*br.borrow()).upgrade().deref()).ReadBits(1) }) != 0) {
                    if ((((*thiscoef.borrow()) as i32) & (*p1.borrow())) == 0) {
                        if (((*thiscoef.borrow()) as i32) >= 0) {
                            let rhs_0 = (((*thiscoef.borrow()) as i32) + (*p1.borrow())) as i16;
                            (*thiscoef.borrow_mut()) = rhs_0;
                        } else {
                            let rhs_0 = (((*thiscoef.borrow()) as i32) + (*m1.borrow())) as i16;
                            (*thiscoef.borrow_mut()) = rhs_0;
                        }
                    }
                }
                let __rhs = (*thiscoef.borrow());
                (*coeffs.borrow())
                    .offset(
                        ((*kJPEGNaturalOrder_13.with(Value::clone).borrow())
                            [(*k.borrow()) as usize]) as isize,
                    )
                    .write(__rhs);
            }
            (*k.borrow_mut()).postfix_inc();
        }
    }
    (*eobrun.borrow()).with_mut(|__v| __v.prefix_dec());
    return true;
}
pub fn ProcessRestart_246(
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
        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
        let _pos: Ptr<usize> = (pos.as_pointer());
        (*(*br.borrow()).upgrade().deref()).FinishStream(_jpg, _pos)
    }) {
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_SCAN;
        return false;
    }
    let expected_marker: Value<i32> = Rc::new(RefCell::new(
        (208 + ((*next_restart_marker.borrow()).read())),
    ));
    if ((*pos.borrow()).wrapping_add(2_usize) > (*len.borrow()))
        || ((((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < (*len.borrow())) {
                (((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND;
        return false;
    };
    let marker: Value<i32> = Rc::new(RefCell::new(
        (((*data.borrow())
            .offset(((*pos.borrow()).wrapping_add(1_usize)) as isize)
            .read()) as i32),
    ));
    if ((*marker.borrow()) != (*expected_marker.borrow())) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected restart marker {:} actual={:}\n",
            (*expected_marker.borrow()),
            (*marker.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::WRONG_RESTART_MARKER;
        return false;
    }
    ({
        let _pos: usize = (*pos.borrow()).wrapping_add(2_usize);
        (*(*br.borrow()).upgrade().deref()).Reset(_pos)
    });
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
pub fn ProcessScan_247(
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
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _len: usize = (*len.borrow());
        let _pos: Ptr<usize> = (*pos.borrow()).clone();
        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
        ProcessSOS_235(_data, _len, _pos, _jpg)
    }) {
        return false;
    }
    let scan_info: Value<Ptr<brunsli_JPEGScanInfo>> = Rc::new(RefCell::new(
        (((*(*jpg.borrow()).upgrade().deref()).scan_info.as_pointer()
            as Ptr<brunsli_JPEGScanInfo>)
            .to_last()),
    ));
    let is_interleaved: Value<bool> = Rc::new(RefCell::new(
        ((*(*(*scan_info.borrow()).upgrade().deref())
            .num_components
            .borrow())
            > 1_usize),
    ));
    let MCUs_per_row: Value<i32> = <Value<i32>>::default();
    let MCU_rows: Value<i32> = <Value<i32>>::default();
    if (*is_interleaved.borrow()) {
        (*MCUs_per_row.borrow_mut()) = (*(*(*jpg.borrow()).upgrade().deref()).MCU_cols.borrow());
        (*MCU_rows.borrow_mut()) = (*(*(*jpg.borrow()).upgrade().deref()).MCU_rows.borrow());
    } else {
        let c: Ptr<brunsli_JPEGComponent> = ((*(*jpg.borrow()).upgrade().deref())
            .components
            .as_pointer()
            as Ptr<brunsli_JPEGComponent>)
            .offset(
                ((*(*((*(*scan_info.borrow()).upgrade().deref())
                    .components
                    .as_pointer() as Ptr<brunsli_JPEGComponentScanInfo>)
                    .offset(0_usize as isize)
                    .upgrade()
                    .deref())
                .comp_idx
                .borrow()) as usize) as isize,
            );
        (*MCUs_per_row.borrow_mut()) = ({
            let _a: i32 = {
                let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).width.borrow());
                _lhs * (*(*c.upgrade().deref()).h_samp_factor.borrow())
            };
            let _b: i32 = (8
                * (*(*(*jpg.borrow()).upgrade().deref())
                    .max_h_samp_factor
                    .borrow()));
            DivCeil_231(_a, _b)
        });
        (*MCU_rows.borrow_mut()) = ({
            let _a: i32 = {
                let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).height.borrow());
                _lhs * (*(*c.upgrade().deref()).v_samp_factor.borrow())
            };
            let _b: i32 = (8
                * (*(*(*jpg.borrow()).upgrade().deref())
                    .max_v_samp_factor
                    .borrow()));
            DivCeil_231(_a, _b)
        });
    }
    let last_dc_coeff: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
        0_i16,
        <i16>::default(),
        <i16>::default(),
        <i16>::default(),
    ])));
    let br: Value<brunsli_BitReaderState> = Rc::new(RefCell::new(
        brunsli_BitReaderState::brunsli_BitReaderState(
            (*data.borrow()).clone(),
            (*len.borrow()),
            ((*pos.borrow()).read()),
        ),
    ));
    let restarts_to_go: Value<i32> = Rc::new(RefCell::new(
        (*(*(*jpg.borrow()).upgrade().deref())
            .restart_interval
            .borrow()),
    ));
    let next_restart_marker: Value<i32> = Rc::new(RefCell::new(0));
    let eobrun: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let block_scan_index: Value<i32> = Rc::new(RefCell::new(0));
    let Al: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*(*scan_info.borrow()).upgrade().deref()).Al.borrow())
    } else {
        0
    }));
    let Ah: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*(*scan_info.borrow()).upgrade().deref()).Ah.borrow())
    } else {
        0
    }));
    let Ss: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*(*scan_info.borrow()).upgrade().deref()).Ss.borrow())
    } else {
        0
    }));
    let Se: Value<i32> = Rc::new(RefCell::new(if (*is_progressive.borrow()) {
        (*(*(*scan_info.borrow()).upgrade().deref()).Se.borrow())
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
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*scan_info.borrow()).upgrade().deref())
            .num_components
            .borrow())
    } {
        let comp_idx: Value<i32> = Rc::new(RefCell::new(
            ((*(*((*(*scan_info.borrow()).upgrade().deref())
                .components
                .as_pointer() as Ptr<brunsli_JPEGComponentScanInfo>)
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .comp_idx
            .borrow()) as i32),
        ));
        let k: Value<i32> = Rc::new(RefCell::new((*Ss.borrow())));
        'loop_: while ((*k.borrow()) <= (*Se.borrow())) {
            if ({
                let _lhs = (((*scan_progression.borrow())
                    .offset((*comp_idx.borrow()) as isize)
                    .read())
                .borrow()[(*k.borrow()) as usize] as i32);
                _lhs & ((*scan_bitmask.borrow()) as i32)
            } != 0)
            {
                write!(
                    libcc2rs::cerr(),
                    "Overlapping scans: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n",
                    (*comp_idx.borrow()),
                    (*k.borrow()),
                    ((*scan_progression.borrow())
                        .offset((*i.borrow()) as isize)
                        .read())
                    .borrow()[(*k.borrow()) as usize],
                    (*scan_bitmask.borrow()),
                );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::OVERLAPPING_SCANS;
                return false;
            }
            if ({
                let _lhs = (((*scan_progression.borrow())
                    .offset((*comp_idx.borrow()) as isize)
                    .read())
                .borrow()[(*k.borrow()) as usize] as i32);
                _lhs & ((*refinement_bitmask.borrow()) as i32)
            } != 0)
            {
                write!( libcc2rs::cerr()  , "Invalid scan order, a more refined scan was already done: component = {:} k = {:} prev_mask: {:} cur_mask: {:}\n", (*comp_idx.borrow()) , (*k.borrow()) , (  (*scan_progression.borrow()) . offset ( ( (*i.borrow()) ) as isize ) .read() ) .borrow() [((*k.borrow()) ) as usize] , (*scan_bitmask.borrow()) ,  );
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_SCAN_ORDER;
                return false;
            }
            let rhs_0 = ((((*scan_progression.borrow())
                .offset((*comp_idx.borrow()) as isize)
                .read())
            .borrow()[(*k.borrow()) as usize] as i32)
                | ((*scan_bitmask.borrow()) as i32)) as u16;
            ((*scan_progression.borrow())
                .offset((*comp_idx.borrow()) as isize)
                .read())
            .borrow_mut()[(*k.borrow()) as usize] = rhs_0;
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
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::NON_REPRESENTABLE_AC_COEFF;
        return false;
    }
    let mcu_y: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*mcu_y.borrow()) < (*MCU_rows.borrow())) {
        let mcu_x: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*mcu_x.borrow()) < (*MCUs_per_row.borrow())) {
            if ((*(*(*jpg.borrow()).upgrade().deref())
                .restart_interval
                .borrow())
                > 0)
            {
                if ((*restarts_to_go.borrow()) == 0) {
                    if ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _len: usize = (*len.borrow());
                        let _next_restart_marker: Ptr<i32> = (next_restart_marker.as_pointer());
                        let _br: Ptr<brunsli_BitReaderState> = (br.as_pointer());
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessRestart_246(_data, _len, _next_restart_marker, _br, _jpg)
                    }) {
                        (*restarts_to_go.borrow_mut()) = (*(*(*jpg.borrow()).upgrade().deref())
                            .restart_interval
                            .borrow());
                        {
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                                .to_any()
                                .memset((0) as u8, ::std::mem::size_of::<[i16; 4]>() as usize);
                            ((last_dc_coeff.as_pointer() as Ptr<i16>) as Ptr<i16>)
                                .to_any()
                                .clone()
                        };
                        if ((*eobrun.borrow()) > 0) {
                            write!(libcc2rs::cerr(), "End-of-block run too long.\n",);
                            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                                brunsli_JPEGReadError::EOB_RUN_TOO_LONG;
                            return false;
                        }
                        (*eobrun.borrow_mut()) = -1_i32;
                    } else {
                        return false;
                    }
                }
                (*restarts_to_go.borrow_mut()).prefix_dec();
            }
            if ({ (*br.borrow()).IsUnhealthy() }) {
                write!(libcc2rs::cerr(), "Unexpected end of scan.\n",);
                (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                    brunsli_JPEGReadError::INVALID_SCAN;
                return false;
            }
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while {
                let _lhs = (*i.borrow());
                _lhs < (*(*(*scan_info.borrow()).upgrade().deref())
                    .num_components
                    .borrow())
            } {
                let si: Value<Ptr<brunsli_JPEGComponentScanInfo>> = Rc::new(RefCell::new(
                    (((*(*scan_info.borrow()).upgrade().deref())
                        .components
                        .as_pointer() as Ptr<brunsli_JPEGComponentScanInfo>)
                        .offset((*i.borrow()) as isize)),
                ));
                let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
                    (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                        as Ptr<brunsli_JPEGComponent>)
                        .offset(
                            ((*(*(*si.borrow()).upgrade().deref()).comp_idx.borrow()) as usize)
                                as isize,
                        )),
                ));
                let dc_lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
                    ((dc_huff_lut.to_strong().as_pointer() as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({
                                let _lhs =
                                    (*(*(*si.borrow()).upgrade().deref()).dc_tbl_idx.borrow());
                                _lhs * (*kJpegHuffmanLutSize_230.with(Value::clone).borrow())
                            }) as usize) as isize,
                        )),
                ));
                let ac_lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(
                    ((ac_huff_lut.to_strong().as_pointer() as Ptr<brunsli_HuffmanTableEntry>)
                        .offset(
                            (({
                                let _lhs =
                                    (*(*(*si.borrow()).upgrade().deref()).ac_tbl_idx.borrow());
                                _lhs * (*kJpegHuffmanLutSize_230.with(Value::clone).borrow())
                            }) as usize) as isize,
                        )),
                ));
                let nblocks_y: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*(*c.borrow()).upgrade().deref()).v_samp_factor.borrow())
                } else {
                    1
                }));
                let nblocks_x: Value<i32> = Rc::new(RefCell::new(if (*is_interleaved.borrow()) {
                    (*(*(*c.borrow()).upgrade().deref()).h_samp_factor.borrow())
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
                            (((((*block_y.borrow()) as u32).wrapping_mul(
                                (*(*(*c.borrow()).upgrade().deref()).width_in_blocks.borrow()),
                            ))
                            .wrapping_add(((*block_x.borrow()) as u32)))
                                as i32),
                        ));
                        let reset_state: Value<bool> = Rc::new(RefCell::new(false));
                        let num_zero_runs: Value<i32> = Rc::new(RefCell::new(0));
                        let coeffs: Value<Ptr<i16>> = Rc::new(RefCell::new(
                            (((*(*c.borrow()).upgrade().deref()).coeffs.as_pointer() as Ptr<i16>)
                                .offset(
                                    (((*block_idx.borrow())
                                        * (*kDCTBlockSize_3.with(Value::clone).borrow()))
                                        as usize) as isize,
                                )),
                        ));
                        if ((*Ah.borrow()) == 0) {
                            if !({
                                let _dc_huff: Ptr<brunsli_HuffmanTableEntry> =
                                    (*dc_lut.borrow()).clone();
                                let _ac_huff: Ptr<brunsli_HuffmanTableEntry> =
                                    (*ac_lut.borrow()).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _eobrun: Ptr<i32> = (eobrun.as_pointer());
                                let _reset_state: Ptr<bool> = (reset_state.as_pointer());
                                let _num_zero_runs: Ptr<i32> = (num_zero_runs.as_pointer());
                                let _br: Ptr<brunsli_BitReaderState> = (br.as_pointer());
                                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                                let _last_dc_coeff: Ptr<i16> =
                                    ((last_dc_coeff.as_pointer() as Ptr<i16>).offset(
                                        (*(*(*si.borrow()).upgrade().deref()).comp_idx.borrow())
                                            as isize,
                                    ));
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                DecodeDCTBlock_244(
                                    _dc_huff,
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _eobrun,
                                    _reset_state,
                                    _num_zero_runs,
                                    _br,
                                    _jpg,
                                    _last_dc_coeff,
                                    _coeffs,
                                )
                            }) {
                                return false;
                            }
                        } else {
                            if !({
                                let _ac_huff: Ptr<brunsli_HuffmanTableEntry> =
                                    (*ac_lut.borrow()).clone();
                                let _Ss: i32 = (*Ss.borrow());
                                let _Se: i32 = (*Se.borrow());
                                let _Al: i32 = (*Al.borrow());
                                let _eobrun: Ptr<i32> = (eobrun.as_pointer());
                                let _reset_state: Ptr<bool> = (reset_state.as_pointer());
                                let _br: Ptr<brunsli_BitReaderState> = (br.as_pointer());
                                let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                                let _coeffs: Ptr<i16> = (*coeffs.borrow()).clone();
                                RefineDCTBlock_245(
                                    _ac_huff,
                                    _Ss,
                                    _Se,
                                    _Al,
                                    _eobrun,
                                    _reset_state,
                                    _br,
                                    _jpg,
                                    _coeffs,
                                )
                            }) {
                                return false;
                            }
                        }
                        if (*reset_state.borrow()) {
                            (*(*scan_info.borrow()).upgrade().deref())
                                .reset_points
                                .as_pointer()
                                .with_mut(|__v: &mut Vec<i32>| {
                                    __v.push((*block_scan_index.borrow_mut()) as i32)
                                });
                        }
                        if ((*num_zero_runs.borrow()) > 0) {
                            let info: Value<brunsli_JPEGScanInfo_ExtraZeroRunInfo> = Rc::new(
                                RefCell::new(<brunsli_JPEGScanInfo_ExtraZeroRunInfo>::default()),
                            );
                            (*(*info.borrow()).block_idx.borrow_mut()) =
                                (*block_scan_index.borrow());
                            (*(*info.borrow()).num_extra_zero_runs.borrow_mut()) =
                                (*num_zero_runs.borrow());
                            {
                                let a0_clone = (*info.borrow()).clone();
                                (*(*(*scan_info.borrow()).upgrade().deref())
                                    .extra_zero_runs
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
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::EOB_RUN_TOO_LONG;
        return false;
    }
    if !({
        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
        let _pos: Ptr<usize> = (*pos.borrow()).clone();
        (*br.borrow()).FinishStream(_jpg, _pos)
    }) {
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::INVALID_SCAN;
        return false;
    }
    if {
        let _lhs = ((*pos.borrow()).read());
        _lhs > (*len.borrow())
    } {
        write!(
            libcc2rs::cerr(),
            "Unexpected end of file during scan. pos={:} len={:}\n",
            ((*pos.borrow()).read()),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::UNEXPECTED_EOF;
        return false;
    }
    return true;
}
pub fn FixupIndexes_248(jpg: Ptr<brunsli_JPEGData>) -> bool {
    let jpg: Value<Ptr<brunsli_JPEGData>> = Rc::new(RefCell::new(jpg));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*(*jpg.borrow()).upgrade().deref()).components.borrow()).len()
    } {
        let c: Value<Ptr<brunsli_JPEGComponent>> = Rc::new(RefCell::new(
            (((*(*jpg.borrow()).upgrade().deref()).components.as_pointer()
                as Ptr<brunsli_JPEGComponent>)
                .offset((*i.borrow()) as isize)),
        ));
        let found_index: Value<bool> = Rc::new(RefCell::new(false));
        let j: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*j.borrow());
            _lhs < (*(*(*jpg.borrow()).upgrade().deref()).quant.borrow()).len()
        } {
            if {
                let _lhs = (*(*((*(*jpg.borrow()).upgrade().deref()).quant.as_pointer()
                    as Ptr<brunsli_JPEGQuantTable>)
                    .offset((*j.borrow()) as isize)
                    .upgrade()
                    .deref())
                .index
                .borrow());
                _lhs == ((*(*(*c.borrow()).upgrade().deref()).quant_idx.borrow()) as i32)
            } {
                (*(*(*c.borrow()).upgrade().deref()).quant_idx.borrow_mut()) =
                    ((*j.borrow()) as u8);
                (*found_index.borrow_mut()) = true;
                break;
            }
            (*j.borrow_mut()).prefix_inc();
        }
        if !(*found_index.borrow()) {
            write!(libcc2rs::cerr(), "Quantization table with index ",);
            libcc2rs::cerr().write_all(
                &([
                    (&[(*(*(*c.borrow()).upgrade().deref()).quant_idx.borrow())] as &[u8]),
                    (b" not found." as &[u8]),
                    (&[b'\n'] as &[u8]),
                ]
                .concat()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::QUANT_TABLE_NOT_FOUND;
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn FindNextMarker_249(data: Ptr<u8>, len: usize, pos: usize) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let pos: Value<usize> = Rc::new(RefCell::new(pos));
    thread_local!(
        static kIsValidMarker_250: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
            1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8,
        ])));
    );
    let num_skipped: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*pos.borrow()).wrapping_add(1_usize) < (*len.borrow()))
        && ((((((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32) != 255)
            || ((((*data.borrow())
                .offset(((*pos.borrow()).wrapping_add(1_usize)) as isize)
                .read()) as i32)
                < 192))
            || (!((*kIsValidMarker_250.with(Value::clone).borrow())[((((*data.borrow())
                .offset(((*pos.borrow()).wrapping_add(1_usize)) as isize)
                .read())
                as i32)
                - 192)
                as usize]
                != 0)))
    {
        (*pos.borrow_mut()).prefix_inc();
        (*num_skipped.borrow_mut()).prefix_inc();
    }
    return (*num_skipped.borrow());
}
pub fn ReadJpeg_195(
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
        || ((((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32) != 255)
    {
        write!(
            libcc2rs::cerr(),
            "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
            (if ((*pos.borrow()) < (*len.borrow())) {
                (((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32)
            } else {
                0
            }),
            (*pos.borrow()),
            (*len.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND;
        return false;
    };
    let marker: Value<i32> = Rc::new(RefCell::new(
        (((*data.borrow())
            .offset(((*pos.borrow()).wrapping_add(1_usize)) as isize)
            .read()) as i32),
    ));
    let rhs_0 = (*pos.borrow()).wrapping_add(2_usize);
    (*pos.borrow_mut()) = rhs_0;
    if ((*marker.borrow()) != 216) {
        write!(
            libcc2rs::cerr(),
            "Did not find expected SOI marker, actual={:}\n",
            (*marker.borrow()),
        );
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::SOI_NOT_FOUND;
        return false;
    }
    let lut_size: Value<i32> = Rc::new(RefCell::new(
        ((*kMaxHuffmanTables_6.with(Value::clone).borrow())
            * (*kJpegHuffmanLutSize_230.with(Value::clone).borrow())),
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
            <u16>::default(),
            <u16>::default(),
            <u16>::default(),
        ]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
        Rc::new(RefCell::new(Box::new([0; 64]))),
    ])));
    {
        let __a0 = 0_usize as usize;
        (*(*(*jpg.borrow()).upgrade().deref())
            .padding_bits
            .borrow_mut())
        .resize_with(__a0, || <i32>::default())
    };
    let is_progressive: Value<bool> = Rc::new(RefCell::new(false));
    let mut __do_while = true;
    'loop_: while __do_while || ((*marker.borrow()) != 217) {
        __do_while = false;
        let num_skipped: Value<usize> = Rc::new(RefCell::new(
            ({
                let _data: Ptr<u8> = (*data.borrow()).clone();
                let _len: usize = (*len.borrow());
                let _pos: usize = (*pos.borrow());
                FindNextMarker_249(_data, _len, _pos)
            }),
        ));
        if ((*num_skipped.borrow()) > 0_usize) {
            (*(*(*jpg.borrow()).upgrade().deref())
                .marker_order
                .borrow_mut())
            .push(255_u8);
            ((*(*jpg.borrow()).upgrade().deref())
                .inter_marker_data
                .as_pointer() as Ptr<Vec<Value<Vec<u8>>>>)
                .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
                    __v.push(Rc::new(RefCell::new(
                        {
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
                        }
                        .clone(),
                    )))
                });
            let rhs_0 = (*pos.borrow()).wrapping_add((*num_skipped.borrow()));
            (*pos.borrow_mut()) = rhs_0;
        }
        if ((*pos.borrow()).wrapping_add(2_usize) > (*len.borrow()))
            || ((((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32) != 255)
        {
            write!(
                libcc2rs::cerr(),
                "Marker byte (0xff) expected, found: {:} pos={:} len={:}\n",
                (if ((*pos.borrow()) < (*len.borrow())) {
                    (((*data.borrow()).offset((*pos.borrow()) as isize).read()) as i32)
                } else {
                    0
                }),
                (*pos.borrow()),
                (*len.borrow()),
            );
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::MARKER_BYTE_NOT_FOUND;
            return false;
        };
        (*marker.borrow_mut()) = (((*data.borrow())
            .offset(((*pos.borrow()).wrapping_add(1_usize)) as isize)
            .read()) as i32);
        let rhs_0 = (*pos.borrow()).wrapping_add(2_usize);
        (*pos.borrow_mut()) = rhs_0;
        let ok: Value<bool> = Rc::new(RefCell::new(true));
        'switch: {
            let __match_cond = (*marker.borrow());
            match __match_cond {
                __v if __v == 192 || __v == 193 || __v == 194 => {
                    (*is_progressive.borrow_mut()) = ((*marker.borrow()) == 194);
                    (*ok.borrow_mut()) = ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _len: usize = (*len.borrow());
                        let _mode: brunsli_JpegReadMode = (*mode.borrow()).clone();
                        let _pos: Ptr<usize> = (pos.as_pointer());
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessSOF_234(_data, _len, _mode, _pos, _jpg)
                    });
                    (*found_sof.borrow_mut()) = true;
                    break 'switch;
                }
                __v if __v == 196 => {
                    (*ok.borrow_mut()) = ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _len: usize = (*len.borrow());
                        let _mode: brunsli_JpegReadMode = (*mode.borrow()).clone();
                        let _dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>> =
                            (dc_huff_lut.as_pointer());
                        let _ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>> =
                            (ac_huff_lut.as_pointer());
                        let _pos: Ptr<usize> = (pos.as_pointer());
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessDHT_236(_data, _len, _mode, _dc_huff_lut, _ac_huff_lut, _pos, _jpg)
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
                    if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
                        (*ok.borrow_mut()) = ({
                            let _data: Ptr<u8> = (*data.borrow()).clone();
                            let _len: usize = (*len.borrow());
                            let _dc_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>> =
                                dc_huff_lut.as_pointer();
                            let _ac_huff_lut: Ptr<Vec<brunsli_HuffmanTableEntry>> =
                                ac_huff_lut.as_pointer();
                            let _scan_progression: Ptr<Value<Box<[u16]>>> =
                                (scan_progression.as_pointer() as Ptr<Value<Box<[u16]>>>);
                            let _is_progressive: bool = (*is_progressive.borrow());
                            let _pos: Ptr<usize> = (pos.as_pointer());
                            let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                            ProcessScan_247(
                                _data,
                                _len,
                                _dc_huff_lut,
                                _ac_huff_lut,
                                _scan_progression,
                                _is_progressive,
                                _pos,
                                _jpg,
                            )
                        });
                    }
                    break 'switch;
                }
                __v if __v == 219 => {
                    (*ok.borrow_mut()) = ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _len: usize = (*len.borrow());
                        let _pos: Ptr<usize> = (pos.as_pointer());
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessDQT_238(_data, _len, _pos, _jpg)
                    });
                    break 'switch;
                }
                __v if __v == 221 => {
                    (*ok.borrow_mut()) = ({
                        let _data: Ptr<u8> = (*data.borrow()).clone();
                        let _len: usize = (*len.borrow());
                        let _pos: Ptr<usize> = (pos.as_pointer());
                        let _found_dri: Ptr<bool> = (found_dri.as_pointer());
                        let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                        ProcessDRI_239(_data, _len, _pos, _found_dri, _jpg)
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
                    if (((*mode.borrow()) as i32)
                        != (brunsli_JpegReadMode::JPEG_READ_TABLES as i32))
                    {
                        (*ok.borrow_mut()) = ({
                            let _data: Ptr<u8> = (*data.borrow()).clone();
                            let _len: usize = (*len.borrow());
                            let _pos: Ptr<usize> = (pos.as_pointer());
                            let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                            ProcessAPP_240(_data, _len, _pos, _jpg)
                        });
                    }
                    break 'switch;
                }
                __v if __v == 254 => {
                    if (((*mode.borrow()) as i32)
                        != (brunsli_JpegReadMode::JPEG_READ_TABLES as i32))
                    {
                        (*ok.borrow_mut()) = ({
                            let _data: Ptr<u8> = (*data.borrow()).clone();
                            let _len: usize = (*len.borrow());
                            let _pos: Ptr<usize> = (pos.as_pointer());
                            let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
                            ProcessCOM_241(_data, _len, _pos, _jpg)
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
                    (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                        brunsli_JPEGReadError::UNSUPPORTED_MARKER;
                    (*ok.borrow_mut()) = false;
                    break 'switch;
                }
            }
        };
        if !(*ok.borrow()) {
            return false;
        }
        (*(*(*jpg.borrow()).upgrade().deref())
            .marker_order
            .borrow_mut())
        .push(((*marker.borrow()) as u8));
        if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode::JPEG_READ_HEADER as i32))
            && (*found_sof.borrow())
        {
            break;
        }
    }
    if !(*found_sof.borrow()) {
        write!(libcc2rs::cerr(), "Missing SOF marker.\n",);
        (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
            brunsli_JPEGReadError::SOF_NOT_FOUND;
        return false;
    }
    if (((*mode.borrow()) as i32) == (brunsli_JpegReadMode::JPEG_READ_ALL as i32)) {
        if ((*pos.borrow()) < (*len.borrow())) {
            ((*(*jpg.borrow()).upgrade().deref()).tail_data.as_pointer() as Ptr<Vec<u8>>).write({
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
        if !({
            let _jpg: Ptr<brunsli_JPEGData> = (*jpg.borrow()).clone();
            FixupIndexes_248(_jpg)
        }) {
            return false;
        }
        if (*(*(*jpg.borrow()).upgrade().deref()).huffman_code.borrow()).is_empty() {
            write!(libcc2rs::cerr(), "Need at least one Huffman code table.\n",);
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::HUFFMAN_TABLE_ERROR;
            return false;
        }
        if {
            let _lhs = (*(*(*jpg.borrow()).upgrade().deref()).huffman_code.borrow()).len();
            _lhs >= ((*kMaxDHTMarkers_10.with(Value::clone).borrow()) as usize)
        } {
            write!(libcc2rs::cerr(), "Too many Huffman tables.\n",);
            (*(*(*jpg.borrow()).upgrade().deref()).error.borrow_mut()) =
                brunsli_JPEGReadError::HUFFMAN_TABLE_ERROR;
            return false;
        }
    }
    return true;
}
pub fn NextTableBitSize_251(count: Ptr<i32>, len: i32) -> i32 {
    let count: Value<Ptr<i32>> = Rc::new(RefCell::new(count));
    let len: Value<i32> = Rc::new(RefCell::new(len));
    let left: Value<i32> = Rc::new(RefCell::new(
        (1 << ((*len.borrow()) - (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()))),
    ));
    'loop_: while ((*len.borrow()) < (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())) {
        let __rhs = ((*count.borrow()).offset((*len.borrow()) as isize).read());
        (*left.borrow_mut()) -= __rhs;
        if ((*left.borrow()) <= 0) {
            break;
        }
        (*len.borrow_mut()).prefix_inc();
        (*left.borrow_mut()) <<= 1;
    }
    return ((*len.borrow()) - (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()));
}
pub fn BuildJpegHuffmanTable_237(
    count: Ptr<i32>,
    symbols: Ptr<i32>,
    lut: Ptr<brunsli_HuffmanTableEntry>,
) {
    let count: Value<Ptr<i32>> = Rc::new(RefCell::new(count));
    let symbols: Value<Ptr<i32>> = Rc::new(RefCell::new(symbols));
    let lut: Value<Ptr<brunsli_HuffmanTableEntry>> = Rc::new(RefCell::new(lut));
    let code: Value<brunsli_HuffmanTableEntry> = Rc::new(RefCell::new(
        brunsli_HuffmanTableEntry::brunsli_HuffmanTableEntry(),
    ));
    let table: Value<Ptr<brunsli_HuffmanTableEntry>> =
        Rc::new(RefCell::new(Ptr::<brunsli_HuffmanTableEntry>::null()));
    let len: Value<i32> = <Value<i32>>::default();
    let idx: Value<i32> = <Value<i32>>::default();
    let key: Value<i32> = <Value<i32>>::default();
    let reps: Value<i32> = <Value<i32>>::default();
    let low: Value<i32> = <Value<i32>>::default();
    let table_bits: Value<i32> = <Value<i32>>::default();
    let table_size: Value<i32> = <Value<i32>>::default();
    let tmp_count: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ])));
    let total_count: Value<i32> = Rc::new(RefCell::new(0));
    (*len.borrow_mut()) = 1;
    'loop_: while ((*len.borrow()) <= (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())) {
        let __rhs = ((*count.borrow()).offset((*len.borrow()) as isize).read());
        (*tmp_count.borrow_mut())[(*len.borrow()) as usize] = __rhs;
        (*total_count.borrow_mut()) += (*tmp_count.borrow())[(*len.borrow()) as usize];
        (*len.borrow_mut()).prefix_inc();
    }
    (*table.borrow_mut()) = (*lut.borrow()).clone();
    (*table_bits.borrow_mut()) = (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow());
    (*table_size.borrow_mut()) = (1 << (*table_bits.borrow()));
    if ((*total_count.borrow()) == 1) {
        (*(*code.borrow()).bits.borrow_mut()) = 0_u8;
        (*(*code.borrow()).value.borrow_mut()) =
            (((*symbols.borrow()).offset((0) as isize).read()) as u16);
        (*key.borrow_mut()) = 0;
        'loop_: while ((*key.borrow()) < (*table_size.borrow())) {
            let __rhs = (*code.borrow()).clone();
            (*table.borrow())
                .offset((*key.borrow()) as isize)
                .write(__rhs);
            (*key.borrow_mut()).prefix_inc();
        }
        return;
    }
    (*key.borrow_mut()) = 0;
    (*idx.borrow_mut()) = 0;
    (*len.borrow_mut()) = 1;
    'loop_: while ((*len.borrow()) <= (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()))
    {
        'loop_: while ((*tmp_count.borrow())[(*len.borrow()) as usize] > 0) {
            (*(*code.borrow()).bits.borrow_mut()) = ((*len.borrow()) as u8);
            (*(*code.borrow()).value.borrow_mut()) = (((*symbols.borrow())
                .offset(((*idx.borrow_mut()).postfix_inc()) as isize)
                .read()) as u16);
            (*reps.borrow_mut()) = (1
                << ((*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow())
                    - (*len.borrow())));
            'loop_: while ((*reps.borrow_mut()).postfix_dec() != 0) {
                let __rhs = (*code.borrow()).clone();
                (*table.borrow())
                    .offset(((*key.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            }
            (*tmp_count.borrow_mut())[(*len.borrow()) as usize].prefix_dec();
        }
        (*len.borrow_mut()).prefix_inc();
    }
    (*table.borrow_mut()) += (*table_size.borrow());
    (*table_size.borrow_mut()) = 0;
    (*low.borrow_mut()) = 0;
    (*len.borrow_mut()) = ((*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()) + 1);
    'loop_: while ((*len.borrow()) <= (*kJpegHuffmanMaxBitLength_7.with(Value::clone).borrow())) {
        'loop_: while ((*tmp_count.borrow())[(*len.borrow()) as usize] > 0) {
            if ((*low.borrow()) >= (*table_size.borrow())) {
                (*table.borrow_mut()) += (*table_size.borrow());
                (*table_bits.borrow_mut()) = ({
                    let _count: Ptr<i32> = (tmp_count.as_pointer() as Ptr<i32>);
                    let _len: i32 = (*len.borrow());
                    NextTableBitSize_251(_count, _len)
                });
                (*table_size.borrow_mut()) = (1 << (*table_bits.borrow()));
                (*low.borrow_mut()) = 0;
                (*(*(*lut.borrow())
                    .offset((*key.borrow()) as isize)
                    .upgrade()
                    .deref())
                .bits
                .borrow_mut()) = (((*table_bits.borrow())
                    + (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()))
                    as u8);
                let __rhs = (({
                    let _lhs = (((*table.borrow()).clone() - (*lut.borrow()).clone()) as i64);
                    _lhs - ((*key.borrow()) as i64)
                }) as u16);
                (*(*(*lut.borrow())
                    .offset((*key.borrow()) as isize)
                    .upgrade()
                    .deref())
                .value
                .borrow_mut()) = __rhs;
                (*key.borrow_mut()).prefix_inc();
            }
            (*(*code.borrow()).bits.borrow_mut()) = (((*len.borrow())
                - (*kJpegHuffmanRootTableBits_229.with(Value::clone).borrow()))
                as u8);
            (*(*code.borrow()).value.borrow_mut()) = (((*symbols.borrow())
                .offset(((*idx.borrow_mut()).postfix_inc()) as isize)
                .read()) as u16);
            (*reps.borrow_mut()) =
                (1 << ((*table_bits.borrow()) - ((*(*code.borrow()).bits.borrow()) as i32)));
            'loop_: while ((*reps.borrow_mut()).postfix_dec() != 0) {
                let __rhs = (*code.borrow()).clone();
                (*table.borrow())
                    .offset(((*low.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            }
            (*tmp_count.borrow_mut())[(*len.borrow()) as usize].prefix_dec();
        }
        (*len.borrow_mut()).prefix_inc();
    }
}
#[derive(Default)]
pub struct brunsli_Storage {
    pub data: Value<Ptr<u8>>,
    pub length: Value<usize>,
    pub pos: Value<usize>,
}
impl brunsli_Storage {
    pub fn brunsli_Storage(data: Ptr<u8>, length: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let mut this = Self {
            data: Rc::new(RefCell::new((*data.borrow()).clone())),
            length: Rc::new(RefCell::new((*length.borrow()))),
            pos: Rc::new(RefCell::new(0_usize)),
        };
        if !((*length.borrow()) > 0_usize) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"Storage");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"write_bits.cc"), 14, _fn)
            });
            'loop_: while true {}
        };
        (*data.borrow()).offset((0) as isize).write(0_u8);
        this
    }
    pub fn GetBytesUsed(&self) -> usize {
        return (((*self.pos.borrow()).wrapping_add(7_usize)) >> 3);
    }
}
impl Clone for brunsli_Storage {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            length: Rc::new(RefCell::new((*self.length.borrow()))),
            pos: Rc::new(RefCell::new((*self.pos.borrow()))),
        };
        this
    }
}
impl Drop for brunsli_Storage {
    fn drop(&mut self) {
        if !(({ self.GetBytesUsed() }) <= (*self.length.borrow())) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"~Storage");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"write_bits.cc"), 26, _fn)
            });
            'loop_: while true {}
        };
    }
}
impl ByteRepr for brunsli_Storage {}
impl brunsli_Storage {}
impl brunsli_Storage {
    pub fn AppendBytes(&self, src: Ptr<u8>, len: usize) {
        let src: Value<Ptr<u8>> = Rc::new(RefCell::new(src));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        if !(((*self.pos.borrow()) & 7_usize) == 0_usize) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"AppendBytes");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"write_bits.cc"), 19, _fn)
            });
            'loop_: while true {}
        };
        if !(({ self.GetBytesUsed() }).wrapping_add((*len.borrow())) <= (*self.length.borrow())) {
            ({
                let _fn: Ptr<u8> = Ptr::from_string_literal(b"AppendBytes");
                BrunsliDumpAndAbort_79(Ptr::from_string_literal(b"write_bits.cc"), 20, _fn)
            });
            'loop_: while true {}
        };
        {
            ((*self.data.borrow()).offset(((*self.pos.borrow()) >> 3) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &((*src.borrow()).clone() as Ptr<u8>).to_any(),
                    (*len.borrow()) as usize,
                );
            ((*self.data.borrow()).offset(((*self.pos.borrow()) >> 3) as isize) as Ptr<u8>)
                .to_any()
                .clone()
        };
        let rhs_0 = (*self.pos.borrow()).wrapping_add((8_usize).wrapping_mul((*len.borrow())));
        (*self.pos.borrow_mut()) = rhs_0;
    }
}
pub fn ReadFileInternal_252(file: Ptr<::std::fs::File>, content: Ptr<Vec<u8>>) -> bool {
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
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| {
            __v.resize(((*input_size.borrow()) as usize) as usize, 0)
        });
        (*content.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(0))
    };
    let read_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*read_pos.borrow());
        _lhs < ((*(*content.borrow()).upgrade().deref()).len() - 1)
    } {
        let bytes_read: Value<usize> = Rc::new(RefCell::new({
            let __a0 = ((if (*read_pos.borrow()) as usize
                >= (*((*content.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
                    .upgrade()
                    .deref())
                .len()
                .saturating_sub(1)
            {
                panic!("out of bounds access")
            } else {
                (((*content.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
                    .to_strong()
                    .as_pointer() as Ptr<u8>)
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
        let rhs_0 = (*read_pos.borrow()).wrapping_add((*bytes_read.borrow()));
        (*read_pos.borrow_mut()) = rhs_0;
    }
    return true;
}
pub fn ReadFile_253(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let content: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(content));
    let file: Value<Ptr<::std::fs::File>> =
        Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"rb").to_rust_string() {
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
            ReadFileInternal_252(_file, _content)
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
pub fn WriteFileInternal_254(file: Ptr<::std::fs::File>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(file));
    let write_pos: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*write_pos.borrow());
        _lhs < ((*content.upgrade().deref()).len() - 1)
    } {
        let bytes_written: Value<usize> = Rc::new(RefCell::new({
            let __a0 = (((content.to_strong().as_pointer() as Ptr<u8>)
                .offset((*write_pos.borrow()) as isize)) as Ptr<u8>)
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
        let rhs_0 = (*write_pos.borrow()).wrapping_add((*bytes_written.borrow()));
        (*write_pos.borrow_mut()) = rhs_0;
    }
    return true;
}
pub fn WriteFile_255(file_name: Ptr<Vec<u8>>, content: Ptr<Vec<u8>>) -> bool {
    let file: Value<Ptr<::std::fs::File>> =
        Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"wb").to_rust_string() {
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
            WriteFileInternal_254(_file, _content)
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
pub fn ProcessFile_256(file_name: Ptr<Vec<u8>>, outfile_name: Ptr<Vec<u8>>) -> bool {
    let input: Value<Vec<u8>> = Rc::new(RefCell::new(vec![0]));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            let _file_name: Ptr<Vec<u8>> = (file_name).clone();
            let _content: Ptr<Vec<u8>> = (input.as_pointer());
            ReadFile_253(_file_name, _content)
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
    (*ok.borrow_mut()) = ({
        let _data: Ptr<u8> = (*input_data.borrow()).clone();
        let _len: usize = ((*input.borrow()).len() - 1);
        let _mode: brunsli_JpegReadMode = brunsli_JpegReadMode::JPEG_READ_ALL;
        let _jpg: Ptr<brunsli_JPEGData> = (jpg.as_pointer());
        ReadJpeg_195(_data, _len, _mode, _jpg)
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
        ({
            let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
            GetMaximumBrunsliEncodedSize_144(_jpg)
        }),
    ));
    {
        (*output.borrow_mut()).pop();
        (*output.borrow_mut()).resize((*output_size.borrow()) as usize, 0);
        (*output.borrow_mut()).push(0)
    };
    let output_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((output.as_pointer() as Ptr<u8>).offset(0_usize as isize)).reinterpret_cast::<u8>(),
    ));
    (*ok.borrow_mut()) = ({
        let _jpg: Ptr<brunsli_JPEGData> = jpg.as_pointer();
        let _data: Ptr<u8> = (*output_data.borrow()).clone();
        let _len: Ptr<usize> = (output_size.as_pointer());
        BrunsliEncodeJpeg_190(_jpg, _data, _len)
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
    (*ok.borrow_mut()) = ({
        let _file_name: Ptr<Vec<u8>> = (outfile_name).clone();
        let _content: Ptr<Vec<u8>> = output.as_pointer();
        WriteFile_255(_file_name, _content)
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
        eprintln!("Usage: cbrunsli FILE [OUTPUT_FILE, default=FILE.brn]");
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
            r.extend(Ptr::from_string_literal(b".brn").to_c_string_iterator());
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
            ProcessFile_256(_file_name, _outfile_name)
        }),
    ));
    return if (*ok.borrow()) { 0 } else { 1 };
}
