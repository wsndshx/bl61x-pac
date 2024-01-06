#[doc = "Register `pds_stat` reader"]
pub type R = crate::R<PDS_STAT_SPEC>;
#[doc = "Register `pds_stat` writer"]
pub type W = crate::W<PDS_STAT_SPEC>;
#[doc = "Field `ro_pds_state` reader - "]
pub type RO_PDS_STATE_R = crate::FieldReader;
#[doc = "Field `ro_pds_state` writer - "]
pub type RO_PDS_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RO_PDS_RF_STATE_R = crate::FieldReader;
#[doc = "Field `ro_pds_rf_state` writer - "]
pub type RO_PDS_RF_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pds_reset_event` reader - "]
pub type PDS_RESET_EVENT_R = crate::FieldReader;
#[doc = "Field `pds_reset_event` writer - "]
pub type PDS_RESET_EVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pds_clr_reset_event` reader - "]
pub type PDS_CLR_RESET_EVENT_R = crate::BitReader;
#[doc = "Field `pds_clr_reset_event` writer - "]
pub type PDS_CLR_RESET_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RO_PDS_STATE_R {
        RO_PDS_STATE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RO_PDS_RF_STATE_R {
        RO_PDS_RF_STATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pds_reset_event(&self) -> PDS_RESET_EVENT_R {
        PDS_RESET_EVENT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&self) -> PDS_CLR_RESET_EVENT_R {
        PDS_CLR_RESET_EVENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_state(&mut self) -> RO_PDS_STATE_W<PDS_STAT_SPEC> {
        RO_PDS_STATE_W::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn ro_pds_rf_state(&mut self) -> RO_PDS_RF_STATE_W<PDS_STAT_SPEC> {
        RO_PDS_RF_STATE_W::new(self, 8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn pds_reset_event(&mut self) -> PDS_RESET_EVENT_W<PDS_STAT_SPEC> {
        PDS_RESET_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pds_clr_reset_event(&mut self) -> PDS_CLR_RESET_EVENT_W<PDS_STAT_SPEC> {
        PDS_CLR_RESET_EVENT_W::new(self, 31)
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
#[doc = "pds_stat.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_STAT_SPEC;
impl crate::RegisterSpec for PDS_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_stat::R`](R) reader structure"]
impl crate::Readable for PDS_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_stat::W`](W) writer structure"]
impl crate::Writable for PDS_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PDS_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
