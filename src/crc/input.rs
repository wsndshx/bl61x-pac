#[doc = "Register `input` writer"]
pub type W = crate::W<INPUT_SPEC>;
#[doc = "Field `data` writer - Write data value into checksum calculator"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write data value into checksum calculator"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<INPUT_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets input to value 0"]
impl crate::Resettable for INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
