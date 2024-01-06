#[doc = "Register `ir_config_0` reader"]
pub type R = crate::R<IR_CONFIG_0_SPEC>;
#[doc = "Register `ir_config_0` writer"]
pub type W = crate::W<IR_CONFIG_0_SPEC>;
#[doc = "Field `ir_clk_div` reader - "]
pub type IR_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `ir_clk_div` writer - "]
pub type IR_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ir_clk_en` reader - "]
pub type IR_CLK_EN_R = crate::BitReader;
#[doc = "Field `ir_clk_en` writer - "]
pub type IR_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IR_CLK_DIV_R {
        IR_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IR_CLK_EN_R {
        IR_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn ir_clk_div(&mut self) -> IR_CLK_DIV_W<IR_CONFIG_0_SPEC> {
        IR_CLK_DIV_W::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ir_clk_en(&mut self) -> IR_CLK_EN_W<IR_CONFIG_0_SPEC> {
        IR_CLK_EN_W::new(self, 23)
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
#[doc = "Infrared configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_CONFIG_0_SPEC;
impl crate::RegisterSpec for IR_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_config_0::R`](R) reader structure"]
impl crate::Readable for IR_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir_config_0::W`](W) writer structure"]
impl crate::Writable for IR_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ir_config_0 to value 0"]
impl crate::Resettable for IR_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
