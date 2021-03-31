// compile-flags: -C no-prepopulate-passes

// Check that we use undef (and not zero) for uninitialized bytes in constants.

#![crate_type = "lib"]

use std::mem::MaybeUninit;

pub struct PartiallyUninit {
    x: u32,
    y: MaybeUninit<[u8; 10]>
}

// CHECK: [[FULLY_UNINIT:@[0-9]+]] = private unnamed_addr constant <{ [10 x i8] }> undef
// CHECK: [[PARTIALLY_UNINIT:@[0-9]+]] = private unnamed_addr constant <{ [4 x i8], [12 x i8] }> <{ [4 x i8] c"\EF\BE\AD\DE", [12 x i8] undef }>, align 4
// CHECK: [[FULLY_UNINIT_HUGE:@[0-9]+]] = private unnamed_addr constant <{ [16384 x i8] }> undef

// This shouldn't contain undef, since generating huge partially undef constants is expensive.
// CHECK: [[UNINIT_PADDING_HUGE:@[0-9]+]] = private unnamed_addr constant <{ [32768 x i8] }> <{ [32768 x i8] c"{{.+}}" }>, align 4

// CHECK-LABEL: @fully_uninit
#[no_mangle]
pub const fn fully_uninit() -> MaybeUninit<[u8; 10]> {
    const M: MaybeUninit<[u8; 10]> = MaybeUninit::uninit();
    // CHECK: call void @llvm.memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 1 %1, i8* align 1 getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* [[FULLY_UNINIT]], i32 0, i32 0, i32 0), i{{(32|64)}} 10, i1 false)
    M
}

// CHECK-LABEL: @partially_uninit
#[no_mangle]
pub const fn partially_uninit() -> PartiallyUninit {
    const X: PartiallyUninit = PartiallyUninit { x: 0xdeadbeef, y: MaybeUninit::uninit() };
    // CHECK: call void @llvm.memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [12 x i8] }>, <{ [4 x i8], [12 x i8] }>* [[PARTIALLY_UNINIT]], i32 0, i32 0, i32 0), i{{(32|64)}} 16, i1 false)
    X
}

// CHECK-LABEL: @fully_uninit_huge
#[no_mangle]
pub const fn fully_uninit_huge() -> MaybeUninit<[u32; 4096]> {
    const F: MaybeUninit<[u32; 4096]> = MaybeUninit::uninit();
    // CHECK: call void @llvm.memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [16384 x i8] }>, <{ [16384 x i8] }>* [[FULLY_UNINIT_HUGE]], i32 0, i32 0, i32 0), i{{(32|64)}} 16384, i1 false)
    F
}

// CHECK-LABEL: @uninit_padding_huge
#[no_mangle]
pub const fn uninit_padding_huge() -> [(u32, u8); 4096] {
    const X: [(u32, u8); 4096] = [(123, 45); 4096];
    // CHECK: call void @llvm.memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [32768 x i8] }>, <{ [32768 x i8] }>* [[UNINIT_PADDING_HUGE]], i32 0, i32 0, i32 0), i{{(32|64)}} 32768, i1 false)
    X
}
