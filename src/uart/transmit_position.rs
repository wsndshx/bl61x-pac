#[doc = "Register `transmit_position` reader"]
pub type R = crate::R<TRANSMIT_POSITION_SPEC>;
#[doc = "Register `transmit_position` writer"]
pub type W = crate::W<TRANSMIT_POSITION_SPEC>;
#[doc = "Field `start` reader - Start position of transmit IR pulse"]
pub type START_R = crate::FieldReader<u16>;
#[doc = "Field `start` writer - Start position of transmit IR pulse"]
pub type START_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `stop` reader - Stop position of transmit IR pulse"]
pub type STOP_R = crate::FieldReader<u16>;
#[doc = "Field `stop` writer - Stop position of transmit IR pulse"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<TRANSMIT_POSITION_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<TRANSMIT_POSITION_SPEC> {
        STOP_W::new(self, 16)
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
#[doc = "IR-mode transmit position control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSMIT_POSITION_SPEC;
impl crate::RegisterSpec for TRANSMIT_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_position::R`](R) reader structure"]
impl crate::Readable for TRANSMIT_POSITION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`transmit_position::W`](W) writer structure"]
impl crate::Writable for TRANSMIT_POSITION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets transmit_position to value 0x009f_0070"]
impl crate::Resettable for TRANSMIT_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x009f_0070;
}
