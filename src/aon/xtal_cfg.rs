#[doc = "Register `xtal_cfg` reader"]
pub type R = crate::R<XTAL_CFG_SPEC>;
#[doc = "Register `xtal_cfg` writer"]
pub type W = crate::W<XTAL_CFG_SPEC>;
#[doc = "Field `xtal_bk_aon` reader - "]
pub type XTAL_BK_AON_R = crate::FieldReader;
#[doc = "Field `xtal_bk_aon` writer - "]
pub type XTAL_BK_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_capcode_extra_aon` reader - "]
pub type XTAL_CAPCODE_EXTRA_AON_R = crate::BitReader;
#[doc = "Field `xtal_capcode_extra_aon` writer - "]
pub type XTAL_CAPCODE_EXTRA_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_ext_sel_aon` reader - "]
pub type XTAL_EXT_SEL_AON_R = crate::BitReader;
#[doc = "Field `xtal_ext_sel_aon` writer - "]
pub type XTAL_EXT_SEL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_buf_en_aon` reader - "]
pub type XTAL_BUF_EN_AON_R = crate::FieldReader;
#[doc = "Field `xtal_buf_en_aon` writer - "]
pub type XTAL_BUF_EN_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_acbuf_en_aon` reader - "]
pub type XTAL_ACBUF_EN_AON_R = crate::BitReader;
#[doc = "Field `xtal_acbuf_en_aon` writer - "]
pub type XTAL_ACBUF_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_acbuf_mode_aon` reader - "]
pub type XTAL_ACBUF_MODE_AON_R = crate::BitReader;
#[doc = "Field `xtal_acbuf_mode_aon` writer - "]
pub type XTAL_ACBUF_MODE_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_buf_hp_aon` reader - "]
pub type XTAL_BUF_HP_AON_R = crate::FieldReader;
#[doc = "Field `xtal_buf_hp_aon` writer - "]
pub type XTAL_BUF_HP_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_socbuf_en_aon` reader - "]
pub type XTAL_SOCBUF_EN_AON_R = crate::BitReader;
#[doc = "Field `xtal_socbuf_en_aon` writer - "]
pub type XTAL_SOCBUF_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_fast_startup_aon` reader - "]
pub type XTAL_FAST_STARTUP_AON_R = crate::BitReader;
#[doc = "Field `xtal_fast_startup_aon` writer - "]
pub type XTAL_FAST_STARTUP_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_sleep_aon` reader - "]
pub type XTAL_SLEEP_AON_R = crate::BitReader;
#[doc = "Field `xtal_sleep_aon` writer - "]
pub type XTAL_SLEEP_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_amp_ctrl_aon` reader - "]
pub type XTAL_AMP_CTRL_AON_R = crate::FieldReader;
#[doc = "Field `xtal_amp_ctrl_aon` writer - "]
pub type XTAL_AMP_CTRL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_capcode_out_aon` reader - "]
pub type XTAL_CAPCODE_OUT_AON_R = crate::FieldReader;
#[doc = "Field `xtal_capcode_out_aon` writer - "]
pub type XTAL_CAPCODE_OUT_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal_capcode_in_aon` reader - "]
pub type XTAL_CAPCODE_IN_AON_R = crate::FieldReader;
#[doc = "Field `xtal_capcode_in_aon` writer - "]
pub type XTAL_CAPCODE_IN_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal_gm_boost_aon` reader - "]
pub type XTAL_GM_BOOST_AON_R = crate::FieldReader;
#[doc = "Field `xtal_gm_boost_aon` writer - "]
pub type XTAL_GM_BOOST_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_rdy_sel_aon` reader - "]
pub type XTAL_RDY_SEL_AON_R = crate::FieldReader;
#[doc = "Field `xtal_rdy_sel_aon` writer - "]
pub type XTAL_RDY_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&self) -> XTAL_BK_AON_R {
        XTAL_BK_AON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&self) -> XTAL_CAPCODE_EXTRA_AON_R {
        XTAL_CAPCODE_EXTRA_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&self) -> XTAL_EXT_SEL_AON_R {
        XTAL_EXT_SEL_AON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&self) -> XTAL_BUF_EN_AON_R {
        XTAL_BUF_EN_AON_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn xtal_acbuf_en_aon(&self) -> XTAL_ACBUF_EN_AON_R {
        XTAL_ACBUF_EN_AON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal_acbuf_mode_aon(&self) -> XTAL_ACBUF_MODE_AON_R {
        XTAL_ACBUF_MODE_AON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&self) -> XTAL_BUF_HP_AON_R {
        XTAL_BUF_HP_AON_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn xtal_socbuf_en_aon(&self) -> XTAL_SOCBUF_EN_AON_R {
        XTAL_SOCBUF_EN_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&self) -> XTAL_FAST_STARTUP_AON_R {
        XTAL_FAST_STARTUP_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&self) -> XTAL_SLEEP_AON_R {
        XTAL_SLEEP_AON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&self) -> XTAL_AMP_CTRL_AON_R {
        XTAL_AMP_CTRL_AON_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&self) -> XTAL_CAPCODE_OUT_AON_R {
        XTAL_CAPCODE_OUT_AON_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&self) -> XTAL_CAPCODE_IN_AON_R {
        XTAL_CAPCODE_IN_AON_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&self) -> XTAL_GM_BOOST_AON_R {
        XTAL_GM_BOOST_AON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&self) -> XTAL_RDY_SEL_AON_R {
        XTAL_RDY_SEL_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_bk_aon(&mut self) -> XTAL_BK_AON_W<XTAL_CFG_SPEC> {
        XTAL_BK_AON_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_extra_aon(&mut self) -> XTAL_CAPCODE_EXTRA_AON_W<XTAL_CFG_SPEC> {
        XTAL_CAPCODE_EXTRA_AON_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_ext_sel_aon(&mut self) -> XTAL_EXT_SEL_AON_W<XTAL_CFG_SPEC> {
        XTAL_EXT_SEL_AON_W::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_buf_en_aon(&mut self) -> XTAL_BUF_EN_AON_W<XTAL_CFG_SPEC> {
        XTAL_BUF_EN_AON_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_acbuf_en_aon(&mut self) -> XTAL_ACBUF_EN_AON_W<XTAL_CFG_SPEC> {
        XTAL_ACBUF_EN_AON_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_acbuf_mode_aon(&mut self) -> XTAL_ACBUF_MODE_AON_W<XTAL_CFG_SPEC> {
        XTAL_ACBUF_MODE_AON_W::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_buf_hp_aon(&mut self) -> XTAL_BUF_HP_AON_W<XTAL_CFG_SPEC> {
        XTAL_BUF_HP_AON_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_socbuf_en_aon(&mut self) -> XTAL_SOCBUF_EN_AON_W<XTAL_CFG_SPEC> {
        XTAL_SOCBUF_EN_AON_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_fast_startup_aon(&mut self) -> XTAL_FAST_STARTUP_AON_W<XTAL_CFG_SPEC> {
        XTAL_FAST_STARTUP_AON_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_sleep_aon(&mut self) -> XTAL_SLEEP_AON_W<XTAL_CFG_SPEC> {
        XTAL_SLEEP_AON_W::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_amp_ctrl_aon(&mut self) -> XTAL_AMP_CTRL_AON_W<XTAL_CFG_SPEC> {
        XTAL_AMP_CTRL_AON_W::new(self, 14)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_out_aon(&mut self) -> XTAL_CAPCODE_OUT_AON_W<XTAL_CFG_SPEC> {
        XTAL_CAPCODE_OUT_AON_W::new(self, 16)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_in_aon(&mut self) -> XTAL_CAPCODE_IN_AON_W<XTAL_CFG_SPEC> {
        XTAL_CAPCODE_IN_AON_W::new(self, 22)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_gm_boost_aon(&mut self) -> XTAL_GM_BOOST_AON_W<XTAL_CFG_SPEC> {
        XTAL_GM_BOOST_AON_W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_rdy_sel_aon(&mut self) -> XTAL_RDY_SEL_AON_W<XTAL_CFG_SPEC> {
        XTAL_RDY_SEL_AON_W::new(self, 30)
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
#[doc = "xtal_cfg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL_CFG_SPEC;
impl crate::RegisterSpec for XTAL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_cfg::R`](R) reader structure"]
impl crate::Readable for XTAL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal_cfg::W`](W) writer structure"]
impl crate::Writable for XTAL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal_cfg to value 0"]
impl crate::Resettable for XTAL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
