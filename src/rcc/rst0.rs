#[doc = "Register `RST0` reader"]
pub type R = crate::R<Rst0Spec>;
#[doc = "Register `RST0` writer"]
pub type W = crate::W<Rst0Spec>;
#[doc = "Field `SAC_RST_N` reader - Sac rst n"]
pub type SacRstNR = crate::BitReader;
#[doc = "Field `SAC_RST_N` writer - Sac rst n"]
pub type SacRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_RST_N` reader - Sec rst n"]
pub type SecRstNR = crate::BitReader;
#[doc = "Field `SEC_RST_N` writer - Sec rst n"]
pub type SecRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_RST_N` reader - Crc rst n"]
pub type CrcRstNR = crate::BitReader;
#[doc = "Field `CRC_RST_N` writer - Crc rst n"]
pub type CrcRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_RST_N` reader - Rtc rst n"]
pub type RtcRstNR = crate::BitReader;
#[doc = "Field `RTC_RST_N` writer - Rtc rst n"]
pub type RtcRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_RST_N` reader - Wdg rst n"]
pub type WdgRstNR = crate::BitReader;
#[doc = "Field `WDG_RST_N` writer - Wdg rst n"]
pub type WdgRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_RST_N` reader - Iwdg rst n"]
pub type IwdgRstNR = crate::BitReader;
#[doc = "Field `IWDG_RST_N` writer - Iwdg rst n"]
pub type IwdgRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER0_RST_N` reader - Lptimer0 rst n"]
pub type Lptimer0RstNR = crate::BitReader;
#[doc = "Field `LPTIMER0_RST_N` writer - Lptimer0 rst n"]
pub type Lptimer0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSTIMER1_RST_N` reader - Bstimer1 rst n"]
pub type Bstimer1RstNR = crate::BitReader;
#[doc = "Field `BSTIMER1_RST_N` writer - Bstimer1 rst n"]
pub type Bstimer1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSTIMER0_RST_N` reader - Bstimer0 rst n"]
pub type Bstimer0RstNR = crate::BitReader;
#[doc = "Field `BSTIMER0_RST_N` writer - Bstimer0 rst n"]
pub type Bstimer0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_RST_N` reader - Timer3 rst n"]
pub type Timer3RstNR = crate::BitReader;
#[doc = "Field `TIMER3_RST_N` writer - Timer3 rst n"]
pub type Timer3RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_RST_N` reader - Timer2 rst n"]
pub type Timer2RstNR = crate::BitReader;
#[doc = "Field `TIMER2_RST_N` writer - Timer2 rst n"]
pub type Timer2RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_RST_N` reader - Timer1 rst n"]
pub type Timer1RstNR = crate::BitReader;
#[doc = "Field `TIMER1_RST_N` writer - Timer1 rst n"]
pub type Timer1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_RST_N` reader - Timer0 rst n"]
pub type Timer0RstNR = crate::BitReader;
#[doc = "Field `TIMER0_RST_N` writer - Timer0 rst n"]
pub type Timer0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOM_RST_N` reader - Iom rst n"]
pub type IomRstNR = crate::BitReader;
#[doc = "Field `IOM_RST_N` writer - Iom rst n"]
pub type IomRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LORA_RST_N` reader - Lora rst n"]
pub type LoraRstNR = crate::BitReader;
#[doc = "Field `LORA_RST_N` writer - Lora rst n"]
pub type LoraRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_RST_N` reader - Dac rst n"]
pub type DacRstNR = crate::BitReader;
#[doc = "Field `DAC_RST_N` writer - Dac rst n"]
pub type DacRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_RST_N` reader - Lcd rst n"]
pub type LcdRstNR = crate::BitReader;
#[doc = "Field `LCD_RST_N` writer - Lcd rst n"]
pub type LcdRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFEC_RST_N` reader - Afec rst n"]
pub type AfecRstNR = crate::BitReader;
#[doc = "Field `AFEC_RST_N` writer - Afec rst n"]
pub type AfecRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RST_N` reader - Adc rst n"]
pub type AdcRstNR = crate::BitReader;
#[doc = "Field `ADC_RST_N` writer - Adc rst n"]
pub type AdcRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_RST_N` reader - I2c2 rst n"]
pub type I2c2RstNR = crate::BitReader;
#[doc = "Field `I2C2_RST_N` writer - I2c2 rst n"]
pub type I2c2RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_RST_N` reader - I2c1 rst n"]
pub type I2c1RstNR = crate::BitReader;
#[doc = "Field `I2C1_RST_N` writer - I2c1 rst n"]
pub type I2c1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_RST_N` reader - I2c0 rst n"]
pub type I2c0RstNR = crate::BitReader;
#[doc = "Field `I2C0_RST_N` writer - I2c0 rst n"]
pub type I2c0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_RST_N` reader - Qspi rst n"]
pub type QspiRstNR = crate::BitReader;
#[doc = "Field `QSPI_RST_N` writer - Qspi rst n"]
pub type QspiRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP2_RST_N` reader - Ssp2 rst n"]
pub type Ssp2RstNR = crate::BitReader;
#[doc = "Field `SSP2_RST_N` writer - Ssp2 rst n"]
pub type Ssp2RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP1_RST_N` reader - Ssp1 rst n"]
pub type Ssp1RstNR = crate::BitReader;
#[doc = "Field `SSP1_RST_N` writer - Ssp1 rst n"]
pub type Ssp1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP0_RST_N` reader - Ssp0 rst n"]
pub type Ssp0RstNR = crate::BitReader;
#[doc = "Field `SSP0_RST_N` writer - Ssp0 rst n"]
pub type Ssp0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART_RST_N` reader - Lpuart rst n"]
pub type LpuartRstNR = crate::BitReader;
#[doc = "Field `LPUART_RST_N` writer - Lpuart rst n"]
pub type LpuartRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_RST_N` reader - Uart3 rst n"]
pub type Uart3RstNR = crate::BitReader;
#[doc = "Field `UART3_RST_N` writer - Uart3 rst n"]
pub type Uart3RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_RST_N` reader - Uart2 rst n"]
pub type Uart2RstNR = crate::BitReader;
#[doc = "Field `UART2_RST_N` writer - Uart2 rst n"]
pub type Uart2RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RST_N` reader - Uart1 rst n"]
pub type Uart1RstNR = crate::BitReader;
#[doc = "Field `UART1_RST_N` writer - Uart1 rst n"]
pub type Uart1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_RST_N` reader - Uart0 rst n"]
pub type Uart0RstNR = crate::BitReader;
#[doc = "Field `UART0_RST_N` writer - Uart0 rst n"]
pub type Uart0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sac rst n"]
    #[inline(always)]
    pub fn sac_rst_n(&self) -> SacRstNR {
        SacRstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sec rst n"]
    #[inline(always)]
    pub fn sec_rst_n(&self) -> SecRstNR {
        SecRstNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crc rst n"]
    #[inline(always)]
    pub fn crc_rst_n(&self) -> CrcRstNR {
        CrcRstNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rtc rst n"]
    #[inline(always)]
    pub fn rtc_rst_n(&self) -> RtcRstNR {
        RtcRstNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wdg rst n"]
    #[inline(always)]
    pub fn wdg_rst_n(&self) -> WdgRstNR {
        WdgRstNR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Iwdg rst n"]
    #[inline(always)]
    pub fn iwdg_rst_n(&self) -> IwdgRstNR {
        IwdgRstNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lptimer0 rst n"]
    #[inline(always)]
    pub fn lptimer0_rst_n(&self) -> Lptimer0RstNR {
        Lptimer0RstNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bstimer1 rst n"]
    #[inline(always)]
    pub fn bstimer1_rst_n(&self) -> Bstimer1RstNR {
        Bstimer1RstNR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bstimer0 rst n"]
    #[inline(always)]
    pub fn bstimer0_rst_n(&self) -> Bstimer0RstNR {
        Bstimer0RstNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer3 rst n"]
    #[inline(always)]
    pub fn timer3_rst_n(&self) -> Timer3RstNR {
        Timer3RstNR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer2 rst n"]
    #[inline(always)]
    pub fn timer2_rst_n(&self) -> Timer2RstNR {
        Timer2RstNR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 rst n"]
    #[inline(always)]
    pub fn timer1_rst_n(&self) -> Timer1RstNR {
        Timer1RstNR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer0 rst n"]
    #[inline(always)]
    pub fn timer0_rst_n(&self) -> Timer0RstNR {
        Timer0RstNR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Iom rst n"]
    #[inline(always)]
    pub fn iom_rst_n(&self) -> IomRstNR {
        IomRstNR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Lora rst n"]
    #[inline(always)]
    pub fn lora_rst_n(&self) -> LoraRstNR {
        LoraRstNR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Dac rst n"]
    #[inline(always)]
    pub fn dac_rst_n(&self) -> DacRstNR {
        DacRstNR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lcd rst n"]
    #[inline(always)]
    pub fn lcd_rst_n(&self) -> LcdRstNR {
        LcdRstNR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Afec rst n"]
    #[inline(always)]
    pub fn afec_rst_n(&self) -> AfecRstNR {
        AfecRstNR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Adc rst n"]
    #[inline(always)]
    pub fn adc_rst_n(&self) -> AdcRstNR {
        AdcRstNR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - I2c2 rst n"]
    #[inline(always)]
    pub fn i2c2_rst_n(&self) -> I2c2RstNR {
        I2c2RstNR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2c1 rst n"]
    #[inline(always)]
    pub fn i2c1_rst_n(&self) -> I2c1RstNR {
        I2c1RstNR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2c0 rst n"]
    #[inline(always)]
    pub fn i2c0_rst_n(&self) -> I2c0RstNR {
        I2c0RstNR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Qspi rst n"]
    #[inline(always)]
    pub fn qspi_rst_n(&self) -> QspiRstNR {
        QspiRstNR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Ssp2 rst n"]
    #[inline(always)]
    pub fn ssp2_rst_n(&self) -> Ssp2RstNR {
        Ssp2RstNR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ssp1 rst n"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> Ssp1RstNR {
        Ssp1RstNR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ssp0 rst n"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> Ssp0RstNR {
        Ssp0RstNR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lpuart rst n"]
    #[inline(always)]
    pub fn lpuart_rst_n(&self) -> LpuartRstNR {
        LpuartRstNR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Uart3 rst n"]
    #[inline(always)]
    pub fn uart3_rst_n(&self) -> Uart3RstNR {
        Uart3RstNR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Uart2 rst n"]
    #[inline(always)]
    pub fn uart2_rst_n(&self) -> Uart2RstNR {
        Uart2RstNR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Uart1 rst n"]
    #[inline(always)]
    pub fn uart1_rst_n(&self) -> Uart1RstNR {
        Uart1RstNR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Uart0 rst n"]
    #[inline(always)]
    pub fn uart0_rst_n(&self) -> Uart0RstNR {
        Uart0RstNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sac rst n"]
    #[inline(always)]
    pub fn sac_rst_n(&mut self) -> SacRstNW<'_, Rst0Spec> {
        SacRstNW::new(self, 0)
    }
    #[doc = "Bit 1 - Sec rst n"]
    #[inline(always)]
    pub fn sec_rst_n(&mut self) -> SecRstNW<'_, Rst0Spec> {
        SecRstNW::new(self, 1)
    }
    #[doc = "Bit 2 - Crc rst n"]
    #[inline(always)]
    pub fn crc_rst_n(&mut self) -> CrcRstNW<'_, Rst0Spec> {
        CrcRstNW::new(self, 2)
    }
    #[doc = "Bit 3 - Rtc rst n"]
    #[inline(always)]
    pub fn rtc_rst_n(&mut self) -> RtcRstNW<'_, Rst0Spec> {
        RtcRstNW::new(self, 3)
    }
    #[doc = "Bit 4 - Wdg rst n"]
    #[inline(always)]
    pub fn wdg_rst_n(&mut self) -> WdgRstNW<'_, Rst0Spec> {
        WdgRstNW::new(self, 4)
    }
    #[doc = "Bit 5 - Iwdg rst n"]
    #[inline(always)]
    pub fn iwdg_rst_n(&mut self) -> IwdgRstNW<'_, Rst0Spec> {
        IwdgRstNW::new(self, 5)
    }
    #[doc = "Bit 6 - Lptimer0 rst n"]
    #[inline(always)]
    pub fn lptimer0_rst_n(&mut self) -> Lptimer0RstNW<'_, Rst0Spec> {
        Lptimer0RstNW::new(self, 6)
    }
    #[doc = "Bit 7 - Bstimer1 rst n"]
    #[inline(always)]
    pub fn bstimer1_rst_n(&mut self) -> Bstimer1RstNW<'_, Rst0Spec> {
        Bstimer1RstNW::new(self, 7)
    }
    #[doc = "Bit 8 - Bstimer0 rst n"]
    #[inline(always)]
    pub fn bstimer0_rst_n(&mut self) -> Bstimer0RstNW<'_, Rst0Spec> {
        Bstimer0RstNW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer3 rst n"]
    #[inline(always)]
    pub fn timer3_rst_n(&mut self) -> Timer3RstNW<'_, Rst0Spec> {
        Timer3RstNW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer2 rst n"]
    #[inline(always)]
    pub fn timer2_rst_n(&mut self) -> Timer2RstNW<'_, Rst0Spec> {
        Timer2RstNW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer1 rst n"]
    #[inline(always)]
    pub fn timer1_rst_n(&mut self) -> Timer1RstNW<'_, Rst0Spec> {
        Timer1RstNW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer0 rst n"]
    #[inline(always)]
    pub fn timer0_rst_n(&mut self) -> Timer0RstNW<'_, Rst0Spec> {
        Timer0RstNW::new(self, 12)
    }
    #[doc = "Bit 13 - Iom rst n"]
    #[inline(always)]
    pub fn iom_rst_n(&mut self) -> IomRstNW<'_, Rst0Spec> {
        IomRstNW::new(self, 13)
    }
    #[doc = "Bit 14 - Lora rst n"]
    #[inline(always)]
    pub fn lora_rst_n(&mut self) -> LoraRstNW<'_, Rst0Spec> {
        LoraRstNW::new(self, 14)
    }
    #[doc = "Bit 15 - Dac rst n"]
    #[inline(always)]
    pub fn dac_rst_n(&mut self) -> DacRstNW<'_, Rst0Spec> {
        DacRstNW::new(self, 15)
    }
    #[doc = "Bit 16 - Lcd rst n"]
    #[inline(always)]
    pub fn lcd_rst_n(&mut self) -> LcdRstNW<'_, Rst0Spec> {
        LcdRstNW::new(self, 16)
    }
    #[doc = "Bit 17 - Afec rst n"]
    #[inline(always)]
    pub fn afec_rst_n(&mut self) -> AfecRstNW<'_, Rst0Spec> {
        AfecRstNW::new(self, 17)
    }
    #[doc = "Bit 18 - Adc rst n"]
    #[inline(always)]
    pub fn adc_rst_n(&mut self) -> AdcRstNW<'_, Rst0Spec> {
        AdcRstNW::new(self, 18)
    }
    #[doc = "Bit 20 - I2c2 rst n"]
    #[inline(always)]
    pub fn i2c2_rst_n(&mut self) -> I2c2RstNW<'_, Rst0Spec> {
        I2c2RstNW::new(self, 20)
    }
    #[doc = "Bit 21 - I2c1 rst n"]
    #[inline(always)]
    pub fn i2c1_rst_n(&mut self) -> I2c1RstNW<'_, Rst0Spec> {
        I2c1RstNW::new(self, 21)
    }
    #[doc = "Bit 22 - I2c0 rst n"]
    #[inline(always)]
    pub fn i2c0_rst_n(&mut self) -> I2c0RstNW<'_, Rst0Spec> {
        I2c0RstNW::new(self, 22)
    }
    #[doc = "Bit 23 - Qspi rst n"]
    #[inline(always)]
    pub fn qspi_rst_n(&mut self) -> QspiRstNW<'_, Rst0Spec> {
        QspiRstNW::new(self, 23)
    }
    #[doc = "Bit 24 - Ssp2 rst n"]
    #[inline(always)]
    pub fn ssp2_rst_n(&mut self) -> Ssp2RstNW<'_, Rst0Spec> {
        Ssp2RstNW::new(self, 24)
    }
    #[doc = "Bit 25 - Ssp1 rst n"]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> Ssp1RstNW<'_, Rst0Spec> {
        Ssp1RstNW::new(self, 25)
    }
    #[doc = "Bit 26 - Ssp0 rst n"]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> Ssp0RstNW<'_, Rst0Spec> {
        Ssp0RstNW::new(self, 26)
    }
    #[doc = "Bit 27 - Lpuart rst n"]
    #[inline(always)]
    pub fn lpuart_rst_n(&mut self) -> LpuartRstNW<'_, Rst0Spec> {
        LpuartRstNW::new(self, 27)
    }
    #[doc = "Bit 28 - Uart3 rst n"]
    #[inline(always)]
    pub fn uart3_rst_n(&mut self) -> Uart3RstNW<'_, Rst0Spec> {
        Uart3RstNW::new(self, 28)
    }
    #[doc = "Bit 29 - Uart2 rst n"]
    #[inline(always)]
    pub fn uart2_rst_n(&mut self) -> Uart2RstNW<'_, Rst0Spec> {
        Uart2RstNW::new(self, 29)
    }
    #[doc = "Bit 30 - Uart1 rst n"]
    #[inline(always)]
    pub fn uart1_rst_n(&mut self) -> Uart1RstNW<'_, Rst0Spec> {
        Uart1RstNW::new(self, 30)
    }
    #[doc = "Bit 31 - Uart0 rst n"]
    #[inline(always)]
    pub fn uart0_rst_n(&mut self) -> Uart0RstNW<'_, Rst0Spec> {
        Uart0RstNW::new(self, 31)
    }
}
#[doc = "reset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst0Spec;
impl crate::RegisterSpec for Rst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst0::R`](R) reader structure"]
impl crate::Readable for Rst0Spec {}
#[doc = "`write(|w| ..)` method takes [`rst0::W`](W) writer structure"]
impl crate::Writable for Rst0Spec {
    type Safety = crate::Unsafe;
}
