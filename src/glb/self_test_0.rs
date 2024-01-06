#[doc = "Register `self_test_0` reader"]
pub type R = crate::R<SELF_TEST_0_SPEC>;
#[doc = "Register `self_test_0` writer"]
pub type W = crate::W<SELF_TEST_0_SPEC>;
#[doc = "Field `ocram_mbist_mode` reader - "]
pub type OCRAM_MBIST_MODE_R = crate::FieldReader;
#[doc = "Field `ocram_mbist_mode` writer - "]
pub type OCRAM_MBIST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wram_mbist_mode` reader - "]
pub type WRAM_MBIST_MODE_R = crate::FieldReader;
#[doc = "Field `wram_mbist_mode` writer - "]
pub type WRAM_MBIST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_wram_ocram_mbist_rst_n` reader - "]
pub type REG_WRAM_OCRAM_MBIST_RST_N_R = crate::BitReader;
#[doc = "Field `reg_wram_ocram_mbist_rst_n` writer - "]
pub type REG_WRAM_OCRAM_MBIST_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ocram_mbist_done` reader - "]
pub type OCRAM_MBIST_DONE_R = crate::FieldReader;
#[doc = "Field `ocram_mbist_done` writer - "]
pub type OCRAM_MBIST_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wram_mbist_done` reader - "]
pub type WRAM_MBIST_DONE_R = crate::FieldReader;
#[doc = "Field `wram_mbist_done` writer - "]
pub type WRAM_MBIST_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ocram_mbist_fail` reader - "]
pub type OCRAM_MBIST_FAIL_R = crate::FieldReader;
#[doc = "Field `ocram_mbist_fail` writer - "]
pub type OCRAM_MBIST_FAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wram_mbist_fail` reader - "]
pub type WRAM_MBIST_FAIL_R = crate::FieldReader;
#[doc = "Field `wram_mbist_fail` writer - "]
pub type WRAM_MBIST_FAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&self) -> OCRAM_MBIST_MODE_R {
        OCRAM_MBIST_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wram_mbist_mode(&self) -> WRAM_MBIST_MODE_R {
        WRAM_MBIST_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_wram_ocram_mbist_rst_n(&self) -> REG_WRAM_OCRAM_MBIST_RST_N_R {
        REG_WRAM_OCRAM_MBIST_RST_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn wram_mbist_done(&self) -> WRAM_MBIST_DONE_R {
        WRAM_MBIST_DONE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn wram_mbist_fail(&self) -> WRAM_MBIST_FAIL_R {
        WRAM_MBIST_FAIL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_mbist_mode(&mut self) -> OCRAM_MBIST_MODE_W<SELF_TEST_0_SPEC> {
        OCRAM_MBIST_MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wram_mbist_mode(&mut self) -> WRAM_MBIST_MODE_W<SELF_TEST_0_SPEC> {
        WRAM_MBIST_MODE_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wram_ocram_mbist_rst_n(&mut self) -> REG_WRAM_OCRAM_MBIST_RST_N_W<SELF_TEST_0_SPEC> {
        REG_WRAM_OCRAM_MBIST_RST_N_W::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_mbist_done(&mut self) -> OCRAM_MBIST_DONE_W<SELF_TEST_0_SPEC> {
        OCRAM_MBIST_DONE_W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn wram_mbist_done(&mut self) -> WRAM_MBIST_DONE_W<SELF_TEST_0_SPEC> {
        WRAM_MBIST_DONE_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_mbist_fail(&mut self) -> OCRAM_MBIST_FAIL_W<SELF_TEST_0_SPEC> {
        OCRAM_MBIST_FAIL_W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn wram_mbist_fail(&mut self) -> WRAM_MBIST_FAIL_W<SELF_TEST_0_SPEC> {
        WRAM_MBIST_FAIL_W::new(self, 28)
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
#[doc = "Machine Built-in Self Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`self_test_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`self_test_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELF_TEST_0_SPEC;
impl crate::RegisterSpec for SELF_TEST_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`self_test_0::R`](R) reader structure"]
impl crate::Readable for SELF_TEST_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`self_test_0::W`](W) writer structure"]
impl crate::Writable for SELF_TEST_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets self_test_0 to value 0"]
impl crate::Resettable for SELF_TEST_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
