#[doc = "Register `tzc_sf_tzsrg_ctrl` reader"]
pub type R = crate::R<TZC_SF_TZSRG_CTRL_SPEC>;
#[doc = "Register `tzc_sf_tzsrg_ctrl` writer"]
pub type W = crate::W<TZC_SF_TZSRG_CTRL_SPEC>;
#[doc = "Field `tzc_sf_tzsrg_r0_id_en` reader - Enable TrustZone Security ID for Range 0."]
pub type TZC_SF_TZSRG_R0_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_id_en` writer - Enable TrustZone Security ID for Range 0."]
pub type TZC_SF_TZSRG_R0_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r1_id_en` reader - Enable TrustZone Security ID for Range 1."]
pub type TZC_SF_TZSRG_R1_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_id_en` writer - Enable TrustZone Security ID for Range 1."]
pub type TZC_SF_TZSRG_R1_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r2_id_en` reader - Enable TrustZone Security ID for Range 2."]
pub type TZC_SF_TZSRG_R2_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_id_en` writer - Enable TrustZone Security ID for Range 2."]
pub type TZC_SF_TZSRG_R2_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r3_id_en` reader - Enable TrustZone Security ID for Range 3."]
pub type TZC_SF_TZSRG_R3_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_id_en` writer - Enable TrustZone Security ID for Range 3."]
pub type TZC_SF_TZSRG_R3_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_rx_id_en` reader - Enable TrustZone Security ID for Range X."]
pub type TZC_SF_TZSRG_RX_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_rx_id_en` writer - Enable TrustZone Security ID for Range X."]
pub type TZC_SF_TZSRG_RX_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r0_en` reader - Enable TrustZone Range 0."]
pub type TZC_SF_TZSRG_R0_EN_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r0_en` writer - Enable TrustZone Range 0."]
pub type TZC_SF_TZSRG_R0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r1_en` reader - Enable TrustZone Range 1."]
pub type TZC_SF_TZSRG_R1_EN_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_en` writer - Enable TrustZone Range 1."]
pub type TZC_SF_TZSRG_R1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r2_en` reader - Enable TrustZone Range 2."]
pub type TZC_SF_TZSRG_R2_EN_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_en` writer - Enable TrustZone Range 2."]
pub type TZC_SF_TZSRG_R2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r3_en` reader - Enable TrustZone Range 3."]
pub type TZC_SF_TZSRG_R3_EN_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_en` writer - Enable TrustZone Range 3."]
pub type TZC_SF_TZSRG_R3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_rx_en` reader - Enable TrustZone Range X."]
pub type TZC_SF_TZSRG_RX_EN_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_en` writer - Enable TrustZone Range X."]
pub type TZC_SF_TZSRG_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r0_lock` reader - Lock TrustZone Range 0."]
pub type TZC_SF_TZSRG_R0_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r0_lock` writer - Lock TrustZone Range 0."]
pub type TZC_SF_TZSRG_R0_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r1_lock` reader - Lock TrustZone Range 1."]
pub type TZC_SF_TZSRG_R1_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_lock` writer - Lock TrustZone Range 1."]
pub type TZC_SF_TZSRG_R1_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r2_lock` reader - Lock TrustZone Range 2."]
pub type TZC_SF_TZSRG_R2_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_lock` writer - Lock TrustZone Range 2."]
pub type TZC_SF_TZSRG_R2_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r3_lock` reader - Lock TrustZone Range 3."]
pub type TZC_SF_TZSRG_R3_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_lock` writer - Lock TrustZone Range 3."]
pub type TZC_SF_TZSRG_R3_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_rx_lock` reader - Lock TrustZone Range X."]
pub type TZC_SF_TZSRG_RX_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_lock` writer - Lock TrustZone Range X."]
pub type TZC_SF_TZSRG_RX_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Enable TrustZone Security ID for Range 0."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_id_en(&self) -> TZC_SF_TZSRG_R0_ID_EN_R {
        TZC_SF_TZSRG_R0_ID_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Enable TrustZone Security ID for Range 1."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_id_en(&self) -> TZC_SF_TZSRG_R1_ID_EN_R {
        TZC_SF_TZSRG_R1_ID_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Enable TrustZone Security ID for Range 2."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_id_en(&self) -> TZC_SF_TZSRG_R2_ID_EN_R {
        TZC_SF_TZSRG_R2_ID_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Enable TrustZone Security ID for Range 3."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_id_en(&self) -> TZC_SF_TZSRG_R3_ID_EN_R {
        TZC_SF_TZSRG_R3_ID_EN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Enable TrustZone Security ID for Range X."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_id_en(&self) -> TZC_SF_TZSRG_RX_ID_EN_R {
        TZC_SF_TZSRG_RX_ID_EN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable TrustZone Range 0."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_en(&self) -> TZC_SF_TZSRG_R0_EN_R {
        TZC_SF_TZSRG_R0_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable TrustZone Range 1."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_en(&self) -> TZC_SF_TZSRG_R1_EN_R {
        TZC_SF_TZSRG_R1_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable TrustZone Range 2."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_en(&self) -> TZC_SF_TZSRG_R2_EN_R {
        TZC_SF_TZSRG_R2_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable TrustZone Range 3."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_en(&self) -> TZC_SF_TZSRG_R3_EN_R {
        TZC_SF_TZSRG_R3_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable TrustZone Range X."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_en(&self) -> TZC_SF_TZSRG_RX_EN_R {
        TZC_SF_TZSRG_RX_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock TrustZone Range 0."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_lock(&self) -> TZC_SF_TZSRG_R0_LOCK_R {
        TZC_SF_TZSRG_R0_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Lock TrustZone Range 1."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_lock(&self) -> TZC_SF_TZSRG_R1_LOCK_R {
        TZC_SF_TZSRG_R1_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock TrustZone Range 2."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_lock(&self) -> TZC_SF_TZSRG_R2_LOCK_R {
        TZC_SF_TZSRG_R2_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lock TrustZone Range 3."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_lock(&self) -> TZC_SF_TZSRG_R3_LOCK_R {
        TZC_SF_TZSRG_R3_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock TrustZone Range X."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_lock(&self) -> TZC_SF_TZSRG_RX_LOCK_R {
        TZC_SF_TZSRG_RX_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable TrustZone Security ID for Range 0."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r0_id_en(&mut self) -> TZC_SF_TZSRG_R0_ID_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R0_ID_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Enable TrustZone Security ID for Range 1."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r1_id_en(&mut self) -> TZC_SF_TZSRG_R1_ID_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R1_ID_EN_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Enable TrustZone Security ID for Range 2."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r2_id_en(&mut self) -> TZC_SF_TZSRG_R2_ID_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R2_ID_EN_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Enable TrustZone Security ID for Range 3."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r3_id_en(&mut self) -> TZC_SF_TZSRG_R3_ID_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R3_ID_EN_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Enable TrustZone Security ID for Range X."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_rx_id_en(&mut self) -> TZC_SF_TZSRG_RX_ID_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_RX_ID_EN_W::new(self, 16)
    }
    #[doc = "Bit 20 - Enable TrustZone Range 0."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r0_en(&mut self) -> TZC_SF_TZSRG_R0_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R0_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable TrustZone Range 1."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r1_en(&mut self) -> TZC_SF_TZSRG_R1_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R1_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable TrustZone Range 2."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r2_en(&mut self) -> TZC_SF_TZSRG_R2_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R2_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable TrustZone Range 3."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r3_en(&mut self) -> TZC_SF_TZSRG_R3_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R3_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable TrustZone Range X."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_rx_en(&mut self) -> TZC_SF_TZSRG_RX_EN_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_RX_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Lock TrustZone Range 0."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r0_lock(&mut self) -> TZC_SF_TZSRG_R0_LOCK_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R0_LOCK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Lock TrustZone Range 1."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r1_lock(&mut self) -> TZC_SF_TZSRG_R1_LOCK_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R1_LOCK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Lock TrustZone Range 2."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r2_lock(&mut self) -> TZC_SF_TZSRG_R2_LOCK_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R2_LOCK_W::new(self, 27)
    }
    #[doc = "Bit 28 - Lock TrustZone Range 3."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_r3_lock(&mut self) -> TZC_SF_TZSRG_R3_LOCK_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_R3_LOCK_W::new(self, 28)
    }
    #[doc = "Bit 29 - Lock TrustZone Range X."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_rx_lock(&mut self) -> TZC_SF_TZSRG_RX_LOCK_W<TZC_SF_TZSRG_CTRL_SPEC> {
        TZC_SF_TZSRG_RX_LOCK_W::new(self, 29)
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
#[doc = "TrustZone Controller SF TrustZone Security Register Group Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SF_TZSRG_CTRL_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_ctrl to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
