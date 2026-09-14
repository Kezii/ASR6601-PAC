#[doc = "Register `CR7` reader"]
pub type R = crate::R<Cr7Spec>;
#[doc = "Register `CR7` writer"]
pub type W = crate::W<Cr7Spec>;
#[doc = "Field `RTC_TAMPER_SECURE_LOCK` reader - tamper configuration security lock in rtc"]
pub type RtcTamperSecureLockR = crate::BitReader;
#[doc = "Field `RTC_TAMPER_SECURE_LOCK` writer - tamper configuration security lock in rtc"]
pub type RtcTamperSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_WAKEUP0_SECURE_LOCK` reader - wakeup0 configuration security lock in rtc"]
pub type RtcWakeup0SecureLockR = crate::BitReader;
#[doc = "Field `RTC_WAKEUP0_SECURE_LOCK` writer - wakeup0 configuration security lock in rtc"]
pub type RtcWakeup0SecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_WAKEUP1_SECURE_LOCK` reader - wakeup1 configuration security lock in rtc"]
pub type RtcWakeup1SecureLockR = crate::BitReader;
#[doc = "Field `RTC_WAKEUP1_SECURE_LOCK` writer - wakeup1 configuration security lock in rtc"]
pub type RtcWakeup1SecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_WAKEUP2_SECURE_LOCK` reader - wakeup2 configuration security lock in rtc"]
pub type RtcWakeup2SecureLockR = crate::BitReader;
#[doc = "Field `RTC_WAKEUP2_SECURE_LOCK` writer - wakeup2 configuration security lock in rtc"]
pub type RtcWakeup2SecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_CALENDAR_SECURE_LOCK` reader - calendar configuration security lock in rtc"]
pub type RtcCalendarSecureLockR = crate::BitReader;
#[doc = "Field `RTC_CALENDAR_SECURE_LOCK` writer - calendar configuration security lock in rtc"]
pub type RtcCalendarSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANALOG_AON_SECURE_LOCK` reader - security lock for aon domain configuration of afec"]
pub type AnalogAonSecureLockR = crate::FieldReader<u16>;
#[doc = "Field `ANALOG_AON_SECURE_LOCK` writer - security lock for aon domain configuration of afec"]
pub type AnalogAonSecureLockW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - tamper configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_tamper_secure_lock(&self) -> RtcTamperSecureLockR {
        RtcTamperSecureLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup0 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup0_secure_lock(&self) -> RtcWakeup0SecureLockR {
        RtcWakeup0SecureLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - wakeup1 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup1_secure_lock(&self) -> RtcWakeup1SecureLockR {
        RtcWakeup1SecureLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup2 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup2_secure_lock(&self) -> RtcWakeup2SecureLockR {
        RtcWakeup2SecureLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - calendar configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_calendar_secure_lock(&self) -> RtcCalendarSecureLockR {
        RtcCalendarSecureLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:14 - security lock for aon domain configuration of afec"]
    #[inline(always)]
    pub fn analog_aon_secure_lock(&self) -> AnalogAonSecureLockR {
        AnalogAonSecureLockR::new(((self.bits >> 5) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - tamper configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_tamper_secure_lock(&mut self) -> RtcTamperSecureLockW<'_, Cr7Spec> {
        RtcTamperSecureLockW::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup0 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup0_secure_lock(&mut self) -> RtcWakeup0SecureLockW<'_, Cr7Spec> {
        RtcWakeup0SecureLockW::new(self, 1)
    }
    #[doc = "Bit 2 - wakeup1 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup1_secure_lock(&mut self) -> RtcWakeup1SecureLockW<'_, Cr7Spec> {
        RtcWakeup1SecureLockW::new(self, 2)
    }
    #[doc = "Bit 3 - wakeup2 configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_wakeup2_secure_lock(&mut self) -> RtcWakeup2SecureLockW<'_, Cr7Spec> {
        RtcWakeup2SecureLockW::new(self, 3)
    }
    #[doc = "Bit 4 - calendar configuration security lock in rtc"]
    #[inline(always)]
    pub fn rtc_calendar_secure_lock(&mut self) -> RtcCalendarSecureLockW<'_, Cr7Spec> {
        RtcCalendarSecureLockW::new(self, 4)
    }
    #[doc = "Bits 5:14 - security lock for aon domain configuration of afec"]
    #[inline(always)]
    pub fn analog_aon_secure_lock(&mut self) -> AnalogAonSecureLockW<'_, Cr7Spec> {
        AnalogAonSecureLockW::new(self, 5)
    }
}
#[doc = "control register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`cr7::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr7Spec;
impl crate::RegisterSpec for Cr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr7::R`](R) reader structure"]
impl crate::Readable for Cr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cr7::W`](W) writer structure"]
impl crate::Writable for Cr7Spec {
    type Safety = crate::Unsafe;
}
