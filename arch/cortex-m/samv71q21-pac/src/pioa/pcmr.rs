#[doc = "Register `PCMR` reader"]
pub type R = crate::R<PCMR_SPEC>;
#[doc = "Register `PCMR` writer"]
pub type W = crate::W<PCMR_SPEC>;
#[doc = "Field `PCEN` reader - Parallel Capture Mode Enable"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - Parallel Capture Mode Enable"]
pub type PCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSIZE` reader - Parallel Capture Mode Data Size"]
pub type DSIZE_R = crate::FieldReader<DSIZESELECT_A>;
#[doc = "Parallel Capture Mode Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSIZESELECT_A {
    #[doc = "0: The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE = 0,
    #[doc = "1: The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD = 1,
    #[doc = "2: The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD = 2,
}
impl From<DSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSIZESELECT_A {
    type Ux = u8;
}
impl DSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSIZESELECT_A> {
        match self.bits {
            0 => Some(DSIZESELECT_A::BYTE),
            1 => Some(DSIZESELECT_A::HALFWORD),
            2 => Some(DSIZESELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSIZESELECT_A::BYTE
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DSIZESELECT_A::HALFWORD
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSIZESELECT_A::WORD
    }
}
#[doc = "Field `DSIZE` writer - Parallel Capture Mode Data Size"]
pub type DSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DSIZESELECT_A>;
impl<'a, REG, const O: u8> DSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZESELECT_A::BYTE)
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZESELECT_A::HALFWORD)
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZESELECT_A::WORD)
    }
}
#[doc = "Field `ALWYS` reader - Parallel Capture Mode Always Sampling"]
pub type ALWYS_R = crate::BitReader;
#[doc = "Field `ALWYS` writer - Parallel Capture Mode Always Sampling"]
pub type ALWYS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HALFS` reader - Parallel Capture Mode Half Sampling"]
pub type HALFS_R = crate::BitReader;
#[doc = "Field `HALFS` writer - Parallel Capture Mode Half Sampling"]
pub type HALFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRSTS` reader - Parallel Capture Mode First Sample"]
pub type FRSTS_R = crate::BitReader;
#[doc = "Field `FRSTS` writer - Parallel Capture Mode First Sample"]
pub type FRSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<PCMR_SPEC, 0> {
        PCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<PCMR_SPEC, 4> {
        DSIZE_W::new(self)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn alwys(&mut self) -> ALWYS_W<PCMR_SPEC, 9> {
        ALWYS_W::new(self)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn halfs(&mut self) -> HALFS_W<PCMR_SPEC, 10> {
        HALFS_W::new(self)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    #[must_use]
    pub fn frsts(&mut self) -> FRSTS_W<PCMR_SPEC, 11> {
        FRSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parallel Capture Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMR_SPEC;
impl crate::RegisterSpec for PCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmr::R`](R) reader structure"]
impl crate::Readable for PCMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmr::W`](W) writer structure"]
impl crate::Writable for PCMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMR to value 0"]
impl crate::Resettable for PCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
