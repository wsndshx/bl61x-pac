#[doc = "Register `PDS_CTL4` reader"]
pub type R = crate::R<PDS_CTL4_SPEC>;
#[doc = "Register `PDS_CTL4` writer"]
pub type W = crate::W<PDS_CTL4_SPEC>;
#[doc = "Field `cr_pds_np_pwr_off` reader - "]
pub type CR_PDS_NP_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_np_pwr_off` writer - "]
pub type CR_PDS_NP_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_reset` reader - "]
pub type CR_PDS_NP_RESET_R = crate::BitReader;
#[doc = "Field `cr_pds_np_reset` writer - "]
pub type CR_PDS_NP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_mem_stby` reader - "]
pub type CR_PDS_NP_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_np_mem_stby` writer - "]
pub type CR_PDS_NP_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_gate_clk` reader - "]
pub type CR_PDS_NP_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_np_gate_clk` writer - "]
pub type CR_PDS_NP_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_pwr_off` reader - "]
pub type CR_PDS_WB_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_wb_pwr_off` writer - "]
pub type CR_PDS_WB_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_reset` reader - "]
pub type CR_PDS_WB_RESET_R = crate::BitReader;
#[doc = "Field `cr_pds_wb_reset` writer - "]
pub type CR_PDS_WB_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_mem_stby` reader - "]
pub type CR_PDS_WB_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_wb_mem_stby` writer - "]
pub type CR_PDS_WB_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_gate_clk` reader - "]
pub type CR_PDS_WB_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_wb_gate_clk` writer - "]
pub type CR_PDS_WB_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_pwr_off` reader - "]
pub type CR_PDS_USB_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_usb_pwr_off` writer - "]
pub type CR_PDS_USB_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_reset` reader - "]
pub type CR_PDS_USB_RESET_R = crate::BitReader;
#[doc = "Field `cr_pds_usb_reset` writer - "]
pub type CR_PDS_USB_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_mem_stby` reader - "]
pub type CR_PDS_USB_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_usb_mem_stby` writer - "]
pub type CR_PDS_USB_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_gate_clk` reader - "]
pub type CR_PDS_USB_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_usb_gate_clk` writer - "]
pub type CR_PDS_USB_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_pwr_off` reader - "]
pub type CR_PDS_MISC_PWR_OFF_R = crate::BitReader;
#[doc = "Field `cr_pds_misc_pwr_off` writer - "]
pub type CR_PDS_MISC_PWR_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_reset` reader - "]
pub type CR_PDS_MISC_RESET_R = crate::BitReader;
#[doc = "Field `cr_pds_misc_reset` writer - "]
pub type CR_PDS_MISC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_mem_stby` reader - "]
pub type CR_PDS_MISC_MEM_STBY_R = crate::BitReader;
#[doc = "Field `cr_pds_misc_mem_stby` writer - "]
pub type CR_PDS_MISC_MEM_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_gate_clk` reader - "]
pub type CR_PDS_MISC_GATE_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_misc_gate_clk` writer - "]
pub type CR_PDS_MISC_GATE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&self) -> CR_PDS_NP_PWR_OFF_R {
        CR_PDS_NP_PWR_OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CR_PDS_NP_RESET_R {
        CR_PDS_NP_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CR_PDS_NP_MEM_STBY_R {
        CR_PDS_NP_MEM_STBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CR_PDS_NP_GATE_CLK_R {
        CR_PDS_NP_GATE_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&self) -> CR_PDS_WB_PWR_OFF_R {
        CR_PDS_WB_PWR_OFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&self) -> CR_PDS_WB_RESET_R {
        CR_PDS_WB_RESET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&self) -> CR_PDS_WB_MEM_STBY_R {
        CR_PDS_WB_MEM_STBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&self) -> CR_PDS_WB_GATE_CLK_R {
        CR_PDS_WB_GATE_CLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_usb_pwr_off(&self) -> CR_PDS_USB_PWR_OFF_R {
        CR_PDS_USB_PWR_OFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_usb_reset(&self) -> CR_PDS_USB_RESET_R {
        CR_PDS_USB_RESET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_usb_mem_stby(&self) -> CR_PDS_USB_MEM_STBY_R {
        CR_PDS_USB_MEM_STBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_pds_usb_gate_clk(&self) -> CR_PDS_USB_GATE_CLK_R {
        CR_PDS_USB_GATE_CLK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CR_PDS_MISC_PWR_OFF_R {
        CR_PDS_MISC_PWR_OFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CR_PDS_MISC_RESET_R {
        CR_PDS_MISC_RESET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CR_PDS_MISC_MEM_STBY_R {
        CR_PDS_MISC_MEM_STBY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CR_PDS_MISC_GATE_CLK_R {
        CR_PDS_MISC_GATE_CLK_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_pwr_off(&mut self) -> CR_PDS_NP_PWR_OFF_W<PDS_CTL4_SPEC> {
        CR_PDS_NP_PWR_OFF_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_reset(&mut self) -> CR_PDS_NP_RESET_W<PDS_CTL4_SPEC> {
        CR_PDS_NP_RESET_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_mem_stby(&mut self) -> CR_PDS_NP_MEM_STBY_W<PDS_CTL4_SPEC> {
        CR_PDS_NP_MEM_STBY_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_gate_clk(&mut self) -> CR_PDS_NP_GATE_CLK_W<PDS_CTL4_SPEC> {
        CR_PDS_NP_GATE_CLK_W::new(self, 3)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_pwr_off(&mut self) -> CR_PDS_WB_PWR_OFF_W<PDS_CTL4_SPEC> {
        CR_PDS_WB_PWR_OFF_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_reset(&mut self) -> CR_PDS_WB_RESET_W<PDS_CTL4_SPEC> {
        CR_PDS_WB_RESET_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_mem_stby(&mut self) -> CR_PDS_WB_MEM_STBY_W<PDS_CTL4_SPEC> {
        CR_PDS_WB_MEM_STBY_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_gate_clk(&mut self) -> CR_PDS_WB_GATE_CLK_W<PDS_CTL4_SPEC> {
        CR_PDS_WB_GATE_CLK_W::new(self, 15)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_usb_pwr_off(&mut self) -> CR_PDS_USB_PWR_OFF_W<PDS_CTL4_SPEC> {
        CR_PDS_USB_PWR_OFF_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_usb_reset(&mut self) -> CR_PDS_USB_RESET_W<PDS_CTL4_SPEC> {
        CR_PDS_USB_RESET_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_usb_mem_stby(&mut self) -> CR_PDS_USB_MEM_STBY_W<PDS_CTL4_SPEC> {
        CR_PDS_USB_MEM_STBY_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_usb_gate_clk(&mut self) -> CR_PDS_USB_GATE_CLK_W<PDS_CTL4_SPEC> {
        CR_PDS_USB_GATE_CLK_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CR_PDS_MISC_PWR_OFF_W<PDS_CTL4_SPEC> {
        CR_PDS_MISC_PWR_OFF_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_reset(&mut self) -> CR_PDS_MISC_RESET_W<PDS_CTL4_SPEC> {
        CR_PDS_MISC_RESET_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CR_PDS_MISC_MEM_STBY_W<PDS_CTL4_SPEC> {
        CR_PDS_MISC_MEM_STBY_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CR_PDS_MISC_GATE_CLK_W<PDS_CTL4_SPEC> {
        CR_PDS_MISC_GATE_CLK_W::new(self, 27)
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
#[doc = "PDS_CTL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_CTL4_SPEC;
impl crate::RegisterSpec for PDS_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl4::R`](R) reader structure"]
impl crate::Readable for PDS_CTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl4::W`](W) writer structure"]
impl crate::Writable for PDS_CTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL4 to value 0"]
impl crate::Resettable for PDS_CTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
