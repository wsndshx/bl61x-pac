#[doc = "Register `clock_config_1` reader"]
pub type R = crate::R<CLOCK_CONFIG_1_SPEC>;
#[doc = "Register `clock_config_1` writer"]
pub type W = crate::W<CLOCK_CONFIG_1_SPEC>;
#[doc = "Field `reg_bclk_div_act_pulse` reader - "]
pub type REG_BCLK_DIV_ACT_PULSE_R = crate::BitReader;
#[doc = "Field `reg_bclk_div_act_pulse` writer - "]
pub type REG_BCLK_DIV_ACT_PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk_div_bypass` reader - "]
pub type REG_BCLK_DIV_BYPASS_R = crate::BitReader;
#[doc = "Field `reg_bclk_div_bypass` writer - "]
pub type REG_BCLK_DIV_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bclk_prot_done` reader - "]
pub type STS_BCLK_PROT_DONE_R = crate::BitReader;
#[doc = "Field `sts_bclk_prot_done` writer - "]
pub type STS_BCLK_PROT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk_sw_done_cnt` reader - "]
pub type REG_BCLK_SW_DONE_CNT_R = crate::FieldReader;
#[doc = "Field `reg_bclk_sw_done_cnt` writer - "]
pub type REG_BCLK_SW_DONE_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fclk_sw_state` reader - "]
pub type FCLK_SW_STATE_R = crate::FieldReader;
#[doc = "Field `fclk_sw_state` writer - "]
pub type FCLK_SW_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bclk_div_act_pulse(&self) -> REG_BCLK_DIV_ACT_PULSE_R {
        REG_BCLK_DIV_ACT_PULSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bclk_div_bypass(&self) -> REG_BCLK_DIV_BYPASS_R {
        REG_BCLK_DIV_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_bclk_prot_done(&self) -> STS_BCLK_PROT_DONE_R {
        STS_BCLK_PROT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_bclk_sw_done_cnt(&self) -> REG_BCLK_SW_DONE_CNT_R {
        REG_BCLK_SW_DONE_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FCLK_SW_STATE_R {
        FCLK_SW_STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_div_act_pulse(&mut self) -> REG_BCLK_DIV_ACT_PULSE_W<CLOCK_CONFIG_1_SPEC> {
        REG_BCLK_DIV_ACT_PULSE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_div_bypass(&mut self) -> REG_BCLK_DIV_BYPASS_W<CLOCK_CONFIG_1_SPEC> {
        REG_BCLK_DIV_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sts_bclk_prot_done(&mut self) -> STS_BCLK_PROT_DONE_W<CLOCK_CONFIG_1_SPEC> {
        STS_BCLK_PROT_DONE_W::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_sw_done_cnt(&mut self) -> REG_BCLK_SW_DONE_CNT_W<CLOCK_CONFIG_1_SPEC> {
        REG_BCLK_SW_DONE_CNT_W::new(self, 4)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_sw_state(&mut self) -> FCLK_SW_STATE_W<CLOCK_CONFIG_1_SPEC> {
        FCLK_SW_STATE_W::new(self, 24)
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
#[doc = "System clock configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CONFIG_1_SPEC;
impl crate::RegisterSpec for CLOCK_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_config_1::R`](R) reader structure"]
impl crate::Readable for CLOCK_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_config_1::W`](W) writer structure"]
impl crate::Writable for CLOCK_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clock_config_1 to value 0"]
impl crate::Resettable for CLOCK_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
