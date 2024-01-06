#[doc = "Register `HBN_PAD_CTRL_0` reader"]
pub type R = crate::R<HBN_PAD_CTRL_0_SPEC>;
#[doc = "Register `HBN_PAD_CTRL_0` writer"]
pub type W = crate::W<HBN_PAD_CTRL_0_SPEC>;
#[doc = "Field `reg_aon_pad_ie_smt` reader - "]
pub type REG_AON_PAD_IE_SMT_R = crate::FieldReader;
#[doc = "Field `reg_aon_pad_ie_smt` writer - "]
pub type REG_AON_PAD_IE_SMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_aon_led_sel` reader - "]
pub type REG_AON_LED_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_led_sel` writer - "]
pub type REG_AON_LED_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_en_aon_ctrl_gpio` reader - "]
pub type REG_EN_AON_CTRL_GPIO_R = crate::FieldReader;
#[doc = "Field `reg_en_aon_ctrl_gpio` writer - "]
pub type REG_EN_AON_CTRL_GPIO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_gpio_keep_en` reader - "]
pub type CR_GPIO_KEEP_EN_R = crate::FieldReader;
#[doc = "Field `cr_gpio_keep_en` writer - "]
pub type CR_GPIO_KEEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_aon_gpio_iso_mode` reader - "]
pub type REG_AON_GPIO_ISO_MODE_R = crate::BitReader;
#[doc = "Field `reg_aon_gpio_iso_mode` writer - "]
pub type REG_AON_GPIO_ISO_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> REG_AON_PAD_IE_SMT_R {
        REG_AON_PAD_IE_SMT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_led_sel(&self) -> REG_AON_LED_SEL_R {
        REG_AON_LED_SEL_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_en_aon_ctrl_gpio(&self) -> REG_EN_AON_CTRL_GPIO_R {
        REG_EN_AON_CTRL_GPIO_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn cr_gpio_keep_en(&self) -> CR_GPIO_KEEP_EN_R {
        CR_GPIO_KEEP_EN_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_aon_gpio_iso_mode(&self) -> REG_AON_GPIO_ISO_MODE_R {
        REG_AON_GPIO_ISO_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_ie_smt(&mut self) -> REG_AON_PAD_IE_SMT_W<HBN_PAD_CTRL_0_SPEC> {
        REG_AON_PAD_IE_SMT_W::new(self, 0)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_led_sel(&mut self) -> REG_AON_LED_SEL_W<HBN_PAD_CTRL_0_SPEC> {
        REG_AON_LED_SEL_W::new(self, 10)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_aon_ctrl_gpio(&mut self) -> REG_EN_AON_CTRL_GPIO_W<HBN_PAD_CTRL_0_SPEC> {
        REG_EN_AON_CTRL_GPIO_W::new(self, 20)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_gpio_keep_en(&mut self) -> CR_GPIO_KEEP_EN_W<HBN_PAD_CTRL_0_SPEC> {
        CR_GPIO_KEEP_EN_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_gpio_iso_mode(&mut self) -> REG_AON_GPIO_ISO_MODE_W<HBN_PAD_CTRL_0_SPEC> {
        REG_AON_GPIO_ISO_MODE_W::new(self, 31)
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
#[doc = "HBN_PAD_CTRL_0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pad_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pad_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_PAD_CTRL_0_SPEC;
impl crate::RegisterSpec for HBN_PAD_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pad_ctrl_0::R`](R) reader structure"]
impl crate::Readable for HBN_PAD_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_pad_ctrl_0::W`](W) writer structure"]
impl crate::Writable for HBN_PAD_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PAD_CTRL_0 to value 0"]
impl crate::Resettable for HBN_PAD_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
