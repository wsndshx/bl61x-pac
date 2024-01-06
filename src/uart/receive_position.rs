#[doc = "Register `receive_position` reader"]
pub type R = crate::R<RECEIVE_POSITION_SPEC>;
#[doc = "Register `receive_position` writer"]
pub type W = crate::W<RECEIVE_POSITION_SPEC>;
#[doc = "Field `start` reader - Start position of received pulse recovered from IR signal"]
pub type START_R = crate::FieldReader<u16>;
#[doc = "Field `start` writer - Start position of received pulse recovered from IR signal"]
pub type START_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start position of received pulse recovered from IR signal"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start position of received pulse recovered from IR signal"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<RECEIVE_POSITION_SPEC> {
        START_W::new(self, 0)
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
#[doc = "IR-mode receive position control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_POSITION_SPEC;
impl crate::RegisterSpec for RECEIVE_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_position::R`](R) reader structure"]
impl crate::Readable for RECEIVE_POSITION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_position::W`](W) writer structure"]
impl crate::Writable for RECEIVE_POSITION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_position to value 0x6f"]
impl crate::Resettable for RECEIVE_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x6f;
}
