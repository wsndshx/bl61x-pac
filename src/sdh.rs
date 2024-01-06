#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    todo: TODO,
}
impl RegisterBlock {
    #[doc = "0x00 - ??"]
    #[inline(always)]
    pub const fn todo(&self) -> &TODO {
        &self.todo
    }
}
#[doc = "todo (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`todo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`todo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@todo`]
module"]
pub type TODO = crate::Reg<todo::TODO_SPEC>;
#[doc = "??"]
pub mod todo;
