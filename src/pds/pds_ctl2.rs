#[doc = "Register `PDS_CTL2` reader"]
pub type R = crate::R<PDS_CTL2_SPEC>;
#[doc = "Register `PDS_CTL2` writer"]
pub type W = crate::W<PDS_CTL2_SPEC>;
#[doc = "Field `cr_pds_force_np_pwr_off` reader - "]
pub type CR_PDS_FORCE_NP_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_force_np_pwr_off` writer - "]
pub type CR_PDS_FORCE_NP_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_pwr_off` reader - "]
pub type CR_PDS_FORCE_WB_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_pwr_off` writer - "]
pub type CR_PDS_FORCE_WB_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_pwr_off` reader - "]
pub type CR_PDS_FORCE_USB_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_pwr_off` writer - "]
pub type CR_PDS_FORCE_USB_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_iso_en` reader - "]
pub type CR_PDS_FORCE_NP_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_force_np_iso_en` writer - "]
pub type CR_PDS_FORCE_NP_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_iso_en` reader - "]
pub type CR_PDS_FORCE_WB_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_iso_en` writer - "]
pub type CR_PDS_FORCE_WB_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_iso_en` reader - "]
pub type CR_PDS_FORCE_USB_ISO_EN_R = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_iso_en` writer - "]
pub type CR_PDS_FORCE_USB_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_pds_rst` reader - "]
pub type CR_PDS_FORCE_NP_PDS_RST_R = crate::BitReader;
#[doc = "Field `cr_pds_force_np_pds_rst` writer - "]
pub type CR_PDS_FORCE_NP_PDS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_pds_rst` reader - "]
pub type CR_PDS_FORCE_WB_PDS_RST_R = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_pds_rst` writer - "]
pub type CR_PDS_FORCE_WB_PDS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_pds_rst` reader - "]
pub type CR_PDS_FORCE_USB_PDS_RST_R = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_pds_rst` writer - "]
pub type CR_PDS_FORCE_USB_PDS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_mem_stby` reader - "]
pub type CR_PDS_FORCE_NP_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_force_np_mem_stby` writer - "]
pub type CR_PDS_FORCE_NP_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_mem_stby` reader - "]
pub type CR_PDS_FORCE_WB_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_mem_stby` writer - "]
pub type CR_PDS_FORCE_WB_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_mem_stby` reader - "]
pub type CR_PDS_FORCE_USB_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_mem_stby` writer - "]
pub type CR_PDS_FORCE_USB_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_gate_clk` reader - "]
pub type CR_PDS_FORCE_NP_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_force_np_gate_clk` writer - "]
pub type CR_PDS_FORCE_NP_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_gate_clk` reader - "]
pub type CR_PDS_FORCE_WB_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_gate_clk` writer - "]
pub type CR_PDS_FORCE_WB_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_gate_clk` reader - "]
pub type CR_PDS_FORCE_USB_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_gate_clk` writer - "]
pub type CR_PDS_FORCE_USB_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&self) -> CR_PDS_FORCE_NP_PWR_OFF_R {
        CR_PDS_FORCE_NP_PWR_OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&self) -> CR_PDS_FORCE_WB_PWR_OFF_R {
        CR_PDS_FORCE_WB_PWR_OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pwr_off(&self) -> CR_PDS_FORCE_USB_PWR_OFF_R {
        CR_PDS_FORCE_USB_PWR_OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&self) -> CR_PDS_FORCE_NP_ISO_EN_R {
        CR_PDS_FORCE_NP_ISO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&self) -> CR_PDS_FORCE_WB_ISO_EN_R {
        CR_PDS_FORCE_WB_ISO_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_usb_iso_en(&self) -> CR_PDS_FORCE_USB_ISO_EN_R {
        CR_PDS_FORCE_USB_ISO_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CR_PDS_FORCE_NP_PDS_RST_R {
        CR_PDS_FORCE_NP_PDS_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CR_PDS_FORCE_WB_PDS_RST_R {
        CR_PDS_FORCE_WB_PDS_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pds_rst(&self) -> CR_PDS_FORCE_USB_PDS_RST_R {
        CR_PDS_FORCE_USB_PDS_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CR_PDS_FORCE_NP_MEM_STBY_R {
        CR_PDS_FORCE_NP_MEM_STBY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CR_PDS_FORCE_WB_MEM_STBY_R {
        CR_PDS_FORCE_WB_MEM_STBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_force_usb_mem_stby(&self) -> CR_PDS_FORCE_USB_MEM_STBY_R {
        CR_PDS_FORCE_USB_MEM_STBY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CR_PDS_FORCE_NP_GATE_CLK_R {
        CR_PDS_FORCE_NP_GATE_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CR_PDS_FORCE_WB_GATE_CLK_R {
        CR_PDS_FORCE_WB_GATE_CLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_force_usb_gate_clk(&self) -> CR_PDS_FORCE_USB_GATE_CLK_R {
        CR_PDS_FORCE_USB_GATE_CLK_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_pwr_off(&mut self) -> CR_PDS_FORCE_NP_PWR_OFF_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_NP_PWR_OFF_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_pwr_off(&mut self) -> CR_PDS_FORCE_WB_PWR_OFF_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_WB_PWR_OFF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_pwr_off(&mut self) -> CR_PDS_FORCE_USB_PWR_OFF_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_USB_PWR_OFF_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_iso_en(&mut self) -> CR_PDS_FORCE_NP_ISO_EN_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_NP_ISO_EN_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_iso_en(&mut self) -> CR_PDS_FORCE_WB_ISO_EN_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_WB_ISO_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_iso_en(&mut self) -> CR_PDS_FORCE_USB_ISO_EN_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_USB_ISO_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CR_PDS_FORCE_NP_PDS_RST_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_NP_PDS_RST_W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CR_PDS_FORCE_WB_PDS_RST_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_WB_PDS_RST_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_pds_rst(&mut self) -> CR_PDS_FORCE_USB_PDS_RST_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_USB_PDS_RST_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CR_PDS_FORCE_NP_MEM_STBY_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_NP_MEM_STBY_W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CR_PDS_FORCE_WB_MEM_STBY_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_WB_MEM_STBY_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_mem_stby(&mut self) -> CR_PDS_FORCE_USB_MEM_STBY_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_USB_MEM_STBY_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CR_PDS_FORCE_NP_GATE_CLK_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_NP_GATE_CLK_W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CR_PDS_FORCE_WB_GATE_CLK_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_WB_GATE_CLK_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_gate_clk(&mut self) -> CR_PDS_FORCE_USB_GATE_CLK_W<PDS_CTL2_SPEC> {
        CR_PDS_FORCE_USB_GATE_CLK_W::new(self, 19)
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
#[doc = "PDS_CTL2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_CTL2_SPEC;
impl crate::RegisterSpec for PDS_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl2::R`](R) reader structure"]
impl crate::Readable for PDS_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl2::W`](W) writer structure"]
impl crate::Writable for PDS_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL2 to value 0"]
impl crate::Resettable for PDS_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
