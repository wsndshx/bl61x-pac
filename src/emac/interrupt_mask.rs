#[doc = "Register `interrupt_mask` reader"]
pub type R = crate::R<INTERRUPT_MASK_SPEC>;
#[doc = "Register `interrupt_mask` writer"]
pub type W = crate::W<INTERRUPT_MASK_SPEC>;
#[doc = "Field `buffer_transmitted` reader - Buffer transmitted interrupt mask"]
pub use CONTROL_RECEIVE_R as BUFFER_TRANSMITTED_R;
#[doc = "Field `transmit_error` reader - Transmit error interrupt mask"]
pub use CONTROL_RECEIVE_R as TRANSMIT_ERROR_R;
#[doc = "Field `frame_received` reader - Frame received interrupt mask"]
pub use CONTROL_RECEIVE_R as FRAME_RECEIVED_R;
#[doc = "Field `receive_error` reader - Receive error interrupt mask"]
pub use CONTROL_RECEIVE_R as RECEIVE_ERROR_R;
#[doc = "Field `busy` reader - Lack of buffer interrupt mask"]
pub use CONTROL_RECEIVE_R as BUSY_R;
#[doc = "Field `control_transmit` reader - Control frame transmitted interrupt mask"]
pub use CONTROL_RECEIVE_R as CONTROL_TRANSMIT_R;
#[doc = "Field `buffer_transmitted` writer - Buffer transmitted interrupt mask"]
pub use CONTROL_RECEIVE_W as BUFFER_TRANSMITTED_W;
#[doc = "Field `transmit_error` writer - Transmit error interrupt mask"]
pub use CONTROL_RECEIVE_W as TRANSMIT_ERROR_W;
#[doc = "Field `frame_received` writer - Frame received interrupt mask"]
pub use CONTROL_RECEIVE_W as FRAME_RECEIVED_W;
#[doc = "Field `receive_error` writer - Receive error interrupt mask"]
pub use CONTROL_RECEIVE_W as RECEIVE_ERROR_W;
#[doc = "Field `busy` writer - Lack of buffer interrupt mask"]
pub use CONTROL_RECEIVE_W as BUSY_W;
#[doc = "Field `control_transmit` writer - Control frame transmitted interrupt mask"]
pub use CONTROL_RECEIVE_W as CONTROL_TRANSMIT_W;
#[doc = "Field `control_receive` reader - Control frame received interrupt mask"]
pub type CONTROL_RECEIVE_R = crate::BitReader<INTERRUPT_MASK_A>;
#[doc = "Control frame received interrupt mask\n\nValue on reset: 0"]
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
impl CONTROL_RECEIVE_R {
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
#[doc = "Field `control_receive` writer - Control frame received interrupt mask"]
pub type CONTROL_RECEIVE_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK_A>;
impl<'a, REG> CONTROL_RECEIVE_W<'a, REG>
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
impl R {
    #[doc = "Bit 0 - Buffer transmitted interrupt mask"]
    #[inline(always)]
    pub fn buffer_transmitted(&self) -> BUFFER_TRANSMITTED_R {
        BUFFER_TRANSMITTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit error interrupt mask"]
    #[inline(always)]
    pub fn transmit_error(&self) -> TRANSMIT_ERROR_R {
        TRANSMIT_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame received interrupt mask"]
    #[inline(always)]
    pub fn frame_received(&self) -> FRAME_RECEIVED_R {
        FRAME_RECEIVED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive error interrupt mask"]
    #[inline(always)]
    pub fn receive_error(&self) -> RECEIVE_ERROR_R {
        RECEIVE_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lack of buffer interrupt mask"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control frame transmitted interrupt mask"]
    #[inline(always)]
    pub fn control_transmit(&self) -> CONTROL_TRANSMIT_R {
        CONTROL_TRANSMIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control frame received interrupt mask"]
    #[inline(always)]
    pub fn control_receive(&self) -> CONTROL_RECEIVE_R {
        CONTROL_RECEIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_transmitted(&mut self) -> BUFFER_TRANSMITTED_W<INTERRUPT_MASK_SPEC> {
        BUFFER_TRANSMITTED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_error(&mut self) -> TRANSMIT_ERROR_W<INTERRUPT_MASK_SPEC> {
        TRANSMIT_ERROR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Frame received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn frame_received(&mut self) -> FRAME_RECEIVED_W<INTERRUPT_MASK_SPEC> {
        FRAME_RECEIVED_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_error(&mut self) -> RECEIVE_ERROR_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_ERROR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Lack of buffer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<INTERRUPT_MASK_SPEC> {
        BUSY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Control frame transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn control_transmit(&mut self) -> CONTROL_TRANSMIT_W<INTERRUPT_MASK_SPEC> {
        CONTROL_TRANSMIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Control frame received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn control_receive(&mut self) -> CONTROL_RECEIVE_W<INTERRUPT_MASK_SPEC> {
        CONTROL_RECEIVE_W::new(self, 6)
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
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_mask to value 0"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
