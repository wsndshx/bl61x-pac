#[doc = "Register `HBN_PIR_CFG` reader"]
pub type R = crate::R<HBN_PIR_CFG_SPEC>;
#[doc = "Register `HBN_PIR_CFG` writer"]
pub type W = crate::W<HBN_PIR_CFG_SPEC>;
#[doc = "Field `pir_hpf_sel` reader - "]
pub type PIR_HPF_SEL_R = crate::FieldReader;
#[doc = "Field `pir_hpf_sel` writer - "]
pub type PIR_HPF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pir_lpf_sel` reader - "]
pub type PIR_LPF_SEL_R = crate::BitReader;
#[doc = "Field `pir_lpf_sel` writer - "]
pub type PIR_LPF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pir_dis` reader - "]
pub type PIR_DIS_R = crate::FieldReader;
#[doc = "Field `pir_dis` writer - "]
pub type PIR_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pir_en` reader - "]
pub type PIR_EN_R = crate::BitReader;
#[doc = "Field `pir_en` writer - "]
pub type PIR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_cs` reader - "]
pub type GPADC_CS_R = crate::BitReader;
#[doc = "Field `gpadc_cs` writer - "]
pub type GPADC_CS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&self) -> PIR_HPF_SEL_R {
        PIR_HPF_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&self) -> PIR_LPF_SEL_R {
        PIR_LPF_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&self) -> PIR_DIS_R {
        PIR_DIS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&self) -> PIR_EN_R {
        PIR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cs(&self) -> GPADC_CS_R {
        GPADC_CS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pir_hpf_sel(&mut self) -> PIR_HPF_SEL_W<HBN_PIR_CFG_SPEC> {
        PIR_HPF_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pir_lpf_sel(&mut self) -> PIR_LPF_SEL_W<HBN_PIR_CFG_SPEC> {
        PIR_LPF_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pir_dis(&mut self) -> PIR_DIS_W<HBN_PIR_CFG_SPEC> {
        PIR_DIS_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pir_en(&mut self) -> PIR_EN_W<HBN_PIR_CFG_SPEC> {
        PIR_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cs(&mut self) -> GPADC_CS_W<HBN_PIR_CFG_SPEC> {
        GPADC_CS_W::new(self, 8)
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
#[doc = "HBN_PIR_CFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pir_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pir_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_PIR_CFG_SPEC;
impl crate::RegisterSpec for HBN_PIR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pir_cfg::R`](R) reader structure"]
impl crate::Readable for HBN_PIR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_pir_cfg::W`](W) writer structure"]
impl crate::Writable for HBN_PIR_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PIR_CFG to value 0"]
impl crate::Resettable for HBN_PIR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
