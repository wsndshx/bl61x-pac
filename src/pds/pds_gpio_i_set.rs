#[doc = "Register `pds_gpio_i_set` reader"]
pub type R = crate::R<PDS_GPIO_I_SET_SPEC>;
#[doc = "Register `pds_gpio_i_set` writer"]
pub type W = crate::W<PDS_GPIO_I_SET_SPEC>;
#[doc = "Field `cr_pds_gpio_ie_set` reader - "]
pub type CR_PDS_GPIO_IE_SET_R = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_ie_set` writer - "]
pub type CR_PDS_GPIO_IE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_pds_gpio_pd_set` reader - "]
pub type CR_PDS_GPIO_PD_SET_R = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_pd_set` writer - "]
pub type CR_PDS_GPIO_PD_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_pds_gpio_pu_set` reader - "]
pub type CR_PDS_GPIO_PU_SET_R = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_pu_set` writer - "]
pub type CR_PDS_GPIO_PU_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_pds_gpio_ie_set(&self) -> CR_PDS_GPIO_IE_SET_R {
        CR_PDS_GPIO_IE_SET_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn cr_pds_gpio_pd_set(&self) -> CR_PDS_GPIO_PD_SET_R {
        CR_PDS_GPIO_PD_SET_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cr_pds_gpio_pu_set(&self) -> CR_PDS_GPIO_PU_SET_R {
        CR_PDS_GPIO_PU_SET_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_ie_set(&mut self) -> CR_PDS_GPIO_IE_SET_W<PDS_GPIO_I_SET_SPEC> {
        CR_PDS_GPIO_IE_SET_W::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_pd_set(&mut self) -> CR_PDS_GPIO_PD_SET_W<PDS_GPIO_I_SET_SPEC> {
        CR_PDS_GPIO_PD_SET_W::new(self, 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_pu_set(&mut self) -> CR_PDS_GPIO_PU_SET_W<PDS_GPIO_I_SET_SPEC> {
        CR_PDS_GPIO_PU_SET_W::new(self, 6)
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
#[doc = "pds_gpio_i_set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_i_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_i_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_GPIO_I_SET_SPEC;
impl crate::RegisterSpec for PDS_GPIO_I_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_i_set::R`](R) reader structure"]
impl crate::Readable for PDS_GPIO_I_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_i_set::W`](W) writer structure"]
impl crate::Writable for PDS_GPIO_I_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_i_set to value 0"]
impl crate::Resettable for PDS_GPIO_I_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
