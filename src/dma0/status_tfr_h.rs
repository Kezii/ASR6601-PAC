#[doc = "Register `STATUS_TFR_H` reader"]
pub type R = crate::R<StatusTfrHSpec>;
#[doc = "Register `STATUS_TFR_H` writer"]
pub type W = crate::W<StatusTfrHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status_tfr_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_tfr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusTfrHSpec;
impl crate::RegisterSpec for StatusTfrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_tfr_h::R`](R) reader structure"]
impl crate::Readable for StatusTfrHSpec {}
#[doc = "`write(|w| ..)` method takes [`status_tfr_h::W`](W) writer structure"]
impl crate::Writable for StatusTfrHSpec {
    type Safety = crate::Unsafe;
}
