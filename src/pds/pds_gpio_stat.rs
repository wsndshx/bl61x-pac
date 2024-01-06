#[doc = "Register `pds_gpio_stat` reader"]
pub type R = crate::R<PDS_GPIO_STAT_SPEC>;
#[doc = "Register `pds_gpio_stat` writer"]
pub type W = crate::W<PDS_GPIO_STAT_SPEC>;
#[doc = "Field `pds_gpio_int_stat` reader - "]
pub type PDS_GPIO_INT_STAT_R = crate::FieldReader<u32>;
#[doc = "Field `pds_gpio_int_stat` writer - "]
pub type PDS_GPIO_INT_STAT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&self) -> PDS_GPIO_INT_STAT_R {
        PDS_GPIO_INT_STAT_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_stat(&mut self) -> PDS_GPIO_INT_STAT_W<PDS_GPIO_STAT_SPEC> {
        PDS_GPIO_INT_STAT_W::new(self, 0)
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
#[doc = "pds_gpio_stat.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_GPIO_STAT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_stat::R`](R) reader structure"]
impl crate::Readable for PDS_GPIO_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_stat::W`](W) writer structure"]
impl crate::Writable for PDS_GPIO_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_stat to value 0"]
impl crate::Resettable for PDS_GPIO_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
