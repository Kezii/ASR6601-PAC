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
#[doc = "Field `WWDG_CLK_EN` reader - Wwdg clk en"]
pub type WwdgClkEnR = crate::BitReader;
#[doc = "Field `WWDG_CLK_EN` writer - Wwdg clk en"]
pub type WwdgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_CLK_EN` reader - Iwdg clk en"]
pub type IwdgClkEnR = crate::BitReader;
#[doc = "Field `IWDG_CLK_EN` writer - Iwdg clk en"]
pub type IwdgClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM0_CLK_EN` reader - Lptim0 clk en"]
pub type Lptim0ClkEnR = crate::BitReader;
#[doc = "Field `LPTIM0_CLK_EN` writer - Lptim0 clk en"]
pub type Lptim0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_CLK_EN` reader - Qspi clk en"]
pub type QspiClkEnR = crate::BitReader;
#[doc = "Field `QSPI_CLK_EN` writer - Qspi clk en"]
pub type QspiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_CNT_CLK_EN` reader - Wwdg cnt clk en"]
pub type WwdgCntClkEnR = crate::BitReader;
#[doc = "Field `WWDG_CNT_CLK_EN` writer - Wwdg cnt clk en"]
pub type WwdgCntClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAC_CLK_EN` reader - Sac clk en"]
pub type SacClkEnR = crate::BitReader;
#[doc = "Field `SAC_CLK_EN` writer - Sac clk en"]
pub type SacClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_CLK_EN` reader - I2s clk en"]
pub type I2sClkEnR = crate::BitReader;
#[doc = "Field `I2S_CLK_EN` writer - I2s clk en"]
pub type I2sClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM0_INF_CLK_EN` reader - Lptim0 inf clk en"]
pub type Lptim0InfClkEnR = crate::BitReader;
#[doc = "Field `LPTIM0_INF_CLK_EN` writer - Lptim0 inf clk en"]
pub type Lptim0InfClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGC_CLK_EN` reader - Rngc clk en"]
pub type RngcClkEnR = crate::BitReader;
#[doc = "Field `RNGC_CLK_EN` writer - Rngc clk en"]
pub type RngcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1_CLK_EN` reader - Lptim1 clk en"]
pub type Lptim1ClkEnR = crate::BitReader;
#[doc = "Field `LPTIM1_CLK_EN` writer - Lptim1 clk en"]
pub type Lptim1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1_INF_CLK_EN` reader - Lptim1 inf clk en"]
pub type Lptim1InfClkEnR = crate::BitReader;
#[doc = "Field `LPTIM1_INF_CLK_EN` writer - Lptim1 inf clk en"]
pub type Lptim1InfClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Wwdg clk en"]
    #[inline(always)]
    pub fn wwdg_clk_en(&self) -> WwdgClkEnR {
        WwdgClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&self) -> IwdgClkEnR {
        IwdgClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptim0 clk en"]
    #[inline(always)]
    pub fn lptim0_clk_en(&self) -> Lptim0ClkEnR {
        Lptim0ClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Qspi clk en"]
    #[inline(always)]
    pub fn qspi_clk_en(&self) -> QspiClkEnR {
        QspiClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wwdg cnt clk en"]
    #[inline(always)]
    pub fn wwdg_cnt_clk_en(&self) -> WwdgCntClkEnR {
        WwdgCntClkEnR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 9 - Lptim0 inf clk en"]
    #[inline(always)]
    pub fn lptim0_inf_clk_en(&self) -> Lptim0InfClkEnR {
        Lptim0InfClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rngc clk en"]
    #[inline(always)]
    pub fn rngc_clk_en(&self) -> RngcClkEnR {
        RngcClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lptim1 clk en"]
    #[inline(always)]
    pub fn lptim1_clk_en(&self) -> Lptim1ClkEnR {
        Lptim1ClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lptim1 inf clk en"]
    #[inline(always)]
    pub fn lptim1_inf_clk_en(&self) -> Lptim1InfClkEnR {
        Lptim1InfClkEnR::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 2 - Wwdg clk en"]
    #[inline(always)]
    pub fn wwdg_clk_en(&mut self) -> WwdgClkEnW<'_, Cgr1Spec> {
        WwdgClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Iwdg clk en"]
    #[inline(always)]
    pub fn iwdg_clk_en(&mut self) -> IwdgClkEnW<'_, Cgr1Spec> {
        IwdgClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Lptim0 clk en"]
    #[inline(always)]
    pub fn lptim0_clk_en(&mut self) -> Lptim0ClkEnW<'_, Cgr1Spec> {
        Lptim0ClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Qspi clk en"]
    #[inline(always)]
    pub fn qspi_clk_en(&mut self) -> QspiClkEnW<'_, Cgr1Spec> {
        QspiClkEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Wwdg cnt clk en"]
    #[inline(always)]
    pub fn wwdg_cnt_clk_en(&mut self) -> WwdgCntClkEnW<'_, Cgr1Spec> {
        WwdgCntClkEnW::new(self, 6)
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
    #[doc = "Bit 9 - Lptim0 inf clk en"]
    #[inline(always)]
    pub fn lptim0_inf_clk_en(&mut self) -> Lptim0InfClkEnW<'_, Cgr1Spec> {
        Lptim0InfClkEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Rngc clk en"]
    #[inline(always)]
    pub fn rngc_clk_en(&mut self) -> RngcClkEnW<'_, Cgr1Spec> {
        RngcClkEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Lptim1 clk en"]
    #[inline(always)]
    pub fn lptim1_clk_en(&mut self) -> Lptim1ClkEnW<'_, Cgr1Spec> {
        Lptim1ClkEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Lptim1 inf clk en"]
    #[inline(always)]
    pub fn lptim1_inf_clk_en(&mut self) -> Lptim1InfClkEnW<'_, Cgr1Spec> {
        Lptim1InfClkEnW::new(self, 12)
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
