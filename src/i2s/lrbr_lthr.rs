#[doc = "Register `LRBR_LTHR` reader"]
pub type R = crate::R<LrbrLthrSpec>;
#[doc = "Register `LRBR_LTHR` writer"]
pub type W = crate::W<LrbrLthrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "right receive buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`lrbr_lthr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrbr_lthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrbrLthrSpec;
impl crate::RegisterSpec for LrbrLthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrbr_lthr::R`](R) reader structure"]
impl crate::Readable for LrbrLthrSpec {}
#[doc = "`write(|w| ..)` method takes [`lrbr_lthr::W`](W) writer structure"]
impl crate::Writable for LrbrLthrSpec {
    type Safety = crate::Unsafe;
}
