#[doc = "Register `self_test_1` reader"]
pub type R = crate::R<SELF_TEST_1_SPEC>;
#[doc = "Register `self_test_1` writer"]
pub type W = crate::W<SELF_TEST_1_SPEC>;
#[doc = "Field `top_mbist_mode` reader - "]
pub type TOP_MBIST_MODE_R = crate::BitReader;
#[doc = "Field `top_mbist_mode` writer - "]
pub type TOP_MBIST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_top_mbist_rst_n` reader - "]
pub type REG_TOP_MBIST_RST_N_R = crate::BitReader;
#[doc = "Field `reg_top_mbist_rst_n` writer - "]
pub type REG_TOP_MBIST_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_mbist_done` reader - "]
pub type EF_MBIST_DONE_R = crate::FieldReader;
#[doc = "Field `ef_mbist_done` writer - "]
pub type EF_MBIST_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usb_mbist_done` reader - "]
pub type USB_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `usb_mbist_done` writer - "]
pub type USB_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdh_mbist_done` reader - "]
pub type SDH_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `sdh_mbist_done` writer - "]
pub type SDH_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sec_mbist_done` reader - "]
pub type SEC_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `sec_mbist_done` writer - "]
pub type SEC_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_mbist_done` reader - "]
pub type SF_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `sf_mbist_done` writer - "]
pub type SF_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `emac_mbist_done` reader - "]
pub type EMAC_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `emac_mbist_done` writer - "]
pub type EMAC_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_mbist_fail` reader - "]
pub type EF_MBIST_FAIL_R = crate::FieldReader;
#[doc = "Field `ef_mbist_fail` writer - "]
pub type EF_MBIST_FAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usb_mbist_fail` reader - "]
pub type USB_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `usb_mbist_fail` writer - "]
pub type USB_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdh_mbist_fail` reader - "]
pub type SDH_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `sdh_mbist_fail` writer - "]
pub type SDH_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sec_mbist_fail` reader - "]
pub type SEC_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `sec_mbist_fail` writer - "]
pub type SEC_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_mbist_fail` reader - "]
pub type SF_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `sf_mbist_fail` writer - "]
pub type SF_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `emac_mbist_fail` reader - "]
pub type EMAC_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `emac_mbist_fail` writer - "]
pub type EMAC_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn top_mbist_mode(&self) -> TOP_MBIST_MODE_R {
        TOP_MBIST_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_top_mbist_rst_n(&self) -> REG_TOP_MBIST_RST_N_R {
        REG_TOP_MBIST_RST_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ef_mbist_done(&self) -> EF_MBIST_DONE_R {
        EF_MBIST_DONE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usb_mbist_done(&self) -> USB_MBIST_DONE_R {
        USB_MBIST_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdh_mbist_done(&self) -> SDH_MBIST_DONE_R {
        SDH_MBIST_DONE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sec_mbist_done(&self) -> SEC_MBIST_DONE_R {
        SEC_MBIST_DONE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_mbist_done(&self) -> SF_MBIST_DONE_R {
        SF_MBIST_DONE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn emac_mbist_done(&self) -> EMAC_MBIST_DONE_R {
        EMAC_MBIST_DONE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_mbist_fail(&self) -> EF_MBIST_FAIL_R {
        EF_MBIST_FAIL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usb_mbist_fail(&self) -> USB_MBIST_FAIL_R {
        USB_MBIST_FAIL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sdh_mbist_fail(&self) -> SDH_MBIST_FAIL_R {
        SDH_MBIST_FAIL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sec_mbist_fail(&self) -> SEC_MBIST_FAIL_R {
        SEC_MBIST_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_mbist_fail(&self) -> SF_MBIST_FAIL_R {
        SF_MBIST_FAIL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn emac_mbist_fail(&self) -> EMAC_MBIST_FAIL_R {
        EMAC_MBIST_FAIL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn top_mbist_mode(&mut self) -> TOP_MBIST_MODE_W<SELF_TEST_1_SPEC> {
        TOP_MBIST_MODE_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_top_mbist_rst_n(&mut self) -> REG_TOP_MBIST_RST_N_W<SELF_TEST_1_SPEC> {
        REG_TOP_MBIST_RST_N_W::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn ef_mbist_done(&mut self) -> EF_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        EF_MBIST_DONE_W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn usb_mbist_done(&mut self) -> USB_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        USB_MBIST_DONE_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sdh_mbist_done(&mut self) -> SDH_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        SDH_MBIST_DONE_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn sec_mbist_done(&mut self) -> SEC_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        SEC_MBIST_DONE_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn sf_mbist_done(&mut self) -> SF_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        SF_MBIST_DONE_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn emac_mbist_done(&mut self) -> EMAC_MBIST_DONE_W<SELF_TEST_1_SPEC> {
        EMAC_MBIST_DONE_W::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ef_mbist_fail(&mut self) -> EF_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        EF_MBIST_FAIL_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn usb_mbist_fail(&mut self) -> USB_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        USB_MBIST_FAIL_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn sdh_mbist_fail(&mut self) -> SDH_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        SDH_MBIST_FAIL_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sec_mbist_fail(&mut self) -> SEC_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        SEC_MBIST_FAIL_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn sf_mbist_fail(&mut self) -> SF_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        SF_MBIST_FAIL_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn emac_mbist_fail(&mut self) -> EMAC_MBIST_FAIL_W<SELF_TEST_1_SPEC> {
        EMAC_MBIST_FAIL_W::new(self, 30)
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
#[doc = "Machine Built-in Self Test register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`self_test_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`self_test_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELF_TEST_1_SPEC;
impl crate::RegisterSpec for SELF_TEST_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`self_test_1::R`](R) reader structure"]
impl crate::Readable for SELF_TEST_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`self_test_1::W`](W) writer structure"]
impl crate::Writable for SELF_TEST_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets self_test_1 to value 0"]
impl crate::Resettable for SELF_TEST_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
