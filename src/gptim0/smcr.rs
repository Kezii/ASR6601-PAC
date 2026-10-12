#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Slave mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sms {
    #[doc = "0: Slave mode disabled"]
    Disabled = 0,
    #[doc = "1: Encoder mode 1"]
    Encoder1 = 1,
    #[doc = "2: Encoder mode 2"]
    Encoder2 = 2,
    #[doc = "3: Encoder mode 3"]
    Encoder3 = 3,
    #[doc = "4: Reset mode, TRGI rising edge resets counter"]
    ResetMode = 4,
    #[doc = "5: Gated mode"]
    Gated = 5,
    #[doc = "6: Trigger mode"]
    Trigger = 6,
    #[doc = "7: External clock mode 1"]
    External1 = 7,
}
impl From<Sms> for u8 {
    #[inline(always)]
    fn from(variant: Sms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sms {
    type Ux = u8;
}
impl crate::IsEnum for Sms {}
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SmsR = crate::FieldReader<Sms>;
impl SmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sms {
        match self.bits {
            0 => Sms::Disabled,
            1 => Sms::Encoder1,
            2 => Sms::Encoder2,
            3 => Sms::Encoder3,
            4 => Sms::ResetMode,
            5 => Sms::Gated,
            6 => Sms::Trigger,
            7 => Sms::External1,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sms::Disabled
    }
    #[doc = "Encoder mode 1"]
    #[inline(always)]
    pub fn is_encoder1(&self) -> bool {
        *self == Sms::Encoder1
    }
    #[doc = "Encoder mode 2"]
    #[inline(always)]
    pub fn is_encoder2(&self) -> bool {
        *self == Sms::Encoder2
    }
    #[doc = "Encoder mode 3"]
    #[inline(always)]
    pub fn is_encoder3(&self) -> bool {
        *self == Sms::Encoder3
    }
    #[doc = "Reset mode, TRGI rising edge resets counter"]
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == Sms::ResetMode
    }
    #[doc = "Gated mode"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Sms::Gated
    }
    #[doc = "Trigger mode"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Sms::Trigger
    }
    #[doc = "External clock mode 1"]
    #[inline(always)]
    pub fn is_external1(&self) -> bool {
        *self == Sms::External1
    }
}
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sms, crate::Safe>;
impl<'a, REG> SmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Disabled)
    }
    #[doc = "Encoder mode 1"]
    #[inline(always)]
    pub fn encoder1(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Encoder1)
    }
    #[doc = "Encoder mode 2"]
    #[inline(always)]
    pub fn encoder2(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Encoder2)
    }
    #[doc = "Encoder mode 3"]
    #[inline(always)]
    pub fn encoder3(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Encoder3)
    }
    #[doc = "Reset mode, TRGI rising edge resets counter"]
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::ResetMode)
    }
    #[doc = "Gated mode"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Gated)
    }
    #[doc = "Trigger mode"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Trigger)
    }
    #[doc = "External clock mode 1"]
    #[inline(always)]
    pub fn external1(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::External1)
    }
}
#[doc = "Trigger source selection, TRGI source"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts {
    #[doc = "0: Internal trigger 0"]
    Itr0 = 0,
    #[doc = "1: Internal trigger 1"]
    Itr1 = 1,
    #[doc = "2: Internal trigger 2"]
    Itr2 = 2,
    #[doc = "4: TI0 edge detector"]
    Ti0fEd = 4,
    #[doc = "5: Filtered timer input 0"]
    Ti0fp0 = 5,
    #[doc = "6: Filtered timer input 1"]
    Ti1fp1 = 6,
    #[doc = "7: Filtered external trigger input"]
    Etrf = 7,
}
impl From<Ts> for u8 {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts {
    type Ux = u8;
}
impl crate::IsEnum for Ts {}
#[doc = "Field `TS` reader - Trigger source selection, TRGI source"]
pub type TsR = crate::FieldReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ts> {
        match self.bits {
            0 => Some(Ts::Itr0),
            1 => Some(Ts::Itr1),
            2 => Some(Ts::Itr2),
            4 => Some(Ts::Ti0fEd),
            5 => Some(Ts::Ti0fp0),
            6 => Some(Ts::Ti1fp1),
            7 => Some(Ts::Etrf),
            _ => None,
        }
    }
    #[doc = "Internal trigger 0"]
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == Ts::Itr0
    }
    #[doc = "Internal trigger 1"]
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == Ts::Itr1
    }
    #[doc = "Internal trigger 2"]
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == Ts::Itr2
    }
    #[doc = "TI0 edge detector"]
    #[inline(always)]
    pub fn is_ti0f_ed(&self) -> bool {
        *self == Ts::Ti0fEd
    }
    #[doc = "Filtered timer input 0"]
    #[inline(always)]
    pub fn is_ti0fp0(&self) -> bool {
        *self == Ts::Ti0fp0
    }
    #[doc = "Filtered timer input 1"]
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == Ts::Ti1fp1
    }
    #[doc = "Filtered external trigger input"]
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == Ts::Etrf
    }
}
#[doc = "Field `TS` writer - Trigger source selection, TRGI source"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ts>;
impl<'a, REG> TsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal trigger 0"]
    #[inline(always)]
    pub fn itr0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Itr0)
    }
    #[doc = "Internal trigger 1"]
    #[inline(always)]
    pub fn itr1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Itr1)
    }
    #[doc = "Internal trigger 2"]
    #[inline(always)]
    pub fn itr2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Itr2)
    }
    #[doc = "TI0 edge detector"]
    #[inline(always)]
    pub fn ti0f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Ti0fEd)
    }
    #[doc = "Filtered timer input 0"]
    #[inline(always)]
    pub fn ti0fp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Ti0fp0)
    }
    #[doc = "Filtered timer input 1"]
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Ti1fp1)
    }
    #[doc = "Filtered external trigger input"]
    #[inline(always)]
    pub fn etrf(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Etrf)
    }
}
#[doc = "Field `MSM` reader - Master-slave mode synchronization"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - Master-slave mode synchronization"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - External trigger input filter"]
pub type EtfR = crate::FieldReader;
#[doc = "Field `ETF` writer - External trigger input filter"]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "External trigger input prescaler"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etps {
    #[doc = "0: No frequency division"]
    Div1 = 0,
    #[doc = "1: Frequency divided by 2"]
    Div2 = 1,
    #[doc = "2: Frequency divided by 4"]
    Div4 = 2,
    #[doc = "3: Frequency divided by 8"]
    Div8 = 3,
}
impl From<Etps> for u8 {
    #[inline(always)]
    fn from(variant: Etps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etps {
    type Ux = u8;
}
impl crate::IsEnum for Etps {}
#[doc = "Field `ETPS` reader - External trigger input prescaler"]
pub type EtpsR = crate::FieldReader<Etps>;
impl EtpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etps {
        match self.bits {
            0 => Etps::Div1,
            1 => Etps::Div2,
            2 => Etps::Div4,
            3 => Etps::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "No frequency division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Etps::Div1
    }
    #[doc = "Frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Etps::Div2
    }
    #[doc = "Frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Etps::Div4
    }
    #[doc = "Frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Etps::Div8
    }
}
#[doc = "Field `ETPS` writer - External trigger input prescaler"]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Etps, crate::Safe>;
impl<'a, REG> EtpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No frequency division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::Div1)
    }
    #[doc = "Frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::Div2)
    }
    #[doc = "Frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::Div4)
    }
    #[doc = "Frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Etps::Div8)
    }
}
#[doc = "Field `ECE` reader - External clock mode 2 enable"]
pub type EceR = crate::BitReader;
#[doc = "Field `ECE` writer - External clock mode 2 enable"]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity selection"]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity selection"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger source selection, TRGI source"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master-slave mode synchronization"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger input filter"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger input prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock mode 2 enable"]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity selection"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<'_, SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger source selection, TRGI source"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, SmcrSpec> {
        TsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master-slave mode synchronization"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger input filter"]
    #[inline(always)]
    pub fn etf(&mut self) -> EtfW<'_, SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger input prescaler"]
    #[inline(always)]
    pub fn etps(&mut self) -> EtpsW<'_, SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14 - External clock mode 2 enable"]
    #[inline(always)]
    pub fn ece(&mut self) -> EceW<'_, SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity selection"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, SmcrSpec> {
        EtpW::new(self, 15)
    }
}
#[doc = "GPTIM slave Mode Control register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
