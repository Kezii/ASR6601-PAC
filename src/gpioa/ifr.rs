#[doc = "Register `IFR` reader"]
pub type R = crate::R<IfrSpec>;
#[doc = "Register `IFR` writer"]
pub type W = crate::W<IfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfrSpec;
impl crate::RegisterSpec for IfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifr::R`](R) reader structure"]
impl crate::Readable for IfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifr::W`](W) writer structure"]
impl crate::Writable for IfrSpec {
    type Safety = crate::Unsafe;
}
