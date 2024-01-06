#[doc = "Register `gpadc_reg_raw_result` reader"]
pub type R = crate::R<GPADC_REG_RAW_RESULT_SPEC>;
#[doc = "Register `gpadc_reg_raw_result` writer"]
pub type W = crate::W<GPADC_REG_RAW_RESULT_SPEC>;
#[doc = "Field `gpadc_raw_data` reader - "]
pub type GPADC_RAW_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `gpadc_raw_data` writer - "]
pub type GPADC_RAW_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GPADC_RAW_DATA_R {
        GPADC_RAW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_raw_data(&mut self) -> GPADC_RAW_DATA_W<GPADC_REG_RAW_RESULT_SPEC> {
        GPADC_RAW_DATA_W::new(self, 0)
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
#[doc = "gpadc_reg_raw_result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_raw_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_raw_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_RAW_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RAW_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_raw_result::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_RAW_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_raw_result::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_RAW_RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_raw_result to value 0"]
impl crate::Resettable for GPADC_REG_RAW_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
