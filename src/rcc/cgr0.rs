#[doc = "Register `CGR0` reader"]
pub type R = crate::R<Cgr0Spec>;
#[doc = "Register `CGR0` writer"]
pub type W = crate::W<Cgr0Spec>;
#[doc = "Field `TIMER3_CLK_EN` reader - Timer3 clk en"]
pub type Timer3ClkEnR = crate::BitReader;
#[doc = "Field `TIMER3_CLK_EN` writer - Timer3 clk en"]
pub type Timer3ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_CLK_EN` reader - Timer2 clk en"]
pub type Timer2ClkEnR = crate::BitReader;
#[doc = "Field `TIMER2_CLK_EN` writer - Timer2 clk en"]
pub type Timer2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_CLK_EN` reader - Timer1 clk en"]
pub type Timer1ClkEnR = crate::BitReader;
#[doc = "Field `TIMER1_CLK_EN` writer - Timer1 clk en"]
pub type Timer1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_CLK_EN` reader - Timer0 clk en"]
pub type Timer0ClkEnR = crate::BitReader;
#[doc = "Field `TIMER0_CLK_EN` writer - Timer0 clk en"]
pub type Timer0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LORA_CLK_EN` reader - Lora clk en"]
pub type LoraClkEnR = crate::BitReader;
#[doc = "Field `LORA_CLK_EN` writer - Lora clk en"]
pub type LoraClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_CLK_EN` reader - Dac clk en"]
pub type DacClkEnR = crate::BitReader;
#[doc = "Field `DAC_CLK_EN` writer - Dac clk en"]
pub type DacClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CLK_EN` reader - Lcd clk en"]
pub type LcdClkEnR = crate::BitReader;
#[doc = "Field `LCD_CLK_EN` writer - Lcd clk en"]
pub type LcdClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFEC_CLK_EN` reader - Afec clk en"]
pub type AfecClkEnR = crate::BitReader;
#[doc = "Field `AFEC_CLK_EN` writer - Afec clk en"]
pub type AfecClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_CLK_EN` reader - Adc clk en"]
pub type AdcClkEnR = crate::BitReader;
#[doc = "Field `ADC_CLK_EN` writer - Adc clk en"]
pub type AdcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_CLK_EN` reader - I2c2 clk en"]
pub type I2c2ClkEnR = crate::BitReader;
#[doc = "Field `I2C2_CLK_EN` writer - I2c2 clk en"]
pub type I2c2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_CLK_EN` reader - I2c1 clk en"]
pub type I2c1ClkEnR = crate::BitReader;
#[doc = "Field `I2C1_CLK_EN` writer - I2c1 clk en"]
pub type I2c1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_CLK_EN` reader - I2c0 clk en"]
pub type I2c0ClkEnR = crate::BitReader;
#[doc = "Field `I2C0_CLK_EN` writer - I2c0 clk en"]
pub type I2c0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP2_CLK_EN` reader - Ssp2 clk en"]
pub type Ssp2ClkEnR = crate::BitReader;
#[doc = "Field `SSP2_CLK_EN` writer - Ssp2 clk en"]
pub type Ssp2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP1_CLK_EN` reader - Ssp1 clk en"]
pub type Ssp1ClkEnR = crate::BitReader;
#[doc = "Field `SSP1_CLK_EN` writer - Ssp1 clk en"]
pub type Ssp1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP0_CLK_EN` reader - Ssp0 clk en"]
pub type Ssp0ClkEnR = crate::BitReader;
#[doc = "Field `SSP0_CLK_EN` writer - Ssp0 clk en"]
pub type Ssp0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART_CLK_EN` reader - Lpuart clk en"]
pub type LpuartClkEnR = crate::BitReader;
#[doc = "Field `LPUART_CLK_EN` writer - Lpuart clk en"]
pub type LpuartClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_CLK_EN` reader - Uart3 clk en"]
pub type Uart3ClkEnR = crate::BitReader;
#[doc = "Field `UART3_CLK_EN` writer - Uart3 clk en"]
pub type Uart3ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_CLK_EN` reader - Uart2 clk en"]
pub type Uart2ClkEnR = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - Uart2 clk en"]
pub type Uart2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_CLK_EN` reader - Uart1 clk en"]
pub type Uart1ClkEnR = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - Uart1 clk en"]
pub type Uart1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_CLK_EN` reader - Uart0 clk en"]
pub type Uart0ClkEnR = crate::BitReader;
#[doc = "Field `UART0_CLK_EN` writer - Uart0 clk en"]
pub type Uart0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_CLK_EN` reader - Syscfg clk en"]
pub type SyscfgClkEnR = crate::BitReader;
#[doc = "Field `SYSCFG_CLK_EN` writer - Syscfg clk en"]
pub type SyscfgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOM3_CLK_EN` reader - Iom3 clk en"]
pub type Iom3ClkEnR = crate::BitReader;
#[doc = "Field `IOM3_CLK_EN` writer - Iom3 clk en"]
pub type Iom3ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOM2_CLK_EN` reader - Iom2 clk en"]
pub type Iom2ClkEnR = crate::BitReader;
#[doc = "Field `IOM2_CLK_EN` writer - Iom2 clk en"]
pub type Iom2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOM1_CLK_EN` reader - Iom1 clk en"]
pub type Iom1ClkEnR = crate::BitReader;
#[doc = "Field `IOM1_CLK_EN` writer - Iom1 clk en"]
pub type Iom1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOM0_CLK_EN` reader - Iom0 clk en"]
pub type Iom0ClkEnR = crate::BitReader;
#[doc = "Field `IOM0_CLK_EN` writer - Iom0 clk en"]
pub type Iom0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSTIMER1_CLK_EN` reader - Bstimer1 clk en"]
pub type Bstimer1ClkEnR = crate::BitReader;
#[doc = "Field `BSTIMER1_CLK_EN` writer - Bstimer1 clk en"]
pub type Bstimer1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSTIMER0_CLK_EN` reader - Bstimer0 clk en"]
pub type Bstimer0ClkEnR = crate::BitReader;
#[doc = "Field `BSTIMER0_CLK_EN` writer - Bstimer0 clk en"]
pub type Bstimer0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_CLK_EN` reader - Crc clk en"]
pub type CrcClkEnR = crate::BitReader;
#[doc = "Field `CRC_CLK_EN` writer - Crc clk en"]
pub type CrcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC1_CLK_EN` reader - Dmac1 clk en"]
pub type Dmac1ClkEnR = crate::BitReader;
#[doc = "Field `DMAC1_CLK_EN` writer - Dmac1 clk en"]
pub type Dmac1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC0_CLK_EN` reader - Dmac0 clk en"]
pub type Dmac0ClkEnR = crate::BitReader;
#[doc = "Field `DMAC0_CLK_EN` writer - Dmac0 clk en"]
pub type Dmac0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR_CLK_EN` reader - Pwr clk en"]
pub type PwrClkEnR = crate::BitReader;
#[doc = "Field `PWR_CLK_EN` writer - Pwr clk en"]
pub type PwrClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer3 clk en"]
    #[inline(always)]
    pub fn timer3_clk_en(&self) -> Timer3ClkEnR {
        Timer3ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer2 clk en"]
    #[inline(always)]
    pub fn timer2_clk_en(&self) -> Timer2ClkEnR {
        Timer2ClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer1 clk en"]
    #[inline(always)]
    pub fn timer1_clk_en(&self) -> Timer1ClkEnR {
        Timer1ClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer0 clk en"]
    #[inline(always)]
    pub fn timer0_clk_en(&self) -> Timer0ClkEnR {
        Timer0ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lora clk en"]
    #[inline(always)]
    pub fn lora_clk_en(&self) -> LoraClkEnR {
        LoraClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dac clk en"]
    #[inline(always)]
    pub fn dac_clk_en(&self) -> DacClkEnR {
        DacClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lcd clk en"]
    #[inline(always)]
    pub fn lcd_clk_en(&self) -> LcdClkEnR {
        LcdClkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Afec clk en"]
    #[inline(always)]
    pub fn afec_clk_en(&self) -> AfecClkEnR {
        AfecClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Adc clk en"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> AdcClkEnR {
        AdcClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - I2c2 clk en"]
    #[inline(always)]
    pub fn i2c2_clk_en(&self) -> I2c2ClkEnR {
        I2c2ClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2c1 clk en"]
    #[inline(always)]
    pub fn i2c1_clk_en(&self) -> I2c1ClkEnR {
        I2c1ClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2c0 clk en"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2c0ClkEnR {
        I2c0ClkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ssp2 clk en"]
    #[inline(always)]
    pub fn ssp2_clk_en(&self) -> Ssp2ClkEnR {
        Ssp2ClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ssp1 clk en"]
    #[inline(always)]
    pub fn ssp1_clk_en(&self) -> Ssp1ClkEnR {
        Ssp1ClkEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ssp0 clk en"]
    #[inline(always)]
    pub fn ssp0_clk_en(&self) -> Ssp0ClkEnR {
        Ssp0ClkEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lpuart clk en"]
    #[inline(always)]
    pub fn lpuart_clk_en(&self) -> LpuartClkEnR {
        LpuartClkEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Uart3 clk en"]
    #[inline(always)]
    pub fn uart3_clk_en(&self) -> Uart3ClkEnR {
        Uart3ClkEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Uart2 clk en"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> Uart2ClkEnR {
        Uart2ClkEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Uart1 clk en"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> Uart1ClkEnR {
        Uart1ClkEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Uart0 clk en"]
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> Uart0ClkEnR {
        Uart0ClkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Syscfg clk en"]
    #[inline(always)]
    pub fn syscfg_clk_en(&self) -> SyscfgClkEnR {
        SyscfgClkEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Iom3 clk en"]
    #[inline(always)]
    pub fn iom3_clk_en(&self) -> Iom3ClkEnR {
        Iom3ClkEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Iom2 clk en"]
    #[inline(always)]
    pub fn iom2_clk_en(&self) -> Iom2ClkEnR {
        Iom2ClkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Iom1 clk en"]
    #[inline(always)]
    pub fn iom1_clk_en(&self) -> Iom1ClkEnR {
        Iom1ClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Iom0 clk en"]
    #[inline(always)]
    pub fn iom0_clk_en(&self) -> Iom0ClkEnR {
        Iom0ClkEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bstimer1 clk en"]
    #[inline(always)]
    pub fn bstimer1_clk_en(&self) -> Bstimer1ClkEnR {
        Bstimer1ClkEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bstimer0 clk en"]
    #[inline(always)]
    pub fn bstimer0_clk_en(&self) -> Bstimer0ClkEnR {
        Bstimer0ClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Crc clk en"]
    #[inline(always)]
    pub fn crc_clk_en(&self) -> CrcClkEnR {
        CrcClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Dmac1 clk en"]
    #[inline(always)]
    pub fn dmac1_clk_en(&self) -> Dmac1ClkEnR {
        Dmac1ClkEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dmac0 clk en"]
    #[inline(always)]
    pub fn dmac0_clk_en(&self) -> Dmac0ClkEnR {
        Dmac0ClkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pwr clk en"]
    #[inline(always)]
    pub fn pwr_clk_en(&self) -> PwrClkEnR {
        PwrClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer3 clk en"]
    #[inline(always)]
    pub fn timer3_clk_en(&mut self) -> Timer3ClkEnW<'_, Cgr0Spec> {
        Timer3ClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer2 clk en"]
    #[inline(always)]
    pub fn timer2_clk_en(&mut self) -> Timer2ClkEnW<'_, Cgr0Spec> {
        Timer2ClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer1 clk en"]
    #[inline(always)]
    pub fn timer1_clk_en(&mut self) -> Timer1ClkEnW<'_, Cgr0Spec> {
        Timer1ClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer0 clk en"]
    #[inline(always)]
    pub fn timer0_clk_en(&mut self) -> Timer0ClkEnW<'_, Cgr0Spec> {
        Timer0ClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Lora clk en"]
    #[inline(always)]
    pub fn lora_clk_en(&mut self) -> LoraClkEnW<'_, Cgr0Spec> {
        LoraClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Dac clk en"]
    #[inline(always)]
    pub fn dac_clk_en(&mut self) -> DacClkEnW<'_, Cgr0Spec> {
        DacClkEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Lcd clk en"]
    #[inline(always)]
    pub fn lcd_clk_en(&mut self) -> LcdClkEnW<'_, Cgr0Spec> {
        LcdClkEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Afec clk en"]
    #[inline(always)]
    pub fn afec_clk_en(&mut self) -> AfecClkEnW<'_, Cgr0Spec> {
        AfecClkEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Adc clk en"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> AdcClkEnW<'_, Cgr0Spec> {
        AdcClkEnW::new(self, 8)
    }
    #[doc = "Bit 10 - I2c2 clk en"]
    #[inline(always)]
    pub fn i2c2_clk_en(&mut self) -> I2c2ClkEnW<'_, Cgr0Spec> {
        I2c2ClkEnW::new(self, 10)
    }
    #[doc = "Bit 11 - I2c1 clk en"]
    #[inline(always)]
    pub fn i2c1_clk_en(&mut self) -> I2c1ClkEnW<'_, Cgr0Spec> {
        I2c1ClkEnW::new(self, 11)
    }
    #[doc = "Bit 12 - I2c0 clk en"]
    #[inline(always)]
    pub fn i2c0_clk_en(&mut self) -> I2c0ClkEnW<'_, Cgr0Spec> {
        I2c0ClkEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Ssp2 clk en"]
    #[inline(always)]
    pub fn ssp2_clk_en(&mut self) -> Ssp2ClkEnW<'_, Cgr0Spec> {
        Ssp2ClkEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Ssp1 clk en"]
    #[inline(always)]
    pub fn ssp1_clk_en(&mut self) -> Ssp1ClkEnW<'_, Cgr0Spec> {
        Ssp1ClkEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Ssp0 clk en"]
    #[inline(always)]
    pub fn ssp0_clk_en(&mut self) -> Ssp0ClkEnW<'_, Cgr0Spec> {
        Ssp0ClkEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Lpuart clk en"]
    #[inline(always)]
    pub fn lpuart_clk_en(&mut self) -> LpuartClkEnW<'_, Cgr0Spec> {
        LpuartClkEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Uart3 clk en"]
    #[inline(always)]
    pub fn uart3_clk_en(&mut self) -> Uart3ClkEnW<'_, Cgr0Spec> {
        Uart3ClkEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Uart2 clk en"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> Uart2ClkEnW<'_, Cgr0Spec> {
        Uart2ClkEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Uart1 clk en"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> Uart1ClkEnW<'_, Cgr0Spec> {
        Uart1ClkEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Uart0 clk en"]
    #[inline(always)]
    pub fn uart0_clk_en(&mut self) -> Uart0ClkEnW<'_, Cgr0Spec> {
        Uart0ClkEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Syscfg clk en"]
    #[inline(always)]
    pub fn syscfg_clk_en(&mut self) -> SyscfgClkEnW<'_, Cgr0Spec> {
        SyscfgClkEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Iom3 clk en"]
    #[inline(always)]
    pub fn iom3_clk_en(&mut self) -> Iom3ClkEnW<'_, Cgr0Spec> {
        Iom3ClkEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Iom2 clk en"]
    #[inline(always)]
    pub fn iom2_clk_en(&mut self) -> Iom2ClkEnW<'_, Cgr0Spec> {
        Iom2ClkEnW::new(self, 23)
    }
    #[doc = "Bit 24 - Iom1 clk en"]
    #[inline(always)]
    pub fn iom1_clk_en(&mut self) -> Iom1ClkEnW<'_, Cgr0Spec> {
        Iom1ClkEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Iom0 clk en"]
    #[inline(always)]
    pub fn iom0_clk_en(&mut self) -> Iom0ClkEnW<'_, Cgr0Spec> {
        Iom0ClkEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Bstimer1 clk en"]
    #[inline(always)]
    pub fn bstimer1_clk_en(&mut self) -> Bstimer1ClkEnW<'_, Cgr0Spec> {
        Bstimer1ClkEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Bstimer0 clk en"]
    #[inline(always)]
    pub fn bstimer0_clk_en(&mut self) -> Bstimer0ClkEnW<'_, Cgr0Spec> {
        Bstimer0ClkEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Crc clk en"]
    #[inline(always)]
    pub fn crc_clk_en(&mut self) -> CrcClkEnW<'_, Cgr0Spec> {
        CrcClkEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Dmac1 clk en"]
    #[inline(always)]
    pub fn dmac1_clk_en(&mut self) -> Dmac1ClkEnW<'_, Cgr0Spec> {
        Dmac1ClkEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Dmac0 clk en"]
    #[inline(always)]
    pub fn dmac0_clk_en(&mut self) -> Dmac0ClkEnW<'_, Cgr0Spec> {
        Dmac0ClkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Pwr clk en"]
    #[inline(always)]
    pub fn pwr_clk_en(&mut self) -> PwrClkEnW<'_, Cgr0Spec> {
        PwrClkEnW::new(self, 31)
    }
}
#[doc = "clock generation register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgr0Spec;
impl crate::RegisterSpec for Cgr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgr0::R`](R) reader structure"]
impl crate::Readable for Cgr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cgr0::W`](W) writer structure"]
impl crate::Writable for Cgr0Spec {
    type Safety = crate::Unsafe;
}
