#[doc = "Register `PRBS` reader"]
pub type R = crate::R<PRBS_SPEC>;
#[doc = "Register `PRBS` writer"]
pub type W = crate::W<PRBS_SPEC>;
#[doc = "Field `M8PR` reader - Master 8 Priority"]
pub type M8PR_R = crate::FieldReader;
#[doc = "Field `M8PR` writer - Master 8 Priority"]
pub type M8PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M9PR` reader - Master 9 Priority"]
pub type M9PR_R = crate::FieldReader;
#[doc = "Field `M9PR` writer - Master 9 Priority"]
pub type M9PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M10PR` reader - Master 10 Priority"]
pub type M10PR_R = crate::FieldReader;
#[doc = "Field `M10PR` writer - Master 10 Priority"]
pub type M10PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M11PR` reader - Master 11 Priority"]
pub type M11PR_R = crate::FieldReader;
#[doc = "Field `M11PR` writer - Master 11 Priority"]
pub type M11PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M12PR` reader - Master 12 Priority"]
pub type M12PR_R = crate::FieldReader;
#[doc = "Field `M12PR` writer - Master 12 Priority"]
pub type M12PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8PR_R {
        M8PR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9PR_R {
        M9PR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10PR_R {
        M10PR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11PR_R {
        M11PR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12PR_R {
        M12PR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m8pr(&mut self) -> M8PR_W<PRBS_SPEC, 0> {
        M8PR_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m9pr(&mut self) -> M9PR_W<PRBS_SPEC, 4> {
        M9PR_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m10pr(&mut self) -> M10PR_W<PRBS_SPEC, 8> {
        M10PR_W::new(self)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m11pr(&mut self) -> M11PR_W<PRBS_SPEC, 12> {
        M11PR_W::new(self)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m12pr(&mut self) -> M12PR_W<PRBS_SPEC, 16> {
        M12PR_W::new(self)
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
#[doc = "Priority Register B for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRBS_SPEC;
impl crate::RegisterSpec for PRBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prbs::R`](R) reader structure"]
impl crate::Readable for PRBS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prbs::W`](W) writer structure"]
impl crate::Writable for PRBS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRBS to value 0"]
impl crate::Resettable for PRBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
