#[doc = "Register `tzc_bmx_tzmid` reader"]
pub type R = crate::R<TZC_BMX_TZMID_SPEC>;
#[doc = "Register `tzc_bmx_tzmid` writer"]
pub type W = crate::W<TZC_BMX_TZMID_SPEC>;
#[doc = "Field `tzc_usb_tzmid` reader - TZC USB TrustZone Master ID"]
pub type TZC_USB_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid` writer - TZC USB TrustZone Master ID"]
pub type TZC_USB_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid` reader - TZC WiFi TrustZone Master ID"]
pub type TZC_WIFI_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid` writer - TZC WiFi TrustZone Master ID"]
pub type TZC_WIFI_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid` reader - TZC CCI TrustZone Master ID"]
pub type TZC_CCI_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid` writer - TZC CCI TrustZone Master ID"]
pub type TZC_CCI_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid` reader - TZC SDHM TrustZone Master ID"]
pub type TZC_SDHM_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid` writer - TZC SDHM TrustZone Master ID"]
pub type TZC_SDHM_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid` reader - TZC EMAC-A TrustZone Master ID"]
pub type TZC_EMAC_A_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid` writer - TZC EMAC-A TrustZone Master ID"]
pub type TZC_EMAC_A_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid` reader - TZC CPU TrustZone Master ID"]
pub type TZC_CPU_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid` writer - TZC CPU TrustZone Master ID"]
pub type TZC_CPU_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid` reader - TZC DMA TrustZone Master ID"]
pub type TZC_DMA_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid` writer - TZC DMA TrustZone Master ID"]
pub type TZC_DMA_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdum_tzmid` reader - TZC SDUM TrustZone Master ID"]
pub type TZC_SDUM_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_sdum_tzmid` writer - TZC SDUM TrustZone Master ID"]
pub type TZC_SDUM_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_usb_tzmid_sel` reader - TZC USB TrustZone Master ID Selection"]
pub type TZC_USB_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_sel` writer - TZC USB TrustZone Master ID Selection"]
pub type TZC_USB_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid_sel` reader - TZC WiFi TrustZone Master ID Selection"]
pub type TZC_WIFI_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_sel` writer - TZC WiFi TrustZone Master ID Selection"]
pub type TZC_WIFI_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid_sel` reader - TZC CCI TrustZone Master ID Selection"]
pub type TZC_CCI_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_sel` writer - TZC CCI TrustZone Master ID Selection"]
pub type TZC_CCI_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid_sel` reader - TZC SDHM TrustZone Master ID Selection"]
pub type TZC_SDHM_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_sel` writer - TZC SDHM TrustZone Master ID Selection"]
pub type TZC_SDHM_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid_sel` reader - TZC EMAC-A TrustZone Master ID Selection"]
pub type TZC_EMAC_A_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid_sel` writer - TZC EMAC-A TrustZone Master ID Selection"]
pub type TZC_EMAC_A_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid_sel` reader - TZC CPU TrustZone Master ID Selection"]
pub type TZC_CPU_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_sel` writer - TZC CPU TrustZone Master ID Selection"]
pub type TZC_CPU_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid_sel` reader - TZC DMA TrustZone Master ID Selection"]
pub type TZC_DMA_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_sel` writer - TZC DMA TrustZone Master ID Selection"]
pub type TZC_DMA_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdum_tzmid_sel` reader - TZC SDUM TrustZone Master ID Selection"]
pub type TZC_SDUM_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_sdum_tzmid_sel` writer - TZC SDUM TrustZone Master ID Selection"]
pub type TZC_SDUM_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - TZC USB TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_usb_tzmid(&self) -> TZC_USB_TZMID_R {
        TZC_USB_TZMID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TZC WiFi TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid(&self) -> TZC_WIFI_TZMID_R {
        TZC_WIFI_TZMID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TZC CCI TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_cci_tzmid(&self) -> TZC_CCI_TZMID_R {
        TZC_CCI_TZMID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TZC SDHM TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid(&self) -> TZC_SDHM_TZMID_R {
        TZC_SDHM_TZMID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TZC EMAC-A TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid(&self) -> TZC_EMAC_A_TZMID_R {
        TZC_EMAC_A_TZMID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TZC CPU TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid(&self) -> TZC_CPU_TZMID_R {
        TZC_CPU_TZMID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TZC DMA TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_dma_tzmid(&self) -> TZC_DMA_TZMID_R {
        TZC_DMA_TZMID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TZC SDUM TrustZone Master ID"]
    #[inline(always)]
    pub fn tzc_sdum_tzmid(&self) -> TZC_SDUM_TZMID_R {
        TZC_SDUM_TZMID_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - TZC USB TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_sel(&self) -> TZC_USB_TZMID_SEL_R {
        TZC_USB_TZMID_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TZC WiFi TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_sel(&self) -> TZC_WIFI_TZMID_SEL_R {
        TZC_WIFI_TZMID_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TZC CCI TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_sel(&self) -> TZC_CCI_TZMID_SEL_R {
        TZC_CCI_TZMID_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TZC SDHM TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_sel(&self) -> TZC_SDHM_TZMID_SEL_R {
        TZC_SDHM_TZMID_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TZC EMAC-A TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_sel(&self) -> TZC_EMAC_A_TZMID_SEL_R {
        TZC_EMAC_A_TZMID_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TZC CPU TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_sel(&self) -> TZC_CPU_TZMID_SEL_R {
        TZC_CPU_TZMID_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TZC DMA TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_sel(&self) -> TZC_DMA_TZMID_SEL_R {
        TZC_DMA_TZMID_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - TZC SDUM TrustZone Master ID Selection"]
    #[inline(always)]
    pub fn tzc_sdum_tzmid_sel(&self) -> TZC_SDUM_TZMID_SEL_R {
        TZC_SDUM_TZMID_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TZC USB TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_usb_tzmid(&mut self) -> TZC_USB_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_USB_TZMID_W::new(self, 2)
    }
    #[doc = "Bit 3 - TZC WiFi TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wifi_tzmid(&mut self) -> TZC_WIFI_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_WIFI_TZMID_W::new(self, 3)
    }
    #[doc = "Bit 4 - TZC CCI TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cci_tzmid(&mut self) -> TZC_CCI_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_CCI_TZMID_W::new(self, 4)
    }
    #[doc = "Bit 5 - TZC SDHM TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdhm_tzmid(&mut self) -> TZC_SDHM_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_SDHM_TZMID_W::new(self, 5)
    }
    #[doc = "Bit 6 - TZC EMAC-A TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_emac_a_tzmid(&mut self) -> TZC_EMAC_A_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_EMAC_A_TZMID_W::new(self, 6)
    }
    #[doc = "Bit 7 - TZC CPU TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cpu_tzmid(&mut self) -> TZC_CPU_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_CPU_TZMID_W::new(self, 7)
    }
    #[doc = "Bit 8 - TZC DMA TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_dma_tzmid(&mut self) -> TZC_DMA_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_DMA_TZMID_W::new(self, 8)
    }
    #[doc = "Bit 11 - TZC SDUM TrustZone Master ID"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdum_tzmid(&mut self) -> TZC_SDUM_TZMID_W<TZC_BMX_TZMID_SPEC> {
        TZC_SDUM_TZMID_W::new(self, 11)
    }
    #[doc = "Bit 18 - TZC USB TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_usb_tzmid_sel(&mut self) -> TZC_USB_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_USB_TZMID_SEL_W::new(self, 18)
    }
    #[doc = "Bit 19 - TZC WiFi TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wifi_tzmid_sel(&mut self) -> TZC_WIFI_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_WIFI_TZMID_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - TZC CCI TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cci_tzmid_sel(&mut self) -> TZC_CCI_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_CCI_TZMID_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - TZC SDHM TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdhm_tzmid_sel(&mut self) -> TZC_SDHM_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_SDHM_TZMID_SEL_W::new(self, 21)
    }
    #[doc = "Bit 22 - TZC EMAC-A TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_emac_a_tzmid_sel(&mut self) -> TZC_EMAC_A_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_EMAC_A_TZMID_SEL_W::new(self, 22)
    }
    #[doc = "Bit 23 - TZC CPU TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cpu_tzmid_sel(&mut self) -> TZC_CPU_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_CPU_TZMID_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - TZC DMA TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_dma_tzmid_sel(&mut self) -> TZC_DMA_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_DMA_TZMID_SEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - TZC SDUM TrustZone Master ID Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdum_tzmid_sel(&mut self) -> TZC_SDUM_TZMID_SEL_W<TZC_BMX_TZMID_SPEC> {
        TZC_SDUM_TZMID_SEL_W::new(self, 27)
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
#[doc = "TZC Bus Matrix TrustZone Master IDs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_tzmid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_tzmid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_BMX_TZMID_SPEC;
impl crate::RegisterSpec for TZC_BMX_TZMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TZC_BMX_TZMID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_tzmid::W`](W) writer structure"]
impl crate::Writable for TZC_BMX_TZMID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_tzmid to value 0"]
impl crate::Resettable for TZC_BMX_TZMID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
