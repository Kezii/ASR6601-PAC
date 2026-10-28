#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Encoder mode control"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckpol {
    #[doc = "0: Encoder mode 1, rising edge counting"]
    EncoderMode1 = 0,
    #[doc = "1: Encoder mode 2, falling edge counting"]
    EncoderMode2 = 1,
    #[doc = "2: Encoder mode 3, double-edge counting"]
    EncoderMode3 = 2,
}
impl From<Ckpol> for u8 {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckpol {
    type Ux = u8;
}
impl crate::IsEnum for Ckpol {}
#[doc = "Field `CKPOL` reader - Encoder mode control"]
pub type CkpolR = crate::FieldReader<Ckpol>;
impl CkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckpol> {
        match self.bits {
            0 => Some(Ckpol::EncoderMode1),
            1 => Some(Ckpol::EncoderMode2),
            2 => Some(Ckpol::EncoderMode3),
            _ => None,
        }
    }
    #[doc = "Encoder mode 1, rising edge counting"]
    #[inline(always)]
    pub fn is_encoder_mode_1(&self) -> bool {
        *self == Ckpol::EncoderMode1
    }
    #[doc = "Encoder mode 2, falling edge counting"]
    #[inline(always)]
    pub fn is_encoder_mode_2(&self) -> bool {
        *self == Ckpol::EncoderMode2
    }
    #[doc = "Encoder mode 3, double-edge counting"]
    #[inline(always)]
    pub fn is_encoder_mode_3(&self) -> bool {
        *self == Ckpol::EncoderMode3
    }
}
#[doc = "Field `CKPOL` writer - Encoder mode control"]
pub type CkpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckpol>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encoder mode 1, rising edge counting"]
    #[inline(always)]
    pub fn encoder_mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::EncoderMode1)
    }
    #[doc = "Encoder mode 2, falling edge counting"]
    #[inline(always)]
    pub fn encoder_mode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::EncoderMode2)
    }
    #[doc = "Encoder mode 3, double-edge counting"]
    #[inline(always)]
    pub fn encoder_mode_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::EncoderMode3)
    }
}
#[doc = "Field `CKFLT` reader - External clock filter configuration"]
pub type CkfltR = crate::FieldReader;
#[doc = "Field `CKFLT` writer - External clock filter configuration"]
pub type CkfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKFLT_EN` reader - Enable the external clock filter"]
pub type CkfltEnR = crate::BitReader;
#[doc = "Field `CKFLT_EN` writer - Enable the external clock filter"]
pub type CkfltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGFLT` reader - Trigger input filter configuration"]
pub type TrgfltR = crate::FieldReader;
#[doc = "Field `TRGFLT` writer - Trigger input filter configuration"]
pub type TrgfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGFLT_EN` reader - Enable the trigger input filter"]
pub type TrgfltEnR = crate::BitReader;
#[doc = "Field `TRGFLT_EN` writer - Enable the trigger input filter"]
pub type TrgfltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock frequency division"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: Divide by 1"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 8"]
    Div8 = 3,
    #[doc = "4: Divide by 16"]
    Div16 = 4,
    #[doc = "5: Divide by 32"]
    Div32 = 5,
    #[doc = "6: Divide by 64"]
    Div64 = 6,
    #[doc = "7: Divide by 128"]
    Div128 = 7,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Clock frequency division"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presc {
        match self.bits {
            0 => Presc::Div1,
            1 => Presc::Div2,
            2 => Presc::Div4,
            3 => Presc::Div8,
            4 => Presc::Div16,
            5 => Presc::Div32,
            6 => Presc::Div64,
            7 => Presc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Presc::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Presc::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Presc::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Presc::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Presc::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Presc::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Presc::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Presc::Div128
    }
}
#[doc = "Field `PRESC` writer - Clock frequency division"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presc, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div128)
    }
}
#[doc = "External trigger source selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: lptim_ext_trig0"]
    Trig0 = 0,
    #[doc = "1: lptim_ext_trig1"]
    Trig1 = 1,
    #[doc = "2: lptim_ext_trig2"]
    Trig2 = 2,
    #[doc = "3: lptim_ext_trig3"]
    Trig3 = 3,
    #[doc = "4: lptim_ext_trig4"]
    Trig4 = 4,
    #[doc = "5: lptim_ext_trig5"]
    Trig5 = 5,
    #[doc = "6: lptim_ext_trig6"]
    Trig6 = 6,
    #[doc = "7: lptim_ext_trig7"]
    Trig7 = 7,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - External trigger source selection"]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsel {
        match self.bits {
            0 => Trigsel::Trig0,
            1 => Trigsel::Trig1,
            2 => Trigsel::Trig2,
            3 => Trigsel::Trig3,
            4 => Trigsel::Trig4,
            5 => Trigsel::Trig5,
            6 => Trigsel::Trig6,
            7 => Trigsel::Trig7,
            _ => unreachable!(),
        }
    }
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Trigsel::Trig0
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == Trigsel::Trig1
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == Trigsel::Trig2
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == Trigsel::Trig3
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == Trigsel::Trig4
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == Trigsel::Trig5
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == Trigsel::Trig6
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == Trigsel::Trig7
    }
}
#[doc = "Field `TRIGSEL` writer - External trigger source selection"]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigsel, crate::Safe>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig0)
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig1)
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig2)
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig3)
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig4)
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig5)
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig6)
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trig7)
    }
}
#[doc = "External trigger enable and polarity selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigen {
    #[doc = "0: Software trigger"]
    Software = 0,
    #[doc = "1: External trigger rising edge"]
    Rising = 1,
    #[doc = "2: External trigger falling edge"]
    Falling = 2,
    #[doc = "3: External trigger double edge"]
    Both = 3,
}
impl From<Trigen> for u8 {
    #[inline(always)]
    fn from(variant: Trigen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigen {
    type Ux = u8;
}
impl crate::IsEnum for Trigen {}
#[doc = "Field `TRIGEN` reader - External trigger enable and polarity selection"]
pub type TrigenR = crate::FieldReader<Trigen>;
impl TrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigen {
        match self.bits {
            0 => Trigen::Software,
            1 => Trigen::Rising,
            2 => Trigen::Falling,
            3 => Trigen::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Trigen::Software
    }
    #[doc = "External trigger rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Trigen::Rising
    }
    #[doc = "External trigger falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Trigen::Falling
    }
    #[doc = "External trigger double edge"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Trigen::Both
    }
}
#[doc = "Field `TRIGEN` writer - External trigger enable and polarity selection"]
pub type TrigenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigen, crate::Safe>;
impl<'a, REG> TrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Software)
    }
    #[doc = "External trigger rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Rising)
    }
    #[doc = "External trigger falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Falling)
    }
    #[doc = "External trigger double edge"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Both)
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - Wave"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Wave"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVPOL` reader - Wavpol"]
pub type WavpolR = crate::BitReader;
#[doc = "Field `WAVPOL` writer - Wavpol"]
pub type WavpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELOAD` reader - Preload"]
pub type PreloadR = crate::BitReader;
#[doc = "Field `PRELOAD` writer - Preload"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTMODE` reader - Countmode"]
pub type CountmodeR = crate::BitReader;
#[doc = "Field `COUNTMODE` writer - Countmode"]
pub type CountmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - Enc"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - Enc"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM_WKUP` reader - Cmpm wkup"]
pub type CmpmWkupR = crate::BitReader;
#[doc = "Field `CMPM_WKUP` writer - Cmpm wkup"]
pub type CmpmWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRM_WKUP` reader - Arrm wkup"]
pub type ArrmWkupR = crate::BitReader;
#[doc = "Field `ARRM_WKUP` writer - Arrm wkup"]
pub type ArrmWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIG_WKUP` reader - Exttrig wkup"]
pub type ExttrigWkupR = crate::BitReader;
#[doc = "Field `EXTTRIG_WKUP` writer - Exttrig wkup"]
pub type ExttrigWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP_WKUP` reader - Up wkup"]
pub type UpWkupR = crate::BitReader;
#[doc = "Field `UP_WKUP` writer - Up wkup"]
pub type UpWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN_WKUP` reader - Down wkup"]
pub type DownWkupR = crate::BitReader;
#[doc = "Field `DOWN_WKUP` writer - Down wkup"]
pub type DownWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_WKUP` reader - Out wkup"]
pub type OutWkupR = crate::BitReader;
#[doc = "Field `OUT_WKUP` writer - Out wkup"]
pub type OutWkupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2 - Encoder mode control"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - External clock filter configuration"]
    #[inline(always)]
    pub fn ckflt(&self) -> CkfltR {
        CkfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Enable the external clock filter"]
    #[inline(always)]
    pub fn ckflt_en(&self) -> CkfltEnR {
        CkfltEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trigger input filter configuration"]
    #[inline(always)]
    pub fn trgflt(&self) -> TrgfltR {
        TrgfltR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable the trigger input filter"]
    #[inline(always)]
    pub fn trgflt_en(&self) -> TrgfltEnR {
        TrgfltEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Clock frequency division"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - External trigger source selection"]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - External trigger enable and polarity selection"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wave"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wavpol"]
    #[inline(always)]
    pub fn wavpol(&self) -> WavpolR {
        WavpolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Preload"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Countmode"]
    #[inline(always)]
    pub fn countmode(&self) -> CountmodeR {
        CountmodeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enc"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cmpm wkup"]
    #[inline(always)]
    pub fn cmpm_wkup(&self) -> CmpmWkupR {
        CmpmWkupR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Arrm wkup"]
    #[inline(always)]
    pub fn arrm_wkup(&self) -> ArrmWkupR {
        ArrmWkupR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Exttrig wkup"]
    #[inline(always)]
    pub fn exttrig_wkup(&self) -> ExttrigWkupR {
        ExttrigWkupR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Up wkup"]
    #[inline(always)]
    pub fn up_wkup(&self) -> UpWkupR {
        UpWkupR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Down wkup"]
    #[inline(always)]
    pub fn down_wkup(&self) -> DownWkupR {
        DownWkupR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Out wkup"]
    #[inline(always)]
    pub fn out_wkup(&self) -> OutWkupR {
        OutWkupR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - Encoder mode control"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<'_, CfgrSpec> {
        CkpolW::new(self, 1)
    }
    #[doc = "Bits 3:4 - External clock filter configuration"]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CkfltW<'_, CfgrSpec> {
        CkfltW::new(self, 3)
    }
    #[doc = "Bit 5 - Enable the external clock filter"]
    #[inline(always)]
    pub fn ckflt_en(&mut self) -> CkfltEnW<'_, CfgrSpec> {
        CkfltEnW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trigger input filter configuration"]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TrgfltW<'_, CfgrSpec> {
        TrgfltW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable the trigger input filter"]
    #[inline(always)]
    pub fn trgflt_en(&mut self) -> TrgfltEnW<'_, CfgrSpec> {
        TrgfltEnW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Clock frequency division"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CfgrSpec> {
        PrescW::new(self, 9)
    }
    #[doc = "Bits 13:15 - External trigger source selection"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<'_, CfgrSpec> {
        TrigselW::new(self, 13)
    }
    #[doc = "Bits 17:18 - External trigger enable and polarity selection"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<'_, CfgrSpec> {
        TrigenW::new(self, 17)
    }
    #[doc = "Bit 19 - Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, CfgrSpec> {
        TimeoutW::new(self, 19)
    }
    #[doc = "Bit 20 - Wave"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, CfgrSpec> {
        WaveW::new(self, 20)
    }
    #[doc = "Bit 21 - Wavpol"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WavpolW<'_, CfgrSpec> {
        WavpolW::new(self, 21)
    }
    #[doc = "Bit 22 - Preload"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<'_, CfgrSpec> {
        PreloadW::new(self, 22)
    }
    #[doc = "Bit 23 - Countmode"]
    #[inline(always)]
    pub fn countmode(&mut self) -> CountmodeW<'_, CfgrSpec> {
        CountmodeW::new(self, 23)
    }
    #[doc = "Bit 24 - Enc"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, CfgrSpec> {
        EncW::new(self, 24)
    }
    #[doc = "Bit 25 - Cmpm wkup"]
    #[inline(always)]
    pub fn cmpm_wkup(&mut self) -> CmpmWkupW<'_, CfgrSpec> {
        CmpmWkupW::new(self, 25)
    }
    #[doc = "Bit 26 - Arrm wkup"]
    #[inline(always)]
    pub fn arrm_wkup(&mut self) -> ArrmWkupW<'_, CfgrSpec> {
        ArrmWkupW::new(self, 26)
    }
    #[doc = "Bit 27 - Exttrig wkup"]
    #[inline(always)]
    pub fn exttrig_wkup(&mut self) -> ExttrigWkupW<'_, CfgrSpec> {
        ExttrigWkupW::new(self, 27)
    }
    #[doc = "Bit 28 - Up wkup"]
    #[inline(always)]
    pub fn up_wkup(&mut self) -> UpWkupW<'_, CfgrSpec> {
        UpWkupW::new(self, 28)
    }
    #[doc = "Bit 29 - Down wkup"]
    #[inline(always)]
    pub fn down_wkup(&mut self) -> DownWkupW<'_, CfgrSpec> {
        DownWkupW::new(self, 29)
    }
    #[doc = "Bit 30 - Out wkup"]
    #[inline(always)]
    pub fn out_wkup(&mut self) -> OutWkupW<'_, CfgrSpec> {
        OutWkupW::new(self, 30)
    }
}
#[doc = "LPTIM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
