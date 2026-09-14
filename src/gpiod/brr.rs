#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR` writer - pin\\[15:0\\] output data clear"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output data clear"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, BrrSpec> {
        BrW::new(self, 0)
    }
}
#[doc = "bit reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
