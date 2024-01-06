#[doc = "Register `rc32k` reader"]
pub type R = crate::R<RC32K_SPEC>;
#[doc = "Register `rc32k` writer"]
pub type W = crate::W<RC32K_SPEC>;
#[doc = "Field `rc32k_cal_done` reader - "]
pub type RC32K_CAL_DONE_R = crate::BitReader;
#[doc = "Field `rc32k_cal_done` writer - "]
pub type RC32K_CAL_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_rdy` reader - "]
pub type RC32K_RDY_R = crate::BitReader;
#[doc = "Field `rc32k_rdy` writer - "]
pub type RC32K_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_cal_inprogress` reader - "]
pub type RC32K_CAL_INPROGRESS_R = crate::BitReader;
#[doc = "Field `rc32k_cal_inprogress` writer - "]
pub type RC32K_CAL_INPROGRESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_cal_div` reader - "]
pub type RC32K_CAL_DIV_R = crate::FieldReader;
#[doc = "Field `rc32k_cal_div` writer - "]
pub type RC32K_CAL_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32k_cal_precharge` reader - "]
pub type RC32K_CAL_PRECHARGE_R = crate::BitReader;
#[doc = "Field `rc32k_cal_precharge` writer - "]
pub type RC32K_CAL_PRECHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_dig_code_fr_cal` reader - "]
pub type RC32K_DIG_CODE_FR_CAL_R = crate::FieldReader<u16>;
#[doc = "Field `rc32k_dig_code_fr_cal` writer - "]
pub type RC32K_DIG_CODE_FR_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rc32k_vref_dly` reader - "]
pub type RC32K_VREF_DLY_R = crate::FieldReader;
#[doc = "Field `rc32k_vref_dly` writer - "]
pub type RC32K_VREF_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32k_allow_cal` reader - "]
pub type RC32K_ALLOW_CAL_R = crate::BitReader;
#[doc = "Field `rc32k_allow_cal` writer - "]
pub type RC32K_ALLOW_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_ext_code_en` reader - "]
pub type RC32K_EXT_CODE_EN_R = crate::BitReader;
#[doc = "Field `rc32k_ext_code_en` writer - "]
pub type RC32K_EXT_CODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_cal_en` reader - "]
pub type RC32K_CAL_EN_R = crate::BitReader;
#[doc = "Field `rc32k_cal_en` writer - "]
pub type RC32K_CAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rc32k` reader - "]
pub type PU_RC32K_R = crate::BitReader;
#[doc = "Field `pu_rc32k` writer - "]
pub type PU_RC32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_code_fr_ext` reader - "]
pub type RC32K_CODE_FR_EXT_R = crate::FieldReader<u16>;
#[doc = "Field `rc32k_code_fr_ext` writer - "]
pub type RC32K_CODE_FR_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&self) -> RC32K_CAL_DONE_R {
        RC32K_CAL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&self) -> RC32K_RDY_R {
        RC32K_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&self) -> RC32K_CAL_INPROGRESS_R {
        RC32K_CAL_INPROGRESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&self) -> RC32K_CAL_DIV_R {
        RC32K_CAL_DIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&self) -> RC32K_CAL_PRECHARGE_R {
        RC32K_CAL_PRECHARGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&self) -> RC32K_DIG_CODE_FR_CAL_R {
        RC32K_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&self) -> RC32K_VREF_DLY_R {
        RC32K_VREF_DLY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&self) -> RC32K_ALLOW_CAL_R {
        RC32K_ALLOW_CAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&self) -> RC32K_EXT_CODE_EN_R {
        RC32K_EXT_CODE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&self) -> RC32K_CAL_EN_R {
        RC32K_CAL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_rc32k(&self) -> PU_RC32K_R {
        PU_RC32K_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&self) -> RC32K_CODE_FR_EXT_R {
        RC32K_CODE_FR_EXT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_cal_done(&mut self) -> RC32K_CAL_DONE_W<RC32K_SPEC> {
        RC32K_CAL_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_rdy(&mut self) -> RC32K_RDY_W<RC32K_SPEC> {
        RC32K_RDY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_cal_inprogress(&mut self) -> RC32K_CAL_INPROGRESS_W<RC32K_SPEC> {
        RC32K_CAL_INPROGRESS_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_cal_div(&mut self) -> RC32K_CAL_DIV_W<RC32K_SPEC> {
        RC32K_CAL_DIV_W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_cal_precharge(&mut self) -> RC32K_CAL_PRECHARGE_W<RC32K_SPEC> {
        RC32K_CAL_PRECHARGE_W::new(self, 5)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_dig_code_fr_cal(&mut self) -> RC32K_DIG_CODE_FR_CAL_W<RC32K_SPEC> {
        RC32K_DIG_CODE_FR_CAL_W::new(self, 6)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_vref_dly(&mut self) -> RC32K_VREF_DLY_W<RC32K_SPEC> {
        RC32K_VREF_DLY_W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_allow_cal(&mut self) -> RC32K_ALLOW_CAL_W<RC32K_SPEC> {
        RC32K_ALLOW_CAL_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_ext_code_en(&mut self) -> RC32K_EXT_CODE_EN_W<RC32K_SPEC> {
        RC32K_EXT_CODE_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_cal_en(&mut self) -> RC32K_CAL_EN_W<RC32K_SPEC> {
        RC32K_CAL_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rc32k(&mut self) -> PU_RC32K_W<RC32K_SPEC> {
        PU_RC32K_W::new(self, 21)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_code_fr_ext(&mut self) -> RC32K_CODE_FR_EXT_W<RC32K_SPEC> {
        RC32K_CODE_FR_EXT_W::new(self, 22)
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
#[doc = "32-kHz internal RC oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32K_SPEC;
impl crate::RegisterSpec for RC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32k::R`](R) reader structure"]
impl crate::Readable for RC32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32k::W`](W) writer structure"]
impl crate::Writable for RC32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32k to value 0"]
impl crate::Resettable for RC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
