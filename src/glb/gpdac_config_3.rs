#[doc = "Register `gpdac_config_3` reader"]
pub type R = crate::R<GPDAC_CONFIG_3_SPEC>;
#[doc = "Register `gpdac_config_3` writer"]
pub type W = crate::W<GPDAC_CONFIG_3_SPEC>;
#[doc = "Field `gpdac_b_data` reader - "]
pub type GPDAC_B_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `gpdac_b_data` writer - "]
pub type GPDAC_B_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `gpdac_a_data` reader - "]
pub type GPDAC_A_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `gpdac_a_data` writer - "]
pub type GPDAC_A_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpdac_b_data(&self) -> GPDAC_B_DATA_R {
        GPDAC_B_DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn gpdac_a_data(&self) -> GPDAC_A_DATA_R {
        GPDAC_A_DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_b_data(&mut self) -> GPDAC_B_DATA_W<GPDAC_CONFIG_3_SPEC> {
        GPDAC_B_DATA_W::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_data(&mut self) -> GPDAC_A_DATA_W<GPDAC_CONFIG_3_SPEC> {
        GPDAC_A_DATA_W::new(self, 16)
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
#[doc = "General Purpose Digital-to-analog convert configuration 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDAC_CONFIG_3_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config_3::R`](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config_3::W`](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config_3 to value 0"]
impl crate::Resettable for GPDAC_CONFIG_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
