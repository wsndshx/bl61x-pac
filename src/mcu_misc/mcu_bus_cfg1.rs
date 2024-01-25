#[doc = "Register `mcu_bus_cfg1` reader"]
pub type R = crate::R<MCU_BUS_CFG1_SPEC>;
#[doc = "Register `mcu_bus_cfg1` writer"]
pub type W = crate::W<MCU_BUS_CFG1_SPEC>;
#[doc = "Field `reg_mcu1_hqos` reader - MCU1 high quality of service (QoS) enable. When this bit is set, MCU1 gets high QoS on the bus."]
pub type REG_MCU1_HQOS_R = crate::BitReader;
#[doc = "Field `reg_mcu1_hqos` writer - MCU1 high quality of service (QoS) enable. When this bit is set, MCU1 gets high QoS on the bus."]
pub type REG_MCU1_HQOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu1_awqos` reader - MCU1 address write QoS enable. When this bit is set, MCU1 gets high QoS on address write operations on the bus."]
pub type REG_MCU1_AWQOS_R = crate::BitReader;
#[doc = "Field `reg_mcu1_awqos` writer - MCU1 address write QoS enable. When this bit is set, MCU1 gets high QoS on address write operations on the bus."]
pub type REG_MCU1_AWQOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu1_arqos` reader - MCU1 address read QoS enable. When this bit is set, MCU1 gets high QoS on address read operations on the bus."]
pub type REG_MCU1_ARQOS_R = crate::BitReader;
#[doc = "Field `reg_mcu1_arqos` writer - MCU1 address read QoS enable. When this bit is set, MCU1 gets high QoS on address read operations on the bus."]
pub type REG_MCU1_ARQOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_x_wthre_mcu2ext` reader - Write threshold for MCU2 to external APB slaves. This field controls the write threshold for MCU2 to external APB slaves."]
pub type REG_X_WTHRE_MCU2EXT_R = crate::FieldReader;
#[doc = "Field `reg_x_wthre_mcu2ext` writer - Write threshold for MCU2 to external APB slaves. This field controls the write threshold for MCU2 to external APB slaves."]
pub type REG_X_WTHRE_MCU2EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mcu_infra_arb_mode` reader - MCU infrastructure arbitration mode. This field controls the arbitration mode for the MCU infrastructure."]
pub type REG_MCU_INFRA_ARB_MODE_R = crate::BitReader;
#[doc = "Field `reg_mcu_infra_arb_mode` writer - MCU infrastructure arbitration mode. This field controls the arbitration mode for the MCU infrastructure."]
pub type REG_MCU_INFRA_ARB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCU1 high quality of service (QoS) enable. When this bit is set, MCU1 gets high QoS on the bus."]
    #[inline(always)]
    pub fn reg_mcu1_hqos(&self) -> REG_MCU1_HQOS_R {
        REG_MCU1_HQOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCU1 address write QoS enable. When this bit is set, MCU1 gets high QoS on address write operations on the bus."]
    #[inline(always)]
    pub fn reg_mcu1_awqos(&self) -> REG_MCU1_AWQOS_R {
        REG_MCU1_AWQOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCU1 address read QoS enable. When this bit is set, MCU1 gets high QoS on address read operations on the bus."]
    #[inline(always)]
    pub fn reg_mcu1_arqos(&self) -> REG_MCU1_ARQOS_R {
        REG_MCU1_ARQOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Write threshold for MCU2 to external APB slaves. This field controls the write threshold for MCU2 to external APB slaves."]
    #[inline(always)]
    pub fn reg_x_wthre_mcu2ext(&self) -> REG_X_WTHRE_MCU2EXT_R {
        REG_X_WTHRE_MCU2EXT_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 16 - MCU infrastructure arbitration mode. This field controls the arbitration mode for the MCU infrastructure."]
    #[inline(always)]
    pub fn reg_mcu_infra_arb_mode(&self) -> REG_MCU_INFRA_ARB_MODE_R {
        REG_MCU_INFRA_ARB_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU1 high quality of service (QoS) enable. When this bit is set, MCU1 gets high QoS on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_hqos(&mut self) -> REG_MCU1_HQOS_W<MCU_BUS_CFG1_SPEC> {
        REG_MCU1_HQOS_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCU1 address write QoS enable. When this bit is set, MCU1 gets high QoS on address write operations on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_awqos(&mut self) -> REG_MCU1_AWQOS_W<MCU_BUS_CFG1_SPEC> {
        REG_MCU1_AWQOS_W::new(self, 1)
    }
    #[doc = "Bit 2 - MCU1 address read QoS enable. When this bit is set, MCU1 gets high QoS on address read operations on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_arqos(&mut self) -> REG_MCU1_ARQOS_W<MCU_BUS_CFG1_SPEC> {
        REG_MCU1_ARQOS_W::new(self, 2)
    }
    #[doc = "Bits 7:8 - Write threshold for MCU2 to external APB slaves. This field controls the write threshold for MCU2 to external APB slaves."]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_mcu2ext(&mut self) -> REG_X_WTHRE_MCU2EXT_W<MCU_BUS_CFG1_SPEC> {
        REG_X_WTHRE_MCU2EXT_W::new(self, 7)
    }
    #[doc = "Bit 16 - MCU infrastructure arbitration mode. This field controls the arbitration mode for the MCU infrastructure."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_infra_arb_mode(&mut self) -> REG_MCU_INFRA_ARB_MODE_W<MCU_BUS_CFG1_SPEC> {
        REG_MCU_INFRA_ARB_MODE_W::new(self, 16)
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
#[doc = "MCU Bus Configuration 1 Register. Controls MCU bus configuration settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bus_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bus_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU_BUS_CFG1_SPEC;
impl crate::RegisterSpec for MCU_BUS_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_bus_cfg1::R`](R) reader structure"]
impl crate::Readable for MCU_BUS_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu_bus_cfg1::W`](W) writer structure"]
impl crate::Writable for MCU_BUS_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_bus_cfg1 to value 0"]
impl crate::Resettable for MCU_BUS_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
