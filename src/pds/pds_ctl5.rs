#[doc = "Register `PDS_CTL5` reader"]
pub type R = crate::R<PDS_CTL5_SPEC>;
#[doc = "Register `PDS_CTL5` writer"]
pub type W = crate::W<PDS_CTL5_SPEC>;
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub type CR_NP_WFI_MASK_R = crate::BitReader;
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub type CR_NP_WFI_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pad_od_en` reader - "]
pub type CR_PDS_PAD_OD_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_pad_od_en` writer - "]
pub type CR_PDS_PAD_OD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usb33` reader - "]
pub type CR_PDS_CTRL_USB33_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usb33` writer - "]
pub type CR_PDS_CTRL_USB33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_ldo18io` reader - "]
pub type CR_PDS_PD_LDO18IO_R = crate::BitReader;
#[doc = "Field `cr_pds_pd_ldo18io` writer - "]
pub type CR_PDS_PD_LDO18IO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gpio_keep_en` reader - "]
pub type CR_PDS_GPIO_KEEP_EN_R = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_keep_en` writer - "]
pub type CR_PDS_GPIO_KEEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CR_NP_WFI_MASK_R {
        CR_NP_WFI_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_pad_od_en(&self) -> CR_PDS_PAD_OD_EN_R {
        CR_PDS_PAD_OD_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb33(&self) -> CR_PDS_CTRL_USB33_R {
        CR_PDS_CTRL_USB33_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo18io(&self) -> CR_PDS_PD_LDO18IO_R {
        CR_PDS_PD_LDO18IO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cr_pds_gpio_keep_en(&self) -> CR_PDS_GPIO_KEEP_EN_R {
        CR_PDS_GPIO_KEEP_EN_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_np_wfi_mask(&mut self) -> CR_NP_WFI_MASK_W<PDS_CTL5_SPEC> {
        CR_NP_WFI_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pad_od_en(&mut self) -> CR_PDS_PAD_OD_EN_W<PDS_CTL5_SPEC> {
        CR_PDS_PAD_OD_EN_W::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_usb33(&mut self) -> CR_PDS_CTRL_USB33_W<PDS_CTL5_SPEC> {
        CR_PDS_CTRL_USB33_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_ldo18io(&mut self) -> CR_PDS_PD_LDO18IO_W<PDS_CTL5_SPEC> {
        CR_PDS_PD_LDO18IO_W::new(self, 9)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_keep_en(&mut self) -> CR_PDS_GPIO_KEEP_EN_W<PDS_CTL5_SPEC> {
        CR_PDS_GPIO_KEEP_EN_W::new(self, 16)
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
#[doc = "PDS_CTL5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_CTL5_SPEC;
impl crate::RegisterSpec for PDS_CTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl5::R`](R) reader structure"]
impl crate::Readable for PDS_CTL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl5::W`](W) writer structure"]
impl crate::Writable for PDS_CTL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL5 to value 0"]
impl crate::Resettable for PDS_CTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
