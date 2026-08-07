#[doc = "Register `STATUS_BLOCK_H` reader"]
pub type R = crate::R<StatusBlockHSpec>;
#[doc = "Register `STATUS_BLOCK_H` writer"]
pub type W = crate::W<StatusBlockHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status_block_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_block_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusBlockHSpec;
impl crate::RegisterSpec for StatusBlockHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_block_h::R`](R) reader structure"]
impl crate::Readable for StatusBlockHSpec {}
#[doc = "`write(|w| ..)` method takes [`status_block_h::W`](W) writer structure"]
impl crate::Writable for StatusBlockHSpec {
    type Safety = crate::Unsafe;
}
