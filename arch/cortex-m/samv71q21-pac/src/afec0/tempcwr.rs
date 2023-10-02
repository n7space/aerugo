#[doc = "Register `TEMPCWR` reader"]
pub type R = crate::R<TEMPCWR_SPEC>;
#[doc = "Register `TEMPCWR` writer"]
pub type W = crate::W<TEMPCWR_SPEC>;
#[doc = "Field `TLOWTHRES` reader - Temperature Low Threshold"]
pub type TLOWTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `TLOWTHRES` writer - Temperature Low Threshold"]
pub type TLOWTHRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `THIGHTHRES` reader - Temperature High Threshold"]
pub type THIGHTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `THIGHTHRES` writer - Temperature High Threshold"]
pub type THIGHTHRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TLOWTHRES_R {
        TLOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> THIGHTHRES_R {
        THIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tlowthres(&mut self) -> TLOWTHRES_W<TEMPCWR_SPEC, 0> {
        TLOWTHRES_W::new(self)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thighthres(&mut self) -> THIGHTHRES_W<TEMPCWR_SPEC, 16> {
        THIGHTHRES_W::new(self)
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
#[doc = "AFEC Temperature Compare Window Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempcwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempcwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMPCWR_SPEC;
impl crate::RegisterSpec for TEMPCWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempcwr::R`](R) reader structure"]
impl crate::Readable for TEMPCWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tempcwr::W`](W) writer structure"]
impl crate::Writable for TEMPCWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPCWR to value 0"]
impl crate::Resettable for TEMPCWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
