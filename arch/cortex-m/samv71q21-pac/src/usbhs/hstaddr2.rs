#[doc = "Register `HSTADDR2` reader"]
pub type R = crate::R<HSTADDR2_SPEC>;
#[doc = "Register `HSTADDR2` writer"]
pub type W = crate::W<HSTADDR2_SPEC>;
#[doc = "Field `HSTADDRP4` reader - USB Host Address"]
pub type HSTADDRP4_R = crate::FieldReader;
#[doc = "Field `HSTADDRP4` writer - USB Host Address"]
pub type HSTADDRP4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP5` reader - USB Host Address"]
pub type HSTADDRP5_R = crate::FieldReader;
#[doc = "Field `HSTADDRP5` writer - USB Host Address"]
pub type HSTADDRP5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP6` reader - USB Host Address"]
pub type HSTADDRP6_R = crate::FieldReader;
#[doc = "Field `HSTADDRP6` writer - USB Host Address"]
pub type HSTADDRP6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HSTADDRP7` reader - USB Host Address"]
pub type HSTADDRP7_R = crate::FieldReader;
#[doc = "Field `HSTADDRP7` writer - USB Host Address"]
pub type HSTADDRP7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&self) -> HSTADDRP4_R {
        HSTADDRP4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&self) -> HSTADDRP5_R {
        HSTADDRP5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&self) -> HSTADDRP6_R {
        HSTADDRP6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&self) -> HSTADDRP7_R {
        HSTADDRP7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp4(&mut self) -> HSTADDRP4_W<HSTADDR2_SPEC, 0> {
        HSTADDRP4_W::new(self)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp5(&mut self) -> HSTADDRP5_W<HSTADDR2_SPEC, 8> {
        HSTADDRP5_W::new(self)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp6(&mut self) -> HSTADDRP6_W<HSTADDR2_SPEC, 16> {
        HSTADDRP6_W::new(self)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp7(&mut self) -> HSTADDRP7_W<HSTADDR2_SPEC, 24> {
        HSTADDRP7_W::new(self)
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
#[doc = "Host Address 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTADDR2_SPEC;
impl crate::RegisterSpec for HSTADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr2::R`](R) reader structure"]
impl crate::Readable for HSTADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstaddr2::W`](W) writer structure"]
impl crate::Writable for HSTADDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTADDR2 to value 0"]
impl crate::Resettable for HSTADDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
