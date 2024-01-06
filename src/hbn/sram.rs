#[doc = "Register `sram` reader"]
pub type R = crate::R<SRAM_SPEC>;
#[doc = "Register `sram` writer"]
pub type W = crate::W<SRAM_SPEC>;
#[doc = "Field `retram_ret` reader - "]
pub type RETRAM_RET_R = crate::BitReader;
#[doc = "Field `retram_ret` writer - "]
pub type RETRAM_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `retram_slp` reader - "]
pub type RETRAM_SLP_R = crate::BitReader;
#[doc = "Field `retram_slp` writer - "]
pub type RETRAM_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&self) -> RETRAM_RET_R {
        RETRAM_RET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&self) -> RETRAM_SLP_R {
        RETRAM_SLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn retram_ret(&mut self) -> RETRAM_RET_W<SRAM_SPEC> {
        RETRAM_RET_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn retram_slp(&mut self) -> RETRAM_SLP_W<SRAM_SPEC> {
        RETRAM_SLP_W::new(self, 7)
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
#[doc = "Static Random-Access Memory hibernate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_SPEC;
impl crate::RegisterSpec for SRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram::R`](R) reader structure"]
impl crate::Readable for SRAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram::W`](W) writer structure"]
impl crate::Writable for SRAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram to value 0"]
impl crate::Resettable for SRAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
