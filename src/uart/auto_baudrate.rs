#[doc = "Register `auto_baudrate` reader"]
pub type R = crate::R<AUTO_BAUDRATE_SPEC>;
#[doc = "Field `by_start_bit` reader - Bit period of auto baudrate detection using start bit"]
pub type BY_START_BIT_R = crate::FieldReader<u16>;
#[doc = "Field `by_five_five` reader - Bit period of auto baudrate detection using codeword 0x55"]
pub type BY_FIVE_FIVE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Bit period of auto baudrate detection using start bit"]
    #[inline(always)]
    pub fn by_start_bit(&self) -> BY_START_BIT_R {
        BY_START_BIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Bit period of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    pub fn by_five_five(&self) -> BY_FIVE_FIVE_R {
        BY_FIVE_FIVE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Auto baudrate detection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_baudrate::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTO_BAUDRATE_SPEC;
impl crate::RegisterSpec for AUTO_BAUDRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_baudrate::R`](R) reader structure"]
impl crate::Readable for AUTO_BAUDRATE_SPEC {}
#[doc = "`reset()` method sets auto_baudrate to value 0"]
impl crate::Resettable for AUTO_BAUDRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
