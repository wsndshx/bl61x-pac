#[doc = "Register `gpadc_reg_result` reader"]
pub type R = crate::R<GPADC_REG_RESULT_SPEC>;
#[doc = "Register `gpadc_reg_result` writer"]
pub type W = crate::W<GPADC_REG_RESULT_SPEC>;
#[doc = "Field `gpadc_data_out` reader - "]
pub type GPADC_DATA_OUT_R = crate::FieldReader<u32>;
#[doc = "Field `gpadc_data_out` writer - "]
pub type GPADC_DATA_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GPADC_DATA_OUT_R {
        GPADC_DATA_OUT_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_data_out(&mut self) -> GPADC_DATA_OUT_W<GPADC_REG_RESULT_SPEC> {
        GPADC_DATA_OUT_W::new(self, 0)
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
#[doc = "gpadc_reg_result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_result::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_result::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_result to value 0"]
impl crate::Resettable for GPADC_REG_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
