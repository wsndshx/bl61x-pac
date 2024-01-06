#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    input: INPUT,
    output: OUTPUT,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Data input register"]
    #[inline(always)]
    pub const fn input(&self) -> &INPUT {
        &self.input
    }
    #[doc = "0x08 - Checksum output register"]
    #[inline(always)]
    pub const fn output(&self) -> &OUTPUT {
        &self.output
    }
}
#[doc = "config (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "input (w) register accessor: Data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "Data input register"]
pub mod input;
#[doc = "output (r) register accessor: Checksum output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "Checksum output register"]
pub mod output;
