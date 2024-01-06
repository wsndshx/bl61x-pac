#[doc = "Register `interrupt` reader"]
pub type R = crate::R<INTERRUPT_SPEC>;
#[doc = "Register `interrupt` writer"]
pub type W = crate::W<INTERRUPT_SPEC>;
#[doc = "Field `transfer_end_state` reader - Transfer ended interrupt state"]
pub use FIFO_ERROR_STATE_R as TRANSFER_END_STATE_R;
#[doc = "Field `transmit_fifo_ready_state` reader - Transmit FIFO ready interrupt state\n\n Auto cleared when data is pushed into transmit FIFO."]
pub use FIFO_ERROR_STATE_R as TRANSMIT_FIFO_READY_STATE_R;
#[doc = "Field `receive_fifo_ready_state` reader - Receive FIFO ready interrupt state\n\n Auto cleared when data is popped from receive FIFO."]
pub use FIFO_ERROR_STATE_R as RECEIVE_FIFO_READY_STATE_R;
#[doc = "Field `not_acknowledged_state` reader - Not-acknowledged response interrupt state"]
pub use FIFO_ERROR_STATE_R as NOT_ACKNOWLEDGED_STATE_R;
#[doc = "Field `arbitrate_lost_state` reader - Arbitration lost interrupt state"]
pub use FIFO_ERROR_STATE_R as ARBITRATE_LOST_STATE_R;
#[doc = "Field `fifo_error_state` reader - Transmit or receive FIFO error interrupt state\n\n Auto cleared when FIFO overflow or underflow error flag is cleared."]
pub type FIFO_ERROR_STATE_R = crate::BitReader<INTERRUPT_STATE_A>;
#[doc = "Transmit or receive FIFO error interrupt state\n\n Auto cleared when FIFO overflow or underflow error flag is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_STATE_A {
    #[doc = "1: Has interrupt"]
    HAS_INTERRUPT = 1,
    #[doc = "0: No interrupt occurred"]
    NO_INTERRUPT = 0,
}
impl From<INTERRUPT_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_ERROR_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_STATE_A {
        match self.bits {
            true => INTERRUPT_STATE_A::HAS_INTERRUPT,
            false => INTERRUPT_STATE_A::NO_INTERRUPT,
        }
    }
    #[doc = "Has interrupt"]
    #[inline(always)]
    pub fn is_has_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::HAS_INTERRUPT
    }
    #[doc = "No interrupt occurred"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::NO_INTERRUPT
    }
}
#[doc = "Field `transfer_end_mask` reader - Transfer ended interrupt mask"]
pub use FIFO_ERROR_MASK_R as TRANSFER_END_MASK_R;
#[doc = "Field `transmit_fifo_ready_mask` reader - Transmit FIFO ready interrupt mask"]
pub use FIFO_ERROR_MASK_R as TRANSMIT_FIFO_READY_MASK_R;
#[doc = "Field `receive_fifo_ready_mask` reader - Receive FIFO ready interrupt mask"]
pub use FIFO_ERROR_MASK_R as RECEIVE_FIFO_READY_MASK_R;
#[doc = "Field `not_acknowledged_mask` reader - Not-acknowledged response interrupt mask"]
pub use FIFO_ERROR_MASK_R as NOT_ACKNOWLEDGED_MASK_R;
#[doc = "Field `arbitrate_lost_mask` reader - Arbitration lost interrupt mask"]
pub use FIFO_ERROR_MASK_R as ARBITRATE_LOST_MASK_R;
#[doc = "Field `transfer_end_mask` writer - Transfer ended interrupt mask"]
pub use FIFO_ERROR_MASK_W as TRANSFER_END_MASK_W;
#[doc = "Field `transmit_fifo_ready_mask` writer - Transmit FIFO ready interrupt mask"]
pub use FIFO_ERROR_MASK_W as TRANSMIT_FIFO_READY_MASK_W;
#[doc = "Field `receive_fifo_ready_mask` writer - Receive FIFO ready interrupt mask"]
pub use FIFO_ERROR_MASK_W as RECEIVE_FIFO_READY_MASK_W;
#[doc = "Field `not_acknowledged_mask` writer - Not-acknowledged response interrupt mask"]
pub use FIFO_ERROR_MASK_W as NOT_ACKNOWLEDGED_MASK_W;
#[doc = "Field `arbitrate_lost_mask` writer - Arbitration lost interrupt mask"]
pub use FIFO_ERROR_MASK_W as ARBITRATE_LOST_MASK_W;
#[doc = "Field `fifo_error_mask` reader - Transmit or receive FIFO error interrupt mask"]
pub type FIFO_ERROR_MASK_R = crate::BitReader<INTERRUPT_MASK_A>;
#[doc = "Transmit or receive FIFO error interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK_A {
    #[doc = "1: Mask interrupt"]
    MASK = 1,
    #[doc = "0: Unmask interrupt"]
    UNMASK = 0,
}
impl From<INTERRUPT_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_ERROR_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK_A {
        match self.bits {
            true => INTERRUPT_MASK_A::MASK,
            false => INTERRUPT_MASK_A::UNMASK,
        }
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INTERRUPT_MASK_A::MASK
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == INTERRUPT_MASK_A::UNMASK
    }
}
#[doc = "Field `fifo_error_mask` writer - Transmit or receive FIFO error interrupt mask"]
pub type FIFO_ERROR_MASK_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK_A>;
impl<'a, REG> FIFO_ERROR_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK_A::UNMASK)
    }
}
#[doc = "Field `transfer_end_clear` writer - Write 1 to clear transfer ended"]
pub use ARBITRATE_LOST_CLEAR_W as TRANSFER_END_CLEAR_W;
#[doc = "Field `not_acknowledged_clear` writer - Write 1 to clear not-acknowledged response"]
pub use ARBITRATE_LOST_CLEAR_W as NOT_ACKNOWLEDGED_CLEAR_W;
#[doc = "Write 1 to clear arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_CLEAR_AW {
    #[doc = "1: Write 1 to clear interrupt state"]
    CLEAR = 1,
}
impl From<INTERRUPT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `arbitrate_lost_clear` writer - Write 1 to clear arbitration lost"]
pub type ARBITRATE_LOST_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_CLEAR_AW>;
impl<'a, REG> ARBITRATE_LOST_CLEAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `transfer_end_enable` reader - Transfer ended interrupt enable"]
pub use FIFO_ERROR_ENABLE_R as TRANSFER_END_ENABLE_R;
#[doc = "Field `transmit_fifo_ready_enable` reader - Transmit FIFO ready interrupt enable"]
pub use FIFO_ERROR_ENABLE_R as TRANSMIT_FIFO_READY_ENABLE_R;
#[doc = "Field `receive_fifo_ready_enable` reader - Receive FIFO ready interrupt enable"]
pub use FIFO_ERROR_ENABLE_R as RECEIVE_FIFO_READY_ENABLE_R;
#[doc = "Field `not_acknowledged_enable` reader - Not-acknowledged response interrupt enable"]
pub use FIFO_ERROR_ENABLE_R as NOT_ACKNOWLEDGED_ENABLE_R;
#[doc = "Field `arbitrate_lost_enable` reader - Arbitration lost interrupt enable"]
pub use FIFO_ERROR_ENABLE_R as ARBITRATE_LOST_ENABLE_R;
#[doc = "Field `transfer_end_enable` writer - Transfer ended interrupt enable"]
pub use FIFO_ERROR_ENABLE_W as TRANSFER_END_ENABLE_W;
#[doc = "Field `transmit_fifo_ready_enable` writer - Transmit FIFO ready interrupt enable"]
pub use FIFO_ERROR_ENABLE_W as TRANSMIT_FIFO_READY_ENABLE_W;
#[doc = "Field `receive_fifo_ready_enable` writer - Receive FIFO ready interrupt enable"]
pub use FIFO_ERROR_ENABLE_W as RECEIVE_FIFO_READY_ENABLE_W;
#[doc = "Field `not_acknowledged_enable` writer - Not-acknowledged response interrupt enable"]
pub use FIFO_ERROR_ENABLE_W as NOT_ACKNOWLEDGED_ENABLE_W;
#[doc = "Field `arbitrate_lost_enable` writer - Arbitration lost interrupt enable"]
pub use FIFO_ERROR_ENABLE_W as ARBITRATE_LOST_ENABLE_W;
#[doc = "Field `fifo_error_enable` reader - Transmit or receive FIFO error interrupt enable"]
pub type FIFO_ERROR_ENABLE_R = crate::BitReader<INTERRUPT_ENABLE_A>;
#[doc = "Transmit or receive FIFO error interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_ENABLE_A {
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
}
impl From<INTERRUPT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_ERROR_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_ENABLE_A {
        match self.bits {
            true => INTERRUPT_ENABLE_A::ENABLE,
            false => INTERRUPT_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::ENABLE
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::DISABLE
    }
}
#[doc = "Field `fifo_error_enable` writer - Transmit or receive FIFO error interrupt enable"]
pub type FIFO_ERROR_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_ENABLE_A>;
impl<'a, REG> FIFO_ERROR_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_ENABLE_A::ENABLE)
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_ENABLE_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Transfer ended interrupt state"]
    #[inline(always)]
    pub fn transfer_end_state(&self) -> TRANSFER_END_STATE_R {
        TRANSFER_END_STATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO ready interrupt state\n\n Auto cleared when data is pushed into transmit FIFO."]
    #[inline(always)]
    pub fn transmit_fifo_ready_state(&self) -> TRANSMIT_FIFO_READY_STATE_R {
        TRANSMIT_FIFO_READY_STATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO ready interrupt state\n\n Auto cleared when data is popped from receive FIFO."]
    #[inline(always)]
    pub fn receive_fifo_ready_state(&self) -> RECEIVE_FIFO_READY_STATE_R {
        RECEIVE_FIFO_READY_STATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Not-acknowledged response interrupt state"]
    #[inline(always)]
    pub fn not_acknowledged_state(&self) -> NOT_ACKNOWLEDGED_STATE_R {
        NOT_ACKNOWLEDGED_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt state"]
    #[inline(always)]
    pub fn arbitrate_lost_state(&self) -> ARBITRATE_LOST_STATE_R {
        ARBITRATE_LOST_STATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit or receive FIFO error interrupt state\n\n Auto cleared when FIFO overflow or underflow error flag is cleared."]
    #[inline(always)]
    pub fn fifo_error_state(&self) -> FIFO_ERROR_STATE_R {
        FIFO_ERROR_STATE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer ended interrupt mask"]
    #[inline(always)]
    pub fn transfer_end_mask(&self) -> TRANSFER_END_MASK_R {
        TRANSFER_END_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO ready interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_ready_mask(&self) -> TRANSMIT_FIFO_READY_MASK_R {
        TRANSMIT_FIFO_READY_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO ready interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_ready_mask(&self) -> RECEIVE_FIFO_READY_MASK_R {
        RECEIVE_FIFO_READY_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Not-acknowledged response interrupt mask"]
    #[inline(always)]
    pub fn not_acknowledged_mask(&self) -> NOT_ACKNOWLEDGED_MASK_R {
        NOT_ACKNOWLEDGED_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Arbitration lost interrupt mask"]
    #[inline(always)]
    pub fn arbitrate_lost_mask(&self) -> ARBITRATE_LOST_MASK_R {
        ARBITRATE_LOST_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit or receive FIFO error interrupt mask"]
    #[inline(always)]
    pub fn fifo_error_mask(&self) -> FIFO_ERROR_MASK_R {
        FIFO_ERROR_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Transfer ended interrupt enable"]
    #[inline(always)]
    pub fn transfer_end_enable(&self) -> TRANSFER_END_ENABLE_R {
        TRANSFER_END_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO ready interrupt enable"]
    #[inline(always)]
    pub fn transmit_fifo_ready_enable(&self) -> TRANSMIT_FIFO_READY_ENABLE_R {
        TRANSMIT_FIFO_READY_ENABLE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive FIFO ready interrupt enable"]
    #[inline(always)]
    pub fn receive_fifo_ready_enable(&self) -> RECEIVE_FIFO_READY_ENABLE_R {
        RECEIVE_FIFO_READY_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Not-acknowledged response interrupt enable"]
    #[inline(always)]
    pub fn not_acknowledged_enable(&self) -> NOT_ACKNOWLEDGED_ENABLE_R {
        NOT_ACKNOWLEDGED_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn arbitrate_lost_enable(&self) -> ARBITRATE_LOST_ENABLE_R {
        ARBITRATE_LOST_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit or receive FIFO error interrupt enable"]
    #[inline(always)]
    pub fn fifo_error_enable(&self) -> FIFO_ERROR_ENABLE_R {
        FIFO_ERROR_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transfer ended interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_end_mask(&mut self) -> TRANSFER_END_MASK_W<INTERRUPT_SPEC> {
        TRANSFER_END_MASK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit FIFO ready interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_ready_mask(&mut self) -> TRANSMIT_FIFO_READY_MASK_W<INTERRUPT_SPEC> {
        TRANSMIT_FIFO_READY_MASK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive FIFO ready interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_ready_mask(&mut self) -> RECEIVE_FIFO_READY_MASK_W<INTERRUPT_SPEC> {
        RECEIVE_FIFO_READY_MASK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Not-acknowledged response interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn not_acknowledged_mask(&mut self) -> NOT_ACKNOWLEDGED_MASK_W<INTERRUPT_SPEC> {
        NOT_ACKNOWLEDGED_MASK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Arbitration lost interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn arbitrate_lost_mask(&mut self) -> ARBITRATE_LOST_MASK_W<INTERRUPT_SPEC> {
        ARBITRATE_LOST_MASK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit or receive FIFO error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_error_mask(&mut self) -> FIFO_ERROR_MASK_W<INTERRUPT_SPEC> {
        FIFO_ERROR_MASK_W::new(self, 13)
    }
    #[doc = "Bit 16 - Write 1 to clear transfer ended"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_end_clear(&mut self) -> TRANSFER_END_CLEAR_W<INTERRUPT_SPEC> {
        TRANSFER_END_CLEAR_W::new(self, 16)
    }
    #[doc = "Bit 19 - Write 1 to clear not-acknowledged response"]
    #[inline(always)]
    #[must_use]
    pub fn not_acknowledged_clear(&mut self) -> NOT_ACKNOWLEDGED_CLEAR_W<INTERRUPT_SPEC> {
        NOT_ACKNOWLEDGED_CLEAR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Write 1 to clear arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn arbitrate_lost_clear(&mut self) -> ARBITRATE_LOST_CLEAR_W<INTERRUPT_SPEC> {
        ARBITRATE_LOST_CLEAR_W::new(self, 20)
    }
    #[doc = "Bit 24 - Transfer ended interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_end_enable(&mut self) -> TRANSFER_END_ENABLE_W<INTERRUPT_SPEC> {
        TRANSFER_END_ENABLE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Transmit FIFO ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_ready_enable(&mut self) -> TRANSMIT_FIFO_READY_ENABLE_W<INTERRUPT_SPEC> {
        TRANSMIT_FIFO_READY_ENABLE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Receive FIFO ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_ready_enable(&mut self) -> RECEIVE_FIFO_READY_ENABLE_W<INTERRUPT_SPEC> {
        RECEIVE_FIFO_READY_ENABLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Not-acknowledged response interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn not_acknowledged_enable(&mut self) -> NOT_ACKNOWLEDGED_ENABLE_W<INTERRUPT_SPEC> {
        NOT_ACKNOWLEDGED_ENABLE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Arbitration lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn arbitrate_lost_enable(&mut self) -> ARBITRATE_LOST_ENABLE_W<INTERRUPT_SPEC> {
        ARBITRATE_LOST_ENABLE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Transmit or receive FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_error_enable(&mut self) -> FIFO_ERROR_ENABLE_W<INTERRUPT_SPEC> {
        FIFO_ERROR_ENABLE_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enables, states and masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt to value 0x3f00_3f00"]
impl crate::Resettable for INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f00_3f00;
}
