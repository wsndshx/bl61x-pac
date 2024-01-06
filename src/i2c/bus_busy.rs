#[doc = "Register `bus_busy` reader"]
pub type R = crate::R<BUS_BUSY_SPEC>;
#[doc = "Register `bus_busy` writer"]
pub type W = crate::W<BUS_BUSY_SPEC>;
#[doc = "Field `busy` reader - Indicator to I2C bus busy signal"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Indicator to I2C bus busy signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "1: Bus is busy"]
    BUSY = 1,
    #[doc = "0: Bus is not busy"]
    IDLE = 0,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            true => BUSY_A::BUSY,
            false => BUSY_A::IDLE,
        }
    }
    #[doc = "Bus is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
    #[doc = "Bus is not busy"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
}
#[doc = "Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_CLEAR_AW {
    #[doc = "1: Write 1 to force clear busy flag"]
    CLEAR = 1,
}
impl From<FORCE_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: FORCE_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `force_clear` writer - Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs"]
pub type FORCE_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_CLEAR_AW>;
impl<'a, REG> FORCE_CLEAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to force clear busy flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_CLEAR_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Indicator to I2C bus busy signal"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs"]
    #[inline(always)]
    #[must_use]
    pub fn force_clear(&mut self) -> FORCE_CLEAR_W<BUS_BUSY_SPEC> {
        FORCE_CLEAR_W::new(self, 1)
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
#[doc = "Bus busy state indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_busy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_busy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_BUSY_SPEC;
impl crate::RegisterSpec for BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_busy::R`](R) reader structure"]
impl crate::Readable for BUS_BUSY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_busy::W`](W) writer structure"]
impl crate::Writable for BUS_BUSY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bus_busy to value 0"]
impl crate::Resettable for BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
