#[doc = "Register `STATUS_INT` reader"]
pub type R = crate::R<StatusIntSpec>;
#[doc = "Register `STATUS_INT` writer"]
pub type W = crate::W<StatusIntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status_int::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusIntSpec;
impl crate::RegisterSpec for StatusIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_int::R`](R) reader structure"]
impl crate::Readable for StatusIntSpec {}
#[doc = "`write(|w| ..)` method takes [`status_int::W`](W) writer structure"]
impl crate::Writable for StatusIntSpec {
    type Safety = crate::Unsafe;
}
