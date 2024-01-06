#[doc = "Register `audio_pll_cfg4` reader"]
pub type R = crate::R<AUDIO_PLL_CFG4_SPEC>;
#[doc = "Register `audio_pll_cfg4` writer"]
pub type W = crate::W<AUDIO_PLL_CFG4_SPEC>;
#[doc = "Field `aupll_sel_sample_clk` reader - "]
pub type AUPLL_SEL_SAMPLE_CLK_R = crate::FieldReader;
#[doc = "Field `aupll_sel_sample_clk` writer - "]
pub type AUPLL_SEL_SAMPLE_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_sel_fb_clk` reader - "]
pub type AUPLL_SEL_FB_CLK_R = crate::FieldReader;
#[doc = "Field `aupll_sel_fb_clk` writer - "]
pub type AUPLL_SEL_FB_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_sdmclk_sel` reader - "]
pub type AUPLL_SDMCLK_SEL_R = crate::BitReader;
#[doc = "Field `aupll_sdmclk_sel` writer - "]
pub type AUPLL_SDMCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sel_sample_clk(&self) -> AUPLL_SEL_SAMPLE_CLK_R {
        AUPLL_SEL_SAMPLE_CLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn aupll_sel_fb_clk(&self) -> AUPLL_SEL_FB_CLK_R {
        AUPLL_SEL_FB_CLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn aupll_sdmclk_sel(&self) -> AUPLL_SDMCLK_SEL_R {
        AUPLL_SDMCLK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sel_sample_clk(&mut self) -> AUPLL_SEL_SAMPLE_CLK_W<AUDIO_PLL_CFG4_SPEC> {
        AUPLL_SEL_SAMPLE_CLK_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sel_fb_clk(&mut self) -> AUPLL_SEL_FB_CLK_W<AUDIO_PLL_CFG4_SPEC> {
        AUPLL_SEL_FB_CLK_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdmclk_sel(&mut self) -> AUPLL_SDMCLK_SEL_W<AUDIO_PLL_CFG4_SPEC> {
        AUPLL_SDMCLK_SEL_W::new(self, 8)
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
#[doc = "audio_pll_cfg4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG4_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg4 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
