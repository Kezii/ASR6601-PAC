#[doc = "Register `ALARM1_SUBSECOND` reader"]
pub type R = crate::R<Alarm1SubsecondSpec>;
#[doc = "Register `ALARM1_SUBSECOND` writer"]
pub type W = crate::W<Alarm1SubsecondSpec>;
#[doc = "Field `RTC_ALARM1_SUB_VALUE` reader - Alarm1 subsecond value"]
pub type RtcAlarm1SubValueR = crate::FieldReader<u16>;
#[doc = "Field `RTC_ALARM1_SUB_VALUE` writer - Alarm1 subsecond value"]
pub type RtcAlarm1SubValueW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RTC_ALARM1_SUB_MASK` reader - Alarm1 subsecond mask"]
pub type RtcAlarm1SubMaskR = crate::FieldReader;
#[doc = "Field `RTC_ALARM1_SUB_MASK` writer - Alarm1 subsecond mask"]
pub type RtcAlarm1SubMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Alarm1 subsecond value"]
    #[inline(always)]
    pub fn rtc_alarm1_sub_value(&self) -> RtcAlarm1SubValueR {
        RtcAlarm1SubValueR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - Alarm1 subsecond mask"]
    #[inline(always)]
    pub fn rtc_alarm1_sub_mask(&self) -> RtcAlarm1SubMaskR {
        RtcAlarm1SubMaskR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Alarm1 subsecond value"]
    #[inline(always)]
    pub fn rtc_alarm1_sub_value(&mut self) -> RtcAlarm1SubValueW<'_, Alarm1SubsecondSpec> {
        RtcAlarm1SubValueW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Alarm1 subsecond mask"]
    #[inline(always)]
    pub fn rtc_alarm1_sub_mask(&mut self) -> RtcAlarm1SubMaskW<'_, Alarm1SubsecondSpec> {
        RtcAlarm1SubMaskW::new(self, 16)
    }
}
#[doc = "alarm1 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1_subsecond::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1_subsecond::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm1SubsecondSpec;
impl crate::RegisterSpec for Alarm1SubsecondSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm1_subsecond::R`](R) reader structure"]
impl crate::Readable for Alarm1SubsecondSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm1_subsecond::W`](W) writer structure"]
impl crate::Writable for Alarm1SubsecondSpec {
    type Safety = crate::Unsafe;
}
