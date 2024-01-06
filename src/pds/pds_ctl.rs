#[doc = "Register `PDS_CTL` reader"]
pub type R = crate::R<PDS_CTL_SPEC>;
#[doc = "Register `PDS_CTL` writer"]
pub type W = crate::W<PDS_CTL_SPEC>;
#[doc = "Field `pds_start_ps` reader - "]
pub type PDS_START_PS_R = crate::BitReader;
#[doc = "Field `pds_start_ps` writer - "]
pub type PDS_START_PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_sleep_forever` reader - "]
pub type CR_SLEEP_FOREVER_R = crate::BitReader;
#[doc = "Field `cr_sleep_forever` writer - "]
pub type CR_SLEEP_FOREVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_xtal_force_off` reader - "]
pub type CR_XTAL_FORCE_OFF_R = crate::BitReader;
#[doc = "Field `cr_xtal_force_off` writer - "]
pub type CR_XTAL_FORCE_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_save_state` reader - "]
pub type CR_PDS_WIFI_SAVE_STATE_R = crate::BitReader;
#[doc = "Field `cr_pds_wifi_save_state` writer - "]
pub type CR_PDS_WIFI_SAVE_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_ldo11` reader - "]
pub type CR_PDS_PD_LDO11_R = crate::BitReader;
#[doc = "Field `cr_pds_pd_ldo11` writer - "]
pub type CR_PDS_PD_LDO11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub type CR_PDS_PD_BG_SYS_R = crate::BitReader;
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub type CR_PDS_PD_BG_SYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_gpio_ie_pu_pd` reader - "]
pub type CR_PDS_CTRL_GPIO_IE_PU_PD_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_gpio_ie_pu_pd` writer - "]
pub type CR_PDS_CTRL_GPIO_IE_PU_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_dcdc18` reader - "]
pub type CR_PDS_PD_DCDC18_R = crate::BitReader;
#[doc = "Field `cr_pds_pd_dcdc18` writer - "]
pub type CR_PDS_PD_DCDC18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gate_clk` reader - "]
pub type CR_PDS_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_gate_clk` writer - "]
pub type CR_PDS_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mem_stby` reader - "]
pub type CR_PDS_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_mem_stby` writer - "]
pub type CR_PDS_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_glb_reg_reset_protect` reader - "]
pub type CR_PDS_GLB_REG_RESET_PROTECT_R = crate::BitReader;
#[doc = "Field `cr_pds_glb_reg_reset_protect` writer - "]
pub type CR_PDS_GLB_REG_RESET_PROTECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_iso_en` reader - "]
pub type CR_PDS_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_iso_en` writer - "]
pub type CR_PDS_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wait_xtal_rdy` reader - "]
pub type CR_PDS_WAIT_XTAL_RDY_R = crate::BitReader;
#[doc = "Field `cr_pds_wait_xtal_rdy` writer - "]
pub type CR_PDS_WAIT_XTAL_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pwr_off` reader - "]
pub type CR_PDS_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_pwr_off` writer - "]
pub type CR_PDS_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_xtal` reader - "]
pub type CR_PDS_PD_XTAL_R = crate::BitReader;
#[doc = "Field `cr_pds_pd_xtal` writer - "]
pub type CR_PDS_PD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_soc_enb` reader - "]
pub type CR_PDS_CTRL_SOC_ENB_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_soc_enb` writer - "]
pub type CR_PDS_CTRL_SOC_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rst_soc` reader - "]
pub type CR_PDS_RST_SOC_R = crate::BitReader;
#[doc = "Field `cr_pds_rst_soc` writer - "]
pub type CR_PDS_RST_SOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rc32m_off_dis` reader - "]
pub type CR_PDS_RC32M_OFF_DIS_R = crate::BitReader;
#[doc = "Field `cr_pds_rc32m_off_dis` writer - "]
pub type CR_PDS_RC32M_OFF_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ldo11_vsel_en` reader - "]
pub type CR_PDS_LDO11_VSEL_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_ldo11_vsel_en` writer - "]
pub type CR_PDS_LDO11_VSEL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usbpll_pd` reader - "]
pub type CR_PDS_CTRL_USBPLL_PD_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usbpll_pd` writer - "]
pub type CR_PDS_CTRL_USBPLL_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_aupll_pd` reader - "]
pub type CR_PDS_CTRL_AUPLL_PD_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_aupll_pd` writer - "]
pub type CR_PDS_CTRL_AUPLL_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_wifipll_pd` reader - "]
pub type CR_PDS_CTRL_WIFIPLL_PD_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_wifipll_pd` writer - "]
pub type CR_PDS_CTRL_WIFIPLL_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ldo11_vol` reader - "]
pub type CR_PDS_LDO11_VOL_R = crate::FieldReader;
#[doc = "Field `cr_pds_ldo11_vol` writer - "]
pub type CR_PDS_LDO11_VOL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cr_pds_ctrl_rf` reader - "]
pub type CR_PDS_CTRL_RF_R = crate::FieldReader;
#[doc = "Field `cr_pds_ctrl_rf` writer - "]
pub type CR_PDS_CTRL_RF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_pds_start_use_tbtt_sleep` reader - "]
pub type CR_PDS_START_USE_TBTT_SLEEP_R = crate::BitReader;
#[doc = "Field `cr_pds_start_use_tbtt_sleep` writer - "]
pub type CR_PDS_START_USE_TBTT_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gpio_iso_mode` reader - "]
pub type CR_PDS_GPIO_ISO_MODE_R = crate::BitReader;
#[doc = "Field `cr_pds_gpio_iso_mode` writer - "]
pub type CR_PDS_GPIO_ISO_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&self) -> PDS_START_PS_R {
        PDS_START_PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CR_SLEEP_FOREVER_R {
        CR_SLEEP_FOREVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CR_XTAL_FORCE_OFF_R {
        CR_XTAL_FORCE_OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_wifi_save_state(&self) -> CR_PDS_WIFI_SAVE_STATE_R {
        CR_PDS_WIFI_SAVE_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&self) -> CR_PDS_PD_LDO11_R {
        CR_PDS_PD_LDO11_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CR_PDS_PD_BG_SYS_R {
        CR_PDS_PD_BG_SYS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_ctrl_gpio_ie_pu_pd(&self) -> CR_PDS_CTRL_GPIO_IE_PU_PD_R {
        CR_PDS_CTRL_GPIO_IE_PU_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&self) -> CR_PDS_PD_DCDC18_R {
        CR_PDS_PD_DCDC18_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&self) -> CR_PDS_GATE_CLK_R {
        CR_PDS_GATE_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&self) -> CR_PDS_MEM_STBY_R {
        CR_PDS_MEM_STBY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_glb_reg_reset_protect(&self) -> CR_PDS_GLB_REG_RESET_PROTECT_R {
        CR_PDS_GLB_REG_RESET_PROTECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&self) -> CR_PDS_ISO_EN_R {
        CR_PDS_ISO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&self) -> CR_PDS_WAIT_XTAL_RDY_R {
        CR_PDS_WAIT_XTAL_RDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&self) -> CR_PDS_PWR_OFF_R {
        CR_PDS_PWR_OFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&self) -> CR_PDS_PD_XTAL_R {
        CR_PDS_PD_XTAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_ctrl_soc_enb(&self) -> CR_PDS_CTRL_SOC_ENB_R {
        CR_PDS_CTRL_SOC_ENB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc(&self) -> CR_PDS_RST_SOC_R {
        CR_PDS_RST_SOC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&self) -> CR_PDS_RC32M_OFF_DIS_R {
        CR_PDS_RC32M_OFF_DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo11_vsel_en(&self) -> CR_PDS_LDO11_VSEL_EN_R {
        CR_PDS_LDO11_VSEL_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usbpll_pd(&self) -> CR_PDS_CTRL_USBPLL_PD_R {
        CR_PDS_CTRL_USBPLL_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_ctrl_aupll_pd(&self) -> CR_PDS_CTRL_AUPLL_PD_R {
        CR_PDS_CTRL_AUPLL_PD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wifipll_pd(&self) -> CR_PDS_CTRL_WIFIPLL_PD_R {
        CR_PDS_CTRL_WIFIPLL_PD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn cr_pds_ldo11_vol(&self) -> CR_PDS_LDO11_VOL_R {
        CR_PDS_LDO11_VOL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&self) -> CR_PDS_CTRL_RF_R {
        CR_PDS_CTRL_RF_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_start_use_tbtt_sleep(&self) -> CR_PDS_START_USE_TBTT_SLEEP_R {
        CR_PDS_START_USE_TBTT_SLEEP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_gpio_iso_mode(&self) -> CR_PDS_GPIO_ISO_MODE_R {
        CR_PDS_GPIO_ISO_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pds_start_ps(&mut self) -> PDS_START_PS_W<PDS_CTL_SPEC> {
        PDS_START_PS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sleep_forever(&mut self) -> CR_SLEEP_FOREVER_W<PDS_CTL_SPEC> {
        CR_SLEEP_FOREVER_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_xtal_force_off(&mut self) -> CR_XTAL_FORCE_OFF_W<PDS_CTL_SPEC> {
        CR_XTAL_FORCE_OFF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_save_state(&mut self) -> CR_PDS_WIFI_SAVE_STATE_W<PDS_CTL_SPEC> {
        CR_PDS_WIFI_SAVE_STATE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_ldo11(&mut self) -> CR_PDS_PD_LDO11_W<PDS_CTL_SPEC> {
        CR_PDS_PD_LDO11_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CR_PDS_PD_BG_SYS_W<PDS_CTL_SPEC> {
        CR_PDS_PD_BG_SYS_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_gpio_ie_pu_pd(&mut self) -> CR_PDS_CTRL_GPIO_IE_PU_PD_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_GPIO_IE_PU_PD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_dcdc18(&mut self) -> CR_PDS_PD_DCDC18_W<PDS_CTL_SPEC> {
        CR_PDS_PD_DCDC18_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gate_clk(&mut self) -> CR_PDS_GATE_CLK_W<PDS_CTL_SPEC> {
        CR_PDS_GATE_CLK_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_mem_stby(&mut self) -> CR_PDS_MEM_STBY_W<PDS_CTL_SPEC> {
        CR_PDS_MEM_STBY_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_glb_reg_reset_protect(&mut self) -> CR_PDS_GLB_REG_RESET_PROTECT_W<PDS_CTL_SPEC> {
        CR_PDS_GLB_REG_RESET_PROTECT_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_iso_en(&mut self) -> CR_PDS_ISO_EN_W<PDS_CTL_SPEC> {
        CR_PDS_ISO_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wait_xtal_rdy(&mut self) -> CR_PDS_WAIT_XTAL_RDY_W<PDS_CTL_SPEC> {
        CR_PDS_WAIT_XTAL_RDY_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pwr_off(&mut self) -> CR_PDS_PWR_OFF_W<PDS_CTL_SPEC> {
        CR_PDS_PWR_OFF_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_xtal(&mut self) -> CR_PDS_PD_XTAL_W<PDS_CTL_SPEC> {
        CR_PDS_PD_XTAL_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_soc_enb(&mut self) -> CR_PDS_CTRL_SOC_ENB_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_SOC_ENB_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rst_soc(&mut self) -> CR_PDS_RST_SOC_W<PDS_CTL_SPEC> {
        CR_PDS_RST_SOC_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rc32m_off_dis(&mut self) -> CR_PDS_RC32M_OFF_DIS_W<PDS_CTL_SPEC> {
        CR_PDS_RC32M_OFF_DIS_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ldo11_vsel_en(&mut self) -> CR_PDS_LDO11_VSEL_EN_W<PDS_CTL_SPEC> {
        CR_PDS_LDO11_VSEL_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_usbpll_pd(&mut self) -> CR_PDS_CTRL_USBPLL_PD_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_USBPLL_PD_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_aupll_pd(&mut self) -> CR_PDS_CTRL_AUPLL_PD_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_AUPLL_PD_W::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_wifipll_pd(&mut self) -> CR_PDS_CTRL_WIFIPLL_PD_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_WIFIPLL_PD_W::new(self, 22)
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ldo11_vol(&mut self) -> CR_PDS_LDO11_VOL_W<PDS_CTL_SPEC> {
        CR_PDS_LDO11_VOL_W::new(self, 23)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_rf(&mut self) -> CR_PDS_CTRL_RF_W<PDS_CTL_SPEC> {
        CR_PDS_CTRL_RF_W::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_start_use_tbtt_sleep(&mut self) -> CR_PDS_START_USE_TBTT_SLEEP_W<PDS_CTL_SPEC> {
        CR_PDS_START_USE_TBTT_SLEEP_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_iso_mode(&mut self) -> CR_PDS_GPIO_ISO_MODE_W<PDS_CTL_SPEC> {
        CR_PDS_GPIO_ISO_MODE_W::new(self, 31)
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
#[doc = "PDS_CTL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_CTL_SPEC;
impl crate::RegisterSpec for PDS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl::R`](R) reader structure"]
impl crate::Readable for PDS_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl::W`](W) writer structure"]
impl crate::Writable for PDS_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL to value 0"]
impl crate::Resettable for PDS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
