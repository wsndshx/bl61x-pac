#[doc = "Register `pds_gpio_pd_set` reader"]
pub type R = crate::R<PDS_GPIO_PD_SET_SPEC>;
#[doc = "Register `pds_gpio_pd_set` writer"]
pub type W = crate::W<PDS_GPIO_PD_SET_SPEC>;
#[doc = "Field `cr_pds_gpio_set_int_mask` reader - "]
pub type CR_PDS_GPIO_SET_INT_MASK_R = crate::FieldReader<u32>;
#[doc = "Field `cr_pds_gpio_set_int_mask` writer - "]
pub type CR_PDS_GPIO_SET_INT_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn cr_pds_gpio_set_int_mask(&self) -> CR_PDS_GPIO_SET_INT_MASK_R {
        CR_PDS_GPIO_SET_INT_MASK_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_set_int_mask(&mut self) -> CR_PDS_GPIO_SET_INT_MASK_W<PDS_GPIO_PD_SET_SPEC> {
        CR_PDS_GPIO_SET_INT_MASK_W::new(self, 0)
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
#[doc = "pds_gpio_pd_set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_pd_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_pd_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_GPIO_PD_SET_SPEC;
impl crate::RegisterSpec for PDS_GPIO_PD_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_pd_set::R`](R) reader structure"]
impl crate::Readable for PDS_GPIO_PD_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_pd_set::W`](W) writer structure"]
impl crate::Writable for PDS_GPIO_PD_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_pd_set to value 0"]
impl crate::Resettable for PDS_GPIO_PD_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
