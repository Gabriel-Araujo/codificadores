pub mod binary_tree;
pub mod buffer;
pub mod io;
pub mod token;

pub const WEIGHTS: [(char, f64); 27] = [
    (' ', 0.1438),
    ('e', 0.1238),
    ('a', 0.1161),
    ('o', 0.0908),
    ('s', 0.0661),
    ('r', 0.0552),
    ('i', 0.0523),
    ('n', 0.0427),
    ('d', 0.0422),
    ('m', 0.0401),
    ('u', 0.0392),
    ('t', 0.0367),
    ('c', 0.0328),
    ('l', 0.0235),
    ('p', 0.0213),
    ('v', 0.0141),
    ('g', 0.0110),
    ('h', 0.0108),
    ('q', 0.0102),
    ('b', 0.0088),
    ('f', 0.0086),
    ('z', 0.0040),
    ('j', 0.0034),
    ('x', 0.0023),
    ('k', 0.0002),
    ('w', 0.0001),
    ('y', 0.0001),
];

pub const TEST_WEIGHTS: [(char, f64); 6] = [
    ('A', 0.25),
    ('B', 0.25),
    ('C', 0.20),
    ('D', 0.15),
    ('E', 0.10),
    ('F', 0.05),
];
