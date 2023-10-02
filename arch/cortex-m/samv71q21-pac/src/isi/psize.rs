#[doc = "Register `PSIZE` reader"]
pub type R = crate::R<PSIZE_SPEC>;
#[doc = "Register `PSIZE` writer"]
pub type W = crate::W<PSIZE_SPEC>;
#[doc = "Field `PREV_VSIZE` reader - Vertical Size for the Preview Path"]
pub type PREV_VSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PREV_VSIZE` writer - Vertical Size for the Preview Path"]
pub type PREV_VSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `PREV_HSIZE` reader - Horizontal Size for the Preview Path"]
pub type PREV_HSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PREV_HSIZE` writer - Horizontal Size for the Preview Path"]
pub type PREV_HSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PREV_VSIZE_R {
        PREV_VSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PREV_HSIZE_R {
        PREV_HSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    #[must_use]
    pub fn prev_vsize(&mut self) -> PREV_VSIZE_W<PSIZE_SPEC, 0> {
        PREV_VSIZE_W::new(self)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    #[must_use]
    pub fn prev_hsize(&mut self) -> PREV_HSIZE_W<PSIZE_SPEC, 16> {
        PREV_HSIZE_W::new(self)
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
#[doc = "ISI Preview Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSIZE_SPEC;
impl crate::RegisterSpec for PSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psize::R`](R) reader structure"]
impl crate::Readable for PSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psize::W`](W) writer structure"]
impl crate::Writable for PSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSIZE to value 0"]
impl crate::Resettable for PSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
