#[doc = "Register `core_cfg17` reader"]
pub type R = crate::R<CORE_CFG17_SPEC>;
#[doc = "Register `core_cfg17` writer"]
pub type W = crate::W<CORE_CFG17_SPEC>;
#[doc = "Field `np_int_sta1` reader - "]
pub type NP_INT_STA1_R = crate::FieldReader<u32>;
#[doc = "Field `np_int_sta1` writer - "]
pub type NP_INT_STA1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_sta1(&self) -> NP_INT_STA1_R {
        NP_INT_STA1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn np_int_sta1(&mut self) -> NP_INT_STA1_W<CORE_CFG17_SPEC> {
        NP_INT_STA1_W::new(self, 0)
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
#[doc = "core_cfg17.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_CFG17_SPEC;
impl crate::RegisterSpec for CORE_CFG17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg17::R`](R) reader structure"]
impl crate::Readable for CORE_CFG17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_cfg17::W`](W) writer structure"]
impl crate::Writable for CORE_CFG17_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core_cfg17 to value 0"]
impl crate::Resettable for CORE_CFG17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
