#[doc = "Register `data_read` reader"]
pub type R = crate::R<DATA_READ_SPEC>;
#[doc = "Field `value` reader - Read data from FIFO"]
pub type VALUE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Read data from FIFO"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FIFO read data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_READ_SPEC;
impl crate::RegisterSpec for DATA_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_read::R`](R) reader structure"]
impl crate::Readable for DATA_READ_SPEC {}
#[doc = "`reset()` method sets data_read to value 0"]
impl crate::Resettable for DATA_READ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
