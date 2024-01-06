#[doc = "Register `audio_pll_cfg10` reader"]
pub type R = crate::R<AUDIO_PLL_CFG10_SPEC>;
#[doc = "Register `audio_pll_cfg10` writer"]
pub type W = crate::W<AUDIO_PLL_CFG10_SPEC>;
#[doc = "Field `aupll_ssc_en` reader - "]
pub type AUPLL_SSC_EN_R = crate::BitReader;
#[doc = "Field `aupll_ssc_en` writer - "]
pub type AUPLL_SSC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_ssc_cnt` reader - "]
pub type AUPLL_SSC_CNT_R = crate::FieldReader;
#[doc = "Field `aupll_ssc_cnt` writer - "]
pub type AUPLL_SSC_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `aupll_ssc_gain` reader - "]
pub type AUPLL_SSC_GAIN_R = crate::FieldReader;
#[doc = "Field `aupll_ssc_gain` writer - "]
pub type AUPLL_SSC_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aupll_ssc_start_gate_en` reader - "]
pub type AUPLL_SSC_START_GATE_EN_R = crate::BitReader;
#[doc = "Field `aupll_ssc_start_gate_en` writer - "]
pub type AUPLL_SSC_START_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_ssc_start` reader - "]
pub type AUPLL_SSC_START_R = crate::BitReader;
#[doc = "Field `aupll_ssc_start` writer - "]
pub type AUPLL_SSC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_ssc_en(&self) -> AUPLL_SSC_EN_R {
        AUPLL_SSC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn aupll_ssc_cnt(&self) -> AUPLL_SSC_CNT_R {
        AUPLL_SSC_CNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn aupll_ssc_gain(&self) -> AUPLL_SSC_GAIN_R {
        AUPLL_SSC_GAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn aupll_ssc_start_gate_en(&self) -> AUPLL_SSC_START_GATE_EN_R {
        AUPLL_SSC_START_GATE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn aupll_ssc_start(&self) -> AUPLL_SSC_START_R {
        AUPLL_SSC_START_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_ssc_en(&mut self) -> AUPLL_SSC_EN_W<AUDIO_PLL_CFG10_SPEC> {
        AUPLL_SSC_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_ssc_cnt(&mut self) -> AUPLL_SSC_CNT_W<AUDIO_PLL_CFG10_SPEC> {
        AUPLL_SSC_CNT_W::new(self, 4)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_ssc_gain(&mut self) -> AUPLL_SSC_GAIN_W<AUDIO_PLL_CFG10_SPEC> {
        AUPLL_SSC_GAIN_W::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_ssc_start_gate_en(&mut self) -> AUPLL_SSC_START_GATE_EN_W<AUDIO_PLL_CFG10_SPEC> {
        AUPLL_SSC_START_GATE_EN_W::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_ssc_start(&mut self) -> AUPLL_SSC_START_W<AUDIO_PLL_CFG10_SPEC> {
        AUPLL_SSC_START_W::new(self, 20)
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
#[doc = "audio_pll_cfg10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG10_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg10::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg10::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg10 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
