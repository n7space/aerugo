#[doc = "Register `US_LONPR` reader"]
pub type R = crate::R<US_LONPR_SPEC>;
#[doc = "Register `US_LONPR` writer"]
pub type W = crate::W<US_LONPR_SPEC>;
#[doc = "Field `LONPL` reader - LON Preamble Length"]
pub type LONPL_R = crate::FieldReader<u16>;
#[doc = "Field `LONPL` writer - LON Preamble Length"]
pub type LONPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&self) -> LONPL_R {
        LONPL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn lonpl(&mut self) -> LONPL_W<US_LONPR_SPEC, 0> {
        LONPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LON Preamble Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONPR_SPEC;
impl crate::RegisterSpec for US_LONPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonpr::R`](R) reader structure"]
impl crate::Readable for US_LONPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonpr::W`](W) writer structure"]
impl crate::Writable for US_LONPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONPR to value 0"]
impl crate::Resettable for US_LONPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
