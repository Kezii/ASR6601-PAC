#[doc = "Register `ALARM0_SUBSECOND` reader"]
pub type R = crate::R<Alarm0SubsecondSpec>;
#[doc = "Register `ALARM0_SUBSECOND` writer"]
pub type W = crate::W<Alarm0SubsecondSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "alarm0 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_subsecond::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_subsecond::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0SubsecondSpec;
impl crate::RegisterSpec for Alarm0SubsecondSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_subsecond::R`](R) reader structure"]
impl crate::Readable for Alarm0SubsecondSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm0_subsecond::W`](W) writer structure"]
impl crate::Writable for Alarm0SubsecondSpec {
    type Safety = crate::Unsafe;
}
