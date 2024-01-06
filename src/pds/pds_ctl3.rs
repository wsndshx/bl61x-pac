#[doc = "Register `PDS_CTL3` reader"]
pub type R = crate::R<PDS_CTL3_SPEC>;
#[doc = "Register `PDS_CTL3` writer"]
pub type W = crate::W<PDS_CTL3_SPEC>;
#[doc = "Field `cr_pds_force_misc_pwr_off` reader - "]
pub type CR_PDS_FORCE_MISC_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_pwr_off` writer - "]
pub type CR_PDS_FORCE_MISC_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_iso_en` reader - "]
pub type CR_PDS_FORCE_MISC_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_iso_en` writer - "]
pub type CR_PDS_FORCE_MISC_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_pds_rst` reader - "]
pub type CR_PDS_FORCE_MISC_PDS_RST_R = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_pds_rst` writer - "]
pub type CR_PDS_FORCE_MISC_PDS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_mem_stby` reader - "]
pub type CR_PDS_FORCE_MISC_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_mem_stby` writer - "]
pub type CR_PDS_FORCE_MISC_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_gate_clk` reader - "]
pub type CR_PDS_FORCE_MISC_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_gate_clk` writer - "]
pub type CR_PDS_FORCE_MISC_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_iso_en` reader - "]
pub type CR_PDS_NP_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_np_iso_en` writer - "]
pub type CR_PDS_NP_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_iso_en` reader - "]
pub type CR_PDS_WB_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_wb_iso_en` writer - "]
pub type CR_PDS_WB_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_iso_en` reader - "]
pub type CR_PDS_USB_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_usb_iso_en` writer - "]
pub type CR_PDS_USB_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_iso_en` reader - "]
pub type CR_PDS_MISC_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_misc_iso_en` writer - "]
pub type CR_PDS_MISC_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&self) -> CR_PDS_FORCE_MISC_PWR_OFF_R {
        CR_PDS_FORCE_MISC_PWR_OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&self) -> CR_PDS_FORCE_MISC_ISO_EN_R {
        CR_PDS_FORCE_MISC_ISO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&self) -> CR_PDS_FORCE_MISC_PDS_RST_R {
        CR_PDS_FORCE_MISC_PDS_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&self) -> CR_PDS_FORCE_MISC_MEM_STBY_R {
        CR_PDS_FORCE_MISC_MEM_STBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&self) -> CR_PDS_FORCE_MISC_GATE_CLK_R {
        CR_PDS_FORCE_MISC_GATE_CLK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_np_iso_en(&self) -> CR_PDS_NP_ISO_EN_R {
        CR_PDS_NP_ISO_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_wb_iso_en(&self) -> CR_PDS_WB_ISO_EN_R {
        CR_PDS_WB_ISO_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_pds_usb_iso_en(&self) -> CR_PDS_USB_ISO_EN_R {
        CR_PDS_USB_ISO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&self) -> CR_PDS_MISC_ISO_EN_R {
        CR_PDS_MISC_ISO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_pwr_off(&mut self) -> CR_PDS_FORCE_MISC_PWR_OFF_W<PDS_CTL3_SPEC> {
        CR_PDS_FORCE_MISC_PWR_OFF_W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_iso_en(&mut self) -> CR_PDS_FORCE_MISC_ISO_EN_W<PDS_CTL3_SPEC> {
        CR_PDS_FORCE_MISC_ISO_EN_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_pds_rst(&mut self) -> CR_PDS_FORCE_MISC_PDS_RST_W<PDS_CTL3_SPEC> {
        CR_PDS_FORCE_MISC_PDS_RST_W::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_mem_stby(&mut self) -> CR_PDS_FORCE_MISC_MEM_STBY_W<PDS_CTL3_SPEC> {
        CR_PDS_FORCE_MISC_MEM_STBY_W::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_gate_clk(&mut self) -> CR_PDS_FORCE_MISC_GATE_CLK_W<PDS_CTL3_SPEC> {
        CR_PDS_FORCE_MISC_GATE_CLK_W::new(self, 13)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_iso_en(&mut self) -> CR_PDS_NP_ISO_EN_W<PDS_CTL3_SPEC> {
        CR_PDS_NP_ISO_EN_W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_iso_en(&mut self) -> CR_PDS_WB_ISO_EN_W<PDS_CTL3_SPEC> {
        CR_PDS_WB_ISO_EN_W::new(self, 27)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_usb_iso_en(&mut self) -> CR_PDS_USB_ISO_EN_W<PDS_CTL3_SPEC> {
        CR_PDS_USB_ISO_EN_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_iso_en(&mut self) -> CR_PDS_MISC_ISO_EN_W<PDS_CTL3_SPEC> {
        CR_PDS_MISC_ISO_EN_W::new(self, 30)
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
#[doc = "PDS_CTL3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_CTL3_SPEC;
impl crate::RegisterSpec for PDS_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl3::R`](R) reader structure"]
impl crate::Readable for PDS_CTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl3::W`](W) writer structure"]
impl crate::Writable for PDS_CTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL3 to value 0"]
impl crate::Resettable for PDS_CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
