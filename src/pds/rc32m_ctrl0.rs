#[doc = "Register `rc32m_ctrl0` reader"]
pub type R = crate::R<RC32M_CTRL0_SPEC>;
#[doc = "Register `rc32m_ctrl0` writer"]
pub type W = crate::W<RC32M_CTRL0_SPEC>;
#[doc = "Field `rc32m_cal_done` reader - "]
pub type RC32M_CAL_DONE_R = crate::BitReader;
#[doc = "Field `rc32m_cal_done` writer - "]
pub type RC32M_CAL_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_rdy` reader - "]
pub type RC32M_RDY_R = crate::BitReader;
#[doc = "Field `rc32m_rdy` writer - "]
pub type RC32M_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_cal_inprogress` reader - "]
pub type RC32M_CAL_INPROGRESS_R = crate::BitReader;
#[doc = "Field `rc32m_cal_inprogress` writer - "]
pub type RC32M_CAL_INPROGRESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_cal_div` reader - "]
pub type RC32M_CAL_DIV_R = crate::FieldReader;
#[doc = "Field `rc32m_cal_div` writer - "]
pub type RC32M_CAL_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32m_cal_precharge` reader - "]
pub type RC32M_CAL_PRECHARGE_R = crate::BitReader;
#[doc = "Field `rc32m_cal_precharge` writer - "]
pub type RC32M_CAL_PRECHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_dig_code_fr_cal` reader - "]
pub type RC32M_DIG_CODE_FR_CAL_R = crate::FieldReader;
#[doc = "Field `rc32m_dig_code_fr_cal` writer - "]
pub type RC32M_DIG_CODE_FR_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rc32m_allow_cal` reader - "]
pub type RC32M_ALLOW_CAL_R = crate::BitReader;
#[doc = "Field `rc32m_allow_cal` writer - "]
pub type RC32M_ALLOW_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_refclk_half` reader - "]
pub type RC32M_REFCLK_HALF_R = crate::BitReader;
#[doc = "Field `rc32m_refclk_half` writer - "]
pub type RC32M_REFCLK_HALF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_ext_code_en` reader - "]
pub type RC32M_EXT_CODE_EN_R = crate::BitReader;
#[doc = "Field `rc32m_ext_code_en` writer - "]
pub type RC32M_EXT_CODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_cal_en` reader - "]
pub type RC32M_CAL_EN_R = crate::BitReader;
#[doc = "Field `rc32m_cal_en` writer - "]
pub type RC32M_CAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_pd` reader - "]
pub type RC32M_PD_R = crate::BitReader;
#[doc = "Field `rc32m_pd` writer - "]
pub type RC32M_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_code_fr_ext` reader - "]
pub type RC32M_CODE_FR_EXT_R = crate::FieldReader;
#[doc = "Field `rc32m_code_fr_ext` writer - "]
pub type RC32M_CODE_FR_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_cal_done(&self) -> RC32M_CAL_DONE_R {
        RC32M_CAL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_rdy(&self) -> RC32M_RDY_R {
        RC32M_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_cal_inprogress(&self) -> RC32M_CAL_INPROGRESS_R {
        RC32M_CAL_INPROGRESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&self) -> RC32M_CAL_DIV_R {
        RC32M_CAL_DIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32m_cal_precharge(&self) -> RC32M_CAL_PRECHARGE_R {
        RC32M_CAL_PRECHARGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn rc32m_dig_code_fr_cal(&self) -> RC32M_DIG_CODE_FR_CAL_R {
        RC32M_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&self) -> RC32M_ALLOW_CAL_R {
        RC32M_ALLOW_CAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&self) -> RC32M_REFCLK_HALF_R {
        RC32M_REFCLK_HALF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&self) -> RC32M_EXT_CODE_EN_R {
        RC32M_EXT_CODE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&self) -> RC32M_CAL_EN_R {
        RC32M_CAL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&self) -> RC32M_PD_R {
        RC32M_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&self) -> RC32M_CODE_FR_EXT_R {
        RC32M_CODE_FR_EXT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_cal_done(&mut self) -> RC32M_CAL_DONE_W<RC32M_CTRL0_SPEC> {
        RC32M_CAL_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_rdy(&mut self) -> RC32M_RDY_W<RC32M_CTRL0_SPEC> {
        RC32M_RDY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_cal_inprogress(&mut self) -> RC32M_CAL_INPROGRESS_W<RC32M_CTRL0_SPEC> {
        RC32M_CAL_INPROGRESS_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_cal_div(&mut self) -> RC32M_CAL_DIV_W<RC32M_CTRL0_SPEC> {
        RC32M_CAL_DIV_W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_cal_precharge(&mut self) -> RC32M_CAL_PRECHARGE_W<RC32M_CTRL0_SPEC> {
        RC32M_CAL_PRECHARGE_W::new(self, 5)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_dig_code_fr_cal(&mut self) -> RC32M_DIG_CODE_FR_CAL_W<RC32M_CTRL0_SPEC> {
        RC32M_DIG_CODE_FR_CAL_W::new(self, 6)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_allow_cal(&mut self) -> RC32M_ALLOW_CAL_W<RC32M_CTRL0_SPEC> {
        RC32M_ALLOW_CAL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_refclk_half(&mut self) -> RC32M_REFCLK_HALF_W<RC32M_CTRL0_SPEC> {
        RC32M_REFCLK_HALF_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_ext_code_en(&mut self) -> RC32M_EXT_CODE_EN_W<RC32M_CTRL0_SPEC> {
        RC32M_EXT_CODE_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_cal_en(&mut self) -> RC32M_CAL_EN_W<RC32M_CTRL0_SPEC> {
        RC32M_CAL_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_pd(&mut self) -> RC32M_PD_W<RC32M_CTRL0_SPEC> {
        RC32M_PD_W::new(self, 21)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_code_fr_ext(&mut self) -> RC32M_CODE_FR_EXT_W<RC32M_CTRL0_SPEC> {
        RC32M_CODE_FR_EXT_W::new(self, 22)
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
#[doc = "rc32m_ctrl0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32M_CTRL0_SPEC;
impl crate::RegisterSpec for RC32M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32m_ctrl0::R`](R) reader structure"]
impl crate::Readable for RC32M_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32m_ctrl0::W`](W) writer structure"]
impl crate::Writable for RC32M_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32m_ctrl0 to value 0"]
impl crate::Resettable for RC32M_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
