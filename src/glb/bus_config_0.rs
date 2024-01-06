#[doc = "Register `bus_config_0` reader"]
pub type R = crate::R<BUS_CONFIG_0_SPEC>;
#[doc = "Register `bus_config_0` writer"]
pub type W = crate::W<BUS_CONFIG_0_SPEC>;
#[doc = "Field `rg_apb2_pck_force` reader - "]
pub type RG_APB2_PCK_FORCE_R = crate::FieldReader<u16>;
#[doc = "Field `rg_apb2_pck_force` writer - "]
pub type RG_APB2_PCK_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rg_apb_pck_force` reader - "]
pub type RG_APB_PCK_FORCE_R = crate::FieldReader<u16>;
#[doc = "Field `rg_apb_pck_force` writer - "]
pub type RG_APB_PCK_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_apb2_pck_force(&self) -> RG_APB2_PCK_FORCE_R {
        RG_APB2_PCK_FORCE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rg_apb_pck_force(&self) -> RG_APB_PCK_FORCE_R {
        RG_APB_PCK_FORCE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rg_apb2_pck_force(&mut self) -> RG_APB2_PCK_FORCE_W<BUS_CONFIG_0_SPEC> {
        RG_APB2_PCK_FORCE_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rg_apb_pck_force(&mut self) -> RG_APB_PCK_FORCE_W<BUS_CONFIG_0_SPEC> {
        RG_APB_PCK_FORCE_W::new(self, 16)
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
#[doc = "Bus configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_CONFIG_0_SPEC;
impl crate::RegisterSpec for BUS_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_config_0::R`](R) reader structure"]
impl crate::Readable for BUS_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_config_0::W`](W) writer structure"]
impl crate::Writable for BUS_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bus_config_0 to value 0"]
impl crate::Resettable for BUS_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
