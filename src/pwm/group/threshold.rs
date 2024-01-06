#[doc = "Register `threshold[%s]` reader"]
pub type R = crate::R<THRESHOLD_SPEC>;
#[doc = "Register `threshold[%s]` writer"]
pub type W = crate::W<THRESHOLD_SPEC>;
#[doc = "Field `low` reader - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type LOW_R = crate::FieldReader<u16>;
#[doc = "Field `low` writer - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `high` reader - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `high` writer - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<THRESHOLD_SPEC> {
        LOW_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<THRESHOLD_SPEC> {
        HIGH_W::new(self, 16)
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
#[doc = "Channel internal counter threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRESHOLD_SPEC;
impl crate::RegisterSpec for THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold::R`](R) reader structure"]
impl crate::Readable for THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`threshold::W`](W) writer structure"]
impl crate::Writable for THRESHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets threshold[%s]
to value 0"]
impl crate::Resettable for THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
