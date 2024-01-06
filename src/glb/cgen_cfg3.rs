#[doc = "Register `cgen_cfg3` reader"]
pub type R = crate::R<CGEN_CFG3_SPEC>;
#[doc = "Register `cgen_cfg3` writer"]
pub type W = crate::W<CGEN_CFG3_SPEC>;
#[doc = "Field `cgen_isp_wifipll_80m` reader - "]
pub type CGEN_ISP_WIFIPLL_80M_R = crate::BitReader;
#[doc = "Field `cgen_isp_wifipll_80m` writer - "]
pub type CGEN_ISP_WIFIPLL_80M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_isp_aupll_div5` reader - "]
pub type CGEN_ISP_AUPLL_DIV5_R = crate::BitReader;
#[doc = "Field `cgen_isp_aupll_div5` writer - "]
pub type CGEN_ISP_AUPLL_DIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_isp_aupll_div6` reader - "]
pub type CGEN_ISP_AUPLL_DIV6_R = crate::BitReader;
#[doc = "Field `cgen_isp_aupll_div6` writer - "]
pub type CGEN_ISP_AUPLL_DIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_aupll_div5` reader - "]
pub type CGEN_TOP_AUPLL_DIV5_R = crate::BitReader;
#[doc = "Field `cgen_top_aupll_div5` writer - "]
pub type CGEN_TOP_AUPLL_DIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_aupll_div6` reader - "]
pub type CGEN_TOP_AUPLL_DIV6_R = crate::BitReader;
#[doc = "Field `cgen_top_aupll_div6` writer - "]
pub type CGEN_TOP_AUPLL_DIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_psramB_wifipll_320m` reader - "]
pub type CGEN_PSRAM_B_WIFIPLL_320M_R = crate::BitReader;
#[doc = "Field `cgen_psramB_wifipll_320m` writer - "]
pub type CGEN_PSRAM_B_WIFIPLL_320M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_psramB_aupll_div1` reader - "]
pub type CGEN_PSRAM_B_AUPLL_DIV1_R = crate::BitReader;
#[doc = "Field `cgen_psramB_aupll_div1` writer - "]
pub type CGEN_PSRAM_B_AUPLL_DIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_wifipll_240m` reader - "]
pub type CGEN_TOP_WIFIPLL_240M_R = crate::BitReader;
#[doc = "Field `cgen_top_wifipll_240m` writer - "]
pub type CGEN_TOP_WIFIPLL_240M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_wifipll_320m` reader - "]
pub type CGEN_TOP_WIFIPLL_320M_R = crate::BitReader;
#[doc = "Field `cgen_top_wifipll_320m` writer - "]
pub type CGEN_TOP_WIFIPLL_320M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_aupll_div2` reader - "]
pub type CGEN_TOP_AUPLL_DIV2_R = crate::BitReader;
#[doc = "Field `cgen_top_aupll_div2` writer - "]
pub type CGEN_TOP_AUPLL_DIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_aupll_div1` reader - "]
pub type CGEN_TOP_AUPLL_DIV1_R = crate::BitReader;
#[doc = "Field `cgen_top_aupll_div1` writer - "]
pub type CGEN_TOP_AUPLL_DIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_isp_wifipll_80m(&self) -> CGEN_ISP_WIFIPLL_80M_R {
        CGEN_ISP_WIFIPLL_80M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_isp_aupll_div5(&self) -> CGEN_ISP_AUPLL_DIV5_R {
        CGEN_ISP_AUPLL_DIV5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_isp_aupll_div6(&self) -> CGEN_ISP_AUPLL_DIV6_R {
        CGEN_ISP_AUPLL_DIV6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_top_aupll_div5(&self) -> CGEN_TOP_AUPLL_DIV5_R {
        CGEN_TOP_AUPLL_DIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cgen_top_aupll_div6(&self) -> CGEN_TOP_AUPLL_DIV6_R {
        CGEN_TOP_AUPLL_DIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_psram_b_wifipll_320m(&self) -> CGEN_PSRAM_B_WIFIPLL_320M_R {
        CGEN_PSRAM_B_WIFIPLL_320M_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cgen_psram_b_aupll_div1(&self) -> CGEN_PSRAM_B_AUPLL_DIV1_R {
        CGEN_PSRAM_B_AUPLL_DIV1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cgen_top_wifipll_240m(&self) -> CGEN_TOP_WIFIPLL_240M_R {
        CGEN_TOP_WIFIPLL_240M_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cgen_top_wifipll_320m(&self) -> CGEN_TOP_WIFIPLL_320M_R {
        CGEN_TOP_WIFIPLL_320M_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cgen_top_aupll_div2(&self) -> CGEN_TOP_AUPLL_DIV2_R {
        CGEN_TOP_AUPLL_DIV2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_top_aupll_div1(&self) -> CGEN_TOP_AUPLL_DIV1_R {
        CGEN_TOP_AUPLL_DIV1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_isp_wifipll_80m(&mut self) -> CGEN_ISP_WIFIPLL_80M_W<CGEN_CFG3_SPEC> {
        CGEN_ISP_WIFIPLL_80M_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_isp_aupll_div5(&mut self) -> CGEN_ISP_AUPLL_DIV5_W<CGEN_CFG3_SPEC> {
        CGEN_ISP_AUPLL_DIV5_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_isp_aupll_div6(&mut self) -> CGEN_ISP_AUPLL_DIV6_W<CGEN_CFG3_SPEC> {
        CGEN_ISP_AUPLL_DIV6_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div5(&mut self) -> CGEN_TOP_AUPLL_DIV5_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_AUPLL_DIV5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div6(&mut self) -> CGEN_TOP_AUPLL_DIV6_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_AUPLL_DIV6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_psram_b_wifipll_320m(&mut self) -> CGEN_PSRAM_B_WIFIPLL_320M_W<CGEN_CFG3_SPEC> {
        CGEN_PSRAM_B_WIFIPLL_320M_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_psram_b_aupll_div1(&mut self) -> CGEN_PSRAM_B_AUPLL_DIV1_W<CGEN_CFG3_SPEC> {
        CGEN_PSRAM_B_AUPLL_DIV1_W::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_wifipll_240m(&mut self) -> CGEN_TOP_WIFIPLL_240M_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_WIFIPLL_240M_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_wifipll_320m(&mut self) -> CGEN_TOP_WIFIPLL_320M_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_WIFIPLL_320M_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div2(&mut self) -> CGEN_TOP_AUPLL_DIV2_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_AUPLL_DIV2_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div1(&mut self) -> CGEN_TOP_AUPLL_DIV1_W<CGEN_CFG3_SPEC> {
        CGEN_TOP_AUPLL_DIV1_W::new(self, 16)
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
#[doc = "cgen_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGEN_CFG3_SPEC;
impl crate::RegisterSpec for CGEN_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg3::R`](R) reader structure"]
impl crate::Readable for CGEN_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg3::W`](W) writer structure"]
impl crate::Writable for CGEN_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg3 to value 0"]
impl crate::Resettable for CGEN_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
