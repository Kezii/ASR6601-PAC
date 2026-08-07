#[doc = "Register `INIT` reader"]
pub type R = crate::R<InitSpec>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<InitSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "init value register\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitSpec;
impl crate::RegisterSpec for InitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for InitSpec {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for InitSpec {
    type Safety = crate::Unsafe;
}
