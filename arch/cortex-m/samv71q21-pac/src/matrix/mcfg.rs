#[doc = "Register `MCFG[%s]` reader"]
pub type R = crate::R<MCFG_SPEC>;
#[doc = "Register `MCFG[%s]` writer"]
pub type W = crate::W<MCFG_SPEC>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader<ULBTSELECT_A>;
#[doc = "Undefined Length Burst Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ULBTSELECT_A {
    #[doc = "0: Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    UNLTD_LENGTH = 0,
    #[doc = "1: Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    SINGLE_ACCESS = 1,
    #[doc = "2: 4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    _4BEAT_BURST = 2,
    #[doc = "3: 8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    _8BEAT_BURST = 3,
    #[doc = "4: 16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    _16BEAT_BURST = 4,
    #[doc = "5: 32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    _32BEAT_BURST = 5,
    #[doc = "6: 64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    _64BEAT_BURST = 6,
    #[doc = "7: 128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    _128BEAT_BURST = 7,
}
impl From<ULBTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ULBTSELECT_A {
    type Ux = u8;
}
impl ULBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULBTSELECT_A {
        match self.bits {
            0 => ULBTSELECT_A::UNLTD_LENGTH,
            1 => ULBTSELECT_A::SINGLE_ACCESS,
            2 => ULBTSELECT_A::_4BEAT_BURST,
            3 => ULBTSELECT_A::_8BEAT_BURST,
            4 => ULBTSELECT_A::_16BEAT_BURST,
            5 => ULBTSELECT_A::_32BEAT_BURST,
            6 => ULBTSELECT_A::_64BEAT_BURST,
            7 => ULBTSELECT_A::_128BEAT_BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn is_unltd_length(&self) -> bool {
        *self == ULBTSELECT_A::UNLTD_LENGTH
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn is_single_access(&self) -> bool {
        *self == ULBTSELECT_A::SINGLE_ACCESS
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn is_4beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_4BEAT_BURST
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn is_8beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_8BEAT_BURST
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn is_16beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_16BEAT_BURST
    }
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn is_32beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_32BEAT_BURST
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn is_64beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_64BEAT_BURST
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    #[inline(always)]
    pub fn is_128beat_burst(&self) -> bool {
        *self == ULBTSELECT_A::_128BEAT_BURST
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, ULBTSELECT_A>;
impl<'a, REG, const O: u8> ULBT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn unltd_length(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::UNLTD_LENGTH)
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn single_access(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::SINGLE_ACCESS)
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn _4beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_4BEAT_BURST)
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn _8beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_8BEAT_BURST)
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn _16beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_16BEAT_BURST)
    }
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn _32beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_32BEAT_BURST)
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn _64beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_64BEAT_BURST)
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    #[inline(always)]
    pub fn _128beat_burst(self) -> &'a mut crate::W<REG> {
        self.variant(ULBTSELECT_A::_128BEAT_BURST)
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> ULBT_W<MCFG_SPEC, 0> {
        ULBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for MCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFG[%s]
to value 0"]
impl crate::Resettable for MCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
