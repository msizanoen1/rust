// compile-flags: -C no-prepopulate-passes
// ignore-riscv32
// ignore-riscv64

#![crate_type = "lib"]
#![feature(repr_simd)]

#[repr(simd)]
struct f32x4(f32, f32, f32, f32);

#[repr(transparent)]
pub struct Vector(f32x4);

// CHECK: define <4 x float> @test_Vector(<4 x float> %_1)
#[no_mangle]
pub extern fn test_Vector(_: Vector) -> Vector { loop {} }
