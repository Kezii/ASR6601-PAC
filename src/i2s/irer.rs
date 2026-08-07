#[doc = "Register `IRER` reader"]
pub type R = crate::R<IrerSpec>;
#[doc = "Register `IRER` writer"]
pub type W = crate::W<IrerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "receiver block enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrerSpec;
impl crate::RegisterSpec for IrerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irer::R`](R) reader structure"]
impl crate::Readable for IrerSpec {}
#[doc = "`write(|w| ..)` method takes [`irer::W`](W) writer structure"]
impl crate::Writable for IrerSpec {
    type Safety = crate::Unsafe;
}
