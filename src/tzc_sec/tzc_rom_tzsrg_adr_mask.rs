#[doc = "Register `tzc_rom_tzsrg_adr_mask` reader"]
pub type R = crate::R<TZC_ROM_TZSRG_ADR_MASK_SPEC>;
#[doc = "Register `tzc_rom_tzsrg_adr_mask` writer"]
pub type W = crate::W<TZC_ROM_TZSRG_ADR_MASK_SPEC>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask` reader - TZC ROM TrustZone Security Region Address Mask"]
pub type TZC_ROM_TZSRG_ADR_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask` writer - TZC ROM TrustZone Security Region Address Mask"]
pub type TZC_ROM_TZSRG_ADR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask_lock` reader - TZC ROM TrustZone Security Region Address Mask Lock"]
pub type TZC_ROM_TZSRG_ADR_MASK_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_adr_mask_lock` writer - TZC ROM TrustZone Security Region Address Mask Lock"]
pub type TZC_ROM_TZSRG_ADR_MASK_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - TZC ROM TrustZone Security Region Address Mask"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask(&self) -> TZC_ROM_TZSRG_ADR_MASK_R {
        TZC_ROM_TZSRG_ADR_MASK_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - TZC ROM TrustZone Security Region Address Mask Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask_lock(&self) -> TZC_ROM_TZSRG_ADR_MASK_LOCK_R {
        TZC_ROM_TZSRG_ADR_MASK_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TZC ROM TrustZone Security Region Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_adr_mask(
        &mut self,
    ) -> TZC_ROM_TZSRG_ADR_MASK_W<TZC_ROM_TZSRG_ADR_MASK_SPEC> {
        TZC_ROM_TZSRG_ADR_MASK_W::new(self, 0)
    }
    #[doc = "Bit 16 - TZC ROM TrustZone Security Region Address Mask Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TZC_ROM_TZSRG_ADR_MASK_LOCK_W<TZC_ROM_TZSRG_ADR_MASK_SPEC> {
        TZC_ROM_TZSRG_ADR_MASK_LOCK_W::new(self, 16)
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
#[doc = "TZC ROM TrustZone Security Region Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_ROM_TZSRG_ADR_MASK_SPEC;
impl crate::RegisterSpec for TZC_ROM_TZSRG_ADR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TZC_ROM_TZSRG_ADR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TZC_ROM_TZSRG_ADR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_rom_tzsrg_adr_mask to value 0"]
impl crate::Resettable for TZC_ROM_TZSRG_ADR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
