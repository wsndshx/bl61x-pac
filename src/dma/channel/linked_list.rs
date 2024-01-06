#[doc = "Register `linked_list` reader"]
pub type R = crate::R<LINKED_LIST_SPEC>;
#[doc = "Register `linked_list` writer"]
pub type W = crate::W<LINKED_LIST_SPEC>;
#[doc = "Field `base_address` reader - Base address of first linked list item, must be aligned to 4 bytes"]
pub type BASE_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `base_address` writer - Base address of first linked list item, must be aligned to 4 bytes"]
pub type BASE_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of first linked list item, must be aligned to 4 bytes"]
    #[inline(always)]
    pub fn base_address(&self) -> BASE_ADDRESS_R {
        BASE_ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of first linked list item, must be aligned to 4 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn base_address(&mut self) -> BASE_ADDRESS_W<LINKED_LIST_SPEC> {
        BASE_ADDRESS_W::new(self, 0)
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
#[doc = "Linked list buffer base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linked_list::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linked_list::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINKED_LIST_SPEC;
impl crate::RegisterSpec for LINKED_LIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linked_list::R`](R) reader structure"]
impl crate::Readable for LINKED_LIST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linked_list::W`](W) writer structure"]
impl crate::Writable for LINKED_LIST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets linked_list to value 0"]
impl crate::Resettable for LINKED_LIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
