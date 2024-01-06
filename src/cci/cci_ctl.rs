#[doc = "Register `cci_ctl` reader"]
pub type R = crate::R<CCI_CTL_SPEC>;
#[doc = "Register `cci_ctl` writer"]
pub type W = crate::W<CCI_CTL_SPEC>;
#[doc = "Field `cci_write_flag` reader - "]
pub type CCI_WRITE_FLAG_R = crate::BitReader;
#[doc = "Field `cci_write_flag` writer - "]
pub type CCI_WRITE_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_read_flag` reader - "]
pub type CCI_READ_FLAG_R = crate::BitReader;
#[doc = "Field `cci_read_flag` writer - "]
pub type CCI_READ_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ahb_state` reader - "]
pub type AHB_STATE_R = crate::FieldReader;
#[doc = "Field `ahb_state` writer - "]
pub type AHB_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CCI_WRITE_FLAG_R {
        CCI_WRITE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CCI_READ_FLAG_R {
        CCI_READ_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AHB_STATE_R {
        AHB_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cci_write_flag(&mut self) -> CCI_WRITE_FLAG_W<CCI_CTL_SPEC> {
        CCI_WRITE_FLAG_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cci_read_flag(&mut self) -> CCI_READ_FLAG_W<CCI_CTL_SPEC> {
        CCI_READ_FLAG_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_state(&mut self) -> AHB_STATE_W<CCI_CTL_SPEC> {
        AHB_STATE_W::new(self, 2)
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
#[doc = "cci_ctl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCI_CTL_SPEC;
impl crate::RegisterSpec for CCI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_ctl::R`](R) reader structure"]
impl crate::Readable for CCI_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cci_ctl::W`](W) writer structure"]
impl crate::Writable for CCI_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_ctl to value 0"]
impl crate::Resettable for CCI_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
