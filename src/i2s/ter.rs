#[doc = "Register `TER` reader"]
pub type R = crate::R<TerSpec>;
#[doc = "Register `TER` writer"]
pub type W = crate::W<TerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "transmitter enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ter::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TerSpec;
impl crate::RegisterSpec for TerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ter::R`](R) reader structure"]
impl crate::Readable for TerSpec {}
#[doc = "`write(|w| ..)` method takes [`ter::W`](W) writer structure"]
impl crate::Writable for TerSpec {
    type Safety = crate::Unsafe;
}
