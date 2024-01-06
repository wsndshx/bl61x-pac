#[doc = "Register `base_clock` reader"]
pub type R = crate::R<BASE_CLOCK_SPEC>;
#[doc = "Register `base_clock` writer"]
pub type W = crate::W<BASE_CLOCK_SPEC>;
#[doc = "Field `divide_low` reader - Lower half of base clock dividing factor"]
pub type DIVIDE_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `divide_low` writer - Lower half of base clock dividing factor"]
pub type DIVIDE_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `divide_high` reader - Higher half of base clock dividing factor"]
pub type DIVIDE_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `divide_high` writer - Higher half of base clock dividing factor"]
pub type DIVIDE_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Lower half of base clock dividing factor"]
    #[inline(always)]
    pub fn divide_low(&self) -> DIVIDE_LOW_R {
        DIVIDE_LOW_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Higher half of base clock dividing factor"]
    #[inline(always)]
    pub fn divide_high(&self) -> DIVIDE_HIGH_R {
        DIVIDE_HIGH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Lower half of base clock dividing factor"]
    #[inline(always)]
    #[must_use]
    pub fn divide_low(&mut self) -> DIVIDE_LOW_W<BASE_CLOCK_SPEC> {
        DIVIDE_LOW_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Higher half of base clock dividing factor"]
    #[inline(always)]
    #[must_use]
    pub fn divide_high(&mut self) -> DIVIDE_HIGH_W<BASE_CLOCK_SPEC> {
        DIVIDE_HIGH_W::new(self, 16)
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
#[doc = "Base clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_clock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_clock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASE_CLOCK_SPEC;
impl crate::RegisterSpec for BASE_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_clock::R`](R) reader structure"]
impl crate::Readable for BASE_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`base_clock::W`](W) writer structure"]
impl crate::Writable for BASE_CLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets base_clock to value 0x0001_0001"]
impl crate::Resettable for BASE_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
