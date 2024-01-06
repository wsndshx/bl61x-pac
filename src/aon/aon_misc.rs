#[doc = "Register `aon_misc` reader"]
pub type R = crate::R<AON_MISC_SPEC>;
#[doc = "Register `aon_misc` writer"]
pub type W = crate::W<AON_MISC_SPEC>;
#[doc = "Field `sw_soc_en_aon` reader - "]
pub type SW_SOC_EN_AON_R = crate::BitReader;
#[doc = "Field `sw_soc_en_aon` writer - "]
pub type SW_SOC_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_wb_en_aon` reader - "]
pub type SW_WB_EN_AON_R = crate::BitReader;
#[doc = "Field `sw_wb_en_aon` writer - "]
pub type SW_WB_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&self) -> SW_SOC_EN_AON_R {
        SW_SOC_EN_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&self) -> SW_WB_EN_AON_R {
        SW_WB_EN_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_soc_en_aon(&mut self) -> SW_SOC_EN_AON_W<AON_MISC_SPEC> {
        SW_SOC_EN_AON_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_wb_en_aon(&mut self) -> SW_WB_EN_AON_W<AON_MISC_SPEC> {
        SW_WB_EN_AON_W::new(self, 1)
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
#[doc = "aon_misc.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_MISC_SPEC;
impl crate::RegisterSpec for AON_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_misc::R`](R) reader structure"]
impl crate::Readable for AON_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_misc::W`](W) writer structure"]
impl crate::Writable for AON_MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_misc to value 0"]
impl crate::Resettable for AON_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
