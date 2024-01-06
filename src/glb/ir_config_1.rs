#[doc = "Register `ir_config_1` reader"]
pub type R = crate::R<IR_CONFIG_1_SPEC>;
#[doc = "Register `ir_config_1` writer"]
pub type W = crate::W<IR_CONFIG_1_SPEC>;
#[doc = "Field `ir_rx_gpio_sel` reader - "]
pub type IR_RX_GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `ir_rx_gpio_sel` writer - "]
pub type IR_RX_GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&self) -> IR_RX_GPIO_SEL_R {
        IR_RX_GPIO_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn ir_rx_gpio_sel(&mut self) -> IR_RX_GPIO_SEL_W<IR_CONFIG_1_SPEC> {
        IR_RX_GPIO_SEL_W::new(self, 8)
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
#[doc = "Infrared configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_CONFIG_1_SPEC;
impl crate::RegisterSpec for IR_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_config_1::R`](R) reader structure"]
impl crate::Readable for IR_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir_config_1::W`](W) writer structure"]
impl crate::Writable for IR_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ir_config_1 to value 0"]
impl crate::Resettable for IR_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
