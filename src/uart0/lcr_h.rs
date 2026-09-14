#[doc = "Register `LCR_H` reader"]
pub type R = crate::R<LcrHSpec>;
#[doc = "Register `LCR_H` writer"]
pub type W = crate::W<LcrHSpec>;
#[doc = "Field `BRK` reader - send break"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - send break"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Pen"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Pen"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS_EVEN` reader - Eps even"]
pub type EpsEvenR = crate::BitReader;
#[doc = "Field `EPS_EVEN` writer - Eps even"]
pub type EpsEvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Stop"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "1: Value 2"]
    Value2 = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::Value1,
            true => Stop::Value2,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Stop::Value1
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Stop::Value2
    }
}
#[doc = "Field `STOP` writer - Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Value1)
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Value2)
    }
}
#[doc = "Field `FEN` reader - Fen"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - Fen"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Wlen"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "0: Value 5"]
    Value5 = 0,
    #[doc = "1: Value 6"]
    Value6 = 1,
    #[doc = "2: Value 7"]
    Value7 = 2,
    #[doc = "3: Value 8"]
    Value8 = 3,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
#[doc = "Field `WLEN` reader - Wlen"]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wlen {
        match self.bits {
            0 => Wlen::Value5,
            1 => Wlen::Value6,
            2 => Wlen::Value7,
            3 => Wlen::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "Value 5"]
    #[inline(always)]
    pub fn is_value_5(&self) -> bool {
        *self == Wlen::Value5
    }
    #[doc = "Value 6"]
    #[inline(always)]
    pub fn is_value_6(&self) -> bool {
        *self == Wlen::Value6
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == Wlen::Value7
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == Wlen::Value8
    }
}
#[doc = "Field `WLEN` writer - Wlen"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 5"]
    #[inline(always)]
    pub fn value_5(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Value5)
    }
    #[doc = "Value 6"]
    #[inline(always)]
    pub fn value_6(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Value6)
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Value7)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Value8)
    }
}
impl R {
    #[doc = "Bit 0 - send break"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pen"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Eps even"]
    #[inline(always)]
    pub fn eps_even(&self) -> EpsEvenR {
        EpsEvenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fen"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Wlen"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - send break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, LcrHSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - Pen"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrHSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - Eps even"]
    #[inline(always)]
    pub fn eps_even(&mut self) -> EpsEvenW<'_, LcrHSpec> {
        EpsEvenW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, LcrHSpec> {
        StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Fen"]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<'_, LcrHSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Wlen"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<'_, LcrHSpec> {
        WlenW::new(self, 5)
    }
}
#[doc = "line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrHSpec;
impl crate::RegisterSpec for LcrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr_h::R`](R) reader structure"]
impl crate::Readable for LcrHSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr_h::W`](W) writer structure"]
impl crate::Writable for LcrHSpec {
    type Safety = crate::Unsafe;
}
