#[doc = "Register `CNDA` reader"]
pub type R = crate::R<CNDA_SPEC>;
#[doc = "Register `CNDA` writer"]
pub type W = crate::W<CNDA_SPEC>;
#[doc = "Field `NDAIF` reader - Channel x Next Descriptor Interface"]
pub type NDAIF_R = crate::BitReader;
#[doc = "Field `NDAIF` writer - Channel x Next Descriptor Interface"]
pub type NDAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDA` reader - Channel x Next Descriptor Address"]
pub type NDA_R = crate::FieldReader<u32>;
#[doc = "Field `NDA` writer - Channel x Next Descriptor Address"]
pub type NDA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&self) -> NDAIF_R {
        NDAIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    #[must_use]
    pub fn ndaif(&mut self) -> NDAIF_W<CNDA_SPEC, 0> {
        NDAIF_W::new(self)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn nda(&mut self) -> NDA_W<CNDA_SPEC, 2> {
        NDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Next Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNDA_SPEC;
impl crate::RegisterSpec for CNDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnda::R`](R) reader structure"]
impl crate::Readable for CNDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnda::W`](W) writer structure"]
impl crate::Writable for CNDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDA to value 0"]
impl crate::Resettable for CNDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
