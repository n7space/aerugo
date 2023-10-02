#[doc = "Register `CMR` reader"]
pub type R = crate::R<CMR_SPEC>;
#[doc = "Register `CMR` writer"]
pub type W = crate::W<CMR_SPEC>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CMR_SPEC, 0> {
        DIV_W::new(self)
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
#[doc = "Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr::R`](R) reader structure"]
impl crate::Readable for CMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
