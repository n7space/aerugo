#[doc = "Register `CCFG_DYNCKG` reader"]
pub type R = crate::R<CCFG_DYNCKG_SPEC>;
#[doc = "Register `CCFG_DYNCKG` writer"]
pub type W = crate::W<CCFG_DYNCKG_SPEC>;
#[doc = "Field `MATCKG` reader - MATRIX Dynamic Clock Gating"]
pub type MATCKG_R = crate::BitReader;
#[doc = "Field `MATCKG` writer - MATRIX Dynamic Clock Gating"]
pub type MATCKG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRIDCKG` reader - Bridge Dynamic Clock Gating Enable"]
pub type BRIDCKG_R = crate::BitReader;
#[doc = "Field `BRIDCKG` writer - Bridge Dynamic Clock Gating Enable"]
pub type BRIDCKG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFCCKG` reader - EFC Dynamic Clock Gating Enable"]
pub type EFCCKG_R = crate::BitReader;
#[doc = "Field `EFCCKG` writer - EFC Dynamic Clock Gating Enable"]
pub type EFCCKG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&self) -> MATCKG_R {
        MATCKG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&self) -> BRIDCKG_R {
        BRIDCKG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&self) -> EFCCKG_R {
        EFCCKG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn matckg(&mut self) -> MATCKG_W<CCFG_DYNCKG_SPEC, 0> {
        MATCKG_W::new(self)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bridckg(&mut self) -> BRIDCKG_W<CCFG_DYNCKG_SPEC, 1> {
        BRIDCKG_W::new(self)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn efcckg(&mut self) -> EFCCKG_W<CCFG_DYNCKG_SPEC, 2> {
        EFCCKG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Dynamic Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_dynckg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_dynckg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_DYNCKG_SPEC;
impl crate::RegisterSpec for CCFG_DYNCKG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_dynckg::R`](R) reader structure"]
impl crate::Readable for CCFG_DYNCKG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_dynckg::W`](W) writer structure"]
impl crate::Writable for CCFG_DYNCKG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_DYNCKG to value 0"]
impl crate::Resettable for CCFG_DYNCKG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
