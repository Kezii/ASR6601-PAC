#[doc = "Register `DIFFSEL` reader"]
pub type R = crate::R<DiffselSpec>;
#[doc = "Register `DIFFSEL` writer"]
pub type W = crate::W<DiffselSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "difference register\n\nYou can [`read`](crate::Reg::read) this register and get [`diffsel::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diffsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiffselSpec;
impl crate::RegisterSpec for DiffselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diffsel::R`](R) reader structure"]
impl crate::Readable for DiffselSpec {}
#[doc = "`write(|w| ..)` method takes [`diffsel::W`](W) writer structure"]
impl crate::Writable for DiffselSpec {
    type Safety = crate::Unsafe;
}
