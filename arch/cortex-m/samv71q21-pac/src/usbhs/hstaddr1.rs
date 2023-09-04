#[doc = "Register `HSTADDR1` reader"]
pub type R = crate::R<HSTADDR1_SPEC>;
#[doc = "Register `HSTADDR1` writer"]
pub type W = crate::W<HSTADDR1_SPEC>;
#[doc = "Field `HSTADDRP0` reader - USB Host Address"]
pub type HSTADDRP0_R = crate::FieldReader;
#[doc = "Field `HSTADDRP0` writer - USB Host Address"]
pub type HSTADDRP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP1` reader - USB Host Address"]
pub type HSTADDRP1_R = crate::FieldReader;
#[doc = "Field `HSTADDRP1` writer - USB Host Address"]
pub type HSTADDRP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP2` reader - USB Host Address"]
pub type HSTADDRP2_R = crate::FieldReader;
#[doc = "Field `HSTADDRP2` writer - USB Host Address"]
pub type HSTADDRP2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP3` reader - USB Host Address"]
pub type HSTADDRP3_R = crate::FieldReader;
#[doc = "Field `HSTADDRP3` writer - USB Host Address"]
pub type HSTADDRP3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp0(&self) -> HSTADDRP0_R {
        HSTADDRP0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&self) -> HSTADDRP1_R {
        HSTADDRP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&self) -> HSTADDRP2_R {
        HSTADDRP2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&self) -> HSTADDRP3_R {
        HSTADDRP3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp0(&mut self) -> HSTADDRP0_W<HSTADDR1_SPEC, 0> {
        HSTADDRP0_W::new(self)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp1(&mut self) -> HSTADDRP1_W<HSTADDR1_SPEC, 8> {
        HSTADDRP1_W::new(self)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp2(&mut self) -> HSTADDRP2_W<HSTADDR1_SPEC, 16> {
        HSTADDRP2_W::new(self)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp3(&mut self) -> HSTADDRP3_W<HSTADDR1_SPEC, 24> {
        HSTADDRP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTADDR1_SPEC;
impl crate::RegisterSpec for HSTADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr1::R`](R) reader structure"]
impl crate::Readable for HSTADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstaddr1::W`](W) writer structure"]
impl crate::Writable for HSTADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTADDR1 to value 0"]
impl crate::Resettable for HSTADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
