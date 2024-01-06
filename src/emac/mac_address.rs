#[doc = "Register `mac_address[%s]` reader"]
pub type R = crate::R<MAC_ADDRESS_SPEC>;
#[doc = "Register `mac_address[%s]` writer"]
pub type W = crate::W<MAC_ADDRESS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MAC_ADDRESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "Media Access Control address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDRESS_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address::R`](R) reader structure"]
impl crate::Readable for MAC_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_address::W`](W) writer structure"]
impl crate::Writable for MAC_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mac_address[%s]
to value 0"]
impl crate::Resettable for MAC_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
