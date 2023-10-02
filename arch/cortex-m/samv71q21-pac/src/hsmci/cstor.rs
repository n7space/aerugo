#[doc = "Register `CSTOR` reader"]
pub type R = crate::R<CSTOR_SPEC>;
#[doc = "Register `CSTOR` writer"]
pub type W = crate::W<CSTOR_SPEC>;
#[doc = "Field `CSTOCYC` reader - Completion Signal Timeout Cycle Number"]
pub type CSTOCYC_R = crate::FieldReader;
#[doc = "Field `CSTOCYC` writer - Completion Signal Timeout Cycle Number"]
pub type CSTOCYC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CSTOMUL` reader - Completion Signal Timeout Multiplier"]
pub type CSTOMUL_R = crate::FieldReader<CSTOMULSELECT_A>;
#[doc = "Completion Signal Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSTOMULSELECT_A {
    #[doc = "0: CSTOCYC x 1"]
    _1 = 0,
    #[doc = "1: CSTOCYC x 16"]
    _16 = 1,
    #[doc = "2: CSTOCYC x 128"]
    _128 = 2,
    #[doc = "3: CSTOCYC x 256"]
    _256 = 3,
    #[doc = "4: CSTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: CSTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: CSTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: CSTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<CSTOMULSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSTOMULSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSTOMULSELECT_A {
    type Ux = u8;
}
impl CSTOMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOMULSELECT_A {
        match self.bits {
            0 => CSTOMULSELECT_A::_1,
            1 => CSTOMULSELECT_A::_16,
            2 => CSTOMULSELECT_A::_128,
            3 => CSTOMULSELECT_A::_256,
            4 => CSTOMULSELECT_A::_1024,
            5 => CSTOMULSELECT_A::_4096,
            6 => CSTOMULSELECT_A::_65536,
            7 => CSTOMULSELECT_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOMULSELECT_A::_1
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CSTOMULSELECT_A::_16
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == CSTOMULSELECT_A::_128
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CSTOMULSELECT_A::_256
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == CSTOMULSELECT_A::_1024
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == CSTOMULSELECT_A::_4096
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == CSTOMULSELECT_A::_65536
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == CSTOMULSELECT_A::_1048576
    }
}
#[doc = "Field `CSTOMUL` writer - Completion Signal Timeout Multiplier"]
pub type CSTOMUL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CSTOMULSELECT_A>;
impl<'a, REG, const O: u8> CSTOMUL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_1)
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_16)
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_128)
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_256)
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_1024)
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_4096)
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_65536)
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOMULSELECT_A::_1048576)
    }
}
impl R {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&self) -> CSTOCYC_R {
        CSTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&self) -> CSTOMUL_R {
        CSTOMUL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn cstocyc(&mut self) -> CSTOCYC_W<CSTOR_SPEC, 0> {
        CSTOCYC_W::new(self)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn cstomul(&mut self) -> CSTOMUL_W<CSTOR_SPEC, 4> {
        CSTOMUL_W::new(self)
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
#[doc = "Completion Signal Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cstor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTOR_SPEC;
impl crate::RegisterSpec for CSTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstor::R`](R) reader structure"]
impl crate::Readable for CSTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cstor::W`](W) writer structure"]
impl crate::Writable for CSTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSTOR to value 0"]
impl crate::Resettable for CSTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
