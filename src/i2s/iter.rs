#[doc = "Register `ITER` reader"]
pub type R = crate::R<IterSpec>;
#[doc = "Register `ITER` writer"]
pub type W = crate::W<IterSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "transmitter block enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iter::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IterSpec;
impl crate::RegisterSpec for IterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iter::R`](R) reader structure"]
impl crate::Readable for IterSpec {}
#[doc = "`write(|w| ..)` method takes [`iter::W`](W) writer structure"]
impl crate::Writable for IterSpec {
    type Safety = crate::Unsafe;
}
