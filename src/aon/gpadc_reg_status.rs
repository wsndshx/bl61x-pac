#[doc = "Register `gpadc_reg_status` reader"]
pub type R = crate::R<GPADC_REG_STATUS_SPEC>;
#[doc = "Register `gpadc_reg_status` writer"]
pub type W = crate::W<GPADC_REG_STATUS_SPEC>;
#[doc = "Field `gpadc_data_rdy` reader - "]
pub type GPADC_DATA_RDY_R = crate::BitReader;
#[doc = "Field `gpadc_data_rdy` writer - "]
pub type GPADC_DATA_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&self) -> GPADC_DATA_RDY_R {
        GPADC_DATA_RDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_data_rdy(&mut self) -> GPADC_DATA_RDY_W<GPADC_REG_STATUS_SPEC> {
        GPADC_DATA_RDY_W::new(self, 0)
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
#[doc = "gpadc_reg_status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_STATUS_SPEC;
impl crate::RegisterSpec for GPADC_REG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_status::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_status::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_status to value 0"]
impl crate::Resettable for GPADC_REG_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
