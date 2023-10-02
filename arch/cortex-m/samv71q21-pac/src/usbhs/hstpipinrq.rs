#[doc = "Register `HSTPIPINRQ[%s]` reader"]
pub type R = crate::R<HSTPIPINRQ_SPEC>;
#[doc = "Register `HSTPIPINRQ[%s]` writer"]
pub type W = crate::W<HSTPIPINRQ_SPEC>;
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type INRQ_R = crate::FieldReader;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type INRQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type INMODE_R = crate::BitReader;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type INMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<HSTPIPINRQ_SPEC, 0> {
        INRQ_W::new(self)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> INMODE_W<HSTPIPINRQ_SPEC, 8> {
        INMODE_W::new(self)
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
#[doc = "Host Pipe IN Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPINRQ_SPEC;
impl crate::RegisterSpec for HSTPIPINRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq::R`](R) reader structure"]
impl crate::Readable for HSTPIPINRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq::W`](W) writer structure"]
impl crate::Writable for HSTPIPINRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPINRQ[%s]
to value 0"]
impl crate::Resettable for HSTPIPINRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
