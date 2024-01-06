#[doc = "Register `receive_timeout` reader"]
pub type R = crate::R<RECEIVE_TIMEOUT_SPEC>;
#[doc = "Register `receive_timeout` writer"]
pub type W = crate::W<RECEIVE_TIMEOUT_SPEC>;
#[doc = "Field `value` reader - Timeout interrupt triggering value by bits received"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - Timeout interrupt triggering value by bits received"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout interrupt triggering value by bits received"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout interrupt triggering value by bits received"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<RECEIVE_TIMEOUT_SPEC> {
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
#[doc = "Receive Time-Out interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_TIMEOUT_SPEC;
impl crate::RegisterSpec for RECEIVE_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_timeout::R`](R) reader structure"]
impl crate::Readable for RECEIVE_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_timeout::W`](W) writer structure"]
impl crate::Writable for RECEIVE_TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_timeout to value 0x0f"]
impl crate::Resettable for RECEIVE_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
