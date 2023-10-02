#[doc = "Register `CVR` reader"]
pub type R = crate::R<CVR_SPEC>;
#[doc = "Register `CVR` writer"]
pub type W = crate::W<CVR_SPEC>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OFFSETCORR_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub type OFFSETCORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GAINCORR_R = crate::FieldReader<u16>;
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub type GAINCORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    #[must_use]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W<CVR_SPEC, 0> {
        OFFSETCORR_W::new(self)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    #[must_use]
    pub fn gaincorr(&mut self) -> GAINCORR_W<CVR_SPEC, 16> {
        GAINCORR_W::new(self)
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
#[doc = "AFEC Correction Values Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CVR_SPEC;
impl crate::RegisterSpec for CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvr::R`](R) reader structure"]
impl crate::Readable for CVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cvr::W`](W) writer structure"]
impl crate::Writable for CVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
