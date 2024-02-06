#[doc = "Register `tzc_wifi_dbg` reader"]
pub type R = crate::R<TZC_WIFI_DBG_SPEC>;
#[doc = "Register `tzc_wifi_dbg` writer"]
pub type W = crate::W<TZC_WIFI_DBG_SPEC>;
#[doc = "Field `tzc_mac_dbg_dis` reader - Disable WiFi MAC Debug."]
pub type TZC_MAC_DBG_DIS_R = crate::BitReader;
#[doc = "Field `tzc_mac_dbg_dis` writer - Disable WiFi MAC Debug."]
pub type TZC_MAC_DBG_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable WiFi MAC Debug."]
    #[inline(always)]
    pub fn tzc_mac_dbg_dis(&self) -> TZC_MAC_DBG_DIS_R {
        TZC_MAC_DBG_DIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable WiFi MAC Debug."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mac_dbg_dis(&mut self) -> TZC_MAC_DBG_DIS_W<TZC_WIFI_DBG_SPEC> {
        TZC_MAC_DBG_DIS_W::new(self, 0)
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
#[doc = "TrustZone Controller WiFi Debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wifi_dbg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wifi_dbg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_WIFI_DBG_SPEC;
impl crate::RegisterSpec for TZC_WIFI_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wifi_dbg::R`](R) reader structure"]
impl crate::Readable for TZC_WIFI_DBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_wifi_dbg::W`](W) writer structure"]
impl crate::Writable for TZC_WIFI_DBG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_wifi_dbg to value 0"]
impl crate::Resettable for TZC_WIFI_DBG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
