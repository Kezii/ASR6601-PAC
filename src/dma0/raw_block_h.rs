#[doc = "Register `RAW_BLOCK_H` reader"]
pub type R = crate::R<RawBlockHSpec>;
#[doc = "Register `RAW_BLOCK_H` writer"]
pub type W = crate::W<RawBlockHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_block_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_block_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawBlockHSpec;
impl crate::RegisterSpec for RawBlockHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_block_h::R`](R) reader structure"]
impl crate::Readable for RawBlockHSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_block_h::W`](W) writer structure"]
impl crate::Writable for RawBlockHSpec {
    type Safety = crate::Unsafe;
}
