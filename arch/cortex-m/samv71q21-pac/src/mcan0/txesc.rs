#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TXESC_SPEC>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TXESC_SPEC>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TBDS_R = crate::FieldReader<TBDSSELECT_A>;
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBDSSELECT_A {
    #[doc = "0: 8-byte data field"]
    _8_BYTE = 0,
    #[doc = "1: 12-byte data field"]
    _12_BYTE = 1,
    #[doc = "2: 16-byte data field"]
    _16_BYTE = 2,
    #[doc = "3: 20-byte data field"]
    _20_BYTE = 3,
    #[doc = "4: 24-byte data field"]
    _24_BYTE = 4,
    #[doc = "5: 32-byte data field"]
    _32_BYTE = 5,
    #[doc = "6: 48-byte data field"]
    _48_BYTE = 6,
    #[doc = "7: 64-byte data field"]
    _64_BYTE = 7,
}
impl From<TBDSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TBDSSELECT_A {
    type Ux = u8;
}
impl TBDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDSSELECT_A {
        match self.bits {
            0 => TBDSSELECT_A::_8_BYTE,
            1 => TBDSSELECT_A::_12_BYTE,
            2 => TBDSSELECT_A::_16_BYTE,
            3 => TBDSSELECT_A::_20_BYTE,
            4 => TBDSSELECT_A::_24_BYTE,
            5 => TBDSSELECT_A::_32_BYTE,
            6 => TBDSSELECT_A::_48_BYTE,
            7 => TBDSSELECT_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == TBDSSELECT_A::_8_BYTE
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == TBDSSELECT_A::_12_BYTE
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == TBDSSELECT_A::_16_BYTE
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == TBDSSELECT_A::_20_BYTE
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == TBDSSELECT_A::_24_BYTE
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == TBDSSELECT_A::_32_BYTE
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == TBDSSELECT_A::_48_BYTE
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == TBDSSELECT_A::_64_BYTE
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub type TBDS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TBDSSELECT_A>;
impl<'a, REG, const O: u8> TBDS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::_64_BYTE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TBDS_W<TXESC_SPEC, 0> {
        TBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXESC_SPEC;
impl crate::RegisterSpec for TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
