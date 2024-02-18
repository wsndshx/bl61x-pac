#[doc = "Register `gpio_output_1` reader"]
pub type R = crate::R<GPIO_OUTPUT_1_SPEC>;
#[doc = "Register `gpio_output_1` writer"]
pub type W = crate::W<GPIO_OUTPUT_1_SPEC>;
#[doc = "Field `gpio_32_o` reader - "]
pub type GPIO_32_O_R = crate::BitReader;
#[doc = "Field `gpio_32_o` writer - "]
pub type GPIO_32_O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_33_o` reader - "]
pub type GPIO_33_O_R = crate::BitReader;
#[doc = "Field `gpio_33_o` writer - "]
pub type GPIO_33_O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_34_o` reader - "]
pub type GPIO_34_O_R = crate::BitReader;
#[doc = "Field `gpio_34_o` writer - "]
pub type GPIO_34_O_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_32_o(&self) -> GPIO_32_O_R {
        GPIO_32_O_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_33_o(&self) -> GPIO_33_O_R {
        GPIO_33_O_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_34_o(&self) -> GPIO_34_O_R {
        GPIO_34_O_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_32_o(&mut self) -> GPIO_32_O_W<GPIO_OUTPUT_1_SPEC> {
        GPIO_32_O_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_33_o(&mut self) -> GPIO_33_O_W<GPIO_OUTPUT_1_SPEC> {
        GPIO_33_O_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_34_o(&mut self) -> GPIO_34_O_W<GPIO_OUTPUT_1_SPEC> {
        GPIO_34_O_W::new(self, 2)
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
#[doc = "Write value to Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_output_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_output_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUTPUT_1_SPEC;
impl crate::RegisterSpec for GPIO_OUTPUT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_output_1::R`](R) reader structure"]
impl crate::Readable for GPIO_OUTPUT_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_output_1::W`](W) writer structure"]
impl crate::Writable for GPIO_OUTPUT_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_output_1 to value 0"]
impl crate::Resettable for GPIO_OUTPUT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
