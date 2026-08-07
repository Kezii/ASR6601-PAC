#[doc = "Register `DBR` reader"]
pub type R = crate::R<DbrSpec>;
#[doc = "Register `DBR` writer"]
pub type W = crate::W<DbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "data buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbrSpec;
impl crate::RegisterSpec for DbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbr::R`](R) reader structure"]
impl crate::Readable for DbrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbr::W`](W) writer structure"]
impl crate::Writable for DbrSpec {
    type Safety = crate::Unsafe;
}
