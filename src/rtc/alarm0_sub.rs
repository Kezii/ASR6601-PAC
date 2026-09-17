#[doc = "Register `ALARM0_SUB` reader"]
pub type R = crate::R<Alarm0SubSpec>;
#[doc = "Register `ALARM0_SUB` writer"]
pub type W = crate::W<Alarm0SubSpec>;
#[doc = "Field `RTC_ALARM0_SUB_VALUE` reader - Alarm0 subsecond value"]
pub type RtcAlarm0SubValueR = crate::FieldReader<u16>;
#[doc = "Field `RTC_ALARM0_SUB_VALUE` writer - Alarm0 subsecond value"]
pub type RtcAlarm0SubValueW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RTC_ALARM0_SUB_MASK` reader - Alarm0 subsecond mask"]
pub type RtcAlarm0SubMaskR = crate::FieldReader;
#[doc = "Field `RTC_ALARM0_SUB_MASK` writer - Alarm0 subsecond mask"]
pub type RtcAlarm0SubMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Alarm0 subsecond value"]
    #[inline(always)]
    pub fn rtc_alarm0_sub_value(&self) -> RtcAlarm0SubValueR {
        RtcAlarm0SubValueR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - Alarm0 subsecond mask"]
    #[inline(always)]
    pub fn rtc_alarm0_sub_mask(&self) -> RtcAlarm0SubMaskR {
        RtcAlarm0SubMaskR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Alarm0 subsecond value"]
    #[inline(always)]
    pub fn rtc_alarm0_sub_value(&mut self) -> RtcAlarm0SubValueW<'_, Alarm0SubSpec> {
        RtcAlarm0SubValueW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Alarm0 subsecond mask"]
    #[inline(always)]
    pub fn rtc_alarm0_sub_mask(&mut self) -> RtcAlarm0SubMaskW<'_, Alarm0SubSpec> {
        RtcAlarm0SubMaskW::new(self, 16)
    }
}
#[doc = "alarm0 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_sub::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_sub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0SubSpec;
impl crate::RegisterSpec for Alarm0SubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_sub::R`](R) reader structure"]
impl crate::Readable for Alarm0SubSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm0_sub::W`](W) writer structure"]
impl crate::Writable for Alarm0SubSpec {
    type Safety = crate::Unsafe;
}
