#[doc = "Register `SUB_SECOND_CNT` reader"]
pub type R = crate::R<SubSecondCntSpec>;
#[doc = "Field `RTC_SUB_SECOND_VALUE` reader - Rtc subsecond value"]
pub type RtcSubSecondValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Rtc subsecond value"]
    #[inline(always)]
    pub fn rtc_sub_second_value(&self) -> RtcSubSecondValueR {
        RtcSubSecondValueR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "subsecond counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sub_second_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubSecondCntSpec;
impl crate::RegisterSpec for SubSecondCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub_second_cnt::R`](R) reader structure"]
impl crate::Readable for SubSecondCntSpec {}
