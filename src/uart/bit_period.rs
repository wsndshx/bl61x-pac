#[doc = "Register `bit_period` reader"]
pub type R = crate::R<BIT_PERIOD_SPEC>;
#[doc = "Register `bit_period` writer"]
pub type W = crate::W<BIT_PERIOD_SPEC>;
#[doc = "Field `transmit` reader - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
pub type TRANSMIT_R = crate::FieldReader<u16>;
#[doc = "Field `transmit` writer - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
pub type TRANSMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `receive` reader - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
pub type RECEIVE_R = crate::FieldReader<u16>;
#[doc = "Field `receive` writer - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
pub type RECEIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
    #[inline(always)]
    pub fn transmit(&self) -> TRANSMIT_R {
        TRANSMIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
    #[inline(always)]
    pub fn receive(&self) -> RECEIVE_R {
        RECEIVE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn transmit(&mut self) -> TRANSMIT_W<BIT_PERIOD_SPEC> {
        TRANSMIT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn receive(&mut self) -> RECEIVE_W<BIT_PERIOD_SPEC> {
        RECEIVE_W::new(self, 16)
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
#[doc = "Bit period control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bit_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bit_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIT_PERIOD_SPEC;
impl crate::RegisterSpec for BIT_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bit_period::R`](R) reader structure"]
impl crate::Readable for BIT_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bit_period::W`](W) writer structure"]
impl crate::Writable for BIT_PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bit_period to value 0x00ff_00ff"]
impl crate::Resettable for BIT_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_00ff;
}
