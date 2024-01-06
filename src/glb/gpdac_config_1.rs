#[doc = "Register `gpdac_config_1` reader"]
pub type R = crate::R<GPDAC_CONFIG_1_SPEC>;
#[doc = "Register `gpdac_config_1` writer"]
pub type W = crate::W<GPDAC_CONFIG_1_SPEC>;
#[doc = "Field `gpdac_a_en` reader - "]
pub type GPDAC_A_EN_R = crate::BitReader;
#[doc = "Field `gpdac_a_en` writer - "]
pub type GPDAC_A_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ioa_en` reader - "]
pub type GPDAC_IOA_EN_R = crate::BitReader;
#[doc = "Field `gpdac_ioa_en` writer - "]
pub type GPDAC_IOA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_a_rng` reader - "]
pub type GPDAC_A_RNG_R = crate::FieldReader;
#[doc = "Field `gpdac_a_rng` writer - "]
pub type GPDAC_A_RNG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_a_outmux` reader - "]
pub type GPDAC_A_OUTMUX_R = crate::FieldReader;
#[doc = "Field `gpdac_a_outmux` writer - "]
pub type GPDAC_A_OUTMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GPDAC_A_EN_R {
        GPDAC_A_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GPDAC_IOA_EN_R {
        GPDAC_IOA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GPDAC_A_RNG_R {
        GPDAC_A_RNG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GPDAC_A_OUTMUX_R {
        GPDAC_A_OUTMUX_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_en(&mut self) -> GPDAC_A_EN_W<GPDAC_CONFIG_1_SPEC> {
        GPDAC_A_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ioa_en(&mut self) -> GPDAC_IOA_EN_W<GPDAC_CONFIG_1_SPEC> {
        GPDAC_IOA_EN_W::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_rng(&mut self) -> GPDAC_A_RNG_W<GPDAC_CONFIG_1_SPEC> {
        GPDAC_A_RNG_W::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_outmux(&mut self) -> GPDAC_A_OUTMUX_W<GPDAC_CONFIG_1_SPEC> {
        GPDAC_A_OUTMUX_W::new(self, 20)
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
#[doc = "General Purpose Digital-to-analog convert configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDAC_CONFIG_1_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config_1::R`](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config_1::W`](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config_1 to value 0"]
impl crate::Resettable for GPDAC_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
