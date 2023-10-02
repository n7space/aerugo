#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `QSPIEN` writer - QSPI Enable"]
pub type QSPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPIDIS` writer - QSPI Disable"]
pub type QSPIDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRST` writer - QSPI Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<CR_SPEC, 0> {
        QSPIEN_W::new(self)
    }
    #[doc = "Bit 1 - QSPI Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qspidis(&mut self) -> QSPIDIS_W<CR_SPEC, 1> {
        QSPIDIS_W::new(self)
    }
    #[doc = "Bit 7 - QSPI Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC, 7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn lastxfer(&mut self) -> LASTXFER_W<CR_SPEC, 24> {
        LASTXFER_W::new(self)
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
#[doc = "Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
