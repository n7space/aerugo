#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<RXF1S_SPEC>;
#[doc = "Field `F1FL` reader - Receive FIFO 1 Fill Level"]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1GI` reader - Receive FIFO 1 Get Index"]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Receive FIFO 1 Put Index"]
pub type F1PI_R = crate::FieldReader;
#[doc = "Field `F1F` reader - Receive FIFO 1 Fill Level"]
pub type F1F_R = crate::BitReader;
#[doc = "Field `RF1L` reader - Receive FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader;
#[doc = "Field `DMS` reader - Debug Message Status"]
pub type DMS_R = crate::FieldReader<DMSSELECT_A>;
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMSSELECT_A {
    #[doc = "0: Idle state, wait for reception of debug messages, DMA request is cleared."]
    IDLE = 0,
    #[doc = "1: Debug message A received."]
    MSG_A = 1,
    #[doc = "2: Debug messages A, B received."]
    MSG_AB = 2,
    #[doc = "3: Debug messages A, B, C received, DMA request is set."]
    MSG_ABC = 3,
}
impl From<DMSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMSSELECT_A {
    type Ux = u8;
}
impl DMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMSSELECT_A {
        match self.bits {
            0 => DMSSELECT_A::IDLE,
            1 => DMSSELECT_A::MSG_A,
            2 => DMSSELECT_A::MSG_AB,
            3 => DMSSELECT_A::MSG_ABC,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle state, wait for reception of debug messages, DMA request is cleared."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DMSSELECT_A::IDLE
    }
    #[doc = "Debug message A received."]
    #[inline(always)]
    pub fn is_msg_a(&self) -> bool {
        *self == DMSSELECT_A::MSG_A
    }
    #[doc = "Debug messages A, B received."]
    #[inline(always)]
    pub fn is_msg_ab(&self) -> bool {
        *self == DMSSELECT_A::MSG_AB
    }
    #[doc = "Debug messages A, B, C received, DMA request is set."]
    #[inline(always)]
    pub fn is_msg_abc(&self) -> bool {
        *self == DMSSELECT_A::MSG_ABC
    }
}
impl R {
    #[doc = "Bits 0:6 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Receive FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Receive FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
