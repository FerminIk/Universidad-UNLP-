; ModuleID = 'probe3.365048ae-cgu.0'
source_filename = "probe3.365048ae-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_1C38D8991A02F778u = comdat any

$__llvm_profile_filename = comdat any

@__covrec_1C38D8991A02F778u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 2033613383835776888, i32 9, i64 3681846349495335764, i64 1329354229273573112, [9 x i8] c"\01\01\00\01\01\01\01\000" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [89 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 89, i32 0, i32 5 }, [89 x i8] c"\02V\00NC:\\Users\\Mar\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCsePH8k6CBxfe_6probe35probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCsePH8k6CBxfe_6probe35probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 2033613383835776888, i64 3681846349495335764, i64 sub (i64 ptrtoint (ptr @__profc__RNvCsePH8k6CBxfe_6probe35probe to i64), i64 ptrtoint (ptr @__profd__RNvCsePH8k6CBxfe_6probe35probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCsePH8k6CBxfe_6probe35probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCsePH8k6CBxfe_6probe35probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_1C38D8991A02F778u, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; probe3::probe
; Function Attrs: uwtable
define void @_RNvCsePH8k6CBxfe_6probe35probe() unnamed_addr #0 !dbg !5 {
start:
  %0 = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  %pgocount = load i64, ptr @__profc__RNvCsePH8k6CBxfe_6probe35probe, align 8, !dbg !11
  %1 = add i64 %pgocount, 1, !dbg !11
  store i64 %1, ptr @__profc__RNvCsePH8k6CBxfe_6probe35probe, align 8, !dbg !11
  store i32 1, ptr %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill.i, metadata !12, metadata !DIExpression()), !dbg !24
  store i32 -2147483648, ptr %0, align 4, !dbg !26
  %2 = load i32, ptr %0, align 4, !dbg !26, !noundef !10
  ret void, !dbg !27
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.bitreverse.i32(i32) #1

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #2

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #3 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nounwind }
attributes #3 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.68.2 (9eb3afe9e 2023-03-27))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe3\\@\\probe3.365048ae-cgu.0", directory: "C:\\Users\\Mar\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCsePH8k6CBxfe_6probe35probe", scope: !7, file: !6, line: 1, type: !8, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !10)
!6 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "eaf77dc2b9749fb037de09345680663a27342131")
!7 = !DINamespace(name: "probe3", scope: null)
!8 = !DISubroutineType(types: !9)
!9 = !{null}
!10 = !{}
!11 = !DILocation(line: 1, scope: !5)
!12 = !DILocalVariable(name: "self", arg: 1, scope: !13, file: !14, line: 285, type: !20)
!13 = distinct !DISubprogram(name: "reverse_bits", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$12reverse_bits17ha51636331e0c7a14E", scope: !15, file: !14, line: 285, type: !18, scopeLine: 285, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !23)
!14 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\num\\uint_macros.rs", directory: "", checksumkind: CSK_SHA1, checksum: "c43b75ca7dcaf959d7af7e4c5178776e42258dbd")
!15 = !DINamespace(name: "impl$9", scope: !16)
!16 = !DINamespace(name: "num", scope: !17)
!17 = !DINamespace(name: "core", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20, !20}
!20 = !DIDerivedType(tag: DW_TAG_typedef, name: "u32", file: !21, baseType: !22)
!21 = !DIFile(filename: "<unknown>", directory: "")
!22 = !DIBasicType(name: "unsigned __int32", size: 32, encoding: DW_ATE_unsigned)
!23 = !{!12}
!24 = !DILocation(line: 285, scope: !13, inlinedAt: !25)
!25 = distinct !DILocation(line: 1, scope: !5)
!26 = !DILocation(line: 286, scope: !13, inlinedAt: !25)
!27 = !DILocation(line: 1, scope: !28)
!28 = !DILexicalBlockFile(scope: !5, file: !6, discriminator: 0)
