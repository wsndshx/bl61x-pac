#[doc = "Register `tzc_glb_ctrl_0` reader"]
pub type R = crate::R<TZC_GLB_CTRL_0_SPEC>;
#[doc = "Register `tzc_glb_ctrl_0` writer"]
pub type W = crate::W<TZC_GLB_CTRL_0_SPEC>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` reader - TZSID enable for power-on reset."]
pub type TZC_GLB_PWRON_RST_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` writer - TZSID enable for power-on reset."]
pub type TZC_GLB_PWRON_RST_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` reader - TZSID enable for CPU reset."]
pub type TZC_GLB_CPU_RESET_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` writer - TZSID enable for CPU reset."]
pub type TZC_GLB_CPU_RESET_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` reader - TZSID enable for system reset."]
pub type TZC_GLB_SYS_RESET_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` writer - TZSID enable for system reset."]
pub type TZC_GLB_SYS_RESET_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` reader - TZSID enable for CPU2 reset."]
pub type TZC_GLB_CPU2_RESET_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` writer - TZSID enable for CPU2 reset."]
pub type TZC_GLB_CPU2_RESET_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_misc_tzsid_en` reader - TZSID enable for miscellaneous reset."]
pub type TZC_GLB_MISC_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_misc_tzsid_en` writer - TZSID enable for miscellaneous reset."]
pub type TZC_GLB_MISC_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_sram_tzsid_en` reader - TZSID enable for SRAM reset."]
pub type TZC_GLB_SRAM_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_sram_tzsid_en` writer - TZSID enable for SRAM reset."]
pub type TZC_GLB_SRAM_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_swrst_tzsid_en` reader - TZSID enable for software reset."]
pub type TZC_GLB_SWRST_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_swrst_tzsid_en` writer - TZSID enable for software reset."]
pub type TZC_GLB_SWRST_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_bmx_tzsid_en` reader - TZSID enable for bus matrix reset."]
pub type TZC_GLB_BMX_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_bmx_tzsid_en` writer - TZSID enable for bus matrix reset."]
pub type TZC_GLB_BMX_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_dbg_tzsid_en` reader - TZSID enable for debug reset."]
pub type TZC_GLB_DBG_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_dbg_tzsid_en` writer - TZSID enable for debug reset."]
pub type TZC_GLB_DBG_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_mbist_tzsid_en` reader - TZSID enable for memory bist reset."]
pub type TZC_GLB_MBIST_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_mbist_tzsid_en` writer - TZSID enable for memory bist reset."]
pub type TZC_GLB_MBIST_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_clk_tzsid_en` reader - TZSID enable for clock reset."]
pub type TZC_GLB_CLK_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_clk_tzsid_en` writer - TZSID enable for clock reset."]
pub type TZC_GLB_CLK_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_int_tzsid_en` reader - TZSID enable for interrupt reset."]
pub type TZC_GLB_INT_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_int_tzsid_en` writer - TZSID enable for interrupt reset."]
pub type TZC_GLB_INT_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_pwr_tzsid_en` reader - TZSID enable for power reset."]
pub type TZC_GLB_PWR_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_glb_pwr_tzsid_en` writer - TZSID enable for power reset."]
pub type TZC_GLB_PWR_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TZSID enable for power-on reset."]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_en(&self) -> TZC_GLB_PWRON_RST_TZSID_EN_R {
        TZC_GLB_PWRON_RST_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TZSID enable for CPU reset."]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_en(&self) -> TZC_GLB_CPU_RESET_TZSID_EN_R {
        TZC_GLB_CPU_RESET_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TZSID enable for system reset."]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_en(&self) -> TZC_GLB_SYS_RESET_TZSID_EN_R {
        TZC_GLB_SYS_RESET_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TZSID enable for CPU2 reset."]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&self) -> TZC_GLB_CPU2_RESET_TZSID_EN_R {
        TZC_GLB_CPU2_RESET_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TZSID enable for miscellaneous reset."]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_en(&self) -> TZC_GLB_MISC_TZSID_EN_R {
        TZC_GLB_MISC_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TZSID enable for SRAM reset."]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_en(&self) -> TZC_GLB_SRAM_TZSID_EN_R {
        TZC_GLB_SRAM_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TZSID enable for software reset."]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_en(&self) -> TZC_GLB_SWRST_TZSID_EN_R {
        TZC_GLB_SWRST_TZSID_EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - TZSID enable for bus matrix reset."]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_en(&self) -> TZC_GLB_BMX_TZSID_EN_R {
        TZC_GLB_BMX_TZSID_EN_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - TZSID enable for debug reset."]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_en(&self) -> TZC_GLB_DBG_TZSID_EN_R {
        TZC_GLB_DBG_TZSID_EN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TZSID enable for memory bist reset."]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_en(&self) -> TZC_GLB_MBIST_TZSID_EN_R {
        TZC_GLB_MBIST_TZSID_EN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - TZSID enable for clock reset."]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_en(&self) -> TZC_GLB_CLK_TZSID_EN_R {
        TZC_GLB_CLK_TZSID_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - TZSID enable for interrupt reset."]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_en(&self) -> TZC_GLB_INT_TZSID_EN_R {
        TZC_GLB_INT_TZSID_EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TZSID enable for power reset."]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_en(&self) -> TZC_GLB_PWR_TZSID_EN_R {
        TZC_GLB_PWR_TZSID_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TZSID enable for power-on reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwron_rst_tzsid_en(
        &mut self,
    ) -> TZC_GLB_PWRON_RST_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_PWRON_RST_TZSID_EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TZSID enable for CPU reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu_reset_tzsid_en(
        &mut self,
    ) -> TZC_GLB_CPU_RESET_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_CPU_RESET_TZSID_EN_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TZSID enable for system reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sys_reset_tzsid_en(
        &mut self,
    ) -> TZC_GLB_SYS_RESET_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_SYS_RESET_TZSID_EN_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TZSID enable for CPU2 reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu2_reset_tzsid_en(
        &mut self,
    ) -> TZC_GLB_CPU2_RESET_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_CPU2_RESET_TZSID_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - TZSID enable for miscellaneous reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_misc_tzsid_en(&mut self) -> TZC_GLB_MISC_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_MISC_TZSID_EN_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TZSID enable for SRAM reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sram_tzsid_en(&mut self) -> TZC_GLB_SRAM_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_SRAM_TZSID_EN_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TZSID enable for software reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_swrst_tzsid_en(&mut self) -> TZC_GLB_SWRST_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_SWRST_TZSID_EN_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - TZSID enable for bus matrix reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_bmx_tzsid_en(&mut self) -> TZC_GLB_BMX_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_BMX_TZSID_EN_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - TZSID enable for debug reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_dbg_tzsid_en(&mut self) -> TZC_GLB_DBG_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_DBG_TZSID_EN_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - TZSID enable for memory bist reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_mbist_tzsid_en(&mut self) -> TZC_GLB_MBIST_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_MBIST_TZSID_EN_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - TZSID enable for clock reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_clk_tzsid_en(&mut self) -> TZC_GLB_CLK_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_CLK_TZSID_EN_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - TZSID enable for interrupt reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_int_tzsid_en(&mut self) -> TZC_GLB_INT_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_INT_TZSID_EN_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - TZSID enable for power reset."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwr_tzsid_en(&mut self) -> TZC_GLB_PWR_TZSID_EN_W<TZC_GLB_CTRL_0_SPEC> {
        TZC_GLB_PWR_TZSID_EN_W::new(self, 24)
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
#[doc = "TrustZone Controller Global Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_glb_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_glb_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_GLB_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_0 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
