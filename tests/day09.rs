extern crate adventofcode2016;

use adventofcode2016::day09::{decompress, decompress_recursive};

#[test]
fn decompression_test_1() {
    assert_eq!("ADVENT", decompress("ADVENT"));
}

#[test]
fn decompression_test_2() {
    assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
}

#[test]
fn decompression_test_3() {
    assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
}

#[test]
fn decompression_test_4() {
    assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
}

#[test]
fn decompression_test_5() {
    assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
}

#[test]
fn decompression_test_6() {
    assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
}



#[test]
fn recursive_decompression_test_1() {
    assert_eq!(9, decompress_recursive("(3x3)XYZ", 1));
}


#[test]
fn recursive_decompression_test_2() {
    assert_eq!(20, decompress_recursive("X(8x2)(3x3)ABCY", 1));
}


#[test]
fn recursive_decompression_test_3() {
    assert_eq!(241920, decompress_recursive("(27x12)(20x12)(13x14)(7x10)(1x12)A", 1));
}


#[test]
fn recursive_decompression_test_4() {
    assert_eq!(445, decompress_recursive("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 1));
}

