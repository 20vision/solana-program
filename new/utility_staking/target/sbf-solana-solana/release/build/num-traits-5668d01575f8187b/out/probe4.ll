; ModuleID = 'probe4.67a9c2c1bd133481-cgu.0'
source_filename = "probe4.67a9c2c1bd133481-cgu.0"
target datalayout = "e-m:e-p:64:64-i64:64-n32:64-S128"
target triple = "sbf"

@alloc_5ede957cf00a314f39ffec25b792bdbb = private unnamed_addr constant <{ [85 x i8] }> <{ [85 x i8] c"/Users/runner/work/platform-tools/platform-tools/out/rust/library/core/src/num/mod.rs" }>, align 1
@alloc_8ea99cec65fe04d5e1ae00b52e56a294 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5ede957cf00a314f39ffec25b792bdbb, [16 x i8] c"U\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h0eed3050f4b3f2a9E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hd0d4eca4de071882E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hf1b4ee6962c0422cE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_8ea99cec65fe04d5e1ae00b52e56a294) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hd0d4eca4de071882E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare void @_ZN4core9panicking5panic17hf1b4ee6962c0422cE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+solana" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" "target-features"="+solana" }
attributes #3 = { noreturn nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
