#[doc = "Register `wifi_pll_config_0` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_0_SPEC>;
#[doc = "Register `wifi_pll_config_0` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_0_SPEC>;
#[doc = "Field `wifipll_sdm_rstb` reader - "]
pub type WIFIPLL_SDM_RSTB_R = crate::BitReader;
#[doc = "Field `wifipll_sdm_rstb` writer - "]
pub type WIFIPLL_SDM_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_postdiv_rstb` reader - "]
pub type WIFIPLL_POSTDIV_RSTB_R = crate::BitReader;
#[doc = "Field `wifipll_postdiv_rstb` writer - "]
pub type WIFIPLL_POSTDIV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_fbdv_rstb` reader - "]
pub type WIFIPLL_FBDV_RSTB_R = crate::BitReader;
#[doc = "Field `wifipll_fbdv_rstb` writer - "]
pub type WIFIPLL_FBDV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_refdiv_rstb` reader - "]
pub type WIFIPLL_REFDIV_RSTB_R = crate::BitReader;
#[doc = "Field `wifipll_refdiv_rstb` writer - "]
pub type WIFIPLL_REFDIV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_clktree` reader - "]
pub type PU_WIFIPLL_CLKTREE_R = crate::BitReader;
#[doc = "Field `pu_wifipll_clktree` writer - "]
pub type PU_WIFIPLL_CLKTREE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_postdiv` reader - "]
pub type PU_WIFIPLL_POSTDIV_R = crate::BitReader;
#[doc = "Field `pu_wifipll_postdiv` writer - "]
pub type PU_WIFIPLL_POSTDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_fbdv` reader - "]
pub type PU_WIFIPLL_FBDV_R = crate::BitReader;
#[doc = "Field `pu_wifipll_fbdv` writer - "]
pub type PU_WIFIPLL_FBDV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_clamp_op` reader - "]
pub type PU_WIFIPLL_CLAMP_OP_R = crate::BitReader;
#[doc = "Field `pu_wifipll_clamp_op` writer - "]
pub type PU_WIFIPLL_CLAMP_OP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_pfd` reader - "]
pub type PU_WIFIPLL_PFD_R = crate::BitReader;
#[doc = "Field `pu_wifipll_pfd` writer - "]
pub type PU_WIFIPLL_PFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_cp` reader - "]
pub type PU_WIFIPLL_CP_R = crate::BitReader;
#[doc = "Field `pu_wifipll_cp` writer - "]
pub type PU_WIFIPLL_CP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_sfreg` reader - "]
pub type PU_WIFIPLL_SFREG_R = crate::BitReader;
#[doc = "Field `pu_wifipll_sfreg` writer - "]
pub type PU_WIFIPLL_SFREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll` reader - "]
pub type PU_WIFIPLL_R = crate::BitReader;
#[doc = "Field `pu_wifipll` writer - "]
pub type PU_WIFIPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sdm_rstb(&self) -> WIFIPLL_SDM_RSTB_R {
        WIFIPLL_SDM_RSTB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_postdiv_rstb(&self) -> WIFIPLL_POSTDIV_RSTB_R {
        WIFIPLL_POSTDIV_RSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_fbdv_rstb(&self) -> WIFIPLL_FBDV_RSTB_R {
        WIFIPLL_FBDV_RSTB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_refdiv_rstb(&self) -> WIFIPLL_REFDIV_RSTB_R {
        WIFIPLL_REFDIV_RSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_wifipll_clktree(&self) -> PU_WIFIPLL_CLKTREE_R {
        PU_WIFIPLL_CLKTREE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_wifipll_postdiv(&self) -> PU_WIFIPLL_POSTDIV_R {
        PU_WIFIPLL_POSTDIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_wifipll_fbdv(&self) -> PU_WIFIPLL_FBDV_R {
        PU_WIFIPLL_FBDV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_wifipll_clamp_op(&self) -> PU_WIFIPLL_CLAMP_OP_R {
        PU_WIFIPLL_CLAMP_OP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_wifipll_pfd(&self) -> PU_WIFIPLL_PFD_R {
        PU_WIFIPLL_PFD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_wifipll_cp(&self) -> PU_WIFIPLL_CP_R {
        PU_WIFIPLL_CP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_wifipll_sfreg(&self) -> PU_WIFIPLL_SFREG_R {
        PU_WIFIPLL_SFREG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_wifipll(&self) -> PU_WIFIPLL_R {
        PU_WIFIPLL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_rstb(&mut self) -> WIFIPLL_SDM_RSTB_W<WIFI_PLL_CONFIG_0_SPEC> {
        WIFIPLL_SDM_RSTB_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_postdiv_rstb(&mut self) -> WIFIPLL_POSTDIV_RSTB_W<WIFI_PLL_CONFIG_0_SPEC> {
        WIFIPLL_POSTDIV_RSTB_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_fbdv_rstb(&mut self) -> WIFIPLL_FBDV_RSTB_W<WIFI_PLL_CONFIG_0_SPEC> {
        WIFIPLL_FBDV_RSTB_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_refdiv_rstb(&mut self) -> WIFIPLL_REFDIV_RSTB_W<WIFI_PLL_CONFIG_0_SPEC> {
        WIFIPLL_REFDIV_RSTB_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_clktree(&mut self) -> PU_WIFIPLL_CLKTREE_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_CLKTREE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_postdiv(&mut self) -> PU_WIFIPLL_POSTDIV_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_POSTDIV_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_fbdv(&mut self) -> PU_WIFIPLL_FBDV_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_FBDV_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_clamp_op(&mut self) -> PU_WIFIPLL_CLAMP_OP_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_CLAMP_OP_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_pfd(&mut self) -> PU_WIFIPLL_PFD_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_PFD_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_cp(&mut self) -> PU_WIFIPLL_CP_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_CP_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_sfreg(&mut self) -> PU_WIFIPLL_SFREG_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_SFREG_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll(&mut self) -> PU_WIFIPLL_W<WIFI_PLL_CONFIG_0_SPEC> {
        PU_WIFIPLL_W::new(self, 11)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_0_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_0::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_0::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_0 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
