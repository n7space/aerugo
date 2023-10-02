#[doc = "Register `MLBC1` reader"]
pub type R = crate::R<MLBC1_SPEC>;
#[doc = "Register `MLBC1` writer"]
pub type W = crate::W<MLBC1_SPEC>;
#[doc = "Field `LOCK` reader - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKM` reader - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type CLKM_R = crate::BitReader;
#[doc = "Field `CLKM` writer - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type CLKM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDA` reader - Node Device Address"]
pub type NDA_R = crate::FieldReader;
#[doc = "Field `NDA` writer - Node Device Address"]
pub type NDA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&self) -> CLKM_R {
        CLKM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<MLBC1_SPEC, 6> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn clkm(&mut self) -> CLKM_W<MLBC1_SPEC, 7> {
        CLKM_W::new(self)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn nda(&mut self) -> NDA_W<MLBC1_SPEC, 8> {
        NDA_W::new(self)
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
#[doc = "MediaLB Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlbc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlbc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MLBC1_SPEC;
impl crate::RegisterSpec for MLBC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlbc1::R`](R) reader structure"]
impl crate::Readable for MLBC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mlbc1::W`](W) writer structure"]
impl crate::Writable for MLBC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLBC1 to value 0"]
impl crate::Resettable for MLBC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
