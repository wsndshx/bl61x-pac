#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub type R = crate::R<TZC_GLB_CTRL_2_SPEC>;
#[doc = "Register `tzc_glb_ctrl_2` writer"]
pub type W = crate::W<TZC_GLB_CTRL_2_SPEC>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` reader - Lock for TZSID enable in power-on reset."]
pub type TZC_GLB_PWRON_RST_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` writer - Lock for TZSID enable in power-on reset."]
pub type TZC_GLB_PWRON_RST_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` reader - Lock for TZSID enable in CPU reset."]
pub type TZC_GLB_CPU_RESET_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` writer - Lock for TZSID enable in CPU reset."]
pub type TZC_GLB_CPU_RESET_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` reader - Lock for TZSID enable in system reset."]
pub type TZC_GLB_SYS_RESET_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` writer - Lock for TZSID enable in system reset."]
pub type TZC_GLB_SYS_RESET_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` reader - Lock for TZSID enable in CPU2 reset."]
pub type TZC_GLB_CPU2_RESET_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` writer - Lock for TZSID enable in CPU2 reset."]
pub type TZC_GLB_CPU2_RESET_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_misc_tzsid_lock` reader - Lock for TZSID enable in miscellaneous reset."]
pub type TZC_GLB_MISC_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_misc_tzsid_lock` writer - Lock for TZSID enable in miscellaneous reset."]
pub type TZC_GLB_MISC_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sram_tzsid_lock` reader - Lock for TZSID enable in SRAM reset."]
pub type TZC_GLB_SRAM_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_sram_tzsid_lock` writer - Lock for TZSID enable in SRAM reset."]
pub type TZC_GLB_SRAM_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_tzsid_lock` reader - Lock for TZSID enable in software reset."]
pub type TZC_GLB_SWRST_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_tzsid_lock` writer - Lock for TZSID enable in software reset."]
pub type TZC_GLB_SWRST_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_bmx_tzsid_lock` reader - Lock for TZSID enable in bus matrix reset."]
pub type TZC_GLB_BMX_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_tzsid_lock` writer - Lock for TZSID enable in bus matrix reset."]
pub type TZC_GLB_BMX_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_dbg_tzsid_lock` reader - Lock for TZSID enable in debug reset."]
pub type TZC_GLB_DBG_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_tzsid_lock` writer - Lock for TZSID enable in debug reset."]
pub type TZC_GLB_DBG_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_mbist_tzsid_lock` reader - Lock for TZSID enable in memory bist reset."]
pub type TZC_GLB_MBIST_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_tzsid_lock` writer - Lock for TZSID enable in memory bist reset."]
pub type TZC_GLB_MBIST_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_clk_tzsid_lock` reader - Lock for TZSID enable in clock reset."]
pub type TZC_GLB_CLK_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_clk_tzsid_lock` writer - Lock for TZSID enable in clock reset."]
pub type TZC_GLB_CLK_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_int_tzsid_lock` reader - Lock for TZSID enable in interrupt reset."]
pub type TZC_GLB_INT_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_int_tzsid_lock` writer - Lock for TZSID enable in interrupt reset."]
pub type TZC_GLB_INT_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_pwr_tzsid_lock` reader - Lock for TZSID enable in power reset."]
pub type TZC_GLB_PWR_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_glb_pwr_tzsid_lock` writer - Lock for TZSID enable in power reset."]
pub type TZC_GLB_PWR_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock for TZSID enable in power-on reset."]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_lock(&self) -> TZC_GLB_PWRON_RST_TZSID_LOCK_R {
        TZC_GLB_PWRON_RST_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock for TZSID enable in CPU reset."]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_lock(&self) -> TZC_GLB_CPU_RESET_TZSID_LOCK_R {
        TZC_GLB_CPU_RESET_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock for TZSID enable in system reset."]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_lock(&self) -> TZC_GLB_SYS_RESET_TZSID_LOCK_R {
        TZC_GLB_SYS_RESET_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock for TZSID enable in CPU2 reset."]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(&self) -> TZC_GLB_CPU2_RESET_TZSID_LOCK_R {
        TZC_GLB_CPU2_RESET_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock for TZSID enable in miscellaneous reset."]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_lock(&self) -> TZC_GLB_MISC_TZSID_LOCK_R {
        TZC_GLB_MISC_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock for TZSID enable in SRAM reset."]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_lock(&self) -> TZC_GLB_SRAM_TZSID_LOCK_R {
        TZC_GLB_SRAM_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock for TZSID enable in software reset."]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_lock(&self) -> TZC_GLB_SWRST_TZSID_LOCK_R {
        TZC_GLB_SWRST_TZSID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock for TZSID enable in bus matrix reset."]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_lock(&self) -> TZC_GLB_BMX_TZSID_LOCK_R {
        TZC_GLB_BMX_TZSID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock for TZSID enable in debug reset."]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_lock(&self) -> TZC_GLB_DBG_TZSID_LOCK_R {
        TZC_GLB_DBG_TZSID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock for TZSID enable in memory bist reset."]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_lock(&self) -> TZC_GLB_MBIST_TZSID_LOCK_R {
        TZC_GLB_MBIST_TZSID_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock for TZSID enable in clock reset."]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_lock(&self) -> TZC_GLB_CLK_TZSID_LOCK_R {
        TZC_GLB_CLK_TZSID_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock for TZSID enable in interrupt reset."]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_lock(&self) -> TZC_GLB_INT_TZSID_LOCK_R {
        TZC_GLB_INT_TZSID_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock for TZSID enable in power reset."]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_lock(&self) -> TZC_GLB_PWR_TZSID_LOCK_R {
        TZC_GLB_PWR_TZSID_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock for TZSID enable in power-on reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwron_rst_tzsid_lock(
        &mut self,
    ) -> TZC_GLB_PWRON_RST_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_PWRON_RST_TZSID_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock for TZSID enable in CPU reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu_reset_tzsid_lock(
        &mut self,
    ) -> TZC_GLB_CPU_RESET_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_CPU_RESET_TZSID_LOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Lock for TZSID enable in system reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sys_reset_tzsid_lock(
        &mut self,
    ) -> TZC_GLB_SYS_RESET_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_SYS_RESET_TZSID_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Lock for TZSID enable in CPU2 reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(
        &mut self,
    ) -> TZC_GLB_CPU2_RESET_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_CPU2_RESET_TZSID_LOCK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Lock for TZSID enable in miscellaneous reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_misc_tzsid_lock(&mut self) -> TZC_GLB_MISC_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_MISC_TZSID_LOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Lock for TZSID enable in SRAM reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sram_tzsid_lock(&mut self) -> TZC_GLB_SRAM_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_SRAM_TZSID_LOCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Lock for TZSID enable in software reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_swrst_tzsid_lock(&mut self) -> TZC_GLB_SWRST_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_SWRST_TZSID_LOCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Lock for TZSID enable in bus matrix reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_bmx_tzsid_lock(&mut self) -> TZC_GLB_BMX_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_BMX_TZSID_LOCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Lock for TZSID enable in debug reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_dbg_tzsid_lock(&mut self) -> TZC_GLB_DBG_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_DBG_TZSID_LOCK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Lock for TZSID enable in memory bist reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_mbist_tzsid_lock(&mut self) -> TZC_GLB_MBIST_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_MBIST_TZSID_LOCK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Lock for TZSID enable in clock reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_clk_tzsid_lock(&mut self) -> TZC_GLB_CLK_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_CLK_TZSID_LOCK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Lock for TZSID enable in interrupt reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_int_tzsid_lock(&mut self) -> TZC_GLB_INT_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_INT_TZSID_LOCK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Lock for TZSID enable in power reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwr_tzsid_lock(&mut self) -> TZC_GLB_PWR_TZSID_LOCK_W<TZC_GLB_CTRL_2_SPEC> {
        TZC_GLB_PWR_TZSID_LOCK_W::new(self, 12)
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
#[doc = "TrustZone Controller Global Control 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_glb_ctrl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_glb_ctrl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_GLB_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
