#[doc = "Register `tzc_sf_tzsrg_msb` reader"]
pub type R = crate::R<TZC_SF_TZSRG_MSB_SPEC>;
#[doc = "Register `tzc_sf_tzsrg_msb` writer"]
pub type W = crate::W<TZC_SF_TZSRG_MSB_SPEC>;
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` reader - MSB of the End address for TrustZone Range 0 in SF."]
pub type TZC_SF_TZSRG_R0_END_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` writer - MSB of the End address for TrustZone Range 0 in SF."]
pub type TZC_SF_TZSRG_R0_END_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` reader - MSB of the Start address for TrustZone Range 0 in SF."]
pub type TZC_SF_TZSRG_R0_START_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` writer - MSB of the Start address for TrustZone Range 0 in SF."]
pub type TZC_SF_TZSRG_R0_START_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` reader - MSB of the End address for TrustZone Range 1 in SF."]
pub type TZC_SF_TZSRG_R1_END_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` writer - MSB of the End address for TrustZone Range 1 in SF."]
pub type TZC_SF_TZSRG_R1_END_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` reader - MSB of the Start address for TrustZone Range 1 in SF."]
pub type TZC_SF_TZSRG_R1_START_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` writer - MSB of the Start address for TrustZone Range 1 in SF."]
pub type TZC_SF_TZSRG_R1_START_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` reader - MSB of the End address for TrustZone Range 2 in SF."]
pub type TZC_SF_TZSRG_R2_END_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` writer - MSB of the End address for TrustZone Range 2 in SF."]
pub type TZC_SF_TZSRG_R2_END_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` reader - MSB of the Start address for TrustZone Range 2 in SF."]
pub type TZC_SF_TZSRG_R2_START_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` writer - MSB of the Start address for TrustZone Range 2 in SF."]
pub type TZC_SF_TZSRG_R2_START_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` reader - MSB of the End address for TrustZone Range 3 in SF."]
pub type TZC_SF_TZSRG_R3_END_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` writer - MSB of the End address for TrustZone Range 3 in SF."]
pub type TZC_SF_TZSRG_R3_END_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` reader - MSB of the Start address for TrustZone Range 3 in SF."]
pub type TZC_SF_TZSRG_R3_START_MSB_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` writer - MSB of the Start address for TrustZone Range 3 in SF."]
pub type TZC_SF_TZSRG_R3_START_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - MSB of the End address for TrustZone Range 0 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end_msb(&self) -> TZC_SF_TZSRG_R0_END_MSB_R {
        TZC_SF_TZSRG_R0_END_MSB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - MSB of the Start address for TrustZone Range 0 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start_msb(&self) -> TZC_SF_TZSRG_R0_START_MSB_R {
        TZC_SF_TZSRG_R0_START_MSB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - MSB of the End address for TrustZone Range 1 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end_msb(&self) -> TZC_SF_TZSRG_R1_END_MSB_R {
        TZC_SF_TZSRG_R1_END_MSB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - MSB of the Start address for TrustZone Range 1 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start_msb(&self) -> TZC_SF_TZSRG_R1_START_MSB_R {
        TZC_SF_TZSRG_R1_START_MSB_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MSB of the End address for TrustZone Range 2 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end_msb(&self) -> TZC_SF_TZSRG_R2_END_MSB_R {
        TZC_SF_TZSRG_R2_END_MSB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - MSB of the Start address for TrustZone Range 2 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start_msb(&self) -> TZC_SF_TZSRG_R2_START_MSB_R {
        TZC_SF_TZSRG_R2_START_MSB_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - MSB of the End address for TrustZone Range 3 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end_msb(&self) -> TZC_SF_TZSRG_R3_END_MSB_R {
        TZC_SF_TZSRG_R3_END_MSB_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - MSB of the Start address for TrustZone Range 3 in SF."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start_msb(&self) -> TZC_SF_TZSRG_R3_START_MSB_R {
        TZC_SF_TZSRG_R3_START_MSB_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MSB of the End address for TrustZone Range 0 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r0_end_msb(&mut self) -> TZC_SF_TZSRG_R0_END_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R0_END_MSB_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - MSB of the Start address for TrustZone Range 0 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r0_start_msb(
        &mut self,
    ) -> TZC_SF_TZSRG_R0_START_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R0_START_MSB_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - MSB of the End address for TrustZone Range 1 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r1_end_msb(&mut self) -> TZC_SF_TZSRG_R1_END_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R1_END_MSB_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - MSB of the Start address for TrustZone Range 1 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r1_start_msb(
        &mut self,
    ) -> TZC_SF_TZSRG_R1_START_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R1_START_MSB_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - MSB of the End address for TrustZone Range 2 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r2_end_msb(&mut self) -> TZC_SF_TZSRG_R2_END_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R2_END_MSB_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - MSB of the Start address for TrustZone Range 2 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r2_start_msb(
        &mut self,
    ) -> TZC_SF_TZSRG_R2_START_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R2_START_MSB_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - MSB of the End address for TrustZone Range 3 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r3_end_msb(&mut self) -> TZC_SF_TZSRG_R3_END_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R3_END_MSB_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - MSB of the Start address for TrustZone Range 3 in SF."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r3_start_msb(
        &mut self,
    ) -> TZC_SF_TZSRG_R3_START_MSB_W<TZC_SF_TZSRG_MSB_SPEC> {
        TZC_SF_TZSRG_R3_START_MSB_W::new(self, 28)
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
#[doc = "TrustZone Controller SF TrustZone Security Register Group MSB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SF_TZSRG_MSB_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_msb::R`](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_MSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_msb::W`](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_MSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_msb to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
