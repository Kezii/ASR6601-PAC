#[doc = "Register `RAW_BLOCK_L` reader"]
pub type R = crate::R<RawBlockLSpec>;
#[doc = "Register `RAW_BLOCK_L` writer"]
pub type W = crate::W<RawBlockLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_block_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_block_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawBlockLSpec;
impl crate::RegisterSpec for RawBlockLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_block_l::R`](R) reader structure"]
impl crate::Readable for RawBlockLSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_block_l::W`](W) writer structure"]
impl crate::Writable for RawBlockLSpec {
    type Safety = crate::Unsafe;
}
