#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DAC_EN` reader - Dac en"]
pub type DacEnR = crate::BitReader;
#[doc = "Field `DAC_EN` writer - Dac en"]
pub type DacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_EN` reader - Trig en"]
pub type TrigEnR = crate::BitReader;
#[doc = "Field `TRIG_EN` writer - Trig en"]
pub type TrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trig src sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrigSrcSel {
    #[doc = "3: Bstimer0 trgo"]
    Bstimer0Trgo = 3,
    #[doc = "2: Bstimer1 trgo"]
    Bstimer1Trgo = 2,
    #[doc = "5: Gpio24"]
    Gpio24 = 5,
    #[doc = "6: Gpio43"]
    Gpio43 = 6,
    #[doc = "4: Gpio6"]
    Gpio6 = 4,
    #[doc = "1: Gptimer0 trgo"]
    Gptimer0Trgo = 1,
    #[doc = "0: Gptimer1 trgo"]
    Gptimer1Trgo = 0,
    #[doc = "7: Software"]
    Software = 7,
}
impl From<TrigSrcSel> for u8 {
    #[inline(always)]
    fn from(variant: TrigSrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TrigSrcSel {
    type Ux = u8;
}
impl crate::IsEnum for TrigSrcSel {}
#[doc = "Field `TRIG_SRC_SEL` reader - Trig src sel"]
pub type TrigSrcSelR = crate::FieldReader<TrigSrcSel>;
impl TrigSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrigSrcSel {
        match self.bits {
            3 => TrigSrcSel::Bstimer0Trgo,
            2 => TrigSrcSel::Bstimer1Trgo,
            5 => TrigSrcSel::Gpio24,
            6 => TrigSrcSel::Gpio43,
            4 => TrigSrcSel::Gpio6,
            1 => TrigSrcSel::Gptimer0Trgo,
            0 => TrigSrcSel::Gptimer1Trgo,
            7 => TrigSrcSel::Software,
            _ => unreachable!(),
        }
    }
    #[doc = "Bstimer0 trgo"]
    #[inline(always)]
    pub fn is_bstimer0_trgo(&self) -> bool {
        *self == TrigSrcSel::Bstimer0Trgo
    }
    #[doc = "Bstimer1 trgo"]
    #[inline(always)]
    pub fn is_bstimer1_trgo(&self) -> bool {
        *self == TrigSrcSel::Bstimer1Trgo
    }
    #[doc = "Gpio24"]
    #[inline(always)]
    pub fn is_gpio24(&self) -> bool {
        *self == TrigSrcSel::Gpio24
    }
    #[doc = "Gpio43"]
    #[inline(always)]
    pub fn is_gpio43(&self) -> bool {
        *self == TrigSrcSel::Gpio43
    }
    #[doc = "Gpio6"]
    #[inline(always)]
    pub fn is_gpio6(&self) -> bool {
        *self == TrigSrcSel::Gpio6
    }
    #[doc = "Gptimer0 trgo"]
    #[inline(always)]
    pub fn is_gptimer0_trgo(&self) -> bool {
        *self == TrigSrcSel::Gptimer0Trgo
    }
    #[doc = "Gptimer1 trgo"]
    #[inline(always)]
    pub fn is_gptimer1_trgo(&self) -> bool {
        *self == TrigSrcSel::Gptimer1Trgo
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TrigSrcSel::Software
    }
}
#[doc = "Field `TRIG_SRC_SEL` writer - Trig src sel"]
pub type TrigSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, TrigSrcSel, crate::Safe>;
impl<'a, REG> TrigSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bstimer0 trgo"]
    #[inline(always)]
    pub fn bstimer0_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Bstimer0Trgo)
    }
    #[doc = "Bstimer1 trgo"]
    #[inline(always)]
    pub fn bstimer1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Bstimer1Trgo)
    }
    #[doc = "Gpio24"]
    #[inline(always)]
    pub fn gpio24(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Gpio24)
    }
    #[doc = "Gpio43"]
    #[inline(always)]
    pub fn gpio43(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Gpio43)
    }
    #[doc = "Gpio6"]
    #[inline(always)]
    pub fn gpio6(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Gpio6)
    }
    #[doc = "Gptimer0 trgo"]
    #[inline(always)]
    pub fn gptimer0_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Gptimer0Trgo)
    }
    #[doc = "Gptimer1 trgo"]
    #[inline(always)]
    pub fn gptimer1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Gptimer1Trgo)
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSrcSel::Software)
    }
}
#[doc = "Trig type sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrigTypeSel {
    #[doc = "1: Falling edge"]
    FallingEdge = 1,
    #[doc = "0: Rising edge"]
    RisingEdge = 0,
    #[doc = "2: Rising falling edge"]
    RisingFallingEdge = 2,
}
impl From<TrigTypeSel> for u8 {
    #[inline(always)]
    fn from(variant: TrigTypeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TrigTypeSel {
    type Ux = u8;
}
impl crate::IsEnum for TrigTypeSel {}
#[doc = "Field `TRIG_TYPE_SEL` reader - Trig type sel"]
pub type TrigTypeSelR = crate::FieldReader<TrigTypeSel>;
impl TrigTypeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TrigTypeSel> {
        match self.bits {
            1 => Some(TrigTypeSel::FallingEdge),
            0 => Some(TrigTypeSel::RisingEdge),
            2 => Some(TrigTypeSel::RisingFallingEdge),
            _ => None,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TrigTypeSel::FallingEdge
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TrigTypeSel::RisingEdge
    }
    #[doc = "Rising falling edge"]
    #[inline(always)]
    pub fn is_rising_falling_edge(&self) -> bool {
        *self == TrigTypeSel::RisingFallingEdge
    }
}
#[doc = "Field `TRIG_TYPE_SEL` writer - Trig type sel"]
pub type TrigTypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, TrigTypeSel>;
impl<'a, REG> TrigTypeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TrigTypeSel::FallingEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TrigTypeSel::RisingEdge)
    }
    #[doc = "Rising falling edge"]
    #[inline(always)]
    pub fn rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TrigTypeSel::RisingFallingEdge)
    }
}
#[doc = "Wave sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WaveSel {
    #[doc = "1: Noise"]
    Noise = 1,
    #[doc = "0: None"]
    None = 0,
    #[doc = "2: Triangle"]
    Triangle = 2,
}
impl From<WaveSel> for u8 {
    #[inline(always)]
    fn from(variant: WaveSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WaveSel {
    type Ux = u8;
}
impl crate::IsEnum for WaveSel {}
#[doc = "Field `WAVE_SEL` reader - Wave sel"]
pub type WaveSelR = crate::FieldReader<WaveSel>;
impl WaveSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WaveSel> {
        match self.bits {
            1 => Some(WaveSel::Noise),
            0 => Some(WaveSel::None),
            2 => Some(WaveSel::Triangle),
            _ => None,
        }
    }
    #[doc = "Noise"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WaveSel::Noise
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WaveSel::None
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WaveSel::Triangle
    }
}
#[doc = "Field `WAVE_SEL` writer - Wave sel"]
pub type WaveSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, WaveSel>;
impl<'a, REG> WaveSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Noise"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(WaveSel::Noise)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(WaveSel::None)
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(WaveSel::Triangle)
    }
}
#[doc = "Mask amp sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MaskAmpSel {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "9: Value 1023"]
    Value1023 = 9,
    #[doc = "6: Value 127"]
    Value127 = 6,
    #[doc = "3: Value 15"]
    Value15 = 3,
    #[doc = "7: Value 255"]
    Value255 = 7,
    #[doc = "1: Value 3"]
    Value3 = 1,
    #[doc = "4: Value 31"]
    Value31 = 4,
    #[doc = "8: Value 511"]
    Value511 = 8,
    #[doc = "5: Value 63"]
    Value63 = 5,
    #[doc = "2: Value 7"]
    Value7 = 2,
}
impl From<MaskAmpSel> for u8 {
    #[inline(always)]
    fn from(variant: MaskAmpSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MaskAmpSel {
    type Ux = u8;
}
impl crate::IsEnum for MaskAmpSel {}
#[doc = "Field `MASK_AMP_SEL` reader - Mask amp sel"]
pub type MaskAmpSelR = crate::FieldReader<MaskAmpSel>;
impl MaskAmpSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MaskAmpSel> {
        match self.bits {
            0 => Some(MaskAmpSel::Value1),
            9 => Some(MaskAmpSel::Value1023),
            6 => Some(MaskAmpSel::Value127),
            3 => Some(MaskAmpSel::Value15),
            7 => Some(MaskAmpSel::Value255),
            1 => Some(MaskAmpSel::Value3),
            4 => Some(MaskAmpSel::Value31),
            8 => Some(MaskAmpSel::Value511),
            5 => Some(MaskAmpSel::Value63),
            2 => Some(MaskAmpSel::Value7),
            _ => None,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == MaskAmpSel::Value1
    }
    #[doc = "Value 1023"]
    #[inline(always)]
    pub fn is_value_1023(&self) -> bool {
        *self == MaskAmpSel::Value1023
    }
    #[doc = "Value 127"]
    #[inline(always)]
    pub fn is_value_127(&self) -> bool {
        *self == MaskAmpSel::Value127
    }
    #[doc = "Value 15"]
    #[inline(always)]
    pub fn is_value_15(&self) -> bool {
        *self == MaskAmpSel::Value15
    }
    #[doc = "Value 255"]
    #[inline(always)]
    pub fn is_value_255(&self) -> bool {
        *self == MaskAmpSel::Value255
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == MaskAmpSel::Value3
    }
    #[doc = "Value 31"]
    #[inline(always)]
    pub fn is_value_31(&self) -> bool {
        *self == MaskAmpSel::Value31
    }
    #[doc = "Value 511"]
    #[inline(always)]
    pub fn is_value_511(&self) -> bool {
        *self == MaskAmpSel::Value511
    }
    #[doc = "Value 63"]
    #[inline(always)]
    pub fn is_value_63(&self) -> bool {
        *self == MaskAmpSel::Value63
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == MaskAmpSel::Value7
    }
}
#[doc = "Field `MASK_AMP_SEL` writer - Mask amp sel"]
pub type MaskAmpSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, MaskAmpSel>;
impl<'a, REG> MaskAmpSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value1)
    }
    #[doc = "Value 1023"]
    #[inline(always)]
    pub fn value_1023(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value1023)
    }
    #[doc = "Value 127"]
    #[inline(always)]
    pub fn value_127(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value127)
    }
    #[doc = "Value 15"]
    #[inline(always)]
    pub fn value_15(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value15)
    }
    #[doc = "Value 255"]
    #[inline(always)]
    pub fn value_255(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value255)
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value3)
    }
    #[doc = "Value 31"]
    #[inline(always)]
    pub fn value_31(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value31)
    }
    #[doc = "Value 511"]
    #[inline(always)]
    pub fn value_511(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value511)
    }
    #[doc = "Value 63"]
    #[inline(always)]
    pub fn value_63(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value63)
    }
    #[doc = "Value 7"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut crate::W<REG> {
        self.variant(MaskAmpSel::Value7)
    }
}
#[doc = "Field `DMA_EN` reader - Dma en"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - Dma en"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_UNDERFLOW_EN` reader - Intr underflow en"]
pub type IntrUnderflowEnR = crate::BitReader;
#[doc = "Field `INTR_UNDERFLOW_EN` writer - Intr underflow en"]
pub type IntrUnderflowEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_EMPTY_EN` reader - Intr empty en"]
pub type IntrEmptyEnR = crate::BitReader;
#[doc = "Field `INTR_EMPTY_EN` writer - Intr empty en"]
pub type IntrEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dac en"]
    #[inline(always)]
    pub fn dac_en(&self) -> DacEnR {
        DacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Trig en"]
    #[inline(always)]
    pub fn trig_en(&self) -> TrigEnR {
        TrigEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Trig src sel"]
    #[inline(always)]
    pub fn trig_src_sel(&self) -> TrigSrcSelR {
        TrigSrcSelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Trig type sel"]
    #[inline(always)]
    pub fn trig_type_sel(&self) -> TrigTypeSelR {
        TrigTypeSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Wave sel"]
    #[inline(always)]
    pub fn wave_sel(&self) -> WaveSelR {
        WaveSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Mask amp sel"]
    #[inline(always)]
    pub fn mask_amp_sel(&self) -> MaskAmpSelR {
        MaskAmpSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Dma en"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Intr underflow en"]
    #[inline(always)]
    pub fn intr_underflow_en(&self) -> IntrUnderflowEnR {
        IntrUnderflowEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Intr empty en"]
    #[inline(always)]
    pub fn intr_empty_en(&self) -> IntrEmptyEnR {
        IntrEmptyEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dac en"]
    #[inline(always)]
    pub fn dac_en(&mut self) -> DacEnW<'_, CrSpec> {
        DacEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Trig en"]
    #[inline(always)]
    pub fn trig_en(&mut self) -> TrigEnW<'_, CrSpec> {
        TrigEnW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Trig src sel"]
    #[inline(always)]
    pub fn trig_src_sel(&mut self) -> TrigSrcSelW<'_, CrSpec> {
        TrigSrcSelW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Trig type sel"]
    #[inline(always)]
    pub fn trig_type_sel(&mut self) -> TrigTypeSelW<'_, CrSpec> {
        TrigTypeSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Wave sel"]
    #[inline(always)]
    pub fn wave_sel(&mut self) -> WaveSelW<'_, CrSpec> {
        WaveSelW::new(self, 8)
    }
    #[doc = "Bits 10:13 - Mask amp sel"]
    #[inline(always)]
    pub fn mask_amp_sel(&mut self) -> MaskAmpSelW<'_, CrSpec> {
        MaskAmpSelW::new(self, 10)
    }
    #[doc = "Bit 14 - Dma en"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, CrSpec> {
        DmaEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Intr underflow en"]
    #[inline(always)]
    pub fn intr_underflow_en(&mut self) -> IntrUnderflowEnW<'_, CrSpec> {
        IntrUnderflowEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Intr empty en"]
    #[inline(always)]
    pub fn intr_empty_en(&mut self) -> IntrEmptyEnW<'_, CrSpec> {
        IntrEmptyEnW::new(self, 16)
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
