#[doc = "Register `SYS_GPBR[%s]` reader"]
pub type R = crate::R<SYS_GPBR_SPEC>;
#[doc = "Register `SYS_GPBR[%s]` writer"]
pub type W = crate::W<SYS_GPBR_SPEC>;
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GPBR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GPBR_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GPBR_VALUE_R {
        GPBR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    #[must_use]
    pub fn gpbr_value(&mut self) -> GPBR_VALUE_W<SYS_GPBR_SPEC, 0> {
        GPBR_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General Purpose Backup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_gpbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_gpbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_GPBR_SPEC;
impl crate::RegisterSpec for SYS_GPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_gpbr::R`](R) reader structure"]
impl crate::Readable for SYS_GPBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_gpbr::W`](W) writer structure"]
impl crate::Writable for SYS_GPBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_GPBR[%s]
to value 0"]
impl crate::Resettable for SYS_GPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
