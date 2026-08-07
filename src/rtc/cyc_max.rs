#[doc = "Register `CYC_MAX` reader"]
pub type R = crate::R<CycMaxSpec>;
#[doc = "Register `CYC_MAX` writer"]
pub type W = crate::W<CycMaxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "cyc max value\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_max::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cyc_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycMaxSpec;
impl crate::RegisterSpec for CycMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cyc_max::R`](R) reader structure"]
impl crate::Readable for CycMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`cyc_max::W`](W) writer structure"]
impl crate::Writable for CycMaxSpec {
    type Safety = crate::Unsafe;
}
