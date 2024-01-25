#[doc = "Register `mcu_e907_rtc` reader"]
pub type R = crate::R<MCU_E907_RTC_SPEC>;
#[doc = "Register `mcu_e907_rtc` writer"]
pub type W = crate::W<MCU_E907_RTC_SPEC>;
#[doc = "Field `reg_mcu_rtc_div` reader - MCU RTC divider. This field controls the divider for the RTC clock."]
pub type REG_MCU_RTC_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `reg_mcu_rtc_div` writer - MCU RTC divider. This field controls the divider for the RTC clock."]
pub type REG_MCU_RTC_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `reg_mcu_rtc_clk_sel` reader - MCU RTC clock select. This bit controls the clock source for the RTC."]
pub type REG_MCU_RTC_CLK_SEL_R = crate::BitReader;
#[doc = "Field `reg_mcu_rtc_clk_sel` writer - MCU RTC clock select. This bit controls the clock source for the RTC."]
pub type REG_MCU_RTC_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_rtc_rst` reader - MCU RTC reset. Writing 1 to this bit resets the RTC."]
pub type REG_MCU_RTC_RST_R = crate::BitReader;
#[doc = "Field `reg_mcu_rtc_rst` writer - MCU RTC reset. Writing 1 to this bit resets the RTC."]
pub type REG_MCU_RTC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_rtc_en` reader - MCU RTC enable. Writing 1 to this bit enables the RTC."]
pub type REG_MCU_RTC_EN_R = crate::BitReader;
#[doc = "Field `reg_mcu_rtc_en` writer - MCU RTC enable. Writing 1 to this bit enables the RTC."]
pub type REG_MCU_RTC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - MCU RTC divider. This field controls the divider for the RTC clock."]
    #[inline(always)]
    pub fn reg_mcu_rtc_div(&self) -> REG_MCU_RTC_DIV_R {
        REG_MCU_RTC_DIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - MCU RTC clock select. This bit controls the clock source for the RTC."]
    #[inline(always)]
    pub fn reg_mcu_rtc_clk_sel(&self) -> REG_MCU_RTC_CLK_SEL_R {
        REG_MCU_RTC_CLK_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MCU RTC reset. Writing 1 to this bit resets the RTC."]
    #[inline(always)]
    pub fn reg_mcu_rtc_rst(&self) -> REG_MCU_RTC_RST_R {
        REG_MCU_RTC_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MCU RTC enable. Writing 1 to this bit enables the RTC."]
    #[inline(always)]
    pub fn reg_mcu_rtc_en(&self) -> REG_MCU_RTC_EN_R {
        REG_MCU_RTC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - MCU RTC divider. This field controls the divider for the RTC clock."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_div(&mut self) -> REG_MCU_RTC_DIV_W<MCU_E907_RTC_SPEC> {
        REG_MCU_RTC_DIV_W::new(self, 0)
    }
    #[doc = "Bit 29 - MCU RTC clock select. This bit controls the clock source for the RTC."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_clk_sel(&mut self) -> REG_MCU_RTC_CLK_SEL_W<MCU_E907_RTC_SPEC> {
        REG_MCU_RTC_CLK_SEL_W::new(self, 29)
    }
    #[doc = "Bit 30 - MCU RTC reset. Writing 1 to this bit resets the RTC."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_rst(&mut self) -> REG_MCU_RTC_RST_W<MCU_E907_RTC_SPEC> {
        REG_MCU_RTC_RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - MCU RTC enable. Writing 1 to this bit enables the RTC."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_rtc_en(&mut self) -> REG_MCU_RTC_EN_W<MCU_E907_RTC_SPEC> {
        REG_MCU_RTC_EN_W::new(self, 31)
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
#[doc = "MCU E907 RTC Register. Controls the E907 RTC settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_e907_rtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_e907_rtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU_E907_RTC_SPEC;
impl crate::RegisterSpec for MCU_E907_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_e907_rtc::R`](R) reader structure"]
impl crate::Readable for MCU_E907_RTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu_e907_rtc::W`](W) writer structure"]
impl crate::Writable for MCU_E907_RTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_e907_rtc to value 0"]
impl crate::Resettable for MCU_E907_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
