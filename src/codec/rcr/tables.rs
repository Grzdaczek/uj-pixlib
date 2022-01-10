type Table = [i8; 64];

pub fn from_quality(quality: usize) -> (Table, Table) {
    match quality {
        0 => (LUMA_Q0, CHROMA_Q0),
        1 => (LUMA_Q1, CHROMA_Q1),
        2 => (LUMA_Q2, CHROMA_Q2),
        3 => (LUMA_Q3, CHROMA_Q3),
        4 => (LUMA_Q4, CHROMA_Q4),
        5 => (LUMA_Q5, CHROMA_Q5),
        6 => (LUMA_Q6, CHROMA_Q6),
        7 => (LUMA_Q7, CHROMA_Q7),

        // TODO: Not working properly due to clipping
        // 8 => (LUMA_Q8, CHROMA_Q8),
        // 9 => (LUMA_Q9, CHROMA_Q9),
        // 10 => (LUMA_Q10, CHROMA_Q10),
        // 11 => (LUMA_Q11, CHROMA_Q11),
        // 12 => (LUMA_Q12, CHROMA_Q12),
        _ => panic!("Table quality out of range")
    }
}

pub const LUMA_Q0: Table = [
    32, 33, 51, 81, 66, 39, 34, 17,
    33, 36, 48, 47, 28, 23, 12, 12,
    51, 48, 47, 28, 23, 12, 12, 12,
    81, 47, 28, 23, 12, 12, 12, 12,
    66, 28, 23, 12, 12, 12, 12, 12,
    39, 23, 12, 12, 12, 12, 12, 12,
    34, 12, 12, 12, 12, 12, 12, 12,
    17, 12, 12, 12, 12, 12, 12, 12,
];

pub const CHROMA_Q0: Table = [
    34, 51, 52, 34, 20, 20, 17, 17,
    51, 38, 24, 14, 14, 12, 12, 12,
    52, 24, 14, 14, 12, 12, 12, 12,
    34, 14, 14, 12, 12, 12, 12, 12,
    20, 14, 12, 12, 12, 12, 12, 12,
    20, 12, 12, 12, 12, 12, 12, 12,
    17, 12, 12, 12, 12, 12, 12, 12,
    17, 12, 12, 12, 12, 12, 12, 12,
];

pub const LUMA_Q1: Table = [
    27, 26, 41, 65, 66, 39, 34, 17, 
    26, 29, 38, 47, 28, 23, 12, 12, 
    41, 38, 47, 28, 23, 12, 12, 12, 
    65, 47, 28, 23, 12, 12, 12, 12, 
    66, 28, 23, 12, 12, 12, 12, 12, 
    39, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q1: Table = [
    29, 41, 52, 34, 20, 20, 17, 17, 
    41, 38, 24, 14, 14, 12, 12, 12, 
    52, 24, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q2: Table = [
    20, 17, 26, 41, 51, 39, 34, 17, 
    17, 18, 24, 39, 28, 23, 12, 12, 
    26, 24, 32, 28, 23, 12, 12, 12, 
    41, 39, 28, 23, 12, 12, 12, 12, 
    51, 28, 23, 12, 12, 12, 12, 12, 
    39, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q2: Table = [
    21, 26, 33, 34, 20, 20, 17, 17, 
    26, 29, 24, 14, 14, 12, 12, 12, 
    33, 24, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q3: Table = [
    18, 14, 22, 35, 44, 39, 34, 17, 
    14, 16, 21, 34, 28, 23, 12, 12, 
    22, 21, 27, 28, 23, 12, 12, 12, 
    35, 34, 28, 23, 12, 12, 12, 12, 
    44, 28, 23, 12, 12, 12, 12, 12, 
    39, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q3: Table = [
    20, 22, 29, 34, 20, 20, 17, 17, 
    22, 25, 24, 14, 14, 12, 12, 12, 
    29, 24, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q4: Table = [
    16, 11, 17, 27, 34, 39, 34, 17, 
    11, 12, 16, 26, 28, 23, 12, 12, 
    17, 16, 21, 28, 23, 12, 12, 12, 
    27, 26, 28, 23, 12, 12, 12, 12, 
    34, 28, 23, 12, 12, 12, 12, 12, 
    39, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q4: Table = [
    17, 17, 22, 34, 20, 20, 17, 17, 
    17, 19, 22, 14, 14, 12, 12, 12, 
    22, 22, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q5: Table = [
    12, 8,  13, 21, 26, 32, 34, 17, 
    8,  9,  12, 20, 27, 23, 12, 12, 
    13, 12, 16, 26, 23, 12, 12, 12, 
    21, 20, 26, 23, 12, 12, 12, 12, 
    26, 27, 23, 12, 12, 12, 12, 12, 
    32, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q5: Table = [
    13, 13, 17, 27, 20, 20, 17, 17, 
    13, 14, 17, 14, 14, 12, 12, 12, 
    17, 17, 14, 14, 12, 12, 12, 12, 
    27, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q6: Table = [
    8,  6,  9,  14, 17, 21, 28, 17, 
    6,  6,  8,  13, 18, 23, 12, 12, 
    9,  8,  11, 17, 23, 12, 12, 12, 
    14, 13, 17, 23, 12, 12, 12, 12, 
    17, 18, 23, 12, 12, 12, 12, 12, 
    21, 23, 12, 12, 12, 12, 12, 12, 
    28, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q6: Table = [
    9,  9,  11, 18, 20, 20, 17, 17, 
    9,  10, 11, 14, 14, 12, 12, 12, 
    11, 11, 14, 14, 12, 12, 12, 12, 
    18, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q7: Table = [
    10, 7,  11, 18, 22, 27, 34, 17, 
    7,  8,  10, 17, 23, 23, 12, 12, 
    11, 10, 14, 22, 23, 12, 12, 12, 
    18, 17, 22, 23, 12, 12, 12, 12, 
    22, 23, 23, 12, 12, 12, 12, 12, 
    27, 23, 12, 12, 12, 12, 12, 12, 
    34, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q7: Table = [
    11, 14, 31, 34, 20, 20, 17, 17, 
    14, 19, 24, 14, 14, 12, 12, 12, 
    31, 24, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q8: Table = [
    6,  4,  7,  11, 14, 17, 22, 17, 
    4,  5,  6,  10, 14, 19, 12, 12, 
    7,  6,  8,  14, 19, 12, 12, 12, 
    11, 10, 14, 19, 12, 12, 12, 12, 
    14, 14, 19, 12, 12, 12, 12, 12, 
    17, 19, 12, 12, 12, 12, 12, 12, 
    22, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q8: Table = [
    7,  9,  19, 34, 20, 20, 17, 17, 
    9,  12, 19, 14, 14, 12, 12, 12, 
    19, 19, 14, 14, 12, 12, 12, 12, 
    34, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q9: Table = [
    4,  3,  4,  7,  9,  11, 14, 17, 
    3,  3,  4,  7,  9,  12, 12, 12, 
    4,  4,  5,  9,  12, 12, 12, 12, 
    7,  7,  9,  12, 12, 12, 12, 12, 
    9,  9,  12, 12, 12, 12, 12, 12, 
    11, 12, 12, 12, 12, 12, 12, 12, 
    14, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q9: Table = [
    4,  6,  12, 22, 20, 20, 17, 17, 
    6,  8,  12, 14, 14, 12, 12, 12, 
    12, 12, 14, 14, 12, 12, 12, 12, 
    22, 14, 14, 12, 12, 12, 12, 12, 
    20, 14, 12, 12, 12, 12, 12, 12, 
    20, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
    17, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q10: Table = [
    2,  2,  3,  4,  5,  6,  8,  11, 
    2,  2,  2,  4,  5,  7,  9,  11, 
    3,  2,  3,  5,  7,  9,  11, 12, 
    4,  4,  5,  7,  9,  11, 12, 12, 
    5,  5,  7,  9,  11, 12, 12, 12, 
    6,  7,  9,  11, 12, 12, 12, 12, 
    8,  9,  11, 12, 12, 12, 12, 12, 
    11, 11, 12, 12, 12, 12, 12, 12, 
];

pub const CHROMA_Q10: Table = [
    3,  3,  7,  13, 15, 15, 15, 15, 
    3,  4,  7,  13, 14, 12, 12, 12, 
    7,  7,  13, 14, 12, 12, 12, 12, 
    13, 13, 14, 12, 12, 12, 12, 12, 
    15, 14, 12, 12, 12, 12, 12, 12, 
    15, 12, 12, 12, 12, 12, 12, 12, 
    15, 12, 12, 12, 12, 12, 12, 12, 
    15, 12, 12, 12, 12, 12, 12, 12, 
];

pub const LUMA_Q11: Table = [
    1, 1, 1, 2, 3, 3, 4, 5, 
    1, 1, 1, 2, 3, 4, 4, 6, 
    1, 1, 2, 3, 4, 4, 5, 7, 
    2, 2, 3, 4, 4, 5, 7, 8, 
    3, 3, 4, 4, 5, 7, 8, 8, 
    3, 4, 4, 5, 7, 8, 8, 8, 
    4, 4, 5, 7, 8, 8, 8, 8, 
    5, 6, 7, 8, 8, 8, 8, 8, 
];

pub const CHROMA_Q11: Table = [
    1, 2, 4, 7, 8, 8, 8, 8, 
    2, 2, 4, 7, 8, 8, 8, 8, 
    4, 4, 7, 8, 8, 8, 8, 8, 
    7, 7, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 
];

pub const LUMA_Q12: Table = [
    1, 1, 1, 1, 1, 1, 1, 2, 
    1, 1, 1, 1, 1, 1, 1, 2, 
    1, 1, 1, 1, 1, 1, 2, 2, 
    1, 1, 1, 1, 1, 2, 2, 3, 
    1, 1, 1, 1, 2, 2, 3, 3, 
    1, 1, 1, 2, 2, 3, 3, 3, 
    1, 1, 2, 2, 3, 3, 3, 3, 
    2, 2, 2, 3, 3, 3, 3, 3, 
];

pub const CHROMA_Q12: Table = [
    1, 1, 1, 2, 3, 3, 3, 3, 
    1, 1, 1, 2, 3, 3, 3, 3, 
    1, 1, 2, 3, 3, 3, 3, 3, 
    2, 2, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3,
];