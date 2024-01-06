#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `rtc_ctl` reader - "]
pub type RTC_CTL_R = crate::FieldReader;
#[doc = "Field `rtc_ctl` writer - "]
pub type RTC_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rtc_dly_option` reader - "]
pub type RTC_DLY_OPTION_R = crate::BitReader;
#[doc = "Field `rtc_dly_option` writer - "]
pub type RTC_DLY_OPTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_mode` reader - "]
pub type HBN_MODE_R = crate::BitReader;
#[doc = "Field `hbn_mode` writer - "]
pub type HBN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trap_mode` reader - "]
pub type TRAP_MODE_R = crate::BitReader;
#[doc = "Field `trap_mode` writer - "]
pub type TRAP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwrdn_hbn_core` reader - "]
pub type PWRDN_HBN_CORE_R = crate::BitReader;
#[doc = "Field `pwrdn_hbn_core` writer - "]
pub type PWRDN_HBN_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwrdn_hbn_rtc` reader - "]
pub type PWRDN_HBN_RTC_R = crate::BitReader;
#[doc = "Field `pwrdn_hbn_rtc` writer - "]
pub type PWRDN_HBN_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_rst` reader - "]
pub type SW_RST_R = crate::BitReader;
#[doc = "Field `sw_rst` writer - "]
pub type SW_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` reader - "]
pub type HBN_DIS_PWR_OFF_LDO11_R = crate::BitReader;
#[doc = "Field `hbn_dis_pwr_off_ldo11` writer - "]
pub type HBN_DIS_PWR_OFF_LDO11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` reader - "]
pub type HBN_DIS_PWR_OFF_LDO11_RT_R = crate::BitReader;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` writer - "]
pub type HBN_DIS_PWR_OFF_LDO11_RT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_ldo11_rt_vout_sel` reader - "]
pub type HBN_LDO11_RT_VOUT_SEL_R = crate::FieldReader;
#[doc = "Field `hbn_ldo11_rt_vout_sel` writer - "]
pub type HBN_LDO11_RT_VOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_ldo11_aon_vout_sel` reader - "]
pub type HBN_LDO11_AON_VOUT_SEL_R = crate::FieldReader;
#[doc = "Field `hbn_ldo11_aon_vout_sel` writer - "]
pub type HBN_LDO11_AON_VOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pu_dcdc18_aon` reader - "]
pub type PU_DCDC18_AON_R = crate::BitReader;
#[doc = "Field `pu_dcdc18_aon` writer - "]
pub type PU_DCDC18_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwr_on_option` reader - "]
pub type PWR_ON_OPTION_R = crate::BitReader;
#[doc = "Field `pwr_on_option` writer - "]
pub type PWR_ON_OPTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_slp_option` reader - "]
pub type SRAM_SLP_OPTION_R = crate::BitReader;
#[doc = "Field `sram_slp_option` writer - "]
pub type SRAM_SLP_OPTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_slp` reader - "]
pub type SRAM_SLP_R = crate::BitReader;
#[doc = "Field `sram_slp` writer - "]
pub type SRAM_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_state` reader - "]
pub type HBN_STATE_R = crate::FieldReader;
#[doc = "Field `hbn_state` writer - "]
pub type HBN_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RTC_CTL_R {
        RTC_CTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RTC_DLY_OPTION_R {
        RTC_DLY_OPTION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_mode(&self) -> HBN_MODE_R {
        HBN_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TRAP_MODE_R {
        TRAP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PWRDN_HBN_CORE_R {
        PWRDN_HBN_CORE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&self) -> PWRDN_HBN_RTC_R {
        PWRDN_HBN_RTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HBN_DIS_PWR_OFF_LDO11_R {
        HBN_DIS_PWR_OFF_LDO11_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&self) -> HBN_DIS_PWR_OFF_LDO11_RT_R {
        HBN_DIS_PWR_OFF_LDO11_RT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&self) -> HBN_LDO11_RT_VOUT_SEL_R {
        HBN_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&self) -> HBN_LDO11_AON_VOUT_SEL_R {
        HBN_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&self) -> PU_DCDC18_AON_R {
        PU_DCDC18_AON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&self) -> PWR_ON_OPTION_R {
        PWR_ON_OPTION_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&self) -> SRAM_SLP_OPTION_R {
        SRAM_SLP_OPTION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&self) -> SRAM_SLP_R {
        SRAM_SLP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&self) -> HBN_STATE_R {
        HBN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctl(&mut self) -> RTC_CTL_W<CONTROL_SPEC> {
        RTC_CTL_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_dly_option(&mut self) -> RTC_DLY_OPTION_W<CONTROL_SPEC> {
        RTC_DLY_OPTION_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_mode(&mut self) -> HBN_MODE_W<CONTROL_SPEC> {
        HBN_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn trap_mode(&mut self) -> TRAP_MODE_W<CONTROL_SPEC> {
        TRAP_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_hbn_core(&mut self) -> PWRDN_HBN_CORE_W<CONTROL_SPEC> {
        PWRDN_HBN_CORE_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_hbn_rtc(&mut self) -> PWRDN_HBN_RTC_W<CONTROL_SPEC> {
        PWRDN_HBN_RTC_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst(&mut self) -> SW_RST_W<CONTROL_SPEC> {
        SW_RST_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HBN_DIS_PWR_OFF_LDO11_W<CONTROL_SPEC> {
        HBN_DIS_PWR_OFF_LDO11_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_dis_pwr_off_ldo11_rt(&mut self) -> HBN_DIS_PWR_OFF_LDO11_RT_W<CONTROL_SPEC> {
        HBN_DIS_PWR_OFF_LDO11_RT_W::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo11_rt_vout_sel(&mut self) -> HBN_LDO11_RT_VOUT_SEL_W<CONTROL_SPEC> {
        HBN_LDO11_RT_VOUT_SEL_W::new(self, 15)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo11_aon_vout_sel(&mut self) -> HBN_LDO11_AON_VOUT_SEL_W<CONTROL_SPEC> {
        HBN_LDO11_AON_VOUT_SEL_W::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dcdc18_aon(&mut self) -> PU_DCDC18_AON_W<CONTROL_SPEC> {
        PU_DCDC18_AON_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_on_option(&mut self) -> PWR_ON_OPTION_W<CONTROL_SPEC> {
        PWR_ON_OPTION_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sram_slp_option(&mut self) -> SRAM_SLP_OPTION_W<CONTROL_SPEC> {
        SRAM_SLP_OPTION_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn sram_slp(&mut self) -> SRAM_SLP_W<CONTROL_SPEC> {
        SRAM_SLP_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_state(&mut self) -> HBN_STATE_W<CONTROL_SPEC> {
        HBN_STATE_W::new(self, 28)
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
#[doc = "Miscellaneous control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
