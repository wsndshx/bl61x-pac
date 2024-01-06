#[doc = "Register `period_start` reader"]
pub type R = crate::R<PERIOD_START_SPEC>;
#[doc = "Register `period_start` writer"]
pub type W = crate::W<PERIOD_START_SPEC>;
#[doc = "Field `phase(0-3)` reader - Length of start condition phase %s"]
pub type PHASE_R = crate::FieldReader;
#[doc = "Field `phase(0-3)` writer - Length of start condition phase %s"]
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Length of start condition phase (0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `phase0` field"]
    #[inline(always)]
    pub fn phase(&self, n: u8) -> PHASE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PHASE_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Length of start condition phase (0-3)"]
    #[inline(always)]
    pub fn phase_iter(&self) -> impl Iterator<Item = PHASE_R> + '_ {
        (0..4).map(move |n| PHASE_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Length of start condition phase 0"]
    #[inline(always)]
    pub fn phase0(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length of start condition phase 1"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Length of start condition phase 2"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Length of start condition phase 3"]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Length of start condition phase (0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `phase0` field"]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self, n: u8) -> PHASE_W<PERIOD_START_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PHASE_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Length of start condition phase 0"]
    #[inline(always)]
    #[must_use]
    pub fn phase0(&mut self) -> PHASE_W<PERIOD_START_SPEC> {
        PHASE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Length of start condition phase 1"]
    #[inline(always)]
    #[must_use]
    pub fn phase1(&mut self) -> PHASE_W<PERIOD_START_SPEC> {
        PHASE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Length of start condition phase 2"]
    #[inline(always)]
    #[must_use]
    pub fn phase2(&mut self) -> PHASE_W<PERIOD_START_SPEC> {
        PHASE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Length of start condition phase 3"]
    #[inline(always)]
    #[must_use]
    pub fn phase3(&mut self) -> PHASE_W<PERIOD_START_SPEC> {
        PHASE_W::new(self, 24)
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
#[doc = "Duration of start phase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIOD_START_SPEC;
impl crate::RegisterSpec for PERIOD_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period_start::R`](R) reader structure"]
impl crate::Readable for PERIOD_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`period_start::W`](W) writer structure"]
impl crate::Writable for PERIOD_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets period_start to value 0x0f0f_0f0f"]
impl crate::Resettable for PERIOD_START_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
