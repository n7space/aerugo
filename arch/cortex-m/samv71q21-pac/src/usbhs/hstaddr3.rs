#[doc = "Register `HSTADDR3` reader"]
pub type R = crate::R<HSTADDR3_SPEC>;
#[doc = "Register `HSTADDR3` writer"]
pub type W = crate::W<HSTADDR3_SPEC>;
#[doc = "Field `HSTADDRP8` reader - USB Host Address"]
pub type HSTADDRP8_R = crate::FieldReader;
#[doc = "Field `HSTADDRP8` writer - USB Host Address"]
pub type HSTADDRP8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP9` reader - USB Host Address"]
pub type HSTADDRP9_R = crate::FieldReader;
#[doc = "Field `HSTADDRP9` writer - USB Host Address"]
pub type HSTADDRP9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&self) -> HSTADDRP8_R {
        HSTADDRP8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&self) -> HSTADDRP9_R {
        HSTADDRP9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp8(&mut self) -> HSTADDRP8_W<HSTADDR3_SPEC, 0> {
        HSTADDRP8_W::new(self)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp9(&mut self) -> HSTADDRP9_W<HSTADDR3_SPEC, 8> {
        HSTADDRP9_W::new(self)
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
#[doc = "Host Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTADDR3_SPEC;
impl crate::RegisterSpec for HSTADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr3::R`](R) reader structure"]
impl crate::Readable for HSTADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstaddr3::W`](W) writer structure"]
impl crate::Writable for HSTADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTADDR3 to value 0"]
impl crate::Resettable for HSTADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
