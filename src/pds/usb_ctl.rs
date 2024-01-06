#[doc = "Register `usb_ctl` reader"]
pub type R = crate::R<USB_CTL_SPEC>;
#[doc = "Register `usb_ctl` writer"]
pub type W = crate::W<USB_CTL_SPEC>;
#[doc = "Field `reg_usb_sw_rst_n` reader - "]
pub type REG_USB_SW_RST_N_R = crate::BitReader;
#[doc = "Field `reg_usb_sw_rst_n` writer - "]
pub type REG_USB_SW_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_ext_susp_n` reader - "]
pub type REG_USB_EXT_SUSP_N_R = crate::BitReader;
#[doc = "Field `reg_usb_ext_susp_n` writer - "]
pub type REG_USB_EXT_SUSP_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_wakeup` reader - "]
pub type REG_USB_WAKEUP_R = crate::BitReader;
#[doc = "Field `reg_usb_wakeup` writer - "]
pub type REG_USB_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_l1_wakeup` reader - "]
pub type REG_USB_L1_WAKEUP_R = crate::BitReader;
#[doc = "Field `reg_usb_l1_wakeup` writer - "]
pub type REG_USB_L1_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_drvbus_pol` reader - "]
pub type REG_USB_DRVBUS_POL_R = crate::BitReader;
#[doc = "Field `reg_usb_drvbus_pol` writer - "]
pub type REG_USB_DRVBUS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_iddig` reader - "]
pub type REG_USB_IDDIG_R = crate::BitReader;
#[doc = "Field `reg_usb_iddig` writer - "]
pub type REG_USB_IDDIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_sw_rst_n(&self) -> REG_USB_SW_RST_N_R {
        REG_USB_SW_RST_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_ext_susp_n(&self) -> REG_USB_EXT_SUSP_N_R {
        REG_USB_EXT_SUSP_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_usb_wakeup(&self) -> REG_USB_WAKEUP_R {
        REG_USB_WAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_usb_l1_wakeup(&self) -> REG_USB_L1_WAKEUP_R {
        REG_USB_L1_WAKEUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_drvbus_pol(&self) -> REG_USB_DRVBUS_POL_R {
        REG_USB_DRVBUS_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_iddig(&self) -> REG_USB_IDDIG_R {
        REG_USB_IDDIG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_sw_rst_n(&mut self) -> REG_USB_SW_RST_N_W<USB_CTL_SPEC> {
        REG_USB_SW_RST_N_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_ext_susp_n(&mut self) -> REG_USB_EXT_SUSP_N_W<USB_CTL_SPEC> {
        REG_USB_EXT_SUSP_N_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_wakeup(&mut self) -> REG_USB_WAKEUP_W<USB_CTL_SPEC> {
        REG_USB_WAKEUP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_l1_wakeup(&mut self) -> REG_USB_L1_WAKEUP_W<USB_CTL_SPEC> {
        REG_USB_L1_WAKEUP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_drvbus_pol(&mut self) -> REG_USB_DRVBUS_POL_W<USB_CTL_SPEC> {
        REG_USB_DRVBUS_POL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_iddig(&mut self) -> REG_USB_IDDIG_W<USB_CTL_SPEC> {
        REG_USB_IDDIG_W::new(self, 5)
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
#[doc = "usb_ctl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CTL_SPEC;
impl crate::RegisterSpec for USB_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_ctl::R`](R) reader structure"]
impl crate::Readable for USB_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_ctl::W`](W) writer structure"]
impl crate::Writable for USB_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_ctl to value 0"]
impl crate::Resettable for USB_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
