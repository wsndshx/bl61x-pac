#[doc = "Register `tzc_bmx_tzmid_lock` reader"]
pub type R = crate::R<TZC_BMX_TZMID_LOCK_SPEC>;
#[doc = "Register `tzc_bmx_tzmid_lock` writer"]
pub type W = crate::W<TZC_BMX_TZMID_LOCK_SPEC>;
#[doc = "Field `tzc_usb_tzmid_lock` reader - TZC USB TrustZone Master ID Lock"]
pub type TZC_USB_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_lock` writer - TZC USB TrustZone Master ID Lock"]
pub type TZC_USB_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid_lock` reader - TZC WiFi TrustZone Master ID Lock"]
pub type TZC_WIFI_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_lock` writer - TZC WiFi TrustZone Master ID Lock"]
pub type TZC_WIFI_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid_lock` reader - TZC CCI TrustZone Master ID Lock"]
pub type TZC_CCI_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_lock` writer - TZC CCI TrustZone Master ID Lock"]
pub type TZC_CCI_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid_lock` reader - TZC SDHM TrustZone Master ID Lock"]
pub type TZC_SDHM_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_lock` writer - TZC SDHM TrustZone Master ID Lock"]
pub type TZC_SDHM_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid_lock` reader - TZC EMAC-A TrustZone Master ID Lock"]
pub type TZC_EMAC_A_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid_lock` writer - TZC EMAC-A TrustZone Master ID Lock"]
pub type TZC_EMAC_A_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid_lock` reader - TZC CPU TrustZone Master ID Lock"]
pub type TZC_CPU_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_lock` writer - TZC CPU TrustZone Master ID Lock"]
pub type TZC_CPU_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid_lock` reader - TZC DMA TrustZone Master ID Lock"]
pub type TZC_DMA_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_lock` writer - TZC DMA TrustZone Master ID Lock"]
pub type TZC_DMA_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdum_tzmid_lock` reader - TZC SDUM TrustZone Master ID Lock"]
pub type TZC_SDUM_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sdum_tzmid_lock` writer - TZC SDUM TrustZone Master ID Lock"]
pub type TZC_SDUM_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - TZC USB TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_lock(&self) -> TZC_USB_TZMID_LOCK_R {
        TZC_USB_TZMID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TZC WiFi TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_lock(&self) -> TZC_WIFI_TZMID_LOCK_R {
        TZC_WIFI_TZMID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TZC CCI TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_lock(&self) -> TZC_CCI_TZMID_LOCK_R {
        TZC_CCI_TZMID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TZC SDHM TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_lock(&self) -> TZC_SDHM_TZMID_LOCK_R {
        TZC_SDHM_TZMID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TZC EMAC-A TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_lock(&self) -> TZC_EMAC_A_TZMID_LOCK_R {
        TZC_EMAC_A_TZMID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TZC CPU TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_lock(&self) -> TZC_CPU_TZMID_LOCK_R {
        TZC_CPU_TZMID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TZC DMA TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_lock(&self) -> TZC_DMA_TZMID_LOCK_R {
        TZC_DMA_TZMID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TZC SDUM TrustZone Master ID Lock"]
    #[inline(always)]
    pub fn tzc_sdum_tzmid_lock(&self) -> TZC_SDUM_TZMID_LOCK_R {
        TZC_SDUM_TZMID_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TZC USB TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_usb_tzmid_lock(&mut self) -> TZC_USB_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_USB_TZMID_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - TZC WiFi TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wifi_tzmid_lock(&mut self) -> TZC_WIFI_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_WIFI_TZMID_LOCK_W::new(self, 3)
    }
    #[doc = "Bit 4 - TZC CCI TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cci_tzmid_lock(&mut self) -> TZC_CCI_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_CCI_TZMID_LOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - TZC SDHM TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdhm_tzmid_lock(&mut self) -> TZC_SDHM_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_SDHM_TZMID_LOCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - TZC EMAC-A TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_emac_a_tzmid_lock(&mut self) -> TZC_EMAC_A_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_EMAC_A_TZMID_LOCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - TZC CPU TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cpu_tzmid_lock(&mut self) -> TZC_CPU_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_CPU_TZMID_LOCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - TZC DMA TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_dma_tzmid_lock(&mut self) -> TZC_DMA_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_DMA_TZMID_LOCK_W::new(self, 8)
    }
    #[doc = "Bit 11 - TZC SDUM TrustZone Master ID Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdum_tzmid_lock(&mut self) -> TZC_SDUM_TZMID_LOCK_W<TZC_BMX_TZMID_LOCK_SPEC> {
        TZC_SDUM_TZMID_LOCK_W::new(self, 11)
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
#[doc = "TZC Bus Matrix TrustZone Master ID Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_tzmid_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_BMX_TZMID_LOCK_SPEC;
impl crate::RegisterSpec for TZC_BMX_TZMID_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TZC_BMX_TZMID_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_tzmid_lock::W`](W) writer structure"]
impl crate::Writable for TZC_BMX_TZMID_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_tzmid_lock to value 0"]
impl crate::Resettable for TZC_BMX_TZMID_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
