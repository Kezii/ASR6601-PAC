#[doc = "Register `ALARM1_SUBSECOND` reader"]
pub type R = crate::R<Alarm1SubsecondSpec>;
#[doc = "Register `ALARM1_SUBSECOND` writer"]
pub type W = crate::W<Alarm1SubsecondSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
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
