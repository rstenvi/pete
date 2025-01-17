/// Defined in [`include/uapi/linux/elf.h`](https://android.googlesource.com/kernel/common/+/refs/heads/android-mainline/include/uapi/linux/elf.h#421).
const NT_ARM_HW_BREAK: i32 = 0x402;
const NT_ARM_HW_WATCH: i32 = 0x403;

pub type DebugRegisters = user_hwdebug_state;

/// Nested, untagged struct declaration in `user_hwdebug_state`.
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct user_hwdebug_state_reg {
    pub addr: u64,
    pub ctrl: u32,
    pad: u32,
}

#[cfg(target_arch = "aarch64")]
impl user_hwdebug_state_reg {
    pub fn new() -> Self {
        Self {
            addr: 0,
            ctrl: 0,
            pad: 0,
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum DebugRegisterType {
    Break = NT_ARM_HW_BREAK,
    Watch = NT_ARM_HW_WATCH,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct user_hwdebug_state {
    pub dbg_info: u32,
    pad: u32,
    pub dbg_regs: [user_hwdebug_state_reg; 4],
}

#[cfg(target_arch = "aarch64")]
impl user_hwdebug_state {
    pub fn new() -> Self {
        Self {
            dbg_info: 0,
            pad: 0,
            dbg_regs: [user_hwdebug_state_reg::new(); 4],
        }
    }
}
