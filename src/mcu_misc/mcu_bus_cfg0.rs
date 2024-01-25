#[doc = "Register `mcu_bus_cfg0` reader"]
pub type R = crate::R<MCU_BUS_CFG0_SPEC>;
#[doc = "Register `mcu_bus_cfg0` writer"]
pub type W = crate::W<MCU_BUS_CFG0_SPEC>;
#[doc = "Field `reg_mcu_infra_timeout_en` reader - MCU infrastructure timeout enable. When this bit is set, a timeout occurs if a transaction through the MCU bus takes longer than reg_mcu_infra_timeout_value."]
pub type REG_MCU_INFRA_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `reg_mcu_infra_timeout_en` writer - MCU infrastructure timeout enable. When this bit is set, a timeout occurs if a transaction through the MCU bus takes longer than reg_mcu_infra_timeout_value."]
pub type REG_MCU_INFRA_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_infra_timeout_clr` reader - MCU infrastructure timeout clear. Writing 1 to this bit clears the timeout, if one is active."]
pub type REG_MCU_INFRA_TIMEOUT_CLR_R = crate::BitReader;
#[doc = "Field `reg_mcu_infra_timeout_clr` writer - MCU infrastructure timeout clear. Writing 1 to this bit clears the timeout, if one is active."]
pub type REG_MCU_INFRA_TIMEOUT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu_infra_timeout` reader - MCU infrastructure timeout status. This bit is set when a timeout occurs."]
pub type STS_MCU_INFRA_TIMEOUT_R = crate::BitReader;
#[doc = "Field `sts_mcu_infra_timeout` writer - MCU infrastructure timeout status. This bit is set when a timeout occurs."]
pub type STS_MCU_INFRA_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCU infrastructure timeout enable. When this bit is set, a timeout occurs if a transaction through the MCU bus takes longer than reg_mcu_infra_timeout_value."]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_en(&self) -> REG_MCU_INFRA_TIMEOUT_EN_R {
        REG_MCU_INFRA_TIMEOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCU infrastructure timeout clear. Writing 1 to this bit clears the timeout, if one is active."]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_clr(&self) -> REG_MCU_INFRA_TIMEOUT_CLR_R {
        REG_MCU_INFRA_TIMEOUT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - MCU infrastructure timeout status. This bit is set when a timeout occurs."]
    #[inline(always)]
    pub fn sts_mcu_infra_timeout(&self) -> STS_MCU_INFRA_TIMEOUT_R {
        STS_MCU_INFRA_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU infrastructure timeout enable. When this bit is set, a timeout occurs if a transaction through the MCU bus takes longer than reg_mcu_infra_timeout_value."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_timeout_en(&mut self) -> REG_MCU_INFRA_TIMEOUT_EN_W<MCU_BUS_CFG0_SPEC> {
        REG_MCU_INFRA_TIMEOUT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCU infrastructure timeout clear. Writing 1 to this bit clears the timeout, if one is active."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_timeout_clr(&mut self) -> REG_MCU_INFRA_TIMEOUT_CLR_W<MCU_BUS_CFG0_SPEC> {
        REG_MCU_INFRA_TIMEOUT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 16 - MCU infrastructure timeout status. This bit is set when a timeout occurs."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_infra_timeout(&mut self) -> STS_MCU_INFRA_TIMEOUT_W<MCU_BUS_CFG0_SPEC> {
        STS_MCU_INFRA_TIMEOUT_W::new(self, 16)
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
#[doc = "MCU Bus Configuration 0 Register. Controls MCU bus configuration settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bus_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bus_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU_BUS_CFG0_SPEC;
impl crate::RegisterSpec for MCU_BUS_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_bus_cfg0::R`](R) reader structure"]
impl crate::Readable for MCU_BUS_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu_bus_cfg0::W`](W) writer structure"]
impl crate::Writable for MCU_BUS_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_bus_cfg0 to value 0"]
impl crate::Resettable for MCU_BUS_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
