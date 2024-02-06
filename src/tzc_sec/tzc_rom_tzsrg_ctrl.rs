#[doc = "Register `tzc_rom_tzsrg_ctrl` reader"]
pub type R = crate::R<TZC_ROM_TZSRG_CTRL_SPEC>;
#[doc = "Register `tzc_rom_tzsrg_ctrl` writer"]
pub type W = crate::W<TZC_ROM_TZSRG_CTRL_SPEC>;
#[doc = "Field `tzc_rom_tzsrg_r0_id_en` reader - TZC ROM TrustZone Security Region 0 ID Enable"]
pub type TZC_ROM_TZSRG_R0_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r0_id_en` writer - TZC ROM TrustZone Security Region 0 ID Enable"]
pub type TZC_ROM_TZSRG_R0_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_rom_tzsrg_r1_id_en` reader - TZC ROM TrustZone Security Region 1 ID Enable"]
pub type TZC_ROM_TZSRG_R1_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r1_id_en` writer - TZC ROM TrustZone Security Region 1 ID Enable"]
pub type TZC_ROM_TZSRG_R1_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_rom_tzsrg_r2_id_en` reader - TZC ROM TrustZone Security Region 2 ID Enable"]
pub type TZC_ROM_TZSRG_R2_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r2_id_en` writer - TZC ROM TrustZone Security Region 2 ID Enable"]
pub type TZC_ROM_TZSRG_R2_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_rom_tzsrg_rx_id_en` reader - TZC ROM TrustZone Security Region RX ID Enable"]
pub type TZC_ROM_TZSRG_RX_ID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_rx_id_en` writer - TZC ROM TrustZone Security Region RX ID Enable"]
pub type TZC_ROM_TZSRG_RX_ID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_rom_tzsrg_r0_en` reader - TZC ROM TrustZone Security Region 0 Enable"]
pub type TZC_ROM_TZSRG_R0_EN_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r0_en` writer - TZC ROM TrustZone Security Region 0 Enable"]
pub type TZC_ROM_TZSRG_R0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r1_en` reader - TZC ROM TrustZone Security Region 1 Enable"]
pub type TZC_ROM_TZSRG_R1_EN_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_en` writer - TZC ROM TrustZone Security Region 1 Enable"]
pub type TZC_ROM_TZSRG_R1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r2_en` reader - TZC ROM TrustZone Security Region 2 Enable"]
pub type TZC_ROM_TZSRG_R2_EN_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_en` writer - TZC ROM TrustZone Security Region 2 Enable"]
pub type TZC_ROM_TZSRG_R2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_rx_en` reader - TZC ROM TrustZone Security Region RX Enable"]
pub type TZC_ROM_TZSRG_RX_EN_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_rx_en` writer - TZC ROM TrustZone Security Region RX Enable"]
pub type TZC_ROM_TZSRG_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bus_rmp_en` reader - TZC Bus Region Mapping Enable"]
pub type TZC_BUS_RMP_EN_R = crate::BitReader;
#[doc = "Field `tzc_bus_rmp_en` writer - TZC Bus Region Mapping Enable"]
pub type TZC_BUS_RMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bus_rmp_en_lock` reader - TZC Bus Region Mapping Enable Lock"]
pub type TZC_BUS_RMP_EN_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bus_rmp_en_lock` writer - TZC Bus Region Mapping Enable Lock"]
pub type TZC_BUS_RMP_EN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r0_lock` reader - TZC ROM TrustZone Security Region 0 Lock"]
pub type TZC_ROM_TZSRG_R0_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r0_lock` writer - TZC ROM TrustZone Security Region 0 Lock"]
pub type TZC_ROM_TZSRG_R0_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r1_lock` reader - TZC ROM TrustZone Security Region 1 Lock"]
pub type TZC_ROM_TZSRG_R1_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_lock` writer - TZC ROM TrustZone Security Region 1 Lock"]
pub type TZC_ROM_TZSRG_R1_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r2_lock` reader - TZC ROM TrustZone Security Region 2 Lock"]
pub type TZC_ROM_TZSRG_R2_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_lock` writer - TZC ROM TrustZone Security Region 2 Lock"]
pub type TZC_ROM_TZSRG_R2_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_rx_lock` reader - TZC ROM TrustZone Security Region RX Lock"]
pub type TZC_ROM_TZSRG_RX_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_rx_lock` writer - TZC ROM TrustZone Security Region RX Lock"]
pub type TZC_ROM_TZSRG_RX_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sboot_done` reader - TZC Secure Boot Done"]
pub type TZC_SBOOT_DONE_R = crate::FieldReader;
#[doc = "Field `tzc_sboot_done` writer - TZC Secure Boot Done"]
pub type TZC_SBOOT_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TZC ROM TrustZone Security Region 0 ID Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_id_en(&self) -> TZC_ROM_TZSRG_R0_ID_EN_R {
        TZC_ROM_TZSRG_R0_ID_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TZC ROM TrustZone Security Region 1 ID Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_id_en(&self) -> TZC_ROM_TZSRG_R1_ID_EN_R {
        TZC_ROM_TZSRG_R1_ID_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TZC ROM TrustZone Security Region 2 ID Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_id_en(&self) -> TZC_ROM_TZSRG_R2_ID_EN_R {
        TZC_ROM_TZSRG_R2_ID_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TZC ROM TrustZone Security Region RX ID Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_rx_id_en(&self) -> TZC_ROM_TZSRG_RX_ID_EN_R {
        TZC_ROM_TZSRG_RX_ID_EN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - TZC ROM TrustZone Security Region 0 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_en(&self) -> TZC_ROM_TZSRG_R0_EN_R {
        TZC_ROM_TZSRG_R0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TZC ROM TrustZone Security Region 1 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_en(&self) -> TZC_ROM_TZSRG_R1_EN_R {
        TZC_ROM_TZSRG_R1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TZC ROM TrustZone Security Region 2 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_en(&self) -> TZC_ROM_TZSRG_R2_EN_R {
        TZC_ROM_TZSRG_R2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TZC ROM TrustZone Security Region RX Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_rx_en(&self) -> TZC_ROM_TZSRG_RX_EN_R {
        TZC_ROM_TZSRG_RX_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - TZC Bus Region Mapping Enable"]
    #[inline(always)]
    pub fn tzc_bus_rmp_en(&self) -> TZC_BUS_RMP_EN_R {
        TZC_BUS_RMP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TZC Bus Region Mapping Enable Lock"]
    #[inline(always)]
    pub fn tzc_bus_rmp_en_lock(&self) -> TZC_BUS_RMP_EN_LOCK_R {
        TZC_BUS_RMP_EN_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TZC ROM TrustZone Security Region 0 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_lock(&self) -> TZC_ROM_TZSRG_R0_LOCK_R {
        TZC_ROM_TZSRG_R0_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TZC ROM TrustZone Security Region 1 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_lock(&self) -> TZC_ROM_TZSRG_R1_LOCK_R {
        TZC_ROM_TZSRG_R1_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TZC ROM TrustZone Security Region 2 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_lock(&self) -> TZC_ROM_TZSRG_R2_LOCK_R {
        TZC_ROM_TZSRG_R2_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TZC ROM TrustZone Security Region RX Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_rx_lock(&self) -> TZC_ROM_TZSRG_RX_LOCK_R {
        TZC_ROM_TZSRG_RX_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - TZC Secure Boot Done"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TZC_SBOOT_DONE_R {
        TZC_SBOOT_DONE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TZC ROM TrustZone Security Region 0 ID Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r0_id_en(&mut self) -> TZC_ROM_TZSRG_R0_ID_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R0_ID_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - TZC ROM TrustZone Security Region 1 ID Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r1_id_en(&mut self) -> TZC_ROM_TZSRG_R1_ID_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R1_ID_EN_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - TZC ROM TrustZone Security Region 2 ID Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r2_id_en(&mut self) -> TZC_ROM_TZSRG_R2_ID_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R2_ID_EN_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - TZC ROM TrustZone Security Region RX ID Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_rx_id_en(&mut self) -> TZC_ROM_TZSRG_RX_ID_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_RX_ID_EN_W::new(self, 12)
    }
    #[doc = "Bit 16 - TZC ROM TrustZone Security Region 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r0_en(&mut self) -> TZC_ROM_TZSRG_R0_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R0_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TZC ROM TrustZone Security Region 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r1_en(&mut self) -> TZC_ROM_TZSRG_R1_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R1_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TZC ROM TrustZone Security Region 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r2_en(&mut self) -> TZC_ROM_TZSRG_R2_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R2_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - TZC ROM TrustZone Security Region RX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_rx_en(&mut self) -> TZC_ROM_TZSRG_RX_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_RX_EN_W::new(self, 19)
    }
    #[doc = "Bit 22 - TZC Bus Region Mapping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bus_rmp_en(&mut self) -> TZC_BUS_RMP_EN_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_BUS_RMP_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - TZC Bus Region Mapping Enable Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bus_rmp_en_lock(&mut self) -> TZC_BUS_RMP_EN_LOCK_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_BUS_RMP_EN_LOCK_W::new(self, 23)
    }
    #[doc = "Bit 24 - TZC ROM TrustZone Security Region 0 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r0_lock(&mut self) -> TZC_ROM_TZSRG_R0_LOCK_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R0_LOCK_W::new(self, 24)
    }
    #[doc = "Bit 25 - TZC ROM TrustZone Security Region 1 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r1_lock(&mut self) -> TZC_ROM_TZSRG_R1_LOCK_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R1_LOCK_W::new(self, 25)
    }
    #[doc = "Bit 26 - TZC ROM TrustZone Security Region 2 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_r2_lock(&mut self) -> TZC_ROM_TZSRG_R2_LOCK_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_R2_LOCK_W::new(self, 26)
    }
    #[doc = "Bit 27 - TZC ROM TrustZone Security Region RX Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_rx_lock(&mut self) -> TZC_ROM_TZSRG_RX_LOCK_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_ROM_TZSRG_RX_LOCK_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - TZC Secure Boot Done"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sboot_done(&mut self) -> TZC_SBOOT_DONE_W<TZC_ROM_TZSRG_CTRL_SPEC> {
        TZC_SBOOT_DONE_W::new(self, 28)
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
#[doc = "TZC ROM TrustZone Security Region Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_ROM_TZSRG_CTRL_SPEC;
impl crate::RegisterSpec for TZC_ROM_TZSRG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TZC_ROM_TZSRG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TZC_ROM_TZSRG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_rom_tzsrg_ctrl to value 0"]
impl crate::Resettable for TZC_ROM_TZSRG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
