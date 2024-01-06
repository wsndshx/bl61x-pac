#[doc = "Register `config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `function` reader - Enable or disable DMA channel"]
pub type FUNCTION_R = crate::BitReader;
#[doc = "Field `function` writer - Enable or disable DMA channel"]
pub type FUNCTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `source_peripheral` reader - Set source peripheral for this DMA channel"]
pub type SOURCE_PERIPHERAL_R = crate::FieldReader;
#[doc = "Field `source_peripheral` writer - Set source peripheral for this DMA channel"]
pub type SOURCE_PERIPHERAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `destination_peripheral` reader - Set destination peripheral for this DMA channel"]
pub type DESTINATION_PERIPHERAL_R = crate::FieldReader;
#[doc = "Field `destination_peripheral` writer - Set destination peripheral for this DMA channel"]
pub type DESTINATION_PERIPHERAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `flow_control` reader - Set data direction for this channel"]
pub type FLOW_CONTROL_R = crate::FieldReader;
#[doc = "Field `flow_control` writer - Set data direction for this channel"]
pub type FLOW_CONTROL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `error_mask` reader - Mask error interrupt"]
pub type ERROR_MASK_R = crate::BitReader;
#[doc = "Field `error_mask` writer - Mask error interrupt"]
pub type ERROR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `terminate_mask` reader - Mask terminal count interrupt"]
pub type TERMINATE_MASK_R = crate::BitReader;
#[doc = "Field `terminate_mask` writer - Mask terminal count interrupt"]
pub type TERMINATE_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `active` reader - ??"]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `lock` reader - ??"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `lock` writer - ??"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `halt` reader - ??"]
pub type HALT_R = crate::BitReader;
#[doc = "Field `halt` writer - ??"]
pub type HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `linked_list_counter` reader - ??"]
pub type LINKED_LIST_COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `linked_list_counter` writer - ??"]
pub type LINKED_LIST_COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Enable or disable DMA channel"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Set source peripheral for this DMA channel"]
    #[inline(always)]
    pub fn source_peripheral(&self) -> SOURCE_PERIPHERAL_R {
        SOURCE_PERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - Set destination peripheral for this DMA channel"]
    #[inline(always)]
    pub fn destination_peripheral(&self) -> DESTINATION_PERIPHERAL_R {
        DESTINATION_PERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - Set data direction for this channel"]
    #[inline(always)]
    pub fn flow_control(&self) -> FLOW_CONTROL_R {
        FLOW_CONTROL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Mask error interrupt"]
    #[inline(always)]
    pub fn error_mask(&self) -> ERROR_MASK_R {
        ERROR_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask terminal count interrupt"]
    #[inline(always)]
    pub fn terminate_mask(&self) -> TERMINATE_MASK_R {
        TERMINATE_MASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ??"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:29 - ??"]
    #[inline(always)]
    pub fn linked_list_counter(&self) -> LINKED_LIST_COUNTER_R {
        LINKED_LIST_COUNTER_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<CONFIG_SPEC> {
        FUNCTION_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - Set source peripheral for this DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn source_peripheral(&mut self) -> SOURCE_PERIPHERAL_W<CONFIG_SPEC> {
        SOURCE_PERIPHERAL_W::new(self, 1)
    }
    #[doc = "Bits 6:10 - Set destination peripheral for this DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn destination_peripheral(&mut self) -> DESTINATION_PERIPHERAL_W<CONFIG_SPEC> {
        DESTINATION_PERIPHERAL_W::new(self, 6)
    }
    #[doc = "Bits 11:13 - Set data direction for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn flow_control(&mut self) -> FLOW_CONTROL_W<CONFIG_SPEC> {
        FLOW_CONTROL_W::new(self, 11)
    }
    #[doc = "Bit 14 - Mask error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error_mask(&mut self) -> ERROR_MASK_W<CONFIG_SPEC> {
        ERROR_MASK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Mask terminal count interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn terminate_mask(&mut self) -> TERMINATE_MASK_W<CONFIG_SPEC> {
        TERMINATE_MASK_W::new(self, 15)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CONFIG_SPEC> {
        LOCK_W::new(self, 16)
    }
    #[doc = "Bit 17 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<CONFIG_SPEC> {
        HALT_W::new(self, 17)
    }
    #[doc = "Bits 20:29 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn linked_list_counter(&mut self) -> LINKED_LIST_COUNTER_W<CONFIG_SPEC> {
        LINKED_LIST_COUNTER_W::new(self, 20)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
