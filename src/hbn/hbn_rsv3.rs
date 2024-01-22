#[doc = "Register `HBN_RSV3` reader"]
pub type R = crate::R<HBN_RSV3_SPEC>;
#[doc = "Register `HBN_RSV3` writer"]
pub type W = crate::W<HBN_RSV3_SPEC>;
#[doc = "Field `hbn_xtal_type` reader - "]
pub type HBN_XTAL_TYPE_R = crate::FieldReader;
#[doc = "Field `hbn_xtal_type` writer - "]
pub type HBN_XTAL_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_xtal_sts` reader - "]
pub type HBN_XTAL_STS_R = crate::FieldReader;
#[doc = "Field `hbn_xtal_sts` writer - "]
pub type HBN_XTAL_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_flash_power_dly` reader - "]
pub type HBN_FLASH_POWER_DLY_R = crate::FieldReader;
#[doc = "Field `hbn_flash_power_dly` writer - "]
pub type HBN_FLASH_POWER_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbn_flash_power_sts` reader - "]
pub type HBN_FLASH_POWER_STS_R = crate::FieldReader;
#[doc = "Field `hbn_flash_power_sts` writer - "]
pub type HBN_FLASH_POWER_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_keep_pin` reader - "]
pub type PDS_GPIO_KEEP_PIN_R = crate::FieldReader;
#[doc = "Field `pds_gpio_keep_pin` writer - "]
pub type PDS_GPIO_KEEP_PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_gpio_keep_pin` reader - "]
pub type HBN_GPIO_KEEP_PIN_R = crate::FieldReader;
#[doc = "Field `hbn_gpio_keep_pin` writer - "]
pub type HBN_GPIO_KEEP_PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_keep_sts` reader - "]
pub type PDS_GPIO_KEEP_STS_R = crate::FieldReader;
#[doc = "Field `pds_gpio_keep_sts` writer - "]
pub type PDS_GPIO_KEEP_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_gpio_keep_sts` reader - "]
pub type HBN_GPIO_KEEP_STS_R = crate::FieldReader;
#[doc = "Field `hbn_gpio_keep_sts` writer - "]
pub type HBN_GPIO_KEEP_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hbn_xtal_type(&self) -> HBN_XTAL_TYPE_R {
        HBN_XTAL_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hbn_xtal_sts(&self) -> HBN_XTAL_STS_R {
        HBN_XTAL_STS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hbn_flash_power_dly(&self) -> HBN_FLASH_POWER_DLY_R {
        HBN_FLASH_POWER_DLY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn hbn_flash_power_sts(&self) -> HBN_FLASH_POWER_STS_R {
        HBN_FLASH_POWER_STS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pds_gpio_keep_pin(&self) -> PDS_GPIO_KEEP_PIN_R {
        PDS_GPIO_KEEP_PIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn hbn_gpio_keep_pin(&self) -> HBN_GPIO_KEEP_PIN_R {
        HBN_GPIO_KEEP_PIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pds_gpio_keep_sts(&self) -> PDS_GPIO_KEEP_STS_R {
        PDS_GPIO_KEEP_STS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn hbn_gpio_keep_sts(&self) -> HBN_GPIO_KEEP_STS_R {
        HBN_GPIO_KEEP_STS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_xtal_type(&mut self) -> HBN_XTAL_TYPE_W<HBN_RSV3_SPEC> {
        HBN_XTAL_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_xtal_sts(&mut self) -> HBN_XTAL_STS_W<HBN_RSV3_SPEC> {
        HBN_XTAL_STS_W::new(self, 4)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_flash_power_dly(&mut self) -> HBN_FLASH_POWER_DLY_W<HBN_RSV3_SPEC> {
        HBN_FLASH_POWER_DLY_W::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_flash_power_sts(&mut self) -> HBN_FLASH_POWER_STS_W<HBN_RSV3_SPEC> {
        HBN_FLASH_POWER_STS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_keep_pin(&mut self) -> PDS_GPIO_KEEP_PIN_W<HBN_RSV3_SPEC> {
        PDS_GPIO_KEEP_PIN_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_gpio_keep_pin(&mut self) -> HBN_GPIO_KEEP_PIN_W<HBN_RSV3_SPEC> {
        HBN_GPIO_KEEP_PIN_W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_keep_sts(&mut self) -> PDS_GPIO_KEEP_STS_W<HBN_RSV3_SPEC> {
        PDS_GPIO_KEEP_STS_W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_gpio_keep_sts(&mut self) -> HBN_GPIO_KEEP_STS_W<HBN_RSV3_SPEC> {
        HBN_GPIO_KEEP_STS_W::new(self, 30)
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
#[doc = "HBN_RSV3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_RSV3_SPEC;
impl crate::RegisterSpec for HBN_RSV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv3::R`](R) reader structure"]
impl crate::Readable for HBN_RSV3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv3::W`](W) writer structure"]
impl crate::Writable for HBN_RSV3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_RSV3 to value 0"]
impl crate::Resettable for HBN_RSV3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
