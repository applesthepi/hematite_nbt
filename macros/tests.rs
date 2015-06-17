#![feature(plugin, test, custom_derive, custom_attribute)]
#![plugin(nbt_macros)]

extern crate nbt;
extern crate test;

use nbt::serialize;

#[derive(NbtFmt)]
struct TestStruct {
    name: String,
    health: i8,
    food: f32,
    #[nbt_field = "emeralds"]
    ems: i16,
    timestamp: i32
}

#[allow(dead_code)]
#[derive(NbtFmt)]
struct TestTupleStruct(i8, i8, f32);

#[test]
fn nbt_test_struct_serialize() {
  let test = TestStruct {
    name: "Herobrine".to_string(),
    health: 100, food: 20.0, ems: 12345, timestamp: 1424778774
  };

  let mut dst = Vec::new();
  serialize::to_writer(&mut dst, test).unwrap();

  let bytes = [
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x04,
                0x6e, 0x61, 0x6d, 0x65,
                0x00, 0x09,
                0x48, 0x65, 0x72, 0x6f, 0x62, 0x72, 0x69, 0x6e, 0x65,
            0x01,
                0x00, 0x06,
                0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
                0x64,
            0x05,
                0x00, 0x04,
                0x66, 0x6f, 0x6f, 0x64,
                0x41, 0xa0, 0x00, 0x00,
            0x02,
                0x00, 0x08,
                0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x64, 0x73,
                0x30, 0x39,
            0x03,
                0x00, 0x09,
                0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
                0x54, 0xec, 0x66, 0x16,
        0x00
    ];

    assert_eq!(&bytes[..], &dst[..]);
}
