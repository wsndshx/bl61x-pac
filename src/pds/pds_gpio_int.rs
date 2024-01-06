#[doc = "Register `pds_gpio_int` reader"]
pub type R = crate::R<PDS_GPIO_INT_SPEC>;
#[doc = "Register `pds_gpio_int` writer"]
pub type W = crate::W<PDS_GPIO_INT_SPEC>;
#[doc = "Field `pds_gpio_set1_int_clr` reader - "]
pub type PDS_GPIO_SET1_INT_CLR_R = crate::BitReader;
#[doc = "Field `pds_gpio_set1_int_clr` writer - "]
pub type PDS_GPIO_SET1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set1_int_mode` reader - "]
pub type PDS_GPIO_SET1_INT_MODE_R = crate::FieldReader;
#[doc = "Field `pds_gpio_set1_int_mode` writer - "]
pub type PDS_GPIO_SET1_INT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set2_int_clr` reader - "]
pub type PDS_GPIO_SET2_INT_CLR_R = crate::BitReader;
#[doc = "Field `pds_gpio_set2_int_clr` writer - "]
pub type PDS_GPIO_SET2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set2_int_mode` reader - "]
pub type PDS_GPIO_SET2_INT_MODE_R = crate::FieldReader;
#[doc = "Field `pds_gpio_set2_int_mode` writer - "]
pub type PDS_GPIO_SET2_INT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set3_int_clr` reader - "]
pub type PDS_GPIO_SET3_INT_CLR_R = crate::BitReader;
#[doc = "Field `pds_gpio_set3_int_clr` writer - "]
pub type PDS_GPIO_SET3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set3_int_mode` reader - "]
pub type PDS_GPIO_SET3_INT_MODE_R = crate::FieldReader;
#[doc = "Field `pds_gpio_set3_int_mode` writer - "]
pub type PDS_GPIO_SET3_INT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set4_int_clr` reader - "]
pub type PDS_GPIO_SET4_INT_CLR_R = crate::BitReader;
#[doc = "Field `pds_gpio_set4_int_clr` writer - "]
pub type PDS_GPIO_SET4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set4_int_mode` reader - "]
pub type PDS_GPIO_SET4_INT_MODE_R = crate::FieldReader;
#[doc = "Field `pds_gpio_set4_int_mode` writer - "]
pub type PDS_GPIO_SET4_INT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_clr(&self) -> PDS_GPIO_SET1_INT_CLR_R {
        PDS_GPIO_SET1_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_mode(&self) -> PDS_GPIO_SET1_INT_MODE_R {
        PDS_GPIO_SET1_INT_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_clr(&self) -> PDS_GPIO_SET2_INT_CLR_R {
        PDS_GPIO_SET2_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_mode(&self) -> PDS_GPIO_SET2_INT_MODE_R {
        PDS_GPIO_SET2_INT_MODE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_clr(&self) -> PDS_GPIO_SET3_INT_CLR_R {
        PDS_GPIO_SET3_INT_CLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_mode(&self) -> PDS_GPIO_SET3_INT_MODE_R {
        PDS_GPIO_SET3_INT_MODE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_clr(&self) -> PDS_GPIO_SET4_INT_CLR_R {
        PDS_GPIO_SET4_INT_CLR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_mode(&self) -> PDS_GPIO_SET4_INT_MODE_R {
        PDS_GPIO_SET4_INT_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set1_int_clr(&mut self) -> PDS_GPIO_SET1_INT_CLR_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET1_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set1_int_mode(&mut self) -> PDS_GPIO_SET1_INT_MODE_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET1_INT_MODE_W::new(self, 4)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set2_int_clr(&mut self) -> PDS_GPIO_SET2_INT_CLR_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET2_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set2_int_mode(&mut self) -> PDS_GPIO_SET2_INT_MODE_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET2_INT_MODE_W::new(self, 12)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set3_int_clr(&mut self) -> PDS_GPIO_SET3_INT_CLR_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET3_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set3_int_mode(&mut self) -> PDS_GPIO_SET3_INT_MODE_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET3_INT_MODE_W::new(self, 20)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set4_int_clr(&mut self) -> PDS_GPIO_SET4_INT_CLR_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET4_INT_CLR_W::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_set4_int_mode(&mut self) -> PDS_GPIO_SET4_INT_MODE_W<PDS_GPIO_INT_SPEC> {
        PDS_GPIO_SET4_INT_MODE_W::new(self, 28)
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
#[doc = "pds_gpio_int.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_GPIO_INT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_int::R`](R) reader structure"]
impl crate::Readable for PDS_GPIO_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_int::W`](W) writer structure"]
impl crate::Writable for PDS_GPIO_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_int to value 0"]
impl crate::Resettable for PDS_GPIO_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
