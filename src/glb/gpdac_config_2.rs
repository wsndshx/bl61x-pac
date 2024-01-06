#[doc = "Register `gpdac_config_2` reader"]
pub type R = crate::R<GPDAC_CONFIG_2_SPEC>;
#[doc = "Register `gpdac_config_2` writer"]
pub type W = crate::W<GPDAC_CONFIG_2_SPEC>;
#[doc = "Field `gpdac_b_en` reader - "]
pub type GPDAC_B_EN_R = crate::BitReader;
#[doc = "Field `gpdac_b_en` writer - "]
pub type GPDAC_B_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_iob_en` reader - "]
pub type GPDAC_IOB_EN_R = crate::BitReader;
#[doc = "Field `gpdac_iob_en` writer - "]
pub type GPDAC_IOB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_b_rng` reader - "]
pub type GPDAC_B_RNG_R = crate::FieldReader;
#[doc = "Field `gpdac_b_rng` writer - "]
pub type GPDAC_B_RNG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_b_outmux` reader - "]
pub type GPDAC_B_OUTMUX_R = crate::FieldReader;
#[doc = "Field `gpdac_b_outmux` writer - "]
pub type GPDAC_B_OUTMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&self) -> GPDAC_B_EN_R {
        GPDAC_B_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&self) -> GPDAC_IOB_EN_R {
        GPDAC_IOB_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&self) -> GPDAC_B_RNG_R {
        GPDAC_B_RNG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&self) -> GPDAC_B_OUTMUX_R {
        GPDAC_B_OUTMUX_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_b_en(&mut self) -> GPDAC_B_EN_W<GPDAC_CONFIG_2_SPEC> {
        GPDAC_B_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_iob_en(&mut self) -> GPDAC_IOB_EN_W<GPDAC_CONFIG_2_SPEC> {
        GPDAC_IOB_EN_W::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_b_rng(&mut self) -> GPDAC_B_RNG_W<GPDAC_CONFIG_2_SPEC> {
        GPDAC_B_RNG_W::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_b_outmux(&mut self) -> GPDAC_B_OUTMUX_W<GPDAC_CONFIG_2_SPEC> {
        GPDAC_B_OUTMUX_W::new(self, 20)
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
#[doc = "General Purpose Digital-to-analog convert configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDAC_CONFIG_2_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config_2::R`](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config_2::W`](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config_2 to value 0"]
impl crate::Resettable for GPDAC_CONFIG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
