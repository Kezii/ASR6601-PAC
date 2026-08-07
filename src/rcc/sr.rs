#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `IWDG_AON_CLK_EN_DONE` reader - Iwdg aon clk en done"]
pub type IwdgAonClkEnDoneR = crate::BitReader;
#[doc = "Field `RTC_AON_CLK_EN_DONE` reader - Rtc aon clk en done"]
pub type RtcAonClkEnDoneR = crate::BitReader;
#[doc = "Field `LPUART_AON_CLK_EN_DONE` reader - Lpuart aon clk en done"]
pub type LpuartAonClkEnDoneR = crate::BitReader;
#[doc = "Field `LCD_AON_CLK_EN_DONE` reader - Lcd aon clk en done"]
pub type LcdAonClkEnDoneR = crate::BitReader;
#[doc = "Field `LPTIM_AON_CLK_EN_DONE` reader - Lptim aon clk en done"]
pub type LptimAonClkEnDoneR = crate::BitReader;
#[doc = "Field `LPTIMER1_AON_CLK_EN_DONE` reader - Lptimer1 aon clk en done"]
pub type Lptimer1AonClkEnDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Iwdg aon clk en done"]
    #[inline(always)]
    pub fn iwdg_aon_clk_en_done(&self) -> IwdgAonClkEnDoneR {
        IwdgAonClkEnDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc aon clk en done"]
    #[inline(always)]
    pub fn rtc_aon_clk_en_done(&self) -> RtcAonClkEnDoneR {
        RtcAonClkEnDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lpuart aon clk en done"]
    #[inline(always)]
    pub fn lpuart_aon_clk_en_done(&self) -> LpuartAonClkEnDoneR {
        LpuartAonClkEnDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lcd aon clk en done"]
    #[inline(always)]
    pub fn lcd_aon_clk_en_done(&self) -> LcdAonClkEnDoneR {
        LcdAonClkEnDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptim aon clk en done"]
    #[inline(always)]
    pub fn lptim_aon_clk_en_done(&self) -> LptimAonClkEnDoneR {
        LptimAonClkEnDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lptimer1 aon clk en done"]
    #[inline(always)]
    pub fn lptimer1_aon_clk_en_done(&self) -> Lptimer1AonClkEnDoneR {
        Lptimer1AonClkEnDoneR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
