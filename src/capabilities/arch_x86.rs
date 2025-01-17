#![allow(dead_code)]
use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    pub struct X86Flags01hEdx: u32 {
        // i486+
        const FPU  = 1 << 0;
        // i586+ (Pentium)
        const CX8  = 1 << 8;
        const MMX  = 1 << 23; //(NOTE: i586c only)
        // i686+ (Pentium II/Pentium Pro)
        const SEP  = 1 << 11;
        const CMOV = 1 << 15;
        const FXSR = 1 << 24;
        // i786+ (Pentium III)
        const SSE = 1 << 25;
        // i886+ (Pentium 4)
        const SSE2 = 1 << 26;
    }

    pub struct X86Flags01hEcx: u32 {
        //x86-64-v2
        const SSE3       = 1 << 0;
        const SSSE3      = 1 << 9;
        const CMPXCHG16B = 1 << 13;
        const SSE4_1     = 1 << 19;
        const SSE4_2     = 1 << 20;
        const POPCNT     = 1 << 23;
        //x86-64-v3
        const FMA        = 1 << 12;
        const MOVBE      = 1 << 22;
        const OSXSAVE    = 1 << 27;
        const AVX        = 1 << 28;
        const F16C       = 1 << 29;
    }

    pub struct X86Flags80000001hEcx: u32 {
        //x86-64-v2
        const LAHF_SAHF = 1 << 0;
        //x86-64-v3
        const LZCNT     = 1 << 5;
    }

    pub struct X86Flags07hEbx: u32 {
        //x86-64-v3
        const BMI1     = 1 << 3;
        const AVX2     = 1 << 5;
        const BMI2     = 1 << 8;
        //x86-64-v4
        const AVX512F  = 1 << 16;
        const AVX512DQ = 1 << 17;
        const AVX512CD = 1 << 28;
        const AVX512BW = 1 << 30;
        const AVX512VL = 1 << 31;
    }
}

// IA32 hwcaps
const I486_HWCAPS: u32 = X86Flags01hEdx::FPU.bits();
const I586_HWCAPS: u32 = I486_HWCAPS | X86Flags01hEdx::CX8.bits() | X86Flags01hEdx::MMX.bits();
const I686_HWCAPS: u32 = I586_HWCAPS | X86Flags01hEdx::SEP.bits() | X86Flags01hEdx::CMOV.bits() | X86Flags01hEdx::FXSR.bits();
//These go unused because practically no 32bit programs are built for these feature sets
const I786_HWCAPS: u32 = I686_HWCAPS | X86Flags01hEdx::SSE.bits();
const I886_HWCAPS: u32 = I786_HWCAPS | X86Flags01hEdx::SSE2.bits();

// X86_64 hwcaps
const X86_64_V1_HWCAPS: u32 = I886_HWCAPS;
const X86_64_V2_HWCAPS_01H_ECX: u32 = X86Flags01hEcx::SSE3.bits() | X86Flags01hEcx::SSSE3.bits() | X86Flags01hEcx::CMPXCHG16B.bits()
                                    | X86Flags01hEcx::SSE4_1.bits() | X86Flags01hEcx::SSE4_2.bits() | X86Flags01hEcx::SSE4_2.bits()
                                    | X86Flags01hEcx::CMPXCHG16B.bits();
const X86_64_V2_HWCAPS_80000001H_ECX: u32 = X86Flags80000001hEcx::LAHF_SAHF.bits();
const X86_64_V3_HWCAPS_01H_ECX: u32 = X86_64_V2_HWCAPS_01H_ECX | X86Flags01hEcx::FMA.bits() | X86Flags01hEcx::MOVBE.bits()
                                    | X86Flags01hEcx::OSXSAVE.bits() | X86Flags01hEcx::AVX.bits() | X86Flags01hEcx::F16C.bits();
const X86_64_V3_HWCAPS_80000001H_ECX: u32 = X86_64_V2_HWCAPS_80000001H_ECX | X86Flags80000001hEcx::LZCNT.bits();
const X86_64_V3_HWCAPS_07H_EBX: u32 = X86Flags07hEbx::BMI1.bits() | X86Flags07hEbx::AVX2.bits() | X86Flags07hEbx::BMI2.bits();
const X86_64_V4_HWCAPS_07H_EBX: u32 = X86_64_V3_HWCAPS_07H_EBX | X86Flags07hEbx::AVX512F.bits() | X86Flags07hEbx::AVX512DQ.bits()
                                    | X86Flags07hEbx::AVX512CD.bits() | X86Flags07hEbx::AVX512BW.bits() | X86Flags07hEbx::AVX512VL.bits();
const X86_HWCAPS_STRING: &'static [u8] = b"i\086";
const X86_HWCAPS_VERSION_INDEX: usize = 1;
const X86_64_HWCAPS_STRING: &'static [u8] = b"x86-64-v\0";
const X86_64_HWCAPS_VERSION_INDEX: usize = X86_64_HWCAPS_STRING.len() - 1;

pub const HWCAPS_CHARS: [u8; 8] = [
    b'3',
    b'4',
    b'5',
    b'6',
    b'1', // x86_64 feature levels (4)
    b'2',
    b'3',
    b'4'
];
const X86_64_HWCAPS_INDEX: u32 = 4;

#[inline]
pub fn arch_name_changed(fl: u32) -> bool {
    return fl + 1 == X86_64_HWCAPS_INDEX
}

#[inline]
pub fn format_arch_name(buffer: &mut [u8], feature_level: u32) -> Result<(usize, usize), ()> {
    let arch_string: &[u8];
    let version_index: usize;

    if feature_level < X86_64_HWCAPS_INDEX {
        arch_string = X86_HWCAPS_STRING;
        version_index = X86_HWCAPS_VERSION_INDEX
    } else {
        arch_string = X86_64_HWCAPS_STRING;
        version_index = X86_64_HWCAPS_VERSION_INDEX
    }

    if buffer.len() < arch_string.len() {
        return Err(())
    }

    unsafe { buffer.get_unchecked_mut(0..arch_string.len()).copy_from_slice(arch_string) };
    buffer[version_index] = HWCAPS_CHARS[feature_level as usize];

    Ok((version_index, arch_string.len()))
}

#[cfg(target_arch = "x86")]
#[inline]
pub fn get_max_feature_level() -> u32 {
    let feature_bitset: u32;

    unsafe {
        asm!(
            //Dark magic adapted from https://www.prowaretech.com/articles/current/assembly/x86/cpuid-library
            // CPUID was only added to a revision of the i486, so we need this workaround to confirm its presence
	        "pushfd",           // push eflags on the stack
	        "pop eax",          // pop them into eax
	        "mov ebx, eax",     // save to ebx for restoring afterwards
	        "xor eax, 200000h", // toggle bit 21
	        "push eax",         // push the toggled eflags
	        "popfd",            // pop them back into eflags
	        "pushfd",           // push eflags
	        "pop eax",          // pop them back into eax
	        "cmp eax, ebx",     // see if bit 21 was reset
	        "jz 2f",
            "jmp 3f",

            "2:", //CPUID supported
            "xor edx, edx", // Set no bits so we fall back to i386
	        "jmp 4f",

            "3:", //CPUID supported
            "mov eax, 1h",
            "pop ebx",
	        "cpuid",
	        "push ebx",

            "4:",
            out("eax") _,
            out("ecx") _,
            out("edx") feature_bitset,
        )
    }

    if feature_bitset == 0 {
        return 0
    }

    let mut feature_level = 0;

    for i in 1..=(X86_64_HWCAPS_INDEX-1) {
        let has_feature = match i {
            1 => feature_bitset & I486_HWCAPS == I486_HWCAPS,
            2 => feature_bitset & I586_HWCAPS == I586_HWCAPS,
            3 => feature_bitset & I686_HWCAPS == I686_HWCAPS,
            _ => false
        };

        if !has_feature {
            break
        }

        feature_level += 1
    }

    feature_level
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn get_max_feature_level() -> u32 {
    let feature_set_01h_ecx: u32;
    let feature_set_80000001h_ecx: u32;
    let feature_set_07h_ebx: u32;

    unsafe {
        asm!(
            "push rbx",

            // Get leaf 1h (Exists on all x86 CPUs with cpuid)
            "mov eax, 1h",
            "cpuid",
            "push rcx",

            // Get leaf 7h (Introduced with Core Duo, exists on all x86-64-v2+ CPUs)
            "mov eax, 7h",
            "xor ecx, ecx",
            "cpuid",
            "push rbx",

            // Get leaf 80000001h (Introduced with Pentium 4, exists on all x86-64 CPUs)
            "mov eax, 80000001h",
            "cpuid",

            "pop rdx",
            "pop rax",

            "pop rbx",

            out("eax") feature_set_01h_ecx,
            out("ecx") feature_set_80000001h_ecx,
            out("edx") feature_set_07h_ebx,
            options(pure, nomem)
        );
    };

    if !(feature_set_01h_ecx & X86_64_V2_HWCAPS_01H_ECX == X86_64_V2_HWCAPS_01H_ECX)
    || !(feature_set_80000001h_ecx  & X86_64_V2_HWCAPS_80000001H_ECX == X86_64_V2_HWCAPS_80000001H_ECX) {
        return X86_64_HWCAPS_INDEX
    }

    if !(feature_set_01h_ecx & X86_64_V3_HWCAPS_01H_ECX == X86_64_V3_HWCAPS_01H_ECX)
    || !(feature_set_07h_ebx & X86_64_V3_HWCAPS_07H_EBX == X86_64_V3_HWCAPS_07H_EBX)
    || !(feature_set_80000001h_ecx & X86_64_V3_HWCAPS_80000001H_ECX == X86_64_V3_HWCAPS_80000001H_ECX) {
        return X86_64_HWCAPS_INDEX + 1
    }

    if !(feature_set_07h_ebx & X86_64_V4_HWCAPS_07H_EBX == X86_64_V4_HWCAPS_07H_EBX) {
        return X86_64_HWCAPS_INDEX + 2
    }

    return X86_64_HWCAPS_INDEX + 3
}
