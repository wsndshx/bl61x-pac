#[doc = "Register `audio_pll_cfg11` reader"]
pub type R = crate::R<AUDIO_PLL_CFG11_SPEC>;
#[doc = "Register `audio_pll_cfg11` writer"]
pub type W = crate::W<AUDIO_PLL_CFG11_SPEC>;
#[doc = "Field `aupll_resv` reader - "]
pub type AUPLL_RESV_R = crate::FieldReader<u16>;
#[doc = "Field `aupll_resv` writer - "]
pub type AUPLL_RESV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `aupll_dl_ctrl_15` reader - "]
pub type AUPLL_DL_CTRL_15_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_15` writer - "]
pub type AUPLL_DL_CTRL_15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_10` reader - "]
pub type AUPLL_DL_CTRL_10_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_10` writer - "]
pub type AUPLL_DL_CTRL_10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_6` reader - "]
pub type AUPLL_DL_CTRL_6_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_6` writer - "]
pub type AUPLL_DL_CTRL_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_5` reader - "]
pub type AUPLL_DL_CTRL_5_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_5` writer - "]
pub type AUPLL_DL_CTRL_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_4` reader - "]
pub type AUPLL_DL_CTRL_4_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_4` writer - "]
pub type AUPLL_DL_CTRL_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_3` reader - "]
pub type AUPLL_DL_CTRL_3_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_3` writer - "]
pub type AUPLL_DL_CTRL_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_2p5` reader - "]
pub type AUPLL_DL_CTRL_2P5_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_2p5` writer - "]
pub type AUPLL_DL_CTRL_2P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_2` reader - "]
pub type AUPLL_DL_CTRL_2_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_2` writer - "]
pub type AUPLL_DL_CTRL_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_1` reader - "]
pub type AUPLL_DL_CTRL_1_R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_1` writer - "]
pub type AUPLL_DL_CTRL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn aupll_resv(&self) -> AUPLL_RESV_R {
        AUPLL_RESV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_15(&self) -> AUPLL_DL_CTRL_15_R {
        AUPLL_DL_CTRL_15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_10(&self) -> AUPLL_DL_CTRL_10_R {
        AUPLL_DL_CTRL_10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_6(&self) -> AUPLL_DL_CTRL_6_R {
        AUPLL_DL_CTRL_6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_5(&self) -> AUPLL_DL_CTRL_5_R {
        AUPLL_DL_CTRL_5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_4(&self) -> AUPLL_DL_CTRL_4_R {
        AUPLL_DL_CTRL_4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_3(&self) -> AUPLL_DL_CTRL_3_R {
        AUPLL_DL_CTRL_3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2p5(&self) -> AUPLL_DL_CTRL_2P5_R {
        AUPLL_DL_CTRL_2P5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2(&self) -> AUPLL_DL_CTRL_2_R {
        AUPLL_DL_CTRL_2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_1(&self) -> AUPLL_DL_CTRL_1_R {
        AUPLL_DL_CTRL_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_resv(&mut self) -> AUPLL_RESV_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_RESV_W::new(self, 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_15(&mut self) -> AUPLL_DL_CTRL_15_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_15_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_10(&mut self) -> AUPLL_DL_CTRL_10_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_10_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_6(&mut self) -> AUPLL_DL_CTRL_6_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_6_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_5(&mut self) -> AUPLL_DL_CTRL_5_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_5_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_4(&mut self) -> AUPLL_DL_CTRL_4_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_4_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_3(&mut self) -> AUPLL_DL_CTRL_3_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_3_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_2p5(&mut self) -> AUPLL_DL_CTRL_2P5_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_2P5_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_2(&mut self) -> AUPLL_DL_CTRL_2_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_2_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_1(&mut self) -> AUPLL_DL_CTRL_1_W<AUDIO_PLL_CFG11_SPEC> {
        AUPLL_DL_CTRL_1_W::new(self, 31)
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
#[doc = "audio_pll_cfg11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG11_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg11::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg11::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg11 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
