#[doc = "Register `audio_pll_cfg1` reader"]
pub type R = crate::R<AUDIO_PLL_CFG1_SPEC>;
#[doc = "Register `audio_pll_cfg1` writer"]
pub type W = crate::W<AUDIO_PLL_CFG1_SPEC>;
#[doc = "Field `aupll_postdiv` reader - "]
pub type AUPLL_POSTDIV_R = crate::FieldReader;
#[doc = "Field `aupll_postdiv` writer - "]
pub type AUPLL_POSTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `aupll_refdiv_ratio` reader - "]
pub type AUPLL_REFDIV_RATIO_R = crate::FieldReader;
#[doc = "Field `aupll_refdiv_ratio` writer - "]
pub type AUPLL_REFDIV_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `aupll_refclk_sel` reader - "]
pub type AUPLL_REFCLK_SEL_R = crate::FieldReader;
#[doc = "Field `aupll_refclk_sel` writer - "]
pub type AUPLL_REFCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_vg11_sel` reader - "]
pub type AUPLL_VG11_SEL_R = crate::FieldReader;
#[doc = "Field `aupll_vg11_sel` writer - "]
pub type AUPLL_VG11_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_vg13_sel` reader - "]
pub type AUPLL_VG13_SEL_R = crate::FieldReader;
#[doc = "Field `aupll_vg13_sel` writer - "]
pub type AUPLL_VG13_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn aupll_postdiv(&self) -> AUPLL_POSTDIV_R {
        AUPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn aupll_refdiv_ratio(&self) -> AUPLL_REFDIV_RATIO_R {
        AUPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn aupll_refclk_sel(&self) -> AUPLL_REFCLK_SEL_R {
        AUPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn aupll_vg11_sel(&self) -> AUPLL_VG11_SEL_R {
        AUPLL_VG11_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn aupll_vg13_sel(&self) -> AUPLL_VG13_SEL_R {
        AUPLL_VG13_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_postdiv(&mut self) -> AUPLL_POSTDIV_W<AUDIO_PLL_CFG1_SPEC> {
        AUPLL_POSTDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_refdiv_ratio(&mut self) -> AUPLL_REFDIV_RATIO_W<AUDIO_PLL_CFG1_SPEC> {
        AUPLL_REFDIV_RATIO_W::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_refclk_sel(&mut self) -> AUPLL_REFCLK_SEL_W<AUDIO_PLL_CFG1_SPEC> {
        AUPLL_REFCLK_SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_vg11_sel(&mut self) -> AUPLL_VG11_SEL_W<AUDIO_PLL_CFG1_SPEC> {
        AUPLL_VG11_SEL_W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_vg13_sel(&mut self) -> AUPLL_VG13_SEL_W<AUDIO_PLL_CFG1_SPEC> {
        AUPLL_VG13_SEL_W::new(self, 24)
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
#[doc = "audio_pll_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG1_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg1 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
