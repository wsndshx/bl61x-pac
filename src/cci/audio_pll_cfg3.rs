#[doc = "Register `audio_pll_cfg3` reader"]
pub type R = crate::R<AUDIO_PLL_CFG3_SPEC>;
#[doc = "Register `audio_pll_cfg3` writer"]
pub type W = crate::W<AUDIO_PLL_CFG3_SPEC>;
#[doc = "Field `aupll_c4_en` reader - "]
pub type AUPLL_C4_EN_R = crate::BitReader;
#[doc = "Field `aupll_c4_en` writer - "]
pub type AUPLL_C4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_r4` reader - "]
pub type AUPLL_R4_R = crate::FieldReader;
#[doc = "Field `aupll_r4` writer - "]
pub type AUPLL_R4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_r4_short` reader - "]
pub type AUPLL_R4_SHORT_R = crate::BitReader;
#[doc = "Field `aupll_r4_short` writer - "]
pub type AUPLL_R4_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_c3` reader - "]
pub type AUPLL_C3_R = crate::FieldReader;
#[doc = "Field `aupll_c3` writer - "]
pub type AUPLL_C3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_cz` reader - "]
pub type AUPLL_CZ_R = crate::FieldReader;
#[doc = "Field `aupll_cz` writer - "]
pub type AUPLL_CZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_rz` reader - "]
pub type AUPLL_RZ_R = crate::FieldReader;
#[doc = "Field `aupll_rz` writer - "]
pub type AUPLL_RZ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_c4_en(&self) -> AUPLL_C4_EN_R {
        AUPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn aupll_r4(&self) -> AUPLL_R4_R {
        AUPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn aupll_r4_short(&self) -> AUPLL_R4_SHORT_R {
        AUPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn aupll_c3(&self) -> AUPLL_C3_R {
        AUPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn aupll_cz(&self) -> AUPLL_CZ_R {
        AUPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn aupll_rz(&self) -> AUPLL_RZ_R {
        AUPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_c4_en(&mut self) -> AUPLL_C4_EN_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_C4_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_r4(&mut self) -> AUPLL_R4_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_R4_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_r4_short(&mut self) -> AUPLL_R4_SHORT_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_R4_SHORT_W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_c3(&mut self) -> AUPLL_C3_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_C3_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_cz(&mut self) -> AUPLL_CZ_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_CZ_W::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_rz(&mut self) -> AUPLL_RZ_W<AUDIO_PLL_CFG3_SPEC> {
        AUPLL_RZ_W::new(self, 16)
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
#[doc = "audio_pll_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG3_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg3 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
