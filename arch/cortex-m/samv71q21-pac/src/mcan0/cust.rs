#[doc = "Register `CUST` reader"]
pub type R = crate::R<CUST_SPEC>;
#[doc = "Register `CUST` writer"]
pub type W = crate::W<CUST_SPEC>;
#[doc = "Field `CSV` reader - Customer-specific Value"]
pub type CSV_R = crate::FieldReader<u32>;
#[doc = "Field `CSV` writer - Customer-specific Value"]
pub type CSV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    pub fn csv(&self) -> CSV_R {
        CSV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    #[must_use]
    pub fn csv(&mut self) -> CSV_W<CUST_SPEC, 0> {
        CSV_W::new(self)
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
#[doc = "Customer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cust::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cust::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CUST_SPEC;
impl crate::RegisterSpec for CUST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cust::R`](R) reader structure"]
impl crate::Readable for CUST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cust::W`](W) writer structure"]
impl crate::Writable for CUST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUST to value 0"]
impl crate::Resettable for CUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
