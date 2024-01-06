#[doc = "Register `data_config` reader"]
pub type R = crate::R<DATA_CONFIG_SPEC>;
#[doc = "Register `data_config` writer"]
pub type W = crate::W<DATA_CONFIG_SPEC>;
#[doc = "Field `bit_order` reader - Enable bit inverse in each data word"]
pub type BIT_ORDER_R = crate::BitReader<BIT_ORDER_A>;
#[doc = "Enable bit inverse in each data word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_ORDER_A {
    #[doc = "1: Each byte is sent out MSB-first"]
    INVERSE = 1,
    #[doc = "0: Each byte is sent out LSB-first"]
    NO_INVERSE = 0,
}
impl From<BIT_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIT_ORDER_A {
        match self.bits {
            true => BIT_ORDER_A::INVERSE,
            false => BIT_ORDER_A::NO_INVERSE,
        }
    }
    #[doc = "Each byte is sent out MSB-first"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == BIT_ORDER_A::INVERSE
    }
    #[doc = "Each byte is sent out LSB-first"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        *self == BIT_ORDER_A::NO_INVERSE
    }
}
#[doc = "Field `bit_order` writer - Enable bit inverse in each data word"]
pub type BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG, BIT_ORDER_A>;
impl<'a, REG> BIT_ORDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each byte is sent out MSB-first"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_ORDER_A::INVERSE)
    }
    #[doc = "Each byte is sent out LSB-first"]
    #[inline(always)]
    pub fn no_inverse(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_ORDER_A::NO_INVERSE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    pub fn bit_order(&self) -> BIT_ORDER_R {
        BIT_ORDER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    #[must_use]
    pub fn bit_order(&mut self) -> BIT_ORDER_W<DATA_CONFIG_SPEC> {
        BIT_ORDER_W::new(self, 0)
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
#[doc = "Data configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_CONFIG_SPEC;
impl crate::RegisterSpec for DATA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_config::R`](R) reader structure"]
impl crate::Readable for DATA_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_config::W`](W) writer structure"]
impl crate::Writable for DATA_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets data_config to value 0"]
impl crate::Resettable for DATA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
