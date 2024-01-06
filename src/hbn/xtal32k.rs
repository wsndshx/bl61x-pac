#[doc = "Register `xtal32k` reader"]
pub type R = crate::R<XTAL32K_SPEC>;
#[doc = "Register `xtal32k` writer"]
pub type W = crate::W<XTAL32K_SPEC>;
#[doc = "Field `xtal32k_ext_sel` reader - "]
pub type XTAL32K_EXT_SEL_R = crate::BitReader;
#[doc = "Field `xtal32k_ext_sel` writer - "]
pub type XTAL32K_EXT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_amp_ctrl` reader - "]
pub type XTAL32K_AMP_CTRL_R = crate::FieldReader;
#[doc = "Field `xtal32k_amp_ctrl` writer - "]
pub type XTAL32K_AMP_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_reg` reader - "]
pub type XTAL32K_REG_R = crate::FieldReader;
#[doc = "Field `xtal32k_reg` writer - "]
pub type XTAL32K_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_outbuf_stre` reader - "]
pub type XTAL32K_OUTBUF_STRE_R = crate::BitReader;
#[doc = "Field `xtal32k_outbuf_stre` writer - "]
pub type XTAL32K_OUTBUF_STRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_otf_short` reader - "]
pub type XTAL32K_OTF_SHORT_R = crate::BitReader;
#[doc = "Field `xtal32k_otf_short` writer - "]
pub type XTAL32K_OTF_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_inv_stre` reader - "]
pub type XTAL32K_INV_STRE_R = crate::FieldReader;
#[doc = "Field `xtal32k_inv_stre` writer - "]
pub type XTAL32K_INV_STRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_capbank` reader - "]
pub type XTAL32K_CAPBANK_R = crate::FieldReader;
#[doc = "Field `xtal32k_capbank` writer - "]
pub type XTAL32K_CAPBANK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal32k_ac_cap_short` reader - "]
pub type XTAL32K_AC_CAP_SHORT_R = crate::BitReader;
#[doc = "Field `xtal32k_ac_cap_short` writer - "]
pub type XTAL32K_AC_CAP_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal32k_buf` reader - "]
pub type PU_XTAL32K_BUF_R = crate::BitReader;
#[doc = "Field `pu_xtal32k_buf` writer - "]
pub type PU_XTAL32K_BUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal32k` reader - "]
pub type PU_XTAL32K_R = crate::BitReader;
#[doc = "Field `pu_xtal32k` writer - "]
pub type PU_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_lowv_en` reader - "]
pub type XTAL32K_LOWV_EN_R = crate::BitReader;
#[doc = "Field `xtal32k_lowv_en` writer - "]
pub type XTAL32K_LOWV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_hiz_en` reader - "]
pub type XTAL32K_HIZ_EN_R = crate::BitReader;
#[doc = "Field `xtal32k_hiz_en` writer - "]
pub type XTAL32K_HIZ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal32k` reader - "]
pub type DTEN_XTAL32K_R = crate::BitReader;
#[doc = "Field `dten_xtal32k` writer - "]
pub type DTEN_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal32k` reader - "]
pub type TEN_XTAL32K_R = crate::BitReader;
#[doc = "Field `ten_xtal32k` writer - "]
pub type TEN_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&self) -> XTAL32K_EXT_SEL_R {
        XTAL32K_EXT_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&self) -> XTAL32K_AMP_CTRL_R {
        XTAL32K_AMP_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&self) -> XTAL32K_REG_R {
        XTAL32K_REG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&self) -> XTAL32K_OUTBUF_STRE_R {
        XTAL32K_OUTBUF_STRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&self) -> XTAL32K_OTF_SHORT_R {
        XTAL32K_OTF_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&self) -> XTAL32K_INV_STRE_R {
        XTAL32K_INV_STRE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&self) -> XTAL32K_CAPBANK_R {
        XTAL32K_CAPBANK_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&self) -> XTAL32K_AC_CAP_SHORT_R {
        XTAL32K_AC_CAP_SHORT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&self) -> PU_XTAL32K_BUF_R {
        PU_XTAL32K_BUF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&self) -> PU_XTAL32K_R {
        PU_XTAL32K_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn xtal32k_lowv_en(&self) -> XTAL32K_LOWV_EN_R {
        XTAL32K_LOWV_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn xtal32k_hiz_en(&self) -> XTAL32K_HIZ_EN_R {
        XTAL32K_HIZ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DTEN_XTAL32K_R {
        DTEN_XTAL32K_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TEN_XTAL32K_R {
        TEN_XTAL32K_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_ext_sel(&mut self) -> XTAL32K_EXT_SEL_W<XTAL32K_SPEC> {
        XTAL32K_EXT_SEL_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_amp_ctrl(&mut self) -> XTAL32K_AMP_CTRL_W<XTAL32K_SPEC> {
        XTAL32K_AMP_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_reg(&mut self) -> XTAL32K_REG_W<XTAL32K_SPEC> {
        XTAL32K_REG_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_outbuf_stre(&mut self) -> XTAL32K_OUTBUF_STRE_W<XTAL32K_SPEC> {
        XTAL32K_OUTBUF_STRE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_otf_short(&mut self) -> XTAL32K_OTF_SHORT_W<XTAL32K_SPEC> {
        XTAL32K_OTF_SHORT_W::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_inv_stre(&mut self) -> XTAL32K_INV_STRE_W<XTAL32K_SPEC> {
        XTAL32K_INV_STRE_W::new(self, 9)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_capbank(&mut self) -> XTAL32K_CAPBANK_W<XTAL32K_SPEC> {
        XTAL32K_CAPBANK_W::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_ac_cap_short(&mut self) -> XTAL32K_AC_CAP_SHORT_W<XTAL32K_SPEC> {
        XTAL32K_AC_CAP_SHORT_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal32k_buf(&mut self) -> PU_XTAL32K_BUF_W<XTAL32K_SPEC> {
        PU_XTAL32K_BUF_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal32k(&mut self) -> PU_XTAL32K_W<XTAL32K_SPEC> {
        PU_XTAL32K_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_lowv_en(&mut self) -> XTAL32K_LOWV_EN_W<XTAL32K_SPEC> {
        XTAL32K_LOWV_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_hiz_en(&mut self) -> XTAL32K_HIZ_EN_W<XTAL32K_SPEC> {
        XTAL32K_HIZ_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dten_xtal32k(&mut self) -> DTEN_XTAL32K_W<XTAL32K_SPEC> {
        DTEN_XTAL32K_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ten_xtal32k(&mut self) -> TEN_XTAL32K_W<XTAL32K_SPEC> {
        TEN_XTAL32K_W::new(self, 23)
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
#[doc = "External crystal oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k::R`](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal32k::W`](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal32k to value 0"]
impl crate::Resettable for XTAL32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
