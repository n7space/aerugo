#[doc = "Register `CLENR` reader"]
pub type R = crate::R<CLENR_SPEC>;
#[doc = "Register `CLENR` writer"]
pub type W = crate::W<CLENR_SPEC>;
#[doc = "Field `CLEN` reader - Plaintext/Ciphertext Length"]
pub type CLEN_R = crate::FieldReader<u32>;
#[doc = "Field `CLEN` writer - Plaintext/Ciphertext Length"]
pub type CLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<CLENR_SPEC, 0> {
        CLEN_W::new(self)
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
#[doc = "Plaintext/Ciphertext Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLENR_SPEC;
impl crate::RegisterSpec for CLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clenr::R`](R) reader structure"]
impl crate::Readable for CLENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clenr::W`](W) writer structure"]
impl crate::Writable for CLENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLENR to value 0"]
impl crate::Resettable for CLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
