#[doc = "Register `CGR1` reader"]
pub type R = crate::R<Cgr1Spec>;
#[doc = "Register `CGR1` writer"]
pub type W = crate::W<Cgr1Spec>;
#[doc = "Field `SEC_CLK_EN` reader - Sec clk en"]
pub type SecClkEnR = crate::BitReader;
#[doc = "Field `SEC_CLK_EN` writer - Sec clk en"]
pub type SecClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_CLK_EN` reader - Rtc clk en"]
pub type RtcClkEnR = crate::BitReader;
#[doc = "Field `RTC_CLK_EN` writer - Rtc clk en"]
pub type RtcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_CLK_EN` reader - Wdg clk en"]
pub type WdgClkEnR = crate::BitReader;
#[doc = "Field `WDG_CLK_EN` writer - Wdg clk en"]
pub type WdgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_CLK_EN` reader - Iwdg clk en"]
pub type IwdgClkEnR = crate::BitReader;
#[doc = "Field `IWDG_CLK_EN` writer - Iwdg clk en"]
pub type IwdgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER0_CLK_EN` reader - Lptimer0 clk en"]
pub type Lptimer0ClkEnR = crate::BitReader;
#[doc = "Field `LPTIMER0_CLK_EN` writer - Lptimer0 clk en"]
pub type Lptimer0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_CLK_EN` reader - Qspi clk en"]
pub type QspiClkEnR = crate::BitReader;
#[doc = "Field `QSPI_CLK_EN` writer - Qspi clk en"]
pub type QspiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_CNT_CLK_EN` reader - Wdg cnt clk en"]
pub type WdgCntClkEnR = crate::BitReader;
#[doc = "Field `WDG_CNT_CLK_EN` writer - Wdg cnt clk en"]
pub type WdgCntClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAC_CLK_EN` reader - Sac clk en"]
pub type SacClkEnR = crate::BitReader;
#[doc = "Field `SAC_CLK_EN` writer - Sac clk en"]
pub type SacClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_CLK_EN` reader - I2s clk en"]
pub type I2sClkEnR = crate::BitReader;
#[doc = "Field `I2S_CLK_EN` writer - I2s clk en"]
pub type I2sClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER0_PCLK_EN` reader - Lptimer0 pclk en"]
pub type Lptimer0PclkEnR = crate::BitReader;
#[doc = "Field `LPTIMER0_PCLK_EN` writer - Lptimer0 pclk en"]
pub type Lptimer0PclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGC_CLK_EN` reader - Rngc clk en"]
pub type RngcClkEnR = crate::BitReader;
#[doc = "Field `RNGC_CLK_EN` writer - Rngc clk en"]
pub type RngcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER1_CLK_EN` reader - Lptimer1 clk en"]
pub type Lptimer1ClkEnR = crate::BitReader;
#[doc = "Field `LPTIMER1_CLK_EN` writer - Lptimer1 clk en"]
pub type Lptimer1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER1_PCLK_EN` reader - Lptimer1 pclk en"]
pub type Lptimer1PclkEnR = crate::BitReader;
#[doc = "Field `LPTIMER1_PCLK_EN` writer - Lptimer1 pclk en"]
pub type Lptimer1PclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sec clk en"]
    #[inline(always)]
    pub fn sec_clk_en(&self) -> SecClkEnR {
        SecClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc clk en"]
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> RtcClkEnR {
        RtcClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wdg clk en"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WdgClkEnR {
        WdgClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&self) -> IwdgClkEnR {
        IwdgClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptimer0 clk en"]
    #[inline(always)]
    pub fn lptimer0_clk_en(&self) -> Lptimer0ClkEnR {
        Lptimer0ClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Qspi clk en"]
    #[inline(always)]
    pub fn qspi_clk_en(&self) -> QspiClkEnR {
        QspiClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wdg cnt clk en"]
    #[inline(always)]
    pub fn wdg_cnt_clk_en(&self) -> WdgCntClkEnR {
        WdgCntClkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sac clk en"]
    #[inline(always)]
    pub fn sac_clk_en(&self) -> SacClkEnR {
        SacClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2s clk en"]
    #[inline(always)]
    pub fn i2s_clk_en(&self) -> I2sClkEnR {
        I2sClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lptimer0 pclk en"]
    #[inline(always)]
    pub fn lptimer0_pclk_en(&self) -> Lptimer0PclkEnR {
        Lptimer0PclkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rngc clk en"]
    #[inline(always)]
    pub fn rngc_clk_en(&self) -> RngcClkEnR {
        RngcClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lptimer1 clk en"]
    #[inline(always)]
    pub fn lptimer1_clk_en(&self) -> Lptimer1ClkEnR {
        Lptimer1ClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lptimer1 pclk en"]
    #[inline(always)]
    pub fn lptimer1_pclk_en(&self) -> Lptimer1PclkEnR {
        Lptimer1PclkEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sec clk en"]
    #[inline(always)]
    pub fn sec_clk_en(&mut self) -> SecClkEnW<'_, Cgr1Spec> {
        SecClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Rtc clk en"]
    #[inline(always)]
    pub fn rtc_clk_en(&mut self) -> RtcClkEnW<'_, Cgr1Spec> {
        RtcClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Wdg clk en"]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WdgClkEnW<'_, Cgr1Spec> {
        WdgClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&mut self) -> IwdgClkEnW<'_, Cgr1Spec> {
        IwdgClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Lptimer0 clk en"]
    #[inline(always)]
    pub fn lptimer0_clk_en(&mut self) -> Lptimer0ClkEnW<'_, Cgr1Spec> {
        Lptimer0ClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Qspi clk en"]
    #[inline(always)]
    pub fn qspi_clk_en(&mut self) -> QspiClkEnW<'_, Cgr1Spec> {
        QspiClkEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Wdg cnt clk en"]
    #[inline(always)]
    pub fn wdg_cnt_clk_en(&mut self) -> WdgCntClkEnW<'_, Cgr1Spec> {
        WdgCntClkEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Sac clk en"]
    #[inline(always)]
    pub fn sac_clk_en(&mut self) -> SacClkEnW<'_, Cgr1Spec> {
        SacClkEnW::new(self, 7)
    }
    #[doc = "Bit 8 - I2s clk en"]
    #[inline(always)]
    pub fn i2s_clk_en(&mut self) -> I2sClkEnW<'_, Cgr1Spec> {
        I2sClkEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Lptimer0 pclk en"]
    #[inline(always)]
    pub fn lptimer0_pclk_en(&mut self) -> Lptimer0PclkEnW<'_, Cgr1Spec> {
        Lptimer0PclkEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Rngc clk en"]
    #[inline(always)]
    pub fn rngc_clk_en(&mut self) -> RngcClkEnW<'_, Cgr1Spec> {
        RngcClkEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Lptimer1 clk en"]
    #[inline(always)]
    pub fn lptimer1_clk_en(&mut self) -> Lptimer1ClkEnW<'_, Cgr1Spec> {
        Lptimer1ClkEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Lptimer1 pclk en"]
    #[inline(always)]
    pub fn lptimer1_pclk_en(&mut self) -> Lptimer1PclkEnW<'_, Cgr1Spec> {
        Lptimer1PclkEnW::new(self, 12)
    }
}
#[doc = "clock generation register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgr1Spec;
impl crate::RegisterSpec for Cgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgr1::R`](R) reader structure"]
impl crate::Readable for Cgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cgr1::W`](W) writer structure"]
impl crate::Writable for Cgr1Spec {
    type Safety = crate::Unsafe;
}
