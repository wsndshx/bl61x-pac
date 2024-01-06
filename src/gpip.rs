#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpadc_config: GPADC_CONFIG,
    gpadc_dma_read: GPADC_DMA_READ,
    _reserved2: [u8; 0x38],
    gpdac_config: GPDAC_CONFIG,
    gpdac_dma_config: GPDAC_DMA_CONFIG,
    gpdac_dma_write: GPDAC_DMA_WRITE,
    gpdac_fifo_state: GPDAC_FIFO_STATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Generic Analog-to-Digital Converter register"]
    #[inline(always)]
    pub const fn gpadc_config(&self) -> &GPADC_CONFIG {
        &self.gpadc_config
    }
    #[doc = "0x04 - DMA data output of Generic Analog-to-Digital Converter"]
    #[inline(always)]
    pub const fn gpadc_dma_read(&self) -> &GPADC_DMA_READ {
        &self.gpadc_dma_read
    }
    #[doc = "0x40 - Generic Digital-to-Analog Converter register"]
    #[inline(always)]
    pub const fn gpdac_config(&self) -> &GPDAC_CONFIG {
        &self.gpdac_config
    }
    #[doc = "0x44 - Digital-to-Analog Converter DMA configuration"]
    #[inline(always)]
    pub const fn gpdac_dma_config(&self) -> &GPDAC_DMA_CONFIG {
        &self.gpdac_dma_config
    }
    #[doc = "0x48 - DMA data input of Generic Digital-to-Analog Converter"]
    #[inline(always)]
    pub const fn gpdac_dma_write(&self) -> &GPDAC_DMA_WRITE {
        &self.gpdac_dma_write
    }
    #[doc = "0x4c - Transmit FIFO state of Generic Digital-to-Analog Converter"]
    #[inline(always)]
    pub const fn gpdac_fifo_state(&self) -> &GPDAC_FIFO_STATE {
        &self.gpdac_fifo_state
    }
}
#[doc = "gpadc_config (rw) register accessor: Generic Analog-to-Digital Converter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_config`]
module"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "Generic Analog-to-Digital Converter register"]
pub mod gpadc_config;
#[doc = "gpadc_dma_read (rw) register accessor: DMA data output of Generic Analog-to-Digital Converter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_dma_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_dma_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_dma_read`]
module"]
pub type GPADC_DMA_READ = crate::Reg<gpadc_dma_read::GPADC_DMA_READ_SPEC>;
#[doc = "DMA data output of Generic Analog-to-Digital Converter"]
pub mod gpadc_dma_read;
#[doc = "gpdac_config (rw) register accessor: Generic Digital-to-Analog Converter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config`]
module"]
pub type GPDAC_CONFIG = crate::Reg<gpdac_config::GPDAC_CONFIG_SPEC>;
#[doc = "Generic Digital-to-Analog Converter register"]
pub mod gpdac_config;
#[doc = "gpdac_dma_config (rw) register accessor: Digital-to-Analog Converter DMA configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_dma_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_dma_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_config`]
module"]
pub type GPDAC_DMA_CONFIG = crate::Reg<gpdac_dma_config::GPDAC_DMA_CONFIG_SPEC>;
#[doc = "Digital-to-Analog Converter DMA configuration"]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_write (rw) register accessor: DMA data input of Generic Digital-to-Analog Converter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_dma_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_dma_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_write`]
module"]
pub type GPDAC_DMA_WRITE = crate::Reg<gpdac_dma_write::GPDAC_DMA_WRITE_SPEC>;
#[doc = "DMA data input of Generic Digital-to-Analog Converter"]
pub mod gpdac_dma_write;
#[doc = "gpdac_fifo_state (rw) register accessor: Transmit FIFO state of Generic Digital-to-Analog Converter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_fifo_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_fifo_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_fifo_state`]
module"]
pub type GPDAC_FIFO_STATE = crate::Reg<gpdac_fifo_state::GPDAC_FIFO_STATE_SPEC>;
#[doc = "Transmit FIFO state of Generic Digital-to-Analog Converter"]
pub mod gpdac_fifo_state;
