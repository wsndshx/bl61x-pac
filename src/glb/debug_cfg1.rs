#[doc = "Register `debug_cfg1` reader"]
pub type R = crate::R<DEBUG_CFG1_SPEC>;
#[doc = "Register `debug_cfg1` writer"]
pub type W = crate::W<DEBUG_CFG1_SPEC>;
#[doc = "Field `debug_ndreset_gate` reader - "]
pub type DEBUG_NDRESET_GATE_R = crate::BitReader;
#[doc = "Field `debug_ndreset_gate` writer - "]
pub type DEBUG_NDRESET_GATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DEBUG_NDRESET_GATE_R {
        DEBUG_NDRESET_GATE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ndreset_gate(&mut self) -> DEBUG_NDRESET_GATE_W<DEBUG_CFG1_SPEC> {
        DEBUG_NDRESET_GATE_W::new(self, 20)
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
#[doc = "debug_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CFG1_SPEC;
impl crate::RegisterSpec for DEBUG_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_cfg1::R`](R) reader structure"]
impl crate::Readable for DEBUG_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_cfg1::W`](W) writer structure"]
impl crate::Writable for DEBUG_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug_cfg1 to value 0"]
impl crate::Resettable for DEBUG_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
