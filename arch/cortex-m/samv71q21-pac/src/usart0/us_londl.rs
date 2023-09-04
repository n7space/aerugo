#[doc = "Register `US_LONDL` reader"]
pub type R = crate::R<US_LONDL_SPEC>;
#[doc = "Register `US_LONDL` writer"]
pub type W = crate::W<US_LONDL_SPEC>;
#[doc = "Field `LONDL` reader - LON Data Length"]
pub type LONDL_R = crate::FieldReader;
#[doc = "Field `LONDL` writer - LON Data Length"]
pub type LONDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&self) -> LONDL_R {
        LONDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn londl(&mut self) -> LONDL_W<US_LONDL_SPEC, 0> {
        LONDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LON Data Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_londl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_londl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONDL_SPEC;
impl crate::RegisterSpec for US_LONDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_londl::R`](R) reader structure"]
impl crate::Readable for US_LONDL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_londl::W`](W) writer structure"]
impl crate::Writable for US_LONDL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONDL to value 0"]
impl crate::Resettable for US_LONDL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
