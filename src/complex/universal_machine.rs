// This file is autogenerated. Do not edit it!
//
// See docs/ragel.md for details.

#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use crate::buffer::Buffer;
use crate::complex::machine_cursor::MachineCursor;
use crate::complex::universal::category;
use crate::GlyphInfo;
use core::cell::Cell;

static _use_syllable_machine_trans_keys: [u8; 126] = [
    1, 1, 1, 1, 0, 36, 26, 27, 27, 27, 5, 33, 5, 33, 1, 1, 9, 33, 10, 33, 11, 32, 12, 32, 13, 32,
    30, 31, 31, 31, 11, 33, 11, 33, 11, 33, 1, 1, 11, 33, 10, 33, 10, 33, 10, 33, 9, 33, 9, 33, 9,
    33, 5, 33, 1, 33, 7, 7, 3, 3, 5, 33, 5, 33, 9, 33, 10, 33, 11, 32, 12, 32, 13, 32, 30, 31, 31,
    31, 11, 33, 11, 33, 11, 33, 11, 33, 10, 33, 10, 33, 10, 33, 9, 33, 9, 33, 9, 33, 5, 33, 1, 33,
    1, 1, 3, 3, 7, 7, 1, 33, 5, 33, 26, 27, 27, 27, 1, 4, 35, 37, 34, 37, 34, 36, 0, 0,
];
static _use_syllable_machine_char_class: [i8; 55] = [
    0, 1, 2, 2, 3, 4, 2, 2, 2, 2, 2, 5, 6, 7, 2, 2, 2, 2, 8, 2, 2, 2, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 2, 23, 24, 25, 2, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
    0, 0,
];
static _use_syllable_machine_index_offsets: [i16; 64] = [
    0, 1, 2, 39, 41, 42, 71, 100, 101, 126, 150, 172, 193, 213, 215, 216, 239, 262, 285, 286, 309,
    333, 357, 381, 406, 431, 456, 485, 518, 519, 520, 549, 578, 603, 627, 649, 670, 690, 692, 693,
    716, 739, 762, 785, 809, 833, 857, 882, 907, 932, 961, 994, 995, 996, 997, 1030, 1059, 1061,
    1062, 1066, 1069, 1073, 0, 0,
];
static _use_syllable_machine_indices: [i8; 1078] = [
    1, 2, 4, 5, 6, 7, 8, 1, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 12, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 9, 35, 6, 36, 4, 38, 38, 40, 41, 39, 39, 42, 43, 44, 45,
    46, 47, 48, 42, 49, 5, 50, 51, 52, 53, 54, 55, 56, 39, 39, 39, 57, 58, 59, 60, 41, 40, 41, 39,
    39, 42, 43, 44, 45, 46, 47, 48, 42, 49, 50, 50, 51, 52, 53, 54, 55, 56, 39, 39, 39, 57, 58, 59,
    60, 41, 40, 42, 43, 44, 45, 46, 39, 39, 39, 39, 39, 39, 51, 52, 53, 54, 55, 56, 39, 39, 39, 43,
    58, 59, 60, 62, 43, 44, 45, 46, 39, 39, 39, 39, 39, 39, 39, 39, 39, 54, 55, 56, 39, 39, 39, 39,
    58, 59, 60, 62, 44, 45, 46, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 58,
    59, 60, 45, 46, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 58, 59, 60, 46,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 58, 59, 60, 58, 59, 59, 44, 45,
    46, 39, 39, 39, 39, 39, 39, 39, 39, 39, 54, 55, 56, 39, 39, 39, 39, 58, 59, 60, 62, 44, 45, 46,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 55, 56, 39, 39, 39, 39, 58, 59, 60, 62, 44, 45, 46, 39,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 56, 39, 39, 39, 39, 58, 59, 60, 62, 64, 44, 45, 46, 39,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 58, 59, 60, 62, 43, 44, 45, 46, 39,
    39, 39, 39, 39, 39, 51, 52, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 43, 44, 45, 46, 39,
    39, 39, 39, 39, 39, 39, 52, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 43, 44, 45, 46, 39,
    39, 39, 39, 39, 39, 39, 39, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 42, 43, 44, 45, 46,
    39, 48, 42, 39, 39, 39, 51, 52, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 42, 43, 44, 45,
    46, 39, 39, 42, 39, 39, 39, 51, 52, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 42, 43, 44,
    45, 46, 47, 48, 42, 39, 39, 39, 51, 52, 53, 54, 55, 56, 39, 39, 39, 43, 58, 59, 60, 62, 40, 41,
    39, 39, 42, 43, 44, 45, 46, 47, 48, 42, 49, 39, 50, 51, 52, 53, 54, 55, 56, 39, 39, 39, 57, 58,
    59, 60, 41, 40, 61, 61, 61, 61, 61, 61, 61, 61, 43, 44, 45, 46, 61, 61, 61, 61, 61, 61, 61, 61,
    61, 54, 55, 56, 61, 61, 61, 61, 58, 59, 60, 62, 66, 7, 40, 41, 39, 39, 42, 43, 44, 45, 46, 47,
    48, 42, 49, 5, 50, 51, 52, 53, 54, 55, 56, 4, 38, 39, 57, 58, 59, 60, 41, 1, 69, 68, 68, 12,
    13, 14, 15, 16, 17, 18, 12, 19, 21, 21, 22, 23, 24, 25, 26, 27, 68, 68, 68, 31, 32, 33, 34, 69,
    12, 13, 14, 15, 16, 68, 68, 68, 68, 68, 68, 22, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34,
    70, 13, 14, 15, 16, 68, 68, 68, 68, 68, 68, 68, 68, 68, 25, 26, 27, 68, 68, 68, 68, 32, 33, 34,
    70, 14, 15, 16, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 32, 33, 34, 15,
    16, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 32, 33, 34, 16, 68, 68, 68,
    68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 32, 33, 34, 32, 33, 33, 14, 15, 16, 68, 68,
    68, 68, 68, 68, 68, 68, 68, 25, 26, 27, 68, 68, 68, 68, 32, 33, 34, 70, 14, 15, 16, 68, 68, 68,
    68, 68, 68, 68, 68, 68, 68, 26, 27, 68, 68, 68, 68, 32, 33, 34, 70, 14, 15, 16, 68, 68, 68, 68,
    68, 68, 68, 68, 68, 68, 68, 27, 68, 68, 68, 68, 32, 33, 34, 70, 14, 15, 16, 68, 68, 68, 68, 68,
    68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 32, 33, 34, 70, 13, 14, 15, 16, 68, 68, 68, 68, 68,
    68, 22, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 13, 14, 15, 16, 68, 68, 68, 68, 68,
    68, 68, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 13, 14, 15, 16, 68, 68, 68, 68, 68,
    68, 68, 68, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 12, 13, 14, 15, 16, 68, 18, 12, 68,
    68, 68, 22, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 12, 13, 14, 15, 16, 68, 68, 12,
    68, 68, 68, 22, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 12, 13, 14, 15, 16, 17, 18,
    12, 68, 68, 68, 22, 23, 24, 25, 26, 27, 68, 68, 68, 13, 32, 33, 34, 70, 1, 69, 68, 68, 12, 13,
    14, 15, 16, 17, 18, 12, 19, 68, 21, 22, 23, 24, 25, 26, 27, 68, 68, 68, 31, 32, 33, 34, 69, 1,
    68, 68, 68, 68, 68, 68, 68, 68, 13, 14, 15, 16, 68, 68, 68, 68, 68, 68, 68, 68, 68, 25, 26, 27,
    68, 68, 68, 68, 32, 33, 34, 70, 1, 72, 10, 5, 68, 68, 5, 1, 69, 10, 68, 12, 13, 14, 15, 16, 17,
    18, 12, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 68, 31, 32, 33, 34, 69, 1, 69, 68, 68, 12,
    13, 14, 15, 16, 17, 18, 12, 19, 20, 21, 22, 23, 24, 25, 26, 27, 68, 68, 68, 31, 32, 33, 34, 69,
    28, 29, 29, 5, 71, 71, 5, 74, 73, 35, 35, 74, 73, 74, 35, 73, 36, 0, 0,
];
static _use_syllable_machine_index_defaults: [i8; 64] = [
    0, 0, 6, 37, 37, 39, 39, 61, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 63, 39, 39, 39, 39, 39,
    39, 39, 39, 61, 65, 67, 39, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68,
    68, 68, 68, 71, 68, 68, 68, 68, 68, 68, 71, 73, 73, 73, 0, 0,
];
static _use_syllable_machine_cond_targs: [i8; 77] = [
    2, 31, 42, 2, 3, 5, 2, 28, 30, 51, 52, 54, 32, 33, 34, 35, 36, 46, 47, 48, 55, 49, 43, 44, 45,
    39, 40, 41, 56, 57, 58, 50, 37, 38, 2, 59, 61, 2, 4, 2, 6, 7, 8, 9, 10, 11, 12, 23, 24, 25, 26,
    20, 21, 22, 15, 16, 17, 27, 13, 14, 2, 2, 18, 2, 19, 2, 29, 2, 2, 0, 1, 2, 53, 2, 60, 0, 0,
];
static _use_syllable_machine_cond_actions: [i8; 77] = [
    1, 2, 2, 0, 0, 0, 5, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2,
    0, 0, 6, 0, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0,
    11, 0, 12, 0, 13, 14, 0, 0, 15, 0, 16, 0, 0, 0,
];
static _use_syllable_machine_to_state_actions: [i8; 64] = [
    0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _use_syllable_machine_from_state_actions: [i8; 64] = [
    0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _use_syllable_machine_eof_trans: [i8; 64] = [
    1, 1, 4, 38, 38, 40, 40, 62, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 64, 40, 40, 40, 40, 40,
    40, 40, 40, 62, 66, 68, 40, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 72, 69, 69, 69, 69, 69, 69, 72, 74, 74, 74, 0, 0,
];
static use_syllable_machine_start: i32 = 2;
static use_syllable_machine_first_final: i32 = 2;
static use_syllable_machine_error: i32 = -1;
static use_syllable_machine_en_main: i32 = 2;
#[derive(Clone, Copy)]
pub enum SyllableType {
    IndependentCluster,
    ViramaTerminatedCluster,
    SakotTerminatedCluster,
    StandardCluster,
    NumberJoinerTerminatedCluster,
    NumeralCluster,
    SymbolCluster,
    HieroglyphCluster,
    BrokenCluster,
    NonCluster,
}

pub fn find_syllables(buffer: &mut Buffer) {
    let mut cs = 0;
    let infos = Cell::as_slice_of_cells(Cell::from_mut(&mut buffer.info));
    let p0 = MachineCursor::new(infos, included);
    let mut p = p0;
    let mut ts = p0;
    let mut te = p0;
    let pe = p.end();
    let eof = p.end();
    let mut syllable_serial = 1u8;

    // Please manually replace assignments of 0 to p, ts, and te
    // to use p0 instead

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts.index(), te.index(), &mut syllable_serial, $kind, infos);
        }};
    }

    {
        cs = (use_syllable_machine_start) as i32;
        ts = p0;
        te = p0;
    }

    {
        let mut _trans = 0;
        let mut _keys: i32 = 0;
        let mut _inds: i32 = 0;
        let mut _ic = 0;
        '_resume: while (p != pe || p == eof) {
            '_again: while (true) {
                match (_use_syllable_machine_from_state_actions[(cs) as usize]) {
                    4 => {
                        ts = p;
                    }

                    _ => {}
                }
                if (p == eof) {
                    {
                        if (_use_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_use_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (cs << 1) as i32;
                        _inds = (_use_syllable_machine_index_offsets[(cs) as usize]) as i32;
                        if ((infos[p.index()].get().use_category() as u8) <= 52) {
                            {
                                _ic = (_use_syllable_machine_char_class[((infos[p.index()]
                                    .get()
                                    .use_category()
                                    as u8)
                                    as i32
                                    - 0)
                                    as usize]) as i32;
                                if (_ic
                                    <= (_use_syllable_machine_trans_keys[(_keys + 1) as usize])
                                        as i32
                                    && _ic
                                        >= (_use_syllable_machine_trans_keys[(_keys) as usize])
                                            as i32)
                                {
                                    _trans = (_use_syllable_machine_indices[(_inds
                                        + (_ic
                                            - (_use_syllable_machine_trans_keys[(_keys) as usize])
                                                as i32)
                                            as i32)
                                        as usize])
                                        as u32;
                                } else {
                                    _trans = (_use_syllable_machine_index_defaults[(cs) as usize])
                                        as u32;
                                }
                            }
                        } else {
                            {
                                _trans =
                                    (_use_syllable_machine_index_defaults[(cs) as usize]) as u32;
                            }
                        }
                    }
                }
                cs = (_use_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_use_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        match (_use_syllable_machine_cond_actions[(_trans) as usize]) {
                            2 => {
                                te = p + 1;
                            }
                            9 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            6 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                }
                            }
                            5 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }
                            10 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::ViramaTerminatedCluster);
                                }
                            }
                            11 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SakotTerminatedCluster);
                                }
                            }
                            8 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            13 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumberJoinerTerminatedCluster);
                                }
                            }
                            12 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumeralCluster);
                                }
                            }
                            7 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }
                            16 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::HieroglyphCluster);
                                }
                            }
                            14 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                }
                            }
                            15 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }
                            1 => {
                                p = (te) - 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                }
                            }

                            _ => {}
                        }
                    }
                }
                break '_again;
            }
            if (p == eof) {
                {
                    if (cs >= 2) {
                        break '_resume;
                    }
                }
            } else {
                {
                    match (_use_syllable_machine_to_state_actions[(cs) as usize]) {
                        3 => {
                            ts = p0;
                        }

                        _ => {}
                    }
                    p += 1;
                    continue '_resume;
                }
            }
            break '_resume;
        }
    }
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &[Cell<GlyphInfo>],
) {
    for i in start..end {
        let mut glyph = buffer[i].get();
        glyph.set_syllable((*syllable_serial << 4) | kind as u8);
        buffer[i].set(glyph);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}

fn not_ccs_default_ignorable(i: &GlyphInfo) -> bool {
    !(matches!(i.use_category(), category::CGJ | category::RSV) && i.is_default_ignorable())
}

fn included(infos: &[Cell<GlyphInfo>], i: usize) -> bool {
    let glyph = infos[i].get();
    if !not_ccs_default_ignorable(&glyph) {
        return false;
    }
    if glyph.use_category() == category::ZWNJ {
        for glyph2 in &infos[i + 1..] {
            if not_ccs_default_ignorable(&glyph2.get()) {
                return !glyph2.get().is_unicode_mark();
            }
        }
    }
    true
}
