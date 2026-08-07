#[doc = "Register `PER` reader"]
pub type R = crate::R<PerSpec>;
#[doc = "Register `PER` writer"]
pub type W = crate::W<PerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "pull enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerSpec;
impl crate::RegisterSpec for PerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per::R`](R) reader structure"]
impl crate::Readable for PerSpec {}
#[doc = "`write(|w| ..)` method takes [`per::W`](W) writer structure"]
impl crate::Writable for PerSpec {
    type Safety = crate::Unsafe;
}
