#[doc = "Register `CWR` reader"]
pub type R = crate::R<CWR_SPEC>;
#[doc = "Register `CWR` writer"]
pub type W = crate::W<CWR_SPEC>;
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub type LOWTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub type LOWTHRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub type HIGHTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub type HIGHTHRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LOWTHRES_R {
        LOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HIGHTHRES_R {
        HIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lowthres(&mut self) -> LOWTHRES_W<CWR_SPEC, 0> {
        LOWTHRES_W::new(self)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn highthres(&mut self) -> HIGHTHRES_W<CWR_SPEC, 16> {
        HIGHTHRES_W::new(self)
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
#[doc = "AFEC Compare Window Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWR_SPEC;
impl crate::RegisterSpec for CWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwr::R`](R) reader structure"]
impl crate::Readable for CWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwr::W`](W) writer structure"]
impl crate::Writable for CWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWR to value 0"]
impl crate::Resettable for CWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
