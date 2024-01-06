#[doc = "Register `output` reader"]
pub type R = crate::R<OUTPUT_SPEC>;
#[doc = "Field `checksum` reader - Read checksum from peripheral"]
pub type CHECKSUM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Read checksum from peripheral"]
    #[inline(always)]
    pub fn checksum(&self) -> CHECKSUM_R {
        CHECKSUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Checksum output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`output::R`](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {}
#[doc = "`reset()` method sets output to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
