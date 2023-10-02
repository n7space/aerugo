#[doc = "Register `MDWE[%s]` reader"]
pub type R = crate::R<MDWE_SPEC>;
#[doc = "Register `MDWE[%s]` writer"]
pub type W = crate::W<MDWE_SPEC>;
#[doc = "Field `MASK` reader - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MASK_R = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<MDWE_SPEC, 0> {
        MASK_W::new(self)
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
#[doc = "MIF Data Write Enable 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdwe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdwe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDWE_SPEC;
impl crate::RegisterSpec for MDWE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdwe::R`](R) reader structure"]
impl crate::Readable for MDWE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdwe::W`](W) writer structure"]
impl crate::Writable for MDWE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDWE[%s]
to value 0"]
impl crate::Resettable for MDWE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
