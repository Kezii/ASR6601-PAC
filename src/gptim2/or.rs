#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Channel 0 remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti0Rmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: xo24m"]
    Xo24m = 1,
    #[doc = "2: xo32m"]
    Xo32m = 2,
    #[doc = "3: rco48m"]
    Rco48m = 3,
    #[doc = "4: xo32k"]
    Xo32k = 4,
    #[doc = "5: rco32k"]
    Rco32k = 5,
    #[doc = "6: mco"]
    Mco = 6,
    #[doc = "7: comp0"]
    Comp0 = 7,
    #[doc = "8: rco3.6m"]
    Rco3_6m = 8,
    #[doc = "9: rtc_alarm1_happen_pulse"]
    RtcAlarm1Pulse = 9,
    #[doc = "10: rtc_alarm0_happen_pulse"]
    RtcAlarm0Pulse = 10,
    #[doc = "11: rtc_cyc_counter_pulse"]
    RtcCycPulse = 11,
}
impl From<Ti0Rmp> for u8 {
    #[inline(always)]
    fn from(variant: Ti0Rmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti0Rmp {
    type Ux = u8;
}
impl crate::IsEnum for Ti0Rmp {}
#[doc = "Field `TI0_RMP` reader - Channel 0 remapping"]
pub type Ti0RmpR = crate::FieldReader<Ti0Rmp>;
impl Ti0RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti0Rmp> {
        match self.bits {
            0 => Some(Ti0Rmp::Iom),
            1 => Some(Ti0Rmp::Xo24m),
            2 => Some(Ti0Rmp::Xo32m),
            3 => Some(Ti0Rmp::Rco48m),
            4 => Some(Ti0Rmp::Xo32k),
            5 => Some(Ti0Rmp::Rco32k),
            6 => Some(Ti0Rmp::Mco),
            7 => Some(Ti0Rmp::Comp0),
            8 => Some(Ti0Rmp::Rco3_6m),
            9 => Some(Ti0Rmp::RtcAlarm1Pulse),
            10 => Some(Ti0Rmp::RtcAlarm0Pulse),
            11 => Some(Ti0Rmp::RtcCycPulse),
            _ => None,
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == Ti0Rmp::Iom
    }
    #[doc = "xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == Ti0Rmp::Xo24m
    }
    #[doc = "xo32m"]
    #[inline(always)]
    pub fn is_xo32m(&self) -> bool {
        *self == Ti0Rmp::Xo32m
    }
    #[doc = "rco48m"]
    #[inline(always)]
    pub fn is_rco48m(&self) -> bool {
        *self == Ti0Rmp::Rco48m
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Ti0Rmp::Xo32k
    }
    #[doc = "rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == Ti0Rmp::Rco32k
    }
    #[doc = "mco"]
    #[inline(always)]
    pub fn is_mco(&self) -> bool {
        *self == Ti0Rmp::Mco
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == Ti0Rmp::Comp0
    }
    #[doc = "rco3.6m"]
    #[inline(always)]
    pub fn is_rco3_6m(&self) -> bool {
        *self == Ti0Rmp::Rco3_6m
    }
    #[doc = "rtc_alarm1_happen_pulse"]
    #[inline(always)]
    pub fn is_rtc_alarm1_pulse(&self) -> bool {
        *self == Ti0Rmp::RtcAlarm1Pulse
    }
    #[doc = "rtc_alarm0_happen_pulse"]
    #[inline(always)]
    pub fn is_rtc_alarm0_pulse(&self) -> bool {
        *self == Ti0Rmp::RtcAlarm0Pulse
    }
    #[doc = "rtc_cyc_counter_pulse"]
    #[inline(always)]
    pub fn is_rtc_cyc_pulse(&self) -> bool {
        *self == Ti0Rmp::RtcCycPulse
    }
}
#[doc = "Field `TI0_RMP` writer - Channel 0 remapping"]
pub type Ti0RmpW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ti0Rmp>;
impl<'a, REG> Ti0RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Iom)
    }
    #[doc = "xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Xo24m)
    }
    #[doc = "xo32m"]
    #[inline(always)]
    pub fn xo32m(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Xo32m)
    }
    #[doc = "rco48m"]
    #[inline(always)]
    pub fn rco48m(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Rco48m)
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Xo32k)
    }
    #[doc = "rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Rco32k)
    }
    #[doc = "mco"]
    #[inline(always)]
    pub fn mco(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Mco)
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Comp0)
    }
    #[doc = "rco3.6m"]
    #[inline(always)]
    pub fn rco3_6m(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Rco3_6m)
    }
    #[doc = "rtc_alarm1_happen_pulse"]
    #[inline(always)]
    pub fn rtc_alarm1_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::RtcAlarm1Pulse)
    }
    #[doc = "rtc_alarm0_happen_pulse"]
    #[inline(always)]
    pub fn rtc_alarm0_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::RtcAlarm0Pulse)
    }
    #[doc = "rtc_cyc_counter_pulse"]
    #[inline(always)]
    pub fn rtc_cyc_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::RtcCycPulse)
    }
}
#[doc = "Channel 1 remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti1Rmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: comp1"]
    Comp1 = 1,
}
impl From<Ti1Rmp> for u8 {
    #[inline(always)]
    fn from(variant: Ti1Rmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti1Rmp {
    type Ux = u8;
}
impl crate::IsEnum for Ti1Rmp {}
#[doc = "Field `TI1_RMP` reader - Channel 1 remapping"]
pub type Ti1RmpR = crate::FieldReader<Ti1Rmp>;
impl Ti1RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti1Rmp> {
        match self.bits {
            0 => Some(Ti1Rmp::Iom),
            1 => Some(Ti1Rmp::Comp1),
            _ => None,
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == Ti1Rmp::Iom
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == Ti1Rmp::Comp1
    }
}
#[doc = "Field `TI1_RMP` writer - Channel 1 remapping"]
pub type Ti1RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ti1Rmp>;
impl<'a, REG> Ti1RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1Rmp::Iom)
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1Rmp::Comp1)
    }
}
#[doc = "ETR remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EtrRmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: comp0"]
    Comp0 = 1,
    #[doc = "2: comp1"]
    Comp1 = 2,
    #[doc = "3: xo32k"]
    Xo32k = 3,
}
impl From<EtrRmp> for u8 {
    #[inline(always)]
    fn from(variant: EtrRmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EtrRmp {
    type Ux = u8;
}
impl crate::IsEnum for EtrRmp {}
#[doc = "Field `ETR_RMP` reader - ETR remapping"]
pub type EtrRmpR = crate::FieldReader<EtrRmp>;
impl EtrRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EtrRmp> {
        match self.bits {
            0 => Some(EtrRmp::Iom),
            1 => Some(EtrRmp::Comp0),
            2 => Some(EtrRmp::Comp1),
            3 => Some(EtrRmp::Xo32k),
            _ => None,
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == EtrRmp::Iom
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EtrRmp::Comp0
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EtrRmp::Comp1
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == EtrRmp::Xo32k
    }
}
#[doc = "Field `ETR_RMP` writer - ETR remapping"]
pub type EtrRmpW<'a, REG> = crate::FieldWriter<'a, REG, 3, EtrRmp>;
impl<'a, REG> EtrRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Iom)
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Comp0)
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Comp1)
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Xo32k)
    }
}
impl R {
    #[doc = "Bits 0:4 - Channel 0 remapping"]
    #[inline(always)]
    pub fn ti0_rmp(&self) -> Ti0RmpR {
        Ti0RmpR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Channel 1 remapping"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> Ti1RmpR {
        Ti1RmpR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - ETR remapping"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> EtrRmpR {
        EtrRmpR::new(((self.bits >> 7) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Channel 0 remapping"]
    #[inline(always)]
    pub fn ti0_rmp(&mut self) -> Ti0RmpW<'_, OrSpec> {
        Ti0RmpW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Channel 1 remapping"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> Ti1RmpW<'_, OrSpec> {
        Ti1RmpW::new(self, 5)
    }
    #[doc = "Bits 7:9 - ETR remapping"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> EtrRmpW<'_, OrSpec> {
        EtrRmpW::new(self, 7)
    }
}
#[doc = "GPTIM option register (GPTIM2 remap layout)\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
