#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RXESC_SPEC>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RXESC_SPEC>;
#[doc = "Field `F0DS` reader - Receive FIFO 0 Data Field Size"]
pub type F0DS_R = crate::FieldReader<F0DSSELECT_A>;
#[doc = "Receive FIFO 0 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F0DSSELECT_A {
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
impl From<F0DSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: F0DSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F0DSSELECT_A {
    type Ux = u8;
}
impl F0DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0DSSELECT_A {
        match self.bits {
            0 => F0DSSELECT_A::_8_BYTE,
            1 => F0DSSELECT_A::_12_BYTE,
            2 => F0DSSELECT_A::_16_BYTE,
            3 => F0DSSELECT_A::_20_BYTE,
            4 => F0DSSELECT_A::_24_BYTE,
            5 => F0DSSELECT_A::_32_BYTE,
            6 => F0DSSELECT_A::_48_BYTE,
            7 => F0DSSELECT_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == F0DSSELECT_A::_8_BYTE
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == F0DSSELECT_A::_12_BYTE
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == F0DSSELECT_A::_16_BYTE
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == F0DSSELECT_A::_20_BYTE
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == F0DSSELECT_A::_24_BYTE
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == F0DSSELECT_A::_32_BYTE
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == F0DSSELECT_A::_48_BYTE
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == F0DSSELECT_A::_64_BYTE
    }
}
#[doc = "Field `F0DS` writer - Receive FIFO 0 Data Field Size"]
pub type F0DS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, F0DSSELECT_A>;
impl<'a, REG, const O: u8> F0DS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0DSSELECT_A::_64_BYTE)
    }
}
#[doc = "Field `F1DS` reader - Receive FIFO 1 Data Field Size"]
pub type F1DS_R = crate::FieldReader<F1DSSELECT_A>;
#[doc = "Receive FIFO 1 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F1DSSELECT_A {
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
impl From<F1DSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: F1DSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F1DSSELECT_A {
    type Ux = u8;
}
impl F1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1DSSELECT_A {
        match self.bits {
            0 => F1DSSELECT_A::_8_BYTE,
            1 => F1DSSELECT_A::_12_BYTE,
            2 => F1DSSELECT_A::_16_BYTE,
            3 => F1DSSELECT_A::_20_BYTE,
            4 => F1DSSELECT_A::_24_BYTE,
            5 => F1DSSELECT_A::_32_BYTE,
            6 => F1DSSELECT_A::_48_BYTE,
            7 => F1DSSELECT_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == F1DSSELECT_A::_8_BYTE
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == F1DSSELECT_A::_12_BYTE
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == F1DSSELECT_A::_16_BYTE
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == F1DSSELECT_A::_20_BYTE
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == F1DSSELECT_A::_24_BYTE
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == F1DSSELECT_A::_32_BYTE
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == F1DSSELECT_A::_48_BYTE
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == F1DSSELECT_A::_64_BYTE
    }
}
#[doc = "Field `F1DS` writer - Receive FIFO 1 Data Field Size"]
pub type F1DS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, F1DSSELECT_A>;
impl<'a, REG, const O: u8> F1DS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1DSSELECT_A::_64_BYTE)
    }
}
#[doc = "Field `RBDS` reader - Receive Buffer Data Field Size"]
pub type RBDS_R = crate::FieldReader<RBDSSELECT_A>;
#[doc = "Receive Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RBDSSELECT_A {
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
impl From<RBDSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RBDSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RBDSSELECT_A {
    type Ux = u8;
}
impl RBDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDSSELECT_A {
        match self.bits {
            0 => RBDSSELECT_A::_8_BYTE,
            1 => RBDSSELECT_A::_12_BYTE,
            2 => RBDSSELECT_A::_16_BYTE,
            3 => RBDSSELECT_A::_20_BYTE,
            4 => RBDSSELECT_A::_24_BYTE,
            5 => RBDSSELECT_A::_32_BYTE,
            6 => RBDSSELECT_A::_48_BYTE,
            7 => RBDSSELECT_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == RBDSSELECT_A::_8_BYTE
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == RBDSSELECT_A::_12_BYTE
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == RBDSSELECT_A::_16_BYTE
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == RBDSSELECT_A::_20_BYTE
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == RBDSSELECT_A::_24_BYTE
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == RBDSSELECT_A::_32_BYTE
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == RBDSSELECT_A::_48_BYTE
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == RBDSSELECT_A::_64_BYTE
    }
}
#[doc = "Field `RBDS` writer - Receive Buffer Data Field Size"]
pub type RBDS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, RBDSSELECT_A>;
impl<'a, REG, const O: u8> RBDS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RBDSSELECT_A::_64_BYTE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<RXESC_SPEC, 0> {
        F0DS_W::new(self)
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<RXESC_SPEC, 4> {
        F1DS_W::new(self)
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<RXESC_SPEC, 8> {
        RBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Buffer / FIFO Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXESC_SPEC;
impl crate::RegisterSpec for RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
