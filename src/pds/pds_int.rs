#[doc = "Register `PDS_INT` reader"]
pub type R = crate::R<PDS_INT_SPEC>;
#[doc = "Register `PDS_INT` writer"]
pub type W = crate::W<PDS_INT_SPEC>;
#[doc = "Field `ro_pds_wake_int` reader - "]
pub type RO_PDS_WAKE_INT_R = crate::BitReader;
#[doc = "Field `ro_pds_wake_int` writer - "]
pub type RO_PDS_WAKE_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub type RO_PDS_RF_DONE_INT_R = crate::BitReader;
#[doc = "Field `ro_pds_rf_done_int` writer - "]
pub type RO_PDS_RF_DONE_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ro_pds_wifi_tbtt_sleep_irq` reader - "]
pub type RO_PDS_WIFI_TBTT_SLEEP_IRQ_R = crate::BitReader;
#[doc = "Field `ro_pds_wifi_tbtt_sleep_irq` writer - "]
pub type RO_PDS_WIFI_TBTT_SLEEP_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ro_pds_wifi_tbtt_wakeup_irq` reader - "]
pub type RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R = crate::BitReader;
#[doc = "Field `ro_pds_wifi_tbtt_wakeup_irq` writer - "]
pub type RO_PDS_WIFI_TBTT_WAKEUP_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub type CR_PDS_WAKE_INT_MASK_R = crate::BitReader;
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub type CR_PDS_WAKE_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub type CR_PDS_RF_DONE_INT_MASK_R = crate::BitReader;
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub type CR_PDS_RF_DONE_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` reader - "]
pub type CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R = crate::BitReader;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` writer - "]
pub type CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` reader - "]
pub type CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R = crate::BitReader;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` writer - "]
pub type CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_int_clr` reader - "]
pub type CR_PDS_INT_CLR_R = crate::BitReader;
#[doc = "Field `cr_pds_int_clr` writer - "]
pub type CR_PDS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wakeup_src_en` reader - "]
pub type CR_PDS_WAKEUP_SRC_EN_R = crate::FieldReader<u16>;
#[doc = "Field `cr_pds_wakeup_src_en` writer - "]
pub type CR_PDS_WAKEUP_SRC_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ro_pds_wakeup_event` reader - "]
pub type RO_PDS_WAKEUP_EVENT_R = crate::FieldReader<u16>;
#[doc = "Field `ro_pds_wakeup_event` writer - "]
pub type RO_PDS_WAKEUP_EVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_sleep_irq(&self) -> RO_PDS_WIFI_TBTT_SLEEP_IRQ_R {
        RO_PDS_WIFI_TBTT_SLEEP_IRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_wakeup_irq(&self) -> RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R {
        RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(&self) -> CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R {
        CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(&self) -> CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R {
        CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&self) -> CR_PDS_WAKEUP_SRC_EN_R {
        CR_PDS_WAKEUP_SRC_EN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&self) -> RO_PDS_WAKEUP_EVENT_R {
        RO_PDS_WAKEUP_EVENT_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_wake_int(&mut self) -> RO_PDS_WAKE_INT_W<PDS_INT_SPEC> {
        RO_PDS_WAKE_INT_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_rf_done_int(&mut self) -> RO_PDS_RF_DONE_INT_W<PDS_INT_SPEC> {
        RO_PDS_RF_DONE_INT_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_wifi_tbtt_sleep_irq(&mut self) -> RO_PDS_WIFI_TBTT_SLEEP_IRQ_W<PDS_INT_SPEC> {
        RO_PDS_WIFI_TBTT_SLEEP_IRQ_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_wifi_tbtt_wakeup_irq(&mut self) -> RO_PDS_WIFI_TBTT_WAKEUP_IRQ_W<PDS_INT_SPEC> {
        RO_PDS_WIFI_TBTT_WAKEUP_IRQ_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W<PDS_INT_SPEC> {
        CR_PDS_WAKE_INT_MASK_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W<PDS_INT_SPEC> {
        CR_PDS_RF_DONE_INT_MASK_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(
        &mut self,
    ) -> CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W<PDS_INT_SPEC> {
        CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(
        &mut self,
    ) -> CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W<PDS_INT_SPEC> {
        CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W<PDS_INT_SPEC> {
        CR_PDS_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wakeup_src_en(&mut self) -> CR_PDS_WAKEUP_SRC_EN_W<PDS_INT_SPEC> {
        CR_PDS_WAKEUP_SRC_EN_W::new(self, 10)
    }
    #[doc = "Bits 21:30"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_wakeup_event(&mut self) -> RO_PDS_WAKEUP_EVENT_W<PDS_INT_SPEC> {
        RO_PDS_WAKEUP_EVENT_W::new(self, 21)
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
#[doc = "PDS_INT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_INT_SPEC;
impl crate::RegisterSpec for PDS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_int::R`](R) reader structure"]
impl crate::Readable for PDS_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_int::W`](W) writer structure"]
impl crate::Writable for PDS_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_INT to value 0"]
impl crate::Resettable for PDS_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
