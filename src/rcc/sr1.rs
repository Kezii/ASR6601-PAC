#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `IWDG_AON_CLK_EN_SYNC` reader - Iwdg aon clk en sync"]
pub type IwdgAonClkEnSyncR = crate::BitReader;
#[doc = "Field `RTC_AON_CLK_EN_SYNC` reader - Rtc aon clk en sync"]
pub type RtcAonClkEnSyncR = crate::BitReader;
#[doc = "Field `LPUART_AON_CLK_EN_SYNC` reader - Lpuart aon clk en sync"]
pub type LpuartAonClkEnSyncR = crate::BitReader;
#[doc = "Field `LCD_AON_CLK_EN_SYNC` reader - Lcd aon clk en sync"]
pub type LcdAonClkEnSyncR = crate::BitReader;
#[doc = "Field `LPTIMER0_AON_CLK_EN_SYNC` reader - Lptimer0 aon clk en sync"]
pub type Lptimer0AonClkEnSyncR = crate::BitReader;
#[doc = "Field `I2S_CLK_EN_SYNC` reader - I2s clk en sync"]
pub type I2sClkEnSyncR = crate::BitReader;
#[doc = "Field `MCO_CLK_EN_SYNC` reader - Mco clk en sync"]
pub type McoClkEnSyncR = crate::BitReader;
#[doc = "Field `RTC_CLK_EN_SYNC` reader - Rtc clk en sync"]
pub type RtcClkEnSyncR = crate::BitReader;
#[doc = "Field `IWDG_CLK_EN_SYNC` reader - Iwdg clk en sync"]
pub type IwdgClkEnSyncR = crate::BitReader;
#[doc = "Field `LCD_CLK_EN_SYNC` reader - Lcd clk en sync"]
pub type LcdClkEnSyncR = crate::BitReader;
#[doc = "Field `LPUART_CLK_EN_SYNC` reader - Lpuart clk en sync"]
pub type LpuartClkEnSyncR = crate::BitReader;
#[doc = "Field `QSPI_CLK_EN_SYNC` reader - Qspi clk en sync"]
pub type QspiClkEnSyncR = crate::BitReader;
#[doc = "Field `LPTIMER0_CLK_EN_SYNC` reader - Lptimer0 clk en sync"]
pub type Lptimer0ClkEnSyncR = crate::BitReader;
#[doc = "Field `ADC_CLK_EN_SYNC` reader - Adc clk en sync"]
pub type AdcClkEnSyncR = crate::BitReader;
#[doc = "Field `SCC_CLK_EN_SYNC` reader - Scc clk en sync"]
pub type SccClkEnSyncR = crate::BitReader;
#[doc = "Field `UART3_CLK_EN_SYNC` reader - Uart3 clk en sync"]
pub type Uart3ClkEnSyncR = crate::BitReader;
#[doc = "Field `UART2_CLK_EN_SYNC` reader - Uart2 clk en sync"]
pub type Uart2ClkEnSyncR = crate::BitReader;
#[doc = "Field `UART1_CLK_EN_SYNC` reader - Uart1 clk en sync"]
pub type Uart1ClkEnSyncR = crate::BitReader;
#[doc = "Field `UART0_CLK_EN_SYNC` reader - Uart0 clk en sync"]
pub type Uart0ClkEnSyncR = crate::BitReader;
#[doc = "Field `LPTIMER1_AON_CLK_EN_SYNC` reader - Lptimer1 aon clk en sync"]
pub type Lptimer1AonClkEnSyncR = crate::BitReader;
#[doc = "Field `LPTIMER1_CLK_EN_SYNC` reader - Lptimer1 clk en sync"]
pub type Lptimer1ClkEnSyncR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Iwdg aon clk en sync"]
    #[inline(always)]
    pub fn iwdg_aon_clk_en_sync(&self) -> IwdgAonClkEnSyncR {
        IwdgAonClkEnSyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc aon clk en sync"]
    #[inline(always)]
    pub fn rtc_aon_clk_en_sync(&self) -> RtcAonClkEnSyncR {
        RtcAonClkEnSyncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lpuart aon clk en sync"]
    #[inline(always)]
    pub fn lpuart_aon_clk_en_sync(&self) -> LpuartAonClkEnSyncR {
        LpuartAonClkEnSyncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lcd aon clk en sync"]
    #[inline(always)]
    pub fn lcd_aon_clk_en_sync(&self) -> LcdAonClkEnSyncR {
        LcdAonClkEnSyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptimer0 aon clk en sync"]
    #[inline(always)]
    pub fn lptimer0_aon_clk_en_sync(&self) -> Lptimer0AonClkEnSyncR {
        Lptimer0AonClkEnSyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2s clk en sync"]
    #[inline(always)]
    pub fn i2s_clk_en_sync(&self) -> I2sClkEnSyncR {
        I2sClkEnSyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mco clk en sync"]
    #[inline(always)]
    pub fn mco_clk_en_sync(&self) -> McoClkEnSyncR {
        McoClkEnSyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rtc clk en sync"]
    #[inline(always)]
    pub fn rtc_clk_en_sync(&self) -> RtcClkEnSyncR {
        RtcClkEnSyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Iwdg clk en sync"]
    #[inline(always)]
    pub fn iwdg_clk_en_sync(&self) -> IwdgClkEnSyncR {
        IwdgClkEnSyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lcd clk en sync"]
    #[inline(always)]
    pub fn lcd_clk_en_sync(&self) -> LcdClkEnSyncR {
        LcdClkEnSyncR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lpuart clk en sync"]
    #[inline(always)]
    pub fn lpuart_clk_en_sync(&self) -> LpuartClkEnSyncR {
        LpuartClkEnSyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Qspi clk en sync"]
    #[inline(always)]
    pub fn qspi_clk_en_sync(&self) -> QspiClkEnSyncR {
        QspiClkEnSyncR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lptimer0 clk en sync"]
    #[inline(always)]
    pub fn lptimer0_clk_en_sync(&self) -> Lptimer0ClkEnSyncR {
        Lptimer0ClkEnSyncR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Adc clk en sync"]
    #[inline(always)]
    pub fn adc_clk_en_sync(&self) -> AdcClkEnSyncR {
        AdcClkEnSyncR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Scc clk en sync"]
    #[inline(always)]
    pub fn scc_clk_en_sync(&self) -> SccClkEnSyncR {
        SccClkEnSyncR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Uart3 clk en sync"]
    #[inline(always)]
    pub fn uart3_clk_en_sync(&self) -> Uart3ClkEnSyncR {
        Uart3ClkEnSyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Uart2 clk en sync"]
    #[inline(always)]
    pub fn uart2_clk_en_sync(&self) -> Uart2ClkEnSyncR {
        Uart2ClkEnSyncR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Uart1 clk en sync"]
    #[inline(always)]
    pub fn uart1_clk_en_sync(&self) -> Uart1ClkEnSyncR {
        Uart1ClkEnSyncR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Uart0 clk en sync"]
    #[inline(always)]
    pub fn uart0_clk_en_sync(&self) -> Uart0ClkEnSyncR {
        Uart0ClkEnSyncR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lptimer1 aon clk en sync"]
    #[inline(always)]
    pub fn lptimer1_aon_clk_en_sync(&self) -> Lptimer1AonClkEnSyncR {
        Lptimer1AonClkEnSyncR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lptimer1 clk en sync"]
    #[inline(always)]
    pub fn lptimer1_clk_en_sync(&self) -> Lptimer1ClkEnSyncR {
        Lptimer1ClkEnSyncR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
