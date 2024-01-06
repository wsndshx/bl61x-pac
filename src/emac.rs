#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mode: MODE,
    interrupt_source: INTERRUPT_SOURCE,
    interrupt_mask: INTERRUPT_MASK,
    backed_gap: BACKED_GAP,
    non_backed_gap_1: NON_BACKED_GAP_1,
    non_backed_gap_2: NON_BACKED_GAP_2,
    frame_length: FRAME_LENGTH,
    collision: COLLISION,
    transmit_buffer: TRANSMIT_BUFFER,
    flow_control: FLOW_CONTROL,
    mii_mode: MII_MODE,
    mii_command: MII_COMMAND,
    mii_address: MII_ADDRESS,
    control_write: CONTROL_WRITE,
    control_read: CONTROL_READ,
    mii_state: MII_STATE,
    mac_address: [MAC_ADDRESS; 2],
    hash: [HASH; 2],
    transmit_control: TRANSMIT_CONTROL,
}
impl RegisterBlock {
    #[doc = "0x00 - Interface enables and configurations"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x04 - Interrupt source register"]
    #[inline(always)]
    pub const fn interrupt_source(&self) -> &INTERRUPT_SOURCE {
        &self.interrupt_source
    }
    #[doc = "0x08 - Interrupt mask register"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &INTERRUPT_MASK {
        &self.interrupt_mask
    }
    #[doc = "0x0c - Back-to-back inter-packet gap register"]
    #[inline(always)]
    pub const fn backed_gap(&self) -> &BACKED_GAP {
        &self.backed_gap
    }
    #[doc = "0x10 - Non back-to-back inter-packet gap register 1"]
    #[inline(always)]
    pub const fn non_backed_gap_1(&self) -> &NON_BACKED_GAP_1 {
        &self.non_backed_gap_1
    }
    #[doc = "0x14 - Non back-to-back inter-packet gap register 2"]
    #[inline(always)]
    pub const fn non_backed_gap_2(&self) -> &NON_BACKED_GAP_2 {
        &self.non_backed_gap_2
    }
    #[doc = "0x18 - Minimum and maximum ethernet frame length"]
    #[inline(always)]
    pub const fn frame_length(&self) -> &FRAME_LENGTH {
        &self.frame_length
    }
    #[doc = "0x1c - Collision time window and maximum retries"]
    #[inline(always)]
    pub const fn collision(&self) -> &COLLISION {
        &self.collision
    }
    #[doc = "0x20 - Transmit buffer descriptor"]
    #[inline(always)]
    pub const fn transmit_buffer(&self) -> &TRANSMIT_BUFFER {
        &self.transmit_buffer
    }
    #[doc = "0x24 - Control frame function register"]
    #[inline(always)]
    pub const fn flow_control(&self) -> &FLOW_CONTROL {
        &self.flow_control
    }
    #[doc = "0x28 - MII clock divider and premable enable"]
    #[inline(always)]
    pub const fn mii_mode(&self) -> &MII_MODE {
        &self.mii_mode
    }
    #[doc = "0x2c - MII control data, read and scan state"]
    #[inline(always)]
    pub const fn mii_command(&self) -> &MII_COMMAND {
        &self.mii_command
    }
    #[doc = "0x30 - Physical layer bus address"]
    #[inline(always)]
    pub const fn mii_address(&self) -> &MII_ADDRESS {
        &self.mii_address
    }
    #[doc = "0x34 - Write data to MII physcial layer"]
    #[inline(always)]
    pub const fn control_write(&self) -> &CONTROL_WRITE {
        &self.control_write
    }
    #[doc = "0x38 - Read data from MII physcial layer"]
    #[inline(always)]
    pub const fn control_read(&self) -> &CONTROL_READ {
        &self.control_read
    }
    #[doc = "0x3c - MII bus and link layer state"]
    #[inline(always)]
    pub const fn mii_state(&self) -> &MII_STATE {
        &self.mii_state
    }
    #[doc = "0x40..0x48 - Media Access Control address"]
    #[inline(always)]
    pub const fn mac_address(&self, n: usize) -> &MAC_ADDRESS {
        &self.mac_address[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - Media Access Control address"]
    #[inline(always)]
    pub fn mac_address_iter(&self) -> impl Iterator<Item = &MAC_ADDRESS> {
        self.mac_address.iter()
    }
    #[doc = "0x48..0x50 - Hash register"]
    #[inline(always)]
    pub const fn hash(&self, n: usize) -> &HASH {
        &self.hash[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x48..0x50 - Hash register"]
    #[inline(always)]
    pub fn hash_iter(&self) -> impl Iterator<Item = &HASH> {
        self.hash.iter()
    }
    #[doc = "0x50 - Transmit control register"]
    #[inline(always)]
    pub const fn transmit_control(&self) -> &TRANSMIT_CONTROL {
        &self.transmit_control
    }
}
#[doc = "mode (rw) register accessor: Interface enables and configurations\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Interface enables and configurations"]
pub mod mode;
#[doc = "interrupt_source (rw) register accessor: Interrupt source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_source::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_source::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_source`]
module"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "Interrupt source register"]
pub mod interrupt_source;
#[doc = "interrupt_mask (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod interrupt_mask;
#[doc = "backed_gap (rw) register accessor: Back-to-back inter-packet gap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backed_gap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backed_gap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backed_gap`]
module"]
pub type BACKED_GAP = crate::Reg<backed_gap::BACKED_GAP_SPEC>;
#[doc = "Back-to-back inter-packet gap register"]
pub mod backed_gap;
#[doc = "non_backed_gap_1 (rw) register accessor: Non back-to-back inter-packet gap register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_backed_gap_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_backed_gap_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@non_backed_gap_1`]
module"]
pub type NON_BACKED_GAP_1 = crate::Reg<non_backed_gap_1::NON_BACKED_GAP_1_SPEC>;
#[doc = "Non back-to-back inter-packet gap register 1"]
pub mod non_backed_gap_1;
#[doc = "non_backed_gap_2 (rw) register accessor: Non back-to-back inter-packet gap register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_backed_gap_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_backed_gap_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@non_backed_gap_2`]
module"]
pub type NON_BACKED_GAP_2 = crate::Reg<non_backed_gap_2::NON_BACKED_GAP_2_SPEC>;
#[doc = "Non back-to-back inter-packet gap register 2"]
pub mod non_backed_gap_2;
#[doc = "frame_length (rw) register accessor: Minimum and maximum ethernet frame length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_length`]
module"]
pub type FRAME_LENGTH = crate::Reg<frame_length::FRAME_LENGTH_SPEC>;
#[doc = "Minimum and maximum ethernet frame length"]
pub mod frame_length;
#[doc = "collision (rw) register accessor: Collision time window and maximum retries\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`collision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`collision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@collision`]
module"]
pub type COLLISION = crate::Reg<collision::COLLISION_SPEC>;
#[doc = "Collision time window and maximum retries"]
pub mod collision;
#[doc = "transmit_buffer (rw) register accessor: Transmit buffer descriptor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_buffer`]
module"]
pub type TRANSMIT_BUFFER = crate::Reg<transmit_buffer::TRANSMIT_BUFFER_SPEC>;
#[doc = "Transmit buffer descriptor"]
pub mod transmit_buffer;
#[doc = "flow_control (rw) register accessor: Control frame function register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_control`]
module"]
pub type FLOW_CONTROL = crate::Reg<flow_control::FLOW_CONTROL_SPEC>;
#[doc = "Control frame function register"]
pub mod flow_control;
#[doc = "mii_mode (rw) register accessor: MII clock divider and premable enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_mode`]
module"]
pub type MII_MODE = crate::Reg<mii_mode::MII_MODE_SPEC>;
#[doc = "MII clock divider and premable enable"]
pub mod mii_mode;
#[doc = "mii_command (rw) register accessor: MII control data, read and scan state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_command`]
module"]
pub type MII_COMMAND = crate::Reg<mii_command::MII_COMMAND_SPEC>;
#[doc = "MII control data, read and scan state"]
pub mod mii_command;
#[doc = "mii_address (rw) register accessor: Physical layer bus address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_address`]
module"]
pub type MII_ADDRESS = crate::Reg<mii_address::MII_ADDRESS_SPEC>;
#[doc = "Physical layer bus address"]
pub mod mii_address;
#[doc = "control_write (rw) register accessor: Write data to MII physcial layer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control_write`]
module"]
pub type CONTROL_WRITE = crate::Reg<control_write::CONTROL_WRITE_SPEC>;
#[doc = "Write data to MII physcial layer"]
pub mod control_write;
#[doc = "control_read (rw) register accessor: Read data from MII physcial layer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control_read`]
module"]
pub type CONTROL_READ = crate::Reg<control_read::CONTROL_READ_SPEC>;
#[doc = "Read data from MII physcial layer"]
pub mod control_read;
#[doc = "mii_state (rw) register accessor: MII bus and link layer state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_state`]
module"]
pub type MII_STATE = crate::Reg<mii_state::MII_STATE_SPEC>;
#[doc = "MII bus and link layer state"]
pub mod mii_state;
#[doc = "mac_address (rw) register accessor: Media Access Control address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address`]
module"]
pub type MAC_ADDRESS = crate::Reg<mac_address::MAC_ADDRESS_SPEC>;
#[doc = "Media Access Control address"]
pub mod mac_address;
#[doc = "hash (rw) register accessor: Hash register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash`]
module"]
pub type HASH = crate::Reg<hash::HASH_SPEC>;
#[doc = "Hash register"]
pub mod hash;
#[doc = "transmit_control (rw) register accessor: Transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_control`]
module"]
pub type TRANSMIT_CONTROL = crate::Reg<transmit_control::TRANSMIT_CONTROL_SPEC>;
#[doc = "Transmit control register"]
pub mod transmit_control;
