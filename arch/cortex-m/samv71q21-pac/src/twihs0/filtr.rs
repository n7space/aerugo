#[doc = "Register `FILTR` reader"]
pub type R = crate::R<FILTR_SPEC>;
#[doc = "Register `FILTR` writer"]
pub type W = crate::W<FILTR_SPEC>;
#[doc = "Field `FILT` reader - RX Digital Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - RX Digital Filter"]
pub type FILT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PADFEN` reader - PAD Filter Enable"]
pub type PADFEN_R = crate::BitReader;
#[doc = "Field `PADFEN` writer - PAD Filter Enable"]
pub type PADFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PADFCFG` reader - PAD Filter Config"]
pub type PADFCFG_R = crate::BitReader;
#[doc = "Field `PADFCFG` writer - PAD Filter Config"]
pub type PADFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES` reader - Digital Filter Threshold"]
pub type THRES_R = crate::FieldReader;
#[doc = "Field `THRES` writer - Digital Filter Threshold"]
pub type THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PADFEN_R {
        PADFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&self) -> PADFCFG_R {
        PADFCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<FILTR_SPEC, 0> {
        FILT_W::new(self)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn padfen(&mut self) -> PADFEN_W<FILTR_SPEC, 1> {
        PADFEN_W::new(self)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    #[must_use]
    pub fn padfcfg(&mut self) -> PADFCFG_W<FILTR_SPEC, 2> {
        PADFCFG_W::new(self)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<FILTR_SPEC, 8> {
        THRES_W::new(self)
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
#[doc = "Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTR_SPEC;
impl crate::RegisterSpec for FILTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filtr::R`](R) reader structure"]
impl crate::Readable for FILTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filtr::W`](W) writer structure"]
impl crate::Writable for FILTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTR to value 0"]
impl crate::Resettable for FILTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
