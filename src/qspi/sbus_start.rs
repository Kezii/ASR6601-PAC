#[doc = "Register `SBUS_START` reader"]
pub type R = crate::R<SbusStartSpec>;
#[doc = "Register `SBUS_START` writer"]
pub type W = crate::W<SbusStartSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "start register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbus_start::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbus_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbusStartSpec;
impl crate::RegisterSpec for SbusStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbus_start::R`](R) reader structure"]
impl crate::Readable for SbusStartSpec {}
#[doc = "`write(|w| ..)` method takes [`sbus_start::W`](W) writer structure"]
impl crate::Writable for SbusStartSpec {
    type Safety = crate::Unsafe;
}
