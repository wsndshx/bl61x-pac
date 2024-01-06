#[doc = "Register `vbat_ldo` reader"]
pub type R = crate::R<VBAT_LDO_SPEC>;
#[doc = "Register `vbat_ldo` writer"]
pub type W = crate::W<VBAT_LDO_SPEC>;
#[doc = "Field `ldo33_bm_aon` reader - "]
pub type LDO33_BM_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_bm_aon` writer - "]
pub type LDO33_BM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo33_cc_aon` reader - "]
pub type LDO33_CC_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_cc_aon` writer - "]
pub type LDO33_CC_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo33_ocp_en_aon` reader - "]
pub type LDO33_OCP_EN_AON_R = crate::BitReader;
#[doc = "Field `ldo33_ocp_en_aon` writer - "]
pub type LDO33_OCP_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo33_ocp_th_aon` reader - "]
pub type LDO33_OCP_TH_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_ocp_th_aon` writer - "]
pub type LDO33_OCP_TH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo33_sstart_delay_aon` reader - "]
pub type LDO33_SSTART_DELAY_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_sstart_delay_aon` writer - "]
pub type LDO33_SSTART_DELAY_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo33_sstart_en_aon` reader - "]
pub type LDO33_SSTART_EN_AON_R = crate::BitReader;
#[doc = "Field `ldo33_sstart_en_aon` writer - "]
pub type LDO33_SSTART_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo33_vout_sel_aon` reader - "]
pub type LDO33_VOUT_SEL_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_vout_sel_aon` writer - "]
pub type LDO33_VOUT_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo33_vout_trim_aon` reader - "]
pub type LDO33_VOUT_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_vout_trim_aon` writer - "]
pub type LDO33_VOUT_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo33_otp_en_aon` reader - "]
pub type LDO33_OTP_EN_AON_R = crate::BitReader;
#[doc = "Field `ldo33_otp_en_aon` writer - "]
pub type LDO33_OTP_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo33_otp_out_aon` reader - "]
pub type LDO33_OTP_OUT_AON_R = crate::BitReader;
#[doc = "Field `ldo33_otp_out_aon` writer - "]
pub type LDO33_OTP_OUT_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo33_ocp_out_aon` reader - "]
pub type LDO33_OCP_OUT_AON_R = crate::BitReader;
#[doc = "Field `ldo33_ocp_out_aon` writer - "]
pub type LDO33_OCP_OUT_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo33_aon` reader - "]
pub type TEN_LDO33_AON_R = crate::BitReader;
#[doc = "Field `ten_ldo33_aon` writer - "]
pub type TEN_LDO33_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo33_otp_th_aon` reader - "]
pub type LDO33_OTP_TH_AON_R = crate::FieldReader;
#[doc = "Field `ldo33_otp_th_aon` writer - "]
pub type LDO33_OTP_TH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo33_otp_sd_aon` reader - "]
pub type LDO33_OTP_SD_AON_R = crate::BitReader;
#[doc = "Field `ldo33_otp_sd_aon` writer - "]
pub type LDO33_OTP_SD_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ldo33_bm_aon(&self) -> LDO33_BM_AON_R {
        LDO33_BM_AON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo33_cc_aon(&self) -> LDO33_CC_AON_R {
        LDO33_CC_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ldo33_ocp_en_aon(&self) -> LDO33_OCP_EN_AON_R {
        LDO33_OCP_EN_AON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo33_ocp_th_aon(&self) -> LDO33_OCP_TH_AON_R {
        LDO33_OCP_TH_AON_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo33_sstart_delay_aon(&self) -> LDO33_SSTART_DELAY_AON_R {
        LDO33_SSTART_DELAY_AON_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo33_sstart_en_aon(&self) -> LDO33_SSTART_EN_AON_R {
        LDO33_SSTART_EN_AON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ldo33_vout_sel_aon(&self) -> LDO33_VOUT_SEL_AON_R {
        LDO33_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo33_vout_trim_aon(&self) -> LDO33_VOUT_TRIM_AON_R {
        LDO33_VOUT_TRIM_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ldo33_otp_en_aon(&self) -> LDO33_OTP_EN_AON_R {
        LDO33_OTP_EN_AON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ldo33_otp_out_aon(&self) -> LDO33_OTP_OUT_AON_R {
        LDO33_OTP_OUT_AON_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ldo33_ocp_out_aon(&self) -> LDO33_OCP_OUT_AON_R {
        LDO33_OCP_OUT_AON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_ldo33_aon(&self) -> TEN_LDO33_AON_R {
        TEN_LDO33_AON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ldo33_otp_th_aon(&self) -> LDO33_OTP_TH_AON_R {
        LDO33_OTP_TH_AON_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ldo33_otp_sd_aon(&self) -> LDO33_OTP_SD_AON_R {
        LDO33_OTP_SD_AON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_bm_aon(&mut self) -> LDO33_BM_AON_W<VBAT_LDO_SPEC> {
        LDO33_BM_AON_W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_cc_aon(&mut self) -> LDO33_CC_AON_W<VBAT_LDO_SPEC> {
        LDO33_CC_AON_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_ocp_en_aon(&mut self) -> LDO33_OCP_EN_AON_W<VBAT_LDO_SPEC> {
        LDO33_OCP_EN_AON_W::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_ocp_th_aon(&mut self) -> LDO33_OCP_TH_AON_W<VBAT_LDO_SPEC> {
        LDO33_OCP_TH_AON_W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_sstart_delay_aon(&mut self) -> LDO33_SSTART_DELAY_AON_W<VBAT_LDO_SPEC> {
        LDO33_SSTART_DELAY_AON_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_sstart_en_aon(&mut self) -> LDO33_SSTART_EN_AON_W<VBAT_LDO_SPEC> {
        LDO33_SSTART_EN_AON_W::new(self, 15)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_vout_sel_aon(&mut self) -> LDO33_VOUT_SEL_AON_W<VBAT_LDO_SPEC> {
        LDO33_VOUT_SEL_AON_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_vout_trim_aon(&mut self) -> LDO33_VOUT_TRIM_AON_W<VBAT_LDO_SPEC> {
        LDO33_VOUT_TRIM_AON_W::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_otp_en_aon(&mut self) -> LDO33_OTP_EN_AON_W<VBAT_LDO_SPEC> {
        LDO33_OTP_EN_AON_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_otp_out_aon(&mut self) -> LDO33_OTP_OUT_AON_W<VBAT_LDO_SPEC> {
        LDO33_OTP_OUT_AON_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_ocp_out_aon(&mut self) -> LDO33_OCP_OUT_AON_W<VBAT_LDO_SPEC> {
        LDO33_OCP_OUT_AON_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo33_aon(&mut self) -> TEN_LDO33_AON_W<VBAT_LDO_SPEC> {
        TEN_LDO33_AON_W::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_otp_th_aon(&mut self) -> LDO33_OTP_TH_AON_W<VBAT_LDO_SPEC> {
        LDO33_OTP_TH_AON_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ldo33_otp_sd_aon(&mut self) -> LDO33_OTP_SD_AON_W<VBAT_LDO_SPEC> {
        LDO33_OTP_SD_AON_W::new(self, 31)
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
#[doc = "vbat_ldo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbat_ldo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbat_ldo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBAT_LDO_SPEC;
impl crate::RegisterSpec for VBAT_LDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbat_ldo::R`](R) reader structure"]
impl crate::Readable for VBAT_LDO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbat_ldo::W`](W) writer structure"]
impl crate::Writable for VBAT_LDO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vbat_ldo to value 0"]
impl crate::Resettable for VBAT_LDO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
