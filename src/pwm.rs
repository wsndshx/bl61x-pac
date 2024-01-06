#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    interrupt_config: INTERRUPT_CONFIG,
    _reserved1: [u8; 0x3c],
    group: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt state and clear register"]
    #[inline(always)]
    pub const fn interrupt_config(&self) -> &INTERRUPT_CONFIG {
        &self.interrupt_config
    }
    #[doc = "0x40..0xa0 - Pulse-Width Modulation channel group"]
    #[inline(always)]
    pub const fn group(&self, n: usize) -> &GROUP {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(64)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0xa0 - Pulse-Width Modulation channel group"]
    #[inline(always)]
    pub fn group_iter(&self) -> impl Iterator<Item = &GROUP> {
        (0..2).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(64)
                .add(64 * n)
                .cast()
        })
    }
}
#[doc = "interrupt_config (rw) register accessor: Interrupt state and clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_config`]
module"]
pub type INTERRUPT_CONFIG = crate::Reg<interrupt_config::INTERRUPT_CONFIG_SPEC>;
#[doc = "Interrupt state and clear register"]
pub mod interrupt_config;
#[doc = "Pulse-Width Modulation channel group"]
pub use self::group::GROUP;
#[doc = r"Cluster"]
#[doc = "Pulse-Width Modulation channel group"]
pub mod group;
