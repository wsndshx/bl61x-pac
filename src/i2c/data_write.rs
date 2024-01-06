#[doc = "Register `data_write` writer"]
pub type W = crate::W<DATA_WRITE_SPEC>;
#[doc = "Field `value` writer - Write data to FIFO"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write data to FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<DATA_WRITE_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "FIFO write data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_write::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_WRITE_SPEC;
impl crate::RegisterSpec for DATA_WRITE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_write::W`](W) writer structure"]
impl crate::Writable for DATA_WRITE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets data_write to value 0"]
impl crate::Resettable for DATA_WRITE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
