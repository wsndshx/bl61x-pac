#[doc = "Register `interrupt_enable` reader"]
pub type R = crate::R<INTERRUPT_ENABLE_SPEC>;
#[doc = "Register `interrupt_enable` writer"]
pub type W = crate::W<INTERRUPT_ENABLE_SPEC>;
#[doc = "Field `transmit_transfer` reader - Transmit transfer signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_TRANSFER_R;
#[doc = "Field `receive_transfer` reader - Receive transfer signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_TRANSFER_R;
#[doc = "Field `transmit_fifo_ready` reader - Transmit FIFO ready signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_FIFO_READY_R;
#[doc = "Field `receive_fifo_ready` reader - Receive FIFO ready signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_FIFO_READY_R;
#[doc = "Field `receive_timeout` reader - Receive timed-out interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_TIMEOUT_R;
#[doc = "Field `receive_parity` reader - Receive parity check failure interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_PARITY_R;
#[doc = "Field `transmit_fifo_error` reader - Transmit FIFO overflow or underflow interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_FIFO_ERROR_R;
#[doc = "Field `receive_fifo_error` reader - Receive FIFO overflow or underflow interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_FIFO_ERROR_R;
#[doc = "Field `receive_sync_error` reader - Receive LIN mode synchronization field error interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_SYNC_ERROR_R;
#[doc = "Field `receive_byte_count` reader - Receive byte count reached interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_BYTE_COUNT_R;
#[doc = "Field `auto_baudrate_start_bit` reader - Receive auto baudrate detection finished using start bit interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as AUTO_BAUDRATE_START_BIT_R;
#[doc = "Field `transmit_transfer` writer - Transmit transfer signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_TRANSFER_W;
#[doc = "Field `receive_transfer` writer - Receive transfer signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TRANSFER_W;
#[doc = "Field `transmit_fifo_ready` writer - Transmit FIFO ready signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_FIFO_READY_W;
#[doc = "Field `receive_fifo_ready` writer - Receive FIFO ready signal interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_FIFO_READY_W;
#[doc = "Field `receive_timeout` writer - Receive timed-out interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TIMEOUT_W;
#[doc = "Field `receive_parity` writer - Receive parity check failure interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_PARITY_W;
#[doc = "Field `transmit_fifo_error` writer - Transmit FIFO overflow or underflow interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_FIFO_ERROR_W;
#[doc = "Field `receive_fifo_error` writer - Receive FIFO overflow or underflow interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_FIFO_ERROR_W;
#[doc = "Field `receive_sync_error` writer - Receive LIN mode synchronization field error interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_SYNC_ERROR_W;
#[doc = "Field `receive_byte_count` writer - Receive byte count reached interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_BYTE_COUNT_W;
#[doc = "Field `auto_baudrate_start_bit` writer - Receive auto baudrate detection finished using start bit interrupt enable"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as AUTO_BAUDRATE_START_BIT_W;
#[doc = "Field `auto_baudrate_five_five` reader - Receive auto baudrate detection finished using 0x55 interrupt enable"]
pub type AUTO_BAUDRATE_FIVE_FIVE_R = crate::BitReader<INTERRUPT_ENABLE_A>;
#[doc = "Receive auto baudrate detection finished using 0x55 interrupt enable\n\nValue on reset: 0"]
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
impl AUTO_BAUDRATE_FIVE_FIVE_R {
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
#[doc = "Field `auto_baudrate_five_five` writer - Receive auto baudrate detection finished using 0x55 interrupt enable"]
pub type AUTO_BAUDRATE_FIVE_FIVE_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_ENABLE_A>;
impl<'a, REG> AUTO_BAUDRATE_FIVE_FIVE_W<'a, REG>
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
    #[doc = "Bit 0 - Transmit transfer signal interrupt enable"]
    #[inline(always)]
    pub fn transmit_transfer(&self) -> TRANSMIT_TRANSFER_R {
        TRANSMIT_TRANSFER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive transfer signal interrupt enable"]
    #[inline(always)]
    pub fn receive_transfer(&self) -> RECEIVE_TRANSFER_R {
        RECEIVE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt enable"]
    #[inline(always)]
    pub fn transmit_fifo_ready(&self) -> TRANSMIT_FIFO_READY_R {
        TRANSMIT_FIFO_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt enable"]
    #[inline(always)]
    pub fn receive_fifo_ready(&self) -> RECEIVE_FIFO_READY_R {
        RECEIVE_FIFO_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt enable"]
    #[inline(always)]
    pub fn receive_timeout(&self) -> RECEIVE_TIMEOUT_R {
        RECEIVE_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt enable"]
    #[inline(always)]
    pub fn receive_parity(&self) -> RECEIVE_PARITY_R {
        RECEIVE_PARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt enable"]
    #[inline(always)]
    pub fn transmit_fifo_error(&self) -> TRANSMIT_FIFO_ERROR_R {
        TRANSMIT_FIFO_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt enable"]
    #[inline(always)]
    pub fn receive_fifo_error(&self) -> RECEIVE_FIFO_ERROR_R {
        RECEIVE_FIFO_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive LIN mode synchronization field error interrupt enable"]
    #[inline(always)]
    pub fn receive_sync_error(&self) -> RECEIVE_SYNC_ERROR_R {
        RECEIVE_SYNC_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive byte count reached interrupt enable"]
    #[inline(always)]
    pub fn receive_byte_count(&self) -> RECEIVE_BYTE_COUNT_R {
        RECEIVE_BYTE_COUNT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive auto baudrate detection finished using start bit interrupt enable"]
    #[inline(always)]
    pub fn auto_baudrate_start_bit(&self) -> AUTO_BAUDRATE_START_BIT_R {
        AUTO_BAUDRATE_START_BIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive auto baudrate detection finished using 0x55 interrupt enable"]
    #[inline(always)]
    pub fn auto_baudrate_five_five(&self) -> AUTO_BAUDRATE_FIVE_FIVE_R {
        AUTO_BAUDRATE_FIVE_FIVE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit transfer signal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_transfer(&mut self) -> TRANSMIT_TRANSFER_W<INTERRUPT_ENABLE_SPEC> {
        TRANSMIT_TRANSFER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive transfer signal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_transfer(&mut self) -> RECEIVE_TRANSFER_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_TRANSFER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_ready(&mut self) -> TRANSMIT_FIFO_READY_W<INTERRUPT_ENABLE_SPEC> {
        TRANSMIT_FIFO_READY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_ready(&mut self) -> RECEIVE_FIFO_READY_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_FIFO_READY_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_timeout(&mut self) -> RECEIVE_TIMEOUT_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_TIMEOUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_parity(&mut self) -> RECEIVE_PARITY_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_PARITY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_error(&mut self) -> TRANSMIT_FIFO_ERROR_W<INTERRUPT_ENABLE_SPEC> {
        TRANSMIT_FIFO_ERROR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_error(&mut self) -> RECEIVE_FIFO_ERROR_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_FIFO_ERROR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive LIN mode synchronization field error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_sync_error(&mut self) -> RECEIVE_SYNC_ERROR_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_SYNC_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive byte count reached interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_byte_count(&mut self) -> RECEIVE_BYTE_COUNT_W<INTERRUPT_ENABLE_SPEC> {
        RECEIVE_BYTE_COUNT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive auto baudrate detection finished using start bit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_start_bit(&mut self) -> AUTO_BAUDRATE_START_BIT_W<INTERRUPT_ENABLE_SPEC> {
        AUTO_BAUDRATE_START_BIT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receive auto baudrate detection finished using 0x55 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_five_five(&mut self) -> AUTO_BAUDRATE_FIVE_FIVE_W<INTERRUPT_ENABLE_SPEC> {
        AUTO_BAUDRATE_FIVE_FIVE_W::new(self, 11)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_enable to value 0xff"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
