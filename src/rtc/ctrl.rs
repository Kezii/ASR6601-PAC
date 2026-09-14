#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `WAKEUP2_FILTER_CFG` reader - Wakeup2 filter cfg"]
pub type Wakeup2FilterCfgR = crate::FieldReader;
#[doc = "Field `WAKEUP2_FILTER_CFG` writer - Wakeup2 filter cfg"]
pub type Wakeup2FilterCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAKEUP2_WKEN1` reader - Wakeup2 sr wakeup enable"]
pub type Wakeup2Wken1R = crate::BitReader;
#[doc = "Field `WAKEUP2_WKEN1` writer - Wakeup2 sr wakeup enable"]
pub type Wakeup2Wken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP2_WKEN0` reader - Wakeup2 level wakeup enable"]
pub type Wakeup2Wken0R = crate::BitReader;
#[doc = "Field `WAKEUP2_WKEN0` writer - Wakeup2 level wakeup enable"]
pub type Wakeup2Wken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP2_LEVEL_SEL` reader - Wakeup2 active level selection"]
pub type Wakeup2LevelSelR = crate::BitReader;
#[doc = "Field `WAKEUP2_LEVEL_SEL` writer - Wakeup2 active level selection"]
pub type Wakeup2LevelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP2_EN` reader - Wakeup2 enable"]
pub type Wakeup2EnR = crate::BitReader;
#[doc = "Field `WAKEUP2_EN` writer - Wakeup2 enable"]
pub type Wakeup2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_FILTER_CFG` reader - Wakeup1 filter cfg"]
pub type Wakeup1FilterCfgR = crate::FieldReader;
#[doc = "Field `WAKEUP1_FILTER_CFG` writer - Wakeup1 filter cfg"]
pub type Wakeup1FilterCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAKEUP1_WKEN1` reader - Wakeup1 sr wakeup enable"]
pub type Wakeup1Wken1R = crate::BitReader;
#[doc = "Field `WAKEUP1_WKEN1` writer - Wakeup1 sr wakeup enable"]
pub type Wakeup1Wken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_WKEN0` reader - Wakeup1 level wakeup enable"]
pub type Wakeup1Wken0R = crate::BitReader;
#[doc = "Field `WAKEUP1_WKEN0` writer - Wakeup1 level wakeup enable"]
pub type Wakeup1Wken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_LEVEL_SEL` reader - Wakeup1 active level selection"]
pub type Wakeup1LevelSelR = crate::BitReader;
#[doc = "Field `WAKEUP1_LEVEL_SEL` writer - Wakeup1 active level selection"]
pub type Wakeup1LevelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_EN` reader - Wakeup1 enable"]
pub type Wakeup1EnR = crate::BitReader;
#[doc = "Field `WAKEUP1_EN` writer - Wakeup1 enable"]
pub type Wakeup1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_FILTER_CFG` reader - Wakeup0 filter cfg"]
pub type Wakeup0FilterCfgR = crate::FieldReader;
#[doc = "Field `WAKEUP0_FILTER_CFG` writer - Wakeup0 filter cfg"]
pub type Wakeup0FilterCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAKEUP0_WKEN1` reader - Wakeup0 sr wakeup enable"]
pub type Wakeup0Wken1R = crate::BitReader;
#[doc = "Field `WAKEUP0_WKEN1` writer - Wakeup0 sr wakeup enable"]
pub type Wakeup0Wken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_WKEN0` reader - Wakeup0 level wakeup enable"]
pub type Wakeup0Wken0R = crate::BitReader;
#[doc = "Field `WAKEUP0_WKEN0` writer - Wakeup0 level wakeup enable"]
pub type Wakeup0Wken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_LEVEL_SEL` reader - Wakeup0 active level selection"]
pub type Wakeup0LevelSelR = crate::BitReader;
#[doc = "Field `WAKEUP0_LEVEL_SEL` writer - Wakeup0 active level selection"]
pub type Wakeup0LevelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_EN` reader - Wakeup0 enable"]
pub type Wakeup0EnR = crate::BitReader;
#[doc = "Field `WAKEUP0_EN` writer - Wakeup0 enable"]
pub type Wakeup0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_FILTER_CFG` reader - Tamper filter cfg"]
pub type TamperFilterCfgR = crate::FieldReader;
#[doc = "Field `TAMPER_FILTER_CFG` writer - Tamper filter cfg"]
pub type TamperFilterCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPER_WKEN1` reader - Tamper sr wakeup enable"]
pub type TamperWken1R = crate::BitReader;
#[doc = "Field `TAMPER_WKEN1` writer - Tamper sr wakeup enable"]
pub type TamperWken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_WKEN0` reader - Tamper level wakeup enable"]
pub type TamperWken0R = crate::BitReader;
#[doc = "Field `TAMPER_WKEN0` writer - Tamper level wakeup enable"]
pub type TamperWken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_LEVEL_SEL` reader - Tamper active level selection"]
pub type TamperLevelSelR = crate::BitReader;
#[doc = "Field `TAMPER_LEVEL_SEL` writer - Tamper active level selection"]
pub type TamperLevelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_EN` reader - Tamper enable"]
pub type TamperEnR = crate::BitReader;
#[doc = "Field `TAMPER_EN` writer - Tamper enable"]
pub type TamperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYC_START_COUNTER` reader - Periodic counter enable"]
pub type CycStartCounterR = crate::BitReader;
#[doc = "Field `CYC_START_COUNTER` writer - Periodic counter enable"]
pub type CycStartCounterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYC_WKEN` reader - Cyc sr wakeup enable"]
pub type CycWkenR = crate::BitReader;
#[doc = "Field `CYC_WKEN` writer - Cyc sr wakeup enable"]
pub type CycWkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_ALARM1_WKEN` reader - Alarm1 sr wakeup enable"]
pub type RtcAlarm1WkenR = crate::BitReader;
#[doc = "Field `RTC_ALARM1_WKEN` writer - Alarm1 sr wakeup enable"]
pub type RtcAlarm1WkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_ALARM0_WKEN` reader - Alarm0 sr wakeup enable"]
pub type RtcAlarm0WkenR = crate::BitReader;
#[doc = "Field `RTC_ALARM0_WKEN` writer - Alarm0 sr wakeup enable"]
pub type RtcAlarm0WkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_START_RTC` reader - Rtc calendar enable"]
pub type RtcStartRtcR = crate::BitReader;
#[doc = "Field `RTC_START_RTC` writer - Rtc calendar enable"]
pub type RtcStartRtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Wakeup2 filter cfg"]
    #[inline(always)]
    pub fn wakeup2_filter_cfg(&self) -> Wakeup2FilterCfgR {
        Wakeup2FilterCfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Wakeup2 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup2_wken1(&self) -> Wakeup2Wken1R {
        Wakeup2Wken1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup2 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup2_wken0(&self) -> Wakeup2Wken0R {
        Wakeup2Wken0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup2 active level selection"]
    #[inline(always)]
    pub fn wakeup2_level_sel(&self) -> Wakeup2LevelSelR {
        Wakeup2LevelSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup2 enable"]
    #[inline(always)]
    pub fn wakeup2_en(&self) -> Wakeup2EnR {
        Wakeup2EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Wakeup1 filter cfg"]
    #[inline(always)]
    pub fn wakeup1_filter_cfg(&self) -> Wakeup1FilterCfgR {
        Wakeup1FilterCfgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Wakeup1 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup1_wken1(&self) -> Wakeup1Wken1R {
        Wakeup1Wken1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup1 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup1_wken0(&self) -> Wakeup1Wken0R {
        Wakeup1Wken0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup1 active level selection"]
    #[inline(always)]
    pub fn wakeup1_level_sel(&self) -> Wakeup1LevelSelR {
        Wakeup1LevelSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup1 enable"]
    #[inline(always)]
    pub fn wakeup1_en(&self) -> Wakeup1EnR {
        Wakeup1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Wakeup0 filter cfg"]
    #[inline(always)]
    pub fn wakeup0_filter_cfg(&self) -> Wakeup0FilterCfgR {
        Wakeup0FilterCfgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Wakeup0 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup0_wken1(&self) -> Wakeup0Wken1R {
        Wakeup0Wken1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup0 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup0_wken0(&self) -> Wakeup0Wken0R {
        Wakeup0Wken0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup0 active level selection"]
    #[inline(always)]
    pub fn wakeup0_level_sel(&self) -> Wakeup0LevelSelR {
        Wakeup0LevelSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wakeup0 enable"]
    #[inline(always)]
    pub fn wakeup0_en(&self) -> Wakeup0EnR {
        Wakeup0EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Tamper filter cfg"]
    #[inline(always)]
    pub fn tamper_filter_cfg(&self) -> TamperFilterCfgR {
        TamperFilterCfgR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Tamper sr wakeup enable"]
    #[inline(always)]
    pub fn tamper_wken1(&self) -> TamperWken1R {
        TamperWken1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tamper level wakeup enable"]
    #[inline(always)]
    pub fn tamper_wken0(&self) -> TamperWken0R {
        TamperWken0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tamper active level selection"]
    #[inline(always)]
    pub fn tamper_level_sel(&self) -> TamperLevelSelR {
        TamperLevelSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tamper enable"]
    #[inline(always)]
    pub fn tamper_en(&self) -> TamperEnR {
        TamperEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Periodic counter enable"]
    #[inline(always)]
    pub fn cyc_start_counter(&self) -> CycStartCounterR {
        CycStartCounterR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cyc sr wakeup enable"]
    #[inline(always)]
    pub fn cyc_wken(&self) -> CycWkenR {
        CycWkenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Alarm1 sr wakeup enable"]
    #[inline(always)]
    pub fn rtc_alarm1_wken(&self) -> RtcAlarm1WkenR {
        RtcAlarm1WkenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Alarm0 sr wakeup enable"]
    #[inline(always)]
    pub fn rtc_alarm0_wken(&self) -> RtcAlarm0WkenR {
        RtcAlarm0WkenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rtc calendar enable"]
    #[inline(always)]
    pub fn rtc_start_rtc(&self) -> RtcStartRtcR {
        RtcStartRtcR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup2 filter cfg"]
    #[inline(always)]
    pub fn wakeup2_filter_cfg(&mut self) -> Wakeup2FilterCfgW<'_, CtrlSpec> {
        Wakeup2FilterCfgW::new(self, 0)
    }
    #[doc = "Bit 2 - Wakeup2 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup2_wken1(&mut self) -> Wakeup2Wken1W<'_, CtrlSpec> {
        Wakeup2Wken1W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup2 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup2_wken0(&mut self) -> Wakeup2Wken0W<'_, CtrlSpec> {
        Wakeup2Wken0W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup2 active level selection"]
    #[inline(always)]
    pub fn wakeup2_level_sel(&mut self) -> Wakeup2LevelSelW<'_, CtrlSpec> {
        Wakeup2LevelSelW::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup2 enable"]
    #[inline(always)]
    pub fn wakeup2_en(&mut self) -> Wakeup2EnW<'_, CtrlSpec> {
        Wakeup2EnW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Wakeup1 filter cfg"]
    #[inline(always)]
    pub fn wakeup1_filter_cfg(&mut self) -> Wakeup1FilterCfgW<'_, CtrlSpec> {
        Wakeup1FilterCfgW::new(self, 6)
    }
    #[doc = "Bit 8 - Wakeup1 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup1_wken1(&mut self) -> Wakeup1Wken1W<'_, CtrlSpec> {
        Wakeup1Wken1W::new(self, 8)
    }
    #[doc = "Bit 9 - Wakeup1 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup1_wken0(&mut self) -> Wakeup1Wken0W<'_, CtrlSpec> {
        Wakeup1Wken0W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup1 active level selection"]
    #[inline(always)]
    pub fn wakeup1_level_sel(&mut self) -> Wakeup1LevelSelW<'_, CtrlSpec> {
        Wakeup1LevelSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup1 enable"]
    #[inline(always)]
    pub fn wakeup1_en(&mut self) -> Wakeup1EnW<'_, CtrlSpec> {
        Wakeup1EnW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Wakeup0 filter cfg"]
    #[inline(always)]
    pub fn wakeup0_filter_cfg(&mut self) -> Wakeup0FilterCfgW<'_, CtrlSpec> {
        Wakeup0FilterCfgW::new(self, 12)
    }
    #[doc = "Bit 14 - Wakeup0 sr wakeup enable"]
    #[inline(always)]
    pub fn wakeup0_wken1(&mut self) -> Wakeup0Wken1W<'_, CtrlSpec> {
        Wakeup0Wken1W::new(self, 14)
    }
    #[doc = "Bit 15 - Wakeup0 level wakeup enable"]
    #[inline(always)]
    pub fn wakeup0_wken0(&mut self) -> Wakeup0Wken0W<'_, CtrlSpec> {
        Wakeup0Wken0W::new(self, 15)
    }
    #[doc = "Bit 16 - Wakeup0 active level selection"]
    #[inline(always)]
    pub fn wakeup0_level_sel(&mut self) -> Wakeup0LevelSelW<'_, CtrlSpec> {
        Wakeup0LevelSelW::new(self, 16)
    }
    #[doc = "Bit 17 - Wakeup0 enable"]
    #[inline(always)]
    pub fn wakeup0_en(&mut self) -> Wakeup0EnW<'_, CtrlSpec> {
        Wakeup0EnW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Tamper filter cfg"]
    #[inline(always)]
    pub fn tamper_filter_cfg(&mut self) -> TamperFilterCfgW<'_, CtrlSpec> {
        TamperFilterCfgW::new(self, 18)
    }
    #[doc = "Bit 20 - Tamper sr wakeup enable"]
    #[inline(always)]
    pub fn tamper_wken1(&mut self) -> TamperWken1W<'_, CtrlSpec> {
        TamperWken1W::new(self, 20)
    }
    #[doc = "Bit 21 - Tamper level wakeup enable"]
    #[inline(always)]
    pub fn tamper_wken0(&mut self) -> TamperWken0W<'_, CtrlSpec> {
        TamperWken0W::new(self, 21)
    }
    #[doc = "Bit 22 - Tamper active level selection"]
    #[inline(always)]
    pub fn tamper_level_sel(&mut self) -> TamperLevelSelW<'_, CtrlSpec> {
        TamperLevelSelW::new(self, 22)
    }
    #[doc = "Bit 23 - Tamper enable"]
    #[inline(always)]
    pub fn tamper_en(&mut self) -> TamperEnW<'_, CtrlSpec> {
        TamperEnW::new(self, 23)
    }
    #[doc = "Bit 24 - Periodic counter enable"]
    #[inline(always)]
    pub fn cyc_start_counter(&mut self) -> CycStartCounterW<'_, CtrlSpec> {
        CycStartCounterW::new(self, 24)
    }
    #[doc = "Bit 25 - Cyc sr wakeup enable"]
    #[inline(always)]
    pub fn cyc_wken(&mut self) -> CycWkenW<'_, CtrlSpec> {
        CycWkenW::new(self, 25)
    }
    #[doc = "Bit 26 - Alarm1 sr wakeup enable"]
    #[inline(always)]
    pub fn rtc_alarm1_wken(&mut self) -> RtcAlarm1WkenW<'_, CtrlSpec> {
        RtcAlarm1WkenW::new(self, 26)
    }
    #[doc = "Bit 27 - Alarm0 sr wakeup enable"]
    #[inline(always)]
    pub fn rtc_alarm0_wken(&mut self) -> RtcAlarm0WkenW<'_, CtrlSpec> {
        RtcAlarm0WkenW::new(self, 27)
    }
    #[doc = "Bit 28 - Rtc calendar enable"]
    #[inline(always)]
    pub fn rtc_start_rtc(&mut self) -> RtcStartRtcW<'_, CtrlSpec> {
        RtcStartRtcW::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
