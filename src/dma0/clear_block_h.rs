#[doc = "Register `CLEAR_BLOCK_H` reader"]
pub type R = crate::R<ClearBlockHSpec>;
#[doc = "Register `CLEAR_BLOCK_H` writer"]
pub type W = crate::W<ClearBlockHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_block_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_block_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearBlockHSpec;
impl crate::RegisterSpec for ClearBlockHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_block_h::R`](R) reader structure"]
impl crate::Readable for ClearBlockHSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_block_h::W`](W) writer structure"]
impl crate::Writable for ClearBlockHSpec {
    type Safety = crate::Unsafe;
}
