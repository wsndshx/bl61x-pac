#[doc = "Register `permit_config` reader"]
pub type R = crate::R<PERMIT_CONFIG_SPEC>;
#[doc = "Register `permit_config` writer"]
pub type W = crate::W<PERMIT_CONFIG_SPEC>;
#[doc = "Field `tzc_glb_pwron_rst_lock` reader - "]
pub type TZC_GLB_PWRON_RST_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_pwron_rst_lock` writer - "]
pub type TZC_GLB_PWRON_RST_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu_reset_lock` reader - "]
pub type TZC_GLB_CPU_RESET_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_cpu_reset_lock` writer - "]
pub type TZC_GLB_CPU_RESET_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sys_reset_lock` reader - "]
pub type TZC_GLB_SYS_RESET_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_sys_reset_lock` writer - "]
pub type TZC_GLB_SYS_RESET_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu2_reset_lock` reader - "]
pub type TZC_GLB_CPU2_RESET_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_cpu2_reset_lock` writer - "]
pub type TZC_GLB_CPU2_RESET_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_pwr_lock` reader - "]
pub type TZC_GLB_PWR_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_pwr_lock` writer - "]
pub type TZC_GLB_PWR_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_int_lock` reader - "]
pub type TZC_GLB_INT_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_int_lock` writer - "]
pub type TZC_GLB_INT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpupll_lock` reader - "]
pub type TZC_GLB_CPUPLL_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_cpupll_lock` writer - "]
pub type TZC_GLB_CPUPLL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_misc_lock` reader - "]
pub type TZC_GLB_MISC_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_misc_lock` writer - "]
pub type TZC_GLB_MISC_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sram_lock` reader - "]
pub type TZC_GLB_SRAM_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_sram_lock` writer - "]
pub type TZC_GLB_SRAM_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_lock` reader - "]
pub type TZC_GLB_SWRST_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_lock` writer - "]
pub type TZC_GLB_SWRST_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_bmx_lock` reader - "]
pub type TZC_GLB_BMX_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_lock` writer - "]
pub type TZC_GLB_BMX_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_dbg_lock` reader - "]
pub type TZC_GLB_DBG_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_lock` writer - "]
pub type TZC_GLB_DBG_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_mbist_lock` reader - "]
pub type TZC_GLB_MBIST_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_lock` writer - "]
pub type TZC_GLB_MBIST_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_clk_lock` reader - "]
pub type TZC_GLB_CLK_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_clk_lock` writer - "]
pub type TZC_GLB_CLK_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_lock(&self) -> TZC_GLB_PWRON_RST_LOCK_R {
        TZC_GLB_PWRON_RST_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_lock(&self) -> TZC_GLB_CPU_RESET_LOCK_R {
        TZC_GLB_CPU_RESET_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_lock(&self) -> TZC_GLB_SYS_RESET_LOCK_R {
        TZC_GLB_SYS_RESET_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_lock(&self) -> TZC_GLB_CPU2_RESET_LOCK_R {
        TZC_GLB_CPU2_RESET_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_pwr_lock(&self) -> TZC_GLB_PWR_LOCK_R {
        TZC_GLB_PWR_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_int_lock(&self) -> TZC_GLB_INT_LOCK_R {
        TZC_GLB_INT_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_cpupll_lock(&self) -> TZC_GLB_CPUPLL_LOCK_R {
        TZC_GLB_CPUPLL_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&self) -> TZC_GLB_MISC_LOCK_R {
        TZC_GLB_MISC_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&self) -> TZC_GLB_SRAM_LOCK_R {
        TZC_GLB_SRAM_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_lock(&self) -> TZC_GLB_SWRST_LOCK_R {
        TZC_GLB_SWRST_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&self) -> TZC_GLB_BMX_LOCK_R {
        TZC_GLB_BMX_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&self) -> TZC_GLB_DBG_LOCK_R {
        TZC_GLB_DBG_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&self) -> TZC_GLB_MBIST_LOCK_R {
        TZC_GLB_MBIST_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&self) -> TZC_GLB_CLK_LOCK_R {
        TZC_GLB_CLK_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwron_rst_lock(&mut self) -> TZC_GLB_PWRON_RST_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_PWRON_RST_LOCK_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu_reset_lock(&mut self) -> TZC_GLB_CPU_RESET_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_CPU_RESET_LOCK_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sys_reset_lock(&mut self) -> TZC_GLB_SYS_RESET_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_SYS_RESET_LOCK_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu2_reset_lock(&mut self) -> TZC_GLB_CPU2_RESET_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_CPU2_RESET_LOCK_W::new(self, 15)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwr_lock(&mut self) -> TZC_GLB_PWR_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_PWR_LOCK_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_int_lock(&mut self) -> TZC_GLB_INT_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_INT_LOCK_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpupll_lock(&mut self) -> TZC_GLB_CPUPLL_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_CPUPLL_LOCK_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_misc_lock(&mut self) -> TZC_GLB_MISC_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_MISC_LOCK_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sram_lock(&mut self) -> TZC_GLB_SRAM_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_SRAM_LOCK_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_swrst_lock(&mut self) -> TZC_GLB_SWRST_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_SWRST_LOCK_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_bmx_lock(&mut self) -> TZC_GLB_BMX_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_BMX_LOCK_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_dbg_lock(&mut self) -> TZC_GLB_DBG_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_DBG_LOCK_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_mbist_lock(&mut self) -> TZC_GLB_MBIST_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_MBIST_LOCK_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_clk_lock(&mut self) -> TZC_GLB_CLK_LOCK_W<PERMIT_CONFIG_SPEC> {
        TZC_GLB_CLK_LOCK_W::new(self, 31)
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
#[doc = "Permission control peripheral configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`permit_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`permit_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERMIT_CONFIG_SPEC;
impl crate::RegisterSpec for PERMIT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`permit_config::R`](R) reader structure"]
impl crate::Readable for PERMIT_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`permit_config::W`](W) writer structure"]
impl crate::Writable for PERMIT_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets permit_config to value 0"]
impl crate::Resettable for PERMIT_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
