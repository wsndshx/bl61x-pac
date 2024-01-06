#[doc = "Register `pds_ram1` reader"]
pub type R = crate::R<PDS_RAM1_SPEC>;
#[doc = "Register `pds_ram1` writer"]
pub type W = crate::W<PDS_RAM1_SPEC>;
#[doc = "Field `cr_pds_ram_clk_cnt` reader - "]
pub type CR_PDS_RAM_CLK_CNT_R = crate::FieldReader;
#[doc = "Field `cr_pds_ram_clk_cnt` writer - "]
pub type CR_PDS_RAM_CLK_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_pds_ram_clk2_cnt` reader - "]
pub type CR_PDS_RAM_CLK2_CNT_R = crate::FieldReader;
#[doc = "Field `cr_pds_ram_clk2_cnt` writer - "]
pub type CR_PDS_RAM_CLK2_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` reader - "]
pub type CR_PDS_CTRL_NP_RAM_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` writer - "]
pub type CR_PDS_CTRL_NP_RAM_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` reader - "]
pub type CR_PDS_CTRL_WB_RAM_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` writer - "]
pub type CR_PDS_CTRL_WB_RAM_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usb_ram_clk` reader - "]
pub type CR_PDS_CTRL_USB_RAM_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usb_ram_clk` writer - "]
pub type CR_PDS_CTRL_USB_RAM_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_misc_ram_clk` reader - "]
pub type CR_PDS_CTRL_MISC_RAM_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_misc_ram_clk` writer - "]
pub type CR_PDS_CTRL_MISC_RAM_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_ram_clk2` reader - "]
pub type CR_PDS_CTRL_RAM_CLK2_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_ram_clk2` writer - "]
pub type CR_PDS_CTRL_RAM_CLK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_ram_clk` reader - "]
pub type CR_PDS_CTRL_RAM_CLK_R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_ram_clk` writer - "]
pub type CR_PDS_CTRL_RAM_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_ram_clk_cnt(&self) -> CR_PDS_RAM_CLK_CNT_R {
        CR_PDS_RAM_CLK_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_ram_clk2_cnt(&self) -> CR_PDS_RAM_CLK2_CNT_R {
        CR_PDS_RAM_CLK2_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_ctrl_np_ram_clk(&self) -> CR_PDS_CTRL_NP_RAM_CLK_R {
        CR_PDS_CTRL_NP_RAM_CLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wb_ram_clk(&self) -> CR_PDS_CTRL_WB_RAM_CLK_R {
        CR_PDS_CTRL_WB_RAM_CLK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb_ram_clk(&self) -> CR_PDS_CTRL_USB_RAM_CLK_R {
        CR_PDS_CTRL_USB_RAM_CLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_pds_ctrl_misc_ram_clk(&self) -> CR_PDS_CTRL_MISC_RAM_CLK_R {
        CR_PDS_CTRL_MISC_RAM_CLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk2(&self) -> CR_PDS_CTRL_RAM_CLK2_R {
        CR_PDS_CTRL_RAM_CLK2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk(&self) -> CR_PDS_CTRL_RAM_CLK_R {
        CR_PDS_CTRL_RAM_CLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ram_clk_cnt(&mut self) -> CR_PDS_RAM_CLK_CNT_W<PDS_RAM1_SPEC> {
        CR_PDS_RAM_CLK_CNT_W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ram_clk2_cnt(&mut self) -> CR_PDS_RAM_CLK2_CNT_W<PDS_RAM1_SPEC> {
        CR_PDS_RAM_CLK2_CNT_W::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_np_ram_clk(&mut self) -> CR_PDS_CTRL_NP_RAM_CLK_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_NP_RAM_CLK_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_wb_ram_clk(&mut self) -> CR_PDS_CTRL_WB_RAM_CLK_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_WB_RAM_CLK_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_usb_ram_clk(&mut self) -> CR_PDS_CTRL_USB_RAM_CLK_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_USB_RAM_CLK_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_misc_ram_clk(&mut self) -> CR_PDS_CTRL_MISC_RAM_CLK_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_MISC_RAM_CLK_W::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_ram_clk2(&mut self) -> CR_PDS_CTRL_RAM_CLK2_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_RAM_CLK2_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_ram_clk(&mut self) -> CR_PDS_CTRL_RAM_CLK_W<PDS_RAM1_SPEC> {
        CR_PDS_CTRL_RAM_CLK_W::new(self, 31)
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
#[doc = "pds_ram1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_RAM1_SPEC;
impl crate::RegisterSpec for PDS_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ram1::R`](R) reader structure"]
impl crate::Readable for PDS_RAM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ram1::W`](W) writer structure"]
impl crate::Writable for PDS_RAM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_ram1 to value 0"]
impl crate::Resettable for PDS_RAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
