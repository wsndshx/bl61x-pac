#[doc = "Register `audio_pll_cfg0` reader"]
pub type R = crate::R<AUDIO_PLL_CFG0_SPEC>;
#[doc = "Register `audio_pll_cfg0` writer"]
pub type W = crate::W<AUDIO_PLL_CFG0_SPEC>;
#[doc = "Field `aupll_sdm_rstb` reader - "]
pub type AUPLL_SDM_RSTB_R = crate::BitReader;
#[doc = "Field `aupll_sdm_rstb` writer - "]
pub type AUPLL_SDM_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_postdiv_rstb` reader - "]
pub type AUPLL_POSTDIV_RSTB_R = crate::BitReader;
#[doc = "Field `aupll_postdiv_rstb` writer - "]
pub type AUPLL_POSTDIV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_fbdv_rstb` reader - "]
pub type AUPLL_FBDV_RSTB_R = crate::BitReader;
#[doc = "Field `aupll_fbdv_rstb` writer - "]
pub type AUPLL_FBDV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_refdiv_rstb` reader - "]
pub type AUPLL_REFDIV_RSTB_R = crate::BitReader;
#[doc = "Field `aupll_refdiv_rstb` writer - "]
pub type AUPLL_REFDIV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_postdiv` reader - "]
pub type PU_AUPLL_POSTDIV_R = crate::BitReader;
#[doc = "Field `pu_aupll_postdiv` writer - "]
pub type PU_AUPLL_POSTDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_fbdv` reader - "]
pub type PU_AUPLL_FBDV_R = crate::BitReader;
#[doc = "Field `pu_aupll_fbdv` writer - "]
pub type PU_AUPLL_FBDV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_clamp_op` reader - "]
pub type PU_AUPLL_CLAMP_OP_R = crate::BitReader;
#[doc = "Field `pu_aupll_clamp_op` writer - "]
pub type PU_AUPLL_CLAMP_OP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_pfd` reader - "]
pub type PU_AUPLL_PFD_R = crate::BitReader;
#[doc = "Field `pu_aupll_pfd` writer - "]
pub type PU_AUPLL_PFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_cp` reader - "]
pub type PU_AUPLL_CP_R = crate::BitReader;
#[doc = "Field `pu_aupll_cp` writer - "]
pub type PU_AUPLL_CP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_sfreg` reader - "]
pub type PU_AUPLL_SFREG_R = crate::BitReader;
#[doc = "Field `pu_aupll_sfreg` writer - "]
pub type PU_AUPLL_SFREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll` reader - "]
pub type PU_AUPLL_R = crate::BitReader;
#[doc = "Field `pu_aupll` writer - "]
pub type PU_AUPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_clktree` reader - "]
pub type PU_AUPLL_CLKTREE_R = crate::BitReader;
#[doc = "Field `pu_aupll_clktree` writer - "]
pub type PU_AUPLL_CLKTREE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_sdm_rstb(&self) -> AUPLL_SDM_RSTB_R {
        AUPLL_SDM_RSTB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_postdiv_rstb(&self) -> AUPLL_POSTDIV_RSTB_R {
        AUPLL_POSTDIV_RSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_fbdv_rstb(&self) -> AUPLL_FBDV_RSTB_R {
        AUPLL_FBDV_RSTB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_refdiv_rstb(&self) -> AUPLL_REFDIV_RSTB_R {
        AUPLL_REFDIV_RSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_aupll_postdiv(&self) -> PU_AUPLL_POSTDIV_R {
        PU_AUPLL_POSTDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_aupll_fbdv(&self) -> PU_AUPLL_FBDV_R {
        PU_AUPLL_FBDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_aupll_clamp_op(&self) -> PU_AUPLL_CLAMP_OP_R {
        PU_AUPLL_CLAMP_OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_aupll_pfd(&self) -> PU_AUPLL_PFD_R {
        PU_AUPLL_PFD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_aupll_cp(&self) -> PU_AUPLL_CP_R {
        PU_AUPLL_CP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_aupll_sfreg(&self) -> PU_AUPLL_SFREG_R {
        PU_AUPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_aupll(&self) -> PU_AUPLL_R {
        PU_AUPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_aupll_clktree(&self) -> PU_AUPLL_CLKTREE_R {
        PU_AUPLL_CLKTREE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_rstb(&mut self) -> AUPLL_SDM_RSTB_W<AUDIO_PLL_CFG0_SPEC> {
        AUPLL_SDM_RSTB_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_postdiv_rstb(&mut self) -> AUPLL_POSTDIV_RSTB_W<AUDIO_PLL_CFG0_SPEC> {
        AUPLL_POSTDIV_RSTB_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_fbdv_rstb(&mut self) -> AUPLL_FBDV_RSTB_W<AUDIO_PLL_CFG0_SPEC> {
        AUPLL_FBDV_RSTB_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_refdiv_rstb(&mut self) -> AUPLL_REFDIV_RSTB_W<AUDIO_PLL_CFG0_SPEC> {
        AUPLL_REFDIV_RSTB_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_postdiv(&mut self) -> PU_AUPLL_POSTDIV_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_POSTDIV_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_fbdv(&mut self) -> PU_AUPLL_FBDV_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_FBDV_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_clamp_op(&mut self) -> PU_AUPLL_CLAMP_OP_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_CLAMP_OP_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_pfd(&mut self) -> PU_AUPLL_PFD_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_PFD_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_cp(&mut self) -> PU_AUPLL_CP_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_CP_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_sfreg(&mut self) -> PU_AUPLL_SFREG_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_SFREG_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll(&mut self) -> PU_AUPLL_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aupll_clktree(&mut self) -> PU_AUPLL_CLKTREE_W<AUDIO_PLL_CFG0_SPEC> {
        PU_AUPLL_CLKTREE_W::new(self, 11)
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
#[doc = "audio_pll_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG0_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg0 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
