#[doc = "Register `cgen_cfg2` reader"]
pub type R = crate::R<CGEN_CFG2_SPEC>;
#[doc = "Register `cgen_cfg2` writer"]
pub type W = crate::W<CGEN_CFG2_SPEC>;
#[doc = "Field `cgen_s0` reader - "]
pub type CGEN_S0_R = crate::BitReader;
#[doc = "Field `cgen_s0` writer - "]
pub type CGEN_S0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s2_wifi` reader - "]
pub type CGEN_S2_WIFI_R = crate::BitReader;
#[doc = "Field `cgen_s2_wifi` writer - "]
pub type CGEN_S2_WIFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s3_bt_ble2` reader - "]
pub type CGEN_S3_BT_BLE2_R = crate::BitReader;
#[doc = "Field `cgen_s3_bt_ble2` writer - "]
pub type CGEN_S3_BT_BLE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s3_m1542` reader - "]
pub type CGEN_S3_M1542_R = crate::BitReader;
#[doc = "Field `cgen_s3_m1542` writer - "]
pub type CGEN_S3_M1542_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_emi_misc` reader - "]
pub type CGEN_S1_EXT_EMI_MISC_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_emi_misc` writer - "]
pub type CGEN_S1_EXT_EMI_MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` reader - "]
pub type CGEN_S1_EXT_PSRAM0_CTRL_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` writer - "]
pub type CGEN_S1_EXT_PSRAM0_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_psram_ctrl` reader - "]
pub type CGEN_S1_EXT_PSRAM_CTRL_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_psram_ctrl` writer - "]
pub type CGEN_S1_EXT_PSRAM_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_usb` reader - "]
pub type CGEN_S1_EXT_USB_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_usb` writer - "]
pub type CGEN_S1_EXT_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_mix2` reader - "]
pub type CGEN_S1_EXT_MIX2_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_mix2` writer - "]
pub type CGEN_S1_EXT_MIX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_audio` reader - "]
pub type CGEN_S1_EXT_AUDIO_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_audio` writer - "]
pub type CGEN_S1_EXT_AUDIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_sdh` reader - "]
pub type CGEN_S1_EXT_SDH_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_sdh` writer - "]
pub type CGEN_S1_EXT_SDH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_emac` reader - "]
pub type CGEN_S1_EXT_EMAC_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_emac` writer - "]
pub type CGEN_S1_EXT_EMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_dma2` reader - "]
pub type CGEN_S1_EXT_DMA2_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_dma2` writer - "]
pub type CGEN_S1_EXT_DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_pio` reader - "]
pub type CGEN_S1_EXT_PIO_R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_pio` writer - "]
pub type CGEN_S1_EXT_PIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s0(&self) -> CGEN_S0_R {
        CGEN_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s2_wifi(&self) -> CGEN_S2_WIFI_R {
        CGEN_S2_WIFI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_s3_bt_ble2(&self) -> CGEN_S3_BT_BLE2_R {
        CGEN_S3_BT_BLE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s3_m1542(&self) -> CGEN_S3_M1542_R {
        CGEN_S3_M1542_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1_ext_emi_misc(&self) -> CGEN_S1_EXT_EMI_MISC_R {
        CGEN_S1_EXT_EMI_MISC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram0_ctrl(&self) -> CGEN_S1_EXT_PSRAM0_CTRL_R {
        CGEN_S1_EXT_PSRAM0_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram_ctrl(&self) -> CGEN_S1_EXT_PSRAM_CTRL_R {
        CGEN_S1_EXT_PSRAM_CTRL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1_ext_usb(&self) -> CGEN_S1_EXT_USB_R {
        CGEN_S1_EXT_USB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1_ext_mix2(&self) -> CGEN_S1_EXT_MIX2_R {
        CGEN_S1_EXT_MIX2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1_ext_audio(&self) -> CGEN_S1_EXT_AUDIO_R {
        CGEN_S1_EXT_AUDIO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1_ext_sdh(&self) -> CGEN_S1_EXT_SDH_R {
        CGEN_S1_EXT_SDH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1_ext_emac(&self) -> CGEN_S1_EXT_EMAC_R {
        CGEN_S1_EXT_EMAC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cgen_s1_ext_dma2(&self) -> CGEN_S1_EXT_DMA2_R {
        CGEN_S1_EXT_DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cgen_s1_ext_pio(&self) -> CGEN_S1_EXT_PIO_R {
        CGEN_S1_EXT_PIO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s0(&mut self) -> CGEN_S0_W<CGEN_CFG2_SPEC> {
        CGEN_S0_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s2_wifi(&mut self) -> CGEN_S2_WIFI_W<CGEN_CFG2_SPEC> {
        CGEN_S2_WIFI_W::new(self, 4)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s3_bt_ble2(&mut self) -> CGEN_S3_BT_BLE2_W<CGEN_CFG2_SPEC> {
        CGEN_S3_BT_BLE2_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s3_m1542(&mut self) -> CGEN_S3_M1542_W<CGEN_CFG2_SPEC> {
        CGEN_S3_M1542_W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_emi_misc(&mut self) -> CGEN_S1_EXT_EMI_MISC_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_EMI_MISC_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_psram0_ctrl(&mut self) -> CGEN_S1_EXT_PSRAM0_CTRL_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_PSRAM0_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_psram_ctrl(&mut self) -> CGEN_S1_EXT_PSRAM_CTRL_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_PSRAM_CTRL_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_usb(&mut self) -> CGEN_S1_EXT_USB_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_USB_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_mix2(&mut self) -> CGEN_S1_EXT_MIX2_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_MIX2_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_audio(&mut self) -> CGEN_S1_EXT_AUDIO_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_AUDIO_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_sdh(&mut self) -> CGEN_S1_EXT_SDH_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_SDH_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_emac(&mut self) -> CGEN_S1_EXT_EMAC_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_EMAC_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_dma2(&mut self) -> CGEN_S1_EXT_DMA2_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_DMA2_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_pio(&mut self) -> CGEN_S1_EXT_PIO_W<CGEN_CFG2_SPEC> {
        CGEN_S1_EXT_PIO_W::new(self, 25)
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
#[doc = "cgen_s1_ext + cgen_s3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGEN_CFG2_SPEC;
impl crate::RegisterSpec for CGEN_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg2::R`](R) reader structure"]
impl crate::Readable for CGEN_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg2::W`](W) writer structure"]
impl crate::Writable for CGEN_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg2 to value 0"]
impl crate::Resettable for CGEN_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
