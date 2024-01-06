#[doc = "Register `cci_rdata` reader"]
pub type R = crate::R<CCI_RDATA_SPEC>;
#[doc = "Register `cci_rdata` writer"]
pub type W = crate::W<CCI_RDATA_SPEC>;
#[doc = "Field `apb_cci_rdata` reader - "]
pub type APB_CCI_RDATA_R = crate::FieldReader<u32>;
#[doc = "Field `apb_cci_rdata` writer - "]
pub type APB_CCI_RDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_rdata(&self) -> APB_CCI_RDATA_R {
        APB_CCI_RDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn apb_cci_rdata(&mut self) -> APB_CCI_RDATA_W<CCI_RDATA_SPEC> {
        APB_CCI_RDATA_W::new(self, 0)
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
#[doc = "cci_rdata.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_rdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_rdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCI_RDATA_SPEC;
impl crate::RegisterSpec for CCI_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_rdata::R`](R) reader structure"]
impl crate::Readable for CCI_RDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cci_rdata::W`](W) writer structure"]
impl crate::Writable for CCI_RDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_rdata to value 0"]
impl crate::Resettable for CCI_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
