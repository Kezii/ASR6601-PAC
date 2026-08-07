#[doc = "Register `DHR` reader"]
pub type R = crate::R<DhrSpec>;
#[doc = "Register `DHR` writer"]
pub type W = crate::W<DhrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhrSpec;
impl crate::RegisterSpec for DhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr::R`](R) reader structure"]
impl crate::Readable for DhrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr::W`](W) writer structure"]
impl crate::Writable for DhrSpec {
    type Safety = crate::Unsafe;
}
