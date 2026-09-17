#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `SET_IWDG_AON_CLK_EN_DONE` reader - Iwdg aon clk en done"]
pub type SetIwdgAonClkEnDoneR = crate::BitReader;
#[doc = "Field `SET_RTC_AON_CLK_EN_DONE` reader - Rtc aon clk en done"]
pub type SetRtcAonClkEnDoneR = crate::BitReader;
#[doc = "Field `SET_LPUART_AON_CLK_EN_DONE` reader - Lpuart aon clk en done"]
pub type SetLpuartAonClkEnDoneR = crate::BitReader;
#[doc = "Field `SET_LCDCTRL_AON_CLK_EN_DONE` reader - Lcdctrl aon clk en done"]
pub type SetLcdctrlAonClkEnDoneR = crate::BitReader;
#[doc = "Field `SET_LPTIM0_AON_CLK_EN_DONE` reader - Lptim0 aon clk en done"]
pub type SetLptim0AonClkEnDoneR = crate::BitReader;
#[doc = "Field `SET_LPTIM1_AON_CLK_EN_DONE` reader - Lptim1 aon clk en done"]
pub type SetLptim1AonClkEnDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Iwdg aon clk en done"]
    #[inline(always)]
    pub fn set_iwdg_aon_clk_en_done(&self) -> SetIwdgAonClkEnDoneR {
        SetIwdgAonClkEnDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc aon clk en done"]
    #[inline(always)]
    pub fn set_rtc_aon_clk_en_done(&self) -> SetRtcAonClkEnDoneR {
        SetRtcAonClkEnDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lpuart aon clk en done"]
    #[inline(always)]
    pub fn set_lpuart_aon_clk_en_done(&self) -> SetLpuartAonClkEnDoneR {
        SetLpuartAonClkEnDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lcdctrl aon clk en done"]
    #[inline(always)]
    pub fn set_lcdctrl_aon_clk_en_done(&self) -> SetLcdctrlAonClkEnDoneR {
        SetLcdctrlAonClkEnDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptim0 aon clk en done"]
    #[inline(always)]
    pub fn set_lptim0_aon_clk_en_done(&self) -> SetLptim0AonClkEnDoneR {
        SetLptim0AonClkEnDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lptim1 aon clk en done"]
    #[inline(always)]
    pub fn set_lptim1_aon_clk_en_done(&self) -> SetLptim1AonClkEnDoneR {
        SetLptim1AonClkEnDoneR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
