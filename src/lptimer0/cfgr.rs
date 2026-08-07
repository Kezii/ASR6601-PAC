#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `CKFLT_EN` reader - Ckflt en"]
pub type CkfltEnR = crate::BitReader;
#[doc = "Field `CKFLT_EN` writer - Ckflt en"]
pub type CkfltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGFLT_EN` reader - Trgflt en"]
pub type TrgfltEnR = crate::BitReader;
#[doc = "Field `TRGFLT_EN` writer - Trgflt en"]
pub type TrgfltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - Wave"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Wave"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVPOL` reader - Wavpol"]
pub type WavpolR = crate::BitReader;
#[doc = "Field `WAVPOL` writer - Wavpol"]
pub type WavpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELOAD` reader - Preload"]
pub type PreloadR = crate::BitReader;
#[doc = "Field `PRELOAD` writer - Preload"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTMODE` reader - Countmode"]
pub type CountmodeR = crate::BitReader;
#[doc = "Field `COUNTMODE` writer - Countmode"]
pub type CountmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - Enc"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - Enc"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM_WKUP` reader - Cmpm wkup"]
pub type CmpmWkupR = crate::BitReader;
#[doc = "Field `CMPM_WKUP` writer - Cmpm wkup"]
pub type CmpmWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRM_WKUP` reader - Arrm wkup"]
pub type ArrmWkupR = crate::BitReader;
#[doc = "Field `ARRM_WKUP` writer - Arrm wkup"]
pub type ArrmWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIG_WKUP` reader - Exttrig wkup"]
pub type ExttrigWkupR = crate::BitReader;
#[doc = "Field `EXTTRIG_WKUP` writer - Exttrig wkup"]
pub type ExttrigWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP_WKUP` reader - Up wkup"]
pub type UpWkupR = crate::BitReader;
#[doc = "Field `UP_WKUP` writer - Up wkup"]
pub type UpWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN_WKUP` reader - Down wkup"]
pub type DownWkupR = crate::BitReader;
#[doc = "Field `DOWN_WKUP` writer - Down wkup"]
pub type DownWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_WKUP` reader - Out wkup"]
pub type OutWkupR = crate::BitReader;
#[doc = "Field `OUT_WKUP` writer - Out wkup"]
pub type OutWkupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Ckflt en"]
    #[inline(always)]
    pub fn ckflt_en(&self) -> CkfltEnR {
        CkfltEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Trgflt en"]
    #[inline(always)]
    pub fn trgflt_en(&self) -> TrgfltEnR {
        TrgfltEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 19 - Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wave"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wavpol"]
    #[inline(always)]
    pub fn wavpol(&self) -> WavpolR {
        WavpolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Preload"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Countmode"]
    #[inline(always)]
    pub fn countmode(&self) -> CountmodeR {
        CountmodeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enc"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cmpm wkup"]
    #[inline(always)]
    pub fn cmpm_wkup(&self) -> CmpmWkupR {
        CmpmWkupR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Arrm wkup"]
    #[inline(always)]
    pub fn arrm_wkup(&self) -> ArrmWkupR {
        ArrmWkupR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Exttrig wkup"]
    #[inline(always)]
    pub fn exttrig_wkup(&self) -> ExttrigWkupR {
        ExttrigWkupR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Up wkup"]
    #[inline(always)]
    pub fn up_wkup(&self) -> UpWkupR {
        UpWkupR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Down wkup"]
    #[inline(always)]
    pub fn down_wkup(&self) -> DownWkupR {
        DownWkupR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Out wkup"]
    #[inline(always)]
    pub fn out_wkup(&self) -> OutWkupR {
        OutWkupR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Ckflt en"]
    #[inline(always)]
    pub fn ckflt_en(&mut self) -> CkfltEnW<'_, CfgrSpec> {
        CkfltEnW::new(self, 5)
    }
    #[doc = "Bit 8 - Trgflt en"]
    #[inline(always)]
    pub fn trgflt_en(&mut self) -> TrgfltEnW<'_, CfgrSpec> {
        TrgfltEnW::new(self, 8)
    }
    #[doc = "Bit 19 - Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, CfgrSpec> {
        TimeoutW::new(self, 19)
    }
    #[doc = "Bit 20 - Wave"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, CfgrSpec> {
        WaveW::new(self, 20)
    }
    #[doc = "Bit 21 - Wavpol"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WavpolW<'_, CfgrSpec> {
        WavpolW::new(self, 21)
    }
    #[doc = "Bit 22 - Preload"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<'_, CfgrSpec> {
        PreloadW::new(self, 22)
    }
    #[doc = "Bit 23 - Countmode"]
    #[inline(always)]
    pub fn countmode(&mut self) -> CountmodeW<'_, CfgrSpec> {
        CountmodeW::new(self, 23)
    }
    #[doc = "Bit 24 - Enc"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, CfgrSpec> {
        EncW::new(self, 24)
    }
    #[doc = "Bit 25 - Cmpm wkup"]
    #[inline(always)]
    pub fn cmpm_wkup(&mut self) -> CmpmWkupW<'_, CfgrSpec> {
        CmpmWkupW::new(self, 25)
    }
    #[doc = "Bit 26 - Arrm wkup"]
    #[inline(always)]
    pub fn arrm_wkup(&mut self) -> ArrmWkupW<'_, CfgrSpec> {
        ArrmWkupW::new(self, 26)
    }
    #[doc = "Bit 27 - Exttrig wkup"]
    #[inline(always)]
    pub fn exttrig_wkup(&mut self) -> ExttrigWkupW<'_, CfgrSpec> {
        ExttrigWkupW::new(self, 27)
    }
    #[doc = "Bit 28 - Up wkup"]
    #[inline(always)]
    pub fn up_wkup(&mut self) -> UpWkupW<'_, CfgrSpec> {
        UpWkupW::new(self, 28)
    }
    #[doc = "Bit 29 - Down wkup"]
    #[inline(always)]
    pub fn down_wkup(&mut self) -> DownWkupW<'_, CfgrSpec> {
        DownWkupW::new(self, 29)
    }
    #[doc = "Bit 30 - Out wkup"]
    #[inline(always)]
    pub fn out_wkup(&mut self) -> OutWkupW<'_, CfgrSpec> {
        OutWkupW::new(self, 30)
    }
}
#[doc = "LPTIMER configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
