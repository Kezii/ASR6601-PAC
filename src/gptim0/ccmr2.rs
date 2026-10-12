#[doc = "Register `CCMR2` reader"]
pub type R = crate::R<Ccmr2Spec>;
#[doc = "Register `CCMR2` writer"]
pub type W = crate::W<Ccmr2Spec>;
#[doc = "Capture compare 2 selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2s {
    #[doc = "0: Channel configured in output mode"]
    Output = 0,
    #[doc = "1: Input mode, capture input mapped to channel 1"]
    InputTi1 = 1,
    #[doc = "2: Input mode, capture input mapped to channel 0"]
    InputTi0 = 2,
    #[doc = "3: Input mode, capture input mapped to trigger input"]
    InputTrc = 3,
}
impl From<Cc2s> for u8 {
    #[inline(always)]
    fn from(variant: Cc2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2s {
    type Ux = u8;
}
impl crate::IsEnum for Cc2s {}
#[doc = "Field `CC2S` reader - Capture compare 2 selection"]
pub type Cc2sR = crate::FieldReader<Cc2s>;
impl Cc2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2s {
        match self.bits {
            0 => Cc2s::Output,
            1 => Cc2s::InputTi1,
            2 => Cc2s::InputTi0,
            3 => Cc2s::InputTrc,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Cc2s::Output
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn is_input_ti1(&self) -> bool {
        *self == Cc2s::InputTi1
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn is_input_ti0(&self) -> bool {
        *self == Cc2s::InputTi0
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn is_input_trc(&self) -> bool {
        *self == Cc2s::InputTrc
    }
}
#[doc = "Field `CC2S` writer - Capture compare 2 selection"]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2s, crate::Safe>;
impl<'a, REG> Cc2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::Output)
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn input_ti1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::InputTi1)
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn input_ti0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::InputTi0)
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn input_trc(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::InputTrc)
    }
}
#[doc = "Field `OC2FE` reader - Channel 2 fast output enable"]
pub type Oc2feR = crate::BitReader;
#[doc = "Field `OC2FE` writer - Channel 2 fast output enable"]
pub type Oc2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Channel 2 output compare shadow register enable"]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - Channel 2 output compare shadow register enable"]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 2 output compare mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc2m {
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
impl From<Oc2m> for u8 {
    #[inline(always)]
    fn from(variant: Oc2m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oc2m {
    type Ux = u8;
}
impl crate::IsEnum for Oc2m {}
#[doc = "Field `OC2M` reader - Channel 2 output compare mode selection"]
pub type Oc2mR = crate::FieldReader<Oc2m>;
impl Oc2mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc2m {
        match self.bits {
            0 => Oc2m::Frozen,
            1 => Oc2m::ActiveOnMatch,
            2 => Oc2m::InactiveOnMatch,
            3 => Oc2m::Toggle,
            4 => Oc2m::ForceInactive,
            5 => Oc2m::ForceActive,
            6 => Oc2m::Pwm1,
            7 => Oc2m::Pwm2,
            _ => unreachable!(),
        }
    }
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Oc2m::Frozen
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == Oc2m::ActiveOnMatch
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == Oc2m::InactiveOnMatch
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Oc2m::Toggle
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == Oc2m::ForceInactive
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == Oc2m::ForceActive
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Oc2m::Pwm1
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == Oc2m::Pwm2
    }
}
#[doc = "Field `OC2M` writer - Channel 2 output compare mode selection"]
pub type Oc2mW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc2m, crate::Safe>;
impl<'a, REG> Oc2mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::Frozen)
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::ActiveOnMatch)
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::InactiveOnMatch)
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::Toggle)
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::ForceInactive)
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::ForceActive)
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::Pwm1)
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(Oc2m::Pwm2)
    }
}
#[doc = "Field `OC2CE` reader - Channel 2 output compare clear enable"]
pub type Oc2ceR = crate::BitReader;
#[doc = "Field `OC2CE` writer - Channel 2 output compare clear enable"]
pub type Oc2ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture compare 3 selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc3s {
    #[doc = "0: Channel configured in output mode"]
    Output = 0,
    #[doc = "1: Input mode, capture input mapped to channel 1"]
    InputTi1 = 1,
    #[doc = "2: Input mode, capture input mapped to channel 0"]
    InputTi0 = 2,
    #[doc = "3: Input mode, capture input mapped to trigger input"]
    InputTrc = 3,
}
impl From<Cc3s> for u8 {
    #[inline(always)]
    fn from(variant: Cc3s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc3s {
    type Ux = u8;
}
impl crate::IsEnum for Cc3s {}
#[doc = "Field `CC3S` reader - Capture compare 3 selection"]
pub type Cc3sR = crate::FieldReader<Cc3s>;
impl Cc3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3s {
        match self.bits {
            0 => Cc3s::Output,
            1 => Cc3s::InputTi1,
            2 => Cc3s::InputTi0,
            3 => Cc3s::InputTrc,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Cc3s::Output
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn is_input_ti1(&self) -> bool {
        *self == Cc3s::InputTi1
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn is_input_ti0(&self) -> bool {
        *self == Cc3s::InputTi0
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn is_input_trc(&self) -> bool {
        *self == Cc3s::InputTrc
    }
}
#[doc = "Field `CC3S` writer - Capture compare 3 selection"]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc3s, crate::Safe>;
impl<'a, REG> Cc3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel configured in output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::Output)
    }
    #[doc = "Input mode, capture input mapped to channel 1"]
    #[inline(always)]
    pub fn input_ti1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::InputTi1)
    }
    #[doc = "Input mode, capture input mapped to channel 0"]
    #[inline(always)]
    pub fn input_ti0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::InputTi0)
    }
    #[doc = "Input mode, capture input mapped to trigger input"]
    #[inline(always)]
    pub fn input_trc(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::InputTrc)
    }
}
#[doc = "Field `OC3FE` reader - Channel 3 fast output enable"]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - Channel 3 fast output enable"]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Channel 3 output compare shadow register enable"]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - Channel 3 output compare shadow register enable"]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 3 output compare mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oc3m {
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
impl From<Oc3m> for u8 {
    #[inline(always)]
    fn from(variant: Oc3m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oc3m {
    type Ux = u8;
}
impl crate::IsEnum for Oc3m {}
#[doc = "Field `OC3M` reader - Channel 3 output compare mode selection"]
pub type Oc3mR = crate::FieldReader<Oc3m>;
impl Oc3mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oc3m {
        match self.bits {
            0 => Oc3m::Frozen,
            1 => Oc3m::ActiveOnMatch,
            2 => Oc3m::InactiveOnMatch,
            3 => Oc3m::Toggle,
            4 => Oc3m::ForceInactive,
            5 => Oc3m::ForceActive,
            6 => Oc3m::Pwm1,
            7 => Oc3m::Pwm2,
            _ => unreachable!(),
        }
    }
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Oc3m::Frozen
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == Oc3m::ActiveOnMatch
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == Oc3m::InactiveOnMatch
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Oc3m::Toggle
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == Oc3m::ForceInactive
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == Oc3m::ForceActive
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Oc3m::Pwm1
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == Oc3m::Pwm2
    }
}
#[doc = "Field `OC3M` writer - Channel 3 output compare mode selection"]
pub type Oc3mW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oc3m, crate::Safe>;
impl<'a, REG> Oc3mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freeze mode, channel output does not change"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::Frozen)
    }
    #[doc = "Channel output valid level after matching"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::ActiveOnMatch)
    }
    #[doc = "Channel output failure level after matching"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::InactiveOnMatch)
    }
    #[doc = "Channel output flipped after matching"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::Toggle)
    }
    #[doc = "Forced failure level output"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::ForceInactive)
    }
    #[doc = "Forced valid level output"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::ForceActive)
    }
    #[doc = "PWM mode 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::Pwm1)
    }
    #[doc = "PWM mode 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(Oc3m::Pwm2)
    }
}
#[doc = "Field `OC3CE` reader - Channel 3 output compare clear enable"]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - Channel 3 output compare clear enable"]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 2 fast output enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> Oc2feR {
        Oc2feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 2 output compare mode selection"]
    #[inline(always)]
    pub fn oc2m(&self) -> Oc2mR {
        Oc2mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> Oc2ceR {
        Oc2ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 3 fast output enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 3 output compare mode selection"]
    #[inline(always)]
    pub fn oc3m(&self) -> Oc3mR {
        Oc3mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> Cc2sW<'_, Ccmr2Spec> {
        Cc2sW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 2 fast output enable"]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> Oc2feW<'_, Ccmr2Spec> {
        Oc2feW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> Oc2peW<'_, Ccmr2Spec> {
        Oc2peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 2 output compare mode selection"]
    #[inline(always)]
    pub fn oc2m(&mut self) -> Oc2mW<'_, Ccmr2Spec> {
        Oc2mW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn oc2ce(&mut self) -> Oc2ceW<'_, Ccmr2Spec> {
        Oc2ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2Spec> {
        Cc3sW::new(self, 8)
    }
    #[doc = "Bit 10 - Channel 3 fast output enable"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<'_, Ccmr2Spec> {
        Oc3feW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow register enable"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, Ccmr2Spec> {
        Oc3peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Channel 3 output compare mode selection"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> Oc3mW<'_, Ccmr2Spec> {
        Oc3mW::new(self, 12)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<'_, Ccmr2Spec> {
        Oc3ceW::new(self, 15)
    }
}
#[doc = "GPTIM capture/compare mode register 2, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2Spec;
impl crate::RegisterSpec for Ccmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2::R`](R) reader structure"]
impl crate::Readable for Ccmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure"]
impl crate::Writable for Ccmr2Spec {
    type Safety = crate::Unsafe;
}
