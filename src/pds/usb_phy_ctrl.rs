#[doc = "Register `usb_phy_ctrl` reader"]
pub type R = crate::R<USB_PHY_CTRL_SPEC>;
#[doc = "Register `usb_phy_ctrl` writer"]
pub type W = crate::W<USB_PHY_CTRL_SPEC>;
#[doc = "Field `reg_usb_phy_ponrst` reader - "]
pub type REG_USB_PHY_PONRST_R = crate::BitReader;
#[doc = "Field `reg_usb_phy_ponrst` writer - "]
pub type REG_USB_PHY_PONRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_oscouten` reader - "]
pub type REG_USB_PHY_OSCOUTEN_R = crate::BitReader;
#[doc = "Field `reg_usb_phy_oscouten` writer - "]
pub type REG_USB_PHY_OSCOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_xtlsel` reader - "]
pub type REG_USB_PHY_XTLSEL_R = crate::FieldReader;
#[doc = "Field `reg_usb_phy_xtlsel` writer - "]
pub type REG_USB_PHY_XTLSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_usb_phy_outclksel` reader - "]
pub type REG_USB_PHY_OUTCLKSEL_R = crate::BitReader;
#[doc = "Field `reg_usb_phy_outclksel` writer - "]
pub type REG_USB_PHY_OUTCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_pllaliv` reader - "]
pub type REG_USB_PHY_PLLALIV_R = crate::BitReader;
#[doc = "Field `reg_usb_phy_pllaliv` writer - "]
pub type REG_USB_PHY_PLLALIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pu_usb20_psw` reader - "]
pub type REG_PU_USB20_PSW_R = crate::BitReader;
#[doc = "Field `reg_pu_usb20_psw` writer - "]
pub type REG_PU_USB20_PSW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_phy_ponrst(&self) -> REG_USB_PHY_PONRST_R {
        REG_USB_PHY_PONRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_phy_oscouten(&self) -> REG_USB_PHY_OSCOUTEN_R {
        REG_USB_PHY_OSCOUTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_usb_phy_xtlsel(&self) -> REG_USB_PHY_XTLSEL_R {
        REG_USB_PHY_XTLSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_phy_outclksel(&self) -> REG_USB_PHY_OUTCLKSEL_R {
        REG_USB_PHY_OUTCLKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_phy_pllaliv(&self) -> REG_USB_PHY_PLLALIV_R {
        REG_USB_PHY_PLLALIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_pu_usb20_psw(&self) -> REG_PU_USB20_PSW_R {
        REG_PU_USB20_PSW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_ponrst(&mut self) -> REG_USB_PHY_PONRST_W<USB_PHY_CTRL_SPEC> {
        REG_USB_PHY_PONRST_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_oscouten(&mut self) -> REG_USB_PHY_OSCOUTEN_W<USB_PHY_CTRL_SPEC> {
        REG_USB_PHY_OSCOUTEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_xtlsel(&mut self) -> REG_USB_PHY_XTLSEL_W<USB_PHY_CTRL_SPEC> {
        REG_USB_PHY_XTLSEL_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_outclksel(&mut self) -> REG_USB_PHY_OUTCLKSEL_W<USB_PHY_CTRL_SPEC> {
        REG_USB_PHY_OUTCLKSEL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_pllaliv(&mut self) -> REG_USB_PHY_PLLALIV_W<USB_PHY_CTRL_SPEC> {
        REG_USB_PHY_PLLALIV_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pu_usb20_psw(&mut self) -> REG_PU_USB20_PSW_W<USB_PHY_CTRL_SPEC> {
        REG_PU_USB20_PSW_W::new(self, 6)
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
#[doc = "usb_phy_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_phy_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_phy_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_PHY_CTRL_SPEC;
impl crate::RegisterSpec for USB_PHY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for USB_PHY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for USB_PHY_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_phy_ctrl to value 0"]
impl crate::Resettable for USB_PHY_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
