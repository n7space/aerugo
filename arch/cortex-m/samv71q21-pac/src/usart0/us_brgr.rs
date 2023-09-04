#[doc = "Register `US_BRGR` reader"]
pub type R = crate::R<US_BRGR_SPEC>;
#[doc = "Register `US_BRGR` writer"]
pub type W = crate::W<US_BRGR_SPEC>;
#[doc = "Field `CD` reader - Clock Divider"]
pub type CD_R = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divider"]
pub type CD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<US_BRGR_SPEC, 0> {
        CD_W::new(self)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<US_BRGR_SPEC, 16> {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_brgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_brgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_BRGR_SPEC;
impl crate::RegisterSpec for US_BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_brgr::R`](R) reader structure"]
impl crate::Readable for US_BRGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_brgr::W`](W) writer structure"]
impl crate::Writable for US_BRGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_BRGR to value 0"]
impl crate::Resettable for US_BRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
