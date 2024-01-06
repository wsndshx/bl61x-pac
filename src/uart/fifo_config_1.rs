#[doc = "Register `fifo_config_1` reader"]
pub type R = crate::R<FIFO_CONFIG_1_SPEC>;
#[doc = "Register `fifo_config_1` writer"]
pub type W = crate::W<FIFO_CONFIG_1_SPEC>;
#[doc = "Field `transmit_count` reader - Count of available data in transmit FIFO"]
pub type TRANSMIT_COUNT_R = crate::FieldReader;
#[doc = "Field `receive_count` reader - Count of available data in receive FIFO"]
pub type RECEIVE_COUNT_R = crate::FieldReader;
#[doc = "Field `transmit_threshold` reader - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
pub type TRANSMIT_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `transmit_threshold` writer - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
pub type TRANSMIT_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `receive_threshold` reader - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub type RECEIVE_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `receive_threshold` writer - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub type RECEIVE_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Count of available data in transmit FIFO"]
    #[inline(always)]
    pub fn transmit_count(&self) -> TRANSMIT_COUNT_R {
        TRANSMIT_COUNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Count of available data in receive FIFO"]
    #[inline(always)]
    pub fn receive_count(&self) -> RECEIVE_COUNT_R {
        RECEIVE_COUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
    #[inline(always)]
    pub fn transmit_threshold(&self) -> TRANSMIT_THRESHOLD_R {
        TRANSMIT_THRESHOLD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn receive_threshold(&self) -> RECEIVE_THRESHOLD_R {
        RECEIVE_THRESHOLD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_threshold(&mut self) -> TRANSMIT_THRESHOLD_W<FIFO_CONFIG_1_SPEC> {
        TRANSMIT_THRESHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    #[must_use]
    pub fn receive_threshold(&mut self) -> RECEIVE_THRESHOLD_W<FIFO_CONFIG_1_SPEC> {
        RECEIVE_THRESHOLD_W::new(self, 24)
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
#[doc = "FIFO configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONFIG_1_SPEC;
impl crate::RegisterSpec for FIFO_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_config_1::R`](R) reader structure"]
impl crate::Readable for FIFO_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_config_1::W`](W) writer structure"]
impl crate::Writable for FIFO_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fifo_config_1 to value 0x20"]
impl crate::Resettable for FIFO_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
