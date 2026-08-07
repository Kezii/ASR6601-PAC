#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `REVERSE_OUT_EN` reader - Reverse out en"]
pub type ReverseOutEnR = crate::BitReader;
#[doc = "Field `REVERSE_OUT_EN` writer - Reverse out en"]
pub type ReverseOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reverse in"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReverseIn {
    #[doc = "1: Byte"]
    Byte = 1,
    #[doc = "2: Hword"]
    Hword = 2,
    #[doc = "0: None"]
    None = 0,
    #[doc = "3: Word"]
    Word = 3,
}
impl From<ReverseIn> for u8 {
    #[inline(always)]
    fn from(variant: ReverseIn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReverseIn {
    type Ux = u8;
}
impl crate::IsEnum for ReverseIn {}
#[doc = "Field `REVERSE_IN` reader - Reverse in"]
pub type ReverseInR = crate::FieldReader<ReverseIn>;
impl ReverseInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReverseIn {
        match self.bits {
            1 => ReverseIn::Byte,
            2 => ReverseIn::Hword,
            0 => ReverseIn::None,
            3 => ReverseIn::Word,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == ReverseIn::Byte
    }
    #[doc = "Hword"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == ReverseIn::Hword
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ReverseIn::None
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == ReverseIn::Word
    }
}
#[doc = "Field `REVERSE_IN` writer - Reverse in"]
pub type ReverseInW<'a, REG> = crate::FieldWriter<'a, REG, 2, ReverseIn, crate::Safe>;
impl<'a, REG> ReverseInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(ReverseIn::Byte)
    }
    #[doc = "Hword"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut crate::W<REG> {
        self.variant(ReverseIn::Hword)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ReverseIn::None)
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(ReverseIn::Word)
    }
}
#[doc = "Poly size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PolySize {
    #[doc = "1: Value 16"]
    Value16 = 1,
    #[doc = "0: Value 32"]
    Value32 = 0,
    #[doc = "3: Value 7"]
    Value7 = 3,
    #[doc = "2: Value 8"]
    Value8 = 2,
}
impl From<PolySize> for u8 {
    #[inline(always)]
    fn from(variant: PolySize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PolySize {
    type Ux = u8;
}
impl crate::IsEnum for PolySize {}
#[doc = "Field `POLY_SIZE` reader - Poly size"]
pub type PolySizeR = crate::FieldReader<PolySize>;
impl PolySizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolySize {
        match self.bits {
            1 => PolySize::Value16,
            0 => PolySize::Value32,
            3 => PolySize::Value7,
            2 => PolySize::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == PolySize::Value16
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn is_value_32(&self) -> bool {
        *self == PolySize::Value32
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == PolySize::Value7
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == PolySize::Value8
    }
}
#[doc = "Field `POLY_SIZE` writer - Poly size"]
pub type PolySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PolySize, crate::Safe>;
impl<'a, REG> PolySizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(PolySize::Value16)
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn value_32(self) -> &'a mut crate::W<REG> {
        self.variant(PolySize::Value32)
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut crate::W<REG> {
        self.variant(PolySize::Value7)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(PolySize::Value8)
    }
}
#[doc = "Field `CALC_INIT` reader - Calc init"]
pub type CalcInitR = crate::BitReader;
#[doc = "Field `CALC_INIT` writer - Calc init"]
pub type CalcInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALC_FLAG` reader - Calc flag"]
pub type CalcFlagR = crate::BitReader;
#[doc = "Field `CALC_FLAG` writer - Calc flag"]
pub type CalcFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reverse out en"]
    #[inline(always)]
    pub fn reverse_out_en(&self) -> ReverseOutEnR {
        ReverseOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reverse in"]
    #[inline(always)]
    pub fn reverse_in(&self) -> ReverseInR {
        ReverseInR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Poly size"]
    #[inline(always)]
    pub fn poly_size(&self) -> PolySizeR {
        PolySizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Calc init"]
    #[inline(always)]
    pub fn calc_init(&self) -> CalcInitR {
        CalcInitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calc flag"]
    #[inline(always)]
    pub fn calc_flag(&self) -> CalcFlagR {
        CalcFlagR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reverse out en"]
    #[inline(always)]
    pub fn reverse_out_en(&mut self) -> ReverseOutEnW<'_, CrSpec> {
        ReverseOutEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reverse in"]
    #[inline(always)]
    pub fn reverse_in(&mut self) -> ReverseInW<'_, CrSpec> {
        ReverseInW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Poly size"]
    #[inline(always)]
    pub fn poly_size(&mut self) -> PolySizeW<'_, CrSpec> {
        PolySizeW::new(self, 3)
    }
    #[doc = "Bit 5 - Calc init"]
    #[inline(always)]
    pub fn calc_init(&mut self) -> CalcInitW<'_, CrSpec> {
        CalcInitW::new(self, 5)
    }
    #[doc = "Bit 6 - Calc flag"]
    #[inline(always)]
    pub fn calc_flag(&mut self) -> CalcFlagW<'_, CrSpec> {
        CalcFlagW::new(self, 6)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
