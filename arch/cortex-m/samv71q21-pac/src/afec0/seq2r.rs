#[doc = "Register `SEQ2R` reader"]
pub type R = crate::R<SEQ2R_SPEC>;
#[doc = "Register `SEQ2R` writer"]
pub type W = crate::W<SEQ2R_SPEC>;
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub type USCH8_R = crate::FieldReader;
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub type USCH8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub type USCH9_R = crate::FieldReader;
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub type USCH9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub type USCH10_R = crate::FieldReader;
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub type USCH10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub type USCH11_R = crate::FieldReader;
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub type USCH11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    #[must_use]
    pub fn usch8(&mut self) -> USCH8_W<SEQ2R_SPEC, 0> {
        USCH8_W::new(self)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    #[must_use]
    pub fn usch9(&mut self) -> USCH9_W<SEQ2R_SPEC, 4> {
        USCH9_W::new(self)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    #[must_use]
    pub fn usch10(&mut self) -> USCH10_W<SEQ2R_SPEC, 8> {
        USCH10_W::new(self)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    #[must_use]
    pub fn usch11(&mut self) -> USCH11_W<SEQ2R_SPEC, 12> {
        USCH11_W::new(self)
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
#[doc = "AFEC Channel Sequence 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ2R_SPEC;
impl crate::RegisterSpec for SEQ2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq2r::R`](R) reader structure"]
impl crate::Readable for SEQ2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq2r::W`](W) writer structure"]
impl crate::Writable for SEQ2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ2R to value 0"]
impl crate::Resettable for SEQ2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
