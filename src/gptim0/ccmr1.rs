#[doc = "Register `CCMR1` reader"]
pub type R = crate::R<Ccmr1Spec>;
#[doc = "Register `CCMR1` writer"]
pub type W = crate::W<Ccmr1Spec>;
#[doc = "Capture compare 0 selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc0s {
    #[doc = "0: Channel configured in output mode"]
    Output = 0,
    #[doc = "1: Input mode, capture input mapped to channel 1"]
    InputTi1 = 1,
    #[doc = "2: Input mode, capture input mapped to channel 0"]
    InputTi0 = 2,
    #[doc = "3: Input mode, capture input mapped to trigger input"]
    InputTrc = 3,
}
impl From<Cc0s> for u8 {
    #[inline(always)]
    fn from(variant: Cc0s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc0s {
    type Ux = u8;
}
impl crate::IsEnum for Cc0s {}
#[doc = "Field `CC0S` reader - Capture compare 0 selection"]
pub type Cc0sR = crate::FieldReader<Cc0s>;
impl Cc0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc0s {
        match self.bits {
            0 => Cc0s::Output,
            1 => Cc0s::InputTi1,
            2 => Cc0s::InputTi0,
            3 => Cc0s::InputTrc,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Cc0s::Output
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn is_input_ti1(&self) -> bool {
        *self == Cc0s::InputTi1
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn is_input_ti0(&self) -> bool {
        *self == Cc0s::InputTi0
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn is_input_trc(&self) -> bool {
        *self == Cc0s::InputTrc
    }
}
#[doc = "Field `CC0S` writer - Capture compare 0 selection"]
pub type Cc0sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc0s, crate::Safe>;
impl<'a, REG> Cc0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Cc0s::Output)
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn input_ti1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc0s::InputTi1)
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn input_ti0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc0s::InputTi0)
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn input_trc(self) -> &'a mut crate::W<REG> {
        self.variant(Cc0s::InputTrc)
    }
}
#[doc = "Field `IC0PSC` reader - Channel 0 input capture prescaler, active when CC0S selects an input"]
pub type Ic0pscR = crate::FieldReader;
#[doc = "Field `IC0PSC` writer - Channel 0 input capture prescaler, active when CC0S selects an input"]
pub type Ic0pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC0FE` reader - Channel 0 fast output enable"]
pub type Oc0feR = crate::BitReader;
#[doc = "Field `OC0FE` writer - Channel 0 fast output enable"]
pub type Oc0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC0PE` reader - Channel 0 output compare shadow register enable"]
pub type Oc0peR = crate::BitReader;
#[doc = "Field `OC0PE` writer - Channel 0 output compare shadow register enable"]
pub type Oc0peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC0F` reader - Channel 0 input filter, active when CC0S selects an input"]
pub type Ic0fR = crate::FieldReader;
#[doc = "Field `IC0F` writer - Channel 0 input filter, active when CC0S selects an input"]
pub type Ic0fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Channel 0 output compare mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc0m {
    #[doc = "0: Freeze mode, channel output does not change"]
    Frozen = 0,
    #[doc = "1: Channel output valid level after matching"]
    ActiveOnMatch = 1,
    #[doc = "2: Channel output failure level after matching"]
    InactiveOnMatch = 2,
    #[doc = "3: Channel output flipped after matching"]
    Toggle = 3,
    #[doc = "4: Forced failure level output"]
    ForceInactive = 4,
    #[doc = "5: Forced valid level output"]
    ForceActive = 5,
    #[doc = "6: PWM mode 1"]
    Pwm1 = 6,
    #[doc = "7: PWM mode 2"]
    Pwm2 = 7,
}
impl From<Oc0m> for u8 {
    #[inline(always)]
    fn from(variant: Oc0m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oc0m {
    type Ux = u8;
}
impl crate::IsEnum for Oc0m {}
#[doc = "Field `OC0M` reader - Channel 0 output compare mode selection"]
pub type Oc0mR = crate::FieldReader<Oc0m>;
impl Oc0mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc0m {
        match self.bits {
            0 => Oc0m::Frozen,
            1 => Oc0m::ActiveOnMatch,
            2 => Oc0m::InactiveOnMatch,
            3 => Oc0m::Toggle,
            4 => Oc0m::ForceInactive,
            5 => Oc0m::ForceActive,
            6 => Oc0m::Pwm1,
            7 => Oc0m::Pwm2,
            _ => unreachable!(),
        }
    }
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Oc0m::Frozen
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == Oc0m::ActiveOnMatch
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == Oc0m::InactiveOnMatch
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Oc0m::Toggle
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == Oc0m::ForceInactive
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == Oc0m::ForceActive
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Oc0m::Pwm1
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == Oc0m::Pwm2
    }
}
#[doc = "Field `OC0M` writer - Channel 0 output compare mode selection"]
pub type Oc0mW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc0m, crate::Safe>;
impl<'a, REG> Oc0mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::Frozen)
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::ActiveOnMatch)
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::InactiveOnMatch)
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::Toggle)
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::ForceInactive)
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::ForceActive)
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::Pwm1)
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(Oc0m::Pwm2)
    }
}
#[doc = "Field `OC0CE` reader - Channel 0 output compare clear enable"]
pub type Oc0ceR = crate::BitReader;
#[doc = "Field `OC0CE` writer - Channel 0 output compare clear enable"]
pub type Oc0ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture compare 1 selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1s {
    #[doc = "0: Channel configured in output mode"]
    Output = 0,
    #[doc = "1: Input mode, capture input mapped to channel 1"]
    InputTi1 = 1,
    #[doc = "2: Input mode, capture input mapped to channel 0"]
    InputTi0 = 2,
    #[doc = "3: Input mode, capture input mapped to trigger input"]
    InputTrc = 3,
}
impl From<Cc1s> for u8 {
    #[inline(always)]
    fn from(variant: Cc1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc1s {
    type Ux = u8;
}
impl crate::IsEnum for Cc1s {}
#[doc = "Field `CC1S` reader - Capture compare 1 selection"]
pub type Cc1sR = crate::FieldReader<Cc1s>;
impl Cc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1s {
        match self.bits {
            0 => Cc1s::Output,
            1 => Cc1s::InputTi1,
            2 => Cc1s::InputTi0,
            3 => Cc1s::InputTrc,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Cc1s::Output
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn is_input_ti1(&self) -> bool {
        *self == Cc1s::InputTi1
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn is_input_ti0(&self) -> bool {
        *self == Cc1s::InputTi0
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn is_input_trc(&self) -> bool {
        *self == Cc1s::InputTrc
    }
}
#[doc = "Field `CC1S` writer - Capture compare 1 selection"]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1s, crate::Safe>;
impl<'a, REG> Cc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::Output)
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn input_ti1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::InputTi1)
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn input_ti0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::InputTi0)
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn input_trc(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::InputTrc)
    }
}
#[doc = "Field `IC1PSC` reader - Channel 1 input capture prescaler, active when CC1S selects an input"]
pub type Ic1pscR = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Channel 1 input capture prescaler, active when CC1S selects an input"]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Channel 1 fast output enable"]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - Channel 1 fast output enable"]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Channel 1 output compare shadow register enable"]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - Channel 1 output compare shadow register enable"]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC1F` reader - Channel 1 input filter, active when CC1S selects an input"]
pub type Ic1fR = crate::FieldReader;
#[doc = "Field `IC1F` writer - Channel 1 input filter, active when CC1S selects an input"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Channel 1 output compare mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc1m {
    #[doc = "0: Freeze mode, channel output does not change"]
    Frozen = 0,
    #[doc = "1: Channel output valid level after matching"]
    ActiveOnMatch = 1,
    #[doc = "2: Channel output failure level after matching"]
    InactiveOnMatch = 2,
    #[doc = "3: Channel output flipped after matching"]
    Toggle = 3,
    #[doc = "4: Forced failure level output"]
    ForceInactive = 4,
    #[doc = "5: Forced valid level output"]
    ForceActive = 5,
    #[doc = "6: PWM mode 1"]
    Pwm1 = 6,
    #[doc = "7: PWM mode 2"]
    Pwm2 = 7,
}
impl From<Oc1m> for u8 {
    #[inline(always)]
    fn from(variant: Oc1m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oc1m {
    type Ux = u8;
}
impl crate::IsEnum for Oc1m {}
#[doc = "Field `OC1M` reader - Channel 1 output compare mode selection"]
pub type Oc1mR = crate::FieldReader<Oc1m>;
impl Oc1mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc1m {
        match self.bits {
            0 => Oc1m::Frozen,
            1 => Oc1m::ActiveOnMatch,
            2 => Oc1m::InactiveOnMatch,
            3 => Oc1m::Toggle,
            4 => Oc1m::ForceInactive,
            5 => Oc1m::ForceActive,
            6 => Oc1m::Pwm1,
            7 => Oc1m::Pwm2,
            _ => unreachable!(),
        }
    }
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Oc1m::Frozen
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == Oc1m::ActiveOnMatch
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == Oc1m::InactiveOnMatch
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Oc1m::Toggle
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == Oc1m::ForceInactive
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == Oc1m::ForceActive
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Oc1m::Pwm1
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == Oc1m::Pwm2
    }
}
#[doc = "Field `OC1M` writer - Channel 1 output compare mode selection"]
pub type Oc1mW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc1m, crate::Safe>;
impl<'a, REG> Oc1mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::Frozen)
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::ActiveOnMatch)
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::InactiveOnMatch)
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::Toggle)
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::ForceInactive)
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::ForceActive)
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::Pwm1)
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(Oc1m::Pwm2)
    }
}
#[doc = "Field `OC1CE` reader - Channel 1 output compare clear enable"]
pub type Oc1ceR = crate::BitReader;
#[doc = "Field `OC1CE` writer - Channel 1 output compare clear enable"]
pub type Oc1ceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture compare 0 selection"]
    #[inline(always)]
    pub fn cc0s(&self) -> Cc0sR {
        Cc0sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler, active when CC0S selects an input"]
    #[inline(always)]
    pub fn ic0psc(&self) -> Ic0pscR {
        Ic0pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 0 fast output enable"]
    #[inline(always)]
    pub fn oc0fe(&self) -> Oc0feR {
        Oc0feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc0pe(&self) -> Oc0peR {
        Oc0peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Channel 0 input filter, active when CC0S selects an input"]
    #[inline(always)]
    pub fn ic0f(&self) -> Ic0fR {
        Ic0fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Channel 0 output compare mode selection"]
    #[inline(always)]
    pub fn oc0m(&self) -> Oc0mR {
        Oc0mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    pub fn oc0ce(&self) -> Oc0ceR {
        Oc0ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler, active when CC1S selects an input"]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 1 fast output enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Channel 1 input filter, active when CC1S selects an input"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Channel 1 output compare mode selection"]
    #[inline(always)]
    pub fn oc1m(&self) -> Oc1mR {
        Oc1mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> Oc1ceR {
        Oc1ceR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture compare 0 selection"]
    #[inline(always)]
    pub fn cc0s(&mut self) -> Cc0sW<'_, Ccmr1Spec> {
        Cc0sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler, active when CC0S selects an input"]
    #[inline(always)]
    pub fn ic0psc(&mut self) -> Ic0pscW<'_, Ccmr1Spec> {
        Ic0pscW::new(self, 2)
    }
    #[doc = "Bit 2 - Channel 0 fast output enable"]
    #[inline(always)]
    pub fn oc0fe(&mut self) -> Oc0feW<'_, Ccmr1Spec> {
        Oc0feW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc0pe(&mut self) -> Oc0peW<'_, Ccmr1Spec> {
        Oc0peW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Channel 0 input filter, active when CC0S selects an input"]
    #[inline(always)]
    pub fn ic0f(&mut self) -> Ic0fW<'_, Ccmr1Spec> {
        Ic0fW::new(self, 4)
    }
    #[doc = "Bits 4:6 - Channel 0 output compare mode selection"]
    #[inline(always)]
    pub fn oc0m(&mut self) -> Oc0mW<'_, Ccmr1Spec> {
        Oc0mW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    pub fn oc0ce(&mut self) -> Oc0ceW<'_, Ccmr1Spec> {
        Oc0ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1Spec> {
        Cc1sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler, active when CC1S selects an input"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> Ic1pscW<'_, Ccmr1Spec> {
        Ic1pscW::new(self, 10)
    }
    #[doc = "Bit 10 - Channel 1 fast output enable"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<'_, Ccmr1Spec> {
        Oc1feW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, Ccmr1Spec> {
        Oc1peW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Channel 1 input filter, active when CC1S selects an input"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> Ic1fW<'_, Ccmr1Spec> {
        Ic1fW::new(self, 12)
    }
    #[doc = "Bits 12:14 - Channel 1 output compare mode selection"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> Oc1mW<'_, Ccmr1Spec> {
        Oc1mW::new(self, 12)
    }
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> Oc1ceW<'_, Ccmr1Spec> {
        Oc1ceW::new(self, 15)
    }
}
#[doc = "GPTIM capture/compare mode register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1Spec;
impl crate::RegisterSpec for Ccmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1::R`](R) reader structure"]
impl crate::Readable for Ccmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure"]
impl crate::Writable for Ccmr1Spec {
    type Safety = crate::Unsafe;
}
