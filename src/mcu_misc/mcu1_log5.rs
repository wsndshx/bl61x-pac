#[doc = "Register `mcu1_log5` reader"]
pub type R = crate::R<MCU1_LOG5_SPEC>;
#[doc = "Register `mcu1_log5` writer"]
pub type W = crate::W<MCU1_LOG5_SPEC>;
#[doc = "Field `sts_mcu1_lockup` reader - MCU1 lockup status. This bit is set when MCU1 is in a lockup state."]
pub type STS_MCU1_LOCKUP_R = crate::BitReader;
#[doc = "Field `sts_mcu1_lockup` writer - MCU1 lockup status. This bit is set when MCU1 is in a lockup state."]
pub type STS_MCU1_LOCKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu1_halted` reader - MCU1 halted status. This bit is set when MCU1 is in a halted state."]
pub type STS_MCU1_HALTED_R = crate::BitReader;
#[doc = "Field `sts_mcu1_halted` writer - MCU1 halted status. This bit is set when MCU1 is in a halted state."]
pub type STS_MCU1_HALTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_ndm_rstn_req` reader - MCU1 NDM reset request. Writing 1 to this bit requests a NDM reset for MCU1."]
pub type MCU1_NDM_RSTN_REQ_R = crate::BitReader;
#[doc = "Field `mcu1_ndm_rstn_req` writer - MCU1 NDM reset request. Writing 1 to this bit requests a NDM reset for MCU1."]
pub type MCU1_NDM_RSTN_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_hart_rstn_req` reader - MCU1 hart reset request. Writing 1 to this bit requests a hart reset for MCU1."]
pub type MCU1_HART_RSTN_REQ_R = crate::BitReader;
#[doc = "Field `mcu1_hart_rstn_req` writer - MCU1 hart reset request. Writing 1 to this bit requests a hart reset for MCU1."]
pub type MCU1_HART_RSTN_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - MCU1 lockup status. This bit is set when MCU1 is in a lockup state."]
    #[inline(always)]
    pub fn sts_mcu1_lockup(&self) -> STS_MCU1_LOCKUP_R {
        STS_MCU1_LOCKUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MCU1 halted status. This bit is set when MCU1 is in a halted state."]
    #[inline(always)]
    pub fn sts_mcu1_halted(&self) -> STS_MCU1_HALTED_R {
        STS_MCU1_HALTED_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - MCU1 NDM reset request. Writing 1 to this bit requests a NDM reset for MCU1."]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_req(&self) -> MCU1_NDM_RSTN_REQ_R {
        MCU1_NDM_RSTN_REQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MCU1 hart reset request. Writing 1 to this bit requests a hart reset for MCU1."]
    #[inline(always)]
    pub fn mcu1_hart_rstn_req(&self) -> MCU1_HART_RSTN_REQ_R {
        MCU1_HART_RSTN_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - MCU1 lockup status. This bit is set when MCU1 is in a lockup state."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_lockup(&mut self) -> STS_MCU1_LOCKUP_W<MCU1_LOG5_SPEC> {
        STS_MCU1_LOCKUP_W::new(self, 24)
    }
    #[doc = "Bit 25 - MCU1 halted status. This bit is set when MCU1 is in a halted state."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_halted(&mut self) -> STS_MCU1_HALTED_W<MCU1_LOG5_SPEC> {
        STS_MCU1_HALTED_W::new(self, 25)
    }
    #[doc = "Bit 28 - MCU1 NDM reset request. Writing 1 to this bit requests a NDM reset for MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_ndm_rstn_req(&mut self) -> MCU1_NDM_RSTN_REQ_W<MCU1_LOG5_SPEC> {
        MCU1_NDM_RSTN_REQ_W::new(self, 28)
    }
    #[doc = "Bit 29 - MCU1 hart reset request. Writing 1 to this bit requests a hart reset for MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_hart_rstn_req(&mut self) -> MCU1_HART_RSTN_REQ_W<MCU1_LOG5_SPEC> {
        MCU1_HART_RSTN_REQ_W::new(self, 29)
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
#[doc = "MCU1 Log 5 Register. Stores information about the last MCU1 lockup and halt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU1_LOG5_SPEC;
impl crate::RegisterSpec for MCU1_LOG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log5::R`](R) reader structure"]
impl crate::Readable for MCU1_LOG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu1_log5::W`](W) writer structure"]
impl crate::Writable for MCU1_LOG5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log5 to value 0"]
impl crate::Resettable for MCU1_LOG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
