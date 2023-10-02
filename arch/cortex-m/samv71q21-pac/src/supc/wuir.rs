#[doc = "Register `WUIR` reader"]
pub type R = crate::R<WUIR_SPEC>;
#[doc = "Register `WUIR` writer"]
pub type W = crate::W<WUIR_SPEC>;
#[doc = "Field `WKUPEN0` reader - Wake-up Input Enable 0 to 0"]
pub type WKUPEN0_R = crate::BitReader<WKUPEN0SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN0SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN0SELECT_A {
        match self.bits {
            false => WKUPEN0SELECT_A::DISABLE,
            true => WKUPEN0SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN0SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN0SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN0` writer - Wake-up Input Enable 0 to 0"]
pub type WKUPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN0SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN0SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN0SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN1` reader - Wake-up Input Enable 0 to 1"]
pub type WKUPEN1_R = crate::BitReader<WKUPEN1SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN1SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN1SELECT_A {
        match self.bits {
            false => WKUPEN1SELECT_A::DISABLE,
            true => WKUPEN1SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN1SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN1SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN1` writer - Wake-up Input Enable 0 to 1"]
pub type WKUPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN1SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN1SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN1SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN2` reader - Wake-up Input Enable 0 to 2"]
pub type WKUPEN2_R = crate::BitReader<WKUPEN2SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN2SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN2SELECT_A {
        match self.bits {
            false => WKUPEN2SELECT_A::DISABLE,
            true => WKUPEN2SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN2SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN2SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN2` writer - Wake-up Input Enable 0 to 2"]
pub type WKUPEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN2SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN2SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN2SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN3` reader - Wake-up Input Enable 0 to 3"]
pub type WKUPEN3_R = crate::BitReader<WKUPEN3SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN3SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN3SELECT_A {
        match self.bits {
            false => WKUPEN3SELECT_A::DISABLE,
            true => WKUPEN3SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN3SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN3SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN3` writer - Wake-up Input Enable 0 to 3"]
pub type WKUPEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN3SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN3SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN3SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN4` reader - Wake-up Input Enable 0 to 4"]
pub type WKUPEN4_R = crate::BitReader<WKUPEN4SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN4SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN4SELECT_A {
        match self.bits {
            false => WKUPEN4SELECT_A::DISABLE,
            true => WKUPEN4SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN4SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN4SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN4` writer - Wake-up Input Enable 0 to 4"]
pub type WKUPEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN4SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN4SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN4SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN5` reader - Wake-up Input Enable 0 to 5"]
pub type WKUPEN5_R = crate::BitReader<WKUPEN5SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN5SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN5SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN5SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN5SELECT_A {
        match self.bits {
            false => WKUPEN5SELECT_A::DISABLE,
            true => WKUPEN5SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN5SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN5SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN5` writer - Wake-up Input Enable 0 to 5"]
pub type WKUPEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN5SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN5SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN5SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN6` reader - Wake-up Input Enable 0 to 6"]
pub type WKUPEN6_R = crate::BitReader<WKUPEN6SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN6SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN6SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN6SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN6SELECT_A {
        match self.bits {
            false => WKUPEN6SELECT_A::DISABLE,
            true => WKUPEN6SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN6SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN6SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN6` writer - Wake-up Input Enable 0 to 6"]
pub type WKUPEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN6SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN6SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN6SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN7` reader - Wake-up Input Enable 0 to 7"]
pub type WKUPEN7_R = crate::BitReader<WKUPEN7SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN7SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN7SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN7SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN7SELECT_A {
        match self.bits {
            false => WKUPEN7SELECT_A::DISABLE,
            true => WKUPEN7SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN7SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN7SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN7` writer - Wake-up Input Enable 0 to 7"]
pub type WKUPEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN7SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN7SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN7SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN8` reader - Wake-up Input Enable 0 to 8"]
pub type WKUPEN8_R = crate::BitReader<WKUPEN8SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN8SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN8SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN8SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN8SELECT_A {
        match self.bits {
            false => WKUPEN8SELECT_A::DISABLE,
            true => WKUPEN8SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN8SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN8SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN8` writer - Wake-up Input Enable 0 to 8"]
pub type WKUPEN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN8SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN8SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN8SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN9` reader - Wake-up Input Enable 0 to 9"]
pub type WKUPEN9_R = crate::BitReader<WKUPEN9SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN9SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN9SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN9SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN9SELECT_A {
        match self.bits {
            false => WKUPEN9SELECT_A::DISABLE,
            true => WKUPEN9SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN9SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN9SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN9` writer - Wake-up Input Enable 0 to 9"]
pub type WKUPEN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN9SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN9SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN9SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN10` reader - Wake-up Input Enable 0 to 10"]
pub type WKUPEN10_R = crate::BitReader<WKUPEN10SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN10SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN10SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN10SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN10SELECT_A {
        match self.bits {
            false => WKUPEN10SELECT_A::DISABLE,
            true => WKUPEN10SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN10SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN10SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN10` writer - Wake-up Input Enable 0 to 10"]
pub type WKUPEN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN10SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN10SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN10SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN11` reader - Wake-up Input Enable 0 to 11"]
pub type WKUPEN11_R = crate::BitReader<WKUPEN11SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN11SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN11SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN11SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN11SELECT_A {
        match self.bits {
            false => WKUPEN11SELECT_A::DISABLE,
            true => WKUPEN11SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN11SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN11SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN11` writer - Wake-up Input Enable 0 to 11"]
pub type WKUPEN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN11SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN11SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN11SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN12` reader - Wake-up Input Enable 0 to 12"]
pub type WKUPEN12_R = crate::BitReader<WKUPEN12SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN12SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN12SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN12SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN12SELECT_A {
        match self.bits {
            false => WKUPEN12SELECT_A::DISABLE,
            true => WKUPEN12SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN12SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN12SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN12` writer - Wake-up Input Enable 0 to 12"]
pub type WKUPEN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN12SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN12SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN12SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPEN13` reader - Wake-up Input Enable 0 to 13"]
pub type WKUPEN13_R = crate::BitReader<WKUPEN13SELECT_A>;
#[doc = "Wake-up Input Enable 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPEN13SELECT_A {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN13SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN13SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN13SELECT_A {
        match self.bits {
            false => WKUPEN13SELECT_A::DISABLE,
            true => WKUPEN13SELECT_A::ENABLE,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN13SELECT_A::DISABLE
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN13SELECT_A::ENABLE
    }
}
#[doc = "Field `WKUPEN13` writer - Wake-up Input Enable 0 to 13"]
pub type WKUPEN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPEN13SELECT_A>;
impl<'a, REG, const O: u8> WKUPEN13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN13SELECT_A::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPEN13SELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPT0` reader - Wake-up Input Type 0 to 0"]
pub type WKUPT0_R = crate::BitReader<WKUPT0SELECT_A>;
#[doc = "Wake-up Input Type 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT0SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT0SELECT_A {
        match self.bits {
            false => WKUPT0SELECT_A::LOW,
            true => WKUPT0SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT0SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT0SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT0` writer - Wake-up Input Type 0 to 0"]
pub type WKUPT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT0SELECT_A>;
impl<'a, REG, const O: u8> WKUPT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT0SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT0SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT1` reader - Wake-up Input Type 0 to 1"]
pub type WKUPT1_R = crate::BitReader<WKUPT1SELECT_A>;
#[doc = "Wake-up Input Type 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT1SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT1SELECT_A {
        match self.bits {
            false => WKUPT1SELECT_A::LOW,
            true => WKUPT1SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT1SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT1SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT1` writer - Wake-up Input Type 0 to 1"]
pub type WKUPT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT1SELECT_A>;
impl<'a, REG, const O: u8> WKUPT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT1SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT1SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT2` reader - Wake-up Input Type 0 to 2"]
pub type WKUPT2_R = crate::BitReader<WKUPT2SELECT_A>;
#[doc = "Wake-up Input Type 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT2SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT2SELECT_A {
        match self.bits {
            false => WKUPT2SELECT_A::LOW,
            true => WKUPT2SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT2SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT2SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT2` writer - Wake-up Input Type 0 to 2"]
pub type WKUPT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT2SELECT_A>;
impl<'a, REG, const O: u8> WKUPT2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT2SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT2SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT3` reader - Wake-up Input Type 0 to 3"]
pub type WKUPT3_R = crate::BitReader<WKUPT3SELECT_A>;
#[doc = "Wake-up Input Type 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT3SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT3SELECT_A {
        match self.bits {
            false => WKUPT3SELECT_A::LOW,
            true => WKUPT3SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT3SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT3SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT3` writer - Wake-up Input Type 0 to 3"]
pub type WKUPT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT3SELECT_A>;
impl<'a, REG, const O: u8> WKUPT3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT3SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT3SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT4` reader - Wake-up Input Type 0 to 4"]
pub type WKUPT4_R = crate::BitReader<WKUPT4SELECT_A>;
#[doc = "Wake-up Input Type 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT4SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT4SELECT_A {
        match self.bits {
            false => WKUPT4SELECT_A::LOW,
            true => WKUPT4SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT4SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT4SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT4` writer - Wake-up Input Type 0 to 4"]
pub type WKUPT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT4SELECT_A>;
impl<'a, REG, const O: u8> WKUPT4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT4SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT4SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT5` reader - Wake-up Input Type 0 to 5"]
pub type WKUPT5_R = crate::BitReader<WKUPT5SELECT_A>;
#[doc = "Wake-up Input Type 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT5SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT5SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT5SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT5SELECT_A {
        match self.bits {
            false => WKUPT5SELECT_A::LOW,
            true => WKUPT5SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT5SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT5SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT5` writer - Wake-up Input Type 0 to 5"]
pub type WKUPT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT5SELECT_A>;
impl<'a, REG, const O: u8> WKUPT5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT5SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT5SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT6` reader - Wake-up Input Type 0 to 6"]
pub type WKUPT6_R = crate::BitReader<WKUPT6SELECT_A>;
#[doc = "Wake-up Input Type 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT6SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT6SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT6SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT6SELECT_A {
        match self.bits {
            false => WKUPT6SELECT_A::LOW,
            true => WKUPT6SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT6SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT6SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT6` writer - Wake-up Input Type 0 to 6"]
pub type WKUPT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT6SELECT_A>;
impl<'a, REG, const O: u8> WKUPT6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT6SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT6SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT7` reader - Wake-up Input Type 0 to 7"]
pub type WKUPT7_R = crate::BitReader<WKUPT7SELECT_A>;
#[doc = "Wake-up Input Type 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT7SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT7SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT7SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT7SELECT_A {
        match self.bits {
            false => WKUPT7SELECT_A::LOW,
            true => WKUPT7SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT7SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT7SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT7` writer - Wake-up Input Type 0 to 7"]
pub type WKUPT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT7SELECT_A>;
impl<'a, REG, const O: u8> WKUPT7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT7SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT7SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT8` reader - Wake-up Input Type 0 to 8"]
pub type WKUPT8_R = crate::BitReader<WKUPT8SELECT_A>;
#[doc = "Wake-up Input Type 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT8SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT8SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT8SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT8SELECT_A {
        match self.bits {
            false => WKUPT8SELECT_A::LOW,
            true => WKUPT8SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT8SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT8SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT8` writer - Wake-up Input Type 0 to 8"]
pub type WKUPT8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT8SELECT_A>;
impl<'a, REG, const O: u8> WKUPT8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT8SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT8SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT9` reader - Wake-up Input Type 0 to 9"]
pub type WKUPT9_R = crate::BitReader<WKUPT9SELECT_A>;
#[doc = "Wake-up Input Type 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT9SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT9SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT9SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT9SELECT_A {
        match self.bits {
            false => WKUPT9SELECT_A::LOW,
            true => WKUPT9SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT9SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT9SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT9` writer - Wake-up Input Type 0 to 9"]
pub type WKUPT9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT9SELECT_A>;
impl<'a, REG, const O: u8> WKUPT9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT9SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT9SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT10` reader - Wake-up Input Type 0 to 10"]
pub type WKUPT10_R = crate::BitReader<WKUPT10SELECT_A>;
#[doc = "Wake-up Input Type 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT10SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT10SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT10SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT10SELECT_A {
        match self.bits {
            false => WKUPT10SELECT_A::LOW,
            true => WKUPT10SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT10SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT10SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT10` writer - Wake-up Input Type 0 to 10"]
pub type WKUPT10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT10SELECT_A>;
impl<'a, REG, const O: u8> WKUPT10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT10SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT10SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT11` reader - Wake-up Input Type 0 to 11"]
pub type WKUPT11_R = crate::BitReader<WKUPT11SELECT_A>;
#[doc = "Wake-up Input Type 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT11SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT11SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT11SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT11SELECT_A {
        match self.bits {
            false => WKUPT11SELECT_A::LOW,
            true => WKUPT11SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT11SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT11SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT11` writer - Wake-up Input Type 0 to 11"]
pub type WKUPT11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT11SELECT_A>;
impl<'a, REG, const O: u8> WKUPT11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT11SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT11SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT12` reader - Wake-up Input Type 0 to 12"]
pub type WKUPT12_R = crate::BitReader<WKUPT12SELECT_A>;
#[doc = "Wake-up Input Type 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT12SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT12SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT12SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT12SELECT_A {
        match self.bits {
            false => WKUPT12SELECT_A::LOW,
            true => WKUPT12SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT12SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT12SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT12` writer - Wake-up Input Type 0 to 12"]
pub type WKUPT12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT12SELECT_A>;
impl<'a, REG, const O: u8> WKUPT12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT12SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT12SELECT_A::HIGH)
    }
}
#[doc = "Field `WKUPT13` reader - Wake-up Input Type 0 to 13"]
pub type WKUPT13_R = crate::BitReader<WKUPT13SELECT_A>;
#[doc = "Wake-up Input Type 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPT13SELECT_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT13SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT13SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT13SELECT_A {
        match self.bits {
            false => WKUPT13SELECT_A::LOW,
            true => WKUPT13SELECT_A::HIGH,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT13SELECT_A::LOW
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT13SELECT_A::HIGH
    }
}
#[doc = "Field `WKUPT13` writer - Wake-up Input Type 0 to 13"]
pub type WKUPT13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUPT13SELECT_A>;
impl<'a, REG, const O: u8> WKUPT13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT13SELECT_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPT13SELECT_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> WKUPEN0_R {
        WKUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> WKUPEN7_R {
        WKUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> WKUPEN8_R {
        WKUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> WKUPEN9_R {
        WKUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> WKUPEN10_R {
        WKUPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> WKUPEN11_R {
        WKUPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> WKUPEN12_R {
        WKUPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> WKUPEN13_R {
        WKUPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> WKUPT0_R {
        WKUPT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> WKUPT1_R {
        WKUPT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> WKUPT2_R {
        WKUPT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> WKUPT3_R {
        WKUPT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> WKUPT4_R {
        WKUPT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> WKUPT5_R {
        WKUPT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> WKUPT6_R {
        WKUPT6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> WKUPT7_R {
        WKUPT7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> WKUPT8_R {
        WKUPT8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> WKUPT9_R {
        WKUPT9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> WKUPT10_R {
        WKUPT10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> WKUPT11_R {
        WKUPT11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> WKUPT12_R {
        WKUPT12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> WKUPT13_R {
        WKUPT13_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen0(&mut self) -> WKUPEN0_W<WUIR_SPEC, 0> {
        WKUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<WUIR_SPEC, 1> {
        WKUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<WUIR_SPEC, 2> {
        WKUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<WUIR_SPEC, 3> {
        WKUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<WUIR_SPEC, 4> {
        WKUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<WUIR_SPEC, 5> {
        WKUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<WUIR_SPEC, 6> {
        WKUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen7(&mut self) -> WKUPEN7_W<WUIR_SPEC, 7> {
        WKUPEN7_W::new(self)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen8(&mut self) -> WKUPEN8_W<WUIR_SPEC, 8> {
        WKUPEN8_W::new(self)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen9(&mut self) -> WKUPEN9_W<WUIR_SPEC, 9> {
        WKUPEN9_W::new(self)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen10(&mut self) -> WKUPEN10_W<WUIR_SPEC, 10> {
        WKUPEN10_W::new(self)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen11(&mut self) -> WKUPEN11_W<WUIR_SPEC, 11> {
        WKUPEN11_W::new(self)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen12(&mut self) -> WKUPEN12_W<WUIR_SPEC, 12> {
        WKUPEN12_W::new(self)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen13(&mut self) -> WKUPEN13_W<WUIR_SPEC, 13> {
        WKUPEN13_W::new(self)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt0(&mut self) -> WKUPT0_W<WUIR_SPEC, 16> {
        WKUPT0_W::new(self)
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt1(&mut self) -> WKUPT1_W<WUIR_SPEC, 17> {
        WKUPT1_W::new(self)
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt2(&mut self) -> WKUPT2_W<WUIR_SPEC, 18> {
        WKUPT2_W::new(self)
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt3(&mut self) -> WKUPT3_W<WUIR_SPEC, 19> {
        WKUPT3_W::new(self)
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt4(&mut self) -> WKUPT4_W<WUIR_SPEC, 20> {
        WKUPT4_W::new(self)
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt5(&mut self) -> WKUPT5_W<WUIR_SPEC, 21> {
        WKUPT5_W::new(self)
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt6(&mut self) -> WKUPT6_W<WUIR_SPEC, 22> {
        WKUPT6_W::new(self)
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt7(&mut self) -> WKUPT7_W<WUIR_SPEC, 23> {
        WKUPT7_W::new(self)
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt8(&mut self) -> WKUPT8_W<WUIR_SPEC, 24> {
        WKUPT8_W::new(self)
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt9(&mut self) -> WKUPT9_W<WUIR_SPEC, 25> {
        WKUPT9_W::new(self)
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt10(&mut self) -> WKUPT10_W<WUIR_SPEC, 26> {
        WKUPT10_W::new(self)
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt11(&mut self) -> WKUPT11_W<WUIR_SPEC, 27> {
        WKUPT11_W::new(self)
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt12(&mut self) -> WKUPT12_W<WUIR_SPEC, 28> {
        WKUPT12_W::new(self)
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt13(&mut self) -> WKUPT13_W<WUIR_SPEC, 29> {
        WKUPT13_W::new(self)
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
#[doc = "Supply Controller Wake-up Inputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUIR_SPEC;
impl crate::RegisterSpec for WUIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuir::R`](R) reader structure"]
impl crate::Readable for WUIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wuir::W`](W) writer structure"]
impl crate::Writable for WUIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUIR to value 0"]
impl crate::Resettable for WUIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
