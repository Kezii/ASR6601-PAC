#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `CLK_DIV` reader - adcclk prescale"]
pub type ClkDivR = crate::FieldReader<u16>;
#[doc = "Field `CLK_DIV` writer - adcclk prescale"]
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMA_EN` reader - dma enable"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - dma enable"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "external trigger selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtTrigSel {
    #[doc = "5: GPIO47"]
    Gpio47 = 5,
    #[doc = "6: GPIO31"]
    Gpio31 = 6,
    #[doc = "7: GPIO19"]
    Gpio19 = 7,
    #[doc = "8: GPIO10"]
    Gpio10 = 8,
    #[doc = "9: GPTIM1 TRGO"]
    Gptim1Trgo = 9,
    #[doc = "10: GPTIM0 CH2 output"]
    Gptim0Ch2Out = 10,
    #[doc = "11: GPTIM3 TRGO"]
    Gptim3Trgo = 11,
    #[doc = "12: GPTIM0 CH3 output"]
    Gptim0Ch3Out = 12,
    #[doc = "13: GPTIM0 TRGO"]
    Gptim0Trgo = 13,
    #[doc = "14: GPTIM2 CH1 output"]
    Gptim2Ch1Out = 14,
}
impl From<ExtTrigSel> for u8 {
    #[inline(always)]
    fn from(variant: ExtTrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExtTrigSel {
    type Ux = u8;
}
impl crate::IsEnum for ExtTrigSel {}
#[doc = "Field `EXT_TRIG_SEL` reader - external trigger selection"]
pub type ExtTrigSelR = crate::FieldReader<ExtTrigSel>;
impl ExtTrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ExtTrigSel> {
        match self.bits {
            5 => Some(ExtTrigSel::Gpio47),
            6 => Some(ExtTrigSel::Gpio31),
            7 => Some(ExtTrigSel::Gpio19),
            8 => Some(ExtTrigSel::Gpio10),
            9 => Some(ExtTrigSel::Gptim1Trgo),
            10 => Some(ExtTrigSel::Gptim0Ch2Out),
            11 => Some(ExtTrigSel::Gptim3Trgo),
            12 => Some(ExtTrigSel::Gptim0Ch3Out),
            13 => Some(ExtTrigSel::Gptim0Trgo),
            14 => Some(ExtTrigSel::Gptim2Ch1Out),
            _ => None,
        }
    }
    #[doc = "GPIO47"]
    #[inline(always)]
    pub fn is_gpio47(&self) -> bool {
        *self == ExtTrigSel::Gpio47
    }
    #[doc = "GPIO31"]
    #[inline(always)]
    pub fn is_gpio31(&self) -> bool {
        *self == ExtTrigSel::Gpio31
    }
    #[doc = "GPIO19"]
    #[inline(always)]
    pub fn is_gpio19(&self) -> bool {
        *self == ExtTrigSel::Gpio19
    }
    #[doc = "GPIO10"]
    #[inline(always)]
    pub fn is_gpio10(&self) -> bool {
        *self == ExtTrigSel::Gpio10
    }
    #[doc = "GPTIM1 TRGO"]
    #[inline(always)]
    pub fn is_gptim1_trgo(&self) -> bool {
        *self == ExtTrigSel::Gptim1Trgo
    }
    #[doc = "GPTIM0 CH2 output"]
    #[inline(always)]
    pub fn is_gptim0_ch2_out(&self) -> bool {
        *self == ExtTrigSel::Gptim0Ch2Out
    }
    #[doc = "GPTIM3 TRGO"]
    #[inline(always)]
    pub fn is_gptim3_trgo(&self) -> bool {
        *self == ExtTrigSel::Gptim3Trgo
    }
    #[doc = "GPTIM0 CH3 output"]
    #[inline(always)]
    pub fn is_gptim0_ch3_out(&self) -> bool {
        *self == ExtTrigSel::Gptim0Ch3Out
    }
    #[doc = "GPTIM0 TRGO"]
    #[inline(always)]
    pub fn is_gptim0_trgo(&self) -> bool {
        *self == ExtTrigSel::Gptim0Trgo
    }
    #[doc = "GPTIM2 CH1 output"]
    #[inline(always)]
    pub fn is_gptim2_ch1_out(&self) -> bool {
        *self == ExtTrigSel::Gptim2Ch1Out
    }
}
#[doc = "Field `EXT_TRIG_SEL` writer - external trigger selection"]
pub type ExtTrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, ExtTrigSel>;
impl<'a, REG> ExtTrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO47"]
    #[inline(always)]
    pub fn gpio47(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gpio47)
    }
    #[doc = "GPIO31"]
    #[inline(always)]
    pub fn gpio31(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gpio31)
    }
    #[doc = "GPIO19"]
    #[inline(always)]
    pub fn gpio19(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gpio19)
    }
    #[doc = "GPIO10"]
    #[inline(always)]
    pub fn gpio10(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gpio10)
    }
    #[doc = "GPTIM1 TRGO"]
    #[inline(always)]
    pub fn gptim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim1Trgo)
    }
    #[doc = "GPTIM0 CH2 output"]
    #[inline(always)]
    pub fn gptim0_ch2_out(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim0Ch2Out)
    }
    #[doc = "GPTIM3 TRGO"]
    #[inline(always)]
    pub fn gptim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim3Trgo)
    }
    #[doc = "GPTIM0 CH3 output"]
    #[inline(always)]
    pub fn gptim0_ch3_out(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim0Ch3Out)
    }
    #[doc = "GPTIM0 TRGO"]
    #[inline(always)]
    pub fn gptim0_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim0Trgo)
    }
    #[doc = "GPTIM2 CH1 output"]
    #[inline(always)]
    pub fn gptim2_ch1_out(self) -> &'a mut crate::W<REG> {
        self.variant(ExtTrigSel::Gptim2Ch1Out)
    }
}
#[doc = "trigger mode and polarity selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrigSel {
    #[doc = "0: software trigger"]
    Sw = 0,
    #[doc = "1: hardware trigger rising edge"]
    HwRising = 1,
    #[doc = "2: hardware trigger falling edge"]
    HwFalling = 2,
    #[doc = "3: hardware trigger both edges"]
    HwBoth = 3,
}
impl From<TrigSel> for u8 {
    #[inline(always)]
    fn from(variant: TrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TrigSel {
    type Ux = u8;
}
impl crate::IsEnum for TrigSel {}
#[doc = "Field `TRIG_SEL` reader - trigger mode and polarity selection"]
pub type TrigSelR = crate::FieldReader<TrigSel>;
impl TrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrigSel {
        match self.bits {
            0 => TrigSel::Sw,
            1 => TrigSel::HwRising,
            2 => TrigSel::HwFalling,
            3 => TrigSel::HwBoth,
            _ => unreachable!(),
        }
    }
    #[doc = "software trigger"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TrigSel::Sw
    }
    #[doc = "hardware trigger rising edge"]
    #[inline(always)]
    pub fn is_hw_rising(&self) -> bool {
        *self == TrigSel::HwRising
    }
    #[doc = "hardware trigger falling edge"]
    #[inline(always)]
    pub fn is_hw_falling(&self) -> bool {
        *self == TrigSel::HwFalling
    }
    #[doc = "hardware trigger both edges"]
    #[inline(always)]
    pub fn is_hw_both(&self) -> bool {
        *self == TrigSel::HwBoth
    }
}
#[doc = "Field `TRIG_SEL` writer - trigger mode and polarity selection"]
pub type TrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, TrigSel, crate::Safe>;
impl<'a, REG> TrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software trigger"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSel::Sw)
    }
    #[doc = "hardware trigger rising edge"]
    #[inline(always)]
    pub fn hw_rising(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSel::HwRising)
    }
    #[doc = "hardware trigger falling edge"]
    #[inline(always)]
    pub fn hw_falling(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSel::HwFalling)
    }
    #[doc = "hardware trigger both edges"]
    #[inline(always)]
    pub fn hw_both(self) -> &'a mut crate::W<REG> {
        self.variant(TrigSel::HwBoth)
    }
}
#[doc = "Field `OVERRUN_MODE` reader - overrun management mode"]
pub type OverrunModeR = crate::BitReader;
#[doc = "Field `OVERRUN_MODE` writer - overrun management mode"]
pub type OverrunModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "conversion mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ConvMode {
    #[doc = "0: single conversion mode"]
    Single = 0,
    #[doc = "1: continuous conversion mode"]
    Continuous = 1,
    #[doc = "2: discontinuous conversion mode"]
    Discontinuous = 2,
}
impl From<ConvMode> for u8 {
    #[inline(always)]
    fn from(variant: ConvMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ConvMode {
    type Ux = u8;
}
impl crate::IsEnum for ConvMode {}
#[doc = "Field `CONV_MODE` reader - conversion mode selection"]
pub type ConvModeR = crate::FieldReader<ConvMode>;
impl ConvModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ConvMode> {
        match self.bits {
            0 => Some(ConvMode::Single),
            1 => Some(ConvMode::Continuous),
            2 => Some(ConvMode::Discontinuous),
            _ => None,
        }
    }
    #[doc = "single conversion mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ConvMode::Single
    }
    #[doc = "continuous conversion mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == ConvMode::Continuous
    }
    #[doc = "discontinuous conversion mode"]
    #[inline(always)]
    pub fn is_discontinuous(&self) -> bool {
        *self == ConvMode::Discontinuous
    }
}
#[doc = "Field `CONV_MODE` writer - conversion mode selection"]
pub type ConvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ConvMode>;
impl<'a, REG> ConvModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(ConvMode::Single)
    }
    #[doc = "continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(ConvMode::Continuous)
    }
    #[doc = "discontinuous conversion mode"]
    #[inline(always)]
    pub fn discontinuous(self) -> &'a mut crate::W<REG> {
        self.variant(ConvMode::Discontinuous)
    }
}
#[doc = "Field `WAIT_MODE` reader - wait conversion mode"]
pub type WaitModeR = crate::BitReader;
#[doc = "Field `WAIT_MODE` writer - wait conversion mode"]
pub type WaitModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - adcclk prescale"]
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - dma enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - external trigger selection"]
    #[inline(always)]
    pub fn ext_trig_sel(&self) -> ExtTrigSelR {
        ExtTrigSelR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - trigger mode and polarity selection"]
    #[inline(always)]
    pub fn trig_sel(&self) -> TrigSelR {
        TrigSelR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - overrun management mode"]
    #[inline(always)]
    pub fn overrun_mode(&self) -> OverrunModeR {
        OverrunModeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - conversion mode selection"]
    #[inline(always)]
    pub fn conv_mode(&self) -> ConvModeR {
        ConvModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - wait conversion mode"]
    #[inline(always)]
    pub fn wait_mode(&self) -> WaitModeR {
        WaitModeR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - adcclk prescale"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<'_, CfgrSpec> {
        ClkDivW::new(self, 0)
    }
    #[doc = "Bit 12 - dma enable"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, CfgrSpec> {
        DmaEnW::new(self, 12)
    }
    #[doc = "Bits 13:16 - external trigger selection"]
    #[inline(always)]
    pub fn ext_trig_sel(&mut self) -> ExtTrigSelW<'_, CfgrSpec> {
        ExtTrigSelW::new(self, 13)
    }
    #[doc = "Bits 17:18 - trigger mode and polarity selection"]
    #[inline(always)]
    pub fn trig_sel(&mut self) -> TrigSelW<'_, CfgrSpec> {
        TrigSelW::new(self, 17)
    }
    #[doc = "Bit 19 - overrun management mode"]
    #[inline(always)]
    pub fn overrun_mode(&mut self) -> OverrunModeW<'_, CfgrSpec> {
        OverrunModeW::new(self, 19)
    }
    #[doc = "Bits 20:21 - conversion mode selection"]
    #[inline(always)]
    pub fn conv_mode(&mut self) -> ConvModeW<'_, CfgrSpec> {
        ConvModeW::new(self, 20)
    }
    #[doc = "Bit 22 - wait conversion mode"]
    #[inline(always)]
    pub fn wait_mode(&mut self) -> WaitModeW<'_, CfgrSpec> {
        WaitModeW::new(self, 22)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
