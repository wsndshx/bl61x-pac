#[doc = "Register `sram_cfg3` reader"]
pub type R = crate::R<SRAM_CFG3_SPEC>;
#[doc = "Register `sram_cfg3` writer"]
pub type W = crate::W<SRAM_CFG3_SPEC>;
#[doc = "Field `em_sel` reader - "]
pub type EM_SEL_R = crate::FieldReader;
#[doc = "Field `em_sel` writer - "]
pub type EM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&self) -> EM_SEL_R {
        EM_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn em_sel(&mut self) -> EM_SEL_W<SRAM_CFG3_SPEC> {
        EM_SEL_W::new(self, 0)
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
#[doc = "sram_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CFG3_SPEC;
impl crate::RegisterSpec for SRAM_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg3::R`](R) reader structure"]
impl crate::Readable for SRAM_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg3::W`](W) writer structure"]
impl crate::Writable for SRAM_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_cfg3 to value 0"]
impl crate::Resettable for SRAM_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
