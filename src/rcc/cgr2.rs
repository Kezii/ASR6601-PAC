#[doc = "Register `CGR2` reader"]
pub type R = crate::R<Cgr2Spec>;
#[doc = "Register `CGR2` writer"]
pub type W = crate::W<Cgr2Spec>;
#[doc = "Field `IWDG_CLK_EN` reader - Iwdg clk en"]
pub type IwdgClkEnR = crate::BitReader;
#[doc = "Field `IWDG_CLK_EN` writer - Iwdg clk en"]
pub type IwdgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_AON_CLK_EN` reader - Rtc aon clk en"]
pub type RtcAonClkEnR = crate::BitReader;
#[doc = "Field `RTC_AON_CLK_EN` writer - Rtc aon clk en"]
pub type RtcAonClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART_AON_CLK_EN` reader - Lpuart aon clk en"]
pub type LpuartAonClkEnR = crate::BitReader;
#[doc = "Field `LPUART_AON_CLK_EN` writer - Lpuart aon clk en"]
pub type LpuartAonClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_AON_CLK_EN` reader - Lcd aon clk en"]
pub type LcdAonClkEnR = crate::BitReader;
#[doc = "Field `LCD_AON_CLK_EN` writer - Lcd aon clk en"]
pub type LcdAonClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER0_AON_CLK_EN` reader - Lptimer0 aon clk en"]
pub type Lptimer0AonClkEnR = crate::BitReader;
#[doc = "Field `LPTIMER0_AON_CLK_EN` writer - Lptimer0 aon clk en"]
pub type Lptimer0AonClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER1_AON_CLK_EN` reader - Lptimer1 aon clk en"]
pub type Lptimer1AonClkEnR = crate::BitReader;
#[doc = "Field `LPTIMER1_AON_CLK_EN` writer - Lptimer1 aon clk en"]
pub type Lptimer1AonClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&self) -> IwdgClkEnR {
        IwdgClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc aon clk en"]
    #[inline(always)]
    pub fn rtc_aon_clk_en(&self) -> RtcAonClkEnR {
        RtcAonClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lpuart aon clk en"]
    #[inline(always)]
    pub fn lpuart_aon_clk_en(&self) -> LpuartAonClkEnR {
        LpuartAonClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lcd aon clk en"]
    #[inline(always)]
    pub fn lcd_aon_clk_en(&self) -> LcdAonClkEnR {
        LcdAonClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptimer0 aon clk en"]
    #[inline(always)]
    pub fn lptimer0_aon_clk_en(&self) -> Lptimer0AonClkEnR {
        Lptimer0AonClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lptimer1 aon clk en"]
    #[inline(always)]
    pub fn lptimer1_aon_clk_en(&self) -> Lptimer1AonClkEnR {
        Lptimer1AonClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&mut self) -> IwdgClkEnW<'_, Cgr2Spec> {
        IwdgClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Rtc aon clk en"]
    #[inline(always)]
    pub fn rtc_aon_clk_en(&mut self) -> RtcAonClkEnW<'_, Cgr2Spec> {
        RtcAonClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Lpuart aon clk en"]
    #[inline(always)]
    pub fn lpuart_aon_clk_en(&mut self) -> LpuartAonClkEnW<'_, Cgr2Spec> {
        LpuartAonClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Lcd aon clk en"]
    #[inline(always)]
    pub fn lcd_aon_clk_en(&mut self) -> LcdAonClkEnW<'_, Cgr2Spec> {
        LcdAonClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Lptimer0 aon clk en"]
    #[inline(always)]
    pub fn lptimer0_aon_clk_en(&mut self) -> Lptimer0AonClkEnW<'_, Cgr2Spec> {
        Lptimer0AonClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Lptimer1 aon clk en"]
    #[inline(always)]
    pub fn lptimer1_aon_clk_en(&mut self) -> Lptimer1AonClkEnW<'_, Cgr2Spec> {
        Lptimer1AonClkEnW::new(self, 5)
    }
}
#[doc = "clock generation register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgr2Spec;
impl crate::RegisterSpec for Cgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgr2::R`](R) reader structure"]
impl crate::Readable for Cgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cgr2::W`](W) writer structure"]
impl crate::Writable for Cgr2Spec {
    type Safety = crate::Unsafe;
}
