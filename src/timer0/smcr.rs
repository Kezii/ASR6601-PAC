#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIMER slave Mode Control register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
