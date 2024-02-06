#[doc = "Register `tzc_wram_tzsrg_r0` reader"]
pub type R = crate::R<TZC_WRAM_TZSRG_R0_SPEC>;
#[doc = "Register `tzc_wram_tzsrg_r0` writer"]
pub type W = crate::W<TZC_WRAM_TZSRG_R0_SPEC>;
#[doc = "Field `tzc_wram_tzsrg_r0_end` reader - End address for TrustZone Range 0 in WRam."]
pub type TZC_WRAM_TZSRG_R0_END_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_r0_end` writer - End address for TrustZone Range 0 in WRam."]
pub type TZC_WRAM_TZSRG_R0_END_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_wram_tzsrg_r0_start` reader - Start address for TrustZone Range 0 in WRam."]
pub type TZC_WRAM_TZSRG_R0_START_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_r0_start` writer - Start address for TrustZone Range 0 in WRam."]
pub type TZC_WRAM_TZSRG_R0_START_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address for TrustZone Range 0 in WRam."]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_end(&self) -> TZC_WRAM_TZSRG_R0_END_R {
        TZC_WRAM_TZSRG_R0_END_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address for TrustZone Range 0 in WRam."]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_start(&self) -> TZC_WRAM_TZSRG_R0_START_R {
        TZC_WRAM_TZSRG_R0_START_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address for TrustZone Range 0 in WRam."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wram_tzsrg_r0_end(&mut self) -> TZC_WRAM_TZSRG_R0_END_W<TZC_WRAM_TZSRG_R0_SPEC> {
        TZC_WRAM_TZSRG_R0_END_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address for TrustZone Range 0 in WRam."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wram_tzsrg_r0_start(&mut self) -> TZC_WRAM_TZSRG_R0_START_W<TZC_WRAM_TZSRG_R0_SPEC> {
        TZC_WRAM_TZSRG_R0_START_W::new(self, 16)
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
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Range 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_r0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_WRAM_TZSRG_R0_SPEC;
impl crate::RegisterSpec for TZC_WRAM_TZSRG_R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TZC_WRAM_TZSRG_R0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_wram_tzsrg_r0::W`](W) writer structure"]
impl crate::Writable for TZC_WRAM_TZSRG_R0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_wram_tzsrg_r0 to value 0"]
impl crate::Resettable for TZC_WRAM_TZSRG_R0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
